mod arkworks;

/// This is the type that we associate with transaction hashes. It can change in the future.
pub type TxHash = u64;

/// An account address. Type can change in the future.
/// TODO: this type may have to be the same as the Addr type used by cometbft
pub type Addr = u64;

pub struct Tx {
    /// The hash of the tx that this tx relies on
    pub prev_tx: TxHash,

    // for now, the amount is always 1

    // the address of the person we send the transaction to
    pub to: Addr,

    /// the id associated with the transaction. This is a random number.
    pub serial_number: TxHash
}

pub struct TxRequest {
    /// The hash of the tx that this tx relies on
    pub prev_tx: TxHash,

    // the address of the person we send the transaction to
    pub to: Addr,
}
