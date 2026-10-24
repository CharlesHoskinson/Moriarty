# astra-2-5: Direct terminal and fan-in budget

## Question
After astra-2-2: audit removing the parent layer. A new terminal directly prepares the actual Sherlock A102 and A80 children, preserves the 16 statement fields, and discharges all KZG obligations. Compare node + decider rows with a fused verifier+pairing circuit. Find measured pairing-only rows. Give fan-in 2/4/8 breakpoints using historical rows as a labeled proxy. Search the mono16 keyed fallback for row/timing evidence and the privacy caveat. Give code hooks (path:line), changed keys, and a threshold formula. No repo writes.

Path roots used: RL = ~/repos/projectfenrir/recursion/stagenet-strawman/recl9; SOL3 = .../stagenet-strawman/brainstorm-l9/sol-3; OPUS4 = .../brainstorm-l9/opus-4; MONO = .../stagenet-strawman/mono; PG = ~/repos/projectfenrir/recursion/pairing-gadget/lanes/opus; MORI = ~/repos/Moriarty-factory.

## Answer

- **Direct terminal is mostly a re-wiring of code that already exists.** The parent's two-child block is `RL/src/sherlock_link.rs:476-493`: two `verifier.prepare` calls under the A102 and A80 pinned VKs (:485-489), `accumulate` (:491), `resolve_fixed_bases` (:492), `collapse` (:493). The 16-field trace and statement are at :495-523, and the accumulator is exposed at :525. The decider's verification of its child is `RL/src/decider.rs:148-153` (`verify_child_incircuit`), with the public-encoding equalities at :178-184 and the residue pairing at :191. The Moriarty spec (`MORI/docs/sherlock/reference/recursive-child-parent-terminal.md`) lists the same 16 fields and the 12-field accumulator tail. The terminal can reuse the :476-525 block and the :148-191 tail, with the same statement digest (`SHK_STMT`, 19-field Poseidon).
- **Pairing-only measured rows exist: 316,833 rows** (k=19, rc=4, default with A and B subgroup-checked; includes assigning A and B and their public encoding). 299,573 rows without the G1 subgroup checks (`PG/lanes/opus/meas/strict.txt:2,4`; `PG/lanes/opus/RESULTS.md:17`). Real proof for the default variant: prove 301 s (contended, load ~28 on 6 cores), 6,432 B, verify 25 ms, peak RSS 7.6 GB (`RESULTS.md`, real-proof section). The earlier run at 914 s was under heavier contention. The general 2-pair check is 443,229 rows (model only).
- **Node + decider vs fused.** Current route, measured: node arity 4 = 957,724 rows (interim crate) or 958,095 (certified), k=20 (`RL/REPORT.md` §3 table; `RL/logs/cost-node-arities.log`); decider = 841,682 rows, k=20 (`RL/logs/decider-keygen.log`). Sum = 1,799,406 rows across two proofs. Fused verifier+pairing, one proof, b children: `R_T(b) ≈ 190,356 + 191,842·b + 316,833 + ε`, where ε is small (about 200 rows from the stand-in decider, `SOL3/IDEAS.md` "What one verifier actually costs"; 8,000 rows segment allowance and 3,840 per image on the soundness side, `SOL3/IDEAS.md` joint-aggregation section). The intercept and slope come from the node sweep: b=2 574,040, b=3 765,907, b=4 957,724 (`RL/REPORT.md` §3). Subtracting the decider and node terms double-counts the parent-verifier block, so these totals cannot be summed directly.
- **Fan-in breakpoints (rows vs usable rows; usable = 2^k − 7; k20 = 1,048,569, k21 = 2,097,145; `RL/REPORT.md` §3).** Labeled as a model from historical rows, not a measured fused circuit:

| fan-in b | model rows, no allowances | with 8,000 + 3,840·b allowances | k needed | headroom at that k |
|---|---:|---:|---:|---:|
| 1 | 699,031 | ~710,900 | 20 | ~338k |
| 2 | 890,873 | ~906,600 | 20 | ~142k |
| 3 | 1,082,715 | ~1,102,200 | 21 | ~995k |
| 4 | 1,274,557 | ~1,297,900 | 21 | ~799k |
| 8 | 2,041,925 | ~2,080,600 | 21 (edge) | ~16k |
| 9 | 2,233,767 | ~2,276,300 | 22 | no authenticated k22 SRS reported |

  Threshold: fused k20 iff `190,356 + 191,842·b + 316,833 + ε ≤ 1,048,569`, i.e. b ≤ 2 (`b ≤ 2.82` before ε). k21 iff b ≤ 8 with allowances (`b ≤ 8.08`). Table rows: k21 table is 2,080,785 (`SOL3/meas/native4-collapse-k21.json`), leaving 16,360 rows of headroom. So fan-in 8 at k21 is at the edge; fan-in 4 at k21 is the practical choice.
- **Sherlock A102/A80 (fixed b=2), updated with actual child sizes.** The actual RUN-REPORT (`~/repos/Moriarty-factory/docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json`, fields `children[0].circuit_k`=13, `children[1].circuit_k`=12, `proof_bytes`=3440 each, `instance_lanes`=253) shows the children are small. The parent's verifier cost is set by the verifier shape, not the child domain. The closest measured verifier proxy is the sol-3 native one-child / four-child synthesis at child_domain_k=13 (`SOL3/meas/native1-collapse-k21.json`: 363,626 rows; `native4-collapse-k21.json`: 948,383 rows). Fitting that gives a per-child slope of 194,919 and an intercept of 168,707. So a direct two-child verifier is ~558,545 rows, plus the pairing 316,833, for ~875,400 rows. That fits k20 (usable 1,048,569), not k19 (524,281). This is a proxy, not a measurement: the native key has 33 transcript points, and the Sherlock child has a different shape (253 instance lanes, 3,440 B proofs).
  The k14–15 target in astra-2's message is a different route. It applies only if the two child relations are inlined as native constraints into one circuit, with no in-circuit verification of child proofs. That route needs the same-witness custody caveat and is not measured. The verifier-based direct terminal stays at k20.
  With the measured sizes, the parent (k20, 882.6 s prove) is already far larger than its children need. Replacing parent+decider with one k20 verifier-based terminal (~0.88M rows) saves one decider-sized proof and its keygen. Prove-time saving is still modeled, not measured (see next bullet).
- **Time, measured vs modeled.** Measured current route: node prove 867.3 s (pk 104.5 s; peak 14.99 GB) and decider prove 586.4 s (pk 46.9 s; peak 14.97 GB) (`RL/REPORT.md` §3). Standin decider at k20 under contention: prove 1,182.8 s, peak 14.9 GB (`stagenet-strawman/decider/meas/run_standin_k20.log`). The Sherlock fixed parent: prove 882.589555 s + setup_pk 62.328385 s, child verifies 0.019 s each, fold 0.010638 s, peak ~14.6 GiB, 6,432 B (`MORI/wiki/research/sherlock-circuit-atlas/UPDATE-028-…md`, "Revision02" table). A direct k21 fan-in-4 proof is not measured. Modeled 2× domain scaling (sol-3's own convention) gives ~2,480 s for a k21 decider-sized proof and ~29.8 GB (`SOL3/IDEAS.md` machine section). Linear scaling in rows gives ~1,150 s for the fan-in-4 fused proof. So the honest range is roughly 1,150–1,820 s for fan-in-4 fused vs 1,454 s measured for node+decider prove time. The fused route is not clearly faster in prove time at fan-in 4. Its real saving is one fewer proof, one fewer keygen (decider vk 412–961 s and pk 47–105 s), one fewer verify, and proof size.
- **Sherlock b=2 at k20 (more favorable).** It replaces the parent (883 s measured) and decider (586–1,183 s measured) with one k20 proof of ~0.88M rows (proxy). If the parent's row count is ~0.57M (node b=2-like), the fused proof scales to about 1,370 s modeled (linear in rows). That is a saving of about 7–34% of prove time against 1,469–2,066 s, plus one fewer keygen. The parent's real row count is unmeasured; `cost_model` at `RL/src/sherlock_link.rs:971,1029,1100` would give it.

## Code hooks

| Purpose | Path:line |
|---|---|
| Parent: A102/A80 prepare, accumulate, resolve, collapse | `RL/src/sherlock_link.rs:476-493` |
| Parent: trace Poseidon and 16 public fields | `RL/src/sherlock_link.rs:495-523` |
| Parent: expose accumulator | `RL/src/sherlock_link.rs:525` |
| Off-circuit statement and accumulator (reference) | `RL/src/sherlock_link.rs:231,389,407-420` |
| Parent keygen/prove (cost_model k) | `RL/src/sherlock_link.rs:968-1030` |
| Decider relation (n_w_instance=28, nr_cols=4) | `RL/src/sherlock_link.rs:1068-1100` (per MORI spec) |
| Decider verifies its child | `RL/src/decider.rs:148-153` |
| Decider public encoding equalities | `RL/src/decider.rs:178-184` |
| Decider residue pairing | `RL/src/decider.rs:191`; `RL/src/pairing.rs:228` |
| Stock child-verify helper that the terminal would call | `RL/src/inner.rs` (`verify_child_incircuit`, imported at `decider.rs:21`) |

## Changed keys and contract surface
- One terminal key pair replaces `link.vk` + `decider.vk` (with `w.vk`/`w_dir` removed). Its statement stays the same 16-field digest, so the 15-slot template and `acceptValidity` public vector are unchanged if the digest is kept. The VK changes, so the registered verifier key changes too. That needs a new StrawAnchor deployment, which the lead said is allowed.
- The hardened verifier patch is still required (the shift fix). It adds 371 rows at node arity 4 (`RL/REPORT.md` §3, 957,724 to 958,095), which is within the model's ε.
- The terminal must pin A102 and A80 VK SHA-256 digests and both context hashes (`MORI` spec, `LinkPins`).

## mono16 keyed fallback: evidence and caveats
- **Rows (measured by MockProver / synthesis only).** k=19, 16 slots: 449,169 rows, table 360,707 (`MONO/logs/cost.log:10,14`). The honest batch-1 accepts under MockProver in 35.19 s, peak 1.43 GB (`MONO/logs/mock-batch.log`). The MockProver result is not a proof. Opus-4 measured 449,419 rows for the same shape (`OPUS4/meas/cost_mono.log`). For 4 slots: 112,098 rows at k=17 (`MONO/logs/leaf4.log`).
- **Keyed evidence.** Keyed and proved: a single-slot k=15 gate, and a 4-slot k=17 segment (see the bullet above). The k=15 gate: keygen vk 1,565 B in 6.0 s, prove 15.9 s, proof 4,208 B, verify 0.019 s against the ledger's embedded 2^14 params, negative checks rejected (`MONO/keys/gate-jubjub/keygen.stats.json`; `MONO/out/gate-jubjub/batch-1.stats.json`; `MONO/logs/gate-jubjub.log`). A k=17 4-slot key was generated (`MONO/keys/leaf4/keygen.stats.json`, keygen 22.2 s). A keyed proof at that key exists: `MONO/out/leaf4/b1-g0/batch-1.stats.json` records `kind: mono-prove-call`, k=17, 4 slots, Segment output, prove 61.86 s, pk 12.61 s, verify 0.051 s, proof 4,208 B, peak RSS 1.46 GB, Poseidon transcript. This is a real proof (not MockProver).
- **Keyed evidence for 4 slots (k=17) exists** (above). **No keyed k=19 16-slot artifact exists.** `MONO/logs/m16.log` contains only "== keygen M (k=19)". There is no k19 keys/ or out/ directory. `RL/REPORT.md` §1 says `mono/` "keeps a 16-tx single-leaf fallback keyed", which the artifacts do not support. Opus-4's "k=19 probe proved in 95.9 s, 7.46 GB" (`SOL3`/`OPUS4` IDEAS §1) is another lane's measurement; I did not find its log in the repos I searched.
- **Privacy caveat (from OPUS4 IDEAS §4).** The L0 wallet rules need the spender secret key (they check pk = sk·G and nk = H(DOM_NK, sk)). In the mono design the batch prover holds every spender's sk. The proof stays sound, but "spender authorized" becomes "someone holding the key signed." The recursive design keeps sk with the user. The fixtures already carry `spender_secret_key_jfr`.
- Other limits: no pads (16 tx only), and the ceiling is about 74 transactions at k=21 (modeled).

## Caveats
- The fan-in model's per-child slope (191,842) comes from node arity sweeps on leaf proofs. The Sherlock children are k13/k12 (see the Sherlock bullet), so their slope is proxied from the native k13 sweep, not measured on those children.
- The Sherlock b=2 row count (~875k) is a proxy from the native k13 sweep, not a measured Sherlock child relation.
- The 316,833 pairing rows come from another lane and assume the same stock gates and the same k19/rc=4 setup. A fused k20/k21 circuit with the pairing included was not measured.
- Prove-time figures mix contention levels (the standin decider 1,182.8 s vs real 586.4 s, both k20). None of the fused-route times were measured.
- The 2× domain scaling is sol-3's convention, not a measurement.
- Lead's suggested capture path `astra-2/raw` is outside my write scope; this report only cites existing local files.

## Source check of astra-2's hint (RESULTS.md:157-160)
- `pairing-gadget/lanes/opus/RESULTS.md:157` says the pairing part is "0.30 M rows, or 0.32 M with G1 subgroup checks". This is the same measurement as 299,573 and 316,833 (`meas/strict.txt:2,4`), rounded. No conflict.
- The same paragraph says the decider's `prepare` plus the fixed-base MSM over about 63 fixed commitments is "0.83–1.68 M rows per the k<25 lane". That range is not in this report's additive model. The model's `190,356 + 191,842·b` is taken from the node sweep and is an approximation of the verifier part. Treat the fused totals as lower-confidence until the fixed-base MSM count for the Sherlock key is measured.

## mono16 communication-commitment note (checked)
- `RL/` mono relation: `mono/src/mono.rs:273-275` assigns both `BindingInput` and `CommunicationCommitment` slots as free public-input cells, with no Poseidon(comm_rand, stmt) opening. The Sherlock decider uses `CommBinding::BoundPoseidonStmtV1` (`RL/src/sherlock_link.rs:1078`). So the mono key cannot stand in for the Sherlock terminal's communication opening. This is a second reason, besides privacy, that mono16 is not a drop-in terminal.
