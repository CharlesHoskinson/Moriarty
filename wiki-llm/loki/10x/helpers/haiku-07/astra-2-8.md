# astra-2-8: Adversarial check of fused terminal budget (A102 + A80, k15 target)

Scope: read-only review of Moriarty-factory sources and the pinned zkir-v3 IR. No repo writes, no builds, no proofs. Compile/keygen/prove/verify are UNOBSERVED.

## Correction to my earlier report
- My astra-2-6 addendum implied the old child interface hides the A102 post. That was wrong. The statement is "all public" (`crates/mori-backend/src/sherlock_machine/literal_host.rs:747-753`, `statement()` = 2 context limbs + program + pre + post + rule_tag + charge; `sherlock_link.rs:188` "all public").
- Lane arithmetic: 2 + 86 (program) + 81 (pre) + 81 (post) + 2 (tag, charge) = 252 statement lanes, plus 1 binding = 253 instance lanes. Matches `RUN-REPORT.json` (`instance_lanes` 253; program 86, state 81). So the post is already a public lane in the old interface. A fused design may keep it public (no custody change for this fixture) or make it private (custody change). I should not have framed the post as currently hidden.

## Answer
- **Why the original cap does NOT prove the fused row count.** `circuit_k` 13 and 12 (`RUN-REPORT.json`) are capacity bounds: each child's constraint rows are at most 2^k - 7 usable. The measured row counts are not recorded (RUN-REPORT gives k, not rows). So 12274 = (2^13-7)+(2^12-7) bounds only the sum of the two children's rows, not the fused rows. Fused rows = A102 rows + A80 rows - shared + glue (binding, identification of A80 pre with A102 post, context/enable checks, terminal output). None of those glue rows is measured. The 8192 packaging/hash allowance is an unsourced design guess (ESTIMATE), and the 32,761 table cap (2^15-7) is a separate per-table limit. The k15 target (usable 32,761) is therefore a design budget, not a measured fit.
- **Equivalence obligations (fused accepts iff both children accept with the linked lanes).**
  1. Enable forced to 1 in both fragments (`literal_bridge.rs:144-151`; A80 `ENABLE_ID` at `sherlock_link.rs:97`).
  2. A102 envelope (3, 1) and A80 envelope (0, 0) remain constrained (`LINK_EQUALITIES`, `sherlock_link.rs:76-81`).
  3. a102.post == a80.pre as one shared operand image, not two copies.
  4. a102.program == a80.program.
  5. a102.context and a80.context remain separate per domain (`sherlock_link.rs:885-886, 990-992`).
  6. Binding: NOT an in-circuit obligation today. Binding lanes are SHA-256 claim commitments; neither child nor the linker recomputes them in circuit (`crates/mori-backend/src/sherlock_link.rs` LINK_OPEN_OBLIGATIONS; recl9 `sherlock_link.rs:642` as astra-2 cites, verified text). A fused design adds no binding soundness unless it adds one explicitly. Any claim that a combined binding is injective or required for soundness is a new contract to state and test, not preserved semantics.
  7. Public-lane interface: if the fused circuit keeps the post public, it must expose exactly the 253 lanes or an agreed subset; a verifier must check it.
- **Preserving the 16-field segment and Decider template.** The recl9 `sherlock_link.rs:515-519` fields array has 16 entries: keys_id, a[c], a[d], b[c], b[d], program_c, pre, h_pre, mid, post, h_post, a[x], b[x], n_steps, work, trace_c (verified). The 16-field composed segment of the leaf is also in `recl9/src/leaf.rs:1-6`. I did not verify the decider line astra-2 cites (recl9 decider.rs:1068). Fusion must reproduce that segment and the Decider's communication-commitment template. The current LocalPublic mode has `do_communications_commitment = false` (`literal_bridge.rs:164`), so the LedgerCall commitment path is not used by the fixture. This is an open obligation.
- **Cheap falsifier (not executed).** Before any proving:
  (a) Concatenate the two emitters' instruction bodies in one `IrSource` with the shared post/pre operands, keeping the 253-lane public interface. Run the existing nonproving SDK check (pattern of `check_child_statement`, `sherlock_child.rs:207-240`) on the RUN-REPORT fixture statement. It must pass.
  (b) Mutate exactly one lane: one post lane +1, then one A80 pre lane +1 (with post unchanged). Each mutation must fail the nonproving check. If either mutation passes, the identification is not enforced and the design is refuted.
  (c) Measure the fused cost model (rows/k) on the same IR before any budget is claimed.
  Steps (a)-(c) need one build and were not run here per this task's no-build rule.

## Raw numbers table
| Quantity | Value | Source | Tag |
|---|---|---|---|
| a102 circuit_k / proof_bytes / instance_lanes | 13 / 3440 / 253 | RUN-REPORT.json (children) | CODE |
| a80 circuit_k / proof_bytes / instance_lanes | 12 / 3440 / 253 | RUN-REPORT.json | CODE |
| program / state lanes | 86 / 81 | RUN-REPORT.json | CODE |
| Statement lanes (2+86+81+81+2) | 252, +1 binding = 253 | literal_host.rs:747-753 arithmetic | CODE (derived) |
| Body cap (sum of child usable rows) | 12,274 | (2^13-7)+(2^12-7) | derived; bounds sum of children's rows, not fused rows |
| Proposed fused total budget | 20,466 (12,274 + 8,192 allowance) | astra-2 design | ESTIMATE / unsourced allowance |
| k15 usable | 32,761 | 2^15 - 7 | derived |
| Fused rows measured | none | - | UNOBSERVED |
| Warm replacement target | <= 88.259 s | astra-2; 882.590 / 10 | target, unmeasured |

## Caveats
- k bounds are capacities, not row counts. Row counts for the children are not recorded.
- Old parent 882.590 s is a lower bound on the original full chain (per astra-2), not the full chain.
- Target (b) only. The original k20 parent is removed, not sped up. Full-39 dispatch and the general 16-tx case remain open on both designs.
- Node: `crates/mori-backend/src/sherlock_child.rs` and `sherlock_link.rs` are read-only; pinned zkir-v3 at `~/.cargo/git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/zkir-v3/src/ir.rs` (PublicInput ~663, PrivateInput ~900, no Embedded mode).
