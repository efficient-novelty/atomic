# Frontier State V1

`FrontierStateRecV1` is fixed-width, little-endian, and 64 bytes long.

```text
offset  size  field
0       8     state_id
8       8     parent_state_id
16      4     last_clause_id
20      4     obligation_set_id
24      8     shape_hash64
32      8     support_hash64
40      2     nu_lower_bound
42      2     nu_upper_bound
44      2     bit_kappa_used
46      2     clause_kappa_used
48      2     depth
50      1     step_index
51      1     band_index
52      2     flags
54      4     priority_key
58      2     worker_hint
60      4     reserved
```

The routing hashes are truncated 64-bit values only. Full canonical fingerprints live
in dedupe segments and remain the source of truth.
