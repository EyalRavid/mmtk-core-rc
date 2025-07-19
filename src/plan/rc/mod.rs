//! Plan: rc

pub(super) mod global;
pub(super) mod mutator;
pub mod barrier;
pub use self::global::RC;
pub use self::global::RC_CONSTRAINTS;
