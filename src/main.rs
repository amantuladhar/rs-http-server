use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
};

use tracing::info;

use crate::utils::setup::setup;
mod utils;

const HTTP_LINE_ENDING: &str = "\r\n";

fn main() {
    setup();

    info!("Logs from your program will appear here!");
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
    Created,
    NotFound,
    InternalServerError,
}
impl StatusCode {
    fn status_line<'a>(&self) -> &'a str {
        match self {
            StatusCode::Ok => "200 OK",
            StatusCode::Created => "201 Created",
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

enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl FromStr for HttpMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "Delete" => Ok(HttpMethod::Delete),
            _ => Err(()),
        }
    }
}

struct Request {
    method: HttpMethod,
    path: String,
    #[allow(dead_code)]
    http_version: String,
    headers: std::collections::HashMap<String, String>,
    body: Vec<u8>,
}

impl Request {
    fn parse(stream: &TcpStream) -> Self {
        let mut reader = BufReader::new(stream);
        let (method, path, http_version) = Request::parse_start_line(&mut reader);
        let headers = Self::parse_headers(&mut reader);
        let body = Self::parse_body(&mut reader, &headers);

        Self {
            method,
            path,
            http_version,
            headers,
            body,
        }
    }

    fn parse_body(
        reader: &mut BufReader<&TcpStream>,
        headers: &HashMap<String, String>,
    ) -> Vec<u8> {
        let length = headers
            .get("Content-Length")
            .map_or("0", |v| v.as_str())
            .parse::<usize>()
            .unwrap();
        let mut body = vec![0; length];
        reader.read_exact(&mut body).unwrap();
        return body;
    }

    fn parse_headers(reader: &mut BufReader<&TcpStream>) -> HashMap<String, String> {
        let mut headers = std::collections::HashMap::<String, String>::new();
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
        return headers;
    }
    fn parse_header(header: &str) -> (&str, &str) {
        // println!("parse_header: {}", header);
        let (key, value) = header.split_once(": ").unwrap();
        (key, &value[..value.len() - HTTP_LINE_ENDING.len()])
    }
    fn parse_start_line(reader: &mut BufReader<&TcpStream>) -> (HttpMethod, String, String) {
        // FIXME: There is a issue where this status_line is empty when using wrk
        let mut start_line = String::new();
        if let Err(err) = reader.read_line(&mut start_line) {
            println!("Unable to read start line of the request: {}", err);
        }
        let start_line = start_line[..(start_line.len() - HTTP_LINE_ENDING.len())]
            .split(" ")
            .collect::<Vec<_>>();
        (
            HttpMethod::from_str(start_line[0]).unwrap(),
            start_line[1].to_string(),
            start_line[2].to_string(),
        )
    }
}

fn handle_incoming_request(mut stream: TcpStream, cmd_args: HashMap<String, String>) {
    let res = Request::parse(&stream);
    let msg = match res.method {
        HttpMethod::Get => match res.path.as_str() {
            "/" => gen_http_response(StatusCode::Ok),
            _ if res.path.starts_with("/echo") => {
                let echo_msg = &res.path[6..];
                gen_http_response_with_msg(StatusCode::Ok, ContentType::Plain, echo_msg)
            }
            _ if res.path.starts_with("/user-agent") => {
                let msg = res
                    .headers
                    .get("User-Agent")
                    .unwrap_or(&"".into())
                    .to_string();
                gen_http_response_with_msg(StatusCode::Ok, ContentType::Plain, &msg)
            }
            _ if res.path.starts_with("/files") => {
                let file_name = &res.path[7..];
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
        },
        HttpMethod::Post => match res.path.as_str() {
            _ if res.path.starts_with("/files") => match cmd_args.get("--directory") {
                None => gen_http_response(StatusCode::InternalServerError),
                Some(dir_name) => {
                    let file_name = &res.path[7..];
                    std::fs::write(format!("{}/{}", dir_name, file_name), &res.body).unwrap();
                    gen_http_response(StatusCode::Created)
                }
            },
            _ => gen_http_response(StatusCode::NotFound),
        },
        _ => gen_http_response(StatusCode::InternalServerError),
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
