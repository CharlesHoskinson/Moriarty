# Independent Audit — SP03.2 Transfer-only provisional K extension

**Verdict: PASS**

## Scope check

The packet stays inside its stated boundary. `run.py` maps `prove` to `PROOF_UNIMPLEMENTED`; argv contains no network, install, kprove, native-proof, or submission step; the working directory is a local worktree; the fixture suite is selected explicitly. The claim set in `resource-proposal.json` ("no full stage admission, proof, native or public execution") matches the artifacts. The plugin boundary statement is accurate: nothing in argv invokes the blocked SP01 registered loan action, and this is a root-supervised local experiment under the existing SP03 contract, not a dispatch of old history or a full-stage promotion. No new infrastructure gates or preference demands appear.

## Semantics vs. financial spec

Guard count in `moriarty.k` `transferPacket`: DUPLICATE, three INVARIANT, INSUFFICIENT_WORK(≥1), OVERFLOW(work spent), ZERO_AMOUNT, SELF_TRANSFER, MISSING_BALANCE, INSUFFICIENT_BALANCE, MISSING_ALLOWANCE, INSUFFICIENT_ALLOWANCE, OVERFLOW(allowance spent), OVERFLOW(receiver) = **14**, versus 21 on the retained two-action path. Matches the packet and the README table.

Against `repayment-kernel.md`: work cost is derived from the action list (one unit), `closureReserve` is untouched and is not ordinary work, debt/allocation history are structurally untouchable on this path (`preparedTransfer` emits no obligation or allocation fields, and the codec deep-copies the input obligation), one Transfer effect is emitted, and the spec's explicit "Transfer-only candidate is legal: cash moves, debt does not" is exactly what is implemented. The existing repayment rules are byte-identical in role — the `packet`/`prepared` rules are unchanged and the shared helpers (`indexOf`, `at`, `moved`, `appended`, `principalPart`, `statusOf`) are reused, not forked.

Codec admission retains the 2 balances / 1 allowance / 1 obligation / empty used-ID / identity-conversion / non-ProRata restriction and extends the action-shape whitelist to `['Transfer']` only. Encoding emits a distinct `transferPacket(` head; `decode` demands `preparedTransfer` with arity 9 and re-derives the receiver index independently from the input, rejecting on `K_OUTPUT_BINDING` mismatch. Digest binding is checked before any field is read. All changed numbers still come only from K.

## Material input/output verification

I re-derived all sixteen expectations by hand against the rules:

- Four successes (`one-work-debt-untouched`, `missing-recipient-append`, `swapped-third-party-non-creditor`, `settled-debt-untouched`) all reduce correctly, including the row-swapped case (SI=1, RI=0 → 31/64) and the append case (`appended(-1,19)=19` with `nums[3]` guard satisfied). Obligation, allocation rule, conversion, denomination and `usedAllocationIds` are unchanged in all four.
- Six rejections land on the correct first-failing guard: `reserve-not-work` → INSUFFICIENT_WORK(−1); `invariant-before-work` → INVARIANT(−1) precedes the work guard, as ordered in the rule; `zero-amount`, `receiver-overflow` (M+19 > UInt128 max, reached only after the sender/allowance guards pass), `missing-exact-allowance`, `insufficient-balance` → index 0. All codes are in the codec's `REJECTIONS[idx]` sets.
- Six regressions reproduce their retained values exactly (AccrualFirst 50/9 N17 → 8/9 split, PrincipalFirst 100/10 N7 → 93/10, TRANSFER_NOT_IN_STEP and TRANSFER_MISMATCH at index 1, DUPLICATE at −1).

The `transfer-only` case at work remaining 1 genuinely discriminates one-unit from two-unit cost. Independence is credible: expectations were derived from the kernel and funded-source specs, the quantities are not reused from any repayment result, and `check-source-cases.mjs` reaches all sixteen through the real parser/elaborator/evaluator (`source-check.txt`: 16/16) without touching K. Codec suite is green (12 tests). `test_transfer.py` correctly labels its synthetic KAST as not establishing K execution.

## FH control explanation

The reduction presentation is faithful: `E ::= □ | E ▷ k` with no `k ▷ E` form correctly models K's inability to skip a pending guard; ABORT matches the `ensure(H,false,…) ~> _REST => .K` context-erasing rule; step counts check out (1+1+14+1 = 17 Transfer-only, 24 two-action, j+2 on failure at guard j). Felleisen–Hieb metatheorems are explicitly disclaimed as not applying to Moriarty, and no correspondence, freeze, proof, or settlement claim is made.

## Material blockers

None.

## Non-blocking observations

1. **Budget is tight, not unsafe.** 180 s compile + 16 × 20 s = 500 s against a 512 s inner deadline and a 512 s outer `RuntimeMaxSec` that starts marginally earlier. A slow compile plus slow traces could be SIGKILLed at the outer bound, consuming the single non-refundable compile attempt. This is mitigated — `observations.json` is rewritten after every case, so partial evidence survives — and it is the declared stop rule, so it does not block the vote. If you want margin later, set the inner deadline strictly below the outer.
2. **`krunInvocations: 16` is exactly at the ceiling** (`count>=16` fails). Sixteen cases consume the allocation with zero headroom; any re-observation needs a new proposal. Consistent with "no retry".
3. **Stale `.build/compile-attempt.json`** in the worktree would fail closed with `COMPILE_ALREADY_ATTEMPTED`. Correct behaviour; root should confirm a clean `.build` before dispatch.
4. The codec's KAST cell whitelist was validated against real output only for the `prepared` shape (`observed-kast-v4-principal-partial.json`); `preparedTransfer` reuses the same cell walk, so residual risk is a fail-closed `K_OUTPUT`, not a wrong acceptance.

## Votes

- **EXACT resource proposal** (`176542f1…`: one compile 180 s, 16 krun × 20 s, aggregate 512 s, 4 GiB, zero swap, immediate SIGKILL whole cgroup, nonblocking flock, no retry, no reset of the prior 4 compile / 33 krun charges): **YES**
- **EXACT candidate** (`2b802c07…`, base `3d6ea750…`): **YES**
