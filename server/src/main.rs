use std::io::prelude::*;
use std::fs;
use std::net::{TcpListener, TcpStream};
mod thread_pool;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(tcp_listener) => tcp_listener,
        Err(_) => {println!("Didn't manage to bind ip address."); return; }
    };
    let pool = thread_pool::ThreadPool::new(10);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
}


fn get_path(buffer: [u8; 1024]) -> String {
    let raw = String::from_utf8_lossy(&buffer[..]);

    let mut path = raw.split(' ');
    let _ = path.next();
    String::from(path.next().unwrap())
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let contents = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        fs::read("../frontend/index.html").unwrap()
    } else {
        let path = get_path(buffer);
        match fs::read(format!("../frontend{}", path)) {
            Ok(res) => {res},
            Err(err) => {println!("Error: {}\n{}", path, err); Vec::new()},
        }
    };
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
        contents.len()
    );
    
    stream.write(response.as_bytes()).unwrap();
    stream.write(contents.as_slice()).unwrap();
    stream.flush().unwrap();
}