# TDC-1 execution protocol

**Status:** implementation protocol only; no TDC-1 certificate or zone is
recorded here. The controlling pre-registration is
`docs/ip1_tdc1_plan.md`.

TDC-1 is split into two executions so that the Step-16 bar cannot influence
typed normalization, weakening, or provenance.

## Phase 1 — bar-independent certificate

```powershell
cargo run -p pen-search --example tdc1_typed_d4 -- `
  certificate docs/tdc1_typed_d4_certificate.json
```

The writer uses create-new semantics. Freeze the emitted certificate before
running Phase 2. Replay is available without performing the zone comparison:

```powershell
cargo run -p pen-search --example tdc1_typed_d4 -- `
  replay docs/tdc1_typed_d4_certificate.json
```

The certificate checks:

- the exact registered surface
  `[App(Univ, Lib(15)), PathCon(4)]` with `kappa = 2`;
- ordinary telescope elaboration, normalization, and equality against the
  sealed `B15` signature;
- two independent materializations of beta plus the ordered `4 by 4` Kan
  index set, including the exact `1 + d^2 = 17` partition;
- per-site typed-realization, weakening, and provenance dispositions, with a
  missing cubical primitive represented as `undefined`, never as zero;
- the complete Stage-16 demand-orbit inventory and the finite local-role
  anchor capacity;
- historical HIT regression for Steps 5–8 through the same machinery; and
- dimension, duplicate-site, missing-site, over-ceiling, and anchor-collision
  mutations.

The index enumeration is not itself a semantic certificate. Kernel v1 has no
interval/cofibration context, `coe`, `hcom`, or constructor computation term.
The implementation therefore refuses to turn an L1 index into a natural
family unless a replayable typed realization exists.

## Phase 2 — frozen comparison

Only after the Phase-1 artifact is frozen:

```powershell
cargo run -p pen-search --example tdc1_typed_d4 -- `
  compare docs/tdc1_typed_d4_certificate.json `
  docs/tdc1_typed_d4_comparison.json
```

The comparison first re-derives the certificate byte-for-byte, then computes
`rho = nu/2`, compares it with `Bar16 = 354333/39040`, and assigns the
pre-registered Z1–Z4 zone. A failed historical regression sets F-T1 and makes
any Step-16 numerical verdict void; history is never adjusted to fit the new
fragment.

## Source map

- `crates/pen-type/src/tdc1.rs`: typed formed-path surface, canonical L1
  index presentations, and explicit cubical-realization gaps.
- `crates/pen-eval/src/tdc1.rs`: weakening/provenance audit, historical
  regression, certificate replay, mutation falsifiers, and separate zone
  comparison.
- `crates/pen-search/examples/tdc1_typed_d4.rs`: create-new two-phase runner.
