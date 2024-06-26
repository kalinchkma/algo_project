use std::fs;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use crate::multi_threading::*;


pub fn run() {
    const PORT: i32 = 7878;

    let listener = TcpListener::bind(format!("127.0.0.1:{}",PORT)).unwrap();

    println!("Listening to: port {}", PORT);

    // creating threadpool
    let pool = ThreadPool::new(4);

    // process the request
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
   
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // printing the request
    println!(
      "Request: {}",
      String::from_utf8_lossy(&buffer[..])
    );

    // simple routing path
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";


    // simple routing handler
    let (status_line, filename) = if buffer.starts_with(get) {
       ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_millis(10000));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
      ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // reading file to send0
    let contents = fs::read_to_string(filename).unwrap();

    // constructing message
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    println!("Response:\n {}", response);
    // 
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
 
}