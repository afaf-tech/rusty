use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();


    // The reason we might receive errors from the incoming method when a client connects to the server is
    // that we’re not actually iterating over connections. 
    //Instead, we’re iterating over connection attempts.
    for stream in listener.incoming() {
        let stream = stream.unwrap();   //unwrap to terminate our program if the stream has any errors

        handle_connection_v2(stream);
    }
    println!("Hello, world!");
}


fn handle_connection_v1(mut stream: TcpStream){
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    
        let contents = fs::read_to_string("public/html/hello.html").unwrap();
    
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        //flush will wait and prevent the program from continuing until all the bytes are written to the connection
    }else{
        // other request
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("public/html/404.html").unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

}

fn handle_connection_v2(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    // like define router in any other framework
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "public/html/hello.html")
    } else if buffer.starts_with(sleep) {
        // when we try to access our server the time we execute this. then the server will wait until this process is finished
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "public/html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/html/404.html")
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
}