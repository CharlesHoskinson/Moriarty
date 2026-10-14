# Security reconciliation addendum — 2026-10-14

**Status.** This is a specified-only addendum by the Opus security owner. It adds no implementation, evidence, approval or acceptance. It does not amend, renumber or replace any existing clause. No command, test, SDK call, native step or target action was executed for it, and no U, M, MC, SP or G exit closes because of it. The SEC observations it cites are source readings at `cb8280ef`, not executed results.

**Baselines.**
- Audit tree: `fc6353bf19b87a76410319c378efe529d52d23d9`. Its merged tree is the same as `11a7d19f`.
- Source observations come from the [Opus security review](../../../docs/audits/2026-10-14/SECURITY-REVIEW.md) (SEC-01–SEC-21) at `cb8280ef`, tree `ce05a5c7…`.

**Coverage.** The Opus currentness pass inspected all 64 OpenSpec handoff entries, and 34 authored wiki paths plus 8 exact-claim locations. That was not an exhaustive audit of all prose, fenced contracts, private material or raw captures.

**Preserved unchanged.**
- All original predicates, IDs and positive/hostile controls stay in force: U0–U7, M1–M17, MC01–MC08, SP01–SP12, G01–G24, L1–L8, ZR01–ZR16, MNR01–MNR08, MPLR-001–035, UNI-001–017, MOR-001–012 and AEO-001–004. The later nonsecurity UNI-018–024 clauses are outside this addendum, which neither modifies nor restates them; UNI-022 target separation is consistent with OSR-1014-06.
- The ACTUS denominator (277 fixtures) and the DeFi denominator (72 rows) also stay unchanged.

**Targets.**
- The user's M table already controls, so it needs no adoption vote.
- Preview and Fenrir are separate evidence targets.
- Fenrir network ID, Anchor, committee and versioning are unknown and are not invented here.

## Conventions

- **Axes.** Implementation status, evidence class and acceptance stay separate axes. The evidence classes are those in the [audit plan](../../../docs/audits/2026-10-14/AUDIT-PLAN.md): `source-inspected`, `historical-local`, `current-local`, `native-local`, `target-observed`. Evidence in one class never advances another class or another axis.
- **Required evidence.** Each requirement below names the evidence class that would close it, and its execution class: `safe-static`, `blocked-SDK-native` or `blocked-external`, as defined in SECURITY-REVIEW §4.
- **Current state.** Every OSR row is `specified-only`, with acceptance `open`.
- **Execution note (non-normative).** SDK `relation::check`, `k()`, VM execution, keygen, prove and verify count as native execution. This includes signed `verify-intent --profile`, `simulate --trace --target`, the replay generator, `prepare-call` and `prepare-signed-call`. Native authority remains zero.

## Existing clauses that already cover a security topic (cross-linked, not restated)

| Topic | Controlling existing clause |
|---|---|
| Durable single consumption, competing branches, restart | MC04 "Durable one-time consumption"; SP09.3 |
| Prover-selected or unrelated verifier | MC04 "Permissionless correspondence validation"; MOR-003 |
| Revoked verifier or inactive specification; oversized verification input | MC05 and MC08 scenarios of those names |
| Guaranteed-phase fees, partiality, DUST (SEC-21) | UNI-006; M4 |
| Consent to new obligations | MPLR-019 |
| Stage authority, revocation, expiry | MPLR-008; SP09.3 |
| Misleading proved/settled status | MC08 "Misleading proof status"; UNI-016 |
| Solver same-name asset substitution | MOR-004 |
| Proof for X used to sign Y | UNI-011; MPLR-030 |
| Separate-process verification of retained bytes (SEC-15) | MC03 "Real recursion and independent verification" |

Two observations get no new requirement:
- **SEC-10** (fee reduces beneficiary net) waits for a U0 beneficiary ruling.
- **SEC-15** is a qualified capability, not a missing requirement. OSR-1014-02 cites it only for the qualification that the October 11 verification evidence is historical and backend-only.

## ADDED Requirements

### Requirement: OSR-1014-01 Authenticated evaluation time
**Maps to:** M7, U0 (time), U2, MC05, MPLR-008, SP09.3. **Source:** SEC-01 (designed limitation (DL) plus a confirmed source gap (CSG)).

When an acceptance path claims to enforce a signed validity window:
- the enforcing statement SHALL constrain the evaluation time to a value supplied or attested by the claimed target;
- admission SHALL relate the signed TTL to the window bounds;
- a time chosen by the prover SHALL be reported as owner-TTL-only enforcement.

#### Scenario: Prover-selected time
- **WHEN** a proof uses a private `at` inside `[not_before, not_after]` while the target's time lies outside that window
- **THEN** the claimed path SHALL reject, or the claim record SHALL state that M7 expiry enforcement is not provided.
- Evidence: `native-local` (WS-C C2, blocked-SDK-native), then `target-observed`.

#### Scenario: TTL beyond window
- **WHEN** signed-call admission receives `ttl > not_after`
- **THEN** it SHALL refuse with a named code, or the specification SHALL state which bound controls.
- Evidence: `current-local` safe-static (WS-B B2). A source reading at `cb8280ef` indicates the request is accepted; this has not been executed.

### Requirement: OSR-1014-02 Declared pipeline per security claim
**Maps to:** M1, M2, M3, M7, M8, U2, U7, the G gates, UNI-002, UNI-003, MOR-003, MC04 "Named compiler correspondence". **Source:** SEC-02, SEC-16; SEC-15 only for the qualification that the October 11 evidence is historical and backend-only.

When a security property of a signed or authorized call is claimed:
- the claim SHALL name its stack and tree;
- it SHALL map every statement field (key, message, nonce, time, domain, caps) as public, private or constant;
- evidence from a different stack, tree or field map SHALL transfer only through a stated differential predicate.

#### Scenario: Cross-stack citation
- **WHEN** the October 11 backend proof at `ee963f61` is cited for the ledger signed-v2 path at `cb8280ef`
- **THEN** the citation SHALL be refused as acceptance evidence for that path.
- Evidence: `source-inspected` field map (WS-A A2), plus a `native-local` differential.

#### Scenario: Build hash as statement identity
- **WHEN** a `source_closure` or compiler hash is offered as proof-statement identity
- **THEN** the record SHALL classify it as byte identity only.

### Requirement: OSR-1014-03 Envelope binding location
**Maps to:** MPLR-022, MC05 "Bound exact-plan claims", M3, M7, U2. **Source:** SEC-03 (DL; ledger-side impact is a hypothesis (HYP)).

When TTL, selected spends, or request, parameter or state hashes are claimed to be bound to an authorization:
- the claim SHALL say whether the binding is in-circuit, by host rebuild, or by the target ledger;
- only in-circuit or target-observed binding SHALL count toward ledger-side acceptance.

#### Scenario: Re-envelope
- **WHEN** a holder of the witness and attachment re-proves the same call inside a different envelope
- **THEN** the target SHALL refuse, or the result SHALL record host-only binding with the predicate left open.
- Evidence: `target-observed` (blocked-external).

### Requirement: OSR-1014-04 Key–account authority
**Maps to:** M7, M8, M13, M17, MPLR-008, UNI-012, MC05 "Authority covers liabilities". **Source:** SEC-04.

When a signed policy authenticates with a compiled fixed key:
- results SHALL NOT report account authority, rotation or revocation;
- any account-authority claim SHALL require an authenticated key↔account association with a lifecycle.

#### Scenario: Fixed key as sole authority
- **WHEN** a valid signature under a compiled key or a locally stipulated registry is the only authority evidence
- **THEN** M7 and M13 authority predicates SHALL remain open.
- Evidence: `source-inspected`, then `target-observed`.

### Requirement: OSR-1014-05 Uncovered effects disclosed
**Maps to:** M10, M13, U0, U2, UNI-016, MC08. **Source:** SEC-07.

When a verification command reports a signature or acceptance result, it SHALL identify every effect whose payer or debtor is not covered by the checked authority. It SHALL NOT present those effects as authorized.

#### Scenario: Third-party debit
- **WHEN** Core `verify-intent` checks an intent that debits a non-signer
- **THEN** the output SHALL mark that effect unauthorized, or the command SHALL refuse.
- Evidence: `current-local` safe-static (WS-B B3). A source reading at `cb8280ef` indicates the output is `accepted`; this has not been executed, and changing that output is deferred.

### Requirement: OSR-1014-06 Target-sourced domain values
**Maps to:** M2, M7, M9, U0, L3, MC05. **Source:** SEC-05.

When a signed domain, nonce domain or context hash is offered as M7 evidence:
- network ID and Anchor SHALL come from the identified target;
- `local-unqualified` and caller-supplied network strings SHALL count only as local separation;
- Preview and Fenrir SHALL each require their own evidence.

#### Scenario: Placeholder Anchor
- **WHEN** evidence binds Anchor `local-unqualified`
- **THEN** M7 SHALL remain open.

#### Scenario: Target transfer
- **WHEN** a Preview receipt is offered for Fenrir, or the reverse
- **THEN** the claim SHALL be refused.
- Evidence: `target-observed` (blocked-external). Fenrir identity is unknown.

### Requirement: OSR-1014-07 Nonce capacity
**Maps to:** M7, M13, U2, MPLR-021; MC04 still controls chain uniqueness. **Source:** SEC-06.

When signed consumption reaches the declared nonce-map capacity:
- admission SHALL refuse further consumption with a named capacity code and no state change;
- the capacity SHALL be published as a declared deployment resource bound.

#### Scenario: Full map
- **WHEN** the 12,153-entry map (including the sentinel) is full
- **THEN** the next signed call SHALL be refused.
- Evidence: `current-local` for the host check, `native-local` for the Impact path.

### Requirement: OSR-1014-08 Unsigned and mapped-role non-promotion
**Maps to:** M11, U2 custody, the UNI-002 hostile witness. **Source:** SEC-08, SEC-20.

When an unsigned or `unsigned-mapped-roles` artifact reaches any consumer that admits, installs, derives keys or reports authorization:
- the consumer SHALL recompile under the single-payer rule and SHALL refuse with `one-public-payer` any artifact with more than one public payer;
- no such output SHALL be cited as authorization or custody evidence.

This requirement does not refuse a legal single-payer unsigned artifact; such an artifact still carries no authority (SEC-20).

#### Scenario: Mapped bytes fed to admission
- **WHEN** mapped-role IR or manifest bytes reach `relation::admit` or the local-deployment preflight
- **THEN** they SHALL be refused before any byte comparison.
- Evidence: `current-local` (WS-B B6; the test body must be inspected for SDK calls first).

#### Scenario: New consumer
- **WHEN** a new constructor of `Artifact`, `AdmittedSeed`, `DerivedLocalDeployment` or `PublicDerivedCallKey` appears
- **THEN** the consumer inventory (WS-A A1) SHALL be updated before any non-promotion claim.
- Current state: closed by source reading at `cb8280ef` for the two consumers inspected; completeness is a hypothesis.

### Requirement: OSR-1014-09 Asset identity in signed caps
**Maps to:** M9, L3, MOR-004, MC05. **Source:** SEC-09 (CSG).

When a signed cap names an asset:
- admission SHALL check the token identifier against (network, issuer, reference);
- it SHALL refuse an asset whose network differs from the deployment network.

#### Scenario: Network mismatch
- **WHEN** signed bindings declare an asset network that differs from the deployment network
- **THEN** `from_source` SHALL refuse.
- Evidence: `current-local` safe-static (WS-B B4). A source reading at `cb8280ef` indicates this is accepted; this has not been executed.

### Requirement: OSR-1014-10 Liability-cap coupling and discharge
**Maps to:** M13, L5, U3, MPLR-019, UNI-008. **Source:** SEC-11, SEC-12.

- When the sole-debtor rule or the liability-cap computation changes, a guard predicate SHALL show that the circuit cap and the Core cap agree.
- When a program defines `discharge`, its conformance SHALL include a hostile unpaid-discharge control.

#### Scenario: Relaxed sole-debtor
- **WHEN** a duty's debtor may differ from the owner
- **THEN** the guard predicate SHALL fail until both computations are reconciled.

#### Scenario: Erased debt
- **WHEN** a duty is discharged without payment or creditor consent
- **THEN** the program SHALL reject the discharge, or its specification SHALL state the permitted term.
- Evidence: `current-local`, then `native-local`.

### Requirement: OSR-1014-11 Parameter, degree and transaction ceilings
**Maps to:** M5, U1, MPLR-021, MC05 "Oversized verification input". **Source:** SEC-13, SEC-14.

- When keys are generated through any public API, the API SHALL enforce the pinned parameter digests and the declared degree ceiling, or label its output unpinned.
- When M5 is claimed, the encoded-transaction ceiling and the fraud-circuit k SHALL be measured on the claimed target.

#### Scenario: Unpinned parameters
- **WHEN** caller-supplied parameters reach `generate_call_keys`
- **THEN** it SHALL refuse them, or label the output unpinned.
- Evidence: no current control exercises this path. WS-B B1 (`parameters::tests`) covers only the pinned loader's degree, byte and digest refusals. A refusal that happens before any SDK call could be `current-local`; key generation itself is `native-local` (blocked-SDK-native).
- SRS trust rests on the SDK digest table, not on a ceremony audit.

#### Scenario: Oversized transaction
- **WHEN** an encoded transaction exceeds the encoded-transaction ceiling measured for the claimed target
- **THEN** it SHALL be refused before submission.
- Evidence: no current-local control exists. SEC-14 found no encoded-transaction ceiling; the existing 64 KiB bounds cover JSON, envelope and source only. The ceiling needs `native-local` measurement, then `target-observed` (blocked-external).

### Requirement: OSR-1014-12 In-circuit Schnorr audit statement
**Maps to:** M8, U1. **Source:** SEC-17 (HYP); SEC-04 only for the compiled-key premise (its authority predicates belong to OSR-1014-04).

When M8 is claimed, the audit SHALL state and cite:
- equivalence of the host and circuit challenge reductions;
- response canonicality;
- the key and R subgroup/on-curve premises, against the cited ZKIR semantics.

#### Scenario: Hostile witnesses
- **WHEN** the ledger relation receives s+1, a wrong key, an R with a torsion component, or a challenge at the reduction boundary
- **THEN** each SHALL be rejected.
- Evidence: `source-inspected` argument (WS-A A3), plus `native-local` controls (WS-C C1, C3). The external M8 audit is deferred.

### Requirement: OSR-1014-13 Sponsor release recording
**Maps to:** M4, M13, U3, UNI-008, MPLR-032. **Source:** SEC-18.

When a hold is released as unused, the state machine SHALL require recorded non-submission or an authenticated observation.

#### Scenario: Release after submission
- **WHEN** `release_unused` follows a submission without `mark_unknown`
- **THEN** it SHALL refuse.
- Evidence: `current-local` (WS-B B5). A source reading at `cb8280ef` indicates the release succeeds (DL); this has not been executed. Double release after submission is a hypothesis (SEC-18 HYP), not an observed result.

### Requirement: OSR-1014-14 Advertised commands refuse
**Maps to:** M10, U7, MC08. **Source:** SEC-19.

When an advertised command has no implementation, it SHALL exit with an explicit unsupported diagnostic and SHALL NOT panic.

#### Scenario: Server stub
- **WHEN** `mori lsp` or `mori mcp` is invoked
- **THEN** it SHALL return a non-zero unsupported status without a `todo!` panic.
- Evidence: `current-local`.

### Requirement: OSR-1014-15 Signature-protocol evidence scope
**Maps to:** M8, UNI-016, MC08. **Source:** the wiki families reconciled in [security currentness](../../../wiki-llm/SECURITY-CURRENTNESS-2026-10-14.md).

When signature or proof evidence is cited, it SHALL name its scheme, curve, hash/framing, proof stack and consumer.

#### Scenario: Different protocol
- **WHEN** any of the following is offered as JubJub in-circuit M8 or signed-v2 evidence:
  - the secp256k1 BIP340/ECDSA results of signed-intent /1 or the September 30 harness;
  - the k6 or k10 numerical proofs (ephemeral unsafe test SRS; the k10 proof's proofs 0.8/curves 0.3 stack is not established as compatible with the ledger proofs 0.7.3 verifier);
  - the R4/R5 fixed-fixture kernel;
- **THEN** the citation SHALL be refused.
- Evidence: `source-inspected`.
