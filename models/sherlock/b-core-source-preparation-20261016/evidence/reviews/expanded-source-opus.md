# Source review: extended pure leaves (13 functions, 2 private carriers)

**Role:** Opus owner, security and semantic boundary judgment. This is a fresh non-author review of the source, which Sol wrote. I wrote the leaf contract this source implements, so this review is independent of the source author but not of the contract author.

**Method:** I read the exact text supplied in the message. I used no tools and executed nothing.

**Scope:** source only. This review grants no compile eligibility, installation, adoption, whole-B closure or vote transfer.

## 1. Bindings

| Item | Declared binding | Checked here |
|---|---|---|
| Candidate manifest | `78ecada7…6f49` | Declared value only. Not recomputed. |
| `functions.proposed.rs` | 6192 B, `8f54e626…6b74` | Text reviewed as supplied. Hash and size not recomputed. |
| Prior two-function prefix | 1407 B, `c97063c2…418a` | Compared visually, line by line, with `inputs/prior-two-functions.proposed.rs`. The text is identical. Byte identity, including trailing newline and whitespace, cannot be confirmed without tools. |
| Raw types | `ef9c70a4…f7d1` | Read in full for type and pattern resolution. The source changes nothing in it and adds no derive to it. |
| Owner contract | `PRIVATE-LEAF-CONTRACTS.md` `31b5f285…cd3c` and `private-leaf-contracts.json` `1c63fb8c…d073` | Every declaration was compared with the `declarations` object in the JSON. |
| R2 sources | schema `ab180b6f…54c3`, rows `a0dd03b1…e75e` | Used for the `logical_kind`, `row_language.functions`, `Scalar.arithmetic`, `Equal.pair/different`, and SI-17 semantics. |

`compile-01` is out of scope. It still contains only the earlier two functions. None of its votes or results transfer to these bytes, and nothing here transfers back to it.

## 2. Inventory against the contract

The source has 13 private `fn`s and 2 private enums. This equals the contract's 15 declarations: 2 earlier functions, 11 new functions, `CheckedArith` and `ValueRow`.

- Every signature matches `private-leaf-contracts.json/declarations` exactly: parameter modes, return types and derive lists.
- Nothing is `pub` or `pub(crate)`.
- No other item is added. There is no new table API, guard, validator, token, `Outcome`, error label, arena read, allocation or public type.

## 3. Function-by-function review

| Function | Rust reading | Semantics vs owner contract / R2 | Result |
|---|---|---|---|
| `const_row_kind` | Matches `&ConstRow` through default binding modes. All 7 variants use `{ .. }`, which is correct because all 7 are struct variants carrying `marker_free`. There is no `_` arm. | Matches `logical_kind.kind(ref)` and contract §2.1. `U128OutOfRange` maps to `Marker`. The reserved kind is never returned. | OK |
| `data_row_kind` | Covers all 6 variants. `DataRow::Nil` is matched as a unit variant, which agrees with the raw type. | Matches §2.1. | OK |
| `CheckedArith` | `UInt128 = u128` provides Debug, Clone, Copy, PartialEq and Eq, so the derives resolve. | Exact owner enum and derive list. | OK |
| `checked_add/sub/mul` | `a.checked_*(b)` is method syntax, so it resolves to the inherent `u128` method, not the same-named free function. There is no recursion and no ambiguity. | See §4. | OK |
| `arity` | Covers all 8 `LogicalKind` variants explicitly. | `Some(0)` for UInt, Bool and Nil. `Some(2)` for Cons and Branch. `Some(1)` for Leaf. `None` only for Marker and ConstructorExtensionReserved (§2.2). | OK |
| `sort_of_kind` | Exhaustive over 8 variants. It builds fresh `SortTag` values, which needs no Copy. | UInt→UInt, Bool→Bool, Nil/Cons→List, Leaf/Branch→Tree. Marker and reserved give `None`. | OK |
| `same_logical_kind` | 8 explicit arms on `a`. Each probes `b` with `matches!` against the same variant. The braced last arm is valid syntax. | This is variant identity. The `_` arm inside `matches!` is the one exception the contract sanctions, and a new variant still breaks the outer match. | OK |
| `same_sort_tag` | 4 explicit arms, built the same way. | As above. | OK |
| `ValueRow<'a>` | `#[derive(Clone, Copy)]` on `&'a` fields needs no bounds on `ConstRow`/`DataRow`, and derive adds no lifetime bounds. It correctly omits `Debug`, which would fail to compile because the raw rows lack `Debug`. | Exact owner carrier. It borrows one row only: no view, image, arena or table. | OK |
| `value_row_kind` | Takes `ValueRow` by value (Copy). The inner `row: &ConstRow` / `&DataRow` is passed directly to the projection. | Delegates through the single projection, as D-23 requires. | OK |
| `value_row_u128` | Nested exhaustive matches, 7 + 6 arms. `n: &u128` is dereferenced to `u128`. No borrow escapes, because the return value is an owned copy. | `Some` only for the two `U128` variants. `U128OutOfRange` gives `None`, never `Some(0)`, and has no payload field in the raw type anyway. So `is_some()` holds exactly when the kind is UInt. | OK |
| `value_row_bool` | Same shape as `value_row_u128`. `b: &bool` is dereferenced to `bool`. | `Some` only for the two `Bool` variants. So `is_some()` holds exactly when the kind is Bool. | OK |
| `shallow_differ` | Unary `!` and `!=` both bind tighter than `\|\|`, so the expression groups as the contract's formula. The `&` taken of `value_row_kind(..)` temporaries is valid for the duration of the call. Using `a` and `b` more than once is fine because `ValueRow` is Copy. Each parameter has its own `'_`, and no result borrows. | This is literally the owner equation. Because of the `is_some` equivalences, it equals the R2 formula. Space is ignored, as are `marker_free` and field slices. HB-28 holds: Const `Bool(true)` vs Data `Bool(true)` gives `false`. Marker vs non-Marker gives `true`. Marker vs Marker gives `false`, as the contract specifies. | OK |

**Exhaustiveness.** Every match over a raw enum lists all variants with no `_` arm, apart from the sanctioned `matches!` probes. None of the raw enums is `#[non_exhaustive]`. A new variant in `ConstRow`, `DataRow`, `LogicalKind` or `SortTag` therefore fails to compile instead of being silently classified.

**Purity and panic freedom.** The source contains:
- no indexing, slicing, or `+ - *` operators;
- no `unwrap`, `expect`, `panic!` or `unreachable!`;
- no allocation, I/O or `unsafe`;
- no construction of `ErrorCode`, `ScalarStaged`, `PrimOutcome`, `HelperResult`, `Outcome`, `Tooling` or `HostViolation`.

Every leaf is total over its Rust parameter types. This is compatible with handoff condition C2 at leaf level.

## 4. Arithmetic results

| Leaf | `Value{n}` exactly when | `Overflow` exactly when | Notes |
|---|---|---|---|
| `checked_add` | a + b < 2^128 | a + b ≥ 2^128 | |
| `checked_sub` | a ≥ b, including a = b giving `Value{0}` | a < b | Underflow folds into `Overflow`. This matches `uiChecked`'s single error 10. |
| `checked_mul` | the full product < 2^128, including any zero operand giving `Value{0}` | the full product ≥ 2^128 | `u128::checked_mul` reports `None` exactly at mathematical overflow, so no 256-bit arithmetic is needed. |

These results are independent of debug or release build and of `overflow-checks`, because only checked methods are used. The leaves do not decide error 11 or `Lt`, and they never see `BinaryOp` or `ScalarMode`. All of that stays with the `Scalar.arithmetic` row, as R2 states.

## 5. Dependency closure

The bodies name exactly these external items: `ConstRow`, `DataRow`, `LogicalKind`, `SortTag`, `UInt128`, `Bool`. They also use prelude items: `Option`/`Some`/`None`, `matches!` (Rust ≥ 1.42) and the `u128`/`bool` primitives.

This equals the declared `required_lexical_types`.
- `ConstFieldSlice`, `FieldSlice`, `ConstFieldPos` and `FieldPos` are needed only for the raw row definitions to exist. The bodies do not name them.
- None of the new names (`CheckedArith`, `ValueRow`, `arity`, `checked_*`, `same_*`, `value_row_*`, `shallow_differ`) collides with an item in the frozen types file.
- The file has no `use` declarations. It assumes same-module or Root-chosen imports, consistent with the "no module path chosen" statement.

## 6. Findings

None of these blocks source approval.

1. **Stale prefix header (documentation, low).** The preserved prefix comment still says "private row projections only" and lists only three required lexical types. For the expanded file both statements are now incomplete. The second comment block corrects the type list, but nothing corrects the word "only". Fixing it would break the exact-prefix condition, so it is left to Root/Sol at installation time. It has no semantic effect.
2. **Unused-code warnings when compiled (compile stage, out of scope).** Nothing calls these leaves, and `ValueRow` is never constructed. A compile would therefore emit `dead_code` warnings, which would fail under `-D warnings`. This is a compile-gate concern for whoever grants compile eligibility, not a source defect.
3. **Non-governing handoff sketch divergence (observation).** ENGINEERING-HANDOFF §3 sketches a 7-variant `LogicalKind` and tuple-form `ConstRow`. The handoff states "names bind, representation does not" and "JSON governs". The source correctly follows the raw types and the schema, which have 8 variants and struct-form rows. No action is needed.
4. **Carried caller obligations (binding on later engine review, not on these bytes).**
   - Marker vs Marker returning `false` is safe only as composed. Two things must hold: SI-17 keeps markers out of the Equal helper, and the `Equal.pair` plan evaluates `arity(kind(T.left))`, maps `None` to HostViolation, and does so in the pure plan phase before any mutation.
   - The engine review must also check the other caller duties:
     - `arity`/`sort_of_kind` `None` maps to HostViolation with no mutation (`ArithmeticUndefined` for `arity(Marker)`);
     - the `ValueRef`-to-`ValueRow` deref enforces its bound and reports out-of-bound reads as HostViolation;
     - the caller supplies an unreachable arm for non-{Add, Sub, Mul} `ScalarMode` values.
   - The `HostViolationKind` labels for `sortOfKind(Marker)` and for the reserved kind remain unnamed engine-contract items.
5. **Pin-set note (observation).** The membership file lists `RESULT.json` (155 B), but it is not among the candidate inputs. It is not needed to review these bytes.

I found no mismatch, ambiguity or invented requirement in the function bodies relative to the pinned owner contract and R2.

## 7. Limitations

- I did not recompute any digest or byte count. Prefix preservation is confirmed by text comparison only.
- I did not compile, run rustfmt or clippy, test, or execute anything. The Rust syntax and type judgments above come from reading the source, not from compiler evidence.
- This review covers source semantics and Rust reading only. It does not cover:
  - representation adoption, schema freeze, installation, module placement or public API;
  - admission, StepInv, the whole-B or family engine, or acceptance.
- Native authority stays 0, and C4 and all gates and counters are unchanged.
- R2 votes and the owner interface clarification do not by themselves approve these bytes. This report is one non-author source review of these exact bytes. Any further required reviews or votes remain outstanding.

VERDICT: APPROVE_WITH_SCOPE. Scope: source-only approval of the exact 13 private functions and 2 private carriers in `functions.proposed.rs` (`8f54e626f3d42dcc35f76545e0a79131cef4d7ad9cb25fd20f0863a03f206b74`) as faithful to `PRIVATE-LEAF-CONTRACTS.md` (`31b5f285…`) and R2 `logical_kind`/`checkedArith`. It grants no compile eligibility, installation, adoption, whole-B closure or transfer to or from compile-01.

CANDIDATE_MANIFEST_SHA256: 78ecada7b2c260017b20eb5511dfa09bda0650c2474ef6ef7fb6f71aa7c86f49
