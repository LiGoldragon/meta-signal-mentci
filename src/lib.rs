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

/// The authored Ethos source of this contract.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`ETHOS`].
pub const ETHOS_RUST: &str = include_str!("generated/signal.rs");
