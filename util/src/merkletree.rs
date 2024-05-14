use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{
    crh::{CRHScheme, CRHSchemeGadget, TwoToOneCRHScheme, TwoToOneCRHSchemeGadget},
    merkle_tree::{
        constraints::{ConfigGadget, PathVar},
        Config, MerkleTree as ArkMerkleTree, Path,
    },
};
use ark_r1cs_std::{fields::fp::FpVar, uint8::UInt8};

use super::poseidon::Bls12PoseidonCrh;
use crate::poseidon::Bls12PoseidonDigestConverter;

#[derive(Clone)]
pub struct MerkleConfig;
impl Config for MerkleConfig {
    type Leaf = Vec<u8>;
    type LeafDigest = BlsFr;
    type InnerDigest = BlsFr;
    type LeafInnerDigestConverter = Bls12PoseidonDigestConverter;
    type LeafHash = Bls12PoseidonCrh;
    type TwoToOneHash = Bls12PoseidonCrh;
}

#[derive(Clone)]
pub struct MerkleConfigVar;
impl ConfigGadget<MerkleConfig, BlsFr> for MerkleConfigVar {
    type Leaf = Vec<UInt8<BlsFr>>;
    type LeafDigest = FpVar<BlsFr>;
    type InnerDigest = FpVar<BlsFr>;
    type LeafInnerConverter = Bls12PoseidonDigestConverter;
    type LeafHash = Bls12PoseidonCrh;
    type TwoToOneHash = Bls12PoseidonCrh;
}

pub type MerkleTree = ArkMerkleTree<MerkleConfig>;

pub type Root = <Bls12PoseidonCrh as TwoToOneCRHScheme>::Output;
pub type Leaf = <Bls12PoseidonCrh as CRHScheme>::Input;
pub type TreePath = Path<MerkleConfig>;

pub type RootVar =
    <Bls12PoseidonCrh as TwoToOneCRHSchemeGadget<Bls12PoseidonCrh, BlsFr>>::OutputVar;
pub type LeafVar = <Bls12PoseidonCrh as CRHSchemeGadget<Bls12PoseidonCrh, BlsFr>>::InputVar;
pub type TreePathVar = PathVar<MerkleConfig, BlsFr, MerkleConfigVar>;
