# Agent C continuation — TRUNC-ER: the endpoint-dependent PathCon realizer

> **Execution update (2026-07-19): completed at the registered d = 1
> scope.** Full H15 and exact B5 replay the two-key endpoint-dependent
> bundle under an explicit source-bound formal eliminator-premise context;
> restricted C-1 checks all four fixed-context variable maps, and the
> successor Agent A handoff is exactly `2/2/5/10`. General C6 and the
> inherited arbitrary-typed-instance gap remain open. See
> `docs/TRUNC_ENDPOINT_REALIZER_RESULT.md` and
> `docs/trunc_endpoint_realizer_v1.json`. HIST-CERT v3 has now consumed this
> handoff and discharged F-B2 while leaving every full historical total
> partial; see `docs/HIST_CERT_V3_RESULT.md`.

**Date:** 2026-07-19. **Target obstruction:**
`C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER`
(`docs/KERNEL_BRIDGE_V3_ELEMENT_OVERLAY_RESULT.md`).

**Standing:** construction task. Admissible context: the v3 element-overlay
result, `docs/boundary_adjudication_proposal.md` and
`docs/element_overlay_adjudication.md` (both ADOPTED),
`pen-type::cubical` as shipped. Forbidden: the bar value in any derivation
path; any Step-16 candidate scoring. Owned module: `pen-type::cubical`
(continues under Agent C ownership); no `candidate_join` changes are
required by this brief.

## Mission

The registered Trunc diagram — in `A : Type; x, y : Trunc(A)`:
`squash(x,y)(0) = x`, `squash(x,y)(1) = y` — is typed at declaration level
but unrealizable at computation level: the current cubical
`PathConstructor`/`PathMethod` compute with one constant base and cannot
carry two endpoint-dependent methods. Build the endpoint-dependent realizer
at d = 1, earning Trunc's typed *declared-boundary* bundle of exactly
2 keys, and thereby completing the four-package handoff (2/2/5/10) that
Agent A's HIST-CERT v3 rerun consumes.

Trunc has exposed one stack layer per generation (rule → diagram →
declaration → computation). This brief closes the computation layer for
d = 1 only. General face-indexed motive methods at arbitrary d remain part
of `C6_V3_GENERAL_HISTORICAL_TERM_LEVEL_COMPLETION` and are **out of
scope** — do not gold-plate.

## Work items

1. **TR-1 (endpoint-dependent constructor).** Extend the cubical
   `PathConstructor` so a d = 1 constructor may declare boundary terms per
   face (the registered `x`/`y` endpoints), with the constant-base form as
   the degenerate case. The existing constant-base code path and its
   issued tokens must remain replay-identical — extend alongside or
   parametrize, but archival compatibility decides, not aesthetics.
2. **TR-2 (face-indexed method).** Extend `PathMethod` so the eliminator's
   method for an endpoint-dependent constructor is a dependent path in the
   motive over the constructor, connecting the method images of the
   declared endpoints (the PathP-shaped method), with:
   - base scrutinees computing through the endpoint instantiation;
   - constructor scrutinees computing to the method's dependent path;
   - typed neutrals giving stuck terms at the motive instance.
3. **TR-3 (boundary-aware transport).** The d = 1 basis under the adopted
   charging convention is exactly 2 keys: `Beta` and one principal
   transport. Deliver the typed realizer for the boundary-aware transport
   (dependent `coe` along the squash cell with varying endpoints) and
   extend the proved key bijection (`Beta → Beta`,
   `PrincipalTransport(1) → Kan(1,1)`) to the endpoint-dependent case.
4. **TR-4 (prefix replay).** Issue and replay the Trunc bundle in both the
   full H15 signature and the exact historical predecessor signature B5
   (digest `blake3:0ee6911b…` per HIST-CERT). Wrong prefixes must fail
   before token issue, as in the v3 overlay result.
5. **TR-5 (substitution scope).** The squash family is parameterized by
   `A; x, y`. Prove instantiation under the C-1 *restricted* theorem
   (sort-identical variable images). Where arbitrary typed images would be
   needed, record the dependence on the inherited gap
   `C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION` explicitly — do not
   extend C-1 here, and do not silently rely on it.
6. **TR-6 (artifact).** Create-new
   `docs/trunc_endpoint_realizer_v1.json` +
   `docs/TRUNC_ENDPOINT_REALIZER_RESULT.md`, with full replay and mutation
   battery (token hash, boundary term, method shape, key-bijection entry,
   prefix digest, conditionality flags). Provide the handoff API naming
   all four typed bundles 2/2/5/10 for Agent A.

## Conservativity gate

All archival artifacts must replay byte-identically after this work:
HIST-CERT v1 (`blake3:e840f99c…`), BOUNDARY-AUDIT v1 (`blake3:76c38976…`),
TDC cubical v3 (`blake3:075791e9…`), schemas 3/4/5
(`blake3:e4cb89fe…`, `blake3:6a6c3ccf…`, `blake3:ee5a0883…`). Any drift is
falsifier F-TR2 and invalidates the realizer, not the archives.

## Falsifiers / exit conditions

- **F-TR1.** The registered Trunc diagram proves *unrealizable* at the
  computation layer after honest effort — i.e., Trunc exposes yet another
  stack layer. Report verbatim and stop: what fails is the adopted
  interpretation or the cubical fragment's expressiveness, and which one
  is the user's adjudication (versioned successor), never a checker
  loosening.
- **F-TR2.** Any archival replay drifts → realizer invalid (above).
- **F-TR3.** The endpoint-dependent d = 1 basis yields ≠ 2 keys. Under
  `boundary-charge-zero-reference-only-v1` the count must equal the
  constant case (1 + d² = 2, c(b) = 0); a third key would contradict the
  trace-derived charging convention — that is a discovery about the
  convention, reported, never absorbed by re-labelling keys.
- **F-TR4.** Any reliance on arbitrary-term instantiation not flagged as
  the inherited C-1 gap → certificate invalid; the dependence must be
  named, not used.

## Done criteria

`C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER` retired by replayed typed
tokens (Beta + boundary-aware transport) in both H15 and B5 signatures;
key bijection extended; archival digests unchanged; handoff API for Agent
A complete with all four bundles; the result document explicitly does
**not** claim general C6, independence/exhaustiveness, C8, intended-schema
classification, or any Step-16/halt conclusion.

**Downstream (not this brief):** Agent A's create-new HIST-CERT v3 rerun
(sharpened F-B2: must reproduce 7/8/10/18 and 2/2/5/10 while consuming the
v3 prefix tokens and this brief's Trunc bundle); then the Schema2(W)
grammar brief (the remaining long pole, with C8 folded in).
