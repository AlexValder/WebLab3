#![feature(with_options)]
use std::fs;
use std::net::{TcpListener, TcpStream};
mod thread_pool;
use std::io::prelude::*;


fn main() {
    let listener = match TcpListener::bind("192.168.0.104:7878") {
        Ok(tcp_listener) => tcp_listener,
        Err(_) => {println!("Didn't manage to bind ip address."); return; }
    };
    
    let pool = thread_pool::ThreadPool::new(10);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}


fn log(data: &[u8]) {
    let mut file = fs::File::with_options().create(true)
                                                 .append(true)
                                                 .write(true)
                                                 .open("server.log")
                                                 .unwrap();
    let data = String::from_utf8_lossy(data);
    let data = data.trim_matches(char::from(0)).as_bytes();
    match file.write(data) {
        Ok(_) => {file.write(b"\n").unwrap();},
        Err(err) => println!("File writing error: {}", err),
    }
}


fn process(data: String) {
    let splitted: Vec<_> = data.split("&").collect();
    for s in splitted {
        log(&s.as_bytes());
    }
    log(b"\n");
}


fn handle_get_request(buffer: &[u8]) -> Vec<u8> {
    let raw = String::from_utf8_lossy(&buffer[4..]);

    let mut path = raw.split(' ');

    let mut path = path.next().unwrap();
    if path == "/" {
        path = &"/index.html";
    }
    let mut path = String::from(path);
    if path.contains("?") {
        process(path);
        path = String::from("/response.html");
    };
    let content = match fs::read(format!("../frontend{}", path)) {
        Ok(val) => val,
        Err(err) => {println!("{}", err); Vec::new()},
    };
    content
}


fn handle_post_request(buffer: &[u8]) -> Vec<u8> {
    let buffer = String::from_utf8_lossy(&buffer);
    let buffer: Vec<_> = buffer.split("\r\n").collect();
    process(String::from(buffer[buffer.len()-1]));
    let content = match fs::read("../frontend/response.html") {
        Ok(val) => val,
        Err(err) => {println!("{}", err); Vec::new()},
    };
    content
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents: Vec<u8> = if buffer.starts_with(b"GET") {
        handle_get_request(&buffer)
    } else if buffer.starts_with(b"POST") {
        handle_post_request(&buffer)
    } else {
       Vec::from(String::from("404"))
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
        contents.len()
    );
    
    stream.write(response.as_bytes()).unwrap();
    stream.write(contents.as_slice()).unwrap();
    stream.flush().unwrap();
}
