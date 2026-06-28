use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Hello, world!");

    // Create TCP Server
    // Redis Port - 6379
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    // Incoming Request Process in stream
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_listener(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_listener(stream: &mut TcpStream) {
    // create a buffer
    let mut buffer = [0; 512];

    // Read from the stream into buffer
    stream.read(&mut buffer).unwrap();

    println!("Received: {:?}", String::from_utf8_lossy(&buffer));

    // hard coded respone
    let response = "+PONG\r\n";

    // write into buffer
    stream.write(response.as_bytes()).unwrap();

    // flush the buffer
    stream.flush().unwrap();
}
