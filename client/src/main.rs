use std::error::Error;

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use bytes::BytesMut;
use prost::Message;


#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestFlush {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:26658").await?;

    let req = RequestFlush {};
    let mut buf = BytesMut::with_capacity(req.encoded_len());
    req.encode(&mut buf)?;

    // Write the serialized bytes to the TCP stream
    stream.write_all(&buf).await?;

    Ok(())
}
