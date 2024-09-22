use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::crh::poseidon::constraints::CRHGadget as PoseidonCRHGadget;
use ark_crypto_primitives::crh::poseidon::CRH as PoseidonCRH;
use ark_crypto_primitives::crh::CRHScheme;
use ark_r1cs_std::fields::fp::FpVar;

pub mod commitment;
mod digest;
pub mod merkletree;
mod util;

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
