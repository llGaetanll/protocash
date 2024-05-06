use std::error::Error;
use std::thread;
use std::time::Duration;

use cometbft_proto::abci::v1::request::Value;
use cometbft_proto::abci::v1::EchoRequest;
use cometbft_proto::abci::v1::FlushRequest;
use cometbft_proto::abci::v1::InfoRequest;
use cometbft_proto::abci::v1::Request;

use bytes::{BufMut, BytesMut};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use prost::Message;

async fn write_request(stream: &mut TcpStream, req: Request) -> Result<(), Box<dyn Error>> {
    let mut buf = BytesMut::new();
    let mut dst = BytesMut::new();

    req.encode(&mut buf)?;
    let buf = buf.freeze();

    prost::encoding::encode_varint(buf.len() as u64, &mut dst);
    dst.put(buf);

    stream.write_all(&dst).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:26658").await?;

    let req = Request {
        value: Some(Value::Info(InfoRequest {
            version: "0.1.0".to_string(),
            block_version: 1,
            p2p_version: 1,
            abci_version: "0_37".to_string(),
        })),
    };

    // every request needs to be ended by a flush to see it on the server side
    let flush = Request {
        value: Some(Value::Flush(FlushRequest {}))
    };

    write_request(&mut stream, req).await?;
    write_request(&mut stream, flush).await?;

    // the server expects the connection with the client to never die
    thread::sleep(Duration::from_secs(5));

    Ok(())
}
