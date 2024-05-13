use crate::poseidon::Bls12PoseidonCommitter;
use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{CommitmentGadget, CommitmentScheme};

pub type CoinCommitment = <Bls12PoseidonCommitter as CommitmentScheme>::Output;
pub type CommRand = <Bls12PoseidonCommitter as CommitmentScheme>::Randomness;

pub type CoinCommitmentVar =
    <Bls12PoseidonCommitter as CommitmentGadget<Bls12PoseidonCommitter, BlsFr>>::OutputVar;

/// A coin identifier, often called the `pre_serial_number`.
pub type CoinID = BlsFr;

/// A type used to represent public/private keys of some user on the network.
pub type Key = BlsFr;

/// A Coin. This is used in the MerkleTree as a `Coin` commitment.
pub struct Coin {
    /// The public key of the owner of this coin.
    pub pk: Key,

    /// The unique, random identifier of the coin.
    pub pre_serial_number: CoinID,

    /// Noise used when generating the coin commitment
    pub com_rnd: u64,
}
