# Verdict: Poseidon-only HASH chip and its bus use

This is the lead verdict on the Gemini Quint lane of 2026-11-10 for the Poseidon-only HASH chip:
- `crates/mori-chip/src/poseidon.rs`, and its use through `bus.rs` and `rfcore.rs`;
- design and evidence: `research/sherlock-processor-analogy-20261110/experiments-3.md` §2 and §4;
- constraints D1, D4 and D8 of `openspec/changes/sherlock-vm-consensus-20261107/design.md` §2.7.8.

Lane details:
- Driver and model: `argon-run` with `gemini-4-argon-high`, conversation `1e8de681-f46e-466d-8523-83efb781b366`, 2 turns. The driver reported `DONE`. The second turn hit the output-token limit, but `REPORT.md` was already written.
- Where it ran: a detached worktree under `~/.cache/gemini-lanes/chipleaf-hash`, inside `heavy.slice`.
- Inputs and outputs: the task is `TASK.md` and the raw log is `lane.log`. The lane changed only its three output files: `hashchip.qnt`, `hashchip_test.qnt` and `REPORT.md`.
- The lane's sandbox refused `quint`, so the lane ran no command. Every result below comes from the lead's re-run.
- The lane's unrepaired models are kept in `lane-original/`.

## 1. Verdict

**SOUND**, with the conditions in §2. In the model, no accepted witness gives a bus HASH request an output other than the sponge of exactly its inputs. The same holds for an in-region sponge's returned cell. This needs the conditions in §2. The current `poseidon.rs` meets C1 to C4. C5 and C6 apply to code that is not built yet.

The model abstracts the permutation as a bijection of Z_p^3 (p = 1000003), with 2 round rows per permutation. Absorb is real addition. The model covers gates, selectors, the fixed capacity, copy constraints, both bus lanes, a second chip (ALU) and D8 padding.

## 2. Required constraints

- **C1 (layout asserts are load-bearing).** `sponge` asserts `n >= 2` and `!provide || n == 2`. These are synthesis-time asserts, not gates. Keygen runs them, so no key made by this code violates them. Keep both:
  - Without `n >= 2`, a 1-input sponge uses `q_first`, which adds the free, uncopied `in1` (H12 `one_input_sponge`, caught).
  - If `q_prov` is set on a chain row, or on a provider sponge of more than 2 inputs, the provider tuple is not pinned and is not a function of `(in0, in1)` (H11 `prov_on_chain_row`, caught).
  - `n = 0` cannot be reached: the assert fires first, and without it `last.expect` panics at synthesis.
- **C2 (capacity is fixed).** `len = n` is folded into the fixed `rc[2]` of the first absorb row only. If it is advice (H5) or the wrong value (H6, chunk count or on chain rows), an accepted output is not the reference sponge.
- **C3 (absorb and round gates).** The following must all hold:
  - `q_first` adds both inputs (H1);
  - `q_chain` adds both inputs (H2) and the previous output row (H4);
  - `q_tail` adds `in0` only (H3: the tail's `in1` is free padding);
  - every round row has its selector (H13).
  Each mutant is caught.
- **C4 (provider row).** The `q_first * q_prov` gate pins `chip = HASH` (H8) and `key = 2` (H7). The provider's `out` is copied from element 0 of the final output row (H9 not copied, H10 non-final row). Each mutant is caught.
- **C5 (D4 and D8: advice chip tag, gate-pinned).** D4 says the provider `chip` field is the region's fixed column. In `poseidon.rs` (as in `alu.rs` and `mem.rs`), `chip` is advice that a gate pins. This is sound only while every row with `q_prov = 1` also has a gate that pins its own nonzero chip id, and every other row has `q_prov = 0`:
  - The table side must stay `q_prov * tuple`. If it is not scaled, the free cells of a padding or round row become provider tuples (H14, caught).
  - Today every `q_prov = 1` row meets this rule:
    - HASH: `q_first`, on a 2-input provider only.
    - ALU: the gate pins `chip` at `alu.rs:117`. `q_alu` is enabled on the same row as each `provide` call (`alu.rs:143-144`, `alu.rs:160-162`).
    - MEM: the gate pins `chip` at `mem.rs:43`. `q_row` is enabled on every row that `provide` writes (`mem.rs:76`, `mem.rs:84`).
    - The provider rows in `cpu.rs` are test-only (`#[cfg(test)]`).
  - If another provider did not pin its own chip id, a HASH request could receive that chip's output. The lead witness `cex_H8_hash_direction` (`mutantH8_hashDirectionTest`) shows this: ALU row 44 is relabelled HASH and requested as `(HASH, 2, 1, 2, aluOut)`. The witness is accepted under `chip_unpinned`, refused by the sound ALU pin, and does not refine.
  - The E6 provider rows (`params.hash`, `hash_tuple`, `rfcore.rs:1156-1162`) pin neither chip nor key (experiments-3 §2.1). They are not built when `pos_hash` is set, because that block returns first. They must not be combined with the Poseidon chip, or must be removed.
  - Either D4's text changes to "gate-pinned advice on every `q_prov` row", or the chip tag moves to a fixed column.
- **C6 (wiring, not built yet).**
  - Requester: the core has no HASH requester yet (experiments-3 §4), so the model assumes one. An active HASH request must gate `chip = HASH`. Pinning `key = 2` on the requester is not needed for soundness: the model leaves the request key free, and `invRefine` holds, because no provider row has `chip = HASH` with a key other than 2. Pin it anyway for clarity.
  - In-region sponges: `rfcore.rs` absorbs free `HIn::Val` values and discards the returned cell. When M3 and the state digest are wired, the inputs must be `HIn::Cell` copies and the returned final-row cell must be copy-constrained to its consumer. Otherwise the in-region result binds nothing (lane F4).

Provider reuse: one provider row serving several requests with the same `(in0, in1)` is honest under set inclusion and is accepted (`hw_provider_reused`, `providerReuseAcceptedTest`).

## 3. Attack questions and the mutants that answer them

| Question | Mutant (`--main hash_<name>`) | Re-run exit | Result |
|---|---|---|---|
| Skipped absorb | H1 `skip_absorb_first`, H2 `skip_absorb_chain` | 1, 1 | caught |
| Reordered / restarted chain | H4 `chain_no_prev` | 1 | caught |
| Wrong padding (tail absorbs free `in1`) | H3 `tail_absorbs_in1` | 1 | caught |
| Capacity chosen by the prover | H5 `capacity_advice` | 1 | caught |
| Wrong length in the capacity | H6 `len_wrong` | 1 | caught |
| Provider row with a different key | H7 `key_unpinned` | 1 | caught |
| Provider row with a different chip tag | H8 `chip_unpinned` (HASH row served as ALU; lead test `cex_H8_hash_direction`: ALU row served as HASH) | 1 | caught |
| Output not bound to the permutation | H9 `out_not_copied` | 1 | caught |
| Output read from a non-final row / sponge ends early | H10 `out_from_nonfinal` | 1 | caught |
| Provider row reused for a different request (provider on a chain row) | H11 `prov_on_chain_row` | 1 | caught; C1 |
| One-input sponge | H12 `one_input_sponge` | 1 | caught; C1 |
| A round skipped | H13 `round_gate_dropped` | 1 | caught |
| Non-provider row on the table side (D8) | H14 `prov_zero_not_zero` | 1 | caught; C5 |
| Zero-input sponge | none (synthesis panics) | none | unreachable; C1 |

How each mutant is caught:
- **H3, H5, H7, H8, H9, H11, H12, H13 and H14: by a prover attack.** The honest witness under the mutant is accepted and refines (`honestUnderMutantsRefinesTest`), so each catch comes from a tampered witness.
- **H1, H2, H4, H6 and H10: by a gate or layout that computes the wrong function.** These change the honest computation itself. The witness the mutant's own gates produce is accepted and does not refine (`gateOrLayoutMutantsDivergeTest`).

## 4. Lead re-run

All commands ran from `qa/gemini-quint/chipleaf-hash/` as `heavy-run --mem 4G -- quint ...`, with quint 0.32.0:
- script: `rerun.sh`;
- results after the repairs: `rerun-results.txt`;
- the first run, on the lane's unrepaired models: `rerun-results-unrepaired.txt`;
- `quint run` options: `--max-samples 4000 --max-steps 3 --seed 0x20261110`.

| Command | Exit | Tag |
|---|---|---|
| `quint typecheck hashchip.qnt`, `hashchip_test.qnt` | 0, 0 | |
| `quint test --main hashchip_test hashchip_test.qnt` | 0 | 22 passing (includes the exhaustive single-cell, request-tuple and per-mutant folds) |
| `quint run hashchip.qnt --main hash_sound --invariant invRefine` | 0 | [ok] |
| `quint run hashchip.qnt --main hash_sound --invariant reachAcceptedBadTamper` | 0 | [ok] |
| `quint run hashchip.qnt --main hash_sound --invariant reachHashRequestServed` | 1 | [violation] (non-vacuity) |
| `quint run hashchip.qnt --main hash_sound --invariant reachTailChunk` | 1 | [violation] (non-vacuity) |
| `quint run hashchip.qnt --main hash_sound --invariant reachChainedSponge` | 1 | [violation] (non-vacuity) |
| `quint run hashchip.qnt --main hash_<H1..H14> --invariant invRefine` (14 runs) | 1 each | [violation] each |

Totals: 22 commands; 17 exit 1, each with a `[violation]` line; 5 exit 0.
- The traces of the 14 mutants each end with `acc: true` and `isTampered: true`. `isTampered` means "differs from the honest witness of the sound layout".
- The traces of the 3 `reach*` runs end with `acc: true` and `isTampered: false`.

### Lead repairs (for execution and vacuity only; the lane's originals are in `lane-original/`)

1. **Execution (all runs).** The lane's integer round function overflowed quint's 64-bit integers: `sbox(x) = x*|x| + x + 1`, squaring on every round. The overflow was QNT601 at `hashchip.qnt:114`. The first re-run therefore ended with these results:
   - `quint test`: 21 tests failed;
   - all 19 `quint run` commands: exit 1 with a runtime error and no `[violation]`, so none was a catch.

   The repair: rounds work in Z_p with p = 1000003, and the S-box is `x^5 mod p`. Because `gcd(5, p - 1) = 1`, the S-box is a bijection. The lane's mixing matrices are kept; their determinants (-1148 and -5396) are not divisible by p. Each round is therefore still an opaque bijection. Absorb and `hashRef` are unchanged.
2. **Vacuity, H8 (`chip_unpinned`).** The lane's attack relabelled the ALU provider row as HASH. That removed the only ALU tuple from the table. The honest ALU request on row 46 lane 1 then failed the lookup, so the witness was never accepted under any gate.

   The repair: the attack now relabels HASH provider row 0 as ALU (`tamperSingleCell(mt, 0, 0, CHIP_ALU)`). The ALU request then receives a hash output. This attack needs only the HASH pin to be dropped.
3. **Vacuity, H10 (`out_from_nonfinal`).** Under the mutant, the honest builder reads `out` from row 1 of every provider sponge, but the lane moved only provider 0's copy constraint. Provider 1's copy constraint therefore rejected the honest witness under the mutant. Both providers now copy from their row 1 under the mutant.

4. **Lead addition, `hashchip_test.qnt`:** `cex_H8_hash_direction` and `mutantH8_hashDirectionTest` (H8 in the HASH direction; see C5). Otherwise the test file is the lane's.

Note on repair 1: it changes the round function, not only its syntax. The repaired function is still an opaque bijection, so the claims of the model are unchanged, but this goes beyond a syntax or vacuity fix. The caller should judge it.

No repair weakens a constraint of the sound model.

## 5. Limits

- **Small instances.**
  - R = 2 round rows per permutation (one full round, one partial round), not 68.
  - One fixed layout: 2 providers, sponges of 2, 3, 4 and 5 inputs, and 2 request rows.
  - Tamper values come from {0, 1, 2, 3, 5, 99}. Tamper rows come from a 20-row subset.
  - The exhaustive folds cover only this domain. Nothing here proves soundness for every witness.
- **Collisions.** Element 0 of the output can collide mod p (probability about 1/p per comparison). No collision appears in the folds.
- **Tamper sync.** Only provider 0 is synced into a request. Tampers on provider 1 change no request.
- **H8 and the two pins.** The H8 mutant drops both chip pins (HASH and ALU). The repaired lane witness uses the dropped HASH pin. The lead test uses the dropped ALU pin. The sampled `quint run` of H8 can find either.
- **Lane `REPORT.md`, superseded in part.** Its §4.1 integer S-box and its H8 attack are replaced by the repairs above. Its §5 run options (5000 or 2000 samples, 5 steps) differ from the lead's (4000 samples, 3 steps). Its claim that `quint test` gives 21 passing is now 22, with the lead test.
- **Lead debugging.** A debug module and a copy of the model went briefly to the session scratchpad under `/tmp`, against the task's rule. They have been deleted.
- **Equality in the gates.** The model's gates compare integers, not residues. For the small tamper domain this changes nothing.
- **Out of scope.** The permutation itself (MDS, round constants, the stdlib differential) is not modelled; `poseidon::tests::differential_*` covers it.
