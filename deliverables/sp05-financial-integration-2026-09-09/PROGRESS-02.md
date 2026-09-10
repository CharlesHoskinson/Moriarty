# SP05 source integration checkpoint

The callable financial path now has full-state comparison and native observation modules. This is source work under the existing SP05 scope. No Docker financial transaction, Preview transaction, transaction proof, or full Compact build has run for this checkpoint.

The root check ran all current ledger tests with the explicitly selected retained `main-integration` skip-zk artifacts: 132 tests passed. `integration-tests-02.txt` retains the output. Three tests execute the actual generated loan and swap runtime and compare every stage's public state and transcript effects. Their UTXOs, fees and finality are simulated and are not network evidence.

## Corrections grounded in actual observations

The historical public hello-world transaction in `ledger/fixtures/historical-dust/` declares a native DUST debit of 300000000000001 SPECK; the indexer reports paid and estimated fee strings of 1. The pinned wallet source adds a safety overhead. The observer now preserves native debit and indexer-reported fees separately, with the indexer's precise encoding and relationship explicitly unresolved. The full native debit remains charged. The regression replays actual public bytes through inert RPC/state adapters; it submits nothing.

The actual generated swap initializer records mint-to-self amounts both as claimed contract mints and as unshielded inputs. The original independently derived economic values remain intact. `expectations-v1.json` preserves the original record; `native-effects-refinement.json` records the two native-input representation corrections and their source. Contract reserve change is inputs minus outputs; adding the self-mint claim again would double-count it.

## Current implemented boundaries

- Provider balancing must preserve the original native contract actions, transcripts and outputs. Only validated payer funding/change and the bounded DUST balancing transaction may be added.
- Reservations are exclusive and durable across reconstruction, including cumulative gross inputs, DUST debits, submissions, identifiers and unresolved operations. Missing, corrupt or crash-locked records require explicit recovery.
- The driver checks actual provider network/payer bindings, requires an explicit comparator PASS, and stops before another SDK call after a failure. It retains public IDs and reservation state when cleanup is incomplete.
- The comparator checks all public state fields, complete cumulative result records, actual derived colors, role capabilities, signed-input ownership, native payouts, gross inputs, net participant deltas, reserves and conservation. Its public snapshots contain observed values, not expected defaults.
- Deployment preparation fixes the contract address before native asset limits are allocated. Its single-use adapter submits the exact prepared native bytes, preserves private constructor data inside the closure, and rejects changed constructor options.
- The proof-asset loader checks source/receipt bindings, the complete artifact inventory and pinned module imports. Its synthetic inspection path cannot return an executable contract. A genuine financial build/import is still unperformed.

The complete production composition is being wired in `integrate-local.mjs`; its later tests are not included in the count above. The original draft and audit remain preserved under `draft-01` and `gpt6-draft-review.md`.

## Next acceptance evidence

Independent GPT-6 and Opus source/resource reviews of the full builder are under `build-review-01`. The proposal permits no wallet, proof-server or financial submission operations. Actual compiler dispatch requires both substantive reviews and verified resource enforcement, followed by inspection and result review of the real artifacts. Full integration source review and admitted Docker loan/swap settlement precede the separate Preview campaign. Mandatory PCD, full SP03 semantics, RP01 and all later roadmap gates remain open according to their own acceptance records.

## Assembled candidate

The integration composition is now implemented. Final package entry-point checks pass: 52 baseline tests, 146 offline ledger tests and three retained-generated-runtime tests (201 total). Exact outputs are in `baseline-tests-final.txt`, `ledger-tests-final.txt` and `compiled-tests-final.txt`; `candidate-02.json` binds the 32 current package/source/fixture files. This supersedes the earlier 132-test intermediate count for the assembled source scope.

GPT-6 approved the frozen builder source/resource proposal conditionally. Both bounded Opus consultations timed out without output, including the safe-mode attempt. `build-review-01/opus-disposition.md` records the unavailable review and preserves both attempts. No audit is inferred from these timeouts; full integration source review and compiler admission remain pending. The candidate is checkpointed locally as unaccepted work and is not merged or published.
