{-# OPTIONS --safe --without-K #-}

module LawV2.H3Compatibility where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import Agda.Builtin.String using (String)

------------------------------------------------------------------------
-- Scope and authority
------------------------------------------------------------------------

-- ORACLE_ONLY_PROVISIONAL.
--
-- This module is not a Law-V2/GSC implementation and is not an acceptance
-- fixture.  In particular, `probeExtractDescriptor` below is a new,
-- deliberately visible oracle matcher.  The registered bootstrap does not
-- carry the structured constructor-group capability that GSC V1 requires,
-- and no semantic manifest fixes an authoritative extractor, so the literal
-- GSC descriptor disposition remains `unknown`.
--
-- The conditional experiment asks only what follows after separately
-- stipulating that the exact anonymous H3 boundary is exposed as a closed,
-- one-nullary-constructor former with judgmental computation.  The
-- stipulation is not derived from the registered boundary.

data Verdict : Set where
  proven           : Verdict
  refuted          : Verdict
  outside-fragment : Verdict
  unknown          : Verdict

literalGscDescriptorDisposition : Verdict
literalGscDescriptorDisposition = unknown

------------------------------------------------------------------------
-- The exact normalized anonymous H3 boundary
------------------------------------------------------------------------

data Slot : Set where
  g1 g2 g3 : Slot

data RawTerm : Set where
  sort     : Nat → RawTerm
  global   : Slot → RawTerm
  unitType : RawTerm
  unit     : RawTerm

record BoundaryDeclaration : Set where
  constructor declaration
  field
    identifier : Slot
    type        : RawTerm
    body        : RawTerm

h3Boundary : List BoundaryDeclaration
h3Boundary =
  declaration g1 (sort (suc zero)) (sort zero) ∷
  declaration g2 (sort zero) unitType ∷
  declaration g3 unitType unit ∷
  []

------------------------------------------------------------------------
-- Probe-local stipulated descriptor
------------------------------------------------------------------------

data ContextCode : Set where
  empty-context : ContextCode

data ConstructorCode : Set where
  one-nullary-constructor : ConstructorCode

data CompleteConstructorCode : ConstructorCode → Set where
  complete-one-nullary :
    CompleteConstructorCode one-nullary-constructor

data RecursivePositionMask : Set where
  no-recursive-positions : RecursivePositionMask

data ComputationMode : Set where
  judgmental : ComputationMode

record ClosedFormerFrame : Set where
  constructor closed-former-frame
  field
    owner                            : Slot
    parameter-context               : ContextCode
    introduction                    : Slot
    constructor-code                : ConstructorCode
    complete-constructor-certificate :
      CompleteConstructorCode constructor-code
    recursive-position-masks        : List RecursivePositionMask
    computation-mode                : ComputationMode
    source-declarations             : List Slot

-- Every field below is a probe assumption.  In particular,
-- `complete-one-nullary` certifies only the stipulated constructor code; it
-- does not prove that the flat H3 declarations form a complete constructor
-- group.  Likewise, `judgmental` is stipulated rather than extracted.
stipulatedH3Descriptor : ClosedFormerFrame
stipulatedH3Descriptor =
  closed-former-frame
    g2
    empty-context
    g3
    one-nullary-constructor
    complete-one-nullary
    (no-recursive-positions ∷ [])
    judgmental
    (g2 ∷ g3 ∷ [])

data DescriptorExtraction : Set where
  emitted             : ClosedFormerFrame → DescriptorExtraction
  unsupported-boundary : DescriptorExtraction

-- This matcher makes the additional probe stipulation inspectable.  Pattern
-- matching on the flat bytes does not turn the stipulated completeness or
-- computation mode into Law facts.  Moving such a matcher into production
-- would require a separately adopted descriptor grammar and an event-time
-- export-index capability.
probeExtractDescriptor :
  List BoundaryDeclaration → DescriptorExtraction
probeExtractDescriptor
  (declaration g1 (sort (suc zero)) (sort zero) ∷
   declaration g2 (sort zero) unitType ∷
   declaration g3 unitType unit ∷
   []) =
  emitted stipulatedH3Descriptor
probeExtractDescriptor _ = unsupported-boundary

probeMatcherReturnsStipulatedDescriptor :
  probeExtractDescriptor h3Boundary ≡ emitted stipulatedH3Descriptor
probeMatcherReturnsStipulatedDescriptor = refl

------------------------------------------------------------------------
-- Probe-local one-nullary dependent-core projection
------------------------------------------------------------------------

-- De Bruijn term codes mirror the current dependent-core projection.  In a
-- context [P , m], variable zero is m and variable one is P.  Under the
-- output Pi binder z, P is therefore variable two and z is variable zero.

data TermCode : Set where
  universe-code : TermCode
  global-code   : Slot → TermCode
  variable-code : Nat → TermCode
  pi-code       : TermCode → TermCode → TermCode
  apply-code    : TermCode → TermCode → TermCode

data PremiseRefCode : Set where

data EquationOutputCode : Set where

data OutputRole : Set where
  use-port : Slot → OutputRole

record TermOutputCode : Set where
  constructor term-output
  field
    port-name : String
    type-code : TermCode
    role      : OutputRole

record OneNullaryUseProjection : Set where
  constructor one-nullary-use-projection
  field
    parameter-context-codes       : List TermCode
    premise-refs                  : List PremiseRefCode
    term-output-ports             : List TermOutputCode
    equation-output-ports         : List EquationOutputCode

-- This is only the dependent-core use-interface projection for the
-- one-nullary probe grammar.  It is not the total GSC `compile_use` function
-- and carries no manifest-defined family identity, verification ledger, or
-- support encoding.
projectOneNullaryUse : ClosedFormerFrame → OneNullaryUseProjection
projectOneNullaryUse frame =
  one-nullary-use-projection
    -- P : Π (z : owner). g1
    -- m : P introduction
    (pi-code
       (global-code (ClosedFormerFrame.owner frame))
       (global-code g1)
     ∷
     apply-code
       (variable-code zero)
       (global-code (ClosedFormerFrame.introduction frame))
     ∷ [])
    []
    (term-output
       "unit-use"
       -- u : Π (z : owner). P z
       (pi-code
         (global-code (ClosedFormerFrame.owner frame))
         (apply-code
           (variable-code (suc (suc zero)))
           (variable-code zero)))
       (use-port (ClosedFormerFrame.owner frame))
     ∷ [])
    []

h3UseProjection : OneNullaryUseProjection
h3UseProjection = projectOneNullaryUse stipulatedH3Descriptor

h3UseProjectionComputed :
  h3UseProjection ≡
  one-nullary-use-projection
    (pi-code (global-code g2) (global-code g1) ∷
     apply-code (variable-code zero) (global-code g3) ∷ [])
    []
    (term-output
       "unit-use"
       (pi-code
         (global-code g2)
         (apply-code
           (variable-code (suc (suc zero)))
           (variable-code zero)))
       (use-port g2)
     ∷ [])
    []
h3UseProjectionComputed = refl

-- Literal GSC V1 separates this use family from the rank-two computation
-- family.  Thus the one-nullary use projection has no equation output port.
-- Conditional on a generated use source u, a future adopted
-- `compile_compute` would ask for:
--
--     u g3  ↦  m : P g3
--
-- The production exact-extension equation grammar needed to state and check
-- that clause does not yet exist.

------------------------------------------------------------------------
-- An analogous host-Agda one-constructor shape model
------------------------------------------------------------------------

data One : Set where
  star : One

record H3Parameters : Set₁ where
  constructor h3-parameters
  field
    motive : One → Set
    method : motive star

H3Output : H3Parameters → Set
H3Output parameters =
  (z : One) → H3Parameters.motive parameters z

unitInd :
  (P : One → Set) →
  P star →
  (z : One) →
  P z
unitInd P method star = method

unitIndBeta :
  (P : One → Set) →
  (method : P star) →
  unitInd P method star ≡ method
unitIndBeta P method = refl

shapeModelUseFill : (parameters : H3Parameters) → H3Output parameters
shapeModelUseFill (h3-parameters P method) = unitInd P method

shapeModelBeta :
  (P : One → Set) →
  (method : P star) →
  unitInd P method star ≡ method
shapeModelBeta = unitIndBeta

-- These proofs use Agda's host datatype eliminator.  This module defines no
-- interpretation or erasure theorem relating `One` and `star` to raw slots
-- `g2` and `g3`, and therefore proves no fill or discharge theorem for
-- `h3UseProjection`.  It establishes only the analogous one-constructor
-- shape model.  Current GF2 candidate syntax and its exact-extension checker
-- may not import this host capability.

------------------------------------------------------------------------
-- Exact archived Stage-4 presentations
------------------------------------------------------------------------

data LegacyExpr : Set where
  legacy-var   : Nat → LegacyExpr
  legacy-app   : LegacyExpr → LegacyExpr → LegacyExpr
  legacy-lam   : LegacyExpr → LegacyExpr
  legacy-pi    : LegacyExpr → LegacyExpr → LegacyExpr
  legacy-sigma : LegacyExpr → LegacyExpr → LegacyExpr

data LegacyRole : Set where
  introduction-role elimination-role : LegacyRole

record LegacyClause : Set where
  constructor legacy-clause
  field
    legacy-role : LegacyRole
    legacy-expr : LegacyExpr

record ArchivedPresentation : Set where
  constructor archived-presentation
  field
    candidate-hash : String
    canonical-key : String
    clauses       : List LegacyClause

v1 : LegacyExpr
v1 = legacy-var (suc zero)

v2 : LegacyExpr
v2 = legacy-var (suc (suc zero))

v3 : LegacyExpr
v3 = legacy-var (suc (suc (suc zero)))

formerPi formerSigma application23 application32 betaSkeleton : LegacyExpr
formerPi = legacy-lam (legacy-pi v1 v2)
formerSigma = legacy-lam (legacy-sigma v1 v2)
application23 = legacy-app (legacy-app v1 v2) v3
application32 = legacy-app (legacy-app v1 v3) v2
betaSkeleton = legacy-app (legacy-lam v1) v2

presentation :
  String → String → LegacyExpr → LegacyExpr → ArchivedPresentation
presentation hash key former application =
  archived-presentation
    hash
    key
    (legacy-clause introduction-role former ∷
     legacy-clause introduction-role application ∷
     legacy-clause elimination-role betaSkeleton ∷ [])

archived201672 archived43a0ed archived4b2211 archivedb4f821 :
  ArchivedPresentation
archived201672 =
  presentation
    "blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407"
    "0e9aa132"
    formerPi
    application23
archived43a0ed =
  presentation
    "blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308"
    "4ede818c"
    formerPi
    application32
archived4b2211 =
  presentation
    "blake3:4b2211ecae25f3afbb1187f3a4a1b644edfc6adbd3512b9ca27c64a7b731265b"
    "d860242f"
    formerSigma
    application32
archivedb4f821 =
  presentation
    "blake3:b4f821d9bb28366d8ae37adc7c1cb3e28c8a0de60f05c81e9ad75ba4f8b0edd4"
    "b516ff09"
    formerSigma
    application23

------------------------------------------------------------------------
-- Probe result matrix
------------------------------------------------------------------------

data RawClauseShape : Set where
  no-candidate-shape          : RawClauseShape
  legacy-three-clause-shape   : RawClauseShape
  one-head-plus-one-beta      : RawClauseShape

record MatrixRow : Set where
  constructor matrix-row
  field
    response-name                 : String
    historical-grammar-admission : Verdict
    gf2-grammar-admission         : Verdict
    typed-head-formation          : Verdict
    use-port-fill                 : Verdict
    beta-port-fill                : Verdict
    full-downward-dag-discharge   : Verdict
    demand-connectedness          : Verdict
    quotient-equivalence-to-direct : Verdict
    raw-public-clause-shape       : RawClauseShape

archivedRow : String → MatrixRow
archivedRow name =
  matrix-row
    name
    proven
    unknown
    unknown
    unknown
    unknown
    unknown
    unknown
    unknown
    legacy-three-clause-shape

libraryRow : MatrixRow
libraryRow =
  matrix-row
    "sealed-H3-library"
    outside-fragment
    unknown
    unknown
    unknown
    unknown
    unknown
    outside-fragment
    unknown
    no-candidate-shape

directRow : MatrixRow
directRow =
  matrix-row
    "direct-unit-eliminator"
    outside-fragment
    unknown
    proven
    proven
    unknown
    unknown
    unknown
    unknown
    one-head-plus-one-beta

compatibilityMatrix : List MatrixRow
compatibilityMatrix =
  libraryRow ∷
  archivedRow "2016726758f3" ∷
  archivedRow "43a0ed707770" ∷
  archivedRow "4b2211ecae25" ∷
  archivedRow "b4f821d9bb28" ∷
  directRow ∷
  []

-- Result:
--
-- * The literal descriptor/compiler theorem is `unknown` before any GSC
--   compiler can run.
-- * Under the stipulated probe descriptor, the one-nullary dependent-core
--   projection has the shape of dependent unit elimination.
-- * Host Agda constructs only an analogous shape-model eliminator and beta
--   proof; no bridge to the registered H3 slots is claimed.
-- * No bridge turns any archived shallow telescope into a typed GF2 response.
-- * No current GF2 equation-extension or quotient checker can decide the
--   remaining beta, full-discharge, connectedness, or equivalence cells.
