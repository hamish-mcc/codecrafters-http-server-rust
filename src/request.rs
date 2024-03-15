use std::{collections::HashMap, io::{BufRead, BufReader}};

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
    pub headers: HashMap<String, String>
}

impl HttpRequest {
    pub fn from_buffer(buffer: &[u8]) -> Option<Self> {
        let reader = BufReader::new(buffer);
        let mut lines = reader.lines();

        let request_line = match lines.next() {
            Some(line) => line.unwrap(),
            None => panic!()
        };

        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() != 3 {
            return None;
        } 

        let method = HttpMethod::from_str(parts[0])?;
        let path = parts[1].to_string();
        let version = parts[2].to_string();

        let mut headers = HashMap::new();

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }

            let parts: Vec<&str> = line.split(':')
                                .map(|s| s.trim())
                                .collect();

            headers.insert(parts[0].to_string(), parts[1].to_string());
        }

        Some(Self { method, path, version, headers })
    }
}