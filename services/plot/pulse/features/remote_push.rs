use crate::Chart;
use pyo3::prelude::*;
use seraplot_macros::sera_doc_impl;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

fn constant_time_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

struct ParsedRequest {
    content_length: usize,
    auth_token: Option<String>,
}

fn parse_headers(reader: &mut BufReader<&TcpStream>) -> Option<ParsedRequest> {
    let mut content_length = 0usize;
    let mut auth_token = None;
    let mut line = String::new();
    loop {
        line.clear();
        if reader.read_line(&mut line).ok()? == 0 {
            return None;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        if trimmed.len() >= 15 && trimmed[..15].eq_ignore_ascii_case("content-length:") {
            content_length = trimmed[15..].trim().parse().ok()?;
        } else if trimmed.len() >= 14 && trimmed[..14].eq_ignore_ascii_case("authorization:") {
            let value = trimmed[14..].trim();
            let token = value.strip_prefix("Bearer ").or_else(|| value.strip_prefix("bearer ")).unwrap_or(value);
            auth_token = Some(token.to_string());
        }
    }
    Some(ParsedRequest { content_length, auth_token })
}

fn read_request(stream: &TcpStream) -> Option<(Vec<u8>, Option<String>)> {
    let mut reader = BufReader::new(stream);
    let parsed = parse_headers(&mut reader)?;
    if parsed.content_length == 0 || parsed.content_length > 64 * 1024 * 1024 {
        return None;
    }
    let mut body = vec![0u8; parsed.content_length];
    reader.read_exact(&mut body).ok()?;
    Some((body, parsed.auth_token))
}

fn respond(stream: &TcpStream, status: u16, reason: &str) {
    let mut w = stream;
    let _ = write!(w, "HTTP/1.1 {status} {reason}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
}

fn validate_frame(hid: u64, body: &[u8]) -> bool {
    if body.len() < 12 {
        return false;
    }
    let recv_hid = u32::from_le_bytes(body[0..4].try_into().unwrap());
    if recv_hid as u64 != hid {
        return false;
    }
    let n = u32::from_le_bytes(body[4..8].try_into().unwrap()) as usize;
    let vpp = u32::from_le_bytes(body[8..12].try_into().unwrap()) as usize;
    let Some(idx_end) = 12usize.checked_add(n.checked_mul(4).unwrap_or(usize::MAX)) else {
        return false;
    };
    let Some(expected_len) = idx_end.checked_add(n.checked_mul(vpp).and_then(|v| v.checked_mul(2)).unwrap_or(usize::MAX)) else {
        return false;
    };
    body.len() == expected_len
}

fn handle_connection(hid: u64, stream: TcpStream, expected_token: Option<&str>) {
    let Some((body, provided_token)) = read_request(&stream) else {
        respond(&stream, 400, "Bad Request");
        return;
    };
    if let Some(expected) = expected_token {
        let authorized = provided_token.as_deref().map(|t| constant_time_eq(t, expected)).unwrap_or(false);
        if !authorized {
            super::metrics::incr("seraplot_pulse_remote_push_unauthorized_total", hid);
            respond(&stream, 401, "Unauthorized");
            return;
        }
    }
    if !validate_frame(hid, &body) {
        respond(&stream, 400, "Bad Request");
        return;
    }
    let Some(meta) = crate::plot::push_registry::get(hid) else {
        respond(&stream, 503, "Service Unavailable");
        return;
    };
    let vpp = u32::from_le_bytes(body[8..12].try_into().unwrap());
    if vpp != meta.values_per_point() {
        respond(&stream, 400, "Bad Request");
        return;
    }
    if !super::rate_limit::check(hid) {
        super::metrics::incr("seraplot_pulse_remote_push_rejected_total", hid);
        respond(&stream, 429, "Too Many Requests");
        return;
    }
    super::metrics::incr("seraplot_pulse_remote_push_total", hid);
    super::health::touch("remote_push", Some(hid));
    if let Some(rec) = crate::plot::record_registry::get(hid) {
        rec.append(&body);
    }
    #[cfg(feature = "webapp")]
    if let Some(tx) = crate::plot::push_registry::get_push_tx(hid) {
        let _ = tx.send((crate::webapp::ws::OPCODE_BINARY, body));
    }
    respond(&stream, 204, "No Content");
}

pub fn serve_ingest_blocking(hid: u64, host: &str, port: u16, token: Option<String>) -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind((host, port))?;
    super::health::register("remote_push", Some(hid), host, listener.local_addr().map(|a| a.port()).unwrap_or(port));
    for stream in listener.incoming().flatten() {
        handle_connection(hid, stream, token.as_deref());
    }
    Ok(())
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pyo3::pymethods]
impl Chart {
    #[pyo3(signature = (host = "127.0.0.1", port = 8788, token = None))]
    #[sera_doc(
        category = "chart_method",
        module = "remote",
        file = "seraplot/pulse-pricing.md",
        aliases("listen_push"),
        en = "Starts a tiny HTTP ingest endpoint so non-Python processes (curl, a Go/Node/Rust service, a shell script) can push delta updates without going through Python's push(). POST the exact same binary frame push()/serve() already use ([hid u32][n u32][vpp u32][indices u32...][values i16...], all little-endian) to this address; it's forwarded to any active .serve() and recording exactly like a local push() would be. Pass token to require an 'Authorization: Bearer <token>' header on every request - unset by default, matching serve()'s optional username/password, but strongly recommended beyond localhost.",
        fr = "Démarre un petit point d'entrée HTTP pour que des processus non-Python (curl, un service Go/Node/Rust, un script shell) puissent pousser des mises à jour delta sans passer par push() en Python. Envoyez en POST exactement la même trame binaire que push()/serve() utilisent déjà ([hid u32][n u32][vpp u32][indices u32...][values i16...], little-endian), elle est relayée vers tout .serve() actif et tout enregistrement, exactement comme un push() local. Passez token pour exiger un en-tête 'Authorization: Bearer <token>' sur chaque requête — non défini par défaut, comme le username/password optionnel de serve(), mais fortement recommandé au-delà de localhost.",
        param(name = "host", ty = "str", en = "Address to bind the ingest endpoint on.", fr = "Adresse sur laquelle lier le point d'entrée."),
        param(name = "port", ty = "int", en = "Port to bind the ingest endpoint on.", fr = "Port sur lequel lier le point d'entrée."),
        param(name = "token", ty = "str | None", en = "If set, every request must carry a matching 'Authorization: Bearer <token>' header or gets a 401.", fr = "Si défini, chaque requête doit porter un en-tête 'Authorization: Bearer <token>' correspondant, sinon 401.")
    )]
    fn enable_remote_push(&self, host: &str, port: u16, token: Option<&str>) -> PyResult<()> {
        let hid = super::require_hid(&self.html)?;
        let host = host.to_string();
        let token = token.map(|t| t.to_string());
        std::thread::spawn(move || {
            let _ = serve_ingest_blocking(hid, &host, port, token);
        });
        Ok(())
    }

    #[pyo3(signature = (host = "127.0.0.1", port = 8788, token = None))]
    fn listen_push(&self, host: &str, port: u16, token: Option<&str>) -> PyResult<()> {
        self.enable_remote_push(host, port, token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(hid: u64, indices: &[u32], values: &[i16], vpp: u32) -> Vec<u8> {
        let n = indices.len() as u32;
        let mut f = Vec::with_capacity(12 + indices.len() * 4 + values.len() * 2);
        f.extend_from_slice(&(hid as u32).to_le_bytes());
        f.extend_from_slice(&n.to_le_bytes());
        f.extend_from_slice(&vpp.to_le_bytes());
        for i in indices {
            f.extend_from_slice(&i.to_le_bytes());
        }
        for v in values {
            f.extend_from_slice(&v.to_le_bytes());
        }
        f
    }

    fn post(addr: std::net::SocketAddr, body: &[u8], bearer: Option<&str>) -> String {
        let mut client = TcpStream::connect(addr).expect("connect");
        let mut request = format!("POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: {}\r\n", body.len());
        if let Some(t) = bearer {
            request.push_str(&format!("Authorization: Bearer {t}\r\n"));
        }
        request.push_str("Connection: close\r\n\r\n");
        client.write_all(request.as_bytes()).unwrap();
        client.write_all(body).unwrap();
        let mut response = String::new();
        client.read_to_string(&mut response).unwrap();
        response
    }

    #[test]
    fn validate_frame_accepts_well_formed_and_rejects_mismatched_hid() {
        let f = frame(42, &[0, 1, 2], &[10, 20, 30], 1);
        assert!(validate_frame(42, &f));
        assert!(!validate_frame(43, &f));
    }

    #[test]
    fn validate_frame_rejects_truncated_and_oversized() {
        let f = frame(1, &[0, 1], &[5, 6], 1);
        assert!(!validate_frame(1, &f[..f.len() - 1]));
        let mut too_long = f.clone();
        too_long.push(0);
        assert!(!validate_frame(1, &too_long));
    }

    #[test]
    fn validate_frame_rejects_short_garbage_without_panicking() {
        assert!(!validate_frame(1, &[1, 2, 3]));
        assert!(!validate_frame(1, &[]));
    }

    #[test]
    fn constant_time_eq_behaves_like_normal_equality() {
        assert!(constant_time_eq("abc", "abc"));
        assert!(!constant_time_eq("abc", "abd"));
        assert!(!constant_time_eq("abc", "ab"));
        assert!(!constant_time_eq("", "a"));
        assert!(constant_time_eq("", ""));
    }

    #[test]
    fn real_http_post_is_forwarded_and_recorded() {
        let hid = 4_000_001u64;
        crate::plot::record_registry::stop(hid);
        let path = std::env::temp_dir().join(format!("sp_remote_push_test_{}.spls", std::process::id()));
        let meta = crate::plot::push_registry::PushMeta::Scalar(crate::plot::push_registry::ScalarMeta {
            min_val: 0.0,
            range_val: 100.0,
            axis_px: 100,
        });
        crate::plot::push_registry::register(hid, meta.clone());
        crate::plot::record_registry::start(hid, path.to_str().unwrap(), &meta).expect("start recording");

        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind test listener");
        let addr = listener.local_addr().unwrap();
        let handle = std::thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                handle_connection(hid, stream, None);
            }
        });

        let body = frame(hid, &[7, 8], &[11, 22], 1);
        let response = post(addr, &body, None);
        handle.join().unwrap();

        assert!(response.starts_with("HTTP/1.1 204"), "expected 204, got: {response}");

        let recording = crate::plot::record_registry::read_all(path.to_str().unwrap()).expect("read back recording");
        let _ = std::fs::remove_file(&path);
        assert_eq!(recording.records.len(), 1);
        assert_eq!(recording.records[0].indices, vec![7, 8]);
        assert_eq!(recording.records[0].values, vec![11, 22]);
    }

    #[test]
    fn vpp_mismatched_against_registered_chart_shape_is_rejected() {
        let hid = 4_000_003u64;
        let meta = crate::plot::push_registry::PushMeta::Scalar(crate::plot::push_registry::ScalarMeta {
            min_val: 0.0,
            range_val: 100.0,
            axis_px: 100,
        });
        crate::plot::push_registry::register(hid, meta);

        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind test listener");
        let addr = listener.local_addr().unwrap();
        let handle = std::thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                handle_connection(hid, stream, None);
            }
        });

        let bad_body = frame(hid, &[0, 1], &[1, 2, 3, 4], 2);
        let response = post(addr, &bad_body, None);
        handle.join().unwrap();

        assert!(response.starts_with("HTTP/1.1 400"), "expected 400 for a vpp that doesn't match the registered chart shape, got: {response}");
    }

    #[test]
    fn malformed_request_gets_400_not_a_hang_or_panic() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind test listener");
        let addr = listener.local_addr().unwrap();
        let hid = 4_000_002u64;
        let handle = std::thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                handle_connection(hid, stream, None);
            }
        });

        let mut client = TcpStream::connect(addr).expect("connect");
        client.write_all(b"garbage not http\r\n\r\n").unwrap();
        let mut response = String::new();
        client.read_to_string(&mut response).unwrap();
        handle.join().unwrap();
        assert!(response.starts_with("HTTP/1.1 400"), "expected 400, got: {response}");
    }

    #[test]
    fn missing_or_wrong_token_is_rejected_when_one_is_required() {
        let hid = 4_000_004u64;
        let meta = crate::plot::push_registry::PushMeta::Scalar(crate::plot::push_registry::ScalarMeta {
            min_val: 0.0,
            range_val: 100.0,
            axis_px: 100,
        });
        crate::plot::push_registry::register(hid, meta);
        let body = frame(hid, &[0], &[1], 1);

        for (bearer, label) in [(None, "missing"), (Some("wrong-token"), "wrong")] {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind test listener");
            let addr = listener.local_addr().unwrap();
            let handle = std::thread::spawn(move || {
                if let Ok((stream, _)) = listener.accept() {
                    handle_connection(hid, stream, Some("correct-token"));
                }
            });
            let response = post(addr, &body, bearer);
            handle.join().unwrap();
            assert!(response.starts_with("HTTP/1.1 401"), "expected 401 for {label} token, got: {response}");
        }
    }

    #[test]
    fn correct_token_is_accepted() {
        let hid = 4_000_005u64;
        crate::plot::record_registry::stop(hid);
        let meta = crate::plot::push_registry::PushMeta::Scalar(crate::plot::push_registry::ScalarMeta {
            min_val: 0.0,
            range_val: 100.0,
            axis_px: 100,
        });
        crate::plot::push_registry::register(hid, meta);
        let body = frame(hid, &[0], &[1], 1);

        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind test listener");
        let addr = listener.local_addr().unwrap();
        let handle = std::thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                handle_connection(hid, stream, Some("correct-token"));
            }
        });
        let response = post(addr, &body, Some("correct-token"));
        handle.join().unwrap();
        assert!(response.starts_with("HTTP/1.1 204"), "expected 204 with the correct token, got: {response}");
    }
}
