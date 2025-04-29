use std::net::TcpListener;
use std::net::TcpStream; //Reading data from TCP stream
use std::io::prelude::*;

fn main() {
    let listener = 
        TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream); 
        }
}


fn handle_connection(mut stream: TcpStream) {
    // Create a buffer to store the incoming request bytes (1 KB buffer)
    let mut buffer = [0; 1024];

    // Read the data from the stream into the buffer
    stream.read(&mut buffer).unwrap();

    // Convert the bytes into a readable string and print the request
    println!(
        "Request: {}",
        String::from_utf8_lossy(&buffer[..])
    )
}