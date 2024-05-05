# Protocash

## How It Works

We consider a model with one key simplification: there are no denominations to
the currency. In other words, any and all transactions just send one single
coin.

### The Data

This is the shape of the data stored by all validators on the blockchain.

```rs
type CoinID = u64; /// A coin identifier, often called the `pre_serial_number`.
type PubKey = u64; /// The public key of some user on the network.

/// The state of our application
struct State {
    /// This `MerkleTree` stores coin commitments as its leaves.
    coins: MerkleTree<Commitment<Coin>>,

    /// This is a set of all used `serial_number`s. This list is used to keep
    /// track of spent coins in order to avoid double spends.
    spent_ids: HashSet<CoinID>
}

/// A Coin
struct Coin {
    /// The public key of the owner of this coin.
    key: PubKey,

    /// The unique, random identifier of the coin.
    pre_serial_number: CoinID,

    /// Noise used when generating the coin commitment
    com_rnd: u64
}

/// A coin commitment. This is the data that our MerkleTree actually
/// stores. This is really just a hash of the coin which takes as input
///     - The `key`
///     - The `pre_serial_number`
///     - The `com_rnd`, for randomness
struct Commitment<T> {
    /// This is really the only important piece of data stored by the
    /// commitment.
    pub hash: u64,

    /// This is just to keep track of what this is a commitment to. In this
    /// case, just a coin.
    _tx: PhantomData<T>
}
```

### Making a transaction

#### Guarantees

When we make a transaction, we need to enforce two guarantees:
1. We can afford to make this transaction.

    Because there are no denominations in our currency, this is equivalent to
    proving that there is a coin in the Merkle Tree which belongs to us.

2. We are not double spending transactions.

    If there is a coin in the tree that is ours, we need to ensure that we can
    only use it once.

#### The Process

The process by which we make a transaction is the following. Suppose that `A`
wants to make a payment to `B`.

Then, `A` needs to prove that
1. There exists a commitment `c` in the Merkle Tree which opens to
  - `pk_A`, `A`'s public key.
  - `pre_serial_no`
  - `com_rnd`

  This is the coin that `A` is spending.

2. The serial number `sn = prf(sk_A, pre_serial_no)` AND `pk_A = H(sk_A)`, where `sk_A` is `A`'s secret key.

   Notice here that only `A` can make this proof because only `A` knows `sk_A`.

If `A` can make this proof, a new commitment `c' = H(pk_B, pre_serial_no', com_rnd')` is created, where
- `pre_serial_no' = H(pre_serial_no)`
- `cmd_rnd'` is random.

##### Inputs to the Proof

Public inputs are:
- `r`: The Merkle Tree root
- `c`: The coin commitment paying the receiver
- `s`: The serial number

##### Questions

This section contains questions about parts of this process which are still
unclear (at least to me.)

- Who adds commitments to the Merkle Tree? When are they added?

  Presumably, the new commitment is added by `A` during the transaction process.

- Where is `sn` stored? I understand that the Merkle Tree stores commitments
  which contain only 3 fields: some public key `pk`, a `pre_serial_no`, some 
  `com_rnd`. So is `sn` *not* stored in the commitments? If not, where else?

- What does it mean for `sn` to *equal* `prf(sk_X, pre_serial_no)`? What is
  `prf` computing about its inputs here?

- If this new commitment `c'` is added by `A` for `B` to open and use, that
  means that `B` needs to know both `com_rnd'`, and either `pre_serial_no'` or
  `pre_serial_no`. But `A` chose `com_rnd'`, so somehow, `A` needs to convey
  that information to `B`. Of course `A` *could* make that information public,
  in which case *anyone* could open this commitment. Is this the intended
  approach?

- As user `B` awaiting a transaction from `A`, how do I know which commitment
  `c'` in the Merkle Tree is the one that `B` left for me to open and use? The
  data stored in the Merkle Tree is completely opaque to me.
