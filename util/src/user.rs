use ark_crypto_primitives::{
    crh::sha256::Sha256,
    signature::{
        schnorr::{Parameters, PublicKey, Schnorr, SecretKey},
        SignatureScheme,
    },
};
use ark_ec::CurveGroup;
use ark_ff::fields::PrimeField;
use ark_std::ops::Mul;

#[derive(Debug)]
pub struct User<C: CurveGroup> {
    params: Parameters<C, Sha256>,
    pub pk: PublicKey<C>,
    pub sk: SecretKey<C>,
}

impl<C> Default for User<C>
where
    C: CurveGroup,
    C::ScalarField: PrimeField,
{
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        let params = Schnorr::<C, Sha256>::setup(&mut rng).unwrap();
        let (pk, sk) = Schnorr::keygen(&params, &mut rng).unwrap();

        Self { params, pk, sk }
    }
}

fn check_pub_key<C: CurveGroup>(user: User<C>) -> bool {
    let params = user.params;
    let pk = params.generator.mul(user.sk.0).into();
    user.pk == pk
}

#[cfg(test)]
mod test {
    use crate::user::{check_pub_key, User};
    use ark_ec::models::twisted_edwards::Projective as TEProjective;
    use ark_ed_on_bls12_381::JubjubConfig;
    use ark_relations::r1cs::Result;

    #[test]
    fn test_schnorr() -> Result<()> {
        let user = User::<TEProjective<JubjubConfig>>::default();

        assert!(check_pub_key(user));

        Ok(())
    }
}
