# Iteration 006: checked Add after operand staging

Iteration 006 follows the handwritten `Scalar.tag` leaf in [iteration005](ITERATION-005-SCALAR-TAG-AND-TWO-SPACE-FETCH.md). There is now real new Rust source for `emit_b_scalar_arithmetic_add`, its public instruction wrapper and eight consumed instruction tests. This note records a source iteration. Compilation, actual SDK constraint checks, proofs and settlement are not results of this entry.

The new leaf is `b.helper.Scalar.arithmetic` at `RetRight / Add / Second`. Iteration005 copied operandA's unsigned value into `regA` and moved the helper to Second. The new source reads operandB and relates the supplied post-helper to a checked addition outcome. It does not allocate the returned value or pop the caller frame. `Scalar.exit`, `Scalar.failure` and the later RetRight completion remain separate relations.

## The next board on the assembly rail

The existing enabled-fetch footprint appears twice: one instance reads the program's Const table and one reads the machine's Data table. The effective operandB space selects the active read. A zero-capacity bank has no fetched row; its enabled read is constrained off. The fetched discriminator and sixteen payload bytes feed the arithmetic strip.

Above that strip, sixteen byte columns run from least to most significant. Each column receives `regA[i]`, operandB's byte and the preceding carry. The source emits additions, range conditions and a `LessThan(255, total)` result for the next carry. The last carry is the overflow wire. On the value branch, the supplied post byte and its outgoing carry are related to the column total. This is a source wiring description, not a measured gate count or a claim that the SDK constraints have passed.

The staging selector has three labelled paths:

- UInt operandB and no final carry: `U128Value`, with the supplied sixteen-byte sum.
- UInt operandB and final carry: `PrimError(10)`.
- A supported non-UInt operandB kind: `PrimError(11)`.

The source emits `post.phase = Done` and equations for the staging tag, error code and inactive union columns. It also emits preservation equations for the named helper fields and caller-supplied rest lanes. The four counter strips show remainingWork minus one, consumedWork plus one, helperWork plus one and helperVisits plus one, using sixteen-byte local carriers and no-wrap terminators. These local lanes do not select a serialized B profile or physical layout.

The public wrapper is a separate border around the leaf. Its input order is declared by `BScalarArithmeticFieldRole`; its output list starts with nineteen diagnostic lanes and then carries the input operands. That border is not a ledger statement, a complete transition or a native verifier.

## Source-faithful presentation

A future Three.js board can add the operandB read, sixteen-column carry strip and staging selector to the cumulative assembly rail. The Const/Data fetches should remain visibly reused components. The phase and preserved helper pins belong to the actual source API. `rest` is a caller partition, so it should remain an off-board connector rather than an invented complete image or stack.

Selection light can identify a source component. Execution light must wait for a separately captured applicable result; it must not animate the current unrun tests as circuit activity. The full source and source conditions should remain reachable from the board. No physical rows, fees, speeds or proving costs are inferred from the byte drawing. Earlier boards retain their own historical source and result bindings.

Visitor copy continues to assume the finished, accepted Sherlock v1 launch. Current internal status belongs in the adjacent provenance record, not in a developer permission flow. The site is not implemented or deployed by this note.

## Grounding and limits

The exact new emitter is 35,652 bytes, SHA256 `424da282d4580279db1d104c6ad51690d2cc94050ff5b5efe00b848458ae39b9`. The corrected tests are 35,840 bytes, SHA256 `e2b57fedcee4bfcf8375385421240ae02c8b8adaaef1ddb6c2fc6c9b9643a430`. The parent afterimage registers the new module; the archived iteration005 source is not silently registered by this addition.

The actual author was Gemini4ArgonHIGH under the standing Opus outage fallback. Root retained the complete initial source and the later exact four test-plumbing edits. The production emitter did not change during that correction; all eight test-function bodies remained byte-identical. Syntax parsing is observed. Rust typechecking, the source-authored tests and actual SDK constraint acceptance remain unobserved. Two fresh non-author source reviews are prepared separately; author completion is not either review vote.

The full physical B image, complete StepInv and exactly-one dispatch, AB/P0–P3/PS, program/image/statement bindings, SDK field/gadget assumptions and the untouched image complement remain external obligations of this local fragment. No general39/full96, canonical/native, financial, proof or target acceptance is claimed.

This is a Root/Sol factual source-and-concept note, not an Opus creative reply or a new security judgment. Publication and integration remain Root-owned.
