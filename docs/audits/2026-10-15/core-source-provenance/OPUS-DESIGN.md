# Opus security/design review: mapped-role source-static emission map

**Vote: AGREE with constraints (design only).** For the command, my vote is **NEED-SOURCE**. There is no GO, no candidate approval and no permission for native execution. I verified no checkout or hashes. Every statement below is based on the supplied snapshot. All U0–U7/M1–M17/MC/SP/G exits remain open. Native authority remains zero, and gross4334/remaining0 is unchanged.

## 1. Disposition of the gap

- **The map is useful and not harmful under the constraints below.** Today a consumer must re-run compiler logic to attribute `source-claim` groups to statements. Origins and widths cannot do this: a Transfer and a Fee to the same `(asset, to)` emit identical 13-group runs.
- **Existing consumers are affected only in these ways:**
  - `static_compile::reopen` compares bytes against a fresh compile, so it absorbs the new field automatically.
  - `mapped_role_static_io.rs` asserts `m["format"]`, so that one assertion changes.
  - `process.rs` and `relation::check` consume only the ordinary `Manifest.groups`, so they are unaffected.
  - `mori-ledger/tests/static_compile.rs:823–835` uses frozen origins and widths. These must remain unchanged because `Group` is unchanged.
- **The map is emission provenance, not semantics.** It records which instructions, groups and replaced fields the compiler emitted while lowering statement *i*. It does not record dependencies: Let wires and claim aggregates cross statements. It does not record activity, because guards can disable groups. It does not record value equivalence, debit binding or native correspondence.
- **The IR does not bind `from` or the transfer/fee kind.** `claim` uses only token and recipient. Kind and `from` exist only in the source and in `abi.actions`. The map must not imply otherwise.

## 2. Exact minimal design

Serialize-only data, appended to `SourceStaticManifest` after `debit_sources`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct DynamicField { pub(crate) group: usize, pub(crate) offset: usize }
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct StatementEmission {
    pub(crate) statement: usize,          // == position; index into abi.actions[action].statements
    pub(crate) expression: ExprId,        // that statement's root (amount/value/predicate)
    pub(crate) instruction_start: usize,  // half-open [start, end) into IrSource.instructions
    pub(crate) instruction_end: usize,
    pub(crate) group_start: usize,        // half-open into manifest.groups (== k-th Impact)
    pub(crate) group_end: usize,
    pub(crate) dynamic_fields: Vec<DynamicField>, // actual Builder::op replacements, absolute group
}
// SourceStaticManifest { ..., debit_sources, statement_emissions: Vec<StatementEmission> }
```

Constraints:

1. **Record one entry per source statement, in order.** Every statement kind is included, even when it has zero groups (Let, Requires). Transfer and Fee are not singled out. This makes the entries a checkable partition.
2. **Do not duplicate kind, asset, from or to.** These resolve through `statement` into `abi.actions` in the same manifest. Do not add semantic role labels such as "amount" or "next-aggregate". `expression` is the single permitted redundancy, and a test checks it.
3. **Handle template-generated fields as follows:**
   - List only offsets that were actually replaced in `Builder::op` (`replacements`), in the order given.
   - Do not list template constants, such as claim keys or static `Popeq` results. They remain IR-only.
   - Do not compute positions synthetically or project absolute public-input offsets. Those are derivable as `2 + prefix-sum(field_count)` and would be a second source of truth.
   - Immediate-valued replacements, such as a literal `next s = 5`, are still listed. The map is the only record that the field was replaced.
4. **Capture point:**
   - `Builder` gains `dynamic: Vec<DynamicField>`, pushed in `op` only after the existing `template-offset` check succeeds.
   - `compile_inner` snapshots `(instructions.len(), groups.len(), dynamic.len())` around each `c.statement(...)`.
   - The entries come back as an extra return member. `compile` and `compile_signed` discard it.
   - No instruction or group push may be added, removed or reordered.
5. **Assembly checks in `compile_source_static_roles` only.** These refuse with `source-static-emission`:
   - entry count must equal the statement count;
   - ranges must be contiguous;
   - offsets must be strictly ascending within each group;
   - each offset must be less than its group's `field_count`.

   Do not change the shared `Builder::op` refusal behaviour.
6. **Boundedness:**
   - Entries are at most the number of statements.
   - `dynamic_fields` are at most the total public fields, which `max_public_inputs` already bounds.
   - Bytes are covered by the existing `artifact-size`, `relation-leaf-limit` and aggregate checks, because the map lives in the manifest leaf.
   - Consequence: fixtures near the leaf limit may begin to refuse. That refusal is the accepted fail-closed outcome.
7. **Out of scope:**
   - No `Deserialize` on the new types. The map is never an input.
   - No CLI output change.
   - Do not touch `ops.rs`, because `templates_hash` would break every binding.
   - Do not touch `layout.rs`, `static_compile.rs`, `event.rs`, `process.rs` or `signed/*`, nor the ordinary `Manifest`/`Group`.

## 3. Compatibility disposition (no byte-compatibility promise)

| Artifact | Effect |
|---|---|
| Emitted IR bytes | Must be unchanged for fixed source and bindings in every mode. This is a design requirement; see test N0 for how it is evidenced. |
| Ordinary and signed `Manifest` | Schema unchanged. **Bytes change**, because `compiler_hash` hashes `include_str!` of mod.rs, compiler.rs and arithmetic.rs. Previously exported ordinary artifacts will fail `admit` with `artifact-identity`. This failure is closed but must be disclosed. |
| All static bundles (all three profiles) | `marker.build = source_closure()` changes if the closure covers these files. Old bundles fail reopen with `marker-identity`. If the closure does not cover them, old bundles fail with `fresh-source-mismatch` instead. Either way they fail closed. |
| Mapped manifest format | **Bump `SOURCE_STATIC_FORMAT` to `moriarty-mapped-role-source-static-relation/2`.** The leaf's shape changes, and `format` is its only self-description. In-tree JSON readers already key on it. |
| Bundle `MAPPED_ROLE_PROFILE` /1 and both qualifications | Keep. The marker schema is unchanged, and build plus fresh-equality already reject mixed bundles. This is not a new profile. |
| Frozen hash literals | Any test, example or doc that freezes `compiler_hash`, `build` or a marker identity must be updated in the same diff, with explicit reviewer sign-off. Do not let them be regenerated silently. |
| Native residual | Anything keyed on these hashes is invalidated. That requires later native requalification, which is not authorized now. |

## 4. Tests

These expectations are Opus-authored and hand-derived from the supplied source. If a result does not match, **stop and report**. Do not edit an expectation to match the output. Expected values must not call the compiler.

**Fixture.** Use the existing `bindings()` helper. Roles are alice, bob and treasury.

- `pair(amount:UInt){transfer(PAY,"alice","bob",amount);fee(PAY,"alice","bob",2);}`
- `derived(amount:UInt){let x = amount + 1; fee(PAY,"alice","treasury",x); transfer(PAY,"bob","treasury",x);}` (adapt the Let syntax to Core grammar only)
- `idle(){emit "observation"(true);}`

### Ordinary normal mapping checks (non-security)

- **N0 – IR invariance.** Before editing, capture per-action IR SHA-256 values from the baseline build using the same export, classified separately. Freeze them as literals and assert they are equal after the edit. If this capture is not obtained, report IR invariance as review-only.
- **N1 – round trip.** Export is accepted. `--inspect-static` reopen is accepted. Manifest `format` is `/2`. `debit_sources` and the qualification strings are unchanged.
- **N2 – entries per action.** `statement_emissions.len() == statements.len()`, `statement == index`, and `expression` equals the root `ExprId` of the matching statement in `abi.actions`.
- **N3 – `pair` exact layout:**
  - `groups.len() == 29`.
  - Groups 0–2 are `abi-root`, and groups 3–28 are `source-claim`.
  - Statement 0 covers groups [3,16) and instructions [7,41).
  - Statement 1 covers groups [16,29) and instructions [41,75).
  - The instruction count is 76; the last instruction is `Output`.
  - Dynamic groups are {8, 11, 13} and {21, 24, 26}, each at `offset == groups[g].field_count − 1`.
- **N4 – `derived`:**
  - The Let entry has `group_start == group_end` and exactly 5 instructions.
  - Each claim statement has 13 groups, 34 instructions and 3 dynamic fields at template positions 5, 8 and 10.
- **N5 – `idle`.** One entry. It covers all groups after the preamble, and every dynamic field lies inside the entry's range.

### Hostile security checks

- **H1 – partition integrity, for every action:**
  - The first entry starts at the end of the preamble: 3 groups, and `inputs + 6` instructions.
  - Entries are contiguous and disjoint.
  - The last entry ends at `groups.len()` for groups and at `instructions.len() − 1` for instructions.
  - Every `(group, offset)` lies inside its own entry's range, satisfies `offset < field_count`, is strictly ascending, and appears in no other entry.
  - With the IR decoded through `mori_ledger::codec::decode_artifact`: group *k* is the *k*-th `Impact` and lies inside its entry's instruction range. Every `Variable` operand among `Impact` inputs is listed. Listed `Immediate` entries occur only where the statement's root is a literal.
- **H2 – kind is not in the IR.** Build a second fixture with `transfer(...,amount); fee(...,amount)` and a third with the two statements swapped. Expect:
  - IR leaf bytes equal;
  - emission ranges equal;
  - manifest bytes differ;
  - the statement kinds resolved through `abi.actions` swap.

  This establishes that the map plus the ABI distinguishes kind and the IR does not.
- **H3 – from is not in the IR.** Change `from` alice→treasury in `pair` and recompute the bindings. Expect:
  - IR bytes equal;
  - emission entries equal;
  - `debit_sources` changes.

  The map creates no debit binding.
- **H4 – tamper with recomputed hashes.** In an exported bundle, mutate `statement_emissions` in turn by: swapping two entries' ranges; shifting one offset; deleting the field; or relabelling `format` to `/1`. In each case recompute the leaf's sha256 and byte count in the marker and recompute the marker identity. Expect reopen to refuse with `fresh-source-mismatch` and no bytes outside the fixture to change.
- **H5 – conditional on the H5 closure in §5.** Run a single-payer fixture through `--authorization unsigned`. Expect:
  - no `statement_emissions` key;
  - ordinary format still `moriarty-native-call-relation/1`;
  - IR bytes equal to the mapped export for the same action;
  - the ordinary manifest with `statement_emissions` injected is refused by `serde_json::from_slice::<mori_ledger::relation::Manifest>`, because the type uses `deny_unknown_fields`.

  If the closure is not cleared, H5 is review-only.

## 5. Missing source for execution closure (required before any command classification)

1. `crates/mori-ledger/src/proving.rs`: `source_closure()`. It is reached on every export and reopen despite living in the proving module. Confirm what it hashes and that it does no file, cache, parameter or key I/O.
2. `crates/mori-ledger/src/call_artifacts.rs`: `json_bytes`, `ArtifactLeaves`, `Leaf`.
3. `crates/mori-ledger/src/profile.rs`: `StaticBindings`, `RoleBinding`, `NativeWorkLimits::{validate,cap}`.
4. `crates/mori-ledger/src/lib.rs`: module visibility and any initializers.
5. In `mori-core`: `parse`, `validate`, `json::{parse,deserialize_strict}`, `intent::canonical`, and the `ExprId`/`Statement` definitions, including whether a zero-literal amount and the Let syntax are admitted.
6. CLI closure: `crates/mori/src/{main.rs,shell.rs,commands/mod.rs,commands/core_io.rs,commands/proof_io.rs}`.
7. Pinned SDK code **executed at runtime** on this path. PR52 linkage/pureCore clearance does not cover runtime execution:
   - `midnight_serialize` tagged (de)serialize of `IrSource`;
   - `FieldRepr::field_vec`/`field_size` on `Op` and `AlignedValue`;
   - `midnight_coin_structure` decoders and `UserAddress::from(SignatureVerifyingKey)`;
   - `ClaimedUnshieldedSpendsKey`;
   - `midnight_storage::arena::Sp::new` with `serialize_to_node_list_bounded` over `InMemoryDB`. The `idle` emit reaches this. Confirm the backend is in-memory only, with no env- or path-configured DB.
8. Manifests and lockfile: `crates/mori/Cargo.toml`, `crates/mori-ledger/Cargo.toml`, `Cargo.lock`, and any `build.rs`. Confirm the test can name `IrSource` without a Cargo change. If it cannot, move H1's IR-decode part to a filtered `mori-ledger` lib test, which needs its own classification.
9. Fixture inputs: `examples/core/static-compile/{work-limits.json,financial-duty.bindings.json}`. Confirm the limits admit the fixture.
10. Repository-wide grep for:
    - `moriarty-mapped-role-source-static-relation/1`;
    - frozen `compiler_hash`, `build` or identity literals in tests, examples and docs;
    - `mori-ledger/tests/static_compile.rs:823–835`.
11. For H5 only: `local_deployment::compile_all_policy`.

## 6. Conditions for a later GO (not granted)

- All sources in §5 are inspected, with no native, keygen, proving-cache, VM or ledger reachability.
- The diff is limited to `relation/{compiler.rs,arithmetic.rs,mod.rs}` plus the single test file, and the frozen-literal updates are disclosed.
- The command is exactly the proposed `cargo +1.99.0 --offline --locked test -p mori --test mapped_role_static_io -j2 source_step_field_correspondence_survives_public_export_and_reopen -- --exact --test-threads=1`.
- `MORI_FEE_POOL_CAPTURE_DIR` is unset.
- Product subprocesses are only mapped (and, for H2/H3, fixture-variant) export and `--inspect-static`. The H5 `unsigned` export runs only if its closure is cleared.
- The build-script and thirdparty residual stays recorded as uncleared, not waived.
- The N0 baseline capture is classified as a separate command.
