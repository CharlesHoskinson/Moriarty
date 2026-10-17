# Literal bridges and ledger interfaces

Composed source merged in PR168 at `011284561884346412a7c7eaa36a195310924878`. SDK compilation/tests and native key/proof/verify/settlement remain UNRUN. SDK calls need separate execution authority.

The [composed archive](../../results/sherlock-composed333-20261017/) records UInt7/fuel1/stack16: the same 333 carriers match independent named/ordered expectations. Initial lexical dep-info FAILED after host compilation; continuation normalized exactly ten members and reused the artifact without host recompilation. Ordinary review approved scoped publication. Context limbs are absent/unbound, not stripped/zero-filled. Checked335, full typed traces, canonical projectA, native and target remain unobserved.

## Statement carriers

| Surface | Types and declarations | Statement |
| --- | --- | --- |
| SDK-free host schema | `literal_host::{LiteralCapacities,LiteralSourceIdentity,ImageValues,LiteralValues,LiteralTransitionRequest,LiteralLayout}`; [declarations](../../../crates/mori-backend/src/sherlock_machine/literal_host.rs#L246) | A102: two context limbs, complete program, pre, post, tag 3, charge 1 |
| Literal relation | `literal_bridge::{CircuitMode,LiteralClaim,LiteralProofBundle,LiteralVerification}`; [source](../../../crates/mori-backend/src/sherlock_machine/literal_bridge.rs#L28) | One charged `Eval(Literal)→Return`; no continuity or finish claim |
| Composed relation | `literal_complete_bridge::{LiteralEnvelopeValues,LiteralCompleteValues,LiteralCompleteExecutionRequest,LiteralCompleteLayout}`; [source](../../../crates/mori-backend/src/sherlock_machine/literal_complete_bridge.rs#L95) | Two context limbs, program, init, intermediate, done, envelopes `(3,1)` and `(0,0)` |

`LiteralCapacities` carries `program: [usize;15]`, `state: [usize;6]`, `stack_limit: u128`. `LiteralSourceIdentity` carries three caller-asserted SHA-256 strings: K source, reference source and program source. They are metadata, not canonicality proofs. `ImageValues { root: Vec<u128>, tables: Vec<Vec<Vec<u128>>> }` contains public lanes.

Layouts expose `new(capacities, identity, max_statement_lanes)` and `statement(values)`. Their `emit(CircuitMode)` emits SDK IR; composed also exposes `emit_with_block`. `LocalPublic` uses public inputs; `LedgerCall` uses communication commitment. Identities depend on exact source closure. Ledger: `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`; ZKIR: `3.0.0-rc.2`.

`build_literal_complete_execution_request`, `build_checked_literal_complete_execution_request` and `produce_checked_literal_complete_execution_request` adapt the producer; [signatures](../../../crates/mori-backend/src/sherlock_machine/literal_complete_bridge.rs#L915). Program images must agree and `A102.post == A80.pre`. Intermediate/done remain noncanonical Constant-space Step images; `projectA`, canonical capacities, full StepInv, G4 and general dispatch remain open.

## Ledger and transaction APIs

`sherlock_literal` and `sherlock_literal_complete` export `derive_keys`, `empty_verified_transcript`, `raw_call_intent`, `prove_call`, `verify_call`. Distinct carriers: `SherlockLedgerKeys` / `SherlockCompleteLedgerKeys`, `SherlockCallOpening` / `SherlockCompleteCallOpening`, `SherlockKeyLimits` / `SherlockCompleteKeyLimits`, `SherlockLocalCallVerification` / `SherlockCompleteLocalCallVerification`. [Exact signatures](../../../crates/mori-ledger/src/sherlock_literal.rs#L63), [composed](../../../crates/mori-ledger/src/sherlock_literal_complete.rs#L83). Limits supply degree, SRS/key bytes and statement lanes. Keys expose operation/IR/VK; imported bytes cannot construct them.

Transaction modules export deployment/call builders, binding/well-formedness checks, extraction and extracted-call verification. [Literal signatures](../../../crates/mori-ledger/src/sherlock_transaction.rs#L219), [composed](../../../crates/mori-ledger/src/sherlock_transaction_complete.rs#L224), [request wrappers](../../../crates/mori-ledger/src/sherlock_transaction_complete.rs#L998). Deploy config supplies entry/data/authority/nonce; transaction config supplies network/segment/TTL/binding/seed. Explicitly unfunded or external-fee-merge policies establish no fee sufficiency.

Calls carry guaranteed empty-program `Transcript(v2.3)`, exact runtime gas and no fallible transcript. Callers supply `ChargedState`, `CostModel`, expected raw intent, address/entry point and off-chain opening; freshness, source/chain association remain assumptions. Tagged Rust decoding establishes no provider/indexer interoperability. Gas equality does not authenticate state; local intent hashes are not chain transaction IDs. [Result qualifications](../../../crates/mori-ledger/src/sherlock_literal_complete.rs#L478), [explanation](../explanation/relations-witnesses-ledger.md).
