use std::io::Write;

use flate2::write::GzEncoder;

use crate::HTTP_LINE_ENDING;

use super::{
    content_type::ContentType, encoding::Encoding, header::Header, status_code::StatusCode,
};

pub struct Response {
    body: Option<Vec<u8>>,
    status_code: StatusCode,
    content_type: ContentType,
    accept_encoding: Option<Encoding>,
    content_length: Option<usize>,
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::new()
    }
    pub fn as_byte(&self) -> Vec<u8> {
        let mut msg_lines = vec![];
        // Start Line
        msg_lines.push(format!("HTTP/1.1 {}", self.status_code.status_line()));
        // Headers
        msg_lines.push(format!(
            "{}: {}",
            Header::ContentType.to_str(),
            self.content_type.to_str()
        ));
        if let Some(accept_encoding) = &self.accept_encoding {
            msg_lines.push(format!(
                "{}: {}",
                Header::ContentEncoding.to_str(),
                accept_encoding.to_str()
            ));
        }
        if let Some(content_length) = &self.content_length {
            msg_lines.push(format!(
                "{}: {}",
                Header::ContentLength.to_str(),
                content_length
            ));
        }
        msg_lines.push(HTTP_LINE_ENDING.to_string());
        // Body
        let mut response_bytes = msg_lines.join(HTTP_LINE_ENDING).as_bytes().to_vec();
        if let Some(body) = &self.body {
            response_bytes.extend(body);
        }
        response_bytes
    }
}

pub struct ResponseBuilder {
    status_code: StatusCode,
    body: Option<Vec<u8>>,
    content_type: ContentType,
    accept_encoding: Option<Encoding>,
    content_length: Option<usize>,
}

impl ResponseBuilder {
    fn new() -> Self {
        ResponseBuilder {
            status_code: StatusCode::Ok,
            body: None,
            content_type: ContentType::Plain,
            accept_encoding: None,
            content_length: None,
        }
    }
    pub fn accept_encoding(mut self, accept_encoding: Option<Encoding>) -> Self {
        self.accept_encoding = accept_encoding;
        self
    }
    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
    pub fn body(mut self, body: Option<Vec<u8>>) -> Self {
        self.body = body;
        if let Some(body) = &self.body {
            self.content_length = Some(body.len());
        }
        self
    }
    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = content_type;
        self
    }
    pub fn build(&mut self) -> Response {
        if let Some(body) = &self.body {
            if let Some(accept_encoding) = &self.accept_encoding {
                match accept_encoding {
                    Encoding::Gzip => {
                        let mut encoder =
                            GzEncoder::new(Vec::new(), flate2::Compression::default());
                        encoder.write_all(body).expect("unable to compress body");
                        self.body = Some(encoder.finish().expect("unable to finish compression"));
                        self.content_length = Some(self.body.as_ref().unwrap().len());
                    }
                }
            }
        }
        Response {
            status_code: self.status_code,
            body: self.body.clone(),
            content_length: self.content_length,
            content_type: self.content_type,
            accept_encoding: self.accept_encoding.clone(),
        }
    }
}
