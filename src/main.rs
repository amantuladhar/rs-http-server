// Uncomment this block to pass the first stage
use std::{io::Write, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let msg = gen_http_response(200);
                _stream.write(msg.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn gen_http_response(status: u16) -> String {
    let status_line = get_status_line(status);
    format!("HTTP/1.1 {status_line}\r\n\r\n")
}

fn get_status_line<'a>(status: u16) -> &'a str {
    match status {
        200 => "200 OK",
        404 => "404 Not Found",
        _ => "500 Internal Server Error",
    }
}
