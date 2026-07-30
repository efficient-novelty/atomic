{-# OPTIONS --safe --without-K #-}

module LawV2.Wire.Decoder where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe using (Maybe; just; nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.Wire.Bytes

Decoder : Set -> Set
Decoder A = List Byte -> Maybe (A × List Byte)

pure : {A : Set} -> A -> Decoder A
pure value input = just (value , input)

fail : {A : Set} -> Decoder A
fail input = nothing

_>>=_ :
  {A B : Set} ->
  Decoder A ->
  (A -> Decoder B) ->
  Decoder B
(parser >>= continuation) input
  with parser input
... | nothing = nothing
... | just (value , rest) =
  continuation value rest

infixl 1 _>>=_

parse-byte : Decoder Byte
parse-byte [] = nothing
parse-byte (value ∷ rest) = just (value , rest)

expect-byte : Byte -> Decoder Byte
expect-byte expected input
  with parse-byte input
... | nothing = nothing
... | just (actual , rest)
  with byte-equal expected actual
... | false = nothing
... | true = just (actual , rest)

parse-u16 : Decoder U16
parse-u16 =
  parse-byte >>= λ b0 ->
  parse-byte >>= λ b1 ->
  pure (little16 b0 b1)

parse-u32 : Decoder U32
parse-u32 =
  parse-byte >>= λ b0 ->
  parse-byte >>= λ b1 ->
  parse-byte >>= λ b2 ->
  parse-byte >>= λ b3 ->
  pure (little32 b0 b1 b2 b3)

parse-u64 : Decoder U64
parse-u64 =
  parse-byte >>= λ b0 ->
  parse-byte >>= λ b1 ->
  parse-byte >>= λ b2 ->
  parse-byte >>= λ b3 ->
  parse-byte >>= λ b4 ->
  parse-byte >>= λ b5 ->
  parse-byte >>= λ b6 ->
  parse-byte >>= λ b7 ->
  pure (little64 b0 b1 b2 b3 b4 b5 b6 b7)

parse-u16-encode :
  (value : U16) ->
  (rest : List Byte) ->
  parse-u16 (encode-u16 value ++ rest)
  ≡
  just (value , rest)
parse-u16-encode (little16 b0 b1) rest = refl

parse-u32-encode :
  (value : U32) ->
  (rest : List Byte) ->
  parse-u32 (encode-u32 value ++ rest)
  ≡
  just (value , rest)
parse-u32-encode (little32 b0 b1 b2 b3) rest = refl

parse-u64-encode :
  (value : U64) ->
  (rest : List Byte) ->
  parse-u64 (encode-u64 value ++ rest)
  ≡
  just (value , rest)
parse-u64-encode
  (little64 b0 b1 b2 b3 b4 b5 b6 b7)
  rest = refl

parse-sized-bytes :
  Decoder (U64 × List Byte)
parse-sized-bytes input
  with parse-u64 input
... | nothing = nothing
... | just (encoded-length , after-length)
  with take-exact (u64-value encoded-length) after-length
... | nothing = nothing
... | just (payload , rest) =
  just ((encoded-length , payload) , rest)

record CanonicalSizedBytes : Set where
  constructor canonical-sized-bytes
  field
    encoded-length : U64
    payload : List Byte
    length-is-canonical :
      u64-value encoded-length ≡ length payload

open CanonicalSizedBytes public

encode-sized-bytes :
  CanonicalSizedBytes ->
  List Byte
encode-sized-bytes value =
  encode-u64 (encoded-length value) ++ payload value

parse-sized-bytes-encode :
  (value : CanonicalSizedBytes) ->
  (rest : List Byte) ->
  parse-sized-bytes (encode-sized-bytes value ++ rest)
  ≡
  just
    ( (encoded-length value , payload value)
    , rest
    )
parse-sized-bytes-encode
  (canonical-sized-bytes encoded-length payload proof)
  rest
  rewrite proof
        | take-exact-append payload rest = refl


parse-u16-nat : Decoder Nat
parse-u16-nat =
  parse-u16 >>= λ value →
  pure (u16-value value)

parse-u32-nat : Decoder Nat
parse-u32-nat =
  parse-u32 >>= λ value →
  pure (u32-value value)

parse-u64-nat : Decoder Nat
parse-u64-nat =
  parse-u64 >>= λ value →
  pure (u64-value value)

parse-fixed-bytes : Nat → Decoder (List Byte)
parse-fixed-bytes count input = take-exact count input

parse-byte-string : Decoder (List Byte)
parse-byte-string =
  parse-sized-bytes >>= λ sized →
  pure (second sized)

parse-many : {A : Set} → Nat → Decoder A → Decoder (List A)
parse-many zero parser = pure []
parse-many (suc count) parser =
  parser >>= λ value →
  parse-many count parser >>= λ values →
  pure (value ∷ values)

parse-list : {A : Set} → Decoder A → Decoder (List A)
parse-list parser =
  parse-u64-nat >>= λ count →
  parse-many count parser

parse-option-bools :
  {A : Set} → Bool → Bool → Decoder A → List Byte →
  Maybe (Maybe A × List Byte)
parse-option-bools true second parser rest = just (nothing , rest)
parse-option-bools false false parser rest = nothing
parse-option-bools false true parser rest
  with parser rest
... | nothing = nothing
... | just (value , after) = just (just value , after)

parse-option : {A : Set} → Decoder A → Decoder (Maybe A)
parse-option parser input
  with parse-byte input
... | nothing = nothing
... | just (tag , rest) =
  parse-option-bools
    (byte-equal tag byte0)
    (byte-equal tag byte1)
    parser
    rest

parse-boolean-bools : Bool → Bool → List Byte → Maybe (Bool × List Byte)
parse-boolean-bools true second rest = just (false , rest)
parse-boolean-bools false true rest = just (true , rest)
parse-boolean-bools false false rest = nothing

parse-boolean : Decoder Bool
parse-boolean input
  with parse-byte input
... | nothing = nothing
... | just (tag , rest) =
  parse-boolean-bools
    (byte-equal tag byte0)
    (byte-equal tag byte1)
    rest
run-complete : {A : Set} → Decoder A → List Byte → Maybe A
run-complete parser input
  with parser input
... | nothing = nothing
... | just (value , []) = just value
... | just (value , _ ∷ _) = nothing