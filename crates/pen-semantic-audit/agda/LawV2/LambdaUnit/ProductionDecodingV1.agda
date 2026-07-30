{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.ProductionDecodingV1 where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.LambdaUnit.ProductionSyntaxV1

PRen : Nat -> Nat -> Set
PRen source target = Fin source -> Fin target

lift-pren :
  {source target : Nat} ->
  PRen source target ->
  PRen (suc source) (suc target)
lift-pren rho fzero = fzero
lift-pren rho (fsuc x) = fsuc (rho x)

prename :
  {global-count source target : Nat} ->
  PRen source target ->
  PTm global-count source ->
  PTm global-count target
prename rho (pvar x) = pvar (rho x)
prename rho (psort level) = psort level
prename rho (pglobal slot) = pglobal slot
prename rho (ppi parameter body) =
  ppi (prename rho parameter) (prename (lift-pren rho) body)
prename rho (plam parameter body) =
  plam (prename rho parameter) (prename (lift-pren rho) body)
prename rho (papp function argument) =
  papp (prename rho function) (prename rho argument)
prename rho punit-type = punit-type
prename rho punit = punit

pweaken :
  {global-count count : Nat} ->
  PTm global-count count ->
  PTm global-count (suc count)
pweaken = prename fsuc

decode-prenaming :
  {source target : Nat} ->
  PRen source target ->
  PVar source ->
  PVar target
decode-prenaming rho x =
  decode-fin (rho (encode-var x))

decode-lift-prenaming :
  {source target : Nat} ->
  (rho : PRen source target) ->
  decode-prenaming (lift-pren rho)
  ≗
  lift-map (decode-prenaming rho)
decode-lift-prenaming rho bound = refl
decode-lift-prenaming rho (free x) = refl

decode-prename :
  {global-count source target : Nat} ->
  (rho : PRen source target) ->
  (term : PTm global-count source) ->
  decode-ptm (prename rho term)
  ≡
  rename (decode-prenaming rho) (decode-ptm term)
decode-prename rho (pvar x) =
  cong var
    (cong
      (λ encoded -> decode-fin (rho encoded))
      (sym (encode-var-decode-fin x)))
decode-prename rho (psort level) = refl
decode-prename rho (pglobal slot) = refl
decode-prename rho (ppi parameter body) =
  cong₂ pi
    (decode-prename rho parameter)
    (trans
      (decode-prename (lift-pren rho) body)
      (rename-cong (decode-lift-prenaming rho) (decode-ptm body)))
decode-prename rho (plam parameter body) =
  cong₂ lam
    (decode-prename rho parameter)
    (trans
      (decode-prename (lift-pren rho) body)
      (rename-cong (decode-lift-prenaming rho) (decode-ptm body)))
decode-prename rho (papp function argument) =
  cong₂ app
    (decode-prename rho function)
    (decode-prename rho argument)
decode-prename rho punit-type = refl
decode-prename rho punit = refl

decode-fsuc :
  {count : Nat} ->
  decode-prenaming (fsuc {count = count})
  ≗
  free
decode-fsuc x =
  cong free (decode-fin-encode-var x)

decode-pweaken :
  {global-count count : Nat} ->
  (term : PTm global-count count) ->
  decode-ptm (pweaken term) ≡ weaken (decode-ptm term)
decode-pweaken term =
  trans
    (decode-prename fsuc term)
    (rename-cong decode-fsuc (decode-ptm term))

PSub : Nat -> Nat -> Nat -> Set
PSub global-count source target =
  Fin source -> PTm global-count target

lift-psub :
  {global-count source target : Nat} ->
  PSub global-count source target ->
  PSub global-count (suc source) (suc target)
lift-psub sigma fzero = pvar fzero
lift-psub sigma (fsuc x) = pweaken (sigma x)

psubstitute :
  {global-count source target : Nat} ->
  PSub global-count source target ->
  PTm global-count source ->
  PTm global-count target
psubstitute sigma (pvar x) = sigma x
psubstitute sigma (psort level) = psort level
psubstitute sigma (pglobal slot) = pglobal slot
psubstitute sigma (ppi parameter body) =
  ppi
    (psubstitute sigma parameter)
    (psubstitute (lift-psub sigma) body)
psubstitute sigma (plam parameter body) =
  plam
    (psubstitute sigma parameter)
    (psubstitute (lift-psub sigma) body)
psubstitute sigma (papp function argument) =
  papp
    (psubstitute sigma function)
    (psubstitute sigma argument)
psubstitute sigma punit-type = punit-type
psubstitute sigma punit = punit

decode-psubstitution :
  {global-count source target : Nat} ->
  PSub global-count source target ->
  PVar source ->
  Tm (PVar target)
decode-psubstitution sigma x =
  decode-ptm (sigma (encode-var x))

decode-lift-psubstitution :
  {global-count source target : Nat} ->
  (sigma : PSub global-count source target) ->
  decode-psubstitution (lift-psub sigma)
  ≗
  lift-substitution (decode-psubstitution sigma)
decode-lift-psubstitution sigma bound = refl
decode-lift-psubstitution sigma (free x) =
  decode-pweaken (sigma (encode-var x))

decode-psubstitute :
  {global-count source target : Nat} ->
  (sigma : PSub global-count source target) ->
  (term : PTm global-count source) ->
  decode-ptm (psubstitute sigma term)
  ≡
  substitute (decode-psubstitution sigma) (decode-ptm term)
decode-psubstitute sigma (pvar x) =
  cong
    (λ encoded -> decode-ptm (sigma encoded))
    (sym (encode-var-decode-fin x))
decode-psubstitute sigma (psort level) = refl
decode-psubstitute sigma (pglobal slot) = refl
decode-psubstitute sigma (ppi parameter body) =
  cong₂ pi
    (decode-psubstitute sigma parameter)
    (trans
      (decode-psubstitute (lift-psub sigma) body)
      (substitute-cong
        (decode-lift-psubstitution sigma)
        (decode-ptm body)))
decode-psubstitute sigma (plam parameter body) =
  cong₂ lam
    (decode-psubstitute sigma parameter)
    (trans
      (decode-psubstitute (lift-psub sigma) body)
      (substitute-cong
        (decode-lift-psubstitution sigma)
        (decode-ptm body)))
decode-psubstitute sigma (papp function argument) =
  cong₂ app
    (decode-psubstitute sigma function)
    (decode-psubstitute sigma argument)
decode-psubstitute sigma punit-type = refl
decode-psubstitute sigma punit = refl

lookup-pctx :
  {global-count count : Nat} ->
  PCtx global-count count ->
  Fin count ->
  PTm global-count count
lookup-pctx pempty ()
lookup-pctx (context psnoc parameter) fzero =
  pweaken parameter
lookup-pctx (context psnoc parameter) (fsuc x) =
  pweaken (lookup-pctx context x)

decode-lookup-pctx :
  {global-count count : Nat} ->
  (context : PCtx global-count count) ->
  (x : Fin count) ->
  decode-ptm (lookup-pctx context x)
  ≡
  decode-pctx context (decode-fin x)
decode-lookup-pctx pempty ()
decode-lookup-pctx (context psnoc parameter) fzero =
  decode-pweaken parameter
decode-lookup-pctx
  (context psnoc parameter)
  (fsuc x) =
  trans
    (decode-pweaken (lookup-pctx context x))
    (cong weaken (decode-lookup-pctx context x))

encode-decoded-context-entry :
  {global-count count : Nat} ->
  (context : PCtx global-count count) ->
  (x : Fin count) ->
  encode-ptm (decode-pctx context (decode-fin x))
  ≡
  just (lookup-pctx context x)
encode-decoded-context-entry context x =
  trans
    (cong encode-ptm (sym (decode-lookup-pctx context x)))
    (encode-decode-ptm (lookup-pctx context x))
