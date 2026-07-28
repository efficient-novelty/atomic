//! Oracle-free executable contract for the law-v2 lane.
//!
//! This crate deliberately contains no constructor basis, historical trace,
//! numerical selector, or presentation-order preference. It defines the
//! register discipline and fixed memory parameter. The proof-relevant state
//! machine is intentionally deferred until its evidence can be replayed by a
//! real deterministic verifier.

#![forbid(unsafe_code)]

mod certificates;
mod contract_status;
mod register;
mod registered_bootstrap;

pub use certificates::*;
pub use contract_status::*;
pub use pen_kernel::{Digest, DigestError};
pub use register::{LegacyStructuralValue, SemanticFamilyValue, ValueRegister};
pub use registered_bootstrap::*;

/// Phase-3a census complete only relative to an explicit finite registry.
pub mod relative_census {
    pub use pen_demand::*;
}

/// The only visible memory parameter admitted by the v2 executable contract.
pub const DEMAND_WINDOW_WIDTH: u8 = 2;
