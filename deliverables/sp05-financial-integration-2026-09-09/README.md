# SP05 financial integration candidate

Status: unaccepted source checkpoint, locally committed as `cb21b30`. The complete candidate still requires independent GPT-6 and Opus reviews. No full financial Compact compile, Docker financial settlement or new Preview transaction has run.

[Candidate 02](candidate-02.json) binds the current package and 31 ledger source/test/fixture files. The [checkpoint](PROGRESS-02.md) explains the implementation, actual-runtime discoveries and limits. The package exposes real callable composition through `ledger/integrate-local.mjs`; it supplies deployment preparation, a proof-asset loader, durable providers, the observer, complete comparator and fixed local driver. It takes existing wallet handles and explicit deployment/resource inputs. It does not create a wallet or provide an admitted financial-network CLI.

Final checks pass: [52 existing financial tests](baseline-tests-final.txt), [146 ledger-module tests](ledger-tests-final.txt), and [three checks using actual retained generated contract code](compiled-tests-final.txt). Generated-code tests use simulated ledger context/transport. They cannot establish financial network or proof acceptance.

Two concrete corrections are preserved. The [historical DUST fixture](../../experiments/moriarty-midnight-financial/ledger/fixtures/historical-dust/REPORT.md) shows why native debit cannot be equated to indexer fee strings. The [mint-to-self refinement](native-effects-refinement.json) corrects native input representation without changing economic expectations; [the original expectations](expectations-v1.json) remain intact. Native state snapshots and effects are retained through the integration's acknowledged per-stage callback.

## Remaining acceptance work

- Complete independent review of the assembled source, including actual SDK composition. Builder-only review does not approve the entire integration.
- Obtain an actual Opus review. Both bounded attempts timed out without output or identity; [the disposition](build-review-01/opus-disposition.md) preserves them. No approval is inferred.
- Enforce and verify the proposed compiler memory, swap, output, deadline and cleanup limits, establish current admission, then generate and review genuine loan/swap proof assets.
- Establish outer process containment for SDK operations that expose no cancellation. Current real driver cleanup remains explicitly incomplete.
- Execute admitted Docker loan and swap transactions and failed-transaction financial rollback controls; retain actual bytes, fees, full state/effects and canonical finality.
- Run the separately admitted Preview campaign and report every actual transaction ID. Mandatory PCD and later roadmap acceptance remain separate.

The original eight-file draft and first audit remain in `draft-01`, `source-candidate.json` and `gpt6-draft-review.md`; their old measurements describe that earlier source. The complete SP01.2/.3 financial contract remains parallel eligible work. The separately published TypeScript/Elm/Unison research branch remains unmerged.
