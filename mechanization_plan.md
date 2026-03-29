# Mechanization Plan

This plan tracks the Agda-side formalization slices for the claims in
`1_coherence_depth.tex` (repository copy:
`tex/coherence_depth_fibonacci_lmcs_draft.tex`).

## Current baseline

1. Abstraction-barrier baseline
   Generated Agda exports now target explicit bridge-contract and opacity modules:
   `agda/BridgePayload.agda`, `agda/AbstractionBarrier.agda`, and
   `agda/StepWitness.agda`. `PayloadNN.agda` exposes a `BridgePayload` plus
   `ContractWitness`, `StepNN.agda` exposes a `StepWitness`, and
   `pen-cli export-agda --verify` emits a self-contained bundle that typechecks
   with Agda.

## Backlog

1. Formalize the depth-two recurrence and cumulative-trace theorem in Agda,
   aligned with `thm:recurrence` and `cor:fibonacci`.
2. Add the extensional collapse interface for Theorem A (`d = 1` under UIP),
   isolating the assumptions needed by the paper statement.
3. Mechanize the adjunction barrier as a depth-two lower-bound witness with
   explicit unit, counit, and triangle data.
4. Add the filtered obligation-complex and `E_2`-degeneration scaffold used by
   the homological upper-bound argument.
5. Add a clutching-family/topological witness module capturing the exact
   depth-two lower-bound family.
