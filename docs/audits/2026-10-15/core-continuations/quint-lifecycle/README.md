# Continuation lifecycle model: normal invocation evidence

This second abstract model increment has **nine passing normal example tests**
and a separate TypeScript simulation of **32 traces**, with at most four resume
invocations per trace. These counts remain separate from the feature's current 86 Rust
tests and the first model increment's two prefix tests/four one-transition
samples. Full source and actual-results Muse and Opus reviews of the successor
containing this append are **PENDING**. No Rust/compiler correspondence or full
U0–U7, M1–M17, MC/SP/G acceptance is claimed.

[OBSERVATIONS.json](OBSERVATIONS.json) retains exact results, and
[EVIDENCE-INVENTORY.json](EVIDENCE-INVENTORY.json) records original/stored hashes
for all append artifacts. The independently frozen
[oracle](../oracle/ORACLE-CONTINUATIONS.json) remains
`76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640`.

## Source and normal observations

The unchanged [prefix module](continuation_prefix.qnt) sits beside the
[lifecycle module](continuation_lifecycle.qnt) and
[corrected test module](continuation_lifecycle_test.qnt), so their relative
imports resolve within this directory. The original six first-increment files
remain unchanged in `../quint-prefix`. The lifecycle model imports only pure
prefix declarations; it imports no candidate Rust evaluator, compiler or artifact.

The model's `c` record holds the abstract financial boundary, pending site,
captures and committed work. Invocation status, effects and attempted work are
recorded separately. A normal false observation changes the invocation record
while preserving the exact model boundary record. This is model-record identity,
not a statement about serialized Rust bytes.

The nine selected tests passed one example each:
`escrowFalseTest`, `escrowTrueTest`, `escrowFalseThenTrueTest`,
`reserveFalseApprovalTest`, `reserveTrueApprovalTest`,
`reserveFalseSettlementTest`, `reserveTrueSettlementTest`,
`reserveFalseThenTrueTrajectoryTest`, and `reserveBusinessRejectionTest`.
Their literal expected states, effects, original source ordinals and work counts
come from the frozen normal oracle. These are example comparisons, not an
exhaustive check of the finite normal domain.

| Normal model case | Frozen expected observation matched by the selected tests |
|---|---|
| Delivery false, then true | False leaves site 1/capture 10/duty 10 unchanged; true moves escrow 10→0 and bob 0→10, retains present-zero duty and clears the frame; total attempted 13, committed 11. |
| Reserve quote 41, then 43 | False quote preserves the first boundary; quote 43 retains held 42, moves reserve 100→58 and target 0→42, reduces duty 47→5, sets basis 78 and captures quote 43 at site 2. |
| Reserve false settlement, then true | False preserves site 2; true charges fee 2, emits quote 43, leaves balances reserve/target/treasury/observer 56/42/2/42 and positive duty 5; total attempted 42, committed 36 for the false/true/false/true trajectory. |
| Reserve accepted quote 51, then business rejection | The accepted boundary has basis 86, held 42 and duty 5; final commercial predicate rejects at source ordinal 9 with attempted 6, retaining that boundary; trajectory attempted 32, committed 26. |

The exact model/adapter definitions and other preserved financial fields are in
[LIFECYCLE.md](LIFECYCLE.md) and the unchanged [ADAPTER.md](ADAPTER.md).
Their original draft/unexecuted comments describe source preparation time;
the later actual captures below are retained without rewriting those files.

## Exact command history

| Capture | Command and actual outcome |
|---|---|
| `quint-lifecycle-01` | Lifecycle-model typecheck: exit 0. |
| `quint-lifecycle-02` | TypeScript run: exit 0; 32 samples, maximum four resume steps; trace lengths min 2/max 5/average 3.48. |
| `quint-lifecycle-03` | Original test-module typecheck: exit 1, QNT404 at 15 unresolved Escrow/Reserve constructor uses. No tests ran in this command. |
| `quint-lifecycle-03b` | Test typecheck after exactly two direct pure constructor imports: exit 0. |
| `quint-lifecycle-04` | TypeScript test command: exit 0; all nine exact selected normal tests passed once each. |

The simulation reported `resumeObserved` in 32/32 traces,
`falseObservationReached` in 21/32, `completionReached` in 19/32, and
`businessRejectionReached` in 10/32. These are sampled reachability witnesses;
they are not exhaustive property proofs or evidence that every source execution
has been covered. The model typecheck and simulation were not rerun after the
test-only import correction.

Each command's exact start/result/stdout/stderr is retained as lossless gzip
under `../checks/quint-lifecycle-*`. Its result receipt records unchanged source,
tool, Node, directory and cache observations and native actions requested 0.
The only known input change between the failed and corrected test typechecks
was the two-line import correction.

## Preserved failure and pin history

The immutable [LIFECYCLE-INPUTS.json](LIFECYCLE-INPUTS.json) records the original
pre-execution test hash. The exact old test bytes remain in
[historical/continuation_lifecycle_test.before-import-fix.qnt](historical/continuation_lifecycle_test.before-import-fix.qnt):
`9a923041c30dba26a97e80cbd9966da195328ec57867a73b6d75a87a3fe07fc9`.
The corrected test hash is
`dc39c999921e51737bb3db17b5349e3a78fb8687a04408ff01a042261c161d6e`.
The [minimal patch](QUINT-LIFECYCLE-TEST-IMPORT-FIX.patch),
[delta pins](QUINT-LIFECYCLE-TEST-IMPORT-FIX-PINS.json),
[diagnosis](QUINT-LIFECYCLE-TEST-IMPORT-FIX.md), and
[exact two-command packet](QUINT-LIFECYCLE-TEST-FIX-CHECKS.json) are retained.
The historical archived test is preserved evidence, not an additional runnable
module. Existing model, prefix, oracle and literal expectation bytes were kept.

[INPUTS.json](INPUTS.json) remains the original tool/oracle input manifest.
The first-increment tree-digest summary remains under `../quint-prefix`; no Node
binary, package tree, native cache or compiled public binary is copied here.

## Review scope and retained limitations

The [initial Opus classification](../reviews/OPUS-QUINT-LIFECYCLE-CLASSIFICATION.md)
and [import-delta classification](../reviews/OPUS-QUINT-IMPORT-DELTA.md), their
returned canonical identity records, and exact terminal/start/owner receipts
are retained. These classify only the specified ordinary commands and premises.
The delta result states: “I did not load the develop AGENTS status.” It also
states: “Steps 01 and 02 are not rerun or re-attested.” Those limitations remain.

Opus's [initial result](../reviews/OPUS-QUINT-LIFECYCLE-CLASSIFICATION-RESULT.md)
states: “These verdicts don't establish model correctness, a match with the Rust
code, or security acceptance.” Its mathematical-integer and adapter limitations
remain explicit. This append supplies no new model or security judgment. Native
authority and all full roadmap gates remain unchanged. A successor that includes
these files still needs full source and results Muse and Opus review before any
corresponding reviewed-publication claim.
