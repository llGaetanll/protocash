use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::crh::CRHScheme;
use ark_crypto_primitives::Error as ArkError;
use ark_std::UniformRand;
use rand::Rng;

use crate::poseidon::BlsPoseidon;
use crate::poseidon::PoseidonParams;
use crate::types::Key;
use crate::types::Rand;

#[derive(Clone)]
pub struct User {
    /// A user's public key
    pub pk: Key,

    /// A user's secret key
    pub sk: Key,

    /// Used in the generation of the user's public key
    pub noise: Rand,
}

impl User {
    pub fn new<R: Rng>(params: &PoseidonParams, rng: &mut R) -> Result<Self, ArkError> {
        let sk = BlsFr::rand(rng);
        let noise = BlsFr::rand(rng);

        let pk = BlsPoseidon::evaluate(params, [sk, noise])?;

        Ok(Self { pk, sk, noise })
    }
}
