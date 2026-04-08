# Autonomous Claim Lane Checklist

Last updated: 2026-04-08

This file owns only binary gates for `desktop_claim_shadow`.

## Contract

- Keep this file as yes/no signoff criteria only.
- Move explanations to [autonomous_progress.md](autonomous_progress.md).
- Move the current work order to [autonomous_next_steps.md](autonomous_next_steps.md).
- Move probe history to [autonomous_ledger.md](autonomous_ledger.md).

## Local Repair Gate

- [ ] Clean step-`15` partial-prefix wall is below `553` on top of canonical
      `v12`.
- [ ] Accepted step-`15` winner remains canonical `103 / 8`.
- [ ] Isolated `single` pocket remains fenced.
- [ ] Unsafe lifted `89 / 8` terminals remain fenced.
- [ ] `small_cluster` is no worse than `3132 / 522 / 522 / 0`.

## Stored Evidence Gate

- [ ] Stored step `15` generated count is at least `5000` with
      accepted-hash parity preserved.
- [ ] Stored step `1` generated count is restored to exactly `2144`.
- [ ] Compare, benchmark, and certification are refreshed for the new
      canonical bundle.
- [ ] `full_telescopes_evaluated` remains within a certified moderate
      threshold.

## Audit Gate

- [ ] One passing `claim_certificate.json` is stored beside the canonical
      bundle.

## Language Gate

- [ ] User-facing and paper-facing wording remains `bounded live recovery`
      until certification passes.
- [ ] `unguided` is not used user-facing before a passing certificate exists.
- [ ] Any stronger wording is tied directly to the stored certificate and
      disclosed desktop bundle.
