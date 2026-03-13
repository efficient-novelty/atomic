# Memory Model

Canonical target host: a 16 GiB desktop.

## Budget summary

- reserve for OS and background load: 4.0 GiB
- hard engine cap: 12.0 GiB RSS
- emergency checkpoint threshold: 11.5 GiB
- pressure threshold: 10.5 GiB
- soft threshold: 9.0 GiB
- target steady state: 7.5 to 8.5 GiB

## Resident component targets

- 2.25 GiB hot frontier
- 2.00 GiB interned AST and clause arenas
- 1.25 GiB dedupe keys and quotient filters
- 0.75 GiB evaluator caches
- 0.75 GiB worker scratch arenas
- 0.50 GiB spill and shard I/O buffers
- 0.25 GiB checkpoint staging and telemetry
- 0.75 GiB emergency slack

## Governor states

- Green: below 7.5 GiB
- Yellow: 7.5 to 9.0 GiB
- Orange: 9.0 to 10.5 GiB
- Red: 10.5 to 11.5 GiB
- Black: above 11.5 GiB
