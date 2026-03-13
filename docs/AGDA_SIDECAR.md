# Agda Sidecar

Agda is a verification sidecar only.

The Rust hot path may export accepted steps into generated Agda modules, but Agda
must never influence search ranking, sound prune decisions, or acceptance.

The initial bridge surface will live under `agda/Generated/` and be driven by the
`pen-agda` crate.
