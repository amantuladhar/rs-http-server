#![warn(clippy::all)]
#![warn(opaque_hidden_inferred_bound)]

use std::{collections::HashMap, str::FromStr, sync::OnceLock};

use http::{
    content_type::ContentType, encoding::Encoding, header::Header, request::Request,
    status_code::StatusCode,
};
use tracing::info;

use crate::{
    http::{response::Response, server::HttpServer},
    utils::setup::setup,
};
mod http;
mod utils;

const HTTP_LINE_ENDING: &str = "\r\n";

pub static ARGS: OnceLock<HashMap<String, String>> = OnceLock::new();

#[tokio::main]
async fn main() {
    setup();
    info!("Logs from your program will appear here!");
    parse_cmd_args();
    HttpServer::builder()
        .get("/", root)
        .get("/echo/:message", echo_route)
        .get("/user-agent", user_agent)
        .get("/files/:file_name", file_route)
        .post("/files/:file_name", file_route_post)
        .start()
        .await
        .expect("unable to start server");
}

fn root(_: Request) -> Response {
    Response::builder().build()
}

fn echo_route(request: Request) -> Response {
    let body = request
        .params
        .get("message")
        .map(|msg| msg.as_bytes().to_vec());

    // TODO(Aman): Can this be put when we parse request??
    let accept_encoding = request
        .headers
        .get(Header::AcceptEncoding.to_str())
        .and_then(|value| {
            value
                .split(",")
                .find_map(|encoding| match Encoding::from_str(encoding.trim()) {
                    Ok(accept_encoding) => Some(accept_encoding),
                    Err(_) => None,
                })
        });
    Response::builder()
        .body(body)
        .accept_encoding(accept_encoding)
        .build()
}
fn user_agent(req: Request) -> Response {
    let body = req
        .headers
        .get(Header::UserAgent.to_str())
        .map(|msg| msg.as_bytes().to_vec());
    Response::builder().body(body).build()
}
fn file_route(req: Request) -> Response {
    let mut res_builder = Response::builder();
    let file_name = req
        .params
        .get("file_name")
        .expect("file_name should be available");

    match ARGS
        .get()
        .expect("ARGS should already be set")
        .get("--directory")
    {
        None => {
            res_builder = res_builder.status_code(StatusCode::NotFound);
        }
        Some(dir_name) => match std::fs::read_to_string(format!("{}/{}", dir_name, file_name)) {
            Err(_) => {
                res_builder = res_builder.status_code(StatusCode::NotFound);
            }
            Ok(file_content) => {
                res_builder = res_builder
                    .status_code(StatusCode::Ok)
                    .content_type(ContentType::OctetStream)
                    .body(Some(file_content.as_bytes().to_vec()))
            }
        },
    }
    res_builder.build()
}
fn file_route_post(req: Request) -> Response {
    let mut res_builder = Response::builder();
    let file_name = req
        .params
        .get("file_name")
        .expect("file_name should be available");

    match ARGS
        .get()
        .expect("ARGS should already be set")
        .get("--directory")
    {
        None => res_builder = res_builder.status_code(StatusCode::InternalServerError),
        Some(dir_name) => {
            tracing::debug!(
                "Writing file to: {}/{}, body: {:?}",
                dir_name,
                file_name,
                &req.body
            );
            std::fs::write(format!("{}/{}", dir_name, file_name), &req.body).unwrap();
            res_builder = res_builder.status_code(StatusCode::Created);
        }
    }
    res_builder.build()
}

fn parse_cmd_args() {
    let arg_vec = std::env::args().collect::<Vec<String>>();
    let params = arg_vec[1..]
        .chunks(2)
        .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
        .collect::<HashMap<_, _>>();
    ARGS.set(params).expect("unable to set ARGS once lock");
}
