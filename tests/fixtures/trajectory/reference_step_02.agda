module Step02 where

open import Agda.Primitive using (Set)

open import StepWitness using (StepWitness)
open import Payload02 as Payload02

-- step: 2
-- label: Unit
-- candidate_hash: blake3:a2dfff0fb8ce1073119da3893e1d195247c234af66b9a7de17c85d5cf718f555
-- canonical_hash: blake3:cd72c5935198d40c0ef48ae172c9ccba1ef9c90435494569121c8dccd3466476
-- proof_payload: Payload02

postulate
  T : Set
  stepWitness : StepWitness Payload02.payload T
  clause01 : Set
    -- translated: (Set x1)
    -- mbtt: {"App":["Univ",{"Var":1}]}
