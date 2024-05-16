use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{
    crh::{poseidon::CRH as PoseidonCRH, CRHScheme},
    Error as ArkError,
};
use ark_std::UniformRand;
use rand::Rng;

use crate::types::{Key, Rand};

#[derive(Clone)]
pub struct User {
    /// A user's public key
    pub pk: Key,

    /// A user's secret key
    pub sk: Key,

    /// Used in the generation of the user's public key
    pub noise: Rand,
}

type PoseidonParams = <PoseidonCRH<BlsFr> as CRHScheme>::Parameters;

impl User {
    pub fn new<R: Rng>(params: &PoseidonParams, rng: &mut R) -> Result<Self, ArkError> {
        let sk = BlsFr::rand(rng);
        let noise = BlsFr::rand(rng);

        let pk = PoseidonCRH::<BlsFr>::evaluate(params, [sk, noise])?;

        Ok(Self { pk, sk, noise })
    }
}
