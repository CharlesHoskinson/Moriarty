# Sol adoption record — final Opus A1

I read DESIGN-OPUS-FINAL.md in full. I agree substantively with its language/compiler architecture and accept A0–A11 as the controlling contract, without substituting any Sol security requirement. The selected compiler is the corrected backend stage stack. Legacy parse/validate remain `/1`-only; syntax-only `pub(crate) parse_v2` is called only by sealed `stage::load`; CLI routing uses `parser::language`. I accept the exact sealed StagedProgram, serialize-only plan types, SegmentEntry/SegmentRun/run_segment interface, compile_staged(src,target,out), module limits and ownership.

Source inspection premises:

- U1: Snapshot has state/balances/liabilities and **no inputs**; A4(d) is vacuous.
- U2: `/3 artifact_bytes` serializes pretty JSON and iterates up to **12** times with checked u32 length and target-byte refusal. `/4` uses that existing procedure.
- U3: Existing six gates, in order: native-proof-generation; circuit-cost-and-k-measurement; fraud-proof-cost-and-k-measurement; authenticated-ledger-frame-and-phase-correspondence; signed-intent-and-predecessor-history-verification; live-ledger-deployment.
- U4: Existing Wire has operand/ty; `immediate(u128)` returns Operand; Builder::public accepts `expected: Option<Operand>` and emits its equality, range check and Impact. Constant expected operands are representable.
- F1 inspection: backend Artifact/CircuitArtifact/Resources and ordered18-element context contain no source_closure/compiler_hash/compiler_id. The backend `/3` whitelist is empty. Ledger native-call closure records remain separate.

One literal interface conflict prevents claiming that every final byte is jointly implementable: clarification §2.4, explicitly retained by final A3, requires **no pending roles for zero-await actions**. Final A6 nevertheless mandates PendingPositions.pre_site/post_site u32 fields and requires those positions to be in range, distinct, and to match A7's pending-site roles. No absent representation is defined. This is a representation contradiction, not a Sol security recommendation. Root has been notified; only Opus can provide its exact disposition. I will not invent a sentinel, optional field, extra role or zero-await refusal.

No other inspected language/compiler obstruction is known. No implementation, build, test, model or native execution occurred. The existing same-packet Muse approval, Opus contract and root-dispatch gates remain; this record adds no discretionary review gate. Full parent exits remain open.


## Final adoption after A1.1

I read DESIGN-OPUS-A2.md in full and unconditionally adopt A0–A11 of DESIGN-OPUS-FINAL.md together with amendment A1.1 verbatim. Required pending_positions: Option<PendingPositions> (serialized null for zero-await), the revised optional frame-helper returns, and removal of the redundant bindings borrow resolve the recorded interface conflict. That conflict is closed; no language/compiler condition remains. Borrow-checker-only signature corrections retain the amendment's exact limits and review reporting. Security authorship and decisions remain Opus-owned. No further design round is requested. Source implementation awaits the existing final Muse approval and root dispatch; execution still requires Opus's closure classification. No code, build, test, model or native execution occurred.
