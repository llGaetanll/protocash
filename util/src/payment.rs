use ark_bls12_381::Fr as BlsFr;
use ark_crypto_primitives::crh::CRHSchemeGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::{alloc::AllocVar, boolean::Boolean, eq::EqGadget};
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_relations::r1cs::{ConstraintSystemRef, Result};

use crate::merkletree::{
    Params, ParamsVar, Root, RootVar, TreePath, TreePathVar,
};
use crate::poseidon::{BlsPoseidonGadget, CoinCommitment, CoinCommitmentVar};
use crate::types::{Coin, CoinID};
use crate::user::User;

#[derive(Clone)]
pub struct PaymentProof {
    params: Params,

    // Public Inputs
    /// The root of the Merkle Tree
    pub root: Root,

    /// The leaf corresponding to the Coin Commitment belonging to the user.
    pub commitment: CoinCommitment,

    // Private Witnesses
    /// The path down the `MerkleTree` which leads to `leaf`.
    pub path: TreePath,

    /// The `Coin` we expect to match the commitment in the `MerkleTree`.
    pub coin: Coin,

    /// The [`User`] who the coin belongs to.
    pub user: User,

    /// The serial number to be revealed by the user. We prove that `serial_no = prf(sk, pre_serial_no)`.
    pub serial_number: CoinID,
}

impl ConstraintSynthesizer<BlsFr> for PaymentProof {
    fn generate_constraints(self, cs: ConstraintSystemRef<BlsFr>) -> Result<()> {
        // public inputs
        let root = RootVar::new_input(ark_relations::ns!(cs, "merkle_root"), || Ok(self.root))?;
        let leaf = CoinCommitmentVar::new_input(ark_relations::ns!(cs, "merkle_leaf"), || {
            Ok(self.commitment)
        })?;

        // private witnesses

        // A private witness of the path down the MerkleTree which leads to the commitment.
        let path =
            TreePathVar::new_witness(ark_relations::ns!(cs, "merkle_tree_path"), || Ok(self.path))?;
        let pk = FpVar::new_witness(ark_relations::ns!(cs, "pub_key"), || Ok(self.coin.pk))?;
        let serial_number = FpVar::new_witness(ark_relations::ns!(cs, "serial_number"), || {
            Ok(&self.serial_number)
        })?;
        let pre_serial_number =
            FpVar::new_witness(ark_relations::ns!(cs, "pre_serial_number"), || {
                Ok(&self.coin.pre_serial_number)
            })?;
        let com_rnd =
            FpVar::new_witness(ark_relations::ns!(cs, "com_rnd"), || Ok(&self.coin.com_rnd))?;

        let sk = FpVar::new_witness(ark_relations::ns!(cs, "sec_key"), || Ok(&self.user.sk))?;
        let noise =
            FpVar::new_witness(ark_relations::ns!(cs, "key_noise"), || Ok(&self.user.noise))?;

        let params =
            ParamsVar::new_constant(ark_relations::ns!(cs, "poseidon_params"), self.params)?;

        // 1. We prove that we have a path down the MerkleTree that leads to a commitment which
        //    hashes to:
        //    - pk
        //    - pre_serial_no
        //    - com_rnd
        let is_member = path.verify_membership(&params, &params, &root, &[leaf.clone()])?;
        is_member.enforce_equal(&Boolean::TRUE)?;

        // Of course, the commitment that we point to in the tree has to be made of the things we
        // claim it is.
        let expected_commitment_hash = BlsPoseidonGadget::evaluate(
            &params,
            &[pk.clone(), pre_serial_number.clone(), com_rnd],
        )?;
        expected_commitment_hash.enforce_equal(&leaf)?;

        // 2. We enforce that `serial_number = prf(sk, pre_serial_number)`, so that the the payer can't lie
        //    to the payee
        //
        //    In this case, prf = Poseidon on `sk` and `pre_serial_number`.
        let expected_serial_number =
            BlsPoseidonGadget::evaluate(&params, &[sk.clone(), pre_serial_number])?;
        expected_serial_number.enforce_equal(&serial_number)?;

        // 3. We prove that `pk = H(sk)`
        let expected_pk = BlsPoseidonGadget::evaluate(&params, &[sk, noise])?;
        expected_pk.enforce_equal(&pk)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        merkletree::{MerkleTree, Root, TreePath},
        poseidon::{commitment, get_default_poseidon_parameters, BlsPoseidon},
        types::Coin,
        user::User,
    };
    use ark_bls12_381::{Bls12_381, Fr as BlsFr};
    use ark_crypto_primitives::crh::CRHScheme;
    use ark_groth16::{r1cs_to_qap::LibsnarkReduction, Groth16};
    use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem};
    use ark_snark::SNARK;
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

        let user = User::new(&params, &mut rng)?;

        let mut coin = Coin::rand(&mut rng);
        coin.pk = user.pk;

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

        let tree = MerkleTree::new(&params, &params, leaves)?;
        let root: Root = tree.root();
        let path: TreePath = tree.generate_proof(index)?;

        let serial_number =
            BlsPoseidon::evaluate(&params, [user.sk, coin.pre_serial_number])?;

        Ok(PaymentProof {
            params,
            root,
            commitment,
            path,
            coin,
            user,
            serial_number,
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

        let user = User::new(&params, &mut rng)?;
        let serial_number =
            BlsPoseidon::evaluate(&params, [user.sk, coin.pre_serial_number])?;

        let payment = PaymentProof {
            params,
            root,
            commitment,
            path,
            coin,
            user,
            serial_number,
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

    #[test]
    /// Check that the proof size is not too big
    fn proof_size() -> Result<(), Box<dyn Error>> {
        let index = 3;
        let height = 14;

        let proof = generate_new_payment(index, height)?;

        let cs = ConstraintSystem::new_ref();
        proof.generate_constraints(cs.clone()).unwrap();

        let n = cs.num_constraints();
        println!("{n}");

        // n = ~8000 for these inputs
        assert!(n < 30_000);

        Ok(())
    }
}
