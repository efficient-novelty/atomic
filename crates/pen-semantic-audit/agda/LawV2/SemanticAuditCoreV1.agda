{-# OPTIONS --safe --without-K #-}

module LawV2.SemanticAuditCoreV1 where

open import Agda.Builtin.Bool using (Bool; true; false)
open import Agda.Builtin.List using (List; []) renaming (_∷_ to _::_)
open import Agda.Builtin.Maybe using (Maybe; just; nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import Agda.Builtin.String using (String)

infix 4 _===_

data _===_ {A : Set} (x : A) : A -> Set where
  same : x === x

data Fin : Nat -> Set where
  fzero : {count : Nat} -> Fin (suc count)
  fsuc : {count : Nat} -> Fin count -> Fin (suc count)

manifest-token : String
manifest-token =
  "gf2-semantic-audit-core-v1/proposed-not-adopted/rank=2/levels=0,1/q3=verified-empty"

-- This is an abstract, finite expected-outcome reference model. Its proofs
-- establish facts only about the datatypes and functions defined in this
-- module. Checking it does not prove equivalence with the Rust
-- implementation, a full dependent Q0 normalization theorem, the generic
-- free-completion theorem, adoption, or any live Profile A result. A separate
-- cross-implementation replay and independent review must establish any such
-- agreement.
reference-model-scope : String
reference-model-scope =
  "abstract-safe-agda-reference-v1/not-rust-equivalence/not-full-q0/not-free-completion/proposed-not-adopted"

data SeedKind : Set where
  public-head : SeedKind
  public-equation : SeedKind
  universal-interface-reserved : SeedKind

data Rule : Set where
  seed-rule : Rule
  generic-application : Rule
  generic-equation-action : Rule

data Rank : Set where
  rank-zero : Rank
  rank-one : Rank
  rank-two : Rank

record SeedCode (seed-count : Nat) : Set where
  constructor seed-code
  field
    ordinal : Fin seed-count
    kind : SeedKind

record DerivationCode (source-count : Nat) : Set where
  constructor derivation-code
  field
    rule : Rule
    left : Fin source-count
    right : Maybe (Fin source-count)
    rank : Rank

data BoundedRank : Rank -> Set where
  bounded-zero : BoundedRank rank-zero
  bounded-one : BoundedRank rank-one
  bounded-two : BoundedRank rank-two

all-ranks-bounded : (rank : Rank) -> BoundedRank rank
all-ranks-bounded rank-zero = bounded-zero
all-ranks-bounded rank-one = bounded-one
all-ranks-bounded rank-two = bounded-two

-- A derivation code contains only a finite rule tag, finite source indices,
-- one optional finite source index, and a rank from the closed rank type.
-- There is no recursive arbitrary term/tree constructor in this carrier.
derivation-rank-is-bounded :
  {source-count : Nat} ->
  (derivation : DerivationCode source-count) ->
  BoundedRank (DerivationCode.rank derivation)
derivation-rank-is-bounded (derivation-code rule left right rank) =
  all-ranks-bounded rank

data LocalRole : Set where
  kernel-head : LocalRole
  adjoint-mate : LocalRole
  support-action : LocalRole
  coherence : LocalRole

role-for : SeedKind -> Maybe LocalRole
role-for public-head = just kernel-head
role-for public-equation = just coherence
role-for universal-interface-reserved = nothing

universal-interface-fails-closed :
  role-for universal-interface-reserved === nothing
universal-interface-fails-closed = same

record FamilyId : Set where
  constructor family-id
  field
    manifest-code : Nat
    source-code : Nat
    telescope-code : Nat
    conclusion-code : Nat
    role : LocalRole

record Instance (family : FamilyId) : Set where
  constructor family-instance
  field
    substitution-code : Nat

origin : {family : FamilyId} -> Instance family -> FamilyId
origin {family} (family-instance substitution-code) = family

specialization-preserves-family :
  {family : FamilyId} ->
  (value : Instance family) ->
  origin value === family
specialization-preserves-family (family-instance substitution-code) = same

data Presentation : Set where
  binder-source : Presentation
  binder-renamed : Presentation
  telescope-curried : Presentation
  telescope-uncurried : Presentation

presentation-code : Presentation -> Nat
presentation-code binder-source = zero
presentation-code binder-renamed = zero
presentation-code telescope-curried = suc zero
presentation-code telescope-uncurried = suc zero

binder-presentations-agree :
  presentation-code binder-source === presentation-code binder-renamed
binder-presentations-agree = same

telescope-presentations-agree :
  presentation-code telescope-curried === presentation-code telescope-uncurried
telescope-presentations-agree = same

-- Q0 below is a deliberately finite abstract normalization model. The
-- soundness/completeness statements are relative to `Q0Equivalent`, which is
-- itself defined by this model's normal forms; they are not a theorem about
-- the Rust kernel or the full dependent source language.
data Q0Term : Set where
  atom : Nat -> Q0Term
  beta-unit : Q0Term
  delta-public-unit : Q0Term
  forced-projection-unit : Q0Term
  fresh-computation-unit : Q0Term

data Q0Normal : Set where
  normal-atom : Nat -> Q0Normal
  normal-unit : Q0Normal

normalize : Q0Term -> Q0Normal
normalize (atom code) = normal-atom code
normalize beta-unit = normal-unit
normalize delta-public-unit = normal-unit
normalize forced-projection-unit = normal-unit
normalize fresh-computation-unit = normal-unit

data Q0Equivalent (left right : Q0Term) : Set where
  same-normal :
    normalize left === normalize right ->
    Q0Equivalent left right

q0-sound :
  {left right : Q0Term} ->
  Q0Equivalent left right ->
  normalize left === normalize right
q0-sound (same-normal proof) = proof

q0-complete :
  {left right : Q0Term} ->
  normalize left === normalize right ->
  Q0Equivalent left right
q0-complete proof = same-normal proof

data Q3Edge : Set where

q3-empty : {A : Set} -> Q3Edge -> A
q3-empty ()

data ScopeFeature : Set where
  core-public-head : ScopeFeature
  core-public-equation : ScopeFeature
  path-feature : ScopeFeature
  univalence-feature : ScopeFeature
  contextual-adjoint-feature : ScopeFeature

data ScopeDecision : Set where
  supported : ScopeDecision
  outside-fragment : ScopeDecision

scope-decision : ScopeFeature -> ScopeDecision
scope-decision core-public-head = supported
scope-decision core-public-equation = supported
scope-decision path-feature = outside-fragment
scope-decision univalence-feature = outside-fragment
scope-decision contextual-adjoint-feature = outside-fragment

unsupported-path-vector :
  scope-decision path-feature === outside-fragment
unsupported-path-vector = same

unsupported-univalence-vector :
  scope-decision univalence-feature === outside-fragment
unsupported-univalence-vector = same

data CostDisposition : Set where
  first-irreducible : CostDisposition
  transparent-alias : CostDisposition
  forced-definitional-completion : CostDisposition
  forced-projection : CostDisposition
  duplicate-presentation : CostDisposition
  unknown : CostDisposition

-- These are the only completion-decision inputs admitted by this abstract
-- reference model. A positive free-completion result requires an opaque
-- theorem capability checked by the production verifier. No such capability
-- has been accepted, so this freely constructible Agda datatype deliberately
-- has no positive constructor.
data CompletionEvidence : Set where
  separating-nonreconstruction : CompletionEvidence
  missing-completion-decision : CompletionEvidence

equation-disposition : CompletionEvidence -> CostDisposition
equation-disposition separating-nonreconstruction =
  first-irreducible
equation-disposition missing-completion-decision =
  unknown

ambient-first-export-vector : CostDisposition
ambient-first-export-vector = first-irreducible

prior-public-alias-vector : CostDisposition
prior-public-alias-vector = transparent-alias

record BodyfulDefinitionResult : Set where
  constructor bodyful-result
  field
    head-disposition : CostDisposition
    ordinary-beta-clause : CostDisposition

bodyful-definition-vector : BodyfulDefinitionResult
bodyful-definition-vector =
  bodyful-result first-irreducible forced-definitional-completion

generated-equation-uncertified-vector : CostDisposition
generated-equation-uncertified-vector =
  equation-disposition missing-completion-decision

generated-equation-separating-vector : CostDisposition
generated-equation-separating-vector =
  equation-disposition separating-nonreconstruction

generated-equation-undecided-vector : CostDisposition
generated-equation-undecided-vector =
  equation-disposition missing-completion-decision

forced-projection-vector : CostDisposition
forced-projection-vector = forced-projection

duplicate-field-vector : CostDisposition
duplicate-field-vector = duplicate-presentation

cost-vectors :
  List CostDisposition
cost-vectors =
  ambient-first-export-vector ::
  prior-public-alias-vector ::
  BodyfulDefinitionResult.ordinary-beta-clause bodyful-definition-vector ::
  generated-equation-uncertified-vector ::
  generated-equation-separating-vector ::
  generated-equation-undecided-vector ::
  forced-projection-vector ::
  duplicate-field-vector ::
  []

cost-vector-result :
  cost-vectors
  ===
  (first-irreducible ::
   transparent-alias ::
   forced-definitional-completion ::
   unknown ::
   first-irreducible ::
   unknown ::
   forced-projection ::
   duplicate-presentation ::
   [])
cost-vector-result = same

-- Ordered expected outcomes for all ten proposed generic cost vectors. This
-- table makes the safe Agda artifact cover the full vector inventory without
-- claiming that this abstract table proves the Rust cost implementation.
data CostVectorOutcome : Set where
  one-disposition : CostDisposition -> CostVectorOutcome
  head-and-equation :
    CostDisposition -> CostDisposition -> CostVectorOutcome
  bodyful-head-and-beta :
    CostDisposition -> CostDisposition -> CostVectorOutcome
  two-irreducible-basis : CostVectorOutcome
  independent-order-invariant : CostVectorOutcome
  nonunique-basis-unknown : CostVectorOutcome

data CostReferenceVector : Set where
  cost-v01-ambient-export : CostReferenceVector
  cost-v02-prior-public-alias : CostReferenceVector
  cost-v03-bodyful-beta : CostReferenceVector
  cost-v04-uncertified-fresh-equation : CostReferenceVector
  cost-v05-separated-fresh-equation : CostReferenceVector
  cost-v06-descriptor-projection : CostReferenceVector
  cost-v07-duplicate-field : CostReferenceVector
  cost-v08-mutual-irreducibles : CostReferenceVector
  cost-v09-independent-order : CostReferenceVector
  cost-v10-nonunique-basis : CostReferenceVector

evaluate-cost-reference : CostReferenceVector -> CostVectorOutcome
evaluate-cost-reference cost-v01-ambient-export =
  one-disposition ambient-first-export-vector
evaluate-cost-reference cost-v02-prior-public-alias =
  one-disposition prior-public-alias-vector
evaluate-cost-reference cost-v03-bodyful-beta =
  bodyful-head-and-beta
    (BodyfulDefinitionResult.head-disposition bodyful-definition-vector)
    (BodyfulDefinitionResult.ordinary-beta-clause bodyful-definition-vector)
evaluate-cost-reference cost-v04-uncertified-fresh-equation =
  head-and-equation first-irreducible generated-equation-uncertified-vector
evaluate-cost-reference cost-v05-separated-fresh-equation =
  head-and-equation first-irreducible generated-equation-separating-vector
evaluate-cost-reference cost-v06-descriptor-projection =
  one-disposition forced-projection-vector
evaluate-cost-reference cost-v07-duplicate-field =
  one-disposition duplicate-field-vector
evaluate-cost-reference cost-v08-mutual-irreducibles =
  two-irreducible-basis
evaluate-cost-reference cost-v09-independent-order =
  independent-order-invariant
evaluate-cost-reference cost-v10-nonunique-basis =
  nonunique-basis-unknown

cost-reference-vectors : List CostVectorOutcome
cost-reference-vectors =
  evaluate-cost-reference cost-v01-ambient-export ::
  evaluate-cost-reference cost-v02-prior-public-alias ::
  evaluate-cost-reference cost-v03-bodyful-beta ::
  evaluate-cost-reference cost-v04-uncertified-fresh-equation ::
  evaluate-cost-reference cost-v05-separated-fresh-equation ::
  evaluate-cost-reference cost-v06-descriptor-projection ::
  evaluate-cost-reference cost-v07-duplicate-field ::
  evaluate-cost-reference cost-v08-mutual-irreducibles ::
  evaluate-cost-reference cost-v09-independent-order ::
  evaluate-cost-reference cost-v10-nonunique-basis ::
  []

cost-reference-vector-result :
  cost-reference-vectors
  ===
  (one-disposition first-irreducible ::
   one-disposition transparent-alias ::
   bodyful-head-and-beta first-irreducible forced-definitional-completion ::
   head-and-equation first-irreducible unknown ::
   head-and-equation first-irreducible first-irreducible ::
   one-disposition forced-projection ::
   one-disposition duplicate-presentation ::
   two-irreducible-basis ::
   independent-order-invariant ::
   nonunique-basis-unknown ::
   [])
cost-reference-vector-result = same

-- Weakening embeds every old family behind the one new-family slot.
wk : {count : Nat} -> Fin count -> Fin (suc count)
wk = fsuc

-- Restriction is defined exactly on the old-support image.
restrict-old : {count : Nat} -> Fin (suc count) -> Maybe (Fin count)
restrict-old fzero = nothing
restrict-old (fsuc family) = just family

restriction-retraction :
  {count : Nat} ->
  (family : Fin count) ->
  restrict-old (wk family) === just family
restriction-retraction family = same

new-family-is-outside-image :
  {count : Nat} ->
  restrict-old {count} fzero === nothing
new-family-is-outside-image = same

data Anchor : Set where
  clause-role : Nat -> LocalRole -> Anchor
  demand-output : Nat -> Nat -> Anchor
  unanchored : Anchor

data InjectionDisposition : Set where
  injected : InjectionDisposition
  collision : InjectionDisposition
  missing-anchor : InjectionDisposition

nat-equal : Nat -> Nat -> Bool
nat-equal zero zero = true
nat-equal zero (suc right) = false
nat-equal (suc left) zero = false
nat-equal (suc left) (suc right) = nat-equal left right

role-equal : LocalRole -> LocalRole -> Bool
role-equal kernel-head kernel-head = true
role-equal adjoint-mate adjoint-mate = true
role-equal support-action support-action = true
role-equal coherence coherence = true
role-equal left right = false

anchor-equal : Anchor -> Anchor -> Bool
anchor-equal (clause-role left-clause left-role)
             (clause-role right-clause right-role) with
  nat-equal left-clause right-clause
... | false = false
... | true = role-equal left-role right-role
anchor-equal (demand-output left-orbit left-output)
             (demand-output right-orbit right-output) with
  nat-equal left-orbit right-orbit
... | false = false
... | true = nat-equal left-output right-output
anchor-equal unanchored unanchored = true
anchor-equal left right = false

contains-anchor : Anchor -> List Anchor -> Bool
contains-anchor target [] = false
contains-anchor target (value :: rest) with anchor-equal target value
... | true = true
... | false = contains-anchor target rest

check-injection-from : List Anchor -> List Anchor -> InjectionDisposition
check-injection-from seen [] = injected
check-injection-from seen (unanchored :: rest) = missing-anchor
check-injection-from seen (value :: rest) with contains-anchor value seen
... | true = collision
... | false = check-injection-from (value :: seen) rest

check-injection : List Anchor -> InjectionDisposition
check-injection = check-injection-from []

two-roles-one-clause :
  check-injection
    (clause-role zero kernel-head ::
     clause-role zero support-action ::
     [])
  === injected
two-roles-one-clause = same

deliberate-collision :
  check-injection
    (clause-role zero kernel-head ::
     clause-role zero kernel-head ::
     [])
  === collision
deliberate-collision = same

missing-anchor-fails-closed :
  check-injection (unanchored :: []) === missing-anchor
missing-anchor-fails-closed = same

-- Ordered expected outcomes for the thirteen semantic vectors. Vector 13 is
-- one prescribed pair of unsupported inputs (path and univalence). As above,
-- this inventory is an abstract reference result, not a Rust-equivalence
-- theorem.
data SemanticVectorOutcome : Set where
  supported-vector : SemanticVectorOutcome
  one-originating-family : SemanticVectorOutcome
  presentation-invariant : SemanticVectorOutcome
  weakening-retraction-proven : SemanticVectorOutcome
  two-distinct-roles : SemanticVectorOutcome
  provenance-collision-fails-closed : SemanticVectorOutcome
  q3-empty-proven : SemanticVectorOutcome
  path-and-univalence-outside : SemanticVectorOutcome

data SemanticReferenceVector : Set where
  semantic-v01-ambient-export : SemanticReferenceVector
  semantic-v02-prior-public-alias : SemanticReferenceVector
  semantic-v03-bodyful-beta : SemanticReferenceVector
  semantic-v04-fresh-equation : SemanticReferenceVector
  semantic-v05-descriptor-projection : SemanticReferenceVector
  semantic-v06-duplicate-field : SemanticReferenceVector
  semantic-v07-specializations : SemanticReferenceVector
  semantic-v08-presentations : SemanticReferenceVector
  semantic-v09-weakening : SemanticReferenceVector
  semantic-v10-two-roles : SemanticReferenceVector
  semantic-v11-provenance-collision : SemanticReferenceVector
  semantic-v12-empty-q3 : SemanticReferenceVector
  semantic-v13-unsupported-pair : SemanticReferenceVector

evaluate-semantic-reference :
  SemanticReferenceVector -> SemanticVectorOutcome
evaluate-semantic-reference semantic-v01-ambient-export = supported-vector
evaluate-semantic-reference semantic-v02-prior-public-alias = supported-vector
evaluate-semantic-reference semantic-v03-bodyful-beta = supported-vector
evaluate-semantic-reference semantic-v04-fresh-equation = supported-vector
evaluate-semantic-reference semantic-v05-descriptor-projection = supported-vector
evaluate-semantic-reference semantic-v06-duplicate-field = supported-vector
evaluate-semantic-reference semantic-v07-specializations =
  one-originating-family
evaluate-semantic-reference semantic-v08-presentations =
  presentation-invariant
evaluate-semantic-reference semantic-v09-weakening =
  weakening-retraction-proven
evaluate-semantic-reference semantic-v10-two-roles = two-distinct-roles
evaluate-semantic-reference semantic-v11-provenance-collision =
  provenance-collision-fails-closed
evaluate-semantic-reference semantic-v12-empty-q3 = q3-empty-proven
evaluate-semantic-reference semantic-v13-unsupported-pair =
  path-and-univalence-outside

semantic-reference-vectors : List SemanticVectorOutcome
semantic-reference-vectors =
  evaluate-semantic-reference semantic-v01-ambient-export ::
  evaluate-semantic-reference semantic-v02-prior-public-alias ::
  evaluate-semantic-reference semantic-v03-bodyful-beta ::
  evaluate-semantic-reference semantic-v04-fresh-equation ::
  evaluate-semantic-reference semantic-v05-descriptor-projection ::
  evaluate-semantic-reference semantic-v06-duplicate-field ::
  evaluate-semantic-reference semantic-v07-specializations ::
  evaluate-semantic-reference semantic-v08-presentations ::
  evaluate-semantic-reference semantic-v09-weakening ::
  evaluate-semantic-reference semantic-v10-two-roles ::
  evaluate-semantic-reference semantic-v11-provenance-collision ::
  evaluate-semantic-reference semantic-v12-empty-q3 ::
  evaluate-semantic-reference semantic-v13-unsupported-pair ::
  []

semantic-reference-vector-result :
  semantic-reference-vectors
  ===
  (supported-vector ::
   supported-vector ::
   supported-vector ::
   supported-vector ::
   supported-vector ::
   supported-vector ::
   one-originating-family ::
   presentation-invariant ::
   weakening-retraction-proven ::
   two-distinct-roles ::
   provenance-collision-fails-closed ::
   q3-empty-proven ::
   path-and-univalence-outside ::
   [])
semantic-reference-vector-result = same
