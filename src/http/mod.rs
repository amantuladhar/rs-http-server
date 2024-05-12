use tokio::io::{AsyncBufRead, AsyncRead};

pub mod content_type;
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
