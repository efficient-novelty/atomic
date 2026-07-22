//! Proof-carrying, finite capability closure for T-BI's intrinsic issuer.
//!
//! This is deliberately not a source scan.  The intrinsic issuer supplies one
//! receipt for every node of the closed execution algebra below.  Replay
//! checks the exact node/edge surface, computes the transitive capability
//! closure, and rejects any path to a post-seal capability.

use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const T_BI_INTRINSIC_ISOLATION_V1_SCHEMA: &str =
    "t-bi-intrinsic-transitive-capability-isolation-v1";
pub const T_BI_INTRINSIC_ISOLATION_V1_DATE: &str = "2026-07-22";
pub const T_BI_B3_THEOREM_ID: &str = "T-BI-B3-transitive-pre-seal-capability-isolation";

const SUPPORT_COMPREHENSION_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/support_comprehension_adjudication.md");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BI_INTRINSIC_ISOLATION_V1_SCHEMA, domain, value))
        .expect("intrinsic capability evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntrinsicCapabilityV1 {
    CandidateTerm,
    SealedPrefix,
    FrozenDeclarationGrammar,
    TermTypeEqualityKernel,
    Schema2Normalization,
    ExactA3Grammar,
    OrdinaryNaturalityKernel,
    CubicalPathKernel,
    ExactFamilyRoleRelation,
    ContentHashing,
    Archive,
    StructuralNu,
    Bar,
    Verdict,
    EnactedFuture,
}

impl IntrinsicCapabilityV1 {
    fn forbidden() -> BTreeSet<Self> {
        [
            Self::Archive,
            Self::StructuralNu,
            Self::Bar,
            Self::Verdict,
            Self::EnactedFuture,
        ]
        .into_iter()
        .collect()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntrinsicOperationV1 {
    CandidateInput,
    ExactPrefix,
    FrozenDeclarationReissue,
    CandidateElaboration,
    ExprToSchema2Bridge,
    ExactA3Inventory,
    OrdinaryFamilyProof,
    CubicalFamilyProof,
    ExactFamilyRoleRelation,
    PackageSeal,
    SequenceSeal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntrinsicCallNodeV1 {
    pub node_id: String,
    pub stage: Option<u32>,
    pub operation: IntrinsicOperationV1,
    pub predecessors: Vec<String>,
    pub direct_capabilities: Vec<IntrinsicCapabilityV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntrinsicExecutionReceiptV1 {
    pub node_id: String,
    pub output_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntrinsicClosureRowV1 {
    pub node_id: String,
    pub transitive_capabilities: Vec<IntrinsicCapabilityV1>,
    pub forbidden_capabilities: Vec<IntrinsicCapabilityV1>,
    pub isolated: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiIntrinsicIsolationV1Token {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub adopted_rule_hash: String,
    pub bound_intrinsic_schema: String,
    pub bound_package_derivation_hashes: Vec<String>,
    pub bound_intrinsic_sequence_seal: String,
    pub call_nodes: Vec<IntrinsicCallNodeV1>,
    pub execution_receipts: Vec<IntrinsicExecutionReceiptV1>,
    pub closure_rows: Vec<IntrinsicClosureRowV1>,
    pub exact_fifteen_stage_surface: bool,
    pub exact_closed_operation_surface: bool,
    pub every_receipt_bound_once: bool,
    pub graph_acyclic: bool,
    pub every_node_reaches_sequence_seal: bool,
    pub no_forbidden_capability_in_transitive_closure: bool,
    pub source_scan_used_as_proof: bool,
    pub runtime_self_report_used_as_proof: bool,
    pub transitive_call_graph_isolation_proved: bool,
    pub proof_scope: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TBiIntrinsicIsolationV1Error {
    #[error("T-BI-B3 adoption replay failed: {0}")]
    Adoption(String),
    #[error("T-BI-B3 binding failed: {0}")]
    Binding(String),
    #[error("T-BI-B3 graph failed: {0}")]
    Graph(String),
}

fn replay_adoption() -> Result<String, TBiIntrinsicIsolationV1Error> {
    let text = std::str::from_utf8(SUPPORT_COMPREHENSION_ADJUDICATION_BYTES)
        .map_err(|error| TBiIntrinsicIsolationV1Error::Adoption(error.to_string()))?;
    if !text.contains("support-comprehension-context-v1")
        || !text.contains("T-BI-B3")
        || !text.contains("source scans remain diagnostics")
        || !text.contains("## ADOPTION")
    {
        return Err(TBiIntrinsicIsolationV1Error::Adoption(
            "support-comprehension adoption did not replay T-BI-B3".to_owned(),
        ));
    }
    Ok(format!(
        "blake3:{}",
        blake3_hex(SUPPORT_COMPREHENSION_ADJUDICATION_BYTES)
    ))
}

fn stage_node_id(stage: u32, suffix: &str) -> String {
    format!("stage-{stage:02}/{suffix}")
}

fn canonical_call_nodes() -> Vec<IntrinsicCallNodeV1> {
    let mut nodes = Vec::new();
    let mut prior_package_seals = Vec::<String>::new();
    for stage in 1..=15 {
        let candidate = stage_node_id(stage, "candidate-input");
        let prefix = stage_node_id(stage, "exact-prefix");
        let declarations = stage_node_id(stage, "frozen-declarations");
        let elaboration = stage_node_id(stage, "candidate-elaboration");
        let bridge = stage_node_id(stage, "expr-schema2-bridge");
        let a3 = stage_node_id(stage, "exact-a3");
        let ordinary = stage_node_id(stage, "ordinary-family-proof");
        let cubical = stage_node_id(stage, "cubical-family-proof");
        let relation = stage_node_id(stage, "family-role-relation");
        let package_seal = stage_node_id(stage, "package-seal");

        nodes.push(IntrinsicCallNodeV1 {
            node_id: candidate.clone(),
            stage: Some(stage),
            operation: IntrinsicOperationV1::CandidateInput,
            predecessors: Vec::new(),
            direct_capabilities: vec![IntrinsicCapabilityV1::CandidateTerm],
        });
        nodes.push(IntrinsicCallNodeV1 {
            node_id: prefix.clone(),
            stage: Some(stage),
            operation: IntrinsicOperationV1::ExactPrefix,
            predecessors: prior_package_seals.clone(),
            direct_capabilities: vec![IntrinsicCapabilityV1::SealedPrefix],
        });
        nodes.push(IntrinsicCallNodeV1 {
            node_id: declarations.clone(),
            stage: Some(stage),
            operation: IntrinsicOperationV1::FrozenDeclarationReissue,
            predecessors: vec![candidate.clone(), prefix.clone()],
            direct_capabilities: vec![IntrinsicCapabilityV1::FrozenDeclarationGrammar],
        });
        nodes.push(IntrinsicCallNodeV1 {
            node_id: elaboration.clone(),
            stage: Some(stage),
            operation: IntrinsicOperationV1::CandidateElaboration,
            predecessors: vec![candidate.clone(), prefix.clone()],
            direct_capabilities: vec![IntrinsicCapabilityV1::TermTypeEqualityKernel],
        });
        nodes.push(IntrinsicCallNodeV1 {
            node_id: bridge.clone(),
            stage: Some(stage),
            operation: IntrinsicOperationV1::ExprToSchema2Bridge,
            predecessors: vec![elaboration.clone(), prefix.clone()],
            direct_capabilities: vec![IntrinsicCapabilityV1::Schema2Normalization],
        });
        nodes.push(IntrinsicCallNodeV1 {
            node_id: a3.clone(),
            stage: Some(stage),
            operation: IntrinsicOperationV1::ExactA3Inventory,
            predecessors: vec![prefix.clone()],
            direct_capabilities: vec![IntrinsicCapabilityV1::ExactA3Grammar],
        });
        nodes.push(IntrinsicCallNodeV1 {
            node_id: ordinary.clone(),
            stage: Some(stage),
            operation: IntrinsicOperationV1::OrdinaryFamilyProof,
            predecessors: vec![bridge.clone()],
            direct_capabilities: vec![IntrinsicCapabilityV1::OrdinaryNaturalityKernel],
        });
        nodes.push(IntrinsicCallNodeV1 {
            node_id: cubical.clone(),
            stage: Some(stage),
            operation: IntrinsicOperationV1::CubicalFamilyProof,
            predecessors: vec![bridge.clone()],
            direct_capabilities: vec![IntrinsicCapabilityV1::CubicalPathKernel],
        });
        nodes.push(IntrinsicCallNodeV1 {
            node_id: relation.clone(),
            stage: Some(stage),
            operation: IntrinsicOperationV1::ExactFamilyRoleRelation,
            predecessors: vec![declarations, ordinary, cubical, a3],
            direct_capabilities: vec![IntrinsicCapabilityV1::ExactFamilyRoleRelation],
        });
        nodes.push(IntrinsicCallNodeV1 {
            node_id: package_seal.clone(),
            stage: Some(stage),
            operation: IntrinsicOperationV1::PackageSeal,
            predecessors: vec![elaboration, bridge, relation],
            direct_capabilities: vec![IntrinsicCapabilityV1::ContentHashing],
        });
        prior_package_seals.push(package_seal);
    }
    nodes.push(IntrinsicCallNodeV1 {
        node_id: "intrinsic-sequence-seal".to_owned(),
        stage: None,
        operation: IntrinsicOperationV1::SequenceSeal,
        predecessors: prior_package_seals,
        direct_capabilities: vec![IntrinsicCapabilityV1::ContentHashing],
    });
    nodes
}

fn derive_closure(
    nodes: &[IntrinsicCallNodeV1],
) -> Result<(Vec<IntrinsicClosureRowV1>, bool), TBiIntrinsicIsolationV1Error> {
    let forbidden = IntrinsicCapabilityV1::forbidden();
    let mut seen = BTreeSet::<String>::new();
    let mut closure = BTreeMap::<String, BTreeSet<IntrinsicCapabilityV1>>::new();
    let mut rows = Vec::with_capacity(nodes.len());
    for node in nodes {
        if !seen.insert(node.node_id.clone()) {
            return Err(TBiIntrinsicIsolationV1Error::Graph(format!(
                "duplicate node {}",
                node.node_id
            )));
        }
        let mut capabilities = node
            .direct_capabilities
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        for predecessor in &node.predecessors {
            let predecessor_closure = closure.get(predecessor).ok_or_else(|| {
                TBiIntrinsicIsolationV1Error::Graph(format!(
                    "node {} references non-predecessor {predecessor}",
                    node.node_id
                ))
            })?;
            capabilities.extend(predecessor_closure.iter().copied());
        }
        let forbidden_capabilities = capabilities
            .intersection(&forbidden)
            .copied()
            .collect::<Vec<_>>();
        let isolated = forbidden_capabilities.is_empty();
        let transitive_capabilities = capabilities.iter().copied().collect::<Vec<_>>();
        let derivation_hash = tagged_hash(
            "transitive-capability-closure-row",
            &(
                &node.node_id,
                &node.predecessors,
                &node.direct_capabilities,
                &transitive_capabilities,
                &forbidden_capabilities,
                isolated,
            ),
        );
        closure.insert(node.node_id.clone(), capabilities);
        rows.push(IntrinsicClosureRowV1 {
            node_id: node.node_id.clone(),
            transitive_capabilities,
            forbidden_capabilities,
            isolated,
            derivation_hash,
        });
    }
    Ok((rows, true))
}

fn all_nodes_reach_sequence_seal(nodes: &[IntrinsicCallNodeV1]) -> bool {
    let Some(root) = nodes.last() else {
        return false;
    };
    if root.node_id != "intrinsic-sequence-seal" {
        return false;
    }
    let by_id = nodes
        .iter()
        .map(|node| (node.node_id.as_str(), node))
        .collect::<BTreeMap<_, _>>();
    let mut reachable = BTreeSet::<String>::new();
    let mut stack = vec![root.node_id.clone()];
    while let Some(node_id) = stack.pop() {
        if !reachable.insert(node_id.clone()) {
            continue;
        }
        let Some(node) = by_id.get(node_id.as_str()) else {
            return false;
        };
        stack.extend(node.predecessors.iter().cloned());
    }
    reachable.len() == nodes.len()
}

fn token_digest(token: &TBiIntrinsicIsolationV1Token) -> String {
    let mut projection = token.clone();
    projection.derivation_hash.clear();
    tagged_hash("transitive-intrinsic-isolation-token", &projection)
}

pub fn issue_t_bi_intrinsic_isolation_v1(
    intrinsic_schema: &str,
    package_derivation_hashes: &[String],
    intrinsic_sequence_seal: &str,
    execution_receipts: &[IntrinsicExecutionReceiptV1],
) -> Result<TBiIntrinsicIsolationV1Token, TBiIntrinsicIsolationV1Error> {
    let adopted_rule_hash = replay_adoption()?;
    if intrinsic_schema.is_empty()
        || package_derivation_hashes.len() != 15
        || package_derivation_hashes
            .iter()
            .any(|hash| !hash.starts_with("blake3:"))
        || !intrinsic_sequence_seal.starts_with("blake3:")
    {
        return Err(TBiIntrinsicIsolationV1Error::Binding(
            "expected one intrinsic schema, fifteen package hashes, and one sequence seal"
                .to_owned(),
        ));
    }

    let call_nodes = canonical_call_nodes();
    let expected_ids = call_nodes
        .iter()
        .map(|node| node.node_id.as_str())
        .collect::<BTreeSet<_>>();
    let receipt_ids = execution_receipts
        .iter()
        .map(|receipt| receipt.node_id.as_str())
        .collect::<BTreeSet<_>>();
    let every_receipt_bound_once = execution_receipts.len() == call_nodes.len()
        && receipt_ids.len() == execution_receipts.len()
        && receipt_ids == expected_ids
        && execution_receipts
            .iter()
            .all(|receipt| receipt.output_hash.starts_with("blake3:"))
        && execution_receipts
            .iter()
            .find(|receipt| receipt.node_id == "intrinsic-sequence-seal")
            .is_some_and(|receipt| receipt.output_hash == intrinsic_sequence_seal)
        && (1..=15).all(|stage| {
            execution_receipts
                .iter()
                .find(|receipt| receipt.node_id == stage_node_id(stage, "package-seal"))
                .is_some_and(|receipt| {
                    receipt.output_hash == package_derivation_hashes[(stage - 1) as usize]
                })
        });

    let exact_fifteen_stage_surface = call_nodes
        .iter()
        .filter_map(|node| node.stage)
        .collect::<BTreeSet<_>>()
        == (1..=15).collect::<BTreeSet<_>>()
        && package_derivation_hashes.len() == 15;
    let exact_closed_operation_surface = call_nodes.len() == 151
        && (1..=15).all(|stage| {
            call_nodes
                .iter()
                .filter(|node| node.stage == Some(stage))
                .count()
                == 10
        })
        && call_nodes
            .iter()
            .filter(|node| node.operation == IntrinsicOperationV1::SequenceSeal)
            .count()
            == 1;
    let (closure_rows, graph_acyclic) = derive_closure(&call_nodes)?;
    let every_node_reaches_sequence_seal = all_nodes_reach_sequence_seal(&call_nodes);
    let no_forbidden_capability_in_transitive_closure = closure_rows
        .iter()
        .all(|row| row.isolated && row.forbidden_capabilities.is_empty());
    let source_scan_used_as_proof = false;
    let runtime_self_report_used_as_proof = false;
    let transitive_call_graph_isolation_proved = exact_fifteen_stage_surface
        && exact_closed_operation_surface
        && every_receipt_bound_once
        && graph_acyclic
        && every_node_reaches_sequence_seal
        && no_forbidden_capability_in_transitive_closure
        && !source_scan_used_as_proof
        && !runtime_self_report_used_as_proof;

    let mut token = TBiIntrinsicIsolationV1Token {
        schema: T_BI_INTRINSIC_ISOLATION_V1_SCHEMA.to_owned(),
        date: T_BI_INTRINSIC_ISOLATION_V1_DATE.to_owned(),
        theorem_id: T_BI_B3_THEOREM_ID.to_owned(),
        adopted_rule_hash,
        bound_intrinsic_schema: intrinsic_schema.to_owned(),
        bound_package_derivation_hashes: package_derivation_hashes.to_vec(),
        bound_intrinsic_sequence_seal: intrinsic_sequence_seal.to_owned(),
        call_nodes,
        execution_receipts: execution_receipts.to_vec(),
        closure_rows,
        exact_fifteen_stage_surface,
        exact_closed_operation_surface,
        every_receipt_bound_once,
        graph_acyclic,
        every_node_reaches_sequence_seal,
        no_forbidden_capability_in_transitive_closure,
        source_scan_used_as_proof,
        runtime_self_report_used_as_proof,
        transitive_call_graph_isolation_proved,
        proof_scope: "Closed typed execution algebra from candidate/prefix/declaration inputs through the fifteen package seals and the intrinsic sequence seal; post-seal archive, structural-nu, bar, verdict, and enacted-future capabilities are absent from every transitive closure.".to_owned(),
        derivation_hash: String::new(),
    };
    token.derivation_hash = token_digest(&token);
    Ok(token)
}

pub fn replay_t_bi_intrinsic_isolation_v1(
    claimed: &TBiIntrinsicIsolationV1Token,
    intrinsic_schema: &str,
    package_derivation_hashes: &[String],
    intrinsic_sequence_seal: &str,
    execution_receipts: &[IntrinsicExecutionReceiptV1],
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != token_digest(claimed) {
        errors.push("T-BI-B3 token digest mismatch".to_owned());
    }
    match issue_t_bi_intrinsic_isolation_v1(
        intrinsic_schema,
        package_derivation_hashes,
        intrinsic_sequence_seal,
        execution_receipts,
    ) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("T-BI-B3 token differs from typed reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    fn receipts() -> Vec<IntrinsicExecutionReceiptV1> {
        canonical_call_nodes()
            .iter()
            .map(|node| IntrinsicExecutionReceiptV1 {
                node_id: node.node_id.clone(),
                output_hash: tagged_hash("test-receipt", &node.node_id),
            })
            .collect()
    }

    #[test]
    fn closed_capability_graph_has_no_post_seal_path() {
        let mut receipts = receipts();
        let sequence_seal = tagged_hash("test-sequence-seal", &15_u32);
        receipts
            .iter_mut()
            .find(|receipt| receipt.node_id == "intrinsic-sequence-seal")
            .expect("sequence receipt")
            .output_hash = sequence_seal.clone();
        let packages = (1..=15)
            .map(|stage| tagged_hash("test-package", &stage))
            .collect::<Vec<_>>();
        for stage in 1..=15 {
            receipts
                .iter_mut()
                .find(|receipt| receipt.node_id == stage_node_id(stage, "package-seal"))
                .expect("package receipt")
                .output_hash = packages[(stage - 1) as usize].clone();
        }
        let token = issue_t_bi_intrinsic_isolation_v1(
            "test-intrinsic-schema",
            &packages,
            &sequence_seal,
            &receipts,
        )
        .expect("isolation theorem");
        assert!(token.transitive_call_graph_isolation_proved);
        assert!(
            replay_t_bi_intrinsic_isolation_v1(
                &token,
                "test-intrinsic-schema",
                &packages,
                &sequence_seal,
                &receipts,
            )
            .is_empty()
        );
    }

    #[test]
    fn missing_or_unbound_receipt_cannot_prove_isolation() {
        let mut receipts = receipts();
        receipts.pop();
        let sequence_seal = tagged_hash("test-sequence-seal", &15_u32);
        let packages = (1..=15)
            .map(|stage| tagged_hash("test-package", &stage))
            .collect::<Vec<_>>();
        let token = issue_t_bi_intrinsic_isolation_v1(
            "test-intrinsic-schema",
            &packages,
            &sequence_seal,
            &receipts,
        )
        .expect("negative isolation token");
        assert!(!token.every_receipt_bound_once);
        assert!(!token.transitive_call_graph_isolation_proved);
    }
}
