# Independent GPT-6 result review

**Verdict: PASS for the exact sixteen-case local repayment branch experiment and its scoped evidence claims. No blocking finding.** This accepts finite observed agreement and the minimal fixture-suite selection change. It does not establish full SP03, source/Core/K correspondence, financial completeness, proofs, native recursion, authorization or Midnight settlement.

Reviewer: independent GPT-6 Astra agent `/root/repayment_branch_audit`, separate from the implementer and fixture author. Date: 2026-09-09. The reviewer previously supplied the exact preexecution candidate/resource vote and now independently audited the frozen actual results. Only offline reads/checks and this report write were performed; no K retry, compile, krun, kprove, native, network, install, commit or publication.

## Exact candidate

- Checkout: `/home/charl/Moriarty/.worktrees/sp03-repayment-branches`.
- HEAD/base: `1bc1b88f2c63383acb250118ce87f1963f5fd9c1`.
- `result-candidate.json` SHA-256: `74f4ff598bbaed2aefc6de4cc4d6e74ede8b5190feff884bf1bf0efa4daeb1bd`.
- Preexecution `candidate.json`: `3c1d1b0d6b9ff35a695e2d8c58ff7077f4f4b616a94813d3eec40c246af21895`.
- `execution-result.json`: `b9ee033e718fc9129e2f1d6e43b5bafeaf98a642f99bce1dcec59279e43c81a1`.
- `execution-supervisor.json`: `cb2327ba0188831140b55507b617ecaea8140361c14bbca97377289edb2008c5`.
- Retained `attempt-01/binding.json`: `930ee3a1f5f1564ca455e686a9e4c6d40003649ea4c0e7da595743339c400b1b`.
- Retained `attempt-01/observations.json`: `5bc3eaf0a4b0cb744c698ae8cd32eee231a26d0a39d1a7d1cdfa5a785d8ec194`.
- `check-k-results.py`: `86819403ea884d7bed12cac12c420aa4d0c3716d9f0cbf878e24fca8b51cd002`.
- Branch expectations: `2a9596d52e934533629354a7ecdbd60282f192efcacd3ef9ec7a95ff6d0845dd`.
- Source observations: `8e3d07c0e9f9473120f5a144d9bf9d769b7952c834f5756ff21185d2bd4789cb`.

Repository observations: independently verified all 133 result-manifest file hashes, manifest digest and HEAD. The only changed file among the original 32 preexecution-bound paths is the SP03 execution note; its changes record the result. Runner, K, codec, financial evaluator, fixture expectations and source observations retain approved bytes. The original sixteen fixtures remain unchanged. Both selected executable files currently match toolchain.lock.json hashes. All 174 live compiled artifact hashes match the retained binding; its compile-attempt source map agrees with its final source map and binds the branches suite rather than initial.

The required Moriarty startup skill remains loaded; refreshed CLI status still reports unresolved SP01 operational history and no pending transactions. This result does not dispatch or unblock that old registered action or establish host interception.

## Raw result audit

Reviewer experiment observations: ran `check-k-results.py` with the live build and `--retained`; both passed all sixteen complete comparisons. Live mode checked 174 compiled artifact hashes, retained mode reported zero, accurately distinguishing unavailable retained binaries from a fresh artifact check. The script re-decodes raw stdout against codec-admitted inputs, verifies exact encoded input bytes, and compares the whole decoded result against independent expectations, source results and retained observations. Its output is an offline result check, not a new K run. The fixed five/eleven and historical totals in its summary are appropriate for this frozen suite and are independently counted below; the script is not a general resource admission mechanism.

To avoid relying solely on the shared decoder, separately parsed every raw KAST result and compared each returned token directly against the independently specified full result: input digest, all thirteen amount/counter integers, obligation status and receiver index for preparations; digest, rejection code and index for rejections. Independently recomputed each digest from its full input. Also checked every retained trace result object's complete contents, each exact krun argv, encoded-input identity via the offline checker, successful command status, absence of timeouts and duration limits. All eighty retained per-trace input/stdout/stderr/result/command files are byte-identical to their live build counterparts. The supervisor stdout equals the retained observations exactly, and every execution-result check references the actual raw-output digest.

| Successful case | Actual ordered balances | Actual debt P/A/O | Receiver index |
| --- | --- | --- | --- |
| missing-receiver-append | Payer 56, Custodian 19, Lender 17 | 42/0/42 | -1 |
| swapped-balance-rows | Lender 28, Payer 56 | 42/0/42 | 0 |
| third-party-payer | Sponsor 56, Lender 28 | 42/0/42 | 1 |
| third-party-row1-missing-receiver | Custodian 19, Sponsor 56, Lender 17 | 42/0/42 | -1 |
| principal-first-crossing | Payer 65, Lender 19 | 0/7/7 | 1 |

The first four actual preparations return allowance remaining/spent 44/24, work 7/6, settlement 17 and principal/accrued discharge 8/9. The last returns allowance 53/15, work 7/6, settlement 8 and discharge 5/3. All preserve reserve 13, obligation metadata, histories and ordered Transfer/Repayment effects as specified. Sponsor funds the two third-party cases with its own allowance while debtor remains Payer. Missing receiver rows append after the preserved input rows using K's amount; no host amount calculation substitutes for K.

The eleven actual first failures are MISSING_BALANCE/0, MISSING_ALLOWANCE/0, SELF_TRANSFER/0, DUPLICATE/null, MISSING_OBLIGATION/1, TRANSFER_MISMATCH/1 twice, ZERO_AMOUNT/1, NOT_OUTSTANDING/1, DUPLICATE/null and OVERFLOW/0, in fixture order. The final two confirm duplicate-before-work and receiver-overflow-before-zero-Repay precedence. Each rejection returns no tentative post-state or effects. This agrees with the specification arithmetic and actual K guard ordering reviewed before execution.

The source comparator still invokes the real parser/elaborator/preparation path and reads expectations only for a complete assertion after execution. The runner invokes the pinned krun command and decodes raw K output before comparing expectations; it never passes expected outputs or case-name success decisions into K. The input hash binds all input metadata, while the codec reconstructs preserved metadata, ordered effects and tombstones from that bound input. The result has thirteen financial amount/counter integers plus a receiver index, status and digest: the corrected deliverable statement is accurate, and the original Opus counting error is preserved as historical text. This remains a trusted-codec comparison, not a proof of that boundary.

## Resource and authority audit

Repository observations: root-admission hashes match the exact preexecution candidate, resource proposal and two review receipts. Its timestamp precedes execution. The supervisor argv equals the approved proposal byte-for-byte as a JSON argument vector: one systemd service, nonblocking flock, branches selection and compile-and-traces. The actual compile command uses LLVM and the pinned kompile path; all sixteen krun commands use the bound definition and expected corresponding input paths. The final binding records sixteen consumed krun invocations.

Actual command receipts show one compile in 5.545107795 seconds; maximum krun duration 2.625110765 seconds, summed krun durations 37.679555993 seconds. All commands exited zero without timeout. systemd reports service runtime 43.508 seconds and peak 705M, swap 0B; root's supervisor measured 43.529334194 seconds. These are within one compile 180 seconds, sixteen krun 20 seconds each and aggregate 512 seconds.

The active-service capture identifies the correct service cgroup and records MemoryMax 4294967296, MemorySwapMax 0, RuntimeMaxUSec 8min 32s, KillMode control-group and KillSignal 9. Its observed MemoryPeak 739315712 and MemorySwapPeak 0 are consistent with the rounded final memory display. This is evidence of the actual run's containment. The later collected-unit query contains infinity/default values and unset accounting; it does not describe the execution caps and is correctly explained in the deliverable. No limit conclusion here relies on that later query.

Prior execution-result.json retains three compile attempts and seventeen krun calls. Adding this run gives four and 33. Initial and branch suites contain 32 distinct full inputs, with 11 Prepared and 21 Rejected expectations; the new sixteen are five/eleven. No historical refund or reset is claimed or observed. Root's no-competing-heavy-process and previously absent-unit checks are recorded preflight observations, not a claim of host-wide interception.

## Claims and disposition

The root README, K README and SP03 append describe the demonstrated branches and explicitly retain broader obligations. The deliverable accurately labels finite observations, the codec boundary, runtime evidence and the pending-at-freeze result review. Its reproduction commands perform offline checks and do not authorize another heavy run. No constructor, financial semantic rule, acceptance gate or public target was widened.

PASS is limited to the exact frozen candidate and reviewed local observations. Both scoped independent result reviews and the root acceptance record are still required for the final combined disposition. No Midnight transaction was submitted by this experiment; full successor semantics, branch completeness, mechanized correspondence, ACTUS/DeFi conformance, native proofs and mandatory proof-backed financial settlement remain open.

## Bounded appendix: separate wiki save draft

**PASS for the proposed four-page content delta only.** This draft is outside the 133-file result freeze and this appendix does not change that candidate identity. Reviewed `wiki-save.json` SHA-256 `509571ce0953b0300f864ca9a3389ff8e30a08b084cb20958edb36b29379530b`, operation `save-repayment-k-branches-20260909`. Independently verified all four expected current-page hashes and all four proposed content hashes, then inspected the complete unified delta against the existing pages.

The exact target set is `wiki/k-framework/k-best-practices.md`, `wiki/index.md`, `wiki/log.md` and `wiki/hot.md`. Changes add the scoped experiment and lessons, add navigation/log entries, refresh the current-context paragraph and update timestamps. Existing historical best-practice and failure records remain unchanged. The hot-page summary replacement retains failed-run history by reference and directs readers to each result acceptance record. No source ID, accepted claim, theorem, financial gate or semantic rule is added or promoted.

The new prose accurately records five successful and eleven rejecting observations, independent full-state expectations, row order and payer/debtor identity, suite binding, actual active-service containment, 43.508 service seconds, rounded 705M peak and zero swap. It preserves finite-evidence and full-semantics/correspondence/settlement limits and does not claim this still-unapplied save already occurred in the live vault. Approval here covers content and exact-delta integrity; root retains responsibility for the inspected portable transaction apply. The reviewer did not apply it or mutate any canonical wiki page.
