# REPORT: four-piece VS split (source)

**Author.** `claude-opus-5-5` (runtime-stated), fresh context, not an author of the parent candidate, operator, reviews or runs. The runtime exposed a reasoning-effort value of 15 on the harness scale; I cannot observe anything further about effort. Read/Write/Edit only. I compiled, executed, hashed, listed and fetched nothing, launched no subagent, and wrote only inside this directory.

**User direction.** "Split the shard in 4 pieces." Implemented as directed; not reopened.

## 1. Implementation (what exists now, as source)

**Next falsifiable capability.** Four distinct coordinated VS pieces whose conjunction preserves every duty of the parent VS, each applied ≤ 14,336 B; then combined size and work measured. Predicate for this source task: the placed 32-file package type-checks, the 147 new tests match `EXPECTATIONS.json` exactly, and each of VS1–VS4 applied is ≤ 14,336 B. Nothing here has tested that predicate.

New and changed files (`candidate/aiken/`, placed per `SOURCE-CHANGE-MAP.json` with 23 unchanged C′ files):

| File | Status | Role |
|---|---|---|
| `lib/pvc2/shard4.ak` | NEW | Production composition: `VsPieceV1`, `coverage`, the four piece bodies and per-thread parts, `claimed_next`, `identities_distinct`, `delegated`, `credentials_free`, `genesis` |
| `validators/vs1.ak`, `vs2.ak`, `vs4.ak` | NEW | Unparameterized withdraw-zero pieces |
| `validators/vs3.ak` | NEW | `vs3(c2_hash, vi_hash)` withdraw-zero piece |
| `validators/s2p.ak` | CHANGED | Six parameters; Settle → `shard4.delegated`; Genesis → `shard4.genesis`; Revoke/Close/Burn unchanged |
| `validators/vs.ak` | REMOVED | No single-VS validator remains deployable from the package |
| `lib/pvc2/shard4_harness.ak` | NEW, test-only | Five-shard transaction builder, piece predicates, mutations |
| `lib/pvc2/shard4_test.ak` | NEW | 125 tests, 9 fail-annotated |
| `lib/pvc2/shard4_cost_test.ak` | NEW | 22 host-CEK benches |

**Partition** (DESIGN.md §4): VS1 structure = shape_structure, shape_cells, own_mint, topup, reserve_floor, global_outputs once; VS2 accounting = recomputed, bounds, intent_effects; VS3 identity = claim_script, shape_digest, claim, statement_digest (with VI's digest); VS4 custody = custody_pre, continuation, payouts. VI unchanged. Each of the 19 W1 components runs exactly once. Every leaf is an unchanged `thread.ak` function.

**Coordination.** S2′ requires all four pieces at every Settle spend (withdrawal, exactly one redeemer, correct tag, own policy, own outref). Each piece checks its tag and exact, non-empty coverage. VS3 requires VI as VS did. Genesis and Settle both require seven distinct identities. The genesis credential guard covers all five shard credentials. No new intermediate value is published: VS4 uses the claim envelope digest, which VS3 checks against the recomputed statement digest.

**Unchanged.** `thread.ak`, `core.ak`, `intent.ak`, `types.ak`, `claim.ak`, `groth16.ak`, `catalogue.ak`, `shard.ak`, `vi.ak`, `c2.ak`, fixtures and all seven parent test modules (472 tests). SR-06 is not implemented.

**Tests frozen before writing.** `EXPECTATIONS.json` was written before any new module. Split predictions come from the R1 frozen false-parts lists mapped through the partition, not from the new code. The financial tests anchor VS4's next datum to the R1 author fixtures `datum_post_p01..p05` and `stmt_digest_p01..p05`. The 472 parent tests are classified 340 R1/W0W1-production, 26 shard VI-production and 106 compatibility-baseline, not rewritten.

**Operator.** Specified in `EXECUTION-PROPOSAL.json` (OPX0–OPX7). Its step and tool files are **not written**: the parent operator's pins, title set, hash chain and 472-test index encode the parent shape and need logic changes, not a literal translation.

## 2. Observations (inputs only; none produced here)

From the freeze, read as raw evidence:

- Parent attempt 2 (root OBSERVATION `dbdd1512…`, RESULT-MANIFEST `43736ae4…`): 14 steps exit 0; 472/472 expected outcomes (61 fail-annotated); applied c2 2,770 B, vi 12,784 B, vs 20,503 B, s2p 12,356 B; PS-S4 false; 934 s charged. `receipts/sizes.json`, `OP2-compare.json`, `chain.json`, `OP2-time.txt` (837 s wall for `aiken check`) agree with the root summary.
- PC-S1 held in the parent: the two-shard single-thread host-CEK sum exceeded the monolithic gate. That is host CEK of library predicates, not ledger ExUnits.
- Parent attempt 1 failed compile at the parser; its debit stays.
- The 16,384 B maximum transaction size was captured from an unauthenticated provider. It is neither a qualified node parameter nor a script-size limit.

Actual-result review of attempt 2 is pending and is not my task. I adopt nothing from it.

## 3. Acceptance (what is and is not established)

- **Established:** nothing beyond the existence of reviewable source bytes in this directory.
- **Not established:** compile success, any test outcome, any size, the fit of any piece, ExUnits, deploy- or settle-transaction size, fees, registration, signatures (FX1), X1, OEX7, K2, M-HASH, any ledger or target behaviour, circuit or proof acceptance.
- **Predicted, falsifiable:** each piece < 20,503 B (high); each ≤ 14,336 B (VS1 moderate-high, VS2 moderate, VS3 and VS4 low-moderate); sum > 20,503 B (high); S2′ ≤ 14,336 B (moderate-high); VI and C2 bytes unchanged (moderate-high, high); per-transaction work higher than the two-shard parent (high). Main risk: duplicated typed decode in every piece.
- **First actual blocker:** no operator exists for this candidate shape. O'' must be written, root-hashed and freshly reviewed together with these candidate bytes, and R4-1 needs a second position, before the root can authorize one offline run.
- **Kept open:** SR-06, SR-08, SR-09, SR-11 (new test, unrun), SR-12, SR-15, SR-16; debtor consent; D-7; OBS-SH-01; RR-03/08/16; PB-01..PB-12; every U0–U7, M1–M17, MC, SP and G exit; all 20 UTxO pattern families; ACTUS and DeFi coverage; universal/PCD recursion; private handoff; multi-parent composition; full financial and ledger exit. Sharding addresses only the size of the VS code.

## 4. Security notes for the reviewer

1. **Non-bypass rests on S2′.** If S2′'s `delegated` were wrong, a piece could be skipped. Review `shard4.ak` `delegated`/`piece_lists` against FW-DEL-1, and the tests `q_s2p_missing_*`, `q_s2p_swapped_piece_tags`, `q_s2p_duplicate_vs3_redeemer`.
2. **VS4's digest substitution** (N-3) is sound only under the conjunction with VS3. Both pieces select the claim with the same deterministic `settle_claim_input`. Check that no path lets VS4 run without VS3 (S2′ requires both).
3. **Shared piece hashes** (N-5) are the same for every deployment. A defect in VS1, VS2 or VS4 affects every deployment that references them.
4. **Halting order.** Pieces build some stages without the earlier guards of the monolithic order. That can turn a False into a halt on malformed input; it cannot turn a refusal into acceptance (DESIGN §6). The parent's error-path cases are refusal-only tests.
5. **Aliasing check** (N-4) cannot detect a fake script deployed in a slot. That remains an identity-chain fact (M-HASH): the S2′ hash, which owners sign, changes.

## 5. Process

No campaign, scheduler, vote round or orchestration was created. Prior debits and failed attempts are untouched. The two-failed-cycles rule is carried into OPX2a. A separate fresh reviewer must audit these exact bytes; my DESIGN-VOTES entries are first positions only.
