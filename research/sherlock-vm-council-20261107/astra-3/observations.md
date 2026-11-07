# Independent observations

Observation window: 2026-11-07 20:46–20:49 UTC. Repository HEAD: `8fd34ed794eda7186c32495f715609ec54a9a297`.

## Stored factory state

`scripts/sherlock-next` returned A 7/10, B 4/9, C 5/6, U 34/35 and F 11/13. It named A8 first. This command reads stored results. `qa/sherlock/status.json` records update time `2026-10-26T13:52:10Z` and head `7e22b5823a030ce0c0a5d7c29fedf7a2ed0b4092`. These counts are not fresh executions at current HEAD.

## Fresh A8 observation

Command, from the repository root:

```sh
timeout 45 scripts/qa/check-a8.sh
```

Exit: 1. Output:

```text
FAIL: only a ContractDeploy (block 1032519); zero contract calls observed
```

The checker queries `https://indexer.stagenet.shielded.tools/api/v4/graphql` and the address in `crates/mori-recursion/tests/fixtures/stagenet-verifier.json`. It accepts any observed `ContractCall` at that address. It does not compare the call's entry point, proof, statement, installed key, or resulting state against a candidate (`scripts/qa/check-a8.sh:8-35`). A8 therefore supplies only one part of deployment evidence.

## Fresh stagenet identity

Read-only JSON-RPC requests to `https://rpc.stagenet.shielded.tools`, with a 20-second transport timeout, returned:

```json
[
  {"method":"midnight_ledgerVersion","result":"=1.0.0"},
  {"method":"system_version","result":"2.0.0-d9729c13"},
  {"method":"chain_getBlockHash","params":[0],"result":"0x2f76825abc239fecf6107c9df99016de57037b451ae57a4394b76c8cf53a9491"}
]
```

The endpoint comes from `scripts/qa/assemble-u023-record.py:28`. These endpoint responses are observations, without an independently authenticated node identity. They do not establish equivalence with the production ledger-9 release. The official Mainnet release names ledger `9.1.0.1` and node `2.1.101`: [node release notes](https://docs.midnight.network/relnotes/node/node-2-1-101).

## Dependency identity

`Cargo.toml:27-42` already selects package `midnight-ledger-v9` at commit `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`, with `midnight-proofs = 0.8.3`, `midnight-circuits = 7.2.4` and `midnight-zk-stdlib = 2.3.5`. The ledger-8 entry in `wiki/midnight-repositories.md:41` is not the current Cargo pin. A compatibility review must compare the actual pins against production.

No build, proof generation, wallet access, transaction submission, source edit, factory status update or commit occurred.
