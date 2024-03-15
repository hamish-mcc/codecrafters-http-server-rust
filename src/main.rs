use std::{collections::HashMap, io::{BufRead, BufReader, Read, Result, Write}, net::{TcpListener, TcpStream}, str};

use itertools::Itertools;

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
    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        let mut buf_reader = BufReader::new(buffer);
        let mut request_line = String::new();

        buf_reader.read_line(&mut request_line).expect("reading request line");

        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() == 3 {
            let method = HttpMethod::from_str(parts[0])?;
            let path = parts[1].to_string();
            let version = parts[2].to_string();

            Some(Self { method, path, version })
        } else {
            None
        }
    }
}

#[derive(Debug)]

enum HttpStatusCode {
    Ok = 200,
    NotFound = 404
}

impl HttpStatusCode {
    fn status_text(&self) -> &'static str {
        match self {
            HttpStatusCode::Ok => "OK",
            HttpStatusCode::NotFound => "Not Found",
        }
    }
}

#[derive(Debug)]
struct HttpResponse {
    http_version: String,
    status_code: HttpStatusCode,    
    headers: HashMap<String, String>,
    body: String
}

impl HttpResponse {
    fn new(status_code:HttpStatusCode, headers: HashMap<String, String>, body: &str)-> Self {
        let http_version = "HTTP/1.1".to_string();
        let body = body.to_string();

        Self {http_version, status_code, headers, body}
    }

    fn to_string(self) -> String {
        let status_text = self.status_code.status_text();
        let mut response = format!("{} {} {}\r\n", self.http_version, self.status_code as u16, status_text);

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");
        response.push_str(&self.body);

        response
    }
}

fn handle_connection(mut stream: TcpStream)-> Result<()>  {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    if let Some(request) = HttpRequest::from_buffer(&buffer) {
        let mut path_segments = request.path.split("/");
   
        let response = match path_segments.nth(1) {
            Some("") => HttpResponse::new(HttpStatusCode::Ok, HashMap::new(), ""),
            Some("echo") => {
                // TODO: Expects the rest of the path as content
                let content = path_segments.join("/");
                let mut headers = HashMap::new();
                
                headers.insert(String::from("Content-Type"), String::from("text/plain"));
                headers.insert(String::from("Content-Length"), String::from(content.len().to_string()));

                HttpResponse::new(HttpStatusCode::Ok, headers, &content)
            },
            None => panic!(),
            _ =>  HttpResponse::new(HttpStatusCode::NotFound, HashMap::new(), ""),
        };

        let response_string = response.to_string();

        stream.write(response_string.as_bytes())?;
        stream.flush()?;
    }
    
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream).expect("error handling connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
