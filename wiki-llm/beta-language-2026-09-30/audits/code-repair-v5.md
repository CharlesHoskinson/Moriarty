# Final inspection and nominal-authoring repair

All original v4 reviews and v3 developer artifacts remain preserved. This batch
follows the independent Astra whole-code medium finding and Grok developer G3's
reproductions. Parent reproduced the stack failure on a6256-byte AuthoringChecked
source, then verified a staged depth guard before changing the frozen checkout.
The first local test invocation exposed a missing test import; that is not counted
as a meaningful RED result. Corrected tests run against the original installed
v3 artifact fail on all four new defect groups, then pass on repaired production.

| Finding | Current disposition |
| --- | --- |
| Astra v4 M1: deep shared values overflow inspect stack | Projection depth64 checked before recursion; bounded InspectionRejected, originalauthoringStatus; no partial projection |
| G3 H1: bridge skips all domain arguments | Only named source/destination are foreign exceptions; local custody and amount nominal domain checks apply |
| G3 H2: horizon optional headers disagree | Provided domain/asset/signer and operation-local domains align; rounds domain also aligns |
| G3 M3: bridge cap in foreign or wrong nominal asset | Explicit asset or inferred local bridge amount binds cap unit; AMM output net floor has its own output unit |
| G3 M4: untyped hints | Retained duties accept text/string arrays only; signed_floor has explicit text-or-localQty shape. G3's Qty itself is a supported hint, not inherently a type error |
| G3 M5: duplicate share identities | Duplicate domain/share_class/id rejects; kinds/domains remain distinct |
| G3 M6: unsupported scenario not schema checked | Keep support refusal before nonexistent profile evaluator; add NotAppliedUnsupported plus diagnostic. No pretend fixture-validation capability or effects |
| G3 L7: empty policy | Keep explicit partial SpecifiedOnly hints; document no policy/duty-preservation validation |
| G3 L8 and earlier init noise | Existing BETA_INIT_EXISTS repair; other filesystem command errors preserve stderr/nonzero exits |
| Grok v4 L1: BOM handling readability/portability | Raw EFBBBF prefix refusal before fataldecode. Original MDN/WHATWG interpretation in report is not adopted as a source fact |
| Grok v4 L2: qualification field absent on pure authoring/CLI refuses | Preserve contextual API shapes: inspection is authoring-only and no scenario exists; no-financial-execution follows rejected status. Document distinct fields; do not label pure inspection as financial preparation |
| Grok v4 L3: coverage labels diverge | Check and inspect use DelegatedToCoreDuringLocalPreparation consistently |
| Grok v4 L4: VSIX extra ignore files | .vscodeignore excludes .gitignore/.npmignore; packaging reverified |
| G3 preference: readable support output | SpecifiedOnly line says execution unsupported and financial relations open |

The original Source/6/Core/5 bytes and complete financial relation/first-failure
schedule remain unchanged. Hint-string linkage/evidence freshness/earliest-round
financial constraints, policy qualification, share accounting and every nonS0
financial execution remain explicitly open. Tests/model concurrence do not close
signatures, provider authentication, proofs, native history or ledger acceptance.
