pub mod accept;
pub mod act_local_provenance;
pub mod act_local_provenance_v2;
pub mod act_local_provenance_v3;
pub mod act_local_semantic_provenance_v4;
pub mod act_local_semantic_provenance_v5;
pub mod agent_a_hist_cert_v4;
pub mod ambient_wrapper_domain;
pub mod bc1_depth_two_completeness_v1;
pub mod bc2_boundary_provenance_join_v1;
pub mod bc3_parent_row_disposition_v1;
pub mod bi0_semantic_register_v4;
pub mod bi0_semantic_register_v5;
pub mod bi0_semantic_register_v6;
pub mod bi1b_prefix_general_sweep_v1;
pub mod bi2_branch_finales_v1;
pub mod bi4_cone_assembly_v2;
pub mod bounds;
pub mod branch_bound;
pub mod branch_prefix_general_provenance_v4;
pub mod branch_semantic_provenance_v3;
pub mod bridge_completion_execution_v1;
// BI-1 continuation/finale engines are intentionally crate-private.  The
// public capability boundary is `branch_invariance_program`, whose only
// issuance path replays a sealed, passing BI-0 certificate first.
pub(crate) mod branch_invariance;
pub(crate) mod branch_invariance_finale;
/// The public, BI-0-gated branch-invariance capability surface.
///
/// Low-level continuation issuance is deliberately not externally callable:
///
/// ```compile_fail
/// use pen_search::branch_invariance::execute_branch_continuation;
/// ```
///
/// Reissuance-based branch replay is gated for the same reason:
///
/// ```compile_fail
/// use pen_search::branch_invariance_program::replay_bi_branch_certificate;
/// ```
pub mod branch_invariance_program;
pub(crate) mod branch_invariance_resume_v1;
pub mod branch_invariance_sweep_v3;
pub mod candidate_join;
pub mod candidate_join_v5;
pub mod certified_halt;
pub mod chronological_slot_map;
pub mod chronological_slot_map_v2;
pub mod chronological_slot_map_v3;
pub mod chronological_slot_map_v4;
pub mod chronological_slot_map_v5;
pub mod completed_basis_membership;
pub mod config;
pub mod contextual_formation_coherence_v3;
pub mod dedupe;
pub mod diversify;
pub mod e2b_quotient_closure;
pub mod e5_a3_successor;
pub mod e5_a3_successor_v2;
pub mod e5_demand_projection;
pub mod e5_future_hole_finale;
pub mod e5_future_hole_finale_v2;
pub mod egp_v2_bootstrap;
pub mod engine;
pub mod enumerate;
pub mod expand;
pub mod falsifier_disposition;
pub mod frontier;
pub mod global_e4_assembly;
pub mod global_e4_assembly_v10;
pub mod global_e4_assembly_v11;
pub mod global_e4_assembly_v2;
pub mod global_e4_assembly_v3;
pub mod global_e4_assembly_v4;
pub mod global_e4_assembly_v5;
pub mod global_e4_assembly_v6;
pub mod global_e4_assembly_v7;
pub mod global_e4_assembly_v8;
pub mod global_e4_assembly_v9;
pub mod halting_probe;
pub mod ip1_certification_boundary;
pub mod m3_e7_e8_bridge_v1;
pub mod motif;
pub mod motive_typed_open_specialization_v4;
pub mod milestone_certificate_v1;
pub mod narrative;
pub mod naturality_orbit_transport;
pub mod phase5b_history_certification;
pub mod phase5b_reselection_v2;
pub mod phase5b_reselection_v3;
pub mod prefix_cache;
pub mod prefix_memo;
pub mod priority;
pub mod r_t2_future_hole_confluence;
pub mod r_t2_future_hole_confluence_v2;
pub mod resume;
pub mod scheduler;
pub mod semantic_nu_invariance_v1;
pub mod semantic_nu_transport_maps_v2;
pub mod semantic_reselection;
pub mod stage4_option_a_execution_v1;
pub mod stage4_semantic_parsimony_v1;
pub mod stage4_semantic_parsimony_v2;
pub mod stage4_semantic_parsimony_v3;
pub mod state;
pub mod step16_automaton;
pub mod step16_semantic_exhaustion;
pub mod support_comprehension_artifact_v5;
pub mod support_comprehension_hardening_v6;
pub mod support_comprehension_v5;
pub mod t_bf1_prefix;
pub mod t_bf3_enactment_equivalence;
pub mod t_bf_tie_protocol_rerun;
pub mod t_bi_intrinsic_isolation_v1;
pub mod t_bi_intrinsic_isolation_v2;
pub mod t_bi_intrinsic_isolation_v3;
pub mod t_bi_nu1_regression;
pub mod t_bi_nu1_regression_v2;
pub mod t_bi_nu1_regression_v3;
pub mod t_bi_nu1_regression_v4;
pub mod t_bi_nu1_regression_v5;
pub mod t_bi_nu1_regression_v6;
pub mod t_d2_1_domain_build_v1;
pub mod t_d2_1_operational_domain_v2;
pub mod t_d2_1_operational_domain_v3;
pub mod t_d2_2_completeness_attempt_v1;
pub mod t_d2_2_operational_surjectivity_v2;
pub mod t_d2_3_candidate_content_stream_v2;
pub mod t_d2_3_candidate_provenance_key_schema_v1;
pub mod t_sm1a_contextual_formation_v4;
pub mod dnfq_theorem_layer_v1;
pub mod uc1_prediction_score;
pub mod uc1_scoring_v1;
pub mod worker;
