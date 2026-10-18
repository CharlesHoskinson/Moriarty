# Source6 remittance consumer facts

The rejected review supplied incorrect narration; its listed assertions, Cargo
manifest path and temporary-root name are not product defects. No production
source defect is established from those claims. The original review remains
ineligible; these facts do not approve source or results.

Current inspected checkout: `823ce081c00786a0c55b92af81009f9e7c5e6a16`; draft222 captured execution:
`5acfebe651af47e8edd6741bba298d7336a03b0e`. Whole source files, SHA256/blob pins, exact line anchors and
unaltered public records accompany SOURCE-FACTS.json.

- **F01** The old selected capped regression asserts process exit0/empty stderr, report.status==accepted, two ordered action names pay_tight/pay_loose and each support==local_s0. It does not assert valid, agreement or intent status fields. Anchors: A02, A03.
- **F02** Actual Rust check JSON is CheckReport{status,file,source_hash,agreement:Option<Named>,declarations,actions,diagnostics}; agreement is an object with name/span, not a bare string; actions carry support (local_s0), not status local-s0. No valid boolean exists in this DTO. Anchors: A04, A05, A06, A07.
- **F03** Current Rust check dispatch selects explicit language Core1/Core2 or Beta lexer/parser/checker. The later extraction into check_source preserves the Beta CheckReport consumer. Draft222 uses the earlier direct check.rs branch; this difference is pinned rather than hidden. Anchors: A08, A09.
- **F04** Rust Source6 expansion returns source6 text plus local-stipulation-only qualification. Its own API explicitly does not prepare Core or publish a post-state. Anchors: A10, A11, A12.
- **F05** Explicit Rust CLI expansion source-selects before scenario I/O, calls selected.expand, and writes expansion.source6 bytes to stdout; no TS parser/preparer is called by this Rust route. Anchors: A13, A14.
- **F06** validate_scenario checks domain/asset/allowance-owner/account identities/order and counters; it does not hardcode numeric balances1000/5/7. The fixture supplies those amounts, which emit transports. The zero-fee conditional omits fee credit while all three supplied opening balance rows are emitted. Anchors: A15, A16, A17.
- **F07** Existing checker performs checked multiplication and same-numeric-kind min/max. For the frozen literals75*3=225, tight min(225,210) selects b and loose min(225,250) selects a. No production checker edit was introduced by the five-file candidate. Anchors: A18, A19.
- **F08** TS parseAndLowerSource6 constructs Source6Ast plus S0State/S0Intent/submittedEffects/postHead. AST programId is agreement identity while lowered S0 intent.programId comes from selected.actionId (TransferLiteralFee). UseReplay nonce is converted to domain/signer/nonce JSON tuple. This is a declared DTO lowering, not Rust Program/Target/Artifact production. Anchors: A20, A21, A22, A23.
- **F09** Source6 preparation calls finite prepareMil4S0 and wraps its candidate as PreparedUnqualified with explicitly unverifiedBindings; it does not call a general compiler in this module. Anchors: A24, A25.
- **F10** Finite Core5 preparation calculates candidate balances/allowances/effects and returns candidatePost with work counters updated by one plus named requiredPremises. Captured status/counters are observations; independent oracle deliberately supplies no qualification or measured-work golden. Anchors: A26, A27.
- **F11** Packaged TypeScript beta bridge simulate expands with its own TS frontend, then calls the same prepareSource6S0Unqualified. Draft222 harness exercises Rust expansion -> actual TS consumers, not this packaged beta simulate entrypoint. Anchors: A28.
- **F12** Public Rust simulate routes Core2 specially and otherwise parses explicit Core1; Core parser first expects language, so beta profile CappedRemittance and raw profile Source6 are outside this branch by source inspection. Rust compile likewise parses Core then target/compile_reported only with native feature; native-OFF compilation is unavailable. Neither consumer dispatches the Source6 text/preparer route. Anchors: A29, A30, A31, A32.
- **F13** Draft222 harness feeds untouched actual Rust stdout to actual TS parser/lowerer and preparer, records complete results before assertions, compares full Source6 AST and ordered effects, and checks selected finite financial frames. It intentionally does not assert complete Core post/global replay inventory, post-work, status/qualification or whole Program/Target/Artifact. Anchors: A33, A34, A35, A36.

The old Rust regression asserts `status: accepted`, action names and
`support: local_s0`. Rust `agreement` is a name/span object and the DTO has no
`valid` boolean. Its actual Cargo manifest is the workspace Cargo.toml and its
removed root is `/tmp/doctorwatson-source6-min-50vxmzgd`.

Draft222's later captured two-case Node pass reaches the actual TS parser,
lowerer and finite preparer with untouched Rust stdout. This establishes more
than the earlier text test, at its own pinned source only. It does not validate
the packaged beta `simulate` source producer or current unrelated composition.
The independent23-member semantic freeze is unchanged and deliberately lacks
work/status/qualification and whole Program/Target/Artifact goldens.

The next concrete production gap is public profile integration: Rust check and
Source6 text expansion accept beta transfer source; Rust simulate/compiler use
separate explicit Core grammar. The inspected Source6 adapter constructs finite
Core5 DTOs and a provisional candidatePost rather than a Rust Program/Target or
compiled Artifact. A useful next task must name the admitted public source/
consumer profile and compare its real ordered effects and declared complete
financial state against independent expectations. Production implementation,
execution, design and security decisions are not performed by this facts task.

No provider retry, builds/native checks, source or Git edits were made. All
original acceptance obligations remain open.
