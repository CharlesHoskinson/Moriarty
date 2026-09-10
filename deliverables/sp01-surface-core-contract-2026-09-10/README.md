# Boolean expression contract revision

The canonical proposed expression contract now identifies itself as
`moriarty-expression-contract/1`. And/Or use the approved short-circuit choice;
all forty constructor names, whole-action admission and financial statement
rules remain. The exact revised source candidate still requires fresh result
review. No runtime, K definition or registered execution profile changes here.

The [design approval](scoped-design-approval-01.json) binds the original proposal
and its two independent votes. Those are design votes, not result audits of /1.
The former GPT-6 reviewer authored this implementation and cannot independently
approve it. A new GPT-6 reviewer and Grok must review source-candidate-02.json.

The six changed files from the published strict /0 candidate are preserved
byte-for-byte under `strict-contract-0/`. The [preservation map](preservation-map-02.json)
resolves every one of the old candidate's 101 pins, using unchanged paths for
the other 95 files. It also resolves the old proposal's nine source pins. Old
receipts and strict rejection cases retain their original identities and hashes.

The changed positive Boolean cases have revision-specific identities. The old
`strict-and-failure` stays a historical /0 rejection. The new
`BC1-skip-and-denominator` returns false after two entered nodes under /1.

The [48 specified fixtures](../../experiments/moriarty-language/spec/successor/boolean-cases.json)
include complete Core trees, named input profiles, work derivations, original
node paths, source and synthetic spans, and whole-action Require/Ensure cases.
The [coverage map](case-coverage-02.json) links all fourteen proposal cases and
all nine independent review corners. Source/Core pairs are manually specified;
no parser, elaborator, evaluator or K trace is claimed.

Run the focused checks from the repository root:

```text
python3 experiments/moriarty-language/spec/successor/check-expression-contract.py
python3 experiments/moriarty-language/spec/successor/test-expression-contract.py
python3 deliverables/sp01-surface-core-contract-2026-09-10/check-preservation-02.py
git diff --check
```

The checker verifies document structure and specified accounting, not semantic
truth. One negative control deliberately changes a truth-table expectation and
confirms that the structural checker does not claim to detect it. Fresh source
review must check mathematical results and error derivations independently.

During author verification the new depth-bound fixture initially misspelled
Not's operand as `operand`; the signature declares `value`. The structural
checker rejected that fixture, the author corrected it, and the subsequent
twenty controls passed. The first failed run is not counted as a passing result.

The correction in candidate03 resolves independent finding A-1. A malformed
input span remains unchanged in the fixture, but the diagnostic uses valid
synthetic [0,0) while retaining nodePath [1], INPUT_SPAN and workUsed0. Valid
original spans still propagate exactly. The original124-pin candidate and its
rejected diagnostic remain retrievable through preservation-map-03.json.
