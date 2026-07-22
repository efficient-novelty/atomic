{-# OPTIONS --safe --without-K #-}

module Schema2GrammarCompletion where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat; zero; suc)

_+_ : Nat -> Nat -> Nat
zero  + n = n
suc m + n = suc (m + n)

infixl 6 _+_

------------------------------------------------------------------------
-- Exact adopted Steps 9--15 signature inventory
------------------------------------------------------------------------

data TraceClass : Set where
  map-class  : TraceClass
  modal      : TraceClass
  axiomatic  : TraceClass
  synthesis  : TraceClass

data TraceStep : Set where
  step9 step10 step11 step12 step13 step14 step15 : TraceStep

class-of-step : TraceStep -> TraceClass
class-of-step step9  = map-class
class-of-step step10 = modal
class-of-step step11 = axiomatic
class-of-step step12 = axiomatic
class-of-step step13 = axiomatic
class-of-step step14 = axiomatic
class-of-step step15 = synthesis

clause-count : TraceStep -> Nat
clause-count step9  = 4
clause-count step10 = 4
clause-count step11 = 5
clause-count step12 = 6
clause-count step13 = 7
clause-count step14 = 9
clause-count step15 = 8

total-clause-count : Nat
total-clause-count =
  clause-count step9 + clause-count step10 + clause-count step11 +
  clause-count step12 + clause-count step13 + clause-count step14 +
  clause-count step15

forty-three-trace-signatures : total-clause-count ≡ 43
forty-three-trace-signatures = refl

data Fin : Nat -> Set where
  fzero : {n : Nat} -> Fin (suc n)
  fsuc  : {n : Nat} -> Fin n -> Fin (suc n)

record TraceDeclaration : Set where
  constructor trace-declaration
  field
    owner-step : TraceStep
    position   : Fin (clause-count owner-step)

------------------------------------------------------------------------
-- Typed normalization and a genuinely non-renaming carrier action
------------------------------------------------------------------------

data Carrier : Set where
  atom  : Nat -> Carrier
  trunc : Carrier -> Carrier

normalize-carrier : Carrier -> Carrier
normalize-carrier (atom n)  = atom n
normalize-carrier (trunc A) = trunc (normalize-carrier A)

data CarrierAction : Set where
  trunc-action : CarrierAction

act : CarrierAction -> Carrier -> Carrier
act trunc-action A = trunc A

normalize-action-naturality :
  (A : Carrier) ->
  normalize-carrier (act trunc-action A) ≡
  act trunc-action (normalize-carrier A)
normalize-action-naturality (atom n)  = refl
normalize-action-naturality (trunc A) =
  cong-trunc (normalize-action-naturality A)
  where
  cong-trunc : {X Y : Carrier} -> X ≡ Y -> trunc X ≡ trunc Y
  cong-trunc refl = refl

record NormalizedTraceDeclaration : Set where
  constructor normalized-trace-declaration
  field
    declaration : TraceDeclaration
    carrier     : Carrier

normalize-declaration : TraceDeclaration -> Carrier -> NormalizedTraceDeclaration
normalize-declaration d A = normalized-trace-declaration d (normalize-carrier A)

trace-declaration-naturality :
  (d : TraceDeclaration) (A : Carrier) ->
  normalize-declaration d (act trunc-action A) ≡
  normalized-trace-declaration d
    (act trunc-action (normalize-carrier A))
trace-declaration-naturality d (atom n)  = refl
trace-declaration-naturality d (trunc A) =
  cong-normalized (normalize-action-naturality A)
  where
  cong-normalized : {X Y : Carrier} ->
    X ≡ Y ->
    normalized-trace-declaration d (trunc X) ≡
    normalized-trace-declaration d (trunc Y)
  cong-normalized refl = refl

------------------------------------------------------------------------
-- Public twelve-form cubical induction
------------------------------------------------------------------------

data CubicalForm : Set where
  point motive-base endpoint-evaluation path-method endpoint-method
    path-elim endpoint-path-elim endpoint-elim-neutral dim-lambda dim-app
    coe hcom : CubicalForm

data PublicForm : CubicalForm -> Set where
  public-point               : PublicForm point
  public-motive-base         : PublicForm motive-base
  public-endpoint-evaluation : PublicForm endpoint-evaluation
  public-path-method         : PublicForm path-method
  public-endpoint-method     : PublicForm endpoint-method
  public-path-elim           : PublicForm path-elim
  public-endpoint-path-elim  : PublicForm endpoint-path-elim
  public-endpoint-neutral    : PublicForm endpoint-elim-neutral
  public-dim-lambda          : PublicForm dim-lambda
  public-dim-app             : PublicForm dim-app
  public-coe                 : PublicForm coe
  public-hcom                : PublicForm hcom

public-cubical-induction : (form : CubicalForm) -> PublicForm form
public-cubical-induction point                = public-point
public-cubical-induction motive-base          = public-motive-base
public-cubical-induction endpoint-evaluation  = public-endpoint-evaluation
public-cubical-induction path-method          = public-path-method
public-cubical-induction endpoint-method      = public-endpoint-method
public-cubical-induction path-elim             = public-path-elim
public-cubical-induction endpoint-path-elim    = public-endpoint-path-elim
public-cubical-induction endpoint-elim-neutral = public-endpoint-neutral
public-cubical-induction dim-lambda            = public-dim-lambda
public-cubical-induction dim-app               = public-dim-app
public-cubical-induction coe                   = public-coe
public-cubical-induction hcom                  = public-hcom

------------------------------------------------------------------------
-- G-6 and the G-8 total classifier boundary
------------------------------------------------------------------------

data UnitGeneratorDecision : Set where
  no-trace-grounded-generator : UnitGeneratorDecision

g6-decision : UnitGeneratorDecision
g6-decision = no-trace-grounded-generator

data RawClass : Set where
  raw-foundation raw-former raw-hit raw-suspension raw-map raw-modal
    raw-axiomatic raw-synthesis raw-unknown : RawClass

data Classification : RawClass -> Set where
  classified-foundation : Classification raw-foundation
  classified-former      : Classification raw-former
  classified-hit         : Classification raw-hit
  classified-suspension  : Classification raw-suspension
  classified-map         : Classification raw-map
  classified-modal       : Classification raw-modal
  classified-axiomatic   : Classification raw-axiomatic
  classified-synthesis   : Classification raw-synthesis
  named-f-g4-obstruction : Classification raw-unknown

classify-total : (raw : RawClass) -> Classification raw
classify-total raw-foundation = classified-foundation
classify-total raw-former     = classified-former
classify-total raw-hit        = classified-hit
classify-total raw-suspension = classified-suspension
classify-total raw-map        = classified-map
classify-total raw-modal      = classified-modal
classify-total raw-axiomatic  = classified-axiomatic
classify-total raw-synthesis  = classified-synthesis
classify-total raw-unknown    = named-f-g4-obstruction
