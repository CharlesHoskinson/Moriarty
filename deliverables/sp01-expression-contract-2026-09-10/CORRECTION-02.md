# Candidate02 correction

Candidate01 (`e64ce2295b40f3fe9212193dd2b812be5ce47a3bb7931c80c1525eb947ab3187`)
was not accepted or published. Fresh GPT-6 requested source changes under
EX-G6-F1 while approving the scoped design choices. Grok's terminal result
labelled its source PASS_SCOPED but listed required corrections. Both original
receipts and all authored candidate01 bytes were copied before edits; the exact
paths/digests are in preservation-map-01.json. Raw verdicts remain unchanged.

| Finding | Corrected source or explicit remaining boundary |
| --- | --- |
| GPT-6 EX-G6-F1: unspecified value tree makes byte admission ambiguous | New `representation.md` defines exact mathematical type/value/schema/snapshot/descriptor/Core/span/result JSON trees and canonical byte counts, including individual W wrappers. No unnamed size oracle or inherited old wire. Fixed public examples calculate matrix47367 bytes, exact65536/65537 argument bounds and duplicate Record16941 bytes/4355 nodes. |
| Grok F3: Quantity unit namespace | Separate Σ.units roster, explicit lookup before admission; unit declarations cannot be replaced by free strings or asset names. Schema dependency traversal includes Option/Collection/Record and Operation→operandRecord, so recursive descriptors reject TYPE_SCHEMA_CYCLE. |
| Grok F4: Require copied Ensure typing | Require is an ordinary statement with no post reads; Ensure alone is suffix-only and permits post in its condition. Machine-readable rows match prose. |
| Grok F5: index rejection prose | Both accessor cases now reject index2 because actual length is2; capacity3 does not create a third element. |
| Grok F6: success work records | Positive judgments now give workRemaining and separate derivationAccounting.workInitial/derivedWorkUsed. Combined x10→11 has ExpressionPrepared post{x:11}, empty descriptors, workRemaining90 from100. These are specified judgments, not captured runtime result bodies. |
| Grok F7: record error order | Resolve record schema; scan lexical names for duplicates; then compare field coverage; then type/evaluate children. Duplicate-before-coverage is now a rule. |
| Root: Unit and namespace phase | `["Unit"]` is a recognized annotation token but never a value type; ConstructNone(Unit) reaches TYPE_NAME. AbsentType is made explicit as `Record<AbsentType>`, structurally valid but unbound. |
| Rechecking all95 prior derivations: Boolean phase | LitBool(1) has the wrong wire scalar kind and now rejects INPUT_SCHEMA at admission, workUsed0. Other literal-domain cases use structurally valid canonical metadata and retain TYPE_LITERAL. |
| Grok F1/F2/F10/F11/F12/F13 | Surface short-circuit vs strict Core, mixed Amount/Rate/Price arithmetic needed by the swap design, Emit/effect/Ensure coupling, 38 financial equations/signing/full RP01, bounds/work correspondence and syntax elaboration remain open. This correction does not remove target requirements or change funded runtime/K. |
| Grok arithmetic-scope note | The price arithmetic check now independently compares19743/10^4 with decimal1.9743 and is explicitly named numeric-only; orientation follows the typed source convention, not that calculation. |

The representation boundary script checks fixed public trees, not arbitrary
input admission. One initially miscounted Record wrapper expectation16944 failed
and is retained in representation-red-02.txt; the exact measured and independently
recounted result is16941 (payload16909 plus32 wrapper bytes). Corrected checks
are in representation-boundaries-02.json. This was a count correction, not a
runtime or codec execution failure.

All80 constructor cases and15 previous combined cases were re-inspected for
structural/static/runtime masking. The declared assumptions remain visible:
these are mathematical fragments with valid bounded schemas and synthetic zero
spans unless stated. Five new combined cases cover the normative matrix,
byte-boundary pair, undeclared unit and recursive operation schema. A structural
validator cannot establish those semantic derivations; fresh source audits must
check them. The validator is unchanged and still checks40 signature records and
40 pairs only, not the combined-case semantics.

No runtime/codec implementation, K/proof/native/network/private-state operation,
new financial equation, source-profile registration, gate completion, commit or
publication occurs in this correction. Both fresh source audits must approve
candidate02 before publication. The earlier EX-D1–D8 design votes are preserved
at their scoped proposed-design status, not promoted into full language acceptance.
