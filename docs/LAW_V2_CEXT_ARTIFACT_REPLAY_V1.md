# Law V2 C_ext artifact replay V1

**Date:** 2026-07-28

**Status:** `REPLAYED_EVIDENCE_ONLY`

**Law-V2 authority:** none

## 1. Result

The archived coherence-depth package was extracted into a new empty
directory and its aggregate checker completed successfully. The replay
checked 66 Agda targets, the theorem map, the theorem-facing lexical
postulate/primitive boundary, and seven case-study fixtures.

This establishes reproducibility of the packaged checker under the recorded
environment. It does not establish a GF2-to-\(C_{\mathrm{ext}}\) adequacy
theorem, and no recovered module has been copied into an Atomic production
crate.

## 2. Artifact identity

Source archive:

```text
C:\DEV\coherence-depth\coherence_depth_LMCS_artifact_v0.1.2.zip
```

| Object | SHA-256 |
|---|---|
| ZIP archive | `735ef5964ab0be332f5000ca68e2709585b6dec2415964cc132ecc2114c25b84` |
| Canonical ZIP payload tree | `77fd396764df4d233b0dce2baa8357d6d6f609b2aeee904ba862ab0bc5c80f8a` |
| LMCS TeX | `afc5907e468fd1c867a57fdf9d66858b6e4e843bc32957aab4d76262368105ff` |
| LMCS PDF | `bb3039bb636bd62f2e3f3ea6a1f21c2e64614e79b9e30c66470f0f76af50e854` |
| `paper-map.yaml` | `e69d161a85d55889e9cdda556c4c7ad46b4fa0c3770d6833ebeac03ab6e838db` |
| `RawStructuralSyntax.agda` | `1444b2dda39033e0bc860cc756739953ab598e2490fe7520891194364b84cfd8` |
| `RawStructuralTyping.agda` | `d2c07932fdaa1e905d41aa5f30e4fc1eb4d663394e4ed42d5c0fea850d63bb79` |
| `StructuralHornDecoding.agda` | `d6e9f13822f368305c48708c0aceb8906e9b7e56b35537f9cb3f5960ed3ff5d1` |

The ZIP contains 143 files under one
`coherence_depth_zenodo/` root. A file-by-file SHA-256 comparison against the
extracted source package reported zero missing or mismatched files.

The canonical payload-tree digest is defined as SHA-256 of the UTF-8
concatenation, in ordinal relative-path order, of:

```text
relative/path NUL lowercase-file-sha256 LF
```

Directory entries and generated `.agdai` files are excluded.

Repository provenance:

```text
archive tag commit: 38a5762de928d088bb4a23ba918d25fa79826c58
local repository HEAD: 1ac054ff5177c29e4fc955ffd77d1a94fde631e4
describe: v0.1.2-lmcs-artifact-2-g1ac054f
```

The two later commits add the packaged ZIP and release metadata and revise
planning/checklist documents. The Agda sources, paper, and theorem map used
here have no diff from the tagged core. Both the coherence-depth and Atomic
worktrees were clean before replay.

## 3. Toolchain

| Component | Replayed value |
|---|---|
| Host Agda | `2.8.0` |
| Host Agda executable | `C:\Users\halvo\.local\bin\agda.exe` |
| Host Agda executable SHA-256 | `43c85d257d42bea11ee637058bf93a5bb2c6b988e736a0f7c2fa4a6de9c53024` |
| Cubical checkout | `C:\Users\halvo\.agda\cubical` |
| Cubical commit | `b150186d2544e7efeddd31e5d14a8b9ecbb100f7` |
| Cubical worktree | clean, detached at the pinned commit |
| Agda library default | `cubical` |
| WSL kernel | `Linux 6.6.87.2-microsoft-standard-WSL2 x86_64` |
| Bash | `GNU bash 5.2.21` |
| Python used by aggregate script | `/usr/bin/python3`, `3.12.3` |
| PowerShell fallback | Windows PowerShell `5.1` |
| Git | `2.52.0.windows.1` |

The aggregate Bash script did not find Agda inside WSL. It used its declared
`powershell.exe` fallback to invoke the digest-recorded Windows Agda
executable.

The package README states the Agda and Cubical pins. Contrary to its wording,
`agda/library_manifest.json` inventories example library objects rather than
the toolchain. The pins are present in the README, trust notes, and CI
workflow; the ZIP contains no independent checksum manifest.

## 4. Cold replay

A fresh extraction containing zero `.agdai` files was created at:

```text
C:\Users\halvo\AppData\Local\Temp\atomic-cext-cold-20260728
```

The exact command was:

```text
bash /mnt/c/Users/halvo/AppData/Local/Temp/atomic-cext-cold-20260728/coherence_depth_zenodo/scripts/check_coherence_depth_artifact.sh
```

Observed run:

| Field | Value |
|---|---|
| Start | `2026-07-28T18:02:17.1330095+02:00` |
| End | `2026-07-28T18:03:35.0978775+02:00` |
| Elapsed | `77.965 s` |
| Agda invocations | `66` |
| Standard-output bytes | `649573` |
| Standard-output lines | `12647` |
| Standard-output SHA-256 | `b857d5c95801c94889478b0352a0d469fe67091eadd3f1a27f8d389886ba2cc0` |
| Standard-error SHA-256 | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| Final success markers | `1` |

The complete output is retained locally at:

```text
C:\Users\halvo\AppData\Local\Temp\atomic-cext-cold-20260728\replay\stdout.log
C:\Users\halvo\AppData\Local\Temp\atomic-cext-cold-20260728\replay\stderr.log
```

Aggregate summaries:

```text
paper-map check passed: 19 entries, 77 modules, 301 names
postulate audit passed: 112 local modules in transitive closure
coherence-depth audit passed: 7 fixtures
coherence-depth artifact check passed
```

The run emitted 1,222 nonfatal
`-W[no]UnsupportedIndexedMatch` warning headers. No other Agda warning code
occurred. These warnings say that definitions using constructor injectivity
will not compute when applied to transports in Cubical Agda. They are part of
the replay record and are not silently treated as absent.

## 5. Paper theorem boundary

The LMCS paper makes the following useful distinctions:

- payload is separate from structural integration trace;
- structural trace has unary action, binary comparison, and higher horn
  roles;
- a higher structural field is derived only with a complete replacement
  witness;
- the contractible object is the total compatible-lid-plus-filler extension,
  not an arbitrary fixed-lid or closed-boundary filler space; and
- transfer outside the fixed raw calculus requires a separate adequacy
  package.

The replacement witness required by the paper contains:

1. a lower public boundary;
2. a cubical replacement term;
3. typing and boundary equations;
4. semantic preservation;
5. allowed-dependency evidence;
6. substitution stability; and
7. public-normalization compatibility.

The paper replacement theorem assumes that package. The Agda
computational-replacement wrapper consumes an already-derived cost
classification or abstract elimination hypothesis; it neither validates the
full package nor constructs a GF2 instance of it.

The machine-readable map records four materially different statuses:

- fully mechanized;
- mechanized for an abstract interface;
- conditional on adequacy; and
- paper-only.

Those boundaries must remain visible in any reuse.

## 6. What the aggregate checker proves

The successful checker establishes:

- every listed Agda target typechecks with the pinned Agda/Cubical pair;
- the checked theorem-facing local import closure contains no lexical local
  `postulate` or `primitive` declaration and uses `--safe`;
- all names listed by `paper-map.yaml` occur in one of their listed modules;
  and
- the seven supplied case-study data fixtures satisfy their audit script.

It does not establish:

- that every record field has been constructed rather than received as an
  argument;
- that a named Agda declaration proves the full prose statement mapped to it;
- that the theorem map describes the LMCS source precisely;
- that the fixed raw surface covers GF2; or
- that the fixed-calculus depth result is an authoritative history-demand
  calculus.

`paper-map.yaml` names `paper/1_coherence_depth.tex`, not the LMCS TeX supplied
for this task. `check_paper_map.py` checks declaration-name occurrence, not
semantic correspondence between a name and a prose theorem.

## 7. Mechanization audit

### 7.1 Reusable positive core

The following are genuine useful components at their stated interfaces:

- `RawStructuralSyntax.agda` defines finite-telescope raw syntax for payload,
  action, comparison, and packaged horn clauses.
- `CubicalOpenBox` defines the artifact's simplified abstract total-extension
  interface and proves its path-singleton contractibility. Its `Filler` does
  not itself encode the paper's dependent side-compatible open extension;
  relating the two is left to the bridge.
- The sources maintain a useful payload/trace split and a narrow adequacy
  boundary.
- The presentation-step and support records provide a vocabulary for a
  finite checked presentation relation.

### 7.2 Assumption-packaging boundaries

The following prevent promotion to Law-V2 authority:

- `RawStructuralTyping.agda` receives sealing, opacity, export soundness, and
  several admissibility claims as fields of type `Type`. Its role witnesses
  classify raw constructors but do not independently build semantic endpoint
  or filler typing.
- `PackagedHornBoundary` proves its displayed support equation by reflexivity;
  it does not construct the paper's full public boundary package.
- `SurfaceNormalizationBridge.agda` fixes a two-layer index, assigns every
  horn `derived` cost by constructor, assigns horns empty normalized support,
  and supplies zero auxiliary primitive/derived collections to its normal
  form.
- `SurfaceToHornImage.agda` maps an already classified role to an image.
  Higher-derived status is reflexive from the preceding assignment, not
  synthesized from an open-box replacement term.
- `PresentationEquivalence.agda` requires support and primitive-cost
  preservation as constructor arguments. Its generic theorems project or
  compose those supplied equalities.
- `StructuralHornDecoding.agda` receives open-box faces, side equations,
  endpoint laws, lower-face availability, a center, and substitution
  stability as fields. Its generated-horn datatype has remote, degenerate,
  and transported cases but no reindexed constructor corresponding to the
  LMCS grammar.
- `StructuralHornToOpenBox.agda` projects an open box already stored in its
  input package, and its displayed isomorphism is the identity instance.
- `DerivedTrace.agda` receives lower-boundary use and replacement soundness as
  fields. `hornDerivedTrace` projects a supplied witness.
- `ReplaceDerivedField.agda` presents its primitive and derived sides over
  the same field type with identity maps; it does not remove a field from a
  dependent telescope.
- `Semantics/RawAdequacy.agda` is a record of required propositions plus the
  identity function on a supplied package, not an implementation of those
  propositions.
- `ExactDepth.agda` obtains its displayed lower-depth exclusions from its
  constructed obligation datatype being empty at those indices; the separate
  swap example is not used to prove that exact-depth wrapper.

The absence of lexical postulates is accurate, but arbitrary propositions
carried as record fields are not lexical postulates and are outside the
audit's detection criterion.

## 8. Atomic reuse decision

The artifact is accepted as:

- design provenance for the separation of payload and sealing trace;
- a candidate-local `Act`/`Cmp`/`Horn` interface vocabulary;
- evidence that the packaged abstract open-box layer replays; and
- a source of regression fixtures and theorem targets.

It is not accepted as:

- the definition of history-generated \(C(W)\);
- an authoritative GF2 structural classifier;
- a primitive/derived trace decision procedure;
- a complete normalizer or presentation quotient;
- a GF2 horn decoder;
- a GF2 replacement theorem; or
- a generic depth theorem for Atomic.

Before any imported theorem can receive Law-V2 authority, a restricted
GF2-to-\(C_{\mathrm{ext}}\) translator and adequacy proof must preserve:

1. typing;
2. dependent substitution;
3. normalized public support;
4. presentation equivalence;
5. primitive versus derived trace;
6. `Act`/`Cmp`/`Horn` classification;
7. exact realization objects; and
8. raw completeness for the fixed sealing discipline;
9. opacity and export discipline;
10. payload, primitive-trace, exported-interface, and \(\mu\) cardinalities;
    and
11. the full horn computation/replacement package, including the lower public
    boundary, replacement term, typing and boundary equations, semantic
    preservation, allowed-dependency evidence, substitution stability, and
    public-normalization compatibility.

The translator must generate closed reviewed certificate sources and accept
no caller-supplied Agda text. A failed Agda check is `Unknown` or malformed
evidence, never a refutation.

## 9. Remaining block

Artifact recovery is complete. The remaining block is semantic, not
forensic: Atomic still needs an adopted anonymous history-demand calculus and
then a GF2-to-\(C_{\mathrm{ext}}\) adequacy bridge. The accompanying
`LAW_V2_GENESIS_SCHEME_CALCULUS_V1.md` is a proposal for the first item. Until
it is adopted and mechanized, and until the second item is proved where
structural trace is used, the production result remains `Unknown`.
