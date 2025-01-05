/*
    Some terms:
    TCP: Transmission Control Protocol
    HTTP: Hyptrtext Transfer Protocol

    TCP manages the data stream, whereas HTTP describes what the data in the stream contains.
    i.e
    TCP contains information about what data has or has not been received yet,
    while HTTP contains specific instructions on how to read and
    process the data once it's received. 
*/



use std::fs;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

// use threadpool::ThreadPool;
use server::ThreadPool; // using the our ThreadPool Implementation



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);


    for stream in listener.incoming().take(2) { // iterator over the incoming connections
        let stream = stream.unwrap();
        println!("Connection Established!");


        /* handle_connection(stream); */
        /*  WILL EASILY EXHAUST THE NUMBER OF THREADS!
            thread::spawn(|| {
                handle_connection(stream);
            });
        */
        pool.execute(|| {
            handle_connection(stream);
        });
    }

}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = 
        if buffer.starts_with(get) {
            thread::sleep(Duration::from_secs(1));
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();  

    println!( 
        "Request: {}", 
        String::from_utf8_lossy(&buffer[..])
    )

    // HTTP-Version Status-Code Reason-Phrase CRLF(Character return Line Feed Sequence)
    // headers CRLF 
    // message-body
    //
    // ex: HTTP/1.1 200 OK\r\n\r\n

}