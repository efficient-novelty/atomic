use super::*;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClaimRemainingOneSurfaceTarget {
    pub prefix_cache_groups: usize,
    pub prefix_cache_candidates: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClaimRemainingOneSurfaceSnapshot {
    pub prefix_states_explored: usize,
    pub prefix_cache_groups: usize,
    pub prefix_cache_candidates: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimRemainingOneClauseMode {
    General,
    ClaimAdmittedOpenBand,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClaimRemainingOneTerminalCandidateFixture {
    pub clause: pen_core::clause::ClauseRec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cached_admissibility_decision: Option<AdmissibilityDecision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connectivity_facts: Option<TerminalClauseConnectivityFacts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nu_facts: Option<TerminalClauseNuFacts>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClaimRemainingOneReplayExpectedSummary {
    pub generated_candidate_count: usize,
    pub admissibility_diagnostics: AdmissibilityDiagnostics,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound: Option<PrefixBound>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub best_accept_primary_rank: Option<TerminalPrefixPrimaryRank>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub best_accept_rank: Option<AcceptRank>,
    pub admitted_candidate_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClaimRemainingOneReplayFixture {
    pub source_profile: String,
    pub step_index: u32,
    pub surface: ClaimRemainingOneSurfaceSnapshot,
    pub prefix_dedupe_key: String,
    pub prefix_telescope: Telescope,
    pub objective_bar: Rational,
    pub library: Library,
    pub admissibility: StrictAdmissibility,
    pub nu_history: Vec<(u32, u32)>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incumbent_rank: Option<AcceptRank>,
    pub clause_mode: ClaimRemainingOneClauseMode,
    pub terminal_candidates: Vec<ClaimRemainingOneTerminalCandidateFixture>,
    pub expected_summary: ClaimRemainingOneReplayExpectedSummary,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClaimRemainingOneReplayOutcome {
    pub summary: ClaimRemainingOneReplayExpectedSummary,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClaimRemainingOneReplayBenchmarkEntry {
    pub fixture_index: usize,
    pub surface: ClaimRemainingOneSurfaceSnapshot,
    pub iterations: usize,
    pub average_micros: u64,
    pub best_micros: u64,
    pub worst_micros: u64,
    pub total_micros: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClaimRemainingOneReplayBenchmark {
    pub entries: Vec<ClaimRemainingOneReplayBenchmarkEntry>,
}

pub const DEFAULT_CLAIM_REMAINING_ONE_SURFACE_TARGETS: [ClaimRemainingOneSurfaceTarget; 5] = [
    ClaimRemainingOneSurfaceTarget {
        prefix_cache_groups: 39,
        prefix_cache_candidates: 144_845,
    },
    ClaimRemainingOneSurfaceTarget {
        prefix_cache_groups: 40,
        prefix_cache_candidates: 147_639,
    },
    ClaimRemainingOneSurfaceTarget {
        prefix_cache_groups: 41,
        prefix_cache_candidates: 154_842,
    },
    ClaimRemainingOneSurfaceTarget {
        prefix_cache_groups: 42,
        prefix_cache_candidates: 157_636,
    },
    ClaimRemainingOneSurfaceTarget {
        prefix_cache_groups: 43,
        prefix_cache_candidates: 160_430,
    },
];

#[derive(Clone)]
pub(super) struct PendingClaimRemainingOneReplayFixture {
    source_profile: String,
    step_index: u32,
    surface: ClaimRemainingOneSurfaceSnapshot,
    prefix_dedupe_key: String,
    prefix_telescope: Telescope,
    objective_bar: Rational,
    library: Library,
    admissibility: StrictAdmissibility,
    nu_history: Vec<(u32, u32)>,
    incumbent_rank: Option<AcceptRank>,
    clause_mode: ClaimRemainingOneClauseMode,
    terminal_candidates: Vec<ClaimRemainingOneTerminalCandidateFixture>,
}

#[derive(Default)]
struct ClaimRemainingOneReplayCaptureState {
    targets: BTreeSet<ClaimRemainingOneSurfaceTarget>,
    fixtures: Vec<ClaimRemainingOneReplayFixture>,
    completed: bool,
    persist_path: Option<PathBuf>,
    persist_error: Option<String>,
}

thread_local! {
    static CLAIM_REMAINING_ONE_REPLAY_CAPTURE: RefCell<Option<ClaimRemainingOneReplayCaptureState>> = const { RefCell::new(None) };
}

pub fn default_claim_remaining_one_surface_targets() -> Vec<ClaimRemainingOneSurfaceTarget> {
    DEFAULT_CLAIM_REMAINING_ONE_SURFACE_TARGETS.to_vec()
}

pub fn write_claim_remaining_one_replay_fixtures(
    path: &Path,
    fixtures: &[ClaimRemainingOneReplayFixture],
) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(fixtures).context("serialize replay fixtures")?;
    fs::write(path, format!("{json}\n")).with_context(|| format!("write {}", path.display()))
}

pub fn read_claim_remaining_one_replay_fixtures(
    path: &Path,
) -> Result<Vec<ClaimRemainingOneReplayFixture>> {
    let json = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let mut fixtures: Vec<ClaimRemainingOneReplayFixture> =
        serde_json::from_str(&json).with_context(|| format!("parse {}", path.display()))?;
    for fixture in &mut fixtures {
        for candidate in &mut fixture.terminal_candidates {
            candidate.nu_facts = Some(
                candidate
                    .nu_facts
                    .take()
                    .unwrap_or_else(|| TerminalClauseNuFacts::from_clause(&candidate.clause))
                    .with_missing_runtime_fields_from_clause(&candidate.clause),
            );
        }
    }
    Ok(fixtures)
}

pub fn write_claim_remaining_one_replay_benchmark(
    path: &Path,
    benchmark: &ClaimRemainingOneReplayBenchmark,
) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(benchmark).context("serialize replay benchmark")?;
    fs::write(path, format!("{json}\n")).with_context(|| format!("write {}", path.display()))
}

pub fn render_claim_remaining_one_replay_benchmark_text(
    benchmark: &ClaimRemainingOneReplayBenchmark,
) -> String {
    let mut lines = Vec::new();
    for entry in &benchmark.entries {
        lines.push(format!(
            "fixture={} surface={}/{}@{} iterations={} avg={}us best={}us worst={}us total={}us",
            entry.fixture_index,
            entry.surface.prefix_cache_groups,
            entry.surface.prefix_cache_candidates,
            entry.surface.prefix_states_explored,
            entry.iterations,
            entry.average_micros,
            entry.best_micros,
            entry.worst_micros,
            entry.total_micros,
        ));
    }
    lines.join("\n")
}

pub fn capture_claim_remaining_one_replay_fixtures(
    surface_targets: &[ClaimRemainingOneSurfaceTarget],
) -> Result<Vec<ClaimRemainingOneReplayFixture>> {
    capture_claim_remaining_one_replay_fixtures_with_seed(surface_targets, &[], None)
}

pub fn capture_claim_remaining_one_replay_fixtures_with_seed(
    surface_targets: &[ClaimRemainingOneSurfaceTarget],
    seed_fixtures: &[ClaimRemainingOneReplayFixture],
    persist_path: Option<&Path>,
) -> Result<Vec<ClaimRemainingOneReplayFixture>> {
    if surface_targets.is_empty() {
        return Ok(seed_fixtures.to_vec());
    }

    let requested_targets = surface_targets.iter().copied().collect::<BTreeSet<_>>();
    let mut fixtures = seed_fixtures
        .iter()
        .filter(|fixture| requested_targets.contains(&fixture_surface_target(fixture)))
        .cloned()
        .collect::<Vec<_>>();
    sort_claim_remaining_one_replay_fixtures(&mut fixtures);
    fixtures.dedup_by(|left, right| fixture_surface_target(left) == fixture_surface_target(right));

    let captured_targets = fixtures
        .iter()
        .map(fixture_surface_target)
        .collect::<BTreeSet<_>>();
    let missing_targets = surface_targets
        .iter()
        .copied()
        .filter(|target| !captured_targets.contains(target))
        .collect::<Vec<_>>();
    if missing_targets.is_empty() {
        if let Some(persist_path) = persist_path {
            write_claim_remaining_one_replay_fixtures(persist_path, &fixtures)?;
        }
        return Ok(fixtures);
    }

    let accepted_prefix = search_bootstrap_prefix_for_profile_with_runtime(
        3,
        2,
        SearchProfile::DesktopClaimShadow,
        FrontierRuntimeLimits::unlimited(),
    )
    .context("build accepted prefix for claim replay capture")?;

    let mut library: Library = Vec::new();
    let mut history = Vec::new();
    for step in &accepted_prefix {
        history.push(DiscoveryRecord::new(
            step.step_index,
            u32::from(step.accepted.nu),
            u32::from(step.accepted.clause_kappa),
        ));
        library.push(LibraryEntry::from_telescope(&step.telescope, &library));
    }

    let window_depth = 2u16;
    let step_index = 4u32;
    let objective_bar = compute_bar(window_depth as usize, step_index, &history).bar;
    let admissibility = strict_admissibility_for_mode(
        step_index,
        window_depth,
        &library,
        AdmissibilityMode::DesktopClaimShadow,
    );
    let retention_policy = summarize_structural_debt(&library, window_depth).retention_policy();
    let nu_history = history
        .iter()
        .map(|record| (record.step_index, record.nu))
        .collect::<Vec<_>>();
    let mut demo_step_budget = None;
    let mut demo_narrative = None;
    let mut progress_observer = None;

    start_capture(
        surface_targets,
        fixtures,
        persist_path.map(Path::to_path_buf),
    );
    let capture_result = discover_realistic_shadow_candidates(
        step_index,
        &library,
        &history,
        admissibility,
        retention_policy,
        objective_bar,
        &nu_history,
        &mut demo_step_budget,
        Instant::now(),
        &mut demo_narrative,
        &mut progress_observer,
    );
    let fixtures = finish_capture()?;
    capture_result.context("capture claim replay fixtures")?;

    let captured_targets = fixtures
        .iter()
        .map(fixture_surface_target)
        .collect::<BTreeSet<_>>();

    let missing_targets = surface_targets
        .iter()
        .copied()
        .filter(|target| !captured_targets.contains(target))
        .collect::<Vec<_>>();
    if !missing_targets.is_empty() {
        bail!(
            "missing claim replay fixtures for surfaces {:?}",
            missing_targets
                .iter()
                .map(|target| format!(
                    "{}/{}",
                    target.prefix_cache_groups, target.prefix_cache_candidates
                ))
                .collect::<Vec<_>>()
        );
    }

    Ok(fixtures)
}

pub fn replay_claim_remaining_one_fixture(
    fixture: &ClaimRemainingOneReplayFixture,
) -> Result<ClaimRemainingOneReplayOutcome> {
    let signature = PrefixSignature::new(
        fixture.step_index,
        &fixture.library,
        &fixture.prefix_telescope,
    );
    let clause_kappa = u16::try_from(fixture.prefix_telescope.clauses.len().saturating_add(1))
        .context("fixture clause_kappa exceeded u16")?;
    let mut prefix_legality_cache = PrefixLegalityCache::default();
    if !prefix_legality_cache.insert_root(
        signature.clone(),
        clause_kappa,
        &fixture.library,
        &fixture.prefix_telescope,
        fixture.admissibility,
        LateFamilySurface::ClaimGeneric,
    ) {
        bail!(
            "failed to seed legality cache for {}",
            fixture.prefix_dedupe_key
        );
    }

    let terminal_candidates = fixture
        .terminal_candidates
        .iter()
        .map(|candidate| TerminalPrefixCandidate {
            clause: &candidate.clause,
            cached_admissibility_decision: candidate.cached_admissibility_decision.clone(),
            connectivity_facts: candidate.connectivity_facts.as_ref(),
            nu_facts: candidate
                .nu_facts
                .as_ref()
                .expect("remaining-one replay fixtures should carry nu facts"),
        })
        .collect::<Vec<_>>();
    let terminal_clauses = match fixture.clause_mode {
        ClaimRemainingOneClauseMode::General => {
            TerminalPrefixClauseCandidates::General(terminal_candidates)
        }
        ClaimRemainingOneClauseMode::ClaimAdmittedOpenBand => {
            TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand(terminal_candidates)
        }
    };
    let summary = compute_terminal_prefix_completion_summary_from_candidates(
        fixture.step_index,
        &fixture.library,
        fixture.admissibility,
        fixture.objective_bar,
        &fixture.nu_history,
        &signature,
        &fixture.prefix_telescope,
        TerminalPrefixSummaryPayload::Compact,
        terminal_clauses,
        fixture.incumbent_rank.as_ref(),
        &mut prefix_legality_cache,
        None,
        Some(RemainingOneSummaryKernelActivationContext {
            prefix_states_explored: fixture.surface.prefix_states_explored,
            prefix_cache_groups: fixture.surface.prefix_cache_groups,
            prefix_cache_candidates: fixture.surface.prefix_cache_candidates,
        }),
    );
    let observed = summary_expectation(&summary);
    if observed != fixture.expected_summary {
        bail!(
            "replay summary mismatch for {} at {}/{}@{}",
            fixture.prefix_dedupe_key,
            fixture.surface.prefix_cache_groups,
            fixture.surface.prefix_cache_candidates,
            fixture.surface.prefix_states_explored,
        );
    }

    Ok(ClaimRemainingOneReplayOutcome { summary: observed })
}

pub fn benchmark_claim_remaining_one_replay_fixtures(
    fixtures: &[ClaimRemainingOneReplayFixture],
    iterations: usize,
) -> Result<ClaimRemainingOneReplayBenchmark> {
    if iterations == 0 {
        bail!("claim replay benchmark iterations must be greater than zero");
    }

    let mut entries = Vec::new();
    for (fixture_index, fixture) in fixtures.iter().enumerate() {
        let mut total_micros = 0u64;
        let mut best_micros = u64::MAX;
        let mut worst_micros = 0u64;
        for _ in 0..iterations {
            let started = Instant::now();
            let _ = replay_claim_remaining_one_fixture(fixture)?;
            let elapsed_micros = u64::try_from(started.elapsed().as_micros()).unwrap_or(u64::MAX);
            total_micros = total_micros.saturating_add(elapsed_micros);
            best_micros = best_micros.min(elapsed_micros);
            worst_micros = worst_micros.max(elapsed_micros);
        }

        entries.push(ClaimRemainingOneReplayBenchmarkEntry {
            fixture_index,
            surface: fixture.surface.clone(),
            iterations,
            average_micros: total_micros / u64::try_from(iterations).unwrap_or(1),
            best_micros,
            worst_micros,
            total_micros,
        });
    }

    Ok(ClaimRemainingOneReplayBenchmark { entries })
}

pub(super) fn capture_should_stop() -> bool {
    CLAIM_REMAINING_ONE_REPLAY_CAPTURE.with(|capture| {
        capture
            .borrow()
            .as_ref()
            .is_some_and(|capture| capture.completed)
    })
}

pub(super) fn prepare_remaining_one_summary_capture(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    prefix_signature: &PrefixSignature,
    prefix_telescope: &Telescope,
    payload: TerminalPrefixSummaryPayload,
    terminal_clauses: &TerminalPrefixClauseCandidates<'_>,
    incumbent_rank: Option<&AcceptRank>,
    remaining_one_summary_kernel_activation: Option<RemainingOneSummaryKernelActivationContext>,
) -> Option<PendingClaimRemainingOneReplayFixture> {
    if !matches!(payload, TerminalPrefixSummaryPayload::Compact) {
        return None;
    }
    let activation = remaining_one_summary_kernel_activation?;
    let target = ClaimRemainingOneSurfaceTarget {
        prefix_cache_groups: activation.prefix_cache_groups,
        prefix_cache_candidates: activation.prefix_cache_candidates,
    };
    let should_capture = CLAIM_REMAINING_ONE_REPLAY_CAPTURE.with(|capture| {
        capture.borrow().as_ref().is_some_and(|capture| {
            !capture.completed
                && capture.targets.contains(&target)
                && !capture.fixtures.iter().any(|fixture| {
                    fixture.surface.prefix_cache_groups == target.prefix_cache_groups
                        && fixture.surface.prefix_cache_candidates == target.prefix_cache_candidates
                })
        })
    });
    if !should_capture {
        return None;
    }

    let (clause_mode, terminal_candidates) = fixture_terminal_candidates(terminal_clauses);
    Some(PendingClaimRemainingOneReplayFixture {
        source_profile: SearchProfile::DesktopClaimShadow.as_str().to_owned(),
        step_index,
        surface: ClaimRemainingOneSurfaceSnapshot {
            prefix_states_explored: activation.prefix_states_explored,
            prefix_cache_groups: activation.prefix_cache_groups,
            prefix_cache_candidates: activation.prefix_cache_candidates,
        },
        prefix_dedupe_key: prefix_signature.dedupe_key(),
        prefix_telescope: prefix_telescope.clone(),
        objective_bar,
        library: library.clone(),
        admissibility,
        nu_history: nu_history.to_vec(),
        incumbent_rank: incumbent_rank.cloned(),
        clause_mode,
        terminal_candidates,
    })
}

pub(super) fn finish_remaining_one_summary_capture(
    pending: Option<PendingClaimRemainingOneReplayFixture>,
    summary: &TerminalPrefixCompletionSummary,
) {
    let Some(pending) = pending else {
        return;
    };
    CLAIM_REMAINING_ONE_REPLAY_CAPTURE.with(|capture| {
        let mut capture = capture.borrow_mut();
        let Some(capture) = capture.as_mut() else {
            return;
        };
        let fixture = ClaimRemainingOneReplayFixture {
            source_profile: pending.source_profile,
            step_index: pending.step_index,
            surface: pending.surface,
            prefix_dedupe_key: pending.prefix_dedupe_key,
            prefix_telescope: pending.prefix_telescope,
            objective_bar: pending.objective_bar,
            library: pending.library,
            admissibility: pending.admissibility,
            nu_history: pending.nu_history,
            incumbent_rank: pending.incumbent_rank,
            clause_mode: pending.clause_mode,
            terminal_candidates: pending.terminal_candidates,
            expected_summary: summary_expectation(summary),
        };
        capture.fixtures.push(fixture);
        sort_claim_remaining_one_replay_fixtures(&mut capture.fixtures);
        capture
            .fixtures
            .dedup_by(|left, right| fixture_surface_target(left) == fixture_surface_target(right));
        capture.completed = capture_is_complete(&capture.targets, &capture.fixtures);
        if let Some(path) = capture.persist_path.as_ref() {
            if capture.persist_error.is_none() {
                if let Err(error) =
                    write_claim_remaining_one_replay_fixtures(path, &capture.fixtures)
                {
                    capture.persist_error = Some(format!(
                        "persist claim replay fixtures to {}: {error:#}",
                        path.display()
                    ));
                }
            }
        }
    });
}

#[cfg(test)]
pub(super) fn build_claim_remaining_one_replay_fixture(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    prefix_signature: &PrefixSignature,
    prefix_telescope: &Telescope,
    incumbent_rank: Option<&AcceptRank>,
    surface: ClaimRemainingOneSurfaceSnapshot,
    terminal_clauses: &TerminalPrefixClauseCandidates<'_>,
    summary: &TerminalPrefixCompletionSummary,
) -> ClaimRemainingOneReplayFixture {
    let (clause_mode, terminal_candidates) = fixture_terminal_candidates(terminal_clauses);
    ClaimRemainingOneReplayFixture {
        source_profile: SearchProfile::DesktopClaimShadow.as_str().to_owned(),
        step_index,
        surface,
        prefix_dedupe_key: prefix_signature.dedupe_key(),
        prefix_telescope: prefix_telescope.clone(),
        objective_bar,
        library: library.clone(),
        admissibility,
        nu_history: nu_history.to_vec(),
        incumbent_rank: incumbent_rank.cloned(),
        clause_mode,
        terminal_candidates,
        expected_summary: summary_expectation(summary),
    }
}

fn fixture_surface_target(
    fixture: &ClaimRemainingOneReplayFixture,
) -> ClaimRemainingOneSurfaceTarget {
    ClaimRemainingOneSurfaceTarget {
        prefix_cache_groups: fixture.surface.prefix_cache_groups,
        prefix_cache_candidates: fixture.surface.prefix_cache_candidates,
    }
}

fn capture_is_complete(
    targets: &BTreeSet<ClaimRemainingOneSurfaceTarget>,
    fixtures: &[ClaimRemainingOneReplayFixture],
) -> bool {
    targets.iter().all(|target| {
        fixtures
            .iter()
            .any(|fixture| fixture_surface_target(fixture) == *target)
    })
}

fn sort_claim_remaining_one_replay_fixtures(fixtures: &mut [ClaimRemainingOneReplayFixture]) {
    fixtures.sort_by_key(|fixture| {
        (
            fixture.surface.prefix_cache_groups,
            fixture.surface.prefix_cache_candidates,
            fixture.surface.prefix_states_explored,
        )
    });
}

fn start_capture(
    surface_targets: &[ClaimRemainingOneSurfaceTarget],
    mut fixtures: Vec<ClaimRemainingOneReplayFixture>,
    persist_path: Option<PathBuf>,
) {
    sort_claim_remaining_one_replay_fixtures(&mut fixtures);
    fixtures.dedup_by(|left, right| fixture_surface_target(left) == fixture_surface_target(right));
    let targets = surface_targets.iter().copied().collect::<BTreeSet<_>>();
    let completed = capture_is_complete(&targets, &fixtures);
    CLAIM_REMAINING_ONE_REPLAY_CAPTURE.with(|capture| {
        let mut capture = capture.borrow_mut();
        *capture = Some(ClaimRemainingOneReplayCaptureState {
            targets,
            fixtures,
            completed,
            persist_path,
            persist_error: None,
        });
    });
}

fn finish_capture() -> Result<Vec<ClaimRemainingOneReplayFixture>> {
    CLAIM_REMAINING_ONE_REPLAY_CAPTURE.with(|capture| {
        let capture = capture.borrow_mut().take().unwrap_or_default();
        if let Some(error) = capture.persist_error {
            bail!("{error}");
        }
        Ok(capture.fixtures)
    })
}

fn fixture_terminal_candidates(
    terminal_clauses: &TerminalPrefixClauseCandidates<'_>,
) -> (
    ClaimRemainingOneClauseMode,
    Vec<ClaimRemainingOneTerminalCandidateFixture>,
) {
    match terminal_clauses {
        TerminalPrefixClauseCandidates::General(clauses) => (
            ClaimRemainingOneClauseMode::General,
            clauses
                .iter()
                .map(|clause| ClaimRemainingOneTerminalCandidateFixture {
                    clause: clause.clause.clone(),
                    cached_admissibility_decision: clause.cached_admissibility_decision.clone(),
                    connectivity_facts: clause.connectivity_facts.cloned(),
                    nu_facts: Some(clause.nu_facts.clone()),
                })
                .collect(),
        ),
        TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand(clauses) => (
            ClaimRemainingOneClauseMode::ClaimAdmittedOpenBand,
            clauses
                .iter()
                .map(|clause| ClaimRemainingOneTerminalCandidateFixture {
                    clause: clause.clause.clone(),
                    cached_admissibility_decision: clause.cached_admissibility_decision.clone(),
                    connectivity_facts: clause.connectivity_facts.cloned(),
                    nu_facts: Some(clause.nu_facts.clone()),
                })
                .collect(),
        ),
    }
}

fn summary_expectation(
    summary: &TerminalPrefixCompletionSummary,
) -> ClaimRemainingOneReplayExpectedSummary {
    ClaimRemainingOneReplayExpectedSummary {
        generated_candidate_count: summary.generated_candidate_count,
        admissibility_diagnostics: summary.admissibility_diagnostics.clone(),
        bound: summary.bound,
        best_accept_primary_rank: summary.best_accept_primary_rank.clone(),
        best_accept_rank: summary.best_accept_rank.clone(),
        admitted_candidate_count: summary.admitted_candidate_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_benchmark_text_includes_surface_coordinates() {
        let benchmark = ClaimRemainingOneReplayBenchmark {
            entries: vec![ClaimRemainingOneReplayBenchmarkEntry {
                fixture_index: 0,
                surface: ClaimRemainingOneSurfaceSnapshot {
                    prefix_states_explored: 24,
                    prefix_cache_groups: 39,
                    prefix_cache_candidates: 144_845,
                },
                iterations: 3,
                average_micros: 12,
                best_micros: 10,
                worst_micros: 15,
                total_micros: 36,
            }],
        };

        let rendered = render_claim_remaining_one_replay_benchmark_text(&benchmark);
        assert!(rendered.contains("39/144845@24"));
        assert!(rendered.contains("avg=12us"));
    }
}
