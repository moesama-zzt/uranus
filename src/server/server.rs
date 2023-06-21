use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use crate::router::Router;

pub struct Server {
    router: Router,
    port: u16,
}

impl Server {
    pub fn new(router: Router, port: u16) -> Server {
        Server {
            router: router,
            port: port,
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        println!("Server started on port {}", self.port);

        for stream in listener.incoming() {
            let mut buffer = [0; 1024];
            let mut stream = stream.unwrap();

            stream.read(&mut buffer).unwrap();

            let request = String::from_utf8_lossy(&buffer[..]);
            let request_line = request.lines().next().unwrap();
            let path = request_line.split_whitespace().nth(1).unwrap();

            let response = match self.router.handle_request(path) {
                Some(content) => format!("HTTP/1.1 200 OK\r\n\r\n{}", content),
                None => "HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found".to_string(),
            };

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}