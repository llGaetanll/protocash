use std::{
    hash::{DefaultHasher, Hasher},
    marker::PhantomData,
};

pub mod arkworks;

/// A coin identifier, often called the `pre_serial_number`.
pub type CoinID = u64;

/// A type used to represent public/private keys of some user on the network.
pub type Key = u64;

/// A Coin. This is used in the MerkleTree as a `Coin` commitment.
pub struct Coin {
    /// The public key of the owner of this coin.
    pub pk: Key,

    /// The unique, random identifier of the coin.
    pub pre_serial_number: CoinID,

    /// Noise used when generating the coin commitment
    pub com_rnd: u64,
}

/// A commitment to some data `T`. Concretely, this is just a commitment to a [`Coin`].
#[repr(transparent)]
pub struct Commitment<T> {
    pub hash: u64,
    _t: PhantomData<T>,
}

impl Commitment<Coin> {
    /// Create a commitment from a `Coin`.
    pub fn from(coin: Coin) -> Self {
        Self {
            hash: Self::hash(coin),
            _t: PhantomData,
        }
    }

    pub fn verify(&self, coin: Coin) -> bool {
        self.hash == Self::hash(coin)
    }

    fn hash(coin: Coin) -> u64 {
        // TODO: maybe use a different hash function
        let mut hasher = DefaultHasher::new();

        hasher.write(&coin.pk.to_be_bytes());
        hasher.write(&coin.pre_serial_number.to_be_bytes());
        hasher.write(&coin.com_rnd.to_be_bytes());

        hasher.finish()
    }
}
