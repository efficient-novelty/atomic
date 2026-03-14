module Prelude where

open import Agda.Primitive using (Set)

postulate
  Sigma : Set -> (Set -> Set) -> Set
  Id : Set -> Set -> Set -> Set
  Refl : Set -> Set
  Susp : Set -> Set
  Trunc0 : Set -> Set
  Flat : Set -> Set
  Sharp : Set -> Set
  Disc : Set -> Set
  Shape : Set -> Set
  Next : Set -> Set
  Eventually : Set -> Set
  PathCon : Set
