//! ORACLE_ONLY_PROVISIONAL H3/GSC compatibility probe.
//!
//! This executable is outside the Law-V2 production dependency closure.  It
//! does not adopt GSC, create an acceptance fixture, or authorize a candidate.

use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_kernel::{
    Declaration, DependentContext, Digest, GlobalId, Kernel, KernelError, KernelLimits,
    OpenJudgment, Term, UncheckedSignature,
};
use pen_law::load_embedded_registered_bootstrap;
use pen_oracle::reference_telescope;
use pen_type::elaborate::{
    SealedSignature, TokenError, candidate_hash, elaborate_telescope, issue_typed_eliminator_token,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

const SCHEMA: &str = "law-v2-h3-compatibility-probe-v1";
const DATE: &str = "2026-07-28";
const AUTHORITY: &str = "ORACLE_ONLY_PROVISIONAL_NON_NORMATIVE";
const PROJECTION_ID: &str = "oracle-only-h3-one-nullary-dependent-core-projection-v1";

const ROOT_CARGO_TOML_BYTES: &[u8] = include_bytes!("../../../Cargo.toml");
const CARGO_LOCK_BYTES: &[u8] = include_bytes!("../../../Cargo.lock");
const RUST_TOOLCHAIN_BYTES: &[u8] = include_bytes!("../../../rust-toolchain.toml");
const PEN_CORE_CARGO_TOML_BYTES: &[u8] = include_bytes!("../../pen-core/Cargo.toml");
const PEN_CORE_CLAUSE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/clause.rs");
const PEN_CORE_EXPR_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/expr.rs");
const PEN_CORE_HASH_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/hash.rs");
const PEN_CORE_STATS_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/stats.rs");
const PEN_CORE_TELESCOPE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/telescope.rs");
const PEN_KERNEL_CARGO_TOML_BYTES: &[u8] = include_bytes!("../../pen-kernel/Cargo.toml");
const PEN_KERNEL_LIB_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-kernel/src/lib.rs");
const PEN_KERNEL_CHECKER_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-kernel/src/checker.rs");
const PEN_KERNEL_DEPENDENCY_GRAPH_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-kernel/src/dependency_graph.rs");
const PEN_KERNEL_SYNTAX_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-kernel/src/syntax.rs");
const PEN_KERNEL_DIGEST_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-kernel/src/digest.rs");
const PEN_LAW_CARGO_TOML_BYTES: &[u8] = include_bytes!("../../pen-law/Cargo.toml");
const BOOTSTRAP_ASSET_BYTES: &[u8] =
    include_bytes!("../../pen-law/assets/law_v2a_registered_bootstrap_v1.json");
const REGISTERED_BOOTSTRAP_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-law/src/registered_bootstrap.rs");
const PEN_TYPE_CARGO_TOML_BYTES: &[u8] = include_bytes!("../../pen-type/Cargo.toml");
const PEN_TYPE_ELABORATE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");
const PEN_TYPE_EQUALITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/equality.rs");
const PEN_TYPE_NORMALIZE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/normalize.rs");
const PEN_ORACLE_CARGO_TOML_BYTES: &[u8] = include_bytes!("../Cargo.toml");
const PEN_ORACLE_LIB_SOURCE_BYTES: &[u8] = include_bytes!("../src/lib.rs");
const STAGE4_ARCHIVE_BYTES: &[u8] =
    include_bytes!("../../../docs/stage4_semantic_parsimony_v3.json");
const GSC_PROPOSAL_BYTES: &[u8] =
    include_bytes!("../../../docs/LAW_V2_GENESIS_SCHEME_CALCULUS_V1.md");
const AGDA_PROBE_BYTES: &[u8] = include_bytes!("../../../agda/LawV2/H3Compatibility.agda");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("law_v2_h3_compatibility_probe.rs");

const EXPECTED_ARCHIVED_HASHES: [&str; 4] = [
    "blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407",
    "blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308",
    "blake3:4b2211ecae25f3afbb1187f3a4a1b644edfc6adbd3512b9ca27c64a7b731265b",
    "blake3:b4f821d9bb28366d8ae37adc7c1cb3e28c8a0de60f05c81e9ad75ba4f8b0edd4",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Verdict {
    Proven,
    Refuted,
    OutsideFragment,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Disposition {
    verdict: Verdict,
    reason: String,
}

impl Disposition {
    fn proven(reason: impl Into<String>) -> Self {
        Self {
            verdict: Verdict::Proven,
            reason: reason.into(),
        }
    }

    fn refuted(reason: impl Into<String>) -> Self {
        Self {
            verdict: Verdict::Refuted,
            reason: reason.into(),
        }
    }

    fn outside(reason: impl Into<String>) -> Self {
        Self {
            verdict: Verdict::OutsideFragment,
            reason: reason.into(),
        }
    }

    fn unknown(reason: impl Into<String>) -> Self {
        Self {
            verdict: Verdict::Unknown,
            reason: reason.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct SourceBinding {
    path: String,
    role: String,
    hash_basis: String,
    canonical_byte_length: usize,
    blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct BoundaryDeclarationReport {
    slot: String,
    id: String,
    ty: Term,
    body: Term,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct H3BoundaryReport {
    bootstrap_artifact_digest: String,
    final_boundary_digest: String,
    declarations: Vec<BoundaryDeclarationReport>,
    exact_registered_shape_replayed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct DescriptorReport {
    authority: String,
    law_level_status: Disposition,
    probe_assumptions: Vec<String>,
    owner: String,
    parameter_context: Vec<Value>,
    introduction_ports: Vec<String>,
    assumed_constructor_code: String,
    assumed_recursive_position_masks: Vec<Vec<u32>>,
    assumed_computation_mode: String,
    matched_source_declarations: Vec<String>,
    matcher_equality: Disposition,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct TermOutputReport {
    port_name: String,
    ty: Term,
    role: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct DependentCoreProjectionReport {
    projection_id: String,
    authority: String,
    parameter_context: DependentContext,
    premise_refs: Vec<Value>,
    term_output_ports: Vec<TermOutputReport>,
    equation_output_ports: Vec<Value>,
    kernel_checks_performed: Vec<OpenJudgment>,
    gsc_support_assignment: Disposition,
    omitted_gsc_components: Vec<String>,
    literal_gsc_compile_use_authority: Disposition,
    probe_local_kernel_projection_well_formed: Disposition,
    law_level_computation_family_status: Disposition,
    current_kernel_equation_extension_feature: Disposition,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct DirectEliminatorReport {
    submitted_global_referencing_head_type: Term,
    kernel_stored_normalized_head_type: Term,
    use_filler_under_family_context: Term,
    beta_left: Term,
    beta_right: Term,
    beta_type: Term,
    kernel_head_type_formation: Disposition,
    bodyless_fresh_head_extension: Disposition,
    use_port_filler_typing: Disposition,
    lambda_p_m_z_to_m_definition: Disposition,
    fresh_head_beta_in_current_kernel: Disposition,
    host_agda_shape_model_source: Disposition,
    host_agda_external_typecheck: Disposition,
    syntactic_self_equality: Disposition,
    raw_public_clause_shape: String,
    comparison_with_legacy_kappa: Disposition,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ArchivedRepresentativeReport {
    candidate_hash: String,
    canonical_key: String,
    telescope: Telescope,
    legacy_candidate_hash_recomputed: bool,
    legacy_elaboration: Value,
    historical_grammar_admission: Disposition,
    legacy_typed_eliminator_token: Disposition,
    gf2_typed_elaboration: Disposition,
    h3_use_discharge: Disposition,
    h3_beta_discharge: Disposition,
    equivalence_to_direct: Disposition,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct MatrixRow {
    response: String,
    historical_grammar_admission: Disposition,
    gf2_grammar_admission: Disposition,
    typed_head_formation: Disposition,
    use_port_fill: Disposition,
    beta_port_fill: Disposition,
    full_downward_dag_discharge: Disposition,
    demand_connectedness: Disposition,
    quotient_equivalence_to_direct: Disposition,
    raw_public_clause_shape: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ProbeConclusion {
    outcome_classification: String,
    earliest_blocker: String,
    conditional_compile_use_result: String,
    historical_stage4_compatibility: Disposition,
    direct_extra_quotient_class: Disposition,
    contextual_internalization_implication: String,
    next_required_capabilities: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ProbeReport {
    schema: String,
    date: String,
    authority: String,
    source_bindings: Vec<SourceBinding>,
    h3_boundary: H3BoundaryReport,
    literal_gsc_descriptor_extraction: Disposition,
    shape_only_extraction_under_proposed_token_rule: Disposition,
    probe_local_descriptor: DescriptorReport,
    dependent_core_projection: DependentCoreProjectionReport,
    direct_eliminator: DirectEliminatorReport,
    archived_representatives: Vec<ArchivedRepresentativeReport>,
    discharge_equivalence_matrix: Vec<MatrixRow>,
    conclusion: ProbeConclusion,
    result_digest: String,
}

fn source_binding(path: &str, role: &str, bytes: &[u8]) -> SourceBinding {
    let canonical_bytes = canonical_lf_bytes(bytes);
    SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        hash_basis: "text bytes with CRLF canonicalized to LF".to_owned(),
        canonical_byte_length: canonical_bytes.len(),
        blake3: format!("blake3:{}", blake3_hex(&canonical_bytes)),
    }
}

fn canonical_lf_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut canonical = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\r' && bytes.get(index + 1) == Some(&b'\n') {
            canonical.push(b'\n');
            index += 2;
        } else {
            canonical.push(bytes[index]);
            index += 1;
        }
    }
    canonical
}

fn wire_id(id: &GlobalId) -> Result<String, Box<dyn Error>> {
    serde_json::to_value(id)?
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| "global identifier did not serialize as a string".into())
}

fn wire_digest(digest: &Digest) -> Result<String, Box<dyn Error>> {
    serde_json::to_value(digest)?
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| "digest did not serialize as a string".into())
}

fn exact_h3_boundary(
    declarations: &[Declaration],
) -> Result<Vec<BoundaryDeclarationReport>, Box<dyn Error>> {
    if declarations.len() != 3 {
        return Err(format!(
            "registered H3 has {} declarations rather than three",
            declarations.len()
        )
        .into());
    }
    let expected = [
        (Term::Sort { level: 1 }, Term::Sort { level: 0 }),
        (Term::Sort { level: 0 }, Term::UnitType),
        (Term::UnitType, Term::Unit),
    ];
    let mut report = Vec::with_capacity(3);
    for (index, (declaration, (expected_ty, expected_body))) in
        declarations.iter().zip(expected).enumerate()
    {
        if declaration.ty != expected_ty || declaration.body.as_ref() != Some(&expected_body) {
            return Err(format!("registered H3 declaration {} changed shape", index + 1).into());
        }
        report.push(BoundaryDeclarationReport {
            slot: format!("g{}", index + 1),
            id: wire_id(&declaration.id)?,
            ty: declaration.ty.clone(),
            body: expected_body,
        });
    }
    Ok(report)
}

fn pi(parameter: Term, body: Term) -> Term {
    Term::Pi {
        parameter: Box::new(parameter),
        body: Box::new(body),
    }
}

fn lambda(parameter_type: Term, body: Term) -> Term {
    Term::Lambda {
        parameter_type: Box::new(parameter_type),
        body: Box::new(body),
    }
}

fn apply(function: Term, argument: Term) -> Term {
    Term::Apply {
        function: Box::new(function),
        argument: Box::new(argument),
    }
}

fn global(id: &GlobalId) -> Term {
    Term::Global { id: id.clone() }
}

fn variable(index: u32) -> Term {
    Term::Var { index }
}

struct FamilyTerms {
    context: DependentContext,
    output_type: Term,
    direct_head_type: Term,
}

fn family_terms(universe: &GlobalId, owner: &GlobalId, intro: &GlobalId) -> FamilyTerms {
    let motive_type = pi(global(owner), global(universe));
    let method_type = apply(variable(0), global(intro));
    let output_type = pi(global(owner), apply(variable(2), variable(0)));
    let direct_head_type = pi(
        motive_type.clone(),
        pi(method_type.clone(), output_type.clone()),
    );
    FamilyTerms {
        context: DependentContext(vec![motive_type, method_type]),
        output_type,
        direct_head_type,
    }
}

fn archived_representatives() -> Result<Vec<ArchivedRepresentativeReport>, Box<dyn Error>> {
    let archive: Value = serde_json::from_slice(STAGE4_ARCHIVE_BYTES)?;
    let roots = archive
        .pointer("/replayed_v2_claim/blind_audit/preseal/live_strict_cone_geometry/roots")
        .and_then(Value::as_array)
        .ok_or("Stage-4 archive no longer exposes the four preseal roots")?;
    if roots.len() != 4 {
        return Err(format!("Stage-4 archive contains {} roots", roots.len()).into());
    }

    let legacy_prefix = SealedSignature::from_telescopes(
        (1..=3)
            .map(|stage| (stage, reference_telescope(stage)))
            .collect(),
    );
    let mut reports = Vec::with_capacity(4);
    for root in roots {
        let hash = root
            .get("candidate_hash")
            .and_then(Value::as_str)
            .ok_or("Stage-4 root omits candidate_hash")?
            .to_owned();
        let key = root
            .get("canonical_key")
            .and_then(Value::as_str)
            .ok_or("Stage-4 root omits canonical_key")?
            .to_owned();
        let telescope: Telescope = serde_json::from_value(
            root.get("telescope")
                .cloned()
                .ok_or("Stage-4 root omits telescope")?,
        )?;
        let recomputed = candidate_hash(&telescope);
        if recomputed != hash {
            return Err(format!("candidate hash drift for {hash}: recomputed {recomputed}").into());
        }
        let elaboration = elaborate_telescope(&legacy_prefix, &telescope, 3)
            .map_err(|error| format!("legacy elaboration failed for {hash}: {error}"))?;
        let token_error = issue_typed_eliminator_token(&legacy_prefix, &telescope, 3)
            .expect_err("the archived Stage-4 row must not issue a typed eliminator token");
        if token_error != TokenError::NoFormationClause {
            return Err(format!(
                "unexpected legacy typed-eliminator-token verdict for {hash}: {token_error}"
            )
            .into());
        }
        reports.push(ArchivedRepresentativeReport {
            candidate_hash: hash,
            canonical_key: key,
            telescope,
            legacy_candidate_hash_recomputed: true,
            legacy_elaboration: serde_json::to_value(elaboration)?,
            historical_grammar_admission: Disposition::proven(
                "archived live Stage-4 cone member under the legacy pen-core grammar",
            ),
            legacy_typed_eliminator_token: Disposition::refuted(
                "legacy issue_typed_eliminator_token returns NoFormationClause",
            ),
            gf2_typed_elaboration: Disposition::unknown(
                "no pen-core Telescope to registered-H3 GF2 exact-extension adapter exists",
            ),
            h3_use_discharge: Disposition::unknown(
                "no typed owner-specific UsePort filler has been constructed",
            ),
            h3_beta_discharge: Disposition::unknown(
                "no typed fresh-head equation clause has been constructed",
            ),
            equivalence_to_direct: Disposition::unknown(
                "no common typed API or adopted quotient theorem exists",
            ),
        });
    }
    let observed = reports
        .iter()
        .map(|row| row.candidate_hash.as_str())
        .collect::<Vec<_>>();
    let expected = EXPECTED_ARCHIVED_HASHES.into_iter().collect::<Vec<_>>();
    if observed != expected {
        return Err(format!("archived Stage-4 source order drifted: {observed:?}").into());
    }
    Ok(reports)
}

fn build_report() -> Result<ProbeReport, Box<dyn Error>> {
    let kernel = Kernel::new(KernelLimits::default())?;
    let bootstrap = load_embedded_registered_bootstrap(&kernel)?;
    let boundary = bootstrap.final_boundary();
    let boundary_rows = exact_h3_boundary(boundary.declarations())?;
    let universe = boundary.declarations()[0].id.clone();
    let owner = boundary.declarations()[1].id.clone();
    let intro = boundary.declarations()[2].id.clone();
    let owner_id = wire_id(&owner)?;
    let intro_id = wire_id(&intro)?;

    let family = family_terms(&universe, &owner, &intro);
    let output_formation = OpenJudgment::TypeFormation {
        context: family.context.clone(),
        term: family.output_type.clone(),
    };
    kernel.verify_open_judgment(boundary, &output_formation)?;
    let context_formation = OpenJudgment::TypeFormation {
        context: DependentContext(vec![family.context.0[0].clone()]),
        term: family.context.0[1].clone(),
    };
    kernel.verify_open_judgment(boundary, &context_formation)?;
    let closed_head_formation = OpenJudgment::TypeFormation {
        context: DependentContext::default(),
        term: family.direct_head_type.clone(),
    };
    kernel.verify_open_judgment(boundary, &closed_head_formation)?;

    let direct_id = GlobalId(Digest::of_domain_bytes(
        "law-v2-h3-compatibility-probe-v1",
        b"direct-unit-eliminator-head",
    ));
    let opaque_direct_extension = UncheckedSignature {
        declarations: vec![Declaration {
            id: direct_id.clone(),
            ty: family.direct_head_type.clone(),
            body: None,
        }],
    };
    let boundary_with_direct = kernel.verify_extension(boundary, &opaque_direct_extension)?;
    let normalized_direct_head_type = boundary_with_direct
        .declarations()
        .last()
        .filter(|declaration| declaration.id == direct_id)
        .map(|declaration| declaration.ty.clone())
        .ok_or("verified direct declaration was not retained at the boundary tip")?;
    let use_filler = apply(apply(global(&direct_id), variable(1)), variable(0));
    let use_filler_judgment = OpenJudgment::HasType {
        context: family.context.clone(),
        term: use_filler.clone(),
        ty: family.output_type.clone(),
    };
    kernel.verify_open_judgment(&boundary_with_direct, &use_filler_judgment)?;

    let beta_left = apply(use_filler.clone(), global(&intro));
    let beta_right = variable(0);
    let beta_type = apply(variable(1), global(&intro));
    let beta_judgment = OpenJudgment::DefinitionallyEqual {
        context: family.context.clone(),
        left: beta_left.clone(),
        right: beta_right.clone(),
        ty: beta_type.clone(),
    };
    let beta_error = kernel
        .verify_open_judgment(&boundary_with_direct, &beta_judgment)
        .expect_err("opaque direct head must not acquire an unverified beta rule");
    if beta_error != KernelError::TypeMismatch {
        return Err(format!("unexpected current-kernel beta verdict: {beta_error}").into());
    }

    let lambda_attempt_id = GlobalId(Digest::of_domain_bytes(
        "law-v2-h3-compatibility-probe-v1",
        b"lambda-p-m-z-to-m-attempt",
    ));
    let lambda_body = lambda(
        family.context.0[0].clone(),
        lambda(
            family.context.0[1].clone(),
            lambda(global(&owner), variable(1)),
        ),
    );
    let lambda_attempt = UncheckedSignature {
        declarations: vec![Declaration {
            id: lambda_attempt_id,
            ty: family.direct_head_type.clone(),
            body: Some(lambda_body),
        }],
    };
    let lambda_error = kernel
        .verify_extension(boundary, &lambda_attempt)
        .expect_err("lambda P m z -> m is not dependent unit elimination");
    if lambda_error != KernelError::TypeMismatch {
        return Err(format!("unexpected lambda-attempt verdict: {lambda_error}").into());
    }

    let archived = archived_representatives()?;
    let archive_rows = archived
        .iter()
        .map(|row| MatrixRow {
            response: row.candidate_hash.clone(),
            historical_grammar_admission: row.historical_grammar_admission.clone(),
            gf2_grammar_admission: Disposition::unknown(
                "the prospective typed GF2 candidate grammar is not implemented",
            ),
            typed_head_formation: row.gf2_typed_elaboration.clone(),
            use_port_fill: row.h3_use_discharge.clone(),
            beta_port_fill: row.h3_beta_discharge.clone(),
            full_downward_dag_discharge: Disposition::unknown(
                "the typed discharge DAG is not implemented",
            ),
            demand_connectedness: Disposition::unknown(
                "demand-connectedness is not defined for legacy Telescope rows",
            ),
            quotient_equivalence_to_direct: row.equivalence_to_direct.clone(),
            raw_public_clause_shape: "legacy_three_clause_telescope".to_owned(),
        })
        .collect::<Vec<_>>();

    let mut matrix = vec![MatrixRow {
        response: "sealed_h3_library".to_owned(),
        historical_grammar_admission: Disposition::outside(
            "library derivability is not a legacy candidate",
        ),
        gf2_grammar_admission: Disposition::unknown(
            "no complete public-operational derivability saturation exists",
        ),
        typed_head_formation: Disposition::unknown(
            "the sealed boundary contains no public unit-eliminator head",
        ),
        use_port_fill: Disposition::unknown(
            "host Agda elimination cannot be imported as a sealed-library capability",
        ),
        beta_port_fill: Disposition::unknown("the boundary has no equation-extension registry"),
        full_downward_dag_discharge: Disposition::unknown(
            "library-only typed saturation is not implemented",
        ),
        demand_connectedness: Disposition::outside(
            "demand-connectedness is a candidate-response property",
        ),
        quotient_equivalence_to_direct: Disposition::unknown(
            "no typed library realization or quotient theorem exists",
        ),
        raw_public_clause_shape: "no_candidate_shape".to_owned(),
    }];
    matrix.extend(archive_rows);
    matrix.push(MatrixRow {
        response: "direct_unit_eliminator".to_owned(),
        historical_grammar_admission: Disposition::outside(
            "pen-core Expr has no owner-specific UnitType/global/equation API",
        ),
        gf2_grammar_admission: Disposition::unknown(
            "the prospective typed GF2 candidate and equation grammars are not implemented",
        ),
        typed_head_formation: Disposition::proven(
            "pen-kernel verifies the exact closed dependent Pi type",
        ),
        use_port_fill: Disposition::proven(
            "conditional on a fresh opaque head, r Var1 Var0 has the probe-local projection output type",
        ),
        beta_port_fill: Disposition::unknown(
            "Law-level equation admission is unadopted; current pen-kernel separately lacks an equation-extension verifier",
        ),
        full_downward_dag_discharge: Disposition::unknown(
            "beta admission, candidate grammar, and coherent DAG discharge remain open",
        ),
        demand_connectedness: Disposition::unknown(
            "would be direct under the proposed owner/fresh-head grammar, which is not adopted",
        ),
        quotient_equivalence_to_direct: Disposition::unknown(
            "candidate admission and beta discharge are unresolved, so no response-quotient element exists to compare",
        ),
        raw_public_clause_shape: "one_head_plus_one_beta".to_owned(),
    });

    let descriptor = DescriptorReport {
        authority: "explicit probe stipulation matched against exact normalized H3 bytes; not GSC authority"
            .to_owned(),
        law_level_status: Disposition::unknown(
            "no adopted descriptor manifest, VerifiedHistory, export index, group code, or extractor exists",
        ),
        probe_assumptions: vec![
            "g2 is a closed former".to_owned(),
            "g3 is its sole nullary, nonrecursive introduction".to_owned(),
            "the constructor code is complete".to_owned(),
            "computation mode is judgmental".to_owned(),
        ],
        owner: owner_id.clone(),
        parameter_context: Vec::new(),
        introduction_ports: vec![intro_id.clone()],
        assumed_constructor_code: "one_nullary_nonrecursive_former".to_owned(),
        assumed_recursive_position_masks: vec![Vec::new()],
        assumed_computation_mode: "judgmental".to_owned(),
        matched_source_declarations: vec![owner_id.clone(), intro_id.clone()],
        matcher_equality: Disposition::proven(
            "the probe matcher returns its stipulated descriptor for the exact g1/g2/g3 bytes; this proves no Law-level classification or completeness theorem",
        ),
    };

    let dependent_core_projection = DependentCoreProjectionReport {
        projection_id: PROJECTION_ID.to_owned(),
        authority: "conditional one-nullary dependent-core projection; not CanonicalDemandFamily and not exact GSC compile_use".to_owned(),
        parameter_context: family.context.clone(),
        premise_refs: Vec::new(),
        term_output_ports: vec![TermOutputReport {
            port_name: "unit_use".to_owned(),
            ty: family.output_type.clone(),
            role: format!(
                "probe owner-specific use shape for {owner_id}; no full GSC UsePort code"
            ),
        }],
        equation_output_ports: Vec::new(),
        kernel_checks_performed: vec![context_formation, output_formation],
        gsc_support_assignment: Disposition::unknown(
            "birth, fixed-type, and parameter-projection support require the unadopted VerifiedHistory and family_and_support_encoding; native normalization unfolds g1/g2/g3",
        ),
        omitted_gsc_components: vec![
            "principal_sources".to_owned(),
            "canonical output port identifiers and contexts".to_owned(),
            "full UsePort owner/operation-shape/footprint/action codes".to_owned(),
            "sequential clause telescope".to_owned(),
            "manifest-defined verification judgments".to_owned(),
            "canonical family identity".to_owned(),
        ],
        literal_gsc_compile_use_authority: Disposition::unknown(
            "GSC semantic manifest and exact compiler equations remain unadopted",
        ),
        probe_local_kernel_projection_well_formed: Disposition::proven(
            "pen-kernel checks the parameter context and output type over registered H3",
        ),
        law_level_computation_family_status: Disposition::unknown(
            "the exact GSC computation compiler and equation grammar are unadopted",
        ),
        current_kernel_equation_extension_feature: Disposition::outside(
            "pen-kernel has no fresh-head equation-extension syntax or verifier",
        ),
    };

    let direct = DirectEliminatorReport {
        submitted_global_referencing_head_type: family.direct_head_type,
        kernel_stored_normalized_head_type: normalized_direct_head_type,
        use_filler_under_family_context: use_filler,
        beta_left,
        beta_right,
        beta_type,
        kernel_head_type_formation: Disposition::proven(
            "closed recursor Pi type forms over the registered H3 signature",
        ),
        bodyless_fresh_head_extension: Disposition::proven(
            "current pen-kernel accepts an opaque syntactic declaration of that type; this proves no CL admission, realization, conservativity, or free sealing",
        ),
        use_port_filler_typing: Disposition::proven(
            "conditional on the opaque head, r Var1 Var0 has the probe-local projection output type",
        ),
        lambda_p_m_z_to_m_definition: Disposition::refuted(
            "pen-kernel returns TypeMismatch because P g3 is not definitionally P z",
        ),
        fresh_head_beta_in_current_kernel: Disposition::outside(
            "pen-kernel has no equation-extension syntax or verifier",
        ),
        host_agda_shape_model_source: Disposition::proven(
            "the bound source contract contains an analogous host One/star eliminator, a refl beta witness, and no postulate token; this is source-shape evidence only",
        ),
        host_agda_external_typecheck: Disposition::unknown(
            "artifact replay binds and inspects source but does not invoke or attest an Agda executable, version, flags, imports, or result",
        ),
        syntactic_self_equality: Disposition::proven(
            "the submitted direct presentation is syntactically equal to itself; this is not quotient membership or equivalence",
        ),
        raw_public_clause_shape: "one fresh head plus one beta clause".to_owned(),
        comparison_with_legacy_kappa: Disposition::unknown(
            "two typed public clauses and three legacy shallow clauses have no adopted common irreducibility measure",
        ),
    };

    let mut report = ProbeReport {
        schema: SCHEMA.to_owned(),
        date: DATE.to_owned(),
        authority: AUTHORITY.to_owned(),
        source_bindings: vec![
            source_binding(
                "Cargo.toml",
                "workspace feature and dependency configuration",
                ROOT_CARGO_TOML_BYTES,
            ),
            source_binding(
                "Cargo.lock",
                "locked verifier dependency versions",
                CARGO_LOCK_BYTES,
            ),
            source_binding(
                "rust-toolchain.toml",
                "pinned Rust toolchain channel and components",
                RUST_TOOLCHAIN_BYTES,
            ),
            source_binding(
                "crates/pen-core/Cargo.toml",
                "legacy syntax dependency configuration",
                PEN_CORE_CARGO_TOML_BYTES,
            ),
            source_binding(
                "crates/pen-core/src/clause.rs",
                "legacy clause roles and serialized clause representation",
                PEN_CORE_CLAUSE_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-core/src/expr.rs",
                "legacy expression grammar and serialized expression representation",
                PEN_CORE_EXPR_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-core/src/hash.rs",
                "BLAKE3 helper used by legacy candidate hashes and report digests",
                PEN_CORE_HASH_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-core/src/stats.rs",
                "legacy structural-statistics implementation used by elaboration",
                PEN_CORE_STATS_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-core/src/telescope.rs",
                "legacy Telescope structure and structural statistics",
                PEN_CORE_TELESCOPE_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-kernel/Cargo.toml",
                "dependent-kernel dependency configuration",
                PEN_KERNEL_CARGO_TOML_BYTES,
            ),
            source_binding(
                "crates/pen-kernel/src/lib.rs",
                "dependent-kernel public module surface",
                PEN_KERNEL_LIB_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-kernel/src/checker.rs",
                "dependent-core formation, typing, equality, and extension verifier",
                PEN_KERNEL_CHECKER_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-kernel/src/dependency_graph.rs",
                "reviewed production dependency graph consulted by the kernel",
                PEN_KERNEL_DEPENDENCY_GRAPH_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-kernel/src/syntax.rs",
                "dependent-core term, judgment, and signature representation",
                PEN_KERNEL_SYNTAX_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-kernel/src/digest.rs",
                "dependent-kernel identifier and signature digest rules",
                PEN_KERNEL_DIGEST_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-law/Cargo.toml",
                "registered-bootstrap dependency configuration",
                PEN_LAW_CARGO_TOML_BYTES,
            ),
            source_binding(
                "crates/pen-law/assets/law_v2a_registered_bootstrap_v1.json",
                "exact registered H3 wire artifact",
                BOOTSTRAP_ASSET_BYTES,
            ),
            source_binding(
                "crates/pen-law/src/registered_bootstrap.rs",
                "registered bootstrap verifier and exact source constructor",
                REGISTERED_BOOTSTRAP_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-type/Cargo.toml",
                "legacy elaborator dependency configuration",
                PEN_TYPE_CARGO_TOML_BYTES,
            ),
            source_binding(
                "crates/pen-type/src/elaborate.rs",
                "legacy candidate hashing, elaboration, and typed-token attempt",
                PEN_TYPE_ELABORATE_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-type/src/equality.rs",
                "legacy elaborator judgmental-equality procedure",
                PEN_TYPE_EQUALITY_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-type/src/normalize.rs",
                "legacy elaborator normalization and binding convention",
                PEN_TYPE_NORMALIZE_SOURCE_BYTES,
            ),
            source_binding(
                "crates/pen-oracle/Cargo.toml",
                "oracle/example dependency configuration",
                PEN_ORACLE_CARGO_TOML_BYTES,
            ),
            source_binding(
                "crates/pen-oracle/src/lib.rs",
                "legacy reference-prefix constructor",
                PEN_ORACLE_LIB_SOURCE_BYTES,
            ),
            source_binding(
                "docs/stage4_semantic_parsimony_v3.json",
                "archived four-root Stage-4 oracle surface",
                STAGE4_ARCHIVE_BYTES,
            ),
            source_binding(
                "docs/LAW_V2_GENESIS_SCHEME_CALCULUS_V1.md",
                "non-adopted GSC architecture under test",
                GSC_PROPOSAL_BYTES,
            ),
            source_binding(
                "agda/LawV2/H3Compatibility.agda",
                "safe host-Agda conditional theorem experiment",
                AGDA_PROBE_BYTES,
            ),
            source_binding(
                "crates/pen-oracle/examples/law_v2_h3_compatibility_probe.rs",
                "oracle-only executable probe",
                THIS_SOURCE_BYTES,
            ),
        ],
        h3_boundary: H3BoundaryReport {
            bootstrap_artifact_digest: wire_digest(bootstrap.artifact_digest())?,
            final_boundary_digest: wire_digest(boundary.digest())?,
            declarations: boundary_rows,
            exact_registered_shape_replayed: true,
        },
        literal_gsc_descriptor_extraction: Disposition::unknown(
            "no adopted semantic manifest, VerifiedHistory, public export index, group enumeration, or authoritative extractor exists",
        ),
        shape_only_extraction_under_proposed_token_rule: Disposition::outside(
            "conditionally, a token-requiring GSC V1 extractor would reject the current flat boundary because it carries no closed-constructor-diagram token",
        ),
        probe_local_descriptor: descriptor,
        dependent_core_projection,
        direct_eliminator: direct,
        archived_representatives: archived,
        discharge_equivalence_matrix: matrix,
        conclusion: ProbeConclusion {
            outcome_classification: "blocked_before_A_B_C".to_owned(),
            earliest_blocker: "no authoritative ClosedFormerFrame for registered H3".to_owned(),
            conditional_compile_use_result:
                "under the explicit one-nullary assumptions, the probe-local dependent-core projection has ordinary dependent-unit-elimination shape; this is not an exact GSC compile_use result"
                    .to_owned(),
            historical_stage4_compatibility: Disposition::unknown(
                "all four archived rows lack typed GF2 APIs and H3 discharge certificates",
            ),
            direct_extra_quotient_class: Disposition::unknown(
                "direct head/use typing is proven, but candidate admission, beta extension, complete cone, and quotient are unavailable",
            ),
            contextual_internalization_implication:
                "a public context-projection/adjoint bridge is a candidate repair if an authoritative census later establishes Outcome C; this probe does not prove it is required"
                    .to_owned(),
            next_required_capabilities: vec![
                "adopted semantic descriptor/compiler manifest".to_owned(),
                "VerifiedHistory plus event-time public export index".to_owned(),
                "typed legacy-Telescope to GF2 candidate adapter or new typed representatives"
                    .to_owned(),
                "restricted fresh-head equation-extension verifier".to_owned(),
                "finite operational derivability/discharge DAG".to_owned(),
                "origin-cutoff quotient/equivalence registry".to_owned(),
            ],
        },
        result_digest: String::new(),
    };
    report.result_digest = report_digest(&report)?;
    Ok(report)
}

fn report_digest(report: &ProbeReport) -> Result<String, Box<dyn Error>> {
    let mut projection = report.clone();
    projection.result_digest.clear();
    Ok(format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&projection)?)
    ))
}

fn logical_errors(report: &ProbeReport) -> Result<Vec<String>, Box<dyn Error>> {
    let mut errors = Vec::new();
    let agda_source = std::str::from_utf8(AGDA_PROBE_BYTES)?;
    if report.schema != SCHEMA
        || report.authority != AUTHORITY
        || report.result_digest != report_digest(report)?
    {
        errors.push("schema, authority, or result digest mismatch".to_owned());
    }
    if !report.h3_boundary.exact_registered_shape_replayed
        || report.h3_boundary.declarations.len() != 3
    {
        errors.push("registered H3 boundary did not replay exactly".to_owned());
    }
    if report.archived_representatives.len() != 4
        || report.archived_representatives.iter().any(|row| {
            !row.legacy_candidate_hash_recomputed
                || row.legacy_typed_eliminator_token.verdict != Verdict::Refuted
        })
    {
        errors.push("archived Stage-4 representative join failed".to_owned());
    }
    if report
        .source_bindings
        .iter()
        .any(|binding| binding.hash_basis != "text bytes with CRLF canonicalized to LF")
    {
        errors.push("source binding hash basis drifted".to_owned());
    }
    if report.literal_gsc_descriptor_extraction.verdict != Verdict::Unknown
        || report
            .shape_only_extraction_under_proposed_token_rule
            .verdict
            != Verdict::OutsideFragment
        || report
            .dependent_core_projection
            .literal_gsc_compile_use_authority
            .verdict
            != Verdict::Unknown
        || report
            .dependent_core_projection
            .gsc_support_assignment
            .verdict
            != Verdict::Unknown
    {
        errors.push("probe blurred provisional and authoritative GSC status".to_owned());
    }
    let direct_matrix_row = report
        .discharge_equivalence_matrix
        .iter()
        .find(|row| row.response == "direct_unit_eliminator");
    if report.dependent_core_projection.term_output_ports.len() != 1
        || !report
            .dependent_core_projection
            .equation_output_ports
            .is_empty()
        || report.direct_eliminator.kernel_head_type_formation.verdict != Verdict::Proven
        || report
            .direct_eliminator
            .fresh_head_beta_in_current_kernel
            .verdict
            != Verdict::OutsideFragment
        || report
            .direct_eliminator
            .host_agda_external_typecheck
            .verdict
            != Verdict::Unknown
        || direct_matrix_row.is_none_or(|row| {
            row.beta_port_fill.verdict != Verdict::Unknown
                || row.quotient_equivalence_to_direct.verdict != Verdict::Unknown
        })
    {
        errors.push("conditional unit-elimination result drifted".to_owned());
    }
    if !agda_source.starts_with("{-# OPTIONS --safe --without-K #-}")
        || !agda_source.contains("module LawV2.H3Compatibility where")
        || !agda_source.contains("literalGscDescriptorDisposition = unknown")
        || !agda_source.contains("record OneNullaryUseProjection : Set where")
        || !agda_source.contains("projectOneNullaryUse :")
        || !agda_source.contains("unitInd :")
        || !agda_source.contains("unitIndBeta :")
        || agda_source.contains("record CanonicalDemandFamily")
        || agda_source.contains("compileUse :")
        || agda_source.contains("postulate")
    {
        errors.push("safe postulate-free Agda theorem source contract failed".to_owned());
    }
    Ok(errors)
}

fn write_create_new(path: &Path, report: &ProbeReport) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(&serde_json::to_vec_pretty(report)?)?;
    file.write_all(b"\n")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [mode, path] if mode == "create-new" => {
            let report = build_report()?;
            let errors = logical_errors(&report)?;
            if !errors.is_empty() {
                return Err(format!("create-new probe failed: {errors:?}").into());
            }
            write_create_new(Path::new(path), &report)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "valid": true,
                    "outcome": report.conclusion.outcome_classification,
                    "result_digest": report.result_digest,
                    "path": path,
                }))?
            );
        }
        [mode, path] if mode == "replay" => {
            let claimed: ProbeReport = serde_json::from_slice(&std::fs::read(path)?)?;
            let expected = build_report()?;
            let mut errors = logical_errors(&claimed)?;
            if claimed != expected {
                errors.push("artifact differs from deterministic reissuance".to_owned());
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "valid": errors.is_empty(),
                    "outcome": claimed.conclusion.outcome_classification,
                    "result_digest": claimed.result_digest,
                    "errors": errors,
                }))?
            );
            if !errors.is_empty() {
                std::process::exit(1);
            }
        }
        _ => {
            return Err(
                "usage: law_v2_h3_compatibility_probe <create-new|replay> <artifact.json>".into(),
            );
        }
    }
    Ok(())
}
