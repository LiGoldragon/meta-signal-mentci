//! Meta Signal contract for Mentci: configuration only.
//!
//! Which sockets the daemon seats, which persona identity it speaks as, and
//! where it sends notifications. Everything Mentci presents to a human lives
//! in the ordinary contract, `signal-mentci`.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against it. The
//! portable rkyv frame, its kinds, the wire framing, and the cross-component
//! taxonomy come from `signal`.

pub mod generated;
pub use generated::signal::*;

/// The portable rkyv frame and its three kinds, re-exported from `signal` so
/// a frame of this contract is the same Rust type as every other contract's
/// frame and a consumer need not name `signal` itself to speak this contract.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

/// The cross-component taxonomy this contract's types are written in,
/// re-exported from `signal` for the same reason. These are `signal`'s own
/// types, not copies of them.
pub use signal::{ComponentKind, StandardSocket};

/// The authored Ethos source of this contract.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`ETHOS`].
pub const ETHOS_RUST: &str = include_str!("generated/signal.rs");
