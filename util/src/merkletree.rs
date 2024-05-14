use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{
    crh::{CRHScheme, CRHSchemeGadget, TwoToOneCRHScheme, TwoToOneCRHSchemeGadget},
    merkle_tree::{
        constraints::{ConfigGadget, PathVar},
        Config, MerkleTree as ArkMerkleTree, Path,
    },
};

use super::poseidon::Bls12PoseidonCrh;
use crate::poseidon::{
    crh::{
        CRHInput, CRHInputVar, CRHOutput, CRHOutputVar, TwoToOneCRHOutput, TwoToOneCRHOutputVar,
    },
    Bls12PoseidonCrhVar, Bls12PoseidonDigest, Bls12PoseidonDigestVar, Bls12PoseidonTwoToOneCrh,
    Bls12PoseidonTwoToOneCrhVar,
};

#[derive(Clone)]
pub struct MerkleConfig;
impl Config for MerkleConfig {
    type Leaf = CRHInput;
    type LeafHash = Bls12PoseidonCrh;
    type LeafDigest = CRHOutput;
    type InnerDigest = TwoToOneCRHOutput;
    type LeafInnerDigestConverter = Bls12PoseidonDigest;
    type TwoToOneHash = Bls12PoseidonTwoToOneCrh;
}

#[derive(Clone)]
pub struct MerkleConfigVar;
impl ConfigGadget<MerkleConfig, BlsFr> for MerkleConfigVar {
    type Leaf = CRHInputVar;
    type LeafHash = Bls12PoseidonCrhVar;
    type LeafDigest = CRHOutputVar;
    type InnerDigest = TwoToOneCRHOutputVar;
    type LeafInnerConverter = Bls12PoseidonDigestVar;
    type TwoToOneHash = Bls12PoseidonTwoToOneCrhVar;
}

pub type MerkleTree = ArkMerkleTree<MerkleConfig>;

pub type Root = <Bls12PoseidonTwoToOneCrh as TwoToOneCRHScheme>::Output;
pub type Leaf = <Bls12PoseidonCrh as CRHScheme>::Input;
pub type TreePath = Path<MerkleConfig>;

pub type RootVar = <Bls12PoseidonTwoToOneCrhVar as TwoToOneCRHSchemeGadget<Bls12PoseidonTwoToOneCrh, BlsFr>>::OutputVar;
pub type LeafVar = <Bls12PoseidonCrhVar as CRHSchemeGadget<Bls12PoseidonCrh, BlsFr>>::InputVar;
pub type TreePathVar = PathVar<MerkleConfig, BlsFr, MerkleConfigVar>;
