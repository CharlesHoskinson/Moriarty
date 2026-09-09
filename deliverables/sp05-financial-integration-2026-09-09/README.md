# SP05 financial integration implementation

Status: unreviewed source in progress. No Docker or Preview transaction, financial proof, or full Compact compile was run for this candidate.

The new local driver calls the actual SDK deployment and fixed loan initialize/accrue/settle or swap initialize/swap/close interfaces. Every stage waits for observation and financial comparison before continuing. Its current tests use inert SDK transport; they prove sequencing and stop behavior only.

The receipt module has a successful native serialized transaction path, checks input signatures and indexed input/output reconciliation, verifies node canonical-finality observations, and queries contract state and balances at the same explicit block hash. Its native test transactions contain no proof bodies. The native API can convert these proof-free records without invoking either prover callback. These are offline tests, not proof or ledger acceptance evidence. Positive receipt tests use a blank native deployment and inert RPC/indexer state; actual financial state remains a separate pending comparison.

The provider constructs the pinned SDK interfaces and performs real balance/sign/finalize/submit calls when used with a wallet. Source tests use inert wallet boundaries. The builder has a full compiler branch and checks source/resource/review bindings and generated assets. Its tests use inert command adapters. Neither is authorized to run by this document.

## Required next work

- Implement the complete financial stage comparator and generated-state projection for both cases, using every independent fixture field and actual native/contract observations. Never copy expected leaves into observations. The old synthetic comparator remains unchanged.
- Wire that comparator, receipt observer, proven asset loader, actual wallet roles and network configuration into the callable driver. No executable CLI exists yet; mandatory comparator injection is not a completed production composition.
- Verify that wallet balancing preserves the original approved contract effects, beyond equality between the signed recipe and finalized transaction. Complete persistent campaign accounting and retain original resource charges.
- Obtain a positive native DUST spend fixture and verify fee units against the pinned SDK and node. Do not equate the ledger package version with the observed protocol version; require an explicit deployment/protocol binding.
- Establish process containment for SDK operations that expose no cancellation. Enforce complete current source/resource review before the first full build or local settlement.
- Finish independent GPT-6 and Opus audits, then an admitted full compile, Docker loan and swap plus rejection/rollback controls, followed by separately admitted Preview. Report each actual transaction ID immediately.

The complete SP01.2/.3 financial contract remains parallel work; inputs are retained in /home/charl/.local/state/moriarty/rp01-next-inputs-2026-09-09.md. The TypeScript/Elm/Unison research branch is separately published at e17f926 and is not merged into main.
