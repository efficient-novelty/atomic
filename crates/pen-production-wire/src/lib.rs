//! Manual canonical cross-language wire for Law V2 production refinement.
//!
//! Decoding is authority-free. It recognizes a closed byte grammar and
//! enforces structural invariants; it does not mint any verified capability.

mod codec;
mod error;
mod model;
mod validate;

pub use codec::{canonical_reencode_bundle_v1, decode_bundle_v1, encode_bundle_v1};
pub use error::{BundleValidationErrorV1, TermRoleV1, WireErrorV1};
pub use model::*;
pub use validate::validate_bundle_v1;
