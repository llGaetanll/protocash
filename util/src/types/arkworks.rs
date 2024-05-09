use ark_crypto_primitives::crh::injective_map::PedersenCRHCompressor;
use ark_crypto_primitives::crh::injective_map::TECompressor;
use ark_crypto_primitives::crh::pedersen::Window;
use ark_crypto_primitives::merkle_tree::Config;
use ark_crypto_primitives::merkle_tree::MerkleTree as ArkMerkleTree;
use ark_ed_on_bls12_381::EdwardsProjective;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LeafWindow;

// `WINDOW_SIZE * NUM_WINDOWS` = 2 * 256 bits = enough for hashing two outputs.
impl Window for LeafWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 144;
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TwoToOneWindow;

// `WINDOW_SIZE * NUM_WINDOWS` = 2 * 256 bits = enough for hashing two outputs.
impl Window for TwoToOneWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 128;
}

pub type LeafHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, LeafWindow>;
pub type TwoToOneHash = PedersenCRHCompressor<EdwardsProjective, TECompressor, TwoToOneWindow>;

#[derive(Clone)]
pub struct MerkleConfig;

impl Config for MerkleConfig {
    // Our Merkle tree relies on two hashes: one to hash leaves, and one to hash pairs of internal nodes.
    type LeafHash = LeafHash;
    type TwoToOneHash = TwoToOneHash;
}

// this merkle tree has runtime fixed height
pub type MerkleTree = ArkMerkleTree<MerkleConfig>;
