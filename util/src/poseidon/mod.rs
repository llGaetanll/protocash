use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{crh::CRH, CRHGadget, CommitmentGadget, Error as ArkError};
use ark_ff::{to_bytes, PrimeField, ToConstraintField};
use ark_r1cs_std::{
    alloc::{AllocVar, AllocationMode},
    fields::fp::FpVar,
    uint8::UInt8,
    R1CSVar, ToBytesGadget, ToConstraintFieldGadget,
};
use ark_relations::r1cs::{ConstraintSystemRef, Namespace, SynthesisError};
use arkworks_native_gadgets::{
    poseidon::{sbox::PoseidonSbox, FieldHasher, Poseidon, PoseidonParameters},
    prelude::ark_crypto_primitives::CommitmentScheme,
};
use arkworks_r1cs_gadgets::poseidon::{FieldHasherGadget, PoseidonGadget};
use arkworks_utils::{bytes_matrix_to_f, bytes_vec_to_f, Curve};
use lazy_static::lazy_static;
use rand::Rng;
use std::{borrow::Borrow, marker::PhantomData};

// TODO: Once arkworks-native-gadgets updates to the new Arkworks version, update this to use the
// new Arkworks trait TwoToOneCRHScheme
// https://github.com/webb-tools/arkworks-gadgets/blob/master/arkworks-native-gadgets/src/mimc.rs#L2=
use ark_crypto_primitives::crh::{TwoToOneCRH, TwoToOneCRHGadget};

// from: https://github.com/rozbb/zkcreds-rs/blob/main/src/poseidon_utils.rs

fn setup_poseidon_params<F: PrimeField>(curve: Curve, exp: i8, width: u8) -> PoseidonParameters<F> {
    let pos_data =
        arkworks_utils::poseidon_params::setup_poseidon_params(curve, exp, width).unwrap();

    let mds_f = bytes_matrix_to_f(&pos_data.mds);
    let rounds_f = bytes_vec_to_f(&pos_data.rounds);

    PoseidonParameters {
        mds_matrix: mds_f,
        round_keys: rounds_f,
        full_rounds: pos_data.full_rounds,
        partial_rounds: pos_data.partial_rounds,
        sbox: PoseidonSbox(pos_data.exp),
        width: pos_data.width,
    }
}

// Pick global parameters for Poseidon over BLS12-381
pub const POSEIDON_WIDTH: u8 = 5;
pub const COM_DOMAIN_SEP: &[u8] = b"pcom";
pub const CRH_DOMAIN_SEP: &[u8] = b"pcrh";
lazy_static! {
    static ref BLS12_POSEIDON_PARAMS: PoseidonParameters<BlsFr> =
        setup_poseidon_params(Curve::Bls381, 3, POSEIDON_WIDTH);
}

/// A commitment scheme defined using the Poseidon hash function over BLS12-381
pub struct Bls12PoseidonCommitter;

/// Represents the collision-resistant hashing functionality of Poseidon over BLS12-381
pub struct Bls12PoseidonCrh;

fn poseidon_iterated_hash(input: &[BlsFr]) -> BlsFr {
    let hasher = Poseidon::new(BLS12_POSEIDON_PARAMS.clone());
    let first_block_len = core::cmp::min(input.len(), (POSEIDON_WIDTH - 1) as usize);

    let first_block = &input[..first_block_len];
    let mut running_hash = hasher.hash(first_block).unwrap();
    for block in input[first_block_len..].chunks((POSEIDON_WIDTH - 2) as usize) {
        let next_input = &[&[running_hash], block].concat();
        running_hash = hasher.hash(next_input).unwrap();
    }
    running_hash
}

fn poseidon_iterated_hash_gadget(
    cs: &mut ConstraintSystemRef<BlsFr>,
    input: &[FpVar<BlsFr>],
) -> Result<FpVar<BlsFr>, SynthesisError> {
    let hasher = Poseidon::new(BLS12_POSEIDON_PARAMS.clone());
    let hasher_var = PoseidonGadget::from_native(cs, hasher)?;
    let first_block_len = core::cmp::min(input.len(), (POSEIDON_WIDTH - 1) as usize);

    let first_block = &input[..first_block_len];
    let mut running_hash = hasher_var.hash(first_block)?;
    for block in input[first_block_len..].chunks((POSEIDON_WIDTH - 2) as usize) {
        let next_input = &[&[running_hash], block].concat();
        running_hash = hasher_var.hash(next_input)?;
    }

    Ok(running_hash)
}

pub mod commitment;
pub mod crh;
