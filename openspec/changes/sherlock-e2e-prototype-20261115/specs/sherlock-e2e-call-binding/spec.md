## ADDED Requirements

### Requirement: CLB-001 The interface has 5 lane pairs and 12,166 public words
CLB-001: The `L` interface SHALL expose 12,166 public words with 5 lane-map pairs.

Verification: test `layout_full_is_12166`; gate `iface-fixture` (Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/run.sh:38`). Exists.

#### Scenario: The layout after the rekey
- **GIVEN** the T25 interface
- **WHEN** `layout_full_is_12166` runs
- **THEN** the count is 12,166 or the record names the new count and the change of `~/.cache/rekey-t11/LIST.md` that caused it

#### Scenario: Status of CLB-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on the T10m keys. FP-3 and slot 15 change the interface; the word count after T25 is not measured. Owner lane: T25. Evidence: openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/iface.rs:42` (`LANE_PAIRS = 5`), test `layout_full_is_12166` (openvm `t10mm` 5b7ada8 `iface.rs:1455`). Owner ruling O7 (LOOP, rulings table). Moriarty 3dc9001a9 `research/sherlock-t10m-20261114/RESULTS.md:41`.

### Requirement: CLB-002 A lane word that the guest did not write is refused
CLB-002: If a call-vector effect lane holds a word that the guest did not write at that lane, then the `L` effect AIR SHALL refuse the witness.

Verification: tests `nt1_residual_shifted_write_is_fp3` and `fp3_bindings_refuse`, failing before and passing after the FP-3 commit. Exists.

#### Scenario: A shifted write
- **GIVEN** the NT-1 residual witness with the anchor write moved by one lane
- **WHEN** `L` is checked
- **THEN** the effect AIR refuses the witness

#### Scenario: Status of CLB-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN: not committed. The t10q push waits on it (LOOP T10q). Owner lane: fp3. Evidence: Refusal side of ANC-018, which stays the normative text for the binding. openvm `fp3` d2abc38 tree: `crates/sherlock-lfr/src/fp3.rs` is untracked (616 lines; doc at `fp3.rs:1` to `:22`). Test `nt1_residual_shifted_write_is_fp3` is committed at openvm `fp3` d2abc38 `crates/sherlock-leaf/tests/t13_t10q_chunk.rs:639` and fails there; in the dirty tree it is at `:759` and `fp3_bindings_refuse` is at `:854`. Logs `~/.cache/fp3/logs/before3.log` (fail) and `after2.log` (12 of 12 pass). Paths corrected after Astra-B review R-12.

### Requirement: CLB-003 A wrong chunk count is refused
CLB-003: If a chunked run states a chunk count `k` that differs from its slice count, then the `L` effect AIR SHALL refuse the witness.

Verification: test `fp3_k_is_the_true_slice_count`, with an assert on the refusal of a run whose `k` is 1 above the slice count. Exists.

#### Scenario: k above the slice count
- **GIVEN** a run of 2 slices that states `k` = 3
- **WHEN** `L` is checked
- **THEN** the forward chain constraint fails

#### Scenario: Status of CLB-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: OPEN. GAP: the pass of the test does not show a refusal. Owner lane: fp3, with a Carbon Quint lane (LPC-004). Evidence: Refusal side of ANC-019, which stays the normative text for the binding. openvm `fp3` d2abc38 `crates/sherlock-leaf/tests/t13_t10q_chunk.rs:651` (`fp3_k_is_the_true_slice_count`; `:781` in the dirty tree). The forward chain is `C_j = vd(Cons(rec_j, C_(j+1)))` (`fp3.rs:1` to `:22`, untracked). In `~/.cache/fp3/logs/after2.log` this test prints "tape accepts" and "L effect AIR accepts" and shows no refusal.

### Requirement: CLB-004 The anchor is at slot 15
CLB-004: The `L_Fr` interface SHALL use the ledger slot constant `ANCHOR_SLOT` = 15 for the anchor read of every call and for the anchor write of a chunk call.

Verification: test `t12_anchor_slot_15` on the T25 tree; ANC-013 tests on the wrap side. Exists.

#### Scenario: A deployed contract with user state
- **GIVEN** a Compact contract whose slot 0 holds user data
- **WHEN** an ordinary call is proved
- **THEN** the interface reads the anchor at slot 15, writes no anchor, and slot 0 stays a user word

#### Scenario: Status of CLB-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on branch `t12-slot15`, not merged. Owner lane: t12-slot15. Evidence: openvm `t12-slot15` 4ea3d05 `crates/sherlock-lfr/src/iface.rs:112` (`ANCHOR_SLOT: u64 = 15`), read prologue at `iface.rs:123`, chunk-only write template at `iface.rs:129`, test `t12_anchor_slot_15` (`iface.rs:1559`). openvm `t10mm` 5b7ada8 has no constant: the slot is the literal `(6, c(0))` at openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/iface.rs:115`. openvm `t10q-ntfix` e3b407d is not an ancestor of openvm `t12-slot15` 4ea3d05, so the two changes need a merge (UKS-009). An ordinary call writes no anchor (CLB-006, ANC-016). Reworded after Astra-B review R-11.

### Requirement: CLB-005 Locks with another run tag, k or link are refused
CLB-005: If a chunk carries a lock whose operand `x_j` differs from `sponge(U_LOCK; c, pack(run_tag), k, d_in, d_out, pack(C_(j+1)), j)` of its run, with `C_(j+1)` the forward claim chain of the run after chunk `j` (ANC-019), then the `L_Fr` circuit SHALL refuse the chunk.

Verification: tests `t10q_nt2_equal_run_tag_splice_refused` (openvm `t10q-ntfix` e3b407d `crates/sherlock-leaf/tests/t13_t10q_chunk.rs:663`) and `t10q_nt3_k_change_refused` (`:670`). Exists.

#### Scenario: Two runs with one run tag
- **GIVEN** chunks of two runs with equal `run_tag`
- **WHEN** a prover splices a chunk of run 2 into run 1
- **THEN** the lock check fails

#### Scenario: Status of CLB-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on `t10q-ntfix` (before 3 of 3 fail, after 9 of 9 pass; `~/Projects/Moriarty/llm-wiki/wiki/log.md` entry 06:58). Not merged into `t10m`. Evidence: Refusal side of ANC-017, which stays the normative text for the operand. openvm `t10q-ntfix` e3b407d `crates/sherlock-lfr/src/iface.rs:277` (`U_LOCK = 8`), `iface.rs:284` (`cap_lock`), `iface.rs:302` (`lock_x`). Owner ruling 2026-11-14: "x_j lock form is canon" (LOOP, "Owner rulings 2026-11-14"). FP-3 adds the link operand `pack(C_(j+1))` before `j` (D-038, OWNER 2026-11-15): openvm `rekey-int` 3826e38 `crates/sherlock-lfr/src/iface.rs:290` (`LOCK_LEN = 7`), `iface.rs:316` (`lock_x`). F-B-BR-2 (LEAD PICK for T15): the text now names the 7 operands of the code. On rekey-int 3826e38 a chunk with the 6-operand lock of the old text is refused (tape asserts #5 and #6), and a lock with the link fixed to a constant lets a 3-slice run close as 2 chunks (`fp3_k_is_the_true_slice_count` fails); `formal/sherlock-e2e/bridge/cx/`. openvm `rekey-chips` dcf1b4c still has the short lock (`iface.rs:289`).

### Requirement: CLB-006 An ordinary call with an anchor write is refused on the interface
CLB-006: If an ordinary call (mode 0) carries the anchor-write template, then the interface decoder `iface::decode_with` SHALL refuse the call.

Verification: test `t10q_nt1_ordinary_call_anchor_write_refused` on the T25 tree. Exists.

#### Scenario: Mode 0 with the template
- **GIVEN** an ordinary call with the write template in lanes 11 to 19 and 21
- **WHEN** the interface decodes the call
- **THEN** the decoder refuses the call

#### Scenario: Status of CLB-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT on `t10q-ntfix`. Not merged into `t10m`. Evidence: Interface twin of ANC-016 (the wrap side). openvm `t10q-ntfix` e3b407d `crates/sherlock-leaf/tests/t13_t10q_chunk.rs:656`. `~/.cache/rekey-t11/LIST.md:6`.

### Requirement: CLB-007 More than 2 chunks is refused
CLB-007: If a schedule cuts one call into more than `K_MAX` = 2 chunks, then the chunk planner SHALL refuse the call before a proof starts.

Verification: `quint test` on `chunkCap.qnt` with the mutant `cap_four`. NEW: a Rust test of the planner refusal. Partly to build.

#### Scenario: A call that needs 3 chunks
- **GIVEN** a call whose effects fill 3 chunks at the measured `L_TX`
- **WHEN** the planner makes the schedule
- **THEN** the planner refuses the call and names `K_MAX`

#### Scenario: Status of CLB-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP: the cap is a model and a spec rule (ANC-012), not code. Owner lane: t20-batched merge (LPC-010), then t23-suite. Evidence: Owner ruling 2026-11-14: "T20 chunk cap K_MAX = 2" (LOOP, "Owner rulings 2026-11-14"). Quint: Moriarty `t20-batched` 5dc1be2ee `models/sherlock/quint/t20-batched/chunkCap.qnt:26`. No Rust constant `K_MAX` exists in any openvm lane.

### Requirement: CLB-008 One anchor slot constant in every component
CLB-008: If the anchor slot constant of the `L_Fr` interface differs from the anchor slot constant of the wrap decoder, then the rekey runner SHALL refuse to start.

Verification: NEW: a check script in the T25 runner that reads the three constants. To build.

#### Scenario: The wrap still reads slot 0
- **GIVEN** an interface with slot 15 and a decoder with slot 0
- **WHEN** the rekey runner starts
- **THEN** the runner stops and names both values

#### Scenario: Status of CLB-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP. Owner lane: T25. Evidence: openvm `t12-slot15` 4ea3d05 `crates/sherlock-lfr/src/iface.rs:112`; Moriarty `t12-slot15` 40a86bea5 `tools/sherlock-wrap/src/decode.rs:70` (dirty) and `tools/sherlock-sdk/anchor-slot.mjs:14` (untracked). Three copies of one constant.
