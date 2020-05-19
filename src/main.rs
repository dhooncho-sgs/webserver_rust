use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut call_count = 0;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &mut call_count);
    }
}

fn handle_connection(mut stream: TcpStream, call_count: &mut i32) {
    *call_count += 1;

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n", "404.html")
    };

    let mut contents = fs::read_to_string(filename).unwrap();
    contents = String::from(contents.trim_end());

    let header = format!("{}{}{}{}{}{}\r\n"
        ,"Cache-Control: no-store, no-cache, must-revalidate\r\n"
        ,"Content-Type: text/html; charset=utf-8\r\n"
        ,"Keep-Alive: timeout=15, max=100\r\n"
        ,"Connection: Keep-Alive\r\n"
        ,"Content-Length: "
        ,contents.len()
    );

    let response = format!("{}{}\r\n{}", status_line, header, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

