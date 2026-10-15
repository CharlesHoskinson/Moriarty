# Opus 5.5 high-security audit: full candidate and actual results

**Candidate** `47cbd1ce69669320d465456695af257c3e9a2a8a` · **Base** `eb2577d6c67684b950288d769ea2fcb91e731bf1`

**Method.** I read the supplied plaintext only. I ran no tools. I make no claim about hashes, diffs or byte identity beyond what the supplied text shows.

## Verdict: APPROVE NARROW SCOPE

The child may merge with the qualifiers below and one non-security documentation correction (D1).

I found **no mandatory source or security defect**. This approval covers only:

- the `unsigned-mapped-roles` source-static `statement_emissions` metadata;
- its producer-side validator;
- the ten selected results, read as source-static provenance observations.

It does not approve the language, any native or semantic correspondence, or any parent exit.

## 1. Source integrity

**`arithmetic.rs` and `compiler.rs` are bookkeeping only, as read.**

- `Builder.op` records `DynamicField { group: groups.len(), offset }` before it pushes the group. The index therefore names the group being emitted, which is correct.
- Every Impact still comes only from `Builder.op`. Template inputs are always `field(..)` Immediates, so a `Variable` can enter an Impact only through a replacement. That makes the completeness invariant true by construction.
- `statement()` now returns the root `ExprId`.
- `compile_inner` snapshots instruction, group and dynamic counts around each statement.
- I found no change to instruction or group emission. The ordinary path (`compile`) and the signed path (`compile_signed`) discard the emissions.
- `Manifest` is unchanged and still `deny_unknown_fields`, so on source reading the ordinary and signed artifacts carry no map. This is the H5 property, established by reading only (see residuals).

**The validator in `relation/mod.rs` is pure and fails closed.** I checked it for panics and logic errors:

- Index safety comes from short-circuit order:
  - `impacts.len() == groups.len()` is checked before `impacts[..prologue]`.
  - `group_end == groups_before(instruction_end) ≤ groups.len()` is checked before `groups[f.group]`.
  - The range test on `f.group` precedes the `field_count` lookup.
- Partition and closure:
  - Statement 0 starts immediately after the last `abi-root` Impact.
  - Ranges are contiguous.
  - Group ranges are derived from instruction ranges.
  - The last entry ends exactly before the single trailing `Output`.
  - No `abi-root` group appears in the body.
- Replacement listing:
  - Fields are strictly ascending, which keeps `binary_search` valid.
  - Every Impact input `Variable` must appear in the list.
- `statement_root` is an exhaustive, wildcard-free duplicate of the compiler's match. A new `Statement` variant forces both matches to be updated.
- Cost is O(I + F log F) under the existing `compiled-work-bound`. No limit changed.

**Brittleness is fail-closed and acceptable.** The validator would refuse at export, publishing nothing, in these cases:

- the literal `"abi-root"` origin is renamed;
- a future op passes replacements in non-ascending or duplicate order;
- a prologue instruction is added after the guard.

The map has no `Deserialize`. The validator therefore checks only the producer's own consistency and cannot be fed caller data. Tamper resistance rests entirely on reopen's fresh-recompile byte equality, which H4 pins.

**Bounds and compatibility.**

- The mapped leaf moves to `/2`. Bundle profile `/1` and both qualifications are unchanged.
- Old bundles fail closed on `marker.build`, via `marker-identity`.
- Old ordinary artifacts fail closed on `compiler_hash`, via `artifact-identity`.
- The marker carries only leaf hashes, so map size does not affect the strict reader's depth or node limits for the marker.
- Larger manifests count against the caller's existing leaf and aggregate caps, which fail closed.

## 2. H2/H3/H4 expectations: not vacuous, not tuned

I re-derived the expectations from source independently of the timed-out author:

- The Transfer/Fee arm passes `kind` and `from` only to `record()`, which is a no-op when unsigned, and to `payments`, which only signed `append` consumes.
- `claim(asset, to, amount)` keys only on token and recipient.
- `debit_sources` is a `BTreeSet` of `from` names.

So kind-invariant and payer-invariant IR is the correct prediction for this compiler. It is not fitted to the outputs. The test code implements E2.1–E2.11, E3.1–E3.10, P4.1–P4.3 and E4.1–E4.3 as frozen, and I found no divergence.

- **H2/H3 are characterization tests of a limitation, not security properties.** Their real value is to record that in mapped mode:
  - Transfer and Fee are indistinguishable in IR;
  - the payer is absent from IR;
  - `debit_sources` survives per-statement payer reattribution unchanged (P2).

  Kind and per-statement payer exist only in the committed source, program and ABI.
- **H4 is the regression for the single tamper dependency.**
  - P4.1 shows the reseal is faithful.
  - Each refusal is therefore attributable to `fresh-source-mismatch`, after `marker-identity`, `action-leaf-identity` and `leaf-identity` all accepted the recomputed hashes.
  - Disk state is unchanged after each refusal, and restoring the original bytes is accepted.
  - The six mutations are semantically varied, but any byte change would be caught. I accept that as the intended design.

## 3. Normal oracle and IR results

The oracle's `source_evidence` cites a **pre-change** `compiler.rs` (`296da2a3…`). It also derives the widths from pinned SDK encoding source. It is therefore independent of the new bookkeeping. It is not independent of the compiler's design: it is a manual trace of that design.

What the selected results show:

- The decoded-IR test checks:
  - 58 Impact positions, widths and origins;
  - 52 literal guards (13 × 4 claims);
  - 13 replacement operands;
  - the five emission rows.
- Every check passed, so the producer's map agrees with the actual IR for this specimen.
- The baseline IR byte comparison is env-gated. `normal-cli-01/start.json` records `MORI_PROVENANCE_BASELINE_BINARY`. A wrong path would panic in `Command::output().unwrap()`, so a pass with the variable set means the comparison ran.
- Unit results show 5 run and 84 filtered. CLI security results show 3 run and 7 filtered. Normal results show 1 run with 9 or 8 filtered. All match the bounded classification.

## 4. Mandatory vs optional

**Mandatory source/security fixes: none.**

**D1 (documentation, non-security, do at merge or as an immediate follow-up).**
- *Problem:* `docs/NATIVE-CORE.md` still says "Local execution checks for this addition have not yet run". The evidence now records ten passing selected checks.
- *Fix:* replace the sentence with a pointer to the evidence README.
- *Risk:* it understates rather than overclaims, so it is not a security hazard.
- *Condition:* if root cannot confirm that `docs/` lies outside `proving::source_closure()`, make the change in a follow-up rather than altering frozen closure bytes.

**Optional notes.**
1. `ROADMAP.md` describes the same evidence twice ("Local tooling observation" second paragraph and "Compiler provenance observation"). Consolidate them.
2. H2/H3 messages such as "kind must not change IR bytes" read like required invariants. Label them as current-limitation characterizations, so a future fix that binds kind or payer in-circuit updates them deliberately instead of reading them as a regression.
3. Make the baseline branch fail loudly or print a marker when required, rather than skipping silently when the variable is absent.
4. C7: moving `emission_tests` out of `mod.rs` would stop test-only edits from churning `compiler_hash` and `marker.build`.
5. Downstream consumers must key on the leaf `format` (`/2`), not on bundle profile `/1` alone.

## 5. Retained qualifiers and residuals (not waived)

- **Prior corrections preserved:** C1–C7 from OPUS-SOURCE-ADDITIONS-DETAIL, including:
  - C2: listed Immediates are permitted;
  - C3: zero-group entries are conditional;
  - C7: closure-hash churn.
- **Unchanged-closure premise:** OPUS-COMMAND-BOUNDED, OPUS-COMMAND-REVIEW and SOURCE-REFERENCES are explicit premises. R1–R7 plus R8 remain recorded: build scripts, proc macros, crypto/storage internals, std/clap, linked-but-unreached code, feature selection and life-before-main constructors are uninspected.
- **Baseline premise:** C2 (the 95df→eb25 production projection is an empty diff) is root's claim, which I have not verified. The baseline binary's provenance rests on root's pin.
- **Coverage limits:**
  - Byte-identical IR is observed for one specimen only. General IR invariance rests on my source reading that emission is unchanged.
  - Temporary IR and bundle leaves were not persisted.
  - I claim no independent artifact digests.
- **H5 is review-only.** By source reading, ordinary and signed artifacts carry no map, but this was not executed.
- **Native requalification is unperformed.** The ordinary and signed schemas are unchanged, but their manifest bytes and build identities change. Old bundles need re-export.
- **What the map does not establish:**
  - It records emission provenance, not semantics.
  - It does not record Impact guards or activity, dependencies, value equivalence, or kind or payer binding.
  - It establishes no debit custody, authorization, runtime, native, ledger or target correspondence.
- **Other pending and unchanged items:**
  - The fresh Muse nonsecurity audit is still required.
  - The H2–H4 author timeout and missing canonical receipt remain on record. This audit is an independent judgment and does not substitute for that receipt.
  - Native authority is ZERO, with gross 4334 and remaining 0 unchanged.
  - All U0–U7, M1–M17 and MC/SP/G exits remain open.
  - No Fenrir identity or target tuple is supplied.
