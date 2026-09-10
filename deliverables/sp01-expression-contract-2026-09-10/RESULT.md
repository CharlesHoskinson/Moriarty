# Reviewed expression specification

The proposed expression contract now defines all 40 declared constructors and
has independent GPT-6 Astra and Grok 4.6 source and scoped design approvals.
It specifies types, operand order, checked arithmetic, staged writes, failure
ordering, finite work, exact representation and byte/node/depth bounds.

The correction resolves a concrete admission ambiguity: the same maximum UInt64
matrix could previously fit or exceed the byte limit depending on an unstated
encoding. The operative representation now fixes its Args size at 47,367 bytes
and standalone value size at 47,430 bytes. The supplied 65,536/65,537-byte pair
checks the inclusive boundary. Unit, schema cycles, unit declarations, Require
versus Ensure, collection indexes and successful work records also have explicit
rules. Candidate01's failed review and all original source commitments remain
preserved.

The [candidate](source-candidate-02.json) binds 101 files. The
[scoped approval](scoped-source-approval-02.json) binds both independent reviews.
Four inert check commands passed; GPT-6 added 66 independent controls and 16
handwritten corner cases. Both reviewers inspected 80 constructor cases and 20
combined derivations. These are specified expectations and finite arithmetic
checks, not execution of a new interpreter. The document validator checks
structure and references; it cannot establish that an expected value is correct.
Grok returned its terminal review after 465.334 seconds and did not run tests.

This closes the missing expression-table and representation portion of the
archived C04-F1 objection. It does not close the complete objection or register
a successor profile. The 38 financial operations, descriptor/effect/postcondition
coupling, required financial arithmetic, surface Boolean behavior, signing,
genesis funding, contextual histories and remaining full RP01 obligations still
need their own complete definitions and audits. Existing funded runtime and K
files are unchanged. Full SP02/SP03 and mandatory end-to-end Midnight Preview
acceptance remain open.
