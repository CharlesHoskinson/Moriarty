# Independent K preexecution review

Verdict: **approve the exact implemented source for the bounded experiment and approve the stated design/resource ceiling.** No blocking finding remains. This is one independent GPT-6 vote. Separate Fable approval and root dispatch checks remain required.

Reviewer: fresh GPT-6 Astra reviewer for this product, agent `/root/hook_dependency_review_gpt6`. I did not author the K product, runner, codec, fixtures, or resource proposal. I loaded worktree AGENTS/develop and guarded status. The existing SP01 operational blocks remain unchanged.

Exact manifest: `candidate.json`, SHA-256 `4775dc47b54f170aa3a19e94090a72e4b11ecf9da74c9396e803a79c4756e199`. All 14 candidate and 13 supporting file hashes match. This review covers the eight formal/k files, resource proposal, SP03 execution note, actual-source comparison script, and bounded reference guidance. I did not re-audit the root README grammar change. Reviewed file hashes are in the JSON receipt.

## Financial design and source findings

Source inspection confirms K receives actual identifiers and numeric input values. Python admits a closed structural projection rather than preselecting financial success. K checks duplicate balance pairs, allowance/debt/work invariants, ordinary work, Transfer, then Repay. The ordered `ensure` continuation determines rejection precedence. A late repayment failure clears the remaining computation and emits only rejection code/index. It exposes no tentative state or Transfer effect.

The rules distinguish transferred cash from nominal repayment. Cash and gross allowance change by Transfer amount. Debt allocation and settlement discharge change by nominal amount under admitted identity conversion. AccrualFirst and PrincipalFirst use different allocation functions. K computes changed balances, allowance, work, principal, accrued, outstanding, status and discharge components. State-sum and addition guards enforce UInt128 bounds before relevant arithmetic. The fixed two-action cost uses remaining work, never closure reserve.

The decoder retains supplied metadata, conversion, row order, closure reserve and action identifiers. It appends the identified tombstones and reconstructs ordered effects. The packet digest binds outputs to the supplied input. Numeric results come from K. Python validates output representation and receiver-row identity without recomputing financial outcomes.

The domain is explicit: two starting balance rows, one allowance, one obligation, empty replay lists, Transfer then Repay, identity conversion, and two allocation rules. Missing recipients can append a balance. Wrong funding IDs, parties, assets, cash, allowance, work and debt values reach K rules. Other shapes and conversions are separately unsupported. This does not establish replay-history preservation, arbitrary unrelated obligations, richer successor records, ProRata, rounding, refunds or full source/Core semantics. Third-party repayment and several represented failure branches have no initial K fixture.

## Resolved review findings

1. A digest-bound Prepared output with receiver index changed from 1 to 0 previously decoded to the same expected result. The final decoder checks the structural receiver lookup. Wrong 0 and -1 indices now reject.
2. Duplicate output JSON keys previously allowed conflicting version fields. The final decoder rejects duplicate keys.
3. Boolean arity previously passed Python integer equality. The final decoder requires integer arity and integer KAST version.
4. The initial resource command allowed a five-second TERM grace after 512 seconds. The final command sets `KillSignal=SIGKILL`, covering the complete control group immediately.

These were concrete local probes or command inspection findings. The first three now have regression coverage. I made no product edits.

## Independent checks

- All 27 manifest bindings verified immediately before this receipt.
- Five codec tests pass. They include six complete synthetic positive result decodes, malformed output, original review repros, and schema/projection separation. Synthetic decoding is not K execution evidence.
- The actual-source checker passes all 16 complete expectations through the real parser and preparation API. The first six input/expected pairs match prior independent financial cases. This checker does not execute K.
- Installed kompile/krun executable hashes match the lock. Read-only runner probes reject stale source binding before dispatch and reject the unimplemented proof route. No K command ran.

## Resource vote and remaining execution conditions

Approve one LLVM O0 compile attempt at most 180 seconds, up to 16 krun attempts at most 20 seconds each, and 512 seconds for the combined run. The explicit 4 GiB memory limit and zero swap apply to the full systemd control group. Immediate cgroup kill addresses the deadline grace finding. The nonblocking lock and named service serialize this experiment. Root must still inspect existing K/native workloads before dispatch.

The combined runner uses one monotonic deadline, clamps later child waits, records attempts before dispatch, and stops on first failure. Compiled-file hashes and source/toolchain bindings reject stale definitions. Source/output parsing errors do not substitute expected financial results. Separate command processes, retries, fallback backends, proofs, installation and network work remain excluded. Preserve all failed attempts and raw command/resource evidence.

Root must verify unchanged candidate bindings and obtain the exact Fable review before executing the reviewed systemd argv. This review makes no K compilation, trace, proof, full correspondence, semantic freeze, SP03 completion, native PCD or ledger acceptance claim. Later result reviews remain required before publication.
