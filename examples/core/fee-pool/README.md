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
Operator/DA claims require the numeric epoch-close plus 30 day threshold; other
claims use epoch close. Claims remain available after finishing. Each claim
transfers the immutable amount to the fixed recipient and then discharges the
exact duty. A discharged duty remains present with amount 0.

Backstop refill uses the current target. Lowering that target preserves existing
custody. Finishing routes operator/DA residual to backstop and then an L2 duty;
it preserves earlier unclaimed entitlements. No action emits a `Fee` effect.
Application fee intake, held reserve and reimbursement are separate from DUST,
service-cost accounting and Capacity Exchange proceeds.

All numeric inputs and all 32 numeric predecessor fields must be below `2^64`
before source arithmetic. Untrusted quotient/remainder witnesses satisfy
`quotient * denominator + remainder == originalTotal * weight`, with a positive
original denominator and `remainder < denominator`. Products use checked Core
u128, so the full reference U64 domain is admitted without a division primitive.
The same constraints handle nondivisible values and products above u64.

`carried` is unallocated custody: gross at intake, gross minus reserved costs
before settlement, operator plus DA remaining after settlement, and 0 after finish.
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

The root manifest contains funded intake and the 75 refusal cases. The complete
independent set contains 162 unique cases: 87 accepted and 75 rejected; see
[cases/COVERAGE.md](cases/COVERAGE.md) for the exact finite-profile coverage. Each accepted-case directory also contains a one-case manifest, keeping
individual command output within the existing serialization limit. The Rust
consumer suite invokes those manifests, exercises every source action, compares
complete expected state and ordered effects, and tests actual refusal behavior:

```sh
cargo test -p mori --test fee_pool_component -- --test-threads=1
```

Cases use independent exact arithmetic expectations, including surplus 99's
Midnight residual 18, original-weight allocation in either order, all nine positive
claim consumers, present-zero duties, full U64 settlement and large weighted
products. Logical work-cap tests measure only the existing local statement and
expression cost profile.

## Public source-static compilation

The bindings contain public fixture identities and encodings only. They commit to
this exact source, Program, ABI, actors and duties. A source edit requires new
matching public commitments. Funding from `funding-source` and later claims from
`fee-pool` remain distinct source operations.

Use the explicit `unsigned-mapped-roles` mode for the intended positive
source-static consumers. It checks each payment's source and recipient against
the public role bindings and records each action's debit-source roles. This is an
opt-in export format; it supplies no authority to debit those roles. Successful
export and exact reopening must be established by current consumer results.

Use a fresh output directory for each program:

```sh
mori compile-call examples/core/fee-pool/fee-pool.mori --all-actions --authorization unsigned-mapped-roles --static-bindings examples/core/fee-pool/static-bindings.json --work-limits examples/core/fee-pool/work-limits.json --out /tmp/fee-pool-mapped-static-new
mori compile-call examples/core/fee-pool/fee-pool.mori --all-actions --authorization unsigned-mapped-roles --static-bindings examples/core/fee-pool/static-bindings.json --work-limits examples/core/fee-pool/work-limits.json --inspect-static /tmp/fee-pool-mapped-static-new
mori compile-call examples/core/fee-pool/contrasting.mori --all-actions --authorization unsigned-mapped-roles --static-bindings examples/core/fee-pool/contrasting.static-bindings.json --work-limits examples/core/fee-pool/work-limits.json --out /tmp/fee-pool-contrast-mapped-static-new
mori compile-call examples/core/fee-pool/contrasting.mori --all-actions --authorization unsigned-mapped-roles --static-bindings examples/core/fee-pool/contrasting.static-bindings.json --work-limits examples/core/fee-pool/work-limits.json --inspect-static /tmp/fee-pool-contrast-mapped-static-new
```

The mapped mode uses these distinct public artifact identities:

| Artifact | Profile or format | Qualification |
| --- | --- | --- |
| Bundle | `moriarty-mapped-role-source-static-bundle/1` | `source-static-mapped-roles-unproven-uninstalled-unauthenticated` |
| Per-action relation | `moriarty-mapped-role-source-static-relation/1` | `source-static-mapped-debits-unauthenticated` |

Mapped export status uses the bundle qualification. Mapped inspect status starts
with `source-static-recompile-admitted` and includes that same qualification.
Reopening checks the exact supplied bytes against fresh source compilation. The
per-action `debit_sources` inventory contains full public role-binding rows for
Transfer/Fee source roles only, deduplicated and sorted by role name; actions
without payments have an empty array. Recipients remain bound in the source ABI.

These artifacts do not establish debit custody or funding ownership,
role/beneficiary authorization, authenticated predecessor/deployment, economic
intent authority, final intent/history/PCD, proof validity or ledger settlement.
They cannot be converted into native installed, preparation or proving
capabilities. Static work limits confer no native execution or proving authority.

Legacy `unsigned` and `signed-v2` retain strict whole-program admission. Both must
refuse this genuine mixed-debit source with `one-public-payer` and publish no
output directory. The shared guard precedes the signed policy's own `sole-payer`
check. A malformed signer policy is not an adequate negative control. Prepare a
valid public policy from the existing public example, with the FeePool owner and
beneficiary names:

```sh
python3 - <<'PUBLIC_POLICY'
import json
from pathlib import Path
policy = json.loads(Path("examples/core/static-compile/signer-policy.json").read_text())
policy.update(owner="fee-pool", beneficiary="operator-a")
Path("/tmp/fee-pool-public-policy.json").write_text(json.dumps(policy) + "\n")
PUBLIC_POLICY
mori compile-call examples/core/fee-pool/fee-pool.mori --all-actions --authorization unsigned --static-bindings examples/core/fee-pool/static-bindings.json --work-limits examples/core/fee-pool/work-limits.json --out /tmp/fee-pool-legacy-unsigned-refused
mori compile-call examples/core/fee-pool/fee-pool.mori --all-actions --authorization signed-v2 --signer-policy /tmp/fee-pool-public-policy.json --static-bindings examples/core/fee-pool/static-bindings.json --work-limits examples/core/fee-pool/work-limits.json --out /tmp/fee-pool-signed-v2-refused
```

Mapped mode requires `--all-actions`, public static bindings, work limits, and
exactly one of `--out` or `--inspect-static`. It rejects `--action`, `--deployment`
and `--signer-policy`; it never falls back to a different authorization mode.

The primary source has 54 states, 23 actions, 1 asset, 9 duties, 4,944 expressions and
7,667 lexer tokens. Its largest action has 47 statements; source size is 55,573
bytes. These fit the existing 78-declaration, 8,192-expression/token, 1,024-statement
and 65,536-byte source bounds. Consumer tests measure these properties.

## Contrasting source

`contrasting.mori` is a separately authored four-action financial example named
`IndependentFeeSplit`. It uses distinct source state/action/duty names, the exact
asset `{local, independent-fee-source, ALT, 0}`, four different actors and a
reversed beneficiary declaration order. It funds 99, constrains the same original
55/10/5/15 floors, creates duties 54 and 18, holds the other 27, and claims through
actual transfers/discharges. Its own bindings use a different token identity.

This smaller example exercises the same parser/interpreter/static consumer with
different source financial identities and nondivisible values. It does not claim
a second complete 23-action FeePool instance. Use `contrasting.static-bindings.json`
with that file and explicit `unsigned-mapped-roles`, as shown above. Legacy
unsigned and native/signed paths retain their single-payer restriction.

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
