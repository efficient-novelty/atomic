# Grammar Ablation Gap Plan

Last updated: 2026-04-17
Status: proposed

This file owns the plan for Gap 1: testing whether the current 15-step PEN
trajectory is materially driven by the MBTT grammar's built-in temporal
constructors rather than by the objective alone.

## Contract

- Preserve the current executable canon and claim-lane evidence as read-only
  controls.
- Do not overwrite existing certified run directories, especially
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15`.
- Keep `strict_canon_guarded` as the default authoritative grammar/profile.
- Put all ablation work under new configs, new run IDs, and new report paths.
- Keep the default grammar byte-for-byte behaviorally identical unless an
  explicit ablation profile is selected.

## Gap

The current executable lane uses an MBTT grammar in which `Flat`, `Sharp`,
`Disc`, `Shape`, `Next`, and `Eventually` are all primitive constructors with
fixed audit costs. That means the same vocabulary the late sequence eventually
selects is already ambient in the search grammar. Kolmogorov invariance is not
enough to answer the concrete reviewer objection at a 15-step scale, because an
`O(1)` grammar constant can easily dominate the ranking this early.

The missing evidence is a hostile-grammar ablation:

- remove the temporal primitives entirely, or
- swap them for a deliberately different primitive pair at comparable cost,

and then rerun the engine to see whether the phase structure survives, shifts,
or collapses.

## Goal

Produce a publishable answer to the following question:

Does the trajectory still look like
`bootstrap -> geometric ascent -> framework abstraction -> late terminal shell`
when the grammar is made hostile to the current temporal endpoint?

Either result is valuable:

- robustness strengthens the independence claim,
- divergence proves the grammar is load-bearing and the paper must say so.

## Read-Only Baseline

These remain the control surfaces and must not be mutated by the ablation work:

- `configs/strict_canon_guarded.toml`
- `tests/fixtures/trajectory/reference_steps_until_15.json`
- `skills/pen-atomic/theory/genesis.md`
- `skills/pen-atomic/references/02-target-sequence.md`
- `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15`

The baseline claim to preserve is the current executable canon:

- steps `1..15` in order,
- late phase `10..15 = cohesion/connections/curvature/metric/hilbert/DCT`,
- step `15 = DCT`, `nu = 103`, `kappa = 8`, `rho = 103/8`.

## Chosen Hostile Grammars

Run two deliberately different hostile profiles first.

### Profile A: `no_temporal`

Purpose:

- strongest negative control against the criticism that `Next` and
  `Eventually` were "waiting in the grammar all along".

Definition:

- `Next` and `Eventually` are not available to new search runs,
- no temporal-shell clause family may be emitted,
- any ablation run under this profile halts when no admissible bar-clearer
  remains.

Interpretation:

- if the engine still reaches a structurally comparable late shell, that is
  strong independence evidence,
- if it stalls before the current step `15`, that is still informative and must
  be reported directly.

### Profile B: `linear_exponential_swap`

Purpose:

- test whether the late lane wants a generic "extra unary operator pair plus
  exchange/closure shell" rather than specifically temporal vocabulary.

Definition:

- replace the temporal pair with linear-logic exponentials such as
  `Bang` / `WhyNot`,
- charge them the same primitive audit cost now paid by
  `Next` / `Eventually`,
- allow only the replacement late-shell clauses, not the temporal ones.

Interpretation:

- convergence toward a similar phase shape would support a grammar-robust late
  abstraction story,
- a wildly different terminal would show that the chosen temporal pair is doing
  real work.

Optional follow-on only if the first two are ambiguous:

- `epistemic_swap` with `Know` / `Possible` or similar epistemic primitives at
  the same cost.

## Minimum Engineering Strategy

Do not build a fully generic user-defined grammar system first. That is larger
than the gap requires.

Instead:

1. add a small explicit `GrammarProfile` enum with values like
   `canonical_mbtt_v1`, `no_temporal`, and `linear_exponential_swap`,
2. thread that profile through config, run metadata, enumeration, and audit,
3. keep the default profile exactly equal to the current code path,
4. add only the hostile variants needed for this experiment.

This is the safest route because the current grammar is hardcoded across
`pen-core`, `pen-type`, `pen-search`, reporting, and tests.

## Important Compatibility Rule

Historical artifacts must stay readable.

That means:

- do not remove `Next` / `Eventually` from legacy AST decoding or reporting,
- instead make hostile grammars a property of new search runs and their audit
  rules,
- record the selected grammar profile in every new run manifest and report.

This preserves the current autonomous claim lane while allowing hostile reruns.

## Workstreams

### Workstream 0: Freeze The Controls

Deliverables:

- one explicit baseline note that current ablation work must not modify the
  certified claim-lane bundle,
- one baseline run reference for comparison, preferably the current canonical
  step-15 control plus the frozen trajectory fixture,
- one check that the default grammar profile still reproduces the current
  reference sequence.

Exit criterion:

- ablation plumbing lands with zero drift in the default grammar path.

### Workstream 1: Grammar Profile Plumbing

Likely touch points:

- `crates/pen-core/src/atom.rs`
- `crates/pen-core/src/expr.rs`
- `crates/pen-core/src/encode.rs`
- `crates/pen-core/src/canonical.rs`
- `crates/pen-search/src/enumerate.rs`
- `crates/pen-type/src/admissibility.rs`
- `crates/pen-cli/src/cli.rs`
- `crates/pen-cli/src/cmd_run.rs`
- `crates/pen-cli/src/cmd_inspect.rs`
- `crates/pen-cli/src/human.rs`

Tasks:

- add `grammar_profile` to runtime config and persisted run metadata,
- make enumeration respect the selected constructor inventory,
- make audit cost computation respect the selected bit table,
- expose grammar profile in `inspect` and report output,
- add tests proving the canonical profile is unchanged.

Exit criterion:

- the repo can run the same engine under multiple named grammar profiles without
  touching the existing baseline results.

### Workstream 2: Hostile Late-Family Definitions

Tasks for `no_temporal`:

- make `Next` / `Eventually` unreachable in enumeration,
- disable temporal-shell clause generation under that profile,
- define the honest stopping rule when no admissible continuation exists.

Tasks for `linear_exponential_swap`:

- add the replacement unary constructors,
- assign them the same primitive audit cost as the current temporal pair,
- add a replacement late-shell clause catalog with exchange/closure structure,
- keep the comparison structural, not semantic.

Important rule:

- do not pretend the replacement shell is "temporal" in the write-up,
- use profile-neutral language such as `late terminal shell` or
  `dynamic/closure shell` unless the result genuinely warrants a temporal
  reading.

Exit criterion:

- both hostile profiles can enumerate and score candidates honestly through
  whatever step they can reach.

### Workstream 3: Run Matrix

Create new configs and run IDs rather than reusing existing ones.

Planned configs:

- `configs/grammar_ablation_baseline.toml`
- `configs/grammar_ablation_no_temporal.toml`
- `configs/grammar_ablation_linear_exponential.toml`

Planned runs:

- baseline control rerun under canonical grammar
- `grammar-ablation-no-temporal`
- `grammar-ablation-linear-exponential`

For each run, record:

- max completed step,
- accepted label/hash per step,
- `nu`, `kappa`, `rho`, and bar,
- first divergence step from the canonical trajectory,
- whether the run halts because no bar-clearer exists,
- whether the late phase still forms a coherent shell progression.

Prefer Rust-native inspection/reporting for the core experiment because this
workspace currently does not reliably have `python` on `PATH`.

Exit criterion:

- at least two hostile runs complete and are inspectable end-to-end.

### Workstream 4: Comparison Report

Produce one ablation report, likely
`grammar_ablation_report.md`, that answers four concrete questions:

1. At what step does each hostile grammar first diverge?
2. Does the geometric ascent (`S1`, `Trunc`, `S2`, `S3`, `Hopf` or structural
   equivalents) survive?
3. Does a framework-abstraction block still appear after the geometric core?
4. Is there any late terminal shell analogous to the current step-15 finish?

The report should separate three notions of similarity:

- exact match:
  same accepted step labels/hashes as baseline,
- phase-shape match:
  same broad architecture even if labels differ,
- no match:
  early or late divergence that destroys the current narrative.

Exit criterion:

- one reviewer can read the report and tell whether the grammar is robust,
  partially load-bearing, or fully load-bearing.

### Workstream 5: Claim Update

Update the paper claim according to the evidence, not according to preference.

If both hostile profiles still converge toward a similar architecture:

- strengthen the independence language,
- say the current temporal vocabulary is not uniquely responsible for the late
  shape,
- keep the claim at the level actually supported by the data.

If `no_temporal` halts but `linear_exponential_swap` recovers a late shell:

- narrow the claim:
  geometry/framework structure is robust, but the exact terminal reading is
  grammar-sensitive.

If both hostile profiles diverge badly:

- say plainly that the present grammar is load-bearing,
- remove any suggestion that invariance already protects the concrete 15-step
  trajectory,
- present the result as a boundary on the current paper's independence claim.

Exit criterion:

- the repo and manuscript tell the same honest story.

## Success Conditions

This gap is closed when all of the following are true:

- the default grammar path still reproduces the current canonical sequence,
- at least two hostile grammar profiles have been run,
- each hostile run has an auditable stored artifact trail,
- there is one comparison report with a direct verdict,
- the paper claim is updated to match the observed result.

## Non-Goals

- building a fully open-ended grammar plugin system,
- replacing the current claim-lane certification machinery,
- mutating the existing autonomous claim bundle,
- arguing from invariance without new experimental evidence.

## Why This Is Worth Doing Now

This experiment is unusually high-value for its scope:

- it directly targets the strongest reviewer objection,
- it preserves the current code and evidence rather than discarding them,
- it is a grammar swap plus rerun matrix, not a new theorem,
- and either outcome improves the paper.

If the trajectory survives hostile grammar changes, the independence claim gets
much stronger. If it does not, the repo still gains a clean, honest result
about where the current conclusion actually comes from.
