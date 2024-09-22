use ark_crypto_primitives::sponge::poseidon::PoseidonConfig;
use ark_crypto_primitives::sponge::poseidon::PoseidonDefaultConfigEntry;
use ark_ff::fields::models::*;
use ark_ff::BigInteger;
use ark_ff::PrimeField;

const PARAMS_OPT_FOR_CONSTRAINTS: [PoseidonDefaultConfigEntry; 7] = [
    PoseidonDefaultConfigEntry::new(2, 17, 8, 31, 0),
    PoseidonDefaultConfigEntry::new(3, 5, 8, 56, 0),
    PoseidonDefaultConfigEntry::new(4, 5, 8, 56, 0),
    PoseidonDefaultConfigEntry::new(5, 5, 8, 57, 0),
    PoseidonDefaultConfigEntry::new(6, 5, 8, 57, 0),
    PoseidonDefaultConfigEntry::new(7, 5, 8, 57, 0),
    PoseidonDefaultConfigEntry::new(8, 5, 8, 57, 0),
];

const PARAMS_OPT_FOR_WEIGHTS: [PoseidonDefaultConfigEntry; 7] = [
    PoseidonDefaultConfigEntry::new(2, 257, 8, 13, 0),
    PoseidonDefaultConfigEntry::new(3, 257, 8, 13, 0),
    PoseidonDefaultConfigEntry::new(4, 257, 8, 13, 0),
    PoseidonDefaultConfigEntry::new(5, 257, 8, 13, 0),
    PoseidonDefaultConfigEntry::new(6, 257, 8, 13, 0),
    PoseidonDefaultConfigEntry::new(7, 257, 8, 13, 0),
    PoseidonDefaultConfigEntry::new(8, 257, 8, 13, 0),
];

struct PoseidonGrainLFSR {
    pub prime_num_bits: u64,

    pub state: [bool; 80],
    pub head: usize,
}

impl PoseidonGrainLFSR {
    pub fn new(
        is_sbox_an_inverse: bool,
        prime_num_bits: u64,
        state_len: u64,
        num_full_rounds: u64,
        num_partial_rounds: u64,
    ) -> Self {
        let mut state = [false; 80];

        // b0, b1 describes the field
        state[1] = true;

        // b2, ..., b5 describes the S-BOX
        state[5] = is_sbox_an_inverse;

        // b6, ..., b17 are the binary representation of n (prime_num_bits)
        {
            let mut cur = prime_num_bits;
            for i in (6..=17).rev() {
                state[i] = cur & 1 == 1;
                cur >>= 1;
            }
        }

        // b18, ..., b29 are the binary representation of t (state_len, rate + capacity)
        {
            let mut cur = state_len;
            for i in (18..=29).rev() {
                state[i] = cur & 1 == 1;
                cur >>= 1;
            }
        }

        // b30, ..., b39 are the binary representation of R_F (the number of full rounds)
        {
            let mut cur = num_full_rounds;
            for i in (30..=39).rev() {
                state[i] = cur & 1 == 1;
                cur >>= 1;
            }
        }

        // b40, ..., b49 are the binary representation of R_P (the number of partial rounds)
        {
            let mut cur = num_partial_rounds;
            for i in (40..=49).rev() {
                state[i] = cur & 1 == 1;
                cur >>= 1;
            }
        }

        // b50, ..., b79 are set to 1
        for i in 50..=79 {
            state[i] = true;
        }

        let head = 0;

        let mut res = Self {
            prime_num_bits,
            state,
            head,
        };
        res.init();
        res
    }

    pub fn get_bits(&mut self, num_bits: usize) -> Vec<bool> {
        let mut res = Vec::new();

        for _ in 0..num_bits {
            // Obtain the first bit
            let mut new_bit = self.update();

            // Loop until the first bit is true
            while !new_bit {
                // Discard the second bit
                let _ = self.update();
                // Obtain another first bit
                new_bit = self.update();
            }

            // Obtain the second bit
            res.push(self.update());
        }

        res
    }

    pub fn get_field_elements_rejection_sampling<F: PrimeField>(
        &mut self,
        num_elems: usize,
    ) -> Vec<F> {
        assert_eq!(F::MODULUS_BIT_SIZE as u64, self.prime_num_bits);

        let mut res = Vec::new();
        for _ in 0..num_elems {
            // Perform rejection sampling
            loop {
                // Obtain n bits and make it most-significant-bit first
                let mut bits = self.get_bits(self.prime_num_bits as usize);
                bits.reverse();

                // Construct the number
                let bigint = F::BigInt::from_bits_le(&bits);

                if let Some(f) = F::from_bigint(bigint) {
                    res.push(f);
                    break;
                }
            }
        }

        res
    }

    pub fn get_field_elements_mod_p<F: PrimeField>(&mut self, num_elems: usize) -> Vec<F> {
        assert_eq!(F::MODULUS_BIT_SIZE as u64, self.prime_num_bits);

        let mut res = Vec::new();
        for _ in 0..num_elems {
            // Obtain n bits and make it most-significant-bit first
            let mut bits = self.get_bits(self.prime_num_bits as usize);
            bits.reverse();

            let bytes = bits
                .chunks(8)
                .map(|chunk| {
                    let mut result = 0u8;
                    for (i, bit) in chunk.iter().enumerate() {
                        result |= u8::from(*bit) << i
                    }
                    result
                })
                .collect::<Vec<u8>>();

            res.push(F::from_le_bytes_mod_order(&bytes));
        }

        res
    }

    #[inline]
    fn update(&mut self) -> bool {
        let new_bit = self.state[(self.head + 62) % 80]
            ^ self.state[(self.head + 51) % 80]
            ^ self.state[(self.head + 38) % 80]
            ^ self.state[(self.head + 23) % 80]
            ^ self.state[(self.head + 13) % 80]
            ^ self.state[self.head];
        self.state[self.head] = new_bit;
        self.head += 1;
        self.head %= 80;

        new_bit
    }

    fn init(&mut self) {
        for _ in 0..160 {
            let _ = self.update();
        }
    }
}

/// TODO: generate this statically for static params
/// Uses the `PoseidonDefaultConfig` to compute the Poseidon parameters.
pub fn get_default_poseidon_parameters<P: FpConfig<N>, const N: usize>(
    rate: usize,
    optimized_for_weights: bool,
) -> Option<PoseidonConfig<Fp<P, N>>> {
    let params_set = if !optimized_for_weights {
        PARAMS_OPT_FOR_CONSTRAINTS
    } else {
        PARAMS_OPT_FOR_WEIGHTS
    };

    for param in params_set.iter() {
        if param.rate == rate {
            let (ark, mds) = find_poseidon_ark_and_mds::<Fp<P, N>>(
                Fp::<P, N>::MODULUS_BIT_SIZE as u64,
                rate,
                param.full_rounds as u64,
                param.partial_rounds as u64,
                param.skip_matrices as u64,
            );

            return Some(PoseidonConfig {
                full_rounds: param.full_rounds,
                partial_rounds: param.partial_rounds,
                alpha: param.alpha as u64,
                ark,
                mds,
                rate: param.rate,
                capacity: 1,
            });
        }
    }

    None
}

/// Internal function that computes the ark and mds from the Poseidon Grain LFSR.
fn find_poseidon_ark_and_mds<F: PrimeField>(
    prime_bits: u64,
    rate: usize,
    full_rounds: u64,
    partial_rounds: u64,
    skip_matrices: u64,
) -> (Vec<Vec<F>>, Vec<Vec<F>>) {
    let mut lfsr = PoseidonGrainLFSR::new(
        false,
        prime_bits,
        (rate + 1) as u64,
        full_rounds,
        partial_rounds,
    );

    let mut ark = Vec::<Vec<F>>::with_capacity((full_rounds + partial_rounds) as usize);
    for _ in 0..(full_rounds + partial_rounds) {
        ark.push(lfsr.get_field_elements_rejection_sampling(rate + 1));
    }

    let mut mds = Vec::<Vec<F>>::with_capacity(rate + 1);
    mds.resize(rate + 1, vec![F::zero(); rate + 1]);
    for _ in 0..skip_matrices {
        let _ = lfsr.get_field_elements_mod_p::<F>(2 * (rate + 1));
    }

    // a qualifying matrix must satisfy the following requirements
    // - there is no duplication among the elements in x or y
    // - there is no i and j such that x[i] + y[j] = p
    // - the resultant MDS passes all the three tests

    let xs = lfsr.get_field_elements_mod_p::<F>(rate + 1);
    let ys = lfsr.get_field_elements_mod_p::<F>(rate + 1);

    for i in 0..(rate + 1) {
        for j in 0..(rate + 1) {
            mds[i][j] = (xs[i] + &ys[j]).inverse().unwrap();
        }
    }

    (ark, mds)
}
