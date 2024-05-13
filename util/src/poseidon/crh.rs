use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{crh::CRH, CRHGadget, Error as ArkError};
use ark_ff::ToConstraintField;
use ark_r1cs_std::{fields::fp::FpVar, uint8::UInt8, R1CSVar, ToConstraintFieldGadget};
use ark_relations::r1cs::SynthesisError;
use rand::Rng;

// TODO: Once arkworks-native-gadgets updates to the new Arkworks version, update this to use the
// new Arkworks trait TwoToOneCRHScheme
// https://github.com/webb-tools/arkworks-gadgets/blob/master/arkworks-native-gadgets/src/mimc.rs#L2=
use ark_crypto_primitives::crh::{TwoToOneCRH, TwoToOneCRHGadget};

use crate::{
    poseidon::{poseidon_iterated_hash_gadget, CRH_DOMAIN_SEP},
    util::UnitVar,
};

use super::poseidon_iterated_hash;
use super::Bls12PoseidonCrh;

impl CRH for Bls12PoseidonCrh {
    const INPUT_SIZE_BITS: usize = 0;

    type Output = BlsFr;
    type Parameters = ();

    fn setup<R: Rng>(_: &mut R) -> Result<Self::Parameters, ArkError> {
        Ok(())
    }

    fn evaluate(_: &Self::Parameters, input: &[u8]) -> Result<Self::Output, ArkError> {
        // We only use this for Merkle tree hashing over BLS12-381, so just fix the input len to 32
        assert_eq!(input.len(), 32);

        // Concat all the inputs and pack them into field elements
        let hash_input: Vec<u8> = [CRH_DOMAIN_SEP, input].concat();
        let packed_input: Vec<BlsFr> = hash_input
            .to_field_elements()
            .expect("could not pack inputs");

        // Compute the hash
        Ok(poseidon_iterated_hash(&packed_input))
    }
}

impl CRHGadget<Bls12PoseidonCrh, BlsFr> for Bls12PoseidonCrh {
    type OutputVar = FpVar<BlsFr>;
    type ParametersVar = UnitVar<BlsFr>;

    fn evaluate(
        _: &Self::ParametersVar,
        input: &[UInt8<BlsFr>],
    ) -> Result<Self::OutputVar, SynthesisError> {
        // We only use this for Merkle tree hashing over BLS12-381, so just fix the input len to 32
        assert_eq!(input.len(), 32);

        let mut cs = input.cs();

        // Concat all the inputs and pack them into field elements
        let hash_input: Vec<UInt8<_>> = [&UInt8::constant_vec(CRH_DOMAIN_SEP), input].concat();
        let packed_input: Vec<FpVar<BlsFr>> = hash_input
            .to_constraint_field()
            .expect("could not pack inputs");

        // Compute the hash
        poseidon_iterated_hash_gadget(&mut cs, &packed_input)
    }
}

impl TwoToOneCRH for Bls12PoseidonCrh {
    // This doesn't matter. We only use it for Merkle tree stuff
    const LEFT_INPUT_SIZE_BITS: usize = 0;
    const RIGHT_INPUT_SIZE_BITS: usize = 0;

    type Parameters = ();
    type Output = BlsFr;

    fn setup<R: Rng>(_: &mut R) -> Result<Self::Parameters, ArkError> {
        Ok(())
    }

    // Evaluates H(left || right)
    fn evaluate(
        _: &Self::Parameters,
        left_input: &[u8],
        right_input: &[u8],
    ) -> Result<BlsFr, ArkError> {
        // We only use this for Merkle tree hashing over BLS12-381, so just fix the input len to 32
        assert_eq!(left_input.len(), 32);
        assert_eq!(right_input.len(), 32);

        // Concat all the inputs and pack them into field elements
        let hash_input: Vec<u8> = [CRH_DOMAIN_SEP, left_input, right_input].concat();
        let packed_input: Vec<BlsFr> = hash_input
            .to_field_elements()
            .expect("could not pack inputs");

        // Compute the hash
        Ok(poseidon_iterated_hash(&packed_input))
    }
}

// Do the same thing for ZK land
impl TwoToOneCRHGadget<Bls12PoseidonCrh, BlsFr> for Bls12PoseidonCrh {
    type ParametersVar = UnitVar<BlsFr>;
    type OutputVar = FpVar<BlsFr>;

    // Evaluates H(left || right)
    fn evaluate(
        _: &UnitVar<BlsFr>,
        left_input: &[UInt8<BlsFr>],
        right_input: &[UInt8<BlsFr>],
    ) -> Result<FpVar<BlsFr>, SynthesisError> {
        // We only use this for Merkle tree hashing over BLS12-381, so just fix the input len to 32
        assert_eq!(left_input.len(), 32);
        assert_eq!(right_input.len(), 32);

        let mut cs = left_input.cs().or(right_input.cs());

        // Concat all the inputs and pack them into field elements
        let hash_input: Vec<UInt8<_>> = [
            &UInt8::constant_vec(CRH_DOMAIN_SEP),
            left_input,
            right_input,
        ]
        .concat();
        let packed_input: Vec<FpVar<BlsFr>> = hash_input
            .to_constraint_field()
            .expect("could not pack inputs");

        // Compute the hash
        poseidon_iterated_hash_gadget(&mut cs, &packed_input)
    }
}
