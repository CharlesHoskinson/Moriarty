# Independent Audit — SP03 Repayment Branch Suite (`sp03-repayment-branches-01`)

**Scope of this audit:** the exact text supplied in the packet. I did not read files, verify SHA-256 values, or execute K, the source path, or the evaluator. No claim is made about PDFs, stage promotion, proof, source/K correspondence, or ledger acceptance.

## Verdict: **PASS**

### Expected results are complete, not patches
All five `Prepared` records carry full `status`, `schemaVersion`, complete `post` (balances in exact array order, allowance, obligation with all metadata and conversion record, both used-ID lists, work triple) and exactly two ordered effects. All eleven `Rejected` records are exactly `{status, code, actionIndex}` with no `post`/`effects`, matching `moriarty.k`'s `ensure` rule, which replaces the entire remaining `~>` continuation with `rejected(...)` on first false guard. First-failure-returns-no-state holds structurally, not by convention.

### Guard ordering — each rejection re-derived against `moriarty.k`
K order is: `DUPLICATE` → three `INVARIANT` → `INSUFFICIENT_WORK` → work `OVERFLOW` → idx0 `ZERO_AMOUNT`, `SELF_TRANSFER`, `MISSING_BALANCE`, `INSUFFICIENT_BALANCE`, `MISSING_ALLOWANCE`, `INSUFFICIENT_ALLOWANCE`, allowance `OVERFLOW`, credit `OVERFLOW` → idx1 `ZERO_AMOUNT`, `MISSING_OBLIGATION`, `NOT_OUTSTANDING`, `EXCEEDS_OUTSTANDING`, `TRANSFER_NOT_IN_STEP`, `TRANSFER_MISMATCH`, `INSUFFICIENT_UNALLOCATED`.

Each of the eleven expectations lands on the first failing guard: `MISSING_BALANCE`/0 (`indexOf` yields −1 before the allowance check); `MISSING_ALLOWANCE`/0 (sender exists and covers 17, so earlier guards pass); `SELF_TRANSFER`/0 (precedes the balance and later mismatch guards); `DUPLICATE`/null ×2; `MISSING_OBLIGATION`/1; `TRANSFER_MISMATCH`/1 for both recipient (`TO ≠ CREDITOR`) and asset (`TA ≠ OA`, with an exact funded `(Payer, Token)` row and allowance so no earlier idx0 guard fires); `ZERO_AMOUNT`/1; `NOT_OUTSTANDING`/1 (the settled input satisfies `P+I==O` and the status/zero pairing, so `INVARIANT` passes and `NOT_OUTSTANDING` precedes `EXCEEDS_OUTSTANDING`).

The two ordering-discriminating cases are correct and non-trivial: `reject-duplicate-before-work` (WR=0 with `0+4+13` invariant-clean, `DUPLICATE` precedes `INSUFFICIENT_WORK`) and `reject-overflow-before-zero-repay` (max receiver + N=0; credit `max+17` fails at idx0 before the idx1 positivity guard). All codes are members of `codec.REJECTIONS` for their index; `-1` decodes to `actionIndex: null`.

### Prepared arithmetic
`indexOf`/`moved`/`appended` reproduce all five: append case (SI=0, RI=−1 → 56/19 + Lender 17), swapped rows (SI=1, RI=0 → 28/56, no sort or append), third-party (`FROM==PAYER`, `TO==CREDITOR` satisfied with debtor ≠ payer), row1+missing receiver (SI=1, RI=−1), and PrincipalFirst (`principalPart=min(8,5)=5`, P0/A7/O7, allowance 53/15, settlement 8). `prepared` argument positions map correctly onto the decoder (`nums[12]`=settlement, `[13]`=dP, `[14]`=dA, `[10]`=remainingOutstanding, `args[11]`=status, `args[15]`=receiver index), and the decoder's `expected_receiver` and `nums[3]=='0'` cross-checks agree with every fixture. `statusOf` yields `Outstanding` in all five (residuals 42 and 7). Every input satisfies `codec.admit`'s closed projection (2/1/1 rows, empty used-ID lists, `Transfer` then `Repay`, non-ProRata, identity conversion, 39-digit max amount within bounds).

### Runner controls
- **Suite binding cannot be reused:** `sources()` hashes `SUITES[SUITE]`, so an `initial` binding fails `COMPILED_STALE` in `evaluate` *before* dispatch; `test_runner.py` asserts `command` uncalled and `krunInvocations` unchanged at 0.
- **Default preserved:** `--suite` defaults to `initial`; `SUITES` is a closed choice set.
- **Calls capped:** `count>=16` → `KRUN_LIMIT`, incremented and persisted *before* subprocess dispatch, so failures never refund. Sixteen fixtures consume exactly the 16 allocated invocations; `FIXTURE_COUNT` rejects >16.
- **Stop rule:** `traces()` raises `EXPECTED_RESULT_MISMATCH` on the first non-match; `main` converts to `HarnessError`, exit 2, no retry, no backend fallback. Compile is single-shot via exclusive `compile-attempt.json` create.
- **argv:** `--suite` correctly precedes the subcommand (argparse top-level option), `--all` is required, `prove` is excluded and fails closed.

## Resource vote: **YES** — on the exact allocation as written

One compile attempt @180 s, 16 krun @20 s, 512 s aggregate, whole-tree 4 GiB, `MemorySwapMax=0`, `KillSignal=SIGKILL` + `KillMode=control-group` (immediate cgroup kill, no TERM grace), non-blocking `flock`, no automatic retry, no reset or refund of the historical 3 compiles / 17 runs. Containment matches the previously accepted K experiment pattern; the internal `DEADLINE=START+512` and per-command `min(seconds, remaining)` clamp sit inside the systemd ceiling.

## Material notes (not blockers, no new gates)

1. **Aggregate headroom is thin:** worst case 180 + 16×20 = 500 s against a 512 s ceiling. A slow compile plus slow traces terminates via `AGGREGATE_DEADLINE` or SIGKILL, consuming the compile attempt with no refund. This is the accepted cost under the stated stop rule; I am not asking for a larger budget.
2. **Per-build counter semantics:** the 16-run cap is bound to the fresh `.build/binding.json`, not to the historical totals, which are bookkeeping only. This is sound solely because the worktree has no `.build`; root's pre-dispatch check that no `.build` exists is load-bearing.
3. **Metadata preservation is decoder-guaranteed:** `codec.decode` rebuilds `post` from a deepcopy of the input, so the preserved obligation/allowance/used-ID fields in the expectations exercise the codec (separately unit-tested, 8 green) rather than K. The branch suite's genuine K coverage is the guard selection and the fifteen numeric outputs. Worth stating plainly in any evidence write-up.
4. **Evidence not supplied in-packet:** `runner-red.txt` and `source-observations.json` appear only as hashes; I audited neither. `source-check.txt` reports 16 matches but is a claim, not verified here.
5. **Not blind:** root already ran the source path green on these exact expectations, so a K mismatch would now read as K/codec-vs-source divergence rather than an expectation error. Authorship order (specified-only freeze before any execution) is asserted in `INDEPENDENT-CASES.md` and is the basis for treating this as independent.
6. **Boundary respected:** this is a root-supervised SP03 experiment under the established SP03 contract, not a dispatch or unblock of the registered SP01 loan action (history unresolved), and carries no stage/proof/correspondence/Preview acceptance. This is one of the two substantive votes required; root admission checks remain required before invocation.
