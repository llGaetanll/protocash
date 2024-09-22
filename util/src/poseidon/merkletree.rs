use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::crh::poseidon::constraints::CRHGadget as PoseidonCRHGadget;
use ark_crypto_primitives::crh::poseidon::constraints::CRHParametersVar;
use ark_crypto_primitives::crh::poseidon::constraints::TwoToOneCRHGadget as PoseidonTwoToOneCRHGadget;
use ark_crypto_primitives::crh::poseidon::TwoToOneCRH as PoseidonTwoToOneCRH;
use ark_crypto_primitives::crh::poseidon::CRH as PoseidonCRH;
use ark_crypto_primitives::crh::TwoToOneCRHScheme;
use ark_crypto_primitives::crh::TwoToOneCRHSchemeGadget;
use ark_crypto_primitives::merkle_tree::constraints::ConfigGadget;
use ark_crypto_primitives::merkle_tree::constraints::PathVar;
use ark_crypto_primitives::merkle_tree::Config;
use ark_crypto_primitives::merkle_tree::MerkleTree as ArkMerkleTree;
use ark_crypto_primitives::merkle_tree::Path;
use ark_crypto_primitives::sponge::poseidon::PoseidonConfig;

use super::digest::PoseidonDigest;
use super::digest::PoseidonDigestVar;
use super::CRHInput;
use super::CRHInputVar;
use super::CRHOutput;
use super::CRHOutputVar;
use super::TwoToOneCRHOutput;
use super::TwoToOneCRHOutputVar;

#[derive(Clone)]
pub struct MerkleConfig;
impl Config for MerkleConfig {
    type Leaf = CRHInput;

    type LeafDigest = CRHOutput;

    type LeafInnerDigestConverter = PoseidonDigest;

    type InnerDigest = TwoToOneCRHOutput;

    type LeafHash = LeafHash;

    type TwoToOneHash = TwoToOneHash;
}

#[derive(Clone)]
pub struct MerkleConfigVar;
impl ConfigGadget<MerkleConfig, BlsFr> for MerkleConfigVar {
    type Leaf = CRHInputVar;

    type LeafDigest = CRHOutputVar;

    type LeafInnerConverter = PoseidonDigestVar;

    type InnerDigest = TwoToOneCRHOutputVar;

    type LeafHash = LeafHashVar;

    type TwoToOneHash = TwoToOneHashVar;
}

pub type LeafHash = PoseidonCRH<BlsFr>;
pub type TwoToOneHash = PoseidonTwoToOneCRH<BlsFr>;

pub type LeafHashVar = PoseidonCRHGadget<BlsFr>;
pub type TwoToOneHashVar = PoseidonTwoToOneCRHGadget<BlsFr>;

pub type Root = <PoseidonTwoToOneCRH<BlsFr> as TwoToOneCRHScheme>::Output;
pub type RootVar = <PoseidonTwoToOneCRHGadget<BlsFr> as TwoToOneCRHSchemeGadget<
    PoseidonTwoToOneCRH<BlsFr>,
    BlsFr,
>>::OutputVar;

pub type Params = PoseidonConfig<BlsFr>;
pub type ParamsVar = CRHParametersVar<BlsFr>;

pub type TreePath = Path<MerkleConfig>;
pub type TreePathVar = PathVar<MerkleConfig, BlsFr, MerkleConfigVar>;

pub type MerkleTree = ArkMerkleTree<MerkleConfig>;
