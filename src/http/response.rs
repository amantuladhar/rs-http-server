use crate::HTTP_LINE_ENDING;

use super::{
    accept_encoding::AcceptEncoding, content_type::ContentType, header::Header,
    status_code::StatusCode,
};

pub struct Response {
    body: Option<Vec<u8>>,
    status_code: StatusCode,
    content_type: ContentType,
    accept_encoding: Option<AcceptEncoding>,
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
        let _ = self.accept_encoding.as_ref().and_then(|accept_encoding| {
            msg_lines.push(format!(
                "{}: {}",
                Header::ContentEncoding.to_str(),
                accept_encoding.to_str()
            ));
            Some(())
        });
        match &self.body {
            Some(body) => msg_lines.push(format!(
                "{}: {}",
                Header::ContentLength.to_str(),
                body.len()
            )),
            _ => msg_lines.push("".to_string()),
        };

        msg_lines.push(HTTP_LINE_ENDING.to_string());
        // Body
        let mut bytes = msg_lines.join(HTTP_LINE_ENDING).as_bytes().to_vec();
        match &self.body {
            Some(body) => bytes.extend(body),
            _ => {}
        };
        bytes
    }
}

pub struct ResponseBuilder {
    status_code: StatusCode,
    body: Option<Vec<u8>>,
    content_type: ContentType,
    accept_encoding: Option<AcceptEncoding>,
}

impl ResponseBuilder {
    fn new() -> Self {
        ResponseBuilder {
            status_code: StatusCode::Ok,
            body: None,
            content_type: ContentType::Plain,
            accept_encoding: None,
        }
    }
    pub fn accept_encoding(mut self, accept_encoding: Option<AcceptEncoding>) -> Self {
        self.accept_encoding = accept_encoding;
        self
    }
    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
    pub fn body(mut self, body: Option<Vec<u8>>) -> Self {
        self.body = body;
        self
    }
    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = content_type;
        self
    }
    pub fn build(&self) -> Response {
        Response {
            status_code: self.status_code,
            body: self.body.clone(),
            content_type: self.content_type,
            accept_encoding: self.accept_encoding.clone(),
        }
    }
}
