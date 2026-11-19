# II.6 financial obligations for ConstantProductEpoch

The source is `experiments/moriarty-language/spec/examples/swap.mori`, profile
`moriarty-bounded-atomic/1`. Its bound program is
`b00a55b8da611d23c08461a11853151aa50ec7ee8150f9d16afbc66d62e018fa`.
The target is `experiments/moriarty-language/compact/generated/swap/kernel.compact`
with SHA-256 `090b9e2ecf5a0850bee0d46570ca9edb3ac374743e2ed29235913c6e1bb58741`.
The receipt pins the source, bounds, arithmetic include, mapper, metadata,
authority evaluator, tests and proof. The registered runner must check both the
finite examples and the pinned Lean analysis.

## Checked domain and independent expectations

The Lean model projects the six balances onto natural numbers of indivisible
quanta. Amounts in successful Compact calls fit UInt128. Checked subtraction
requires the debit to fit. Asset IDs 0 and 1 have quantum 1. Slots f0/f1 are
pool reserves, f2/f3 trader balances, f4/f5 provider balances. Text IDs 2, 3
and 4 mean pool, provider and trader. See the committed companion metadata.

| Obligation | Statement, premises and boundary |
| --- | --- |
| Accounting, `conservation` | Every finite chain of projected swap and close writes preserves the pool + trader + provider total separately in A and B. Swap credits exactly the input to the A reserve and debits it from trader A. The B reserve pays exactly the output to trader B. Close moves both reserves to the provider. Dropping a single credit invalidates this claim. |
| Gross budget, `cumulative_budget` | Sum all trader A input debits along the chain. The sum plus final trader A equals the initial isolated trader A inventory. If that inventory was funded within one owner-authorized budget, total debits stay within it. A closing transfer or incoming B cannot replenish trader A. With 10,000 A, a 10,000 debit exhausts inventory and 9,999 cannot fund it. Resetting a predecessor or externally replenishing A breaks the premise. The per-call simulator cap has independent exact and one-short cases. |
| Owner and recipient, `owner_authority` | For each transfer in a projected authorized action, the principal equals the designated trader (swap) or provider (close), and its recipient occurs in that principal's permitted recipient list. The source actor and recipient guards and the enclosing outcome-intent checks both apply. A changed actor, changed payout recipient, missing pool or trader destination, mismatched authenticated principal, or failed signature premise prevents authorization. The actor is the instruction authorizer. Pool debits require separate custody authority. |

The finite tests compare both kernel transitions against independent raw state
and effect operands. The exact-balance boundary uses a hypothetical kernel
input. The source simulator uses genuine program genesis with simulated
authentication flags. The fixture's fixed price/output checks restrict reachable
swaps. Trace quantification in Lean is over the accounting projection and does
not claim every projected trace is executable from genesis.

## Conditional relation to ledger effects

1. Compiler correspondence is open. Regenerating identical Compact bytes and
   executing the compiled JavaScript circuits on fixed inputs checks fixtures.
   A proof that all successful source, Compact and ZKIRv3 executions refine the
   Lean projection, including checked arithmetic, denomination and ABI decoding,
   remains outstanding.
2. Custody correspondence is open. The six synthetic balance fields would need
   authenticated matching funded accounts and asset identifiers. Every listed
   transfer would need an equal debit and credit, every pool spend would need
   its own authorization, and protocol or application fees would need explicit
   accounting. A budget spanning calls additionally needs isolated inventory
   and a single chain without replenishment or predecessor reuse.
3. Acceptance correspondence is open. Binding actual signatures, chain,
   program, order, nonce, current predecessor and validity interval requires
   native validation. It must commit the complete authorized effect list and
   state atomically, reject duplicate use and reconcile ledger balances. This
   evidence includes no contract address, funded transaction or verifier key.

The source's lifetime of eight and its reserve for closure constrain source
execution. Static source validation checks the typed program and declared
policies. Neither result proves the three financial statements. The Lean proofs
concern finite chains of successful projected writes. The tests execute two
specific kernel entrypoints and source admission cases. Ledger observation is
still open. See `result.json` for separate replay commands.
