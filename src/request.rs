use std::{collections::HashMap, io::{BufRead, BufReader, Read}};

#[derive(Debug)]
pub enum HttpMethod {
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
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub fn from_buffer(buffer: &[u8]) -> Option<Self> {
        let mut reader = BufReader::new(buffer);

        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();

        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() != 3 {
            return None;
        } 

        let method = HttpMethod::from_str(parts[0])?;
        let path = parts[1].to_string();
        let version = parts[2].to_string();

        let mut headers = HashMap::new();

        let mut line = String::new();

        loop {
            line.clear();

            let bytes_read = reader.read_line(&mut line).unwrap();

            if bytes_read == 0 || line.trim().is_empty() {
                break;
            }

            let parts: Vec<&str> = line.split(':')
                                .map(|s| s.trim())
                                .collect();

            headers.insert(parts[0].to_string(), parts[1].to_string());
        }

        let content_length = match headers.get("Content-Length") {
            Some(value) => value.parse::<usize>().unwrap(),
            None => 0
        };

        let mut body= vec![0; content_length];

        reader.read_exact(&mut body).unwrap();

        let body = String::from_utf8(body).unwrap();

        Some(Self { method, path, version, headers, body })
    }
}