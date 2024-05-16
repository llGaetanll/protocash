//! This library provides utilities shared by both the nodes and the clients.
pub mod payment;
pub mod types;
pub mod user;
pub mod util;

pub mod poseidon;
pub mod poseidon_native;

// pub use poseidon::merkletree;
pub use poseidon_native::merkletree;
