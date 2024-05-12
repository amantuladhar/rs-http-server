#[derive(Copy, Clone)]
pub enum ContentType {
    Plain,
    OctetStream,
}
impl ContentType {
    pub fn to_str(&self) -> &str {
        match self {
            ContentType::Plain => "text/plain",
            ContentType::OctetStream => "application/octet-stream",
        }
    }
}
