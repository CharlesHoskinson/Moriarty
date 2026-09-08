# Evaluation decoder correction

Grok 4.6 high added the regression tests and implementation in separate calls. Fresh GPT-6 approved the isolated correction.
The decoder now bounds primitive strings before encoding, rejects hostile carriers without caller property access, copies admitted bytes, and returns a frozen admitted record.

The parent ran the unchanged semantics baseline, verified nine substantive failures in the new tests, froze those tests, then verified the correction. The complete language suite and TypeScript check passed. Existing Compact checks used `--skip-zk`; no native proof or public transaction was executed.

`candidate.json` pins the exact reviewed bytes. `gpt6-result-review.json` records the independent verdict and limits. Command, capability, test output, and resource receipts are retained here. `dependency-paths.json` disambiguates the external runtime pins. `run.py` records the supervision implementation, not a portable claim that these absolute dependencies exist in every clone.

The baseline and tests-only worker had an external binding and ledger before dispatch. Canonical campaign links were added afterward, before source implementation. The campaign record preserves this timing. No backdated index is claimed.

The correction does not complete SP01 or MC01. Separate frontend diagnostic findings remain open. Private provider reasoning was not copied into this evidence directory.
