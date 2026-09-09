# Independent GPT-6 preexecution review

Reviewer: fresh independent GPT-6 Astra agent `/root/repayment_branch_audit`.
Date: 2026-09-09. Scope: read-only candidate, expectation and prospective resource review; only this review report was written. No K, kompile, krun, kprove, native proof, network, installation, commit or publication was performed by the reviewer.

**Candidate vote: APPROVE. Resource allocation vote: APPROVE.** These are substantive votes for the exact candidate and the one prospective bounded experiment below. No blocking defect was found. They are not result acceptance, stage promotion, proof, general source/K correspondence, financial suitability or ledger acceptance.

## Exact identity and startup

- Checkout: `/home/charl/Moriarty/.worktrees/sp03-repayment-branches`.
- HEAD/base commit: `1bc1b88f2c63383acb250118ce87f1963f5fd9c1`.
- `candidate.json` SHA-256: `3c1d1b0d6b9ff35a695e2d8c58ff7077f4f4b616a94813d3eec40c246af21895`.
- `resource-proposal.json`: `beab5791d95e79707e053a4712b4e18520b9f5f91824542e54be3ceb47edf85f`.
- `fixtures/branches.json`: `2a9596d52e934533629354a7ecdbd60282f192efcacd3ef9ec7a95ff6d0845dd`.
- `INDEPENDENT-CASES.md`: `e2edd68c0735a6320c74befc9c7af12b24c304b1a052e37f2886d4519936072b`.
- `run.py`: `034a882be66ba3e6369e06aecb80077c904c575f73395114239c653602a94bef`.
- `source-observations.json`: `8e3d07c0e9f9473120f5a144d9bf9d769b7952c834f5756ff21185d2bd4789cb`.

Repository observation: independently checked the manifest digest, current HEAD and all 32 bound file hashes. Independently compared K, codec, initial fixtures and toolchain lock bytes to HEAD; all are unchanged. The initial fixture digest is `d46679961af39a10adec03cb86b4481a39fc9393d15ceb48b37b10b8ce28f832`. No `.build` directory existed at review.

Loaded AGENTS.md and `moriarty-dev:develop`, read the orchestration stop rules, and ran the required guarded CLI status. Status retains SP01.6 unresolved operational history, `sp01-loan-report` as its next action, and no pending transactions. This review neither dispatches nor unblocks that registered action. The proposed distinct SP03 experiment uses the established root-supervised task contract; this vote does not establish host interception.

## Financial expectations and actual implementation

Repository observation and independent arithmetic review: all 16 inputs stay inside codec admission, with two input balance rows, one allowance, one obligation, identity conversion, empty histories, and Transfer followed by Repay. All five complete Prepared expectations preserve metadata, array order, reserve, histories and ordered effects. All eleven rejection objects contain only status, code and actionIndex.

The first four successes discharge accrued 9 then principal 8: P50/A9/O59 becomes P42/A0/O42. Transfer/settlement is 17; allowance 61/7 becomes 44/24; work 9/4/reserve13 becomes 7/6/reserve13. The checked ordered balances are respectively `[Payer56,Custodian19,Lender17]`, `[Lender28,Payer56]`, `[Sponsor56,Lender28]`, and `[Custodian19,Sponsor56,Lender17]`. Sponsor remains distinct from debtor Payer in both relevant records. PrincipalFirst crossing discharges principal 5 and accrued 3 from P5/A10/O15, producing P0/A7/O7, balances 65/19 and allowance 53/15. All five effects and tombstones match these full transitions.

Actual K review: `indexOf`, `moved` and `appended` distinguish receiver absent, receiver index 0 and sender index 1. The codec preserves the original two rows and reconstructs a third row only from K's receiver `-1` and returned append amount, checking receiver identity against the input. It does not compute the credited amount or repayment arithmetic. K matches Transfer sender to Repay payer, recipient to creditor and asset to settlement asset, without imposing payer equals debtor. Its two allocation functions support exactly the admitted AccrualFirst and PrincipalFirst branches.

| Fixture | First failing K/source guard | Expected actionIndex |
| --- | --- | --- |
| reject-missing-sender | MISSING_BALANCE | 0 |
| reject-allowance-owner | MISSING_ALLOWANCE | 0 |
| reject-self-transfer | SELF_TRANSFER | 0 |
| reject-duplicate-balances | DUPLICATE | null |
| reject-obligation-id | MISSING_OBLIGATION | 1 |
| reject-recipient-mismatch | TRANSFER_MISMATCH | 1 |
| reject-asset-mismatch | TRANSFER_MISMATCH | 1 |
| reject-zero-repay | ZERO_AMOUNT | 1 |
| reject-settled-obligation | NOT_OUTSTANDING | 1 |
| reject-duplicate-before-work | DUPLICATE | null |
| reject-overflow-before-zero-repay | OVERFLOW | 0 |

For each rejection, earlier relevant guards pass except the deliberately combined faults. Duplicate balances precede the ordinary work requirement. Crediting UInt128 maximum by 17 fails in Transfer before the zero Repay is visited. A valid Settled zero obligation passes admission and fails NOT_OUTSTANDING before EXCEEDS_OUTSTANDING. The recipient and asset mismatch cases fund the Transfer with the actual sender asset/allowance first, reaching the intended Repay guard.

The source comparator builds source and invocation from inputs, invokes the real `prepareSuccessor` parser/elaborator/lowering/kernel path, and uses expected outputs only for complete deep comparison. Token amounts use Token in the asset-mismatch source, preventing an earlier SETTLEMENT_UNIT failure from masking the intended financial test. The independently authored fixture rationale identifies specification-based arithmetic and no evaluator-derived expectations. Inspection found no case-name dispatch, expected-output arithmetic, constant success path or oracle laundering. Finite agreement remains an observation, not a universal equivalence claim.

## Runner and resource reasoning

The CLI uses closed argparse choices `initial|branches` and defaults to initial. `sources()` binds the selected fixture path and its bytes alongside runner, K, codec and toolchain lock. Switching suites makes the source-map binding stale before any krun charge or command. The new offline test exercises that real check while intercepting external execution. The existing exclusive compile-attempt file and charged-before-dispatch krun counter remain intact, and a mismatch stops the suite immediately.

Approve exactly one new compile attempt (180 seconds), at most 16 krun invocations (20 seconds each), a 512-second aggregate whole-process-tree ceiling, 4 GiB memory, zero swap and immediate SIGKILL containment. The proposal's exact systemd argv supplies RuntimeMaxSec, MemoryMax, MemorySwapMax, KillMode=control-group and KillSignal=SIGKILL; its named unit and nonblocking flock serialize this allocation. The runner also enforces its cumulative deadline and kills timed-out child process groups. These layers are necessary because direct runner execution alone does not provide the whole-tree memory boundary. The proposal requires root to check for competing heavy work before dispatch and forbids separate processes, automatic retries and backend fallback.

The new budget is justified by decisive unobserved receiver reconstruction, row-order, third-party and guard-precedence branches in unchanged semantics. It is not a retry of an unexplained failure. Historical charges remain three compile attempts and seventeen krun calls, with no refunds or deletion of prior evidence. The fresh worktree's empty build state is a distinct new allocation, not a reset of the old one. This vote applies only with the specified root containment and exact frozen candidate; changed candidate bytes require refreshed reviews.

## Independent cheap checks and limits

Reviewer experiment observations:

- `python3 -m unittest test_runner test_codec`: eight tests passed, 0.047 seconds, including cross-suite stale-binding rejection without K dispatch.
- `node deliverables/repayment-k-branches-2026-09-09/check-source-cases.mjs`: all sixteen full source results matched the independent expectations; no output artifact was overwritten.
- Independent manifest/input/evidence validation confirmed all 32 hashes, all sixteen codec-admitted inputs, retained source observation order, exact full expected results and invocation-state bindings.

The SP03 append accurately leaves K execution and both result audits pending. K behavior for these new cases is still specified-only at this preexecution review. Root must retain actual compile/trace outputs, containment/resource evidence, exact source binding and both independent result audits before reporting finite observed agreement. Full SP03, proofs, native recursion, ACTUS/DeFi coverage and financial ledger acceptance remain open.
