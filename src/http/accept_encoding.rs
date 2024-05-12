use std::str::FromStr;

#[derive(Clone)]
pub enum AcceptEncoding {
    Gzip,
}

impl AcceptEncoding {
    pub fn to_str(&self) -> &str {
        match self {
            AcceptEncoding::Gzip => "gzip",
        }
    }
}

impl FromStr for AcceptEncoding {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gzip" => Ok(AcceptEncoding::Gzip),
            _ => Err("Unsupported Accept-Encoding".to_string()),
        }
    }
}
