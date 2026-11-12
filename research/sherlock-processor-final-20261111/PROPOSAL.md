# Sherlock co-processor: the merged proposal

Sherlock is a universal, elastic proving co-processor for Midnight. A user runs any program on the Sherlock machine, proves the run as a tree of fixed-size small-field STARK shards on the user's own GPU, compresses the tree into one last STARK layer `L` hashed over BLS12-381 Fr, and wraps `L` in a blinded midnight-proofs PLONK proof that the ledger 9.1.0.1 verifier checks. Ledger 9 refuses a key with the G4 lane gates (check M7-0, MEASURED), so the ledger-readable form of the wrap is the open owner decision OD-21 (section 9).

## Conventions

Path prefixes, all under `~/Projects/`:

| Prefix | Path |
|---|---|
| `B/` | `Moriarty/research/sherlock-stark-bls-bridge-20261111/` |
| `LEAN` | `research/sherlock-stark-bls-bridge-20261111/LEAN.md` at commit `bdb92973a` of branch `bridge-lean` (`git show bdb92973a:<path>` in `Moriarty-wt/bridge-lean`) |
| `F/` | `Moriarty/research/sherlock-stark-framework-20261111/` |
| `CT/` | `Moriarty/research/sherlock-call-time-opt-20261111/` |
| `SP/` | `Moriarty/research/sherlock-shard-preprocessing-20261111/` |
| `PR/` | `Moriarty/research/sherlock-design-paper-review-20261111/` |
| `CO/` | `Moriarty/research/sherlock-compact-open-problems-20261111/` |
| `K21/` | `Moriarty/research/sherlock-k21-recalibration-20261111/` |
| `X9` | `Moriarty-wt/xc9/research/sherlock-k21-recalibration-20261111/measured/XC-9-ledger9-tx-path.md` |
| `PV` | `Moriarty-wt/preview-deploy/research/sherlock-preview-qualification-20261111/DEPLOY-REHEARSAL.md` |
| `GW` | `Moriarty-wt/gpu-wrap/research/sherlock-gpu-wrap-20261111/RESULTS.md` on branch `gpu-wrap`, commits `b158d4741` and `52ad95069` (final table for k = 19, 20 and 21) |
| `GWR` | `~/.cache/gpu-wrap/runs/chain-20261111T194102/k20/run.log`, the per-config records of the k = 20 chain stage of `GW:11` |
| `E1` | `Moriarty-wt/e1-test/research/sherlock-compact-open-problems-20261111/E1-devnet-test.md` at commit `4a4abd875` of branch `e1-test` |
| `D3` | `Moriarty-wt/d3-soundness/research/sherlock-processor-final-20261111/D3-SOUNDNESS.md` at commit `234954947` of branch `d3-soundness` (calculator log `d3/test-run2.log`) |
| `UDS` | `~/.cache/ud-sizing/ud-sweep.log`, the D3 calculator (branch `d3-soundness`, commit `81abb8f7a`) run on the unique decoding sets with test `crates/stark-backend/tests/ud_sizing.rs`; method and tables in `PR/UD-SIZING.md` |
| `TP` | `Moriarty/qa/sherlock/target-pins.json` (working copy) |
| `SB/` | `Moriarty/repos/stark/openvm-org_stark-backend/` (stark-backend v2.1.0) |
| `MZK/` | `Moriarty/repos/midnightntwrk/midnight-zk/` |
| `ML/` | `Moriarty/repos/midnightntwrk/midnight-ledger/` |
| `CC/` | `Moriarty/repos/LFDT-Minokawa/compact/` |
| `WH/` | `Moriarty/repos/stark/WizardOfMenlo_whir/` (WHIR reference code, commit `83bc417`) |

Marks:

- MEASURED: a number with the `path:line` of its measurement record.
- ESTIMATE: a derived number. Its inputs are named in the same row or sentence.
- DERIVED: a number computed by a stated rule from MEASURED numbers.
- STATIC COUNT: counted by script from compiler output; exact for the ZKIR that `compactc` 0.36.0 emits.
- PENDING: a measurement that has not run. Section 13 lists each one.
- LEAD: a constant or rule that the sources leave open and that this proposal sets as the lead's recommendation. Each one is an owner item in section 12 and binds only after the owner confirms it.

Hard rules. The design keeps each of them by construction except the proven 100-bit rule, which holds only once the security qualification gate of section 11.5 passes; until then every bit count in this document is a calculator figure (MEASURED by D3, or ESTIMATE), not a proven bound. The rules: never k = 22; every proof is a fixed-size shard; per-program, witness-independent preprocessing bound to `prog_d` (section 4); zero knowledge end to end, with every STARK layer on the user's machine and no helper that sees the plain witness; proven 100-bit security with no unproved conjecture; GP-1 to GP-4 never narrowed; one universal key set; the ledger 9.1.0.1 verifier unchanged and a verifier key of at most 50,000 bytes.

---

## 1. Summary

**What runs where.**

| # | Layer | Field and hash | Size | Runs on | Hiding |
|---|---|---|---|---|---|
| 0 | Per-program plan (`sherlock plan`), once per program, no witness | n/a | output bound to `prog_d` | user or build host | n/a |
| 1 | Leaf shards: the program's steps, emulated Fr work (tier E), log writes | BabyBear, `EF = BabyBear^4`, Poseidon2 over BabyBear | fixed shapes from the plan's menu | user's GPU | none needed |
| 2 | Nodes: verify children, merge log digests, check leaf keys against `key_root` | same | fixed arity | user's GPU | none needed |
| 3 | `L`: verifies the node root, exposes the statement, the log and the call vector as public values | BabyBear; Merkle and transcript in Midnight Poseidon over Fr | one shape, height at most 2^20 | user's GPU | none needed |
| 4 | Wrap `W`: verifies `L` as a private witness, replays the Fr log natively | midnight-proofs PLONK with KZG over BLS12-381, Blake2b transcript | one key; G4 at k = 20 or 21 on the benchmark `L`, ESTIMATE; ledger path open (OD-21) | user's GPU, CPU fallback | yes: the only published proof |
| 5 | Ledger 9 `VerifierKey::verify` | n/a | key at most 50,000 bytes | Midnight | n/a |

Source: `B/CANDIDATE.md:22-31`, with the owner sections at `B/CANDIDATE.md:442-452`.

**The five facts a new reader needs.**

1. **Framework.** OpenVM stark-backend v2.1.0 with its CUDA prover (SWIRL: LogUp-GKR, batched zerocheck, stacked commitment, WHIR) proves layers 1 to 3. Plonky3 with Plonky3-recursion is the second choice (`F/DECISION.md:13-14`). The GPU leaf and node build and prove on this host without a source patch (`F/SHORTLIST.md:20`).
2. **Bridge.** Only `L` changes hash: it uses Midnight's Poseidon over Fr, fed by 8 BabyBear elements per Fr element. BLS12-381 Fr work that a program asks for goes either into a deferred field log, which the wrap replays natively, or into emulation chips in the leaf. The split is a prover choice inside the plan and never enters the statement (`B/CANDIDATE.md:18,98`).
3. **Wrap shape.** The G4 wrap is a 4-lane circuit of degree at most 5 with a 19-row Poseidon layout. Ledger 9 refuses a G4 key (M7-0, MEASURED), so every G4 figure below is a G4 figure on the benchmark `L` and holds only under OD-21 option A or as the first layer of option C. Keygen of `L` gives 39 AIRs, 1,537 trace columns and 487 interactions, MEASURED (`F/L-STRUCTURE.md:38-42`). On that structure, with the unique decoding `L` set of section 3.4 at `w_stack` 48, the wrap needs 728,763 of 1,048,569 usable rows at the middle estimate and 761,393 at the high one, so it fits k = 20; the worst gadget case, 1,523,302 rows at the middle estimate, needs k = 21, ESTIMATE (`PR/UD-SIZING.md` section 3.1). No row fits the k = 19 gate. The decider-style single lane fails the k = 21 gate at the middle estimate, ESTIMATE (`F/L-STRUCTURE.md:3`).
4. **Call time.** The prove stages of one warm Compact call (leaf and node, `L`, wrap) take 8.90 s (6.31 to 16.20 s) on the full GPU path with a G4 wrap at k = 20 on the benchmark `L` (OD-21 option A), ESTIMATE (`PR/UD-RECONCILE.md`, from the call-time model of `PR/UD-SIZING.md` section 4), against about 203 s for the same stages on today's CPU paths, ESTIMATE (`CT/RESULT.md:105`). The whole warm call on the full GPU path, with witness fill, transaction prebuild, the balance and DUST proof and submit to inclusion, is 19.0 s (11.1 to 45.0 s), ESTIMATE (`PR/UD-SIZING.md` section 4). Today's figure of 195 s covers the warm prove-only parts alone and leaves out part of the grinding and the BabyBear part of `L` (`CT/RESULT.md:13,105`). Two kernels carry the change: Midnight Poseidon over Fr on the GPU for `L`, and a full GPU wrap prover (`CT/RESULT.md:22`). That figure assumes the wrap at k = 20, the Fr Poseidon kernel (no prototype) and a rewritten MSM kernel (no prototype) (`CT/RESULT.md:101,155-156`). MEASURED baseline with today's kernels on today's decider, one proof per cell on a loaded host (`GW` section 1, commits `b158d4741` and `52ad95069`): full GPU path 34.8 / 95.0 / 155.2 s against the CPU path 118.8 / 243.3 / 441.7 s at k = 19 (decider-shaped proxy), 20 and 21; every proof byte-identical to the CPU proof and accepted by ledger 9 `VerifierKey::verify` (k = 21 also by the unpatched standalone verifier at ledger 9 `f15fb9d`). On a quiet host the k = 21 GPU path is about 1.35 times faster than the 111.8 s CPU path with the GPU MSM and about 1.45 times without it, ESTIMATE (`GW` section 2). The wrap times of call-time paths 2 and 3 (5 to 12 s at k = 20, 3.5 s with the full-device pipeline) are ESTIMATE conditional on two kernels that are not built: the fused quotient kernel and the MSM rewrite (`GW` section 3). Today's kernels are far from the wrap time these call-time figures assume.
5. **Midnight side.** The path through the unchanged ledger is MEASURED for the existing k = 21 decider, a `ZkStdLib`-architecture key; ledger 9 refuses a G4 key (M7-0, MEASURED), so it is not yet MEASURED for any wrap of this design: a local devnet call (`X9:3`) and a Preview deploy and call at blocks 71100 and 71431 with three refusals (`PV:1,54,77,103-105`). The same local ledger accepts the cross-entry attack E-1 today, and a prototype of the bind statement refuses it (`E1:3`; section 7.2). The new wrap has the same proof system and transcript, so the same verifier takes it. Its proof bytes, verify time and `2 + L_TX` public inputs are PENDING.

**First build items.** The Compact frontend (milestones M0 and M1) and the E-1 entry-label binding in placement V come first, then atomic chunked calls (mode 1F, with 1G for callees of guaranteed callers). Section 10 gives the full order.

**Security.** Status: not qualified. Proven 100-bit security is a gate of this design, not a result: checks G1 to G4 of the security qualification gate of section 11.5 have not passed, and no bit count below is a proven bound. The target is 100 bits of proven round-by-round soundness on every layer. Every layer runs WHIR in unique decoding (owner decision OD-19), so every term rests on theorems that are proven as stated: WHIR Lemma 4.10, Corollary 4.11 and the unique decoding branch of Theorem 4.8 for folding, WHIR Theorem 5.2 with list size 1, WHIR Lemma 4.25 with one OOD sample, and the unique decoding branch of SWIRL Theorem 4.2.4 for `mu` batching. The paper half of G5 is discharged for all layers by these results (`PR/G5-PAPER-MAP.md` sections 1, 2 and 6; `PR/UD-SIZING.md` section 1.2). D3, the OpenVM soundness calculator, prints a weakest term of 101.0730 bits (`mu`) for the `L` set at `w_stack` 48 and 101.1643 bits (query) for the internal set, MEASURED (`UDS:351,535`). The app and leaf-aggregation sets reach 101.0891 and 101.0110 bits with query PoW 21 (`D3:13-14`; section 11.2). The bridge theorem `bridge_sound` builds in Lean with 0 sorry, 112 theorems and only the standard axioms (`LEAN:1,134`), but it takes the soundness of `L` and of the recursion as premises and checks an older parameter gate (`LEAN:49,52,77`), so its build does not establish the bit count of the frozen sets. The claim of proven 100 bits waits on G1 to G4: frozen per-layer parameter sets and shape bounds, the measured `w_stack` of `L` (D10), the parameter and calculator code in the prover and verifier, the transcript differential test, and the Lean restatement for the unique decoding sets. A written proof and a Lean proof of the list decoding MCA bound are in progress (`research/sherlock-mca-proof-20261111/`, branch `mca-lean`); no term of this design depends on them. No universal key freezes before the gate passes. Section 11 lists every premise.

**Budget and cut list.** The wrap row gate is 85 percent of `2^k - 7` usable rows (`F/WRAP.md:127,129`). Out of the design: any k = 22 circuit, a second Fr proving engine, pairing or foreign verifiers inside the wrap, helpers that prove STARK layers, a second wrap key for long transcripts, and every proof-format change that the ledger 9.1.0.1 verifier rejects (`CT/RESULT.md:186,202-208`).

---

## 2. Framework

**Choice.** OpenVM stark-backend v2.1.0 (`a6b4761`) with its CUDA prover for leaves, nodes and `L`. Second: Plonky3 0.9.0-rc.2 with Plonky3-recursion (`3a69ba6`), CPU only on this host. Third, not benched: SP1 v6.10 (`F/DECISION.md:13-14`; `F/SHORTLIST.md:61-63`).

**Why OpenVM carries the design.**

- The recursion circuit reads the child's constraint DAG as a cached trace, so the internal-recursive key and the root key are the same for every app key (`SP/RESEARCH.md:23`; `F/SHORTLIST.md:74`). That is the mechanism for per-program leaf chips under one universal key set.
- It is the only shortlisted framework with a GPU prover on this host. The CUDA build is native `sm_121` with no source patch (`F/bench/RESULTS.md:88-91`).
- Its root parameters target 100 bits of proven round-by-round security for `EF = BabyBear^4` (`B/CANDIDATE.md:59`).

**Measured cells (OpenVM, 20 host threads).**

| Cell | Prove s | Source |
|---|---:|---|
| W1 leaf 2^16, GPU | 2.601 | MEASURED `F/bench/RESULTS.md:13` |
| W1 leaf 2^20, GPU | 5.829 | MEASURED `F/bench/RESULTS.md:15` |
| W1 leaf 2^22, GPU (scaling point only) | 18.378 | MEASURED `F/bench/RESULTS.md:16` |
| W1 leaf 2^20, CPU | 43.959 | MEASURED `F/bench/RESULTS.md:33` |
| W1 leaf 2^22, CPU | 190.924 | MEASURED `F/bench/RESULTS.md:34` |
| W1 generic leaf key 2^20, GPU | 6.291 | MEASURED `F/bench/RESULTS.md:28` |
| Fr multiply (W3, 194 columns) 2^20, GPU | 9.126 | MEASURED `F/bench/RESULTS.md:30` |
| Node arity 2, 2^20 children, GPU | 1.809 | MEASURED `F/bench/RESULTS.md:22` |
| Node arity 4, 2^16 children, GPU | 2.595 | MEASURED `F/bench/RESULTS.md:23` |
| Node arity 8, 2^16 children, GPU | 4.682 | MEASURED `F/bench/RESULTS.md:24` |

Derived from these cells, ESTIMATE:

- GPU over CPU on W1 is 7.5 times at 2^20 (43.959 / 5.829) and 10.4 times at 2^22 (190.924 / 18.378).
- A generic leaf key costs 7.9 percent over a program-specialized key at 2^20 (6.291 / 5.829).
- Per leaf with its node share, `t_leaf + t_node / (a - 1)`: 6.50 s for a 2^20 leaf under arity-8 nodes (`F/DECISION.md:51`).
- Node cost is nearly flat in child height, so leaves are tall, 2^20 rows or more (`F/DECISION.md:56`).

**Rules for production code.**

- Set `zerocheck_save_memory = false` per AIR set. The GPU default costs about 34.5 times on W1 at 2^16 (89.642 s against 2.601 s, MEASURED `F/bench/RESULTS.md:12-13`).
- List only the AIRs a program's leaves use. A generic child key with absent preprocessed AIRs gives a node proof that the verifier rejects (`SumClaimMismatch`, MEASURED `F/bench/RESULTS.md:109`).

**Switch conditions to Plonky3** (`F/DECISION.md:80-89`): the OpenVM root does not fit a k <= 21 wrap and a Plonky3 FRI last layer does; the `SumClaimMismatch` is a recursion-verifier bug that listing used AIRs does not avoid; the target becomes CPU only and Plonky3 is faster per leaf in the same security model; or helper-host proving becomes a requirement and Plonky3's LogUp terminals can be masked first.

**MEASURED** (`F/bench/RESULTS.md`, campaign results): all 59 cells, every one on a loaded host. Per W1 2^20 leaf at arity 2: OpenVM GPU 7.64 s, Plonky3 CPU 13.26 s, OpenVM CPU 55.60 s. Rule 3 splits (OpenVM on the GPU, Plonky3 on the CPU), so the order rests on rule 2: Plonky3 has no universal node key (`F/DECISION.md:14,59`). PENDING: Plonky3 at proven 100-bit parameters (every Plonky3 cell runs at 124 bits conjectured).

---

## 3. Bridge and layer stack

### 3.1 The tree

A run is a tree of fixed-size proofs. Leaves of fixed shapes prove the steps; nodes of fixed arity verify their children; one `L` verifies the node root; one wrap verifies `L`. Work grows by adding shards, never by growing a circuit. No key constant bounds run length or Fr work (`B/CANDIDATE.md:171`).

The one-leaf Compact call takes path C: one leaf, one OpenVM leaf layer at internal parameters, no internal nodes, then `L` and the wrap (`CT/RESULT.md:44,161`). Path C is the shortest path through the same tree. Longer runs, GP-2 consumption and GP-4 trees use the full tree with internal nodes. Path C has no universal node, so the key check of section 4.3 moves into `L` and the wrap there (the DAG key check, section 4.3).

Keys:

- Per program: one app key, one OpenVM leaf-layer key and one internal-for-leaf key per chip set the program uses. Their commitments form a Merkle tree whose root `key_root` is inside `prog_d` (`CO/FRONTEND.md:241-246`).
- Universal: the internal-recursive key, the `L` key and the wrap key. They are the same for every program (`B/CANDIDATE.md:33`; `CO/FRONTEND.md:545-546`).

### 3.2 The deferred field log

- **Entries.** `e = (op, inputs, outputs)`, each Fr value as 9 limbs of 30 bits with `dec(x) < r`. The frozen operation list is `PERM`, `H2`, `SPONGE(n)`, `PATH(d)`, `ECMUL`, `ECMULG`, `PUB(i)`, `DIGEST`, `NOP`. `FADD`, `FMUL`, `FINV` and `ECADD` are never deferred, because binding costs more than emulation (`B/CANDIDATE.md:72-73`).
- **Digest.** A tagged onion in Poseidon2 over BabyBear, carried as `(d, n)` in the boundary state, so it chains through leaves, continuations, suspension and resume (`B/CANDIDATE.md:74`). The empty digest is `d_0 = P2(TAG_EMPTY)`, a hash output, and each step is `d_(i+1) = P2(tagStep, d_i, P2(tagEnt, enc(e_i)))` (`B/CANDIDATE.md:458`; `LEAN:106`).
- **Binding into `L`.** `L` takes the flattened log and its tree shape as private witness, recomputes `d`, and binds both tables of the encoding below to their parts of the public-value vector, each by its own multilinear extension `T(x) = sum_i eq(x, i) pv_i`. This is the fixed `pv` length of `LEAN:107,118`, not the fixed `B l_e` slot of `B/CANDIDATE.md:81`. The Fr transcript of `L` absorbs `pv` in the verifier preamble, before the first challenge (`B/CANDIDATE.md:81`).
- **Wrap checks.** Range and `Canon` on every logged value, a bounded-quotient range check on every lazy reduction, a canonical check on every squeeze recomposition, native absorption of `pv`, one `eq` evaluation, and native replay of every entry (`B/CANDIDATE.md:82,459-460`).
- **Capacity.** Four constants of the wrap key: `B` entries, `V` logged Fr values, `P` permutation slots and `E` Jubjub slots. The slot counts are owner item OD-6: today `V = P = 1,024`, `E = 16` (`B/CANDIDATE.md:92-94`); the recommendation is `V = P = 256`, `E = 0` (`CT/RESULT.md:271`). The encoding below fixes the length of `pv` per wrap key, as the Lean amendment requires (`B/CANDIDATE.md:461`; `LEAN:107`). `B` and the encoding are LEAD constants (owner item OD-17).
- **Fixed encoding of `pv`.** `pv` is two tables in a fixed order, both absorbed in the preamble of `L`:
  - the entry table, `B` rows of 2 BabyBear words `(op, param)`;
  - the value table, `V` rows of 9 words: one canonical Fr value as 9 limbs of 30 bits. The `d2` operand of `DIGEST` is logged as the Fr value `pack(d2)`, so every row has the same range and `Canon` check.

  Entry `i` owns the value rows `off_i .. off_i + vals(op_i, param_i)` with `off_0 = 0` and `off_(i+1) = off_i + vals(op_i, param_i)`, and the permutation slots by the same running-sum rule with `perms`. `L` and the wrap both check the running sums, so the offsets are not words of `pv`. The encoded length is `W_pv = 2B + 9V` words: 2,816 at `B = V = 256`, DERIVED.
- **Per-operation counts and bounds.** Value counts follow `B/proposals/fable2.md:73-81` and `CO/FRONTEND.md:446-447`; `param` is 0 where the column shows none.

  | `op` (code) | `param` | Values | `P` slots | `E` slots | Bound |
  |---|---|---:|---:|---:|---|
  | `NOP` (0) | 0 | 0 | 0 | 0 | padding only |
  | `PERM` (1) | 0 | 6 | 1 | 0 | |
  | `H2` (2) | 0 | 3 | 1 | 0 | |
  | `SPONGE(n)` (3) | `n` | `n + 1` | `ceil(n/2)` | 0 | `1 <= n <= V - 1` |
  | `PATH(d)` (4) | `d` | 3 (leaf, index, root; siblings wrap-private) | `d` | 0 | `1 <= d <= 32`, the Compact maximum depth (`CC/compiler/langs.ss:103`) |
  | `ECMUL` (5) | 0 | 5 | 0 | 1 | none admitted while `E = 0` |
  | `ECMULG` (6) | 0 | 3 | 0 | 1 | none admitted while `E = 0` |
  | `PUB(i)` (7) | `i` | 1 | 0 | 0 | `0 <= i < 2 + L_TX` |
  | `DIGEST` (8) | tag index | 2 (`pack(d2)`, output) | 1 | 0 | tag from the frozen list of section 3.6 |

- **Mandatory entries.** Some `DIGEST` entries must be in the log, because the wrap reads their outputs and tier E cannot replace them: the anchor entries of section 7.3 and the public digests `input_d` and `outcome_d`, which come only from `DIGEST` entries (last bullet of this section). The anchor reservation `R_A` is 1 entry, 2 values and 1 permutation for an ordinary call, whose `a_in` and `a_out` are the output of one `DIGEST` entry of `C_P`, and 2 entries, 4 values and 2 permutations for a chunk, whose `a_in` and `a_out` are two distinct values (section 7.3). The mandatory set `R` is `R_A` plus the public-digest entries of the run's statement. `R` depends only on the mode and the statement, so it is static, and it is the same in the planner, in `L` and in the wrap (LEAD, owner item OD-17).
- **Simultaneous capacity rule.** A run's deferred set is `R` plus a discretionary set `S`. It is admissible only if all four limits hold together for `R` and `S`: `|R| + |S| <= B`, `vals(R) + sum vals(S) <= V`, `perms(R) + sum perms(S) <= P`, `E(R) + sum E-slots(S) <= E`, with `E(R) = 0`. The planner places `R` first, before any discretionary operation, and then adds a discretionary operation to the log only if every limit and its per-operation bound still hold for `R` and `S` after it; otherwise it sends that operation to tier E. Keygen refuses a capacity that does not hold the largest `R`, the chunk case with both public digests: 4 entries, 8 values and 4 permutations, DERIVED from the table above. A discretionary operation over a bound, such as a `SPONGE` longer than `V - 1` inputs or a deeper `PATH`, is proved inline by tier E and never refused, so GP-1 keeps no bound on Fr work. Every non-`NOP` entry logs at least one value, so `B = V` makes the entry limit redundant; the recommendation is `B = 256`, LEAD. The largest checked entry, `zerocash spend` at 111 values and 39 permutations, fits with a margin of 2.31 times on values and 6.56 times on permutations, DERIVED from `CO/FRONTEND.md:439`. That demand already counts the three mandatory `DIGEST` entries of an ordinary call (`C_P`, `input_d`, `outcome_d`: 6 values, 3 permutations, `CO/FRONTEND.md:446-447`), so the margins hold with `R` in place. As a chunk, `spend` adds the second anchor entry, 113 values and 40 permutations, 2.27 and 6.40 times, DERIVED, before the claim commitment `comm_(j+1)` of section 7.3.
- **Overflow tests.** Every capacity test runs with `R` reserved. Frontend M4 adds two forced-overflow cases: an ordinary call whose discretionary operations fill `V` and then `P` exactly, and a run whose discretionary demand exceeds every limit. The chunk test T3 adds the same run as chunk 1, chunk `j` and chunk `k` of a chunked call (section 7.3). In each case the log holds every entry of `R`, the wrap accepts, and the effect vector equals the native one. A negative case removes one anchor entry from the log and the wrap refuses.
- **Padding.** Entry rows after the last used entry are `NOP` with `param` 0, and no used entry follows a `NOP`. Value rows after `off_n` are all zero. The wrap checks both, so the used counts stay private and the encoding of a log is unique.
- **Overflow.** When the slots that `R` leaves are full, every further discretionary Fr operation is proved inline by tier E chips in its leaf. A claim that another run consumes is closed by a discharge shard, which is a fixed leaf shape (`B/CANDIDATE.md:98`). Both prove the same `Valid(e)`; Lean proves that any split of the discretionary operations gives the same claim (`spill_equiv`, `discharge_sound`, `LEAN:78`).
- **Internal digests.** Every internal digest is on Poseidon2 over BabyBear (`sherlock-value/2`). Only `L` and the wrap use Poseidon over Fr. Public digests are `PoseidonFr(tag, pack(d2))` from `DIGEST` entries (`B/CANDIDATE.md:106,444`).

### 3.3 GP-1 to GP-4

| Requirement | Mechanism |
|---|---|
| GP-1 universal VM | Programs and inputs are data; fixed leaf shapes and a fixed wrap; the spill rule removes every bound on Fr work; `(d, n)` in the boundary state keeps stop and resume (`B/CANDIDATE.md:171`) |
| GP-2 cross-run composition | A claim from the same machine joins the log tree by `TAG_NODE` or closes in a discharge shard. A claim from another machine arrives as that run's wrap proof and is verified in a GP-3 shard (`B/CANDIDATE.md:172`) |
| GP-3 foreign proofs | STARK shards with OpenVM's algebra and pairing extensions, a fact of the OpenVM code (the OpenVM whitepaper names no such extension); the wrap holds no pairing and no foreign verifier (`B/CANDIDATE.md:173`). Cost PENDING (measurement M11) |
| GP-4 chains, trees, DAGs | Nodes merge tagged digests; a claim consumed twice is replayed twice; nodes never reject fanout (`B/CANDIDATE.md:174`) |

### 3.4 Parameters of `L`

The parameter set below is the unique decoding set of owner decision OD-19 (`PR/UD-SIZING.md` sections 1.3 and 5.1; `DECISIONS-ASK.md` row OD-19).

| Parameter | Value | Source |
|---|---|---|
| Base field, extension | BabyBear `p = 2^31 - 2^27 + 1`; `EF` degree 4 | `B/CANDIDATE.md:41-42` |
| Leaf and transcript hash | Midnight Poseidon over BLS12-381 Fr, width 3, rate 2, 8 full and 60 partial rounds, `x^5` | `B/CANDIDATE.md:43` |
| Packing | base 2^31, 8 BabyBear elements per Fr element, rate 16 BabyBear per absorb | `B/CANDIDATE.md:45` |
| Sampling | 5 base-`p` digits per squeeze, bias at most 2^-100 | `B/CANDIDATE.md:46` |
| Proximity test | WHIR, `k_whir` = 4, 3 rounds, final polynomial 2^8, unique decoding (`WhirProximityStrategy::UniqueDecoding`) in every round | `PR/UD-SIZING.md` section 1.1 |
| Log blowup, `l_skip`, `n_stack` | 3, 2, 18 | `UDS:351` |
| `w_stack` | at most 48, a shape bound of the `L` key (LEAD under OD-19, owner to confirm), with `mu` PoW 6; D10 measures it from the trace heights, which keygen does not fix, and may tighten the bound | `UDS:167,259,351`; `F/L-STRUCTURE.md:7`; `CT/RESULT.md:295` |
| Proximity radius | the unique decoding radius `(1 - rho)/2`, which WHIR Theorem 4.8 admits directly | `PR/G5-PAPER-MAP.md` section 1; `PR/UD-SIZING.md` section 1.2 |
| Queries per round | 98, 83, 82 | `UDS:351` |
| Folding PoW | 0 on every fold | `UDS:351` |
| `mu` PoW, query PoW, LogUp PoW | 6, 20, 18: `mu` PoW 6 holds both gates at `w_stack` 36 and 48 (5 suffices at 16); LogUp PoW 18 is the code rule and gives 102.7205 bits | MEASURED, `UDS:167,259,331-351` |
| Out-of-domain samples | 1 per round, the stock WHIR order | `UDS:351` |
| Trace height | at most 2^20 | `B/CANDIDATE.md:29` |
| Shape bounds of the key | at most 50 AIRs, 1,000 constraints and 100 interactions per AIR, 2,000 trace columns in total, as for the stock root set, and the total interaction count of one proof below `p` (section 11.2); keyed `L` has 39 AIRs, 1,537 columns, 487 interactions and 2,210 constraints | `SB/crates/stark-sdk/src/config/mod.rs:203-208`; `F/L-STRUCTURE.md:38-44` |
| Weakest term | 101.0730 bits (`mu`) at `w_stack` 48; 101.1427 bits (query, round 1) at 36; 101.1427 bits at 16 with `mu` PoW 5, so at least that with `mu` PoW 6, DERIVED | MEASURED, `UDS:167,259,351` |
| Other terms at `w_stack` 48 | LogUp 102.7205, zerocheck 119.7207, constraint batching 113.6546, stacked reduction 111.6618, folding 101.6276, query 101.3466 / 101.1427 / 101.7684 per round, `gamma` 116.9982, OOD 108.6276 (round 0) and 112.6276 (round 1) bits | MEASURED, `UDS:331-351`; `PR/UD-SIZING.md` section 2 |
| Prover work | 1.0 x 10^7 Fr permutations of hashing plus 3.4 x 10^6 of grinding | ESTIMATE, `PR/UD-SIZING.md` section 4: hashing by the rule 2.0 x 10^7 x 2^(b - 4) of `B/CANDIDATE.md:66` at `w_stack` 16, grinding as the expected `2^bits` over the PoW checks of the set |
| Proof of `L` | about 682.5 KiB at the middle case and `w_stack` 48; private, read only by the wrap prover, so no on-chain byte depends on it | ESTIMATE, `PR/UD-SIZING.md` section 3.4 |

Measured shape of `L`: keygen of `L` (`VerifierSubCircuit::<1>` over the bench node at root parameters, with the child DAG as a cached trace) gives 39 AIRs, 1,537 trace columns (1,525 common and 12 cached), 487 interactions, 2,210 constraints and maximum constraint degree 4, MEASURED (`F/L-STRUCTURE.md:27-48`). The log add-on adds 2 AIRs and 69 columns, ESTIMATE (`F/L-STRUCTURE.md:38-39,149`). Keygen fixes no trace heights, so the main-cell area of `L` and its `w_stack` are not measured (`F/L-STRUCTURE.md:7`). The model puts one child at about 2^23 main cells, ESTIMATE (`CT/proposals/last-layer.md:78`), below the `w_stack x 2^20` = 2^24 cells that a bound of 16 would allow, and far below the bound of 48. The heights of the 2-child bench node, used as a proxy, give 37.5 M cells, which would need `w_stack` about 36; the child of `L` need not have the node's heights (`F/L-STRUCTURE.md:145`). D10 reads the heights from `L` tracegen over one node proof (`F/L-STRUCTURE.md:146`). Keygen of `L` used a child at leaf-aggregation parameters, 283 queries (`F/L-STRUCTURE.md` section 5 item 1). A child at the internal parameters of section 11.2 has 260 queries (97/82/81), inside that measured child; on path C that child is the program's leaf layer at internal parameters, and its verifier area inside `L` is not modelled (section 13).

**`w_stack` rule.** `w_stack` enters the bound through the `mu` batching term over the stacked columns (`SB/crates/stark-backend/src/soundness/calculator.rs:379-387`). In unique decoding that term is the SWIRL Theorem 4.2.4 bound `N = (w - 1)|L|`, `log2 |EF| - log2(w_stack - 1) - log2 |L|` bits before PoW, and it is grindable. `w_stack` also sets the LogUp parameters (`SB/crates/stark-sdk/src/config/mod.rs:91`). Only the `mu` term moves with `w_stack`: the smallest `mu` PoW that holds both gates is 5 at `w_stack` 16 and 6 at 36 and 48, and the weakest term is then 101.1427, 101.1427 and 101.0730 bits, MEASURED (`UDS:167,259,351`). The keyed structure of `L` points to `w_stack` about 36. The bound of the `L` key is `w_stack` at most 48 with `mu` PoW 6 (LEAD under OD-19, owner to confirm). The internal set keeps its own bound 512. D10 (the trace heights) is the measurement that may tighten the bound; a tighter bound reruns D3 before the `L` key freezes.

**One OOD sample: transcript and batching.** `L` keeps the stock WHIR order of WHIR Construction 5.1 (p.32): in each non-final round the verifier observes the round's codeword commitment, samples one OOD point, observes its answer, and batches that answer with the query answers under one `gamma` (`SB/crates/stark-backend/src/verifier/whir.rs:148-157,248-254`). In unique decoding the list size is 1 (WHIR Theorem 5.2, hypothesis H4), so WHIR Lemma 4.25 with `s = 1` bounds the OOD error per round by `2^(m_i)/(2|EF|)`; D3 prints 108.6276 bits in round 0 and 112.6276 bits in round 1, MEASURED (`UDS:347-348`), and the `gamma` batching term counts `q_i + 1` claims. Every challenge of every layer comes from the Fiat-Shamir transcript after the commitment it binds; no prover component draws a challenge from a device random source (section 4.5).

### 3.5 Wrap row budget

Usable rows are `2^k - 7`: 2,097,145 at k = 21 and 1,048,569 at k = 20, MEASURED (`K21/measured/M-k21.md:9`), and 524,281 at k = 19, DERIVED by the same rule. The gate is 85 percent of usable rows.

| Shape | Low | Mid | High | Status |
|---|---:|---:|---:|---|
| 4 lanes (G4), 19-row Poseidon, 80 percent packing, `w_stack` 48 | 666,081 | 728,763 | 761,393 | ESTIMATE on the keyed `L`; fits k = 20 in all three (high at 85.4 percent of the gate) (`PR/UD-SIZING.md` section 3.1) |
| G4 at `w_stack` 36, the bench-node proxy area | 627,995 | 697,659 | 730,290 | ESTIMATE on the keyed `L`; fits k = 20 (`PR/UD-SIZING.md` section 3.2) |
| G4 at `w_stack` 16 to 18 (the scenario heights) | 562,558 | 639,486 | 672,117 | ESTIMATE on the keyed `L`; fits k = 20 (`PR/UD-SIZING.md` section 3.2) |
| G4, no limb overlap and 60 percent packing, `w_stack` 48 | 1,439,685 | 1,523,302 | 1,567,993 | ESTIMATE on the keyed `L`; over the k = 20 gate in all three, fits k = 21 (high at 88.0 percent of the gate) (`PR/UD-SIZING.md` section 3.1) |
| Single lane, 70-row Poseidon (degree 6), k = 21 | 2,064,403 | 2,440,107 | 2,652,569 | ESTIMATE on the keyed `L` at the S4 query constants 57/28/19, fewer queries than the section 3.4 set; fails the k = 21 gate in all three (`F/L-STRUCTURE.md:102,123`) |

The G4 rows come from the wrap model of `F/wrap_model.py` on the keyed structure `F/L-STRUCTURE.json` (branch `l-structure`, commit `3957ddf7c`, variant `L_cached`) with log blowup 3, queries 98/83/82 and Merkle depth `20 + b - 4 - r` in round `r`, and with the heights of the bench node as a proxy for the heights of `L` (`PR/UD-SIZING.md` section 3; `F/L-STRUCTURE.md:72,143`). The model counts two OOD Horner evaluations in the final check, one more than the set uses, so the rows are an upper bound in that term. The heights of D10 replace the proxy before M7. Every row is a G4 row on the benchmark `L` (OD-21 open; section 9). Every row is above the k = 19 gate of 445,638 rows.

The key's k is the smaller of 20 and 21 at which the measured G4 rows fit the gate (`B/proposals/fable-gpu-wrap.md:363`). The log capacity of OD-6 (256/256/0) is a constant of the same key.

Separate gates:

- **Lookup table.** One fixed 2^16-row range table, at most 4 lookups of input degree 2 (`B/proposals/fable-gpu-wrap.md:116-117`). A table sized to the domain leaves only 16,102 free table rows at both k = 20 and k = 21, derived from the measured table rows (`K21/measured/M-k21.md:34-35`).
- **Degree.** Every gate and lookup has degree at most 5, so the extended domain is 4n. One degree-6 gate doubles the quotient work (`B/proposals/fable-gpu-wrap.md:104-106`).

### 3.6 Hash domains

Every hash that Sherlock defines for itself has its own domain, a fixed length encoding and a fixed initial state. The rules below are LEAD constants (owner item OD-18) and freeze with the universal keys. The paper review left the Fr Merkle and transcript capacity tags open (`PR/REVIEW.md:111-113`).

**Fr side: Midnight Poseidon permutation, width 3, rate 2.** Two sponge algorithms run on this one permutation. The fixed-length hashes (`u` = 0, 1, 2, 4 below) use Midnight's own sponge. The transcript of `L` (`u` = 3) uses the stark-backend duplex of the transcript table below, not Midnight's sponge. Midnight's sponge puts the input length in the capacity cell for a fixed-length hash and `2^64` for a variable-length one; the rate cells start at zero (`MZK/circuits/src/hash/poseidon/poseidon_cpu.rs:131-133`; `MZK/circuits/src/hash/poseidon/poseidon_chip.rs:478-482`). Sherlock keeps that rule for the fixed-length hashes and adds a use code `u` times `2^66` to the capacity cell. The initial capacity is `cap(u, l) = l + u 2^66` for a fixed length `l`, and `2^64 + u 2^66` for the transcript. Every `l` is below `2^64`, so no two uses share an initial state. The tag is a constant cell, so it adds no permutation to the 1.0 x 10^7 hashing of `L` and no wrap row.

| `u` | Use | Input and length encoding | Output |
|---:|---|---|---|
| 0 | program operations `PERM`, `H2`, `SPONGE(n)`, `PATH(d)` | unchanged native Midnight definitions, so the effect vector equals native | as native |
| 1 | Merkle leaf of `L` | the leaf's BabyBear words, a count `w` fixed per tree by the key, packed 8 per Fr element (`pack` with its checked length, `LEAN:73`); `l = ceil(w/8)`; the last element holds `w mod 8` words when that is not 0 | first rate cell after the last permutation |
| 2 | Merkle node of `L` | `(left, right)`, `l = 2`, one permutation | first rate cell |
| 3 | Fr transcript of `L` | the multi-field overwrite duplex of the transcript table below; the first message is the pre-hash of the `L` verifying key, as the stark-backend preamble observes it first (`SB/crates/stark-backend/src/verifier/mod.rs:128-129`); every later message has a length fixed by the key's schedule `sched`; BabyBear words are buffered and packed 8 per Fr element in order; each squeeze gives 5 base-`p` digits with the canonical check | squeezed digits |
| 4 | public digest (`DIGEST`) | `(tag, pack(d2))`, `l = 2`, one permutation; `tag` from the list below | first rate cell |

Public digest tags, one Fr constant each, all distinct (their Fr values are not written yet: ENC-1, `FD/ARCHITECTURE.md` section 1.12): `SHK_PROGC` (anchor value `C_P`), `SHK_LOCK` (lock values `A_j`, section 7.3), `SHK_INPUT` (`input_pub` over `input_d`), `SHK_OUTCOME` (`outcome_pub` over `P2(DOM_OUTHIDE, input_d, outcome_d)`, so the outcome is hidden under the salt of the run). `C_P = H(SHK_PROGC, r_P, prog_d)` (`CO/FRONTEND.md:220`) is computed in this form as `PoseidonFr_4(SHK_PROGC, pack(P2(DOM_PROGC, r_P, prog_d)))`, with `r_P` as 8 BabyBear words.

**Transcript and sponge algorithms.** Each sponge is one of three algorithms. The prover, the stark-backend verifier and the wrap's verifier of `L` run the same algorithm for each use, and section 13 lists the transcript vectors that check it (T-FS).

| Algorithm | Used by | Absorption | Partial block | Squeeze order | Buffers and transitions | Sherlock change |
|---|---|---|---|---|---|---|
| Midnight fixed-length sponge, state `(r_0, r_1, c)` (`MZK/circuits/src/hash/poseidon/poseidon_cpu.rs:131-180`) | `u` = 0, 1, 2, 4 | additive: inputs queue until the squeeze; at the squeeze each block of 2 is added into `r_0, r_1`, then one permutation per block | the last block adds only the cells it has; the other rate cell keeps its value; no padding word | one output, `r_0` after the last permutation; a second squeeze of a fixed-length sponge is refused (`MZK/circuits/src/hash/poseidon/poseidon_cpu.rs:149-153`) | the queue length must equal `l` (`MZK/circuits/src/hash/poseidon/poseidon_cpu.rs:165-172`); `u` = 1, 2, 4 use this fixed-length mode only. `u` = 0 runs each native operation as native does, in the fixed-length mode (`MZK/circuits/src/hash/poseidon/mod.rs:133`) or in the variable-length mode with its length word (`MZK/circuits/src/hash/poseidon/poseidon_cpu.rs:161-164`), and `PERM` is the bare permutation | the capacity `cap(u, l)` above; for `u` = 1 and 2 this sponge replaces the stock template's `MultiFieldHasher` leaf hash and truncated-permutation compression (`SB/crates/stark-sdk/src/config/baby_bear_bn254_poseidon2.rs:50-59`; `B/CANDIDATE.md:44`), a second change to the stark-backend configuration beside the transcript; for `u` = 0 none |
| Multi-field overwrite duplex over Fr, state `(s_0, s_1, s_2)`, rate 2 (`SB/crates/stark-backend/src/transcript/duplex_sponge.rs:61-83`; `SB/crates/stark-backend/src/transcript/multi_field.rs:104-178`), instantiated as the BN254 template (`SB/crates/stark-sdk/src/config/baby_bear_bn254_poseidon2.rs:161-162`) with Midnight's permutation and Fr in place of BN254 | `u` = 3, the transcript of `L` | overwrite: an absorbed Fr element replaces `s_(absorb_idx)`; when `absorb_idx` reaches 2 the sponge permutes, sets `absorb_idx` to 0 and `sample_idx` to 2; no addition and no length word | an Fr element pending in `s_0` leaves `s_1` at the output of the previous permutation; the next squeeze permutes over it with no marker. A base-field observe buffer of 1 to 7 words is packed as one Fr element `sum_i b_i 2^(31 i)` with no length marker (`SB/crates/stark-backend/src/transcript/multi_field.rs:143-149`; `SB/crates/stark-backend/src/multi_field_packing.rs:27-35`) | a squeeze permutes if a block is pending (`absorb_idx != 0`) or the outputs are used up (`sample_idx = 0`), then returns `s_1`, then `s_0`; a squeeze right after a block-completing absorb uses that permutation and adds none. Each Fr output gives 5 base-`p` digits, least significant first, and `sample` returns them in the order `b_0 .. b_4` (`SB/crates/stark-backend/src/transcript/multi_field.rs:116-128,166-178`) | a base-field observe first drops every buffered digit, then buffers the word and packs and absorbs at 8 words (`SB/crates/stark-backend/src/transcript/multi_field.rs:105-113,137-139`); a sample takes a buffered digit if one is left, otherwise flushes the observe buffer and squeezes; a commitment observe drops the digits, flushes the observe buffer, then absorbs the digest, one Fr element, as is (`SB/crates/stark-backend/src/transcript/multi_field.rs:152-158,198-200`) | one change to the transcript: the initial capacity cell `s_2 = 2^64 + 3 * 2^66` in place of 0 (stock initial state all zero, `SB/crates/stark-backend/src/transcript/duplex_sponge.rs:31-38`); `s_0 = s_1 = 0`; every transition is stock |
| Stock overwrite duplex over BabyBear, width 16, rate 8 (`SB/crates/stark-sdk/src/config/baby_bear_poseidon2.rs:21-23,40`) | transcripts of the leaf, leaf-layer and internal layers | as the row above with rate 8, one BabyBear word per cell | as the row above | outputs `s_7` down to `s_0` after each permutation | as `SB/crates/stark-backend/src/transcript/duplex_sponge.rs:61-83` | none; premise A4 |

The packing of a partial observe buffer has no length marker, so two observe sequences of different lengths can pack to the same Fr element; the stark-backend allows this only under a deterministic observe and sample pattern (`SB/crates/stark-backend/src/transcript/multi_field.rs:17-23`). The schedule `sched` of the `L` key fixes that pattern: the count of words between any two flushes is a key constant, and the wrap's verifier of `L` follows `sched`, not the proof. The wrap implements the overwrite duplex row on its 19-row permutation layout. It does not use Midnight's sponge chip, whose absorption is additive and whose transcript mode appends a length word, and it exposes the bare permutation itself, because the chip keeps it `pub(super)` (`MZK/circuits/src/hash/poseidon/poseidon_chip.rs:419`).

**BabyBear side: Poseidon2.** The leaf, leaf-layer and internal layers keep the stock OpenVM Merkle hash, compression and challenger; their soundness is premise A4. Every Sherlock-defined Poseidon2 hash `P2(tag, x_1 .. x_l)` absorbs the tag word first and then the inputs, with the total length `1 + l` in the first capacity word and the last rate block padded with zeros, so equal digests imply equal tags, lengths and inputs under premise A5. Tags, all distinct BabyBear constants:

| Tag | Use | Source of the form |
|---|---|---|
| `TAG_EMPTY`, `tagStep`, `tagEnt`, `TAG_NODE` | log onion and its tree | `B/CANDIDATE.md:458`; `LEAN:106` |
| `DOM_PROG` | `prog_d` | `CO/FRONTEND.md:246` |
| `DOM_PROGC`, `DOM_LOCK` | the `d2` operands of `C_P` and `A_j` | this section and section 7.3 |
| `DOM_KEYTRIPLE`, `DOM_KEYDAG`, `DOM_KEYNODE` | `key_root` leaves for key triples and for the child DAG commitment `X`, and its internal nodes | section 4.3 |
| `DOM_LAYOUT` | `layout_d` of V+ | section 7.2 |

BabyBear tag codes (LEAD, `FD/ARCHITECTURE.md` section 1.11, FP-4): 1 `TAG_EMPTY`, 2 `tagStep`, 3 `TAG_NODE`, 4 `tagEnt`, 5 `DOM_PROG`, 6 `DOM_PROGC`, 7 `DOM_LOCK`, 8 to 10 `DOM_KEYTRIPLE`, `DOM_KEYDAG`, `DOM_KEYNODE`, 11 `DOM_LAYOUT`, 12 to 29 the `sherlock-value/2` digests (`SHK_IMAGE`, `SHK_VALUE_V2`, `SHK_OUTCOME_V2`, `SHK_IN`, `SHK_RUN`, `SHK_CLAIM`, the six `SHK_SET_*` forms, `SHK_ADM`, `SHK_ADM_STATE`, `SHK_VIEW`, `SHK_VIEW_STATE`, `SHK_DEPS`, `SHK_SD2`), 30 `DOM_STATE_BODY`, 31 `DOM_OUTHIDE`, 32 `DOM_UKEYS`. Each constant is the hash of its tag alone. The one digest table, with the preimage order and the salt of each digest, is `FD/ARCHITECTURE.md` section 1.11, FP-4. Each new digest takes a new code, never a used one.

---

## 4. Shard plan and per-program preprocessing

### 4.1 What the compiler emits

Stage 1 makes every per-entry quantity a compile-time constant and writes it to `abi.json`: the straight-line step count `N_e`, the effect words `L_e`, the tape length, and the counts per builtin (FMUL, FADD, FINV, SPONGE permutations with their input lengths, SHA blocks, ECADD, ECMUL, ECMULG) (`CO/FRONTEND.md:227-231`).

### 4.2 What `sherlock plan` produces

`sherlock plan` runs once per program, with no witness, and produces the static plan, which `plan_d` binds (`plan_d = P2(DOM_PLAN, 1, itab_d, menu_d, policy_d, layout_rule_d, place_d)`, word lists pending ENC-5):

1. **Instruction table.** The straight-line instruction table of each entry is a preprocessed trace committed once (`CO/FRONTEND.md:235`; `SP/RESEARCH.md:147-155`).
2. **Chip set and leaf shape.** A shape is (chip set, area bucket) on the stacked commitment. Compact uses three chip sets: base (CPU, ALU, range, Poseidon2, SHA2, log writer); base plus FIELD-EMU and POSEIDON-FR-EMU; base plus JUBJUB-EMU (`CO/FRONTEND.md:556-559`). That is about 10 to 15 chips per program, below the knee of about 100 chips where powdr found the OpenVM 1.3 recursion verifier too large (`CO/FRONTEND.md:561`; `SP/RESEARCH.md:24`).
3. **Scheduling policy, not a schedule.** For Stage 1 Compact the cut points follow from `N_e` alone, with no metered run. For general programs the plan holds a per-rule maximum table, the cut rule and the list of admitted padded shard counts; the prover makes each run's schedule (cut points, tree, padded count, resumes) from a metered run, and that schedule is not in `plan_d` (`SP/RESEARCH.md:92-95`; `FD/ARCHITECTURE.md` section 1.6, FZ-5). Most checked entries are one leaf: `spend` is 104 to 550 straight-line steps, ESTIMATE (`CO/FRONTEND.md:238`).
4. **Log layout.** The planner first reserves the mandatory entries `R` of section 3.2 (the anchor `DIGEST` entries and the public digests). Each discretionary Fr operation then goes to the log or to tier E. The planner fills what `R` leaves of `P`, then `E`, then `V`, in the order `PATH`, `H2`, `ECMUL`; `FMUL` is never deferred (`CO/FRONTEND.md:240`; `B/CANDIDATE.md:100`). The assignment depends only on static counts, so it is deterministic, and `NOP` padding hides it.
5. **Discharge and GP placement.** Where discharge shards sit in the tree (claims with fanout, cross-run claims), and which leaf shapes carry tier E and GP-3 chips (`B/CANDIDATE.md:200-202`).
6. **Keys.** Per chip set: one app key, one OpenVM leaf-layer key, one internal-for-leaf key. Their commitments form a small Merkle tree with root `key_root` (`CO/FRONTEND.md:241-244`). The same tree holds one more leaf per entry and tree path: the Fr commitment `X` of the child DAG that `L` loads on that path (`CT/proposals/stark-path.md:53`), under its own tag (section 3.6).

### 4.3 Binding the plan to `prog_d`

```text
prog_d = H2(DOM_PROG, image_d, plan_d, key_root, machine_version)      (sherlock-value/2)
C_P    = H(SHK_PROGC, r_P, prog_d)                                      (anchor cell, one r_P per deployment)
```

Sources: `CO/FRONTEND.md:246,220`.

- The plan exists before deploy, because `prog_d` contains `plan_d` and `key_root`.
- **Full tree.** The first universal node checks each child's key commitment by an inclusion path into `key_root` and forwards the root. The check is a small-field Poseidon2 path per child inside the STARK node, not in the wrap (`SP/RESEARCH.md:142,145`).
- **Path C (the DAG key check).** Path C has no universal node, so `L` and the wrap carry the check (`CT/proposals/stark-path.md:51-57`):
  1. `sherlock plan` computes, per entry and per path, the Fr-Poseidon commitment `X` of the child DAG trace that `L` loads, and adds `P2(DOM_KEYDAG, entry, path, enc(X))` as a leaf of the `key_root` tree. `enc(X)` is the canonical 9 by 30-bit limb encoding of section 3.2. The leaf names the entry and the path, because the child DAG of `L` depends on the path: on path C it is the program's leaf layer at internal parameters, on the full tree the universal internal-recursive DAG. A child verifying key is the pair (cached DAG commitment, `h_pre`), and `h_pre` guards against weak Fiat-Shamir (OpenVM whitepaper section 5.3.1, p.17). On the full tree `L` hardcodes the pair of the universal internal-recursive key, as the stock root does (whitepaper p.20). On path C the child `h_pre` changes per program, so it is open whether the stock recursion code binds it once `X` is bound: the code check of section 13 runs before the `L` key freezes (FP-8), and if the code does not bind it, `h_pre` is one more BabyBear digest in this leaf, at no wrap row beyond the check of item 3. The arity of the path C leaf-layer key is a const generic and is fixed before D12 measures that layer (whitepaper p.25).
  2. `L` has one more public value, `X`, and proves the inclusion of that leaf in `key_root` with a BabyBear Poseidon2 path, in the AIR that checks the app and leaf commitments. `L` runs the check on every path, so `L` keeps one key.
  3. The wrap checks `Canon` on the limbs of `X` and equates `X` with the cached-trace commitment of `L` that it already reads as one of the initial commitments of `L`, about 120 rows, ESTIMATE (`CT/proposals/stark-path.md:55`).

  An accepted wrap then gives an `L` proof whose child DAG is one that the plan fixed under this `prog_d`. This adds one refinement step to `wrap_refines` and no premise beyond A2b and A5 (`CT/proposals/stark-path.md:57`).
- The wrap sees `key_root` only inside the statement of `L`. It opens the `prog_d` binding as private witness and publishes only salted values. OpenVM's plain `app_exe_commit` and `app_vm_commit` never reach the ledger (`SP/RESEARCH.md:143-144`).
- A public program publishes `(prog_d, r_P)` in its manifest. A private program keeps both secret (`CO/FRONTEND.md:221`).
- The anchor read binds the contract address to `prog_d`. On chain, a valid proof for another program digest is refused at that read, MEASURED in XC-9 control R3 (`X9:94`; `CO/FRONTEND.md:223`).
- `machine_version` in `prog_d` keeps the keys of old programs when the rule family grows (`CO/FRONTEND.md:563`).

The wrap key, the `L` key and the log capacity do not depend on the plan (`B/CANDIDATE.md:204`). Only the per-program keys and the plan change from program to program.

### 4.4 What freezes with the universal keys

The log operation list, the capacity `(B, V, P, E)` with the encoding of `pv` and its per-operation bounds (section 3.2), the 9 by 30-bit limb encoding, `L_TX` and the public-table layout, the anchor template words with the anchor statement of section 7.3, the hash domains of section 3.6, the per-layer parameter sets and shape bounds of section 11.2, the public value `X` of `L`, and the internal layer's constant fields: AIR count and order, cached and preprocessed layout, interactions per row and public-value layout (`CO/FRONTEND.md:548-554`).

Freeze status: the freeze is in two steps. The interface shapes (bus tuple, kit interfaces, public statement, `claim_d`) freeze at loop step S11 in the form of `FD/ARCHITECTURE.md` section 1.11. It closes FP-1 (16-bit limbs, one LogUp bus index for each chip id, ports as permutation buses), FP-2 (74-word node statement with the log `(log_d, log_n)`, `key_root` and `keys_d`), FP-4 (one digest table), FP-5 (`view_root` as a header word), FP-6 (`n_steps` not public) and FP-7 (claim address and `ep_hash` from the binding preimage), each with LEAD choices F-1 to F-13 (`FD/DECISIONS.md`) for the owner to confirm. FP-3 stays open: the opening layer (`L`), `C_P` (N-11) and the index table of `L` are LEAD choices, but the carrier of the effect region from a multi-shard terminal state, the call-vector lanes of the ledger contract and the contract-side form of a collection (with the `openClaim` key and its public `n_steps`, so FP-6 holds for collections only once this is fixed) are missing. The universal key set freezes only after G1 to G4 pass and the owner confirms OD-4 (the app and leaf-aggregation parts), OD-17 and OD-18; OD-19 fixes the internal and `L` sets (FP-8). Plain `prog_d` is never call data (section 5). No foreign proof is folded into an accumulator, under any SRS; FV-FINAL checks it in circuit.

### 4.5 Production settings and PENDING

- The OpenVM CUDA cache of the stacked matrix and its RS codeword is off by default; turning it on avoids the eval-to-coefficient step and the forward NTT at opening time, at the cost of peak memory (`SP/RESEARCH.md:166`). Whether the plan turns it on for the instruction table waits for the on-and-off measurement: no source measures the saving (`SP/RESEARCH.md:25,274-275`).
- Challenges come only from the transcript. Every challenge of every layer is squeezed from the Fiat-Shamir transcript after the commitment it binds. A trace generation offload that draws a challenge from a device random source while the trace is still being produced, as ZK-Tracer does for LogUp (arXiv 2605.25493, Section 3.1, p.3), is unsound under Fiat-Shamir and is refused at review.
- PENDING: byte-equal internal-recursive keys across two chip sets, and the cost of the key-root inclusion check in the node (`SP/RESEARCH.md:270`); a planned shard against a generic shard in seconds per proved step (measurement M13, `B/CANDIDATE.md:204`); the static plan against a metered run (`SP/RESEARCH.md:286`).

---

## 5. Zero knowledge end to end

**Where the witness lives.** Every STARK layer (leaves, nodes, `L`) and the wrap prover run on the user's machine. STARK proofs and prover checkpoints are secrets like the witness. The wrap is the only proof that leaves the machine (`B/CANDIDATE.md:176`). Masking of intermediate layers is not needed while they stay local; it becomes required only if STARK layers move to untrusted hosts, which this design does not do (`B/CANDIDATE.md:178`).

**The wrap.**

- Blinded midnight-proofs PLONK with fresh CSPRNG blinders. Seeded proofs are a test mode only (`B/CANDIDATE.md:176`).
- Private witness: the proof of `L`, its Merkle roots, PoW witnesses, query positions, the log, `pv` and the onion digests. Public inputs: the statement only. Under the unified layout (OD-15) that is the call vector of `2 + L_TX` words (section 7.1).
- Nothing derived from the transcript of `L` becomes a public input or a constant of the wrap (`B/CANDIDATE.md:176`).
- The log is padded with `NOP` to its fixed capacity, so the count of used entries and the deferred-or-emulated split stay hidden. The call vector is padded with constrained zeros to `L_TX` (`CO/FRONTEND.md:210`).
- The anchor lane carries `C_P` or a lock value `A_j`, both salted digests (section 7.3). Plain `prog_d` never reaches the ledger.
- The DAG commitment `X`, the anchor statement words `mode`, `j` and `k`, and `run_tag` are public values of `L` only. `L` stays on the user's machine, and the wrap takes them as private witness, never as public inputs.

**Statement digests** (`FD/ARCHITECTURE.md` section 1.11, FP-4). The run salt is 8 BabyBear words from the user's CSPRNG, first in the preimage of `input_d`, fixed for the run at INIT, and never published on its own. `outcome_d` has no salt of its own; `outcome_pub = PoseidonFr_4(SHK_OUTCOME, pack(P2(DOM_OUTHIDE, input_d, outcome_d)))` hides it under the salt inside `input_d`, `input_pub = PoseidonFr_4(SHK_INPUT, pack(input_d))`, and `C_P` salts `prog_d` with the per-deployment `r_P`. The salts are private witness of the wrap. The shard plan binds to the unsalted program digest inside the STARK layers only (`B/CANDIDATE.md:176`). The tape enters the run behind the salted `input_d`.

**Shape.** Per-AIR heights and the set of AIRs present are part of a STARK proof's shape (`F/compat/opus.md:40`). Shapes stay inside the user's trust boundary, so no helper learns them. `n_steps` is not public: it stays private inside the SET forms, because the exact height leaks control flow and no source defines a padding rule for it (LEAD F-12, owner to confirm; `FD/ARCHITECTURE.md` section 1.11, FP-6). Leaf counts follow the public padding of the plan (`B/CANDIDATE.md:176`). A chunked call's chunk count is a function of the public transcript length; where a secret branch changes the length, filler chunks pad it to the entry bound (`CO/ATOMIC-CHUNKS.md:33,269`).

**Helpers.** Only two jobs could leave the machine without the plain witness: wrap MSMs on one-time-masked scalars, and grinding of `L` on a salted, squeezed challenge `H2(g, w)` (`CT/proposals/elastic.md:11,60-61`). The design keeps both on the user's machine: local devices and GPUs on hosts the user owns count as the user's machine (owner item OD-9). Never allowed: a helper that proves a STARK layer, a remote unmasked MSM, NTT, quotient or Merkle hash, a helper that proves `L`, the full sponge state on a helper, reuse of a blinding slot, or a helper-made slot (`CT/RESULT.md:201-202`).

**Aggregation.** No aggregator receives STARK proofs. One wrap settles one `ContractCall` (`CT/RESULT.md:204`). A user's own batch (owner item OD-12) keeps every witness local.

---

## 6. Optimizations

### 6.1 Execution optimizations on the STARK engine

| Optimization (`K21/BRIEF.md:39-44`) | Status on the STARK engine | Source |
|---|---|---|
| Profile-guided superinstructions | carries over; the 4-bus-request limit is gone, so a superinstruction may bind all arguments in one row; +1 to 3 selector columns | `F/compat/opus.md:149` |
| Straight-line mode `R_SL_*` | carries over; 42 to 340 times fewer steps, ESTIMATE; one CPU row plus 2 to 3 interactions per step | `F/compat/opus.md:150` |
| Fixed-base table for `ecMulGenerator` | carries over as a 1,024-row preprocessed AIR, or the multiplication goes to the log as `ECMULG` | `F/compat/opus.md:151` |
| SHA-256 spread tables shared with RANGE | replaced: the OpenVM `sha2` chip (17 rows per block). Spread chunks must be at most 15 bits in BabyBear and cost 12,000 to 19,000 cells per block against 7,728 for the bit layout, ESTIMATE | `F/compat/opus.md:152`; `CO/FRONTEND.md:87,596-602` |
| Compute once, consume as a claim (GP-2) | unchanged; break-even falls to tens of steps, ESTIMATE | `F/compat/opus.md:153` |
| Pre-admitted standard library | unchanged | `F/compat/opus.md:154` |

### 6.2 Preprocessing

In order of adoption: universal node over per-program leaf keys under `prog_d`; program as a cached trace with a resident codeword; straight-line blocks and superinstructions as leaf chips, capped by the node budget; static shard plan; a small shape menu on the stacked commitment; the deferred field log plan; prepared nodes and cached domains; per-program memory layout (`SP/RESEARCH.md:268-308`). Each one reports two measured numbers: planned against generic seconds per proved step, and recursion seconds saved per proved step (`SP/RESEARCH.md:266`).

### 6.3 Call time

Every G4 and k = 20 wrap figure in this section is a G4 figure on the benchmark `L` (`L_cached`). Ledger 9 refuses a G4 key (check M7-0, MEASURED), so these figures hold only under OD-21 option A or as the first layer of option C (section 9).

| Rank | Lever | Effect, ESTIMATE | Source |
|---:|---|---|---|
| 1 | Midnight Poseidon over Fr on the GPU for hashing and grinding of `L` | `L` hashing and grinding (1.34 x 10^7 permutations) from 56.3 s on the CPU to 1.34 s mid on the GPU | `PR/UD-SIZING.md` section 4; rates of `CT/RESULT.md:155` |
| 2 | Full-device GPU wrap: skew-robust batched MSM, NVRTC numerator, resident pipeline | wrap from 111.83 s to 2.45 to 4.49 s at k = 20 | `CT/RESULT.md:156`; `CT/models/paths.py` as used in `PR/UD-SIZING.md` section 4 |
| 3 | Unique decoding `L` at log blowup 3 | grinding of `L` 3.4 x 10^6 permutations (folding PoW 0, `mu` PoW 6) and half the codeword hashing of blowup 4: 2.5 s less mid on the full GPU path and 104.7 s less mid on the CPU path than the list decoding set at the same k | `PR/UD-SIZING.md` section 4 |
| 4 | CPU levers in halo2-algos (affine SRS, one pool, MSM retile, cached twiddles, dynamic chunks and others) | CPU wrap at k = 21 from 111.83 s to 38 to 57 s; fallback and oracle | `CT/RESULT.md:158` |
| 5 | Direct witness writer for the wrap advice | 7.15 s to 0.1 to 0.3 s | `CT/RESULT.md:160` |
| 6 | Path C | STARK part from 3.9 to 4.4 s to 1.3 to 3.2 s | `CT/RESULT.md:161` |
| 7 | Smaller `L` fitted to the measured area | `L` at height 2^19 in place of 2^20; not modelled for the section 3.4 set | `CT/RESULT.md:162` |
| 8 | Resident prover daemon with a one-use blinding slot pool | 19 to 98 s off a cold call | `CT/RESULT.md:163` |
| 9 | Hybrid routing: native entries for small circuits | 0.031 to 1.755 s MEASURED per native entry | `CT/RESULT.md:164` |
| 10 | Local elastic proving on user-owned GPUs | 1.3 to 2.6 times on the full GPU path while MSM is slow | `CT/RESULT.md:165` |
| 11 | Anchor lanes as key constants; one `L_TX` | about 780 rows | `CT/RESULT.md:166` |

Throughput, not latency: the user's own batch gives 19.3 s to 3.5 s per call at N = 64, ESTIMATE (`CT/RESULT.md:167`).

The path rows are whole warm calls: prove stages, witness fill, transaction prebuild, the balance and DUST proof and submit to inclusion. The row for today and the path 3 prove row cover the prove stages only. The call-time figures per path are ESTIMATE from `CT/models/call_time.py` with the levers of `CT/models/paths.py` (E1-V, path C, resident, the GPU `L` on paths 2 and 3) and the `L` work of the section 3.4 set, wrap at k = 20 (`PR/UD-SIZING.md` section 4). The wrap part of paths 2 and 3 is conditional on the fused quotient kernel and the MSM rewrite, neither built; the MEASURED wrap baseline with today's kernels is in section 8:

| Path | lo / mid / hi (s) |
|---|---:|
| Today, warm, prove stages only | about 195; about 203 with all PoW checks and the BabyBear part of `L` (`CT/RESULT.md:105`) |
| 3. Full GPU, prove stages only (leaf and node, `L`, wrap) | 6.31 / 8.90 / 16.20, ESTIMATE (`PR/UD-RECONCILE.md`) |
| 1. CPU only, wrap at k = 20 | 81.6 / 97.5 / 130.5 at the MEASURED CPU `L` rate of the model, 238,136 permutations per second mid (37.541 us per permutation on one X925 core, 8.94 core equivalents, host load 15 to 40, `CT/models/call_time.py`) |
| 2. Mixed CPU and GPU, wrap at k = 20 | 15.7 / 24.0 / 50.6 |
| 3. Full GPU, wrap at k = 20 | 11.1 / 19.0 / 45.0 |

Below about 20 s the floor is fixed per-call work (`CT/RESULT.md:22`): the balance and DUST proof, 3.05 s DERIVED from the XC-9 log (`CT/RESULT.md:52`); submit to inclusion, 3.92 s mid of 3.79 to 4.01 s, with 3.79 s MEASURED (`CT/RESULT.md:53`; `X9:142`); trace generation, 1.95 s ESTIMATE; and the BabyBear part of `L`, 1.81 s ESTIMATE (`CT/RESULT.md:45,48`).

### 6.4 Rejected

Proof-format changes the ledger 9.1.0.1 verifier rejects (LogUp lookups, quotient limbs, grouped commitments and others); degree 9; `EF` of degree 5; list decoding in the internal and `L` layers, whose MCA bound beyond unique decoding has no complete written proof (BCHKS25 Theorem 4.6 is a proof sketch, `PR/G5-PAPER-MAP.md` Verdict); list decoding with BCGM26 Theorem 9.2 as the bound, which needs 35 to 50 PoW bits per check, above the 30.9 bits one BabyBear grind digit carries (`PR/UD-SIZING.md` section 5.2); unique decoding `L` at log blowup 1, whose worst wrap row is over the k = 21 gate (`PR/UD-SIZING.md` section 3.1); query PoW 22 on `L`, which saves 2 queries per round and costs 9.4 x 10^6 permutations (`PR/UD-SIZING.md` section 4); parameters that rest on the p.23 remark; lower grinding than the bound needs; accumulation and folding schemes in place of STARK recursion (WARP, Arc, Accumulation Without Homomorphism, LatticeFold+, Neo, SuperNeo): no paper measures one against STARK recursion, Arc's proven field bound needs about 2^188 against `|EF|` = 2^123.6, WARP over RS is free of conjecture only at unique decoding or the cube-root bound, Accumulation Without Homomorphism has bounded depth with a concrete attack beyond it, none has zero knowledge, and the lattice schemes rest on estimator security over 61 to 64-bit fields; a decider does not exclude them, because it can be a fixed-size STARK shard before `L` (`PR/PAPERS-ANALYSIS.html` finding 3); Poseidon2-BabyBear leaves under an Fr tree; the app proof straight into `L`; a CPU STARK shard prover; fr-terminal form B; `L_TX` = 2,048 at today's estimates; chunked calls as a call-time lever, because `k` chunks cost `k` times `L` plus the wrap (`CT/RESULT.md:186-208`).

---

## 7. Compact frontend, E-1 placement V, then atomic chunked calls

These are the first build items. They need no bridge layer for their first milestones and they fix the words that freeze with the wrap key.

### 7.1 Compact frontend

**Tap.** The frontend reads `Lflattened` after `optimize-circuit2` (`CC/compiler/passes.ss:138`), the exact input of `reduce-to-zkir`. The manifest and the TypeScript runtime come from `analyzed-ir` unchanged. One new analysis pass, `add-sherlock-anchor`, follows `determine-ledger-paths`. The frontend replaces `reduce-to-zkir`, the `zkir-v3 compile-many` key step and the per-circuit key fingerprint (`CO/FRONTEND.md:59-63`).

**Workflow.**

```text
sherlock compile contract.compact -o build/    # images, abi.json; TypeScript runtime unchanged
sherlock plan build/                            # witness-free plan, per-program keys, prog_d
sherlock simulate build/ <entry> ...            # reference machine, no proof
sherlock deploy build/ --network local          # anchor cell C_P, every entry holds the wrap key
sherlock prove build/ <entry> --preimage ...    # STARK layers on the GPU
sherlock call build/ <entry> --network local    # host order, ending in submit
```

Source: `CO/FRONTEND.md:252-266`.

**Lowering.** Stage 1 runs in straight-line mode: `pc` indexes a per-program instruction table, operands are register records, results are appends (`CO/FRONTEND.md:146`). Field arithmetic uses FIELD-EMU: FMUL is one row of 194 main columns, and 2^20 multiplies prove in 9.126 s on the GPU, MEASURED (`F/bench/RESULTS.md:30`; `CO/FRONTEND.md:571`). `transientHash` is `SPONGE(n)` in the log or POSEIDON-FR-EMU inline. `persistentHash` uses the OpenVM `sha2` chip. `ecMul` and `ecMulGenerator` are log entries while `E` slots last, otherwise JUBJUB-EMU. A native call with no kit is a compile error that names the kit (`CO/FRONTEND.md:162-171`).

**Witnesses and transcript.** The tape is `Cons(salt, Cons(selector, args ++ comm_rand ++ witness, read and callee results in program order))`, static in length and layout per entry (`CO/FRONTEND.md:175-178`). Ledger reads are `popeq` values from the tape, which the ledger replays and refuses when stale. Writes are effect words at static indices; a guarded-off impact writes zeros, and the host inserts the same `Noop` merges as the native transaction builder (`CO/FRONTEND.md:180-189`).

**Call vector.** One fixed layout of `2 + L_TX` words for every program:

| Lanes | Content |
|---|---|
| 0 | `binding_input` (E-1, section 7.2) |
| 1 | `comm` |
| `2 .. 2+A` | anchor prefix: uncached `idx` of cell `(0)`, `popeq a_in`, where `a_in` is `C_P` for an ordinary call and the lock value of section 7.3 for a chunk |
| `2+A .. 2+A+L_e` | effect words |
| rest | constrained zero |

Source: `CO/FRONTEND.md:202-210`. Under the unified layout a GP run is a Sherlock program whose effect words write the run statement, so the GP path and the Compact path share the one wrap key (`CO/FRONTEND.md:212`; owner item OD-15).

**`L_TX`.** 1,024 words is a provisional profile frozen with the wrap key after M7 (`CO/FRONTEND.md:483`). The largest of 27 checked entry points needs 474 words, STATIC COUNT (`CO/FRONTEND.md:73`). The single-lane row budget gave 1,683,792 rows of 2,097,145 at `L_TX` = 1,024 and a ceiling of about 1,847 words at k = 21, ESTIMATE on the single-lane base of 1,560,912 rows (`CO/FRONTEND.md:72,481,509`). The G4 shape replaces that base; M7 measures the binding rows per word on G4. The largest checked contract, `zerocash spend`, needs `V` 111, `P` 39, `E` 0, ESTIMATE from STATIC COUNT (`CO/FRONTEND.md:83`). At the recommended capacity `V = P = 256` that is a margin of 2.31 times on values and 6.56 times on permutations, DERIVED (`CT/proposals/wrap-rows.md:144`); the ninefold margin of `CO/FRONTEND.md:83` belongs to the historical 1,024 / 1,024 / 16 profile. An entry with `L_e > L_TX - A` is a compile error until chunked calls exist (`CO/FRONTEND.md:518`).

**Milestones** (`CO/FRONTEND.md:691-697`):

| # | The developer sees | Pass condition |
|---|---|---|
| M0 | `sherlock compile` prints `abi.json` and the effect template | golden vectors: `effect_vector(preimage)` equals the native public-input vector word for word for every example entry; four mutants caught |
| M1 | `sherlock simulate` runs the counter on the reference machine | 100 random preimages per entry equal native; `N_e` equal for two tapes of one entry; `spend` at most 550 steps; halt flag check: a run that does not reach its declared end has no accepting result, with failure as a fixed point of the step function in the Lean reference machine (Valida ISA spec, Section 7, p.17); Gassmann et al. record an SP1 proof that verifies for a run that aborted mid-run (arXiv 2508.17518 p.7) |
| M2a | `sherlock plan` and a STARK-only prove give a verified leaf and root on the GPU | root verifies under the stock OpenVM verifier; two chip sets give byte-equal internal-recursive keys |
| M2b | one wrap proof that ledger 9 `VerifierKey::verify` accepts | `2 + L_TX` public inputs; negative controls; EQ-ZK on two wraps |
| M3 | deploy and call on the local devnet | XC-9 harness; refusals for a tampered bit, another `C_P`, a stale read |
| M4 | acceptance sets 1 and 2 end to end | vectors equal native; the forced-overflow cases of section 3.2, with the mandatory entries `R` reserved, give the same vector; negative cases: no proof verifies for a run that did not reach its declared end, and the wrap refuses an output that no call produced (OpenVM `OUTPUT` is not tied to a `CALL`, OpenVM whitepaper p.22) |
| M5 | Jubjub contracts, one GP-2 consumption, one GP-3 aggregation | Schnorr equal to the runtime |

M0 and M1 need no heavy job and prove the frontend's correctness claim before any chip exists (`CO/FRONTEND.md:687,700`).

### 7.2 E-1: the entry label, placement V

**Mechanism.** The ledger computes `binding_input = le31(SHA256(m))` from public transaction bytes `m`. The label window starts at byte offset 58 for every call, and SCALE length prefixes are prefix-free, so the bytes at offset 58 equal `enc(e)` for exactly one label (`CO/E1-entry-label.md:65-70`). The window starts at offset 58 in each of four calls on the local devnet, MEASURED (`E1:61`). The statement is:

```text
pi[0] = le31( SHA256( m ) )
m[0 .. 26)             = "midnight:binding-input[v1]"
m[58 .. 58 + |enc(e)|) = enc(e)
```

Source: `CO/E1-entry-label.md:156-158`.

**Placement V.** A library routine `bind_label(e)` reads `m` from the private tape, pins the two windows, computes SHA-256 on the `sha2` chip and writes `bind = le31(digest)` into the outcome. The wrap requires lane 0 equal to `bind`, as lane 1 equals `comm`. There is no hash in the wrap and no capacity limit, because SHA blocks are run steps (`CO/E1-entry-label.md:173-177`). Cost: 3 to 32 SHA blocks, 51 to 544 leaf rows, under 0.1 s, and 0 wrap rows, ESTIMATE (`CT/RESULT.md:143`).

**V+.** Under plain V the domain bytes and the label offset are constants of the program image, so they sit inside every `prog_d` (`CO/E1-entry-label.md:238`). V+ moves them out of the image (`CT/proposals/call-shape.md:71-75`; `CT/RESULT.md:145`):

1. `bind_label` reads the layout parameters from the private tape: the domain bytes with their length and the label offset.
2. It uses exactly those parameters for the window checks: `m[0 .. |domain|)` equals the domain bytes, and `m[offset .. offset + |enc(e)|)` equals `enc(e)`. The program image holds no domain byte and no offset.
3. It hashes the same parameters into `layout_d = P2(DOM_LAYOUT, |domain|, domain, offset)` and writes `layout_d` as one statement word.
4. The wrap pins `layout_d` to a constant of its key, at most 120 wrap rows, ESTIMATE.

A tape that names any other layout gives another `layout_d`, which the wrap refuses. So a ledger layout change replaces one key constant of the universal wrap, and no `prog_d` changes.

**Soundness.** If the real label `e'` differs from `e`, then `m' != m`, and `(m, m')` is a collision of SHA-256 truncated to 248 bits, about 2^124 hash calls. The anchor read binds the contract address to `prog_d`, which contains the entry's code (`CO/E1-entry-label.md:192-195`).

**Rebuilds.** Under V a transaction rebuild re-proves the whole run (`CO/E1-entry-label.md:218`). Every byte of `m` is fixed when the call intent is built; fees, DUST and balancing use a second intent and never rebuild the call. The triggers left are TTL expiry and a cost-model change, both avoided by a TTL of at least 10 times the prove time; a stale read re-proves the run under every placement (`CT/RESULT.md:141`). The host keeps the order prove, then `balanceUnbound`, until experiment D4 passes. On the full GPU path a rebuild is one call, about 20 s (`CT/RESULT.md:144`).

**Alternatives.** Placement S (SHA in the wrap) adds about 62,000 to 65,000 wrap rows at 32 SHA blocks, MEASURED by the cost model of a midnight-proofs prototype of the bind (`E1:113-128`), and a SHA chip that the G4 shape excludes. Placement R (SHA in `L`) stays the recorded alternative if D4 shows frequent rebuilds and M7 shows 60,000 spare rows at the key's k. A separate binding shard (placement B) is dominated, at +3.9 s of GPU per call (`CT/RESULT.md:142,147`). Owner item OD-5.

**Tests** (`CO/E1-entry-label.md:283-295`). T0 rebuilds `m` from public transaction bytes and compares `le31(SHA256)` with `public_inputs[0]`; it gates the design. T1 is the on-chain control with lane 0 free. T2 holds the bind-circuit unit tests and eight mutants. T3 to T6 cover the honest prover, the devnet, disabling and native callers. T7 (seal rows for placement S) is not needed under V. Pass rule: T0 equal; T1 or T1m accepted with lane 0 free; T2 kills every mutant; T3 gives no proof; T4 (i) accepted and (iii) refused; T5 and T6 as stated.

**Results** on a local ledger 9.1.0.1 devnet (node 2.1.101), MEASURED (`E1:3,41-46,70-82,96-107`):

- T0 passes. The host rebuild of `m` equals the ledger's `binding_input` on the XC-9 call and on three devnet calls. Calls with a fallible transcript or non-empty effects are not run (`E1:64`).
- T1 passes, so E-1 is real on the current ledger. A contract holds today's decider key under two labels, `acceptRun` and `acceptRunB`. A fresh decider proof of `acceptRun`'s transcript, bound to a call labelled `acceptRunB`, is included at block 1,085, and the state shows `acceptRun`'s write (`E1:74-80`). The honest proof replayed under the other label is refused because lane 0 differs; that control says nothing about E-1 (`E1:73,81`).
- T2 in part. A midnight-proofs prototype of the bind statement (`BindLabel`: the domain and label windows pinned, SHA-256 over the preimage, `le31` of the digest as lane 0) accepts the honest call and verifies a real proof at k = 13. It refuses the attack call in MockProver, and with a forged window its real proof does not verify. It costs 5,749 rows for the 3-block preimage and about 1,928 rows per SHA block; the window pins add 0 rows (`E1:96-107,115,126`).
- Open: the remaining T2 mutants, the `label_d` link from the run outcome to the pinned window, and T3 to T6 (`E1:130`). The right alignment of the stdlib `sha2_256_varlen` (`E1:129`) applies to placement S only. Under placement V, `m` is a tape array padded to the per-entry length `M_B(e)` of `abi.json` with the true length `len_m` on the tape; the window offsets are absolute; `bind_label` runs `ceil((M_B(e) + 9) / 64)` compressions and selects the digest after block `ceil((len_m + 9) / 64)` (LEAD F-13, `FD/ARCHITECTURE.md` section 1.11, FP-7; vector `bind_len_select`).

### 7.3 Atomic chunked calls: modes 1F and 1G

Chunked calls are the capability for one logical call above `L_TX`. They are not a call-time lever.

**Mechanism.** One logical call runs as `k` contract calls in one intent of one transaction on the unmodified ledger, with a forward claim chain (chunk `j` claims chunk `j+1`) and an anchor lock (the anchor cell holds a run-specific value during the run) (`CO/ATOMIC-CHUNKS.md:26-28`).

- **Mode 1F (default).** All chunks are fallible-only, in one segment. The ledger applies a segment's fallible actions as one unit and drops all of them on any error, so a middle failure leaves contract state unchanged, and the transaction is included with its fee paid (`CO/ATOMIC-CHUNKS.md:29,157`).
- **Mode 1G.** All chunks are guaranteed-only, for a callee of a guaranteed caller. A failure makes the transaction invalid (`CO/ATOMIC-CHUNKS.md:29,82`).
- **Refused.** Mixed chains. A guaranteed claim cannot target a call with a fallible transcript (`CO/ATOMIC-CHUNKS.md:30`).

**Rules.**

- R-phase: one phase for all chunks.
- R-cut: cut only where the native run has stack depth exactly 3, at a boundary the run certifies; the slice shard checks the cut in the proof.
- R-ctx: hoist every read of context slot 0 and effects slot 1 into a prologue in chunk 1.
- R-outcome: 1F gives all applied or none applied with fee paid; 1G gives all applied or invalid.

Source: `CO/ATOMIC-CHUNKS.md:79-82`.

**Anchor values.** The anchor cell holds `C_P`, never plain `prog_d` (section 4.3). The lock follows the salted anchor:

```text
A_0 = A_k = C_P
A_j = PoseidonFr_4( SHK_LOCK, pack( P2(DOM_LOCK, r_P, prog_d, run_tag, j) ) )      1 <= j < k
```

`run_tag` is a fresh salted value per run (`CO/ATOMIC-CHUNKS.md:129`). `SHK_LOCK` differs from `SHK_PROGC`, so no lock value equals `C_P` except by a collision, and `r_P` with `run_tag` keeps every lock value unlinkable to `prog_d` and to other runs. The last chunk writes `C_P` back, so every ordinary call after the run matches the anchor again. Plain `prog_d` never reaches the ledger. This replaces the plain `prog_d` lock values of `CO/ATOMIC-CHUNKS.md:126,276,281` (LEAD, owner item OD-16).

**Anchor statement and the one wrap.** Every call, chunked or not, carries the anchor statement `(mode, j, k, a_in, a_out)` among the statement words of `L`. The anchor prologue that `add-sherlock-anchor` emits into every entry computes it, and the slice shard proves `(j, k)` and `run_tag`:

| Case | `mode` | `a_in` (lane of `popeq`) | `a_out` | Anchor write |
|---|---|---|---|---|
| Ordinary call | ORD | `C_P` | `C_P` | none; user code cannot address cell `(0)` |
| Chunk 1 of `k` | CHUNK | `C_P` | `A_1` | `ins anchor := a_out` in the effect region |
| Chunk `j`, `1 < j < k` | CHUNK | `A_(j-1)` | `A_j` | same |
| Chunk `k` | CHUNK | `A_(k-1)` | `C_P` | same |

The universal wrap checks the same constraints for both modes: the `popeq` lane equals `a_in`; `a_in` and `a_out` are outputs of `DIGEST` entries of the log; the tag of the `a_in` entry is `SHK_PROGC` when `mode` is ORD or `j = 1` and `SHK_LOCK` otherwise; the tag of the `a_out` entry is `SHK_PROGC` when `mode` is ORD or `j = k` and `SHK_LOCK` otherwise. Inside the STARK the run computes both `d2` operands from its own `prog_d`, `r_P`, `run_tag` and `j`, and writes the lock word of the effect region from `a_out`. A chunk adds at most one `DIGEST` entry over an ordinary call (2 values, 1 permutation) and the commitment `comm_(j+1)` of its claim, counted as `comm` is (`CO/FRONTEND.md:446-447`). The anchor entries are mandatory: the planner reserves them before any discretionary operation (`R_A` of section 3.2: 1 entry for an ordinary call, 2 for a chunk), so a full log never sends an anchor entry to tier E. The universal-key chunk test T3 includes chunk 1, chunk `j` and chunk `k` runs whose discretionary operations fill the log; T1 and T2 have no deferred log. Modes 1F and 1G use the same anchor rule.

**Atomicity.** No dropped suffix (`RealCallsSubsetCheckFailure`), no dropped prefix (without chunk `j-1` the anchor holds `C_P` or another run's lock, so the `popeq` of `A_(j-1)` fails), no interleaving by other intents, no interleaving inside the intent (the anchor lock), replay bound through `binding_input`, and fees checked for the whole transaction before application (`CO/ATOMIC-CHUNKS.md:106-112`). The lock argument needs a lock check in every entry of the contract (`CO/ATOMIC-CHUNKS.md:110`): every Sherlock entry has the anchor prologue, and so does every native-routed entry of a hybrid contract (section 9, owner item OD-8). During a run the anchor holds `A_j`, which differs from `C_P`, so every such entry fails in its prologue.

**Binding inside the proof.** The claim address, the claim `ep_hash`, `comm_{j+1}`, `pi[1] = comm_j`, the anchor statement and the slice bounds are all fixed by the proof (`CO/ATOMIC-CHUNKS.md:120-127`, with the lock values above). The specification takes the claim address and `ep_hash` from a per-chunk binding shard (`CO/ATOMIC-CHUNKS.md:272`). Under placement V, `bind_label` computes them inside the chunk's run (LEAD F-13, `FD/ARCHITECTURE.md` section 1.11, FP-7): the claim address is `m[|domain| .. |domain| + 32)` with the checked offset `|domain| + 32` (`ML/ledger/src/verify.rs:2151-2156`), and `ep_hash = SHA256("midnight:entry-point" and 12 zero bytes, raw label)` (`ML/onchain-state/src/state.rs:913-918`).

**Costs, ESTIMATE** (`CO/ATOMIC-CHUNKS.md:165-177`): about 30 words of overhead per chunk, so about 990 words per chunk at `L_TX` = 1,024; 5 to 30 chunks per transaction with the code defaults; about `k` times the fee of one call; proving is one run plus `k` times (slice shard, binding, `L`, wrap).

**Tests** (`CO/ATOMIC-CHUNKS.md:305-358`). T1 at ledger level with no proofs: 13 cases, among them the middle failure in 1F and 1G, a non-atomic control, dropped and reordered chunks, a mixed chain and a cut at depth 4. T2 on the XC-9 devnet with native keys and a `chunky.compact` contract. T3 with universal-key chunks after M5. The `atomic-chunks` worktree holds the T1 test (`atomic_chunks.rs`) and the T2 contract and script. Results are PENDING. The anchor values in every tier take the `C_P` and `A_j` form above. T1-i places a native-style call between chunks at ledger level (`CO/ATOMIC-CHUNKS.md:324`), and T2's extra entry `poke` has no anchor check (`CO/ATOMIC-CHUNKS.md:338`), so neither tests a native-routed entry. T2 adds that negative control:

- **T2-N.** `chunky.compact` gains an entry `nat`, routed native under its own key, with the native anchor prologue of section 9. A `nat` call placed between `c1` and `c2` is refused: either no proof exists for its prologue (the anchor holds `A_1`), or a proof made against the state before the run fails the anchor `popeq` on the ledger.
- **T2-N0 (control).** The same entry without the prologue, between `c1` and `c2`, is included and its write lands inside the run. This shows that T2-N can detect an interleaving.

---

## 8. GPU wrap

Every G4 and k = 20 wrap figure in this section is a G4 figure on the benchmark `L` (`L_cached`). Ledger 9 refuses a G4 key (check M7-0, MEASURED), so these figures hold only under OD-21 option A or as the first layer of option C (section 9).

**Requirement.** The wrap runs on the user's GPU, with the stock CPU path as fallback (`B/CANDIDATE.md:446-448`).

**Shape G4.** Every gate and lookup has degree at most 5 (extended domain 4n). There are 4 identical lanes of 8 advice columns (32 advice) over about 14 shared fixed columns, 18 permutation columns, 4 lookups into one fixed 2^16-row table, and gate rotations within {-1, 0, +1}. No foreign-field, pairing or SHA chip (`B/proposals/fable-gpu-wrap.md:15,359`). Poseidon over Fr uses the hinted 19-row stdlib layout; the 70-row chip has a degree-6 gate and would double the extended domain (`B/proposals/fable-gpu-wrap.md:365`). One lane takes one query of Merkle work, one extension-field coefficient and one log slot, so a region is a fixed program on 4 data streams (`B/proposals/fable-gpu-wrap.md:16`).

**k.** The smaller of 20 and 21 at which the measured rows fit with 15 percent margin (`B/proposals/fable-gpu-wrap.md:363`). The row estimates are in section 3.5: k = 20 on the G4 model row, k = 21 on the worst gadget row. One k step is worth 1.1 to 2.2 s on the full GPU path, 3.5 s on the mixed path and 11 to 17 s on the CPU path, ESTIMATE (`CT/RESULT.md:107`).

**Kernels**, in build order (`B/proposals/fable-gpu-wrap.md:310-319`):

| Kernel | Work | Share of today's CPU wall |
|---|---|---:|
| K0 | Fr field header, 8 x 32-bit Montgomery limbs, shared with the Fr Poseidon kernel of `L` | enabler |
| K1 | NTT over Fr, coset shift fused, batched over columns | 0.15 to 0.17, ESTIMATE |
| K2 | MSM on G1, cached affine tables, batched, digit-aware | 0.264, MEASURED split (`B/proposals/fable-gpu-wrap.md:312`) |
| K3 | coset-tiled quotient numerator and vanishing division | 0.42 to 0.43, ESTIMATE |
| K4 | grand products for the permutation and lookup columns | inside the 0.193 residual |
| K5 | evaluations and multiopen | inside the residual |
| K6 | lookup permutation sort for the 2^16 table, CPU first | inside the residual |

**Estimates.**

- G4 device wall: 1.9 to 7.1 s at k = 20; whole wrap with residual 5 to 12 s at k = 20, ESTIMATE conditional on the fused quotient kernel and the MSM rewrite (`GW` section 3), against 111.83 s MEASURED on the CPU at k = 21 for today's decider (`B/proposals/fable-gpu-wrap.md:145-148`; `K21/measured/M-k21.md:41`).
- With the full-device pipeline and the rewritten MSM: 3.5 s (2.45 to 4.49) at k = 20, ESTIMATE (`CT/models/paths.py`); it needs an MSM of 40 to 70 ms per 2^20 (`CT/RESULT.md:87`).
- The GPU MSM as built takes 0.479 s per 2^20, MEASURED on a loaded host; idle is 0.2 to 0.35 s, ESTIMATE. The MSM stays split between CPU and GPU while it is above 0.2 s per 2^20 (`CT/RESULT.md:101`).
- Memory about 5 GB at k = 19, ESTIMATE (`B/proposals/fable-gpu-wrap.md:150`), against 33.34 GB MEASURED for today's shape at k = 21 (`B/proposals/fable-gpu-wrap.md:150`).
- Proof about 8 to 9 kB, ESTIMATE, against 6,432 bytes today; ledger verify time grows with the column count and is not measured; the verifier key holds 14 fixed and 18 permutation commitments against 63 in today's 2,217-byte key, and its bytes are PENDING (`B/proposals/fable-gpu-wrap.md:154-156`).
- Floor without the shape: today's k = 21 shape cannot go below 21.5 s even with perfect K1 to K3 (`B/proposals/fable-gpu-wrap.md:321`).

**Measured on today's decider at k = 20 and k = 21 and a decider-shaped proxy at k = 19**, a stand-in for the G4 wrap (`GW` sections 1 and 2):

- Byte identity. Seven configs, CPU and GPU, give one proof SHA-256, and every proof passes the library verifier, ledger 9 `VerifierKey::verify` and a negative control. The GPU quotient numerator matches the CPU in all 4,194,304 extended-domain values. The foreign-field ECC chip of midnight-circuits 7.3.0 draws its incomplete-addition point from `OsRng` on every synthesis, so byte-identity tests use a test-only seed patch, active only under `GW_DET=1`; the G4 wrap has no foreign-field chip (`GW:9-10`).
- Time, one run at load average 16 to 24: CPU 243.3 s; full GPU path 95.0 s (2.6 times); GPU NTT, coset and quotient without the GPU MSM 131.6 s; GPU MSM alone 197.2 s (`GW:11`; `GWR:3-6`). The quotient numerator falls from 101.6 s to 8.4 s. Advice synthesis, about 30 s, stays on the CPU and is the next floor. A quiet host proves the same decider on the CPU in 68.1 s (`GW:11`), so the ratio holds for the same loaded run only.
- MSM skew. On the real witness the stock GPU MSM takes 622 to 647 s over 45 calls: witness columns repeat small range-checked values, and sppark accumulates each bucket serially. The MSM skew mask, `MSM(s) = MSM(s + r) - MSM(r)` with a fixed vector `r` and `MSM(r)` cached per base table and length, brings the 45 calls to 23.2 s with unchanged proof bytes (`GW:10-11`). The mask is witness-independent and runs on the user's GPU; it is not the helper masked MSM of OD-9.
- Final table, one proof per cell under load (`GW` section 1): full GPU 34.8 / 95.0 / 155.2 s against CPU 118.8 / 243.3 / 441.7 s at k = 19 (proxy), 20 and 21; MSM on the CPU 45.1 / 131.6 / 247.9 s; quotient numerator 4.0 / 8.4 / 17.4 s against 58.1 / 101.6 / 191.8 s; all byte-identical and accepted by ledger 9, k = 21 also by the unpatched standalone verifier (`f15fb9d`). Quiet-host projection at k = 21: about 83 s with the GPU MSM (about 1.35 times) and about 77 s without (about 1.45 times) against 111.8 s, ESTIMATE (`GW` section 2). Today's kernels are far from the 2.45 to 4.49 s wrap at k = 20 that call-time path 3 assumes; those figures rest on the fused quotient kernel (target under 1 s at k = 21, ESTIMATE) and the MSM rewrite, both still to build (`GW` section 3).
- PENDING: a quiet-host repeat with at least 3 repetitions, and every G4 figure (`GW` section 3).

**Fallback triggers** for an Fr-terminal layer between `L` and the wrap: rows over budget after M7, or a measured GPU wrap above 8 s at the chosen k after K1 to K5, or an owner request for a larger log capacity than the wrap rows allow (`B/proposals/fable-gpu-wrap.md:300`). Form A of that layer is not built now (owner item OD-11).

---

## 9. Midnight side

**Ledger key reader: open BLOCKER (OD-21).** The ledger checks each `ContractCall` with `midnight_transient_crypto::proofs::VerifierKey::verify` on the key installed under the entry label (`K21/measured/M-k21.md:51`). The wrap is a midnight-proofs PLONK circuit with KZG over BLS12-381 and the Blake2b transcript, the same proof system as the existing decider, but the same proof system does not make a key readable. Ledger 9 reads a key with `MidnightVK::read`, which rebuilds the constraint system from `ZkStdLib::configure` under the architecture flags in the key bytes (`ML/transient-crypto/src/proofs.rs:533`; `midnight-zk-stdlib` 2.4.0 `src/lib.rs:1573-1587`). A key with the G4 lane gates is refused at parsing under three architecture headers, MEASURED on a toy key (check M7-0, `research/sherlock-m7-wrap-20261112/RESULTS.md` in the `m7-wrap` worktree, first commit `c039d5dd8`, sections 1.1 and 1.3). The single-lane stdlib shape is 2,370,944 to 3,051,763 rows, ESTIMATE, over the k = 21 gate. The path is an owner decision with no adopted default: (A) a G4 architecture in `midnight-zk-stdlib` and a ledger release that reads it; (C) the G4 wrap off chain, then a `ZkStdLib`-architecture decider that verifies it and exposes the 1,026 public inputs, two BLS12-381 PLONK layers, with today's decider at 68.1 to 111.8 s on a quiet CPU (MEASURED, `K21/measured/M-k21.md:41`) and the decider over a G4 proof not sized; (E) an Fr-terminal layer between `L` and a `ZkStdLib`-architecture wrap (OD-11), being sized. M7 rejects (B) the single-lane stdlib wrap and (D) a return to list decoding. Each option changes a standing rule, and the owner decides which: (A) changes the ledger, against the rule that the co-processor needs no ledger change (CPR-004, MID-001); (C) puts two BLS12-381 PLONK layers on the path, against the owner rule of one BLS12-381 wrap; (E) builds the Fr-terminal layer whose form A OD-11 and LW-014 do not build. Every exported entry of a Sherlock contract holds the same universal wrap key (`CO/E1-entry-label.md:188`). Small entries can stay native under their own keys, outside the Sherlock key set (owner item OD-8).

**Native anchor prologue.** A native-routed entry of a hybrid contract runs the anchor prologue too, or a submitter can place it between two chunks of a run (section 7.3). Its `popeq` operand is a transcript word that the native circuit must tie to `C_P`, and `C_P` exists only after deploy, so it cannot be a constant of a native key made at compile time. The prologue (LEAD, owner item OD-16):

- The `add-sherlock-anchor` analysis pass shifts the ledger paths for the TypeScript and Sherlock backends (`CO/FRONTEND.md:121`); in a hybrid contract the native backend must take the same wrapped state. The pass gives a hybrid contract a second cell `anchor_home` beside the anchor. Deploy writes `C_P` into both; no entry can write `anchor_home`, and the frontend refuses any write to it.
- A native entry starts with two uncached reads, `popeq anchor` and `popeq anchor_home`, and its circuit asserts that the two read values are equal. The ledger checks each read against the state, so during a run (`anchor = A_j`, `anchor_home = C_P`) no valid native call exists.
- The prologue needs no deploy-time constant in the native circuit, and `prog_d` holds no native key, so the native keys and `C_P` do not depend on each other.
- Sherlock entries keep the one-cell anchor prefix of section 7.1: their proof fixes `a_in` itself.

**Key and transaction limits.**

| Limit | Value | Today's decider | Source |
|---|---|---|---|
| Verifier key bytes | 50,000 (`ML/transient-crypto/src/proofs.rs:422`) | 2,217, MEASURED | `X9:29,130` |
| Public inputs per call | 65,536 (`MAX_CALL_PUBLIC_INPUTS`) | 70; the wrap has `2 + L_TX` = 1,026 under OD-15 | `X9:130`; `CO/FRONTEND.md:488` |
| `VERIFIER_MAX_DEGREE` | 14; it bounds the number of public inputs, not the circuit k | k = 21 key verifies | `K21/measured/M-k21.md:53,59` |
| Transaction bytes | 1,048,576 | call 10,421 bytes, 1.0 percent | `X9:62,119,128` |
| Block limits | read 2 s, compute 2 s, `block_usage` 1,000,000, `bytes_written` 50,000 | compute share 0.53 percent | `X9:119,128` |
| Verify time | not a ledger constant | 0.025 s at k = 20, 0.049 s at k = 21 (repeat), MEASURED | `K21/measured/M-k21.md:55` |
| Validation at 1,026 public inputs | not a ledger constant | 3.16 ms (code model) or 2.37 ms (file), against a 15 ms floor, ESTIMATE | `CO/FRONTEND.md:488` |

**Evidence on the unchanged ledger.** These records are for the existing k = 21 run decider over stand-in leaves with 70 public inputs. They show the transaction path, not the G4 wrap.

| Record | Result | Source |
|---|---|---|
| Local devnet, node 2.1.101, ledger 9.1.0.1 line | PASS: deploy with the k = 21 key on `acceptRun`; one call included at block 237 with a 6,432-byte proof body; fee 220,207,775,523,237; submit to inclusion 3.79 s | `X9:3,57,60,63-64` |
| Local refusals | R1 result changed, R2 one proof bit flipped: `Malformed(InvalidProof)`. R3 valid proof to a contract with `prog_d + 1`: refused at the guaranteed transcript read | `X9:92-94` |
| Preview deploy | block 71100; fee 1,995,523,200,000,001; installed key sha256 `d18989e8...64fd55`, the XC-9 key | `PV:54-55,60` |
| Preview call | block 71431; result 6, steps 6; fee 315,789,942,857,143 | `PV:77,80,82` |
| Preview refusals | R1 and R2: custom error 115 (`InvalidProof`); R3: custom error 104 (`Transcript`) | `PV:103-107` |
| Local devnet, E-1 | a decider proof of `acceptRun`'s transcript in a call labelled `acceptRunB` of a contract with the same key under both labels: included at block 1,085 with `acceptRun`'s write; the unchanged honest proof under the other label: `Malformed(InvalidProof)` | `E1:72-80` |

The Preview node reports build `2.1.101-bbd616b1` (`PV:18-20`), and the Preview pin records the same build (`TP:108,120`). The Preview record does not qualify the wrap; the wrap deployment is a later rehearsal (`PV:5`).

**PENDING for the wrap.** Verifier-key bytes against the 50,000-byte limit, proof bytes (8 to 9 kB, ESTIMATE), verify time, fee at `2 + L_TX` public inputs, and the deploy and call of a G4 wrap key on the local devnet, then on Preview.

---

## 10. Build order

Each step is useful whatever later experiments show. Heavy jobs run one at a time under `heavy-run`.

**Phase 1: the first build items, no heavy job.**

1. Compact frontend M0 (golden vectors) and M1 (reference machine, differential against native) (`CO/FRONTEND.md:691-692`).
2. E-1 in placement V: the `bind_label` library routine on the `sha2` chip and the V+ `layout_d` word (`CO/E1-entry-label.md:175`; `CT/RESULT.md:145`). T0 and T1 pass (section 7.2). First D4 (rebuilds with a new TTL and with the DUST intent added, `CT/RESULT.md:289`) and T0 on calls with a fallible transcript or non-empty effects; then the bind-circuit tests T2, with every mutant and the `label_d` link, and T3 (`CO/E1-entry-label.md:285-287`; `E1:130`).
3. Atomic chunked calls: T1 at ledger level, no proofs, with anchor values in the `C_P` and `A_j` form (`CO/ATOMIC-CHUNKS.md:305-328`), then the frontend's chunked mode (R-phase, R-cut, R-ctx, the anchor statement of section 7.3) and the native anchor prologue of section 9.
4. stark-backend parameter code: an explicit per-round query vector in place of the derivation from `security_bits - query_pow`, with the keygen check of section 11.2; query PoW 21 for the app, leaf-aggregation and internal sets; the unique decoding sets of section 11.2 for the internal layer and `L` (`WhirProximityStrategy::UniqueDecoding` in every round, one OOD sample, folding PoW 0); the global interaction bound of section 11.2 in keygen of every layer; the `L` Merkle hash of section 3.6 (`u` = 1 and 2, Midnight's fixed-length sponge in place of the template's leaf hasher and compression) and the `L` transcript of section 3.6 (the multi-field duplex with Midnight's permutation and the initial capacity `2^64 + 3 * 2^66`), with the transcript vectors T-FS of section 13 on the CPU and GPU provers and the stark-backend verifier. The calculator needs no new term: its unique decoding fold and `mu` formulas match WHIR Corollary 4.11 and SWIRL Theorem 4.2.4 as stated (`PR/G5-PAPER-MAP.md` section 6), and the explicit query vectors are the input of `PR/UD-SIZING.md` section 1.1. Then D3 reruns on the frozen sets of section 11.2 at their shape bounds (`CT/RESULT.md:173,288`).
5. Lean: the restatement of section 11.4 on the committed round-2 files (`LEAN:1`).

**Phase 2: kernels and STARK path.**

6. The Fr Poseidon GPU kernel for `L`, starting with the one-file microbench D7 (`CT/RESULT.md:174,292`).
7. CPU wrap items of a day each: affine SRS, cached twiddles and parallel bit reversal, dynamic chunks, limb sort, multiopen fix (`CT/RESULT.md:175`).
8. sppark one-batch patch and window from the call size, then D9 (`CT/RESULT.md:176`).
9. Frontend M2a: base chip set leaf AIRs, planner B1 and C1, `key_root` under `prog_d`, byte-equal internal-recursive keys across two chip sets (`CO/FRONTEND.md:693`).
10. Atomic chunks T2 on the XC-9 devnet, with the native-routed controls T2-N and T2-N0 (section 7.3).

**Phase 3: the wrap.**

11. D10: `L` tracegen over one node proof for the trace heights, then `w_stack` and the high row case (`F/L-STRUCTURE.md:146`; `CT/RESULT.md:295`); the keyed structure of `L` is MEASURED (section 3.4). The bound is `w_stack` at most 48 with `mu` PoW 6; a tighter bound from D10 reruns D3 before the `L` key freezes (section 3.4).
12. M7: the G4 wrap circuit for the `L` set of section 3.4 (queries 98/83/82, one OOD sample), V+, the anchor statement checks, the DAG key check and the direct writer, then the cost model on the real `L` shape (`CT/RESULT.md:177`). Freeze k, `L_TX`, the capacity with the encoding of `pv`, the hash domains and the anchor template with the wrap key, and only after the security qualification gate of section 11.5 passes.
13. Batched MSM on the MSM skew mask of section 8, then the NVRTC numerator (`CT/RESULT.md:178`; `GW:11`).
14. Frontend M2b: one wrap proof that the ledger 9 verifier accepts with `2 + L_TX` public inputs (`CO/FRONTEND.md:694`).

**Phase 4: on chain and coverage.**

15. Path C (`CT/RESULT.md:179`) and the resident daemon with its slot pool (`CT/RESULT.md:180`).
16. Frontend M3 (local devnet), M4 (acceptance sets, forced overflow), M5 (Jubjub, GP-2, GP-3) (`CO/FRONTEND.md:695-697`).
17. E-1 T4 to T6 on the real wrap; atomic chunks T3 with universal-key chunks (`CO/ATOMIC-CHUNKS.md:354-358`).
18. Preview deploy and call of the wrap key.

---

## 11. Security statement

### 11.1 Target

The target: every layer on the chain of a call (leaf, OpenVM leaf layer, internal layers, `L`, wrap) has at least 100 bits of round-by-round soundness proven from theorems as stated, with no unproved conjecture.

Every STARK layer runs WHIR in unique decoding. Every term of every layer rests on results that are proven as stated, and the paper half of G5 is discharged for all layers by the unique decoding theorems that `PR/G5-PAPER-MAP.md` maps for the app and leaf-aggregation layers: WHIR Lemma 4.10, Corollary 4.11 and the unique decoding branch of Theorem 4.8 (folding), WHIR Theorem 5.2 with list size 1 (round-by-round composition, queries, `gamma`), WHIR Lemma 4.25 with `s = 1` (OOD), and the unique decoding branch of SWIRL Theorem 4.2.4 (`mu` batching) (`PR/G5-PAPER-MAP.md` sections 1, 2 and 6; `PR/UD-SIZING.md` section 1.2). No term uses an MCA bound beyond unique decoding or a list size bound.

Status: not qualified. The target is a gate, not a result. G1 to G4 of section 11.5 still gate the claim; until they pass, every bit count in this section is a calculator figure (MEASURED by D3, or ESTIMATE), not a proven bound, and no universal key freezes.

### 11.2 Frozen parameter sets and bounds

Each layer on the chain of a call has one parameter set and one set of shape bounds, frozen with the universal keys. A bit count holds only for a key whose shape is inside its bounds; keygen checks the bounds and refuses a key outside them. The stock sets and bounds are the stark-sdk constants (`SB/crates/stark-sdk/src/config/mod.rs:112-130,143-166,174-197`). Sherlock sets the query PoW of the app and leaf-aggregation sets (OD-4) and the internal and `L` sets (OD-19).

Every layer has one more shape bound: the total interaction count of one proof is below `p = 2^31 - 2^27 + 1`. LogUp-GKR Proposition 2 (ePrint 2023/1284 p.8) needs `char(F) > M 2^n`, and the characteristic of `EF = BabyBear^4` is `p`, whatever the size of `EF`. The code enforces it: `max_interaction_count = BabyBear::ORDER_U32` (`SB/crates/stark-sdk/src/config/log_up_params.rs:24`) is a keygen trace-height constraint (`SB/crates/stark-backend/src/keygen/mod.rs:175-178`). The check at the app layer's largest shape (stacked height 2^24, 100 AIRs, 1,000 interactions per AIR) is in section 13.

| Layer | Log blowup, `l_skip`, `n_stack`, `w_stack` | Proximity | PoW: folding, `mu`, query | OOD per round | Queries per round | Shape bounds |
|---|---|---|---|---|---|---|
| App (leaf shards) | 1, 4, at most 20 (stacked height at most 2^24; 2^17 to 2^18 on path C), 2,048 | unique decoding | 5, 15, 21 (stock 5, 15, 20), LEAD; LogUp 18 | 1 | 193/88/81/81, frozen | at most 5,000 constraints and 1,000 interactions per AIR, 100 AIRs, 30,000 columns; total interactions below `p` |
| OpenVM leaf layer (leaf aggregation) | 2, 4, 17, 2,048 | unique decoding | 4, 13, 21 (stock 4, 13, 20), LEAD | 1 | 118/84/81, frozen | height at most 2^21; 1,000 constraints and 100 interactions per AIR, 50 AIRs, 2,000 columns; total interactions below `p` |
| Internal (internal-recursive nodes; the leaf layer on path C) | 3, 2, 17, 512 | unique decoding | 0, 9, 21 (stock 18, 20, 20), OD-19; LogUp 18 (code rule) | 1 | 97/82/81, frozen, OD-19 | height at most 2^19; 1,000 constraints and 100 interactions per AIR, 50 AIRs, 2,000 columns; total interactions below `p` |
| `L` | 3, 2, 18, at most 48 (LEAD under OD-19) | unique decoding | 0, 6, 20, OD-19; LogUp 18 (code rule) | 1 | 98/83/82, frozen, OD-19 | height at most 2^20; 1,000 constraints and 100 interactions per AIR, 50 AIRs, 2,000 columns; total interactions below `p` |

Sources for the stock values: `CT/proposals/security-budget.md:74-76`; `CT/proposals/stark-path.md:29`. For the internal and `L` sets: `PR/UD-SIZING.md` sections 1.3 and 1.4; `UDS:351,535`.

**Internal and `L` constants (OD-19).** Both sets run `WhirProximityStrategy::UniqueDecoding` in every WHIR round, with one OOD sample, `k_whir` 4 and 3 WHIR rounds. The query vector is the smallest per-round count with a query term of at least 101 bits at the fixed query PoW (21 internal, 20 `L`); the folding PoW is then the smallest that holds 101 bits (0 for both); the `mu` PoW is then the smallest that holds 101 bits (9 internal at `w_stack` 512, 6 `L` at 48); the LogUp PoW is the code rule, 18, which gives 102.7205 bits (`PR/UD-SIZING.md` section 1.1; `UDS:331-351,515-535`). Internal grinding is Poseidon2 over BabyBear on the GPU: 6.6 x 10^6 permutations per proof, DERIVED (`PR/UD-SIZING.md` section 4). No Sherlock path in this proposal runs the hook set; a layer that runs it takes unique decoding first, as the internal set does.

**Query PoW 21 and frozen query counts (LEAD, OD-4).** With query PoW 20 the query terms of the app and leaf-aggregation sets are 100.09 and 100.01 bits, below the 101 bits that the margin rule asks of a grindable term. These two sets take query PoW 21 with their listed query counts unchanged; the internal set has its own vector at query PoW 21. The query term of WHIR round `i` is `-q_i log2(alpha'_i) + e(b_q)`, from these formulas:

- `alpha_i = (1 + rho_i)/2`, the largest agreement in unique decoding at the round's rate `rho_i` (`SB/crates/stark-backend/src/config.rs:254-274`); the rate falls by `2^-(k_whir - 1)` = `2^-3` per round (`SB/crates/stark-backend/src/config.rs:302-313`).
- `alpha'_i = alpha_i (1 - r/p) + min(alpha_i N, r)/p` charges the bias of `sample_bits` over a query domain of `N` points, with `r = p mod N`, which is 1 for BabyBear (`SB/crates/stark-backend/src/soundness/calculator.rs:630-647`).
- `e(b) = -log2((floor(p/2^b) + 1)/p)` is the effective value of a `b`-bit grind (`SB/crates/stark-backend/src/soundness/calculator.rs:649-660`): `e(20)` = 19.99925 and `e(21)` = 20.99850.

| Set | Round | Rate | Queries | Query domain | Before PoW | Query PoW 20 | Query PoW 21 |
|---|---:|---:|---:|---:|---:|---:|---:|
| App, stacked height 2^24 | 0 | 2^-1 | 193 | 2^21 | 80.10224 | 100.10149 | 101.10074 |
| | 1 | 2^-4 | 88 | 2^20 | 80.30327 | 100.30252 | 101.30177 |
| | 2 | 2^-7 | 81 | 2^19 | 80.09059 | 100.08984 | **101.08909** |
| | 3 | 2^-10 | 81 | 2^18 | 80.88594 | 100.88518 | 101.88443 |
| Leaf aggregation, height 2^21 | 0 | 2^-2 | 118 | 2^19 | 80.01248 | 100.01173 | **101.01098** |
| | 1 | 2^-5 | 84 | 2^18 | 80.27089 | 100.27014 | 101.26939 |
| | 2 | 2^-8 | 81 | 2^17 | 80.54441 | 100.54366 | 101.54291 |
| Internal, height 2^19 | 0 | 2^-3 | 97 | 2^18 | 80.5173 | 100.5166 | 101.5158 |
| | 1 | 2^-6 | 82 | 2^17 | 80.1658 | 100.1651 | **101.1643** |
| | 2 | 2^-9 | 81 | 2^16 | 80.7720 | 100.7713 | 101.7705 |

Mark: app and leaf-aggregation rows ESTIMATE, the calculator formulas above evaluated outside the calculator; their minima at PoW 20 equal the weakest terms of `CT/proposals/security-budget.md:74-75`, and D3 prints their query PoW 21 column as 101.1007/101.3018/101.0891/101.8844 and 101.0110/101.2694/101.5429, MEASURED (`D3:126`). Internal rows: the query PoW 21 column is MEASURED (`UDS:531-533`); the other two columns are DERIVED from it by `e(21)` and `e(20)`. D3 prints the `L` query terms at query PoW 20 as 101.3466 / 101.1427 / 101.7684 bits, MEASURED (`UDS:347-349`). WHIR Theorem 4.8 admits the unique decoding radius `(1 - rho)/2` directly, so no radius correction applies to any unique decoding query term (`PR/G5-PAPER-MAP.md` section 1). The app set at a smaller stacked height, such as 2^17 to 2^18 on path C, runs the leading rounds of the same vector, and D3 runs at every admitted height. D3 prints the other grindable terms of the app and leaf-aggregation sets: LogUp 102.7205, `mu` 102.6282 and 102.6283, folding 104.6276 and 105.6276 bits, so the query term is the weakest term of each set, MEASURED (`D3:109-118`). The added query bit is one more doubling of the query grind, Poseidon2 over BabyBear on the GPU.

**Query counts are key constants.** `WhirConfig::new` derives each round's count as `ceil((security_bits - query_pow) / per-query bits)` (`SB/crates/stark-backend/src/config.rs:284-313,345-352`), and the stark-sdk sets pass `security_bits` = 100 (`SB/crates/stark-sdk/src/config/mod.rs:32,101`). That rule regenerates 191/87/80/80, 117/83/80 and 96/81/80 for the app, leaf-aggregation and internal sets at query PoW 21, and 97/82/81 for `L` at query PoW 20, ESTIMATE by the same formulas; the weakest query terms then fall to 100.10, 100.31, 100.19 and about 100.17 bits. Sherlock therefore freezes every set's per-round query vector as an explicit constant of its parameter set: the parameter code of build step 4 takes the vector as an input in place of the derivation, keygen checks that the key's vector equals the frozen one and refuses a key otherwise, and D3 runs on the frozen vectors. A derivation target of 80 protocol bits reproduces every listed count of the app, leaf-aggregation and internal sets, and for `L` it gives 97/82/81 against the frozen 98/83/82, so the explicit vector, not a changed target, is the rule.

**Margin rule (OD-4).** Every term of every layer is at least 100 bits. Grindable terms (folding, `mu`, query and LogUp) take at least 1 bit of margin, set by D3 through their PoW constants. Terms that no grinding moves (out-of-domain, `gamma` batching, sumcheck, stacked reduction, zerocheck, constraint batching, `pv` binding) need 100 bits and no margin. With one OOD sample per round and list size 1, the OOD terms are 108.6276 bits for `L` and 109.6276 bits for the internal set, MEASURED (`UDS:347,531`).

| Layer | Bound | Mark and source |
|---|---|---|
| `L`, section 3.4 set | weakest term 101.0730 bits (`mu`) at `w_stack` 48 and 101.1427 bits (query, round 1) at 36; at 16 the run with `mu` PoW 5 prints 101.1427, so `mu` PoW 6 holds at least that, DERIVED; every grindable term at least 101 bits, every term at least 100 | MEASURED, `UDS:167,259,331-351` |
| `L`, out-of-domain term | one sample per round, list size 1 (WHIR Lemma 4.25 with `s = 1`): 108.6276 bits in round 0, 112.6276 in round 1 | MEASURED, `UDS:347-348` |
| Internal layers | unique decoding; weakest term 101.1643 bits (query, round 1); OOD 109.6276 bits; `mu` 101.6304; every grindable term at least 101 bits, every term at least 100 | MEASURED, `UDS:515-535` |
| App and leaf aggregation | query terms 101.0891 and 101.0110 bits at query PoW 21 with the frozen query vectors under unique decoding (WHIR Corollary 4.11, Theorem 4.8 unique decoding branch, Theorem 5.2); every other term above them | MEASURED, `D3:13-14,105-124`; `PR/G5-PAPER-MAP.md` sections 1 and 2 |
| LogUp | SWIRL Theorem 4.2.1 with the characteristic hypothesis: total interactions of one proof below `p`, a keygen shape bound of every layer | `SB/crates/stark-sdk/src/config/log_up_params.rs:24`; LogUp-GKR Proposition 2, ePrint 2023/1284 p.8 |
| Fiat-Shamir | SWIRL Cor 4.2.6: `Q_RO x eps_rbr + 3(Q_RO^2 + 1)/2^kappa`; squeeze and query-index bias move no term; the collision term never sets the bit count of a layer | `CT/RESULT.md:252`; DERIVED, `D3:137-148` |
| Poseidon over Fr | Midnight's width 3 has 60 partial rounds against the paper's 57 for `x^5`; collision security from one 255-bit capacity element; domains of section 3.6 | `PR/REVIEW.md:111` |
| E-1 label binding | collision of SHA-256 truncated to 248 bits, about 2^124 | `CO/E1-entry-label.md:194` |

### 11.3 Premises

The bridge theorem rests on these named premises at commit `bdb92973a` (`LEAN:46-56`; `B/CANDIDATE.md:456`):

| Name | Statement |
|---|---|
| A1 `wrapKnowledgeSound` | if ledger 9 accepts a proof for `pub`, a wrap witness exists (PLONK with KZG, knowledge sound in the AGM with a q-type assumption) |
| A2 `lastLayerSound` | if the reference verifier of `L` accepts, the root AIR of `L` accepted with some log digest; WHIR round-by-round soundness (WHIR Theorem 5.2 with list size 1) with mutual correlated agreement in unique decoding (WHIR Lemma 4.10, Corollary 4.11, Theorem 4.8 unique decoding branch), one OOD sample per round (WHIR Lemma 4.25, `s = 1`), `mu` batching (SWIRL Theorem 4.2.4, unique decoding branch), LogUp (SWIRL Theorem 4.2.1, with total interactions below `p`), the composition of SWIRL Theorem 4.2.5, Fiat-Shamir in the random-oracle model (SWIRL Corollary 4.2.6), the `eq` binding error, the squeeze bias |
| A2b `frSpongeInj` | Midnight Poseidon over Fr has no collision on its carrier domain |
| A3 `rootBinds` | the root AIR binds its public values to a carrier log tree of at most `B` entries, padded with `NOP` |
| A4 `recSound` | the recursion proves the run against the flattened log; each Fr operation is in the log or proved inline |
| A5 `p2Inj` | Poseidon2 over BabyBear has no collision on its carrier domain, separate from A2b |
| A6 `oracleElim` | a run whose Fr operations are all valid is a run of the reference machine |
| A7 `wrapZK` | the blinded wrap is zero knowledge; used by `wrap_view_only_claim` |
| static `vkL_ok` | the transcript program of the frozen key absorbs `pv` before its first challenge (`pvFirstT`), the key is in `ProvenRegime` with its PoW bits read from the transcript, and every compiled expression has a tracked bound below `r`; decidable on the concrete key |

Every theorem inside A2 is read in full and mapped to the frozen sets: WHIR (ACFY24, ePrint 2024/1586), BCGM26 (ePrint 2025/2051) and SWIRL (`PR/G5-PAPER-MAP.md`). In unique decoding the list size is 1, so no list size bound and no MCA bound beyond unique decoding enters A2; BCHKS25 is not a premise. LogUp-GKR (ePrint 2023/1284) is an informal note that proves nothing, so A2 cites SWIRL Theorem 4.2.1; that theorem's text does not state the characteristic condition, and the mapping confirms it in the OpenVM LogUp note [Ope25a, Theorem 3.4] that SWIRL cites (section 13). The bounded-quotient gadget is covered by no paper and rests on the Lean lemmas (`PR/REVIEW.md:107`). All 22 papers of the paper review are read in full and WHIR and BCGM26 are on disk; SuperNeo (ePrint 2026/242) is in the set (`PR/REVIEW-PASS2.md`; `PR/PAPERS-ANALYSIS.html`).

### 11.4 Lean status

**Committed, `bdb92973a`.**

- BUILD PASS: `bridge_sound` and `wrap_view_only_claim` build (`lake build`, 31 jobs) with 0 sorry, no `admit`, no `native_decide`, no `axiom` declarations; `#print axioms` over all 112 theorems gives only `propext`, `Classical.choice` and `Quot.sound` (`LEAN:1,129-134`; `B/CANDIDATE.md:456`).
- Proved on the path: the bounded-quotient encoding, canonical logged values, canonical squeeze digits, packing with its length and range checks, `pv` before the first challenge, the fixed log capacity and fixed `pv` length with overflow to STARK shards, and the empty onion as a hash output. Negative lemmas show each check is needed, for example `unbounded_quotient_unsound` and `squeeze_alias` (`LEAN:68-80`).
- The transcript of `L` is a program in the key that `VerifyL` runs, so `pvFirstT` and the PoW part of `ProvenRegime` are derived from what the verifier executes. Only the WHIR analysis parameters (`logBlowup`, `eta`, `mult`) stay key metadata, and the link from the transcript program to the real stark-backend verifier of `L` stays inside A2 (`LEAN:85-86`).
- The model applies the lead amendments `d_0 = P2(TAG_EMPTY)` and a fixed `pv` length per key (`LEAN:106-107`; `B/CANDIDATE.md:458-461`).
- Lemma 23 (`pv_binding_count`) is proved over the prime field `Z/q` (BabyBear). The binding challenge of `L` lies in `EF`, and the bound enters `bridge_sound` only through A2 (`LEAN:88,105`).
- `ProvenRegime` as committed takes the list decoding regime: m as Theorem 1.5 sets it and a weakest folding, `mu` and query PoW of at least 21, 21 and 20; the example key is log blowup 4, `eta = 1/8`, m = 3 (`LEAN:77`).
- `BridgeAxioms.consistent` is a model of all premises, so they are jointly satisfiable (`LEAN:60`).
- Limits: the wrap gadget in the model is unsigned `ArithGadget`; a signed lazy reduction needs `ReduceRel` in its place (`LEAN:84`).

**Restatement.** The section 3.4 set (unique decoding, log blowup 3, folding PoW 0, `mu` PoW 6, query PoW 20, one OOD sample) does not pass `ProvenRegime` as committed, which takes the list decoding regime (`LEAN:77,139`). The restated Lean statement:

- `ProvenRegime` takes the unique decoding regime with the frozen per-round query vector, the folding, `mu`, query and LogUp PoW constants, one OOD sample per round, and the `w_stack` bound of the key;
- `encode` follows the two-table layout of `pv` with the running-sum offsets and the padding rule of section 3.2, in place of the one zero-padded table of `LEAN:100,107`;
- `wrap_refines` gains the DAG key check of section 4.3 and the anchor statement checks of section 7.3;
- the Fr side names its two algorithms on the one permutation: `frSpongeInj` (A2b) covers Midnight's fixed-length sponge under the capacity codes `u` = 1, 2, 4, and the overwrite duplex of `u` = 3 is the transcript that the Fiat-Shamir part of A2 takes as its random oracle, with the key schedule `sched` as the fixed absorb lengths of `LEAN:116`.

The proximity-gap theorems stay inside A2 and the recursion stays premise A4 (`LEAN:49,52`), so a passing build of the restated theorem shows the parameter gate and the bridge, not the bit counts; D3 prints those (section 11.2). The Lean proof of the list decoding MCA bound on branch `mca-lean` is separate work and is not on the path of this gate.

### 11.5 Security qualification gate

Status: not passed. D3 passes the calculator part of G2 for the unique decoding sets of section 11.2 (`UDS:259,351,535`; `L` at `w_stack` 16 with `mu` PoW 5, `UDS:167`) and for the app and leaf-aggregation sets (`D3:13-14`). G5 is discharged at the paper level for every layer. G1, the rest of G2, G3 and G4 are open and gate the claim. The design claims proven 100-bit security only when G1 to G4 pass. Until then this section, not the hard-rule line, states the security status, and no universal key freezes. The checks are one gate: a failed check blocks the claim, whichever it is.

| # | Check | Pass condition |
|---|---|---|
| G1 | Parameters frozen | the owner confirms OD-4 (app and leaf-aggregation query PoW, margin rule), OD-17 and OD-18, with the internal and `L` sets of OD-19; every set and bound of section 11.2, with the explicit per-round query vectors, the global interaction bound and query PoW 21 for the app, leaf-aggregation and internal sets, is fixed in the key-generation code, and keygen refuses a key whose query vector differs |
| G2 | D3 on the frozen sets | the stock calculator, after build step 4, prints every term of every layer at its shape bounds at 100 bits or more and every grindable term at 101 or more, on the frozen query vectors and not on counts regenerated from `100 - query_pow`; for `L` at the `w_stack` that D10 measured. Calculator part: passed for the unique decoding internal set and for `L` at `w_stack` 36 and 48 with `mu` PoW 6 and at 16 with `mu` PoW 5 (`UDS:167,259,351,535`), and for the app and leaf-aggregation sets (`D3:9-16`). Open: the frozen sets of G1, the `w_stack` of D10, and the parameter code in the prover and verifier (`D3:26`) |
| G3 | Transcript | the prover, the stark-backend verifier, the calculator and the wrap's verifier of `L` follow the stock one-sample WHIR order of section 3.4 and the transcript algorithm of section 3.6; the transcript vectors T-FS of section 13 give equal outputs on the CPU prover, the GPU prover, the stark-backend verifier and the wrap's verifier of `L` |
| G4 | Lean | the restated theorem of section 11.4 is committed with a build log: 0 sorry, standard axioms only, and the concrete `L` key passes `ProvenRegime` by `decide` |
| G5 | Theorem-to-configuration mapping | discharged at the paper level for every layer: each hypothesis of WHIR Theorem 5.2 holds for each frozen set in unique decoding (the code, rate and radius `(1 - rho)/2` of every round, list size 1, the field size of `EF`, the number of stacked columns that `mu` batches, one OOD sample and the `gamma` batching of `q_i + 1` claims) by WHIR Lemma 4.10, Corollary 4.11, the unique decoding branch of Theorem 4.8 and Lemma 4.25, and SWIRL Theorem 4.2.4 unique decoding branch; the calculator's unique decoding formulas match these statements (`PR/G5-PAPER-MAP.md` sections 1, 2 and 6; `PR/UD-SIZING.md` section 1.2). The LogUp characteristic hypothesis is part of A2 and of the G1 shape bounds; its citation check in [Ope25a, Theorem 3.4] is in section 13. The conjecture-based capacity regime of the WHIR reference code (`WH/src/protocols/params/regime.rs:6-9`) is excluded |

---

## 12. Open owner decisions

Each item has one recommended option. Items 1 to 13 are the batch of `CT/RESULT.md:266-278`.

| # | Decision | Options | Recommended |
|---|---|---|---|
| OD-1 | OOD closure for the internal and `L` layers | superseded by OD-19 for both layers: in unique decoding the list size is 1, and one OOD sample per round gives 108.6276 bits (`L`) and 109.6276 bits (internal), MEASURED (`UDS:347,531`) | **closed by OD-19** |
| OD-2 | Wording of the `L` PoW rule | superseded by OD-19: folding PoW 0 on every fold, `mu` PoW 6, query PoW 20, LogUp PoW 18 (section 3.4) | **closed by OD-19** |
| OD-3 | `L` parameter set | superseded by OD-19: the unique decoding set of section 3.4, log blowup 3, queries 98/83/82, `w_stack` at most 48 (LEAD shape bound; D10 may tighten it) | **closed by OD-19** |
| OD-4 | Proven rule on inner layers, and the margin rule | (a) every layer, every term at least 100 bits, 1 bit of margin only on grindable terms, query PoW 21 on the app and leaf-aggregation sets with their query vectors frozen as listed in section 11.2 | **(a)**: query PoW 20 leaves the app and leaf-aggregation query terms at 100.09 and 100.01 bits, below the grindable margin (section 11.2); the internal parts of this item (radius, PoW, LogUp PoW, OOD count) are superseded by OD-19 |
| OD-5 | E-1 placement | (a) V with the V+ layout word; (b) S; (c) R | **(a)**: 0 wrap rows, rebuilds rare (`CT/RESULT.md:138-147`) |
| OD-6 | Log capacity in the wrap key | (a) `V = P = 256`, `E = 0`, Jubjub inline, spill to tier E; (b) 1,024 / 1,024 / 16 | **(a)**: the largest checked contract needs 111 / 39 / 0 with the mandatory `DIGEST` entries counted; GP-1 holds by the spill rule for discretionary operations, after the reservation `R` of section 3.2 (`CT/RESULT.md:271`) |
| OD-7 | `L_TX` and the key menu | (a) one value, 1,024, frozen with the wrap key after M7; (b) a frozen menu of (`L`, wrap) key pairs, such as {128, 1,024}; (c) a second key with a longer vector | **(a)**: one key set; chunked calls cover longer transcripts (`CT/RESULT.md:272`; `CO/ATOMIC-CHUNKS.md:238`; `CO/FRONTEND.md:507-509`) |
| OD-8 | Hybrid native and Sherlock contracts | (a) route small entries native at compile time, GP-1 to GP-4 available to every entry, every native entry with the anchor prologue of section 9; (b) Sherlock only | **(a)** (`CT/RESULT.md:273`), gated on the T2-N control of section 7.3 |
| OD-9 | Elastic trust domain, and salted grinding or masked MSM on helpers | (a) local devices and user-owned GPUs count as the user's machine; no untrusted helpers; (b) also masked-MSM and salted challenge-only grinding helpers | **(a)**: on the full GPU path the helper gain is below 2.5 s; (b) changes the PoW predicate to `H2(g, w)` and needs a new calculator run (`CT/RESULT.md:254,274`) |
| OD-10 | Fallback if the Fr Poseidon kernel slips | (a) wait for it; (b) Poseidon2-BabyBear grinding with its added random-oracle premise | **(a)**; (b) only on a measured slip (`CT/RESULT.md:210,275`) |
| OD-11 | Second Fr proving engine (fr-terminal form A) | (a) no; (b) build as a hedge | **(a)**; reopen if D9 or the numerator kernel fails (`CT/RESULT.md:205,276`) |
| OD-12 | User batches and the batched entry | (a) A-GP-root, one batch digest per `acceptRun`, opt-in; (b) also the fused batched entry `sherlock$batch` (A-C) | **(a)**: A-C links N calls to one party and fuses their fallible parts (`CT/RESULT.md:277`; `CT/proposals/amortize.md:47,165`) |
| OD-13 | Resident prover daemon | (a) required, local socket, one-use slots; (b) per-call processes | **(a)**: a cold call adds 19 to 98 s (`CT/RESULT.md:278`) |
| OD-14 | Keep Compact `Field` emulated | (a) yes: FIELD-EMU in the leaf, or a log entry, as section 7.1; (b) no | **(a)**: FMUL is measured at one row of 194 columns and 9.126 s per 2^20 on the GPU, and every checked contract fits the recommended log capacity `V = P = 256`, the largest with a margin of 2.31 times on values and 6.56 times on permutations (`CO/FRONTEND.md:80-84`; `CT/proposals/wrap-rows.md:144`; `LOOP-AGENDA.md:24`) |
| OD-15 | Call-vector layout | (a) one layout of `2 + L_TX` words for Compact and GP runs; (b) keep the 70-word run statement, with a second wrap key for it | **(a)**: one wrap key; (b) breaks one key set (`CO/FRONTEND.md:212-215`) |
| OD-16 | Anchor lock and native prologue (LEAD) | (a) `A_0 = A_k = C_P`, salted locks `A_j` under `SHK_LOCK` and `DOM_LOCK`, the anchor statement of section 7.3 in the one wrap, and for hybrid contracts the `anchor_home` cell with the native prologue of section 9; (b) as (a), but native keys made at deploy with `C_P` as a circuit constant | **(a)**: no plain `prog_d` on the ledger, no per-deployment native keygen, one wrap key (`CO/FRONTEND.md:220`; `CO/ATOMIC-CHUNKS.md:110,126`) |
| OD-17 | Encoding of `pv` and the entry capacity (LEAD) | (a) `B = V = 256`, entry table of 2 words and value table of 9 words, `W_pv = 2,816`, per-operation bounds and padding of section 3.2, the mandatory reservation `R` (anchor and public-digest `DIGEST` entries) placed before discretionary operations, the simultaneous capacity rule over `R` and `S`; (b) a fixed slot of `l_e` words per entry (`B/CANDIDATE.md:81`) | **(a)**: variable arity without a widest-entry slot, and the entry limit never binds before `V` |
| OD-18 | Hash domains and transcript algorithm (LEAD) | (a) the capacity use codes `u` on the Fr side and the tag list on the BabyBear side of section 3.6; Midnight's sponge as native for `u` = 0 and its fixed-length mode for `u` = 1, 2, 4, in place of the template's Merkle hash; for the `L` transcript the stark-backend multi-field overwrite duplex on Midnight's permutation, stock in every transition, with the initial capacity `2^64 + 3 * 2^66` as the one change; (b) tags absorbed as Fr inputs | **(a)**: no added permutation in `L` and no wrap row; native program hashes keep Midnight's own initialization. It changes the public digest form `PoseidonFr(tag, pack(d2))` of `B/CANDIDATE.md:106,220` to the capacity code `u` = 4, so a tool or a native Compact circuit that recomputes `C_P` or another public digest from a manifest uses the Sherlock form, not Midnight's plain two-input hash |
| OD-19 | Proven bound for the internal and `L` layers | (a) unique decoding: `L` at log blowup 3, queries 98/83/82, folding PoW 0, `mu` PoW 6, query PoW 20, LogUp PoW 18, one OOD sample, weakest term 101.0730 bits; internal at log blowup 3, queries 97/82/81, folding PoW 0, `mu` PoW 9, query PoW 21, LogUp PoW 18, one OOD sample, weakest term 101.1643 bits (MEASURED, `UDS:351,535`); wrap at k = 20 on the G4 model row and k = 21 on the worst row (section 3.5) | **owner decision, 2026-11-11: (a)**. The written proof and the Lean proof of the list decoding MCA bound continue (`research/sherlock-mca-proof-20261111/`, branch `mca-lean`); once both exist, list decoding can return for the `L` layer to reach k = 19. OD-19 supersedes OD-1, OD-2, OD-3 and the internal part of OD-4 (`DECISIONS-ASK.md` row OD-19; `PR/UD-SIZING.md`) |
| OD-20 | List size `D_Y` | withdrawn: unique decoding uses no list size bound | **closed by OD-19** |
| OD-21 | Ledger-readable wrap path (M7-0 BLOCKER) | (A) G4 architecture in `midnight-zk-stdlib` and a ledger release; (C) G4 off chain, then a stdlib decider; (E) an Fr-terminal layer before a stdlib wrap. (B) and (D) rejected by M7 | **OWNER**, no default (section 9) |
| F-1 to F-13 | Interface freeze prerequisites (LEAD) | the choices of `FD/ARCHITECTURE.md` section 1.11 and `FD/DECISIONS.md`: word types and bus registry (F-1, F-2), node statement, DISCHARGE mode and key routing (F-3 to F-5), `L` opens the SET head, `C_P`, discriminator and index table of `L` (F-6 to F-8), tag codes, constants and one salt per run (F-9, F-10), `view_root` header word (F-11), `n_steps` not public (F-12), claim address, `ep_hash` and varlen window under placement V (F-13) | **confirm as written**; F-7 is the form of N-11, F-11 keeps AD-17 open, F-12 needs the amendments of SRC-006, SRC-007 and LDG-001 |

---

## 13. Pending evidence

| Item | What is missing | Source |
|---|---|---|
| Bench write-up | every Plonky3 cell, remaining OpenVM CPU cells, CPU thread scaling, CPU nodes; ranking by seconds per proved leaf step | `F/bench/RESULTS.md:36-70`; `LOOP-AGENDA.md:10` |
| E-1 tests | the remaining T2 mutants and the `label_d` link; T3 to T6; T0 on calls with a fallible transcript or non-empty effects; `bind_len_select` and `ep_hash_vs_ledger` (the vectors of the FP-7 form); D4 | `E1:64,129-130`; `CO/E1-entry-label.md:283-295` |
| Atomic chunks | T1 and T2 results in the `atomic-chunks` worktree, with the native-routed controls T2-N and T2-N0; T3 after M5; mutants `chunk_claim_addr_free` and `chunk_ep_hash_other_label` of the FP-7 form | `CO/ATOMIC-CHUNKS.md:120-127,272,305-358`; `LOOP-AGENDA.md:21` |
| GPU wrap | a quiet-host repeat at k = 19 (proxy), 20 and 21 with at least 3 repetitions (the loaded table is MEASURED, `GW` section 1); the fused quotient kernel and the MSM rewrite that the path 2 and 3 wrap times assume; idle MSM (D2, D9); the G4 wrap on the GPU | `GW:3,5`; `CT/RESULT.md:101-102` |
| G4 wrap rows | M7: the real wrap circuit and cost model; the k of the key | `CT/RESULT.md:300` |
| `L` heights | D10: the trace heights of `L` by tracegen over one node proof, then `T`, the distinct heights and `w_stack`, which fixes the `w_stack` bound and the `mu` PoW of section 3.4; keygen on the BN254 root configuration; the log add-on AIRs | `F/L-STRUCTURE.md:7,141-149`; `CT/RESULT.md:295` |
| Path C child verifier | the rows and the call time of the path C child verifier inside `L`: an internal proof at the section 11.2 set carries 260 queries (97/82/81), so `L`'s trace area and BabyBear work move toward the 283-query leaf-parameter child that keygen of `L` measured; neither the wrap row model nor the call-time model has it | `PR/UD-SIZING.md` sections 1.4, 3.3 and 4; `F/L-STRUCTURE.md` section 5 |
| Path C child key | the code check of section 4.3: whether the stock recursion binds the child `h_pre` on path C, and the arity of the path C leaf-layer key before D12 | OpenVM whitepaper pp.17, 20, 25; section 4.3 |
| Soundness calculator | D3 again on the frozen sets of G1, `L` at the `w_stack` of D10; the parameter code in the prover and verifier | `D3:26`; `UDS` |
| Lean | the restatement of section 11.4 for the unique decoding sets | `LEAN:136-139`; section 11.4 |
| Security qualification | checks G1, G3 and G4, and the rest of G2 | section 11.5 |
| LogUp hypothesis | confirm the characteristic condition `char(F) > M 2^n` in the OpenVM LogUp note [Ope25a, Theorem 3.4] that SWIRL Theorem 4.2.1 cites, and check the total interaction count below `p` at the app layer's largest shape (stacked height 2^24, 100 AIRs, 1,000 interactions per AIR) | section 11.2; `PR/PAPERS-ANALYSIS.html` finding 3 |
| Transcript vectors T-FS | one fixed vector per transition of the `L` transcript of section 3.6: the key pre-hash then one sample; 1 to 8 words observed then a sample (each partial pack); 16 words then a sample (no added permutation); 9 words then a commitment (flush, then the digest); 11 samples in a row (digit refill at the 6th, a permutation at `sample_idx` 0 at the 11th); sample, observe, sample (dropped digits); one non-final WHIR round of `L` with the OOD point, its answer, the query PoW, the query indices and `gamma`; and the full transcript of one `L` proof from D10. Each vector gives the same digits and the same final state (`SB/crates/stark-backend/src/transcript/multi_field.rs:82-100`) on the CPU prover, the GPU prover, the stark-backend verifier and the wrap's verifier of `L` under the mock prover; and one vector each for `u` = 1, 2, 4 against Midnight's CPU sponge with the Sherlock capacity | section 3.6; G3 |
| Fr Poseidon kernel rate | D7: 5 to 15 M permutations per second | `CT/RESULT.md:292` |
| Preprocessing savings | cached trace on and off; planned against generic shard (M13) | `SP/RESEARCH.md:274-275`; `B/CANDIDATE.md:204` |
| GP-3 cost | foreign-proof shards (M11) | `B/CANDIDATE.md:173` |
| Wrap on chain | verifier-key bytes against 50,000, proof bytes, verify time and fee at `2 + L_TX` public inputs; devnet and Preview deploy of the wrap key | section 9 |
