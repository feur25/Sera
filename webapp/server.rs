use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

use super::http::{authorized, html_response, not_found_response, not_authorized_response, parse_request};
use super::ws::{accept_key, decode_frame, encode_text_frame, OPCODE_CLOSE, OPCODE_PING, OPCODE_PONG, OPCODE_TEXT};

pub trait EventDispatcher: Send + Sync {
    fn page_html(&self, path: &str) -> Option<String>;
    fn on_event(&self, session: u64, component_id: &str, value: &str) -> Vec<(String, String)>;
    fn open_session(&self) -> u64;
    fn credentials(&self) -> Option<(String, String)>;
    fn subscribe(&self) -> broadcast::Receiver<(String, String)>;
    fn timer_intervals(&self) -> Vec<f64>;
    fn tick_timer(&self, index: usize);
}

pub async fn run_server<D: EventDispatcher + 'static>(addr: &str, dispatcher: Arc<D>) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    for (index, secs) in dispatcher.timer_intervals().into_iter().enumerate() {
        let dispatcher = dispatcher.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs_f64(secs.max(0.01)));
            loop {
                ticker.tick().await;
                dispatcher.tick_timer(index);
            }
        });
    }

    loop {
        let (stream, _) = listener.accept().await?;
        let dispatcher = dispatcher.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, dispatcher).await {
                eprintln!("seraplot webapp: connection error: {e}");
            }
        });
    }
}

async fn handle_connection<D: EventDispatcher + 'static>(mut stream: TcpStream, dispatcher: Arc<D>) -> std::io::Result<()> {
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }
    let raw = String::from_utf8_lossy(&buf[..n]);
    let req = match parse_request(&raw) {
        Some(r) => r,
        None => return Ok(()),
    };

    if !authorized(&req, dispatcher.credentials()) {
        stream.write_all(not_authorized_response().as_bytes()).await?;
        return Ok(());
    }

    if req.path == "/ws" {
        if let Some(key) = req.headers.get("sec-websocket-key") {
            let accept = accept_key(key);
            let response = format!(
                "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\n\r\n"
            );
            stream.write_all(response.as_bytes()).await?;
            let session = dispatcher.open_session();
            let pushes = dispatcher.subscribe();
            return handle_ws_loop(stream, dispatcher, session, pushes).await;
        }
        return Ok(());
    }

    let response = match dispatcher.page_html(&req.path) {
        Some(body) => html_response(&body),
        None => not_found_response(),
    };
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

async fn handle_ws_loop<D: EventDispatcher + 'static>(
    mut stream: TcpStream,
    dispatcher: Arc<D>,
    session: u64,
    mut pushes: broadcast::Receiver<(String, String)>,
) -> std::io::Result<()> {
    let mut pending = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        tokio::select! {
            read = stream.read(&mut chunk) => {
                let n = read?;
                if n == 0 {
                    return Ok(());
                }
                pending.extend_from_slice(&chunk[..n]);

                while let Some(frame) = decode_frame(&pending) {
                    let consumed = frame.consumed;
                    match frame.opcode {
                        OPCODE_TEXT => match String::from_utf8(frame.payload) {
                            Ok(text) => match serde_json::from_str::<serde_json::Value>(&text) {
                                Ok(msg) => {
                                    let id = msg.get("id").and_then(|v| v.as_str()).unwrap_or("");
                                    let value = msg.get("value").and_then(|v| v.as_str()).unwrap_or("");
                                    for (out_id, html) in dispatcher.on_event(session, id, value) {
                                        let update = serde_json::json!({ "type": "update", "id": out_id, "html": html });
                                        stream.write_all(&encode_text_frame(&update.to_string())).await?;
                                    }
                                }
                                Err(e) => eprintln!("seraplot webapp: malformed ws message JSON: {e}"),
                            },
                            Err(e) => eprintln!("seraplot webapp: non-utf8 ws frame: {e}"),
                        },
                        OPCODE_PING => {
                            stream.write_all(&super::ws::encode_frame(OPCODE_PONG, &frame.payload)).await?;
                        }
                        OPCODE_CLOSE => {
                            return Ok(());
                        }
                        _ => {}
                    }
                    pending.drain(..consumed);
                }
            }
            pushed = pushes.recv() => {
                match pushed {
                    Ok((out_id, html)) => {
                        let update = serde_json::json!({ "type": "update", "id": out_id, "html": html });
                        stream.write_all(&encode_text_frame(&update.to_string())).await?;
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(broadcast::error::RecvError::Closed) => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::webapp::ws::accept_key;

    struct TestDispatcher {
        pushes: broadcast::Sender<(String, String)>,
    }

    impl TestDispatcher {
        fn new() -> Self {
            Self { pushes: broadcast::channel(8).0 }
        }
    }

    impl EventDispatcher for TestDispatcher {
        fn page_html(&self, path: &str) -> Option<String> {
            match path {
                "/" => Some("<html><body>test-page</body></html>".to_string()),
                "/about" => Some("<html><body>about-page</body></html>".to_string()),
                _ => None,
            }
        }

        fn on_event(&self, _session: u64, component_id: &str, value: &str) -> Vec<(String, String)> {
            vec![("out".to_string(), format!("{component_id}:{value}"))]
        }

        fn open_session(&self) -> u64 {
            1
        }

        fn credentials(&self) -> Option<(String, String)> {
            None
        }

        fn subscribe(&self) -> broadcast::Receiver<(String, String)> {
            self.pushes.subscribe()
        }

        fn timer_intervals(&self) -> Vec<f64> {
            Vec::new()
        }

        fn tick_timer(&self, _index: usize) {}
    }

    #[tokio::test]
    async fn full_http_and_websocket_round_trip_over_real_tcp() {
        let addr = "127.0.0.1:18787";
        tokio::spawn(run_server(addr, Arc::new(TestDispatcher::new())));
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        let mut plain = TcpStream::connect(addr).await.unwrap();
        plain.write_all(b"GET / HTTP/1.1\r\nHost: x\r\n\r\n").await.unwrap();
        let mut buf = vec![0u8; 4096];
        let n = plain.read(&mut buf).await.unwrap();
        assert!(String::from_utf8_lossy(&buf[..n]).contains("test-page"));

        let mut about = TcpStream::connect(addr).await.unwrap();
        about.write_all(b"GET /about HTTP/1.1\r\nHost: x\r\n\r\n").await.unwrap();
        let mut buf_about = vec![0u8; 4096];
        let n_about = about.read(&mut buf_about).await.unwrap();
        assert!(String::from_utf8_lossy(&buf_about[..n_about]).contains("about-page"));

        let mut missing = TcpStream::connect(addr).await.unwrap();
        missing.write_all(b"GET /nope HTTP/1.1\r\nHost: x\r\n\r\n").await.unwrap();
        let mut buf_missing = vec![0u8; 4096];
        let n_missing = missing.read(&mut buf_missing).await.unwrap();
        assert!(String::from_utf8_lossy(&buf_missing[..n_missing]).starts_with("HTTP/1.1 404"));

        let mut ws = TcpStream::connect(addr).await.unwrap();
        let key = "dGhlIHNhbXBsZSBub25jZQ==";
        let req = format!("GET /ws HTTP/1.1\r\nHost: x\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {key}\r\n\r\n");
        ws.write_all(req.as_bytes()).await.unwrap();
        let mut buf2 = vec![0u8; 4096];
        let n2 = ws.read(&mut buf2).await.unwrap();
        let resp2 = String::from_utf8_lossy(&buf2[..n2]);
        assert!(resp2.contains("101 Switching Protocols"));
        assert!(resp2.contains(&accept_key(key)));

        let msg = serde_json::json!({ "id": "dropdown-1", "value": "B" }).to_string();
        let mask = [0x12u8, 0x34, 0x56, 0x78];
        let mut frame = vec![0x80 | OPCODE_TEXT, 0x80 | (msg.len() as u8)];
        frame.extend_from_slice(&mask);
        for (i, b) in msg.bytes().enumerate() {
            frame.push(b ^ mask[i % 4]);
        }
        ws.write_all(&frame).await.unwrap();

        let mut buf3 = vec![0u8; 4096];
        let n3 = ws.read(&mut buf3).await.unwrap();
        let decoded = decode_frame(&buf3[..n3]).unwrap();
        let text = String::from_utf8(decoded.payload).unwrap();
        let v: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["type"], "update");
        assert_eq!(v["id"], "out");
        assert_eq!(v["html"], "dropdown-1:B");
    }

    #[tokio::test]
    async fn wrong_basic_auth_credentials_get_401() {
        struct AuthedDispatcher(broadcast::Sender<(String, String)>);
        impl EventDispatcher for AuthedDispatcher {
            fn page_html(&self, _path: &str) -> Option<String> {
                Some("<html></html>".to_string())
            }
            fn on_event(&self, _session: u64, _id: &str, _value: &str) -> Vec<(String, String)> {
                Vec::new()
            }
            fn open_session(&self) -> u64 {
                1
            }
            fn credentials(&self) -> Option<(String, String)> {
                Some(("admin".to_string(), "secret".to_string()))
            }
            fn subscribe(&self) -> broadcast::Receiver<(String, String)> {
                self.0.subscribe()
            }
            fn timer_intervals(&self) -> Vec<f64> {
                Vec::new()
            }
            fn tick_timer(&self, _index: usize) {}
        }

        let addr = "127.0.0.1:18788";
        tokio::spawn(run_server(addr, Arc::new(AuthedDispatcher(broadcast::channel(1).0))));
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream.write_all(b"GET / HTTP/1.1\r\nHost: x\r\n\r\n").await.unwrap();
        let mut buf = vec![0u8; 4096];
        let n = stream.read(&mut buf).await.unwrap();
        assert!(String::from_utf8_lossy(&buf[..n]).starts_with("HTTP/1.1 401"));
    }
}
