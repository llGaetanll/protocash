use super::{digest::{PoseidonDigest, PoseidonDigestVar}, CRHInput, CRHInputVar, CRHOutput, CRHOutputVar, TwoToOneCRHOutput, TwoToOneCRHOutputVar};
use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::{
    crh::{
        poseidon::{
            constraints::{
                CRHGadget as PoseidonCRHGadget, CRHParametersVar,
                TwoToOneCRHGadget as PoseidonTwoToOneCRHGadget,
            },
            TwoToOneCRH as PoseidonTwoToOneCRH, CRH as PoseidonCRH,
        },
        TwoToOneCRHScheme, TwoToOneCRHSchemeGadget,
    },
    merkle_tree::{
        constraints::{ConfigGadget, PathVar},
        Config, MerkleTree as ArkMerkleTree, Path,
    },
    sponge::poseidon::PoseidonConfig,
};

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
