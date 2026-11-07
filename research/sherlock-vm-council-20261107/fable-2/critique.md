# Critique: fable-2 (recursion and proving lens)

Date: 2026-11-07, 21:00Z-21:40Z. Read after all six positions were on the bus. Paths relative to `/home/hoskinson/Projects/Moriarty` at `8fd34ed79`. Where I checked a peer's source claim myself, I say so.

## Where the six positions already agree

All six keep the native route (leaf -> node -> decider -> ledger-9 contract with a hand-written decider VK), reject `VerifyProof` and ledger 10 as prerequisites, want the Cargo and SDK pins moved to the 9.1.0.1 line with every key regenerated, want A8 bound to a candidate manifest, treat stagenet as an rc-era chain (node `2.0.0-d9729c13`, `specVersion 2000000`; fable-3, astra-3 and I each read it from the RPC), want Preview as the pre-mainnet rehearsal, want the dispatch test split rather than its wall cap raised, want the C0 profile before any speedup, and want evidence classes labelled. None of this needs debate. The chair can write it as settled.

## fable-1 (chair, VM architecture)

Agreements: K1-K8; I1 (one universal one-hot leaf with S steps) is the same design as my I1/I3 and G4 of `docs/sherlock/stagenet-gp-recursion.md`; I2 (bounded dispatch receipt split into T1/T2) matches my I9 and the two-failures rule; I4 (key identity = relation id, layout schema SHA-256, bounds, ZKIR, zk-stdlib and circuits versions; `prog_d` includes the layout digest) is my I4 and I7 in a cleaner form; C1-C8.

Disagreements, with evidence:

1. **Leaf statement shape.** fable-1 I1 proposes `[prog_d, pre_d(0), post_d(S), rules_d, h_pre]`. The node already consumes seven lanes: `[binding, prog_d, pre_d, post_d, rule, h_pre, steps]` (`crates/mori-recursion/src/ivc.rs:57-66`, and the in-circuit leaf instance at `ivc.rs:352`: `vec![binding, prog_d, prev_d, cur_d, rule, prev_h, steps]`). Dropping `steps` breaks `h = prev_h + steps` (`ivc.rs:309`). Proposal: version the leaf statement as `sherlock-leaf-stmt/1 = [binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`, seven lanes, where `rules_d` is the digest of the S selected rules (fable-1's idea) in the lane the node now calls `rule`. One schema, one producer, one consumer; this also answers astra-2's six-versus-seven finding.
2. **Re-earning A8/A9/A10 and B3-B9 on stagenet (fable-1 M4).** I said the same in my position (K2 "keep A8 cheap to re-earn"). I now withdraw it. Stagenet runs `specVersion 2000000`, mainnet `2001100` (fable-3 section 6; my own RPC read). Every stagenet pass would have to be repeated on Preview after the re-key anyway. Concede to fable-3 C3 and astra-3: move the A and B network rows to Preview after M0, keep the stagenet rows as history.
3. **K5 wording.** "Digest leaf stand-in k=13" is not a stand-in; M2 keyed "one C6-shaped digest leaf (96 private lanes)" (`stagenet-gp-recursion.md:186-189`). The stand-in is `standin.rs`. Minor, but the distinction is the one astra-1 wants labelled.

Concessions: fable-1's `prog_d`-includes-layout-digest is better than my separate `cfg_d` lane; I take it, provided the qualification host config caps (frames, data, env, value slots) are part of "Bounds" in that digest. fable-1 Q5 (S copies of carrier lanes as private witness) is a real memory question I missed; it belongs in M2's measurement.

## astra-2 (adversarial soundness)

Agreements, each checked in source:

1. **Six versus seven lanes.** `crates/mori-backend/src/sherlock_c6_child.rs:817` builds six; `ivc.rs:352` and `tests/sherlock_gp.rs:184` need seven. Confirmed by astra-3 too. The bridge must prove `steps` inside the leaf key; appending a host number is not a proof. This is the first task of my M2.
2. **`rule` and `h` lanes unconstrained in the C6 digest child.** `sherlock_c6_child.rs:598-608` emits `PublicInput` + `Impact` for `rule` and `h` with no `ConstrainEq`. The universal leaf must equate `rule` (or `rules_d`) with the dispatch selector output and `h_pre`/`steps` with the chained step count. Until then the node's `rule` lane is a label, not a fact.
3. **No range check on `prev_h`, `steps`, `h` in the node.** `ivc.rs:308-310`: `assert_not_equal_to_fixed(steps, 0)`, then `h = prev_h + steps`, nothing else. The A-chain parent does it right: `parent.rs:665-670` asserts `h_pre < 2^64` and `h_pre + 2 < 2^64` with `assert_lower_than_fixed`. Fix: three `assert_lower_than_fixed(2^64)` calls on `prev_h`, `steps`, `h`. Cheap; do it in M3.
4. **Genesis carried accumulator.** `ivc.rs:328-343`: `proof_acc` is scaled by `not_genesis`, but `prev_acc` (assigned at `ivc.rs:318-327` from the witness) is folded unscaled at `ivc.rs:364`. At genesis there is no previous proof, so `prev_acc` is a free prover choice. Whether this is exploitable depends on how `verifier.accumulate` combines its inputs: if it uses a binding challenge over all three accumulators, a prover cannot cancel an invalid `leaf_acc`; if it sums, it can. I did not read `midnight_circuits::verifier::accumulate`. Either way the safe fix is one line: `scale_by_bit(not_genesis, &mut prev_acc)` as well, so genesis starts from the trivial accumulator. I ask the chair to make this a soundness requirement with a negative test (malformed carried accumulator at genesis through the final pairing).

Disagreement: astra-2's admin-publication canonicality challenge (Return reference tag or padding mutation) is in the C6 fragment layer, which is fable-1's lens; I have no evidence either way and do not oppose the test.

Concession: my position listed "a step relabelled as genesis is refused (the genesis bit)". There is no bit; genesis is `is_zero(prev_h)` (`ivc.rs:311`). A chain that restarts from `init_d` at `prev_h = 0` is a legitimate shorter run, so the relabel is harmless only because `prev_d` must equal `init_d` and `init_d` is pinned by the decider. I restate the obligation as: genesis is `prev_h = 0 and prev_d = init_d and prev_acc = identity`.

## astra-3 (independent critic)

Agreements: A8 is a deployment-specific failure, not a soundness failure (`qa/sherlock/runs.jsonl:93-94,391`); the dispatch test calls `ProofPreimage::check`, a nonproving preprocessing check (`c6_dispatch_consumer.rs:1-8`), so its receipt is not native proof evidence; measure one state and one rule first; cut a replacement zkVM, simultaneous layout candidates and speculative ISA expansion; stale pass flags must be bound to source and target.

Disagreements:

1. astra-3 writes "Compact 0.36 emits ZKIR 3.1". The scout's refresh says `zkir-3.2.0` (tag `446c7fec`, released 2026-11-04) makes IR 3.2 the default and the ledger notes say the proof server supports zkir v3 through `midnight-zkir 3.2.0` (`wiki/midnight-ledger-9-hardfork.md:96,121`; https://github.com/midnightntwrk/midnight-ledger/releases/tag/ledger-9.1.0.0). The ZKIR minor version matters for our leaf (`IrSource` under `midnight-zkir-v3`), so the manifest should record the exact IR minor the toolchain emits, measured from a compiled `.zkir`, not from a release note.
2. astra-3 Q5 asks for a complete-path budget before accepting any batch-shape optimization. My position supplies one (I1: a program of at most 320 steps proves, folds, decides and verifies under 2 h and 48 GB on the 20-core host). I ask astra-3 to accept or counter that number rather than leave it open.

Concession: astra-3's "retain the fresh stagenet submission only as an optional diagnostic" is right; see my concession to fable-1 item 2.

## astra-1 (assurance)

Agreements: mutation accounting (`qa_mutation.rs:458-486` counts synthesis failures and panics as rejected) must be fixed before any "N lanes mutated, all refused" claim counts as constraint evidence. This touches my K5 and the M4 decider record ("every one of the 70 lanes, mutated, is refused", `stagenet-gp-recursion.md:195-196`): that record predates the finding and should be re-run under ASR-003 accounting. ASR-001..006 are acceptable as written; ASR-004 (undischarged obligations refuse final-valid) is UISEC-010 restated and I support it. The evidence-class table is compatible with my five classes; I defer to astra-1's seven-row table because it separates "constraint check" from "native cryptography", which my list merged.

Disagreement: astra-1 M1 starts from "the existing literal program and the smallest recursive example" through "recursive parent" (the A102/A80 parent). The parent is the literal A chain; the general-purpose path uses the IVC node, and the two have different statements. I propose M1 use `entry 7;` (6 steps, `stagenet-gp-recursion.md:232`) through the node, so no work lands on the parent (fable-1 C3, my C2).

Concession: "a second process using the same verifier implementation checks artifact handling independently, but is not a second cryptographic implementation" is exactly right and my E6 wording should say so.

## fable-3 (delivery workflow)

Agreements: the workflow runs on `scripts/doctorwatson` (surface 1), never on the legacy plan engine; operations versus predicates (a mutating network step has an id, budget, precondition predicates, `heavy` command, post-condition predicate and authorization line); committed receipts under `qa/sherlock/receipt-traces/` in the same commit as the status flip; `target-pins.json` plus a T0 check (my I7 key manifest and fable-3's I8 pins check are one artifact with two tables); the promotion ladder `undeployed -> preview -> mainnet`.

Disagreement: fable-3 Q1 leans to "preview is the exit; mainnet is a successor change". I want the consensus change to define the T4 gate and the rollback procedure now (my M6), even if the owner executes it later, because the ledger-9 facts that drive it (price floor, `MAX_CALL_PUBLIC_INPUTS`, maintenance authority, node#2375 `VerifierKeyRemove` defect) are known today and the deploy script design depends on them (deploy `SherlockRun` with a maintenance authority or not). Compromise: T4 defined in `workflow.md` with "execution requires an owner decision and an amendment of `openspec/MORIARTY-COMPLETION-PROGRAM.md:269`"; no T4 task is runnable without that line.

Concession: I had B4/B5/B8/B9 "demoted to parallel, non-gating". fable-3's handling (they come back as preview rows, gated by T3 like everything else, not by the Sherlock GP milestones) is more precise; I accept it.

## Points I ask the chair to put in ISSUES

1. Leaf statement schema v1: seven lanes `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`, with `prog_d` committing program image, layout schema digest and bounds (including the qualification host caps). Producer, node and decider all consume this one schema.
2. Node soundness fixes before any new node keygen: range checks on `prev_h`, `steps`, `h` (`ivc.rs:308-310`, pattern `parent.rs:665-670`); `prev_acc` scaled by `not_genesis` at genesis (`ivc.rs:318-364`); `rules_d`/`steps` constrained inside the leaf (`sherlock_c6_child.rs:598-608`).
3. Network ladder: stagenet rows become history; A/B rows re-earned on Preview after M0; T4 defined, execution owner-gated.
4. Throughput budget: at most 320 machine steps in under 2 h and 48 GB on the 20-core host; S, node k19, concurrency and GPU adopted only after the C0 profile on leaf, node and decider.
5. Mutation accounting (ASR-003) applies retroactively to the M3/M4 refusal records before they are cited in `assurance.md`.
