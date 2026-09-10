# Existing local loan recovery source

Source candidate03 is approved by independent GPT-6 Astra and Grok4.6 reviews. The launcher can recover the retained deployment without redeploying: public native/finality/current-state checks precede seed access; verified original constructor, strict wallet checks and metadata-only database-copy inspection precede isolated private restore; readback precedes the three remaining calls.

- [Exact candidate](source-candidate-03.json), [GPT-6 review](source-review-gpt6-03.json), [Grok terminal review](source-grok-result-03.json).
- [288 passing source tests](ledger-regression-05.tap); these do not establish live recovery.
- [Original LevelDB lock reproduction](source-review-gpt6-level-lock-reproduction-01.json) and [review correction dispositions](source-corrections-03.md). Original rejected candidates and reviews remain retained.
- [Approved design](recovery-design-02.md), [implementation checklist](implementation-plan.md), and [separate runtime envelope](../local-recovery-01/resource-proposal-01.json) and [failed public-preflight result](../local-recovery-01/RESULT.md).

The first admitted recovery attempt stopped at public preflight before creating private files or launching the wallet. No original database has been opened for recovery and no new recovery transaction has been attempted. Canonical finality/current state, original copied database contents, live wallet input reconciliation, actual private restore and initialize/accrue/settle remain runtime gates. Original deployment and all earlier charges stay intact. Local loan recovery would not close swap, Preview, full SP05 or mandatory PCD acceptance.

Grok's full source review completed in750.576seconds and its correction in131.907seconds, both under900-second allowances. Quiet output was not treated as failure. Raw provider streams remain private; only public review results and metadata are retained here.
