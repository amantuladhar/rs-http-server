use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
    str::FromStr,
};

use crate::HTTP_LINE_ENDING;

use super::method::Method;

pub struct Request {
    pub method: Method,
    pub path: String,
    #[allow(dead_code)]
    pub http_version: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn parse(stream: &TcpStream) -> Self {
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

    pub fn parse_body(
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

    pub fn parse_headers(reader: &mut BufReader<&TcpStream>) -> HashMap<String, String> {
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
    pub fn parse_header(header: &str) -> (&str, &str) {
        // println!("parse_header: {}", header);
        let (key, value) = header.split_once(": ").unwrap();
        (key, &value[..value.len() - HTTP_LINE_ENDING.len()])
    }
    pub fn parse_start_line(reader: &mut BufReader<&TcpStream>) -> (Method, String, String) {
        // FIXME: There is a issue where this status_line is empty when using wrk
        let mut start_line = String::new();
        if let Err(err) = reader.read_line(&mut start_line) {
            println!("Unable to read start line of the request: {}", err);
        }
        let start_line = start_line[..(start_line.len() - HTTP_LINE_ENDING.len())]
            .split(" ")
            .collect::<Vec<_>>();
        (
            Method::from_str(start_line[0]).unwrap(),
            start_line[1].to_string(),
            start_line[2].to_string(),
        )
    }
}
