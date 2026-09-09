# RESULT AUDIT — repayment-k-branches-2026-09-09

**Verdict: PASS** (scoped). No material blockers found. Publication is gated only on the still-pending result reviews and the not-yet-existing `acceptance.json`.

Basis: supplied text only. I did not hash files or run anything; byte/hash verification is delegated to the fresh GPT-6 pass.

## Execution packet

- Supervisor argv is the admitted contained form: `systemd-run --user --unit=… --wait --pipe`, `MemoryMax=4G`, `MemorySwapMax=0`, `RuntimeMaxSec=512`, `KillMode=control-group`, `KillSignal=SIGKILL`, `TimeoutStopSec=5`, `MemoryAccounting=yes`, worktree cwd, `flock --nonblock`, then `run.py --suite branches compile-and-traces --all`. `returncode: 0`.
- Active-service snapshot corroborates the *executed* unit: `MemoryMax=4294967296`, `MemorySwapMax=0`, `RuntimeMaxUSec=8min 32s` (=512 s), `KillMode=control-group`, `KillSignal=9`, `MemoryPeak=739315712` (705.06 MiB), `MemorySwapPeak=0`.
- Post-service query (`RuntimeMaxUSec=infinity`, `MemoryMax=infinity`, `KillSignal=15`, `[not set]`) is *not* executed-unit state; unit was collected. README says exactly this and retains both captures. Correct handling.
- Timing coherent: compile 5.545 s; trace-16 `aggregateSeconds` 43.456 < service 43.508 s (stderr) < supervisor 43.529 s (`execution-supervisor.json` = `aggregateSupervisorSeconds`). CPU 1 m 36 s > wall is consistent with parallel kompile/krun. Peak 705 M ≪ 4 GiB; zero swap. All 16 per-trace `elapsedSeconds` in `execution-result.json` match the corresponding `trace-NN.command.json`; all `returncode 0`, `timedOut false`.
- Charges: admission preflight 3 compiles / 17 krun + this 1 / 16 = 4 / 33, matching `execution-result.json` and README, which states inclusion of earlier failures and the repeated smoke result and no reset/refund. Consistent.

## Raw outputs vs. claims (all 16 re-derived by hand)

Every `trace-NN.stdout` is KAST v4 with a single `<out>`; `prepared` arity 16, `rejected` arity 3.

- Successes (5): T01 `[56,19,17]`, receiver `-1` → append Lender 17, unrelated Custodian row preserved at index 1. T02 `[28,56]`, receiver `0`, append field `0` — order preserved, no sort. T03 `[56,28]`, receiver `1`, payer Sponsor, obligation debtor stays Payer. T04 `[19,56,17]`, receiver `-1`, sender at index 1, unrelated index 0 untouched. T05 PrincipalFirst: `65/19`, allowance `53/15`, P 0 / A 7 / O 7, settlement 8, dP 5, dA 3.
- All four AccrualFirst successes: allowance 44/24 (sum 68 preserved), work 7/6, reserve 13 unchanged (sum 26), P 42 / A 0 / O 42, status `Outstanding`, settlement 17, dP 8, dA 9. Matches K's `principalPart`/`statusOf` rules, `branches.json` expectations, `source-observations.json`, and INDEPENDENT-CASES arithmetic — field for field, including balance order and appended-row position.
- Rejections (11): `MISSING_BALANCE/0`, `MISSING_ALLOWANCE/0`, `SELF_TRANSFER/0`, `DUPLICATE/-1`, `MISSING_OBLIGATION/1`, `TRANSFER_MISMATCH/1` ×2, `ZERO_AMOUNT/1`, `NOT_OUTSTANDING/1`, `DUPLICATE/-1`, `OVERFLOW/0`. Each is the *first* failing guard under `moriarty.k`'s 21-check order; I verified precedence individually, including the two designed cases: duplicate rows beat exhausted work (`DUPLICATE` at guard 1 vs `INSUFFICIENT_WORK` at 5), and receiver overflow (guard 14, index 0) beats zero repay (guard 15, index 1).
- Failure carries no state: the K abort rule drops the continuation including `finish`; raw rejected terms carry only digest/code/index; codec returns `{status, code, actionIndex}` only. Claim verified at the raw-output level, not merely asserted.
- `rawOutputSha256` / `inputDigest` values in `execution-result.json` are mutually consistent with the manifest entries and with the digests echoed inside each raw output; the digest↔input binding itself needs hashing (GPT-6 scope).

## Metadata vs. K-computed values

The distinction is correctly drawn. K emits exactly 13 integers (args 1–10, 12–14), one receiver index, one status, one digest. Decoded metadata (obligation id/debtor/creditor/denomination/settlementAsset/rule/conversion, allowance owner/asset, used-ID appends, Transfer effect, Repayment identity fields) is reconstructed by `codec.py` from the input, not computed by K — README states this and calls the codec trusted-unproved. The `"thirteen"` count is right; the earlier Opus `"fifteen numeric outputs"` phrase was wrong, and the correction is in README with the original report left unedited, as required.

## Suite selector / default preservation

`SUITES` maps `initial→cases.json` (default) and `branches→branches.json`; `sources()` includes the selected fixture, so fixture bytes enter `binding.json`, and `evaluate()` fails `COMPILED_STALE` before any krun on cross-suite use. `check-k-results.py` asserts `branches.json` present and `cases.json` absent in the binding, and `krunInvocations == 16`. The 16-call ceiling (`count>=16 → KRUN_LIMIT`) and single-compile lock (`compile-attempt.json` `open('x')`) are intact. Claims accurate.

## Limits (documented, non-blocking)

1. `binding.json`, `observations.json`, `check-source-cases.mjs`, `trace-NN.input`, and `compile-attempt.json` contents were not supplied — so `compiledArtifactsUnchanged: 174`, input-file↔fixture binding, and the *provenance* of `source-observations.json` results (that they came from executing the real `.mori` parser/preparation rather than transcription) are not verifiable here. The three-way equality itself is machine-asserted in `check-k-results.py` and matches what I re-derived from raw outputs.
2. Independence of the expectations is an author attestation (INDEPENDENT-CASES "specified-only"), not a mechanically checkable property; the frozen fixture hash is consistent across manifest, freeze table, and `sourceBinding`.
3. `check-k-results.py` prefers a live `.build` over retained `attempt-01/` unless `--retained`; a stale local branches build would be checked instead of the receipts. README documents the two modes; worth preferring `--retained` in audits.
4. "32 distinct inputs" spans two suites; distinctness across suites is not re-checked here.
5. `execution-result.json` and README defer disposition to `acceptance.json`, which is **not** in the candidate manifest. SP03.md's follow-up prose ("Final result reviews are recorded in the follow-up deliverable acceptance record") reads as if that record exists, while its checkbox is correctly unchecked. Tighten to "will be recorded" before publication.

## Scope discipline

No full SP03, correspondence theorem, proof, native execution, or ledger-acceptance claim appears anywhere in the reviewed text; `prove` still hard-fails `PROOF_UNIMPLEMENTED`. No new financial or on-chain transaction is evidenced — the packet is a local, contained run. Counts, seconds, memory, and charges are stated with their qualifiers.
