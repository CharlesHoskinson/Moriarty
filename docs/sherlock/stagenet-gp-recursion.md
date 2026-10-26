# Sherlock on stagenet with general-purpose recursion: end state and gaps

Date: 2026-10-25. Owner: sherlock-gpr. Coordinator: sherlock-fable.
Base: `factory-u-c4` `577f0470b`.
Sources: `docs/sherlock/design.md`, `qa/sherlock/predicates.yaml`
(A1-A10, B1-B9, U023, C4), the UIREQ-023 spec scenarios,
`qa/holdout/contracts/U023.json` and `qa/holdout/lib/plans.py::custom_native_qualification`
(both read-only), `docs/sherlock/stagenet-deployment.md` and
`docs/sherlock/loki-stagenet-acceptance.md` (branch `loki-stagenet`).

The owner's goal (2026-10-25) is: "Sherlock on stagenet with full general-purpose recursion."
This file states that goal as an end state that can be checked. It also lists every gap between
that end state and the code as it stands today.

## 1. End state

The end state is reached when every check E1-E9 below passes for **each** of three
committed programs:

| Program | Source | Reference result |
|---|---|---|
| P1 `fact` | `fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }`, `entry fact(3)` | 6 |
| P2 `list123` | `fn sum(xs: List): UInt { match xs { nil => 0, cons(h,t) => h + sum(t) } }`, `entry sum(cons(1,cons(2,cons(3,nil))))` | 6 |
| P3 fresh | written **after** the leaf and node keys are frozen (see E2), not shown to the circuit authors beforehand, and mixing recursion, `match` and arithmetic (for example `fib(5)` or a tree depth) | the `mori simulate` result |

All three use the `moriarty-sherlock-draft/1` source profile, which is what the holdout
reference engine runs. Each source is committed under `docs/sherlock/qualification/programs/`.

- **E1. Compile (general machine).** The public route
  `mori simulate --execution-profile sherlock-helper-microstep/1 --source-profile moriarty-sherlock-draft/1 --entry`
  admits the program under one committed qualification host config. The run halts with the
  reference value, and its owner-step event stream is the input to E3. No program-specific
  branch or literal emitter is involved.
- **E2. One fixed relation family.** Every step proof of every program uses the same leaf
  relation identity. That identity is either a single universal step key or a committed finite
  family table of keys. The node and decider keys are the same for every program. P3 gets **no**
  new key: if P3 needs a key that P1 and P2 did not use, E2 fails. The keys are committed together
  with their SHA-256 values.
- **E3. Leaf proofs.** Every owner step of the run is proved as a leaf: a C6-realized step
  (dispatch selection plus rule fragment over the `sherlock-c6-phys/1` carriers) under the Poseidon
  transcript. The leaf's public statement binds at least the program-image digest, the pre-state
  digest, the post-state digest, the rule id and the step index or height. The pre-state of step
  i+1 equals the post-state of step i. Rule coverage must be 100% of the rules the program
  exercises. A rule with no fragment means the program is unsupported. It is never skipped.
- **E4. Fold.** The leaves fold through node proofs into a single root proof. The root statement
  binds the program digest, the initial-state digest (the admitted image at height 0), the final
  state digest, the total step count and the halted result value. The tree may have any depth: the
  node relation must accept node children (by self-recursion or by fixed layers that are proven
  to cover the program's step count). It must not be a relation that only fits two fixed literal
  steps (A102/A80).
- **E5. Decider.** A terminal decider proof over the root finishes the KZG accumulator pairing in
  the circuit, as the existing `crates/mori-recursion/src/decider.rs` does. Its public transcript
  matches the called entry of the stagenet contract, including the result slot.
- **E6. Independent verification.** A verifier that is not the prover (the stock 0.8.3 verifier
  when Loki proves, or a separate process) re-verifies off-circuit the exact leaf, node and decider
  proof bytes that were submitted. It reconstructs the accumulator, checks every link
  (post = next pre, heights, program digest) and refuses mutations. This is the A5 pattern. The
  holdout checks verifier independence by name only, so this check is the real guarantee and it
  must be recorded.
- **E7. Deploy.** A Sherlock general-run contract is deployed on stagenet (ledger 9, the node, the
  SDK and the proof server unchanged). The called entry has the **hand-written decider VK**
  installed. The VK file is committed with its `midnight:verifier-key[vN]:` header. Its body
  occurs exactly once in the deploy state, at most 64 bytes after the entry name. The deploy
  transaction is indexed exactly once with `SUCCESS`.
- **E8. Call.** One `ContractCall` to that entry carries the decider proof of the program run. The
  stagenet node verifies that proof and the call succeeds. The entry writes the result into a
  `Uint<128>` ledger field. The stored value is bound by the decider statement, so a valid proof
  for a different program, input or result cannot write it.
- **E9. Readback.** From the public indexer alone (read-only, no wallet): the deploy-state result
  equals `layout.initial`, the post-call result equals the `mori simulate` reference and differs
  from the initial value, and the state SHA-256 is recorded. The pinned VK is installed for the
  entry in both states. `effects == []`.

E7-E9 are the UIREQ-023 scenario 2 holdout. For P1 they are written into
`docs/sherlock/qualification/u023-stagenet-e2e.json`, the record the holdout reads: six stages
`run`, verifier != prover, pins network stagenet / ledger 9 / node / toolchain / relation_id,
plus `state_layout_path` and its SHA-256. P2 and P3 get sibling records with the same schema
(`u023-stagenet-e2e-list123.json`, `...-p3.json`) and the same holdout-style check. E2, E3, E4 and
E6 are **stricter than the holdout**: the holdout accepts any proof under the installed key, and
those checks are what "general-purpose recursion" adds.

The end state also requires:

- A negative control on stagenet: a decider proof whose result differs, or whose program digest
  differs, is refused. The local proof-verifying ledger-9 refusal (the A6 pattern) is required.
  The stagenet refusal follows the A9 pattern.
- `doctorwatson check U023` passes on the branch that carries the record.

## 2. Status today (2026-10-25, `577f0470b`)

None of the six stages can run for a general program yet.

| Piece | State | Where |
|---|---|---|
| General machine (simulate) | **works**. fact(3) halts after 293 events (292 work, 63 helper visits). | `mori simulate` route used by the holdout |
| C6 rule fragments | source plus nonproving `ProofPreimage::check` only. fact 37/46 rules, fold 41/57. No fragment has ever had keygen, a proof or a verification. | `crates/mori-backend/src/sherlock_machine/c6_*` |
| Dispatch owner (`native_selection`) | `null`: source-only candidate | `c6_dispatch_step.rs`, `native_inventory.rs` |
| Child producer (Poseidon transcript) | works for **one** fixed A102 literal relation (k15) and A80 (k12) | `crates/mori-backend/src/sherlock_child.rs` |
| Parent | **two** children, both keys circuit constants, linked A102 to A80 only, k20, about 16.5 GB RSS | `crates/mori-recursion/src/parent.rs` |
| Node verifying a node | **missing**. The rollup `compose`/`segment` explicitly leave out the verifier halves. Fenrir `recl9/src/node.rs` is not pinned. | `~/repos/Moriarty-rollup` `3f3e6652f` |
| Decider | works (residue pairing). Template slots: free, constant, statement digest. | `decider.rs`, `template.rs` |
| Contract | `SherlockAnchor.acceptValidity(stmt)` = `validity_ok.insert(stmt)`. **No result field**, so the holdout readback cannot pass on it. | `tools/sherlock-sdk/sherlock-anchor.compact` |
| Stagenet path | deploy/call/readback harness proven: A8 block 1014763; Loki proofs accepted at blocks 1016992 (CPU) and 1024894 (GPU) | `tools/sherlock-sdk/{deploy,call}-current.mjs`, `loki-stagenet` |
| Qualification record | absent, so U023 scenario 2 = not-implemented | `docs/sherlock/qualification/` |

## 3. Architecture (proposed, decided by M1-M3)

```
mori simulate owner run  ──►  per-step C6 witness (pre/post carriers, rule)      [host]
      │
      ▼
LEAF  L(rule): dispatch(pre)=rule ∧ fragment_rule(pre,post);                     [ZKIR v3 → IrSource as
      public = (prog_d, pre_d, post_d, rule, h)   (Poseidon digests, carriers private)  zk-stdlib Relation,
      │                                                                             PoseidonState transcript]
      ▼
NODE  N: verifies 2 children (leaf key from family table, or N's own key via
      assign_vk_as_public_input); links left.post_d = right.pre_d, h contiguous;
      public = (prog_d, pre_d, post_d, h_lo, h_hi, vk_N_repr, acc)
      │   (tree; depth ⌈log2(#leaves)⌉; odd leaf padded with identity Done→Done)
      ▼
DECIDER D: verifies root N, pins vk_N_repr to the real N key, pins pre_d = genesis image digest,
      requires the final state = Halted(result); pairing in-circuit;
      transcript = acceptRun(stmt, result)  (stmt = H(SHK_RUN, instance, prog_d, init_d, fin_d, n, result))
      │
      ▼
CONTRACT SherlockRun (new Compact): acceptRun(stmt, result) { runs.insert(stmt); result_out = result }
      with the decider VK installed for acceptRun (adapter override as in A4); sealed prog_d/instance.
```

Rationale and alternatives:

- **Leaf key (M2 decides).** Option U is one universal step relation, all fragments gated by
  dispatch, giving one key. The parent then keeps keys as constants, which is the smallest change
  to `parent.rs`. Option F is a finite family: each rule is its own IrSource wrapped in a Relation
  with a **forced uniform `ZkStdLibArch` and k**. The constraint system is then identical and only
  fixed and permutation commitments differ, and the node checks the child's `transcript_repr` (and
  resolves its fixed bases) against a committed family table. Pick U if its k is at most about 17.
  Otherwise pick F.
- **Leaf statement must be digests.** At the holdout draft config (frames 256, data 512, env 512,
  value slots 8192), one carrier side is about 35k lanes: image 48+256·13, data 5·512+…, value
  slots 8192·ref. That is far above `MAX_CHILD_INSTANCE = 2^14-1`, and assigning the complete
  instance in the parent (as is done for A102) is impossible. The leaf therefore Poseidon-hashes
  pre and post inside the circuit (ZKIR `TransientHash` over the lanes) and exposes only the
  digests. Leaves also run under a tight **qualification host config** sized to P1-P3 (frames,
  data, env, value slots in the tens). It is committed, and the same config feeds E1.
- **Fold (M3 decides).** The node either self-recurses (one node key, any depth) or uses fixed
  layers (separate keys per level, which only works if each layer's k stays within the 48 GB cap).
  Node count is #leaves-1. With one step per leaf, fact(3) needs about 291 node proofs. At the A3
  cost (about 123 s keygen, about 140 s prove, 16.5 GB, serial under heavy.lock) that is roughly
  11 hours. Loki GPU and S steps per leaf (a chained leaf of S fragments) cut this: S = 8 gives 37
  leaves and 36 nodes.
- **Fallback (only if M3 shows a self-recursive node is out of reach).** Sequential anchored
  segments: the depth-2 chain (leaf, parent, decider) proves a segment of S steps and advances an
  on-chain machine-state commitment `(root, height)`, as SherlockAnchor already does. A final
  segment writes the result. This is general-purpose, but it costs one stagenet call per segment
  and does not meet E4 as written. Use it only with an explicit owner trade-off.

## 4. Measurements that gate the design (first, under the locks)

| Id | Measurement | Lane | Decides |
|---|---|---|---|
| M1 | Lane count of one C6 step statement (`C6StepWitness::to_statement_frs().len()`) at the draft config and at a tight qualification config. Rows and k of 3 typical fragments. | light | the qualification config, digest statement |
| M2 | **First real keygen, prove and verify of one C6 step relation** under `PoseidonState` (one fragment, then the dispatch plus fragment union), with k, RSS and time | heavy.lock (≤ 48 G) | leaf option U or F |
| M3 | A node relation that verifies one leaf and one node child: k, RSS and time on CPU and on Loki GPU | heavy.lock + gpu.lock | self-recursive node or layers, S |

## 5. Gap list

| # | Gap | Items | Code / step | Owner (proposed) |
|---|---|---|---|---|
| G1 | Missing rule fragments for P1/P2/P3 (fact 9, fold 16, list/match rules: BHelperMatch ×10, BHelperEqual/Validate ×8, ScalarConstruct, KAdminPublishValue) | U023 s1, C4 | `sherlock_machine/c6_*_{step,consumer,tests}.rs` | s2, s7 (Match), s4 (Equal/Validate) |
| G2 | Compiled-and-run receipts for every fragment (per-fragment filters) | U023 s1 | `testdata/receipts/*.json`, trace_copy | s6 |
| G3 | Dispatch owner `native_selection` | U023 s1 | `c6_dispatch_step.rs`, receipt `dispatch.json` | c4 |
| G4 | Leaf adapter: C6 step → Poseidon child with digest statement (pre_d, post_d, prog_d, rule, h). Measure M1/M2. Generalize `sherlock_child.rs` from A102 to any C6 IrSource. Uniform-arch wrapper if option F. | U023 s1 adapter, E3 | `crates/mori-backend/src/sherlock_c6_child.rs` (new) | **s8** (with s4), interface owned by gpr |
| G5 | Host witness stream: owner run → per-step C6 witnesses for the whole program (`materializeStep`-style), under a committed qualification config | E1/E3 | `mori-core` `c6_phys_*` + a driver | gpr (or worker) |
| G6 | Node relation N (verifies leaf or node; links digests and heights; vk_N passthrough) and measurement M3 | E4 | `crates/mori-recursion/src/node.rs` (new), reusing `inner.rs` and `child.rs` | **gpr** |
| G7 | Decider over N with the root statement and a result slot in the template | E5 | `decider.rs` (W := N), `template.rs` new slot kind `result` | gpr |
| G8 | SherlockRun contract: `acceptRun(stmt, result)` writes a `Uint<128>` result; sealed prog/instance; adapter installs the decider VK for acceptRun | E7/E8 | `tools/sherlock-sdk/sherlock-run.compact`, `deploy-run.mjs`, `call-run.mjs` from `*-current.mjs` | gpr |
| G9 | Local ledger-9 proof-verifying acceptance and refusal (A6 pattern) for acceptRun | E8 negative | `examples/validate_call.rs` | gpr |
| G10 | Independent verifier over the exact submitted bundle (A5 / `verify_chain` pattern, stock verifier when Loki proves) | E6 | `examples/verify_chain.rs` generalized | gpr / worker |
| G11 | Stagenet deploy, call and indexer readback. Compute the layout offset from the **indexed** deploy bytes, never predict it. Check `_installed_vk` locally on the deploy state before spending the call. | E7-E9 | wallet.lock session | **gpr** |
| G12 | Qualification records (P1 at the holdout path, P2/P3 siblings), committed VK file, layout file, program sources | U023 s2 | `docs/sherlock/qualification/` | gpr |
| G13 | Proving throughput for about 300 leaves and about 300 nodes per program (Loki GPU, S steps per leaf, batching) | E3/E4 time | Loki path (`Moriarty-wt-loki-stagenet`) | gpr + Loki owner |

Critical path: M2 (G4) → M3 (G6) → G7 → G8 → G9 → G11 for P1, then P2 and P3. G1-G3 and G5 run in
parallel and gate E3 coverage. Until M2 runs, everything after G4 is design, not evidence.
