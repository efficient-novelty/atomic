{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.ProductionSyntaxV1 where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.LambdaUnit.TypingSyntax public
open import LawV2.LambdaUnit.Substitution using (_≗_)

-- Concrete finite indices. `fzero` is the newest local binder. For globals it
-- is declaration-order slot zero; `fsuc` advances one slot.
data Fin : Nat -> Set where
  fzero : {count : Nat} -> Fin (suc count)
  fsuc : {count : Nat} -> Fin count -> Fin (suc count)

fin-ordinal : {count : Nat} -> Fin count -> Nat
fin-ordinal fzero = zero
fin-ordinal (fsuc x) = suc (fin-ordinal x)

fsuc-ordinal :
  {count : Nat} ->
  (x : Fin count) ->
  fin-ordinal (fsuc x) ≡ suc (fin-ordinal x)
fsuc-ordinal x = refl

encode-fin-ordinal :
  (count ordinal : Nat) ->
  Maybe (Fin count)
encode-fin-ordinal zero ordinal = nothing
encode-fin-ordinal (suc count) zero = just fzero
encode-fin-ordinal (suc count) (suc ordinal)
  with encode-fin-ordinal count ordinal
... | nothing = nothing
... | just x = just (fsuc x)

encode-fin-ordinal-round-trip :
  {count : Nat} ->
  (x : Fin count) ->
  encode-fin-ordinal count (fin-ordinal x) ≡ just x
encode-fin-ordinal-round-trip {suc count} fzero = refl
encode-fin-ordinal-round-trip {suc count} (fsuc x)
  rewrite encode-fin-ordinal-round-trip x = refl

empty-eliminate :
  {A : Set} ->
  Empty ->
  A
empty-eliminate ()

nothing-not-just :
  {A : Set} {x : A} ->
  nothing ≡ just x ->
  Empty
nothing-not-just ()

just-injective-maybe :
  {A : Set} {left right : A} ->
  just left ≡ just right ->
  left ≡ right
just-injective-maybe refl = refl

fsuc-injective :
  {count : Nat} {left right : Fin count} ->
  fsuc left ≡ fsuc right ->
  left ≡ right
fsuc-injective refl = refl

fsuc-not-fzero :
  {count : Nat} {x : Fin count} ->
  fsuc x ≡ fzero ->
  Empty
fsuc-not-fzero ()

fin-ordinal-encode :
  (count ordinal : Nat) ->
  {x : Fin count} ->
  encode-fin-ordinal count ordinal ≡ just x ->
  fin-ordinal x ≡ ordinal
fin-ordinal-encode zero ordinal {()} proof
fin-ordinal-encode (suc count) zero {fzero} refl = refl
fin-ordinal-encode (suc count) zero {fsuc x} ()
fin-ordinal-encode
  (suc count)
  (suc ordinal)
  {fzero}
  proof
  with encode-fin-ordinal count ordinal
... | nothing =
  empty-eliminate (nothing-not-just proof)
... | just encoded =
  empty-eliminate
    (fsuc-not-fzero
      (just-injective-maybe proof))
fin-ordinal-encode
  (suc count)
  (suc ordinal)
  {fsuc x}
  proof
  with encode-fin-ordinal count ordinal in equation
... | nothing =
  empty-eliminate (nothing-not-just proof)
... | just encoded =
  cong suc
    (fin-ordinal-encode
      count
      ordinal
      (trans
        equation
        (cong just
          (fsuc-injective
            (just-injective-maybe proof)))))

-- The abstract calculus represents binder extension by `Lift`. Iterating it
-- from the empty set gives the exact abstract variable set at each concrete
-- context length.
PVar : Nat -> Set
PVar zero = Empty
PVar (suc count) = Lift (PVar count)

decode-fin : {count : Nat} -> Fin count -> PVar count
decode-fin {zero} ()
decode-fin {suc count} fzero = bound
decode-fin {suc count} (fsuc x) = free (decode-fin x)

encode-var : {count : Nat} -> PVar count -> Fin count
encode-var {zero} ()
encode-var {suc count} bound = fzero
encode-var {suc count} (free x) = fsuc (encode-var x)

decode-fin-encode-var :
  {count : Nat} ->
  (x : PVar count) ->
  decode-fin (encode-var x) ≡ x
decode-fin-encode-var {zero} ()
decode-fin-encode-var {suc count} bound = refl
decode-fin-encode-var {suc count} (free x) =
  cong free (decode-fin-encode-var x)

encode-var-decode-fin :
  {count : Nat} ->
  (x : Fin count) ->
  encode-var (decode-fin x) ≡ x
encode-var-decode-fin {zero} ()
encode-var-decode-fin {suc count} fzero = refl
encode-var-decode-fin {suc count} (fsuc x) =
  cong fsuc (encode-var-decode-fin x)

map-maybe :
  {A B : Set} ->
  (A -> B) ->
  Maybe A ->
  Maybe B
map-maybe function nothing = nothing
map-maybe function (just value) = just (function value)

map-maybe₂ :
  {A B C : Set} ->
  (A -> B -> C) ->
  Maybe A ->
  Maybe B ->
  Maybe C
map-maybe₂ function (just left) (just right) =
  just (function left right)
map-maybe₂ function nothing right = nothing
map-maybe₂ function (just left) nothing = nothing

-- Intrinsically locally scoped production terms with a genuinely finite
-- declaration-order global slot. The Rust GlobalId stored at each slot is not
-- represented here; binding this ordinal to the verified Rust slot table is a
-- separate cross-language theorem.
data PTm (global-count count : Nat) : Set where
  pvar : Fin count -> PTm global-count count
  psort : Nat -> PTm global-count count
  pglobal : Fin global-count -> PTm global-count count
  ppi :
    PTm global-count count ->
    PTm global-count (suc count) ->
    PTm global-count count
  plam :
    PTm global-count count ->
    PTm global-count (suc count) ->
    PTm global-count count
  papp :
    PTm global-count count ->
    PTm global-count count ->
    PTm global-count count
  punit-type : PTm global-count count
  punit : PTm global-count count

decode-ptm :
  {global-count count : Nat} ->
  PTm global-count count ->
  Tm (PVar count)
decode-ptm (pvar x) = var (decode-fin x)
decode-ptm (psort level) = sort level
decode-ptm (pglobal slot) = global (fin-ordinal slot)
decode-ptm (ppi parameter body) =
  pi (decode-ptm parameter) (decode-ptm body)
decode-ptm (plam parameter body) =
  lam (decode-ptm parameter) (decode-ptm body)
decode-ptm (papp function argument) =
  app (decode-ptm function) (decode-ptm argument)
decode-ptm punit-type = unit-type
decode-ptm punit = unit

-- Reification is partial only at globals: an abstract Nat global outside the
-- finite declaration-order table is not in the production image.
encode-ptm :
  {global-count count : Nat} ->
  Tm (PVar count) ->
  Maybe (PTm global-count count)
encode-ptm (var x) = just (pvar (encode-var x))
encode-ptm (sort level) = just (psort level)
encode-ptm {global-count} (global slot) =
  map-maybe pglobal (encode-fin-ordinal global-count slot)
encode-ptm (pi parameter body) =
  map-maybe₂ ppi
    (encode-ptm parameter)
    (encode-ptm body)
encode-ptm (lam parameter body) =
  map-maybe₂ plam
    (encode-ptm parameter)
    (encode-ptm body)
encode-ptm (app function argument) =
  map-maybe₂ papp
    (encode-ptm function)
    (encode-ptm argument)
encode-ptm unit-type = just punit-type
encode-ptm unit = just punit

encode-decode-ptm :
  {global-count count : Nat} ->
  (term : PTm global-count count) ->
  encode-ptm (decode-ptm term) ≡ just term
encode-decode-ptm (pvar x)
  rewrite encode-var-decode-fin x = refl
encode-decode-ptm (psort level) = refl
encode-decode-ptm (pglobal slot)
  rewrite encode-fin-ordinal-round-trip slot = refl
encode-decode-ptm (ppi parameter body)
  rewrite encode-decode-ptm parameter
        | encode-decode-ptm body = refl
encode-decode-ptm (plam parameter body)
  rewrite encode-decode-ptm parameter
        | encode-decode-ptm body = refl
encode-decode-ptm (papp function argument)
  rewrite encode-decode-ptm function
        | encode-decode-ptm argument = refl
encode-decode-ptm punit-type = refl
encode-decode-ptm punit = refl

-- Oldest-first snoc contexts. The appended type is scoped in exactly the
-- preceding declarations, as in `DependentContext(Vec<Term>)`.
data PCtx (global-count : Nat) : Nat -> Set where
  pempty : PCtx global-count zero
  _psnoc_ :
    {count : Nat} ->
    PCtx global-count count ->
    PTm global-count count ->
    PCtx global-count (suc count)

infixl 5 _psnoc_

decode-pctx :
  {global-count count : Nat} ->
  PCtx global-count count ->
  Context (PVar count)
decode-pctx pempty ()
decode-pctx (context psnoc parameter) bound =
  weaken (decode-ptm parameter)
decode-pctx (context psnoc parameter) (free x) =
  weaken (decode-pctx context x)

decode-pctx-snoc :
  {global-count count : Nat} ->
  (context : PCtx global-count count) ->
  (parameter : PTm global-count count) ->
  decode-pctx (context psnoc parameter)
  ≗
  extend-context (decode-pctx context) (decode-ptm parameter)
decode-pctx-snoc context parameter bound = refl
decode-pctx-snoc context parameter (free x) = refl
