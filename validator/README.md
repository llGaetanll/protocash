# Protocash

## How It Works

Consider a simplified model of the application. In this model, we look at the
state stored by the blockchain. There is one major simplification to our
currency: that you can only send one coin per transactions. That is to say,
there are no denominations of our currency, each and every transaction just
sends a single coin. The reason for this simplification will be explained later.

### The Data

```rs
type TxID = u64; /// A transaction identifier, often called the `serial_number`.
type Addr = u64; /// Some account address on the network.

/// The state of our application
struct State {
    /// This `MerkleTree` stores the complete list of all transaction commitment
    /// as its leaves.
    transactions: MerkleTree<Commitment<Tx>>,

    /// This is a set of all used `serial_number`s. This list is used to keep
    /// track of spent transactions in order to avoid double spends.
    spent_ids: HashSet<TxID>
}

/// A transaction.
struct Tx {
    /// The unique, random identifier of the transaction.
    pub serial_number: TxID,

    /// Who the transaction is destined to.
    addr: Addr,

    /// Noise used when generating the commitment to the transaction.
    nonce: u64
}

/// A transaction commitment. This is the data that our MerkleTree actually
/// stores. This is really just a hash of the transaction which takes as input
///     - The `serial_number`
///     - The `addr`
///     - The `nonce`, for randomness
struct Commitment<Tx> {
    /// This is really the only important piece of data stored by the
    /// commitment.
    pub hash: u64,

    /// This is just to keep track of what this is a commitment to. In this
    /// case, just a transaction.
    _tx: PhantomData<Tx>
}
```

### Making a transaction

When we make a transaction, we need to enforce two guarantees:
1. We can afford to make this transaction.

    Because there are no denominations in our currency, this is equivalent to
    proving that there is an unspent transaction that points to us. (i.e. to our
    address.)
2. We are not double spending transactions.

    If there is a transaction in the blockchain that points to us, we need to
    ensure that we can only use it once.

The way that we enforce these guarantees is by making a zero knowledge
proof that there is a leaf in the MerkleTree which is a transaction that points
to us.

TODO: how to do this.
