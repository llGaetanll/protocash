use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{
    crh::CRHScheme,
    Error as ArkError,
};
use rand::Rng;

use crate::types::Coin;

use super::{BlsPoseidon, CoinCommitment, PoseidonParams};

pub fn new_commitment(
    parameters: &PoseidonParams,
    coin: &Coin,
) -> Result<CoinCommitment, ArkError> {
    let input: [BlsFr; 3] = [coin.pk, coin.pre_serial_number, coin.com_rnd];

    BlsPoseidon::evaluate(parameters, input)
}

pub fn rand<R: Rng>(parameters: &PoseidonParams, rng: &mut R) -> Result<CoinCommitment, ArkError> {
    new_commitment(parameters, &Coin::rand(rng))
}
