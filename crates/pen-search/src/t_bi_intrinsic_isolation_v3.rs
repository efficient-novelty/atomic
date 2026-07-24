//! Prefix-generic, source-first typed isolation for T-BI-B3.
//!
//! V5 now emits a receipt at each real prefix-local issuance site.  B3 is
//! consequently a validator and transitive-closure theorem over that trace;
//! it must not manufacture a second, more flattering DAG after issuance.

use crate::act_local_semantic_provenance_v5::{
    ACT_LOCAL_SEMANTIC_PROVENANCE_V5_SCHEMA, V5PrefixLocalIssuanceCapability,
    V5PrefixLocalIssuanceOperation, V5PrefixLocalIssuanceReceipt,
    V5PrefixLocalSemanticPackageProof, V5PrefixLocalSemanticSequence,
    issue_prefix_local_semantic_sequence_v5, replay_prefix_local_issuance_receipt,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use thiserror::Error;

pub const T_BI_INTRINSIC_ISOLATION_V3_SCHEMA: &str =
    "t-bi-intrinsic-prefix-local-source-first-isolation-v3";
pub const T_BI_INTRINSIC_ISOLATION_V3_DATE: &str = "2026-07-22";
pub const T_BI_B3_V3_THEOREM_ID: &str = "T-BI-B3-v3-real-issuance-trace-transitive-isolation";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_BI_INTRINSIC_ISOLATION_V3_SCHEMA, domain, value))
        .expect("T-BI-B3 v3 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

/// Reproduce the content-addressing domain of the v5 issuer.  This is used
/// only to check roots and outputs that v5 publishes; B3 never issues a v5
/// receipt with it.
fn v5_tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(ACT_LOCAL_SEMANTIC_PROVENANCE_V5_SCHEMA, domain, value))
        .expect("v5 receipt commitment serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn v5_sequence_hash(sequence: &V5PrefixLocalSemanticSequence) -> String {
    let mut projection = sequence.clone();
    projection.derivation_hash.clear();
    v5_tagged_hash("prefix-local-authoritative-semantic-sequence", &projection)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PrefixLocalBindingV3 {
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub v3_declaration_hash: String,
    pub v4_candidate_local_source_hash: String,
    pub observed_grammar_derivation_hash: String,
    pub semantic_construction_hash: String,
    pub semantic_package_derivation_hash: String,
    pub predecessor_package_proof_hashes: Vec<String>,
    pub predecessor_member_surface_digest: String,
    pub contributing_predecessor_member_hashes: Vec<String>,
    pub cubical_decision_surface_digest: String,
    pub previous_accumulator_hash: String,
    pub next_accumulator_hash: String,
    pub candidate_prefix_receipt_hash: String,
    pub v3_declaration_receipt_hash: String,
    pub v4_candidate_local_source_receipt_hash: String,
    pub observed_grammar_receipt_hash: String,
    pub semantic_construction_receipt_hash: String,
    pub semantic_package_receipt_hash: String,
    pub prefix_accumulator_receipt_hash: String,
    pub exact_candidate_and_prefix_binding: bool,
    pub exact_package_source_v3_grammar_predecessor_commitments: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntrinsicClosureRowV3 {
    pub node_id: String,
    pub receipt_hash: String,
    pub operation: V5PrefixLocalIssuanceOperation,
    pub direct_capabilities: Vec<V5PrefixLocalIssuanceCapability>,
    pub transitive_capabilities: Vec<V5PrefixLocalIssuanceCapability>,
    pub reaches_sequence_seal: bool,
    pub operation_capability_surface_closed: bool,
    pub isolated: bool,
    pub derivation_hash: String,
}

/// Minimal B3 theorem token.  The full v5 sequence and its semantic-nu vector
/// are deliberately not copied here.  A consumer that needs package contents
/// should use the combined replay context below; the legacy narrow adapter
/// remains byte-compatible and also checks this token before returning them.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiIntrinsicIsolationV3Token {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub prefix_len: usize,
    pub last_stage: u32,
    pub prefix_bindings: Vec<PrefixLocalBindingV3>,
    pub package_proof_hashes: Vec<String>,
    pub rule_bundle_receipt_hash: String,
    pub sequence_seal_receipt_hash: String,
    pub issuance_receipt_count: usize,
    pub issuance_trace_root: String,
    pub closure_rows: Vec<IntrinsicClosureRowV3>,
    pub authoritative_prefix_semantic_seal: String,
    pub nonempty_contiguous_stage1_through_n: bool,
    pub prefix_local_sequence_replayed: bool,
    pub every_local_package_proved_b1_b2: bool,
    pub every_registry_extension_projection_equal: bool,
    pub exact_one_rule_seven_operations_per_stage_and_sequence_seal: bool,
    pub every_receipt_replayed_at_its_topological_position: bool,
    pub every_predecessor_hash_bound: bool,
    pub every_capability_derived_from_operation: bool,
    pub exact_package_source_v3_grammar_predecessor_commitments: bool,
    pub cross_stage_prefix_accumulator_dependencies_exact: bool,
    pub issuance_trace_root_exact: bool,
    pub every_node_reaches_sequence_seal: bool,
    pub no_caller_receipt_parameter: bool,
    pub no_historical_registry_or_legacy_v5_authority: bool,
    pub no_archive_structural_bar_verdict_or_future_input: bool,
    pub prefix_generic_transitive_isolation_proved: bool,
    pub proof_scope: String,
    pub derivation_hash: String,
}

/// One proof-preserving B3 issuance/replay transaction.
///
/// The context is constructed from exactly two independent executions of the
/// prefix-local v5 issuer over the same act surface.  B3 is derived from each
/// sequence without invoking the v5 replay adapter (which would issue a third
/// sequence), and both the complete sequence evidence and the resulting B3
/// token must agree before this value can be returned.  `sequence` is the
/// original authoritative issuance, now already replayed by that comparison.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TBiIntrinsicIsolationV3ReplayContext {
    pub token: TBiIntrinsicIsolationV3Token,
    pub sequence: V5PrefixLocalSemanticSequence,
    pub independent_sequence_derivation_hash: String,
    pub independent_token_derivation_hash: String,
    pub sequence_evidence_equal: bool,
    pub token_evidence_equal: bool,
    pub replay_errors: Vec<String>,
    pub proved: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum TBiIntrinsicIsolationV3Error {
    #[error("T-BI-B3 v3 invalid input: {0}")]
    Input(String),
    #[error("T-BI-B3 v3 source binding failed: {0}")]
    Binding(String),
    #[error("T-BI-B3 v3 typed isolation failed: {0}")]
    Isolation(String),
}

fn binding_hash(binding: &PrefixLocalBindingV3) -> String {
    let mut projection = binding.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-local-binding", &projection)
}

fn closure_row_hash(row: &IntrinsicClosureRowV3) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("real-issuance-capability-closure-row", &projection)
}

fn token_hash(token: &TBiIntrinsicIsolationV3Token) -> String {
    let mut projection = token.clone();
    projection.derivation_hash.clear();
    tagged_hash("prefix-local-isolation-token", &projection)
}

fn node(stage: u32, suffix: &str) -> String {
    format!("stage-{stage:02}/{suffix}")
}

fn contiguous(entries: &[(u32, Telescope)]) -> bool {
    !entries.is_empty()
        && entries
            .iter()
            .map(|(stage, _)| *stage)
            .eq(1..=entries.len() as u32)
}

fn expected_operation(
    receipt: &V5PrefixLocalIssuanceReceipt,
    node_id: &str,
    stage: Option<u32>,
    operation: V5PrefixLocalIssuanceOperation,
) -> Result<(), TBiIntrinsicIsolationV3Error> {
    if receipt.node_id != node_id || receipt.stage != stage || receipt.operation != operation {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
            "receipt {} is not the expected {:?} node {node_id} at {stage:?}",
            receipt.node_id, operation
        )));
    }
    if receipt.direct_capabilities != operation.direct_capabilities() {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
            "receipt {node_id} has a capability set not derived from its operation"
        )));
    }
    Ok(())
}

fn expect_hashes(
    label: &str,
    actual: &[String],
    expected: &[String],
) -> Result<(), TBiIntrinsicIsolationV3Error> {
    if actual != expected {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
            "{label} commitment drifted: expected {expected:?}, found {actual:?}"
        )));
    }
    Ok(())
}

fn build_closure_rows(
    receipts: &[V5PrefixLocalIssuanceReceipt],
    sequence_node: &str,
) -> Result<Vec<IntrinsicClosureRowV3>, TBiIntrinsicIsolationV3Error> {
    let receipt_by_id = receipts
        .iter()
        .map(|receipt| (receipt.node_id.clone(), receipt))
        .collect::<BTreeMap<_, _>>();
    if receipt_by_id.len() != receipts.len() || !receipt_by_id.contains_key(sequence_node) {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(
            "real issuance trace has a duplicate node or no sequence seal".to_owned(),
        ));
    }

    let mut transitive_by_id = BTreeMap::<String, BTreeSet<V5PrefixLocalIssuanceCapability>>::new();
    for receipt in receipts {
        let mut capabilities = receipt
            .direct_capabilities
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        for predecessor in &receipt.predecessor_node_ids {
            let Some(predecessor_capabilities) = transitive_by_id.get(predecessor) else {
                return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
                    "real issuance trace is not topologically ordered at {}",
                    receipt.node_id
                )));
            };
            capabilities.extend(predecessor_capabilities.iter().copied());
        }
        transitive_by_id.insert(receipt.node_id.clone(), capabilities);
    }

    let mut successors = BTreeMap::<String, Vec<String>>::new();
    for receipt in receipts {
        successors.entry(receipt.node_id.clone()).or_default();
        for predecessor in &receipt.predecessor_node_ids {
            successors
                .entry(predecessor.clone())
                .or_default()
                .push(receipt.node_id.clone());
        }
    }

    receipts
        .iter()
        .map(|receipt| {
            let mut visited = BTreeSet::new();
            let mut queue = VecDeque::from([receipt.node_id.clone()]);
            while let Some(current) = queue.pop_front() {
                if !visited.insert(current.clone()) {
                    continue;
                }
                if let Some(next) = successors.get(&current) {
                    queue.extend(next.iter().cloned());
                }
            }
            let reaches_sequence_seal = visited.contains(sequence_node);
            let operation_capability_surface_closed =
                receipt.direct_capabilities == receipt.operation.direct_capabilities();
            let mut row = IntrinsicClosureRowV3 {
                node_id: receipt.node_id.clone(),
                receipt_hash: receipt.receipt_hash.clone(),
                operation: receipt.operation,
                direct_capabilities: receipt.direct_capabilities.clone(),
                transitive_capabilities: transitive_by_id
                    .get(&receipt.node_id)
                    .expect("topological pass inserted every receipt")
                    .iter()
                    .copied()
                    .collect(),
                reaches_sequence_seal,
                operation_capability_surface_closed,
                isolated: reaches_sequence_seal && operation_capability_surface_closed,
                derivation_hash: String::new(),
            };
            row.derivation_hash = closure_row_hash(&row);
            Ok(row)
        })
        .collect()
}

#[derive(Debug)]
struct ValidatedTraceV3 {
    prefix_bindings: Vec<PrefixLocalBindingV3>,
    package_proof_hashes: Vec<String>,
    closure_rows: Vec<IntrinsicClosureRowV3>,
    rule_bundle_receipt_hash: String,
    sequence_seal_receipt_hash: String,
}

fn validate_real_issuance_trace(
    entries: &[(u32, Telescope)],
    sequence: &V5PrefixLocalSemanticSequence,
) -> Result<ValidatedTraceV3, TBiIntrinsicIsolationV3Error> {
    if sequence.packages.len() != entries.len() {
        return Err(TBiIntrinsicIsolationV3Error::Binding(
            "prefix-local semantic sequence does not cover every act".to_owned(),
        ));
    }
    let receipts = &sequence.issuance_receipts;
    let expected_receipt_count = 1 + 7 * entries.len() + 1;
    if receipts.len() != expected_receipt_count {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
            "real issuance trace has {} receipts; expected one rule bundle + seven per stage + one sequence seal = {expected_receipt_count}",
            receipts.len()
        )));
    }
    let unique_nodes = receipts
        .iter()
        .map(|receipt| receipt.node_id.as_str())
        .collect::<BTreeSet<_>>();
    if unique_nodes.len() != receipts.len() {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(
            "real issuance trace repeats a node id".to_owned(),
        ));
    }
    for (index, receipt) in receipts.iter().enumerate() {
        if !replay_prefix_local_issuance_receipt(&receipts[..index], receipt) {
            return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
                "real issuance receipt {} failed topological replay",
                receipt.node_id
            )));
        }
    }
    let expected_trace_root = v5_tagged_hash(
        "prefix-local-issuance-trace-root",
        &receipts
            .iter()
            .map(|receipt| receipt.receipt_hash.as_str())
            .collect::<Vec<_>>(),
    );
    if sequence.issuance_trace_root != expected_trace_root {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(
            "published issuance trace root does not bind the real receipt order".to_owned(),
        ));
    }

    let rule = &receipts[0];
    expected_operation(
        rule,
        "authority/rules",
        None,
        V5PrefixLocalIssuanceOperation::RuleAuthorityIssue,
    )?;
    if !rule.predecessor_node_ids.is_empty() || rule.exact_input_hashes.len() != 3 {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(
            "rule bundle must be the unique root and bind exactly the v3, v4, and v5 typed authorities"
                .to_owned(),
        ));
    }
    if rule.exact_input_hashes[2] != sequence.rule_authority.derivation_hash {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(
            "rule bundle does not bind the v5 typed rule authority".to_owned(),
        ));
    }
    let expected_rule_output = v5_tagged_hash(
        "prefix-local-rule-authority-bundle",
        &(
            &rule.exact_input_hashes[0],
            &rule.exact_input_hashes[1],
            &rule.exact_input_hashes[2],
        ),
    );
    if rule.output_hash != expected_rule_output {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(
            "rule bundle output is not the hash of its three typed authorities".to_owned(),
        ));
    }

    let package_proof_hashes = sequence
        .packages
        .iter()
        .map(|package| package.derivation_hash.clone())
        .collect::<Vec<_>>();
    let mut accepted = Vec::<(u32, Telescope)>::new();
    let mut prior_v3_hashes = Vec::<String>::new();
    let mut previous_accumulator_node = None::<String>;
    let mut previous_accumulator_hash = v5_tagged_hash("prefix-local-empty-accumulator", &0u8);
    let mut prefix_bindings = Vec::with_capacity(entries.len());

    for (index, ((stage, candidate), package)) in entries.iter().zip(&sequence.packages).enumerate()
    {
        let base = 1 + index * 7;
        let candidate_receipt = &receipts[base];
        let v3_receipt = &receipts[base + 1];
        let v4_receipt = &receipts[base + 2];
        let grammar_receipt = &receipts[base + 3];
        let semantic_receipt = &receipts[base + 4];
        let package_receipt = &receipts[base + 5];
        let accumulator_receipt = &receipts[base + 6];
        let candidate_node = node(*stage, "candidate-prefix");
        let v3_node = node(*stage, "v3-declaration");
        let v4_node = node(*stage, "v4-candidate-local-source");
        let grammar_node = node(*stage, "observed-role-grammar");
        let semantic_node = node(*stage, "b1-b2-semantic-package");
        let package_node = node(*stage, "authoritative-package-seal");
        let accumulator_node = node(*stage, "prefix-accumulator");
        expected_operation(
            candidate_receipt,
            &candidate_node,
            Some(*stage),
            V5PrefixLocalIssuanceOperation::CandidatePrefixBind,
        )?;
        expected_operation(
            v3_receipt,
            &v3_node,
            Some(*stage),
            V5PrefixLocalIssuanceOperation::V3DeclarationIssue,
        )?;
        expected_operation(
            v4_receipt,
            &v4_node,
            Some(*stage),
            V5PrefixLocalIssuanceOperation::V4CandidateLocalSourceIssue,
        )?;
        expected_operation(
            grammar_receipt,
            &grammar_node,
            Some(*stage),
            V5PrefixLocalIssuanceOperation::ObservedGrammarIssue,
        )?;
        expected_operation(
            semantic_receipt,
            &semantic_node,
            Some(*stage),
            V5PrefixLocalIssuanceOperation::B1B2SemanticPackageIssue,
        )?;
        expected_operation(
            package_receipt,
            &package_node,
            Some(*stage),
            V5PrefixLocalIssuanceOperation::AuthoritativePackageSeal,
        )?;
        expected_operation(
            accumulator_receipt,
            &accumulator_node,
            Some(*stage),
            V5PrefixLocalIssuanceOperation::PrefixAccumulatorAdvance,
        )?;

        let prefix = SealedSignature::from_telescopes(accepted.clone());
        let candidate_digest = candidate_hash(candidate);
        let prefix_digest = prefix.digest().to_owned();
        let mut candidate_predecessors = vec!["authority/rules".to_owned()];
        if let Some(previous) = &previous_accumulator_node {
            candidate_predecessors.push(previous.clone());
        }
        expect_hashes(
            &format!("Stage {stage} candidate predecessors"),
            &candidate_receipt.predecessor_node_ids,
            &candidate_predecessors,
        )?;
        expect_hashes(
            &format!("Stage {stage} candidate/prefix inputs"),
            &candidate_receipt.exact_input_hashes,
            &[candidate_digest.clone(), prefix_digest.clone()],
        )?;
        let expected_candidate_output = v5_tagged_hash(
            "prefix-local-candidate-prefix-binding",
            &(*stage, &candidate_digest, &prefix_digest),
        );
        if candidate_receipt.output_hash != expected_candidate_output {
            return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
                "Stage {stage} candidate/prefix output is not its exact typed hash"
            )));
        }

        let mut expected_v3_inputs = vec![rule.exact_input_hashes[0].clone()];
        expected_v3_inputs.extend(prior_v3_hashes.iter().cloned());
        expect_hashes(
            &format!("Stage {stage} v3 predecessors"),
            &v3_receipt.predecessor_node_ids,
            std::slice::from_ref(&candidate_node),
        )?;
        expect_hashes(
            &format!("Stage {stage} v3 declaration inputs"),
            &v3_receipt.exact_input_hashes,
            &expected_v3_inputs,
        )?;
        if v3_receipt.output_hash != package.v3_declaration_hash {
            return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
                "Stage {stage} v3 receipt output does not equal the package declaration commitment"
            )));
        }

        expect_hashes(
            &format!("Stage {stage} v4 predecessors"),
            &v4_receipt.predecessor_node_ids,
            &[candidate_node.clone(), v3_node.clone()],
        )?;
        expect_hashes(
            &format!("Stage {stage} v4 source inputs"),
            &v4_receipt.exact_input_hashes,
            &[
                package.v3_declaration_hash.clone(),
                rule.exact_input_hashes[0].clone(),
                rule.exact_input_hashes[1].clone(),
            ],
        )?;
        if v4_receipt.output_hash != package.v4_candidate_local_source_hash {
            return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
                "Stage {stage} v4 receipt output does not equal the package source commitment"
            )));
        }

        expect_hashes(
            &format!("Stage {stage} grammar predecessors"),
            &grammar_receipt.predecessor_node_ids,
            std::slice::from_ref(&v3_node),
        )?;
        if grammar_receipt.exact_input_hashes.len()
            != package.observed_grammar.declaration_count + 1
            || grammar_receipt.exact_input_hashes.last() != Some(&package.v3_declaration_hash)
            || grammar_receipt.output_hash != package.observed_grammar.derivation_hash
        {
            return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
                "Stage {stage} grammar receipt does not exactly bind its declaration cover and v3 source"
            )));
        }

        let mut semantic_predecessors = vec![v4_node.clone(), grammar_node.clone()];
        if let Some(previous) = &previous_accumulator_node {
            semantic_predecessors.push(previous.clone());
        }
        expect_hashes(
            &format!("Stage {stage} semantic construction predecessors"),
            &semantic_receipt.predecessor_node_ids,
            &semantic_predecessors,
        )?;
        expect_hashes(
            &format!("Stage {stage} semantic construction inputs"),
            &semantic_receipt.exact_input_hashes,
            &[
                package.v4_candidate_local_source_hash.clone(),
                package.observed_grammar.derivation_hash.clone(),
                package.predecessor_member_surface_digest.clone(),
                package.cubical_decision_surface_digest.clone(),
                previous_accumulator_hash.clone(),
            ],
        )?;

        expect_hashes(
            &format!("Stage {stage} package-seal predecessors"),
            &package_receipt.predecessor_node_ids,
            std::slice::from_ref(&semantic_node),
        )?;
        expect_hashes(
            &format!("Stage {stage} package-seal inputs"),
            &package_receipt.exact_input_hashes,
            &[
                semantic_receipt.output_hash.clone(),
                package.observed_grammar.derivation_hash.clone(),
                package.predecessor_member_surface_digest.clone(),
            ],
        )?;
        if package_receipt.output_hash != package.derivation_hash {
            return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
                "Stage {stage} package seal does not output the authoritative package proof hash"
            )));
        }

        let mut accumulator_predecessors = vec![package_node.clone()];
        if let Some(previous) = &previous_accumulator_node {
            accumulator_predecessors.push(previous.clone());
        }
        expect_hashes(
            &format!("Stage {stage} accumulator predecessors"),
            &accumulator_receipt.predecessor_node_ids,
            &accumulator_predecessors,
        )?;
        if accumulator_receipt.exact_input_hashes.len() != 4
            || accumulator_receipt.exact_input_hashes[0] != previous_accumulator_hash
            || accumulator_receipt.exact_input_hashes[1] != package.derivation_hash
            || accumulator_receipt.exact_input_hashes[3] != package.cubical_decision_surface_digest
        {
            return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
                "Stage {stage} prefix accumulator does not bind its prior accumulator, package, member surface, and cubical surface"
            )));
        }
        if let Some(next_package) = sequence.packages.get(index + 1) {
            if accumulator_receipt.exact_input_hashes[2]
                != next_package.predecessor_member_surface_digest
            {
                return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
                    "Stage {stage} accumulator member surface is not the next package's predecessor surface"
                )));
            }
        }
        let expected_accumulator_output = v5_tagged_hash(
            "prefix-local-semantic-accumulator",
            &(
                &accumulator_receipt.exact_input_hashes[0],
                &accumulator_receipt.exact_input_hashes[1],
                &accumulator_receipt.exact_input_hashes[2],
                &accumulator_receipt.exact_input_hashes[3],
            ),
        );
        if accumulator_receipt.output_hash != expected_accumulator_output {
            return Err(TBiIntrinsicIsolationV3Error::Isolation(format!(
                "Stage {stage} accumulator output is not the hash of its four exact inputs"
            )));
        }

        let exact_candidate_and_prefix_binding = package.stage == *stage
            && package.candidate_hash == candidate_digest
            && package.predecessor_signature_digest == prefix_digest;
        let exact_commitments = package.rule_authority_derivation_hash
            == sequence.rule_authority.derivation_hash
            && package.predecessor_package_proof_hashes == package_proof_hashes[..index]
            && package.predecessor_commitments_exact
            && v3_receipt.output_hash == package.v3_declaration_hash
            && v4_receipt.output_hash == package.v4_candidate_local_source_hash
            && grammar_receipt.output_hash == package.observed_grammar.derivation_hash
            && package_receipt.output_hash == package.derivation_hash;
        if !exact_candidate_and_prefix_binding || !exact_commitments {
            return Err(TBiIntrinsicIsolationV3Error::Binding(format!(
                "Stage {stage} package/source/v3/grammar/predecessor binding drifted"
            )));
        }
        let mut binding = PrefixLocalBindingV3 {
            stage: *stage,
            candidate_hash: candidate_digest,
            predecessor_signature_digest: prefix_digest,
            v3_declaration_hash: package.v3_declaration_hash.clone(),
            v4_candidate_local_source_hash: package.v4_candidate_local_source_hash.clone(),
            observed_grammar_derivation_hash: package.observed_grammar.derivation_hash.clone(),
            semantic_construction_hash: semantic_receipt.output_hash.clone(),
            semantic_package_derivation_hash: package.derivation_hash.clone(),
            predecessor_package_proof_hashes: package.predecessor_package_proof_hashes.clone(),
            predecessor_member_surface_digest: package.predecessor_member_surface_digest.clone(),
            contributing_predecessor_member_hashes: package
                .contributing_predecessor_member_hashes
                .clone(),
            cubical_decision_surface_digest: package.cubical_decision_surface_digest.clone(),
            previous_accumulator_hash: previous_accumulator_hash.clone(),
            next_accumulator_hash: accumulator_receipt.output_hash.clone(),
            candidate_prefix_receipt_hash: candidate_receipt.receipt_hash.clone(),
            v3_declaration_receipt_hash: v3_receipt.receipt_hash.clone(),
            v4_candidate_local_source_receipt_hash: v4_receipt.receipt_hash.clone(),
            observed_grammar_receipt_hash: grammar_receipt.receipt_hash.clone(),
            semantic_construction_receipt_hash: semantic_receipt.receipt_hash.clone(),
            semantic_package_receipt_hash: package_receipt.receipt_hash.clone(),
            prefix_accumulator_receipt_hash: accumulator_receipt.receipt_hash.clone(),
            exact_candidate_and_prefix_binding,
            exact_package_source_v3_grammar_predecessor_commitments: exact_commitments,
            derivation_hash: String::new(),
        };
        binding.derivation_hash = binding_hash(&binding);
        prefix_bindings.push(binding);

        previous_accumulator_node = Some(accumulator_node);
        previous_accumulator_hash = accumulator_receipt.output_hash.clone();
        prior_v3_hashes.push(package.v3_declaration_hash.clone());
        accepted.push((*stage, candidate.clone()));
    }

    let sequence_receipt = receipts
        .last()
        .expect("nonempty trace count was checked above");
    expected_operation(
        sequence_receipt,
        "prefix-sequence/seal",
        None,
        V5PrefixLocalIssuanceOperation::SequenceSeal,
    )?;
    let mut expected_sequence_predecessors = entries
        .iter()
        .map(|(stage, _)| node(*stage, "authoritative-package-seal"))
        .collect::<Vec<_>>();
    expected_sequence_predecessors.push(
        previous_accumulator_node
            .clone()
            .expect("nonempty prefix produced an accumulator"),
    );
    expect_hashes(
        "sequence seal predecessors",
        &sequence_receipt.predecessor_node_ids,
        &expected_sequence_predecessors,
    )?;
    let mut expected_sequence_inputs = package_proof_hashes.clone();
    expected_sequence_inputs.push(previous_accumulator_hash);
    expect_hashes(
        "sequence seal inputs",
        &sequence_receipt.exact_input_hashes,
        &expected_sequence_inputs,
    )?;
    let expected_semantic_sequence_seal = v5_tagged_hash(
        "prefix-local-semantic-package-seal",
        &package_proof_hashes
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    );
    if sequence.authoritative_sequence_seal != expected_semantic_sequence_seal
        || sequence_receipt.output_hash != expected_semantic_sequence_seal
    {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(
            "sequence receipt does not output the exact package-proof seal".to_owned(),
        ));
    }

    let closure_rows = build_closure_rows(receipts, "prefix-sequence/seal")?;
    Ok(ValidatedTraceV3 {
        prefix_bindings,
        package_proof_hashes,
        closure_rows,
        rule_bundle_receipt_hash: rule.receipt_hash.clone(),
        sequence_seal_receipt_hash: sequence_receipt.receipt_hash.clone(),
    })
}

fn all_packages_close(package: &V5PrefixLocalSemanticPackageProof) -> bool {
    package.proved
        && package.t_bi_b1_proved
        && package.t_bi_b2_proved
        && package.every_role_declaration_resolved
        && package.every_marginal_family_credited_or_theorem_impossible
        && package.local_anchor_nonreuse_holds
        && package.predecessor_commitments_exact
}

fn derive_token_from_sequence(
    entries: &[(u32, Telescope)],
    sequence: &V5PrefixLocalSemanticSequence,
    prefix_local_sequence_replayed: bool,
) -> Result<TBiIntrinsicIsolationV3Token, TBiIntrinsicIsolationV3Error> {
    if !contiguous(entries) {
        return Err(TBiIntrinsicIsolationV3Error::Input(
            "requires a nonempty contiguous Stage-1-through-N act surface".to_owned(),
        ));
    }
    if sequence.derivation_hash != v5_sequence_hash(sequence) {
        return Err(TBiIntrinsicIsolationV3Error::Binding(
            "prefix-local v5 sequence digest mismatch".to_owned(),
        ));
    }
    let validated = validate_real_issuance_trace(entries, sequence)?;

    let prefix_len = entries.len();
    let last_stage = u32::try_from(prefix_len)
        .map_err(|_| TBiIntrinsicIsolationV3Error::Input("prefix length exceeds u32".to_owned()))?;
    let nonempty_contiguous_stage1_through_n = contiguous(entries);
    let every_local_package_proved_b1_b2 = sequence.packages.iter().all(all_packages_close)
        && sequence.t_bi_b1_proved_on_sequence
        && sequence.t_bi_b2_proved_on_sequence;
    let every_registry_extension_projection_equal = sequence
        .packages
        .iter()
        .all(|package| package.unused_registry_extension_projection_equal)
        && sequence.every_package_registry_extension_invariant
        && sequence.every_package_closed_observed_grammar;
    let exact_one_rule_seven_operations_per_stage_and_sequence_seal =
        sequence.issuance_receipts.len() == 1 + 7 * prefix_len + 1;
    let every_receipt_replayed_at_its_topological_position = sequence
        .issuance_receipts
        .iter()
        .enumerate()
        .all(|(index, receipt)| {
            replay_prefix_local_issuance_receipt(&sequence.issuance_receipts[..index], receipt)
        });
    let every_predecessor_hash_bound = every_receipt_replayed_at_its_topological_position;
    let every_capability_derived_from_operation = sequence
        .issuance_receipts
        .iter()
        .all(|receipt| receipt.direct_capabilities == receipt.operation.direct_capabilities());
    let exact_package_source_v3_grammar_predecessor_commitments =
        validated.prefix_bindings.iter().all(|binding| {
            binding.exact_candidate_and_prefix_binding
                && binding.exact_package_source_v3_grammar_predecessor_commitments
        });
    let cross_stage_prefix_accumulator_dependencies_exact = validated
        .prefix_bindings
        .windows(2)
        .all(|window| window[0].next_accumulator_hash == window[1].previous_accumulator_hash);
    let issuance_trace_root_exact = sequence.issuance_trace_root
        == v5_tagged_hash(
            "prefix-local-issuance-trace-root",
            &sequence
                .issuance_receipts
                .iter()
                .map(|receipt| receipt.receipt_hash.as_str())
                .collect::<Vec<_>>(),
        );
    let every_node_reaches_sequence_seal = validated.closure_rows.iter().all(|row| {
        row.reaches_sequence_seal && row.operation_capability_surface_closed && row.isolated
    });
    let no_caller_receipt_parameter = true;
    let no_historical_registry_or_legacy_v5_authority = sequence
        .no_historical_registry_or_future_input
        && sequence.packages.iter().all(|package| {
            !package.historical_registry_consulted
                && !package.archive_structural_bar_verdict_or_future_read
        });
    let no_archive_structural_bar_verdict_or_future_input =
        no_historical_registry_or_legacy_v5_authority && every_capability_derived_from_operation;
    let authoritative_prefix_semantic_seal = tagged_hash(
        "minimal-source-first-prefix-semantic-seal",
        &(
            validated
                .prefix_bindings
                .iter()
                .map(|binding| binding.derivation_hash.as_str())
                .collect::<Vec<_>>(),
            validated
                .package_proof_hashes
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            sequence.issuance_trace_root.as_str(),
            validated
                .closure_rows
                .iter()
                .map(|row| row.derivation_hash.as_str())
                .collect::<Vec<_>>(),
        ),
    );
    let prefix_generic_transitive_isolation_proved = nonempty_contiguous_stage1_through_n
        && prefix_local_sequence_replayed
        && every_local_package_proved_b1_b2
        && every_registry_extension_projection_equal
        && exact_one_rule_seven_operations_per_stage_and_sequence_seal
        && every_receipt_replayed_at_its_topological_position
        && every_predecessor_hash_bound
        && every_capability_derived_from_operation
        && exact_package_source_v3_grammar_predecessor_commitments
        && cross_stage_prefix_accumulator_dependencies_exact
        && issuance_trace_root_exact
        && every_node_reaches_sequence_seal
        && no_caller_receipt_parameter
        && no_historical_registry_or_legacy_v5_authority
        && no_archive_structural_bar_verdict_or_future_input
        && !authoritative_prefix_semantic_seal.is_empty();
    if !prefix_generic_transitive_isolation_proved {
        return Err(TBiIntrinsicIsolationV3Error::Isolation(
            "the real prefix-local issuance trace did not close its typed capability DAG"
                .to_owned(),
        ));
    }

    let mut token = TBiIntrinsicIsolationV3Token {
        schema: T_BI_INTRINSIC_ISOLATION_V3_SCHEMA.to_owned(),
        date: T_BI_INTRINSIC_ISOLATION_V3_DATE.to_owned(),
        theorem_id: T_BI_B3_V3_THEOREM_ID.to_owned(),
        prefix_len,
        last_stage,
        prefix_bindings: validated.prefix_bindings,
        package_proof_hashes: validated.package_proof_hashes,
        rule_bundle_receipt_hash: validated.rule_bundle_receipt_hash,
        sequence_seal_receipt_hash: validated.sequence_seal_receipt_hash,
        issuance_receipt_count: sequence.issuance_receipts.len(),
        issuance_trace_root: sequence.issuance_trace_root.clone(),
        closure_rows: validated.closure_rows,
        authoritative_prefix_semantic_seal,
        nonempty_contiguous_stage1_through_n,
        prefix_local_sequence_replayed,
        every_local_package_proved_b1_b2,
        every_registry_extension_projection_equal,
        exact_one_rule_seven_operations_per_stage_and_sequence_seal,
        every_receipt_replayed_at_its_topological_position,
        every_predecessor_hash_bound,
        every_capability_derived_from_operation,
        exact_package_source_v3_grammar_predecessor_commitments,
        cross_stage_prefix_accumulator_dependencies_exact,
        issuance_trace_root_exact,
        every_node_reaches_sequence_seal,
        no_caller_receipt_parameter,
        no_historical_registry_or_legacy_v5_authority,
        no_archive_structural_bar_verdict_or_future_input,
        prefix_generic_transitive_isolation_proved,
        proof_scope: "For every supplied nonempty contiguous Stage-1-through-N act prefix: deterministically reissue the prefix-local v5 construction; validate the one rule-authority receipt, the seven receipts emitted at the real issuance sites of every stage, and the final sequence seal; bind each candidate, exact predecessor prefix, v3 declaration, v4 source, closed observed grammar, B1/B2 construction, package proof, predecessor semantic surface, cubical surface, and cross-stage accumulator; replay every receipt against only earlier receipts; recompute the trace root and transitive capability closure; and prove that the closed operation surface contains no channel for historical registry, legacy package, archive, score, bar, verdict, or enacted-future authority. The token seals only exact prefix bindings, package proof hashes, the receipt root, and closure rows; semantic package contents are available solely by checked deterministic reissuance.".to_owned(),
        derivation_hash: String::new(),
    };
    token.derivation_hash = token_hash(&token);
    Ok(token)
}

fn compare_independent_issue_and_replay(
    entries: &[(u32, Telescope)],
    sequence: V5PrefixLocalSemanticSequence,
    independent_sequence: V5PrefixLocalSemanticSequence,
) -> Result<TBiIntrinsicIsolationV3ReplayContext, TBiIntrinsicIsolationV3Error> {
    let mut replay_errors = Vec::new();
    if sequence.derivation_hash != v5_sequence_hash(&sequence) {
        replay_errors.push("authoritative prefix-local v5 sequence digest mismatch".to_owned());
    }
    if independent_sequence.derivation_hash != v5_sequence_hash(&independent_sequence) {
        replay_errors.push("independent prefix-local v5 sequence digest mismatch".to_owned());
    }

    let token = derive_token_from_sequence(entries, &sequence, true)?;
    let independent_token = derive_token_from_sequence(entries, &independent_sequence, true)?;
    let sequence_evidence_equal = sequence == independent_sequence;
    let token_evidence_equal = token == independent_token;
    if !sequence_evidence_equal {
        replay_errors.push(
            "prefix-local v5 sequence differs from its independent deterministic replay".to_owned(),
        );
    }
    if !token_evidence_equal {
        replay_errors
            .push("T-BI-B3 v3 token differs across the two independent v5 sequences".to_owned());
    }
    let proved = replay_errors.is_empty() && sequence_evidence_equal && token_evidence_equal;
    if !proved {
        return Err(TBiIntrinsicIsolationV3Error::Binding(
            replay_errors.join("; "),
        ));
    }

    Ok(TBiIntrinsicIsolationV3ReplayContext {
        token,
        sequence,
        independent_sequence_derivation_hash: independent_sequence.derivation_hash,
        independent_token_derivation_hash: independent_token.derivation_hash,
        sequence_evidence_equal,
        token_evidence_equal,
        replay_errors,
        proved,
    })
}

/// Issue and replay the prefix-local v5/B3 evidence in one transaction.
///
/// This is the preferred consumer API when both the B3 token and the semantic
/// packages are needed.  It executes the v5 issuer exactly twice and returns
/// the first sequence only after the independently issued second sequence has
/// reproduced both that complete evidence and its derived B3 token.
pub fn issue_replayed_t_bi_intrinsic_isolation_v3_context(
    entries: &[(u32, Telescope)],
) -> Result<TBiIntrinsicIsolationV3ReplayContext, TBiIntrinsicIsolationV3Error> {
    if !contiguous(entries) {
        return Err(TBiIntrinsicIsolationV3Error::Input(
            "requires a nonempty contiguous Stage-1-through-N act surface".to_owned(),
        ));
    }
    let sequence = issue_prefix_local_semantic_sequence_v5(entries)
        .map_err(|error| TBiIntrinsicIsolationV3Error::Binding(error.to_string()))?;
    let independent_sequence = issue_prefix_local_semantic_sequence_v5(entries)
        .map_err(|error| TBiIntrinsicIsolationV3Error::Binding(error.to_string()))?;
    compare_independent_issue_and_replay(entries, sequence, independent_sequence)
}

fn issue_token_and_sequence(
    entries: &[(u32, Telescope)],
) -> Result<
    (TBiIntrinsicIsolationV3Token, V5PrefixLocalSemanticSequence),
    TBiIntrinsicIsolationV3Error,
> {
    let context = issue_replayed_t_bi_intrinsic_isolation_v3_context(entries)?;
    Ok((context.token, context.sequence))
}

/// Issue T-BI-B3 from the acts alone.  There is deliberately no historical
/// package, caller receipt, verdict, score, archive, or future-suffix input.
pub fn issue_t_bi_intrinsic_isolation_v3(
    entries: &[(u32, Telescope)],
) -> Result<TBiIntrinsicIsolationV3Token, TBiIntrinsicIsolationV3Error> {
    issue_token_and_sequence(entries).map(|(token, _)| token)
}

pub fn replay_t_bi_intrinsic_isolation_v3(
    entries: &[(u32, Telescope)],
    claimed: &TBiIntrinsicIsolationV3Token,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != token_hash(claimed) {
        errors.push("T-BI-B3 v3 token digest mismatch".to_owned());
    }
    match issue_token_and_sequence(entries) {
        Ok((expected, _)) if expected == *claimed => {}
        Ok(_) => errors
            .push("T-BI-B3 v3 token differs from deterministic real-trace reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

/// Narrow consumer adapter: recover the full semantic packages only through
/// deterministic v5 reissuance after the minimal B3 token has replayed.
pub fn reissue_isolated_prefix_local_semantic_sequence_v3(
    entries: &[(u32, Telescope)],
    claimed: &TBiIntrinsicIsolationV3Token,
) -> Result<V5PrefixLocalSemanticSequence, TBiIntrinsicIsolationV3Error> {
    let (expected, sequence) = issue_token_and_sequence(entries)?;
    if claimed.derivation_hash != token_hash(claimed) || expected != *claimed {
        return Err(TBiIntrinsicIsolationV3Error::Binding(
            "B3 token does not match deterministic reissuance".to_owned(),
        ));
    }
    Ok(sequence)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference_prefix(n: u32) -> Vec<(u32, Telescope)> {
        (1..=n)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect()
    }

    fn v5_receipt_hash_for_test(receipt: &V5PrefixLocalIssuanceReceipt) -> String {
        let mut projection = receipt.clone();
        projection.receipt_hash.clear();
        v5_tagged_hash("prefix-local-issuance-receipt", &projection)
    }

    fn v5_sequence_hash_for_test(sequence: &V5PrefixLocalSemanticSequence) -> String {
        let mut projection = sequence.clone();
        projection.derivation_hash.clear();
        v5_tagged_hash("prefix-local-authoritative-semantic-sequence", &projection)
    }

    fn fully_rehash_trace(sequence: &mut V5PrefixLocalSemanticSequence) {
        let mut prior = BTreeMap::<String, String>::new();
        for receipt in &mut sequence.issuance_receipts {
            receipt.predecessor_receipt_hashes = receipt
                .predecessor_node_ids
                .iter()
                .map(|node| prior.get(node).cloned().expect("test predecessor exists"))
                .collect();
            receipt.receipt_hash = v5_receipt_hash_for_test(receipt);
            prior.insert(receipt.node_id.clone(), receipt.receipt_hash.clone());
        }
        sequence.issuance_trace_root = v5_tagged_hash(
            "prefix-local-issuance-trace-root",
            &sequence
                .issuance_receipts
                .iter()
                .map(|receipt| receipt.receipt_hash.as_str())
                .collect::<Vec<_>>(),
        );
        sequence.derivation_hash = v5_sequence_hash_for_test(sequence);
    }

    #[test]
    fn stage4_prefix_closes_over_the_real_thirty_receipt_trace() {
        let entries = reference_prefix(4);
        let token = issue_t_bi_intrinsic_isolation_v3(&entries).expect("B3 v3");
        assert_eq!(token.prefix_len, 4);
        assert_eq!(token.issuance_receipt_count, 30);
        assert_eq!(token.closure_rows.len(), 30);
        assert_eq!(token.package_proof_hashes.len(), 4);
        assert_eq!(token.prefix_bindings.len(), 4);
        assert!(token.prefix_local_sequence_replayed);
        assert!(token.every_local_package_proved_b1_b2);
        assert!(token.every_registry_extension_projection_equal);
        assert!(token.every_node_reaches_sequence_seal);
        assert!(token.no_historical_registry_or_legacy_v5_authority);
        assert!(token.prefix_generic_transitive_isolation_proved);
        assert!(token.closure_rows.iter().all(|row| {
            row.reaches_sequence_seal && row.operation_capability_surface_closed && row.isolated
        }));
        let sequence = reissue_isolated_prefix_local_semantic_sequence_v3(&entries, &token)
            .expect("checked package adapter");
        assert_eq!(
            token.package_proof_hashes,
            sequence
                .packages
                .iter()
                .map(|package| package.derivation_hash.clone())
                .collect::<Vec<_>>()
        );
        assert!(replay_t_bi_intrinsic_isolation_v3(&entries, &token).is_empty());
    }

    #[test]
    fn combined_context_returns_the_authoritative_sequence_after_independent_replay() {
        let entries = reference_prefix(4);
        let context = issue_replayed_t_bi_intrinsic_isolation_v3_context(&entries)
            .expect("combined B3 issue/replay context");
        assert!(context.proved);
        assert!(context.replay_errors.is_empty());
        assert!(context.sequence_evidence_equal);
        assert!(context.token_evidence_equal);
        assert_eq!(
            context.independent_sequence_derivation_hash,
            context.sequence.derivation_hash
        );
        assert_eq!(
            context.independent_token_derivation_hash,
            context.token.derivation_hash
        );
        assert_eq!(context.token.prefix_len, context.sequence.packages.len());
        assert_eq!(
            context.token.package_proof_hashes,
            context
                .sequence
                .packages
                .iter()
                .map(|package| package.derivation_hash.clone())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn combined_context_rejects_a_fully_rehashed_independent_sequence_mutation() {
        let entries = reference_prefix(4);
        let sequence =
            issue_prefix_local_semantic_sequence_v5(&entries).expect("authoritative v5 sequence");
        let mut independent_sequence =
            issue_prefix_local_semantic_sequence_v5(&entries).expect("independent v5 sequence");

        // This field is intentionally outside B3's minimal token projection.
        // Rehashing it therefore tests that the combined context compares the
        // complete v5 evidence, rather than accepting token equality alone.
        independent_sequence.semantic_nu_vector[3] += 1;
        independent_sequence.derivation_hash = v5_sequence_hash(&independent_sequence);

        let error = compare_independent_issue_and_replay(&entries, sequence, independent_sequence)
            .expect_err("independent semantic-vector mutation must be rejected");
        assert!(
            error
                .to_string()
                .contains("differs from its independent deterministic replay")
        );
    }

    #[test]
    fn exact_fifteen_prefix_uses_one_plus_seven_n_plus_one_real_receipts() {
        let entries = reference_prefix(15);
        let token = issue_t_bi_intrinsic_isolation_v3(&entries).expect("B3 v3 at N=15");
        assert_eq!(token.issuance_receipt_count, 107);
        assert_eq!(token.closure_rows.len(), 107);
        assert_eq!(token.package_proof_hashes.len(), 15);
        assert_eq!(token.prefix_bindings.len(), 15);
        assert!(token.exact_one_rule_seven_operations_per_stage_and_sequence_seal);
        assert!(token.cross_stage_prefix_accumulator_dependencies_exact);
        assert!(token.prefix_generic_transitive_isolation_proved);
    }

    #[test]
    fn empty_and_gapped_surfaces_are_rejected() {
        assert!(issue_t_bi_intrinsic_isolation_v3(&[]).is_err());
        let mut gapped = reference_prefix(4);
        gapped[3].0 = 5;
        assert!(issue_t_bi_intrinsic_isolation_v3(&gapped).is_err());
    }

    #[test]
    fn fully_rehashed_exact_input_mutation_fails_real_trace_validation() {
        let entries = reference_prefix(4);
        let mut sequence = issue_prefix_local_semantic_sequence_v5(&entries).expect("v5 trace");
        let v4 = sequence
            .issuance_receipts
            .iter_mut()
            .find(|receipt| {
                receipt.stage == Some(2)
                    && receipt.operation
                        == V5PrefixLocalIssuanceOperation::V4CandidateLocalSourceIssue
            })
            .expect("Stage 2 v4 receipt");
        v4.exact_input_hashes[2] = "blake3:fully-rehashed-wrong-v4-rule".to_owned();
        fully_rehash_trace(&mut sequence);
        let error = validate_real_issuance_trace(&entries, &sequence).unwrap_err();
        assert!(error.to_string().contains("v4 source inputs"));
    }

    #[test]
    fn fully_rehashed_capability_mutation_fails_operation_mapping() {
        let entries = reference_prefix(4);
        let mut sequence = issue_prefix_local_semantic_sequence_v5(&entries).expect("v5 trace");
        let grammar = sequence
            .issuance_receipts
            .iter_mut()
            .find(|receipt| {
                receipt.stage == Some(3)
                    && receipt.operation == V5PrefixLocalIssuanceOperation::ObservedGrammarIssue
            })
            .expect("Stage 3 grammar receipt");
        grammar
            .direct_capabilities
            .push(V5PrefixLocalIssuanceCapability::ContentAddressedSeal);
        fully_rehash_trace(&mut sequence);
        let error = validate_real_issuance_trace(&entries, &sequence).unwrap_err();
        assert!(error.to_string().contains("topological replay"));
    }

    #[test]
    fn fully_rehashed_dependency_mutation_fails_cross_stage_accumulator_check() {
        let entries = reference_prefix(4);
        let mut sequence = issue_prefix_local_semantic_sequence_v5(&entries).expect("v5 trace");
        let candidate = sequence
            .issuance_receipts
            .iter_mut()
            .find(|receipt| {
                receipt.stage == Some(2)
                    && receipt.operation == V5PrefixLocalIssuanceOperation::CandidatePrefixBind
            })
            .expect("Stage 2 candidate receipt");
        candidate.predecessor_node_ids[1] = "stage-01/authoritative-package-seal".to_owned();
        fully_rehash_trace(&mut sequence);
        let error = validate_real_issuance_trace(&entries, &sequence).unwrap_err();
        assert!(error.to_string().contains("candidate predecessors"));
    }

    #[test]
    fn fully_rehashed_minimal_token_mutation_still_fails_reissuance() {
        let entries = reference_prefix(4);
        let token = issue_t_bi_intrinsic_isolation_v3(&entries).expect("B3 v3");
        let mut forged = token.clone();
        forged.package_proof_hashes[3] = "blake3:forged-package".to_owned();
        forged.derivation_hash = token_hash(&forged);
        let errors = replay_t_bi_intrinsic_isolation_v3(&entries, &forged);
        assert!(errors.iter().any(|error| error.contains("reissuance")));
    }
}
