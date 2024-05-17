# Protocash

## Note to the grader

(Temporary section. Will be removed once project is graded)

I encourage you to check out the commit history for context on the work put in!
The repo at this time may not look like a lot, but many failed attempts at a
proof were tried before we got here. At its peak, this repo contained ~`8000`
SLOC. [This
commit](https://github.com/llGaetanll/protocash/tree/46485afc2209190d7b86d68d4b2bf2066460ea98)
is a good example.

Of course, this is not counting [the abci bridge](https://github.com/llGaetanll/tower-abci), in which went a lot of trial
and error, or [the several arkworks
examples](https://github.com/llGaetanll/arkworks-tests) which helped me gain a
foundation into the framework.

Hard work aside, working on this project is really enjoyable! I will
continue to do so, at least for the forseeable future.


## Project Structure

In this repo you will find three crates.
- `protocash-node`: A binary used by nodes on the network to validate
   transactions.
- `protocash-client`: A binary used by clients to transact to other clients.
  The transactions are checked by nodes running `protocash-node`.
- `protocash-util`: A library of shared utilities between `protocash-node` and
  `protocash-client`.

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

Then, `A` needs to provide a zero knowledge proof that

1. There exists a commitment `c` in the Merkle Tree which opens to
      - `pk_A`, `A`'s public key.
      - `pre_serial_no`
      - `com_rnd`
   
   This is the coin that `A` is spending.

2. The serial number `sn = prf(sk_A, pre_serial_no)` AND `pk_A = H(sk_A)`, where
   `sk_A` is `A`'s secret key, and `prf` is just some pseudo-random function
   which, in this case, is just a hash function.

   At this point, there are a few things to note
   - Notice here that only `A` can make this proof because only `A` knows `sk_A`.
   - It's also important to note that `sn` is *not* stored in the commitment here.
     In fact, `sn` is not stored anywhere, it is just used in the proof that `A`
     provides when making a transaction.

If `A` can make this proof, a new commitment `c' = H(pk_B, pre_serial_no',
com_rnd')` is added by the validators, to the Merkle Tree, where
- `pre_serial_no' = H(sn)`
- `cmd_rnd'` is random.

Both `pre_serial_no'` and `cmr_rnd'` are yielded back to `A`. Then, `A` is
reponsible for messaging both `cmd_rnd'` and `pre_serial_no'` privately to `B`,
in order for `B` to use this transaction.

In order for `B` to find this coin in the Merkle Tree, they can simply compute
the commitment from the information sent over by `A`.

##### Inputs to the Proof

Public inputs are:
- `r`: The Merkle Tree root
- `c`: The coin commitment owned by `A`
- `s`: The serial number

<!-- ##### Questions -->
<!---->
<!-- 1. Assuming that users keep their `pre_serial_number`s private, what's the point -->
<!--    of `com_rnd`? People still can't open commitments unless they know *both* the -->
<!--    public key and the `pre_serial_number` associated with the commitment. -->
<!---->
<!-- 2. If `A` pays `B` via a commitment `c'`, I now completely understand how *both* -->
<!--    `A` and `B` are able to open `c'`, but how *only* `B` can spend it.  -->
<!---->
<!--    My issue now is with `A` tracking when `B` spends `c'`. When `B` wants to spend -->
<!--    `c'`, it makes a proof that it knows `c'` and that `pk_B = H(sk_B)`. So far so -->
<!--    good. The problem now is that `B` needs to reveal a serial_number to avoid -->
<!--    the double spending problem. -->
<!---->
<!--    - If `B` reveals `pre_serial_number'`, then of course `A` knows it, and so `A` -->
<!--      can track `B`. -->
<!---->
<!--    - If `B` reveals `sn' = prf(sk_B, pre_serial_number')`, I now worry that, if `B` -->
<!--      has evil intentions, it could lie and reveal a made-up serial number `sn'` -->
<!--      everytime, and so re-use `c'` over and over again. After all, `B` is the only -->
<!--      one to know `sk_B`, so it could pass off `sn_i = prf(sk_B, pre_serial_number_i)` -->
<!--      as just a different and made-up `pre_serial_number_i` everytime. -->
