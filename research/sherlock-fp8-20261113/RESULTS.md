# FP8 results: the universal key set under the frozen sets and the new node bounds

Loop item FP8 of `research/sherlock-e-path-20261112/LOOP.md`. Branch `fp8` (from `e6-int` cd5b13c23, with `x-exp` 77b735941 merged at 5d9a2d705). Openvm worktree `~/.cache/m7-wrap/src/openvm-fp8`, branch `fp8` (from `e6-int` 780db4a); own stark-backend worktree `~/.cache/m7-wrap/src/stark-backend-fp8` (branch `fp8` at e2-lfr 5f698b5, no change). Gate: FP-8 of `FD/ARCHITECTURE.md` (row at line 3349 and section 1.12; FZ-4).

Labels: MEASURED (read from a run log), DERIVED (computed from MEASURED numbers by a stated rule), ESTIMATE. Choices taken while the owner is AFK are marked **FOR OWNER CONFIRMATION**.

## 1. Status

**FP-8 gate: BLOCKED.** The six work items of this lane pass (table below). The FP-8 row of `FD/ARCHITECTURE.md` (FZ-4) also needs items that this lane does not close; section 9 names them. The first blocker is the GP-3 FV work-unit keys: their shapes are still being measured by X-EXP, and the measured units are over the 3,000-column node bound.

| # | Item | Result | Section |
|---|---|---|---|
| 1 | One universal key set under the new node bounds (leaf, node, `L`, `L_Fr`, wrap); per-program preprocessing bound to `prog_d` | PASS for the keys: every key generated through G1 keygen, node key menu with its fixed point. Binding: the `prog_d` encoding is computed; its enforcement in a circuit is not built (section 6) | 3, 4, 6 |
| 2 | G1 parameters in keygen | PASS: `sherlock_lfr::frozen` with `keygen_checked`, used by the FP-8 generator; 4 of 4 tests pass. The older bench examples and tests are not routed through it (section 2) | 2 |
| 3 | Size and digest of every key; wrap key at most 50,000 bytes | PASS: wrap `MidnightVK` 1,325 bytes | 3 |
| 4 | D3 on every key at the measured shapes; every term at least 100 bits | PASS: weakest 101.0110 bits (leaf layer); every grindable term at least 101 | 5 |
| 5 | The wrap key verifies under the unchanged stock ledger 9.1.0.1 | PASS: `Ok(())`, a changed input and a flipped byte refused 3 of 3; ledger-check unchanged against `e6-int` | 7 |
| 6 | The keys that changed against E6, and why | every key changed; causes in section 8 | 8 |

## 2. G1: the frozen sets in keygen (item 2)

New module `crates/sherlock-lfr/src/frozen.rs` (openvm `fp8` 9d4105f; copy `src/frozen.rs.txt`). It holds one `FrozenSet` per layer: blowup, `l_skip`, `n_stack`, `w_stack`, folding, `mu`, query and LogUp PoW, the explicit per-round query vector, and the shape bounds. `params()` builds the `SystemParams` with the frozen vector in place of the derived one. `keygen_checked` runs keygen only for an engine whose parameters equal the frozen set in every field. It then refuses a key outside the bounds: AIR count, total columns, constraints, interactions and degree per AIR. `check_heights` checks the trace-height bound of a proof. `fixture::internal_ud`, `fixture::l_ud(48)` and `testing::lfr_params` now return the frozen sets, so the E2 to E6 call sites take the frozen internal, `L` and `L_Fr` parameters. Those call sites still call raw `engine.keygen` (READ: `lfr_call.rs`, `lfr_e3.rs`, `lfr_real.rs`, `iface_fixture.rs`, `l_fixture.rs`, tests `i10_gate`, `j1_public`, `j5_gate`, `testing.rs`), and their app leaves use the stock app set. In particular, the E6 per-call prover `lfr_call` still builds the E6 topology (W1 -> one node -> `L`). It cannot prove against the FP-8 wrap key until it is moved to the FP-8 chain and `keygen_checked`. That move is open.

| Set | Blowup, `l_skip`, `n_stack`, `w_stack` | PoW folding, `mu`, query; LogUp | Queries | Bounds (height; constraints and interactions per AIR; AIRs; columns; degree) |
|---|---|---|---|---|
| app | 1, 4, 20, 2,048 | 5, 15, 21; 18 | 193/88/81/81 | 2^24; 5,000, 1,000; 100; 30,000; 3 |
| leaf aggregation | 2, 4, 17, 2,048 | 4, 13, 21; 17 | 118/84/81 | 2^21; 1,000, 100; 50; 2,000; 4 |
| internal | 3, 2, 17, 512 | 0, 9, 21; 18 | 97/82/81 | 2^19; 1,000, **200**; **64**; **3,000**; 4 (owner rulings 2026-11-12 20:35) |
| `L` | 3, 2, 18, 48 | 0, 6, 20; 18 | 98/83/82 | 2^20; 1,000, 100; 50; 2,000; 4 |
| `L_Fr` | 3, 2, 18, 48 | 0, 0, 20; 0 | 98/83/82 | 2^20; 1,000, 100; 50; 2,000; 4 |

Tests (`crates/sherlock-lfr/tests/g1_frozen.rs`, 4 of 4 pass, MEASURED, `out/g1test.log`):
- The internal, `L` and `L_Fr` sets equal the E2 to E6 formulas field for field, so G1 alone changes none of those keys.
- The stock app set already has the vector 193/88/81/81 (derived at query PoW 20). G1 refuses it on `query_pow` (20 against the OD-4 value 21). The stock derivation at query PoW 21 gives 191/87/80/80, and G1 refuses that on the query vector (MEASURED; `PR` 11.2 predicted 191/87/80/80).
- G1 refuses a changed query count, query PoW, `mu` PoW and LogUp PoW (internal), and LogUp PoW 0 for the leaf-aggregation set.
- G1 refuses a key over the columns, constraints or AIR count of a tightened bound, a node AIR above a tightened interaction bound, and a proof with an AIR at 2^20 under the internal height bound 2^19.

The bounds are admission checks: no field of a key holds them (stark-backend `SystemParams` has no per-AIR bound field, READ). So the new internal bounds change no key byte. A node accepted under the new bounds is also accepted under the old ones (MEASURED, `out/keys.json` "bounds"). **FOR OWNER CONFIRMATION:** G1 lives in the Sherlock crate that every Sherlock keygen goes through, not in stark-backend keygen, and no bound digest is written into any key.

## 3. The key set (items 1 and 3; MEASURED, `out/chain.json`, `out/wrapkey.log`)

Chain, all in one run (systemd unit `fp8-chain`, heavy slots, shared host): two app leaves W1 (Poseidon2 AIR, 2^20 rows) -> two leaf-layer proofs -> node `N1` (arity 2) -> `N2` over (`N1`, `N1`) -> `N3` over (`N2`, `N2`) -> `L_iface` (the E3 bench `L` with the 4 interface AIRs, `Dims::FULL`, E3 case all-entries seed 14) -> `L_Fr` (E3 J5 tape) -> the wrap at k = 21. Every keygen went through `keygen_checked` and every one was accepted. Every proof whose heights were recorded (app, leaf, `N1`, `N3`, `L`, `L_Fr`) is inside its set's height bound. `N1` sits exactly at the internal bound 2^19, so an arity 3 or 4 node over the leaf layer may exceed it (unmeasured).

| Key | Set | Role | AIRs | Columns | Max constraints / interactions per AIR | Max log height (bound) | Key bytes | Digest |
|---|---|---|---:|---:|---|---|---:|---|
| app W1 | app | per program | 1 | 299 | 282 / 0 | 20 (24) | 12,743 | pre-hash `[1944710894, 340692597, ...]`; bitcode sha256 `a7f5c7bf...3ed4` |
| leaf layer `<1>` | leaf aggregation | per program | 39 | 1,531 | 282 / 77 | 17 (21) | 78,050 | `[1039084273, 235834964, ...]`; `547fa71a...fbfa` |
| node `N1<2>` over the leaf layer | internal | universal (equal for two programs, section 4) | 39 | 1,549 | 282 / 77 | 19 (19) | 81,654 | `[132761478, 361854733, ...]`; `b3d02b35...4430` |
| node `N2<2>` over `N1` | internal | universal | 39 | 1,549 | 282 / 77 | proved and verified in the chain; heights not recorded | 81,541 | `[318794432, 328435825, ...]`; `21991313...54af` |
| node `N3<2>` over `N2`, the internal-recursive key (`N4 = N3`) | internal | universal | 39 | 1,549 | 282 / 77 | 17 (19) | 81,541 | `[1083965961, 338922679, 144227788, 1195169794, 1944564697, 118437343, 1785929472, 1166543170]`; `2c525b73...2891` |
| `L` (`L_iface` over `N3`) | `L` | universal | 43 | 1,805 | 282 / 77 | 16 (20); stacked width 18 (17 + 1) of 48 | 238,341 | pre-hash `37026598407252285687841374476094739314673363040944337351795480463086523737945`; `4b3f31e4...108f` |
| `L_Fr` | `L_Fr` | universal | 15 | 561 | 171 / 46 | 19 (20); widths [33, 10] | 45,528 (proving key 3,065,426,767) | pre-hash `25266591154478280991082187637282912940818142043152807661338592691984604540818`; `f8e493af...f57a` |
| wrap (BLS12-381, k = 21, relation from `lfr-fp8.json`) | stdlib wrap | universal, on chain | 8 advice, 27 fixed, 1 lookup | | | 1,605,413 rows of the 1,782,573 gate (90.1 %) | **1,325** (`MidnightVK`, at most 50,000: PASS) | sha256 `4990db4dc0407b39c20441b8a89db92b76ba39224c9336a1814c3cad2573a28d` |

Key bytes for the STARK keys are the bitcode `MultiStarkVerifyingKey`; these keys stay off chain. Full pre-hashes and sha256 values are in `out/chain.json`. `L` is inside its own bounds (43 of 50 AIRs, 1,805 of 2,000 columns, 77 of 100 interactions), so the `L` bound needs no change. Times (shared host, not timings): app 2 proofs 77.1 s, leaf 2 proofs 21.1 s, `N1` 30.2 s, `N2` 16.9 s, `N3` 16.2 s, `L` 170.0 s, `L_Fr` keygen 203.6 s, `L_Fr` prove 692.8 s, wrap mock 45.0 s, keygen 83.5 s, prove 135.0 s (MEASURED).

The wrap fixture `~/.cache/fp8/out/chain/lfr-fp8.json` (sha256 `c8aa14e9...0822`) has domains u = 5, 6, 7, stock verdict true, and the 11 D-9 decode cases of `lfr_e3`: honest and `free_lane_changed` decode, and the other 9 are refused (MEASURED). Wrap mock: the honest run is accepted, and a changed public input and a changed proof value are refused; proof 2,912 bytes, 1,026 public inputs, library verify ok (MEASURED, `out/wrapkey.log`).

## 4. The node key menu and the fixed point (MEASURED, `out/keys.json`, `out/fixpoint.json`)

- **Fixed point.** A node over a node of the same arity changes key at depth 1 to 2 (`trace_height_constraints` and the constraints of 22 AIRs) and at depth 2 to 3 (the constraints of AIR 16). From depth 3 on it is constant: depth 3 = 4 = 5 = 6 = 7. The universal internal-recursive key is therefore `N3`, and `L` verifies `N3`. **FOR OWNER CONFIRMATION (pick):** on the full tree, a chain of fewer than 3 node levels pads with nodes until depth 3, at a cost of 16.9 s + 16.2 s per call for `N2` and `N3` here (MEASURED, shared host).
- **Universality.** For two programs (app 1 AIR against 2 AIRs, different leaf-layer keys), `N1`, `N2`, `N3` and the `L` over them are byte-equal (pre-hash equal). The node keys depend on the child's shape class, not on the child key (the child vk is a cached trace). So `N1` to `N3` are universal for leaf layers of this shape.
- **Arity menu** (arity at most 4, LD-S10x-2; keygen only, all accepted under the internal bounds): over the leaf layer, 1,537 / 1,549 / 1,561 / 1,573 columns at arity 1 / 2 / 3 / 4, all 39 AIRs, at most 77 interactions, keys 80,561 to 83,844 bytes, 4 distinct digests; over a node, the same columns with 4 distinct digests. The FP-8 chain uses arity 2 (as E6). **FOR OWNER CONFIRMATION:** the universal set fixes the node arity at 2, and a leaf count that is not a power of 2 pads by repeating a child proof (as `N2` and `N3` do here). Other arities would each need their own fixed-point chain and their own `L`.
- **Path C** (`L` directly over the leaf layer at internal parameters) was not keyed here; it stays with I-17 and D10 (section 9).

## 5. D3 per key (item 4; MEASURED, `out/d3.json`, `out/d3.log`)

Stock calculator of the `fp8` stark-backend (one OOD sample). Three shapes per key:
- measured: the key's AIRs, columns and constraints, with the proof's largest height and stacked width;
- vk: `calculate_from_vk`;
- bounds: the set's shape bounds of section 2.

| Key | Weakest, measured | Weakest, vk | Weakest, bounds | LogUp | `mu` (vk) | Query | OOD | Stacked (bounds) | Gate |
|---|---|---|---|---:|---:|---:|---:|---:|---|
| app | 101.0891 (query, shift) | 101.0891 | 101.0891 | 102.7205 | 102.6282 | 101.0891 | 104.6276 | 107.7549 | PASS |
| leaf layer | 101.0110 | 101.0110 | 101.0110 | 101.7206 | 102.6283 | 101.0110 | 107.6276 | 111.6618 | PASS |
| `N1` | 101.1643 | 101.1643 | 101.1643 | 102.7205 | 101.6304 | 101.1643 | 109.6276 | 111.0768 | PASS |
| `N2` | heights not recorded; bounded below by the vk column (DERIVED) | 101.1643 | 101.1643 | 102.7205 | 101.6304 | 101.1643 | 109.6276 | 111.0768 | PASS |
| `N3` | 101.1643 | 101.1643 | 101.1643 | 102.7205 | 101.6304 | 101.1643 | 109.6276 | 111.0768 | PASS |
| `L` | 101.1427 (stacked width 18) | 101.0730 (`mu`, `w_stack` 48) | 101.0730 | 102.7205 | 101.0730 | 101.1427 | 108.6276 | 111.6618 | PASS |
| `L_Fr` | 101.1435 (query) | 101.1435 | 101.1435 | 215.9502 | 226.3025 | 101.1435 | 239.8571 | 242.8913 | PASS |

Every term of every key is at least 100 bits, and every grindable term (LogUp, `mu`, folding, query) is at least 101 bits, at all three shapes. The internal stacked term at the new bounds is 111.0768, equal to X14 (a). The leaf-aggregation LogUp is 101.7206 at LogUp PoW 17, as `PR` 11.2. D3 does not apply to the wrap. The wrap is a halo2 PLONK proof under KZG on BLS12-381: its soundness rests on the KZG/AGM premise A1 `wrapKnowledgeSound`, not on a round-by-round STARK bound.

If X-EXP takes 3,300 columns for the FV work units (coordinator note 2026-11-13), only the stacked term of the internal set at its bound moves; X-EXP reports 101.1643 bits as the weakest term at 3,300 (X-EXP, not rerun here).

## 6. `prog_d` binds the per-program preprocessing (item 1; MEASURED, `out/keys.json` "prog_d")

The vk pre-hash covers the parameters, every per-AIR key (preprocessed commitment included) and the trace-height constraints (READ, stark-backend `keygen/mod.rs` 197-218). So a pre-hash binds a key's preprocessing. Encoding (**LEAD, FOR OWNER CONFIRMATION**; `PR` 4.2 and 4.3 do not write the word list of a `key_root` leaf):
- `key_root` leaf of chip set `c`: `P2(DOM_KEYTRIPLE = 8, c, pre(app), pre(leaf layer), pre(internal-for-leaf N1))`;
- nodes `P2(DOM_KEYNODE = 10, l, r)`, padded with `P2(TAG_EMPTY = 1)` to at least 2 leaves;
- `prog_d = P2(DOM_PROG = 5, image_d, plan_d, key_root, machine_version)`.

Here `P2` is the ENC P-3 overwrite form and the codes are those of `PR` 3.6. `image_d` and `plan_d` are stand-ins (ENC-5 word lists open).

Enforcement (READ, `sherlock-lfr/src/lverify.rs` 296-299): the `L_Fr` tape takes `L`'s cached commitments, which are the child node's vk commitment, as prover input (`g.fr_in`), not as a constant. No circuit checks `key_root` inclusion, and nothing in `L`, `L_Fr` or the wrap pins `N3`. The keys agree: `c1/L_iface` equals the E6 `L` although the node key under it changed, and `N1` is equal for two programs. So a parent key does not pin its child key, and the universal keys do not enforce the app and leaf-layer G1 sets either. The `PR` 4.3 checks (the first universal node checks each child key against `key_root`; on the full tree `L` hardcodes the pair of the universal internal-recursive key) are not built. This section shows the encoding computed in Rust, not an enforced binding.

Results: `key_root` A `1439f56f...8c97`, `prog_d` A `00d09a07...86ba`. A second program (app with 2 AIRs) gives `key_root` B `2685ab6e...8af9` and `prog_d` B `4ad3a42c...d93d`. A changed per-program key changes `prog_d`, and recomputation is deterministic. The bench app has no preprocessed trace (0 preprocessed AIRs), so the binding is shown through the pre-hash only. A per-program instruction table (preprocessed) enters the same pre-hash. E6 R3 already measured the on-chain side, where a wrong `prog_d` is refused at the transcript read. The universal keys `N1` to `N3`, `L`, `L_Fr` and the wrap are the same for both programs (section 4), so only the per-program keys and `prog_d` change from program to program, as `PR` 4.3 says.

## 7. Ledger 9.1.0.1 (item 5; MEASURED, `out/ledger.log`)

Stock `midnight-ledger` rev a2b30128 (tag ledger-9.1.0.1), stock crates: `Ok(())` in 0.096 s on the FP-8 wrap key and proof. A changed public input is refused 3 of 3, and a flipped proof byte is refused 3 of 3. `git diff e6-int -- tools/sherlock-wrap/` is empty, so the wrap circuit and the ledger check (lockfile at a2b30128) are unchanged against E6.

## 8. Keys that changed against E6 (item 6)

First, E6 was reproduced. In this build the E6 chain (W1 at the stock app set -> node -> `L_iface`) regenerates the E6 node key and the E6 `L` pre-hash `1353...1140` byte-equal (MEASURED, `out/keys.json` "baseline"). Then one cause at a time:

| Key | E6 | FP-8 | Changed | Cause (MEASURED unless marked) |
|---|---|---|---|---|
| app (W1) | stock app set, query PoW 20 | frozen app set, query PoW 21 (vector 193/88/81/81 unchanged) | yes | G1: OD-4 query PoW 21 |
| leaf layer | none (E6 had no leaf layer) | new, leaf-aggregation set with LogUp PoW 17 | new | production topology app -> leaf layer -> internal |
| node | one node over W1 `[1549559576, ...]` | `N1` over the leaf layer, `N2`, `N3` (internal-recursive fixed point) | yes | (a) query PoW 21 of the app child changes a node over the app (`c1` keys); (b) the leaf-layer child changes the node's shape (1,543 to 1,549 columns); (c) the universal key is the depth-3 fixed point. The new bounds change no byte (section 2) |
| `L` | `1353...1140` | `3702...7945` | yes | its child is `N3`. `L` over a node depends only on the node's shape: the app PoW change alone kept `L` byte-equal (`c1/L_iface` = E6) |
| `L_Fr` | `4981...6066`, 15 AIRs, 561 columns, 1,522 words | `2526...0818`, 15 AIRs, 561 columns, 1,522 words, widths [33, 10] | yes | the tape pins the `L` key (pre-hash and constants) and the `L` heights; the shape is unchanged |
| wrap | `2fc75636...93dd`, 1,325 bytes | `4990db4d...a28d`, 1,325 bytes, same rows | yes | the relation pins the `L_Fr` key and the `L` pre-hash (DERIVED from E5/E6 3.1) |

The coordinator's "every node key changes" holds, but through G1 and the topology, not through the bounds. **FOR OWNER CONFIRMATION:** the devnet contract of E6 holds the old wrap key `2fc75636...`, so a deploy of the FP-8 set is a new contract key. No deploy was made.

## 9. What keeps FP-8 BLOCKED

| FZ-4 / 1.9 item | State | Evidence |
|---|---|---|
| GP-3 FV work-unit keys (X14) | open. Final shapes from X-EXP (agent message 2026-11-13, MEASURED there, arity-1 unit over one internal-set node, sink excluded): add unit 45 AIRs, 3,138 columns, at most 200 interactions; double unit 45 AIRs, 2,946 columns, at most 199. Against the G1 internal bounds (DERIVED): the double unit is inside 64 / 3,000 / 200; the add unit is over 3,000 columns, so `keygen_checked` refuses it unless the owner takes the X-EXP AFK pick of 3,300 columns for FV units (D3 at 3,300: weakest 101.1643, stacked 110.9393, X-EXP). The double unit needs an on-curve check by the point producer (no `INV_y`). No unit key was generated, and `fp8_keyset` has no unit mode (the unit AIRs live in the stark-bench tree, `~/.cache/stark-bench/src/openvm-org_openvm/crates/stark-bench-ovm/examples/x_exp_node.rs`). By the shape finding of section 4, a node over a unit node is a new key, not `N3`, so the set gains keys. Whether the chain returns to `N3`, and so to this `L`, is unmeasured. X-EXP measures the units with the condition rows at 3,138 and 3,189 columns, over 3,000. Either they fit under 3,000 or an AFK pick of 3,300 for FV units. The keys get generated when X-EXP sends the final shapes (coordinator note). The 17-bit unit range checker needs owner sign-off (departs from LD-S10x-3) | X-EXP RESULTS (d), uncommitted on `x-exp`; `x-exp` head at handback 77b735941, unchanged since the merge |
| The `L` interface on this branch | `fp8` carries the `e6-int` interface. E3 phase 2 (OnionAir, the j, k rule of 95ed47b, the D-9 SPONGE(3) lane map 099d6d8) is on other branches. Each changes `L` or `L_Fr` and so the wrap key. The key set is regenerated by `fp8_keyset chain` after those merge | wiki log 2026-11-13 03:57 and 04:07 |
| G3 transcript vectors | partial: Rust reference and AIR-side (ENC), wrap side (E5 I9); the GPU prover and stark-backend verifier legs of T-FS are not on record as a set | ENC RESULTS 6, E5 6j |
| G4 Lean | `lfrUD_gate` exists (bridge-lean `UDBits.lean:306`); `decide` on the concrete keys of this G1 freeze is not run | bridge-lean ecbce035f |
| I-17 (path C `h_pre`) | not run | none found |
| `key_root` and child-key enforcement (`PR` 4.3) | not built: `L` takes the child node commitment as prover input, so no key pins `N3`, the leaf-layer key or the app key (section 6) | `lverify.rs` 296-299 (READ); `keys.json` (MEASURED) |
| Per-call prover on the FP-8 keys | `lfr_call` still builds the E6 topology with raw keygen (section 2) | READ |
| D10 on path C | not measured (D10 measured the full-tree `L`: `w_stack` 18, matched here: 17 + 1) | d10-heights 3b979bd5c |
| FZ-3 production qualification | not qualified: no frontend run, no J6 replay (E6 section 2) | E6 RESULTS |

## 10. Owner confirmations (FOR OWNER CONFIRMATION)

1. G1 is enforced in `sherlock_lfr::frozen::keygen_checked`, not in stark-backend keygen. No bound value is written into a key.
2. The node arity of the universal set is 2, with the depth-3 fixed point `N3` as the internal-recursive key. Padding repeats a child proof.
3. The `key_root` leaf and node encodings of section 6.
4. The FP-8 wrap key replaces the E6 devnet key at the next deploy.

## 11. Gemini Quint lane

None is due. No AIR or wrap circuit source changed: the new keys come from the unchanged recursion, `L_iface`, `L_Fr` and wrap code under other parameters and another topology. `frozen.rs` is a keygen admission check, not a circuit. The FV work-unit AIRs are new circuits, and they have their X-EXP lanes.

## 12. Merge note

The merge of `x-exp` into `fp8` (5d9a2d705) had 4 conflicts in `FD/ARCHITECTURE.md`. The resolution kept the `e6-int` text "Facts 1 to 3: closed" (FP-3 done in enc-fp3) and the `x-exp` (final-design-2) text of rows I-27, I-28, L AND WRAP, GP-2, GP-3 and LEDGER CONTRACT. It removed from those rows the FREEZE-PENDING clauses on FP-3 facts 1 to 3, because the facts are closed. The DOC lane owns that file and should check this resolution.

## 13. Files

- Moriarty `fp8`: this folder; `out/` holds `keys.json`, `fixpoint.json`, `chain.json`, `d3.json` and the logs; `src/` holds copies of the openvm sources and `run-chain.sh`.
- openvm `~/.cache/m7-wrap/src/openvm-fp8`, branch `fp8` (local, not pushed): `crates/sherlock-lfr/src/frozen.rs`, `tests/g1_frozen.rs`, `fixture.rs` and `testing.rs` (delegate to `frozen`), `crates/sherlock-wrap-export/examples/fp8_keyset.rs` (modes `keys`, `chain`, `fixpoint`), `examples/fp8_d3.rs`; `Cargo.toml` patch path to `../stark-backend-fp8`.
- Outputs: `~/.cache/fp8/out/chain/` (vk bitcode files, `lfr-fp8.json`), `~/.cache/fp8/out/wrap/` (`wrap.vk`, `wrap.proof`, `wrap.public.json`), logs `~/.cache/fp8/logs/`.

## 14. Proposed ARCHITECTURE rows (for the DOC lane; not edited here)

- Section 1.9 internal row, column "Shape bounds": "height at most 2^19; 1,000 constraints and 200 interactions per AIR, 64 AIRs, 3,000 columns (owner rulings 2026-11-12 20:35); D3 101.1643 bits at these bounds and at the FP-8 keys".
- Section 1.12 key set: "Internal-recursive key: `N3<2>`, the depth-3 fixed point of the node over node at the internal set (`N4 = N3`), 39 AIRs, 1,549 columns, 81,541 bytes, pre-hash `[1083965961, ...]`; `N1` and `N2` below it; equal for programs of the same leaf-layer shape (FP-8, MEASURED)". "`L` key: `L_iface` over `N3`, pre-hash `3702...7945`, 43 AIRs, 1,805 columns". "`L_Fr` key: pre-hash `2526...0818`, 15 AIRs, 561 columns". "Wrap key: 1,325-byte `MidnightVK` sha256 `4990db4d...a28d`, 1,605,413 rows at k = 21, ledger 9.1.0.1 `Ok`". Status of each: FREEZE-PENDING the items of section 9.
- G1 row of the qualification gate: "the sets and bounds are fixed in `sherlock_lfr::frozen`, and keygen refuses another parameter set or a key outside the bounds (openvm `fp8` 9d4105f, 4 tests)".

