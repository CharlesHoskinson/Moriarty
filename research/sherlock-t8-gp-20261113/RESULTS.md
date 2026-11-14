# T8 results: the GP hooks on the node layer

Row T8 of `research/sherlock-tested-design-20261113/LOOP.md`. Trees, all on branch `t8-gp`: Moriarty `~/Projects/Moriarty-wt/t8-gp` (base `fc830df0f`), openvm `~/.cache/m7-wrap/src/openvm-t8` (base `662235f`, the T7 tip), stark-backend `~/.cache/m7-wrap/src/stark-backend-t8` (no change). State, logs and outputs under `~/.cache/t8/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

## 0. For the lead

Status: in progress (draft). Readings and picks are added as each hook lands.

1. **N-4 reading: option (a).** `continuation.qnt` (section 1) shows that no circuit relation needs a checkpoint tag: with no tag in any circuit relation, and even with a record digest that carries the `SHK_RUN` tag (mutant `recordTagRun`), every attempt to offer a checkpoint record or a prefix SEG as a claim is refused by SEAL or by the consumer (`CheckpointRejectedBySealAndGP2` and `circuitSide` hold). The tag is needed on the persisted record digest only: a consumer that checks the claim tag and not SET membership (mutant `consumeAnyRun`) is stopped by the record tag, and with both faults the record is accepted (`t_anyRunUntagged`). Lead to confirm the reading of R-7.
2. **`SHK_CKPT` code (LEAD pick).** The record tag is a host digest and takes the next free BabyBear code after the slice picks (`SHK_RULES` 64): 67, after `SHK_FV` 65 and `SHK_REG_ENTRY` 66 (section 3).

## 1. N-4: `continuation.qnt`

Model `models/sherlock/quint/continuation.qnt`, tests `models/sherlock/quint/continuation_test.qnt`, runner `research/sherlock-t8-gp-20261113/quint/rerun-continuation.sh`, logs `quint/runs/r2/` (quint 0.32.0, 5,000 samples, 40 steps, seed `0x20261113`).

What the model holds: one genuine run that the uninterrupted proof seals as Exhausted at `h = 6` (fuel 6, 7 steps to Halted, so a refreshed fuel would reach a different claim); leaves of one step proved by workers in any order; the tier 1 carry stack; MERGE (interval and stitch), SEAL (`h_lo = 0`, terminal) and the genesis gate of the leaf as the circuit relations; mode 1 records with the `CKPT` tag; an adversarial host that resumes from tampered records, proves leaves from any state it holds at any claimed height, merges and seals any proven SEG, and offers any digest it holds to a consumer that accepts only the head of a verified `LEAF` SET.

Invariants (the four of the kit plus three): `ResumeProducesIdenticalClaimD`, `OutOfOrderWorkerCompletionPreservesSegInterval`, `CheckpointRejectedBySealAndGP2`, `MaxStackDepthLeLogLPlus2`, `AdminStepsKeptApart` (pause and resume change no height, state or fuel), `RecordNeverParsesAsClaim` (N-4 (a)), `NoTerminalCheckpoint` (Exhausted is never a checkpoint trigger).

Lead-style runs (MEASURED, `quint/runs/r2/summary.txt`):

| Command | Result |
|---|---|
| typecheck model and tests | exit 0 |
| `continuation_test`: positives (out-of-order tier 1, resume at a power-of-two boundary, at an odd leaf and twice) and the 7 named mutants as attacks on the sound instance | 11 passing |
| one module per mutant (`noStitch` 2 cases, `noInterval`, `noGenesis`, `noTerminal`, `recordTagRun`, `consumeAnyRun`, `anyRunUntagged`, `pauseTerminal`) | every module passes: the attack completes and the named invariant breaks (or, for `consumeAnyRun`, the record tag still stops it) |
| `run` sound instance, `soundAll` and each invariant alone | `[ok]` |
| `run cont_recordTagRun --invariant circuitSide` | `[ok]`: the circuit side holds with an untagged record |
| reach witnesses `reachSealed`, `reachSealedAfterResume`, `reachConsumed` | `[violation]` as expected (not vacuous) |
| sampled mutants `noStitch`, `noInterval`, `noGenesis`, `noTerminal`, `recordTagRun`, `anyRunUntagged`, `pauseTerminal` | `[violation]` (caught) |
| sampled `consumeAnyRun` | `[ok]`: the record tag stops the tag-only consumer (expected) |

Named mutants of the kit and where each is refused: `cont_refresh_fuel` and `cont_change_input` by the MERGE stitch; `cont_skip_predecessor` by the MERGE interval; `cont_reset_height` by the genesis gate of the leaf; `cont_publish_as_run` by the SEAL terminal check; `cont_ckpt_as_gp2_claim` by the consumer's SET-membership check; Exhausted as a checkpoint by the scheduler rule. `cont_tampered_read_cell_unvalidated` and the host refusals of the HOST CHECKPOINT STORE kit are outside this model (no memory cells).
