module AbstractionBarrier where

open import Agda.Primitive using (Set)

open import BridgePayload using (BridgePayload; ContractWitness)

record PublicInterface : Set1 where
  constructor mkPublicInterface
  field
    schema : Set

record SealedLayer : Set1 where
  constructor mkSealedLayer
  field
    hiddenDerivation : Set
    publicInterface : PublicInterface

record OpaqueUse (layer : SealedLayer) : Set1 where
  constructor mkOpaqueUse
  field
    consumeSchema : PublicInterface.schema (SealedLayer.publicInterface layer) -> Set

record AbstractionBarrier (payload : BridgePayload) (layer : SealedLayer) : Set1 where
  constructor mkAbstractionBarrier
  field
    payloadWitness : ContractWitness payload
    futureUse : OpaqueUse layer
