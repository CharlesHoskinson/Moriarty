# E6 results: keys and integration on the option E path

Loop item E6 of `research/sherlock-e-path-20261112/LOOP.md`; scope: the E6 row of the E1 to E6 table in `research/sherlock-m7-wrap-20261112/RESULTS.md` (worktree `~/Projects/Moriarty-wt/m7-wrap`, line 183). Gate: the devnet call is included and the refusals R1 to R3 pass; the D3 gates pass at the measured shapes.

Every number carries MEASURED, DERIVED or ESTIMATE. Choices marked **LEAD, owner to confirm** were taken while the owner was AFK (coordinator order 2026-11-12 22:30: recommended option, no questions).

## 1. Status

**E6 gate: PASS.** On the local devnet, the valid `acceptRun` call A with a fresh option E wrap proof is included (block 2,879, status SUCCESS). R1 and R2 are refused with `Malformed(InvalidProof)`. R3 has a valid proof, and the node refuses it at the guaranteed transcript read of `program_d`. All results are MEASURED, `~/.cache/e6-int/logs/call-final.log`. The wrap is the E5 wrap on the E3 map (D-9 decoder, E5 d044d01b3), as the coordinator ordered at 01:00.

| # | Item | Status | Section |
|---|---|---|---|
| 1 | The `L_Fr` key joins the universal key set; wrap key at most 50,000 bytes | PASS (wrap key 1,325 bytes, byte-equal to the E5 lfr-e3 key, ledger 9.1.0.1 accepts) | 3 |
| 2 | OD-18 hash tags for `L_Fr`: u = 5, 6, 7, transcript on u = 7 | PASS | 4 |
| 3 | Fr STARK premise A2' beside A2 (bridge-lean 46622c969); Lean wording | PASS (bridge-lean ecbce035f) | 5 |
| 4 | D3 at the measured shapes | PASS (101.1435 bits at all three keys) | 6 |
| 5 | Devnet deploy and call on the XC-9 path; R1 to R3 | PASS | 7, 8 |

Inputs: E5 PASS, Moriarty `e5-wrap` 7c75add31 then d044d01b3: the wrap on the E3 map with the D-9 decoder in the circuit, 1,605,413 rows at k = 21, MEASURED. E3 J5 PASS, Moriarty `e3-iface` 4b5156e1f and openvm 2f09caf. E4: the gate failed once; the owner ruled that E6 proceeds without it. All are merged into `e6-int` (cb1d20f8a, then aa6998370).

## 2. What the devnet call needs, and the scope of this call

The ledger checks a `ContractCall` proof against `[binding_input, comm] ++ public_transcript_inputs` (XC-9 `gp_call_prove`, `crates/mori-recursion/tests/sherlock_gp.rs:546-587` in `~/Projects/Moriarty-wt/xc9`). `binding_input` is fixed only when the transaction is built, so no proof made beforehand can be included. Each included call needs a fresh chain `L` -> `L_Fr` -> wrap whose statement is that vector.

Scope (stated, not a gate claim): `L_iface` is the E3 stand-in (E3 D-5): the bench `L` (39 AIRs over a bench node) plus the 4 interface AIRs that carry the 12,164 public words. The call-vector lanes of a call run are free inputs of `L`. `L` and `L_Fr` bind them, and the wrap decodes them, but no Sherlock program run produces them. So the call qualifies the **transaction path of the option E wrap on the unchanged ledger** (key framing, 1,026 public inputs, proof verification, the transcript read of `program_d`, fees). It does not qualify production FZ-3 (no frontend, no J6 deferred-log replay).

## 3. Item 1: the `L_Fr` key in the universal key set

### 3.1 Key set entries (E6)

One key set holds the option E entries (MEASURED: `~/.cache/e6-int/logs/lfrkey.log`, `wrapkey3.log`, `ledger3.log`, `runs.log`):

| Entry | Identity | Size | Where it is checked |
|---|---|---|---|
| `L_Fr` key (`L_iface` -> `L_Fr`, 15 AIRs, 561 columns, 1,522 public words, widths [33, 10]) | pre-hash `49816496883923548632055645801181396281864907914044199298774228413112923466066`; vk JSON sha256 `e900cb7f...ee82`, equal for the E5 `lfr-e3` fixture and the E6 key fixture | proving key 3.07 GB (not on chain) | inside the wrap: constants of the wrap relation |
| `L` key it wraps (E3 `L_iface`) | pre-hash `13534312134393826985920763714316908995664261901307810568034291294117731376140` | not on chain | `L_Fr` public word 0, pinned by the decoder |
| Wrap key (BLS12-381, k = 21, E3 map, relation `sherlock-wrap-v1` built from `~/.cache/e5-wrap/fixtures/lfr-e3.json`) | `MidnightVK` sha256 `2fc7563673dcc317aaf20ab5600dc1bb8916d21d71dec4b079d70d2d1d1a93dd`, byte-equal to E5 `~/.cache/e5-wrap/out/lfr-e3-k21/wrap.vk` | **1,325 bytes** (at most 50,000: PASS) | the deployed contract operation key; ledger 9.1.0.1 verifier, unchanged |

The wrap at this key (MEASURED, `wrapkey3.log`, run in the e6-int build):
- 1,605,413 rows against the k = 21 gate of 1,782,573. Table rows 1,256,451. 8 advice columns, 27 fixed columns, 1 lookup. 1,026 public inputs.
- Mock: 40.4 s. The honest run is accepted. A changed public input and a changed proof value are refused.
- Keygen 82.9 s, prove 133.3 s, proof 2,912 bytes, library verify ok.
- Stock ledger 9.1.0.1 (`midnight-ledger` rev a2b30128): `Ok(())` in 0.084 s. A changed public input and a flipped proof byte are each refused 3 of 3.

The `L_Fr` key and the `L` pre-hash are constants of the wrap relation, so the one on-chain key fixes all three entries (DERIVED). Proposed key-set row for `ARCHITECTURE`, for the final-design-2 DOC lane, which owns that file (not edited here): "Option E wrap key, BLS12-381 k = 21, 1,325-byte `MidnightVK` (sha256 2fc75636...93dd), E3 map with the D-9 decoder; fixes `L_Fr` pre-hash 4981...6066 (OD-18 codes u = 5, 6, 7) and `L` pre-hash 1353...1140."

### 3.2 The wrap statement: the E3 map (D-9 decoder, from E5)

The wrap statement is the call vector that the D-9 decoder (`tools/sherlock-wrap/src/decode.rs`, E5 7a4dfb308) computes from the `L_Fr` public words in the circuit. That covers range and `Canon` of each lane, the padding rules, the lane map and `PUB(i)`. E5 RESULTS section 6i has the full list. A call run of E6 (empty lane map, log = the reservation R only: 3 `DIGEST` entries) decodes to its 1,026 lanes. The native decoder accepts the E6 key fixture under the `lfr-e3` relation (cost run 1,605,413 rows, MEASURED, `wrapcheck3.log`). For both calls the prover checked that the wrap statement equals the ledger call vector, lane for lane.

History: before the E5 decoder arrived, E6 built its own map `callvec` (commit f62da0bb3). It handled call-vector lanes only and refused a non-empty lane map: 1,544,986 rows, key 1,325 bytes (sha256 `5be9344c...6a6f`), ledger `Ok(())`, Gemini Quint lane SOUND on all 5 questions (MEASURED, `gemini-quint/`). The merge aa6998370 removed it in favour of the E5 decoder, which is a superset. E6 makes no circuit change of its own on top of E5. **Open item:** no Gemini Quint lane for the D-9 decoder circuit is on record. The E5 RESULTS at d044d01b3 name none, and `~/.cache/gemini-lanes/` has no decoder lane. The E5 lane or the coordinator owns it. After the merge, `od18_vectors` (3 of 3) and E5 `i7_decode` (2 of 2) pass on aa6998370 (MEASURED, `~/.cache/e6-int/logs/wraptest3.log`).

## 4. Item 2: OD-18 tags of `L_Fr` (PASS)

- Code: stark-sdk `FrDomains::l_fr` = (u_leaf 5, u_node 6, u_transcript 7) (`crates/stark-sdk/src/config/bls12381_fr_poseidon.rs:71-83` in the stark-backend clone). The wrap uses `U_LFR_LEAF/NODE/TRANSCRIPT` = 5, 6, 7 (`tools/sherlock-wrap/src/native.rs:16-18`).
- The fixture proofs carry the domains: `lfr-real.json` and `mix-small.json` both record `{u_leaf 5, u_node 6, u_transcript 7}` with stock verdict true (MEASURED, read from `~/.cache/e5-wrap/fixtures`).
- Wrap-side vectors (new test `tools/sherlock-wrap/tests/od18_vectors.rs`, 3 tests, MEASURED pass): `cap_var(7)` equals `g3.Lfr.s2`. The native transcript and the generic circuit transcript (the `Native` back end of the code the `Gadget` back end runs) reproduce the ENC G3 `L_Fr` vectors `g3.Lfr.fresh3` and `g3.Lfr.obs2_sample3`, and the native one also `g3.Lfr.obs1_sample` (`research/sherlock-enc-fp3-20261112/VECTORS.txt`). The first u = 3 squeeze differs from the u = 7 one. This closes the wrap side of the ENC FREEZE-PENDING note for the `L_Fr` transcript.
- Comment to fix in the stark-backend clone: `FrDomains::l_fr` still says "LEAD pending OD-18". The owner confirmed OD-18 (a) on 2026-11-12. That clone belongs to the E1/E2 lanes, so the fix is noted here, not made.

## 5. Item 3: premise A2' and the Lean wording

A2' `lfrSound` already exists on `bridge-lean` (46622c969, `ProvenRegimeFr`, `lfrKey` with uses `[5, 6, 7]` and `tTag 7`). E6 changes docstrings only: `Clean/Sherlock/Bridge.lean` (KeyLFr/VerifyLFr: built and measured in E2 and E3, codes u = 5, 6, 7, `trStart 7`; EmulL: the measured `L_Fr` is a tape; A2': codes named, weakest term 101.1435 bits at the E2 I10 and E3 J5 keys) and `BridgeAxioms.lean` (A2' item). `lake build SherlockSpec`: Build completed successfully (32 jobs), 0 sorry, standard axioms (MEASURED, `~/.cache/e6-int/logs/lean.log`). The first commit attempt failed only the factory timing test (makespan 2.415 s against 1.9 s); the retry passed: bridge-lean ecbce035f.

## 6. Item 4: D3 at the measured shapes

`d3_wrapin` (openvm `e6-int`, MEASURED, `~/.cache/e6-int/logs/d3.log`, `d3e3.log`). Each run asserts the 100-bit gate:

| Fixture | AIRs | Columns | Public values | Domains | Weakest term | Total | Gate 100 |
|---|---|---|---|---|---|---|---|
| `~/.cache/e5-wrap/fixtures/lfr-real.json` (E2 I10 key) | 15 | 506 | 1 | u = 5, 6, 7 | query / shift 101.1435 | 101.1435 | PASS |
| `~/.cache/e5-wrap/fixtures/lfr-e3.json` (the deployed wrap's `L_Fr` key) | 15 | 561 | 1,522 | u = 5, 6, 7 | query / shift 101.1435 | 101.1435 | PASS |
| `~/.cache/e6-int/key/lfr-call.json` (E6 key fixture, same `L_Fr` vk) | 15 | 561 | 1,522 | u = 5, 6, 7 | query / shift 101.1435 | 101.1435 | PASS |

Other terms at the 561-column key (MEASURED): logup 215.95, stacked reduction 244.73, WHIR mu 226.30, fold 232.86, OOD 239.86, gamma 248.23, constraint batching 247.40, zerocheck 250.95, GKR sumcheck 253.27 and GKR batching 254.86 bits. The stock verdict is true at every fixture.

## 7. Devnet path and the call (MEASURED)

- Devnet: compose project `e6`, local images (node 2.1.101, indexer 4.4.101, proof server 9.0.1) on 127.0.0.1 ports 9954, 8098 and 6340. Genesis is `0x823383ec0302be8417cfcbcc62f21df08f4047bca37aaea89e3f54b550ebae1d` (the dev-preset genesis, same as XC-9). No Preview and no Mainnet.
- Wallet: the dev-preset genesis wallet. The runner reads its public seed in-process from testkit-js `LocalTestEnvironment.genesisMintWalletSeed[1]`. No file, log or script of this lane holds the seed.
- Deploy with the wrap key of section 3.1 (key framing `midnight:verifier-key[v7]:` + SCALE length + `MidnightVK`): MAIN `748d3885ee4feab9128afb1991573c4a648971cf93686d16f0f290f7ebdc5a63`, OTHER-PROGRAM `ac456c008515bee51013a50f3a307e60661abed3e9ae9d0970eba33cbb4929ab`. Both are included (`~/.cache/e6-int/logs/deploy-final.log`, state in `~/.cache/e6-int/run-e3`).
- Call vector of `acceptRun` (SherlockRun contract, XC-9 build): 68 transcript words plus one trailing `Noop{956}`, which gives exactly 1,026 words.
- Prover (`tools/sherlock-sdk/e6-wrap-prover.sh`, inside one exclusive heavy slot), one fresh chain per call:
  - the dump decoder;
  - `lfr_call call` (openvm `e6-int`: `L_iface` run with the call vector as lanes, then `L_Fr` at the deployed key);
  - the wrap at k = 21, relation from `lfr-e3.json`, with the regenerated key asserted byte-equal to the deployed key;
  - a check that the statement equals the ledger vector;
  - framing as `midnight:proof[v5]:`.
- Prover times: 1,209 s (A) and 1,201 s (R3), wall for the call phase 2,466 s.
- Intent TTL: 14,400 s (**LEAD, owner to confirm**; a chain takes 20 to 45 min on this host; ledger `global_ttl` 1,209,600 s).

| Call | Transaction sha256 | Verdict | Node evidence |
|---|---|---|---|
| A valid `acceptRun` | `fa6fae7b34042086316d90aaa8036f7ae1b7a02d9a1272982cb862a3ce7fe9f2` | **included**, block 2,879 (`0xa157b59e...8904`), status SUCCESS, indexer readback true, fee 196,601,674,643,093, 6,912 bytes | `run-e3/node-log-A.txt` |
| R1 result differs (r + 1, reused proof of r) | `1b32eb8f...a292` | refused | `Malformed(InvalidProof)` |
| R2 tampered proof (one bit flipped) | `b7a88a6d...4f29` | refused | `Malformed(InvalidProof)` |
| R3 other program (fresh valid proof of `prog_d`; contract holds `prog_d + 1`) | `435ce246...0b2b` | refused at the transcript read | "guaranteed execution would fail: mismatch between expected (`b6a63ceb...`) and actual (`b7a63ceb...`) read" |

Runner verdict: `{"A": true, "R1": true, "R2": true, "R3": true, "rehearsal": false, "pass": true}`.

## 8. Refusals R1 to R3 (definitions, XC-9 section 5)

- R1 result differs: result r + 1 with the valid proof of r (and another binding). Expected `Malformed(InvalidProof)`: MEASURED as expected.
- R2 tampered proof: the valid unbound call with one proof bit flipped, balanced fresh. Expected `Malformed(InvalidProof)`: MEASURED as expected.
- R3 other program: a fresh valid proof of the run, sent to OTHER-PROGRAM (`prog_d + 1`). Expected: the proof is accepted and the call is refused at the guaranteed transcript read of `program_d`. MEASURED as expected: the native check accepts the proof (`native.accepted` true), and the node refuses with the `prog_d` read mismatch.

## 9. Owner confirmations (LEAD)

1. Intent TTL 14,400 s for the call (section 7).
2. The E3 map scope as E5 states it: every decoder check is in the circuit, with no J6 deferred-log replay. The E6 call runs use an empty lane map and a log of R only (no `PUB` entries). The decoder's `PUB(i)` and lane-map checks were therefore not exercised on-chain; only the E5 I7 cases cover them (native and `MockProver`).
3. The superseded E6 `callvec` map was removed in favour of the E5 decoder (coordinator order, 2026-11-13 01:00).

Notes for other lanes:
- In the stark-backend clone, the `FrDomains::l_fr` comment "LEAD pending OD-18" is stale (E1/E2 lane).
- Clippy `-D warnings` reports 7 errors in the E5 files `merkle.rs`, `merkle_test.rs`, `verifier.rs` and `whir.rs` (E5 lane).
- Rebuilt from source (coordinator, 2026-11-13). Openvm `e6-int` 780db4a merges the e5-wrap `lfr_e3` exporter e147918. `lfr_e3 1` regenerates `~/.cache/e6-int/fixtures-src/lfr-e3.json` (keygen 196.2 s, prove 731.0 s). Its `L_Fr` vk and `decode` section equal the cached E5 fixture. The k = 21 wrap of that fixture (1,605,413 rows, mock pass, keygen 119.8 s, prove 167.5 s) gives a wrap key byte-identical to the deployed key, sha256 `2fc7563673dcc317aaf20ab5600dc1bb8916d21d71dec4b079d70d2d1d1a93dd`. All MEASURED: `~/.cache/e6-int/logs/e3export.log`, `wrapkey4.log`, `runs.log`, script `~/.cache/e6-int/prep4.sh`.
- The per-call prover regenerates the wrap key on each call (about 83 s). Caching the proving key would save that time.

## 10. Run log

- 2026-11-12 21:03. Worktree `e6-int` from `e5-wrap` a9de22611; e3-iface and enc-fp3 merged.
- 2026-11-12 22:15. Lean build pass; bridge-lean ecbce035f.
- 2026-11-12 22:22. Wrap tests pass (callvec, od18).
- 2026-11-12 22:30. Inputs E5 7c75add31 and E3 4b5156e1f merged (cb1d20f8a).
- 2026-11-12 23:15. E6 key fixture: wall 1,285 s (node 142 s, `L` prove 165 s, `L_Fr` keygen 197 s, `L_Fr` prove 661 s, MEASURED).
- 2026-11-12 23:54. D3 pass at the E2 I10 and E6 keys.
- 2026-11-13 00:55. callvec wrap key (1,544,986 rows, 1,325 bytes) and ledger check pass; first deploy done; the call was waiting for the exclusive slot.
- 2026-11-13 01:00. Coordinator: use the E5 lfr-e3 wrap. The waiting call was stopped before any proving. E6 work committed (f62da0bb3), e5-wrap d044d01b3 merged (aa6998370), D3 on lfr-e3 pass.
- 2026-11-13 01:49. Wrap key on lfr-e3 (1,605,413 rows), byte-equal to the E5 key; ledger `Ok(())`.
- 2026-11-13 02:31. Deploy, then A included and R1 to R3 refused. Gate PASS.
- 2026-11-13 04:06. lfr-e3 and the wrap key rebuilt from source (openvm 780db4a); key byte-identical.

## 11. Files

- Moriarty `e6-int`:
  - `tools/sherlock-wrap/src/bin/wrap.rs`: key fixture argument and `E6_EXPECT_VK`.
  - `tools/sherlock-wrap/tests/od18_vectors.rs`.
  - `tools/sherlock-sdk/`: `e6-local-run.mjs`, `e6-padding.mjs`, `e6-padding.test.mjs`, `e6-dump-prover.mjs`, `e6-wrap-prover.sh` (`E6_REL_FIXTURE`).
- openvm `~/.cache/m7-wrap/src/openvm-e6` (branch `e6-int` = e3-iface 2f09caf + e5-wrap e0d43ff, commits a0f07fa and merge 780db4a of e5-wrap e147918): `examples/lfr_e3.rs`, `crates/sherlock-wrap-export/examples/lfr_call.rs`, `examples/d3_wrapin.rs`, Cargo.toml dev-dependencies.
- bridge-lean: `clean/fork/Clean/Sherlock/Bridge.lean`, `BridgeAxioms.lean` (docstrings).
- Scripts and logs: `~/.cache/e6-int/` (`prep.sh`, `prep2.sh`, `prep3.sh`, `callphase.sh`, `run-call.sh`, `logs/`, `run-e3/`).
- Gemini Quint lane (callvec): `~/.cache/gemini-lanes/e6-callvec`, `research/sherlock-e6-int-20261112/gemini-quint/`.
