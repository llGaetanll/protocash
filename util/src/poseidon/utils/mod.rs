use ark_ff::PrimeField;
use hex::FromHexError;

pub mod poseidon_params;

#[derive(Copy, Clone)]
pub enum Curve {
	Bls381,
	Bn254,
}

pub type Bytes = Vec<u8>;

pub struct PoseidonData {
	pub mds: Vec<Vec<Bytes>>,
	pub rounds: Vec<Bytes>,
	pub full_rounds: u8,
	pub partial_rounds: u8,
	pub width: u8,
	pub exp: i8,
}

impl PoseidonData {
	pub fn new(
		mds: Vec<Vec<Bytes>>,
		rounds: Vec<Bytes>,
		full_rounds: u8,
		partial_rounds: u8,
		width: u8,
		exp: i8,
	) -> Self {
		Self {
			mds,
			rounds,
			full_rounds,
			partial_rounds,
			exp,
			width,
		}
	}
}

pub fn decode_hex(s: &str) -> Result<Bytes, FromHexError> {
	let s = &s[2..];
	hex::decode(s)
}

pub fn parse_vec(arr: Vec<&str>) -> Result<Vec<Bytes>, FromHexError> {
	let mut res = Vec::new();
	for r in arr.iter() {
		res.push(decode_hex(r)?);
	}
	Ok(res)
}

pub fn parse_matrix(mds_entries: Vec<Vec<&str>>) -> Result<Vec<Vec<Bytes>>, FromHexError> {
	let width = mds_entries.len();
	let mut mds = vec![vec![Vec::new(); width]; width];
	for i in 0..width {
		for j in 0..width {
			mds[i][j] = decode_hex(mds_entries[i][j])?;
		}
	}
	Ok(mds)
}

pub fn bytes_vec_to_f<F: PrimeField>(bytes_vec: &Vec<Vec<u8>>) -> Vec<F> {
	bytes_vec
		.iter()
		.map(|x| F::from_be_bytes_mod_order(x))
		.collect()
}

pub fn bytes_matrix_to_f<F: PrimeField>(bytes_matrix: &Vec<Vec<Vec<u8>>>) -> Vec<Vec<F>> {
	bytes_matrix.iter().map(|x| bytes_vec_to_f(x)).collect()
}
