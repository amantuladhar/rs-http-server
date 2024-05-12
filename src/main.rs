#![warn(clippy::all)]

use std::collections::HashMap;

use http::request::Request;
use tracing::info;

use crate::{
    http::{response::Response, server::HttpServer},
    utils::setup::setup,
};
mod http;
mod utils;

const HTTP_LINE_ENDING: &str = "\r\n";

#[tokio::main]
async fn main() {
    setup();
    info!("Logs from your program will appear here!");
    // let cmd_args = parse_cmd_args();
    HttpServer::builder()
        .get("/", root)
        .get("/echo/:message", echo_route)
        .start()
        .await
        .expect("unable to start server");
}

fn root(_: Request) -> Response {
    Response::builder().build()
}

fn echo_route(request: Request) -> Response {
    let mut res_builder = Response::builder();
    if let Some(msg) = request.params.get("message") {
        res_builder = res_builder.body(msg.as_bytes().to_vec());
    }
    res_builder.build()
}
/*
fn handle_incoming_request(mut stream: TcpStream, cmd_args: HashMap<String, String>) {
    use http::method::Method::*;
    let res = Request::parse(&stream);
    let msg = match res.method {
        Get => match res.path.as_str() {
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
        Post => match res.path.as_str() {
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
*/

fn parse_cmd_args() -> HashMap<String, String> {
    let arg_vec = std::env::args().collect::<Vec<String>>();
    let params = arg_vec[1..]
        .chunks(2)
        .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
        .collect::<HashMap<_, _>>();
    params
}
