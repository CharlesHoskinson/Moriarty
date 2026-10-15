# Opus 5.5 security source additions: mapped-role emission assembly integrity

Status: **SOURCE ONLY. Nothing has been compiled, formatted, executed or approved.** Author: Opus 5.5 (high), sole security owner. Checkout `/home/charl/Moriarty/.worktrees/core-source-provenance-20261015`, base `eb2577d6c67684b950288d769ea2fcb91e731bf1`, branch `feat/core-source-provenance-20261015`.

I loaded the installed develop skill and read the checkout AGENTS. I did not run guarded status, because the prompt says "only if needed" and nothing here needed it. Commands used: `ls`, `git status`, `git diff` and `sha256sum`/`wc` on the changed source files. I ran no cargo, rustfmt, product, SDK, check, VM, degree, keygen, proof, deploy, ledger, private-material or cache command. Nothing was staged or committed. Native authority stays at zero, with gross 4334 / remaining 0. This report gives no security approval, no GO and no command classification. Every U0–U7, M1–M17, MC, SP and G exit stays open.

## 1. Write scope used

| Path | Change by Opus |
|---|---|
| `crates/mori-ledger/src/relation/mod.rs` | Added the validator, its single call in mapped `compile_source_static_roles`, and the `#[cfg(test)] mod emission_tests` module |
| this file | report |

I did not change `compiler.rs`, `arithmetic.rs`, `event.rs`, `process.rs`, shared `Group`/`Manifest`, the signed paths, `layout.rs`, `ops.rs`, `static_compile.rs`, the CLI, Sol's normal integration test, examples, docs or evidence. I did not overwrite `OPUS-SOURCE-ADDITIONS.md`.

Hashes after my edit (repository observation, `sha256sum`):

| File | SHA-256 | Note |
|---|---|---|
| `relation/mod.rs` | `2c0dcd208c8afdcd8947db261a7729e64148038362b8c6bcefb92699ec1bfb11` (22451 bytes) | mine, on top of Sol's `ad718dca…` |
| `relation/compiler.rs` | `203e7589ddc137b88b3317dc54014e38f8d084ce30d213c04172b2ac4b9d4548` | same as AUTHOR-SOURCE-PLUMBING |
| `relation/arithmetic.rs` | `dea48a884062407cd745c9dc02a56b055f472aee50e8f8ed6e5f7c290725e47a` | same as AUTHOR-SOURCE-PLUMBING |
| `relation/event.rs` | `bb71ab5c58c313565b423060ce7f329b6dbf146bf72c816b789d3742852a18b8` | not in the diff |
| `relation/process.rs` | `1bd16e3df59a8fe7eae827fcf04649e3782063a965d43c958061dba31656439d` | not in the diff |

**Observed changes I did not make (disclosed, not reviewed here):**
- `crates/mori/tests/mapped_role_static_io.rs` is now `dfec0e675babb5686c019739f239ac5a1e48854ca70f30ac7ecf6bf61be16389` (26515 bytes, mtime 20:10:53 −0600). That differs from the `c2241d08…`/25810 bytes recorded in AUTHOR-SOURCE-PLUMBING.json.
- `docs/NATIVE-CORE.md` has a new uncommitted 34-line section, "Mapped-role statement provenance" (mtime 20:13:30 −0600). It quotes this design.

Both writes happened after the plumbing record and are outside my scope. Root must attribute them and audit them separately. My text in that doc section is consistent with the validator: replacements may be Immediate, and the prelude and epilogue lie outside statement ranges.

## 2. Exact source change (`relation/mod.rs`)

### 2.1 Imports

```rust
-use midnight_zkir_v3::IrSource;
+use midnight_zkir_v3::{Instruction, IrSource, ir::Operand};
```

### 2.2 Validator, added after `impl SourceStaticArtifact`

```rust
fn statement_root(s: &mori_core::Statement) -> ExprId {
    use mori_core::Statement as S;
    match s {
        S::Let { value, .. } | S::Next { value, .. } | S::Emit { value, .. } => *value,
        S::Requires { predicate } | S::Ensures { predicate } => *predicate,
        S::Transfer { amount, .. }
        | S::Fee { amount, .. }
        | S::Liability { amount, .. }
        | S::Discharge { amount, .. } => *amount,
    }
}
/// Emission bookkeeping closure only. Entries partition the body between the
/// ABI-root prologue and the final Output; group k is the k-th Impact; every
/// Variable Impact input is a listed replacement. A listed replacement may
/// still be Immediate. Activity, values, kind, payer and debits are not bound.
fn validate_statement_emissions(
    statements: &[mori_core::Statement],
    instructions: &[Instruction],
    groups: &[Group],
    emissions: &[StatementEmission],
) -> Result<()> {
    let refuse = || refusal("source-static-emission");
    let body_end = match instructions.split_last() {
        Some((Instruction::Output { .. }, body)) => body.len(),
        _ => return Err(refuse()),
    };
    let impacts = instructions
        .iter()
        .enumerate()
        .filter_map(|(i, instruction)| match instruction {
            Instruction::Impact { inputs, .. } => Some((i, inputs)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let prologue = groups
        .iter()
        .take_while(|g| g.origin == "abi-root")
        .count();
    if emissions.len() != statements.len()
        || impacts.len() != groups.len()
        || impacts
            .iter()
            .zip(groups)
            .any(|((_, inputs), g)| inputs.len() != g.field_count)
        || groups[prologue..].iter().any(|g| g.origin == "abi-root")
    {
        return Err(refuse());
    }
    let groups_before = |instruction: usize| impacts.partition_point(|(i, _)| *i < instruction);
    let mut instruction = impacts[..prologue].last().map_or(0, |(i, _)| i + 1);
    let mut group = prologue;
    let mut listed: Vec<(usize, usize)> = Vec::new();
    for (index, (e, s)) in emissions.iter().zip(statements).enumerate() {
        if e.statement != index
            || e.expression != statement_root(s)
            || e.instruction_start != instruction
            || e.group_start != group
            || e.instruction_end < e.instruction_start
            || e.instruction_end > body_end
            || groups_before(e.instruction_start) != e.group_start
            || groups_before(e.instruction_end) != e.group_end
        {
            return Err(refuse());
        }
        for f in &e.dynamic_fields {
            if !(e.group_start..e.group_end).contains(&f.group)
                || f.offset >= groups[f.group].field_count
                || listed.last().is_some_and(|last| *last >= (f.group, f.offset))
            {
                return Err(refuse());
            }
            listed.push((f.group, f.offset));
        }
        instruction = e.instruction_end;
        group = e.group_end;
    }
    if instruction != body_end
        || group != groups.len()
        || impacts.iter().enumerate().any(|(k, (_, inputs))| {
            inputs.iter().enumerate().any(|(j, v)| {
                matches!(v, Operand::Variable(_)) && listed.binary_search(&(k, j)).is_err()
            })
        })
    {
        return Err(refuse());
    }
    Ok(())
}
```

### 2.3 Single call site, mapped path only

In `compile_source_static_roles`, immediately after `source_action` is resolved and before `debit_sources` and manifest assembly:

```rust
    validate_statement_emissions(
        &source_action.statements,
        ir.instructions.as_slice(),
        &groups,
        &statement_emissions,
    )?;
```

### 2.4 What did not change

- Sol's `SourceStaticManifest` fields and order are unchanged, including `statement_emissions` last after `debit_sources`.
- `DynamicField` and `StatementEmission` remain Serialize-only. I added no `Deserialize`, and the map is never an input.
- No IR instruction or group is added, removed or reordered. The validator only reads.
- `Builder::op` refusal behaviour (`template-offset`, `empty-operation`) is unchanged.
- The ordinary `compile`/`compile_bindings`/`admit` paths and the signed path do not call the validator. They receive no map.
- All caps are unchanged: `compiled-work-bound` (1,000,000 instructions / 100,000 groups), `artifact-size`/`OUTPUT_BYTES`, and the leaf and aggregate checks. I added no new limit.
- Static reopen recompiles through this same function, as Sol's closure notes for `static_compile.rs:612`. The validator therefore runs on both export and fresh reopen. I did not re-inspect `static_compile.rs` in this assignment.

## 3. Checks enforced, all refusing `native-relation` / `source-static-emission`

The design's §2.5 checks (D) are implemented as specified. I added further closure checks (A), each with its reason:

| # | Check | Origin | Security reason |
|---|---|---|---|
| 1 | IR ends with exactly one trailing `Output`, which marks the body end | A | The epilogue must be known before the partition can be closed |
| 2 | entry count == statement count | D | Every statement is present, none fabricated |
| 3 | `#Impact == groups.len()` and each Impact's `inputs.len() == field_count` | A | Group *k* is the *k*-th Impact; `offset < field_count` is then measured against real IR width |
| 4 | `abi-root` groups form exactly the leading prologue; none appear in the body | A | The prologue cannot be attributed to a statement, and source groups cannot hide in the prologue |
| 5 | first entry starts immediately after the last `abi-root` Impact (instruction) and at the prologue group count | A | No source-emitted instruction is left unattributed before statement 0. For the mapped path this equals `inputs + 6` (the prior H1 numeric form) without hard-coding it |
| 6 | `statement == index` | A | The ordered index into `abi.actions[action].statements` |
| 7 | `expression == root(statement)` | A, from design §2.2 ("a test checks it"), promoted to enforcement | The one permitted redundant field is bound to the source |
| 8 | contiguous: each `start` equals the previous `end`; `end >= start` | D | Partition |
| 9 | `group_start`/`group_end` equal the number of Impacts before `instruction_start`/`instruction_end` | A | Group ranges are bound to instruction ranges in the actual IR, so they cannot drift independently |
| 10 | the last entry ends at `body_end` (just before Output) and at `groups.len()` | A | No unattributed epilogue work, and nothing claims the Output |
| 11 | each dynamic `group` lies inside its own entry's group range | A (prior H1) | Replacements cannot be credited to another statement |
| 12 | `offset < groups[group].field_count` | D | |
| 13 | `(group, offset)` strictly ascending across all entries | D (per group), strengthened | Rules out duplicates and reordering. Global order follows from contiguity and enables the completeness search |
| 14 | every `Operand::Variable` among Impact inputs is a listed `(group, offset)` | A (prior H1, promoted) | Completeness: no non-constant public field is missing from the map |

Work is bounded by the existing caps: O(I + F·log F), with an `impacts` vector of at most the group cap and a `listed` vector of at most the total public fields. The validator does no I/O, uses no SDK relation or VM, does no hashing and allocates nothing beyond those two vectors.

**Brittleness, which fails closed by design:** the literal origin `"abi-root"`. The mapped path always emits the non-consuming guard (`compiler.rs:703–707`). If someone renames the origin or adds a prologue Impact of another origin, export refuses. That refusal is the intended outcome; a silent mis-partition is not.

`statement_root` duplicates the compiler's Statement→root match on purpose. It is the independent side of check 7. It is exhaustive with no wildcard, so a new `Statement` variant fails to compile until both matches are updated.

## 4. Unit tests authored, not executed

Module `relation::emission_tests` in `relation/mod.rs` (`#[cfg(test)]`). Every test calls the production `validate_statement_emissions` on hand-built pure data: `mori_core::Statement`, `midnight_zkir_v3::Instruction::{Impact, Assert, ConstrainBits, Output}`, `Group` and `StatementEmission`. None of them calls the compiler, the SDK relation check, the VM, a proof, file I/O or deserialization. The specimen is **hand-derived, not compiler output, and not an oracle for product emission**:

```
instr 0 ConstrainBits %input.0         prologue
instr 1 Impact [1,2]          group 0  abi-root
instr 2 Assert                          statement 0 (Transfer, root 0)
instr 3 Impact [3,%native.0]  group 1  source-claim, dynamic (1,1)
instr 4 Impact [4,5,6]        group 2  source-claim, constant
instr 5 Assert                          statement 1 (Let, root 2), no groups
instr 6 Output
entries: (0,0,[2,5),[1,3),[(1,1)])  (1,2,[5,6),[3,3),[])
```

| Test | Expectation (frozen before any execution) |
|---|---|
| `exact_emission_partition_is_admitted` | `Ok` |
| `constant_replacement_may_be_listed` | adding Immediate slot `(2,0)` → `Ok` (supports correction C2) |
| `statement_kind_and_payer_are_not_bound_by_emission_check` | Transfer→Fee and alice→bob → `Ok`. This records a negative property: the validator binds the root only, never the kind or payer |
| `statementless_action_has_only_prologue_groups` | `[Impact abi-root, Output]` with no entries → `Ok`; relabelling that group `source-claim` → refused |
| `assembly_violations_refuse` | 22 single mutations, each refused with layer `native-relation`, code `source-static-emission`: missing entry; extra entry; entries reordered; statement index; root expression; prologue attributed to a statement; statement instruction left in the prologue; instruction gap; group range detached from Impact; reversed range; entry stops before Output; entry covers Output; missing Output; Impact without group; width differs from Impact; `abi-root` origin in body; field in another entry; offset beyond width; duplicate field; descending fields; Variable replacement omitted; Variable in an unlisted slot |

I traced each mutation by hand against the validator to the check that refuses it. That trace is a source reading, not an execution result.

**Formatting and compilation are unverified.** No rustfmt and no build were run. Possible issues a later build could expose include rustfmt line-breaking differences and type inference in the closure patterns. I wrote these conservatively, with an explicit `Vec<(usize, usize)>` and an `impl FnOnce` mutation helper rather than fn-pointer tables. Any failure is a source defect to repair, not a result to tune.

Positive coverage on real compiler output: Sol's normal test `source_step_field_correspondence_survives_public_export_and_reopen` exports the oracle five-step fixture through `compile_source_static_roles`. If it goes GREEN under a classified run, that will be the first observation that the validator admits honest emission. I checked the oracle rows against the validator by reading only: prologue `[0,7)` with groups `[0,3)` and Impacts 1, 2, 6, so the first start is 7. The Let groups 29–31 are `state-read`/`state-opening`, not `abi-root`. The dynamic Variables are at 8/11/13, 21/24/26, 31, 37/40/42 and 50/53/55. Output is at 154. I found no conflict. This is a reading, not an observation.

Execution needs a separately classified command, which I am not proposing or classifying here. The test path filter is `relation::emission_tests::`. Building the `mori-ledger` lib test target compiles every `#[cfg(test)]` module in the crate. Whoever classifies the command must account for that.

## 5. Corrections to my prior OPUS-DESIGN.md

The original OPUS-DESIGN.md/.json are kept unchanged. The corrections are:

- **C1 (retained and strengthened).** H1 "every `Variable` operand among `Impact` inputs is listed". Source confirms that `Builder::op` (`arithmetic.rs:191–217`) is the only Impact and Group producer, and that template inputs are always `field(...)` Immediates. This check now runs at every export and reopen (check 14), not only in a test.
- **C2 (withdrawn: the premise was wrong).** H1 said "listed `Immediate` entries occur only where the statement's root is a literal". That is neither necessary nor sufficient:
  - A literal-root Transfer or Fee yields **Variable** replacements. The claim's member and old values are `open` outputs, and `next` is an `Add` output (`compiler.rs:358–384`). The oracle's literal steps 0 and 4 confirm this.
  - A `Name` root bound by `let x = 5` yields an **Immediate** state-write replacement. `Expr::Name` returns the env wire (`compiler.rs:473–477`), `Let` stores the literal wire (`:521`), and `Next` replaces with `value.val` (`:544–546`).

  The corrected property is: Variable ⊆ listed, and listed Immediates are permitted. No validator check targets Immediate placement, because the property has no security value on its own.
- **C3 (qualified).** §2.1 cited Let and Requires as zero-group examples. They emit groups whenever their root reads state (`compiler.rs:513–517`) or observes a duty (`:452–465`). The oracle Let at step 2 has three groups. Entry presence is unconditional; having zero groups is not.
- **C4 (scope).** §2.5 listed four checks. The implementation adds the ten closure checks in §3, marked "A". All use the same refusal code. H1's prologue form "`inputs + 6` instructions" is now enforced structurally (check 5). For the mapped path the two are equivalent: one range per input, then Dup/Size Impacts, then PublicInput, CondSelect, ConstrainEq and the Popeq Impact.
- **C5 (fixtures superseded).** My illustrative `pair`/`derived`/`idle` fixtures and expectations N3–N5 are superseded by the root-approved oracle five-step specimen and are not to be implemented. N0 IR invariance is still unobserved, because no baseline capture exists. It remains review-only until a classified capture is made.
- **C6 (diff scope).** §6 expected the diff to be limited to the three relation files plus one test file. The live diff also contains:
  - the authorized mechanical `/1`→`/2` literal updates in `mori-ledger/tests/mapped_role_static.rs`, `mori/tests/fee_pool_component.rs` and `examples/core/fee-pool/README.md`;
  - the non-Opus `docs/NATIVE-CORE.md` section noted in §1.

  These must be disclosed to any GO decision.
- **C7 (new compatibility consequence).** `include_str!("mod.rs")` feeds both ordinary `compiler_hash` (`relation/mod.rs` `compile_bindings`) and `proving::source_closure()` (`proving.rs:154`). The test module's text is therefore part of the hashed closure, so any later test-only edit inside `mod.rs` refreshes ordinary manifest bytes and static bundle build identities. Root can avoid this by moving the tests to a separate file referenced by `#[cfg(test)] mod emission_tests;`. That needs a new path outside my write scope; I did not do it.

## 6. Compatibility, unchanged from the prior disposition and still explicit

- The mapped leaf format is now `/2`. Bundle profile `/1` and both qualifications are unchanged.
- Ordinary and signed manifest schemas are unchanged, but their bytes change because `compiler_hash` changes. Earlier ordinary artifacts now fail `admit` with `artifact-identity`.
- Every static bundle's `marker.build` changes, so old bundles fail reopen. Re-export is required.
- Anything keyed on these hashes, including native residuals, needs later requalification. That is not authorized and has not been performed.
- I updated no frozen hash literals and edited no historical evidence.

## 7. Pending security coverage (not implemented now)

- **H2 (kind is not in the IR) and H3 (`from` is not in the IR):** still needed as product-level observations. Variants should derive from the oracle five-step source, for example swapping steps 0 and 1 between Transfer and Fee, or changing step 4's payer. Their expectations must be independently frozen first. The pure-data test above records only that the validator does not bind kind or payer; it does not show the IR property.
- **H4 (tamper with recomputed hashes):** still required. The validator is producer-side self-consistency and never sees a caller-supplied map, because there is no Deserialize. Tamper refusal depends entirely on reopen's fresh-recompile equality (`fresh-source-mismatch`). H4 is the regression for that dependency.
- **H5 (ordinary unsigned path carries no map; `deny_unknown_fields` refuses an injected key):** still conditional on the `local_deployment::compile_all_policy` closure (prior §5.11).
- **Validator on real hostile emission:** a test that makes the compiler mis-emit (for example a group pushed outside `Builder::op`) needs compiler-side seams outside my write scope. The pure-data tests cover the validator logic only. **Missing seam:** a `#[cfg(test)]` injection point in `compiler.rs`/`arithmetic.rs`. It is not required now.
- **Command classification:** for the lib unit test target and Sol's exact-filtered integration test, together with the prior §5 closure items (event `serialize_to_node_list_bounded`/InMemoryDB, tagged codec, build scripts and third-party residual). All of this is still pending and Opus-owned.

## 8. Limitations of the map and validator (security interpretation)

- Emission provenance is not semantics. The validator does not establish group activity or guards, value equivalence, dependencies across statements (Let wires, claim aggregates), debit or payer binding, kind binding, custody, authorization, runtime correspondence or native correspondence.
- Non-Impact prologue instructions are bound only structurally, through check 5. Their content is not checked.
- The checks apply only to the mapped-role path. Ordinary and signed artifacts have no map and no new checks.
- A passing validator and a green test cannot close U0/U2 or any parent exit.
