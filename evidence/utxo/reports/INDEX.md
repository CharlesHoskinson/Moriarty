# eUTXO validation dossier index

Task root `/home/charl/moriarty-eutxo-validation-2026-10-12`.  
Worktree `/home/charl/Moriarty-wt-eutxo-validation-20261012`.  
Research (immutable) `/home/charl/moriarty-utxo-research-2026-10-12`.

Author freeze: Grok 4.6. Independent Grok review is parent-owned. Do not treat this index as approval.

**FULL MORIARTY PRIVATE/FINANCIAL COMPATIBILITY IS NOT CONFIRMED.**

## Verdict documents

| File | Role |
| --- | --- |
| [ASSESSMENT.md](ASSESSMENT.md) | Scoped public native compatibility verdict, hashes, gaps, commands |
| [PATTERN-RESULTS.md](PATTERN-RESULTS.md) | 20 families, 83 source controls, prototype UPLC/native |
| [PATTERN-INVENTORY.json](PATTERN-INVENTORY.json) | Frozen 20-family inventory |
| [SOURCE-SUITE-RESULTS.json](SOURCE-SUITE-RESULTS.json) | Five independent verifier receipts |
| [final-validate.stdout.json](final-validate.stdout.json) | `npm run validate` 20 patterns / 24 rows / 0 discrepancies |
| [NATIVE-MATRIX-OVERLAY.json](NATIVE-MATRIX-OVERLAY.json) | Overlay receipt linking native prototype onto matrix |
| [experiments/moriarty-eutxo/README.md](/home/charl/Moriarty-wt-eutxo-validation-20261012/experiments/moriarty-eutxo/README.md) | Public compile/validate commands and isolated native command |

## Source and compiler

| Path | Role |
| --- | --- |
| `designers/01-state` … `05-authority-private` | Independent designs and `verify.mjs` |
| `run-source-suite.py` | Runs all five verifiers |
| `experiments/moriarty-eutxo/src/admit.mjs` | SHA `e1a156d05ac52b6e6e0d5a79e1619377e907495d909f8a07a90f66a04e0fbf65` |
| `experiments/moriarty-eutxo/artifacts/matrix.json` | Source/UPLC/native/private matrix |
| `audit-backend/REVIEW-REPAIR-01.md` | Independent first-repair review (quantum required) |
| `audit-backend/repro-quantum.mjs` | Quantum-2 compilePublic must not publish blueprint |
| `audit-grok-final/` | Prior Grok isolated backend review (historical) |
| `audit-design/REVIEW.md` | Design audit (historical) |

## Native

| Path | Role |
| --- | --- |
| `tooling/final-runtime/` | Isolated final Yaci/Haskell home, compile, submit, CBOR, readback |
| `tooling/final-runtime/native-final-transactions.json` | Confirmed, rejected, and unsubmitted hashes/status from retained CBOR |
| `tooling/final-runtime/native-compile.json` | Repaired compilePublic on live policy IDs |
| `tooling/final-runtime/native-script-binding.json` | Lucid bind of CIP-57 script to address/credential `56d0170d…` |
| `tooling/final-runtime/native-cli-txid-verify.json` | Independent cardano-cli TxId of retained signed CBOR |
| `tooling/gift-card-reference/plutus.json` | Official consumed-seed mint (unchanged) |
| `tooling/runtime/` | Frozen old network; do not submit |
| `tooling/native-probe/` | Old pre-repair candidate snapshot |

Do not read `*.seed` or `devnet-start.log` (Yaci boot prints keys). Public JSON (`cash-policy.json`, `thread-policy.json`, `task-wallet-public.json`) is safe.

## Graph

| Path | Role |
| --- | --- |
| `graphify-out/graph.json` | Full exact-identity export |
| `graphify-out/graph.json.pre-luna-02-merge` | Prior full graph |
| `graphify-out/graph.json.pre-grok-native-evidence` | Post-Luna, pre-Grok-native |
| `graphify-out/luna-experiments.json` | Luna 82/212 20-family graph |
| `graphify-out/grok-native-evidence.json` | Grok native nodes/edges |
| `graphify-out/knowledge-focus.json` | 20-family + Grok native view |
| `graphify-out/graph.html` | Community HTML |
| `graphify-out/knowledge-focus.html` | Focused experiment HTML |
| `graphify-out/EXPORT-VERIFICATION.json` | Inventory hash integrity |
| `graphify-out/LUNA-02-MERGE-VERIFICATION.json` | Luna merge receipt |
| `graphify-out/GROK-NATIVE-GRAPH-VERIFICATION.json` | Grok native merge receipt |
| `graphify-out/COVERAGE-LUNA-02.json` | Luna coverage (historical candidate native) |
| `graphify-out/COVERAGE-GROK-NATIVE.json` | Grok native coverage |

Luna did not review Grok final native facts. Old candidate nodes stay historical. Independent native review is provisional (`audit-grok-native-graph/REVIEW.md`). Independent backend review is scoped public-backend only (`audit-grok-final/REVIEW-BACKEND.md`). This index is not final native/graph approval.

Rejected hashes: T3 `5377a951…e97c` `InvalidWitnessesUTXOW`; T3b `1697c0c3…a72b` phase-2 wrong-state; loser `619cb0f7…d394` `BadInputsUTxO`; preimage `ebeb43a4…e97e` unsubmitted. Race is sequential conflicting-spend.

## Reproducible commands

```sh
npm --prefix /home/charl/Moriarty-wt-eutxo-validation-20261012/experiments/moriarty-eutxo run validate
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/overlay_native_matrix.py
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/run-source-suite.py
```

Isolated native is a single-use chain fixture: `tooling/final-runtime/start-final-devnet.py`, `wait-final-devnet.py`, `compile-native.mjs`, `native-final.mjs`, `native-final-t3b-race.mjs`. Invoke `yaci-cli` directly. Do not use `npm start.mjs`. Do not reuse stale heads, restart the live network, or resubmit retained CBOR without a new namespace. After `npm run validate`, restore the overlay with `overlay_native_matrix.py`.

Author freeze: [GROK-AUTHOR-FREEZE.json](GROK-AUTHOR-FREEZE.json), SHA-256 sidecar [GROK-AUTHOR-FREEZE.sha256](GROK-AUTHOR-FREEZE.sha256). Graph byte hash: [graphify-out/GRAPH-SHA256.txt](graphify-out/GRAPH-SHA256.txt). Independent Grok review is parent-owned. This index does not self-approve native or graph.
