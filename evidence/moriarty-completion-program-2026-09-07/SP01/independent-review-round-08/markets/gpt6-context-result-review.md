# Market candidate04 independent result audit

**BLOCKED for the specified-only fragment.** Three concrete issues remain. This is not full-map, language, semantic-freeze, K, proof, runtime, ledger or sprint acceptance.

Candidate SHA-256: `221cb8f08f993b7b0c7c88a3d20822e0794919c808d7db10c1efc4b8275e29c1`; 490,024 bytes across three owned files.

1. **MARKET-CR01 — crossing history still disagrees with its patch.** At `traces[1].controls.boundedCross.postState.patch.replace.history`, genesis, admin and admission registry still reference baseline shared records. The stored post-state references the crossing sibling. Independent patch application finds exactly these three differences; root's green report checks only the six primary traces. Mutation7 also retains baseline `hist:position:P1:v1` in `checkOrder[1]`, despite admitting `hist:position:P1:margin90:v1`. Use one sibling context through the patch, post-state and ordered prerequisites.

2. **MARKET-CR02 — step writes and the acceptance frame remain incomplete.** Redemption changes T1 status, all three duty records and nominal discharge authority without assigning the corresponding named writes to its three steps. CL finalization omits changed metadata and necessary operand reads. The projection still mixes parent/child resources without a complete field partition; crossing declares36 unique writes but contains35. Its claim defines the frame using the supplied patch. After synchronizing only CR01's three history references in memory, changing `postState.pool.feeGrowthGlobal1` and the matching patch from0 to999 still preserves patch equality, accounts, gross partitions, duties and work, while violating the declared resource write set. Define the primitive transitions and permitted resource frame independently of the patch.

3. **MARKET-CR03 — the closed bound domains are not closed.** Nonempty arrays such as plan `orderedSteps`, CL `initializedLiquidityBounds`, nominal authority and post outgoing caps are unlisted and lack checked-empty projections. Literal enforcement rejects the supplied positives; ignoring them leaves gaps. Sidecar object encoding is also ambiguous: compact JSON containing 1,000 é characters is2,008 bytes with literal Unicode or6,008 with Unicode escapes, crossing the4,096 limit. Specify all supported containers, fixed record schemas and exact encoding, then count actual values before claims.

The useful repairs are preserved:

- All six primary patches materialize exactly. Financial replay passes122 checks; its one failure is the crossing patch.
- All304 structured references resolve. Five admitted sibling contexts now bind their own complete pre-state and current records.
- All11 resolved rollback comparisons have zero differences. Caps19,744/4,883 survive rejection; ordinary work is charged and reserve16 remains unspent.
- All11 numeric rejection discriminators reproduce, including margin90, ordered burn150 and the initialized four-segment path995/9,275.
- The six oracles retain exact output19,743/fee30; CL477/4,882/fee1; AMM D1,999/y850/dy50; redemption150/297/3; vault10/9/1; and margin withdrawal75.
- The former crossing remaining980→1,024 counterexample now fails the explicit work-conservation equation. Vault temporary token1−1 is explicit; raw vault31/21 and AMM31/45 counts reconcile.

The retained67-check author capture and recipe hashes match. I freshly ran its60 read-only-prefix checks on the candidate and eight corrupted copies; all passed. Corruptions included an actual4,097-byte sidecar,33 schedule events, a revoked sibling head, a wrong complete-state reference and a changed transfer amount. These demonstrate inadequate tests; they are **not** claims that a financial evaluator accepted the data. The remaining seven author checks were not run verbatim because they execute subprocesses and overwrite captures. Independent patch checks ran in memory.

All17 protected candidate/input/source/evidence paths matched before and after. Only these review files were written. The JSON retains exact in-memory corruption recipes, case outputs, hashes, probe corrections and limits.

Reviewer: independent Codex, system-described GPT-6; requested routing GPT-6 Astra high. Exact serving model/effort metadata was not independently exposed. Review started09:18:16 UTC; substantive work stopped09:30:34 UTC, within780 seconds. Report completed 2026-09-08 09:31:56 UTC, elapsed820 seconds within the900-second native allowance, which was not cgroup enforced.
