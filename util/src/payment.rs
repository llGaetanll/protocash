use ark_bls12_381::Fr as ConstraintF;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::{alloc::AllocVar, boolean::Boolean, eq::EqGadget};
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_relations::r1cs::{ConstraintSystemRef, Result};

use crate::merkletree::{Root, RootVar, TreePath, TreePathVar};
use crate::poseidon::UnitVar;
use crate::types::{Coin, CoinCommitment, CoinCommitmentVar, CoinID, Key};

pub struct PaymentProof {
    // Public Inputs
    /// The root of the Merkle Tree
    pub root: Root,

    /// The leaf corresponding to the Coin Commitment belonging to the user.
    pub leaf: CoinCommitment,

    // Private Witnesses
    /// The path down the [`MerkleTree`] which leads to `leaf`.
    pub path: TreePath,

    /// The `Coin` we expect to match the commitment in the [`MerkleTree`].
    pub coin: Coin,

    /// The user's secret key. We prove that `pk = H(sk)`.
    pub sk: Key,

    /// The serial number to be revealed by the user. We prove that `serial_no = prf(sk, pre_serial_no)`.
    pub serial_no: CoinID,
}

impl ConstraintSynthesizer<ConstraintF> for PaymentProof {
    fn generate_constraints(self, cs: ConstraintSystemRef<ConstraintF>) -> Result<()> {
        // public inputs
        let root = RootVar::new_input(ark_relations::ns!(cs, "merkle_root"), || Ok(&self.root))?;
        let leaf =
            CoinCommitmentVar::new_input(ark_relations::ns!(cs, "merkle_leaf"), || Ok(self.leaf))?;

        // private witnesses

        // A private witness of the path down the MerkleTree which leads to the commitment.
        let path =
            TreePathVar::new_witness(ark_relations::ns!(cs, "merkle_tree_path"), || Ok(self.path))?;

        let pk = FpVar::new_witness(ark_relations::ns!(cs, "pub_key"), || Ok(self.coin.pk))?;

        // let pre_serial_no = UInt64::new_witness(ark_relations::ns!(cs, "pre_serial_no"), || {
        //     Ok(&self.coin.pre_serial_no)
        // })?;
        // let com_rnd =
        //     UInt64::new_witness(ark_relations::ns!(cs, "com_rnd"), || Ok(&self.coin.com_rnd))?;
        //
        // let sk = UInt64::new_witness(ark_relations::ns!(cs, "sec_key"), || Ok(&self.sk))?;
        let serial_no =
            FpVar::new_witness(ark_relations::ns!(cs, "serial_no"), || Ok(&self.serial_no))?;

        let p = UnitVar::new_constant(ark_relations::ns!(cs, "tree_params"), ())?;

        // 1. We prove that we have a path down the MerkleTree that leads to a commitment which
        //    hashes to:
        //    - pk
        //    - pre_serial_no
        //    - com_rnd
        let is_member = path.verify_membership(&p, &p, &root, &leaf)?;

        is_member.enforce_equal(&Boolean::TRUE)?;

        // TODO
        // // 2. We enforce that `serial_no = prf(sk, pre_serial_no)`, so that the the payer can't lie
        // //    to the payee
        // let expected_serial_no = UInt64::new_constant(
        //     ark_relations::ns!(cs, "expected_serial_no"),
        //     f(self.sk, self.coin.pre_serial_no),
        // )?;
        //
        // expected_serial_no.enforce_equal(&serial_no)?;
        //
        // // 3. We prove that `pk = H(sk)`
        // let expected_pk = UInt64::new_constant(ark_relations::ns!(cs, "expected_pk"), h(self.sk))?;
        // expected_pk.enforce_equal(&pk)?;

        Ok(())
    }
}
