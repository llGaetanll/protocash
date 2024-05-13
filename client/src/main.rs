use bytes::{BufMut, BytesMut};
use cometbft_proto::abci::v1::{request::Value, FlushRequest, InfoRequest, Request};
use prost::Message;
use std::{error::Error, thread, time::Duration};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use protocash_util::types::{CoinCommitment, Key};

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

struct Client {
    /// The client's public key
    pub pk: Key,

    /// The client's secret key
    sk: Key,

    /// These are the client's commitments. These commitments should be in the MerkleTree
    my_coins: Vec<CoinCommitment>,

    /// These are *all* the transactions on the network. The client needs to know this - and in
    /// fact, keep an up-to-date picture of this - in order to make the proof of payment to the
    /// validator nodes.
    all_coins: Vec<CoinCommitment>,
}

impl Client {
    pub fn new() -> Self {
        todo!()
    }

    fn pay(&self, user: Key) {
        // In order for `self` to pay `user`, `self` needs to know a few things.
        //
        // - The Merkle Tree of transactions
        //
        // - A list of `self`'s commitments, which sit in this Merkle Tree
        //   Specifically here, the commitments need to live in the Merkle Tree and Client needs to
        //   keep track of its commitments.
        //
        // Once `self` knows these things, a zk proof needs to be made
    }

    /// Withdraw a transaction from the MerkleTree. Formally, when somebody makes a transaction to
    /// us, they are responsible for sending us the `pre_serial_no` and the `com_rnd`. Once this is
    /// done, the client can then hash this commitment into a leaf of the MerkleTree to verify its
    /// existence. It is then added to `self.commitments` for `self` to use.
    fn withdraw(&self) {}
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
        value: Some(Value::Flush(FlushRequest {})),
    };

    write_request(&mut stream, req).await?;
    write_request(&mut stream, flush).await?;

    // the server expects the connection with the client to never die
    thread::sleep(Duration::from_secs(5));

    Ok(())
}
