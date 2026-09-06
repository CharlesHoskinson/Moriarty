# R2 local language evidence

Status: S3 experiment. The [workspace](../../experiments/moriarty-developer-mock/README.md)
executes bounded JSON loan/swap packages, independently checks exact-plan
effects and signs locally. Real contract/transition/history proofs remain
unavailable. This record establishes only the predicates exercised below.

The [verification manifest](verification.json) records commands, exits and
SHA-256 hashes of the experiment's source/configuration/tests. Build ran before
tests so the tests importing compiled Core/package modules used current output.

| Check | Recorded result and scope |
| --- | --- |
| TypeScript build | Exit 0, [output](build.stdout.txt). |
| Node suite | 47 passed, zero failed: 16 language, 13 independent policy, 8 claim/signature and 10 prior mock tests; [output](node-tests.stdout.txt). |
| Browser smoke | Exit 0, [output](browser.stdout.txt); loan accrual/settlement, swap, named failures, independent policy, real browser Ed25519, missing required proofs, edits and async invalidation, export and mobile layout. |
| Additional visual check | [Receipt](visual-check.json), [desktop](desktop-language.png) and [mobile](mobile-language.png); signature plus unavailable claims and original-workspace navigation. |

Independent reviewer `r2_review` inspected the complete R2 modules and made
focused Node/browser reproductions. Three material findings were corrected:

- Shared mutable effect schemas poisoned subsequent elaborations. Each bundle
  now receives an isolated schema; [failing regression](schema-isolation-red.txt)
  preceded the fix, and the final suite includes that control.
- Edited loan terms left a default-interest claim in the quantization note.
  The note now uses the active elaboration's calculated description.
- Input edits cleared statuses but left stale results visible. Panels now clear
  immediately, preserving editor focus; new actions invalidate in-flight signing.

The reviewer found no additional material issue in that bounded pass and no
route to real-proof acceptance. This is source review and test evidence, not a
security audit or formal correspondence proof. Earlier task RED receipts are
preserved where available; they are not reconstructed from later behavior.

Financial limits: one micro-USD LAM period with explicit floor quantization,
one finite AMM epoch, synthetic balances and fixed role profiles. Signature
verification authenticates a locally trusted key and complete exact-plan
bindings. It does not establish wallet authority, freshness on a live ledger,
program correctness or predecessor proof validity. General outcome IntentIR,
nonce/expiry enforcement, cross-recipient aggregate budgets and residual
authority remain in R2b. R3 is specified-only.
