# DNF-2 unified typed judgments result

**Status:** `StoppedNamedGaps`. **Certificate:** `blake3:b743483ddd36f2b489f353e37cbaed0a4a0aa4379189edd603ea2470607d7f21`.

Exact payload-preserving translations: 106/106 (core 62, ordinary 24, cubical 19, R1 1). Locally context-complete typed judgments: 60. All four surfaces are represented: `true`; every source payload is retained byte-structurally: `true`.

Every surface has a full typed judgment: `false`. Equality composition on the full translated domain: `false`.

## Named gaps

- `DNF2_EXACT_DEPENDENT_CONTEXT_ATTACHMENT_GAP`: Exact source payloads are retained for all four surfaces, but ordinary, cubical, and opaque-parameter core members do not export an exact canonical dependent telescope. Wrapping their payload is not a typed-judgment translation.
- `DNF2_ALL_SURFACE_FROZEN_EQUALITY_COMPOSITION_GAP`: The v5 finite equality closure replays on its historical members, but without exact target contexts there is no theorem that every surface translation composes with frozen equality as a typed judgment.

No archive identifier or digest was used as missing presentation content. T-D2-2 reopened: `false`. M-4 authorized: `false`.
