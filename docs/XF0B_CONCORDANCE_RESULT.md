# XF-0b Concordance Result — The Family Concordance Study

**Campaign:** XF-1. **Plan:** `docs/xf0b_family_concordance_plan.md` /
`blake3:b340555c7f80ac93b76b0fc5a6a590f46ff9b081e471653e9132a31b72e798da`
(frozen; ledger event 005). **Date:** 2026-07-24. **Artifact:**
`docs/xf0b_concordance_v1.json`, result digest
`blake3:907b7a474dcc530fe5b332d3caec51758dc104ab3226e915a5c357f7388785d2`
— sealed and mutation-guarded: `cargo run -p pen-search --example
xf0b_concordance_v1 -- replay docs` recomputes the digest, checks every row
against the certified per-stage family counts, enforces the naming standard
per row, and grounds every inverse-join reference in the sealed blind
column. Flipping any verdict, translation clause, citation, or join row
invalidates replay.

**Scope executed:** all 32 credited families (27 rich-stratum + 5 sparse
singletons; F-C5 satisfied at the extended scope). **Method:** C-1 dossiers
extracted by digest from the sealed semantic ledger v6; C-2 naming by one
isolated analyst per family under the translation standard; C-3 one
adversarial verifier per proposal, adjudication on dispute (the XF-0
verification pattern); the XF-0 blind column quarantined from every naming
context and consumed only by the C-4 join.

## Headline

**Outcome U — by exactly one family.** 31 of 32 certified families carry
lawful classical names under clause-preserving translations with citations.
The single NO-NAME survivor is the stage-14 functional head
**df4feb52882e**: `Pi(Lam(Var 1), Sigma(Var 1, Var 2))` — a dependent
product whose domain position is a term-level abstraction — which failed
four recorded naming attempts, undisputed by both audits. Per the
registered Outcome-U semantics it is registered as a **candidate novel
mathematical object** in the DCT pattern (specification first, existence
obligations named): the study's potential export.

One **Outcome-X row** stands in the inverse join: the blind guarded-fixpoint
datum at stage 15 maps to **no certified family** — no sealed stage-15
clause is a fixpoint/Löb operator. This is consistent with the record's own
DCT trichotomy (strict Löb refuted; the repair lives in the semantic
completion, not the sealed shell) and is referred back to the internal
program as a named question: should the temporal shell carry a
guarded-fixpoint clause?

## The concordance (primary names; full translations in the JSON)

| Stage | Family | Verdict | Primary lawful name |
|---|---|---|---|
| 1 | 25a30e71 | named† | Tarski decoding family El : U₀ → Type; synonym: its display-map presentation |
| 3 | 7eb09c9e | named | Function application at a generic argument / evaluation counit of the exponential adjunction |
| 4 | 19ce26ca | named | Constant-family Π-formation / exponential object |
| 4 | a5b8df00 | named | Curried two-argument application spine / iterated evaluation |
| 4 | d0d9aee0 | named | β-conversion datum of the dependent-product adjunction |
| 5 | c79eff70 | named | Circle formation rule (carrier datum of the S¹-algebra signature) |
| 5 | 740fd77f | named | Formation datum of the circle HIT / classifying-map name |
| 5 | 30fd073e | named | Representing object of the based loop-space functor |
| 5 | e5ce4766 | named | base — the point constructor / unique 0-cell of the minimal CW structure |
| 5 | 9cefa52a | named | loop — the 1-path constructor / generating 1-cell |
| 5 | 0dca433d | named | Attaching-map datum of the 1-cell (CW boundary / face judgment) |
| 6 | 3dc2a654 | named | Propositional-truncation former / bracket (squash) type former |
| 6 | 26620f29 | named | Truncation unit constructor \|−\| / bracket introduction |
| 6 | f7dfbb | named | Truncation former as parametric formation action |
| 7 | 8b6e3ea5 | named | surf — the 2-path constructor / top-cell attaching datum of S² |
| 8 | b5a82d4e | named | Top-cell 3-path constructor loop₃ / unit of the suspension–loop adjunction |
| 10 | 82c78f61 | named | Flat modality ♭ (discrete-coreflection comonad Δ∘Γ) |
| 10 | 28f4d67a | named | Sharp modality ♯ (codiscrete monad ∇Γ) |
| 10 | bf5408a5 | named | Discrete-objects inclusion Disc (Lawvere constant-object / inverse-image functor Δ) |
| 10 | 3525ca6f | named | Shape modality ʃ (shape monad of a cohesive ∞-topos) |
| 11 | 10d43028 | named | Exponential functor at a fixed exponent / corepresentable hom-functor |
| 13 | a33b3dd2 | named | Unary algebra of similarity type (1,1) / free-monoid action |
| 13 | 992e25c4 | named | Magma structure (binary law of composition) |
| 14 | 67134aea | named | Rooted proof-relevant directed graph (pointed quiver) |
| 14 | 4efc2e4d | named | Coalgebra for the squaring endofunctor |
| 14 | df4feb52 | **NO-NAME** | — (4 recorded attempts; candidate novel object) |
| 15 | 589d3fa6 | named | Later modality ▷ (Nakano) |
| 15 | eedcb269 | named† | Basic modal similarity type (unary-operator signature) — signature-level, weaker than classical ◇ |
| 15 | 4ef6624f | named | LTL validity ○φ ⊃ ◇φ / step transformation ▷ ⇒ ◇ |
| 15 | e8f5d179 | named | Flat–later interchange law (mode-theory 2-cell ♭∘▷ ⇒ ▷∘♭) |
| 15 | 154b9549 | named | Sharp–eventually commutation axiom (adjudicated: axiom-scheme reading; the distributive-law reading was struck for unfunded naturality) |
| 15 | 8dbc4ba5 | named | Modal density axiom C4 (○○ → ○) |

† = verdict flipped from NO-NAME between the interrupted and the complete
run — see the stability disclosure below.

## C-4: what the blind census actually measured

**Forward (the undercount explanation, per rich stratum).** The register
mints per *aspect* — formation datum, classifying datum, representing
datum, constructor, attaching datum, exchange law — and mathematics has
names at exactly that granularity (CW/cell data, adjunction units and
counits, mode-theory 2-cells, modal axioms). The blind unit's zero-classes
(P/C/D) absorbed precisely those named aspects: the circle's five
non-constructor families absorbed as one C-classed formation rule plus D/P
data; Disc absorbed as adjoint-determined; stage 15's four law-families
absorbed as P-classed coherence. Full per-stratum tables in the JSON.

**Inverse (19 blind class-N data → families):** 11 exact hits, 2 landing on
*uncredited internal* families — the two bootstrap overcounts are now fully
explained: blind counted structures the register knows and *refuses credit*
(stage-1 carrier: internal formation completion; stage-2 unit former:
internal-identical to the stage-1 decoding family) — 5 partial (Σ-former
[branch-indexed, U_T2 open], H-space μ, connection ∇, metric g, inner
product ⟨·,·⟩: in each case the certified family is the structural head
without the classical scalar/geometric clothing), and 1 orphan (guarded
fixpoint — the Outcome-X row).

## Consequences and recommendation (registered)

1. **The v1 census unit was coarser than nomenclature, not deeper than
   it.** With 31/32 named, the Outcome-N payoff essentially holds: a
   principled v2 census unit exists — *named classical structures at family
   granularity* — conditional on resolving df4feb52882e, and any v2 census
   must disclose this study (and the spent v1) among the reasons its
   blindness is weaker (F-C4).
2. **The export candidate:** df4feb52882e is registered as a potential
   novel mathematical object. Given the stability disclosure below, the
   recommended next step is one targeted, versioned re-examination of this
   single family before the export is treated as firm.
3. **The named question:** the stage-15 fix orphan goes back to the
   internal program (does the temporal shell lawfully demand a
   guarded-fixpoint clause, or does its absence follow from the DCT
   trichotomy?).

## Disclosures

- **Information flow (F-C4):** this study read the certified register
  directly and is not blind; its outputs contaminate all future blind work.
- **Adjudication stability:** the naming fleet's first run lost one
  adjudicator to a session limit; the complete resumed run (68 agents, 0
  errors, every dispute adjudicated) is authoritative. Two verdicts flipped
  between runs (s01 25a30e71: NO-NAME → named; s15 eedcb269: NO-NAME →
  named), both at the NO-NAME boundary. The interpretive naming layer has
  judgment variance there; the surviving NO-NAME is therefore one-family
  thin. Both runs' existence and the flip direction are recorded in the
  sealed artifact — nothing is suppressed (F-C2).
- **Granularity honesty (F-C3):** no family was split, merged, or
  reinterpreted; where the classical name is weaker than the stage's
  physical reading (stage 15's Eventually named at signature level; stage
  13/14 heads named without scalar structure), the weakness is recorded as
  the result.
