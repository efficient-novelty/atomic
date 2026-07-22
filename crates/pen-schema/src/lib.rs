//! Proof-facing foundations for the intended depth-two schema grammar.
//!
//! The crate now contains the scoped E-1 context/substitution judgement, the
//! historical-reference E-2 grammar, and fail-closed E-3/E-4 development
//! fragments for typed normalization, frozen equality, and naturality
//! generators.  Global `Schema2(W)` completeness, classification, scoring,
//! and acceptance remain outside the proved boundary; partial-phase modules
//! cannot issue an independent-family verdict or a halt judgement.

pub mod certificate;
pub mod context;
pub mod e2_certificate;
pub mod e34_certificate;
pub mod e34_class_certificate;
pub mod e34_class_induction;
pub mod e34_m1_sweep;
pub mod e3_normalization;
pub mod e4_generator_basis;
pub mod g1_vocabulary_survey;
pub mod grammar;
pub mod grammar_completion;
pub mod internal_classifier_branch;
pub mod internal_classifier_branch_v10;
pub mod internal_classifier_branch_v2;
pub mod internal_classifier_branch_v3;
pub mod internal_classifier_branch_v4;
pub mod internal_classifier_branch_v5;
pub mod internal_classifier_branch_v6;
pub mod internal_classifier_branch_v7;
pub mod internal_classifier_branch_v8;
pub mod internal_classifier_branch_v9;
pub mod motive_parametric_coherence_certificate;
pub mod ordinary;
pub mod stage1_r1;
pub mod step8_r2;
pub mod total_classifier;
pub mod trunc_regression;

pub use certificate::*;
pub use context::*;
pub use e2_certificate::*;
pub use e3_normalization::*;
pub use e4_generator_basis::*;
pub use e34_certificate::*;
pub use e34_class_certificate::*;
pub use e34_class_induction::*;
pub use e34_m1_sweep::*;
pub use g1_vocabulary_survey::*;
pub use grammar::*;
pub use grammar_completion::*;
pub use ordinary::*;
pub use stage1_r1::*;
pub use step8_r2::*;
pub use trunc_regression::*;
