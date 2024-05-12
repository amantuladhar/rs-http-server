use tokio::io::{AsyncBufRead, AsyncRead};

pub mod accept_encoding;
pub mod content_type;
pub mod header;
pub mod method;
pub mod request;
pub mod response;
pub mod server;
pub mod status_code;

pub trait Parse<R>
where
    R: AsyncRead + AsyncBufRead + Unpin,
{
    async fn parse(reader: &mut R) -> Self
    where
        Self: Sized;
}
