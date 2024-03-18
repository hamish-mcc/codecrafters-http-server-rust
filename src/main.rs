use std::{collections::HashMap, fs, io::{Read, Result, Write}, net::{TcpListener, TcpStream}, path::Path};

use http_server_starter_rust::{pool::ThreadPool, request::HttpRequest, response::{HttpResponse, HttpStatus}};
use itertools::Itertools;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    directory: String
}

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
                let content = request.headers.get("User-Agent").unwrap();

                let mut headers = HashMap::new();
                headers.insert(String::from("Content-Type"), String::from("text/plain"));
                headers.insert(String::from("Content-Length"), String::from(content.len().to_string()));

                HttpResponse::new(HttpStatus::Ok, headers, content)
            },
            Some("files") => {
                let args = Args::parse();
                let dir_name = args.directory;

                let file_name = path_segments.next().unwrap();

                let file_path = Path::new(&dir_name).join(&file_name);

                if file_path.exists() {
                    let content = fs::read_to_string(&file_path).unwrap();

                    let mut headers = HashMap::new();
                    headers.insert(String::from("Content-Type"), String::from("application/octet-stream"));
                    headers.insert(String::from("Content-Length"), String::from(content.len().to_string()));

                    HttpResponse::new(HttpStatus::Ok, headers, &content)
                } else {
                    HttpResponse::new(HttpStatus::NotFound, HashMap::new(), "")
                }
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
