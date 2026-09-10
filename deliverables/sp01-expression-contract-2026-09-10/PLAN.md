# SP01.3 expression contract implementation plan

> Execute inline under the existing AFK authority. Independent fresh GPT-6 and Grok reviews decide proposed choices; no self-approval, commit or publication before those reviews.

Goal: remove the missing expression-signature portion of C04-F1 for all 40 declared constructors. Architecture: finite typed value algebra and declarative expression/statement judgments, a complete machine-readable signature table, independently derived cases, and one small structural validator. No production evaluator or K change. Python standard library only for structural checks.

The prior candidate's eight signature rows and 40-name declaration are inputs, not an accepted profile. Preserve all archived bytes. The alternatives were extending the funded runtime immediately (would guess missing types/semantics) or adding more fixed K packets (would leave the constructor blocker). The selected work completes the common expression layer first. New decisions are proposed; inherited pre/next/post, residual-duty and no-host-acceptance constraints remain intact.

Files created:
- `experiments/moriarty-language/spec/successor/semantic-contract.md`: scoped proposal, finite values, evaluation contexts, work, rejection/frames, unresolved financial boundary.
- `experiments/moriarty-language/spec/successor/static-semantics.md`: total typing/overloads and whole-action admission.
- `experiments/moriarty-language/spec/successor/expression-signatures.json`: exact 40 constructor records, types, decisions, diagnostics and references.
- `experiments/moriarty-language/spec/successor/expression-cases.json`: independently derived positive/rejection cases for every constructor and staged discriminators. These are derivations, not runtime executions or independent external reviews.
- `experiments/moriarty-language/spec/successor/check-expression-contract.py` and `test-expression-contract.py`: structural document validation only; no semantic evaluator.
- This deliverable: plan, checks, source provenance and frozen candidate for both reviews.

Steps:
- [x] Specify finite values and evaluation/typing rules. Use exact proposal widths/bounds; do not silently apply them to current syntax/funded profiles. Resolve short-circuiting, signed rounding, post reads and observation effects explicitly.
- [x] Populate all 40 signatures and independent case pairs, covering inherited staging, wrong asset, duplicate writes, post/next misuse, first of two failures and work exhaustion.
- [x] Write validator tests first: complete document accepted; missing/duplicate constructor and dangling type/operand/source/case references rejected. Run with absent validator and retain initial failure.
- [x] Implement only document structural checks; run focused standard-library tests and retain output. Distinguish semantic derivations from machine-checked structure.
- [x] Check constructor coverage, links and immutable input hashes; freeze exact candidate. Keep financial operation equations/signing history/full RP01 and SP02/SP03 gates open.

Semantic choices to review: strict left-to-right Boolean operations; same-type checked integer overloads; indexed amount/share operations; quantity scale/unit rules; explicit post-view on ReadPre restricted to Ensure; closed trusted-by-assumption observation snapshot with labels (no authentication claim); finite operation descriptor for Emit with no automatic financial execution; one work unit per entered expression/statement node, rollback of candidate state but retained diagnostic work.
