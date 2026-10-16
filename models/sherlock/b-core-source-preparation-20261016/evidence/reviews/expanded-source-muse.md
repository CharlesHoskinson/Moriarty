# Nonsecurity Rust/source/mapping faithfulness audit

VERDICT: APPROVE_WITH_SCOPE

CANDIDATE_MANIFEST_SHA256: 78ecada7b2c260017b20eb5511dfa09bda0650c2474ef6ef7fb6f71aa7c86f49

## Scope and bindings

This approval is limited to source-level faithfulness of the exact expanded, runtime-only, uninstalled source candidate. It is not a compile-eligibility grant, an installation approval, a representation or schema adoption, or a whole-B determination.

The reviewed function source is bound by the supplied [function mapping](/FUNCTION‑MAPPING.json) to:

- `functions.proposed.rs`, 6192 bytes, SHA256 `8f54e626f3d42dcc35f76545e0a79131cef4d7ad9cb25fd20f0863a03f206b74`
- frozen raw-types input SHA256 `ef9c70a41933acf2c4310dab566c83fd72087b8ea435869a8c283f6c40aff7d1`
- owner leaf-contract SHA256 `31b5f2854e93a2010cbd3c1d69395b8eea6a7658a9d7d0bd8f87862a509edecd`
- preserved prior-two-function prefix, 1407 bytes, SHA256 `c97063c24b4c33faaca3797da6d2e22ad1556477627131799b90a18de667418a`

Those hashes and the prefix-preservation claim are accepted as supplied because this review was constrained to reading the provided exact text, without tool execution or hash recomputation. No result from the separate compile-01 candidate, which covered only the earlier two functions, was transferred to the additional functions or carriers.

## Findings

No faithful-mapping, Rust-reading, exhaustiveness, carrier-lifetime, payload, or checked-arithmetic mismatch was found in the 13 functions and two new private carriers.

- `const_row_kind` covers the seven frozen `ConstRow` variants and maps `U128OutOfRange` to `LogicalKind::Marker`.
- `data_row_kind` covers the six frozen `DataRow` variants. It correctly uses the unit pattern `DataRow::Nil`, while the remaining struct-like variants use `{ .. }`.
- Neither projection inspects `marker_free`, payloads, or field slices. Neither can return `ConstructorExtensionReserved`.
- `CheckedArith` has exactly the owner-specified variants and derives: `Debug, Clone, Copy, PartialEq, Eq`.
- `checked_add`, `checked_sub`, and `checked_mul` correctly implement the owner cases through `u128::checked_add`, `u128::checked_sub`, and `u128::checked_mul`. A `None` result becomes `Overflow`, including subtraction underflow and a multiplication whose full mathematical product reaches or exceeds `2^128`. No mode argument, application error, or staging decision was added.
- `arity` matches all eight `LogicalKind` variants: `UInt`, `Bool`, and `Nil` map to `Some(0)`; `Cons` and `Branch` to `Some(2)`; `Leaf` to `Some(1)`; `Marker` and `ConstructorExtensionReserved` to `None`.
- `sort_of_kind` matches all eight variants: `UInt -> Some(SortTag::UInt)`, `Bool -> Some(SortTag::Bool)`, `Nil` and `Cons` to `Some(SortTag::List)`, `Leaf` and `Branch` to `Some(SortTag::Tree)`, and both undefined kinds to `None`.
- `same_logical_kind` has eight exhaustive arms on `a`, each using the permitted `matches!` probe on `b`.
- `same_sort_tag` has four exhaustive arms on `a`, each using the same probe form on `b`.
- Every match over the frozen raw enums explicitly enumerates the relevant variants, with no wildcard arm. The `Option` matches in the checked functions are also exhaustive through `Some` and `None`.
- `ValueRow<'a>` is exactly the owner-specified private borrowed carrier `Const(&'a ConstRow)` and `Data(&'a DataRow)`, deriving only `Clone` and `Copy`. Deriving `Debug` would have been inappropriate because the frozen row types provide no `Debug`.
- `value_row_kind` delegates to the two row-kind projections without arena access, bounds checks, field inspection, or allocation.
- `value_row_u128` exhausts all seven `ConstRow` variants and all six `DataRow` variants. It returns a copied primitive only for the two `U128` variants. In particular, `ConstRow::U128OutOfRange` returns `None`, never `Some(0)`.
- `value_row_bool` likewise exhausts all thirteen row variants and returns a copied primitive only for the two `Bool` variants.
- The payload invariants are aligned: `value_row_u128` is `Some` exactly when the projected kind is `UInt`, and `value_row_bool` is `Some` exactly when it is `Bool`.
- `shallow_differ` follows the owner literal equation: negated logical-kind identity, or an optional-`UInt128` mismatch, or an optional-`Bool` mismatch. It is space-independent across Const/Data combinations and ignores `marker_free`, field slices, and field contents. `Marker` versus `Marker` consequently evaluates to `false`, as required by the literal owner formula. Different kinds evaluate to `true`; equal `UInt` and `Bool` kinds compare their payloads; equal `Nil`, `Cons`, `Leaf`, or `Branch` kinds evaluate to `false`.
- Because `ValueRow` is `Copy`, repeated use of each argument in `value_row_kind`, `value_row_u128`, and `value_row_bool` does not move or consume it. Both accessors return owned copied primitives, and `value_row_kind` returns an owned `LogicalKind`, so no borrowed payload escapes.
- The functions introduce no table API, caller guard or policy, validator, capability token, `Outcome`, error label, arena read, allocation, public export, or schema change. The items are private, and the leaves construct none of the prohibited caller-level result types.

## Limitations and separate responsibilities

By ordinary Rust reading, the unqualified names `ConstRow`, `DataRow`, `LogicalKind`, `UInt128`, `Bool`, and `SortTag` are all present in the frozen raw-types source with compatible variants, fields, and aliases. The fragment assumes eventual placement in, or import into, a scope where those names resolve. It chooses no module path and provides no imports; installation and module placement remain separate.

The source has not been compiled or executed in this review. Accordingly, this is a careful syntax, type, lifetime, dependency, and mapping reading, not compiler evidence. Callers remain responsible for resolving a `ValueRef` into a borrowed `ValueRow`, enforcing read bounds, converting `CheckedArith` and `None` results into row staging or the separately specified caller dispositions, and selecting any caller-level violation labels.
