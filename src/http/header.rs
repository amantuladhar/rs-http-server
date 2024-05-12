use std::str::FromStr;

pub enum Header {
    ContentType,
    AcceptEncoding,
    ContentLength,
    ContentEncoding,
    UserAgent,
    Host,
}

impl Header {
    pub fn to_str(&self) -> &str {
        match self {
            Header::ContentType => "content-type",
            Header::AcceptEncoding => "accept-encoding",
            Header::ContentLength => "content-length",
            Header::ContentEncoding => "content-encoding",
            Header::UserAgent => "user-agent",
            Header::Host => "host",
        }
    }
}

impl FromStr for Header {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "content-type" => Ok(Header::ContentType),
            "accept-encoding" => Ok(Header::AcceptEncoding),
            "content-length" => Ok(Header::ContentLength),
            "content-encoding" => Ok(Header::ContentEncoding),
            "user-agent" => Ok(Header::UserAgent),
            "host" => Ok(Header::Host),
            _ => Err("Unsupported Header".to_string()),
        }
    }
}
