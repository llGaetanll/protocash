use ark_crypto_primitives::merkle_tree::constraints::DigestVarConverter;
use ark_crypto_primitives::merkle_tree::DigestConverter;
use ark_crypto_primitives::Error as ArkError;
use ark_relations::r1cs::SynthesisError;

use super::CRHOutput;
use super::CRHOutputVar;
use super::TwoToOneCRHInput;
use super::TwoToOneCRHInputVar;

pub struct PoseidonDigest;
impl DigestConverter<CRHOutput, TwoToOneCRHInput> for PoseidonDigest {
    type TargetType = TwoToOneCRHInput;

    fn convert(item: CRHOutput) -> Result<Self::TargetType, ArkError> {
        Ok(item)
    }
}

pub struct PoseidonDigestVar;
impl DigestVarConverter<CRHOutputVar, TwoToOneCRHInputVar> for PoseidonDigestVar {
    type TargetType = TwoToOneCRHInputVar;

    fn convert(from: CRHOutputVar) -> Result<Self::TargetType, SynthesisError> {
        Ok(from)
    }
}
