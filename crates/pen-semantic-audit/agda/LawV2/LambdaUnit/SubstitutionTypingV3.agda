{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.SubstitutionTypingV3 where

open import Agda.Builtin.Equality using (_≡_; refl)
open import LawV2.LambdaUnit.TypingJudgment
open import LawV2.LambdaUnit.SubstitutionReduction
  using (beta-contractum-naturality; public-delta-naturality)

transport-type :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {term left right : Tm V} ->
  left ≡ right ->
  Σ ⊢ Γ ∶ term ∶ left ->
  Σ ⊢ Γ ∶ term ∶ right
transport-type refl derivation = derivation

ContextRenaming :
  {V W : Set} ->
  Context V ->
  Context W ->
  (V -> W) ->
  Set
ContextRenaming source target rho =
  (x : _) ->
  target (rho x) ≡ rename rho (source x)

closed-renaming :
  {V W : Set} ->
  (rho : V -> W) ->
  (term : Tm Empty) ->
  rename rho (closed term) ≡ closed term
closed-renaming rho term =
  trans
    (rename-as-substitution rho (closed term))
    (public-delta-naturality
      (λ x -> var (rho x))
      term)

instantiate-renaming :
  {V W : Set} ->
  (rho : V -> W) ->
  (body : Tm (Lift V)) ->
  (argument : Tm V) ->
  rename rho (instantiate body argument)
  ≡
  instantiate
    (rename (lift-map rho) body)
    (rename rho argument)
instantiate-renaming rho body argument =
  trans
    (rename-as-substitution rho (instantiate body argument))
    (trans
      (beta-contractum-naturality
        (λ x -> var (rho x))
        body
        argument)
      (cong₂ instantiate body-equality argument-equality))
  where
    lifted-pointwise :
      lift-substitution (λ x -> var (rho x))
      ≗
      (λ x -> var (lift-map rho x))
    lifted-pointwise bound = refl
    lifted-pointwise (free x) = refl

    body-equality :
      substitute
        (lift-substitution (λ x -> var (rho x)))
        body
      ≡
      rename (lift-map rho) body
    body-equality =
      trans
        (substitute-cong lifted-pointwise body)
        (sym (rename-as-substitution (lift-map rho) body))

    argument-equality :
      substitute (λ x -> var (rho x)) argument
      ≡
      rename rho argument
    argument-equality =
      sym (rename-as-substitution rho argument)

lift-context-renaming :
  {V W : Set} {source : Context V} {target : Context W}
  (rho : V -> W) ->
  ContextRenaming source target rho ->
  (parameter : Tm V) ->
  ContextRenaming
    (extend-context source parameter)
    (extend-context target (rename rho parameter))
    (lift-map rho)
lift-context-renaming rho respects parameter bound =
  trans
    (rename-composition rho free parameter)
    (sym (rename-composition free (lift-map rho) parameter))
lift-context-renaming {source = source}
  rho respects parameter (free x) =
  trans
    (cong weaken (respects x))
    (trans
      (rename-composition rho free (source x))
      (sym
        (rename-composition
          free
          (lift-map rho)
          (source x))))

rename-typing :
  {V W : Set} {Σ : Signature}
  {source : Context V} {target : Context W}
  (rho : V -> W) ->
  ContextRenaming source target rho ->
  {term type : Tm V} ->
  Σ ⊢ source ∶ term ∶ type ->
  Σ ⊢ target
    ∶ rename rho term
    ∶ rename rho type
rename-typing rho respects (type-sort level) =
  type-sort level
rename-typing rho respects type-unit-type =
  type-unit-type
rename-typing rho respects type-unit =
  type-unit
rename-typing rho respects (type-variable x) =
  transport-type
    (respects x)
    (type-variable (rho x))
rename-typing rho respects
  (type-global {entry = entry} lookup formation) =
  transport-type
    (sym (closed-renaming rho (declared-type entry)))
    (type-global lookup formation)
rename-typing rho respects
  (type-pi parameter body) =
  type-pi
    (rename-typing rho respects parameter)
    (rename-typing
      (lift-map rho)
      (lift-context-renaming rho respects _)
      body)
rename-typing rho respects
  (type-lambda parameter body) =
  type-lambda
    (rename-typing rho respects parameter)
    (rename-typing
      (lift-map rho)
      (lift-context-renaming rho respects _)
      body)
rename-typing rho respects
  (type-application {body = body} function argument) =
  transport-type
    (sym (instantiate-renaming rho body _))
    (type-application
      (rename-typing rho respects function)
      (rename-typing rho respects argument))

weakening-context-renaming :
  {V : Set} ->
  (context : Context V) ->
  (parameter : Tm V) ->
  ContextRenaming
    context
    (extend-context context parameter)
    free
weakening-context-renaming context parameter x = refl

weakening-typing :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {term type parameter : Tm V} ->
  Σ ⊢ Γ ∶ term ∶ type ->
  Σ ⊢ extend-context Γ parameter
    ∶ weaken term
    ∶ weaken type
weakening-typing {Γ = Γ} {parameter = parameter} derivation =
  rename-typing
    free
    (weakening-context-renaming Γ parameter)
    derivation

record TypedSubstitution
  {V W : Set}
  (Σ : Signature)
  (source : Context V)
  (target : Context W)
  (substitution : V -> Tm W) : Set where
  constructor typed-substitution
  field
    typed-image :
      (x : V) ->
      Σ ⊢ target
        ∶ substitution x
        ∶ substitute substitution (source x)

open TypedSubstitution public

lifted-entry-equality :
  {V W : Set} ->
  (substitution : V -> Tm W) ->
  (type : Tm V) ->
  weaken (substitute substitution type)
  ≡
  substitute (lift-substitution substitution) (weaken type)
lifted-entry-equality substitution type =
  trans
    (renaming-after-substitution substitution free type)
    (sym
      (substitution-after-renaming
        free
        (lift-substitution substitution)
        type))

lift-typed-substitution :
  {V W : Set} {Σ : Signature}
  {source : Context V} {target : Context W}
  {substitution : V -> Tm W} ->
  TypedSubstitution Σ source target substitution ->
  (parameter : Tm V) ->
  TypedSubstitution
    Σ
    (extend-context source parameter)
    (extend-context target (substitute substitution parameter))
    (lift-substitution substitution)
lift-typed-substitution
  {source = source}
  {target = target}
  {substitution = substitution}
  typed
  parameter =
  typed-substitution lifted-image
  where
    lifted-image :
      (x : _) ->
      _ ⊢ extend-context target (substitute substitution parameter)
        ∶ lift-substitution substitution x
        ∶ substitute
            (lift-substitution substitution)
            (extend-context source parameter x)
    lifted-image bound =
      transport-type
        (lifted-entry-equality substitution parameter)
        (type-variable bound)
    lifted-image (free x) =
      transport-type
        (lifted-entry-equality substitution (source x))
        (weakening-typing
          {parameter = substitute substitution parameter}
          (typed-image typed x))

substitution-typing :
  {V W : Set} {Σ : Signature}
  {source : Context V} {target : Context W}
  {substitution : V -> Tm W} ->
  TypedSubstitution Σ source target substitution ->
  {term type : Tm V} ->
  Σ ⊢ source ∶ term ∶ type ->
  Σ ⊢ target
    ∶ substitute substitution term
    ∶ substitute substitution type
substitution-typing typed (type-sort level) =
  type-sort level
substitution-typing typed type-unit-type =
  type-unit-type
substitution-typing typed type-unit =
  type-unit
substitution-typing typed (type-variable x) =
  typed-image typed x
substitution-typing {substitution = substitution} typed
  (type-global {entry = entry} lookup formation) =
  transport-type
    (sym
      (public-delta-naturality
        substitution
        (declared-type entry)))
    (type-global lookup formation)
substitution-typing {substitution = substitution} typed
  (type-pi {parameter = parameter} parameter-typing body-typing) =
  type-pi
    (substitution-typing typed parameter-typing)
    (substitution-typing
      (lift-typed-substitution typed parameter)
      body-typing)
substitution-typing {substitution = substitution} typed
  (type-lambda {parameter = parameter} parameter-typing body-typing) =
  type-lambda
    (substitution-typing typed parameter-typing)
    (substitution-typing
      (lift-typed-substitution typed parameter)
      body-typing)
substitution-typing {substitution = substitution} typed
  (type-application {body = body} function argument) =
  transport-type
    (sym (beta-contractum-naturality substitution body _))
    (type-application
      (substitution-typing typed function)
      (substitution-typing typed argument))

identity-typed-substitution :
  {V : Set} {Σ : Signature} ->
  (context : Context V) ->
  TypedSubstitution
    Σ
    context
    context
    identity-substitution
identity-typed-substitution context =
  typed-substitution
    (λ x ->
      transport-type
        (sym (substitution-identity (context x)))
        (type-variable x))

compose-typed-substitutions :
  {U V W : Set} {Σ : Signature}
  {first-context : Context U}
  {middle-context : Context V}
  {last-context : Context W}
  {first : U -> Tm V}
  {second : V -> Tm W} ->
  TypedSubstitution
    Σ first-context middle-context first ->
  TypedSubstitution
    Σ middle-context last-context second ->
  TypedSubstitution
    Σ first-context last-context (first then second)
compose-typed-substitutions
  {first-context = first-context}
  {first = first}
  {second = second}
  first-typed
  second-typed =
  typed-substitution composed-image
  where
    composed-image :
      (x : _) ->
      _ ⊢ _
        ∶ (first then second) x
        ∶ substitute (first then second) (first-context x)
    composed-image x =
      transport-type
        (substitution-composition
          first
          second
          (first-context x))
        (substitution-typing
          second-typed
          (typed-image first-typed x))

typed-substitution-left-identity :
  {V W : Set} {Σ : Signature}
  {source : Context V} {target : Context W}
  {substitution : V -> Tm W} ->
  TypedSubstitution Σ source target substitution ->
  (identity-substitution then substitution)
  ≗
  substitution
typed-substitution-left-identity typed x = refl

typed-substitution-right-identity :
  {V W : Set} {Σ : Signature}
  {source : Context V} {target : Context W}
  {substitution : V -> Tm W} ->
  TypedSubstitution Σ source target substitution ->
  (substitution then identity-substitution)
  ≗
  substitution
typed-substitution-right-identity
  {substitution = substitution}
  typed
  x =
  substitution-identity (substitution x)

typed-substitution-associativity :
  {T U V W : Set} {Σ : Signature}
  {first-context : Context T}
  {second-context : Context U}
  {third-context : Context V}
  {fourth-context : Context W}
  {first : T -> Tm U}
  {second : U -> Tm V}
  {third : V -> Tm W} ->
  TypedSubstitution
    Σ first-context second-context first ->
  TypedSubstitution
    Σ second-context third-context second ->
  TypedSubstitution
    Σ third-context fourth-context third ->
  ((first then second) then third)
  ≗
  (first then (second then third))
typed-substitution-associativity
  {first = first}
  {second = second}
  {third = third}
  first-typed
  second-typed
  third-typed =
  substitution-associativity first second third
