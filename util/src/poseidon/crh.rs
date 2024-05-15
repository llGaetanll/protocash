use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{
    crh::{CRHScheme, CRHSchemeGadget, TwoToOneCRHScheme, TwoToOneCRHSchemeGadget},
    merkle_tree::{constraints::DigestVarConverter, DigestConverter},
    Error as ArkError,
};
use ark_ff::{BigInteger, ToConstraintField};
use ark_r1cs_std::{
    fields::fp::FpVar, uint8::UInt8, R1CSVar, ToBytesGadget, ToConstraintFieldGadget,
};
use ark_relations::r1cs::SynthesisError;
use rand::Rng;

use crate::{
    poseidon::{poseidon_iterated_hash_gadget, CRH_DOMAIN_SEP},
    types::{CoinCommitment, CoinCommitmentVar},
    util::UnitVar,
};

use super::{poseidon_iterated_hash, Bls12PoseidonDigest};
use super::{
    Bls12PoseidonCrh, Bls12PoseidonCrhVar, Bls12PoseidonDigestVar, Bls12PoseidonTwoToOneCrh,
    Bls12PoseidonTwoToOneCrhVar,
};

// input and output types

pub type CRHInput = CoinCommitment;
pub type CRHInputVar = CoinCommitmentVar;

pub type CRHOutput = BlsFr;
pub type CRHOutputVar = FpVar<BlsFr>;

pub type TwoToOneCRHInput = BlsFr;
pub type TwoToOneCRHInputVar = FpVar<BlsFr>;

pub type TwoToOneCRHOutput = BlsFr;
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
        let input: &BlsFr = input.borrow();
        let input: Vec<u8> = input.0.to_bytes_le(); // arkworks uses little-endian

        // We only use this for Merkle tree hashing over BLS12-381, so just fix the input len to 32
        assert_eq!(input.len(), 32);

        // Concat all the inputs and pack them into field elements
        let hash_input: Vec<u8> = [CRH_DOMAIN_SEP, &input].concat();
        let packed_input: Vec<BlsFr> = hash_input
            .to_field_elements()
            .expect("could not pack inputs");

        // Compute the hash
        Ok(poseidon_iterated_hash(&packed_input))
    }
}

impl CRHSchemeGadget<Bls12PoseidonCrh, BlsFr> for Bls12PoseidonCrhVar {
    type InputVar = CRHInputVar;
    type OutputVar = CRHOutputVar;
    type ParametersVar = UnitVar<BlsFr>;

    fn evaluate(
        _: &Self::ParametersVar,
        input: &Self::InputVar,
    ) -> Result<Self::OutputVar, SynthesisError> {
        let input: &FpVar<BlsFr> = input;
        let input = input.to_bytes()?; // NOTE: uses little-endian

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

impl TwoToOneCRHScheme for Bls12PoseidonTwoToOneCrh {
    type Input = TwoToOneCRHInput;
    type Output = TwoToOneCRHOutput;
    type Parameters = ();

    fn setup<R: Rng>(_: &mut R) -> Result<Self::Parameters, ArkError> {
        Ok(())
    }

    // takes a borrow of the inputs, and outputs a single of the outputs
    fn evaluate<T: std::borrow::Borrow<Self::Input>>(
        parameters: &Self::Parameters,
        left_input: T,
        right_input: T,
    ) -> Result<Self::Output, ArkError> {
        Self::compress(parameters, left_input, right_input)
    }

    // takes two of the outputs, and outputs one output
    fn compress<T: std::borrow::Borrow<Self::Output>>(
        _: &Self::Parameters,
        left_input: T,
        right_input: T,
    ) -> Result<Self::Output, ArkError> {
        let left_input: &BlsFr = left_input.borrow();
        let left_input = left_input.0.to_bytes_le();

        let right_input: &BlsFr = right_input.borrow();
        let right_input = right_input.0.to_bytes_le();

        // We only use this for Merkle tree hashing over BLS12-381, so just fix the input len to 32
        assert_eq!(left_input.len(), 32);
        assert_eq!(right_input.len(), 32);

        // Concat all the inputs and pack them into field elements
        let hash_input: Vec<u8> = [CRH_DOMAIN_SEP, &left_input, &right_input].concat();
        let packed_input: Vec<BlsFr> = hash_input
            .to_field_elements()
            .expect("could not pack inputs");

        // Compute the hash
        Ok(poseidon_iterated_hash(&packed_input))
    }
}

impl TwoToOneCRHSchemeGadget<Bls12PoseidonTwoToOneCrh, BlsFr> for Bls12PoseidonTwoToOneCrhVar {
    type InputVar = TwoToOneCRHInputVar;
    type OutputVar = TwoToOneCRHOutputVar;
    type ParametersVar = UnitVar<BlsFr>;

    fn evaluate(
        parameters: &Self::ParametersVar,
        left_input: &Self::InputVar,
        right_input: &Self::InputVar,
    ) -> Result<Self::OutputVar, SynthesisError> {
        Self::compress(parameters, left_input, right_input)
    }

    fn compress(
        _: &Self::ParametersVar,
        left_input: &Self::OutputVar,
        right_input: &Self::OutputVar,
    ) -> Result<Self::OutputVar, SynthesisError> {
        let left_input: &FpVar<BlsFr> = left_input;
        let left_input: Vec<UInt8<_>> = left_input.to_bytes().unwrap();

        let right_input: &FpVar<BlsFr> = right_input;
        let right_input: Vec<UInt8<_>> = right_input.to_bytes().unwrap();

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
}

impl DigestConverter<CRHOutput, TwoToOneCRHInput> for Bls12PoseidonDigest {
    type TargetType = TwoToOneCRHInput;

    fn convert(item: CRHOutput) -> Result<Self::TargetType, ArkError> {
        Ok(item)
    }
}

impl DigestVarConverter<CRHOutputVar, TwoToOneCRHInputVar> for Bls12PoseidonDigestVar {
    type TargetType = TwoToOneCRHInputVar;

    fn convert(from: CRHOutputVar) -> Result<Self::TargetType, SynthesisError> {
        Ok(from)
    }
}
