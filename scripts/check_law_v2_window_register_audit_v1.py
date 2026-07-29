#!/usr/bin/env python3
"""Replay the immutable issued-incomplete Law-V2 window-audit certificate."""

from __future__ import annotations

import argparse
import json
import pathlib
import sys
from typing import Any


REPO = pathlib.Path(__file__).resolve().parents[1]
DEFAULT_AUDIT = REPO / "docs" / "law_v2_window_register_audit_v1.json"
DEFAULT_H3 = REPO / "docs" / "law_v2_h3_inductive_completion_v1.json"
DEFAULT_H4 = REPO / "docs" / "law_v2_h4_continuation_v1.json"
DEFAULT_BOOTSTRAP = (
    REPO / "crates" / "pen-law" / "assets" / "law_v2a_registered_bootstrap_v1.json"
)
DEFAULT_REGISTRY = (
    REPO / "crates" / "pen-law" / "assets" / "law_v2_profile_registry_v1.json"
)

OBLIGATIONS = frozenset(
    {
        "kernel_clause_basis",
        "marginal_semantic_family_basis",
        "typed_predecessor_weakening",
        "provenance_injection",
        "family_versus_instance_quotient",
    }
)
RESIDUAL_SUFFIXES = {
    "kernel_clause_basis": "KERNEL-FIRST-IRREDUCIBILITY",
    "marginal_semantic_family_basis": "MARGINAL-FAMILY-CARRIER",
    "typed_predecessor_weakening": "TYPED-FAMILY-WEAKENING",
    "provenance_injection": "SR2-INJECTION",
    "family_versus_instance_quotient": "FAMILY-INSTANCE-QUOTIENT",
}
ALLOWED_OUTCOMES = (
    "ProvenClears",
    "ProvenFails",
    "UndefinedAudit",
    "Unknown",
)


class AuditError(RuntimeError):
    """The issued audit is inconsistent with its bound evidence."""


def load_json(path: pathlib.Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        raise AuditError(f"{path} must contain a JSON object")
    return value


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AuditError(message)


def require_exact_keys(value: dict[str, Any], keys: set[str], scope: str) -> None:
    actual = set(value)
    if actual != keys:
        added = sorted(actual - keys)
        missing = sorted(keys - actual)
        raise AuditError(
            f"{scope} field surface drifted; added={added or 'none'}, "
            f"missing={missing or 'none'}"
        )


def verify_inputs(
    audit: dict[str, Any],
    h3: dict[str, Any],
    h4: dict[str, Any],
    bootstrap: dict[str, Any],
    registry: dict[str, Any],
) -> None:
    bindings = audit["input_bindings"]
    h3_report = h3["report"]
    h4_report = h4["report"]
    require(
        bindings["h3_artifact"] == "docs/law_v2_h3_inductive_completion_v1.json"
        and bindings["h4_artifact"] == "docs/law_v2_h4_continuation_v1.json",
        "input artifact path binding drifted",
    )

    require(h3["status"] == "proven", "H3 artifact is not Proven")
    require(h4["status"] == "halted", "H4 artifact is not Halted")
    require(
        bindings["h3_profile_id"] == h3_report["profile_id"],
        "H3 profile binding drifted",
    )
    require(
        bindings["h3_semantic_manifest_digest"]
        == h3_report["semantic_manifest_digest"],
        "H3 semantic digest drifted",
    )
    require(
        bindings["h3_result_digest"] == h3_report["result_digest"],
        "H3 result digest drifted",
    )
    require(
        bindings["h3_composite_provenance_digest"]
        == h3_report["verifier"]["composite_provenance_digest"],
        "H3 composite provenance drifted",
    )
    require(
        bindings["h4_profile_id"] == h4_report["profile_id"],
        "H4 profile binding drifted",
    )
    require(
        bindings["h4_continuation_semantic_manifest_digest"]
        == h4_report["continuation_semantic_manifest_digest"],
        "H4 continuation semantic digest drifted",
    )
    require(
        bindings["h4_result_digest"] == h4_report["result_digest"],
        "H4 result digest drifted",
    )
    require(
        bindings["h4_composite_provenance_digest"]
        == h4_report["verifier"]["composite_provenance_digest"],
        "H4 composite provenance drifted",
    )
    require(
        bindings["h4_halt_certificate_digest"]
        == h4_report["halt"]["halt_certificate_digest"],
        "H4 halt certificate drifted",
    )

    events = h3_report["history"]["events"]
    require(len(events) == 3, "H3 must bind exactly three registered events")
    acts = audit["acts"]
    require([act["ordinal"] for act in acts] == [1, 2, 3, 4], "act order drifted")
    for event, act in zip(events, acts[:3], strict=True):
        verify_event_binding(act["event_binding"], event)

    seal = h4_report["free_seal"]
    seal_event = {
        "event_id": seal["event_id"],
        "predecessor": seal["predecessor"],
        "source_identity": seal["source_identity"],
        "binding_identity": seal["binding_identity"],
        "pre_boundary_digest": seal["pre_boundary_digest"],
        "post_boundary_digest": seal["post_boundary_digest"],
        "extension_digest": seal["extension_digest"],
    }
    verify_event_binding(acts[3]["event_binding"], seal_event)

    termination = audit["profile_a_termination"]
    census = h4_report["prospective_census"]
    require(
        termination["termination"] == "HaltedDebtFree",
        "Profile A is not classified HaltedDebtFree",
    )
    require(
        termination["prospective_stage_ordinal"]
        == h4_report["halt"]["prospective_stage_ordinal"],
        "prospective ordinal drifted",
    )
    for key in (
        "extracted_port_count",
        "derived_port_count",
        "live_orbit_count",
        "extraction_complete",
        "derivability_complete",
        "expiration_complete",
    ):
        require(termination[key] == census[key], f"termination census field {key} drifted")
    require(
        census["extracted_port_count"] == census["derived_port_count"]
        and census["live_orbit_count"] == 0
        and census["extraction_complete"] is True
        and census["derivability_complete"] is True
        and census["expiration_complete"] is True,
        "H4 evidence does not derive debt freedom",
    )
    require(
        h4_report["halt"]["no_new_structure_generated"] is True
        and h4_report["halt"]["disposition"]
        == "halted_complete_empty_live_obligation_profile"
        and h4_report["halt"]["positive_cost_exclusion_digest"].startswith("blake3:"),
        "H4 halt evidence is incomplete",
    )

    verify_bound_surfaces(audit, h3_report, h4_report, bootstrap)
    verify_profile_registry(bindings, registry)


def verify_bound_surfaces(
    audit: dict[str, Any],
    h3_report: dict[str, Any],
    h4_report: dict[str, Any],
    bootstrap: dict[str, Any],
) -> None:
    acts = audit["acts"]
    bootstrap_acts = bootstrap["acts"]
    require(len(bootstrap_acts) == 3, "bootstrap asset does not contain three acts")
    require(
        bootstrap["claimed_artifact_digest"]
        == h3_report["bootstrap"]["bootstrap_artifact_digest"],
        "bootstrap artifact digest drifted",
    )
    for audit_act, bootstrap_act in zip(acts[:3], bootstrap_acts, strict=True):
        normalized = bootstrap_act["normalized_extension"]["declarations"]
        declaration_ids = [declaration["id"] for declaration in normalized]
        require(
            audit_act["bound_public_clause_sources"] == declaration_ids,
            f"Act {audit_act['ordinal']} public-clause binding drifted",
        )
        require(
            bootstrap_act["source_identity"]
            == audit_act["event_binding"]["source_identity"]
            and bootstrap_act["binding_identity"]
            == audit_act["event_binding"]["binding_identity"]
            and bootstrap_act["predecessor_checked_boundary_digest"]
            == audit_act["event_binding"]["pre_boundary_digest"]
            and bootstrap_act["checked_boundary_digest"]
            == audit_act["event_binding"]["post_boundary_digest"],
            f"Act {audit_act['ordinal']} bootstrap binding drifted",
        )

    use_family = h3_report["use_family"]
    compute_families = h3_report["compute_families"]
    require(len(compute_families) == 1, "H3 demand-family surface is not the frozen pair")
    expected_families = [
        (use_family["family_id"], "G-Use", use_family["birth_support_digests"]),
        (
            compute_families[0]["family_id"],
            "G-Compute",
            compute_families[0]["birth_support_digests"],
        ),
    ]
    require(acts[0]["observed_demand_families"] == [], "Act 1 gained a demand family")
    for act in acts[1:3]:
        observed = act["observed_demand_families"]
        require(len(observed) == 2, f"Act {act['ordinal']} family binding is incomplete")
        for row, (family_id, family_kind, birth_support) in zip(
            observed, expected_families, strict=True
        ):
            require(
                row["family_id"] == family_id
                and row["family_kind"] == family_kind
                and row["birth_support_includes_act"] is True
                and row["authority_for_nu"] is False
                and act["event_binding"]["event_id"] in birth_support,
                f"Act {act['ordinal']} observed family binding drifted",
            )

    seal = h4_report["free_seal"]
    equation = h3_report["equation_extension"]
    require(
        acts[3]["bound_public_clause_sources"]
        == [seal["extension"]["id"], equation["clause_digest"]],
        "Act 4 public-clause binding drifted",
    )
    require(
        seal["equation_set_digest"] == equation["set_digest"],
        "Act 4 equation-set binding drifted",
    )

    decisions = h4_report["prospective_census"]["decisions"]
    observed = acts[3]["observed_demand_families"]
    require(len(observed) == len(decisions) == 2, "Act 4 demand binding is incomplete")
    expected_kinds = {"use": "G-Use", "compute": "G-Compute"}
    for row, decision in zip(observed, decisions, strict=True):
        require(
            row["family_id"] == decision["family_id"]
            and row["family_kind"] == expected_kinds[decision["rule"]]
            and row["disposition_after_seal"].lower() == decision["disposition"]
            and row["authority_for_nu"] is False,
            "Act 4 observed demand disposition drifted",
        )

    quotient_rows = acts[3]["observed_nonsemantic_quotients"]
    require(len(quotient_rows) == 1, "Act 4 response quotient binding is incomplete")
    quotient = quotient_rows[0]
    require(
        quotient["kind"] == "response_candidate_Q0_Q2_Q3"
        and quotient["digest"] == h3_report["quotient"]["quotient_digest"]
        and quotient["complete"] == h3_report["quotient"]["complete"]
        and quotient["authority_for_family_instance_quotient"] is False,
        "Act 4 response quotient binding drifted",
    )
    require(
        all(not act["observed_nonsemantic_quotients"] for act in acts[:3]),
        "a bootstrap act gained an unbound quotient",
    )


def verify_profile_registry(
    bindings: dict[str, Any], registry: dict[str, Any]
) -> None:
    require(
        registry["schema_version"] == 1
        and registry["registry_id"] == "law-v2-profile-registry-v1",
        "profile registry identity drifted",
    )
    profiles = registry["profiles"]
    require(len(profiles) == 3, "profile registry surface drifted")
    profile_a = profiles[0]
    require(
        profile_a["id"] == bindings["h3_profile_id"]
        and profile_a["status"] == "frozen_executed"
        and profile_a["semantic_manifest_digests"]
        == [
            bindings["h3_semantic_manifest_digest"],
            bindings["h4_continuation_semantic_manifest_digest"],
        ]
        and profile_a["result_digests"]
        == [bindings["h3_result_digest"], bindings["h4_result_digest"]]
        and profile_a["termination"] == "HaltedDebtFree"
        and not profile_a["privileged_candidate_fixtures"],
        "Profile A registry binding drifted",
    )
    for profile in profiles[1:]:
        require(
            profile["status"] == "proposed_not_adopted"
            and not profile["semantic_manifest_digests"]
            and not profile["result_digests"]
            and profile["result"] is None
            and profile["termination"] is None
            and not profile["artifact_paths"]
            and not profile["privileged_candidate_fixtures"],
            f"proposed profile {profile['id']} carries execution authority",
        )


def verify_event_binding(binding: dict[str, Any], event: dict[str, Any]) -> None:
    for key in (
        "event_id",
        "predecessor",
        "source_identity",
        "binding_identity",
        "pre_boundary_digest",
        "post_boundary_digest",
        "extension_digest",
    ):
        require(binding[key] == event[key], f"event binding {key} drifted")


def verify_audit_logic(audit: dict[str, Any]) -> tuple[int, int]:
    require_exact_keys(
        audit,
        {
            "schema_version",
            "audit_id",
            "date",
            "status",
            "outcome",
            "allowed_outcomes",
            "semantic_register",
            "input_bindings",
            "excluded_authority",
            "global_prerequisite",
            "acts",
            "calculation",
            "profile_a_termination",
            "blockers",
            "next_phase_authority",
        },
        "audit",
    )
    require(audit["schema_version"] == 1, "audit schema is not v1")
    require(
        audit["audit_id"] == "law-v2-window-register-audit-v1",
        "audit identity drifted",
    )
    require(
        audit["date"] == "2026-07-29"
        and audit["status"] == "issued_incomplete"
        and audit["outcome"] == "UndefinedAudit",
        "issued V1 disposition drifted",
    )
    require(
        tuple(audit["allowed_outcomes"]) == ALLOWED_OUTCOMES,
        "outcome vocabulary drifted",
    )
    semantic_register = audit["semantic_register"]
    require_exact_keys(
        semantic_register,
        {
            "authority",
            "family_carrier",
            "marginal_definition",
            "kernel_cost_definition",
            "previous_window_formula",
            "direct_act_efficiency_formula",
            "comparison_convention",
            "selective_authority",
        },
        "semantic register",
    )
    require(
        semantic_register["previous_window_formula"]
        == "(nu_3 + nu_2)/(kappa_3 + kappa_2)"
        and semantic_register["direct_act_efficiency_formula"] == "nu_4/kappa_4"
        and semantic_register["comparison_convention"]
        == "descriptive_strict_clearance_only_not_an_adopted_gate"
        and semantic_register["selective_authority"] is False,
        "semantic-register formula or authority drifted",
    )
    require_exact_keys(
        audit["input_bindings"],
        {
            "h3_artifact",
            "h3_profile_id",
            "h3_semantic_manifest_digest",
            "h3_result_digest",
            "h3_composite_provenance_digest",
            "h4_artifact",
            "h4_profile_id",
            "h4_continuation_semantic_manifest_digest",
            "h4_result_digest",
            "h4_composite_provenance_digest",
            "h4_halt_certificate_digest",
        },
        "input bindings",
    )
    excluded = audit["excluded_authority"]
    require_exact_keys(
        excluded,
        {
            "legacy_structural_vector_consumed",
            "archived_semantic_vector_consumed",
            "archived_candidate_catalog_consumed",
            "future_outcome_consumed",
            "reason",
        },
        "excluded authority",
    )
    for key in (
        "legacy_structural_vector_consumed",
        "archived_semantic_vector_consumed",
        "archived_candidate_catalog_consumed",
        "future_outcome_consumed",
    ):
        require(excluded[key] is False, f"forbidden authority consumed: {key}")
    prerequisite = audit["global_prerequisite"]
    require_exact_keys(
        prerequisite,
        {
            "id",
            "status",
            "required_certificate",
            "repository_disposition",
            "blocks",
        },
        "global prerequisite",
    )
    require(
        prerequisite["id"] == "GF2-FINITE-SCHEMA-CARRIER-COMPLETENESS"
        and prerequisite["status"] == "missing"
        and prerequisite["repository_disposition"]
        == (
            "The formal appendix lists the finite GF2 carrier, checker, "
            "and completeness certificate as not implemented."
        ),
        "global GF2 prerequisite disposition drifted",
    )
    require(
        prerequisite["blocks"]
        == [
            "marginal semantic-family enumeration",
            "typed family weakening",
            "family-versus-instance quotient",
            "SR2 provenance injection completeness",
            "zero residual audit gaps",
        ],
        "global prerequisite blocker surface drifted",
    )

    complete_acts = 0
    residual_count = 0
    for act in audit["acts"]:
        require_exact_keys(
            act,
            {
                "ordinal",
                "act_kind",
                "event_binding",
                "bound_public_clause_sources",
                "observed_demand_families",
                "observed_nonsemantic_quotients",
                "audit_obligations",
                "kappa_sem",
                "nu_sem",
                "zero_residual_audit_gaps",
                "residual_gaps",
            },
            f"Act {act.get('ordinal', '?')}",
        )
        expected_kind = (
            "registered_bootstrap"
            if act["ordinal"] in (1, 2, 3)
            else "direct_eliminator_free_seal"
        )
        require(
            act["act_kind"] == expected_kind,
            f"Act {act['ordinal']} kind binding drifted",
        )
        require_exact_keys(
            act["event_binding"],
            {
                "event_id",
                "predecessor",
                "source_identity",
                "binding_identity",
                "pre_boundary_digest",
                "post_boundary_digest",
                "extension_digest",
            },
            f"Act {act['ordinal']} event binding",
        )
        obligations = act["audit_obligations"]
        require(set(obligations) == OBLIGATIONS, "act obligation surface drifted")
        for name, obligation in obligations.items():
            require_exact_keys(
                obligation,
                {"status", "certified_input", "missing", "residual_id"},
                f"Act {act['ordinal']} {name}",
            )
            require(
                obligation["status"] == "incomplete",
                f"Act {act['ordinal']} cannot certify {name} while the global carrier is missing",
            )
            require(
                obligation["residual_id"]
                == f"A{act['ordinal']}-{RESIDUAL_SUFFIXES[name]}",
                f"Act {act['ordinal']} {name} residual binding drifted",
            )
        for family in act["observed_demand_families"]:
            family_keys = (
                {
                    "family_id",
                    "family_kind",
                    "birth_support_includes_act",
                    "authority_for_nu",
                }
                if act["ordinal"] in (2, 3)
                else {
                    "family_id",
                    "family_kind",
                    "disposition_after_seal",
                    "authority_for_nu",
                }
            )
            require_exact_keys(
                family, family_keys, f"Act {act['ordinal']} observed family"
            )
        for quotient in act["observed_nonsemantic_quotients"]:
            require_exact_keys(
                quotient,
                {
                    "kind",
                    "digest",
                    "complete",
                    "authority_for_family_instance_quotient",
                },
                f"Act {act['ordinal']} observed quotient",
            )
        residuals = act["residual_gaps"]
        residual_count += len(residuals)
        for gap in residuals:
            require_exact_keys(
                gap, {"id", "disposition"}, f"Act {act['ordinal']} residual"
            )
        residual_ids = {gap["id"] for gap in residuals}
        require(
            len(residuals) == len(residual_ids) == len(OBLIGATIONS),
            f"Act {act['ordinal']} must disclose five distinct residuals",
        )
        require(
            all(gap["disposition"] == "open_missing_certificate" for gap in residuals),
            "an audit residual has an unrecognized disposition",
        )
        require(
            {obligation["residual_id"] for obligation in obligations.values()}
            == residual_ids,
            "obligation-to-residual binding is not total",
        )
        require(
            act["kappa_sem"] is None and act["nu_sem"] is None,
            "an incomplete act must not carry kappa or nu",
        )
        require(
            act["zero_residual_audit_gaps"] is False and residuals,
            "an incomplete act must disclose residual gaps",
        )

    calculation = audit["calculation"]
    require_exact_keys(
        calculation,
        {
            "previous_window_bar",
            "direct_act_efficiency",
            "comparison",
            "reason",
        },
        "calculation",
    )
    require(
        complete_acts == 0,
        "issued V1 cannot contain a complete act while its global carrier is missing",
    )
    require(residual_count == 20, "issued V1 must disclose exactly 20 residual gaps")
    require(
        calculation["previous_window_bar"] is None
        and calculation["direct_act_efficiency"] is None
        and calculation["comparison"] == "unrankable",
        "undefined audit must not publish a numerical comparison",
    )
    require(
        audit["next_phase_authority"]["productivity_experiment_authorized"] is False,
        "undefined window cannot authorize a productivity experiment",
    )
    next_authority = audit["next_phase_authority"]
    require_exact_keys(
        next_authority,
        {
            "productivity_experiment_authorized",
            "contextual_live_run_authorized",
            "nonadopted_self_containment_draft_authorized",
        },
        "next-phase authority",
    )
    require(
        next_authority["contextual_live_run_authorized"] is False
        and next_authority["nonadopted_self_containment_draft_authorized"] is True,
        "window audit granted unauthorized contextual execution authority",
    )
    termination = audit["profile_a_termination"]
    require_exact_keys(
        termination,
        {
            "termination",
            "prospective_stage_ordinal",
            "extracted_port_count",
            "derived_port_count",
            "live_orbit_count",
            "extraction_complete",
            "derivability_complete",
            "expiration_complete",
        },
        "Profile A termination",
    )
    blockers = audit["blockers"]
    require(len(blockers) == 3, "window-audit blocker surface drifted")
    expected_blockers = [
        (
            "WINDOW-AUDIT-B1",
            "Implement and certify the finite GF2 raw-schema carrier, typed normalizer, and sound-and-complete natural-family quotient.",
        ),
        (
            "WINDOW-AUDIT-B2",
            "Reissue each act's first-irreducible kernel basis and typed semantic-family weakening map.",
        ),
        (
            "WINDOW-AUDIT-B3",
            "Construct the act-local SR2 provenance injections and close every recorded residual before assigning kappa or nu.",
        ),
    ]
    for blocker, (expected_id, expected_summary) in zip(
        blockers, expected_blockers, strict=True
    ):
        require_exact_keys(blocker, {"id", "disposition", "summary"}, "blocker")
        require(
            blocker["id"] == expected_id
            and blocker["disposition"] == "blocking"
            and blocker["summary"] == expected_summary,
            "blocker disposition drifted",
        )
    return complete_acts, residual_count


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--audit", type=pathlib.Path, default=DEFAULT_AUDIT)
    parser.add_argument("--h3", type=pathlib.Path, default=DEFAULT_H3)
    parser.add_argument("--h4", type=pathlib.Path, default=DEFAULT_H4)
    parser.add_argument("--bootstrap", type=pathlib.Path, default=DEFAULT_BOOTSTRAP)
    parser.add_argument("--registry", type=pathlib.Path, default=DEFAULT_REGISTRY)
    args = parser.parse_args()

    try:
        audit = load_json(args.audit)
        h3 = load_json(args.h3)
        h4 = load_json(args.h4)
        bootstrap = load_json(args.bootstrap)
        registry = load_json(args.registry)
        verify_inputs(audit, h3, h4, bootstrap, registry)
        complete_acts, residual_count = verify_audit_logic(audit)
    except (AuditError, KeyError, TypeError, ValueError) as error:
        print(json.dumps({"status": "invalid", "reason": str(error)}, sort_keys=True))
        return 1

    print(
        json.dumps(
            {
                "status": "valid",
                "outcome": audit["outcome"],
                "complete_acts": complete_acts,
                "residual_gap_count": residual_count,
                "profile_a_termination": "HaltedDebtFree",
            },
            sort_keys=True,
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
