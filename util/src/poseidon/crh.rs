use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{
    crh::{CRHScheme, CRHSchemeGadget, TwoToOneCRHScheme, TwoToOneCRHSchemeGadget},
    merkle_tree::{constraints::DigestVarConverter, DigestConverter},
    Error as ArkError,
};
use ark_ff::{BigInteger, ToConstraintField};
use ark_r1cs_std::{fields::fp::FpVar, uint8::UInt8, R1CSVar, ToConstraintFieldGadget};
use ark_relations::r1cs::SynthesisError;
use rand::Rng;

use crate::{
    poseidon::{poseidon_iterated_hash_gadget, CRH_DOMAIN_SEP},
    util::UnitVar,
};

use super::Bls12PoseidonCrh;
use super::{poseidon_iterated_hash, Bls12PoseidonDigestConverter};

pub type CRHInput = Vec<u8>;
pub type CRHOutput = BlsFr;

pub type CRHInputVar = Vec<UInt8<BlsFr>>;
pub type CRHOutputVar = FpVar<BlsFr>;

pub type TwoToOneCRHInput = Vec<u8>;
pub type TwoToOneCRHOutput = BlsFr;

pub type TwoToOneCRHInputVar = Vec<UInt8<BlsFr>>;
pub type TwoToOneCRHOutputVar = FpVar<BlsFr>;

impl CRHScheme for Bls12PoseidonCrh {
    type Input = CRHInput;
    type Output = CRHOutput;
    type Parameters = ();

    fn setup<R: Rng>(_: &mut R) -> Result<Self::Parameters, ArkError> {
        Ok(())
    }

    fn evaluate<T: std::borrow::Borrow<Self::Input>>(
        _: &Self::Parameters,
        input: T,
    ) -> Result<Self::Output, ArkError> {
        let input: &[u8] = input.borrow();

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

impl CRHSchemeGadget<Bls12PoseidonCrh, BlsFr> for Bls12PoseidonCrh {
    type InputVar = CRHInputVar;
    type OutputVar = CRHOutputVar;
    type ParametersVar = UnitVar<BlsFr>;

    fn evaluate(
        _: &Self::ParametersVar,
        input: &Self::InputVar,
    ) -> Result<Self::OutputVar, SynthesisError> {
        // We only use this for Merkle tree hashing over BLS12-381, so just fix the input len to 32
        assert_eq!(input.len(), 32);

        let mut cs = input.cs();

        // Concat all the inputs and pack them into field elements
        let hash_input: Vec<UInt8<_>> =
            [UInt8::constant_vec(CRH_DOMAIN_SEP), input.to_owned()].concat();
        let packed_input: Vec<FpVar<BlsFr>> = hash_input
            .to_constraint_field()
            .expect("could not pack inputs");

        // Compute the hash
        poseidon_iterated_hash_gadget(&mut cs, &packed_input)
    }
}

impl TwoToOneCRHScheme for Bls12PoseidonCrh {
    type Input = TwoToOneCRHInput;
    type Output = TwoToOneCRHOutput;
    type Parameters = ();

    fn setup<R: Rng>(_: &mut R) -> Result<Self::Parameters, ArkError> {
        Ok(())
    }

    fn evaluate<T: std::borrow::Borrow<Self::Input>>(
        _: &Self::Parameters,
        left_input: T,
        right_input: T,
    ) -> Result<Self::Output, ArkError> {
        let left_input: &[_] = left_input.borrow();
        let right_input: &[_] = right_input.borrow();

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

    fn compress<T: std::borrow::Borrow<Self::Output>>(
        parameters: &Self::Parameters,
        left_input: T,
        right_input: T,
    ) -> Result<Self::Output, ArkError> {
        unimplemented!()
    }
}

impl TwoToOneCRHSchemeGadget<Bls12PoseidonCrh, BlsFr> for Bls12PoseidonCrh {
    type InputVar = TwoToOneCRHInputVar;
    type OutputVar = TwoToOneCRHOutputVar;
    type ParametersVar = UnitVar<BlsFr>;

    fn evaluate(
        _: &Self::ParametersVar,
        left_input: &Self::InputVar,
        right_input: &Self::InputVar,
    ) -> Result<Self::OutputVar, SynthesisError> {
        // We only use this for Merkle tree hashing over BLS12-381, so just fix the input len to 32
        assert_eq!(left_input.len(), 32);
        assert_eq!(right_input.len(), 32);

        let mut cs = left_input.cs().or(right_input.cs());

        // Concat all the inputs and pack them into field elements
        let hash_input: Vec<UInt8<_>> = [
            &UInt8::constant_vec(CRH_DOMAIN_SEP),
            left_input.as_slice(),
            right_input.as_slice(),
        ]
        .concat();
        let packed_input: Vec<FpVar<BlsFr>> = hash_input
            .to_constraint_field()
            .expect("could not pack inputs");

        // Compute the hash
        poseidon_iterated_hash_gadget(&mut cs, &packed_input)
    }

    fn compress(
        parameters: &Self::ParametersVar,
        left_input: &Self::OutputVar,
        right_input: &Self::OutputVar,
    ) -> Result<Self::OutputVar, SynthesisError> {
        unimplemented!()
    }
}

impl DigestConverter<CRHOutput, CRHInput> for Bls12PoseidonDigestConverter {
    type TargetType = Vec<u8>;

    fn convert(item: BlsFr) -> Result<Self::TargetType, ArkError> {
        Ok(item.0.to_bytes_be())
    }
}

impl DigestVarConverter<CRHOutputVar, CRHInputVar> for Bls12PoseidonDigestConverter {
    type TargetType = Vec<UInt8<BlsFr>>;

    fn convert(from: FpVar<BlsFr>) -> Result<Self::TargetType, SynthesisError> {
        todo!()
    }
}
