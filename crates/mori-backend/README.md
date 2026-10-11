# Moriarty ledger-9 relation compiler

`compile(&Program, &Target)` validates the public core IR and emits one genuine
ZKIRv3 JSON circuit for each action. It depends directly on Midnight's serializer,
loader and instruction types at revision
`04c9c5d9bcebb8d4427d8589fb54d58a55599c14` (ZKIR `3.0.0-rc.2`). No external checkout,
scenario fixture, project reviewer receipt or scenario success flag is consumed.

The supported relation covers UInt128 and Bool state and typed inputs, ordered
let/require/ensure/update/event statements, exact addition/subtraction/multiplication,
comparisons, Boolean operations, transfers and fees. UInt arithmetic rejects
underflow and overflow. Multiplication decomposes operands into 64-bit limbs;
products and cross terms remain below the native field modulus before the upper
product limbs are constrained to zero. Every intermediate UInt result is range
constrained. Source operations retain source order and all effects are public.

Public values include a two-limb SHA-256 commitment to the canonical program,
target and selected action, the declared predecessor state, action inputs,
opening and closing balances for each touched exact asset/account, every ordered
effect value and all declared successor state. The artifact contains a positional
ABI and an effect/frame description. Each ABI entry is read as a `PublicInput`,
constrained, and explicitly exposed by `Impact`. Upstream `PublicInput` by itself
only assigns witness data. The `public_transcript_inputs` and
`public_transcript_outputs` vectors both follow `public_inputs` ABI order.

The program hash uses the same canonical JSON convention as owner intent
preparation. Original source bytes are a separate intent identity; a compiler
receiving only `Program` cannot infer a byte-for-byte source hash.

Cross-domain assets refuse compilation: every declared asset network must match
the selected target. Liability creation and discharge currently refuse compilation: typed liability
presence, identity and predecessor authentication are not yet lowered. They remain
available in the reference evaluator. There are no native escape hatches,
unconstrained host verdicts, arbitrary recursion or floating point amounts.

These circuits express bounded transition relations. An authenticated ledger
adapter must bind the predecessor, state/balance frame, exact assets and complete
effects to actual ledger reads/writes, preserve untouched accounts and duties,
and implement the selected ledger phases. That correspondence remains an open
gate. Signature/authority/history verification, proof generation, proof-cost and
fraud-proof k measurement, deployment and real settlement are also open gates.
Instruction counts, emitted ZKIR bytes, the complete pretty JSON artifact envelope
size and source replay step counts are measured;
`cost_measured` is false and k fields are null. Successful upstream preimage
checking in tests is witness-generation validation and is not a native proof.

`toolchains/ledger-9.json` selects a local, unqualified example domain. Developers
supply network and anchor identifiers for their environment; the example does not
establish public network compatibility or deployment.

Tests compile independently authored counter and conditional payment programs,
parse their artifacts with the actual pinned loader, and compare valid public
preimages with reference execution. Changed context/state/effects/balances,
false conditions, wide inputs, integer overflow/underflow, malformed IR, unknown
targets and unsupported semantics reject. Integer multiplication also has
randomized correspondence checks. Run `cargo test -p mori-backend --locked`.
