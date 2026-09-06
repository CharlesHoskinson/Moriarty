# ZKIR v3 test corpus

Programs used by the ZKIR-in-K semantics (see `wiki/zkir-k-semantics-plan.md`).

- `ledger9-92e8bdd3-tests/`: the 43 inline JSON programs from `zkir-v3/tests/` of
  `midnightntwrk/midnight-ledger` at commit `92e8bdd3` (the surface mechanized by
  arc-zkir), extracted verbatim and pretty-printed; `manifest.json` maps each file
  to its test function and source line. Apache-2.0, Copyright Midnight Foundation.
- `midnight-zkir-2ffe2d1-precompiles/`: the six version-3 programs under
  `zkir-precompiles/` of `midnightntwrk/midnight-zkir` at `2ffe2d1`. Apache-2.0.
- Moriarty's own artifacts: `experiments/moriarty-compact-escrow/output/zkir/*.zkir`
  and `experiments/moriarty-core-swap/output/zkir/*.zkir` (version 3, emitted by
  compactc with `--feature-zkir-v3`).
- `moriarty-contexts/`: one real ProofPreimage per Moriarty artifact, produced by
  executing the compiled contracts through the Compact runtime
  (`tools/moriarty_preimages.mjs`; `manifest.json` records the scenario, block time
  and inputs of each). Checked by `tools/moriarty_contexts.py`; see
  `plan-iter3/m4-contexts.md`.

The ledger's own `zkir-precompiles/` at `92e8bdd3` are still version 2 and are not
part of this corpus.
