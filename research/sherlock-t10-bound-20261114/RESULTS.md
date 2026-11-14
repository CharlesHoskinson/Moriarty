# T10 results: the bound wrap and the design contract

Row T10 of `research/sherlock-tested-design-20261113/LOOP.md`. Trees on branch `t10-bound`: Moriarty `~/Projects/Moriarty-wt/t10-bound` (base `316b17682`), openvm `~/.cache/m7-wrap/src/openvm-t10` (base `662235f`, the T7 tip), stark-backend unchanged (`deeeb0a`, the openvm `[patch]` still points at `../stark-backend-t6`). State, logs and outputs under `~/.cache/t10/`.

Labels: MEASURED (read from a run record), DERIVED (computed from MEASURED numbers or source by a stated rule), ESTIMATE.

STATUS: IN PROGRESS. This file is updated as each stage lands.

## 0. For the lead

1. **The `L` log tie (lead ruling on T7 pick 8) does not fit the frozen `L` column bound.** Probe: the J8 onion AIR (`iface_l::OnionAir`, 70 columns) added to the T7 `L` key gives G1 `Columns { set: "L", got: 2056, bound: 2000 }` (MEASURED, `~/.cache/t10/logs/probe-onion.log`, `chain.sh probe`). The `L` key has 14 free columns, and the brief bars T10 from slimming `L` (that is T7c). So the tie is not built in T10. Everything else in T10 runs on the T7 `L`.
2. **`h_hi` range check (lead ruling on T7 pick 9): built.** N1 gets one more AIR, `HhiRangeAir` (2 columns, 2^16 rows, bus 963), and the First statement AIR looks up each of the four `h_hi` limbs of every enabled row.
