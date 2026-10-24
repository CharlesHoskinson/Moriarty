module

public import Std

@[expose] public section

/-!
# Canonical Sherlock type identity

This is the semantic carrier of the production `TypeIdC(usize)` migration. Builtin names
designate rows 0 through 3 of the same carrier; later rows retain their complete numeric identity.
No declaration-table membership, host-width admission or bounded-field encoding is inferred here.
-/

namespace Clean.Sherlock.CanonicalTypeId

structure TypeId where
  index : Nat
  deriving Repr, DecidableEq

structure CtorId where
  index : Nat
  deriving Repr, DecidableEq

inductive BuiltinType where
  | uint | bool | list | tree
  deriving Repr, DecidableEq

def TypeId.ofBuiltin : BuiltinType → TypeId
  | .uint => ⟨0⟩
  | .bool => ⟨1⟩
  | .list => ⟨2⟩
  | .tree => ⟨3⟩

def TypeId.builtin : TypeId → Option BuiltinType
  | ⟨0⟩ => some .uint
  | ⟨1⟩ => some .bool
  | ⟨2⟩ => some .list
  | ⟨3⟩ => some .tree
  | ⟨_ + 4⟩ => none

theorem builtin_ofBuiltin (kind : BuiltinType) :
    (TypeId.ofBuiltin kind).builtin = some kind := by
  cases kind <;> rfl

/-- A non-builtin row is not converted to an old four-way source sort. -/
theorem builtin_nominal (offset : Nat) : (TypeId.mk (offset + 4)).builtin = none := rfl

/-- Canonical identity retains the full row index, including all non-builtin rows. -/
theorem typeId_eq_iff (left right : Nat) : TypeId.mk left = TypeId.mk right ↔ left = right := by
  constructor
  · intro h
    exact congrArg TypeId.index h
  · intro h
    cases h
    rfl

end Clean.Sherlock.CanonicalTypeId
