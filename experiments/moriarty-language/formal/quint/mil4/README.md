# Provisional MIL/4 S0 Quint model

`s0.qnt` models local Source/6 and Core/5 S0 transfer and funded
AccrualFirst repayment. It is a finite design experiment. It does not establish
a semantic freeze, native proof, K correspondence or ledger admission.

The model separates `sign` from `submitTransfer` and `submitRepay`. The stored
signed record fixes domain, asset, program, policy digest, signer, payer,
endpoints, obligation parties, exact amount and fee, nonce, predecessor,
validity, caps, conversion and terminal policy. Submitted intent and amounts
must match it. Repayment requires absent recipient and fee-recipient fields,
zero fee cap, zero net floor and identity conversion. Its creditor is the
party in the authenticated obligation. Transfer endpoints must be distinct;
a zero fee omits its credit line.

Every submission carries a typed `ComparisonRequest`. It names the submitted
successor, requested phase, retained effects and retained duties. Its
`StipulatedTuple` names an available premise, the exact stored intent, the
complete financial pre-state, round, expected successor and requested outcome. The financial
pre-state contains balances, allowance rows, work counters, obligations,
head and consumed replay keys. The request and tuple are supplied assumptions.
Comparing them does not authenticate their contents.

The observations follow `Stage → Intent → Effect → Authority → History →
Failure`. Stage checks required state cells and premise availability and
binding, including equality of the premise outcome and requested outcome.
Unavailable or mismatched premises return `S0_STAGE_PREMISE`;
a malformed required cell takes `S0_STAGE_UNSUPPORTED` first. Effect compares
the complete ordered financial vector against the submitted successor.
History then checks stale predecessor, consumed replay, successor mismatch
and self-successor in that order. The latter two return
`S0_HISTORY_SUCCESSOR`. Head IDs are abstract identities: the model uses no
arithmetic successor rule. A tuple can stipulate `0 → 9`; acceptance checks
identity and never establishes that token's external validity.

Failure accepts only terminal success with no retained effects or duties and
the stored terminal-only policy. Other typed outcomes return
`S0_FAILURE_UNSUPPORTED` after all earlier judgments pass. No committed
failure, pending episode, retained duty, grant, foreign evidence, fee reserve
or alias resolution is implemented.

`transferObservation` and `repayObservation` inspect decisions without
changing state. A rejected attempt returns one abstract diagnostic-work unit
and disables its submit action. It publishes no new state or effects and
consumes no financial authority, replay key or execution work. A successful
submit consumes one abstract work unit and stores the exact submitted vector,
including all money lines, allowance use, replay use, head advance and the
repayment obligation write. This cost is a provisional experiment schedule.

`signatureVerified`, `authenticatedSnapshots`, `nativeQualified` and
`ledgerAtomicReady` remain explicit external premises. The model establishes
none of their truth. Every current head must appear in the stipulated
`authenticatedSnapshots` set. A local commit never authenticates its new
head. Existing postcommit History fixtures explicitly stipulate both `0` and
`1`; a new witness stipulates only `0`, commits head `1`, and observes
`S0_STAGE_PREMISE` on its next submission.
`seedAccount`, `seedBalanceOnly` and `seedObligation` stipulate initial cells.
Quint integers are unbounded; guards separate UInt128 state cells from the
`2^127−1` nominal bound. Source formation and its earlier range rejection
are outside this model.

## Executed finite witnesses

Quint 0.32.0, TypeScript backend, seed `0x5`: all 83 current witnesses passed.
The retained 35 comprise 15 original, 14 ordering regressions, 3 complete
common-state positives and 3 diagnostic witnesses. Original rejection
witnesses now pin their exact codes. The 24 S1B witnesses preserve the
original 18 and add outcome binding, a self-successor also stipulated as
expected, stale with replay, effect mismatch with replay, effect mismatch
with wrong successor and a postcommit unavailable snapshot premise. F1–F3
stipulate the submitted outcome and reach Failure; P2Outcome supplies an
unequal premise outcome and reaches Stage. S1B compares literal complete financial states and
complete decisions, including diagnostic work; its expected effects are
literal vectors. The H/F/P baseline is the common T-10-1 numeric fixture:
round 0, owner allowance 11/0, work 1/0, net floor 0 and balances 100/0/0.
P2PreState changes the premise owner balance to 99; P3 omits the owner cell.
This receipt records Quint execution only, not a cross-artifact comparison.

The later Stage guard repair added 22 negative footprint/work-domain controls
and two positive work-boundary controls. The prior model failed all 22 negative
controls; the repaired model retains the original 59 and passes 83/83. Its
current receipt is `corpus/s1b-stage-guard-command-results.json`, with the
pre-repair red result in `corpus/s1b-stage-guard-pre-repair-command-results.json`.

Every model and witness file typechecked. Exact commands, outputs and code
hashes for the current model are in `corpus/s1b-stage-guard-command-results.json`.
The earlier 59-witness receipt is `corpus/s1b-command-results.json`; earlier receipts are
historical results for their recorded hashes; the previous S1B receipt is
preserved as `corpus/s1b-pre-audit-command-results.json`. No exhaustive simulation,
`quint verify`, native qualification or ledger execution was performed.
W-D0–W-D4 and Sprint 1 remain open.
