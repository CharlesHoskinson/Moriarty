# Verdict (lane run): task 2.5, VALUE DIGEST

Status: LANE RUN, 2026-11-15, kit-vdg lane (Claude). The Carbon (Gemini) Quint lane is OWED. Its prompt is `~/.cache/gemini-lanes/value-digest/PROMPT.md`. The lead re-run is also OWED. Do not tick 2.5 and do not call VDG-006 met before both are done.

Model: `models/sherlock/quint/valueDigest.qnt`. Commands: `commands.tsv` (20 lines, each with its expected exit code). Re-run: `rerun.sh OUTDIR`. Every quint call runs in heavy.slice through `heavy-run`. Results of this run: `rerun-results.txt`. Logs: `~/.cache/eui-kits/vdg/quint-run2/NN.log`.

## What the model checks

- The model has two separate physical representations. `heap` is the write-once DATA arena that CORE appends to. `rom` is the constants table, in its own arena. A heap record's `vd` is computed from its form, its scalar and the `vd` lanes of its children (`preimage`). The reference key `refKey` comes from the form, the decoded scalar and the decoded children. It does not depend on addresses or on `vd` lanes.
- The hash is interning (`vt`): it is injective on its domain, which is the collision-resistance premise. The constructor fold (`foldStart`, `foldStep`, `foldDone`) carries `(m, cur, acc, kids, ty)`.
- The FP-4 head of every preimage is `[length with the tag, 13, 2, form, arity]`.
- Bounds: `--max-samples 20000 --max-steps 12 --seed 0x20261115`.

Invariants (VDG-006): `VdRelocationInvariant`, `VdConstDataEqual`, `VdSharingEqual`, `VdInjective` (O24 under the premise) and `VdFormsDisjoint`. `VdAll` is their conjunction. These reachability witnesses are each violated, which shows the invariants are not vacuous: `reachSharedPair`, `reachConstEqualsData` and `reachCtor3`.

## Results

All 20 lines of `commands.tsv` match their expected exit codes. `vdBase` holds every invariant (cmds 2-7).

| Mutant (instance) | Rust mutant | Invariant violated (cmd) | Honest shared pair still reached (cmd) |
|---|---|---|---|
| `vdBusChain` | `vd_bus_rehash_chain` | `VdInjective` (11) | yes (12) |
| `vdChildKind` | `child_kind_unconstrained` | `VdInjective` (13) | yes (14) |
| `vdHostVd` | `host_supplied_vd` | `VdRelocationInvariant` (15) | yes (16) |
| `vdCursorSkip` | `fold_cursor_skipped` | `VdRelocationInvariant` (17) | yes (18) |
| `vdBoolNonBool` | `bool_nonboolean_selection` | `VdRelocationInvariant` (19) | yes (20) |

## Not in this model

- The `opened` fold cell (VDG-005, `vd_open_field_used_before_completion`). The Rust kit test covers it (`VdDriver` cell `OPN`, weak `DRV_OPENED`). The Carbon lane is asked whether Quint needs it.
- The ranges of UInt and Fr limbs, and the outcome forms. They are in the Rust differential and in Lean O23 (`pre_injective`).
- The read port (`AuthRec`, `AuthRom`). That is task 2.4.
