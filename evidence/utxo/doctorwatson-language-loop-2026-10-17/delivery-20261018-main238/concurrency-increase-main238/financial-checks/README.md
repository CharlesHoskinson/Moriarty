# PR236 financial checks on current main

Current main `71c22f8a477231be869786eb42f04e3b50d1b3a7` / tree`6b6204185c316ceaf7ecd19cc6fb8164ad1d79b4`.
Candidate source comes unchanged from PR236 `360f32ba5b6680788d1c0f1e7b68c2207a877d47`.
CHANGE.patch applies those15bodies without rewriting any frozen source/oracle.
Patch SHA256:`485145ee519e41ed1941afffb0812ca6d42ff7024e9b94e2e48f9c3b77bd7c52`. The exact original Git patch is retained as
PR236-EXACT-GIT.patch, SHA256:`f85b98ea6c1ef3ea98ad796bd89e58acc2dabf65ebe5ebccbc02891625814d95`.

Full before/after fixtures, exact file/Gitblob hashes, current consumer context,
and the parser diagnostic-only source delta are retained. The original eight
manifest entries,16fixture files and harnessprefix are unchanged. All six new
cases and their byte-exact expected stdout are identical toPR236. The original
author e355patch extra-final-newline history is preserved; PR236 includes root's
format-only newline trim. No new oracle or golden values were generated.

The author had implementation/schema exposure. Expectations are separately
declared from evaluator output, not blinded or independent-provider semantic
oracles. Exposure disclosures are preserved byte-for-byte. SourceCheckReport
serialization is not financial execution or compiler/native/target qualification.

REGRESSION-PLAN.json gives one exact argv selecting only financial_: expected
6pass/0fail/8filtered and12publicbinarycalls. Run only after root constructs/pins
the exact integrated candidate. This preparation executed no product/tests or
providers. Prior6PASS receipts and pending review cover old360f only; current
source/result review and regression remainUNRUN. No parent acceptance changes.
