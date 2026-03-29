module StepWitness where

open import Agda.Primitive using (Set)

open import AbstractionBarrier using (AbstractionBarrier; PublicInterface; SealedLayer)
open import BridgePayload using (BridgePayload)

record StepWitness (payload : BridgePayload) (carrier : Set) : Set1 where
  constructor mkStepWitness
  field
    sealedLayer : SealedLayer
    barrier : AbstractionBarrier payload sealedLayer
    exportCarrier :
      carrier -> PublicInterface.schema (SealedLayer.publicInterface sealedLayer)
