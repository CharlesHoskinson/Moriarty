# Conversion rounding and ProRata in K

Status: actual K execution passed. Independent result-review disposition is recorded in `acceptance.json` when available.

The definition now computes converted settlement with none/floor/ceil rounding,
rejects overflow before division and rejects free debt discharge through zero cash.
ProRata divides checked nominal*principal and preserves both residual components.
Scale checks precede exponentiation. The codec passes invalid financial inputs to
K rather than deciding success. Settled ProRata debt remains valid for Transfer-only.

[Independent cases](INDEPENDENT-CASES.md):22 new numeric records plus42 unchanged
prior distinct inputs. `check-source-cases.mjs` compares complete preparation
results; `check-cli-cases.mjs` exercises the actual successor simulation CLI.
Neither generates expected values or invokes K. The latter writes a new CLI
receipt, so do not run it over an immutable reviewed observation file.

The proposed allocation is one compile180s plus64krun20s each,1512s aggregate,
whole-tree4GiB zeroSwap immediateSIGKILL and flock. Both exact preexecution votes
and root admission are required. Previous five compile attempts and49krun calls
remain charged. No full SP03, semantic freeze, proof or Midnight settlement claim.

## Actual execution and checks

All64 complete K results match independent expectations, source preparation and
the actual successor simulation CLI, including rejection-only records and CLI
exit codes. The22 new cases have13 Prepared/9 Rejected;42 retained distinct cases
have15 Prepared/27 Rejected; total28 Prepared/36 Rejected. This is finite coverage,
not every numeric combination or a universal correspondence theorem.

[Raw commands and K output](attempt-01/), [execution summary](execution-result.json)
and [root admission](root-admission.json) retain exact inputs, observations and
resource identity. One LLVM compile and64krun calls succeeded. The service reported
328.743seconds; root supervision measured328.769seconds. Reported rounded memory
peak was494.8M with zero swap. The active snapshot confirms4GiB/zeroSwap/1512seconds
and control-group SIGKILL; post-collection defaults are not active execution limits.
The root verified all187 compiled artifact hashes unchanged since this compile.
Cumulative history is six compile attempts and113krun calls; no old charge was reset.

[Preexecution GPT-6](gpt6-preexecution.md) and [bound Opus JSON](opus-preexecution.json)
approved exact source, design clarification and resources. Opus resolved to
claude-opus-5 at medium effort. The first unusable Opus payload is retained and
excluded from admission; see [its disposition](opus-preexecution-disposition.md).
Result audits are separate from these preexecution votes.

Verification without another K run:

```sh
python3 -m unittest discover -s experiments/moriarty-language/formal/k -p 'test_*.py'
node deliverables/numeric-k-2026-09-09/check-source-cases.mjs
python3 deliverables/numeric-k-2026-09-09/check-k-results.py --retained
```

The offline suite records17 passes and the language suite236 passes. The retained
checker decodes saved raw K output without rerunning K. It cannot independently
rehash compiled artifacts on a fresh clone where those artifacts are absent;
without `--retained`, it also verifies them when the preserved local build exists.
The actual CLI receipts preserve output/exits and exact source/invocation data,
although their original temporary input paths were removed after the check.
The inspected four-page wiki save passed strict lint with zero issues.

## Review scope and remaining boundaries

The reference TypeScript kernel has a defensive zero-total ProRata allocation guard.
It is not an extra admission invariant, and K does not emit an index1 INVARIANT
for it: the positive-outstanding prerequisite prevents that division from being
reached. Transfer-only correctly preserves settled ProRata debt. The referenced
kernel's stable-error table is a reference-kernel contract, not a claim that every
row is a reachable K rejection. No zero-divisor rejection trace is invented.

Ceil-increment overflow and ALLOCATION_COMPONENT rejection are also unreachable
under these admitted prerequisites; their presence does not mean they were
sampled false. For ProRata, N≤P+A makes both component discharges fit. This is an
algebraic reachability observation, not a mechanized proof. Exact rounding/mode
combinations beyond these22 cases remain finite-test coverage gaps.

The codec copies unchanged metadata and constructs effects from K-computed changed
amounts. Transfer-only debt preservation is a property of the complete K-plus-codec
path, not a separately emitted debt result or a proved K invariant. The codec
permits an out-only synthetic KAST wrapper; actual runtime output also contains
an empty terminal continuation. Input variation alone cannot exclude host computation;
reviewed source, artifact bindings and actual command evidence support that claim.
Out-of-bound K scalar output currently fails closed with MALFORMED_INPUT rather
than a more specific K_OUTPUT diagnostic; this is not a financial acceptance.

At a hard process termination, the current in-place binding write could be
interrupted. A damaged binding fails closed and never authorizes a retry; preserve
all outer admission and attempt/trace evidence and require an explicit reviewed
accounting reconciliation. This run finished normally with intact matching bindings.
Do not treat a corrupt or missing counter as zero.

The next full-roadmap gate is SP01.2/.3's missing financial contract, challenge map,
signing/display decisions and theorem obligations for rp01-full. This numeric
increment does not close that gate, full SP03, Core semantics, proof-carrying
transactions or Midnight financial settlement. No public transaction was submitted.
