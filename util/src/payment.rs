use ark_bls12_381::Fr as ConstraintF;
use ark_r1cs_std::{alloc::AllocVar, boolean::Boolean, eq::EqGadget};
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_relations::r1cs::{ConstraintSystemRef, Result};

use crate::merkletree::{
    CoinCommitment, CoinCommitmentVar, LeafParams, LeafParamsVar, Root, RootVar, TreePath,
    TreePathVar, TwoToOneParams, TwoToOneParamsVar,
};
use crate::types::{Coin, CoinID, Key};

#[derive(Clone)]
pub struct PaymentProof {
    leaf_crh_params: LeafParams,
    two_to_one_crh_params: TwoToOneParams,

    // Public Inputs
    /// The root of the Merkle Tree
    pub root: Root,

    /// The leaf corresponding to the Coin Commitment belonging to the user.
    pub commitment: CoinCommitment,

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
        let root = RootVar::new_input(ark_relations::ns!(cs, "merkle_root"), || Ok(self.root))?;
        let leaf = CoinCommitmentVar::new_input(ark_relations::ns!(cs, "merkle_leaf"), || {
            Ok(self.commitment)
        })?;

        // private witnesses

        // A private witness of the path down the MerkleTree which leads to the commitment.
        let path =
            TreePathVar::new_witness(ark_relations::ns!(cs, "merkle_tree_path"), || Ok(self.path))?;
        // let pk = FpVar::new_witness(ark_relations::ns!(cs, "pub_key"), || Ok(self.coin.pk))?;

        // let pre_serial_no = UInt64::new_witness(ark_relations::ns!(cs, "pre_serial_no"), || {
        //     Ok(&self.coin.pre_serial_no)
        // })?;
        // let com_rnd =
        //     UInt64::new_witness(ark_relations::ns!(cs, "com_rnd"), || Ok(&self.coin.com_rnd))?;
        //
        // let sk = UInt64::new_witness(ark_relations::ns!(cs, "sec_key"), || Ok(&self.sk))?;
        // let serial_no = FpVar::new_witness(ark_relations::ns!(cs, "serial_no"), || Ok(&self.serial_no))?;
        let leaf_params = LeafParamsVar::new_constant(
            ark_relations::ns!(cs, "leaf_params"),
            self.leaf_crh_params,
        )?;
        let two_to_one_params = TwoToOneParamsVar::new_constant(
            ark_relations::ns!(cs, "leaf_params"),
            self.two_to_one_crh_params,
        )?;

        // 1. We prove that we have a path down the MerkleTree that leads to a commitment which
        //    hashes to:
        //    - pk
        //    - pre_serial_no
        //    - com_rnd
        let is_member = path.verify_membership(&leaf_params, &two_to_one_params, &root, &[leaf])?;

        is_member.enforce_equal(&Boolean::TRUE)?;

        // // 2. We enforce that `serial_no = prf(sk, pre_serial_no)`, so that the the payer can't lie
        // //    to the payee
        // let expected_serial_no = UInt64::new_constant(
        //     ark_relations::ns!(cs, "expected_serial_no"),
        //     f(self.sk, self.coin.pre_serial_number),
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

#[cfg(test)]
mod test {
    use crate::{
        merkletree::{MerkleTree, Root, TreePath},
        poseidon_native::{commitment, get_default_poseidon_parameters},
        types::{Coin, CoinID, Key},
    };
    use ark_bls12_381::{Bls12_381, Fr as BlsFr};
    use ark_groth16::{r1cs_to_qap::LibsnarkReduction, Groth16};
    use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem};
    use ark_snark::SNARK;
    use ark_std::UniformRand;
    use std::error::Error;

    use super::PaymentProof;
    use rand;

    pub fn generate_new_payment(
        index: usize,
        tree_height: u8,
    ) -> Result<PaymentProof, Box<dyn Error>> {
        let mut rng = rand::thread_rng();

        let rate = 4;
        let optimized_for_weights = true;
        let params = get_default_poseidon_parameters(rate, optimized_for_weights)
            .expect("failed to build params for poseidon");

        let num_leaves: usize = 2usize.pow(tree_height as u32);
        let range = 0..num_leaves;

        assert!(range.contains(&index));

        let coin = Coin::rand(&mut rng);
        let commitment =
            commitment::new_commitment(&params, &coin).expect("failed to create commitment");
        let mut leaves: Vec<Vec<BlsFr>> = range
            .map(|_| {
                let com = commitment::rand(&params, &mut rng)
                    .expect("failed to create poseidon commitment");

                vec![com]
            })
            .collect();

        leaves[index] = vec![commitment];

        let params = get_default_poseidon_parameters(rate, optimized_for_weights)
            .expect("failed to build params for poseidon");

        let tree = MerkleTree::new(&params, &params, leaves)?;
        let root: Root = tree.root();
        let path: TreePath = tree.generate_proof(index)?;

        let sk = Key::rand(&mut rng);
        let serial_no = CoinID::rand(&mut rng);

        Ok(PaymentProof {
            leaf_crh_params: params.clone(),
            two_to_one_crh_params: params,
            root,
            commitment,
            path,
            coin,
            sk,
            serial_no,
        })
    }

    #[test]
    fn cs_sat() -> Result<(), Box<dyn Error>> {
        let index = 1;
        let height = 5; // 32 leaves

        let proof = generate_new_payment(index, height)?;

        let cs = ConstraintSystem::new_ref();
        proof.generate_constraints(cs.clone()).unwrap();

        assert!(cs.is_satisfied()?);

        Ok(())
    }

    #[test]
    /// Check that a false proof is invalid
    fn proof_sound() -> Result<(), Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let index = 1;
        let height = 5; // 32 leaves

        let rate = 4;
        let optimized_for_weights = true;
        let params = get_default_poseidon_parameters(rate, optimized_for_weights)
            .expect("failed to build params for poseidon");

        let num_leaves: usize = 2usize.pow(height as u32);
        let range = 0..num_leaves;

        assert!(range.contains(&index));

        let coin = Coin::rand(&mut rng);
        let commitment =
            commitment::new_commitment(&params, &coin).expect("failed to create commitment");
        let mut leaves: Vec<Vec<BlsFr>> = range
            .map(|_| {
                let com = commitment::rand(&params, &mut rng)
                    .expect("failed to create poseidon commitment");

                vec![com]
            })
            .collect();

        leaves[index] = vec![commitment];

        let params = get_default_poseidon_parameters(rate, optimized_for_weights)
            .expect("failed to build params for poseidon");

        let tree = MerkleTree::new(&params, &params, leaves)?;
        let root: Root = tree.root();
        let path: TreePath = tree.generate_proof(index + 1)?; // NOTE: wrong index on purpose

        let sk = Key::rand(&mut rng);
        let serial_no = CoinID::rand(&mut rng);

        let payment = PaymentProof {
            leaf_crh_params: params.clone(),
            two_to_one_crh_params: params,
            root,
            commitment,
            path,
            coin,
            sk,
            serial_no,
        };

        let cs = ConstraintSystem::new_ref();
        payment.generate_constraints(cs.clone())?;
        let valid_circuit = cs.is_satisfied()?;

        // should fail due to wrong index
        assert!(!valid_circuit);

        Ok(())
    }

    #[test]
    /// Check that a true proof is valid
    fn proof_complete() -> Result<(), Box<dyn Error>> {
        let mut rng = rand::thread_rng();

        let index = 1;
        let height = 5; // 32 leaves
        let payment = generate_new_payment(index, height)?;
        let (pk, vk) = Groth16::<Bls12_381, LibsnarkReduction>::circuit_specific_setup(
            payment.clone(),
            &mut rng,
        )?;

        let proof = Groth16::<Bls12_381, LibsnarkReduction>::prove(&pk, payment.clone(), &mut rng)?;

        let public_inputs = [payment.root, payment.commitment];
        let is_valid = Groth16::<Bls12_381>::verify(&vk, &public_inputs, &proof)?;

        assert!(is_valid);

        Ok(())
    }
}
