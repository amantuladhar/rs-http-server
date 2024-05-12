#[derive(Copy, Clone)]
pub enum StatusCode {
    Ok,
    Created,
    NotFound,
    InternalServerError,
}
impl StatusCode {
    pub fn status_line<'a>(&self) -> &'a str {
        match self {
            StatusCode::Ok => "200 OK",
            StatusCode::Created => "201 Created",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::InternalServerError => "500 Internal Server Error",
        }
    }
}
