# Recursion VM design review

Run `sherlock-recursion-design-review-20261110`. Date: 2026-11-10. Editor: `fable`. Second
reviewer: `astra` (two rounds through the Codex CLI, read-only, both exit 0).

Inputs: `BRIEF.md`; `fable/review-1.md` (independent); `astra/review-1.md` (independent);
`astra/review-2.md` (Astra's attack on the Fable review). Nothing was built, run or proved.
Every Astra claim used below was re-read in the repository by the editor, except where marked "per Astra". Round 2 found errors in
`fable/review-1.md`; they are corrected here and listed in section 4.3, and that file is left as
written.

Conventions: paths are relative to the repository root; `D` =
`openspec/changes/sherlock-vm-consensus-20261107`. Ledger citations are at tag `ledger-9.1.0.1`
(`a2b30128`) of `repos/midnightntwrk/midnight-ledger`, written `L:<path>:<line>`; the clone's
working tree is on branch `ledger-8` and was not used. ESTIMATE marks a number that is computed,
not measured.

## 1. Verdict

**Makes sense with changes.** Both reviewers reach this verdict independently. The recursion core
should be kept: the linear self-recursive node `SHKIVC06`, the run decider and `acceptRun`. Its
height, genesis-carry and terminal-opening constraints are implemented and match the revised Lean
statements, and at 8 or 16 steps per leaf the node chain for fact(3) or list123 is 19-38 node
proofs. What the evidence since the consensus does not establish is the layer the whole plan
rests on. Every measurement is for the node and decider over stand-in or 96-lane digest leaves;
the universal leaf, which carries the VM, has no row count, no proof and no settled statement
semantics. The throughput plan is a function of that one unmeasured number (`k_U`): the
estimates support gate G-C only at S >= 8; S=4 has no supporting estimate; S <= 2 misses on the
node chain alone; and no fallback named in the design is available for the bad case, because
option F fixes S=1 (about 5 h), the tree is at k=21, the k=19 cut failed, and the GPU backend is
unbuilt and unfunded with its gate metric recorded wrongly. So: keep the core; stop naming F and the tree as fallbacks;
measure the leaf now, first by a cheap diagnostic probe and then by a small real shared relation;
settle the leaf statement and the genesis mechanism before any key; and state "general purpose"
as what the code gives, which is one key set for Sherlock programs inside one frozen `Bounds` and
profile, not aggregation of arbitrary proofs.

## 2. Keep / change / stop

| Decision | Call | Evidence |
|---|---|---|
| D1, D2, D7 machine, bounded carriers, host boundary | Keep | Not contradicted by anything measured. The whole carrier is hashed in each leaf (`crates/mori-backend/src/sherlock_c6_child.rs:551-569`), which ties leaf size to `Bounds` (concern 5). |
| D3 one-hot dispatch | Keep; measure early | Dispatch range-constrains every pre-image and helper lane to 128 bits (`crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs:59,612-614`) and emits for one constant rule today (`:1317-1331`). Cost inside one shared relation is unmeasured. |
| D4 universal leaf with S steps | Keep as the only route; change the gate | S >= 8 is what the estimates support (C3). §3.4 forces F above `k_U = 17`, 4.10 allows U at 18 with S=4, 4.4 uses a third expression (`D/design.md:95`; `D/tasks.md:65,68`). |
| D4 option F as "the measured fallback" | Stop for this milestone | F fixes S=1 (`D/requirements.md:35`); S=1 is 4.98-5.66 h on the node chain alone (C3). Both reviewers: "defined, not planned for SRC-011". |
| D5 seven-lane statement | Keep; finish its meaning before keying | Producer publishes 6 lanes (`qa/sherlock/manifests/c6-leaf.json:26`), with outcome tag and value as relation constants (`sherlock_c6_child.rs:560,572`) and `rule`, `h` unconstrained (`:598-608`); `prog_d` hashes the PROG section only (`:551`) against the identity of `D/design.md:54`. `rules_d == rule` "when `steps == 1`" (`D/requirements.md:23`) differs from "for S=1" (`D/design.md:42`). |
| D6 linear node, decider, `acceptRun` | Keep | `crates/mori-recursion/src/ivc.rs:363-376,383-421,498-501,517-564`; `crates/mori-recursion/src/run_decider.rs:206,243-288`; Lean `src004_height_bridge`, `genesis_collapse_both`, `lin_refines_chain` (`clean/fork/Clean/Sherlock/Aggregation.lean:1201,1260,1454`). Node step 61.07 s, 16.58 GB; decider 70.35 s (`D/tasks.md:44`). |
| Node k=19 (lever 1, SRC-015(a)) | Stop | 652,437 rows, 128,145 over 2^19; two collapses are 78% (`D/tasks.md:40`). A16 stays a failing row. |
| `S_MAX = 16` | Change | A compile-time constant that changes constraints (`ivc.rs:77,372`) but is not in `write_relation` (`ivc.rs:571-576`), so two different bounds give the same relation bytes. Serialize it, and set it to the final S at the 4.4 re-key. F20 covers S in 1, 2, 4, 8 only (`scripts/qa/check-f20.sh:29`). |
| D13' three-slot tree | Stop; record as closed | 1,078,147 rows, k=21 (`D/tasks.md:36`). `speed-levers.json:60` says "kept as fallback (closed)". Reopening needs a different node and a new row probe. |
| GPU track (§3.7) | Change the gate metric; no credit until built and measured | The decision sums m + q = 0.83 (`qa/sherlock/decisions/speed-levers.json:67`). The profiler says q "overlaps m, so m_or_q is the union and m + q is never summed" (`scripts/qa/profile-recursion.py:873-877`); the same JSON line gives the union as 0.77, and q is from a different run (`speed-levers.json` `q_note`). The union still clears the 0.4 gate, so the track is open on paper; but q includes quotient work that an NTT kernel does not remove, the MSM-only Amdahl bound is 1.34x, and the backend is unbuilt and unfunded (`D/dissent.md:181`). |
| P = 3 leaf workers | Keep provisionally; requalify | The only contention data is three genesis node provers at 6 threads (`wiki-llm/recursion-perf/profiles/c0-first-profile.md:57`), under a 60 GiB cap with the host scope at 72 G (`speed-levers.json:70`). No node-beside-leaf run exists. |
| SRC-010 independent verifier (`verify_chain`) | Change: it does not exist for this chain | `crates/mori-recursion/examples/verify_chain.rs:89,134` loads the historical A102/A80 pair and asserts lengths (419, 419, 28). The GP harness writes only the last node proof (`crates/mori-recursion/tests/sherlock_gp.rs:289-291`) and checks with `VerifierKey::verify` (`:330-341`), not `validate_call`. |
| SRC-017 / §3.6 wording | Change | Concern 5. |
| F20 / F21 | Keep; add only the leaf-facing part | Lean `Leaf` is boundary fields only (`Aggregation.lean:35`); Quint slots have no publication field (`models/sherlock/quint/aggregation.qnt:99-118`). 86 `theorem`/`lemma` lines by grep; the record says 90 (`D/tasks.md:40`); not re-run. |
| §5 ledger 9 text | Change (stale) | Concern 7. `D/design.md:142,145` describe Preprod on 8.1.3 and the old workspace pins; `qa/sherlock/target-pins.json:21,26` records Preview, Preprod and Mainnet on 2.1.101 / 9.1.0.1. |
| G-C (2 h, 48 GB, 100 GB) | Keep unchanged | `D/requirements.md:33`. A recorded shortfall is not a pass. |
| D8, D12, D14, M6/M7 order | Keep | Not affected. |

## 3. Ranked concerns

Ranking is the editor's. Astra ranks C2 first and C1 second; see 4.2.

### C1 (critical). The universal leaf's size is unmeasured and every later gate depends on it. Tasks 4.1, 4.10, 3.4, 5.3.

- No record holds rows or k for any dispatch or step circuit. Task 3.1 records build time,
  check time and instruction count, not rows (`D/tasks.md:56`); 4.1 is open (`:64`).
- What exists is indirect and does not fix a number: reference per-rule circuits of 2.2-2.5
  million ZKIR instructions for the two rule families the pilot measured
  (`research/skeleton-coding-20261108/pilot-smoke.md:28-35`); a 128-bit constraint on every image
  and helper lane in dispatch (above); 108 supported rules per Astra's source count (not re-counted by the editor)
  (`c6_dispatch_step.rs:140,214,360`; "about 100", `research/skeleton-coding-20261108/testing.md:21`).
  The editor's round-1 reading "one rule circuit is probably near k=17-18" is withdrawn as
  unsupported (4.3). The fair statement: the evidence shows a cost worth measuring before
  anything else and gives no reason to assume `k_U <= 16`.
- The envelope is leaf plus node plus decider, not `k_U + log2 S`. The node models 652,437 rows
  over the stand-in leaf and 741,708 over the k=13 C6 digest leaf, the same `cost_model` row field
  (`speed-levers.json:22`; `qa/sherlock/manifests/node.json:22`;
  `crates/mori-recursion/tests/sherlock_keygen_regression.rs:81,132`): +89,271 rows from the leaf
  key alone. The decider over that node models 929,166 rows
  (`qa/sherlock/manifests/run-decider.json:20`), 119,410 ordinary rows under 2^20. A wider leaf
  key can push the decider to k=21 while the leaf itself fits. Today this is seen only at 5.3.
- The leaf prover path refuses k above 19 (`crates/mori-backend/src/proving.rs:22`;
  `sherlock_c6_child.rs:868,880`). This is a wrapper constant, not an upstream limit, but
  `k_U + log2 S <= 20` is not usable until it is raised and a k=20 leaf is shown to fit 16 GB.

Proposed change: (a) now, a diagnostic probe: rows, tables, arch and minimum k from
`IrSource::model()` (`L:zkir-v3/src/ir.rs:936-948`) for every existing per-rule circuit and for
the dispatch block alone, at one fixed qualification config; (b) next, a real shared relation on
a small rule subset (dispatch once, gated fragments, candidate-post selection) to get the
marginal row cost per rule; (c) at 4.1, node and decider `cost_model` rows over a key of the
universal leaf's arch. (a) and (b) are projections, not bounds; only the full relation over
frozen `R` closes G-A'.

### C2 (critical). The leaf-to-run soundness boundary is unfinished. Tasks 4.2, 4.3, 4.5, 4.6, 4.9, 5.2.

Not an attack on the node; an unfinished obligation that is larger than "add the `steps` lane".
- The C6 bridge takes pre and post outcome tag and value as constants (`sherlock_c6_child.rs:560,572`);
  the decider trusts the tag and result inside `fin_d` (`run_decider.rs:279-288`). The universal
  leaf must derive them from constrained carriers (SRC-006).
- `rule` is published without a tie to dispatch (`sherlock_c6_child.rs:598-608`); `prog_d` is
  not yet the identity of `D/design.md:54`; the claim pins are stale
  (`qa/sherlock/manifests/c6-leaf.json:2-6`).
- Canonical genesis has no specified mechanism. The node checks `prev_d == init_d` only
  (`ivc.rs:498-501`); the constructor checks `init != 0` only
  (`tools/sherlock-sdk/sherlock-run.compact:29-38`); Lean `leaf_sound` and `decider_pinning` take
  a carrier state opening `init_d` as a premise (`Aggregation.lean:204-213,1110-1120`). O1 and 4.9
  say "refused at the leaf/node boundary" (`D/assurance.md:11`; `D/tasks.md:72`) without saying
  which circuit sees the lanes. Initialization also takes fuel and a stack limit
  (`crates/mori-core/src/sherlock/reference.rs:14`), so "a function of the program image" is not
  enough.
- The `rules_d` encoding for disabled slots, the enable-prefix constraint and the terminal-last
  rule are still open council asks (`research/sherlock-speed-replan-20261108/council-review.md:387-394`),
  and padding every program to the frozen `Bounds` (`:357-363`) is in no task.

Proposed change: write the statement and slot contract first (one `rules_d` encoding, enable
prefix, publication as last enabled slot, outcome derivation, `prog_d` identity), choose the
genesis mechanism (dissent 2 in section 4.2), and require fresh malicious-witness cases for forged outcome, selector,
count, internal link and initial carriers at the keyed boundary.

### C3 (critical). Throughput: only S >= 8 is supported by the estimates, and no fallback is available. Tasks 4.10, 5.5, 5.6, 5.9, 5.10; D4, SRC-013, SRC-014.

Arithmetic. N = ceil(steps / S). Node step 61 s (A16) to 67 s (SHK23), decider 70 s
(`D/tasks.md:40,44`). Node chain plus decider only; no leaf proving, setup, verification, call
validation or contention. All ESTIMATE from stand-in-leaf timings.

| S | N fact(3) / list123 | fact(3), 61-67 s | list123, 61-67 s |
|---|---|---|---|
| 1 | 293 / 303 | 17,943-19,701 s (4.98-5.47 h) | 18,553-20,371 s (5.15-5.66 h) |
| 2 | 147 / 152 | 9,037-9,919 s (2.51-2.76 h) | 9,342-10,254 s (2.60-2.85 h) |
| 4 | 74 / 76 | 4,584-5,028 s (1.27-1.40 h) | 4,706-5,162 s (1.31-1.43 h) |
| 8 | 37 / 38 | 2,327-2,549 s (0.65-0.71 h) | 2,388-2,616 s (0.66-0.73 h) |
| 16 | 19 / 19 | 1,229-1,343 s (0.34-0.37 h) | 1,229-1,343 s |

Both reviewers computed this table independently and the numbers agree.

- S=1 and S=2 miss 7,200 s before any leaf is proved. F at S=1 would need a node step under
  (7,200 - 70) / 303 = 23.5 s.
- S=4 (the plan's answer at `k_U = 18`, leaf at k=20). Leaf time is unmeasured; the planning
  figures are 94 s nominal and about 315 s stress for a k=20 leaf
  (`research/sherlock-speed-replan-20261108/prover.md:33-41`). Serial, list123 is
  76 x (61 + 94) + 70 = 11,850 s. Leaves first on three workers at the 130 s measured for
  concurrent k=20 node proofs: 26 waves x 130 = 3,380 s, plus the solo chain 4,706 s, is 8,086 s.
  Overlapped, the node step under contention must stay below (7,200 - 70) / 76 = 93.8 s with
  leaves keeping up. No available estimate supports selecting S=4. It is not proved impossible.
- S=8 (leaf at k=20 if `k_U = 17`): serial 38 x (61 + 94) + 70 = 5,960 s (1.66 h); under 2 h,
  over the plan's 0.8 x 7,200 = 5,760 s bound (`D/tasks.md:65`). Needs leaf workers to have margin.
- S=16 (`k_U <= 16`): serial 19 x (61 + 94) + 70 = 3,015 s (0.84 h). At the 315 s stress leaf it
  is 19 x (61 + 315) + 70 = 7,214 s, over the cap, and 20 x (61 + 315) + 70 = 7,590 s at the
  320-step limit of SRC-011.
- The measured run shows overhead outside the proofs: the A16 fold process took 464 s for 253 s
  of proving (`speed-levers.json:102-109`). Proof-only margins of a few percent are not margins.
- f3 = 1.69-1.74 compares three 6-thread proofs with a 6-thread solo (75.77 s); against the
  20-thread solo each concurrent proof is about 2.1x slower. It is the only contention figure and
  is not a node-beside-leaf measurement.
- The fallbacks named for the bad case are all closed: k=19 (failed), tree (k=21), F (S=1); the
  GPU backend is unbuilt and unfunded and gets no credit until measured. `D/dissent.md:183` already lists this as an open owner
  question.

Proposed change: one rule. The leaf is U. S is the largest power of two, at most 16, for which
the measured leaf, node and decider all fit their k and memory caps; S >= 8 is the planning
target; S <= 2 is a redesign trigger; S=4 needs a measured leaf time before it is chosen. If no
S >= 4 fits, the leaf design is revisited before any key. G-C stays as written.

### C4 (high). No independent verifier or full call validation exists for this chain. Tasks 5.1, 2.5. (Found by Astra; verified.)

SRC-010 and task 5.1 name `verify_chain` (`D/requirements.md:32`; `D/tasks.md:77`). The example of
that name verifies the retired A102/A80 chain (`verify_chain.rs:89,134`). The GP harness keeps
only the final node proof and calls `VerifierKey::verify` (`sherlock_gp.rs:289-291,330-341`). The
run template predates the re-pin and its compiler version is unobserved
(`qa/sherlock/manifests/sherlock-run-contract.json:2,10-13`; `D/tasks.md:43`). G-C includes
`verify_chain` and `validate_call` in the 2 h, so their cost is also unmeasured.

Proposed change: split 5.1 into (a) a versioned GP bundle that keeps every leaf and node artifact,
(b) an independent verifier over that bundle, (c) full local `validate_call` with the recaptured
template. Do not tick SRC-010 through the legacy example.

### C5 (high). "General purpose recursion" is narrower than the text. §3.6, SRC-017, 4.3, 4.4, M6.

What the code gives:
- One key set for every Sherlock program **inside one frozen inventory, layout, `Bounds`,
  profile and S**. `Bounds` and the layout are part of the leaf key identity (`D/design.md:54-56`);
  the leaf key is a node constant (`ivc.rs:575`) and the node key a decider constant
  (`run_decider.rs:206-216`). The qualification config is sized to P1-P3
  (`docs/sherlock/stagenet-gp-recursion.md:162-163`). A program that needs more capacity needs a
  new key closure and a new contract key. The full-carrier hash grows with carrier length, so
  larger `Bounds` add leaf work; there is no design for large states.
- Sherlock leaves only. The node verifies one fixed leaf key and itself (`ivc.rs:548-562`);
  linked segments share one `prog_d` (`Aggregation.lean:49`). Nothing aggregates a proof of
  another circuit or of two programs. That needs authenticated circuit identity, a key policy
  and statement adapters: a different design.
- Only `Halted` with a u128 result is accepted (`run_decider.rs:278-288`;
  `crates/mori-backend/src/sherlock_c6_witness_stream.rs:138-158`). `Rejected`, `Exhausted` and
  structured results have no path.
- One contract per sealed `(program_d, init_d)` (`sherlock-run.compact:25-26,40-47`), and no
  run-input interface (`sherlock_c6_witness_stream.rs:170-177`). Under the soundness premises the
  Lean uniqueness result (`Aggregation.lean:1136`) means at most one valid `stmt` per contract.
- "Any shape" is the linear chain in practice. The tree is a different node statement and key
  (`D/tasks.md:83`), and it is closed; `D/design.md:79` "the statement, decider and `acceptRun`
  are the same" is too broad.
- Runs of at least one step (`ivc.rs:359-372`).

Proposed change: restate SRC-017 and §3.6 in three scopes (admitted Sherlock programs; other
aggregation shapes for Sherlock segments; arbitrary-circuit aggregation as a separate
extension), and put one owner question before the Mainnet deploy: which of run input, a registry
contract taking `prog_d` as an argument, non-halted outcomes, and foreign proofs are in scope.
Run input is not a template-only change: it needs admission, initialization, binding and witness
production.

### C6 (high). Task order has cycles and freezes interfaces after keying. M3-M5.

- 4.1 probes a relation over frozen `R` that 4.2 builds and 4.4 freezes; 4.10 picks S from leaf
  time and RSS that 4.4 measures; `prog_d` (4.3), the bridge (4.5) and initialization (4.9) come
  after the keying task (`D/tasks.md:64-72`). The council raised this (`council-review.md:404-408`).
- 3.4 produces more stand-alone fragments and receipts (`native_inventory.rs:638` `RECEIPTED` is
  empty) before the shared-relation shape is known.
- The one-hot selector model sits in 5.4, after the chain is built (`D/tasks.md:84`).
- 2.1 is unticked while 2.4 and 2.6 ran at the new pins (`D/tasks.md:29,42,44`; `Cargo.toml:28`).
- 2.8 has not met its 1.01 criterion (bound 1.017) and blocks A16 through A15 (`D/tasks.md:34,44`).
  It needs a lead decision: change the criterion or keep the label; another long run on a busy
  host does not change any design choice.
- 2.10b-f, 5.7, 5.8 and the tree parts of 5.6 read as open work but are closed.

Proposed order in section 5.

### C7 (medium). Ledger 9 fit is fine; the text names the wrong sources. Tasks 2.5, SMN-003, SMN-006.

- `MAX_CALL_PUBLIC_INPUTS = 1 << 16` (`L:onchain-runtime/src/transcript.rs:87`; enforced
  `L:ledger/src/verify.rs:2138-2145`) and `MAX_CONTRACT_OPERATIONS = 256`
  (`L:ledger/src/structure.rs:1928`) are source constants. `D/design.md:152,159` and
  `D/tasks.md:43` say "from `LedgerParameters`". The 256 cap counts operations stored in a
  contract, not machine steps. Transaction bytes, block limits and costs are `LedgerParameters`
  fields and must be read from the target (`L:ledger/src/structure.rs:1265-1307`).
- Raw verifier-key bytes are capped at 50,000 (`L:transient-crypto/src/proofs.rs:422-431`); the
  acceptRun verifier is 2,217 B (`docs/sherlock/stagenet-gp-recursion.md:196-197`). The 70-slot
  statement is far under 65,536.
- A comment says `PARAMS_VERIFIER` "is a 2^14 setup, so no circuit the ledger can verify against
  ... exceeds 2^14 rows" (`L:onchain-runtime/src/transcript.rs:80-83`). It is wrong as
  cryptography: verifier parameters hold G2 elements only
  (`tools/profile/vendor/midnight-proofs-0.8.3/src/poly/kzg/params.rs:235-237`), `VerifierKey::verify` has no k check
  (`L:transient-crypto/src/proofs.rs:563-575`), and the k=20 decider verifies at these pins on
  the stand-in chain (`speed-levers.json:95-109`). No incompatibility. It is worth one line in
  the re-pin checklist because it shows what upstream believes (see 4.2).
- No full-transaction acceptance exists yet for this chain (C4).

### C8 (medium). Formal results cover composition, not the leaf. Tasks 2.11, 2.12, 4.2.

`valid_accumulate` and leaf soundness are premises (`Aggregation.lean:175-213,1244`); the
IVC06 model is model evidence by its own report (`qa/gemini-quint/ivc06/REPORT.md:194-198`). The
sentence "the slot semantics equal the `Leaf` of the Lean theorems" (`D/tasks.md:66`;
`D/requirements.md:36`) is not checkable while Lean has no slots. Add S=16 to F20 if 16 stays a
candidate.

### C9 (low). Stale or contradictory text.

`D/design.md:79` (95 s, 26.2 GB, 531,989 rows) and §3.5's reasoning from 95 s; `D/design.md:50`
"S=8 at `k_U <= 17`" beside `S_MAX = 16` and provisional S = 16; "kept as fallback (closed)";
`ivc.rs:69-70` documents the leaf statement twice with different lanes.

## 4. Agreement and dissent

### 4.1 Agreed (independently in round 1, or after round 2)

- Verdict: makes sense with changes; keep the hardened linear node and the decider.
- The node-only table of C3, number for number.
- Option F is not a fallback for SRC-011. Astra's round 1 called it "a useful correctness
  fallback" and withdrew that in round 2; it may stay as a diagnostic or as the start of a
  separately reviewed batching design.
- The tree is closed and must not be reopened by a chain shortfall.
- k=19 by small trims is over.
- "General purpose" must be restated; arbitrary-circuit aggregation is not in this design. Astra
  confirmed the editor's additions (fixed `Bounds`, no non-halted outcomes, no run input) as
  correct and material.
- Ledger limits are constants, the Preprod line is stale, sizes fit.
- A diagnostic row probe of existing circuits should move ahead of further fragment work (Astra
  changed its order in round 2), and a small real shared relation is the next measurement.
- S=4 cannot be selected on present estimates (Astra withdrew "plausible but limited margin";
  the editor withdrew "fails").
- The leaf statement, `rules_d` encoding and genesis mechanism must be settled before final keys.

### 4.2 Dissent kept

1. **What is first.** Editor: leaf feasibility (C1) is first, because a leaf that does not fit
   makes the soundness work on that leaf moot, and the probe costs hours. Astra: leaf soundness
   and authenticated initialization are first, as the highest-consequence gap; feasibility is
   second. Both agree the probe need not wait for the soundness work. The ranking in section 3
   is the editor's.
2. **Canonical genesis.** Editor: off-chain recomputation of `init_d` by `verify_chain` and by
   readers is an acceptable mechanism and avoids adding a whole-image canonical check to the
   leaf, if O1 and 4.9 are rewritten to say so. Astra: it is sound only for a verifier that
   performs the recomputation; any reader or downstream contract that treats an `acceptRun` as
   proof of a canonical run can be fooled, because the constructor accepts any nonzero digest;
   for an unauthenticated `init_d` the Lean premise may not hold at all, so not even "a valid
   run from the supplied state" follows; and it does not meet O1 as written. Astra prefers the
   first-leaf or another authenticated mechanism. Not resolved; it is an owner-level choice of
   product guarantee, to be made before 4.2 because it changes `k_U`.
3. **Lean before the leaf.** Astra: a Lean refinement from S-slot execution to the boundary
   `Leaf` should exist before the full leaf is implemented. Editor: the written slot contract and
   the Quint slot model are enough to start; the Lean refinement can follow the small shared
   relation, and until it exists the text must say Lean proves boundary composition only. Astra
   accepts that wording as an alternative, but still favours the refinement first.
4. **The 2^14 comment.** Editor: keep it as a named re-pin check, since it records an upstream
   belief that no verified circuit is that large. Astra: a misleading comment with no
   cryptographic content; a future cap is speculation and should not be rated a risk.
5. **How strong the `k_U` warning is.** Editor: the indirect evidence leans against a small
   `k_U`. Astra: the evidence identifies a cost to measure and supports no direction. Section 3
   uses Astra's wording for the facts and keeps the editor's priority.
6. **2.8.** Editor: decide it by lead decision now. Astra: a bound of 1.017 does not meet 1.01;
   the criterion may be changed or retired, but it must not be recorded as passed. The edit in
   section 5 follows Astra.

### 4.3 Corrections to `fable/review-1.md` from round 2 (all verified)

- `IrSource::model()` is at `L:zkir-v3/src/ir.rs:936-948`, not `:810-822` (that was the
  `ledger-8` working tree).
- "One rule circuit is probably near k=17-18" is withdrawn. The k=18 MockProver test passes
  `Some(18)` as a chosen parameter on a small fixture (`c6_scalar_failure_step.rs:1849,2232`);
  advice cells are not rows; the 2.2-2.5 M instruction counts cover two rule families.
- The proposed "lower bound" formula is a projection, not a bound: `cost_model` takes the maximum
  over row kinds and standalone circuits repeat work a shared relation reorganises.
- "S=4 fails" is "S=4 is unsupported by any estimate"; leaves-first needs ceil(76/3) = 26 waves
  (8,086 s, not 7,999 s); the 130 s figure is not an upper bound for node-beside-leaf.
- `MAX_K = 19` is a wrapper constant; the relevant checks are `sherlock_c6_child.rs:868,880`.
- "A contract can accept exactly one run" is "at most one valid `stmt` under the soundness
  premises"; the contract has no capacity check.
- `h_pre` has a stated purpose (the node binds it to the previous height).
- "Leaf cost is linear in state size" is "the carrier hash grows with carrier length".
- Run input is not a cheap template-only change.
- 3.4 is fragments and receipts, not twenty new fragments.
- GPU: round 1 repeated the m + q = 0.83 premise; it is a double count.
- Citations: `testing.md:21`; `stagenet-gp-recursion.md:162-163`.
- "Sound as designed, built and cheap enough" overstated the evidence; section 1 is reworded.

### 4.4 What each reviewer found alone

Astra: the legacy `verify_chain` and the missing bundle (C4); the GPU double count; decider
headroom (929,166 rows); `S_MAX` missing from the serialized relation; F20 stops at S=8; `prog_d`
hashes PROG only; `RECEIPTED` empty. Editor: the leaf prover's k cap; the consequence of fixed `Bounds` for key identity and state
size (Astra also named fixed bounds); no non-halted outcome and no run input; the upstream 2^14
comment. Both noted the 2.1 status.

## 5. Proposed edits (text only; no file was changed)

### `D/design.md`

1. **D4**, replace the last clause: "Option F (finite family, in-circuit key membership) is
   defined and not planned: it fixes S=1, and at the measured node step S=1 is about 5 h on the
   node chain alone, which fails SRC-011. It may be used as a diagnostic. A mixed-rule batched
   family is a new design with its own review."
2. **§2.3 and §3.4**, replace the S and U/F rules: "The leaf is U. S and the worker count are
   chosen at G-A' from measurements of the complete admitted relation and of the node and
   decider keyed over a leaf of its arch. `k_U + log2 S <= K_LEAF_MAX` is a first sizing
   estimate; `K_LEAF_MAX` is the largest leaf k the leaf prover accepts within 16 GB (the wrapper
   cap is 19 today). S >= 8 is the planning target. S=4 may be chosen only with a measured leaf
   time. S <= 2 does not meet G-C at the measured node step and is a redesign trigger."
3. **§2.2(6), §3.2(3), SRC-001**: one `rules_d` encoding with its domain, the disabled-slot id
   and the single-enabled-step case; the enable-prefix constraint; publication as the last
   enabled slot; outcome tag and result derived from constrained carriers; `prog_d` as defined
   in §2.4.
4. **§3.2 / §3.3**, add: "Canonical initialization: [first-leaf check selected by `h_pre == 0`]
   or [off-chain recomputation of `init_d` from program, configuration, fuel and stack limit by
   the independent verifier; an `acceptRun` is then a claim relative to the sealed `init_d`, and
   O1 is restated]. Equality with a supplied `init_d` alone establishes neither." (Dissent 2.)
5. **§3.2(1)**: "`S_MAX` is a serialized field of the node relation; changing it changes the
   relation bytes and triggers the key cascade."
6. **§3.1**, replace the measured paragraph: node `SHKIVC06` k=20, 652,437 rows over the stand-in
   leaf and 741,708 over the C6 digest-leaf key; decider 929,166 rows over that node; node step
   59.6-63.6 s, 16.6-17.4 GB, decider 70.35 s, all with stand-in leaves (A16). Remove "the
   statement, decider and `acceptRun` are the same" for the tree.
7. **§3.5**: lever 1 "closed 2026-11-09, not reachable by trims"; lever 5 and D13' "closed by F22
   (k=21); a chain shortfall does not reopen it; reopening needs a different node and a new row
   probe"; lever 4 "gate on the union share and on exclusive, offloadable phase shares from one
   qualified run; m and q are never summed; no credit in G-C until measured". Add the node-only
   table of C3 and the projection rule: total wall = setup + max(node service, leaf service)
   under measured mixed contention + decider + independent verification + call validation.
8. **§3.6**, replace "One fixed key set serves every program and every step count": "For a fixed
   rule inventory, layout, `Bounds`, profile and S, the leaf, node and decider keys do not depend
   on the program, the initial state or the run length (at least one step, below 2^64). A
   different `Bounds` is a different key set. A run is accepted only when it halts with a u128
   result. A deployed `acceptRun` instance binds one program and one initial state. The node
   aggregates Sherlock leaves only; aggregation of other circuits is a separate extension. The
   shape in use is the linear chain." Add: "Lean proves boundary composition under premises
   (canonical initial state, leaf soundness, digest binding, accumulator soundness); the slot
   contract is specified in Quint until a Lean refinement exists."
9. **§5.1**: Preview, Preprod and Mainnet on node 2.1.101, ledger 9.1.0.1 (observed 2026-11-10);
   workspace at `a2b30128`, circuits 7.3.0, zk-stdlib 2.4.0. **§5.2(3) and Q6**: "The
   public-input cap (65,536) and the contract-operation cap (256) are source constants at
   `a2b30128`. Transaction-byte, block and cost limits are read from the target
   `LedgerParameters`. The raw verifier-key limit is 50,000 bytes."

### `D/tasks.md`

1. **New 4.1a (now, before more of 3.4)**: "Diagnostic row probe: rows, table rows, arch and
   minimum k from `IrSource::model()` for every existing per-rule C6 circuit and for the dispatch
   block alone, at one recorded qualification config. A projection, not a bound; written to
   `speed-levers.json` `G-A-prime.probe`."
2. **New 4.2a (statement and slot contract, no circuit)**: design edits 3 and 4, the Quint slot
   model with a publication bit and the one-hot selector model moved here from 5.4, and the
   padding of every admitted program to the frozen `Bounds`.
3. **New 4.2b**: "A real shared relation on a small rule subset at S=1: dispatch once, gated
   fragments, candidate-post selection; record marginal rows per rule." 3.4's remaining
   fragments are written in the 4.2b shape.
4. **4.1 (after 4.2b)**: full row probe at candidate S values, plus node and decider
   `cost_model` rows over a key of the universal leaf's arch (the structural part of 5.3).
5. **4.10**: design edit 2; delete "option F is on the table"; inputs labelled measured or
   forecast.
6. **4.3, 4.5, 4.9** move before 4.4. **4.4**: mark the F branch "not planned"; serialize
   `S_MAX`.
7. **5.1** split: 5.1a GP bundle with every leaf and node artifact; 5.1b independent verifier
   over the bundle (not `examples/verify_chain.rs`); 5.1c full local `validate_call` with the
   recaptured template.
8. **2.5**: "assert the two ledger constants from the pinned source; read transaction limits
   from the target parameters; record the k=20 verifier-key acceptance with the ledger rev".
9. **2.8**: lead decision recorded: either the 1.01 criterion is changed with a reason or the
   rows keep the overhead-unqualified label; it is not recorded as passed.
10. **2.1**: tick with its A1 evidence or state what is open.
11. **2.10b-2.10f, 5.7, 5.8, tree parts of 5.6**: "not planned; reopened only by a recorded gate
    decision with new evidence". 2.15's record: tree "closed"; GPU gate metric corrected.
12. **2.11**: add S=16 to the F20 sweep if 16 remains a candidate.

### `D/dissent.md` (owner questions to add)

1. The genesis guarantee: in-circuit first-leaf check, or off-chain recomputation with a
   restated O1 (dissent 2).
2. Which of run input, a registry contract, non-halted outcomes and foreign-circuit aggregation
   "general purpose" must include before the Mainnet deploy (C5).
3. Open item 3 of 2026-11-08 restated: if no S >= 4 fits, the leaf design is revisited; F, the
   tree and the GPU are not available answers today.
