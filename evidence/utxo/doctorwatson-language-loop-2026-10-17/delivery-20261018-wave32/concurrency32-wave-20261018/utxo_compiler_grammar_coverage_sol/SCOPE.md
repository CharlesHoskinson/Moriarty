# Ordinary reserve, partial spend and refund source candidate

Base 64c6dedff1d99b33c23832330d7911a5fad9ed59; input freeze ce878c845e3b059beff1f441c1f5c5ff2e8cccf24bd250449a5d90039b684e1e. Source preparation only; no product execution.

Actual profile: `language "moriarty/1"`, semantics `moriarty-core/1`. The one run call uses existing UInt application state, Let/Requires/Next/Ensures/Emit, Transfer/Fee/Liability/Discharge. The new source records grossReserved, grossSpent and refunded independently; refund never decrements grossReserved or grossSpent. remaining is ordinary application accounting, not the language's allowance/authorization primitive.

Core/1 has no allowance declaration, consume-allowance statement or owner-authorized allowance consumer. Therefore a claim of allowance enforcement is blocked at that interface. This candidate does not add or stand in for such an interface. It exercises a reserve accounting pattern through an existing local evaluator, preserving movement and duty updates as distinct ordered statements. It defines no security/admission policy or target lowering.

Falsifiable predicate: this exact input's complete financial-prefix observations through the existing library evaluator and actual public ordinary simulate result match independently frozen source-based expectations. Compare every source ordinal's typed value, complete state/balance/duty frame, new effect delta and independently specified cumulative effect prefix. Complete Program/ExprIds, public byte formatting and work are compared only if independently justified; otherwise exact missing fields stay explicit.

Inputs include untouched observer state, multiple balances of another exact asset, a CASH observer balance and an incoming BOND duty. The normal report contract requires them in every normalized pre/post and Step frame. Each movement and duty reduction remains visible separately. A final net balance alone is insufficient.

Public trace: crates/mori/src/commands/simulate.rs:114 parses Core/1; :136 calls mori_core::simulate; :139 prints the ordinary three-field accepted envelope. Library parser.rs calls validate; runtime.rs builds the normalized scenario and ordered statements. The test must invoke the actual CLI and evaluator, never calculate replacement transitions. DTO source identifies fields; independently frozen arithmetic is supplied separately by two_program_financial_oracles. Author implementation exposure is explicit.

Root's future scoped command: cargo test -p mori --no-default-features --locked --test core_reserve_spend_refund -- --nocapture. This is a proposed inspected local reference check, not execution or new authority. Default native is disabled; this path uses no SDK/backend compile/proof/ledger/wallet/provider. Root owns execution and the retained process receipt.

All 148requirements / 292 scenarios / 20 families and original U0–U7/M1–M17/MC/SP/G/full finance/composition/targets remain. Existing passes and SDK/native allocations are unchanged. No hostile/security test or target judgment is authored.
