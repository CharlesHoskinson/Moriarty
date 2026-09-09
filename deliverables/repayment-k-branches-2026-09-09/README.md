# Repayment branch execution in K

All sixteen new cases executed successfully in the pinned K LLVM backend: five
complete preparations and eleven exact rejections. Every result agrees with the
independent specification-derived expectation and real `.mori` parser/preparation
path. [Execution record](execution-result.json), [raw traces](attempt-01/),
[independent cases](INDEPENDENT-CASES.md) and [source observations](source-observations.json)
retain the complete comparison. This is finite evidence for the existing
`moriarty-funded-repayment/0` projection, not full successor semantics or a
correspondence theorem.

The new successes cover creation of a recipient balance, preservation of row
order, repayment by a party other than the debtor, a combination of those
branches, and PrincipalFirst payment crossing from principal into accrued debt.
Rejections cover missing sender/allowance, self-transfer, duplicate balances,
wrong obligation/recipient/asset, zero repayment, settled debt and two competing
failure cases. A failed Repay returns no tentative Transfer state or effects.

The run used one compile and sixteen krun invocations, taking 43.508 seconds of
service runtime (43.529 seconds measured by the root supervisor). systemd reported
705M peak memory and zero swap. The active-service property capture confirms the
4 GiB cap, zero-swap cap, 512-second runtime limit and cgroup SIGKILL containment.
The post-service property query was made after the transient unit had been
collected and shows defaults/unset accounting; those values do not describe the
executed service. Both raw observations are retained. All 174 compiled artifact
hashes remained unchanged after execution.

The original sixteen fixtures and their outputs are unchanged. Together, the two
suites cover 32 distinct inputs (11 successes, 21 rejections); these counts are not
branch-completeness or proof coverage. Historical charges now total four compile
attempts and 33 krun invocations, including earlier failures and the repeated
initial smoke result. No charge was reset or refunded.

## Reproduce the comparisons

From the repository root:

```sh
python3 -m unittest discover -s experiments/moriarty-language/formal/k -p 'test_*.py'
node deliverables/repayment-k-branches-2026-09-09/check-source-cases.mjs
python3 deliverables/repayment-k-branches-2026-09-09/check-k-results.py
```

These commands do not invoke K. The last re-decodes retained raw K outputs when
no local build exists; it verifies live compiled artifact hashes only when that
local build is present. Its report distinguishes those cases. Actual new K runs
require a separate current bounded allocation; use the recorded resource argv,
not an uncontained standalone command. `run.py --suite branches` selects the new
suite; default `initial` selects the original suite. Selected fixture bytes enter
the build binding, so a build for one suite rejects the other before krun.

The unchanged K definition computes financial values and selects rejection
codes. The codec checks the output envelope/digest and reconstructs metadata,
effects and used-ID lists. Complete result comparison checks that trusted codec
boundary; it does not prove it. For precision, the prepared K result contains
thirteen amount/counter integers, a receiver index, a status and an input digest;
the Opus preexecution report's “fifteen numeric outputs” phrase is a counting
error, preserved in that original report rather than copied into this claim.

Both preexecution candidate/resource votes passed. Result review was pending at
this execution freeze; the subsequent disposition belongs in `acceptance.json`.
Full SP03, general source/Core/K correspondence, financial extensions, native
proofs and proof-backed Midnight settlement remain open. No Midnight transaction
was submitted by this local experiment.
