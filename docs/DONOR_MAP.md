# Donor Map

The frozen v1 donor mapping is:

- `Kolmogorov.hs` -> `pen-core/{atom,encode}.rs`
- `Telescope.hs`, `MBTTCanonical.hs`, parts of `Capability.hs` ->
  `pen-core/{telescope,library,canonical,capability}.rs`
- `TelescopeCheck.hs` ->
  `pen-type/{infer,check,admissibility,connectivity}.rs`
- `MBTTNu.hs`, `StructuralNu.hs`, `CoherenceWindow.hs`, `StrictMinimality.hs`,
  `AbInitioPolicy.hs` -> `pen-eval/{nu,nu_trace,coherence,minimality,bar}.rs`
- `MBTTEnum.hs`, parts of `Parallel.hs`, `RunAbInitio.hs` -> `pen-search/*`
- `AgdaExport.hs` -> `pen-agda/*`

Do not port molecular search or shell inventory into the new hot path.
