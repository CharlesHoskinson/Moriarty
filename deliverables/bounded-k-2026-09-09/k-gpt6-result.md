# Independent GPT-6 result audit

Verdict: **APPROVE publication of this bounded repayment result.** No blocking or nonblocking finding remains within the reviewed scope. Separate exact Fable result approval and the final acceptance disposition remain required. This is not full SP03 acceptance.

Reviewer: GPT-6 Astra, agent `/root/hook_dependency_review_gpt6`, independent product reviewer. I reviewed earlier candidates and repairs but did not author the product or run K. I refreshed Moriarty guarded status. Its historical operational blocks remain separate from this scoped result.

Result manifest SHA-256: `5dc5e479259b58bf5fb6b84ac54828797cead874dfb0ac21d8c43e47a40e38bb`. I verified all 239 frozen file hashes, including immediately before this receipt. The runtime inputs still match execution candidate 03 (`bec04a6b4de57255bd328cf677ca99475cc0b9877c16e943ae901944de4a4c23`). Final review receipts and acceptance.json are explicitly outside that evidence freeze.

## Independently verified execution result

I decoded all sixteen captured raw KAST outputs offline with the reviewed strict v4 codec. Each captured input exactly equals the codec encoding of its frozen financial input. Every decoded result equals its saved trace result, independent expectation, saved source result and observations entry. IDs, packet digests and array/effect order match. No expected JSON replaced observed K output.

Six cases prepare successfully: principal partial repayment, AccrualFirst, PrincipalFirst, full repayment, crossing the interest boundary, and transferred cash 40 with nominal repayment 30. The latter debits cash and gross allowance by 40 while discharging debt by 30. Principal partial repayment leaves 70. Interest-first payment 7 leaves principal 100/accrued 3, while principal-first leaves principal 93/accrued 10. Complete comparison includes balances, allowance, work, closure reserve, debt/status, conversion and other retained metadata, tombstones and ordered effects.

Ten cases reject with exact expected code and actionIndex: insufficient balance, insufficient allowance, insufficient ordinary work despite reserve, receiver overflow, outstanding invariant with insufficient work, missing transfer ID, insufficient unallocated funding, payer mismatch, zero Transfer and excessive nominal repayment. Each rejected object contains only status, code and actionIndex. The invariant-before-work case preserves precedence. Late Repay failures expose no Transfer effect or post-state.

I reran the actual-source checker: all sixteen cases pass through the real parser and preparation API. All six codec tests pass. The root README whole-grammar drift test passes. No compiler, K, proof or native command ran during this audit.

## Artifact and resource evidence

The successful attempt contains one compile receipt and sixteen distinct krun receipts. All exit codes are zero, no command timed out, each krun stayed below 20 seconds, compilation stayed below 180 seconds, and cumulative execution stayed below 512 seconds. The recorded compile took 4.038793838 seconds. Krun durations sum to 30.488086097 seconds. Systemd reports service runtime 34.763 seconds and memory peak 418.6M with swap 0B. The wrapper's wall time is 34.773424149 seconds. The cgroup receipt records memory.max 4294967296 and memory.swap.max 0.

The observed supervisor argv exactly matches resource-proposal-03.json, including the named unit, nonblocking lock, full control-group termination and immediate SIGKILL. The retained counter is sixteen krun invocations. All 174 currently present compiled files match the binding's complete artifact map. Runtime source hashes also match that binding. This supports stability through this suite, not permanent artifact immutability.

I counted the three retained allocations and checked their admission-to-candidate hashes: attempt 01 has one failed compiler call and zero krun, attempt 02 has one successful compile and one krun followed by decoder failure, and attempt 03 has one compile and sixteen krun. Totals remain three compiler calls and seventeen krun. Earlier failures and charges are preserved. No failure was relabeled as a successful suite.

## Publication claims and limits

The K README, result report, root README, roadmap and dated wiki additions describe finite local preparation evidence and leave full SP03, semantic freeze, formal correspondence, PCD and Midnight financial acceptance open. The raw-K-term domain is excluded explicitly. Untested represented branches and unsupported conversion/allocation/replay domains remain identified. The source/evaluator equality observed here is finite evidence, not a correspondence theorem.

I checked the nine official documentation capture/extract digests and the robots capture against the dated manifest. The intake and lessons distinguish current website guidance from the pinned installed interface, preserve both observed failures, and retain the earlier claims as historical context. The stored wiki lint reports zero issues in its scoped 37-page check. This audit does not turn those source documents into runtime evidence or revalidate every legacy wiki claim.

Pending-result-review statements describe the intentional evidence-freeze point. Record the later review and publication disposition in acceptance.json. Preserve the frozen files and require separate exact Fable result approval. No executable change is covered by adding disposition records.
