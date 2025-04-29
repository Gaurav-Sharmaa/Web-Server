use std::fs;                 // Used to read files like index.html or 404.html
use std::io::prelude::*;     // For reading and writing to the TCP stream
use std::net::TcpListener;   // Listens for incoming TCP connections
use std::net::TcpStream;     // Represents a single TCP connection (client)


fn main() {
    //  Rusty sets up a listener at localhost:7878 and waits for connections
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //  Rusty listens in an infinite loop — each client connection becomes a stream
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream); //  Pass each connection to the handler
    }
}

fn handle_connection(mut stream: TcpStream) {

    //  Rusty prepares a 1024-byte buffer to hold the incoming HTTP request
    let mut buffer = [0; 1024];

    //  Read the request from the client (Charlie) into the buffer
    stream.read(&mut buffer).unwrap();

    //  Define the pattern for a valid GET request to "/"
    let get = b"GET / HTTP/1.1\r\n";


    //  Rusty checks: "Did Charlie ask for the home page (`/`)?"
    let (status_line, filename) = 
    if buffer.starts_with(get) {

        //  Yes — serve the home page
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        //  No — serve the custom 404 error page
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    //  Read the contents of the appropriate file (either index or 404 page)
    let contents = fs::read_to_string(filename).unwrap();

    //  Build the HTTP response with status + headers + actual HTML content
    let response = format!(
        "{}\r\nContent-Lenght: {}\r\n\r\n{}",
        status_line,       // Example: HTTP/1.1 200 OK
        contents.len(),    // Tells Charlie how many bytes are in the body
        contents           // The actual content of index.html or 404.html
    );

    //  Send the response to the browser
    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}

