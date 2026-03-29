`agda/` holds the sidecar verification surface only.

The generated modules are deliberately conservative: each accepted step is exported as
an Agda witness module with structural MBTT comments, candidate fingerprints, and a
machine-readable manifest. The hot search loop never depends on Agda.

The hand-written baseline support modules are:

- `BridgePayload.agda`
- `AbstractionBarrier.agda`
- `StepWitness.agda`

`pen-cli export-agda` copies those support modules into each output directory so the
generated `PayloadNN.agda` and `StepNN.agda` files form a self-contained verification
bundle.

Typical entry points:

- `cargo run -p pen-cli -- export-agda --until-step 15`
- `cargo run -p xtask -- export-reference-agda 15`

If the `agda` executable is available, `pen-cli export-agda --verify` will also write
per-step verification logs next to the generated modules. If it is not available, the
export manifest records verification as skipped instead of pretending it passed.
