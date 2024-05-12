#![warn(clippy::all)]

use std::{collections::HashMap, sync::OnceLock};

use http::{content_type::ContentType, request::Request, status_code::StatusCode};
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
    let mut res_builder = Response::builder();
    if let Some(msg) = request.params.get("message") {
        res_builder = res_builder.body(msg.as_bytes().to_vec());
    }
    res_builder.build()
}
fn user_agent(req: Request) -> Response {
    let mut res_builder = Response::builder();
    if let Some(msg) = req.headers.get("User-Agent") {
        res_builder = res_builder.body(msg.as_bytes().to_vec());
    }
    res_builder.build()
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
                    .body(file_content.as_bytes().to_vec())
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
        None => {
            res_builder = res_builder.status_code(StatusCode::InternalServerError);
        }
        Some(dir_name) => {
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
