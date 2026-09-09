# SP05 financial integration candidate

Status: unaccepted source checkpoint. The original assembled candidate is locally committed as `cb21b30`; subsequent builder and launcher repairs are described below. No full financial Compact compile, Docker financial settlement or new Preview transaction has run.

[Candidate 02](candidate-02.json) binds the earlier assembled package and 31 ledger source/test/fixture files. Builder repairs supersede its two builder-file hashes; the old manifest and reviews remain historical evidence. The [checkpoint](PROGRESS-02.md) explains the implementation, actual-runtime discoveries and limits. The package exposes real callable composition through `ledger/integrate-local.mjs`; it supplies deployment preparation, a proof-asset loader, durable providers, the observer, complete comparator and fixed local driver. It takes existing wallet handles and explicit deployment/resource inputs. It does not create a wallet or provide an admitted financial-network CLI.

Candidate 02 checks passed: [52 existing financial tests](baseline-tests-final.txt), [146 ledger-module tests](ledger-tests-final.txt), and [three checks using actual retained generated contract code](compiled-tests-final.txt). Generated-code tests use simulated ledger context/transport. They cannot establish financial network or proof acceptance.

Two concrete corrections are preserved. The [historical DUST fixture](../../experiments/moriarty-midnight-financial/ledger/fixtures/historical-dust/REPORT.md) shows why native debit cannot be equated to indexer fee strings. The [mint-to-self refinement](native-effects-refinement.json) corrects native input representation without changing economic expectations; [the original expectations](expectations-v1.json) remain intact. Native state snapshots and effects are retained through the integration's acknowledged per-stage callback.

## Remaining acceptance work

- Complete independent review of the assembled source, including actual SDK composition. Builder-only review does not approve the entire integration.
- Complete Opus review of the assembled integration. Canonical `claude-opus-5` has now reviewed the builder and its repairs; those limited source approvals do not cover the complete integration. The original two timeouts and the launcher response that supplied no substantive review remain preserved.
- Enforce and verify the proposed compiler memory, swap, output, deadline and cleanup limits, establish current admission, then generate and review genuine loan/swap proof assets.
- Establish outer process containment for SDK operations that expose no cancellation. Current real driver cleanup remains explicitly incomplete.
- Execute admitted Docker loan and swap transactions and failed-transaction financial rollback controls; retain actual bytes, fees, full state/effects and canonical finality.
- Run the separately admitted Preview campaign and report every actual transaction ID. Mandatory PCD and later roadmap acceptance remain separate.

The original eight-file draft and first audit remain in `draft-01`, `source-candidate.json` and `gpt6-draft-review.md`; their old measurements describe that earlier source. The complete SP01.2/.3 financial contract remains parallel eligible work. The separately published TypeScript/Elm/Unison research branch remains unmerged.

## Builder repair checkpoint

The changed builder rejects an attempt path equal to the output directory before consuming its allowance. Manifest commitments use fixed object-key order. A post-charge directory or receipt failure preserves the original error and attempts a separate diagnostic outside the output quota. The receipt explicitly distinguishes compiler key artifacts from transaction proofs.

All 24 builder tests and the full 150-test offline ledger suite passed after these changes. Independent GPT-6 and Opus source reviews bind build-source candidate `a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6` in [build-review-02](build-review-02/). These are source checks, with no compiler invocation.

The fixed launcher enforces a per-case tmpfs generation quota and a separate logical retained-content ceiling. Its seven tests include actual ENOSPC, sparse/hardlink rejection and growth during copying. The original retention defects and rejection are preserved. Filesystem allocation rounding and metadata are outside the logical retention ceiling; this is not a sandbox for malicious writes to arbitrary host paths. Two harmless serial service probes recorded raw memory, swap, process and runtime settings. Actual compiler admission and terminal artifact review remain required.
