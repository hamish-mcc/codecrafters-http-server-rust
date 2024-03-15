use std::collections::HashMap;

#[derive(Debug)]

pub enum HttpStatus {
    Ok = 200,
    NotFound = 404
}

impl HttpStatus {
    fn status_text(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "Not Found",
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    http_version: String,
    status_code: HttpStatus,    
    headers: HashMap<String, String>,
    body: String
}

impl HttpResponse {
    pub fn new(status_code:HttpStatus, headers: HashMap<String, String>, body: &str)-> Self {
        let http_version = "HTTP/1.1".to_string();
        let body = body.to_string();

        Self {http_version, status_code, headers, body}
    }

    pub fn to_string(self) -> String {
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