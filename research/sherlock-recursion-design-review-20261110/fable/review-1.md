# Fable review, round 1: Sherlock recursion VM design

Date: 2026-11-10. Reviewer: `fable` (lead). Read-only review; nothing was built, run or proved.
Written before reading Astra's round-1 output. Estimates are marked ESTIMATE.

Paths are relative to the repository root. `D` = `openspec/changes/sherlock-vm-consensus-20261107`.
Ledger citations use the tag, `ledger-9.1.0.1:<path>:<line>` in `repos/midnightntwrk/midnight-ledger`
(the clone's working tree is on branch `ledger-8`; the tag resolves to `a2b30128`).

## 1. Verdict

**Makes sense with changes.** The recursion layer (linear self-recursive node `SHKIVC06`, run
decider, `acceptRun`) is sound as designed, is built, and is cheap enough: at 16 steps per leaf the
node chain for fact(3) or list123 is 19 node proofs, about 20 minutes. What is not established is
the layer under it. Every measured number since the consensus is for the node and the decider over
stand-in or 96-lane digest leaves. The universal leaf, which carries the whole VM, has no row
measurement (`k_U`, task 4.1, open), and the whole throughput plan is a function of that one
number: the plan passes gate G-C only if `k_U <= 17` (comfortably only at `k_U <= 16`), and at
`k_U >= 18` the design has no defined route that passes, because every named fallback is closed or
fails the arithmetic (option F fixes S=1, about 5 h; the tree is at k=21; k=19 failed; the GPU
track is unfunded). The indirect evidence in the repository leans against a small `k_U`. So the
design should keep its recursion layer, measure `k_U` now with a lower-bound probe before more
fragment or formal work is spent, stop treating option F and the tree as fallbacks, and state
"general purpose" for what the code gives: one key set for every program inside one frozen
`Bounds` and profile, with a `Halted(u128)` result and one contract per program.

## 2. Keep / change / stop

| Decision | Call | Evidence |
|---|---|---|
| D1, D2, D7 reference machine, bounded images, host boundary | Keep | Unchanged since consensus; nothing measured contradicts them. Note the cost consequence under concern 1: flat carriers are hashed whole in every leaf (`crates/mori-backend/src/sherlock_c6_child.rs:551-578`). |
| D3 one-hot dispatch | Keep the idea; change the order of work | Dispatch today rebuilds full `Facts` per rule circuit, including "a 128-bit decomposition of every image and helper lane" (`research/skeleton-coding-20261108/testing.md:20`). Its cost in one shared leaf is unmeasured. |
| D4 universal leaf U, S steps per leaf | Keep as the only route; change the gate rule | S is the lever that works (section 3, concern 2). Rule `k_U + log2 S <= 20` is inconsistent with code: the leaf path refuses `circuit_k > MAX_K` with `MAX_K = 19` (`crates/mori-backend/src/proving.rs:22`, `sherlock_c6_child.rs:245`). |
| D4 option F as "the measured fallback" | Stop calling it a fallback | F fixes S=1 (`D/requirements.md:35`). S=1 is 293 x 61 s = 4.96 h, list123 5.13 h (concern 2). F cannot pass SRC-011 on this host without the GPU. |
| D5 seven-lane leaf statement | Keep; change two details | Producer still publishes 6 lanes, with outcome tag and value as constants (`sherlock_c6_child.rs:556-578,598-608`); the node consumes 7 (`crates/mori-recursion/src/ivc.rs:74,549`). `h_pre` is unconstrained by the leaf and has no stated use; give it one (genesis check, concern 5) or say it is a free lane. |
| D6 linear IVC node, run decider, `acceptRun` | Keep | `ivc.rs:363-376` (heights), `:383-421,517-521` (genesis realization (a), collapsed carry), `:498-501` (genesis link); decider pins the node key and pairs in circuit (`crates/mori-recursion/src/run_decider.rs:206,243-276`), opens `Halted(result)` (`:279-288`). Measured 61.07 s, 16.58 GB (`D/tasks.md:44`). |
| Node at k=19 (lever 1, SRC-015(a)) | Stop | Not reachable: 652,437 rows, 128,145 over 2^19; the two collapses are 78% (`D/tasks.md:40`). Over the real 96-lane C6 leaf key the node model is 741,708 rows (`qa/sherlock/manifests/node.json:22`). A16 stays a failing row as written (`D/tasks.md:44`). |
| `S_MAX = 16` in the node | Keep as an upper bound; set it to the final S at the 4.4 re-key | `ivc.rs:77`. The node is re-keyed over the universal leaf anyway (leaf VK is a node constant, `ivc.rs:575`). Task 4.10 still says "S=8 at `k_U <= 17`, S=4 at `k_U = 18`" (`D/tasks.md:65`), which never yields 16. |
| D13' tree (three-slot node) | Stop on this host | F22: 1,078,147 rows, k=21 (`D/tasks.md:36`). f3 = 1.69-1.74 (`wiki-llm/recursion-perf/profiles/c0-first-profile.md:57`), so three parallel provers give at most 3/1.73 = 1.73x throughput, less than the 2x cost of k=21. "Kept as fallback (closed)" (`qa/sherlock/decisions/speed-levers.json` `gates.G-A.decisions.tree`) should read "closed". |
| GPU track (§3.7) | Keep closed until `k_U` is known; then decide | m + q = 0.83 opens it on paper, but it is unfunded (`D/dissent.md:179-182`) and 7-12 weeks by its own estimates (`D/design.md:130`). If `k_U <= 16` it is not needed for G-C; if `k_U >= 18` it is the only lever left. |
| P = 3 leaf workers, 100 GB aggregate | Change | The host scope is 72 G and the probe ran under 60 GiB (`speed-levers.json` `c0_metric_per_decision.leaf_workers_P`). A k=20 ZKIR leaf is ESTIMATE 15-26 GB (`research/sherlock-speed-replan-20261108/prover.md:194`). Node 17 GB + 3 leaves does not fit 72 G at the upper estimate. |
| §3.6 / SRC-017 "general purpose recursion" wording | Change | See concern 4. |
| F20 / F21 formal work | Keep; stop adding to it until the leaf exists | 86 `theorem`/`lemma` lines by grep in `clean/fork/Clean/Sherlock/Aggregation.lean` (the record says 90, `D/tasks.md:40`; not re-run here). `lin_refines_chain` (`:1454`), `genesis_collapse_both` (`:1260`), `src004_height_bridge` (`:1201`) exist and match `ivc.rs`. The Lean `Leaf` still has no slots (grep finds no slot-list lemma), so "slot semantics equal the Lean `Leaf`" (`D/tasks.md:66`) is still not a checkable sentence. |
| D11, §5 ledger 9 facts | Change (stale text) | Limits are constants, not `LedgerParameters` (concern 6). `D/design.md:142` and `D/dissent.md:177` say Preprod is on 8.1.3; `qa/sherlock/target-pins.json:21,26` records all three networks on 2.1.101 / 9.1.0.1. |
| D12, D14, M6/M7 order | Keep | Not affected by the measurements. |

## 3. Ranked concerns

### C1 (critical). `k_U` is unmeasured and the evidence leans against the plan. Touches 4.1, 4.10, 3.x order.

Facts:
- The per-rule C6 step circuits (dispatch plus one fragment) that exist today have 2.20-2.50
  million ZKIR instructions each at the fact(3) config (`research/skeleton-coding-20261108/pilot-smoke.md:28-35`;
  4,255 statement lanes, `:45`).
- One step fragment's MockProver test is set at k=18 and the mutation receipt counts 2,259,225
  advice cells there (`crates/mori-backend/src/sherlock_machine/c6_scalar_failure_step.rs:2232,2240`;
  `D/tasks.md:32`). Another runs at k=16 with 403,843 cells.
- There are about 100 supported rules, and caching 100 dispatch circuits takes about 47 GB
  (`testing.md:20`).
- The leaf hashes the PROG section three times and the pre and post carriers once each
  (`sherlock_c6_child.rs:551-569`).
- No record gives `IrSource::k()` for any dispatch or step circuit (search of `qa/sherlock/runs.jsonl`,
  `budgets.json`, docs; task 3.1 that would record it is open, `D/tasks.md:56`).

Reading (ESTIMATE): one rule's circuit is probably already near k=17-18 at the qualification
config. A universal leaf at S=1 has the shared dispatch once plus about 100 gated fragments, so
`k_U <= 16` (needed for S=16) is unlikely without re-engineering dispatch, and `k_U <= 17` is not
safe. Instruction count is not row count, so this is a risk reading, not a measurement.

Proposed change: run a lower-bound probe now, before the M3 fragment work continues. For every
existing per-rule circuit record `IrSource::model()` rows and k (modeling only, no SRS; the API is
`ledger-9.1.0.1:zkir-v3/src/ir.rs:810-822`). Lower bound for `k_U` = rows(shared dispatch) + sum
over rules of (rows(rule circuit) - rows(shared dispatch)), on the fragments that exist. If the
lower bound is already above 2^17, G-A' is decided today and the remaining fragment work (3.4) is
aimed at a different leaf design. This is task 4.1 split in two: 4.1a (lower bound, now) and 4.1b
(the full probe over frozen `R`).

### C2 (critical). At `k_U >= 18` no defined route passes G-C. Touches D4, SRC-013, SRC-014, 4.10, 5.10.

Arithmetic. Node step 61 s (A16, `D/tasks.md:44`), upper 67 s (`D/tasks.md:40`); decider 70 s.
Node count N = ceil(steps / S). Node chain alone, no leaf time, no contention:

| S | N (293 / 303) | 293 at 61 s | 303 at 61 s | 303 at 67 s | 303, node at 130 s (two workers beside it) |
|---|---|---|---|---|---|
| 1 | 293 / 303 | 17,943 s = 4.98 h | 18,553 s = 5.15 h | 20,371 s = 5.66 h | 39,460 s |
| 2 | 147 / 152 | 9,037 s = 2.51 h | 9,342 s = 2.60 h | 10,254 s | 19,830 s |
| 4 | 74 / 76 | 4,584 s = 1.27 h | 4,706 s = 1.31 h | 5,162 s = 1.43 h | 9,950 s = 2.76 h |
| 8 | 37 / 38 | 2,327 s = 0.65 h | 2,388 s = 0.66 h | 2,616 s | 5,010 s = 1.39 h |
| 16 | 19 / 19 | 1,229 s = 0.34 h | 1,229 s | 1,343 s | 2,540 s = 0.71 h |

The last column uses the only measured contention figure: three k=20 node proofs at 6 threads each
take 128-132 s (`c0-first-profile.md:57`). Using it for "node plus two leaf workers" is an
upper-bound ESTIMATE; no node-beside-leaf run exists.

Leaf time is unmeasured. `prover.md:33-41` ESTIMATE: 0.09 ms x 2^k per proof, so 94 s at k=20,
47 s at k=19, upper bound 3.3x.

- S=1 (and therefore option F): fails on the node chain alone by 2.5x. No leaf number can fix it.
- S=2: fails on the node chain alone.
- S=4 (the plan's answer at `k_U = 18`, leaf at k=20): node chain alone 1.31 h, so the 76 leaves
  must cost under (7,200 - 4,706) / 76 = 32.8 s each if run serially, or run beside the chain.
  Beside the chain the node slows to about 130 s and the chain alone is 2.76 h. Leaves first with
  three workers (76 x 130 / 3 = 3,293 s) then the chain solo (4,706 s) is 7,999 s. Fails under the
  estimates; passes only if a k=20 leaf proves in about 35 s, which nothing supports.
- S=8 (`k_U <= 17`, leaf at k=20): chain with two workers beside it about 1.39 h; passes with
  about 30% margin (ESTIMATE). Serial, 38 x (61 + 94) + 70 = 5,960 s = 1.66 h, which passes the
  2 h cap and fails the plan's own 0.8 x 7,200 = 5,760 s planning bound (`D/tasks.md:65`).
- S=16 (`k_U <= 16`): 19 x (61 + 94) + 70 = 3,015 s = 0.84 h serial. Passes with margin even at
  the 3.3x leaf upper bound (19 x (61 + 311) + 70 = 7,138 s, barely).

Two further shifts:
- `MAX_K = 19` on the leaf path (`proving.rs:22`, `sherlock_c6_child.rs:245,283`). Until that
  constant is raised and a k=20 leaf is shown to fit memory, every threshold moves down one:
  S=16 needs `k_U <= 15`, S=8 needs `k_U <= 16`.
- The fallbacks the design names for `k_U >= 18` (`D/design.md:22`, `D/dissent.md:153`) are k=19
  (failed), the tree (k=21, closed) and the GPU (unfunded). SRC-013 selects F whenever
  `k_U > 17` while SRC-014 and 4.10 prefer S=4 at `k_U = 18` (`D/requirements.md:35-36`,
  `D/tasks.md:65`); the council flagged this (`research/sherlock-speed-replan-20261108/council-review.md:409-411`)
  and the text is unchanged. Both branches fail.

Proposed change: replace SRC-013's U/F rule with one rule: the leaf is U; S is the largest power
of two with `k_U + log2 S <= K_LEAF_MAX` and S >= 8; if no such S exists the leaf design is
revisited (open owner question 3 in `D/dissent.md:182` already asks this) before any key. Record
option F as "defined, fails SRC-011 at S=1, not a fallback". Is F the better default? No.

What "revisit the leaf design" would mean (not designed here, listed so the question is concrete):
lower dispatch cost per step (decompose only the lanes a guard reads; range tables instead of
bit decomposition); hash PROG once per leaf and not three times; move the canonical-genesis check
off chain (C5); or two-level leaves (a rule-family leaf set plus a batching relation), which is a
new design and needs its own review.

### C3 (high). The node over the real leaf may not stay at k=20. Touches 4.1, 4.4, 5.3, Q11.

The node keyed over the 96-lane, k=13 C6 digest leaf models 741,708 rows
(`qa/sherlock/manifests/node.json:22`; `D/tasks.md:42`) against 652,437 over the stand-in leaf
(`D/tasks.md:40`). If both are the same `cost_model` measure, the leaf key's shape moved the node
by about 89,000 rows. The universal leaf will have a wider arch (more chips, more columns and
commitments), and the in-circuit verifier and the final collapse grow with the leaf proof's
commitment count. Headroom to 2^20 is about 307,000 rows. A node at k=21 doubles the node step
and breaks the 48 GB reading of A16. Today this is caught only at 5.3 / G-B, after the leaf is
keyed (`D/design.md:159` Q11).

Proposed change: 4.1 also reports node `cost_model` rows over a leaf VK with the universal
leaf's `ZkStdLibArch` and column counts (a modeling-only stub key of that arch), so G-A' sees both
numbers.

### C4 (high). "General purpose recursion" is narrower in the code than in the text. Touches §3.6, SRC-017, 4.3, 4.4, M6.

What the design gives, read from the code:
- One leaf key for every program **inside one frozen `Bounds`, layout and profile**. The leaf key
  identity includes `Bounds` and the layout digest (`D/design.md:54-56`); the leaf VK is a node
  constant (`ivc.rs:575`) and the node VK a decider constant (`run_decider.rs:206-216`). A program
  that needs more frames, data or value slots than the qualification config (sized to P1-P3, "in
  the tens", `docs/sherlock/stagenet-gp-recursion.md:152-154`) needs a new leaf, node, decider and
  contract key. The council's condition that every program is padded to the frozen `Bounds`
  (`council-review.md:357-363`, item 18) is not in `D/tasks.md`.
- Leaf cost is linear in the state size because carriers are flat and hashed whole each leaf.
  Larger `Bounds` raise `k_U` directly. The design has no path to large states (no Merkle or
  memory-argument carrier).
- The decider opens only `Halted` with a u128 result (`ivc.rs:86-87`, `run_decider.rs:279-288`).
  A run that ends `Rejected(code)` or `Exhausted` has no acceptance path, and a non-u128 result
  has none.
- One contract per sealed `(program_d, init_d)` (`tools/sherlock-sdk/sherlock-run.compact:25-26,40-47`).
  The machine is deterministic, so for a sealed pair exactly one `stmt` exists; `runs` can hold one
  element. Programs have no run-time input.
- It is a Sherlock-run chain only. Nothing in the node aggregates a proof of another circuit:
  the node verifies exactly one fixed leaf key and itself (`ivc.rs:549-562`). Segment composition
  in §3.6 is general over shapes of Sherlock leaves, not over relations.
- The tree shape is closed, so "any shape" is the linear chain in practice.

None of this is a defect of the recursion. It is a scope statement the text does not make:
`D/design.md:116` says "One fixed key set serves every program and every step count".

Proposed change: restate SRC-017 and §3.6 with the scope above. Decide, as an owner question,
which of these "general purpose" must include for Mainnet: (a) program input (an `input_d` lane
in `init_d` and the contract), (b) a registry contract that takes `prog_d` as an argument instead
of sealing it, (c) non-halted outcomes, (d) proofs of other relations (needs a key-table node, a
different design). (a) and (b) are contract and decider-template changes only and are cheap now,
expensive after the Mainnet deploy.

### C5 (high). Canonical genesis has no specified mechanism. Touches 4.9, 4.2, O1.

SVM-002 is conditional on `StepInv(pre)`; O1 asks that non-canonical genesis be "refused at the
leaf/node boundary" (`D/assurance.md` §1 O1; `D/tasks.md:72`). The node sees only digests
(`ivc.rs:498-501`), and the leaf does not know it is first: `h_pre` is "bound by the node, not by
the leaf" (`D/design.md:42`). The contract constructor takes `init` unchecked
(`sherlock-run.compact:29-38`). So today a deployer can seal any `init_d`, including a digest of
a non-canonical state, and every later proof is relative to it. The Lean `decider_pinning` has
the same shape (the council's muse 1.3, `council-review.md:313-319`).

Two honest options: (i) the leaf runs `decodeCanonical` when `h_pre == 0` (this is what gives the
`h_pre` lane a purpose), which adds a whole-image canonical check to `k_U`; or (ii) `init_d` is a
public deterministic function of the program image, recomputed by `verify_chain` and by anyone
reading the contract, with O1 moved to "off-chain recomputation, target observation" and no
in-circuit cost. The design should pick one before 4.2. (ii) is cheaper and is what the contract
already implies; it must then be said that an on-chain acceptance is only meaningful with the
recomputed `init_d`.

### C6 (medium). Ledger 9 fit: limits are constants; one upstream assumption contradicts the decider's size. Touches 2.5, SMN-003, SMN-006.

- `MAX_CALL_PUBLIC_INPUTS = 1 << 16` (`ledger-9.1.0.1:onchain-runtime/src/transcript.rs:87`,
  enforced `ledger-9.1.0.1:ledger/src/verify.rs:2140-2145`); `MAX_CONTRACT_OPERATIONS = 256`
  (`ledger-9.1.0.1:ledger/src/structure.rs:1928`). Both are constants. `D/design.md:152`, Q6
  (`:159`) and task 2.5 (`D/tasks.md:43`) still say "read from `LedgerParameters`". The 70-slot
  statement and the one-operation contract are far inside both.
- Verifier key bytes are capped at 50,000 (`ledger-9.1.0.1:transient-crypto/src/proofs.rs:422-431`).
  The acceptRun verifier is 2,217 B (`docs/sherlock/stagenet-gp-recursion.md:196-197`). Fits.
- The comment on the public-input cap says "`PARAMS_VERIFIER` is a 2^14 setup, so no circuit the
  ledger can verify against ... exceeds 2^14 rows" (`ledger-9.1.0.1:onchain-runtime/src/transcript.rs:80-83`).
  The decider is k=20 and is accepted by the stock and installed verifier at the 9.1.0.1 pins
  (run `20261110T032716Z-A16-fold3-decider`, `D/tasks.md:44`); no k check exists in
  `VerifierKey::verify` at the tag (`proofs.rs:563-575`). So the fit is real today but rests on
  something upstream believes is not possible. A later ledger release could add a k cap that
  matches the comment.

Proposed change: 2.5 reads the two constants from the pinned source and asserts them in a test;
A6p records "k=20 verifier key accepted by ledger 9.1.0.1" as an explicit row with the ledger
rev; the re-pin checklist (SRC-009) adds "search the new ledger for a verifier-key k or domain
cap". Fix the three stale sentences and the Preprod line.

### C7 (medium). Task 4.2 carries too much and the leaf-facing formal contract is still missing. Touches 4.2, 4.5, SRC-003, SRC-014.

4.2 must deliver, in one keyed relation: one-hot dispatch over about 100 rules, gated fragments
and candidate-post selection, S slots with the enable prefix, `rules_d`, `steps`, the derived
outcome tag and result (today constants, `sherlock_c6_child.rs:556-578`), the seventh lane, the
stale claim pins (`qa/sherlock/manifests/c6-leaf.json` `claim_pins`), padding to frozen `Bounds`,
and possibly the genesis check. The council's asks for this layer are not in the text yet: the
`rules_d` encoding for disabled slots, the `steps == 1` versus `S == 1` wording, the enable-prefix
constraint, and Lean L-B (`council-review.md:387-394`). `D/requirements.md:23` still says
"`rules_d == rule` when `steps == 1`".

Proposed change: split 4.2 into 4.2a (statement and slot contract: text, Quint `leafOk` with a
publish bit, the `rules_d` encoding; no circuit), 4.2b (one-hot dispatch and gated fragments at
S=1 on a three-rule subset, measured), 4.2c (S slots, full `R`). 4.2b gives the first real
per-rule marginal row cost, which is the number C1 needs.

### C8 (medium). Order of work. Touches M2-M5.

- 4.1 is the gate for everything and sits after M3. Move the lower-bound probe (C1) to now.
- 3.4 builds 20 more per-rule fragments as stand-alone circuits before anyone knows whether the
  universal leaf takes them in that form. Build them after 4.2b fixes the gated-fragment shape.
- 2.1 is unticked (`D/tasks.md:29`) while 2.4 and 2.6 ran at the new pins. Either tick it with
  its A1 evidence or say what part is open; 2.5 and 6.1 depend on the SDK half.
- 2.8 (overhead qualification) is still inconclusive and A16 is blocked by A15. It no longer
  decides anything: G-A is decided and the numbers are 5-10% from the bar. Close it with the
  recorded bound instead of a 60-pair run on a host that is busy.
- 2.10b-f, 5.7, 5.8 and the tree parts of 5.6 are dead text until `k_U` is known. Mark them
  "not planned" so the loop does not read them as open work.
- G-B (5.9) is the last cheap re-key point but comes after the full leaf is keyed. With 4.2b
  measured early, most of what G-B protects against is known before 4.4.

### C9 (low). Wording that will mislead a later reader.

- `D/design.md:79` still gives 95 s, 26.2 GB and 531,989 rows as "measured"; §3.5 still plans on
  `t_node < 24 s` reasoning from 95 s. Replace with the A16 numbers.
- `D/design.md:50` gives "S=8 at `k_U <= 17`" while the node has `S_MAX = 16` and G-A recorded
  provisional S = 16.
- `speed-levers.json` says the tree is "kept as fallback (closed)". Pick one.
- `ivc.rs:69-70` documents the leaf statement twice with different lane lists.

## 5. Proposed edits (text only)

`D/design.md`:

1. D4, replace the last clause: "Option F (finite family, in-circuit key membership) is defined
   but is not a fallback: it fixes S=1, and S=1 costs 293 x 61 s = 4.96 h for fact(3) at the
   measured node step (A16), which fails SRC-011. If no S >= 8 satisfies
   `k_U + log2 S <= K_LEAF_MAX`, the leaf design is revisited before any key."
2. §2.3, replace the S rule: "`K_LEAF_MAX` is the largest leaf k the leaf prover accepts and that
   fits 16 GB (today `MAX_K = 19`, `proving.rs:22`; 20 only after a measured k=20 leaf). S is the
   largest power of two with `k_U + log2 S <= K_LEAF_MAX`, at most 16. S < 8 fails G-C on the
   measured node step."
3. §3.1, replace the measured paragraph with: node `SHKIVC06` k=20, 652,437 rows over the
   stand-in leaf and 741,708 over the C6 digest leaf; step 59.6-63.6 s, 16.6-17.4 GB; decider
   70.35 s (A16).
4. §3.5 lever table: row 1 (k=19) "closed 2026-11-09, not reachable"; row 5 (tree) "closed by F22
   (k=21); reopening needs a new row probe under 0.95 x 2^20"; delete "or `k_U >= 18` / k=19
   failed / chain alone misses 2 h" from row 5. D13': status "closed".
5. §3.6, replace "One fixed key set serves every program and every step count" with: "One fixed
   key set serves every program that fits the frozen `Bounds`, layout and profile `/1`, for every
   step count below 2^64, when the run ends `Halted` with a u128 result. A different `Bounds` is a
   different key set. The node aggregates Sherlock leaves only; proofs of other relations are out
   of scope. The tree shape is closed (F22), so the shape in use is the linear chain."
6. New §3.3 sentence: "`init_d` is not checked in circuit against `prog_d`. [Option chosen in
   C5.]"
7. §5.1: Preprod runs 9.1.0.1 (observed 2026-11-10, `target-pins.json:21,26`). §5.2(3) and Q6: "the
   public-input cap (65,536) and the operation cap (256) are constants
   (`onchain-runtime/src/transcript.rs:87`, `ledger/src/structure.rs:1928` at `a2b30128`), not
   `LedgerParameters` fields." Add: "The ledger source states an assumption that verified circuits
   have at most 2^14 rows; the k=20 decider is accepted at 9.1.0.1 and this is re-checked at
   every re-pin."

`D/tasks.md`:

1. New 4.1a, placed before M3 continues: "Lower-bound `k_U` probe: `IrSource::model()` rows and k
   of every existing per-rule C6 circuit and of the dispatch block alone, at the qualification
   config; modeling only. Output: `qa/sherlock/decisions/speed-levers.json` `G-A-prime.k_U_lower`.
   If the lower bound exceeds 2^17 rows, stop 3.4 and open the leaf-design question
   (`dissent.md` open item 3)." 4.1 becomes 4.1b and adds "node `cost_model` rows over a stub leaf
   key of the universal leaf's arch".
2. 4.10: replace the S rule with edit 2 above; delete "option F is on the table".
3. 4.2: split into 4.2a / 4.2b / 4.2c as in C7. 3.4 depends on 4.2b.
4. 4.4: delete "if F, uniform arch wrapper + in-circuit key-table membership ..." or mark it "not
   planned".
5. 4.9: state the chosen genesis mechanism (C5).
6. 2.5: "assert the two ledger constants from the pinned source" instead of "read ... from
   `LedgerParameters`"; add the k=20 acceptance row.
7. 2.8: close with the recorded bound (1.017) and the label; unblock A16's dependency on A15.
8. 2.10b-2.10f, 5.7, 5.8: "not planned; reopened only by a G-A' decision".
9. New owner question in `dissent.md`: which of input, registry contract, non-halted outcomes and
   foreign proofs "general purpose" must include before the Mainnet deploy (C4).


Note added after round 2: parts of this file are superseded. See `../review.md` section 4.3 for the corrections.
