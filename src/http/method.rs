use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl FromStr for Method {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "Delete" => Ok(Method::Delete),
            _ => Err(()),
        }
    }
}
