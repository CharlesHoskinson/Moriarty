# Holdout scenarios (evaluator-owned)

Sealed scenarios for the DoctorWatson factory. The implementing agent sees only the exit code.
Entry point: `qa/holdout/run.sh <suite>` with suites `rejections-local`, `rejections-undeployed`,
`rejections-preview`, `rejections-mainnet`, `uireq-001` ... `uireq-035` and `list`;
`qa/holdout/run.sh uireq-023 --network NET` selects the U023 scenario 2 target network.

Exit codes: 0 pass, 1 fail, 2 unimplemented (specified, but the system under test does not yet
expose what the case needs), 3 host unfit.

Networks (E12; owner decision 2026-11-08: Mainnet is the target, Preview the test network, stagenet
retired): `config/networks.json` is the sealed registry. Selectable networks are `undeployed` (local
Docker at ledger 9.1.0.1), `preview` and `mainnet`. Stagenet is retired: `rejections-stagenet` and
`DW_HOLDOUT_NETWORK=stagenet` are refused, and the `s-*` cases and `lib/stagenet_check.py` stay only
as history. The rejection suite takes the network from its suite name; U023 scenario 2 takes it from
`--network`, else `DW_HOLDOUT_NETWORK`, else `default_network` in `contracts/U023.json`. The mainnet
indexer (Blockfrost) needs `DW_BLOCKFROST_MAINNET_PROJECT_ID` (through `vault run`; never committed);
without it the run is host unfit. The holdout only reads indexers; it never opens a wallet or submits.

- `cases/rejections/*/case.json`: mutation, expected verdict (REJECT), target entrypoint, sources;
  `scope` is `local`, `network` (with the `networks` it applies to) or the retired `stagenet`.
- `cases/uireq/NNN/case.json`: scenario text anchored to the openspec spec, expected outcome, executor.
- `config/`: harness configuration and the network registry (no expectations).
- `lib/`: runners (Python standard library; no new dependencies).
- `tests/`: unit tests of the holdout tooling, run with
  `python3 -m unittest discover -s qa/holdout/tests -t qa/holdout/tests` (no network, wallet or cargo).

Expected outcomes come from the security and interpreter requirements, never from current
implementation behavior. Only the evaluator edits this tree (`DW_ROLE=evaluator`), then reseals
`MANIFEST.sha256` (`<sha256>  <path relative to qa/holdout>`, sorted, manifest excluded) with
`DW_ROLE=evaluator scripts/doctorwatson holdout seal`.
