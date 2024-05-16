use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{
    crh::{poseidon::CRH as PoseidonCRH, CRHScheme},
    Error as ArkError,
};
use rand::Rng;

use crate::{merkletree::CoinCommitment, types::Coin};

type PoseidonParams = <PoseidonCRH<BlsFr> as CRHScheme>::Parameters;

pub fn new_commitment(
    parameters: &PoseidonParams,
    coin: &Coin,
) -> Result<CoinCommitment, ArkError> {
    let input: [BlsFr; 3] = [coin.pk, coin.pre_serial_number, coin.com_rnd];

    PoseidonCRH::<BlsFr>::evaluate(parameters, input)
}

pub fn rand<R: Rng>(parameters: &PoseidonParams, rng: &mut R) -> Result<CoinCommitment, ArkError> {
    new_commitment(parameters, &Coin::rand(rng))
}
