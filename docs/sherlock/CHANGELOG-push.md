# Sherlock finish push: change log

Dated 2026-10-25. Records what the finish push changed in the design and interfaces. Counts are per branch
(A, B and F on `factory-b`; U on `factory-u` and `factory-u-c4`) until the lines are consolidated.

## Design

- Profile `sherlock-helper-microstep/2` adds the `Await` control (C6 tag 11), `Waiting{site}` outcome (tag 4)
  and `LogicalCap` exhaust reason (tag 7). `/1` refuses all three; the owner engine pins its selected profile.
  Source: commits 1523aa9b6 and 88d1a0c0b. Not yet in code: the resume rules, `XFinCommit`, draft language `/2`.
- Suspension and finance design note, rev 4: [design/suspension-and-finance.md](design/suspension-and-finance.md).
  A Waiting checkpoint is an ordinary settled root; the resume observation is disclosed and recomputed by the
  settling contract; no ledger, node, SDK or proof-server change.
- Technical chunking, K-A `ReferenceSlice` grouping and chunk materialization through the `WitnessPlan` DAG.
- C6 native fragments (Eval schedule, CallBind, Find/EnvGet, Append, Bind, CallArguments, legacy control) behind
  one dispatch owner; receipted compiled-and-run evidence accepted for U023.
- U021 projection and U026 stage package entrypoints in `models/sherlock/qa/model_package.py`.
- B line: `sherlock-v3-batch/1` leaf, parent and decider; `FSYNB002` fixed-key prefix trace; B7 memory cap raised
  from decimal 16 GB to 32 GB with swap zero (owner, 2026-10-25).
- A line: A9 passes on stagenet (rejection cases against deployments D2 and D3); A 10/10.

## Interfaces

- `openspec/changes/sherlock-universal-interpreter-design/interfaces.md`: the open Commit/Yield/Await item is
  decided; new sections for profile `/2`, the model correspondence package and the rollup interfaces.
- `docs/sherlock/design.md`: new section "2026-10-25: implemented design by group".

## Visualization

- `visualization/index.html`: pipeline strip with eight stages and their state, above the workbench. The 3D die
  engine is unchanged; `motion.html` (Quinelings animation) is unchanged.

## Not changed

Ledger 9, the node, the Midnight SDK, the proof server, the 39-rule table of profile `/1`, the existing proof format.
