# Semantic-nu typed transport maps v2

**Date:** 2026-07-23. **Outcome:** `five_v1_bijections_exported_as_exact_typed_maps_f_uc4_transport_gate_ready`. **Certificate:** `blake3:b9d84d8927e4d1e369e6cf9904664e5f0a9ecc1cdf9e3e92644133cae14e804c`.

M1 exports 5 `[proof_inventory]` typed family-class maps, one for each v1 bijection. Every map carries its source and target proof witnesses, typed forward map, typed inverse, and both composition-to-identity computations. Reconstructing the v1 map rows from these typed rows reproduces every predecessor theorem exactly: **true**.

## Exported maps

| relation | source object | target object | source `nu` `[semantic_family_nu]` | target `nu` `[semantic_family_nu]` | exact typed inverse/coherence |
|---|---|---|---:|---:|---|
| `adopted-A3-naturality-pointwise-to-direct-family-identity` | `A3-direct-J3-family-carrier` | `A3-pointwise-specialization-family-carrier` | 8 | 8 | true |
| `stage4-r-t1-reflexive-family-identity` | `blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407` | `blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407` | 3 | 3 | true |
| `stage4-r-t1-reflexive-family-identity` | `blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308` | `blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308` | 2 | 2 | true |
| `stage4-r-t1-reflexive-family-identity` | `blake3:4b2211ecae25f3afbb1187f3a4a1b644edfc6adbd3512b9ca27c64a7b731265b` | `blake3:4b2211ecae25f3afbb1187f3a4a1b644edfc6adbd3512b9ca27c64a7b731265b` | 2 | 2 | true |
| `stage4-r-t1-reflexive-family-identity` | `blake3:b4f821d9bb28366d8ae37adc7c1cb3e28c8a0de60f05c81e9ad75ba4f8b0edd4` | `blake3:b4f821d9bb28366d8ae37adc7c1cb3e28c8a0de60f05c81e9ad75ba4f8b0edd4` | 3 | 3 | true |

## Order-axis finite obstructions (not UC-1 scoring)

| fixed coordinate | candidates | semantic-family `nu` | no accepted typed bijection |
|---|---|---:|---|
| `former=pi` | `20167267` / `43a0ed70` | 3 / 2 | true |
| `former=sigma` | `b4f821d9` / `4b2211ec` | 3 / 2 | true |

The F-UC4 transport gate is ready for the separate M2 scoring artifact: **true**. M1 itself issues no UC-1 verdict (`uc1_scored = false`). Distinct Stage-4 cross-package equivalences constructed: 0 `[proof_inventory]`.

Scope boundary `M1_PROOF_BEARING_TYPED_TRANSPORT_CLASS_ONLY`: The theorem covers exactly proof-bearing semantic-family equivalences that export a typed total family-class map, a typed inverse, and both identity/composition laws. It does not construct a Stage-4 parameter-swap act equivalence, does not construct a Pi/Sigma library-act equivalence, and does not assert functoriality for an unformalized broader equivalence class.

Permitted conclusion: Each of the five v1 bijections is now induced by an exported proof-bearing family-class map with a typed inverse and replayable identity/composition coherence. The v1 cardinality theorem replays from those exact induced maps. Within this adopted proof-bearing transport class semantic-family nu is invariant; the two Stage-4 order pairs have certified 3-versus-2 finite no-bijection obstructions ready for the separate UC-1 scoring artifact.

Forbidden conclusion: M1 does not score or burn any UC-1 clause, does not construct a distinct Stage-4 cross-package equivalence, does not prove Pi/Sigma act equivalence, and does not extend the adopted equivalence class beyond transports carrying the exported map/inverse/coherence data.
