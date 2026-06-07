# Grammar Ablation Experiment

Last updated: 2026-04-19

## Executive Summary

This experiment tested whether the current 15-step PEN trajectory is genuinely
driven by the objective, or whether the late endpoint depends materially on the
MBTT grammar already containing temporal primitives at fixed audit cost. The
specific concern was that the canonical late finish, culminating in step `15`
(`DCT`), might be easier to discover because the search grammar already treats
constructors such as `Next` and `Eventually` as ambient primitives.

To test that concern, we preserved the canonical lane as a read-only control
and ran the same engine under two deliberately hostile grammar profiles:
`no_temporal`, which removes the temporal primitives entirely, and
`linear_exponential_swap`, which replaces them with a different unary pair of
comparable audit cost. The central question was whether the search still
followed the same broad architecture,
`bootstrap -> geometric ascent -> framework abstraction -> late terminal shell`,
and whether it could recover a step-`15` terminal comparable to the canonical
`DCT` finish.

The result is mixed but clear. Under both hostile grammars, the system still
replays the canonical sequence exactly through step `14` / `Hilbert`. That is
strong evidence that the early and middle trajectory, including the geometric
and framework phases, is not merely an artifact of the canonical temporal
vocabulary. However, neither hostile profile recovers a stored step-`15`
terminal analogous to the canonical `DCT` finish. On current head and at the
current experimental scale, the executable MBTT grammar is therefore
load-bearing for the concrete final step.

This is an empirical boundary, not a proof of impossibility. The experiment
shows that the hostile profiles we implemented and audited do not regenerate
the stored step-`15` endpoint. It does not prove that no future hostile grammar
or different technical route could ever do so.

## Context And Motivation

The canonical executable lane currently reproduces a 15-step trajectory with
the late phase
`cohesion -> connections -> curvature -> metric -> Hilbert -> DCT`.
In the baseline grammar, `Flat`, `Sharp`, `Disc`, `Shape`, `Next`, and
`Eventually` are all primitive constructors with fixed audit costs. That makes
the temporal vocabulary available to the search from the start, which creates a
legitimate reviewer concern: perhaps the late temporal-cohesive shell is not
being discovered independently, but is instead partially subsidized by the
grammar.

Kolmogorov-invariance-style reasoning is not enough to answer that objection at
15-step scale. An `O(1)` grammar constant can matter a great deal this early in
the trajectory. The right response was therefore not a philosophical argument,
but an executable ablation: make the grammar hostile to the current terminal
reading and rerun the system.

The goal was not to build a fully general grammar plugin system. The goal was
to produce a publishable empirical answer to a narrow question: does the
observed phase structure survive when the grammar is made hostile to the
current temporal endpoint?

## Experimental Design

The experiment was designed around three principles.

First, the canonical lane had to remain intact. Existing certified evidence was
treated as read-only, and all ablation work lived under new configs, new run
IDs, and new report surfaces.

Second, the canonical profile had to remain behaviorally identical by default.
To support the experiment cleanly, the codebase was extended with an explicit
`grammar_profile` setting threaded through configuration, run metadata,
enumeration, admissibility, inspection, and reporting. This let the same
engine run under multiple named grammar profiles without rewriting historical
artifacts.

Third, the hostile profiles had to be concrete and limited in scope. We chose
two intentionally different negative controls:

- `canonical_mbtt_v1`: the executable control grammar.
- `no_temporal`: remove `Next` and `Eventually` from new search runs and forbid
  temporal-shell clause generation.
- `linear_exponential_swap`: replace the temporal pair with a different unary
  pair, intended to test whether the late lane only wants "some extra unary
  operator pair plus closure shell" rather than specifically temporal
  vocabulary.

The comparison criteria were straightforward:

- how far exact replay with the canonical run survives,
- whether the broad phase shape survives,
- where the first empirical break occurs,
- and whether any hostile profile recovers a late terminal shell analogous to
  step `15`.

## What We Ran

The canonical control run was
`runs/grammar-ablation-baseline-v15-initial`. It completed through step `15`
under `grammar_profile: canonical_mbtt_v1` and reproduced the current `DCT`
finish with `nu = 103`, `kappa = 8`, and `rho = 103/8`.

The first hostile run was
`runs/grammar-ablation-no-temporal-v15-initial`. This profile is the strongest
negative control against the criticism that `Next` and `Eventually` were
"waiting in the grammar all along." Its stored manifest ended as a stale-owner
failure before writing a final terminal status, but the live step-`15`
evidence remained useful once inspected on current head.

The second hostile run was
`runs/grammar-ablation-linear-exponential-v15-initial`. This profile tested a
different idea: maybe the late shell does not require temporal vocabulary as
such, but only a different pair of primitive unary operators at comparable
cost.

In addition to running the profiles, we added instrumentation needed to tell an
honest story about failure. In the `no_temporal` lane, current head now records
the grammar profile in manifests, refreshes stale running manifests during
inspection, exposes explicit handoff checkpoints around telescope enumeration,
and splits DFS-local rejection counts into structurally disconnected versus
connected-but-unqualified leaves. That work mattered because it separated a
real search boundary from a tooling artifact.

## What We Found

The baseline behaved exactly as expected. The canonical control reproduced the
full stored sequence through step `15`.

Both hostile profiles also behaved similarly up to a point: each preserved
exact replay through step `14` / `Hilbert`. This means the experiment did not
show an early collapse. The phases
`bootstrap -> geometric ascent -> framework abstraction`
survived under both hostile grammars. The first empirical break was therefore
late, at step `15`, not earlier.

The hostile profiles diverged sharply at that final step.

In `no_temporal`, the search did not recover any stored step-`15` terminal.
The step-`15` live window was large and active rather than empty. It reached
`generated_raw_surface = 30789`,
`enumerated_candidates = 12478`, and
`prefix_states_explored = 29034`.
The dominant rejections were connectivity-local:
`dfs_leaf_connectivity_rejections = 16548`, split into
`dfs_leaf_disconnected_rejections = 15730` and
`dfs_leaf_connected_unqualified_rejections = 818`.
Downstream candidate-filter counters remained `0`.
The important point is that this was not a superficial "nothing happened"
failure. The hostile grammar generated a substantial step-`15` search surface,
but the wall appeared inside the connectivity witness path and was dominated by
structural disconnection.

In `linear_exponential_swap`, the failure was different and much sharper. This
run also did not recover any stored step-`15` terminal, but it failed on a
tiny hostile shell rather than a broad search wall. The run reported
`failure_note: no atomic candidates were generated for step 15`. The live
surface reached only `raw_catalog_telescope_count = 1`,
`generated_raw_surface = 8`,
`prefix_states_explored = 8`,
`prefixes_created = 1`, and a single
`dfs_leaf_connected_unqualified_rejections = 1` with
`dfs_leaf_disconnected_rejections = 0`.
This profile therefore did not merely "struggle" with the canonical terminal;
it failed to open a meaningful candidate space for a step-`15` analogue at
all.

## Interpretation

The cleanest summary is that the experiment found robustness through step `14`
and grammar sensitivity at step `15`.

That distinction matters. The hostile grammars did not destroy the broad
trajectory. They preserved exact accepted-step parity through the geometric and
framework phases, which is genuine evidence against the strongest form of the
objection that the entire sequence was simply encoded in the canonical MBTT
grammar.

At the same time, the hostile grammars did fail exactly where the canonical
story becomes most specific: the concrete temporal-cohesive terminal finish.
Neither hostile profile recovered a stored step-`15` analogue of the canonical
`DCT` endpoint. The right conclusion is therefore not "the whole trajectory is
fake," but also not "the endpoint is grammar-independent." The supported claim
is narrower: the early and middle structure is robust, while the concrete final
step is grammar-sensitive on current head and at the current scale.

This also clarifies what the experiment does and does not establish. It does
establish that the implemented hostile profiles failed to regenerate the full
stored 15-step terminal outcome. It does not establish a universal impossibility
result. A future hostile profile, further search redesign, or different late
family might yet recover a comparable terminal shell. We simply do not have
that evidence now, and the current paper should not claim it.

## Empirical Conclusions

The experiment supports the following conclusions.

- The canonical control still reproduces the full 15-step sequence, including
  the step-`15` `DCT` finish.
- Both hostile grammar profiles preserve exact replay through step `14` /
  `Hilbert`.
- The broad phase structure survives hostile grammar changes through the
  framework-abstraction corridor.
- Neither hostile profile recovers a stored step-`15` terminal analogous to the
  canonical `DCT` finish.
- The `no_temporal` failure is a large connectivity-dominated late wall; the
  `linear_exponential_swap` failure is a small connected-but-unqualified
  collapse with no atomic candidates.
- The strongest honest claim is therefore that the current MBTT grammar is
  load-bearing for the concrete final step, but not obviously for the earlier
  geometric and framework structure.

As a result, manuscript language was narrowed to treat step `15` as a
canonical-grammar result rather than as grammar-independent executable
evidence. The lane is now in monitoring mode rather than active expansion: the
current boundary stands unless new hostile evidence or new claim language
creates a concrete contradiction.
