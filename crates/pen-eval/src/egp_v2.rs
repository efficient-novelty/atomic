//! Proof-carrying EGP-v2 independent-generative-basis certificates.
//!
//! This module implements the frozen bootstrap law in
//! `docs/EGP_V2_BOOTSTRAP_PROGRAM.md`.  It deliberately does not know a
//! Genesis stage, a historical score, or a bar.  Its only inputs are the
//! checked predecessor signature/closure, the candidate surface, and (when
//! present) replayable typed declaration evidence.
//!
//! The first burned-run audit found that declaration/presentation bridging,
//! declaration-class marginality, live-ledger binding, and typed free
//! completion remain open.  Consequently these certificates are experimental
//! evidence for the frozen EGP-v2 result, not a completed semantic theorem;
//! see `docs/EGP_V2_BOOTSTRAP_RESULT.md`.

use crate::semantic_provenance::{LocalRole, blind_local_role_coefficient};
use crate::typed_families::{
    CandidateExtractionOutcome, CandidateFamilyExtraction, ExtractedFamily, InstanceKind,
    MarginalityDisposition, PredecessorClosure, extract_candidate_families,
};
use pen_core::clause::ClauseRole;
use pen_core::declaration::{ConstructorCompletionOverlay, InductiveFormationOverlay};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::{SealedSignature, candidate_hash, minimal_ambient_parameters};
use pen_type::fresh_declaration::{
    ConstructorCompletionEvidence, InductiveFormationEvidence, replay_constructor_completion,
    replay_inductive_formation,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const EGP_V2_LAW_ID: &str = "egp-v2-independent-generative-basis-v1";

#[derive(Clone, Copy, Debug)]
pub enum DeclarationEvidence<'a> {
    None,
    Formation {
        overlay: &'a InductiveFormationOverlay,
        evidence: &'a InductiveFormationEvidence,
    },
    Completion {
        formation_overlay: &'a InductiveFormationOverlay,
        formation: &'a InductiveFormationEvidence,
        overlay: &'a ConstructorCompletionOverlay,
        evidence: &'a ConstructorCompletionEvidence,
    },
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticTargetKind {
    NaturalFamily,
    FreshInductiveDeclaration,
    FreshConstructorDeclaration,
    GeneratedOneConstructorEliminator,
    GeneratedOneConstructorBeta,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SemanticTarget {
    pub id: String,
    pub kind: SemanticTargetKind,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ProvenanceTag {
    Local { clause_index: u16, role: LocalRole },
    LiveDemand { owner: String, slot_ordinal: u16 },
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BasisAtom {
    pub id: String,
    pub target: String,
    pub generator_clause: u16,
    pub provenance: ProvenanceTag,
    pub anchor_derivation_hash: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RegisteredCompletionRule {
    UniverseDecodeFromSamePackageFormation,
    FreshDeclarationPresentation,
    OneConstructorEliminator,
    OneConstructorBeta,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CoverageRule {
    PredecessorInternal {
        witness: String,
    },
    CreditedAtom {
        atom_id: String,
    },
    RegisteredCompletion {
        rule: RegisteredCompletionRule,
        premises: Vec<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CoverageDerivation {
    pub target: String,
    pub rule: CoverageRule,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct IndispensabilityWitness {
    pub atom_id: String,
    pub uncovered_target_after_removal: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MinimumBasisCertificate {
    pub candidate_atom_count: usize,
    pub minimum_cardinality: usize,
    pub subsets_checked: usize,
    pub smaller_subsets_checked: usize,
    pub tie_break_order: Vec<String>,
    pub indispensability: Vec<IndispensabilityWitness>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BasisCertificate {
    pub law_id: String,
    pub subject_hash: String,
    pub semantic_subject_digest: String,
    pub extraction_derivation_hash: String,
    pub targets: Vec<SemanticTarget>,
    pub candidate_atoms: Vec<BasisAtom>,
    pub basis_atoms: Vec<BasisAtom>,
    pub coverage_derivations: Vec<CoverageDerivation>,
    pub minimum_basis: MinimumBasisCertificate,
    pub provenance_injective: bool,
    pub local_capacity: u32,
    pub revised_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum EgpV2Error {
    #[error("candidate extraction subject does not match the supplied telescope")]
    SubjectMismatch,
    #[error("candidate extraction does not replay against the supplied signature and closure")]
    ExtractionReplayMismatch,
    #[error("candidate extraction has an inconsistent marginal-family count")]
    MarginalCountMismatch,
    #[error("candidate extraction contains duplicate family IDs")]
    DuplicateFamily,
    #[error("family {family} does not have exactly one in-range generator")]
    InvalidGenerator { family: String },
    #[error("family {family} has a failed naturality square")]
    FailedNaturality { family: String },
    #[error("typed declaration evidence does not replay: {0}")]
    DeclarationReplay(String),
    #[error("the bootstrap declaration bridge requires an exact one-clause presentation")]
    DeclarationPresentationArity,
    #[error("the one-constructor formation does not expose exactly one pending nullary slot")]
    InvalidOneConstructorFormation,
    #[error("the constructor does not close the exact one-constructor signature")]
    InvalidOneConstructorCompletion,
    #[error("the finite atom inventory is too large for exhaustive basis minimization")]
    BasisInventoryTooLarge,
    #[error("no subset of candidate atoms covers the complete target inventory")]
    UncoveredInventory,
    #[error("selected basis provenance tags are not injective")]
    ProvenanceCollision,
    #[error("selected local basis exceeds the blindly enumerated local-role capacity")]
    LocalCapacityExceeded,
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(EGP_V2_LAW_ID, domain, payload))
        .expect("EGP-v2 hashes only serializable in-memory proof data");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn generator_clause(family: &ExtractedFamily, kappa: usize) -> Result<u16, EgpV2Error> {
    let generators: Vec<u16> = family
        .instances
        .iter()
        .filter_map(|instance| {
            matches!(instance.kind, InstanceKind::Generator).then_some(instance.clause_index)
        })
        .collect();
    if generators.len() != 1 || usize::from(generators[0]) >= kappa {
        return Err(EgpV2Error::InvalidGenerator {
            family: family.id.as_str().to_owned(),
        });
    }
    Ok(generators[0])
}

fn role_for_generator(role: ClauseRole) -> LocalRole {
    match role {
        ClauseRole::Formation | ClauseRole::Introduction => LocalRole::KernelHead,
        ClauseRole::Elimination => LocalRole::AdjointMate,
        ClauseRole::PathAttach => LocalRole::Coherence,
        ClauseRole::Computation => LocalRole::SupportAction,
    }
}

fn family_target_id(family: &ExtractedFamily) -> String {
    format!("family:{}", family.id.as_str())
}

fn internal_witness(family: &ExtractedFamily) -> String {
    match &family.marginality {
        MarginalityDisposition::InternalIdentical {
            predecessor_step,
            predecessor_clause,
            equality,
        } => tagged_hash(
            "internal-identical",
            &(
                family.id.as_str(),
                predecessor_step,
                predecessor_clause,
                equality,
            ),
        ),
        MarginalityDisposition::InternalDerivable {
            predecessor_step,
            predecessor_clause,
            substitution,
        } => tagged_hash(
            "internal-derivable",
            &(
                family.id.as_str(),
                predecessor_step,
                predecessor_clause,
                substitution,
            ),
        ),
        MarginalityDisposition::MarginalNoClosurePreimage { .. } => {
            unreachable!("only internal families request an internal witness")
        }
    }
}

fn universe_package_source<'a>(
    extraction: &'a CandidateFamilyExtraction,
    telescope: &Telescope,
    target: &ExtractedFamily,
) -> Option<&'a ExtractedFamily> {
    if target.generator_role != ClauseRole::Formation {
        return None;
    }
    let target_clause = generator_clause(target, telescope.kappa()).ok()?;
    let Expr::App(head, argument) = &telescope.clauses[usize::from(target_clause)].expr else {
        return None;
    };
    let (Expr::Univ, Expr::Var(level)) = (head.as_ref(), argument.as_ref()) else {
        return None;
    };
    let ambient = minimal_ambient_parameters(telescope);
    extraction.marginal_families().find(|source| {
        if source.generator_role != ClauseRole::Formation {
            return false;
        }
        let Ok(source_clause) = generator_clause(source, telescope.kappa()) else {
            return false;
        };
        source_clause < target_clause
            && telescope.clauses[usize::from(source_clause)].expr == Expr::Univ
            && *level == ambient + u32::from(source_clause) + 1
    })
}

fn prefix_before_last(signature: &SealedSignature) -> SealedSignature {
    SealedSignature::from_telescopes(
        signature
            .entries()
            .iter()
            .take(signature.len().saturating_sub(1))
            .map(|entry| (entry.step, entry.telescope.clone()))
            .collect(),
    )
}

fn add_atom(
    atoms: &mut Vec<BasisAtom>,
    target: &str,
    generator_clause: u16,
    provenance: ProvenanceTag,
    anchor_payload: &impl Serialize,
) -> String {
    let id = tagged_hash(
        "basis-atom-id",
        &(target, generator_clause, &provenance, anchor_payload),
    );
    atoms.push(BasisAtom {
        id: id.clone(),
        target: target.to_owned(),
        generator_clause,
        provenance,
        anchor_derivation_hash: tagged_hash("basis-anchor", anchor_payload),
    });
    id
}

fn coverage_derivation(target: &str, rule: CoverageRule) -> CoverageDerivation {
    let derivation_hash = tagged_hash("coverage", &(target, &rule));
    CoverageDerivation {
        target: target.to_owned(),
        rule,
        derivation_hash,
    }
}

fn covered(
    target: &str,
    rules: &BTreeMap<String, CoverageRule>,
    selected: &BTreeSet<String>,
    visiting: &mut BTreeSet<String>,
    memo: &mut BTreeMap<String, bool>,
) -> bool {
    if let Some(result) = memo.get(target) {
        return *result;
    }
    if !visiting.insert(target.to_owned()) {
        return false;
    }
    let result = match rules.get(target) {
        Some(CoverageRule::PredecessorInternal { .. }) => true,
        Some(CoverageRule::CreditedAtom { atom_id }) => selected.contains(atom_id),
        Some(CoverageRule::RegisteredCompletion { premises, .. }) => premises
            .iter()
            .all(|premise| covered(premise, rules, selected, visiting, memo)),
        None => false,
    };
    visiting.remove(target);
    memo.insert(target.to_owned(), result);
    result
}

fn first_uncovered(
    targets: &[SemanticTarget],
    rules: &BTreeMap<String, CoverageRule>,
    selected: &BTreeSet<String>,
) -> Option<String> {
    let mut memo = BTreeMap::new();
    targets
        .iter()
        .find(|target| !covered(&target.id, rules, selected, &mut BTreeSet::new(), &mut memo))
        .map(|target| target.id.clone())
}

fn minimum_basis(
    targets: &[SemanticTarget],
    atoms: &[BasisAtom],
    rules: &BTreeMap<String, CoverageRule>,
) -> Result<(Vec<BasisAtom>, MinimumBasisCertificate), EgpV2Error> {
    if atoms.len() > 20 {
        return Err(EgpV2Error::BasisInventoryTooLarge);
    }
    let mut ordered = atoms.to_vec();
    ordered.sort_by(|left, right| left.id.cmp(&right.id));
    let mut subsets_checked = 0usize;
    let mut selected_ids = None;

    for cardinality in 0..=ordered.len() {
        for mask in 0u64..(1u64 << ordered.len()) {
            if mask.count_ones() as usize != cardinality {
                continue;
            }
            subsets_checked += 1;
            let ids: BTreeSet<String> = ordered
                .iter()
                .enumerate()
                .filter(|(index, _)| mask & (1u64 << index) != 0)
                .map(|(_, atom)| atom.id.clone())
                .collect();
            if first_uncovered(targets, rules, &ids).is_none() {
                selected_ids = Some(ids);
                break;
            }
        }
        if selected_ids.is_some() {
            break;
        }
    }
    let selected_ids = selected_ids.ok_or(EgpV2Error::UncoveredInventory)?;
    let basis_atoms: Vec<BasisAtom> = ordered
        .iter()
        .filter(|atom| selected_ids.contains(&atom.id))
        .cloned()
        .collect();
    let minimum_cardinality = basis_atoms.len();
    // The loop checks every subset at smaller cardinality and then the
    // lexicographically ordered masks at the minimum cardinality up to the
    // first cover.  Record the exact number of strictly smaller subsets.
    let smaller_subsets_checked = (0..minimum_cardinality)
        .map(|cardinality| binomial(ordered.len(), cardinality))
        .sum();
    let indispensability = basis_atoms
        .iter()
        .map(|removed| {
            let remaining: BTreeSet<String> = selected_ids
                .iter()
                .filter(|id| *id != &removed.id)
                .cloned()
                .collect();
            let uncovered_target_after_removal = first_uncovered(targets, rules, &remaining)
                .expect("a minimum-cardinality basis atom is indispensable");
            IndispensabilityWitness {
                atom_id: removed.id.clone(),
                uncovered_target_after_removal,
            }
        })
        .collect::<Vec<_>>();
    let tie_break_order = ordered
        .iter()
        .map(|atom| atom.id.clone())
        .collect::<Vec<_>>();
    let derivation_hash = tagged_hash(
        "minimum-basis",
        &(
            ordered.len(),
            minimum_cardinality,
            subsets_checked,
            smaller_subsets_checked,
            &tie_break_order,
            &indispensability,
        ),
    );
    Ok((
        basis_atoms,
        MinimumBasisCertificate {
            candidate_atom_count: ordered.len(),
            minimum_cardinality,
            subsets_checked,
            smaller_subsets_checked,
            tie_break_order,
            indispensability,
            derivation_hash,
        },
    ))
}

fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    (0..k).fold(1usize, |result, index| result * (n - index) / (index + 1))
}

/// Replays the complete extraction and constructs the least covering basis.
pub fn certify_independent_basis(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
    telescope: &Telescope,
    extraction: &CandidateFamilyExtraction,
    declaration: DeclarationEvidence<'_>,
) -> Result<BasisCertificate, EgpV2Error> {
    if candidate_hash(telescope) != extraction.subject_hash {
        return Err(EgpV2Error::SubjectMismatch);
    }
    let replay = extract_candidate_families(
        signature,
        closure,
        telescope,
        u32::try_from(signature.len()).expect("signature length fits u32"),
    );
    if replay != CandidateExtractionOutcome::Extracted(extraction.clone()) {
        return Err(EgpV2Error::ExtractionReplayMismatch);
    }
    if extraction.marginal_family_count != extraction.marginal_families().count() {
        return Err(EgpV2Error::MarginalCountMismatch);
    }
    let mut family_ids = BTreeSet::new();
    for family in &extraction.families {
        if !family_ids.insert(family.id.as_str()) {
            return Err(EgpV2Error::DuplicateFamily);
        }
        generator_clause(family, telescope.kappa())?;
        if !family.naturality.square.equal {
            return Err(EgpV2Error::FailedNaturality {
                family: family.id.as_str().to_owned(),
            });
        }
    }

    let mut targets: Vec<SemanticTarget> = extraction
        .families
        .iter()
        .map(|family| SemanticTarget {
            id: family_target_id(family),
            kind: SemanticTargetKind::NaturalFamily,
        })
        .collect();
    let mut atoms = Vec::new();
    let mut rules: BTreeMap<String, CoverageRule> = BTreeMap::new();
    for family in &extraction.families {
        if !family.marginality.is_marginal() {
            rules.insert(
                family_target_id(family),
                CoverageRule::PredecessorInternal {
                    witness: internal_witness(family),
                },
            );
        }
    }

    let semantic_subject_digest = match declaration {
        DeclarationEvidence::None => extraction.subject_hash.clone(),
        DeclarationEvidence::Formation { overlay, evidence } => {
            if telescope.kappa() != 1 || overlay.presentation() != telescope {
                return Err(EgpV2Error::DeclarationPresentationArity);
            }
            replay_inductive_formation(signature, overlay, evidence)
                .map_err(|error| EgpV2Error::DeclarationReplay(error.to_string()))?;
            if evidence.legacy_presentation_hash() != extraction.subject_hash
                || evidence.signature_digest() != extraction.signature_digest
                || evidence.pending_slots().len() != 1
                || evidence.pending_slot_shape(&evidence.pending_slots()[0])
                    != Some(pen_core::declaration::ConstructorSlotShape::Nullary)
            {
                return Err(EgpV2Error::InvalidOneConstructorFormation);
            }
            let declaration_target =
                format!("inductive-declaration:{}", evidence.declaration_digest());
            targets.push(SemanticTarget {
                id: declaration_target.clone(),
                kind: SemanticTargetKind::FreshInductiveDeclaration,
            });
            let atom_id = add_atom(
                &mut atoms,
                &declaration_target,
                0,
                ProvenanceTag::Local {
                    clause_index: 0,
                    role: LocalRole::KernelHead,
                },
                &(
                    evidence.semantic_subject_digest(),
                    evidence.declaration_digest(),
                    evidence.fresh_head(),
                    evidence.derivation_hash(),
                    extraction.derivation_hash.as_str(),
                ),
            );
            rules.insert(
                declaration_target.clone(),
                CoverageRule::CreditedAtom { atom_id },
            );
            for family in extraction.marginal_families() {
                rules.insert(
                    family_target_id(family),
                    CoverageRule::RegisteredCompletion {
                        rule: RegisteredCompletionRule::FreshDeclarationPresentation,
                        premises: vec![declaration_target.clone()],
                    },
                );
            }
            evidence.semantic_subject_digest().to_owned()
        }
        DeclarationEvidence::Completion {
            formation_overlay,
            formation,
            overlay,
            evidence,
        } => {
            if telescope.kappa() != 1 || overlay.presentation() != telescope {
                return Err(EgpV2Error::DeclarationPresentationArity);
            }
            let formation_signature = prefix_before_last(signature);
            replay_inductive_formation(&formation_signature, formation_overlay, formation)
                .map_err(|error| EgpV2Error::DeclarationReplay(error.to_string()))?;
            replay_constructor_completion(signature, formation, overlay, evidence)
                .map_err(|error| EgpV2Error::DeclarationReplay(error.to_string()))?;
            if evidence.legacy_presentation_hash() != extraction.subject_hash
                || evidence.answered_slot().owner() != formation.fresh_head()
                || evidence.answered_slot() != &formation.pending_slots()[0]
                || !evidence.is_inductive_signature_closed()
            {
                return Err(EgpV2Error::InvalidOneConstructorCompletion);
            }
            let declaration_target =
                format!("constructor-declaration:{}", evidence.declaration_digest());
            targets.push(SemanticTarget {
                id: declaration_target.clone(),
                kind: SemanticTargetKind::FreshConstructorDeclaration,
            });
            // One atom, one tag: answering the live constructor slot is not
            // also counted under a local KernelHead tag.
            let atom_id = add_atom(
                &mut atoms,
                &declaration_target,
                0,
                ProvenanceTag::LiveDemand {
                    owner: evidence.answered_slot().owner().as_str().to_owned(),
                    slot_ordinal: evidence.answered_slot().ordinal(),
                },
                &(
                    evidence.semantic_subject_digest(),
                    evidence.declaration_digest(),
                    evidence.fresh_constructor(),
                    evidence.answered_slot(),
                    evidence.derivation_hash(),
                    extraction.derivation_hash.as_str(),
                ),
            );
            rules.insert(
                declaration_target.clone(),
                CoverageRule::CreditedAtom { atom_id },
            );
            for family in extraction.marginal_families() {
                rules.insert(
                    family_target_id(family),
                    CoverageRule::RegisteredCompletion {
                        rule: RegisteredCompletionRule::FreshDeclarationPresentation,
                        premises: vec![declaration_target.clone()],
                    },
                );
            }
            let eliminator_target =
                format!("one-constructor-eliminator:{}", evidence.derivation_hash());
            let beta_target = format!("one-constructor-beta:{}", evidence.derivation_hash());
            targets.push(SemanticTarget {
                id: eliminator_target.clone(),
                kind: SemanticTargetKind::GeneratedOneConstructorEliminator,
            });
            targets.push(SemanticTarget {
                id: beta_target.clone(),
                kind: SemanticTargetKind::GeneratedOneConstructorBeta,
            });
            rules.insert(
                eliminator_target,
                CoverageRule::RegisteredCompletion {
                    rule: RegisteredCompletionRule::OneConstructorEliminator,
                    premises: vec![declaration_target.clone()],
                },
            );
            rules.insert(
                beta_target,
                CoverageRule::RegisteredCompletion {
                    rule: RegisteredCompletionRule::OneConstructorBeta,
                    premises: vec![declaration_target],
                },
            );
            evidence.semantic_subject_digest().to_owned()
        }
    };

    if matches!(declaration, DeclarationEvidence::None) {
        for family in extraction.marginal_families() {
            let target = family_target_id(family);
            if let Some(source) = universe_package_source(extraction, telescope, family) {
                rules.insert(
                    target,
                    CoverageRule::RegisteredCompletion {
                        rule: RegisteredCompletionRule::UniverseDecodeFromSamePackageFormation,
                        premises: vec![family_target_id(source)],
                    },
                );
                continue;
            }
            let clause = generator_clause(family, telescope.kappa())?;
            let provenance = ProvenanceTag::Local {
                clause_index: clause,
                role: role_for_generator(family.generator_role),
            };
            let atom_id = add_atom(
                &mut atoms,
                &target,
                clause,
                provenance,
                &(
                    extraction.derivation_hash.as_str(),
                    family.id.as_str(),
                    clause,
                    family.generator_role,
                ),
            );
            rules.insert(target, CoverageRule::CreditedAtom { atom_id });
        }
    }

    targets.sort();
    atoms.sort_by(|left, right| left.id.cmp(&right.id));
    if rules.len() != targets.len() {
        return Err(EgpV2Error::UncoveredInventory);
    }
    let (basis_atoms, minimum_basis) = minimum_basis(&targets, &atoms, &rules)?;
    let provenance: BTreeSet<&ProvenanceTag> =
        basis_atoms.iter().map(|atom| &atom.provenance).collect();
    if provenance.len() != basis_atoms.len() {
        return Err(EgpV2Error::ProvenanceCollision);
    }
    let local_count = basis_atoms
        .iter()
        .filter(|atom| matches!(atom.provenance, ProvenanceTag::Local { .. }))
        .count() as u32;
    let local_capacity = u32::from(blind_local_role_coefficient())
        .saturating_mul(u32::try_from(telescope.kappa()).expect("kappa fits u32"));
    if local_count > local_capacity {
        return Err(EgpV2Error::LocalCapacityExceeded);
    }
    let coverage_derivations = targets
        .iter()
        .map(|target| {
            coverage_derivation(
                &target.id,
                rules
                    .get(&target.id)
                    .expect("every checked target has a coverage rule")
                    .clone(),
            )
        })
        .collect::<Vec<_>>();
    let revised_nu = u32::try_from(basis_atoms.len()).expect("basis length fits u32");
    let derivation_hash = tagged_hash(
        "basis-certificate",
        &(
            extraction.subject_hash.as_str(),
            semantic_subject_digest.as_str(),
            extraction.derivation_hash.as_str(),
            &targets,
            &atoms,
            &basis_atoms,
            &coverage_derivations,
            &minimum_basis,
            local_capacity,
            revised_nu,
        ),
    );
    Ok(BasisCertificate {
        law_id: EGP_V2_LAW_ID.to_owned(),
        subject_hash: extraction.subject_hash.clone(),
        semantic_subject_digest,
        extraction_derivation_hash: extraction.derivation_hash.clone(),
        targets,
        candidate_atoms: atoms,
        basis_atoms,
        coverage_derivations,
        minimum_basis,
        provenance_injective: true,
        local_capacity,
        revised_nu,
        derivation_hash,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typed_families::predecessor_closure;
    use pen_core::declaration::{
        ConstructorDeclaration, DeclarationType, FreshExportSite, InductiveHeadDeclaration,
    };
    use pen_type::fresh_declaration::{issue_constructor_completion, issue_inductive_formation};

    fn extracted<'a>(outcome: &'a CandidateExtractionOutcome) -> &'a CandidateFamilyExtraction {
        match outcome {
            CandidateExtractionOutcome::Extracted(extraction) => extraction,
            CandidateExtractionOutcome::KernelInvalid { failure } => {
                panic!("test candidate is kernel-invalid: {failure}")
            }
        }
    }

    #[test]
    fn universe_package_has_a_unique_one_atom_basis() {
        let signature = SealedSignature::from_telescopes(vec![]);
        let closure = predecessor_closure(&signature).expect("closure");
        let telescope = Telescope::reference(1);
        let outcome = extract_candidate_families(&signature, &closure, &telescope, 0);
        let certificate = certify_independent_basis(
            &signature,
            &closure,
            &telescope,
            extracted(&outcome),
            DeclarationEvidence::None,
        )
        .expect("basis");
        assert_eq!(certificate.targets.len(), 2);
        assert_eq!(certificate.revised_nu, 1);
        assert_eq!(certificate.minimum_basis.minimum_cardinality, 1);
        assert!(
            certificate
                .coverage_derivations
                .iter()
                .any(|coverage| matches!(
                    coverage.rule,
                    CoverageRule::RegisteredCompletion {
                        rule: RegisteredCompletionRule::UniverseDecodeFromSamePackageFormation,
                        ..
                    }
                ))
        );
    }

    #[test]
    fn ambient_universe_application_cannot_borrow_the_package_rule() {
        let signature = SealedSignature::from_telescopes(vec![(1, Telescope::reference(1))]);
        let closure = predecessor_closure(&signature).expect("closure");
        let telescope = Telescope::reference(2);
        let outcome = extract_candidate_families(&signature, &closure, &telescope, 1);
        let certificate = certify_independent_basis(
            &signature,
            &closure,
            &telescope,
            extracted(&outcome),
            DeclarationEvidence::None,
        )
        .expect("basis");
        assert_eq!(certificate.revised_nu, 0);
        assert!(
            !certificate
                .coverage_derivations
                .iter()
                .any(|coverage| matches!(
                    coverage.rule,
                    CoverageRule::RegisteredCompletion {
                        rule: RegisteredCompletionRule::UniverseDecodeFromSamePackageFormation,
                        ..
                    }
                ))
        );
    }

    #[test]
    fn proto_unit_and_exact_completion_each_have_one_atom() {
        let stage_one = Telescope::reference(1);
        let signature_one = SealedSignature::from_telescopes(vec![(1, stage_one.clone())]);
        let closure_one = predecessor_closure(&signature_one).expect("closure");
        let formation_telescope = Telescope::reference(2);
        let formation_overlay = InductiveFormationOverlay::new(
            formation_telescope.clone(),
            InductiveHeadDeclaration::one_nullary(
                FreshExportSite::new(signature_one.digest(), 0),
                DeclarationType::existing(Expr::Univ),
            ),
        );
        let formation = issue_inductive_formation(&signature_one, &formation_overlay)
            .expect("formation evidence");
        let formation_outcome =
            extract_candidate_families(&signature_one, &closure_one, &formation_telescope, 1);
        let formation_basis = certify_independent_basis(
            &signature_one,
            &closure_one,
            &formation_telescope,
            extracted(&formation_outcome),
            DeclarationEvidence::Formation {
                overlay: &formation_overlay,
                evidence: &formation,
            },
        )
        .expect("formation basis");
        assert_eq!(formation_basis.revised_nu, 1);
        assert_eq!(formation.pending_slots().len(), 1);

        let signature_two = SealedSignature::from_telescopes(vec![
            (1, stage_one),
            (2, formation_telescope.clone()),
        ]);
        let closure_two = predecessor_closure(&signature_two).expect("closure");
        let completion_telescope = Telescope::reference(3);
        let completion_overlay = ConstructorCompletionOverlay::new(
            completion_telescope.clone(),
            ConstructorDeclaration::fill_nullary(
                FreshExportSite::new(formation.resulting_context_digest(), 0),
                formation.pending_slots()[0].clone(),
            ),
        );
        let completion =
            issue_constructor_completion(&signature_two, &formation, &completion_overlay)
                .expect("completion evidence");
        let completion_outcome =
            extract_candidate_families(&signature_two, &closure_two, &completion_telescope, 2);
        let completion_basis = certify_independent_basis(
            &signature_two,
            &closure_two,
            &completion_telescope,
            extracted(&completion_outcome),
            DeclarationEvidence::Completion {
                formation_overlay: &formation_overlay,
                formation: &formation,
                overlay: &completion_overlay,
                evidence: &completion,
            },
        )
        .expect("completion basis");
        assert_eq!(completion_basis.revised_nu, 1);
        assert!(completion.is_inductive_signature_closed());
        assert_eq!(
            completion_basis
                .targets
                .iter()
                .filter(|target| matches!(
                    target.kind,
                    SemanticTargetKind::GeneratedOneConstructorEliminator
                        | SemanticTargetKind::GeneratedOneConstructorBeta
                ))
                .count(),
            2
        );
        assert_eq!(
            completion_basis
                .basis_atoms
                .iter()
                .filter(|atom| matches!(atom.provenance, ProvenanceTag::LiveDemand { .. }))
                .count(),
            1
        );
    }
}
