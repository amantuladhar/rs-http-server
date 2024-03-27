use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut reader = BufReader::new(&_stream);
                let mut request_str = String::new();
                // Just read one line for now
                reader.read_line(&mut request_str).unwrap();
                let http_path = request_str.split(" ").collect::<Vec<&str>>()[1];

                let msg = match http_path {
                    "/" => gen_http_response(200),
                    _ if http_path.starts_with("/echo") => {
                        let _ = 1;
                        let echo_msg = &http_path[6..];
                        gen_http_response_with_msg(200, echo_msg)
                    }
                    _ => gen_http_response(404),
                };
                if let Err(err) = _stream.write(msg.as_bytes()) {
                    println!("Error occurred while sending data: {}", err);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn gen_http_response(status: u16) -> String {
    gen_http_response_with_msg(status, "")
}
fn gen_http_response_with_msg(status: u16, msg: &str) -> String {
    let status_line = get_status_line(status);

    let mut msg_lines = vec![];
    msg_lines.push(format!("HTTP/1.1 {status_line}"));
    msg_lines.push("Content-Type: text/plain".to_string());
    msg_lines.push(format!("Content-Length: {}", msg.len()));

    msg_lines.push("".to_string());
    msg_lines.push(msg.to_string());
    msg_lines.join("\r\n")
}

fn get_status_line<'a>(status: u16) -> &'a str {
    match status {
        200 => "200 OK",
        404 => "404 Not Found",
        _ => "500 Internal Server Error",
    }
}
