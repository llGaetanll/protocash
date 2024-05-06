use std::{hash::{DefaultHasher, Hasher}, marker::PhantomData};

mod arkworks;

pub type CoinID = u64; /// A coin identifier, often called the `pre_serial_number`.
pub type PubKey = u64; /// The public key of some user on the network.

pub struct Coin {
    /// The public key of the owner of this coin.
    pub key: PubKey,

    /// The unique, random identifier of the coin.
    pub pre_serial_number: CoinID,

    /// Noise used when generating the coin commitment
    pub com_rnd: u64
}

#[repr(transparent)]
pub struct Commitment<T> {
    pub hash: u64,
    _t: PhantomData<T>
}

impl Commitment<Coin> {
    /// Create a commitment from a `Coin`.
    pub fn from(coin: Coin) -> Self {
        Self {
            hash: Self::hash(coin),
            _t: PhantomData
        }
    }

    pub fn verify(&self, coin: Coin) -> bool {
        self.hash == Self::hash(coin)
    }

    fn hash(coin: Coin) -> u64 {
        let mut hasher = DefaultHasher::new();

        hasher.write(&coin.key.to_be_bytes());
        hasher.write(&coin.pre_serial_number.to_be_bytes());
        hasher.write(&coin.com_rnd.to_be_bytes());

        hasher.finish()
    }
}

type Proof = u64; // TODO

pub struct TxRequest {
    /// The address of the recipient of the transaction
    to: PubKey,

    /// A proof from the sender of the transaction
    pf: Proof,
}
