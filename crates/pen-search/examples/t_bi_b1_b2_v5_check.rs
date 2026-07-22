use pen_search::act_local_semantic_provenance_v5::issue_reference_act_local_semantic_sequence_v5;
use serde_json::json;

fn main() {
    match issue_reference_act_local_semantic_sequence_v5() {
        Ok(sequence) => {
            let per_stage = sequence
                .packages
                .iter()
                .map(|package| {
                    json!({
                        "stage": package.stage,
                        "bridges": package.bridges.len(),
                        "families": package.semantic_families.len(),
                        "marginal": package.marginal_family_count,
                        "credited": package.credited_semantic_family_count,
                        "role_families": package.proved_family_declaration_count,
                        "role_impossibilities": package.theorem_impossibility_declaration_count,
                        "named_role_residuals": package.named_role_residual_count,
                        "named_quotient_residuals": package.named_quotient_residual_count,
                        "total_named_residuals": package.total_named_residual_count,
                        "anchor_impossibilities": package.theorem_anchor_impossibility_count,
                        "quotient": {
                            "proved": package.unified_quotient.proved,
                            "members": package.unified_quotient.current_members.len(),
                            "within_candidate_pairs": package.unified_quotient.within_candidate_decisions.len(),
                            "predecessor_pairs": package.unified_quotient.predecessor_decisions.len(),
                            "core_members": package.unified_quotient.current_members.iter().filter(|member| matches!(member.presentation, pen_search::act_local_semantic_provenance_v5::V5UnifiedFamilyPresentation::CoreExpr { .. })).count(),
                            "ordinary_members": package.unified_quotient.current_members.iter().filter(|member| matches!(member.presentation, pen_search::act_local_semantic_provenance_v5::V5UnifiedFamilyPresentation::OrdinarySchema2 { .. })).count(),
                            "cubical_members": package.unified_quotient.current_members.iter().filter(|member| matches!(member.presentation, pen_search::act_local_semantic_provenance_v5::V5UnifiedFamilyPresentation::CubicalPath { .. })).count(),
                            "core_inventory_exact": package.unified_quotient.fresh_core_inventory_exact,
                            "ordinary_inventory_exact": package.unified_quotient.fresh_ordinary_inventory_exact,
                            "cubical_inventory_exact": package.unified_quotient.fresh_cubical_inventory_exact,
                            "equivalence_closure": {
                                "proved": package.unified_quotient.equivalence_closure.proved,
                                "base_pairs": package.unified_quotient.equivalence_closure.base_decisions.len(),
                                "named_residuals": package.unified_quotient.equivalence_closure.named_residual_count,
                                "only_cross_constructor_is_bridge": package.unified_quotient.equivalence_closure.only_cross_surface_constructor_is_exact_expr_schema2_bridge,
                                "cubical_cross_edges_absent": package.unified_quotient.equivalence_closure.no_base_equality_edge_touches_cubical_and_noncubical,
                                "failed_base_decisions": package.unified_quotient.equivalence_closure.base_decisions.iter().filter(|decision| !decision.proved).collect::<Vec<_>>(),
                            },
                        },
                        "a3": {
                            "schemes": package.exact_a3_capability.fresh_scheme_count,
                            "instances": package.exact_a3_capability.fresh_instance_count,
                            "orbits": package.exact_a3_capability.fresh_orbit_count,
                            "exported_orbits": package.exact_a3_capability.fresh_exported_orbit_count,
                            "constructed_exported_outputs": package.exact_a3_capability.constructed_exported_output_count,
                            "named_live_export_residuals": package.exact_a3_capability.named_live_export_residual_count,
                            "no_fallback": package.exact_a3_capability.no_constructed_exported_a3_fallback,
                            "proved": package.exact_a3_capability.proved,
                        },
                        "finite_closure": package.finite_closure.proved,
                        "roles_resolved": package.every_role_declaration_resolved,
                        "anchors_total": package.every_marginal_family_credited_or_theorem_impossible,
                        "anchor_nonreuse": package.local_anchor_nonreuse_holds,
                        "stage1_r1": package.stage1_r1_preserved,
                        "stage2_internal": package.stage2_constitutive_question_not_assumed,
                        "stage9_boundary": package.stage9_boundary_decided_by_relation_theorem,
                        "r2": package.r2_generated_instance_not_exported,
                        "t_bi_b1": package.t_bi_b1_proved,
                        "t_bi_b2": package.t_bi_b2_proved,
                        "failed_role_resolutions": package.role_resolutions.iter().filter_map(|resolution| {
                            if resolution.resolved && !matches!(resolution.resolution, pen_search::act_local_semantic_provenance_v5::V5RoleResolution::NamedResidual { .. }) {
                                None
                            } else {
                                Some(json!({
                                    "declaration": resolution.declaration_id,
                                    "kind": resolution.constructor_search.role_kind,
                                    "search_proved": resolution.constructor_search.proved,
                                    "resolution": resolution.resolution,
                                }))
                            }
                        }).collect::<Vec<_>>(),
                        "a3_residuals": package.exact_a3_capability.orbit_capabilities.iter().filter_map(|orbit| {
                            if matches!(orbit.disposition, pen_search::act_local_semantic_provenance_v5::V5ExactA3OrbitDisposition::NamedResidual { .. }) {
                                Some(json!({
                                    "orbit": orbit.orbit_id,
                                    "scheme": orbit.scheme_id,
                                    "representative": orbit.representative_instance_id,
                                    "constructor": orbit.rule_constructor,
                                    "required_output": orbit.required_output,
                                    "disposition": orbit.disposition,
                                }))
                            } else {
                                None
                            }
                        }).collect::<Vec<_>>(),
                        "stage1_evidence": if package.stage == 1 { Some(json!({
                            "completion_equality": package.stage1_exact_completion_equality_derivation_hash,
                            "carrier_cases": package.stage1_carrier_role_case_derivation_hashes,
                            "role_resolutions": package.role_resolutions,
                            "families": package.semantic_families,
                        })) } else { None },
                        "nu": package.semantic_family_nu,
                    })
                })
                .collect::<Vec<_>>();
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "package_hashes": sequence.exact_package_derivation_hashes,
                    "intrinsic_sequence_seal": sequence.intrinsic_sequence_seal,
                    "role_declarations": sequence.role_declaration_count,
                    "proved_family_declarations": sequence.proved_family_declaration_count,
                    "theorem_impossibility_declarations": sequence.theorem_impossibility_declaration_count,
                    "named_role_residuals": sequence.named_role_residual_count,
                    "named_quotient_residuals": sequence.named_quotient_residual_count,
                    "named_a3_residuals": sequence.named_a3_residual_count,
                    "total_named_residuals": sequence.total_named_residual_count,
                    "silent_residue": sequence.silent_residue_count,
                    "t_bi_b1": sequence.t_bi_b1_proved_on_sequence,
                    "t_bi_b2": sequence.t_bi_b2_proved_on_sequence,
                    "semantic_vector": sequence.packages.iter().map(|package| package.semantic_family_nu).collect::<Vec<_>>(),
                    "per_stage": per_stage,
                }))
                .expect("v5 summary serializes")
            );
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
