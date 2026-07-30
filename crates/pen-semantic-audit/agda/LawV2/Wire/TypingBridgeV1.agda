{-# OPTIONS --safe --without-K #-}

-- Phase E typing-judgment bridge, abstract layer.
--
-- The algorithmic layer (`TypingReplayV1`) replays kernel judgments and
-- produces intrinsic evidence; this module carries that evidence into
-- the abstract lambda/unit typing and conversion-typing judgments:
--
-- - `PExactTypingV1` is the intrinsic mirror of the abstract
--   syntax-directed judgment `_⊢_∶_∶_`, over `PTm`/`PCtx` and the
--   strict-prior signature mirror `PSig`, with a per-slot universe
--   assignment; `pexact-typing-decode` proves every intrinsic
--   derivation decodes to a genuine abstract derivation under the
--   abstract signature built from the same `PSig`;
-- - `decode-pstep` decodes every intrinsic base-Q0 step — beta,
--   stored-body delta, and all six congruence frames — to the abstract
--   `Step` relation (whose congruence closure is introduced for
--   exactly this purpose);
-- - `PTypedStepsV1` is the intrinsic mirror of the abstract
--   `TypedSteps` (per-edge raw step plus exact typings of both sides
--   at one type), and decodes to it;
-- - the intrinsic `TypeFormation`/`HasType` equivalence records decode
--   to `TypeFormationEquivalentV2`/`HasTypeEquivalentV2`, hence to
--   `BaseQ0Equivalent`, and the intrinsic application assembly lands
--   in the abstract conversion-typing judgment `_⊢c_∶_∶_` through
--   `application-elimination-conversion-soundness-v2`.
--
-- The exact-typing hypotheses these theorems consume are the
-- deliberately explicit frontier: the algorithmic checker accepts
-- kernel-convertible content whose intermediates need not be typable in
-- the conversion-free exact judgment, and closing that gap is the
-- abstract module's own registered frontier
-- (`full-eight-constructor-decoded-soundness-not-yet-derivable` in
-- `ProductionSynthesisCodeV2`), which requires conversion-typing
-- subject-reduction metatheory outside Phase E. Nothing here mints
-- authority.

module LawV2.Wire.TypingBridgeV1 where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe
  using ()
  renaming (Maybe to WireMaybe; just to wire-just; nothing to wire-nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.LambdaUnit.ProductionSyntaxV1
open import LawV2.LambdaUnit.ProductionDecodingV1
open import LawV2.LambdaUnit.Substitution
  using (rename-as-substitution; substitute-cong; _≗_)
open import LawV2.LambdaUnit.SubstitutionReduction
  using (Step; beta-step; public-delta-step;
         pi-parameter-congruence-step; pi-body-congruence-step;
         lambda-parameter-congruence-step; lambda-body-congruence-step;
         apply-function-congruence-step; apply-argument-congruence-step;
         transport-step; empty-elimination)
open import LawV2.LambdaUnit.TypingJudgment
open import LawV2.LambdaUnit.SubstitutionTypingV3 using (transport-type)
open import LawV2.LambdaUnit.ReductionTyping using (typed-reduction)
open import LawV2.LambdaUnit.ProductionConversionTyping
  using (TypedSteps; typed-steps-refl; typed-steps-next;
         BaseQ0Equivalent; _⊢c_∶_∶_; conversion-exact;
         conversion-convert; conversion-application;
         transport-conversion-type)
open import LawV2.LambdaUnit.ProductionSynthesisCodeV2
  using (TypeFormationEquivalentV2; type-formation-equivalent-v2;
         HasTypeEquivalentV2; has-type-equivalent-v2;
         type-formation-equivalent-to-base-q0-v2;
         application-elimination-conversion-soundness-v2;
         transport-context-typing-v2)
open import LawV2.Wire.ContextCorrespondenceV1
  using (PSig; psig-empty; psig-snoc; lookup-psig-type;
         lookup-psig-body; pinstantiate; decode-pinstantiate)
open import LawV2.Wire.SemanticReplayV1
  using (PStepV1; pstep-beta; pstep-delta; pstep-pi-parameter;
         pstep-pi-body; pstep-lambda-parameter; pstep-lambda-body;
         pstep-apply-function; pstep-apply-argument; PStepsV1;
         psteps-refl; psteps-next; pembed-closed; fin-absurd)

-- Transport helpers on the abstract judgment.
transport-term-typing :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {left right ty : Tm V} →
  left ≡ right →
  Σ ⊢ Γ ∶ left ∶ ty →
  Σ ⊢ Γ ∶ right ∶ ty
transport-term-typing refl derivation = derivation

-- The closed embedding decodes to the abstract `closed` image.
decode-pembed-closed :
  {globals locals : Nat} (term : PTm globals zero) →
  decode-ptm (pembed-closed {globals} {locals} term)
  ≡ closed (decode-ptm term)
decode-pembed-closed {globals} {locals} term =
  trans
    (decode-prename fin-absurd term)
    (trans
      (rename-as-substitution
        (decode-prenaming (fin-absurd {Fin locals})) (decode-ptm term))
      (substitute-cong vacuous (decode-ptm term)))
  where
  vacuous :
    (λ x → var (decode-prenaming (fin-absurd {Fin locals}) x))
    ≗ empty-elimination
  vacuous ()

-- The abstract signature carried by a strict-prior signature mirror and
-- a per-slot universe assignment: chronological ordinal `n` resolves to
-- the decoded declared type at slot `n` with the assigned universe.
psig-abstract-signature :
  {globals : Nat} → PSig globals → (Fin globals → Nat) → Signature
psig-abstract-signature {globals} sig levels =
  signature lookup
  where
  lookup : Nat → Maybe GlobalDeclaration
  lookup identifier = branch (encode-fin-ordinal globals identifier)
    where
    branch : Maybe (Fin globals) → Maybe GlobalDeclaration
    branch nothing = nothing
    branch (just slot) =
      just
        (declaration (decode-ptm (lookup-psig-type sig slot))
          (levels slot))

psig-lookup-global :
  {globals : Nat} (sig : PSig globals) (levels : Fin globals → Nat)
  (slot : Fin globals) →
  lookup-global (psig-abstract-signature sig levels) (fin-ordinal slot)
  ≡ just
      (declaration (decode-ptm (lookup-psig-type sig slot))
        (levels slot))
psig-lookup-global {globals} sig levels slot
  rewrite encode-fin-ordinal-round-trip slot = refl

-- The intrinsic mirror of the abstract exact typing judgment. The
-- global rule carries the formation of the embedded strict-prior
-- declared type at its assigned universe, exactly as the abstract
-- `type-global` rule does.
data PExactTypingV1
  {globals : Nat} (sig : PSig globals)
  (levels : Fin globals → Nat) :
  {locals : Nat} → PCtx globals locals →
  PTm globals locals → PTm globals locals → Set where

  pexact-sort :
    {locals : Nat} {context : PCtx globals locals} (level : Nat) →
    PExactTypingV1 sig levels context
      (psort level) (psort (suc level))

  pexact-unit-type :
    {locals : Nat} {context : PCtx globals locals} →
    PExactTypingV1 sig levels context punit-type (psort zero)

  pexact-unit :
    {locals : Nat} {context : PCtx globals locals} →
    PExactTypingV1 sig levels context punit punit-type

  pexact-variable :
    {locals : Nat} {context : PCtx globals locals} (x : Fin locals) →
    PExactTypingV1 sig levels context
      (pvar x) (lookup-pctx context x)

  pexact-global :
    {locals : Nat} {context : PCtx globals locals}
    (slot : Fin globals) →
    PExactTypingV1 sig levels pempty
      (pembed-closed (lookup-psig-type sig slot))
      (psort (levels slot)) →
    PExactTypingV1 sig levels context
      (pglobal slot) (pembed-closed (lookup-psig-type sig slot))

  pexact-pi :
    {locals : Nat} {context : PCtx globals locals}
    {parameter : PTm globals locals}
    {body : PTm globals (suc locals)}
    {parameter-level body-level : Nat} →
    PExactTypingV1 sig levels context
      parameter (psort parameter-level) →
    PExactTypingV1 sig levels (context psnoc parameter)
      body (psort body-level) →
    PExactTypingV1 sig levels context
      (ppi parameter body) (psort (parameter-level max body-level))

  pexact-lambda :
    {locals : Nat} {context : PCtx globals locals}
    {parameter : PTm globals locals}
    {body body-type : PTm globals (suc locals)}
    {parameter-level : Nat} →
    PExactTypingV1 sig levels context
      parameter (psort parameter-level) →
    PExactTypingV1 sig levels (context psnoc parameter)
      body body-type →
    PExactTypingV1 sig levels context
      (plam parameter body) (ppi parameter body-type)

  pexact-application :
    {locals : Nat} {context : PCtx globals locals}
    {function argument : PTm globals locals}
    {pi-parameter : PTm globals locals}
    {pi-body : PTm globals (suc locals)} →
    PExactTypingV1 sig levels context
      function (ppi pi-parameter pi-body) →
    PExactTypingV1 sig levels context argument pi-parameter →
    PExactTypingV1 sig levels context
      (papp function argument) (pinstantiate pi-body argument)

-- Every intrinsic exact derivation decodes to a genuine abstract
-- derivation under the abstract signature built from the same mirror.
pexact-typing-decode :
  {globals : Nat} {sig : PSig globals}
  {levels : Fin globals → Nat}
  {locals : Nat} {context : PCtx globals locals}
  {term ty : PTm globals locals} →
  PExactTypingV1 sig levels context term ty →
  psig-abstract-signature sig levels
    ⊢ decode-pctx context ∶ decode-ptm term ∶ decode-ptm ty
pexact-typing-decode (pexact-sort level) = type-sort level
pexact-typing-decode pexact-unit-type = type-unit-type
pexact-typing-decode pexact-unit = type-unit
pexact-typing-decode {context = context} (pexact-variable x) =
  transport-type
    (sym (decode-lookup-pctx context x))
    (type-variable (decode-fin x))
pexact-typing-decode {sig = sig} {levels = levels}
  (pexact-global slot formation) =
  transport-type
    (sym (decode-pembed-closed (lookup-psig-type sig slot)))
    (type-global
      (psig-lookup-global sig levels slot)
      (transport-context-typing-v2 empty-pointwise
        (transport-term-typing
          (decode-pembed-closed (lookup-psig-type sig slot))
          (pexact-typing-decode formation))))
  where
  empty-pointwise : decode-pctx pempty ≗ empty-context
  empty-pointwise ()
pexact-typing-decode (pexact-pi parameter body) =
  type-pi
    (pexact-typing-decode parameter)
    (transport-context-typing-v2
      (decode-pctx-snoc _ _)
      (pexact-typing-decode body))
pexact-typing-decode (pexact-lambda parameter body) =
  type-lambda
    (pexact-typing-decode parameter)
    (transport-context-typing-v2
      (decode-pctx-snoc _ _)
      (pexact-typing-decode body))
pexact-typing-decode
  (pexact-application {argument = argument} {pi-body = pi-body}
    function argument-typing) =
  transport-type
    (sym (decode-pinstantiate pi-body argument))
    (type-application
      (pexact-typing-decode function)
      (pexact-typing-decode argument-typing))

pexact-conversion-decode :
  {globals : Nat} {sig : PSig globals}
  {levels : Fin globals → Nat}
  {locals : Nat} {context : PCtx globals locals}
  {term ty : PTm globals locals} →
  PExactTypingV1 sig levels context term ty →
  psig-abstract-signature sig levels
    ⊢c decode-pctx context ∶ decode-ptm term ∶ decode-ptm ty
pexact-conversion-decode derivation =
  conversion-exact (pexact-typing-decode derivation)

-- Every intrinsic base-Q0 step decodes to the abstract Step relation.
-- The delta case forgets which delta map authorized the unfolding: the
-- abstract `public-delta-step` records only the identifier and body,
-- while authorization lives in the intrinsic delta map itself.
decode-pstep :
  {globals : Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {left right : PTm globals locals} →
  PStepV1 delta left right →
  Step (decode-ptm left) (decode-ptm right)
decode-pstep
  (pstep-beta {parameter = parameter} {body = body}
    {argument = argument}) =
  transport-step refl
    (sym (decode-pinstantiate body argument))
    (beta-step (decode-ptm parameter) (decode-ptm body)
      (decode-ptm argument))
decode-pstep (pstep-delta {slot = slot} {body = body} looked) =
  transport-step refl
    (sym (decode-pembed-closed body))
    (public-delta-step (fin-ordinal slot) (decode-ptm body))
decode-pstep (pstep-pi-parameter {body = body} premise) =
  pi-parameter-congruence-step (decode-ptm body)
    (decode-pstep premise)
decode-pstep (pstep-pi-body {parameter = parameter} premise) =
  pi-body-congruence-step (decode-ptm parameter)
    (decode-pstep premise)
decode-pstep (pstep-lambda-parameter {body = body} premise) =
  lambda-parameter-congruence-step (decode-ptm body)
    (decode-pstep premise)
decode-pstep (pstep-lambda-body {parameter = parameter} premise) =
  lambda-body-congruence-step (decode-ptm parameter)
    (decode-pstep premise)
decode-pstep (pstep-apply-function {argument = argument} premise) =
  apply-function-congruence-step (decode-ptm argument)
    (decode-pstep premise)
decode-pstep (pstep-apply-argument {function = function} premise) =
  apply-argument-congruence-step (decode-ptm function)
    (decode-pstep premise)

-- The intrinsic mirror of the abstract `TypedSteps`: a raw intrinsic
-- step whose endpoints are exactly typed at one type, chained
-- reflexively-transitively.
record PTypedStepV1
  {globals : Nat} (sig : PSig globals)
  (levels : Fin globals → Nat)
  (delta : Fin globals → WireMaybe (PTm globals zero))
  {locals : Nat} (context : PCtx globals locals)
  (ty left right : PTm globals locals) : Set where
  constructor ptyped-step
  field
    typed-step-raw : PStepV1 delta left right
    typed-step-left : PExactTypingV1 sig levels context left ty
    typed-step-right : PExactTypingV1 sig levels context right ty

open PTypedStepV1 public

data PTypedStepsV1
  {globals : Nat} (sig : PSig globals)
  (levels : Fin globals → Nat)
  (delta : Fin globals → WireMaybe (PTm globals zero))
  {locals : Nat} (context : PCtx globals locals)
  (ty : PTm globals locals) :
  PTm globals locals → PTm globals locals → Set where

  ptyped-steps-refl :
    {term : PTm globals locals} →
    PExactTypingV1 sig levels context term ty →
    PTypedStepsV1 sig levels delta context ty term term

  ptyped-steps-next :
    {left middle right : PTm globals locals} →
    PTypedStepV1 sig levels delta context ty left middle →
    PTypedStepsV1 sig levels delta context ty middle right →
    PTypedStepsV1 sig levels delta context ty left right

decode-ptyped-steps :
  {globals : Nat} {sig : PSig globals}
  {levels : Fin globals → Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {context : PCtx globals locals}
  {ty left right : PTm globals locals} →
  PTypedStepsV1 sig levels delta context ty left right →
  TypedSteps (psig-abstract-signature sig levels)
    (decode-pctx context) (decode-ptm ty)
    (decode-ptm left) (decode-ptm right)
decode-ptyped-steps (ptyped-steps-refl typing) =
  typed-steps-refl (pexact-typing-decode typing)
decode-ptyped-steps (ptyped-steps-next step rest) =
  typed-steps-next
    (typed-reduction
      (decode-pstep (typed-step-raw step))
      (pexact-typing-decode (typed-step-left step))
      (pexact-typing-decode (typed-step-right step)))
    (decode-ptyped-steps rest)

-- Intrinsic conversion equivalences with exact typings, and their
-- decodings into the abstract replay-mode records.
record PTypeFormationEquivalentV1
  {globals : Nat} (sig : PSig globals)
  (levels : Fin globals → Nat)
  (delta : Fin globals → WireMaybe (PTm globals zero))
  {locals : Nat} (context : PCtx globals locals)
  (left right : PTm globals locals) : Set where
  constructor ptype-formation-equivalent
  field
    pformation-universe : Nat
    pformation-common : PTm globals locals
    pformation-left-trace :
      PTypedStepsV1 sig levels delta context
        (psort pformation-universe) left pformation-common
    pformation-right-trace :
      PTypedStepsV1 sig levels delta context
        (psort pformation-universe) right pformation-common

open PTypeFormationEquivalentV1 public

record PHasTypeEquivalentV1
  {globals : Nat} (sig : PSig globals)
  (levels : Fin globals → Nat)
  (delta : Fin globals → WireMaybe (PTm globals zero))
  {locals : Nat} (context : PCtx globals locals)
  (expected left right : PTm globals locals) : Set where
  constructor phas-type-equivalent
  field
    phas-type-common : PTm globals locals
    phas-type-left-trace :
      PTypedStepsV1 sig levels delta context expected left
        phas-type-common
    phas-type-right-trace :
      PTypedStepsV1 sig levels delta context expected right
        phas-type-common

open PHasTypeEquivalentV1 public

decode-type-formation-equivalent :
  {globals : Nat} {sig : PSig globals}
  {levels : Fin globals → Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {context : PCtx globals locals}
  {left right : PTm globals locals} →
  PTypeFormationEquivalentV1 sig levels delta context left right →
  TypeFormationEquivalentV2 (psig-abstract-signature sig levels)
    (decode-pctx context) (decode-ptm left) (decode-ptm right)
decode-type-formation-equivalent equivalent =
  type-formation-equivalent-v2
    (pformation-universe equivalent)
    (decode-ptm (pformation-common equivalent))
    (decode-ptyped-steps (pformation-left-trace equivalent))
    (decode-ptyped-steps (pformation-right-trace equivalent))

decode-has-type-equivalent :
  {globals : Nat} {sig : PSig globals}
  {levels : Fin globals → Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {context : PCtx globals locals}
  {expected left right : PTm globals locals} →
  PHasTypeEquivalentV1 sig levels delta context expected left right →
  HasTypeEquivalentV2 (psig-abstract-signature sig levels)
    (decode-pctx context) (decode-ptm expected)
    (decode-ptm left) (decode-ptm right)
decode-has-type-equivalent equivalent =
  has-type-equivalent-v2
    (decode-ptm (phas-type-common equivalent))
    (decode-ptyped-steps (phas-type-left-trace equivalent))
    (decode-ptyped-steps (phas-type-right-trace equivalent))

decode-formation-base-q0 :
  {globals : Nat} {sig : PSig globals}
  {levels : Fin globals → Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {context : PCtx globals locals}
  {left right : PTm globals locals} →
  PTypeFormationEquivalentV1 sig levels delta context left right →
  BaseQ0Equivalent (psig-abstract-signature sig levels)
    (decode-pctx context) (decode-ptm left) (decode-ptm right)
decode-formation-base-q0 equivalent =
  type-formation-equivalent-to-base-q0-v2
    (decode-type-formation-equivalent equivalent)

-- Conversion transport at the abstract level, from intrinsic
-- equivalence evidence.
pconversion-convert-decode :
  {globals : Nat} {sig : PSig globals}
  {levels : Fin globals → Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {context : PCtx globals locals}
  {term left right : PTm globals locals} →
  psig-abstract-signature sig levels
    ⊢c decode-pctx context ∶ decode-ptm term ∶ decode-ptm left →
  PTypeFormationEquivalentV1 sig levels delta context left right →
  psig-abstract-signature sig levels
    ⊢c decode-pctx context ∶ decode-ptm term ∶ decode-ptm right
pconversion-convert-decode typing equivalent =
  conversion-convert typing (decode-formation-base-q0 equivalent)

-- Conversion-mediated application elimination, assembled from intrinsic
-- premises: the synthesized function type converts to a dependent
-- product through a type-formation equivalence, the synthesized
-- argument type converts to its parameter, and the abstract
-- conversion-typing judgment closes the converted premises under
-- application at the decoded intrinsic instantiation.
papplication-elimination-decode :
  {globals : Nat} {sig : PSig globals}
  {levels : Fin globals → Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {context : PCtx globals locals}
  {function argument function-type argument-type :
    PTm globals locals}
  {pi-parameter : PTm globals locals}
  {pi-body : PTm globals (suc locals)} →
  psig-abstract-signature sig levels
    ⊢c decode-pctx context
    ∶ decode-ptm function ∶ decode-ptm function-type →
  PTypeFormationEquivalentV1 sig levels delta context
    function-type (ppi pi-parameter pi-body) →
  psig-abstract-signature sig levels
    ⊢c decode-pctx context
    ∶ decode-ptm argument ∶ decode-ptm argument-type →
  PTypeFormationEquivalentV1 sig levels delta context
    argument-type pi-parameter →
  psig-abstract-signature sig levels
    ⊢c decode-pctx context
    ∶ app (decode-ptm function) (decode-ptm argument)
    ∶ decode-ptm (pinstantiate pi-body argument)
papplication-elimination-decode
  {argument = argument} {pi-body = pi-body}
  function-typing function-equivalent argument-typing
  argument-equivalent =
  transport-conversion-type
    (sym (decode-pinstantiate pi-body argument))
    (application-elimination-conversion-soundness-v2
      function-typing
      (decode-type-formation-equivalent function-equivalent)
      argument-typing
      (decode-type-formation-equivalent argument-equivalent))
