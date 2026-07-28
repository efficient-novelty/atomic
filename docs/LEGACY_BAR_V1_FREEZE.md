# Legacy bar v1 freeze

Status: immutable profile and partial manifest recorded; full physical replay
freeze remains incomplete.

This document defines how the current bar-optimizing engine is to be
preserved as `legacy_bar_v1`. It does not promote that engine to Law V2 and
records the immutable profile and partial verification manifest already
created. It does not claim that the full artifact bundle or complete
verification manifest exists.

## 1. Purpose

The legacy engine remains valuable as:

- a reproducible historical implementation;
- an oracle corpus;
- a performance laboratory;
- a regression baseline; and
- a source of negative controls.

It is not the specification of the blind algorithm. Its results are
`LegacyStructuralV1` testimony unless a separate Law V2 certificate proves a
law-level fact.

## 2. Baseline to preserve

The Phase-0 audit identified this repository baseline:

```text
source commit:
924133af85f81c41cd4aa23b660febbcb13b99fa

current profile source:
configs/strict_canon_guarded.toml

search profile:
strict_canon_guarded

grammar profile:
canonical_mbtt_v1

configured endpoint:
until_step = 15

window:
window_depth = 2

selector:
minimal_positive_overshoot
```

The source commit identifies tracked content only. A freeze manifest must also
record dirty and untracked inputs. In particular,
`docs/autonomous_genesis_plan.md` and
`docs/app_a_two_laws_formal_axioms.tex` were supplied outside the baseline
commit and must not be implied to have governed the historical executable.

The legacy implementation currently exposes target-bearing behavior through,
among other locations:

- `crates/pen-core/src/telescope.rs`:
  `Telescope::reference` and `all_reference_telescopes`;
- `crates/pen-type/src/elaborate.rs`:
  `SEALED_CANDIDATE_HASHES` and `SealedSignature::genesis_del_h15`;
- `crates/pen-type/src/obligations.rs`: `StructuralDebt` and named
  `requires_*_package` predicates;
- `crates/pen-type/src/admissibility.rs`: `StructuralFamily`,
  stage-indexed bootstrap handling, named focus policies, and exact clause
  bands;
- `crates/pen-search/src/accept.rs`: bar clearance, overshoot ranking, and a
  deterministic canonical key;
- `crates/pen-search/src/config.rs`: `until_step` and `selector`;
- `crates/pen-search/src/engine.rs`: the configured step loop and
  target-shaped search paths;
- `crates/pen-cli/src/human.rs` and `xtask/src/main.rs`: human step labels;
  and
- `tests/fixtures/trajectory/reference_steps_until_15.json`: the frozen
  reference trajectory.

This inventory is descriptive, not exhaustive. The oracle-firewall migration
must perform a complete dependency and source audit.

## 3. Frozen semantics

`legacy_bar_v1` preserves, rather than repairs:

- configured termination at step 15;
- the current fixed grammar and stage-indexed policies;
- structural `nu` as calculated by the legacy evaluator;
- the historical Step-15 structural value `nu = 103`;
- bar clearance and minimal positive overshoot;
- deterministic presentation ordering and tie behavior;
- current checkpoint, resume, telemetry, and report schemas; and
- the accepted reference trajectory and its recorded hashes.

All such numeric extension values must be exposed with the register
`LegacyStructuralV1`. The freeze must not rename them
`SemanticFamilyV2`.

The normative appendix reports a corrected historical structural sum of 358,
while an existing older halt document reports 359. Phase 0 must preserve the
underlying artifacts and disclose this disagreement; it must not silently
choose or normalize a total. The authoritative frozen value is whatever the
versioned replay manifest derives from the pinned source, configuration, and
fixtures.

## 4. Required freeze bundle

The physical freeze must contain:

```text
legacy_bar_v1/
    manifest.json
    source-provenance.json
    dependency-graph.json
    config.toml
    Cargo.lock
    schemas/
    reference-telescopes/
    reference-trajectory/
    expected-hashes/
    run-artifacts/
    test-results/
    toolchain.json
```

`manifest.json` must bind:

- source commit and dirty-state inventory;
- exact bytes or cryptographic digests of every input;
- Rust toolchain and relevant platform metadata;
- feature set and dependency graph;
- resource settings and worker count;
- random seeds, if any;
- accepted candidate and canonical hashes;
- structural `(nu, kappa)` records with register tags;
- bars, overshoots, and tie keys with diagnostic/testimony tags;
- checkpoint and output schema versions;
- commands used to reproduce the run; and
- exact output and test-artifact digests.

An artifact not named or transitively bound by the manifest is not part of
the freeze.

## 5. Reproduction gate

The physical freeze is complete only when a clean checkout can:

1. restore the pinned toolchain and configuration;
2. build the legacy binary;
3. replay the accepted reference trajectory;
4. reproduce the pinned candidate/canonical hashes and structural records;
5. exercise checkpoint/resume and deterministic replay;
6. regenerate the freeze manifest without unexplained drift; and
7. archive any expected platform variance explicitly.

Representative existing tests and fixtures may seed this gate, but passing
the current test suite alone is not a freeze manifest.

## 6. Separation from Law V2

The legacy lane may depend on the future oracle crate. The Law V2 production
lane may not depend on the legacy lane, oracle, diagnostics, reference
fixtures, or this freeze bundle.

Allowed direction:

```text
lawful sealed artifacts ---> legacy diagnostics/comparison
legacy artifacts ---------> post-run oracle comparison
```

Forbidden direction:

```text
legacy winner/bar/score ---> Law V2 demand, synthesis, acceptance, or halt
oracle expectation -------> Law V2 production dependency closure
```

Baseline parity is a diagnostic result, never Law V2 acceptance evidence.

## 7. Change policy

After the physical freeze:

- historical artifacts are immutable;
- fixes occur in a new versioned legacy profile;
- schema migrations preserve the original bytes and record the transform;
- replay drift is disclosed rather than repaired in place; and
- new Law V2 work is implemented in a separate lane.

The name `legacy_bar_v1` must not be reused for behavior that differs from its
manifest.

## 8. Provenance status and open blockers

### Recovered old appendix

`docs/app_a_two_laws_formal_axioms.tex` states that the former bar-law text is
retained unedited as `docs/app_a_two_laws_formal_axioms_old.tex`. The exact
source has been recovered byte-for-byte from
`book/appendices/app_a_two_laws_formal_axioms_old.tex` and is bound in
`configs/legacy_bar_v1.freeze.json` by SHA-256
`75aa7e45edd8bb2e19d53acfa535f1eea3e0fdb88ea8956e64f5010bf6fbc443`.
The recovery closes this historical-source item while leaving the manifest
explicitly partial.

### Stage-4 quotient status

Legacy `legacy_bar_v1` records the single enacted Stage-4 branch chosen by its
historical selector. That does not decide the Law V2 cone. The normative
appendix's unresolved four-versus-two act-equivalence tension must remain
outside the legacy replay claim.

### Freeze bundle not yet complete

`configs/legacy_bar_v1.toml` and
`configs/legacy_bar_v1.freeze.json` now pin the testimony profile, baseline
source blobs, primary oracle inputs, observed toolchain, and baseline test
disposition. The manifest is explicitly `partial`: the full artifact
directory, clean-checkout replay, checkpoint/resume replay, and platform
variance archive remain outstanding. Until those are produced and replayed,
the Phase-0 freeze exit gate remains open.

Several historical certificates bind the exact bytes of
`crates/pen-core/src/telescope.rs`. Consequently, moving the legacy reference
constructors is a versioned artifact migration, not a safe mechanical rename.
PR 1 quarantines a checked copy and proves the new Law V2 production closure
can build without either copy; it deliberately leaves those historical source
bindings unchanged.
