use std::str::FromStr;

#[derive(Clone)]
pub enum Encoding {
    Gzip,
}

impl Encoding {
    pub fn to_str(&self) -> &str {
        match self {
            Encoding::Gzip => "gzip",
        }
    }
}

impl FromStr for Encoding {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gzip" => Ok(Encoding::Gzip),
            _ => Err("Unsupported Encoding".to_string()),
        }
    }
}
