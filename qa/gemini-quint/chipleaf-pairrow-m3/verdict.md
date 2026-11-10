# Verdict: layout B pair rows and M3 aligned-segment memory root

Lead verdict on the Gemini Quint lane of 2026-11-10 for two parts of the chip leaf design
(`research/sherlock-processor-analogy-20261110/experiments-2.md` §2 and §3.4 to §3.6, `crates/mori-chip/src/rf.rs`,
`crates/mori-chip/src/rfcore.rs`, `openspec/changes/sherlock-vm-consensus-20261107/design.md` §2.7).

Lane: `argon-run`, `gemini-4-argon-high`, conversation `f6fd563f-cefc-4f51-8a37-f1ad7dd7ded8`, 4 turns, `DONE`.
It ran in a detached worktree under `~/.cache/gemini-lanes/chipleaf-pairrow-m3` inside `heavy.slice`.
Task: `TASK.md`. Raw log: `lane.log`. Lane report: `REPORT.md`. It changed only its five output files.
The lane could not run `quint` (its sandbox refused the command), so every result below comes from the lead's re-run.

## 1. Verdicts

| Part | Verdict | Condition |
|---|---|---|
| 1. Layout B pair rows | SOUND | with C1 below. The spike `rfcore.rs` already meets C1. |
| 2. M3 aligned segment, C = 8 | SOUND only with C2 to C6 | not sound as written against `rf.rs` (`NIL = 0`, genesis `len = 1`) and the current D3 text |

## 2. Required design constraints

- **C1 (pair rows, scratch registers).** A register that the leaf digest leaves out (`v0..v2`, words 29..31, `NDIG = 29`) is write-only: no lane and no digest-covered register depends on `Pre(v_k)`. Its only use is its own carry in a disabled slot (`off * p`). Otherwise include it in the digest. Slot 0's `pre` of an undigested word is free, so at the start of every leaf after the first the prover chooses it (`cex_P9_scratch_read`). In `rfcore.rs`, `next_state` uses `Pre(V0..V2)` only in the `V0..V2` lines themselves, and no lane constraint reads them, so the spike meets C1. A future rule set must keep it.
- **C2 (M3, no honest record is the zero tuple).** "A read SHALL refuse the zero tuple" is complete only if no honest record is the zero tuple. That includes frame and task spills.
  - With `NIL = 0` (`rf.rs`), D7's depth-0 sentinel spill `(top = 0, fp = NIL)` is the zero tuple, so every honest POP at depth 1 is refused (`w_d7_sentinel_nil_zero`).
  - An untagged all-zero data record fails the same way (`w_zero_record_untagged`).
  - `NIL != 0` alone is not enough once genesis starts at `len = 0` (C3). `PUSH 0` sets `fp := 0`, and the next PUSH spills `(top = 0, fp = 0)`, the zero tuple, at address 1, so the POP that loads it is refused (`w_push_zero_at_addr0` in `m3seg_test.qnt`, lead addition, with NIL = 99).
  - Fix: a non-zero tag word in every written record, frame and task spills included. The other way is never to allocate address 0 and keep `NIL != 0`, but that conflicts with C3's genesis `len = 0` unless the whole first segment is reserved.
  - This is a completeness failure, not a soundness failure, but M3 cannot be adopted without it.
- **C3 (M3, aligned base and genesis).** `base = pre_d.len` must be a multiple of C, as a circuit constraint (for example `base = C * seg` with `seg` range-checked). Genesis must start at an aligned `len`: 0, or C with the first segment reserved. `rf.rs`'s `len = 1` is not aligned. An `Init` with input rows must pad `len` to a multiple of C. If the base is not aligned, MEM row `k` is hashed at segment offset `k`, but PAGE opens `base + k` at offset `(base + k) mod C`, so a later leaf reads the wrong cell (`cex_M5_base_unaligned`).
- **C4 (M3, D3 text).** D3 as written ("hashes MEM rows `0 .. post.len - base - 1`") marks all C rows active once `post.len = base + C`, which turns off the padding check (`cex_M1_padding_nonzero`). D3 must state:
  - the active prefix is `used = len_end - base`, where `len_end` is the running `len` after the last slot;
  - rows `used .. C - 1` are the zero tuple;
  - the segment hash covers all C rows;
  - leaf close sets `post_d.len := base + C`.
  The replacement text in `experiments-2.md` §4 already says this. It is not yet in `design.md`.
- **C5 (M3, the rest of the clause).** Each of the following is load-bearing. Each has a mutant that is caught:
  - `len` advances by exactly 1 per append (M2);
  - the read refuses the zero tuple (M3);
  - leaf close sets `post_d.len = base + C`, not the running `len` (M4);
  - the root update authenticates the empty slot at `base / C` against `pre_d.mem_root` and reuses its siblings (M6).

- **C6 (M3, partition-dependent addresses).** A leaf may close early; that is the prover's choice (design 2.7.7). Under M3 every leaf allocates C addresses, a zero-append leaf too (`hw_part_2_2`, `hw_part_4_0`). So `len`, `fp`, every stored link and `mem_root` in `post_d` depend on where the prover closed the leaves. After the same two PUSHes, `fp = 4` under one partition (`hw_part_1_3`) and `fp = 1` under another. The model's reference uses the same per-leaf allocation as the circuit, so it does not catch this. Part 2 is sound only if one of these holds:
  - no program-visible value and no published statement depends on raw addresses (addresses are opaque handles: compared only for equality, never printed, hashed into an outcome or used in arithmetic), or
  - the claim the leaf chain proves names the partition (the reference is run with the same partition).
  A literal address in a program makes the partition decide acceptance. Program 1 loads address 1, which under partition [1, 3] is a padding cell, so the sound model refuses that run but accepts the same program under [2, 2] (`validRunsTest`, lead addition). The Sherlock VM must state which option it takes.
- **Cross-part note (P7 and M4).** P7 requires `post_d` to digest slot S's pre cells. M4 requires `post_d.len = base + C`, while slot S's `len` is the running `base + used`. So LEN is the one digest input that is not a copy of a slot-S cell. It must be a separate close cell, computed as `base + C`, and the digest must take that cell.

D2 under M3: the model accepts PAGE without `addr < base` when C2 to C5 hold (`m3seg_M7_alone`, exit 0). Every segment at or above `base / C` is empty in `pre_d.mem_root`, so it opens only to the zero tuple, which a read refuses. Dropping both `addr < base` and the zero refusal is caught (`cex_M7_page_serves_segment`: an intra-leaf read is served a stale zero). Keep D2. It costs nothing and keeps PAGE and MEM on disjoint address ranges.

Observations that change no verdict:
- **Enable prefix.** Dropping the prefix leaves register refinement intact (`pairrow_enable_not_prefix`, exit 0). A disabled slot is the identity, because the frame gate is unconditional and the copy links are unconditional. The prefix matters for publication (the terminal step is the last enabled slot) and for slot-position commitments, not for the register file.
- **Address space.** At C = 8 and depth 20 there are 2^17 segments, one per leaf (`experiments-2.md` §3.5).

## 3. Attack questions and the mutants that answer them

| Question | Mutant / witness | Re-run exit | Result |
|---|---|---|---|
| Change a register the rule does not name | P1 `frame_gate_dropped`; P4 `flag_replica_unlinked` | 1, 1 | caught |
| Break the chain between slots | P3 `chain_link_dropped` | 1 | caught |
| Exploit disabled slots | P2 `frame_gate_enabled_only` | 1 | caught |
| Leaf boundary: post_d from the wrong slot | P7 `post_d_wrong_slot` | 1 | caught |
| Leaf boundary: a read register left out of the digest | P8 `digest_omits_register` | 1 | caught |
| Leaf boundary: a rule reads an undigested scratch register | P9 `scratch_read` | 1 | caught; constraint C1 |
| Replica / routed operand cells | P5 `lane_replica_unlinked`, P6 `routed_operand_unlinked` | 1, 1 | caught |
| Enable not a prefix | `enable_not_prefix` | 0 | not caught; not load-bearing for registers (see above) |
| Append a cell not requested / non-zero padding | M1 `padding_nonzero` | 1 | caught |
| Skip an append | M2 `append_skips` | 1 | caught |
| Read a padding cell as data | M3 `padding_read` | 1 | caught |
| Desync `len` and the root | M4 `close_len_running`, M5 `base_unaligned`, M6 `no_empty_slot_check` | 1, 1, 1 | caught; C3, C5 |
| D2 interaction | M7 `page_serves_segment` (with zero refusal off) / `M7_alone` | 1 / 0 | caught only together with the zero refusal off |
| Completeness: zero-tuple records | `w_d7_sentinel_nil_zero`, `w_zero_record_untagged`, `w_push_zero_at_addr0` | tests pass (each run refused, as asserted) | finding; C2 |
| Partition decides addresses / acceptance | `hw_part_1_3` vs `hw_part_2_2`; `validRunsTest` | tests pass | finding; C6 |

## 4. Lead re-run

All commands ran from `qa/gemini-quint/chipleaf-pairrow-m3/` as `heavy-run --mem 4G -- quint ...`, with `quint` 0.32.0. Script: `rerun.sh`. Output: `rerun-results.txt`. Run options: `--max-samples 4000 --max-steps 3 --seed 0x20261110`.

| Command | Exit | Tag |
|---|---|---|
| `quint typecheck` on `pairrow.qnt`, `pairrow_test.qnt`, `m3seg.qnt`, `m3seg_test.qnt` | 0 each | |
| `quint test --main pairrow_test pairrow_test.qnt` | 0 | 16 passing |
| `quint test --main m3segTest m3seg_test.qnt` | 0 | 18 passing |
| `quint run pairrow.qnt --main pairrowSound --invariant invRefine` | 0 | [ok] |
| `quint run pairrow.qnt --main pairrowSound --invariant reachAcceptedBadTamper` | 0 | [ok] |
| `quint run pairrow.qnt --main pairrowSound --invariant reachHonestFullChain` | 1 | [violation] (non-vacuity) |
| `quint run pairrow.qnt --main pairrowSound --invariant reachHonestDisabledSlot` | 1 | [violation] (non-vacuity) |
| `quint run pairrow.qnt --main pairrow_P1_frame_gate_dropped .. pairrow_P9_scratch_read --invariant invRefine` (9 runs) | 1 each | [violation] each |
| `quint run pairrow.qnt --main pairrow_enable_not_prefix --invariant invRefine` | 0 | [ok] |
| `quint run m3seg.qnt --main m3segSound --invariant invRefine` | 0 | [ok] |
| `quint run m3seg.qnt --main m3segSound --invariant reachAcceptedBadTamper` | 0 | [ok] |
| `quint run m3seg.qnt --main m3segSound --invariant reachHonestWithPadding` | 1 | [violation] (non-vacuity) |
| `quint run m3seg.qnt --main m3segSound --invariant reachCrossLeafPageRead` | 1 | [violation] (non-vacuity) |
| `quint run m3seg.qnt --main m3segSoundC8 --invariant invRefine` | 0 | [ok] |
| `quint run m3seg.qnt --main m3seg_M1_padding_nonzero .. m3seg_M7_page_serves_segment --invariant invRefine` (7 runs) | 1 each | [violation] each |
| `quint run m3seg.qnt --main m3seg_M7_alone --invariant invRefine` | 0 | [ok] |

Every exit 1 above has a `[violation]` line. In every mutant trace the last state has `acc: true` and `isTampered: true`, so an accepted witness that differs from the honest one breaks the invariant. Every exit 0 has `[ok]`.

The `--main` names and the invariant names in the lane's `REPORT.md` §4 are partly wrong (`reachHonestFull`, `reachHonestPartial`). The names above are the ones in the files.

### Lead repairs before the models ran

- Syntax and types:
  - renamed the parameter `enabled` (a QNT101 built-in) to `slotOn` in both models;
  - renamed the record field `val` (a keyword) to `cv` in `m3seg.qnt` and `m3seg_test.qnt`;
  - changed `List.forall` to `indices().forall`;
  - changed `Set.foldl` over `0.to(n - 1)` to `range(0, n).foldl`, which also fixes the order of appended lists;
  - made a zero-argument `def` a `val`.
- `pairrow.qnt`, two changes to the refinement check:
  - **P9 was vacuous.** The circuit's scratch-reading rule was compared with a reference that did not read `v0`, so the honest run already failed refinement. The reference now takes the variant (`refStepOne(mt, ...)`, `refinesWith(mt, w)`), so only the forged leaf-2 `v0` breaks refinement.
  - **P8 was caught by a format mismatch.** `post_d` was compared in the full digest format, so even the honest chain failed. It is now compared in the variant's format.
- `pairrow_test.qnt`:
  - added `honestUnderMutantsRefinesTest`: for every mutant except P7, the honest chain under the mutant is accepted and refines;
  - the P9 test now also asserts that the honest chain refines.
- `m3seg_test.qnt`:
  - added `w_push_zero_at_addr0` and `completenessFailurePushZeroAtAddr0Test` (C2);
  - added `VALID_RUNS` and `validRunsTest` (C6);
  - added `honestUnderMutantsRefinesTest`: for M1, M3, M5, M6, M7 and M7-alone, the honest chain under the mutant is accepted and refines on every run in `VALID_RUNS`. M2 and M4 are left out, because their honest runs already diverge.

The commands total 33: 4 typechecks, 2 test suites (16 + 18 tests) and 27 `quint run`s. 20 exit 1 and 13 exit 0, each as expected.

None of these changes weakens a constraint of the sound model. No file under `models/` was changed.

## 5. Limits

- **Small instances.** The models use 8 registers in 3 pair rows (a 7-row slot, not the 13-row slot), 4 rules, S = 3 or 4, C = 4 (and one C = 8 run), and 4 segments. Hashes and Merkle trees are exact maps (collision-free premise). The bus is set inclusion.
- **Tamper coverage.**
  - Part 1: any one `pre` or `post` register cell, flag replica, lane replica or routed cell, with downstream recomputation, over values {0, 3, 7}. The pure test checks this whole set; `quint run` samples it.
  - Part 2: mostly targeted. Only the padding/hole row tamper (kind 1) varies over rows and values; kinds 2 to 6 are fixed attacks. A pass of `m3segSound` says nothing about tampers outside these kinds.
  - Nothing here proves soundness for every witness.
- **Part 2 reference.** The reference memory uses the same aligned per-leaf allocation as the circuit (`base2 = base1 + C`). It checks write-once contents, read values and `len`/root sync. It does not check that M3's addressing is the right one.
- **M2.** The honest run under `append_skips` already diverges: the reference writes at consecutive addresses. M2 is a wrong design under any witness, and it is also caught by the forged hole cell.
- **Other modelling choices.** The task stack, PAGE for PROG rows, HASH rows and the Init table's four `(root, length)` pairs are not modelled.
