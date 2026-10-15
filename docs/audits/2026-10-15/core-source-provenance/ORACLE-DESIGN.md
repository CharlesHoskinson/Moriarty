# Independent normal source provenance oracle

Frozen **2026-10-15T01:59:26.720394+00:00**, base `eb2577d6c67684b950288d769ea2fcb91e731bf1`. Independent NONAUTHOR normal functional work; **specified-only**. Installed development skill, new checkout AGENTS/status and prior diagnosis were restored. No product build/run, compiler/evaluator import, SDK/native operation, security review or source edit occurred. Python serialized fixed literal expectations and public file hashes only.

One falsifiable predicate: the existing public `unsigned-mapped-roles` compile-call export/reopen preserves each admitted source statement's exact source index/root expression and actual IR instruction/Impact-group/replacement positions, with unchanged IR for fixed source/bindings. The source map remains correspondence-support metadata; this predicate does not establish runtime or native semantic equivalence.

Minimal fixture: [oracle-normal.mori](oracle-normal.mori), SHA-256 `f8e9f77d16743637a1a08d3e23a821673180d2de4ad2f4f2372fd714ab8f455d`. It declares `base: UInt =3`, PAY matching the existing public static fixture, and one action `mapped(amount:UInt)`:

```text
0 transfer(PAY, "alice", "treasury", 2)
1 fee(PAY, "alice", "treasury", amount)
2 let local = base + 1
3 transfer(PAY, "alice", "treasury", local)
4 fee(PAY, "bob", "treasury", 2)
```

Adjacent Transfer/Fee to the same recipient retain different kinds and source steps. All four payments accumulate the same token/treasury claim, including the last payment's different payer. Literal/input/state-let amounts are distinct. Parser allocation gives expression0=literal2,1=Name amount,2=Name base,3=literal1,4=Add(2,3),5=Name local,6=literal2. Source state initializers do not consume expression IDs. Root5 belongs to the transfer's Name local; producer Let root4 remains separate context.

All ranges below are zero-based and half-open. Instruction indices count every IR instruction; group indices count only Impact instructions.

| Source step/kind | Root expression | IR instruction range | Group range | Impact instruction indices |
| --- | --- | --- | --- | --- |
| Prelude, no source step | none | [0,7) | [0,3) | 1,2,6 |
| 0 Transfer literal2 | 0 | [7,41) | [3,16) | 28 through40 |
| 1 Fee input | 1 | [41,75) | [16,29) | 62 through74 |
| 2 Let base+1 | 4 | [75,86) | [29,32) | 78,79,80 |
| 3 Transfer local | 5 | [86,120) | [32,45) | 107 through119 |
| 4 Fee literal2 | 6 | [120,154) | [45,58) | 141 through153 |
| Final Output | none | [154,155) | empty | none |

Each payment emits the complete13-operation claim template: Swap,Idx,Dup,Push,Member,Popeq,Dup,Idx,Popeq,Push,Push,Ins,Swap. Its widths are `[1,4,1,19,1,4,1,18,4,19,5,1,1]`, totaling79 fields. Replacement offsets occur only in local groups5/8/10 at offsets3/3/4 for member/previous aggregate/new aggregate respectively. All other group fields remain static template operands. The three old-lookup groups6/7/8 use the old guard; the other ten use the positive-amount guard. The full static13-group structure remains present even for the first claim.

| Source payment step | Dynamic global group indices | Dynamic instruction indices | Operation-field positions (no prefix) | Operand identifiers |
| --- | --- | --- | --- | --- |
| 0 | 8,11,13 | 33,36,38 | 35,58,82 | native9,native12,native7 |
| 1 | 21,24,26 | 67,70,72 | 114,137,161 | native21,native24,native19 |
| 3 | 37,40,42 | 112,115,117 | 206,229,253 | native36,native39,native34 |
| 4 | 50,53,55 | 146,149,151 | 285,308,332 | native48,native51,native46 |

Operand spelling is `%native.N`. Add2 to an operation-field position only when addressing the public transcript including its two prefix fields. Typed source input `%input.0` is a separate coordinate. The state read belongs to Let step2/root4: groups29/30/31 have widths1/8/4 and origins state-read/state-read/state-opening; instruction80/group31 replaces offset3 with `%native.27`, operation-field position176 (prefix-inclusive178). Its Add result is `%native.28`, reused by source Name local at step3.

These widths are calculated from the pinned public SDK encoding source, not from a generated compiler result. Scalar AlignedValue has3 fields (alignment length, one alignment atom, one value), giving scalar Popeq4/Push5. A32-byte hash has4 fields, so the state lookup Idx is1+3+4=8. A claim key has6 alignment atoms and10 value fields, total1+6+10=17, giving claim-key Idx18/Push19. Operation opcode, Cell header and aligned representation account for the respective additions. The read2+1+3+5 arithmetic/range instructions and four fixed34-instruction payment blocks give7+4*34+11+1=155 total instructions,58 groups and335 operation fields (337 with transcript prefix). Action typed input_count is1. Full exact per-group widths, offsets, guards and replacement operands are in the JSON.

Hand financial example ([public scenario](oracle-normal-scenario.json)): amount7, base3, local4. Ordered effects are Transfer2 alice→treasury, Fee7 alice→treasury, Transfer4 alice→treasury, Fee2 bob→treasury. Alice100→87, bob50→48, treasury0→15; total150 is preserved. Gross debits15 comprise transfers6 and fees9; treasury net receipt15. State base3 persists, liabilities stay empty, and no reserve dust arises. Shared claim totals are2,9,13,15. Predicted positive guards are all1; old guards are0,1,1,1. Predicted member/previous/new dynamic values are (0,0,2),(1,2,9),(1,9,13),(1,13,15). These are arithmetic expectations, not native execution observations.

For actual public binding setup reuse `examples/core/static-compile/financial-duty.bindings.json` PAY token and alice/bob/treasury public role points, clear duties, preserve existing domain/toolchain/bounds, and recompute source/program/ABI commitments using the existing helper. Include `p.states` (base UInt3), unlike the older stateless helper's empty state list. Setup parsing/hashing does not supply expected maps. This oracle deliberately does not invent an application compiler or new manifest schema.

Proposed verification remains unexecuted: after exact command classification by root/Opus, use existing public mapped-role export and fresh public inspect-static reopen. Structurally read tagged IR and compare these exact independent source roots, ranges, instruction variants, group widths and replacement offsets/operands. Require unchanged baseline IR bytes for fixed inputs; no baseline IR digest is claimed because compiling that baseline has not been authorized/executed here. Keep the full financial oracle separate from metadata observations.

Scope limits: one normal fixture does not close admitted-grammar coverage, U0/U2 or any native/ledger/target/release exit. No resource policy, security requirement/test/judgment, proof or SDK result is supplied. Manifest field names and compatibility treatment remain root/Opus decisions; this file fixes mathematical expectations, not a parallel framework. Do not tune these expectations to implementation output. If actual emission contradicts a row, preserve the disagreement and examine its source before changing this freeze.

Machine-readable freeze: [ORACLE-DESIGN.json](ORACLE-DESIGN.json), SHA-256 `33e37dc74dd01bfec329b38eee3adeea462cdb4514e24f0116b75a3081cc1c88`. It records input/source evidence hashes, exact rows, complete normal financial pre/post/effects and explicit limitations.
