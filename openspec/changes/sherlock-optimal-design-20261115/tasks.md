# Tasks

Status: S2, nothing ticked. Source: FD sections 5 and 6 after the tie audit. Boxes: huginn (RTX 5080 Laptop, 100 W limit, the h185 baseline box) and odin (RTX 5090, WSL2). "Either" means no GPU timing is involved. Timing runs take the exclusive heavy slot. No GPU or power setting changes. A planned command is not evidence: tick a task only when its pass bar has a run record.

Order: 1.1 to 1.4 first (they replace ESTIMATE rows of the ladder). 2.x need no GPU and run in parallel. Prototype rekeys (section 3) decide the collection route and the blowup. Section 4 follows the engine-mutex removal. The final rekey (section 5) runs once, after FQL-006 holds and every gate has a result.

## 1. Key-neutral now: service, GPU merge and timing (T25 prototype keys)

- [ ] 1.1 Build the resident service with the `rslchain` serve loop, the key data of KCH-003 loaded under KCH-004 and KCH-005, and the wrap key cache (PRT-001). Label: removes about 167 s, DERIVED.
- [ ] 1.2 Backend-generic layer code; leaf, nodes and `L` on the GPU (GTR-001); G3 (GTR-005). Label: removes about 82 s, ESTIMATE.
- [ ] 1.3 **E-S1** One resident R_SL call on the T25 keys, merged GPU stack, stage timers, 10 warm runs, no profiler. Pass: warm median at or under 50 s (45 s once E-GAP explains the `L_Fr` gap); zero child-VK commits and zero key loads in the timed call. Box: huginn, repeated on odin. Settles C-12, C-13, KCH-003, OQ-4.
- [ ] 1.4 **E-S2** nsys GPU-busy time per stage on the E-S1 call. Pass: STARK and wrap kernel seconds reported; under 15 s combined reopens C-15 (a); above 70 percent busy means the card already limits. Box: huginn.
- [ ] 1.5 **E-GAP** `L_Fr` prove 21.8 s (h185) against 17.55 s (`t14-gpu`), both MEASURED: find the cause. Pass: cause named; resident encode on by default. Box: huginn.
- [ ] 1.6 **E-WRAPSPLIT** Wrap phase split with CPU time per phase. Pass: 6 s or more of movable CPU work admits GTR-003 and the 18 s ladder row; under 4 s drops it. Box: huginn.
- [ ] 1.7 GPU kernels against the Fr Poseidon and sumcheck interfaces in the order of GTR-002: grind, parallel trace fill, `L_Fr` sumcheck and GKR preparation. Box: huginn, checked on odin.
- [ ] 1.8 **E-G3** G3 transcript vectors on CPU prover, GPU prover and verifiers at the new `L_Fr` set, grind on a fixed witness. Pass: all legs equal (GTR-005). Box: huginn.
- [ ] 1.9 **E-ZK** T11 slot-reuse and blinder-lifetime checks on the GPU path with retry, cancel and restart. Pass: no blinder or run state carried between proofs; a production build refuses a seed override (PRT-003 to PRT-006). Box: huginn.
- [ ] 1.10 The settlement client (STC-001 to STC-003) and the retained root proof (STC-005).
- [ ] 1.11 **E-CLIENT** One call with a resident synced wallet, client stages timed apart. Pass: each stage reported; build to submit at or under 30 s; then a client target may be stated. Box: huginn (prover) with the wallet host.
- [ ] 1.12 **E-REBIND** Rebuild a real `acceptRun` and a collection transaction; diff the proofs that had to change. Pass: only `L`, `L_Fr` and wrap proofs change and the rebuilt transaction settles (STC-004); for the collection, the collector record (lane 1 `comm`, `reg_root` read) is unchanged, else S3 re-proves the collector run (STC-006). Box: huginn, Preview or devnet.

## 2. Key-neutral now: no-proving studies

- [ ] 2.1 **E-PCS step 1** Wrap cost model and D3 at `b3k3p20n19` and `b2k3p20n19` on the T25 shape with the collection terminal change in. No proving. Pass: as E-PCS (task 3.1). Box: either.
- [ ] 2.2 **E-TAPE** Op census of the `L` verifier tape by source (round-0 batching, folding, constraint DAG, stacking, LogUp/GKR, transcript) with the constant-input share. Pass: constant-input ops at least 20 percent of ALU rows and a route to `BbAluProgAir` at most 2^18 rows, program table at most 2^19, `PoseidonProgAir` at most 2^15; then the diet becomes a gate and a paired proof measures its gain. No time credited before (C-10). Box: either.
- [ ] 2.3 **E-SHAPE** `t7_chain shape` of `L` with the root tag and `DEP` head opening, of the collector statement AIR, of the extended DL-STEP node and of the completed MERGE-WORK kernel. Pass: `L` within 2,200 columns and 50 AIRs; node within 3,000 columns and 1,000 constraints per AIR; kernel within 5,400 columns and 1,000 constraints per AIR (RSH-009, CSL-005). Otherwise OQ-5. Box: either.
- [ ] 2.4 **E-CHUNK** Counterexample: swap non-initial CORE/FRAME chunks across rows on real code, existing split and new split. Pass: verdict CONFIRMED, NOT REPRODUCED or GUARD HOLDS recorded; refusal by constraints and the stock verifier; regression test kept (RSH-010). Box: either.
- [ ] 2.5 Quint model of the collector and of the normalization relation (CSL-011, FQL-005), with a Carbon lane.

## 3. Prototype rekeys (allowed; never a production set, FQL-006)

- [ ] 3.1 **E-PCS step 2** One real `L_Fr` key and timed proof at `b3k3p20n19` and `b2k3p20n19` on the resident stack, MockProver at `k_whir` 3 and `n_stack` 19. Pass: blowup 2 is adopted only if the wrap has at least 100,000 rows under the gate after the 60,000-row reserve, every D3 term is at 100 bits or more and grindable terms at 101 or more at the measured, vk and bounds shapes, 2 s or more is saved on the full resident path, and memory is no worse (LFT-006, WRB-003). Otherwise blowup 3 (LFT-005). Box: huginn, checked on odin.
- [ ] 3.2 **E-COLL** Collector end to end: 4 members including one `FOREIGN` and one `DEP`, computed `deps_d`, SEAL-WITH-DEPS, one nested level, `DEP` head opening in `L`, settled on a local devnet through `acceptCollection`; the forgery set of FQL-005; serialized VK comparison of N1, N2, N3 (KCH-006). Pass: the devnet accepts the honest collection (first ledger settlement of a non-fallback record); every forgery is refused by the stock verifier or the ledger; `L` and `L_Fr` shapes equal at N = 1 and N = 1,024 (RSH-007). Report the collector's complete cost. Any failure switches that item to its named fallback (CSL-002, CSL-009). Box: huginn.
- [ ] 3.3 **E-KEYDEP** Change the collector image; compare `L`, `L_Fr` and wrap VK bytes; check the self-read of `prog_d` for a key cycle. Pass: VKs byte-equal; no cycle (KCH-002). Box: either.
- [ ] 3.4 **E-PERM** One emulated Fr Poseidon permutation in a 2^20-row collector shard, through leaf and node. Pass: at or under 30 ms and 3,000 rows per permutation; otherwise a dedicated AIR (CSL-008). Box: huginn.
- [ ] 3.5 **E-MODES** GPU timings of SEAL, DL-STEP and SEAL-WITH-DEPS; whether SEAL takes a leaf directly. Pass: per node proof at or under 0.5 s; DL-STEP above 1 s per member reopens C-01. Box: huginn.
- [ ] 3.6 **E-PATHC** R_SL leaf on path C: key-route tests (T7c template), fixed `L` heights with the fourth child. Pass: 25 of 25 route tests; saves 0.5 s or more; `L` heights do not rise (RSH-006). Box: huginn.

## 4. Key-neutral after the engine-mutex removal

- [ ] 4.1 Remove the per-process GPU engine mutex; build the multi-leaf R_SL driver.
- [ ] 4.2 **E-S5** Concurrency sweep: 1, 2, 4, 8, 16 leaf streams on one card. Pass: marginal member at or under 2.5 s; scaling holds to 8 streams or more. Box: odin.
- [ ] 4.3 **E-BATCH** Collection sweep at N = 1, 16, 256, 1,024 under one tail. Pass: runs per hour per card at or above 1,200; critical path of the nested DL chain at or under 90 s at N = 1,024. Box: odin.
- [ ] 4.4 **E-S7** Two pipelines on one card. Pass: more than 1.5 times the calls per hour admits a second pipeline (PRT-008). Box: odin.
- [ ] 4.5 **E-ARITY** Arity-2 against arity-4 node cost on the same children, fixed point re-derived, parent chain sized. Pass: arity 4 only if `C4/3 < C2` on the complete tree and every bound holds (RSH-005). Box: odin.
- [ ] 4.6 **E-S6** Four cards against one, lone calls and a collection, tail-card layout against a mixed layout. Pass: at least 3.2 times one card (GTR-008); the tail card idle under half of a checkpoint at N = 1,000; a mixed layout that wins by 10 percent replaces GTR-006 (GTR-007). Box: the 4x RTX PRO 6000 box, which does not exist yet; a two-box proxy on odin + huginn is a scheduling test only.

## 5. The one final rekey (after FQL-006 holds and every gate above has a result)

- [ ] 5.1 Cut the final key set with the changes of FD section 6: `L_Fr` reshape (`k_whir` 3, `n_stack` 19, blowup per E-PCS, D3-minimum vector, measured `w_stack`); the tape diet only if E-TAPE passes; `L` root tag and `DEP` head opening (or the CSL-009 fallback); the collector chip set; the DL-STEP `FOREIGN` head form; path C; the X8 verifier rows of `keys_d` (GP-3) and `SHK_FCLAIM`; STACKS, VALUE DIGEST and memory in MERGE-WORK with the CORE split; EUI-010 batches 3 to 6; arity 4 only if E-ARITY passes.
- [ ] 5.2 On the final keys: D3 and G1 to G5 (FQL-001, FQL-002); FQL-003 to FQL-005; T10m, T10q, T8b and T12 gates; MockProver at the new opening shape; ledger 9.1.0.1 native check; wrap rows and VK bytes MEASURED (WRB-001, WRB-002).
- [ ] 5.3 One contract redeploy (CSL-013) and S3 on Preview with a consumer (CSL-014).
