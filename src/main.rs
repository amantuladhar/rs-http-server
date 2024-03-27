use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

const HTTP_LINE_ENDING: &str = "\r\n";

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                // let mut reader = BufReader::new(&_stream);
                // let mut request_str = String::new();
                // // Just read one line for now
                // reader.read_line(&mut request_str).unwrap();
                // let http_path = request_str.split(" ").collect::<Vec<&str>>()[1];
                let res = Request::parse(&_stream);
                let http_path = res.start_line.split(" ").collect::<Vec<_>>()[1];

                let msg = match http_path {
                    "/" => gen_http_response(200),
                    _ if http_path.starts_with("/echo") => {
                        let echo_msg = &http_path[6..];
                        gen_http_response_with_msg(200, echo_msg)
                    }
                    _ if http_path.starts_with("/user-agent") => {
                        let msg = res
                            .headers
                            .get("User-Agent")
                            .unwrap_or(&"".into())
                            .to_string();
                        gen_http_response_with_msg(200, &msg)
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

struct Request {
    start_line: String,
    headers: std::collections::HashMap<String, String>,
}

impl Request {
    fn parse(stream: &TcpStream) -> Self {
        let mut start_line = String::new();
        let mut headers = std::collections::HashMap::<String, String>::new();

        let mut reader = BufReader::new(stream);
        if let Err(err) = reader.read_line(&mut start_line) {
            println!(
                "Error occurred while reading start line of the request: {}",
                err
            );
        }

        loop {
            let mut cur_header = String::new();
            if let Err(err) = reader.read_line(&mut cur_header) {
                println!("Error occurred while reading header: {}", err);
                break;
            }
            if cur_header == HTTP_LINE_ENDING {
                // If cur_header is empty, it means we've reached the end of headers
                break;
            }
            let (key, value) = Self::parse_header(&cur_header);
            headers.insert(key.to_string(), value.to_string());
        }

        Self {
            start_line,
            headers,
        }
    }
    fn parse_header(header: &str) -> (&str, &str) {
        let (key, value) = header.split_once(": ").unwrap();
        (key, &value[..value.len() - HTTP_LINE_ENDING.len()])
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
    msg_lines.join(HTTP_LINE_ENDING)
}

fn get_status_line<'a>(status: u16) -> &'a str {
    match status {
        200 => "200 OK",
        404 => "404 Not Found",
        _ => "500 Internal Server Error",
    }
}
