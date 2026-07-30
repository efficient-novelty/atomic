{-# OPTIONS --safe --without-K #-}

-- Cross-language decode/check vectors for the eleven-section bundle.
--
-- `canonical-vector-v1` is the exact `encode_bundle_v1` output of the
-- cross-language fixture bundle: the canonical `pen-production-wire`
-- fixture extended with a discriminating context (variable and
-- under-binder Pi entries), identity and beta conversion certificates,
-- and an application-elimination synthesis certificate, so that the
-- ordinal, entry-selection, shift, reduction, and instantiation
-- conventions are all byte-visible. Rust verdicts for every vector
-- below are pinned by the generating harness: the canonical vector
-- decodes and round-trips; every mutated vector is rejected by
-- `decode_bundle_v1` (Rust folds structural validation into decoding,
-- while this Agda surface separates parse rejection from check
-- rejection). These are development regression vectors, not
-- correspondence authority.

module LawV2.Wire.BundleDecodeTestV1 where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe using (Maybe; just; nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.Wire.Bytes
open import LawV2.Wire.ProductionBundleV1
open import LawV2.Wire.BundleChecker

canonical-vector-v1 : List Nat
canonical-vector-v1 =
  80 ∷ 69 ∷ 78 ∷ 45 ∷ 80 ∷ 82 ∷ 79 ∷ 68 ∷ 45 ∷ 87 ∷ 73 ∷ 82 ∷ 
  69 ∷ 45 ∷ 86 ∷ 49 ∷ 1 ∷ 0 ∷ 11 ∷ 0 ∷ 1 ∷ 247 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 3 ∷ 0 ∷ 33 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 103 ∷ 102 ∷ 50 ∷ 45 ∷ 115 ∷ 101 ∷ 109 ∷ 97 ∷ 110 ∷ 
  116 ∷ 105 ∷ 99 ∷ 45 ∷ 97 ∷ 117 ∷ 100 ∷ 105 ∷ 116 ∷ 45 ∷ 108 ∷ 97 ∷ 
  109 ∷ 98 ∷ 100 ∷ 97 ∷ 45 ∷ 117 ∷ 110 ∷ 105 ∷ 116 ∷ 45 ∷ 118 ∷ 51 ∷ 
  20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 
  20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 
  20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 20 ∷ 0 ∷ 0 ∷ 0 ∷ 21 ∷ 
  21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 
  21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 
  21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 21 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 
  22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 
  22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 22 ∷ 
  22 ∷ 22 ∷ 22 ∷ 35 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 112 ∷ 
  101 ∷ 110 ∷ 45 ∷ 107 ∷ 101 ∷ 114 ∷ 110 ∷ 101 ∷ 108 ∷ 45 ∷ 115 ∷ 121 ∷ 
  110 ∷ 116 ∷ 104 ∷ 101 ∷ 115 ∷ 105 ∷ 115 ∷ 47 ∷ 108 ∷ 97 ∷ 109 ∷ 98 ∷ 
  100 ∷ 97 ∷ 45 ∷ 117 ∷ 110 ∷ 105 ∷ 116 ∷ 47 ∷ 118 ∷ 50 ∷ 2 ∷ 0 ∷ 
  2 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 
  3 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 
  2 ∷ 0 ∷ 4 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  1 ∷ 0 ∷ 2 ∷ 0 ∷ 3 ∷ 0 ∷ 32 ∷ 0 ∷ 8 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 2 ∷ 3 ∷ 4 ∷ 5 ∷ 6 ∷ 7 ∷ 
  2 ∷ 140 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 23 ∷ 23 ∷ 23 ∷ 
  23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 
  23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 
  23 ∷ 23 ∷ 23 ∷ 23 ∷ 23 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 
  24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 
  24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 24 ∷ 
  24 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 
  25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 
  25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 25 ∷ 1 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 1 ∷ 1 ∷ 
  1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 
  1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 
  1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 3 ∷ 89 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 2 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 
  1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 
  1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 1 ∷ 6 ∷ 1 ∷ 
  7 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 
  2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 
  2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 2 ∷ 
  2 ∷ 2 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 4 ∷ 50 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 6 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 6 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 3 ∷ 1 ∷ 1 ∷ 0 ∷ 0 ∷ 
  0 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 5 ∷ 166 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 4 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 5 ∷ 
  5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 
  5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 
  5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 5 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 2 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 0 ∷ 6 ∷ 7 ∷ 
  2 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 1 ∷ 2 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  7 ∷ 7 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 1 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 7 ∷ 7 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 
  12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 
  12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 
  12 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 3 ∷ 6 ∷ 6 ∷ 
  3 ∷ 6 ∷ 6 ∷ 1 ∷ 3 ∷ 6 ∷ 6 ∷ 3 ∷ 6 ∷ 6 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 3 ∷ 6 ∷ 6 ∷ 3 ∷ 6 ∷ 6 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 3 ∷ 6 ∷ 6 ∷ 3 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 3 ∷ 6 ∷ 6 ∷ 3 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 6 ∷ 6 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 1 ∷ 6 ∷ 6 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 
  13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 
  13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 
  13 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 6 ∷ 6 ∷ 1 ∷ 
  6 ∷ 6 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 6 ∷ 6 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 6 ∷ 1 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 6 ∷ 6 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 
  14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 
  14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 14 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 5 ∷ 4 ∷ 6 ∷ 1 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 7 ∷ 7 ∷ 0 ∷ 6 ∷ 7 ∷ 5 ∷ 4 ∷ 6 ∷ 1 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 5 ∷ 4 ∷ 6 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 7 ∷ 
  7 ∷ 7 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 1 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 7 ∷ 7 ∷ 6 ∷ 8 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 200 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 2 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 
  6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 
  6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 6 ∷ 1 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 6 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 6 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 
  0 ∷ 0 ∷ 0 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 
  15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 
  15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 15 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 5 ∷ 4 ∷ 6 ∷ 1 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 7 ∷ 6 ∷ 7 ∷ 6 ∷ 1 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 2 ∷ 12 ∷ 12 ∷ 
  12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 
  12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 
  12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 12 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 
  13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 
  13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 13 ∷ 
  13 ∷ 13 ∷ 6 ∷ 8 ∷ 15 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  7 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 2 ∷ 3 ∷ 
  4 ∷ 5 ∷ 6 ∷ 9 ∷ 80 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  1 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 
  7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 
  7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 
  7 ∷ 7 ∷ 7 ∷ 7 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 
  1 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 6 ∷ 5 ∷ 2 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 2 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 6 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 10 ∷ 11 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 1 ∷ 2 ∷ 11 ∷ 90 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 4 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 8 ∷ 8 ∷ 
  8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 
  8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 
  8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 6 ∷ 0 ∷ 9 ∷ 9 ∷ 
  9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 
  9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 
  9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 1 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 
  7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 
  7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 
  7 ∷ 7 ∷ 7 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 
  6 ∷ 1 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 
  10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 
  10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 10 ∷ 8 ∷ 8 ∷ 
  8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 
  8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 
  8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 
  9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 
  9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 9 ∷ 
  9 ∷ 9 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 6 ∷ 
  2 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 
  11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 
  11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 11 ∷ 7 ∷ 7 ∷ 7 ∷ 
  7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 
  7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 
  7 ∷ 7 ∷ 7 ∷ 7 ∷ 7 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 
  8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 
  8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 8 ∷ 
  8 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 7 ∷ 6 ∷ 
  []

set-at : Nat → Nat → List Nat → List Nat
set-at index value [] = []
set-at zero value (x ∷ xs) = value ∷ xs
set-at (suc index) value (x ∷ xs) = x ∷ set-at index value xs

drop-last : List Nat → List Nat
drop-last [] = []
drop-last (x ∷ []) = []
drop-last (x ∷ xs) = x ∷ drop-last xs

decode-check-nats : List Nat → Maybe Bool
decode-check-nats values =
  maybe-bind (bytes-from-nats values) (λ bytes →
  maybe-bind (decode-production-bundle-v1 bytes) (λ bundle →
  just (check-production-bundle-structure-v1 bundle)))

-- The genuine Rust byte vector decodes and passes the structural check.
canonical-vector-accepted :
  decode-check-nats canonical-vector-v1 ≡ just true
canonical-vector-accepted = refl

-- Envelope-level mutation: wrong magic byte fails the parse outright.
magic-flip-rejected :
  decode-check-nats (set-at 0 81 canonical-vector-v1) ≡ nothing
magic-flip-rejected = refl

-- Length mutation: dropping the final byte breaks full consumption.
truncation-rejected :
  decode-check-nats (drop-last canonical-vector-v1) ≡ nothing
truncation-rejected = refl

-- Tag mutation: an unknown term tag inside the global-slot table fails
-- the parse.
unknown-term-tag-rejected :
  decode-check-nats (set-at 478 255 canonical-vector-v1) ≡ nothing
unknown-term-tag-rejected = refl

-- Authority mutation: a frozen manifest decodes but fails the check.
manifest-frozen-rejected :
  decode-check-nats (set-at 105 1 canonical-vector-v1) ≡ just false
manifest-frozen-rejected = refl

-- Universe mutation: public level 2 decodes but fails the check.
public-universe-rejected :
  decode-check-nats (set-at 226 2 canonical-vector-v1) ≡ just false
public-universe-rejected = refl

-- Scope mutation: a synthesis subject variable out of context range
-- decodes but fails the check.
synthesis-scope-rejected :
  decode-check-nats (set-at 1089 1 canonical-vector-v1) ≡ just false
synthesis-scope-rejected = refl

-- Inventory mutation: swapping the first two Q0 rules decodes but fails
-- the exact-inventory check.
q0-swap-rejected :
  decode-check-nats
    (set-at 1257 0 (set-at 1256 1 canonical-vector-v1)) ≡ just false
q0-swap-rejected = refl
