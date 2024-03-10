use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

#[derive(Debug)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl HttpMethod {
    fn from_str(method: &str) -> Option<HttpMethod> {
        match method {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "DELETE" => Some(HttpMethod::DELETE),
            "PATCH" => Some(HttpMethod::PATCH),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct HttpRequest {
    method: HttpMethod,
    path: String,
    version: String
}

impl HttpRequest {
    fn from_str(request: &str) -> Option<Self> {
        let mut lines = request.lines();
        let start_line = lines.next()?;
        let parts: Vec<&str> = start_line.split_whitespace().collect();
        if parts.len() == 3 {
            let method = HttpMethod::from_str(parts[0]).unwrap();
            let path = parts[1].to_string();
            let version = parts[2].to_string();
            Some(Self { method, path, version})
        } else {
            None
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let received = String::from_utf8_lossy(&buffer);

    let request = HttpRequest::from_str(&received).unwrap();

    let response = match request.path.as_str() {
        "/" => "HTTP/1.1 200 OK\r\n\r\n",
        _ => "HTTP/1.1 404 Not Found\r\n\r\n"
    };
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
