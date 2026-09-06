# R2 executable agreement slice

Status: scoped implementation design under the user's “begin” for the R2 sprint.
The unified semantic design and PCD report revision remain controlling. This
records routine first-slice decisions, not full ACTUS or proof-system adoption.

The [intents report amendment](../../research/2026-09-06-intents-report-integration.md)
clarifies this implementation as **exact-plan authorization**. The signature
binds the concrete action, predecessor, result and policy. General outcome
IntentIR, route choice after signing, aggregate capability residualization,
expiry/nonces and durable consumption are R2b obligations, not implemented here.

## Developer language and shared execution

Use a bounded JSON agreement document as the first executable authoring form:
`{language:"moriarty-r2/1", package, terms}`. Packages are
`Actus.LAM.FirstPeriod` and `Exchange.ConstantProduct`; this is a typed package
subset, not final textual syntax. Reject unknown keys, packages and malformed
values. Elaboration specializes terms into a finite generic Core program.
The evaluator dispatches only Core instructions and never branches on package
names. It has no host callbacks, recursion, dynamic code or unbounded loops.

Core supports bounded string IDs, UInt128, equality/order, checked add/sub/mul,
floor division, local bindings, guards, state assignments and typed effect
emission. Programs have at most 64 instructions per entry point, expression
depth 16, 256 expression nodes, 64 fields/locals and 16 effects. All intermediate
integers are checked UInt128; integers are canonical decimal strings on the
wire. Core results are evaluated/rejected, never certified ledger acceptance.
State has instance, revision, remaining step allowance and typed values. Every
successful action advances revision and decreases remaining allowance; rejection
leaves the caller's inputs unchanged. Mutation is committed only after all
instructions and final checks succeed. A closed state cannot resume ordinary
work. The pool reserves its last allowance for closure; loan first-period
accrual and full settlement have two allowances. This is a finite episode/epoch,
not a complete loan lifecycle or perpetual AMM.

## Financial scope and numeric profile

Loan amounts are micro-USD (six decimal places); accrual uses checked exact
integer products then one explicit floor quantization. Default first period:
principal 5,000 USD; principal due 500 USD; rate 8/100; 31/365 accrual.
Interest is 33.972602 USD, with the discarded fraction displayed. This does not
match the entire reference decimal 33.972602739726 and is not ACTUS conformance.
PR/IP is one ordered first-period action that accrues on the old principal,
reduces notional and creates separate principal/interest dues. Settlement needs
matching transfers in the explicitly configured demo:USD6 asset and updates
outstanding/paid accounting. The display never calls an unpaid due settled.

The pool uses integer asset base units, fee multiplier 997/1000, floor output,
full input added to reserve A and exact output removed from reserve B. Default
reserves 1,000,000/2,000,000, input 10,000 => output 19,743. A minimum of 19,744
rejects. Asset and actor identities are checked. Close refunds the two remaining
reserves to the declared provider. Synthetic account balances are explicit;
there is no real ledger or custody claim.

## Signed claims and independent checks

Canonical encoding forbids JSON numbers, duplicate/noncanonical encodings,
unknown critical fields and oversized data. Restrict depth, collections and
bytes. Use SHA-256 commitments and Ed25519 local demonstration signing through
WebCrypto; this is genuine signature checking with an explicitly trusted local
public key, not wallet authorization or PCD. Ephemeral keys never leave the
process or enter exports. Binding order is TxCore/ClaimSpec => manifest root =>
intent digest => signature => BoundClaim/evidence. ClaimSpec excludes its own
ID and enclosing roots; no signature or proof is in its own preimage.

The signed intent independently specifies allowed recipients/assets/debits,
minimum credits, maximum fees, due operations and allowed state writes.
Transfer/fee authority caps gross cumulative movement per asset/from/to edge;
refunds do not replenish a cap. Minimum credits use net asset changes after
outgoing movements and fees. Cross-recipient aggregate budgets are not yet a
separate policy field. The policy is an exact-plan profile, not full IntentIR.
The effect checker does not call the evaluator or accept a supplied list of
writes as complete: it computes state differences and checks every effect,
account balance delta and due delta against the signed policy. Unknown effects
(including approvals), hidden state writes, wrong assets/recipients, excess
fees and incomplete projections reject. Invalid mandatory claims and substituted
program/domain/state/effects reject. All four required cryptographic/contract
claims remain unavailable until actual checkers exist; valid signatures or
local effect checks cannot mint a real VerificationCertificate.

## Browser and proof feasibility

Add a separate executable workspace reachable from the existing mock. It has
an editable JSON agreement, action editor, elaborated instruction display,
before/after state and effects, independent policy checks, local demo-key signing
and fail-closed required-claim verification. State advancement is explicitly
local simulation; it remains independent of real proof acceptance. Keep the
old imported reference views and simulated workflow available and labeled.

Specify the R3 experiment against the inspected Midnight IVC interfaces using
one of these financial transitions. Record the native-vs-ledger distinction,
inputs, commands/resource envelope and tamper controls. This sprint does not
run an expensive proof campaign or assert native/ledger compatibility.
