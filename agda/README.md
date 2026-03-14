`agda/` holds the sidecar verification surface only.

The generated modules are deliberately conservative: each accepted step is exported as
an Agda stub module with structural MBTT comments, candidate fingerprints, and a
machine-readable manifest. The hot search loop never depends on Agda.

Typical entry points:

- `cargo run -p pen-cli -- export-agda --until-step 15`
- `cargo run -p xtask -- export-reference-agda 15`

If the `agda` executable is available, `pen-cli export-agda --verify` will also write
per-step verification logs next to the generated modules. If it is not available, the
export manifest records verification as skipped instead of pretending it passed.
