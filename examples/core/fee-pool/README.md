# FeePool monetary component

`fee-pool.mori` is a finite source component for one closed epoch, two operator
seats, one DA seat, one watcher, one ordinary cost receipt, one stipend receipt,
and one watcher reserve spend. It uses existing `moriarty/1` syntax. The configured
asset is `{network: local, issuer: fee-pool-source, reference: SERVICE, scale: 0}`.
The accounts and nine duty identities are source literals.

Funding moves the gross amount from `funding-source` into `fee-pool`. Ordinary
costs and the watcher stipend create immutable reimbursement duties before the
surplus split. Settlement independently floors 55/10/5/15 percent of that original
surplus; Midnight receives the residual. The watcher share remains held until an
explicit kind7/8 reserve spend. Watcher stipends remain metered costs.

Operator and DA allocations use the original category total and aggregate weight.
They create duties even for zero allocations, and every registered reward seat
must be allocated before finishing. Only positive entitlements can be claimed.
Operator/DA claims require the numeric epoch-close plus30day threshold; other
claims use epoch close. Claims remain available after finishing. Each claim
transfers the immutable amount to the fixed recipient and then discharges the
exact duty. A discharged duty remains present with amount0.

Backstop refill uses the current target. Lowering that target preserves existing
custody. Finishing routes operator/DA residual to backstop and then an L2 duty;
it preserves earlier unclaimed entitlements. No action emits a `Fee` effect.
Application fee intake, held reserve and reimbursement are separate from DUST,
service-cost accounting and Capacity Exchange proceeds.

All numeric inputs and all32 numeric predecessor fields must be below `2^64`
before source arithmetic. Untrusted quotient/remainder witnesses satisfy
`quotient * denominator + remainder == originalTotal * weight`, with a positive
original denominator and `remainder < denominator`. Products use checked Core
u128, so the full reference U64 domain is admitted without a division primitive.
The same constraints handle nondivisible values and products above u64.

`carried` is unallocated custody: gross at intake, gross minus reserved costs
before settlement, operator plusDA remaining after settlement, and0 after finish.
From the clean genesis root, source transitions preserve:

```text
gross = claimedTotal + sum(current nine duty amounts) + backstop + watcherHeld + carried
```

The source guards and transition arithmetic establish history invariants from the
clean initial state. A caller-supplied local snapshot does not authenticate that
history or its starting balances. No claimant can replace an action's asset,
recipient, amount, receipt, pool or epoch through an input.

## Run the local consumers

Build the ordinary `mori` executable using the repository's Rust toolchain, then:

```sh
mori test examples/core/fee-pool
mori simulate examples/core/fee-pool/fee-pool.mori --action intake --scenario examples/core/fee-pool/cases/intake-funded.json
```

The root manifest contains a bounded selection of complete accepted results and
refusals. Each accepted-case directory also contains a one-case manifest, keeping
individual command output within the existing serialization limit. The Rust
consumer suite invokes those manifests, exercises every source action, compares
complete expected state and ordered effects, and tests actual refusal behavior:

```sh
cargo test -p mori --test fee_pool_component -- --test-threads=1
```

Cases use independent exact arithmetic expectations, including surplus99's
Midnight residual18, original-weight allocation in either order, all nine positive
claim consumers, present-zero duties, full U64 settlement and large weighted
products. Logical work-cap tests measure only the existing local statement and
expression cost profile.

## Public source-static compilation

The bindings contain public fixture identities and encodings only. They commit to
this exact source/Program/ABI and its nine actors/duties. A source edit requires
new matching public commitments. The following intended consumer commands are currently blocked by the shared
`layout::admit_bindings` restriction `one-public-payer`: unsigned export requires
every source debit to use the same bound payer. This program deliberately funds
from `funding-source` and later claims from `fee-pool`. General source-static
compiler repair is required before positive export/reopen can be reported.
Preserve those distinct funding/custody owners. Use a fresh output directory when
that repair is available:

```sh
mori compile-call examples/core/fee-pool/fee-pool.mori --all-actions --authorization unsigned --static-bindings examples/core/fee-pool/static-bindings.json --work-limits examples/core/fee-pool/work-limits.json --out /tmp/fee-pool-static-new
mori compile-call examples/core/fee-pool/fee-pool.mori --all-actions --authorization unsigned --static-bindings examples/core/fee-pool/static-bindings.json --work-limits examples/core/fee-pool/work-limits.json --inspect-static /tmp/fee-pool-static-new
```

A successful future export would be `source-static-unproven-uninstalled`;
reopening would compare the exact public artifacts and recompile them. Neither
would establish a proof, installation or ledger acceptance. Current whole-program
signed-v2 sole-owner policy must also refuse this component. At present the common
`one-public-payer` guard rejects it before the signed policy can reach its own
`sole-payer` check. A valid signed-policy negative consumer remains required after
the unsigned-static repair; malformed policy rejection is insufficient. The
supplied work limits bound public static serialization and confer no native
execution or proving authority.

The primary source has54 states,23 actions,1 asset,9 duties,4,944 expressions and
7,667 lexer tokens. Its largest action has47 statements; source size is55,573
bytes. These fit the existing78-declaration,8,192-expression/token,1,024-statement
and65,536-byte source bounds. Consumer tests measure these properties.

## Contrasting source

`contrasting.mori` is a separately authored four-action financial example named
`IndependentFeeSplit`. It uses distinct source state/action/duty names, the exact
asset `{local, independent-fee-source, ALT, 0}`, four different actors and a
reversed beneficiary declaration order. It funds99, constrains the same original
55/10/5/15 floors, creates duties54 and18, holds the other27, and claims through
actual transfers/discharges. Its own bindings use a different token identity.

This smaller example exercises the same parser/interpreter/static consumer with
different source financial identities and nondivisible values. It does not claim
a second complete23-action FeePool instance. Use `contrasting.static-bindings.json`
with that file for the intended unsigned compile/inspect commands. Its funded
intake and later claims encounter the same current shared payer restriction.

## Retained scope

Importer/governance/beneficiary authorization, canonical normalized imports,
authenticated block time and recipient-secret hashing remain external premises.
`observedAt` and `closeObservedAt` enforce numerical thresholds only. Fixed receipt
slots do not implement general cryptographic registries, cross-instance replay or
multi-epoch global reserve spending. The monetary profile excludes other cost
kinds and `signal_step_down`.

The associated Quint model supplies finite shared-state QA; deterministic model
results do not establish correspondence to a native ledger. The earlier full
sampled-model timeout has an unknown invariant verdict and unknown witness counts.
Full M11, Capacity Exchange, general registries/authority/clock interfaces, native
withdrawal/custody correspondence, recursive proofs, resource qualification and
actual financial ledger acceptance remain open.
