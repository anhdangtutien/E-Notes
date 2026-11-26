use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

#[allow(dead_code)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn json<'a, T: serde::Deserialize<'a>>(&'a self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }

    pub fn query_param(&self, key: &str) -> Option<&String> {
        self.query.get(key)
    }
}

pub struct Response {
    pub status: u16,
    pub status_text: &'static str,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: u16, status_text: &'static str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Server".to_string(), "E-Notes-Rust/1.0".to_string());
        headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
        headers.insert(
            "Access-Control-Allow-Methods".to_string(),
            "GET, POST, PUT, DELETE, OPTIONS".to_string(),
        );
        headers.insert(
            "Access-Control-Allow-Headers".to_string(),
            "Content-Type, Authorization".to_string(),
        );
        headers.insert("Connection".to_string(), "close".to_string());
        Self {
            status,
            status_text,
            headers,
            body: Vec::new(),
        }
    }

    pub fn ok() -> Self {
        Self::new(200, "OK")
    }

    #[allow(dead_code)]
    pub fn created() -> Self {
        Self::new(201, "Created")
    }

    pub fn not_found(msg: &str) -> Self {
        let mut res = Self::new(404, "Not Found");
        res.set_body("text/plain; charset=utf-8", msg.as_bytes().to_vec());
        res
    }

    pub fn bad_request(msg: &str) -> Self {
        let mut res = Self::new(400, "Bad Request");
        res.set_body("text/plain; charset=utf-8", msg.as_bytes().to_vec());
        res
    }

    pub fn internal_error(msg: &str) -> Self {
        let mut res = Self::new(500, "Internal Server Error");
        res.set_body("text/plain; charset=utf-8", msg.as_bytes().to_vec());
        res
    }

    pub fn json<T: serde::Serialize>(data: &T) -> Self {
        let mut res = Self::new(200, "OK");
        let body = serde_json::to_vec(data).unwrap_or_default();
        res.set_body("application/json; charset=utf-8", body);
        res
    }

    pub fn set_body(&mut self, content_type: &str, body: Vec<u8>) {
        self.headers
            .insert("Content-Type".to_string(), content_type.to_string());
        self.headers
            .insert("Content-Length".to_string(), body.len().to_string());
        self.body = body;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut output = format!("HTTP/1.1 {} {}\r\n", self.status, self.status_text);
        for (k, v) in &self.headers {
            output.push_str(&format!("{}: {}\r\n", k, v));
        }
        output.push_str("\r\n");
        let mut bytes = output.into_bytes();
        bytes.extend_from_slice(&self.body);
        bytes
    }
}

pub type Handler<T> = Box<dyn Fn(&Request, &T) -> Response + Send + Sync>;

pub struct Server<T: Clone + Send + Sync + 'static> {
    state: T,
    routes: Arc<Vec<(String, String, Handler<T>)>>, // (Method, PathPattern, Handler)
}

impl<T: Clone + Send + Sync + 'static> Server<T> {
    pub fn new(state: T) -> Self {
        Self {
            state,
            routes: Arc::new(Vec::new()),
        }
    }

    pub fn route<F>(mut self, method: &str, pattern: &str, handler: F) -> Self
    where
        F: Fn(&Request, &T) -> Response + Send + Sync + 'static,
    {
        let routes = Arc::get_mut(&mut self.routes).unwrap();
        routes.push((method.to_uppercase(), pattern.to_string(), Box::new(handler)));
        self
    }

    pub fn run_with_fallback(self, preferred_host: &str, preferred_port: u16) -> std::io::Result<()> {
        let mut ports_to_try = vec![preferred_port, 8080, 8000, 3001, 3030, 5000, 8888];
        ports_to_try.dedup();

        let mut bound_listener = None;
        let mut bound_port = preferred_port;

        for port in ports_to_try {
            // Try binding on 127.0.0.1 first (safe, no firewall prompt), then preferred_host
            let candidates = [
                format!("127.0.0.1:{}", port),
                format!("{}:{}", preferred_host, port),
            ];

            for candidate in &candidates {
                if let Ok(listener) = TcpListener::bind(candidate) {
                    bound_listener = Some(listener);
                    bound_port = port;
                    break;
                }
            }

            if bound_listener.is_some() {
                break;
            }
        }

        let listener = match bound_listener {
            Some(l) => l,
            None => {
                eprintln!("✗ Không thể mở bất kỳ cổng mạng nào (8080, 8000, 3001, 3030, 5000).");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::AddrNotAvailable,
                    "No available ports to bind",
                ));
            }
        };

        println!("--------------------------------------------------");
        println!("🚀 E-Notes server đã khởi động THÀNH CÔNG!");
        println!("👉 Truy cập ngay tại: http://localhost:{}", bound_port);
        println!("--------------------------------------------------");

        let routes = self.routes.clone();
        let state = self.state.clone();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let routes = routes.clone();
                    let state = state.clone();
                    thread::spawn(move || {
                        handle_connection(stream, &routes, &state);
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
        Ok(())
    }

    pub fn run(self, addr: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(addr)?;
        println!("🚀 E-Notes server running at http://{}", addr);

        let routes = self.routes.clone();
        let state = self.state.clone();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let routes = routes.clone();
                    let state = state.clone();
                    thread::spawn(move || {
                        handle_connection(stream, &routes, &state);
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
        Ok(())
    }
}

fn handle_connection<T: Clone>(
    mut stream: TcpStream,
    routes: &[(String, String, Handler<T>)],
    state: &T,
) {
    let mut buffer = [0u8; 8192];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let req_data = &buffer[..bytes_read];
    let req_str = String::from_utf8_lossy(req_data);

    let mut lines = req_str.split("\r\n");
    let request_line = match lines.next() {
        Some(l) if !l.is_empty() => l,
        _ => return,
    };

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }

    let method = parts[0].to_uppercase();
    let full_uri = parts[1];

    let (path, query) = parse_uri(full_uri);

    // Headers
    let mut headers = HashMap::new();
    let mut content_length = 0usize;

    for line in lines {
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim().to_lowercase();
            let val = v.trim().to_string();
            if key == "content-length" {
                content_length = val.parse().unwrap_or(0);
            }
            headers.insert(key, val);
        }
    }

    // Body
    let mut body = Vec::new();
    if let Some(pos) = req_data.windows(4).position(|w| w == b"\r\n\r\n") {
        let initial_body = &req_data[pos + 4..];
        body.extend_from_slice(initial_body);

        let needed = content_length.saturating_sub(body.len());
        if needed > 0 {
            let mut remaining = vec![0u8; needed];
            if stream.read_exact(&mut remaining).is_ok() {
                body.extend(remaining);
            }
        }
    }

    let request = Request {
        method: method.clone(),
        path: path.clone(),
        query,
        headers,
        body,
    };

    // Preflight OPTIONS
    let response = if method == "OPTIONS" {
        Response::ok()
    } else {
        match_route(&request, routes, state)
    };

    let _ = stream.write_all(&response.to_bytes());
    let _ = stream.flush();
    let _ = stream.shutdown(std::net::Shutdown::Both);
}

fn parse_uri(uri: &str) -> (String, HashMap<String, String>) {
    let mut query = HashMap::new();
    let (path_str, query_str) = match uri.split_once('?') {
        Some((p, q)) => (p, Some(q)),
        None => (uri, None),
    };

    if let Some(q) = query_str {
        for pair in q.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                let decoded_k = percent_decode(k);
                let decoded_v = percent_decode(v);
                query.insert(decoded_k, decoded_v);
            }
        }
    }

    (path_str.to_string(), query)
}

fn percent_decode(input: &str) -> String {
    let mut result = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&input[i + 1..i + 3], 16) {
                result.push(byte);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            result.push(b' ');
            i += 1;
            continue;
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&result).to_string()
}

fn match_route<T: Clone>(
    req: &Request,
    routes: &[(String, String, Handler<T>)],
    state: &T,
) -> Response {
    for (method, pattern, handler) in routes {
        if *method != req.method {
            continue;
        }

        if path_matches(pattern, &req.path) {
            return handler(req, state);
        }
    }

    Response::not_found(&format!("Route not found: {} {}", req.method, req.path))
}

pub fn path_matches(pattern: &str, path: &str) -> bool {
    let p_segs: Vec<&str> = pattern.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
    let r_segs: Vec<&str> = path.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();

    if p_segs.len() != r_segs.len() {
        return false;
    }

    for (p, r) in p_segs.iter().zip(r_segs.iter()) {
        if p.starts_with('{') && p.ends_with('}') {
            continue; // wildcard param
        }
        if p != r {
            return false;
        }
    }

    true
}

pub fn extract_param(pattern: &str, path: &str, param_name: &str) -> Option<String> {
    let p_segs: Vec<&str> = pattern.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
    let r_segs: Vec<&str> = path.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();

    if p_segs.len() != r_segs.len() {
        return None;
    }

    let target = format!("{{{}}}", param_name);
    for (p, r) in p_segs.iter().zip(r_segs.iter()) {
        if *p == target {
            return Some(r.to_string());
        }
    }
    None
}
