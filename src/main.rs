use std::{collections::HashMap, io::{Read, Result, Write}, net::{TcpListener, TcpStream}};

use http_server_starter_rust::{pool::ThreadPool, request::HttpRequest, response::{HttpResponse, HttpStatus}};
use itertools::Itertools;

fn handle_connection(mut stream: TcpStream)-> Result<()>  {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    if let Some(request) = HttpRequest::from_buffer(&buffer) {
        let mut path_segments = request.path.split("/");
   
        let response = match path_segments.nth(1) {
            Some("") => HttpResponse::new(HttpStatus::Ok, HashMap::new(), ""),
            Some("echo") => {
                let content = path_segments.join("/");
                let mut headers = HashMap::new();
                
                headers.insert(String::from("Content-Type"), String::from("text/plain"));
                headers.insert(String::from("Content-Length"), String::from(content.len().to_string()));

                HttpResponse::new(HttpStatus::Ok, headers, &content)
            },
            Some("user-agent") => {
                let content = match request.headers.get("User-Agent") {
                    Some(value) => value,
                    None => panic!()
                };

                let mut headers = HashMap::new();
                
                headers.insert(String::from("Content-Type"), String::from("text/plain"));
                headers.insert(String::from("Content-Length"), String::from(content.len().to_string()));

                HttpResponse::new(HttpStatus::Ok, headers, content)
            },
            _ =>  HttpResponse::new(HttpStatus::NotFound, HashMap::new(), ""),
        };

        let response_string = response.to_string();

        stream.write(response_string.as_bytes())?;
        stream.flush()?;
    }
    
    Ok(())
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream).unwrap();
        });
    }
}
