# Runtime correction02: finite type metadata

This implementation repairs independent finding EXRT-G6-01 against runtime
candidate01. The original review remains REQUEST_CHANGES for its frozen bytes;
this author correction is pending fresh independent review.

The original 512-level JSON transport guard rejected ConstructNone with an
Option chain of509 layers. That term has one Core constructor and one value
node; its Core is5,716 bytes and its W value encoding is5,638 bytes. Both are
within the normative65,536-byte bounds. The review is correct: representation.md
excludes type metadata from mathematical value nodes/depth and defines Core
depth by constructor nesting. No512-level type metadata restriction exists.

The corrected canonical writer and JSON-tree size counter use explicit stacks.
Type shape and resolution traverse Option/Collection metadata iteratively;
collection capacity checks retain their original inner-to-outer order. Schema
cycle dependency discovery also unwraps these chains iteratively. JSON.parse
has no reviver. The transport remains byte bounded before parsing. The existing
2,000,000-byte request ceiling exceeds the four65,536-byte JSON components plus
the at-most6x escaped65,536-byte source and fixed envelope fields. Independent
Core/schema/snapshot limits still apply after exact canonical decoding. This
repair adds no metadata depth cap and changes no normative contract bytes.

The largest Option chain fitting the ConstructNone Core encoding has5,947
layers:65,534 Core bytes and65,456 W bytes, with constructor and value depth1.
It now succeeds with one unit of work. The next layer exceeds Core bytes and
rejects INPUT_BOUND with workUsed0. Deep static errors still precede evaluation,
including errors in an unselected branch; work0 still yields WORK_EXHAUSTED for
a statically valid expression. No caller callback or host evaluation is added.

One existing adversarial test changes its expectation from INPUT_BOUND to
INPUT_SCHEMA: a10,000-level array is a malformed request envelope within the
transport byte bound. It no longer hits an unsupported generic depth gate.
The old test and both changed runtime files are preserved byte-for-byte under
candidate-01-original; preservation-map-02.json resolves all176 original pins,
173 at unchanged paths and3 in the archive. The original candidate, patch,
review, and independent probe/reproducer files remain unchanged. Root-authored
admission and schema ownership controls remain unchanged.

Evidence is actual local execution. depth-reproducer-red-02.log records the
unchanged independent reproducer failing on candidate01; depth-tests-red-02.log
records the initial14 depth tests before the repair. The final suite adds3
structural typing/dead-branch controls, for17 new public-API tests. The final
runs pass205 focused tests and441 total language tests; tsc passes. The original
independent probe script passes all23,435 assertions when rerun by the author;
its508/509/600-layer cases now all report ExpressionValue. This rerun preserves
independent test authorship but is not a fresh independent result audit.

The API remains createExpressionContractV1(schemaCanonicalJSON).evaluate(requestCanonicalJSON).
This is the local forty-constructor expression contract/1 boundary. The funded
runtime and registered profiles are unchanged. This work establishes no full
financial operation semantics, source elaboration, K correspondence, proof,
private handoff, signing authorization or ledger acceptance. Mainline publication
and fresh GPT-6/Grok reviews remain the parent's responsibility.
