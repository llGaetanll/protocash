use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::crh::{poseidon::{CRH as PoseidonCRH, constraints::CRHGadget as PoseidonCRHGadget}, CRHScheme};
use ark_r1cs_std::fields::fp::FpVar;

mod digest;
mod util;
pub mod commitment;
pub mod merkletree;

pub type BlsPoseidon = PoseidonCRH<BlsFr>;
pub type BlsPoseidonGadget = PoseidonCRHGadget<BlsFr>;

// Input and output types
pub type CoinCommitment = BlsFr;
pub type CoinCommitmentVar = FpVar<BlsFr>;

pub type CRHInput = [BlsFr];
pub type CRHInputVar = [FpVar<BlsFr>];

pub type CRHOutput = BlsFr;
pub type CRHOutputVar = FpVar<BlsFr>;

pub type TwoToOneCRHInput = BlsFr;
pub type TwoToOneCRHInputVar = FpVar<BlsFr>;

pub type TwoToOneCRHOutput = BlsFr;
pub type TwoToOneCRHOutputVar = FpVar<BlsFr>;

pub type PoseidonParams = <PoseidonCRH<BlsFr> as CRHScheme>::Parameters;

// re-exports
pub use util::get_default_poseidon_parameters;
