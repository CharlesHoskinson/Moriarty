# Native numerical transfer: Grok 4.6 source/resource vote

Verdict: **approve-bounded**

This vote covers one local numerical native-proof experiment only. It does not approve signed-intent delivery, Preview, wallet work, ledger settlement, or financial gates.

## Reviewer identity

- Requested model: Grok 4.6 at high effort (`grok-4.6`)
- Returned model identity: Grok 4.6 (session identity: Grok 4.6 released by xAI)
- Requested effort: high
- Returned effort: high, as requested. This session did not expose a separate numeric effort receipt.
- Terminal status: review completed. No nested agents. No source edits, commits, wallet, network, or ignored native proof. Cheap MockProver/RowSizer diagnostics exited 0 (7 passed, 1 ignored). Verdict file written.
- Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930` branch `feat/signed-intent-runner-20261001` HEAD `2905cb6da0bf0ccfdca28b0f22478e0eda2420e3`
- Moriarty develop status at start: capability `SP01.6 loan-swap-subset`; `pendingTransactions` empty; implementation/repair blocked on operational-history/admission gaps. This read-only vote is independent of that blocked dispatch.

## Frozen hashes (verified)

| File | Claimed SHA-256 | Actual SHA-256 |
| --- | --- | --- |
| `experiments/midnight-crypto/src/financial_transfer.rs` | `f72e902b336582ded4b720efbfa362e30039d6f79c07d4b1165ef8317b692de4` | `f72e902b336582ded4b720efbfa362e30039d6f79c07d4b1165ef8317b692de4` |
| `experiments/midnight-crypto/tests/financial_transfer.rs` | `41e632534b815f984cd72bdb0f49153ed3602bfe95b34a8bb19bc61305406f6e` | `41e632534b815f984cd72bdb0f49153ed3602bfe95b34a8bb19bc61305406f6e` |

Both claimed hashes match the files in this worktree. The two files are untracked; the command will execute these working-tree bytes.

Additional runnable pins this reviewer hashed independently (not in the two-file freeze):

| File | SHA-256 |
| --- | --- |
| `experiments/midnight-crypto/fixtures/moriarty.json` | `0761254e1166e5ba3ec3f6059d9fa2540d54cedf4accfee7107ba9ecd73095d5` |
| `experiments/midnight-crypto/Cargo.toml` | `15dedfcb94a3d8ffe8290e30e4c9858e200b9a10682de3db759f93e402204388` |
| `experiments/midnight-crypto/Cargo.lock` | `53731faef893a205c4c96933845d72253e3b55277b3bdefc6cf7c355d8b46bab` |
| `experiments/midnight-crypto/src/lib.rs` | `396f0e86dcd72a9e4dfd7684df872b88c9f4415cab42a96779c11e3622a7d041` |

Root should record those extra pins with the command. They uniquely identify the binary the ignored test will load. This is a freeze-set addition, not a circuit edit.

## Exact command and limits

From the worktree root:

```sh
timeout 600s env CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/home/charl/research/moriarty-crypto-2026-09-30/target cargo test --offline --locked --manifest-path experiments/midnight-crypto/Cargo.toml --features native-proof --test financial_transfer native_transfer_proof_bounded_k10 -- --ignored --exact --nocapture
```

Limits this vote accepts, with root process-group enforcement:

- wall time of the entire process group <= 600s
- CPU time <= 600s
- memory <= 8GiB
- `CARGO_TARGET_DIR` artifacts <= 8GiB
- filesystem free >= 10GiB at launch; stop if the ceiling is approached
- disk monitored at 5s
- `CARGO_BUILD_JOBS=2`
- `--offline --locked`
- one native proof
- `k <= 10` asserted in-test before `ParamsKZG::unsafe_setup`
- no k increase, R3 retry, full suite, network, SRS download, or production setup
- ephemeral `unsafe_setup` test SRS only

Independent resource snapshot at review time: target `2181178956` bytes (~2.03 GiB) after diagnostics; filesystem free 26 GiB. Both sit inside the stated ceilings.

## Independent source observations

`TransferCircuit` is a nonrecursive PLONK relation over 28 named public numerical fields, version label `moriarty-transfer-numerical/1`, in-circuit version cell equal to 1. Eight private slacks sit at indices 28..35.

Range: every public cell and every slack has a 16-byte high-to-low decomposition of `Fq::to_repr()` bytes 15..=0, eight boolean bits per byte, accumulator starting at 0, base-256 recurrence, final equality to the assigned cell. Midnight `Fq` here is the BLS12-381 scalar field (`Engine::Fr = Fq`, modulus `0x73eda753...`). Values outside `0..2^128-1`, including `-1` and `u128::MAX+1` as field elements, cannot meet that equality.

Signed127: selector `signed_start` forces bit 7 of byte 15 (bit 127) to 0 on indices 7..=12: amount, fee, gross, gross cap, fee cap, net floor. Combined with UInt128 range this is `0 <= x < 2^127`.

Positive amount: slack 35 is amount-minus-one, UInt128-ranged, with gate `slack + 1 = amount`. Amount 0 cannot satisfy both the range and the equality.

Equalities, with every operand ranged, so modular wrap cannot hide overflow/underflow below the scalar modulus:

- gross = amount + fee
- sender_pre = sender_post + gross
- recipient_post = recipient_pre + amount
- fee_recipient_post = fee_recipient_pre + fee
- work_remaining_pre = work_remaining_post + 1
- work_spent_post = work_spent_pre + 1
- allowance_remaining_pre = allowance_remaining_post + gross
- allowance_spent_post = allowance_spent_pre + gross
- work_total = work_remaining_pre + work_spent_pre (UInt128)
- allowance_total = allowance_remaining_pre + allowance_spent_pre (UInt128)
- gross + slack0 = gross_cap, fee + slack1 = fee_cap, net_floor + slack2 = amount
- not_before + slack3 = round, round + slack4 = not_after
- effect_debit = gross, effect_recipient_credit = amount, effect_fee_credit = fee, effect_allowance_use = gross

Host slack hints use saturating add/sub. Gates and ranges, not the hints, decide satisfaction. Replacing any slack with 42 fails MockProver.

`fixture_public_values` is an off-circuit projection. It requires Core5 Transfer, three distinct ordered accounts, canonical decimal strings, one owner allowance, and Core5 effect order. Fee row: 6 effects including fee credit. Zero-fee row: 5 effects, omitted fee-credit line, numerical fee-credit slot 0, fee balance unchanged. Work updates are state deltas, not invented Core5 effects. This adapter is not a source-hash, identity, or compiler correspondence proof.

Inspected ignored test `native_transfer_proof_bounded_k10`:

- `RowSizer::min_k` then `assert!(k <= 10)` before SRS
- `ParamsKZG::<Bls12>::unsafe_setup(k, OsRng)`
- `keygen_vk_with_k` on `without_witnesses()` at that k, then `keygen_pk`
- one `create_proof` on the fee fixture (`fixtures[0]`)
- `prepare(pk.get_vk(), &[], &[public], &mut reader)` (proofs 0.8 default feature `committed-instances`)
- `reader.assert_empty()` then `guard.verify(&params.verifier_params())`
- each of 28 public fields mutated by 1 must refuse
- first-byte XOR, half truncation, and a trailing 0 byte must refuse

The ignored test does not emit an invalid-witness proof and does not prove the zero-fee fixture. `midnight-proofs` `prepare` does not call `assert_empty`; the test's explicit EOF check is the trailing-byte control. The crate comment that prepare errors on trailing bytes is not implemented in that function.

Old `tests/native_proof.rs` context: k=6 field equality `10000 = 8990 + 1010`, same 0.8 `create_proof`/`prepare`/`Guard.verify`/`unsafe_setup` pattern, plus an invalid-witness emitted proof under `debug-assertions = false`. The new test reuses that API, adds k bound and EOF, and drops invalid-witness proving. The proposed command uses `--test financial_transfer`, so the old smoke is not executed.

Pins: this crate's `native-proof` feature uses git `midnight-zk` `0ededef0e605701fc5139ebdcf011b11f3d86ba7` (`midnight-proofs` 0.8.0, `midnight-curves` 0.3.0). Ledger crates at `9f9842ebed66cdff0f54d3fb09efc6a7cd077ed8` still resolve `midnight-proofs` 0.7.3 / `midnight-curves` 0.2.1. `lib.rs` exports `financial_transfer` only under `native-proof`.

## Independent constraint diagnostics (not the ignored proof)

Command used:

```sh
env CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/home/charl/research/moriarty-crypto-2026-09-30/target cargo test --offline --locked --manifest-path experiments/midnight-crypto/Cargo.toml --features native-proof --test financial_transfer -- --nocapture
```

Result, observed here: compile 0.96s against the existing target; `test result: ok. 7 passed; 0 failed; 1 ignored`; wall 1.11s for the test binary. Printed RowSizer line: `numerical circuit RowSizer k=10, domain rows=1024; constraint diagnostics only, no SRS/proof`. The ignored native test was compiled (0.8 API type-checks) and not run.

Passing tests: frozen fee and zero-fee MockProver, all 28 public-instance mutations, invalid economics, integer boundaries including `1`, `2^64±1`, `2^127-1` accept and `2^127` reject, sender-style out-of-UInt128 and `-1` witnesses, hostile slacks, row budget.

## Numbered findings

1. Severity: medium (process). The two frozen Rust hashes do not uniquely pin the runnable experiment. The ignored test `include_str!`s `fixtures/moriarty.json`, and Cargo plus `lib.rs` select proofs 0.8 and the feature export. Root should freeze the four extra hashes above with the command. No circuit change required.

2. Severity: limitation (high if overclaimed). A passing local proof is not importable by the Preview/ledger verifier. Proofs 0.8.0/curves 0.3.0 versus ledger proofs 0.7.3/curves 0.2.1 remains unproved. Keep NO-GO for signed financial and Preview acceptance.

3. Severity: limitation (in-scope exclusion). Native invalid-witness emitted-proof refusal and zero-fee native proving are absent. MockProver covers those numerical cases. They remain unspecified for real KZG.

4. Severity: limitation. The test does not monitor disk, RSS, or CPU. Root must enforce the 600s/8GiB/8GiB/10GiB process-group ceilings. A proof failure or timeout is a retained failure. No k increase.

5. Severity: low. `check` collapses prepare error, EOF failure, and pairing failure into `false`. Enough for this bounded experiment. It does not produce a classified refusal receipt.

6. Severity: limitation. `unsafe_setup` with `OsRng` is toxic-waste test SRS, non-reproducible, and not production setup evidence. The `midnight-proofs` test profile `debug-assertions = false` is unused by this valid-witness test.

7. Severity: limitation. Off-circuit adapter checks account/effect identities and Core5 ordering. The circuit proves numerical claims only. Owner signatures, source hash, authentication, replay, head, asset identity, and ledger transfer stay open.

None of these is a blocker for the bounded local command as written.

## Independent expected outcomes

Success, all required:

- cargo builds `--offline --locked` with 2 jobs into the stated target
- RowSizer k is 10 (already observed) and the assertion `k <= 10` passes before SRS
- one ephemeral unsafe SRS of size `2^k` with k<=10
- one fee-transfer proof for public numbers: version 1, sender 10000 -> 8990, recipient 0 -> 1000, fee recipient 0 -> 10, amount 1000, fee 10, gross 1010, caps 1010/10, net floor 1000, round 1 in [0,10], work 10 -> 9 and 0 -> 1, allowance 10000 -> 8990 and 0 -> 1010, effects 1010/1000/10/1010
- original instances pass `prepare` + `assert_empty` + `Guard.verify`
- each of 28 public +1 mutations refuses
- XOR of proof byte 0 refuses
- truncated half-proof refuses
- proof plus trailing 0 refuses
- stdout contains proof byte length and k, with no signature/hash/registry/Preview claim
- process exits 0 inside 600s, RSS <= 8GiB, target <= 8GiB, free >= 10GiB
- no second proof, no network, no SRS download

Fail-closed outcomes (retain, do not retry, do not raise k):

- compile or lock/offline failure
- `k <= 10` assertion failure
- keygen, prove, or pairing failure
- valid proof rejected by `assert_empty` (unexpected leftover bytes)
- wall/CPU/memory/disk ceiling
- any attempt to continue into invalid-witness native proving, zero-fee native proving, R3, or Preview

Independent resource expectation: k=10, 45 advice columns, domain 1024. Compile against the warm target was under 1s here. Proving at k=10 should be far below 600s and 8GiB. This is an expectation, not a measured native-proof RSS.

## Limitations

- MockProver passing does not establish native soundness, pairing, or SRS honesty.
- This vote does not run, and has not run, the ignored proof.
- Fixture projection is not in-circuit source correspondence.
- Range gadgets use canonical `to_repr()` bytes. That is the correct UInt128 embedding for this scalar field; it is not a general field-to-integer proof for values near the modulus.
- Conserved post work/allowance totals follow from the paired ±1 and ±gross gates. There is no separate post-total gate.
- Work always decrements by one. That is the stated precursor, not a general work-cost model.
- Historical R3 k=17 failure is a different encoding. This k=10 relation does not revive that campaign.

## Scope this vote does not close

Identities, source hash, authentication, replay, head, ledger transfer, Preview, owner signatures, asset identity, duty/repayment semantics, PCD, F0 ledger-verifier route, and the rest of signed-intent delivery. Whole-delivery result audits remain owed later.

## Vote

**approve-bounded**: GO for the exact local command on the verified hashes and limits above. NO-GO for signed financial proof, Preview, or closing financial gates.
