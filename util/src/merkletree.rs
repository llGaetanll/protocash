use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{
    crh::{TwoToOneCRH, TwoToOneCRHGadget},
    merkle_tree::{Config, MerkleTree as ArkMerkleTree, Path},
    PathVar,
};

use super::poseidon::Bls12PoseidonCrh;

#[derive(Clone)]
pub struct MerkleConfig;
impl Config for MerkleConfig {
    type LeafHash = Bls12PoseidonCrh;
    type TwoToOneHash = Bls12PoseidonCrh;
}

pub type MerkleTree = ArkMerkleTree<MerkleConfig>;

pub type Root = <Bls12PoseidonCrh as TwoToOneCRH>::Output;
pub type TreePath = Path<MerkleConfig>;

pub type RootVar = <Bls12PoseidonCrh as TwoToOneCRHGadget<Bls12PoseidonCrh, BlsFr>>::OutputVar;
pub type TreePathVar = PathVar<MerkleConfig, Bls12PoseidonCrh, Bls12PoseidonCrh, BlsFr>;
