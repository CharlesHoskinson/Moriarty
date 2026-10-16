# Fresh non-author review: arithmetic-consumer-01, actual results

**Basis.** I reviewed only the supplied text. I ran no tools or commands, made no edits, delegated nothing and recomputed no SHA-256. Hashes and path metadata are as supplied. I did not author any reviewed file, runner, CONFIG or prior report.

**Identity.** I am Opus 5.5 (`claude-opus-5-5`), reporting on myself. I cannot see provider-side returned-identity metadata, so Root must record that from the provider. My independence from the runner/CONFIG author (also Opus) is at the session level only, not the model level.

## 1. Source equations, checked by hand

**ADD.** Byte `i` computes `a_i + b_i + c_i = r_i + 256·c_{i+1}`, with `c_0 = 0` and `ovf = c_16`. The u16 headroom is ≤ 511.

**SUB/LT.** The borrow is `bo_{i+1} = [a_i < b_i + bo_i]`. The expression `a + 256·bo_{i+1} − (b + bo_i)` cannot underflow, because the subtrahend is at most 256. The result always lands in 0..=255. `ovf` is gated by `sel_sub` and `lt_out` by `sel_lt`, both on `bo_16`.

**MUL.** All 32 diagonals are computed, including `S_31 = 0`. Accumulation is in u32 and there is no u128 product. The carry is stored as u16.

I proved the owner M-6 closed form against the recurrence, independently of the test:
- For k ≤ 15: `S_k + c_k = (k+1)·65025 + 255k − 1 = 256·(255k + 254)`. So `c_{k+1} = 255(k+1) − 1` and the digit is 0.
- For k ≥ 16: `S_k + c_k = 256·255·(31−k) + 255` (k=16 checked directly). So `c_{k+1} = 255(32−(k+1))` and the digit is 255, except `h_0 = 254` at k=16.
- This gives `r = 1`, `h = [254, 255×15]`, `c_16 = 4079`, `c_32 = 0` and `h_sum = 4079`, all matching what was asserted.

**FUEL.** The code computes `charged = en_evt ∧ ¬term ∧ ¬zp_16`. The decrement `pre + 256·bw_{i+1} − bw_i` cannot underflow: when `bw_i` is set and `z_i` is clear, `pre_i ≥ 1`. When charged, `bw_16` is always false.

**Normal vectors.** I traced every literal case through the code: A1–A5, S1–S4, L1–L5, M1–M6, T1–T4, P1 and F1–F8. Notable intermediate values:
- L4: `[01, FF×14, 00]` with `bo = chain(1,15)`.
- L5: `[FF, 0…, FF]` with `bo_16` set.
- F2: post `[FF, FF, 0…]` with `bw = chain(0,2)`.

## 2. Arithmetic wording reconciliation (979454 vs 1044224)

The figure 979,454 is the **k=16** diagonal total for M-6: `S_16 + c_16 = 975,375 + 4,079`. It is not the maximum.

The maximum is at **k=15**: `S_15 + c_15 = 1,040,400 + 3,824 = 1,044,224 = 4079·256`. That matches the owner M-6 figure.

This is the global maximum over all inputs. MAX×MAX maximizes every `S_j`, each carry is monotone in the earlier diagonal sums, and the neighbouring diagonals are smaller (k=14 gives 978,944; k=16 gives 979,454).

| Quantity | Value | Bound |
|---|---|---|
| Max diagonal total | 1,044,224 | < 2²¹, far below u32 |
| Max carry | 4,079 | < 2¹² (u16 and the 12-bit design both hold) |
| Max `h_sum` | 4,080 (16·255) | fits u16 |

The current code comment ("sum ≤ 1,040,400; u32 also holds its carry") is accurate. The earlier "max 979454" wording is wrong. It affects no code or executed claim, and it should not be carried forward.

## 3. Consumer flow (`layout_a/arithmetic_fuel.rs`)

**Decode first.** `decode_observed_a(Step)` runs before any row is read, and any refusal propagates. After `compare_program_block`, the supplied block equals the prepared block row for row, so fetching from `image.program` is sound. The `fetch` refusal cannot be reached after a successful decode, so it is defensive only.

**Derived values, all from the decoded rows:**
- `en_evt`: Pending and nonempty control.
- CR0 and FS0: the head rows, matching the encoder's head-first and top-first order.
- `term`: Return at depth 0, or Error.
- `en_bin`: charged, Return, and FS0 tag 2.
- Operation map: 0, 1, 2, 4 only.
- Operands: FS0 and CR0 `valueRef`, with `uu` requiring `type_id == 0` on both sides. The decoder ties `type_id 0` to `kind 0`.

No caller-supplied guard, operation, operand or charge exists in the API.

**Consumer tests traced through the encoder layout** (Constants[0]=a, Constants[1]=b, Data[0]=b, Data[1]=a):
- P1: constant left operand and Data[2] right operand, result 12, fuel 3→2.
- A3, S3, L4 and M6 at fuel 1.
- T1 and T2 (Nil re-pointed to Constants[0]), and T3/T4.
- Both fuel loops. O1 holds against `initial = u128::MAX`, and depth-0 frames are zeroed so padding stays valid.

## 4. Actual results versus the receipt

- **Compiler.** `rustc-version.stdout` is release 1.99.0, commit `b940084d7…`. I hand-counted it at 196 bytes, matching the receipt.
- **Compile.** Exit 0 in 0.466 s. stderr holds exactly 5 `dead_code` warnings, all in `reference_types.rs`. There are none in the four new leaves.
  - The warnings confirm that only `Expr::Literal` and `Expr::Binary` are constructed. The other prepare/decode paths are compiled but not exercised.
- **Dep-info.** It lists exactly the 15 closure files, all under `attempt-01/source/`, in module declaration order. It matches the receipt's `compile_inputs` and the CONFIG closure.
- **Tests.** stdout shows `running 9 tests` and 9 `ok` names identical to CONFIG `expected_tests`. The summary is `9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`. The run took 0.004 s.
- **Bindings.**
  - The manifest SHA `5639b21a…` (3857 bytes) is in the receipt `reviewed_manifest`.
  - Execution-preflight report hashes and sizes match between the receipt and the actual-result manifest.
  - Log, dep-info and stderr hashes and sizes match between the receipt and the manifest. The empty files carry the SHA-256 of empty input.
  - `reviewed_candidate_after` equals the pre-run values, and every source has before = snapshot-after = checkout-after.
  - The interpreter and compiler hashes are unchanged after the run.
  - Both postflights are `ok`, and the terminal state is `success`.
- **Leaf bytes.** The four leaf hashes in LEAF-SOURCE-MANIFEST equal the source, receipt and after values. The two witness leaves are byte-identical to those run in `arithmetic-witness-01`, so there are **9 distinct current tests, not 14**. The prior 3-file/5-test run is retained separately.

## 5. Non-blocking observations and limits

1. **Report byte counts.** `report_bytes` in RESULT.json differs from the actual REPORT.md bytes by 1–20: 5670/5690, 19543/19544, 5199/5206 and 9449/9467. This is consistent with counting code points rather than bytes (§, →, —, ×, −, en-dashes) plus a trailing newline. It is provider metadata only; the binding uses the actual file bytes. Label it as a character count.
2. **Untested fuel paths.** The `en_evt = false` path and non-Pending outcomes are never executed. All fuel cases use Pending.
3. **Untested control shapes.** No test has a Push at CR0 or depth > 1.
4. **Header lanes.** T1/T2 do not assert the header lanes. The claims are already narrowed to say so.
5. **Duplicate test calls.** The witness T-3/T-4 loop makes the same call twice.
6. **Entry id.** `prepare_a` never sets `entry_expr_id`. It is correct only because the entry is occurrence 0, and this is pre-existing.

The following remain outside this result:
- Canonical mode, which is compiled but not exercised.
- Transitions, reachability, appendData, aux/2 packing, ABI, native binding, constraints and certification.
- Product `mod.rs`/`lib.rs` and full-crate compilation.
- Hostile, N1–N16 and refusal cases.

The recorded containment residuals still stand: no memory, disk or process bound; 0444 is not a same-UID boundary; the sysroot, linker, libc, Python stdlib and kernel are unhashed. For this narrow honest-host result I am adding no new security policy and no hostile case.

**Dissent.** I dissent only from the earlier "mul total max 979454" wording (§2). I have no other disagreement with the source-preflight or execution-preflight reports.

**Scope of this approval.** I approve only this: in the standalone 15-file rustc 1.99 compile, the private callable decoded A program/state consumer produced honest numeric and fuel outputs matching the owner normal specimens across exactly 9 passing tests, with the stated pins. Native authority stays 0, C4 is unchanged, and all G4, IO1–8 and roadmap exits remain open.

VERDICT: APPROVE_WITH_SCOPE
ACTUAL_RESULT_MANIFEST_SHA256: 5c10e513f219ce5ed3b9b2948437c630f24a95eb5d645f3e8057b34735a60c84
