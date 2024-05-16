use ark_bls12_381::Fr as BlsFr;
use ark_std::UniformRand;
use rand::Rng;

pub use crate::merkletree::CoinCommitment;

/// A coin identifier, often called the `pre_serial_number`.
pub type CoinID = BlsFr;

/// A type used to represent public/private keys of some user on the network.
pub type Key = BlsFr;

/// A type used to represent the randomness associated with a commitment.
pub type Rand = BlsFr;

/// A Coin. This is used in the MerkleTree as a `Coin` commitment.
#[derive(Clone)]
pub struct Coin {
    /// The public key of the owner of this coin.
    pub pk: Key,

    /// The unique, random identifier of the coin.
    pub pre_serial_number: CoinID,

    /// Noise used when generating the coin commitment
    pub com_rnd: Rand,
}

impl Coin {
    /// Generate a random [`Coin`]
    pub fn rand<R>(rng: &mut R) -> Self
    where
        R: Rng + ?Sized,
    {
        // generate a random public key
        let pk = BlsFr::rand(rng);

        // generate a random pre_serial_number
        let pre_serial_number = BlsFr::rand(rng);

        // generate some random noise
        let com_rnd = BlsFr::rand(rng);

        Self {
            pk,
            pre_serial_number,
            com_rnd,
        }
    }
}
