# astra-2-9: independent fusion equivalence audit (direct A102+A80 terminal)

## Question
Audit the proposed direct A102+A80 terminal, which replaces proof recursion. Which exact 16-field statement, transcript, pinned-VK, context and commitment-opening obligations must the new circuit enforce to preserve the application relation? Separate proof-byte provenance from semantics. New keys are allowed; the fixed two-step fixture only. Name any fatal counterexample. No repo writes. Adversarial reading, no invented row counts.

Path roots: RL = `~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src`; MORI = `~/repos/Moriarty-factory`.

## Answer

**Verdict.** I found no fatal counterexample in the verifier-based direct terminal. It is the existing parent body (`RL/sherlock_link.rs:476-525`) moved into the decider next to the existing decider pairing (`RL/decider.rs:148-191`). It stays sound only if the obligations below are carried over exactly. Two items are blocking: the cross-graph VK/transcript check (A1), and the inlined variant, which is a different claim (A2). Nothing here is measured.

### A. Obligations the verifier-based direct terminal must keep

1. **Exact 16-field statement.** The fields are built at `RL/sherlock_link.rs:500-512`, in this order: `keys_id, a102.ctx_lo, a102.ctx_hi, a80.ctx_lo, a80.ctx_hi, program_c, pre, h_pre, mid, post, h_post, a102.bind, a80.bind, n_steps, work, trace_c`. This matches the Moriarty table (`MORI/docs/sherlock/reference/recursive-child-parent-terminal.md`, "Parent statement: exactly 16 fields"). The terminal's stmt digest is `Poseidon(SHK_STMT, instance_id, batch_no, FIELD_NAMES[0..16])` (`RL/decider.rs:193-199`; `stmt_fields = 0..16` at `RL/sherlock_link.rs:1076`). Keep the same ordering or the anchor's recomputed statement will not match.
2. **Child-key pinning (transcript and VK).** Each child is prepared under a relation constant key, `assign_fixed_vk(name, domain, cs, transcript_repr)` (`RL/sherlock_link.rs:485-489`). The transcript comes from the pinned VK, not from the proof. Names are `vk_name(prefix, vk)`, a 16-hex prefix of the VK SHA-256 (`RL/inner.rs:48-50`). Fixed-base collisions are refused by `merge_fixed_bases` (`RL/sherlock_link.rs:176-195`, collision check at `:183`), with a shared `-G` that must equal the negated generator. Keep the VKs as relation fields, never witnesses. The Moriarty `LinkPins` (`a102_vk_sha256`, `a80_vk_sha256`, context hashes, lane counts) must be checked by the terminal before keygen, not only in the reader (`MORI spec`, "Fenrir byte boundary").
3. **Context and envelopes.** A102 and A80 contexts are pinned to relation constants (`RL/sherlock_link.rs:464-466`). Envelopes are pinned to (3,1) and (0,0) (`:468-470`). Those pins make `work = a.charge + b.charge = 1` (`:507`). `work` is exposed but not asserted to 1 in-circuit, so the equality depends on the envelope pins. Removing or moving them breaks `work`.
4. **Child-instance aliasing.** A80's program lanes and pre cells are A102's (`RL/sherlock_link.rs:459`: `b = [b_head, a[program..pre], a[post..rule_tag], b_post, b_env]`). So A102.post = A80.pre and the program lanes agree. Keep this aliasing, or the mid-root and program commitment are not shared.
5. **Accumulator and pairing.** A102 is prepared first, then A80, then one accumulate, one `resolve_fixed_bases`, one collapse (`RL/sherlock_link.rs:489-493`). Discharge is the one pairing in the decider: `A, B` are re-assigned with the subgroup check (`RL/decider.rs:173-174`), tied to the encoding and both unit scalars (`:183-184`), then `e(A,[τ]2)·e(B,−[1]2)=1` (`:191`, residue method at `RL/pairing.rs:228`). Dropping either the subgroup assignment or the unit-scalar check reopens the shift-fix gap in `RL/REPORT.md` §2.
6. **Communication commitment mode.** The Sherlock decider uses `CommBinding::BoundPoseidonStmtV1` (`RL/sherlock_link.rs:1078`). In bound mode the circuit opens slot 1 as `Poseidon(comm_rand, stmt)` (`RL/decider.rs:203-209`), and `validate` requires slot 1 to be `CommunicationCommitment` (`RL/decider.rs:103-106`). A new terminal set to `Free` silently drops the commitment opening. This is a concrete trap.
7. **Anchor-side obligations (not proved in-circuit).** `h_pre` is only range-checked to 2^64 (`RL/sherlock_link.rs:502`) and not tied to the anchored predecessor height or root. Moriarty says this is an anchor obligation (`MORI spec`, "Parent statement" text and "trace" paragraph). `instance_id` and `batch_no` are free witnesses (`RL/decider.rs:195-196`), bound only through the stmt hash. The anchor must recompute stmt with its own `instance_id` and `batch_no`. None of these three move into the new circuit, so they must be preserved as anchor checks.
8. **Hardened verifier.** The shift fix (`RL/REPORT.md` §2) is required for the child MSM. The stock `assign_without_subgroup_check` path is unsafe. A direct terminal on the stock crate must not be shipped.

### B. Provenance versus semantics

- **Verifier-based direct terminal (keeps proof bytes as witnesses).** Semantics are the same as today: "the child VK's relation holds for the public instance." Proof bytes stay private witnesses, so no custody change. Provenance is only as good as the children's transcript and VK check (A1, A2).
- **Inlined variant (no child proofs; child constraints written into one circuit).** Semantics now depend on the inlined constraint system being exactly the child VK's constraint system, not a hand re-implementation. Check gate and permutation sets against `vk.cs()` for both children, and check the lookup and fixed-column sets too. Otherwise the anchor can accept a statement the real child relation does not support. This is the fatal risk to avoid. It is not verified here.
- **Witness custody (astra-2 claim disputed).** Astra-2 says the children have "public-lane witnesses," so there is no wallet custody change. The artifacts I read do not show that. `RUN-REPORT.json` (`~/repos/Moriarty-factory/docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json`) lists `instance_lanes` 253, `proof_bytes` 3440 and circuit_k 13/12 only. The 253 lanes are public instance values. The child circuit's private witness is not listed. An inlined prover needs that private witness, and the producer is the only place it lives. Confirm whether the child witness contains any private lanes before accepting "no custody change."

### C. Blocking open items

- **A1. Cross-graph VK/transcript compatibility.** Moriarty's root lock resolves midnight-proofs to both 0.7.3 and 0.8.3 (`~/repos/Moriarty-factory/Cargo.lock:2011,2031`). Recl9 is pinned to 0.8.2 (`RL/REPORT.md` §1). The Moriarty spec lists "Cross-graph 0.8.3→0.8.2 VK/transcript compatibility" as separately open (`MORI spec`, "Remaining connected work"; also says the reader "does not demonstrate cryptographic/transcript compatibility"). A terminal in the recl9 graph verifying 0.8.3-produced children is a fatal risk until closed. Both the verifier path and the transcript depend on this.
- **A2. The inlined variant needs the child constraint systems, not new code.** See B.
- **A3. Height and anchor state.** Keep the anchor checks in item A.7. The new circuit does not replace them.

## Counterexample search (no fatal found)
- Swap A102 and A80 inputs: envelope pins (`RL/sherlock_link.rs:468-470`) and `assign_fixed_vk` per name (`:485-489`) reject, because each VK is a constant.
- Substitute A80.pre: the aliasing at `:459` makes A80.pre equal to A102.post. A different A80.pre fails the equality at the verifier level.
- Reuse the same VK for both children: `merge_fixed_bases` refuses an identical prefix (`RL/sherlock_link.rs:183`).
- Off-subgroup A or B: the subgroup-checked reassignment at `RL/decider.rs:173-174` plus the unit-scalar checks at `:183-184` cover it (per `RL/REPORT.md` §3 MockProver rows).
- Comm opening dropped: fatal only if the mode is changed to `Free` (item A.6).

## Caveats
- Only read code and docs; nothing built or run. No row counts or timings are claimed here.
- Line numbers are from the files as they stand, checked with grep on the current tree.
- The Moriarty producer README did not confirm child witness custody; this is an open question, not a finding.

## Update: witness custody (code check)
- Verified `crates/mori-backend/src/sherlock_child.rs:244-310` (`child_witness`, Moriarty). The carrier refuses `ir.inputs` and communications (:249-253). It accepts only `Instruction::PublicInput` with `val_t: Native` and no guard (:264-268). It refuses `PrivateInput` (:271-274). Each statement lane is the public instance lane (:293-300, `memory.insert(id, IrValue::Native(*lane))`).
- So for this fixed fixture the child witness is exactly its public instance lanes. Arithmetic intermediates are regenerated from them. The "public-lane witness" claim is supported by code, and my earlier "custody disputed" note is withdrawn for this fixture. This is a fixture-scope result: the relation has no private inputs, so nothing about wallet privacy follows.
- Remaining check (unchanged): exact constraint-equivalence testing between the fused fragments and the child VK constraint systems (see the fatal-counterexample candidate in section B).
- Confirmed (second read): the A80 prover path calls `child_witness(&p.ir, &p.instance, pi_skips)` at `crates/mori-backend/src/sherlock_link.rs:735` before `setup_pk` and `stdlib_prove`. Both children use the same native-public carrier. This is a fixture-specific code fact.
