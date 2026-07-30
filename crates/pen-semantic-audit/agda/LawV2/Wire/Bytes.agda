{-# OPTIONS --safe --without-K #-}

module LawV2.Wire.Bytes where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe using (Maybe; just; nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc; _+_; _*_)

infixr 5 _++_
infixr 4 _,_

-- Intrinsic eight-bit bytes keep values above 255 out of the authoritative
-- decoder input. `bytes-from-nats` below is the sole decimal-octet adapter.
record Byte : Set where
  constructor bits
  field
    bit0 bit1 bit2 bit3 bit4 bit5 bit6 bit7 : Bool

open Byte public

record _×_ (A B : Set) : Set where
  constructor _,_
  field
    first : A
    second : B

open _×_ public

_++_ : {A : Set} -> List A -> List A -> List A
[] ++ right = right
(x ∷ left) ++ right = x ∷ (left ++ right)

length : {A : Set} -> List A -> Nat
length [] = zero
length (_ ∷ values) = suc (length values)

nat-equal : Nat -> Nat -> Bool
nat-equal zero zero = true
nat-equal zero (suc right) = false
nat-equal (suc left) zero = false
nat-equal (suc left) (suc right) = nat-equal left right

bool-equal : Bool -> Bool -> Bool
bool-equal false false = true
bool-equal false true = false
bool-equal true false = false
bool-equal true true = true

_and_ : Bool -> Bool -> Bool
true and right = right
false and right = false

byte-equal : Byte -> Byte -> Bool
byte-equal
  (bits a0 a1 a2 a3 a4 a5 a6 a7)
  (bits b0 b1 b2 b3 b4 b5 b6 b7) =
  bool-equal a0 b0 and
  (bool-equal a1 b1 and
  (bool-equal a2 b2 and
  (bool-equal a3 b3 and
  (bool-equal a4 b4 and
  (bool-equal a5 b5 and
  (bool-equal a6 b6 and
   bool-equal a7 b7))))))

byte-value : Byte -> Nat
byte-value (bits b0 b1 b2 b3 b4 b5 b6 b7) =
  bit-value b0 +
  (2 * bit-value b1) +
  (4 * bit-value b2) +
  (8 * bit-value b3) +
  (16 * bit-value b4) +
  (32 * bit-value b5) +
  (64 * bit-value b6) +
  (128 * bit-value b7)
  where
  bit-value : Bool -> Nat
  bit-value false = zero
  bit-value true = suc zero

zero-byte : Byte
zero-byte =
  bits false false false false false false false false

increment-byte-maybe : Byte -> Maybe Byte
increment-byte-maybe (bits false b1 b2 b3 b4 b5 b6 b7) =
  just (bits true b1 b2 b3 b4 b5 b6 b7)
increment-byte-maybe (bits true false b2 b3 b4 b5 b6 b7) =
  just (bits false true b2 b3 b4 b5 b6 b7)
increment-byte-maybe (bits true true false b3 b4 b5 b6 b7) =
  just (bits false false true b3 b4 b5 b6 b7)
increment-byte-maybe (bits true true true false b4 b5 b6 b7) =
  just (bits false false false true b4 b5 b6 b7)
increment-byte-maybe (bits true true true true false b5 b6 b7) =
  just (bits false false false false true b5 b6 b7)
increment-byte-maybe (bits true true true true true false b6 b7) =
  just (bits false false false false false true b6 b7)
increment-byte-maybe (bits true true true true true true false b7) =
  just (bits false false false false false false true b7)
increment-byte-maybe (bits true true true true true true true false) =
  just (bits false false false false false false false true)
increment-byte-maybe (bits true true true true true true true true) =
  nothing

nat-to-byte : Nat -> Maybe Byte
nat-to-byte zero = just zero-byte
nat-to-byte (suc value)
  with nat-to-byte value
... | nothing = nothing
... | just byte = increment-byte-maybe byte

bytes-from-nats : List Nat -> Maybe (List Byte)
bytes-from-nats [] = just []
bytes-from-nats (value ∷ values)
  with nat-to-byte value
... | nothing = nothing
... | just byte
  with bytes-from-nats values
... | nothing = nothing
... | just bytes = just (byte ∷ bytes)

-- Fixed small protocol tags.
byte0 byte1 byte2 byte3 byte4 byte5 byte6 : Byte
byte7 byte8 byte9 byte10 byte11 : Byte
byte0 = zero-byte
byte1 = bits true false false false false false false false
byte2 = bits false true false false false false false false
byte3 = bits true true false false false false false false
byte4 = bits false false true false false false false false
byte5 = bits true false true false false false false false
byte6 = bits false true true false false false false false
byte7 = bits true true true false false false false false
byte8 = bits false false false true false false false false
byte9 = bits true false false true false false false false
byte10 = bits false true false true false false false false
byte11 = bits true true false true false false false false

record U16 : Set where
  constructor little16
  field
    u16-byte0 u16-byte1 : Byte

open U16 public

record U32 : Set where
  constructor little32
  field
    u32-byte0 u32-byte1 u32-byte2 u32-byte3 : Byte

open U32 public

record U64 : Set where
  constructor little64
  field
    u64-byte0 u64-byte1 u64-byte2 u64-byte3 : Byte
    u64-byte4 u64-byte5 u64-byte6 u64-byte7 : Byte

open U64 public

u16-value : U16 -> Nat
u16-value (little16 b0 b1) =
  byte-value b0 + byte-value b1 * 256

u32-value : U32 -> Nat
u32-value (little32 b0 b1 b2 b3) =
  byte-value b0 +
  (byte-value b1 +
  (byte-value b2 +
   byte-value b3 * 256) * 256) * 256

u64-value : U64 -> Nat
u64-value (little64 b0 b1 b2 b3 b4 b5 b6 b7) =
  byte-value b0 +
  (byte-value b1 +
  (byte-value b2 +
  (byte-value b3 +
  (byte-value b4 +
  (byte-value b5 +
  (byte-value b6 +
   byte-value b7 * 256) * 256) * 256) * 256) * 256) * 256) * 256

encode-u16 : U16 -> List Byte
encode-u16 (little16 b0 b1) = b0 ∷ b1 ∷ []

encode-u32 : U32 -> List Byte
encode-u32 (little32 b0 b1 b2 b3) =
  b0 ∷ b1 ∷ b2 ∷ b3 ∷ []

encode-u64 : U64 -> List Byte
encode-u64 (little64 b0 b1 b2 b3 b4 b5 b6 b7) =
  b0 ∷ b1 ∷ b2 ∷ b3 ∷ b4 ∷ b5 ∷ b6 ∷ b7 ∷ []

take-exact :
  {A : Set} ->
  Nat ->
  List A ->
  Maybe (List A × List A)
take-exact zero input = just ([] , input)
take-exact (suc count) [] = nothing
take-exact (suc count) (x ∷ input)
  with take-exact count input
... | nothing = nothing
... | just (prefix , rest) =
  just (x ∷ prefix , rest)

take-exact-append :
  {A : Set} ->
  (prefix rest : List A) ->
  take-exact (length prefix) (prefix ++ rest)
  ≡
  just (prefix , rest)
take-exact-append [] rest = refl
take-exact-append (x ∷ prefix) rest
  rewrite take-exact-append prefix rest = refl
