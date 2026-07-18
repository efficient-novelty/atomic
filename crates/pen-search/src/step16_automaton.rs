//! Finite-signature audit of the shipped Step-16 surface.
//!
//! The raw Step-16 telescope product is far too large to materialize.  This
//! module separates two proof obligations that are easy to conflate:
//!
//! 1. an exact bottom-up finite automaton counts the *actual* expression
//!    catalog at each telescope position (`ambient_depth + position`), and
//! 2. exact replay of concrete witnesses certifies SAT for the shipped
//!    evaluator.
//!
//! The present certificate deliberately does **not** claim an exhaustive
//! maximum or an UNSAT proof.  Its completeness flags make that boundary
//! machine-readable.  A later telescope-state quotient can strengthen
//! `exhaustive_telescope_quotient` without changing the witness format.

use crate::enumerate::{EnumerationContext, assess_raw_surface_membership};
use pen_core::canonical::canonical_key_telescope;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::bar::{clears_bar, compute_rho};
use pen_eval::halting::{accepted_canonical_keys, genesis_bar_16, genesis_history};
use pen_eval::minimality::analyze_semantic_minimality;
use pen_eval::nu::compute_native_nu;
use pen_eval::p5_record::{ImportDag, P5ImportAudit, conditional_p5_nu_c};
use pen_type::admissibility::{
    AdmissibilityMode, assess_strict_admissibility, strict_admissibility_for_mode,
};
use pen_type::check::{CheckResult, check_telescope};
use pen_type::connectivity::analyze_connectivity;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const STEP_INDEX: u32 = 16;
const WINDOW_DEPTH: u16 = 2;
const LIB_PREVIOUS: u32 = 14;
const LIB_LATEST: u32 = 15;
const UNARY_CONSTRUCTOR_COUNT: u128 = 7; // Lam + 4 modal + 2 temporal.
const BINARY_CONSTRUCTOR_COUNT: u128 = 3; // App, Pi, Sigma.

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Step16AutomatonConfig {
    pub step_index: u32,
    pub library_size: u32,
    pub ambient_depth: u32,
    pub max_expr_nodes: u8,
    pub min_clause_kappa: u16,
    pub max_clause_kappa: u16,
    pub available_library_refs: Vec<u32>,
    pub max_path_dimension: u32,
    pub include_trunc: bool,
    pub include_modal: bool,
    pub include_temporal: bool,
    pub include_linear_exponential: bool,
    pub historical_anchor_ref: Option<u32>,
    pub late_family_surface: String,
    pub unary_constructor_count: u8,
    pub binary_constructor_count: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AutomatonCompleteness {
    /// The expression counts are exact for the generic Step-16 raw grammar.
    pub raw_expression_surface_exact: bool,
    /// Scope is `ambient_depth + position`, rather than the widest-scope
    /// capacity hint used by `raw_clause_catalog_widths`.
    pub actual_position_scopes_used: bool,
    /// Every reported SAT witness is replayed through every shipped gate.
    pub sat_witness_replay_complete: bool,
    /// False in this version: signature products are not yet closed under
    /// all telescope-level connectivity/minimality transitions.
    pub exhaustive_telescope_quotient: bool,
    /// Consequently the observed maxima are lower bounds, not global maxima.
    pub global_maximum_certified: bool,
    /// Consequently this module must never emit an UNSAT/halt verdict.
    pub unsat_certificate_available: bool,
    pub limitation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Step16Decision {
    /// At least one fully replayed raw, admissible, minimal clearer exists.
    Sat,
    /// Reserved for configurations where the witness basis did not decide
    /// existence.  This is intentionally not an UNSAT verdict.
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExpressionSurfaceRow {
    pub clause_kappa: u16,
    pub position: usize,
    pub scope_size: u32,
    pub max_expr_nodes: u8,
    /// Exact number of raw expressions of sizes `1..=max_expr_nodes`.
    pub raw_expression_count: String,
    /// Exact number surviving the shallow checker.  Raw scope construction
    /// already enforces the ambient-depth bound; the remaining rejection is
    /// a bare `Univ` used as an application argument.
    pub checker_valid_expression_count: String,
    /// Number of finite observation states after bottom-up quotienting.
    pub finite_signature_count: usize,
    pub checker_valid_signature_count: usize,
    /// Exact counts of two dangerous scorer-recognized expression families.
    pub polymorphic_temporal_expression_count: String,
    pub spatial_temporal_expression_count: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct KappaSurfaceCertificate {
    pub clause_kappa: u16,
    pub positions: Vec<ExpressionSurfaceRow>,
    /// Exact product of the real position-specific raw catalogs.
    pub raw_telescope_product: String,
    /// Exact product after the per-expression checker predicate.  This is a
    /// useful coverage bound, not yet a telescope-level quotient count.
    pub checker_valid_telescope_product: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WitnessNu {
    pub nu_g: u32,
    pub nu_c: u32,
    pub nu_h: u32,
    pub total: u32,
    pub rho: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct P5ImportAuditCertificate {
    pub direct_imports: Vec<u32>,
    pub dominant_imports: Vec<u32>,
    pub unique_dominant_import: Option<u32>,
    pub unique_dominant_import_holds: bool,
}

impl From<P5ImportAudit> for P5ImportAuditCertificate {
    fn from(audit: P5ImportAudit) -> Self {
        Self {
            direct_imports: audit.direct_imports,
            dominant_imports: audit.dominant_imports,
            unique_dominant_import: audit.unique_dominant_import,
            unique_dominant_import_holds: audit.unique_dominant_import_holds,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WitnessCertificate {
    pub name: String,
    pub mechanism: String,
    pub telescope: Telescope,
    pub raw_surface_member: bool,
    pub raw_surface_rejections: Vec<String>,
    pub type_correct: bool,
    pub type_error: Option<String>,
    pub admissible: bool,
    pub admissibility_class: String,
    pub admissibility_reason: String,
    pub connected: bool,
    pub references_active_window: bool,
    pub self_contained: bool,
    pub historical_reanchor: bool,
    pub semantically_minimal: bool,
    pub terminal_components: Vec<Vec<usize>>,
    pub detachable_subbundle_count: usize,
    pub bar_clearing_detachable_subbundle_count: usize,
    pub canonically_identified_with_accepted: bool,
    pub nu: WitnessNu,
    pub clears_bar: bool,
    pub p5_import_audit: Option<P5ImportAuditCertificate>,
    pub conditional_p5_nu_c: Option<u32>,
    /// This conjunction is the independently replayable SAT proposition.
    pub certifies_sat: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct KappaObservedMaximum {
    pub clause_kappa: u16,
    /// Maximum only over the fully replayed witness basis.
    pub max_verified_witness_nu: Option<u32>,
    pub max_verified_witness_rho: Option<String>,
    pub max_verified_witness_name: Option<String>,
    pub clearing_witness_names: Vec<String>,
    pub is_global_maximum: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Step16AutomatonCertificate {
    pub schema_version: u32,
    pub config: Step16AutomatonConfig,
    pub completeness: AutomatonCompleteness,
    pub bar_16: String,
    pub surfaces: Vec<KappaSurfaceCertificate>,
    pub witnesses: Vec<WitnessCertificate>,
    pub observed_maxima: Vec<KappaObservedMaximum>,
    pub decision: Step16Decision,
    pub digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CertificateReplay {
    pub valid: bool,
    pub decision: Step16Decision,
    pub errors: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum RootKind {
    Univ,
    Var(u8),
    LibPrevious,
    LibLatest,
    Path,
    Lam,
    App,
    Pi,
    Sigma,
    Flat,
    Sharp,
    Disc,
    Shape,
    Next,
    Eventually,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ExprSignature {
    root: RootKind,
    checker_valid: bool,
    raw_var_mask: u8,
    checker_free_var_mask: u8,
    scc_free_var_mask: u8,
    lib_mask: u8,
    has_temporal: bool,
    next_var: bool,
    next_next_var: bool,
    eventually_var: bool,
    app_eventually_var: bool,
    app_lib_temporal_var: bool,
    modal_wrapping_temporal: bool,
    temporal_wrapping_modal: bool,
    polymorphic_temporal: bool,
    spatial_temporal: bool,
}

impl ExprSignature {
    fn is_var(&self) -> bool {
        matches!(self.root, RootKind::Var(_))
    }

    fn is_lib(&self) -> bool {
        matches!(self.root, RootKind::LibPrevious | RootKind::LibLatest)
    }

    fn is_temporal_root(&self) -> bool {
        matches!(self.root, RootKind::Next | RootKind::Eventually)
    }

    fn is_modal_root(&self) -> bool {
        matches!(
            self.root,
            RootKind::Flat | RootKind::Sharp | RootKind::Disc | RootKind::Shape
        )
    }
}

#[derive(Clone, Debug)]
struct SignatureClass {
    count: u128,
    representative: Expr,
}

type SignatureBucket = BTreeMap<ExprSignature, SignatureClass>;

fn shift_free_var_mask(mask: u8) -> u8 {
    // Bit i records de Bruijn index i. Entering a checker binder discharges
    // index 1 and shifts every larger free index down by one.
    (mask >> 1) & !1
}

fn leaf_bucket(scope_size: u32) -> SignatureBucket {
    let mut bucket = SignatureBucket::new();
    insert_signature(
        &mut bucket,
        ExprSignature {
            root: RootKind::Univ,
            checker_valid: true,
            raw_var_mask: 0,
            checker_free_var_mask: 0,
            scc_free_var_mask: 0,
            lib_mask: 0,
            has_temporal: false,
            next_var: false,
            next_next_var: false,
            eventually_var: false,
            app_eventually_var: false,
            app_lib_temporal_var: false,
            modal_wrapping_temporal: false,
            temporal_wrapping_modal: false,
            polymorphic_temporal: false,
            spatial_temporal: false,
        },
        Expr::Univ,
        1,
    );
    for index in 1..=scope_size {
        let bit = 1u8 << index;
        insert_signature(
            &mut bucket,
            ExprSignature {
                root: RootKind::Var(u8::try_from(index).expect("Step-16 scope fits u8")),
                checker_valid: true,
                raw_var_mask: bit,
                checker_free_var_mask: bit,
                scc_free_var_mask: bit,
                lib_mask: 0,
                has_temporal: false,
                next_var: false,
                next_next_var: false,
                eventually_var: false,
                app_eventually_var: false,
                app_lib_temporal_var: false,
                modal_wrapping_temporal: false,
                temporal_wrapping_modal: false,
                polymorphic_temporal: false,
                spatial_temporal: false,
            },
            Expr::Var(index),
            1,
        );
    }
    for (root, expr, mask) in [
        (RootKind::LibPrevious, Expr::Lib(LIB_PREVIOUS), 1u8),
        (RootKind::LibLatest, Expr::Lib(LIB_LATEST), 2u8),
    ] {
        insert_signature(
            &mut bucket,
            ExprSignature {
                root,
                checker_valid: true,
                raw_var_mask: 0,
                checker_free_var_mask: 0,
                scc_free_var_mask: 0,
                lib_mask: mask,
                has_temporal: false,
                next_var: false,
                next_next_var: false,
                eventually_var: false,
                app_eventually_var: false,
                app_lib_temporal_var: false,
                modal_wrapping_temporal: false,
                temporal_wrapping_modal: false,
                polymorphic_temporal: false,
                spatial_temporal: false,
            },
            expr,
            1,
        );
    }
    insert_signature(
        &mut bucket,
        ExprSignature {
            root: RootKind::Path,
            checker_valid: true,
            raw_var_mask: 0,
            checker_free_var_mask: 0,
            scc_free_var_mask: 0,
            lib_mask: 0,
            has_temporal: false,
            next_var: false,
            next_next_var: false,
            eventually_var: false,
            app_eventually_var: false,
            app_lib_temporal_var: false,
            modal_wrapping_temporal: false,
            temporal_wrapping_modal: false,
            polymorphic_temporal: false,
            spatial_temporal: false,
        },
        Expr::PathCon(1),
        1,
    );
    bucket
}

fn unary_signature(root: RootKind, child: &ExprSignature) -> ExprSignature {
    let checker_mask = shift_free_var_mask(child.checker_free_var_mask);
    let scc_mask = if root == RootKind::Lam {
        shift_free_var_mask(child.scc_free_var_mask)
    } else {
        child.scc_free_var_mask
    };
    let next_var = root == RootKind::Next && child.is_var();
    let eventually_var = root == RootKind::Eventually && child.is_var();
    let next_next_var = root == RootKind::Next && child.next_var;
    let modal_wrapping_temporal = matches!(
        root,
        RootKind::Flat | RootKind::Sharp | RootKind::Disc | RootKind::Shape
    ) && child.is_temporal_root();
    let temporal_wrapping_modal =
        matches!(root, RootKind::Next | RootKind::Eventually) && child.is_modal_root();
    ExprSignature {
        root,
        checker_valid: child.checker_valid,
        raw_var_mask: child.raw_var_mask,
        checker_free_var_mask: checker_mask,
        scc_free_var_mask: scc_mask,
        lib_mask: child.lib_mask,
        has_temporal: child.has_temporal || matches!(root, RootKind::Next | RootKind::Eventually),
        next_var,
        next_next_var,
        eventually_var,
        app_eventually_var: false,
        app_lib_temporal_var: false,
        modal_wrapping_temporal,
        temporal_wrapping_modal,
        polymorphic_temporal: root == RootKind::Lam && child.app_eventually_var,
        spatial_temporal: root == RootKind::Lam && child.app_lib_temporal_var,
    }
}

fn binary_signature(root: RootKind, left: &ExprSignature, right: &ExprSignature) -> ExprSignature {
    let app = root == RootKind::App;
    let binder_right = matches!(root, RootKind::Pi | RootKind::Sigma);
    let checker_right_mask = if binder_right {
        shift_free_var_mask(right.checker_free_var_mask)
    } else {
        right.checker_free_var_mask
    };
    let checker_valid =
        left.checker_valid && right.checker_valid && !(app && right.root == RootKind::Univ);
    let app_eventually_var = app && left.eventually_var;
    let app_lib_temporal_var = app && left.is_lib() && (right.next_var || right.eventually_var);
    let polymorphic_temporal = root == RootKind::Pi
        && ((left.next_next_var && right.next_var) || (left.next_var && right.eventually_var));
    ExprSignature {
        root,
        checker_valid,
        raw_var_mask: left.raw_var_mask | right.raw_var_mask,
        checker_free_var_mask: left.checker_free_var_mask | checker_right_mask,
        // The SCC implementation treats Pi/Sigma children at the same binder
        // depth; preserve that shipped behavior exactly.
        scc_free_var_mask: left.scc_free_var_mask | right.scc_free_var_mask,
        lib_mask: left.lib_mask | right.lib_mask,
        has_temporal: left.has_temporal || right.has_temporal,
        next_var: false,
        next_next_var: false,
        eventually_var: false,
        app_eventually_var,
        app_lib_temporal_var,
        modal_wrapping_temporal: false,
        temporal_wrapping_modal: false,
        polymorphic_temporal,
        spatial_temporal: false,
    }
}

fn unary_expr(root: RootKind, child: Expr) -> Expr {
    let child = Box::new(child);
    match root {
        RootKind::Lam => Expr::Lam(child),
        RootKind::Flat => Expr::Flat(child),
        RootKind::Sharp => Expr::Sharp(child),
        RootKind::Disc => Expr::Disc(child),
        RootKind::Shape => Expr::Shape(child),
        RootKind::Next => Expr::Next(child),
        RootKind::Eventually => Expr::Eventually(child),
        _ => unreachable!("non-unary root"),
    }
}

fn binary_expr(root: RootKind, left: Expr, right: Expr) -> Expr {
    let left = Box::new(left);
    let right = Box::new(right);
    match root {
        RootKind::App => Expr::App(left, right),
        RootKind::Pi => Expr::Pi(left, right),
        RootKind::Sigma => Expr::Sigma(left, right),
        _ => unreachable!("non-binary root"),
    }
}

fn insert_signature(
    bucket: &mut SignatureBucket,
    signature: ExprSignature,
    representative: Expr,
    count: u128,
) {
    bucket
        .entry(signature)
        .and_modify(|class| {
            class.count = class
                .count
                .checked_add(count)
                .expect("finite-signature expression count exceeded u128")
        })
        .or_insert(SignatureClass {
            count,
            representative,
        });
}

fn expression_signature_buckets(scope_size: u32, max_nodes: u8) -> Vec<SignatureBucket> {
    let mut buckets = vec![SignatureBucket::new(); usize::from(max_nodes) + 1];
    if max_nodes == 0 {
        return buckets;
    }
    buckets[1] = leaf_bucket(scope_size);

    const UNARY_ROOTS: [RootKind; 7] = [
        RootKind::Lam,
        RootKind::Flat,
        RootKind::Sharp,
        RootKind::Disc,
        RootKind::Shape,
        RootKind::Next,
        RootKind::Eventually,
    ];
    const BINARY_ROOTS: [RootKind; 3] = [RootKind::App, RootKind::Pi, RootKind::Sigma];

    for nodes in 2..=usize::from(max_nodes) {
        let mut bucket = SignatureBucket::new();
        for (child_sig, child_class) in &buckets[nodes - 1] {
            for root in UNARY_ROOTS {
                insert_signature(
                    &mut bucket,
                    unary_signature(root, child_sig),
                    unary_expr(root, child_class.representative.clone()),
                    child_class.count,
                );
            }
        }
        if nodes >= 3 {
            for left_nodes in 1..=nodes - 2 {
                let right_nodes = nodes - 1 - left_nodes;
                for (left_sig, left_class) in &buckets[left_nodes] {
                    for (right_sig, right_class) in &buckets[right_nodes] {
                        let count = left_class
                            .count
                            .checked_mul(right_class.count)
                            .expect("finite-signature expression product exceeded u128");
                        for root in BINARY_ROOTS {
                            insert_signature(
                                &mut bucket,
                                binary_signature(root, left_sig, right_sig),
                                binary_expr(
                                    root,
                                    left_class.representative.clone(),
                                    right_class.representative.clone(),
                                ),
                                count,
                            );
                        }
                    }
                }
            }
        }
        buckets[nodes] = bucket;
    }
    buckets
}

fn expression_surface_row(
    clause_kappa: u16,
    position: usize,
    scope_size: u32,
    max_nodes: u8,
) -> ExpressionSurfaceRow {
    let buckets = expression_signature_buckets(scope_size, max_nodes);
    let mut raw_count = 0u128;
    let mut valid_count = 0u128;
    let mut signature_count = 0usize;
    let mut valid_signature_count = 0usize;
    let mut polymorphic_count = 0u128;
    let mut spatial_count = 0u128;
    for bucket in buckets.iter().skip(1) {
        signature_count += bucket.len();
        for (signature, class) in bucket {
            raw_count = raw_count
                .checked_add(class.count)
                .expect("raw expression surface count exceeded u128");
            if signature.checker_valid {
                valid_count = valid_count
                    .checked_add(class.count)
                    .expect("checker-valid expression surface count exceeded u128");
                valid_signature_count += 1;
                if signature.polymorphic_temporal {
                    polymorphic_count = polymorphic_count
                        .checked_add(class.count)
                        .expect("polymorphic-temporal expression count exceeded u128");
                }
                if signature.spatial_temporal {
                    spatial_count = spatial_count
                        .checked_add(class.count)
                        .expect("spatial-temporal expression count exceeded u128");
                }
            }
        }
    }
    ExpressionSurfaceRow {
        clause_kappa,
        position,
        scope_size,
        max_expr_nodes: max_nodes,
        raw_expression_count: raw_count.to_string(),
        checker_valid_expression_count: valid_count.to_string(),
        finite_signature_count: signature_count,
        checker_valid_signature_count: valid_signature_count,
        polymorphic_temporal_expression_count: polymorphic_count.to_string(),
        spatial_temporal_expression_count: spatial_count.to_string(),
    }
}

fn parse_count(value: &str) -> u128 {
    value.parse().expect("automaton count must be a u128")
}

fn surface_certificates(config: &Step16AutomatonConfig) -> Vec<KappaSurfaceCertificate> {
    (config.min_clause_kappa..=config.max_clause_kappa)
        .map(|clause_kappa| {
            let positions = (0..usize::from(clause_kappa))
                .map(|position| {
                    expression_surface_row(
                        clause_kappa,
                        position,
                        config.ambient_depth + position as u32,
                        config.max_expr_nodes,
                    )
                })
                .collect::<Vec<_>>();
            let raw_product = positions.iter().fold(1u128, |product, row| {
                product
                    .checked_mul(parse_count(&row.raw_expression_count))
                    .expect("raw telescope surface product exceeded u128")
            });
            let valid_product = positions.iter().fold(1u128, |product, row| {
                product
                    .checked_mul(parse_count(&row.checker_valid_expression_count))
                    .expect("checker-valid telescope surface product exceeded u128")
            });
            KappaSurfaceCertificate {
                clause_kappa,
                positions,
                raw_telescope_product: raw_product.to_string(),
                checker_valid_telescope_product: valid_product.to_string(),
            }
        })
        .collect()
}

fn temporal_polymorphism_witness() -> Telescope {
    let clause = ClauseRec::new(
        ClauseRole::Formation,
        Expr::Pi(
            Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
        ),
    );
    Telescope::new(vec![clause.clone(), clause])
}

fn hit_without_formation_witness() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(1)),
        ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
    ])
}

fn single_import_axiomatic_witness() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
        ),
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
        ),
        ClauseRec::new(
            ClauseRole::Introduction,
            Expr::App(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
        ),
    ])
}

fn incomparable_import_axiomatic_witness() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Pi(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
        ),
        ClauseRec::new(
            ClauseRole::Formation,
            Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
        ),
        ClauseRec::new(
            ClauseRole::Introduction,
            Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Var(1))),
        ),
    ])
}

fn witness_basis() -> Vec<(&'static str, &'static str, Telescope)> {
    vec![
        (
            "hit_no_formation_d1",
            "HIT no-formation branch inherits the shipped +|B| term",
            hit_without_formation_witness(),
        ),
        (
            "temporal_polymorphic_kappa2",
            "two recognized polymorphic temporal eliminators each receive |B|",
            temporal_polymorphism_witness(),
        ),
        (
            "axiomatic_single_l15_kappa3",
            "single-import Axiomatic record inherits nu(L15) under the structural evaluator",
            single_import_axiomatic_witness(),
        ),
        (
            "axiomatic_inheritance_kappa3",
            "the shipped structural evaluator scores incomparable imports {L14,L15}, while the P5 audit records that no unique dominant import exists",
            incomparable_import_axiomatic_witness(),
        ),
    ]
}

fn witness_certificate(name: &str, mechanism: &str, telescope: Telescope) -> WitnessCertificate {
    let (library, history, _) = genesis_history();
    let bar = genesis_bar_16();
    let admissibility = strict_admissibility_for_mode(
        STEP_INDEX,
        WINDOW_DEPTH,
        &library,
        AdmissibilityMode::Guarded,
    );
    let context = EnumerationContext::from_admissibility(&library, admissibility);
    let raw = assess_raw_surface_membership(context, &telescope);
    let check = check_telescope(&library, &telescope);
    let type_correct = check == CheckResult::Ok;
    let decision = assess_strict_admissibility(STEP_INDEX, &library, &telescope, admissibility);
    let connectivity = analyze_connectivity(&library, &telescope);
    let minimality = analyze_semantic_minimality(
        STEP_INDEX,
        bar,
        admissibility,
        &telescope,
        &library,
        &history,
    );
    let native = compute_native_nu(&telescope, &library, &history);
    let kappa = u32::try_from(telescope.kappa()).expect("Step-16 kappa fits u32");
    let rho = compute_rho(native.total, kappa).expect("witness kappa is positive");
    let clears = clears_bar(rho, bar);
    let identified = accepted_canonical_keys().contains(&canonical_key_telescope(&telescope));
    let graph = ImportDag::genesis_prefix(15);
    let p5_import_audit = name
        .contains("axiomatic")
        .then(|| P5ImportAudit::check(&telescope, &graph));
    let conditional_p5 = p5_import_audit
        .as_ref()
        .and_then(|_| conditional_p5_nu_c(&telescope, &graph, &history).ok());
    let semantically_minimal = minimality.is_minimal();
    let connected = connectivity.connected
        && (connectivity.references_active_window
            || connectivity.self_contained
            || connectivity.historical_reanchor);
    let certifies_sat = raw.is_member
        && type_correct
        && decision.is_admitted()
        && connected
        && semantically_minimal
        && !identified
        && clears;

    WitnessCertificate {
        name: name.to_owned(),
        mechanism: mechanism.to_owned(),
        telescope,
        raw_surface_member: raw.is_member,
        raw_surface_rejections: raw.rejection_reasons,
        type_correct,
        type_error: match check {
            CheckResult::Ok => None,
            CheckResult::Err(error) => Some(error.to_string()),
        },
        admissible: decision.is_admitted(),
        admissibility_class: decision.class.as_str().to_owned(),
        admissibility_reason: decision.reason,
        connected,
        references_active_window: connectivity.references_active_window,
        self_contained: connectivity.self_contained,
        historical_reanchor: connectivity.historical_reanchor,
        semantically_minimal,
        terminal_components: minimality.structural.terminal_components,
        detachable_subbundle_count: minimality.structural.detachable_subbundles.len(),
        bar_clearing_detachable_subbundle_count: minimality.admissible_bar_clear_subbundles.len(),
        canonically_identified_with_accepted: identified,
        nu: WitnessNu {
            nu_g: native.nu_g,
            nu_c: native.nu_c,
            nu_h: native.nu_h,
            total: native.total,
            rho: rho.to_string(),
        },
        clears_bar: clears,
        p5_import_audit: p5_import_audit.map(Into::into),
        conditional_p5_nu_c: conditional_p5,
        certifies_sat,
    }
}

fn observed_maxima(
    min_kappa: u16,
    max_kappa: u16,
    witnesses: &[WitnessCertificate],
) -> Vec<KappaObservedMaximum> {
    (min_kappa..=max_kappa)
        .map(|clause_kappa| {
            let candidates = witnesses
                .iter()
                .filter(|witness| witness.telescope.kappa() == usize::from(clause_kappa))
                .collect::<Vec<_>>();
            let best = candidates
                .iter()
                .copied()
                .max_by_key(|witness| witness.nu.total);
            KappaObservedMaximum {
                clause_kappa,
                max_verified_witness_nu: best.map(|witness| witness.nu.total),
                max_verified_witness_rho: best.map(|witness| witness.nu.rho.clone()),
                max_verified_witness_name: best.map(|witness| witness.name.clone()),
                clearing_witness_names: candidates
                    .into_iter()
                    .filter(|witness| witness.certifies_sat)
                    .map(|witness| witness.name.clone())
                    .collect(),
                is_global_maximum: false,
            }
        })
        .collect()
}

fn default_config() -> Step16AutomatonConfig {
    let (library, _, _) = genesis_history();
    let admissibility = strict_admissibility_for_mode(
        STEP_INDEX,
        WINDOW_DEPTH,
        &library,
        AdmissibilityMode::Guarded,
    );
    Step16AutomatonConfig {
        step_index: STEP_INDEX,
        library_size: u32::try_from(library.len()).expect("library size fits u32"),
        ambient_depth: admissibility.ambient_depth,
        max_expr_nodes: admissibility.max_expr_nodes,
        min_clause_kappa: admissibility.min_clause_kappa,
        max_clause_kappa: admissibility.max_clause_kappa,
        available_library_refs: vec![LIB_PREVIOUS, LIB_LATEST],
        max_path_dimension: admissibility.max_path_dimension,
        include_trunc: admissibility.include_trunc,
        include_modal: admissibility.include_modal,
        include_temporal: admissibility.include_temporal,
        include_linear_exponential: admissibility.include_linear_exponential,
        historical_anchor_ref: admissibility.historical_anchor_ref,
        late_family_surface: "none".to_owned(),
        unary_constructor_count: u8::try_from(UNARY_CONSTRUCTOR_COUNT)
            .expect("constructor count fits u8"),
        binary_constructor_count: u8::try_from(BINARY_CONSTRUCTOR_COUNT)
            .expect("constructor count fits u8"),
    }
}

fn certificate_digest(certificate: &Step16AutomatonCertificate) -> String {
    let mut payload = certificate.clone();
    payload.digest.clear();
    let bytes = serde_json::to_vec(&payload).expect("Step-16 certificate must serialize");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn build_certificate() -> Step16AutomatonCertificate {
    let config = default_config();
    let surfaces = surface_certificates(&config);
    let witnesses = witness_basis()
        .into_iter()
        .map(|(name, mechanism, telescope)| witness_certificate(name, mechanism, telescope))
        .collect::<Vec<_>>();
    let observed_maxima =
        observed_maxima(config.min_clause_kappa, config.max_clause_kappa, &witnesses);
    let decision = if witnesses.iter().any(|witness| witness.certifies_sat) {
        Step16Decision::Sat
    } else {
        Step16Decision::Unknown
    };
    let mut certificate = Step16AutomatonCertificate {
        schema_version: 1,
        config,
        completeness: AutomatonCompleteness {
            raw_expression_surface_exact: true,
            actual_position_scopes_used: true,
            sat_witness_replay_complete: true,
            exhaustive_telescope_quotient: false,
            global_maximum_certified: false,
            unsat_certificate_available: false,
            limitation: "The finite expression automaton exactly counts the raw position catalogs, but this version does not multiply observation states through every telescope-level connectivity, canonical-identification, and semantic-minimality transition. It gives a complete SAT proof by exact witness replay, not an exhaustive maximum or UNSAT proof.".to_owned(),
        },
        bar_16: genesis_bar_16().to_string(),
        surfaces,
        witnesses,
        observed_maxima,
        decision,
        digest: String::new(),
    };
    certificate.digest = certificate_digest(&certificate);
    certificate
}

/// Run the finite-signature surface counter and exact Step-16 SAT replay.
pub fn run_step16_automaton() -> Step16AutomatonCertificate {
    build_certificate()
}

/// Serialize a fresh certificate as a stable, human-readable JSON artifact.
///
/// The returned object can be embedded alongside a certified-calculus bound;
/// replay only depends on the `Step16AutomatonCertificate` subobject itself.
pub fn step16_automaton_json_pretty() -> String {
    serde_json::to_string_pretty(&run_step16_automaton())
        .expect("Step-16 automaton certificate must serialize as JSON")
}

/// Replay both the digest and every finite count/witness from definitions.
///
/// This verifier intentionally compares the whole payload, so changing a
/// witness, a count, a completeness flag, or the evaluator result invalidates
/// the certificate.
pub fn replay_step16_certificate(certificate: &Step16AutomatonCertificate) -> CertificateReplay {
    let mut errors = Vec::new();
    let digest = certificate_digest(certificate);
    if digest != certificate.digest {
        errors.push(format!(
            "digest mismatch: certificate has {}, replay computed {digest}",
            certificate.digest
        ));
    }
    let expected = build_certificate();
    if *certificate != expected {
        errors.push("certificate payload differs from a fresh exact replay".to_owned());
    }
    if matches!(certificate.decision, Step16Decision::Sat)
        && !certificate
            .witnesses
            .iter()
            .any(|witness| witness.certifies_sat)
    {
        errors.push("SAT decision has no fully replayed witness".to_owned());
    }
    CertificateReplay {
        valid: errors.is_empty(),
        decision: certificate.decision.clone(),
        errors,
    }
}

/// Parse and replay a certificate carried in a standalone or extracted JSON
/// artifact.  Malformed input is an explicit invalid/unknown replay, never an
/// UNSAT result.
pub fn replay_step16_certificate_json(json: &str) -> CertificateReplay {
    match serde_json::from_str::<Step16AutomatonCertificate>(json) {
        Ok(certificate) => replay_step16_certificate(&certificate),
        Err(error) => CertificateReplay {
            valid: false,
            decision: Step16Decision::Unknown,
            errors: vec![format!("invalid Step-16 certificate JSON: {error}")],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Step16Decision, expression_signature_buckets, expression_surface_row,
        replay_step16_certificate, replay_step16_certificate_json, run_step16_automaton,
        step16_automaton_json_pretty,
    };
    use crate::enumerate::{EnumerationContext, LateFamilySurface, enumerate_exprs};
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::telescope::Telescope;
    use pen_eval::halting::genesis_history;
    use pen_type::check::{CheckResult, check_telescope};

    fn small_context(scope_size: u32, max_expr_nodes: u8) -> EnumerationContext {
        EnumerationContext {
            library_size: 15,
            scope_size,
            max_path_dimension: 1,
            include_trunc: false,
            include_modal: true,
            include_temporal: true,
            include_linear_exponential: false,
            max_expr_nodes,
            require_former_eliminator_clauses: false,
            require_initial_hit_clauses: false,
            require_truncation_hit_clauses: false,
            require_higher_hit_clauses: false,
            require_sphere_lift_clauses: false,
            require_axiomatic_bundle_clauses: false,
            require_modal_shell_clauses: false,
            require_connection_shell_clauses: false,
            require_curvature_shell_clauses: false,
            require_operator_bundle_clauses: false,
            require_hilbert_functional_clauses: false,
            require_temporal_shell_clauses: false,
            historical_anchor_ref: None,
            late_family_surface: LateFamilySurface::None,
        }
    }

    #[test]
    fn small_cap_signature_automaton_bisimulates_raw_enumerator_and_checker() {
        let (library, _, _) = genesis_history();
        let context = small_context(2, 4);
        let concrete = enumerate_exprs(context);
        let row = expression_surface_row(2, 0, 2, 4);
        assert_eq!(
            row.raw_expression_count.parse::<usize>().unwrap(),
            concrete.len()
        );

        let concrete_valid = concrete
            .into_iter()
            .filter(|expr| {
                let telescope =
                    Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, expr.clone())]);
                check_telescope(&library, &telescope) == CheckResult::Ok
            })
            .count();
        assert_eq!(
            row.checker_valid_expression_count.parse::<usize>().unwrap(),
            concrete_valid
        );
    }

    #[test]
    fn actual_position_scope_changes_exact_surface_counts() {
        let first = expression_surface_row(4, 0, 2, 4);
        let terminal = expression_surface_row(4, 3, 5, 4);
        assert!(
            first.raw_expression_count.parse::<u128>().unwrap()
                < terminal.raw_expression_count.parse::<u128>().unwrap()
        );
        assert_ne!(
            first.finite_signature_count,
            terminal.finite_signature_count
        );
    }

    #[test]
    fn signature_buckets_are_a_strict_quotient_at_small_cap() {
        let context = small_context(2, 4);
        let concrete_count = enumerate_exprs(context).len();
        let signature_count = expression_signature_buckets(2, 4)
            .iter()
            .skip(1)
            .map(|bucket| bucket.len())
            .sum::<usize>();
        assert!(signature_count < concrete_count);
    }

    #[test]
    fn shipped_step16_is_sat_with_four_exact_regression_survivors() {
        let certificate = run_step16_automaton();
        assert_eq!(certificate.decision, Step16Decision::Sat);
        assert!(!certificate.completeness.exhaustive_telescope_quotient);
        assert!(!certificate.completeness.global_maximum_certified);
        assert!(!certificate.completeness.unsat_certificate_available);
        assert_eq!(certificate.witnesses.len(), 4);
        assert_eq!(
            certificate
                .witnesses
                .iter()
                .map(|witness| witness.name.as_str())
                .collect::<Vec<_>>(),
            vec![
                "hit_no_formation_d1",
                "temporal_polymorphic_kappa2",
                "axiomatic_single_l15_kappa3",
                "axiomatic_inheritance_kappa3",
            ]
        );
        assert!(
            certificate
                .witnesses
                .iter()
                .all(|witness| witness.certifies_sat)
        );
        // These are the terminal (widest-scope) raw catalog counts already
        // measured by the production generator.  Matching them at the full
        // six-node cap checks the finite recurrence beyond the small fixture.
        let terminal_widths = certificate
            .surfaces
            .iter()
            .map(|surface| {
                surface
                    .positions
                    .last()
                    .unwrap()
                    .raw_expression_count
                    .parse::<u128>()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(terminal_widths, vec![910_182, 1_207_872, 1_559_142]);

        let hit = certificate
            .witnesses
            .iter()
            .find(|witness| witness.name == "hit_no_formation_d1")
            .unwrap();
        assert_eq!(
            (hit.nu.nu_g, hit.nu.nu_c, hit.nu.nu_h, hit.nu.total),
            (0, 17, 2, 19)
        );

        let temporal = certificate
            .witnesses
            .iter()
            .find(|witness| witness.name == "temporal_polymorphic_kappa2")
            .unwrap();
        assert_eq!(
            (
                temporal.nu.nu_g,
                temporal.nu.nu_c,
                temporal.nu.nu_h,
                temporal.nu.total
            ),
            (0, 32, 0, 32)
        );
        assert_eq!(temporal.terminal_components, vec![vec![0]]);

        let axiomatic = certificate
            .witnesses
            .iter()
            .find(|witness| witness.name == "axiomatic_single_l15_kappa3")
            .unwrap();
        assert_eq!(
            (
                axiomatic.nu.nu_g,
                axiomatic.nu.nu_c,
                axiomatic.nu.nu_h,
                axiomatic.nu.total
            ),
            (1, 106, 0, 107)
        );
        assert_eq!(axiomatic.conditional_p5_nu_c, Some(106));
        assert_eq!(
            axiomatic
                .p5_import_audit
                .as_ref()
                .and_then(|audit| audit.unique_dominant_import),
            Some(15)
        );

        let incomparable = certificate
            .witnesses
            .iter()
            .find(|witness| witness.name == "axiomatic_inheritance_kappa3")
            .unwrap();
        assert_eq!(
            (
                incomparable.nu.nu_g,
                incomparable.nu.nu_c,
                incomparable.nu.nu_h,
                incomparable.nu.total
            ),
            (1, 107, 0, 108)
        );
        assert_eq!(incomparable.conditional_p5_nu_c, None);
        let audit = incomparable.p5_import_audit.as_ref().unwrap();
        assert_eq!(audit.direct_imports, vec![14, 15]);
        assert!(audit.dominant_imports.is_empty());
        assert_eq!(audit.unique_dominant_import, None);
        assert!(!audit.unique_dominant_import_holds);

        let kappa_three_maximum = certificate
            .observed_maxima
            .iter()
            .find(|maximum| maximum.clause_kappa == 3)
            .unwrap();
        assert_eq!(kappa_three_maximum.max_verified_witness_nu, Some(108));
        assert_eq!(
            kappa_three_maximum.max_verified_witness_name.as_deref(),
            Some("axiomatic_inheritance_kappa3")
        );
        assert!(!kappa_three_maximum.is_global_maximum);
    }

    #[test]
    fn certificate_replay_rejects_mutation() {
        let certificate = run_step16_automaton();
        let replay = replay_step16_certificate(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);

        let mut mutated = certificate;
        mutated.witnesses[0].nu.total += 1;
        let replay = replay_step16_certificate(&mutated);
        assert!(!replay.valid);
        assert!(!replay.errors.is_empty());
    }

    #[test]
    fn json_artifact_round_trips_and_malformed_json_is_unknown() {
        let json = step16_automaton_json_pretty();
        let replay = replay_step16_certificate_json(&json);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(replay.decision, Step16Decision::Sat);

        let malformed = replay_step16_certificate_json("{not a certificate}");
        assert!(!malformed.valid);
        assert_eq!(malformed.decision, Step16Decision::Unknown);
        assert_eq!(malformed.errors.len(), 1);
    }
}
