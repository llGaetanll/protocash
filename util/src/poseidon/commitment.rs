use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{CommitmentGadget, Error as ArkError};
use ark_ff::{to_bytes, ToConstraintField};
use ark_r1cs_std::{
    fields::fp::FpVar, uint8::UInt8, R1CSVar, ToBytesGadget, ToConstraintFieldGadget,
};
use ark_relations::r1cs::SynthesisError;
use arkworks_native_gadgets::prelude::ark_crypto_primitives::CommitmentScheme;
use rand::Rng;

use crate::util::UnitVar;

use super::{
    poseidon_iterated_hash, poseidon_iterated_hash_gadget, Bls12PoseidonCommitter, COM_DOMAIN_SEP,
};

impl CommitmentScheme for Bls12PoseidonCommitter {
    type Output = BlsFr;
    // We don't need parameters because they're set globally in the above lazy_static
    type Parameters = ();
    type Randomness = BlsFr;

    fn setup<R: Rng>(_: &mut R) -> Result<Self::Parameters, ArkError> {
        Ok(())
    }

    // Computes H(domain_sep || randomness || input)
    fn commit(
        _parameters: &Self::Parameters,
        input: &[u8],
        r: &Self::Randomness,
    ) -> Result<Self::Output, ArkError> {
        // Concat all the inputs and pack them into field elements
        let hash_input: Vec<u8> = [COM_DOMAIN_SEP, &to_bytes!(r).unwrap(), input].concat();
        let packed_input: Vec<BlsFr> = hash_input
            .to_field_elements()
            .expect("could not pack inputs");

        // Compute the hash
        Ok(poseidon_iterated_hash(&packed_input))
    }
}

impl CommitmentGadget<Bls12PoseidonCommitter, BlsFr> for Bls12PoseidonCommitter {
    type OutputVar = FpVar<BlsFr>;
    type ParametersVar = UnitVar<BlsFr>;
    type RandomnessVar = FpVar<BlsFr>;

    // Computes H(domain_sep || randomness || input)
    fn commit(
        _parameters: &Self::ParametersVar,
        input: &[UInt8<BlsFr>],
        r: &Self::RandomnessVar,
    ) -> Result<Self::OutputVar, SynthesisError> {
        let mut cs = input.cs();

        // Concat all the inputs and pack them into field elements
        let hash_input: Vec<UInt8<BlsFr>> = [
            &UInt8::constant_vec(COM_DOMAIN_SEP),
            &r.to_bytes().unwrap(),
            input,
        ]
        .concat();
        let packed_input: Vec<FpVar<BlsFr>> = hash_input
            .to_constraint_field()
            .expect("could not pack inputs");

        // Compute the hash
        poseidon_iterated_hash_gadget(&mut cs, &packed_input)
    }
}