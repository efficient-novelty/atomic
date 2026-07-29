{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.ProductionSynthesisCodeV2 where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.LambdaUnit.ProductionSyntaxV1
open import LawV2.LambdaUnit.ProductionDecodingV1
open import LawV2.LambdaUnit.ProductionConversionTyping
open import LawV2.LambdaUnit.TypingJudgment
open import LawV2.LambdaUnit.SubstitutionTypingV3
  using (transport-type)
open import LawV2.LambdaUnit.Substitution
  using (_≗_; cong; cong₂; sym; trans)

-- Exact canonical tags from Rust SynthesisCodeV2.
data SynthesisTagV2 : Set where
  sort-tag-v2 : SynthesisTagV2
  unit-type-tag-v2 : SynthesisTagV2
  unit-tag-v2 : SynthesisTagV2
  variable-lookup-tag-v2 : SynthesisTagV2
  global-lookup-tag-v2 : SynthesisTagV2
  pi-formation-tag-v2 : SynthesisTagV2
  lambda-introduction-tag-v2 : SynthesisTagV2
  application-elimination-tag-v2 : SynthesisTagV2

synthesis-tag-number-v2 : SynthesisTagV2 -> Nat
synthesis-tag-number-v2 sort-tag-v2 = zero
synthesis-tag-number-v2 unit-type-tag-v2 = suc zero
synthesis-tag-number-v2 unit-tag-v2 = suc (suc zero)
synthesis-tag-number-v2 variable-lookup-tag-v2 =
  suc (suc (suc zero))
synthesis-tag-number-v2 global-lookup-tag-v2 =
  suc (suc (suc (suc zero)))
synthesis-tag-number-v2 pi-formation-tag-v2 =
  suc (suc (suc (suc (suc zero))))
synthesis-tag-number-v2 lambda-introduction-tag-v2 =
  suc (suc (suc (suc (suc (suc zero)))))
synthesis-tag-number-v2 application-elimination-tag-v2 =
  suc (suc (suc (suc (suc (suc (suc zero))))))

decode-synthesis-tag-v2 : Nat -> Maybe SynthesisTagV2
decode-synthesis-tag-v2 zero =
  just sort-tag-v2
decode-synthesis-tag-v2 (suc zero) =
  just unit-type-tag-v2
decode-synthesis-tag-v2 (suc (suc zero)) =
  just unit-tag-v2
decode-synthesis-tag-v2 (suc (suc (suc zero))) =
  just variable-lookup-tag-v2
decode-synthesis-tag-v2
  (suc (suc (suc (suc zero)))) =
  just global-lookup-tag-v2
decode-synthesis-tag-v2
  (suc (suc (suc (suc (suc zero))))) =
  just pi-formation-tag-v2
decode-synthesis-tag-v2
  (suc (suc (suc (suc (suc (suc zero)))))) =
  just lambda-introduction-tag-v2
decode-synthesis-tag-v2
  (suc (suc (suc (suc (suc (suc (suc zero))))))) =
  just application-elimination-tag-v2
decode-synthesis-tag-v2
  (suc (suc (suc (suc (suc (suc (suc (suc extra)))))))) =
  nothing

decode-encode-synthesis-tag-v2 :
  (tag : SynthesisTagV2) ->
  decode-synthesis-tag-v2 (synthesis-tag-number-v2 tag)
  ≡
  just tag
decode-encode-synthesis-tag-v2 sort-tag-v2 = refl
decode-encode-synthesis-tag-v2 unit-type-tag-v2 = refl
decode-encode-synthesis-tag-v2 unit-tag-v2 = refl
decode-encode-synthesis-tag-v2 variable-lookup-tag-v2 = refl
decode-encode-synthesis-tag-v2 global-lookup-tag-v2 = refl
decode-encode-synthesis-tag-v2 pi-formation-tag-v2 = refl
decode-encode-synthesis-tag-v2 lambda-introduction-tag-v2 = refl
decode-encode-synthesis-tag-v2 application-elimination-tag-v2 =
  refl

just-injective :
  {A : Set} {left right : A} ->
  just left ≡ just right ->
  left ≡ right
just-injective refl = refl

synthesis-tag-number-injective-v2 :
  {left right : SynthesisTagV2} ->
  synthesis-tag-number-v2 left
  ≡
  synthesis-tag-number-v2 right ->
  left ≡ right
synthesis-tag-number-injective-v2 {left} {right} equality =
  just-injective
    (trans
      (sym (decode-encode-synthesis-tag-v2 left))
      (trans
        (cong decode-synthesis-tag-v2 equality)
        (decode-encode-synthesis-tag-v2 right)))

decode-synthesis-tag-after-seven-v2 :
  (extra : Nat) ->
  decode-synthesis-tag-v2
    (suc (suc (suc (suc (suc (suc (suc (suc extra))))))))
  ≡
  nothing
decode-synthesis-tag-after-seven-v2 extra = refl

data SynthesisTagAtV2 :
  SynthesisTagV2 ->
  Nat ->
  Set where

  sort-tag-at-zero-v2 :
    SynthesisTagAtV2 sort-tag-v2 zero

  unit-type-tag-at-one-v2 :
    SynthesisTagAtV2 unit-type-tag-v2 (suc zero)

  unit-tag-at-two-v2 :
    SynthesisTagAtV2 unit-tag-v2 (suc (suc zero))

  variable-lookup-tag-at-three-v2 :
    SynthesisTagAtV2
      variable-lookup-tag-v2
      (suc (suc (suc zero)))

  global-lookup-tag-at-four-v2 :
    SynthesisTagAtV2
      global-lookup-tag-v2
      (suc (suc (suc (suc zero))))

  pi-formation-tag-at-five-v2 :
    SynthesisTagAtV2
      pi-formation-tag-v2
      (suc (suc (suc (suc (suc zero)))))

  lambda-introduction-tag-at-six-v2 :
    SynthesisTagAtV2
      lambda-introduction-tag-v2
      (suc (suc (suc (suc (suc (suc zero))))))

  application-elimination-tag-at-seven-v2 :
    SynthesisTagAtV2
      application-elimination-tag-v2
      (suc (suc (suc (suc (suc (suc (suc zero)))))))

synthesis-tag-inventory-complete-v2 :
  (tag : SynthesisTagV2) ->
  SynthesisTagAtV2 tag (synthesis-tag-number-v2 tag)
synthesis-tag-inventory-complete-v2 sort-tag-v2 =
  sort-tag-at-zero-v2
synthesis-tag-inventory-complete-v2 unit-type-tag-v2 =
  unit-type-tag-at-one-v2
synthesis-tag-inventory-complete-v2 unit-tag-v2 =
  unit-tag-at-two-v2
synthesis-tag-inventory-complete-v2 variable-lookup-tag-v2 =
  variable-lookup-tag-at-three-v2
synthesis-tag-inventory-complete-v2 global-lookup-tag-v2 =
  global-lookup-tag-at-four-v2
synthesis-tag-inventory-complete-v2 pi-formation-tag-v2 =
  pi-formation-tag-at-five-v2
synthesis-tag-inventory-complete-v2 lambda-introduction-tag-v2 =
  lambda-introduction-tag-at-six-v2
synthesis-tag-inventory-complete-v2 application-elimination-tag-v2 =
  application-elimination-tag-at-seven-v2

synthesis-tag-inventory-unique-v2 :
  {tag : SynthesisTagV2} {number : Nat} ->
  SynthesisTagAtV2 tag number ->
  number ≡ synthesis-tag-number-v2 tag
synthesis-tag-inventory-unique-v2 sort-tag-at-zero-v2 = refl
synthesis-tag-inventory-unique-v2 unit-type-tag-at-one-v2 = refl
synthesis-tag-inventory-unique-v2 unit-tag-at-two-v2 = refl
synthesis-tag-inventory-unique-v2
  variable-lookup-tag-at-three-v2 = refl
synthesis-tag-inventory-unique-v2
  global-lookup-tag-at-four-v2 = refl
synthesis-tag-inventory-unique-v2
  pi-formation-tag-at-five-v2 = refl
synthesis-tag-inventory-unique-v2
  lambda-introduction-tag-at-six-v2 = refl
synthesis-tag-inventory-unique-v2
  application-elimination-tag-at-seven-v2 = refl

-- Payload-erased eight-constructor structural image of SynthesisCodeV2.
data ProductionSynthesisStructureV2
  (global-count count : Nat) :
  Set where

  code-sort-v2 :
    Nat ->
    ProductionSynthesisStructureV2 global-count count

  code-unit-type-v2 :
    ProductionSynthesisStructureV2 global-count count

  code-unit-v2 :
    ProductionSynthesisStructureV2 global-count count

  code-variable-lookup-v2 :
    Fin count ->
    ProductionSynthesisStructureV2 global-count count

  code-global-lookup-v2 :
    Fin global-count ->
    ProductionSynthesisStructureV2 global-count count

  code-pi-formation-v2 :
    ProductionSynthesisStructureV2 global-count count ->
    ProductionSynthesisStructureV2 global-count (suc count) ->
    ProductionSynthesisStructureV2 global-count count

  code-lambda-introduction-v2 :
    ProductionSynthesisStructureV2 global-count count ->
    ProductionSynthesisStructureV2 global-count (suc count) ->
    ProductionSynthesisStructureV2 global-count count

  code-application-elimination-v2 :
    ProductionSynthesisStructureV2 global-count count ->
    ProductionSynthesisStructureV2 global-count count ->
    ProductionSynthesisStructureV2 global-count count

-- Intrinsically scoped mirror of the exact eight Rust constructors and their
-- field arities. `ConversionPayload count` stands for each serialized
-- BaseQ0ConversionCodeV2. Its exact trace/census wire correspondence remains a
-- frontier. A verified Rust u32 variable index and global slot become Fin;
-- the redundant variable metadata and abstract identifier are retained.
data ProductionSynthesisCodeV2
  (ConversionPayload : Nat -> Set)
  (global-count count : Nat) :
  Set where

  synthesis-sort-v2 :
    (level : Nat) ->
    ProductionSynthesisCodeV2
      ConversionPayload global-count count

  synthesis-unit-type-v2 :
    ProductionSynthesisCodeV2
      ConversionPayload global-count count

  synthesis-unit-v2 :
    ProductionSynthesisCodeV2
      ConversionPayload global-count count

  synthesis-variable-lookup-v2 :
    (index : Fin count) ->
    (context-ordinal shift-distance : Nat) ->
    ProductionSynthesisCodeV2
      ConversionPayload global-count count

  synthesis-global-lookup-v2 :
    (identifier : Nat) ->
    (global-slot : Fin global-count) ->
    ProductionSynthesisCodeV2
      ConversionPayload global-count count

  synthesis-pi-formation-v2 :
    ProductionSynthesisCodeV2
      ConversionPayload global-count count ->
    ProductionSynthesisCodeV2
      ConversionPayload global-count (suc count) ->
    ProductionSynthesisCodeV2
      ConversionPayload global-count count

  synthesis-lambda-introduction-v2 :
    ProductionSynthesisCodeV2
      ConversionPayload global-count count ->
    ProductionSynthesisCodeV2
      ConversionPayload global-count (suc count) ->
    ProductionSynthesisCodeV2
      ConversionPayload global-count count

  synthesis-application-elimination-v2 :
    ProductionSynthesisCodeV2
      ConversionPayload global-count count ->
    ProductionSynthesisCodeV2
      ConversionPayload global-count count ->
    (function-conversion argument-conversion :
      ConversionPayload count) ->
    ProductionSynthesisCodeV2
      ConversionPayload global-count count

production-synthesis-code-tag-v2 :
  {ConversionPayload : Nat -> Set}
  {global-count count : Nat} ->
  ProductionSynthesisCodeV2
    ConversionPayload global-count count ->
  SynthesisTagV2
production-synthesis-code-tag-v2
  (synthesis-sort-v2 level) =
  sort-tag-v2
production-synthesis-code-tag-v2
  synthesis-unit-type-v2 =
  unit-type-tag-v2
production-synthesis-code-tag-v2
  synthesis-unit-v2 =
  unit-tag-v2
production-synthesis-code-tag-v2
  (synthesis-variable-lookup-v2
    index context-ordinal shift-distance) =
  variable-lookup-tag-v2
production-synthesis-code-tag-v2
  (synthesis-global-lookup-v2 identifier global-slot) =
  global-lookup-tag-v2
production-synthesis-code-tag-v2
  (synthesis-pi-formation-v2 parameter body) =
  pi-formation-tag-v2
production-synthesis-code-tag-v2
  (synthesis-lambda-introduction-v2 parameter body) =
  lambda-introduction-tag-v2
production-synthesis-code-tag-v2
  (synthesis-application-elimination-v2
    function argument
    function-conversion argument-conversion) =
  application-elimination-tag-v2

erase-production-synthesis-code-v2 :
  {ConversionPayload : Nat -> Set}
  {global-count count : Nat} ->
  ProductionSynthesisCodeV2
    ConversionPayload global-count count ->
  ProductionSynthesisStructureV2 global-count count
erase-production-synthesis-code-v2
  (synthesis-sort-v2 level) =
  code-sort-v2 level
erase-production-synthesis-code-v2
  synthesis-unit-type-v2 =
  code-unit-type-v2
erase-production-synthesis-code-v2
  synthesis-unit-v2 =
  code-unit-v2
erase-production-synthesis-code-v2
  (synthesis-variable-lookup-v2
    index context-ordinal shift-distance) =
  code-variable-lookup-v2 index
erase-production-synthesis-code-v2
  (synthesis-global-lookup-v2 identifier global-slot) =
  code-global-lookup-v2 global-slot
erase-production-synthesis-code-v2
  (synthesis-pi-formation-v2 parameter body) =
  code-pi-formation-v2
    (erase-production-synthesis-code-v2 parameter)
    (erase-production-synthesis-code-v2 body)
erase-production-synthesis-code-v2
  (synthesis-lambda-introduction-v2 parameter body) =
  code-lambda-introduction-v2
    (erase-production-synthesis-code-v2 parameter)
    (erase-production-synthesis-code-v2 body)
erase-production-synthesis-code-v2
  (synthesis-application-elimination-v2
    function argument
    function-conversion argument-conversion) =
  code-application-elimination-v2
    (erase-production-synthesis-code-v2 function)
    (erase-production-synthesis-code-v2 argument)

production-synthesis-tag-v2 :
  {global-count count : Nat} ->
  ProductionSynthesisStructureV2 global-count count ->
  SynthesisTagV2
production-synthesis-tag-v2 (code-sort-v2 level) =
  sort-tag-v2
production-synthesis-tag-v2 code-unit-type-v2 =
  unit-type-tag-v2
production-synthesis-tag-v2 code-unit-v2 =
  unit-tag-v2
production-synthesis-tag-v2 (code-variable-lookup-v2 x) =
  variable-lookup-tag-v2
production-synthesis-tag-v2 (code-global-lookup-v2 slot) =
  global-lookup-tag-v2
production-synthesis-tag-v2
  (code-pi-formation-v2 parameter body) =
  pi-formation-tag-v2
production-synthesis-tag-v2
  (code-lambda-introduction-v2 parameter body) =
  lambda-introduction-tag-v2
production-synthesis-tag-v2
  (code-application-elimination-v2 function argument) =
  application-elimination-tag-v2

production-synthesis-code-tag-erasure-v2 :
  {ConversionPayload : Nat -> Set}
  {global-count count : Nat} ->
  (code :
    ProductionSynthesisCodeV2
      ConversionPayload global-count count) ->
  production-synthesis-tag-v2
    (erase-production-synthesis-code-v2 code)
  ≡
  production-synthesis-code-tag-v2 code
production-synthesis-code-tag-erasure-v2
  (synthesis-sort-v2 level) =
  refl
production-synthesis-code-tag-erasure-v2
  synthesis-unit-type-v2 =
  refl
production-synthesis-code-tag-erasure-v2
  synthesis-unit-v2 =
  refl
production-synthesis-code-tag-erasure-v2
  (synthesis-variable-lookup-v2
    index context-ordinal shift-distance) =
  refl
production-synthesis-code-tag-erasure-v2
  (synthesis-global-lookup-v2 identifier global-slot) =
  refl
production-synthesis-code-tag-erasure-v2
  (synthesis-pi-formation-v2 parameter body) =
  refl
production-synthesis-code-tag-erasure-v2
  (synthesis-lambda-introduction-v2 parameter body) =
  refl
production-synthesis-code-tag-erasure-v2
  (synthesis-application-elimination-v2
    function argument
    function-conversion argument-conversion) =
  refl

decode-production-synthesis-structure-v2 :
  {global-count count : Nat} ->
  ProductionSynthesisStructureV2 global-count count ->
  PTm global-count count
decode-production-synthesis-structure-v2
  (code-sort-v2 level) =
  psort level
decode-production-synthesis-structure-v2
  code-unit-type-v2 =
  punit-type
decode-production-synthesis-structure-v2
  code-unit-v2 =
  punit
decode-production-synthesis-structure-v2
  (code-variable-lookup-v2 x) =
  pvar x
decode-production-synthesis-structure-v2
  (code-global-lookup-v2 slot) =
  pglobal slot
decode-production-synthesis-structure-v2
  (code-pi-formation-v2 parameter body) =
  ppi
    (decode-production-synthesis-structure-v2 parameter)
    (decode-production-synthesis-structure-v2 body)
decode-production-synthesis-structure-v2
  (code-lambda-introduction-v2 parameter body) =
  plam
    (decode-production-synthesis-structure-v2 parameter)
    (decode-production-synthesis-structure-v2 body)
decode-production-synthesis-structure-v2
  (code-application-elimination-v2 function argument) =
  papp
    (decode-production-synthesis-structure-v2 function)
    (decode-production-synthesis-structure-v2 argument)

encode-production-synthesis-structure-v2 :
  {global-count count : Nat} ->
  PTm global-count count ->
  ProductionSynthesisStructureV2 global-count count
encode-production-synthesis-structure-v2 (pvar x) =
  code-variable-lookup-v2 x
encode-production-synthesis-structure-v2 (psort level) =
  code-sort-v2 level
encode-production-synthesis-structure-v2 (pglobal slot) =
  code-global-lookup-v2 slot
encode-production-synthesis-structure-v2
  (ppi parameter body) =
  code-pi-formation-v2
    (encode-production-synthesis-structure-v2 parameter)
    (encode-production-synthesis-structure-v2 body)
encode-production-synthesis-structure-v2
  (plam parameter body) =
  code-lambda-introduction-v2
    (encode-production-synthesis-structure-v2 parameter)
    (encode-production-synthesis-structure-v2 body)
encode-production-synthesis-structure-v2
  (papp function argument) =
  code-application-elimination-v2
    (encode-production-synthesis-structure-v2 function)
    (encode-production-synthesis-structure-v2 argument)
encode-production-synthesis-structure-v2 punit-type =
  code-unit-type-v2
encode-production-synthesis-structure-v2 punit =
  code-unit-v2

decode-encode-production-synthesis-structure-v2 :
  {global-count count : Nat} ->
  (term : PTm global-count count) ->
  decode-production-synthesis-structure-v2
    (encode-production-synthesis-structure-v2 term)
  ≡
  term
decode-encode-production-synthesis-structure-v2 (pvar x) = refl
decode-encode-production-synthesis-structure-v2 (psort level) =
  refl
decode-encode-production-synthesis-structure-v2 (pglobal slot) =
  refl
decode-encode-production-synthesis-structure-v2
  (ppi parameter body) =
  cong₂ ppi
    (decode-encode-production-synthesis-structure-v2 parameter)
    (decode-encode-production-synthesis-structure-v2 body)
decode-encode-production-synthesis-structure-v2
  (plam parameter body) =
  cong₂ plam
    (decode-encode-production-synthesis-structure-v2 parameter)
    (decode-encode-production-synthesis-structure-v2 body)
decode-encode-production-synthesis-structure-v2
  (papp function argument) =
  cong₂ papp
    (decode-encode-production-synthesis-structure-v2 function)
    (decode-encode-production-synthesis-structure-v2 argument)
decode-encode-production-synthesis-structure-v2 punit-type =
  refl
decode-encode-production-synthesis-structure-v2 punit =
  refl

encode-decode-production-synthesis-structure-v2 :
  {global-count count : Nat} ->
  (code :
    ProductionSynthesisStructureV2 global-count count) ->
  encode-production-synthesis-structure-v2
    (decode-production-synthesis-structure-v2 code)
  ≡
  code
encode-decode-production-synthesis-structure-v2
  (code-sort-v2 level) =
  refl
encode-decode-production-synthesis-structure-v2
  code-unit-type-v2 =
  refl
encode-decode-production-synthesis-structure-v2
  code-unit-v2 =
  refl
encode-decode-production-synthesis-structure-v2
  (code-variable-lookup-v2 x) =
  refl
encode-decode-production-synthesis-structure-v2
  (code-global-lookup-v2 slot) =
  refl
encode-decode-production-synthesis-structure-v2
  (code-pi-formation-v2 parameter body) =
  cong₂ code-pi-formation-v2
    (encode-decode-production-synthesis-structure-v2 parameter)
    (encode-decode-production-synthesis-structure-v2 body)
encode-decode-production-synthesis-structure-v2
  (code-lambda-introduction-v2 parameter body) =
  cong₂ code-lambda-introduction-v2
    (encode-decode-production-synthesis-structure-v2 parameter)
    (encode-decode-production-synthesis-structure-v2 body)
encode-decode-production-synthesis-structure-v2
  (code-application-elimination-v2 function argument) =
  cong₂ code-application-elimination-v2
    (encode-decode-production-synthesis-structure-v2 function)
    (encode-decode-production-synthesis-structure-v2 argument)

-- The public code accepts only Sort 0 and Sort 1. Its inferred checker output
-- may contain Sort 2.
data PublicSortLevelV2 : Nat -> Set where
  public-sort-zero-v2 :
    PublicSortLevelV2 zero
  public-sort-one-v2 :
    PublicSortLevelV2 (suc zero)

extend-context-pointwise-v2 :
  {V : Set}
  {left right : Context V}
  {parameter : Tm V} ->
  left ≗ right ->
  extend-context left parameter
  ≗
  extend-context right parameter
extend-context-pointwise-v2 pointwise bound = refl
extend-context-pointwise-v2 pointwise (free x) =
  cong weaken (pointwise x)

-- Typing is extensional in the context. This is needed because PCtx snoc
-- decoding is pointwise, rather than judgmentally, equal to extend-context.
transport-context-typing-v2 :
  {V : Set} {Σ : Signature}
  {left right : Context V}
  {term type : Tm V} ->
  left ≗ right ->
  Σ ⊢ left ∶ term ∶ type ->
  Σ ⊢ right ∶ term ∶ type
transport-context-typing-v2 pointwise
  (type-sort level) =
  type-sort level
transport-context-typing-v2 pointwise
  type-unit-type =
  type-unit-type
transport-context-typing-v2 pointwise
  type-unit =
  type-unit
transport-context-typing-v2 pointwise
  (type-variable x) =
  transport-type
    (sym (pointwise x))
    (type-variable x)
transport-context-typing-v2 pointwise
  (type-global lookup formation) =
  type-global lookup formation
transport-context-typing-v2 pointwise
  (type-pi parameter-typing body-typing) =
  type-pi
    (transport-context-typing-v2
      pointwise
      parameter-typing)
    (transport-context-typing-v2
      (extend-context-pointwise-v2 pointwise)
      body-typing)
transport-context-typing-v2 pointwise
  (type-lambda parameter-typing body-typing) =
  type-lambda
    (transport-context-typing-v2
      pointwise
      parameter-typing)
    (transport-context-typing-v2
      (extend-context-pointwise-v2 pointwise)
      body-typing)
transport-context-typing-v2 pointwise
  (type-application function-typing argument-typing) =
  type-application
    (transport-context-typing-v2
      pointwise
      function-typing)
    (transport-context-typing-v2
      pointwise
      argument-typing)

-- Syntax-directed local introduction fragment. It covers tags 0,1,2,3,5,6.
-- GlobalLookup is separate because it needs the external slot typing bridge;
-- ApplicationElimination is separate because it consumes two explicit
-- TypeFormation conversion certificates.
data LocalProductionSynthesisV2
  {global-count count : Nat}
  (Σ : Signature)
  (Γ : PCtx global-count count) :
  PTm global-count count ->
  PTm global-count count ->
  Set where

  local-sort-v2 :
    (level : Nat) ->
    PublicSortLevelV2 level ->
    LocalProductionSynthesisV2
      Σ Γ
      (psort level)
      (psort (suc level))

  local-unit-type-v2 :
    LocalProductionSynthesisV2
      Σ Γ
      punit-type
      (psort zero)

  local-unit-v2 :
    LocalProductionSynthesisV2
      Σ Γ
      punit
      punit-type

  local-variable-lookup-v2 :
    (x : Fin count) ->
    LocalProductionSynthesisV2
      Σ Γ
      (pvar x)
      (lookup-pctx Γ x)

  local-pi-formation-v2 :
    {parameter : PTm global-count count}
    {body : PTm global-count (suc count)}
    {parameter-level body-level : Nat} ->
    LocalProductionSynthesisV2
      Σ Γ
      parameter
      (psort parameter-level) ->
    LocalProductionSynthesisV2
      Σ (Γ psnoc parameter)
      body
      (psort body-level) ->
    LocalProductionSynthesisV2
      Σ Γ
      (ppi parameter body)
      (psort (parameter-level max body-level))

  local-lambda-introduction-v2 :
    {parameter : PTm global-count count}
    {body body-type : PTm global-count (suc count)}
    {parameter-level : Nat} ->
    LocalProductionSynthesisV2
      Σ Γ
      parameter
      (psort parameter-level) ->
    LocalProductionSynthesisV2
      Σ (Γ psnoc parameter)
      body
      body-type ->
    LocalProductionSynthesisV2
      Σ Γ
      (plam parameter body)
      (ppi parameter body-type)

local-production-synthesis-exact-soundness-v2 :
  {global-count count : Nat}
  {Σ : Signature}
  {Γ : PCtx global-count count}
  {term type : PTm global-count count} ->
  LocalProductionSynthesisV2 Σ Γ term type ->
  Σ ⊢ decode-pctx Γ
    ∶ decode-ptm term
    ∶ decode-ptm type
local-production-synthesis-exact-soundness-v2
  (local-sort-v2 level level-proof) =
  type-sort level
local-production-synthesis-exact-soundness-v2
  local-unit-type-v2 =
  type-unit-type
local-production-synthesis-exact-soundness-v2
  local-unit-v2 =
  type-unit
local-production-synthesis-exact-soundness-v2
  {Γ = Γ}
  (local-variable-lookup-v2 x) =
  transport-type
    (sym (decode-lookup-pctx Γ x))
    (type-variable (decode-fin x))
local-production-synthesis-exact-soundness-v2
  (local-pi-formation-v2 parameter body) =
  type-pi
    (local-production-synthesis-exact-soundness-v2
      parameter)
    (transport-context-typing-v2
      (decode-pctx-snoc _ _)
      (local-production-synthesis-exact-soundness-v2
        body))
local-production-synthesis-exact-soundness-v2
  (local-lambda-introduction-v2 parameter body) =
  type-lambda
    (local-production-synthesis-exact-soundness-v2
      parameter)
    (transport-context-typing-v2
      (decode-pctx-snoc _ _)
      (local-production-synthesis-exact-soundness-v2
        body))

local-production-synthesis-conversion-soundness-v2 :
  {global-count count : Nat}
  {Σ : Signature}
  {Γ : PCtx global-count count}
  {term type : PTm global-count count} ->
  LocalProductionSynthesisV2 Σ Γ term type ->
  Σ ⊢c decode-pctx Γ
    ∶ decode-ptm term
    ∶ decode-ptm type
local-production-synthesis-conversion-soundness-v2 derivation =
  conversion-exact
    (local-production-synthesis-exact-soundness-v2
      derivation)

local-production-synthesis-structure-v2 :
  {global-count count : Nat}
  {Σ : Signature}
  {Γ : PCtx global-count count}
  {term type : PTm global-count count} ->
  LocalProductionSynthesisV2 Σ Γ term type ->
  ProductionSynthesisStructureV2 global-count count
local-production-synthesis-structure-v2
  (local-sort-v2 level level-proof) =
  code-sort-v2 level
local-production-synthesis-structure-v2
  local-unit-type-v2 =
  code-unit-type-v2
local-production-synthesis-structure-v2
  local-unit-v2 =
  code-unit-v2
local-production-synthesis-structure-v2
  (local-variable-lookup-v2 x) =
  code-variable-lookup-v2 x
local-production-synthesis-structure-v2
  (local-pi-formation-v2 parameter body) =
  code-pi-formation-v2
    (local-production-synthesis-structure-v2 parameter)
    (local-production-synthesis-structure-v2 body)
local-production-synthesis-structure-v2
  (local-lambda-introduction-v2 parameter body) =
  code-lambda-introduction-v2
    (local-production-synthesis-structure-v2 parameter)
    (local-production-synthesis-structure-v2 body)

decode-local-production-synthesis-structure-v2 :
  {global-count count : Nat}
  {Σ : Signature}
  {Γ : PCtx global-count count}
  {term type : PTm global-count count} ->
  (derivation : LocalProductionSynthesisV2 Σ Γ term type) ->
  decode-production-synthesis-structure-v2
    (local-production-synthesis-structure-v2 derivation)
  ≡
  term
decode-local-production-synthesis-structure-v2
  (local-sort-v2 level level-proof) =
  refl
decode-local-production-synthesis-structure-v2
  local-unit-type-v2 =
  refl
decode-local-production-synthesis-structure-v2
  local-unit-v2 =
  refl
decode-local-production-synthesis-structure-v2
  (local-variable-lookup-v2 x) =
  refl
decode-local-production-synthesis-structure-v2
  (local-pi-formation-v2 parameter body) =
  cong₂ ppi
    (decode-local-production-synthesis-structure-v2 parameter)
    (decode-local-production-synthesis-structure-v2 body)
decode-local-production-synthesis-structure-v2
  (local-lambda-introduction-v2 parameter body) =
  cong₂ plam
    (decode-local-production-synthesis-structure-v2 parameter)
    (decode-local-production-synthesis-structure-v2 body)

production-synthesis-tag-inventory-complete-v2 :
  {global-count count : Nat} ->
  (code :
    ProductionSynthesisStructureV2 global-count count) ->
  SynthesisTagAtV2
    (production-synthesis-tag-v2 code)
    (synthesis-tag-number-v2
      (production-synthesis-tag-v2 code))
production-synthesis-tag-inventory-complete-v2 code =
  synthesis-tag-inventory-complete-v2
    (production-synthesis-tag-v2 code)

production-synthesis-tag-inventory-unique-v2 :
  {global-count count number : Nat}
  {code :
    ProductionSynthesisStructureV2 global-count count} ->
  SynthesisTagAtV2
    (production-synthesis-tag-v2 code)
    number ->
  number
  ≡
  synthesis-tag-number-v2
    (production-synthesis-tag-v2 code)
production-synthesis-tag-inventory-unique-v2 classification =
  synthesis-tag-inventory-unique-v2 classification

-- Every structure decoded to the finite production syntax is reifiable. This
-- is the precise "production image" statement: abstract terms containing a
-- global ordinal outside global-count are intentionally not covered.
reify-decoded-production-synthesis-image-v2 :
  {global-count count : Nat} ->
  (code :
    ProductionSynthesisStructureV2 global-count count) ->
  encode-ptm
    (decode-ptm
      (decode-production-synthesis-structure-v2 code))
  ≡
  just (decode-production-synthesis-structure-v2 code)
reify-decoded-production-synthesis-image-v2 code =
  encode-decode-ptm
    (decode-production-synthesis-structure-v2 code)

abstract-production-synthesis-round-trip-v2 :
  {global-count count : Nat} ->
  (term : PTm global-count count) ->
  decode-ptm
    (decode-production-synthesis-structure-v2
      (encode-production-synthesis-structure-v2 term))
  ≡
  decode-ptm term
abstract-production-synthesis-round-trip-v2 term =
  cong decode-ptm
    (decode-encode-production-synthesis-structure-v2 term)

-- TypeFormation conversion carries an existential universe. In particular,
-- the common formation universe may be 3 even though public certificate terms
-- are capped at Sort 2; no PTm universe field is fabricated for that internal
-- checker fact.
record TypeFormationEquivalentV2
  {V : Set}
  (Σ : Signature)
  (Γ : Context V)
  (left right : Tm V) :
  Set₁ where

  constructor type-formation-equivalent-v2

  field
    formation-universe-v2 : Nat
    formation-common-v2 : Tm V
    formation-left-trace-v2 :
      TypedSteps
        Σ Γ
        (sort formation-universe-v2)
        left
        formation-common-v2
    formation-right-trace-v2 :
      TypedSteps
        Σ Γ
        (sort formation-universe-v2)
        right
        formation-common-v2

open TypeFormationEquivalentV2 public

type-formation-equivalent-to-base-q0-v2 :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {left right : Tm V} ->
  TypeFormationEquivalentV2 Σ Γ left right ->
  BaseQ0Equivalent Σ Γ left right
type-formation-equivalent-to-base-q0-v2 equivalent =
  base-q0-equivalent
    (formation-universe-v2 equivalent)
    (formation-common-v2 equivalent)
    (formation-left-trace-v2 equivalent)
    (formation-right-trace-v2 equivalent)

-- HasType conversion is a distinct replay mode with an arbitrary expected
-- type. It must not be confused with existential TypeFormation replay.
record HasTypeEquivalentV2
  {V : Set}
  (Σ : Signature)
  (Γ : Context V)
  (expected left right : Tm V) :
  Set₁ where

  constructor has-type-equivalent-v2

  field
    has-type-common-v2 : Tm V
    has-type-left-trace-v2 :
      TypedSteps
        Σ Γ
        expected
        left
        has-type-common-v2
    has-type-right-trace-v2 :
      TypedSteps
        Σ Γ
        expected
        right
        has-type-common-v2

open HasTypeEquivalentV2 public

data ConversionReplayModeV2
  {V : Set}
  (Σ : Signature)
  (Γ : Context V)
  (left right : Tm V) :
  Set₁ where

  type-formation-replay-v2 :
    TypeFormationEquivalentV2 Σ Γ left right ->
    ConversionReplayModeV2 Σ Γ left right

  has-type-replay-v2 :
    (expected : Tm V) ->
    HasTypeEquivalentV2 Σ Γ expected left right ->
    ConversionReplayModeV2 Σ Γ left right

-- This is the minimum finite abstract global bridge needed for GlobalLookup
-- soundness. The Rust GlobalId at each declaration-order slot and the proof
-- that its verified slot table decodes to this record remain cross-language.
record FiniteGlobalSlotTypingV2
  (Σ : Signature)
  (global-count : Nat) :
  Set where

  constructor finite-global-slot-typing-v2

  field
    finite-global-entry-v2 :
      Fin global-count ->
      GlobalDeclaration
    finite-global-lookup-v2 :
      (slot : Fin global-count) ->
      lookup-global Σ (fin-ordinal slot)
      ≡
      just (finite-global-entry-v2 slot)
    finite-global-formation-v2 :
      (slot : Fin global-count) ->
      Σ ⊢ empty-context
        ∶ closed
            (declared-type
              (finite-global-entry-v2 slot))
        ∶ sort
            (declared-universe
              (finite-global-entry-v2 slot))

open FiniteGlobalSlotTypingV2 public

finite-global-lookup-conversion-soundness-v2 :
  {global-count count : Nat}
  {Σ : Signature}
  {Γ : PCtx global-count count} ->
  (bridge : FiniteGlobalSlotTypingV2 Σ global-count) ->
  (slot : Fin global-count) ->
  Σ ⊢c decode-pctx Γ
    ∶ decode-ptm (pglobal slot)
    ∶ closed
        (declared-type
          (finite-global-entry-v2 bridge slot))
finite-global-lookup-conversion-soundness-v2 bridge slot =
  conversion-exact
    (type-global
      (finite-global-lookup-v2 bridge slot)
      (finite-global-formation-v2 bridge slot))

-- Native conversion-aware application elimination. The two conversion
-- certificates align synthesized premise types with the Pi and its parameter;
-- the relation now closes those converted premises under application.
application-elimination-conversion-soundness-v2 :
  {V : Set}
  {Σ : Signature}
  {Γ : Context V}
  {function argument : Tm V}
  {function-type argument-type parameter : Tm V}
  {body : Tm (Lift V)} ->
  Σ ⊢c Γ ∶ function ∶ function-type ->
  TypeFormationEquivalentV2
    Σ Γ
    function-type
    (pi parameter body) ->
  Σ ⊢c Γ ∶ argument ∶ argument-type ->
  TypeFormationEquivalentV2
    Σ Γ
    argument-type
    parameter ->
  Σ ⊢c Γ
    ∶ app function argument
    ∶ instantiate body argument
application-elimination-conversion-soundness-v2
  function-typing
  function-conversion
  argument-typing
  argument-conversion =
  conversion-application
    (conversion-convert
      function-typing
      (type-formation-equivalent-to-base-q0-v2
        function-conversion))
    (conversion-convert
      argument-typing
      (type-formation-equivalent-to-base-q0-v2
        argument-conversion))

-- Frontier constructors are propositions naming absent correspondences, not
-- axioms and not evidence that the correspondence holds.
data ProductionSynthesisCorrespondenceFrontierV2 : Set where

  rust-global-id-slot-table-to-finite-agda-bridge-not-mechanized :
    ProductionSynthesisCorrespondenceFrontierV2

  rust-variable-metadata-and-conversion-payload-erasure-not-mechanized :
    ProductionSynthesisCorrespondenceFrontierV2

  existential-type-formation-universe-replay-not-bound-to-rust :
    ProductionSynthesisCorrespondenceFrontierV2

  full-eight-constructor-decoded-soundness-not-yet-derivable :
    ProductionSynthesisCorrespondenceFrontierV2
