# Protocash

Protocash is a [layer 1](https://www.investopedia.com/what-are-layer-1-and-layer-2-blockchain-scaling-solutions-7104877)
cryptocurrency running on top of [cometbft](https://cometbft.com/) with a focus on privacy and speed. All transactions on protocash are done in
[zero-knowledge](https://en.wikipedia.org/wiki/Zero-knowledge_proof) using [`arkworks`](https://arkworks.rs/).

## Project Structure

In this repo you will find three crates.
- `node`: A binary used by nodes on the network to validate
   transactions.
- `client`: A binary used by clients to transact to other clients.
  The transactions are checked by nodes running `node`.
- `util`: A library of shared utilities between `node` and `client`.

## Getting Started
(you may want to check that rust is up to date.)

In two separate shells,

1. Start a node

  From the root of the project, run

  ```
  cargo run --release -p node
  ```

2. Start a client

  From the root of the project, run

  ```
  cargo run --release -p client
  ```

You should see the `client` make a connection to the `node` over the ABCI.

## Running Tests

`util` contains various tests for payment proofs.

```
cargo test --release
```

## How It Works

Currently, protocash has no denominations. In other words, any single
transaction just send *one* coin.

### The Data

This is a simplified shape of the data stored by all validators on the blockchain.

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
