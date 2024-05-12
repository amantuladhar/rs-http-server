use std::{
    collections::HashMap,
    path,
    sync::{Arc, OnceLock},
};

use itertools::Itertools;
use regex::Regex;
use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};
use tracing::{debug, error};

use crate::http::method::Method;

use super::{
    method::Method::{Delete, Get, Post, Put},
    request::Request,
    response::Response,
    status_code::StatusCode,
    Parse,
};

pub type RouteHandler = fn(Request) -> Response;
type RouteMap = HashMap<ServerRoute, RouteHandler>;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum ServerRoute {
    Get(String),
    Post(String),
}

#[derive(Default)]
pub struct HttpServer {
    routes: Arc<RouteMap>,
}

impl HttpServer {
    pub fn builder() -> HttpServerBuilder {
        HttpServerBuilder::new()
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:4221")
            .await
            .expect("unable to bind tcp listener");
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let routes = Arc::clone(&self.routes);
                    tokio::spawn(async move {
                        Self::handle_request(stream, routes).await;
                    });
                }
                Err(e) => error!("error while accepting client connection. {:?}", e),
            };
        }
    }

    async fn handle_request(mut stream: TcpStream, routes: Arc<RouteMap>) {
        let (r, mut writer) = stream.split();
        let mut reader = BufReader::new(r);
        let mut request = Request::parse(&mut reader).await;
        let route_method = find_matching_route_method(&routes, &request);
        debug!("Received: {:?}", route_method);
        match route_method {
            None => {
                let response = Response::builder()
                    .status_code(StatusCode::NotFound)
                    .build();
                writer
                    .write_all(&response.as_byte())
                    .await
                    .expect("unable to write HTTP response");
            }
            Some((route, route_params)) => {
                request.params = route_params;
                let handler = routes
                    .get(route)
                    .expect("route should be available. This was already checked");
                let response = handler(request);
                writer
                    .write_all(&response.as_byte())
                    .await
                    .expect("unable to write HTTP response");
            }
        }
    }
}

fn find_matching_route_method<'a>(
    routes: &'a Arc<RouteMap>,
    request: &Request,
) -> Option<(&'a ServerRoute, HashMap<String, String>)> {
    let route_method = routes
        .iter()
        .map(|(routes, _)| routes)
        // Match with only those routes that match RequestMethod
        .filter(|routes| match request.method {
            Method::Get => match routes {
                ServerRoute::Get(_) => true,
                _ => false,
            },
            Method::Post => match routes {
                ServerRoute::Post(_) => true,
                _ => false,
            },
            _ => unimplemented!(),
        })
        // Use Regex to find route parameteres
        .find_map(|route| {
            let path = match route {
                ServerRoute::Get(p) => p,
                ServerRoute::Post(p) => p,
            };
            let mut grp_names = vec![];
            let path_regex = path
                .split("/")
                .map(|part| {
                    if !part.starts_with(":") {
                        return part.to_string();
                    }
                    let grp_name = part[1..].to_string();
                    grp_names.push(grp_name.clone());
                    // This might be better regex but test sends `/` in the path
                    //  "(?<%s>[\\w]*[^\\/])"
                    return format!(r"(?<{}>.*)", grp_name);
                })
                .join("\\/");
            let path_regex = format!(r"^{}$", path_regex);
            let regex = Regex::new(&path_regex).unwrap();
            let Some(caps) = regex.captures(&request.path) else {
                debug!(
                    "Path: {:?}, Request Path: {:?}, Match: {:?}",
                    path, request.path, false
                );
                return None;
            };
            let mut route_params = HashMap::new();
            grp_names.iter().for_each(|grp_name| {
                let value = caps.name(grp_name).unwrap().as_str().to_string();
                route_params.insert(grp_name.clone(), value);
            });
            debug!(
                "Path: {:?}, Request Path: {:?}, Match: {:?}, Params: {:?}",
                path, request.path, true, route_params
            );
            return Some((route, route_params));
        });
    route_method
}

#[derive(Default)]
pub struct HttpServerBuilder {
    routes: RouteMap,
}

impl HttpServerBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes.insert(ServerRoute::Get(path.into()), handler);
        self
    }
    pub fn post(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes.insert(ServerRoute::Post(path.into()), handler);
        self
    }
    pub fn build(self) -> HttpServer {
        HttpServer {
            routes: Arc::new(self.routes),
        }
    }
    pub async fn start(self) -> anyhow::Result<()> {
        self.build().start().await
    }
}
