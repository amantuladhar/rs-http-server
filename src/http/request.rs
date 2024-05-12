use std::{collections::HashMap, str::FromStr};

use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt};

use crate::HTTP_LINE_ENDING;

use super::{method::Method, Parse};

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    #[allow(dead_code)]
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl<R> Parse<R> for Request
where
    R: AsyncRead + AsyncBufRead + Unpin,
{
    async fn parse(reader: &mut R) -> Self {
        let start_line = StartLine::parse(reader).await;
        let headers = Self::parse_headers(reader).await;
        let body = Self::parse_body(reader, &headers).await;
        Request {
            method: start_line.method,
            path: start_line.path,
            http_version: start_line.version,
            headers: headers,
            params: HashMap::default(),
            body: body,
        }
    }
}
struct StartLine {
    method: Method,
    path: String,
    version: String,
}

impl<R> Parse<R> for StartLine
where
    R: AsyncRead + AsyncBufRead + Unpin,
{
    async fn parse(reader: &mut R) -> Self {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .await
            .expect("unable to read status line from a request stream");
        let line = line[..line.len() - HTTP_LINE_ENDING.len()]
            .split(" ")
            .collect::<Vec<&str>>();
        let method = Method::from_str(line[0]).expect("not able to parse request method type");
        let path = line[1].into();
        let version = line[2].into();
        StartLine {
            method,
            path,
            version,
        }
    }
}

impl Request {
    pub async fn parse_body<R>(reader: &mut R, headers: &HashMap<String, String>) -> Vec<u8>
    where
        R: AsyncRead + AsyncReadExt + AsyncBufRead + Unpin,
    {
        let length = headers
            .get("content-length")
            .map_or("0", |v| v.as_str())
            .parse::<usize>()
            .unwrap();
        let mut body = vec![0; length];
        reader.read_exact(&mut body).await.unwrap();
        return body;
    }

    pub async fn parse_headers<R>(reader: &mut R) -> HashMap<String, String>
    where
        R: AsyncRead + AsyncBufRead + Unpin,
    {
        let mut headers = HashMap::<String, String>::new();
        loop {
            let mut cur_header = String::new();
            if let Err(err) = reader.read_line(&mut cur_header).await {
                println!("Error occurred while reading header: {}", err);
                break;
            }
            if cur_header == HTTP_LINE_ENDING || cur_header == "\n" || cur_header.is_empty() {
                // If cur_header is empty, it means we've reached the end of headers
                break;
            }
            let (key, value) = Self::parse_header(&cur_header);
            headers.insert(key.to_lowercase(), value.to_string());
        }
        return headers;
    }
    pub fn parse_header(header: &str) -> (&str, &str) {
        let (key, value) = header.split_once(": ").unwrap();
        (key, &value[..value.len() - HTTP_LINE_ENDING.len()])
    }
}
