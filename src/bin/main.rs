use std::fs; // Used to read files like index.html or 404.html
use std::io::prelude::*; // For reading and writing to the TCP stream
use std::net::TcpListener; // Listens for incoming TCP connections
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use server::ThreadPool; // Represents a single TCP connection (client)

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //  Rusty sets up a listener at localhost:7878 and waits for connections

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        //  Rusty listens in an infinite loop — each client connection becomes a stream
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; //  Rusty prepares a 1024-byte buffer to hold the incoming HTTP request

    stream.read(&mut buffer).unwrap(); //  Read the request from the client (Charlie) into the buffer

    let get = b"GET / HTTP/1.1\r\n"; //  Define the pattern for a valid GET request to "/"
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = //  Rusty checks: "Did Charlie ask for the home page (`/`)?"
    if buffer.starts_with(get) {

        ("HTTP/1.1 200 OK", "index.html")// Yes — serve the home page
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")//  No — serve the custom 404 error page
    };

    let contents = fs::read_to_string(filename).unwrap(); //  Read the contents of the appropriate file (either index or 404 page)

    let response = format!(
        //  Build the HTTP response with status + headers + actual HTML content
        "{}\r\nContent-Lenght: {}\r\n\r\n{}",
        status_line,    // Example: HTTP/1.1 200 OK
        contents.len(), // Tells Charlie how many bytes are in the body
        contents        // The actual content of index.html or 404.html
    );

    stream.write(response.as_bytes()).unwrap(); //  Send the response to the browser

    stream.flush().unwrap();
}
