use std::collections::HashMap;

use base64::Engine;

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
}

pub fn parse_request(raw: &str) -> Option<HttpRequest> {
    let mut lines = raw.split("\r\n");
    let request_line = lines.next()?;
    let mut parts = request_line.split(' ');
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();

    let mut headers = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        }
    }

    Some(HttpRequest { method, path, headers })
}

pub fn html_response(body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    )
}

pub fn not_found_response() -> String {
    let body = "404 Not Found";
    format!(
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    )
}

pub fn not_authorized_response() -> String {
    let body = "401 Unauthorized";
    format!(
        "HTTP/1.1 401 Unauthorized\r\nWWW-Authenticate: Basic realm=\"seraplot\"\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    )
}

pub fn authorized(req: &HttpRequest, credentials: Option<(String, String)>) -> bool {
    let Some((user, pass)) = credentials else { return true };
    let Some(header) = req.headers.get("authorization") else { return false };
    let Some(encoded) = header.strip_prefix("Basic ") else { return false };
    let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(encoded) else { return false };
    let Ok(decoded) = String::from_utf8(decoded) else { return false };
    decoded.split_once(':').is_some_and(|(u, p)| u == user && p == pass)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_request_line_and_headers() {
        let raw = "GET /ws HTTP/1.1\r\nHost: 127.0.0.1\r\nSec-WebSocket-Key: abc123==\r\n\r\n";
        let req = parse_request(raw).unwrap();
        assert_eq!(req.method, "GET");
        assert_eq!(req.path, "/ws");
        assert_eq!(req.headers.get("sec-websocket-key"), Some(&"abc123==".to_string()));
    }

    #[test]
    fn no_credentials_configured_means_always_authorized() {
        let req = parse_request("GET / HTTP/1.1\r\nHost: x\r\n\r\n").unwrap();
        assert!(authorized(&req, None));
    }

    #[test]
    fn correct_basic_auth_header_is_authorized() {
        let encoded = base64::engine::general_purpose::STANDARD.encode("admin:secret");
        let raw = format!("GET / HTTP/1.1\r\nHost: x\r\nAuthorization: Basic {encoded}\r\n\r\n");
        let req = parse_request(&raw).unwrap();
        assert!(authorized(&req, Some(("admin".to_string(), "secret".to_string()))));
    }

    #[test]
    fn missing_or_wrong_basic_auth_header_is_rejected() {
        let req = parse_request("GET / HTTP/1.1\r\nHost: x\r\n\r\n").unwrap();
        assert!(!authorized(&req, Some(("admin".to_string(), "secret".to_string()))));

        let encoded = base64::engine::general_purpose::STANDARD.encode("admin:wrong");
        let raw = format!("GET / HTTP/1.1\r\nHost: x\r\nAuthorization: Basic {encoded}\r\n\r\n");
        let req = parse_request(&raw).unwrap();
        assert!(!authorized(&req, Some(("admin".to_string(), "secret".to_string()))));
    }
}
