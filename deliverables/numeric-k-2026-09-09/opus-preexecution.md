# Independent Pre‑Execution Audit — Numeric K Extension (`sp03-numeric-01`)

**Verdict: PASS.** Exact source correctness **YES**; design clarification **YES**; resource allocation **YES**. No material blockers. Findings below are one documentation defect and several scoped gaps.

Reviewed as supplied text only: `moriarty.k`, `codec.py`, `run.py`, `test_numeric.py`, `fixtures/numeric.json` (all 64 rows), `repayment-kernel.md`, README control section, `INDEPENDENT-CASES.md`, `check-source-cases.mjs`, `resource-proposal.json`, `candidate.json`.

## 1. Staging and guard order

The two central staging claims hold **structurally**, not merely by fixture sampling:

- **Guard before exponentiation.** `M > 0 andBool SCALE <= 18` is the third `ensure` in `inspect`, at the state stage (index `-1`). `10 ^Int SCALE` appears only in the RHS of `divideRepay`, six KItems later. Since `ensure(H,false,…) ~> _REST => .K` discards the entire continuation, `divideRepay` is never reduced when the scale guard fails, so `10^SCALE` is never constructed. `numeric-scale-uint128max` (scale = 2¹²⁸−1) is admitted by `codec.uint` and rejected by K as INVARIANT — correct, and it is the case that actually exercises this ordering.
- **Product before division.** `convertRepay` prepends `N *Int M <= maxU()` and only then hands `N *Int M` to `divideRepay`. `numeric-conversion-product-overflow` (M = UInt128max, N = 2, scale 18) rejects OVERFLOW even though the mathematical quotient would fit — this is the discriminating case, and it is present.
- **Conversion before allocation.** `numeric-conversion-before-prorata-overflow` and `numeric-prorata-product-overflow` differ only in scale (1 vs 0) and yield INEXACT_CONVERSION vs OVERFLOW respectively. That pair pins the stage boundary rather than asserting it.
- **Exactness → dust → funding → allocation.** `fundRepay` orders `CASH<=maxU` → `CASH>0` (DUST) → `CASH<=T` (INSUFFICIENT_UNALLOCATED), matching the kernel spec's prose ordering, with `allocateRepay`/`splitRepay` strictly after. `numeric-insufficient-converted-funding` (CASH 6 > T 5) and `numeric-floor-dust`/`numeric-ceil-rescues-dust` separate these.

Guard counts match the README: Transfer‑only = 15 guards + START + EXPAND + PREPARE‑T = **18**; Repay = 21 initial + 7 numeric-stage guards = 28, plus 8 stage contractions + PREPARE‑R = **37**. I recounted both from the rules; the README figures are correct.

## 2. First‑failure semantics vs the retained reference contract

The K `ensure` chain reproduces the spec's stable-error table ordering exactly at every point I could cross-check, including the retained adversarial orderings: `reject-overflow-before-zero-repay` (receiver OVERFLOW at index 0 beats ZERO_AMOUNT at index 1), `reject-duplicate-before-work`, `transfer-only-invariant-before-work`, `outstanding-invariant`, and the new `numeric-zero-mantissa-before-work-transfer` (mantissa INVARIANT beats INSUFFICIENT_WORK and beats ZERO_AMOUNT). `codec.REJECTIONS` partitions codes by index consistently with the K rules, and additionally enforces `int(idx) < len(actions)`, so a Transfer‑only packet cannot return an index‑1 rejection.

`run.py::traces` writes `observations.json` *before* comparing and then `fail('EXPECTED_RESULT_MISMATCH')`, so the first mismatch halts with the divergence recorded. No retry, no fallback, no refund. Consistent with the declared stop rule.

## 3. Financial arithmetic — full independent recomputation

I recomputed all 22 new cases from the kernel spec, not from the fixtures' stated reasons. All 22 are arithmetically correct, including balances, allowance gross debit/credit, work 9/5 → 7/7 (two-action) or 8/6 (Transfer-only), reserve 13 preserved, and effect components. Representative checks: `none-exact` 10·3/10=3 exact, A10→0, O 110→100; `floor/ceil-fraction` 4·3=12 → q1 r2 → cash 1 vs 2 with identical allocation (A6/O106), which is the correct way to isolate rounding from allocation; `nominal-exceeds-cash` 30·2/10=6 with dA 10, dP 20 → P80/O80; `max-mantissa-exact` 1·(2¹²⁸−1) at scale 0, with sender→0, receiver 0→max, allowance 0→max, all three overflow guards at their boundary equality (`<= maxU`), which correctly admits rather than rejects; `prorata-partial` ⌊700/110⌋=6 → P94/A9/O103; `prorata-full` exact 100/10 → Settled; `scale-eighteen-exact` 10¹⁸/10¹⁸=1 → Settled.

Retained-corpus accounting is right: 16 + 16 + 10 = 42, plus 22 = **64 rows, 64 unique IDs** (near-collisions such as `insufficient-balance` vs `transfer-only-insufficient-balance` are distinct). The stated exclusion of the six trailing `transfer-only.json` duplicates is consistent with a 10-row retention.

## 4. Reachability honesty

`INDEPENDENT-CASES.md` correctly declares ceil-increment overflow and runtime ProRata zero-divisor unreachable, with valid reasoning (scale 0 ⇒ remainder 0; scale ≥ 1 ⇒ q ≤ max/10). I add one it does not state: **`ALLOCATION_COMPONENT` is also unreachable**. For AccrualFirst and PrincipalFirst, `N ≤ O = P+A` makes both components fit by construction; for ProRata, `dP = ⌊NP/(P+A)⌋ ≤ P` and `dA = ⌈NA/(P+A)⌉ ≤ A`. The guard is defensive only and is never sampled false. This is a coverage note, not a defect — but the deliverable should say so alongside the other two, since silence implies it was sampled.

## 5. Material findings

**(a) Documented-but-unemittable INVARIANT code — documentation defect, non-blocking.** The stable-error table says INVARIANT covers "a zero ProRata denominator at allocation". The K definition has no such `ensure`; the condition is a side condition on `principalPart`'s ProRata rule (`P +Int I >Int 0`). If it were ever violated, the term would go **stuck** (non-empty `<k>` ⇒ `codec.walk` ⇒ `K_OUTPUT` ⇒ HarnessError), not produce `rejected(_,"INVARIANT",1)`. `codec.REJECTIONS[1]` does not even contain `INVARIANT`, so that output would be rejected as malformed. The path is unreachable (NOT_OUTSTANDING and `O > 0` precede it), so there is no behavioural consequence, but spec, K, and codec disagree on a documented code. Recommend rewording the table row to "unreachable after NOT_OUTSTANDING; not an emitted code." Do not gate the run on this.

**(b) Non-atomic accounting write at the exact deadline boundary — minor.** `RuntimeMaxSec=1512` equals `run.py`'s internal `DEADLINE`. The internal check runs before dispatch and normally wins, but a whole-cgroup SIGKILL landing inside `write(BINDING, …)` (`path.write_text`, non-atomic) could truncate `binding.json`, losing the invocation count. Under the stop rule a changed/re-reviewed candidate is required before any re-run, so this cannot silently refund attempts today; still, an atomic replace would be the correct hardening later.

**(c) Codec mislabels host-side output bound failures.** `uint(v)` inside `decode` raises `MALFORMED_INPUT` for an out-of-range **K output**, which surfaces under an input-shaped code rather than `K_OUTPUT`. Cosmetic; the run still fails closed.

**(d) Brittle-but-fail-safe output shape.** `walk` requires `<generatedCounter>` to be Int token `'0'` and `<k>` to be an empty KSequence. Any deviation fails the *first* case, consuming exactly 1 krun and 1 compile attempt. This is fail-fast, not fail-open, and the retained `observed-kast-v4-principal-partial.json` grounds the shape empirically. Budget for it as a real first-failure cost, not a defect.

**Scoped proof gaps (not new mandatory infrastructure):** the TypeScript path (`core/elaborate/evaluate/repayment.ts`) is hash-pinned but not supplied, so `source-check.txt` (64/64) and the CLI transcript are attested, not re-derivable here; K correspondence remains unexecuted by design; 64 finite fixtures establish no universal source/Core/K correspondence. All three are correctly disclaimed in the deliverable and README.

## 6. Resource vote

Bounds are arithmetically consistent and containment is adequate. Worst case = 180 s compile + 64 × 20 s krun = 1460 s against a 1512 s aggregate ceiling — ~52 s (3.5%) slack for codec/serialization overhead across 64 iterations, which is ample. `MemoryMax=4G` matches `limits.memoryBytes = 4294967296`; `MemorySwapMax=0`, `KillMode=control-group`, `KillSignal=SIGKILL`, `TimeoutStopSec=5` with `terminationGraceSeconds: 0` give no TERM grace extension. Fixture count (64) exactly equals `SUITE_LIMITS['numeric'][0]`, so `traces` cannot exceed the krun ceiling; the counter increments **before** dispatch and `test_exhausted_attempt_limits_reject_without_dispatch_or_refund` proves no refund and no dispatch at the limit for all four suites. `compile-attempt.json` via `open('x')` makes the single compile attempt non-repeatable. Defaults for `initial`/`branches`/`transfer-only` remain (16, 512), asserted by test. Historical 5 compiles / 49 kruns stay charged; new totals become 6 / 113. Non-blocking `flock` on a named unit in a fresh worktree serializes correctly. `TasksMax` is absent, but the cgroup memory cap plus whole-cgroup SIGKILL bound the blast radius.

Plugin boundary is respected: this is a root-supervised SP03 experiment under the existing task contract, not a dispatch of the blocked SP01 loan action, and no full-stage promotion is implied.

**Recommendation: dispatch the exact candidate as proposed.** Fix finding (a) in the spec text at the next candidate revision, not before this run.
