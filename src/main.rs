use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

const HTTP_LINE_ENDING: &str = "\r\n";

fn main() {
    println!("Logs from your program will appear here!");
    let cmd_args = parse_cmd_args();
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let cloned_args = cmd_args.clone();
                std::thread::spawn(move || {
                    handle_incoming_request(_stream, cloned_args);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

enum StatusCode {
    Ok,
    NotFound,
    #[allow(dead_code)]
    InternalServerError,
}
impl StatusCode {
    fn status_line<'a>(&self) -> &'a str {
        match self {
            StatusCode::Ok => "200 OK",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::InternalServerError => "500 Internal Server Error",
        }
    }
}

enum ContentType {
    Plain,
    OctetStream,
}
impl ContentType {
    fn to_str(&self) -> &str {
        match self {
            ContentType::Plain => "text/plain",
            ContentType::OctetStream => "application/octet-stream",
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
            if cur_header == HTTP_LINE_ENDING || cur_header == "\n" || cur_header.is_empty() {
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
        // println!("parse_header: {}", header);
        let (key, value) = header.split_once(": ").unwrap();
        (key, &value[..value.len() - HTTP_LINE_ENDING.len()])
    }
}

fn handle_incoming_request(mut stream: TcpStream, cmd_args: HashMap<String, String>) {
    let res = Request::parse(&stream);
    let http_path = res.start_line.split(" ").collect::<Vec<_>>()[1];
    let msg = match http_path {
        "/" => gen_http_response(StatusCode::Ok),
        _ if http_path.starts_with("/echo") => {
            let echo_msg = &http_path[6..];
            gen_http_response_with_msg(StatusCode::Ok, ContentType::Plain, echo_msg)
        }
        _ if http_path.starts_with("/user-agent") => {
            let msg = res
                .headers
                .get("User-Agent")
                .unwrap_or(&"".into())
                .to_string();
            gen_http_response_with_msg(StatusCode::Ok, ContentType::Plain, &msg)
        }
        _ if http_path.starts_with("/files") => {
            let file_name = &http_path[7..];
            match cmd_args.get("--directory") {
                None => gen_http_response_with_msg(
                    StatusCode::NotFound,
                    ContentType::Plain,
                    "path doesn't have file name",
                ),
                Some(dir_name) => {
                    match std::fs::read_to_string(format!("{}/{}", dir_name, file_name)) {
                        Err(_err) => gen_http_response_with_msg(
                            StatusCode::NotFound,
                            ContentType::Plain,
                            "file not found",
                        ),
                        Ok(file_content) => gen_http_response_with_msg(
                            StatusCode::Ok,
                            ContentType::OctetStream,
                            &file_content,
                        ),
                    }
                }
            }
        }
        _ => gen_http_response(StatusCode::NotFound),
    };
    if let Err(err) = stream.write(msg.as_bytes()) {
        println!("Error occurred while sending data: {}", err);
    }
}

fn gen_http_response(status: StatusCode) -> String {
    gen_http_response_with_msg(status, ContentType::Plain, "")
}
fn gen_http_response_with_msg(status: StatusCode, content_type: ContentType, msg: &str) -> String {
    let mut msg_lines = vec![];
    msg_lines.push(format!("HTTP/1.1 {}", status.status_line()));
    msg_lines.push(format!("Content-Type: {}", content_type.to_str()));
    msg_lines.push(format!("Content-Length: {}", msg.len()));

    msg_lines.push("".to_string());
    msg_lines.push(msg.to_string());
    msg_lines.join(HTTP_LINE_ENDING)
}

fn parse_cmd_args() -> HashMap<String, String> {
    let arg_vec = std::env::args().collect::<Vec<String>>();
    let params = arg_vec[1..]
        .chunks(2)
        .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
        .collect::<HashMap<_, _>>();
    params
}
