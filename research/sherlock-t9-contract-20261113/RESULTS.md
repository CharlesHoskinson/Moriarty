# T9 and T10a: encodings, the contract surface and the round 3 decoder

Rows T9 and T10a of `research/sherlock-tested-design-20261113/LOOP.md`, run alongside T6 on the existing `L_Fr` proofs (owner 2026-11-13). The `prog_d` key binding stays with T7 and T10.

Trees:
- Moriarty `t9-contract`.
- openvm `~/.cache/m7-wrap/src/openvm-t9`, branch `t9-contract` on `b3fbaa5` (T3 included).

The state, logs and fixtures are in `~/.cache/t9/`, and the chain runner is `~/.cache/t9/chain.sh`. Every number carries a label: MEASURED, DERIVED or ESTIMATE.

## 0. For the lead: picks and conflicts

1. **The ledger anchor is the ledger's own identity `C_L`.** The anchor is `C_L = DIGEST(SHK_PROGC, anchor_d2)`, fixed at deploy, and not the run's `C_P`.
   - The design text says the `a_in` of an ORD call is the run's own `C_P` (ARCHITECTURE 1.8; `A_0 = C_P`). One universal ledger cannot hold the `C_P` of each run, and the ledger has no chunked entry. So every ledger call is ORD with `a_in = a_out = C_L` (LEAD).
   - The cost: with a free `anchor_d2`, the lane-10 check binds the prologue template and the deploy-time anchor, not the program identity. The run's `C_P` is still bound, through `claim_pub`.
   - `anchor_d2` in `lfr_call` is a placeholder constant (`ANCHOR_D2`, `crates/sherlock-wrap-export/examples/lfr_call.rs`). Its real form, `pack(P2(DOM_PROGC, r_L, prog_d_ledger))`, is part of the T7/T10 key binding.
   - I-23 and I-24 therefore stay FREEZE-PENDING N-11.
2. **The opening-proof form of `R`.** The form bit is `open = [param of entry 4 = SHK_SET]`. In this form, `DIGEST(SHK_SET)` is entry 4 and the anchors move to entries 5 and 6, with `e_in = 4 + open` and `e_out = e_in + mode`. The `R` offsets are `[0, 2, 4, 6, 10, 12, 14]` (LEAD).
3. **`X` is a key constant.** For the bench cases it is `Keys::bench().x = 0x5e7c0de0001`. For `lfr_call` it is the cached-trace commitment of its `L` (`commit_child_vk(..).commitment[0]`).
4. **`layout_d` is pinned to the ENC-3 vector** `[1120272538, 1014741024, 1563488800, 1022622642, 354000869, 288819430, 75200499, 498114873]`, from `DOM_LAYOUT` 11 and `"midnight:binding-input[v1]"` at offset 58.
5. **The `PATH` node hash is the `H2` of the log (placeholder, LEAD).** The siblings are wrap-private: `path_sibs` in `wrapin` and `sibs` in each decode case.
6. **ENC-4 has a conflict on task lane 3.** It is typed `tag(2)` (FP-9.1 `rtag`, where `Input` is 2), but Appendix A says `tag(1)`. Four pins depend on the choice: `enc4.task[1].EqualTask`, `enc4.tasks.step.rw_push`, `enc4.tasks.below(count=3,rw_push)` and `enc4.tasks.cells(3)`.
7. **ENC-5 part tags 58 to 62** (`itab`, `menu`, `policy`, `layout`, `place`) are the next free codes after FP-9.5 (45 to 57). `SHK_SHAPE` is named in ARCHITECTURE 3.x but has no code yet, so it takes the next free code after 62 (LEAD).

## 1. Done-when status

| Gate | Result | Label |
|---|---|---|
| T9: the Rust reference, the AIR and the wrap give equal values on every pinned vector | holds on every vector with a wrap side; two named gaps (section 3) | MEASURED |
| T9: the compiled contract tests pass | 9 of 9 (`tools/sherlock-sdk/sherlock-ledger.test.mjs`) | MEASURED |
| T10a: wrap rows at most 1,782,573 at k = 21 | 1,680,729 rows (94.3 percent) | MEASURED (cost model) |
| T10a: wrap key at most 50,000 bytes | 1,325 bytes (MidnightVK framing) | MEASURED |
| T10a: ledger 9.1.0.1 Ok with refusals | `Ok(())`; 3 changed public inputs and 3 flipped proof bytes refused | MEASURED |
| T10a: devnet call included, non-empty lane map and `PUB` entries | included at block 12,951 (tx `0x5b3f37b9…`), lane map with 4 used pairs, 4 `PUB` entries; R1, R2 and R3 refused | MEASURED |
| T10a: no plain `prog_d` or `n_steps` in the call vector or ledger state | no forbidden word on the 1,026-word vector, 13 transcript ops or the indexed state; the controls fire | MEASURED, with the bench limit of section 5 |

The wrap rows are +12,546 over the T3 baseline of 1,668,183 (DERIVED). They are +4,507 over the T4 probe total of 1,676,222, which priced only the PATH replay and the head checks (DERIVED). The extra rows are the opening-proof form of `R`, the fifth lane-map pair and the chunk anchor-write lanes. Both the E5 fixture `lfr-e3` and the T10a key fixture `lfr-call` give the same rows, because the relation is one shape.

## 2. What changed

### 2.1 The contract: `tools/sherlock-sdk/sherlock-ledger.compact` (Moriarty `a71bb18a8`)

The contract holds:
- the sealed `anchor` at field 0;
- one `runs` map shared by `acceptRun` and `openClaim`, so a claim settles at most once (FP3-3 (a));
- `verified_roots`;
- the sealed `reg_root`.

Its entries:
- `acceptRun(c_p, input_pub, outcome_pub)` replaces the stand-in `sherlock-run.compact`, which disclosed `program_d` and `steps` (E6 finding).
- `acceptCollection(reg_pub, set_pub)` has no `shape_d` (O9).
- `openClaim(set_pub, c_p, input_pub, outcome_pub)`.

The lanes below are MEASURED on the zkir, and the devnet dump confirms them word for word:
- **Prologue, every entry:** template lanes 2 to 9 are `0x30, 0x50, 1, 1, 0, 0x0c, 1, -2`, and lane 10 is the anchor read.
- **`acceptRun`:** 42 lanes, with `claim_pub` at 20 and 34 and `outcome_pub` at 39.
- **`acceptCollection`:** 48 lanes, with the `reg_root` read at 19 and `set_pub` at 29 and 43.
- **`openClaim`:** 57 lanes, with `set_pub` at 20, `claim_pub` at 35 and 49, and `outcome_pub` at 54.
- `C_P` and `input_pub` are on no lane.

The compiled tests (all pass, 9 of 9):
- XC-3b parity: `transientHash` equals the u = 0 sponge and the Rust `claim_pub` values.
- `fp3_cross_entry_replay` is refused in both orders. The two-map control accepts both.
- The `openClaim` and collection refusals.
- The privacy scan, with a control: the same scanner finds `pack(prog_d)` and `steps` in a `sherlock-run.compact` call.
- LW-009: the `acceptCollection` inputs are exactly `[reg_pub, set_pub]`, and it changes only `verified_roots`. The shape control fails all three checks.
- The prologue lanes in all three entries.

The runner `t10a-ledger-run.mjs` covers deploy, dump, call and dry-run, with R1 to R3 and the forbidden-word scan. The lanes are in `contract/lanes.json`.

### 2.2 Circuit changes (for the Gemini Carbon Quint lane)

**Wrap decoder, `tools/sherlock-wrap/src/decode.rs` (Moriarty `c4a899cba`):**
- `:55-72`: STMT 116, `LANE_PAIRS` 5, `LAYOUT_WORD` 89, `X_WORD` 97, `LANE_POPEQ` 10, `LANE_A_OUT` 20, `TPL_ORD`, `TPL_CHUNK`.
- `:102`: `R_OFF` `[0, 2, 4, 6, 10, 12, 14]`.
- `:176`: `DecodeShape::from_doc` reads the key constants `keys.x` and `keys.layout_d`.
- `:318`: `decode_with(.., sibs)`.
- `:366-372` (T4 items 7, 8, 21): `X` as 9 limbs of 30 bits with `Canon`, equal to the pinned cached commitment; `layout_d` equal to the key constant.
- `:494-530` (T4 item 13): the opening-proof form of `R`. `open = [param_4 = SHK_SET]`; `a_in` is the output of entry `4 + open`; `a_out` is that of entry `5 + open` when `mode` is 1.
- `:557-559` (item 20, wrap part): no lane-map pair names lane 0 or 1.
- `:592-605` (items 17, 18, 19): the template lanes; the `popeq` lane equals `a_in`; for a chunk, the anchor-write template lanes and lane 20 = `a_out`.
- `:629`, `:702`, `:727-760` (item 26): the replay tuple gains `PATH`. The siblings are wrap-private witnesses, with an index and root check.

**Wrap plumbing:**
- `tools/sherlock-wrap/src/wrap.rs:49-111`: the sibling witness from `path_sibs`.
- `src/verifier.rs:216-264`: `ProofData.path_sibs`.

**Stock decoder (Rust reference), openvm `crates/sherlock-lfr/src/iface.rs` (`609c4f1`):**
- `:39-43`: the index table at 5 pairs.
- `:96-125`: the prologue lanes and `tpl_ord` / `tpl_chunk`.
- `:149-175`: `layout_d_of` and `Keys`.
- `:659`: `r_entries(head, open)`.
- `:728`: `decode_with`, checks 1 to 8.
- `:897`: `reservation_with(.., open, anchor_d2)`.
- `:1027`: `Case::Open`.

**`L_Fr` tape (the AIR side of `R`):** `crates/sherlock-lfr/src/lverify.rs:1732-1820`, `gen_r_check` in the opening-proof form (open bit, entries 4 to 6 with cswap gating, `e_in`/`e_out`).

**The T10a run:**
- `crates/sherlock-wrap-export/examples/lfr_call.rs:119` holds `ANCHOR_D2`, the T7/T10 seam for the ledger's `prog_d`.
- `:128` is `call_run`: X = the cached commitment, ledger anchor `C_L`, 4 `PUB` entries on lanes 21, 22, 40 and 41, and the 5-pair lane map `(10, 4) (20, 3) (34, 3) (39, 2) (0, 0)`.
- `:177` is `run_args`.

**ENC-4/5:** `crates/sherlock-lfr/src/enc45.rs` (`c7c0c2e`). Host-side encodings; no circuit consumes them yet (T6).

### 2.3 Docs (Moriarty `a84f8123a`)

- ARCHITECTURE FP-3: the index table has 5 pairs and 12,166 words, and fact 2 gives the lanes after the prologue.
- I-23 and I-24 keep FREEZE-PENDING N-11 and now carry the T9 evidence. I-28 loses the O7 gate.
- LW-009 and DCD-004 drop `shape_d`, and LW-009 gains a compiled-contract scenario. `openspec validate --strict` passes on both changes.
- The prose fix `zi^-1` -> `zi_root^-1` is in `tools/sherlock-wrap/src/whir.rs:224` and `research/sherlock-e5-wrap-20261112/RESULTS.md:174`.

## 3. Vector matrix

All rows are MEASURED. "n/a" means that side does not compute the value, and the reason is given.

| Vector | Rust reference | AIR | Wrap |
|---|---|---|---|
| ENC-1 (19: tags, `C_P`, `A_1`, `input_pub`, `outcome_pub`, `claim_d`, digests) | stark-sdk `enc_vectors.rs`; `iface::tests::enc1_*` | `enc_fp3_vectors_on_machine_airs` (`C_P`, `A_1`, `input_pub`, `outcome_pub`, `u0.h2`) | I9 Part A, 40 checks, native and MockProver (k 15) |
| ENC-2 `enc[0..6]` words, `limbs(r-1)`, `pv[0..12]`, `pv` fingerprint | `enc_wrap` asserts each against the pins; `iface::tests::enc2_*` | `t9_enc2_onion6_on_the_onion_air`: the onion AIR takes `enc(e)` of the 6 entries and accepts `log_d` = `enc2.onion6`; a change of word 0 or 7 is refused (`01eb63f`) | I9: each `enc[i]` value word, `limbs(r-1)` and the pv words equal the pins; 12 decode cases equal the stock verdict |
| ENC-2 `D_EMPTY`, `onion6`, `node(2,4)` | `iface::tests::enc2_onion_vectors`, `enc2_node_merge` | onion6 as above | n/a: BabyBear `P2`, which the wrap never computes (the onion is checked in `L`) |
| ENC-3 `layout_d` | `iface::tests::enc3_layout_d_vector` | n/a: a key constant that `L` exposes as head words 89 to 96 | pinned by the decoder; mutant `wrap_layout_d_other` refused |
| ENC-4 (frame, task, StackStep, StackCells, EMPTY_F/T, below chains) and ENC-5 (`itab`, `menu`, `policy`, `layout`, `place`, `plan_d`): 48 pins | `t9_enc45_vectors`, refusals `t9_enc4_refusals`, `t9_enc5_refusals` (3 of 3) | waits on T6: no STACKS, BOUNDARY or plan AIR exists before the leaf build | n/a: BabyBear-internal |
| FP-3 lanes after the prologue | the stock decoder and `lfr_call args` on a real ledger vector | J4 tapes | I7, 46 cases with stock = wrap; 27 MockProver cases |

The vectors are in `research/sherlock-enc-fp3-20261112/VECTORS.txt` (55) and `VECTORS-ENC45.txt` (48, this folder).

### 3.1 Decoder checks (T4 list) on the three sides

- **Stock decoder (`lfr_e3 0`):** it refuses the T9 named mutants:
  - `wrap_x_not_cached_commitment`, `wrap_x_not_canonical`, `wrap_layout_d_other`;
  - `wrap_popeq_lane_not_a_in`, `wrap_anchor_template_lane`, `wrap_lane_map_lane_0`, `wrap_lane_map_lane_1`;
  - `wrap_open_e_in_4`, `wrap_chunk_a_out_not_written`, `wrap_chunk_write_template`;
  - `wrap_path_root_changed`, `wrap_path_sibling_changed`, `wrap_path_index_changed`.

  It accepts `open_honest`, `chunk_honest` and `path_honest`, along with the E3 cases.
- **Wrap (I7, `~/.cache/t9/logs/wrap-i7i9.log`):** on all 46 cases the stock verdict equals the native wrap verdict. In MockProver, the honest case, the 3 T9 positives and 23 mutants all agree.
- **`L_Fr` tape (J4, `~/.cache/t9/logs/j4-tapes.log`):** 11 `L_iface` proofs at 12,166 words. The 8 positives have 0 failed assertions, the new `open` included, and the 3 negatives are refused.
- **Wrap suite:** 25 tests pass, I0 to I9 and OD-18 (`wrap-suite.log`).
- **sherlock-lfr:** the lib and every test pass (29 tests).

## 4. T10a devnet call

Local e6 devnet only. Node 2.1.101, indexer 4.4.101, proof server 9.0.1, runner `tools/sherlock-sdk/t10a-ledger-run.mjs`, script `~/.cache/t9/devnet-call.sh`, log `~/.cache/t9/logs/devnet-call.log`. All MEASURED.

1. **Deploy.** One `SherlockLedger` with the final wrap key (1,325 bytes) on all three entries, deployed with the anchor `C_L` = `9927351763…829022232` (the `lfr_call args` value) and the default `reg_pub`. The readback matches: anchor, `reg_root`, both maps empty. The key body is stored once.
2. **The proof of A.** One fresh chain through `e6-wrap-prover.sh`, from the ledger's own call vector (1,026 words), with no per-call keygen of `L_Fr`:
   - `lfr_call call`: the `L_iface` proof (45 s); the `L_Fr` proof under the cached key (the same `L_Fr` pre-hash as the key fixture).
   - The wrap at k = 21 under the deployed key, asserted byte-equal.
   - The wrap statement equals the ledger vector lane for lane.

   Before proving, `lfr_call` asserts two things: lanes 10, 20, 34 and 39 equal the `R` outputs, and the native decode equals the vector. The log holds `R` (5 entries) and 4 `PUB` entries on lanes 21, 22, 40 and 41. The lane map has 4 used pairs: `(10, 4)`, `(20, 3)`, `(34, 3)`, `(39, 2)`.
3. **A, the valid `acceptRun`.** It was included at block 12,951 with status SUCCESS: tx `0x5b3f37b97e0546985d994a187a2f697050e8042fe043b88588e8457b5ecb02d0`, 6,783 bytes.
   - The readback has `runs[claim_pub] = outcome_pub`, `runs` size 1, the anchor and the keys unchanged.
   - The included program equals the proven one.
   - The vector layout: `claim_pub` at 20 and 34, `outcome_pub` at 39, the anchor at 10; `C_P` and `input_pub` on no lane.
4. **The scan.** It covers the call vector (1,026 words), the 13 included transcript ops and the indexed state. It finds none of the run's private `R` operands (`R0.d2`, `R1.d2`, `R2.d2`) or `anchor_d2`. The controls `claim_pub`, `outcome_pub` and the anchor are found on every surface, so the scanner fires.
5. **The refusals.**
   - **R1:** `outcome_pub + 1` with the proof of A reused. Refused by the node with `Malformed(InvalidProof)` (error 115).
   - **R2:** one proof bit flipped. Refused by the node.
   - **R3:** A replayed after it landed (A's proof, fresh binding). Refused by the node at the proof. The replay rule itself is shown on the post-A state: the circuit reports "claim already settled", and a proofs-off ledger apply reports a read mismatch.
6. **Wall time.** The call phase took 533 s, including the prove.

## 5. Open items and honest gaps

- **`prog_d` key binding (T7/T10).** `ANCHOR_D2` is a placeholder, and T4 item 27 (the 6 FP-8 key words, 1,528-word statement) stays with T7.
- **The bench head has no real `n_steps` or `prog_d`.** The scan therefore forbids the private `R` operands of this run (the three `d2` words and `anchor_d2`). The scanner control, a `sherlock-run.compact` call, shows that the scanner finds `pack(prog_d)` and `steps` when they are present (compiled test d).
- **ENC-4/5 AIR sides** wait on T6 (section 3).
- **`PATH` node hash.** It is the log's `H2`, a LEAD placeholder (section 0, item 5).
- **Lane 0 = `bind`, lane 1 = `comm`** is an `L`-side check (T4 item 20). The bench `L` does not build it, and the wrap keeps the lane map off both lanes.

## 6. Runs and artifacts

- Chain: `~/.cache/t9/chain.sh`, units `t9-chain3` and `t9-chain4`. The 18:34 reboot killed `t9-chain3` during `lfr_e3 1`, and `t9-chain4` resumed from that stage. Log `~/.cache/t9/logs/runs.log`.
- Fixtures: `~/.cache/t9/fix/{e3,e5}` and `~/.cache/t9/key/lfr-call.json`. The wrap key is `~/.cache/t9/out/key-k21/wrap.vk`.
- Timings (MEASURED, under load from other lanes):
  - `iface_fixture`: 55 min, 11 proofs.
  - `lfr_e3 1`: keygen 71 s, prove 250 s.
  - `lfr_call key`: `L_Fr` prove 203 s.
  - Wrap: keygen 32 s, prove 51 s, proof 2,912 bytes.
- Patch series: `research/sherlock-stark-code-20261112/openvm-t9-contract/` (30 patches, first-parent export, tip `01eb63f`). Round trip checked: base plus `git am` gives tree `60db66a`, equal to the tip tree.
