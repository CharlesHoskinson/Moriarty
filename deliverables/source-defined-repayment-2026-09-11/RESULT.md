# Source-defined repayment result

All four source-authoring steps are implemented and independently approved.

| Step | Demonstrated behavior |
| --- | --- |
| Source declarations | One .mori file declares units, assets, records, ordinary state, protected operations and one repayment action. |
| Schema elaboration | Source declarations produce the same canonical checked schema as the existing funded example. |
| CLI | Check, format and funded simulate use the new profile without --schema. Snapshots still supply runtime values. |
| Rejection | Duplicates, unknown types, mixed units, altered protected operations, reserved names and record/operation cycles reject. Declaration diagnostics retain original UTF-8 spans. |

The example's 10+20 payment leaves payer cash70, principal70 and work53, with
closure reserve16 unchanged. Interest-first7 against principal100/accrued10
leaves principal100/accrued3. Complete results and continuation match the
previously reviewed funded adapter. Failed funding, guards, postconditions,
unit witnesses and work checks publish no tentative state or effects.

## Evidence

- Final candidate: candidate-02.json, SHA256
  306294df868c12fa7e42d49eb431eed2b7f6b52c13f761cdd4cacb6f17a5b79f.
- Author: Grok 4.6 high, returned identity grok-4.6-build, session
  01a09288-cc1a-7871-9703-04544cf69322; all three runs ended with end_turn.
- Root: 720 full package tests, typecheck, 18 CLI cases, 15 diagnostic cases and
  57 legacy differential cases pass. See root-verification-02.json.
- Fresh independent GPT-6 Astra medium approved all affected code and dependencies
  on the final candidate. It verified all156 file hashes before/after and ran
  32 focused tests,45 independent probes and typecheck. See audit-02/review.json.
- Earlier audit-01 requested two diagnostic repairs. Its original findings and
  failed receipts remain intact; the fresh final review confirms both resolved.
- Reviewed source was applied to the main checkout after verifying all148
  baseline files. All156 final candidate hashes match in main. Existing user
  changes were preserved. See integration.json and main-cli-simulate.json.

Run the documented commands in
../../experiments/moriarty-language/spec/successor/financial-agreement-source.md.
This is bounded single-action local language acceptance. Financial ledger
settlement, native/K proofs and Preview execution were outside this task.
