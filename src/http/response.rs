use crate::HTTP_LINE_ENDING;

use super::{content_type::ContentType, status_code::StatusCode};

pub struct Response {
    status_code: StatusCode,
    body: Option<Vec<u8>>,
    content_type: ContentType,
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
        msg_lines.push(format!("Content-Type: {}", self.content_type.to_str()));
        match &self.body {
            Some(body) => msg_lines.push(format!("Content-Length: {}", body.len())),
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
}

impl ResponseBuilder {
    fn new() -> Self {
        ResponseBuilder {
            status_code: StatusCode::Ok,
            body: None,
            content_type: ContentType::Plain,
        }
    }
    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
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
        }
    }
}
