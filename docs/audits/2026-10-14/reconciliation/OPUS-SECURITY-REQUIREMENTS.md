## OpenSpec security addendum

```markdown
# Security reconciliation addendum — 2026-10-14

**Status.** This is a specified-only addendum by the Opus security owner. It adds no implementation, evidence, approval or acceptance. It does not amend, renumber or replace any existing clause.

**Baselines.**
- Audit tree: `fc6353bf19b87a76410319c378efe529d52d23d9`. Its merged tree is the same as `11a7d19f`.
- Source observations come from the [Opus security review](../../../docs/audits/2026-10-14/SECURITY-REVIEW.md) (SEC-01–SEC-21) at `cb8280ef`, tree `ce05a5c7…`.

**Preserved unchanged.**
- All original predicates, IDs and positive/hostile controls stay in force: U0–U7, M1–M17, MC01–MC08, SP01–SP12, G01–G24, L1–L8, ZR01–ZR16, MNR01–MNR08, MPLR-001–035, UNI-001–017, MOR-001–012 and AEO-001–004.
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
- **SEC-15** is a qualified capability, not a missing requirement.

## ADDED Requirements

### Requirement: OSR-1014-01 Authenticated evaluation time
**Maps to:** M7, U2, MC05, MPLR-008, SP09.3. **Source:** SEC-01 (designed limitation (DL) plus a confirmed source gap (CSG)).

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
- Evidence: `current-local` safe-static (WS-B B2). Source reading indicates the request is accepted today; this has not been executed.

### Requirement: OSR-1014-02 Declared pipeline per security claim
**Maps to:** M1, M3, M7, M8, UNI-002, UNI-003, MOR-003, MC04 "Named compiler correspondence". **Source:** SEC-02, SEC-15, SEC-16.

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
**Maps to:** MPLR-022, MC05 "Bound exact-plan claims", M3, M7. **Source:** SEC-03 (DL; ledger-side impact is a hypothesis (HYP)).

When TTL, selected spends, or request, parameter or state hashes are claimed to be bound to an authorization:
- the claim SHALL say whether the binding is in-circuit, by host rebuild, or by the target ledger;
- only in-circuit or target-observed binding SHALL count toward ledger-side acceptance.

#### Scenario: Re-envelope
- **WHEN** a holder of the witness and attachment re-proves the same call inside a different envelope
- **THEN** the target SHALL refuse, or the result SHALL record host-only binding with the predicate left open.
- Evidence: `target-observed` (blocked-external).

### Requirement: OSR-1014-04 Key–account authority
**Maps to:** M7, M8, M13, MPLR-008, UNI-012, MC05 "Authority covers liabilities". **Source:** SEC-04.

When a signed policy authenticates with a compiled fixed key:
- results SHALL NOT report account authority, rotation or revocation;
- any account-authority claim SHALL require an authenticated key↔account association with a lifecycle.

#### Scenario: Fixed key as sole authority
- **WHEN** a valid signature under a compiled key or a locally stipulated registry is the only authority evidence
- **THEN** M7 and M13 authority predicates SHALL remain open.
- Evidence: `source-inspected`, then `target-observed`.

### Requirement: OSR-1014-05 Uncovered effects disclosed
**Maps to:** M10, U0, U2, UNI-016, MC08. **Source:** SEC-07.

When a verification command reports a signature or acceptance result, it SHALL identify every effect whose payer or debtor is not covered by the checked authority. It SHALL NOT present those effects as authorized.

#### Scenario: Third-party debit
- **WHEN** Core `verify-intent` checks an intent that debits a non-signer
- **THEN** the output SHALL mark that effect unauthorized, or the command SHALL refuse.
- Evidence: `current-local` safe-static (WS-B B3). Today the source reports `accepted`; changing that output is deferred.

### Requirement: OSR-1014-06 Target-sourced domain values
**Maps to:** M2, M7, L3, MC05. **Source:** SEC-05.

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
- the consumer SHALL recompile under the single-payer rule and refuse with `one-public-payer`;
- no such output SHALL be cited as authorization or custody evidence.

#### Scenario: Mapped bytes fed to admission
- **WHEN** mapped-role IR or manifest bytes reach `relation::admit` or the local-deployment preflight
- **THEN** they SHALL be refused before any byte comparison.
- Evidence: `current-local` (WS-B B6; the test body must be inspected for SDK calls first).

#### Scenario: New consumer
- **WHEN** a new constructor of `Artifact`, `AdmittedSeed`, `DerivedLocalDeployment` or `PublicDerivedCallKey` appears
- **THEN** the consumer inventory (WS-A A1) SHALL be updated before any non-promotion claim.
- Current state: closed for the two consumers inspected; completeness is a hypothesis.

### Requirement: OSR-1014-09 Asset identity in signed caps
**Maps to:** M9, L3, MOR-004, MC05. **Source:** SEC-09 (CSG).

When a signed cap names an asset:
- admission SHALL check the token identifier against (network, issuer, reference);
- it SHALL refuse an asset whose network differs from the deployment network.

#### Scenario: Network mismatch
- **WHEN** signed bindings declare an asset network that differs from the deployment network
- **THEN** `from_source` SHALL refuse.
- Evidence: `current-local` safe-static (WS-B B4). Source reading indicates this is accepted today.

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

#### Scenario: Oversized transaction
- **WHEN** an encoded transaction exceeds the 64 KB ceiling
- **THEN** it SHALL be refused before submission.
- Evidence: `current-local` (WS-B B1), `native-local`, `target-observed`.
- SRS trust rests on the SDK digest table, not on a ceremony audit.

### Requirement: OSR-1014-12 In-circuit Schnorr audit statement
**Maps to:** M8, U1. **Source:** SEC-17 (HYP), SEC-04.

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
- Evidence: `current-local` (WS-B B5). Current state: succeeds (DL).

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
  - the k6 or k10 numerical proofs (unsafe test SRS, proofs 0.8);
  - the R4/R5 fixed-fixture kernel;
- **THEN** the citation SHALL be refused.
- Evidence: `source-inspected`.
```

## Wiki security currentness

```markdown
# Security currentness — 2026-10-14

Owner: Opus (all security judgments). Baseline `fc6353bf` (same merged tree as `11a7d19f`). This page is a reading guide for the historical security families. It is not a security audit, an approval, native qualification or a milestone exit.

- Native authority is zero.
- Root-supplied status: the old campaign is still blocked and there are no pending transactions. This review did not query that status itself.

Findings are in the [Opus security review](../docs/audits/2026-10-14/SECURITY-REVIEW.md). The requirement addendum is [OSR-1014](../openspec/changes/consolidated-language-kernel/security-reconciliation-2026-10-14.md).

## Current security qualification by profile

| Profile / consumer | Security mechanism | Strongest current evidence | Not established |
|---|---|---|---|
| Rust Core `moriarty/1` / `moriarty-core/1` `verify-intent` | Owner-debit checks only; the signer label is not bound to a key | source-inspected | Third-party debits are reported `accepted` (SEC-07) |
| Ledger signed-v2 (`compile-call --authorization signed-v2`) | In-circuit JubJub Schnorr under a compiled key; private `at`; Impact nonce map | source-inspected; PR47/49 source-static capture at the same tree | Any current proof or verification; time binding (SEC-01); account authority (SEC-04); token identity (SEC-09) |
| Backend `mori-backend` signed relation | Public key, message, nonce and `at` | historical-local native, one case at `ee963f61` | Transfer to signed-v2 (SEC-02) |
| Unsigned / `unsigned-mapped-roles` | None | source-inspected non-promotion for two consumers | Authorization or custody of any kind (SEC-08, SEC-20) |
| Beta / Source6 / TS Core5 signed-intent /1 | secp256k1 BIP340 / ECDSA over a closed JSON frame; local stipulated registry | historical-local, reviewed scoped stage `fd9725e0` | Key authority, authenticated head, native or ledger acceptance; still `PreparedUnqualified` |
| Fixed native financial kernel | Manually specialized kernel with trusted genesis | historical-local native (R4/R5) | Generic correspondence, authenticated funding and history, Preview settlement |
| Targets | — | Preview: historical scope only. Fenrir: identity unknown. Local: `local-unqualified` | Any transfer between targets |

## Historic families: original tree, scope and present implication

| Family | Original tree and scope | Review identities (historical, preserved) | Implication today |
|---|---|---|---|
| [midnight-crypto-2026-09-30](midnight-crypto-2026-09-30/README.md) | Candidate `7e7f3861`, 74 files, merged PR12 `db23fe02`. Harness for secp256k1 primitives, Merkle state, a k6 proof under an unsafe test SRS, privileged local ledger | Astra medium; Grok 4.6 high | Primitive-level evidence only. Not M8, not signed-v2. PLAN.md's unchecked items are completed in DELIVERY.md |
| [signed-intent-2026-10-01](signed-intent-2026-10-01/README.md) | Base `2905cb6d`, reviewed candidate `fd9725e0`, 252 files. Closed /1 codec, Beta consumer, LocalSettlementStore, k10 numerical proof | Sol, Sonnet 5.5 seats; Astra and Grok 4.6 final audits | A different protocol from signed-v2 (OSR-1014-15). The registry is a stipulation. The k10 proof excludes identities, signatures and replay. "Pending" labels are superseded by DELIVERY.md |
| [native-financial-kernel-2026-10-01](native-financial-kernel-2026-10-01/README.md) | After PR13 `033e9a90`. V4–V8, R3 refusals, R4/R5 success, freeze `32305c38…` | Astra, Grok 4.6; R4/R5: GPT-6.1 Sol high, Claude Opus 5.5 high, Grok 4.7 xhigh; Opus CPU dissent and the 2–1 vote kept | A fixed trusted fixture. ECDSA owner ≠ Schnorr NIGHT payer. No generic compiler, PCD or Preview claim. "Native goal active" and "currently live" are as of Oct 1 |

## Statements to read as historical

| Location | Statement | Current reading |
|---|---|---|
| `signed-intent-2026-10-01/README.md:3,20-21,23` | "Active", "audits pending", "native goal is armed" | DELIVERY.md records both approvals of `fd9725e0`. No live goal is asserted here |
| `signed-intent-2026-10-01/RESULT.md:3`, `SECURITY-CASES.md:67-68`, `NATIVE-RESULT.md:26` | Audits pending | Superseded by DELIVERY.md. The files themselves stay frozen |
| `midnight-crypto-2026-09-30/DESIGN.md:27`, `signed-intent-2026-10-01/PLAN.md:27-28` | Astra/Grok routing | Historical. Current routing: GPT-6.1 Sol high implementation, Muse Wonder nonsecurity review, Opus 5.5 security. The older Grok and Astra rosters are not restored |
| `native-financial-kernel-2026-10-01/README.md:22`, `V4-ACTUAL-RESULT.md:9` | Preview ledger 8.1.2 incompatibility | An observation as of Oct 1. Preview's current version was not re-observed |
| `AGENTS.md:100` | "Preview is the sole public execution target" | Stale (SECURITY-REVIEW §5 item 12). Owned by the nonsecurity authors |

## Next security work

- **Source-only:** WS-A A1–A4 (consumer inventory, two-stack field map, Schnorr argument, premise ledger).
- **Safe-static, future and root-authorized:** WS-B B1–B7.
- **Blocked-SDK-native:** WS-C C1–C5.
- **Blocked-external:** WS-D.
- **Requirements:** OSR-1014-01…15. Planned outputs go to the audit dossier's `security/` directory, as described in the [audit plan](../docs/audits/2026-10-14/AUDIT-PLAN.md).
```

## Coverage and dispositions

**Read in full**
- `docs/audits/2026-10-14/{SECURITY-REVIEW,MID-ROADMAP-REVIEW,AUDIT-PLAN}.md`
- `plugins/moriarty-dev/skills/develop/SKILL.md`
- runtime `security-handoffs-openspec.json` and `opus-security-requirements-config.json`
- `wiki-llm/README.md`
- `midnight-crypto-2026-09-30/{README,DESIGN,PLAN,DELIVERY,RESULT}.md`
- `signed-intent-2026-10-01/{README,PLAN,PROTOCOL,PREVIEW-ROUTE,NATIVE-TRANSFER-PROPOSAL,SECURITY-CASES,LOCAL-SETTLEMENT,DELIVERY,NATIVE-RESULT,REPAIR-CRYPTO,REPAIR-AUDIT,RESULT}.md`
- `native-financial-kernel-2026-10-01/{README,R4-R5-REVIEWED-SUCCESS}.md`
- consolidated-language-kernel `spec.md`
- kernel-security-and-ai-solvers `spec.md`
- MC04 and MC05 `spec.md`

**Read in part**
- `AGENTS.md:1-135` plus a grep
- `traceability.md:90-209`
- MC03 spec `:1-62`
- MC06 spec `:1-35`
- MC08 spec `:15-81`
- `sp09…md:55-84`
- permissionless-intention spec `:20-49`
- partial-conditional spec: grep of the MPLR-006/008/011/013/016/018 headings
- native-kernel `REVIEWED-KEYGEN-MILESTONE:1-24`, `BETA-DISPLAY-FOLLOWUP`, `V7-ACTUAL-KEYGEN:1-15`, `V8-03-ACTUAL-REFUSAL:1-15`, `R3-04…:1-15`, `V4-ACTUAL-RESULT`, `V5-ACTUAL-PREPARATION`, `V6-TRANSPORT-FAILURE`
- `crates/mori-ledger/src/signed/context.rs`: grep only, which confirmed the `open_predicates` and `ttl_hex`/`not_after` anchors

**Metadata only (path or handoff heading, not opened)**
- native-kernel `LEDGER-PREPARATION-PROGRESS`, `NEXT-PROOF-SOURCE-PREPARATION`, `LEDGER-BUILD-DIAGNOSTIC`, `V4-HANDOFF-SOURCE`
- signed-intent `DEVEX-RESULT`, `REPAIR-PROCESS`, `REPAIR-TYPES`
- 49 of the 57 handoff files: COMPLETION-PROGRAM, REPORT-RECONCILIATION, the sprint-program spec and tasks, consolidated `design.md`, kernel `requirements.md` (MPLR-022–034 were reviewed through its `spec.md` mirror), the language-to-ledger-lifecycle files, MC01/MC02 files, MC03–MC07 README/design/proposal/tasks, MC07 spec, MC08 tasks, partial-conditional tasks, the permissionless implementation plan, `sprints/README`, `SP01` execution, and SP01, SP02, SP04–SP08 and SP10–SP12

**Not inspected**
- `wiki/research/mplr/*` notes
- `deliverables/native-owner-intent-2026-10-11/RESULT.md` (cited only through SECURITY-REVIEW)
- `raw/assignments/*`
- all evidence, provider, archive, PK and SRS artifacts
- the mutable nonsecurity worktree, apart from a Glob confirming that neither reserved output file exists
- `security-handoffs-wiki.json`, which is absent

**Handoff dispositions**
- All 57 files and all handoff locations are preserved byte-for-byte.
- The UNI, MPLR-019–034, MC03–MC06, MC08 and MOR-003/004 locations that were read are applicable and either cross-linked or extended by OSR rows.
- Headings named "Verification commands", "Verification entry points" or "Tasks" govern maintainer delivery. They create no product security clause. Their native items are still classified blocked-SDK-native.
- Every metadata-only location is preserved and recorded as uninspected. Its applicability is unknown, not cleared.

**EARS mapping (SEC → OSR)**

| SEC | OSR |
|---|---|
| 01 | 01 |
| 02, 15, 16 | 02 |
| 03 | 03 |
| 04 | 04, 12 |
| 05 | 06 |
| 06 | 07 |
| 07 | 05 |
| 08, 20 | 08 |
| 09 | 09 |
| 11, 12 | 10 |
| 13, 14 | 11 |
| 17 | 12 |
| 18 | 13 |
| 19 | 14 |
| 21 | Cross-linked to UNI-006 / M4 |
| 10 | Deferred to a U0 ruling |

The wiki families add OSR-15. M7, M8 and M9 previously had only table rows in `traceability.md`. OSR-01, 04, 06, 09, 12 and 15 now give them falsifiable scenarios without changing those rows.

**Frozen originals.** Every RESULT, REVIEW, DELIVERY, audit, receipt and evidence file stays unchanged. Insertions go only into the three active family README files. Historical reviewer identities are kept as written, and no old roster is restored. Old native contracts, budgets and counters cannot change and are not touched.

**Handoffs to nonsecurity authors (routing facts, no security judgment)**
- `SKILL.md:19-23` still names Sol and Grok 4.7 authors, and neither `SKILL.md` nor `AGENTS.md:3-13` states that all security work is owned by Opus.
- `AGENTS.md:100` is stale.
- `wiki-llm/README.md` should link the new currentness page.

## Maintained entrypoint insertions

**1. `wiki-llm/signed-intent-2026-10-01/README.md`.** Insert after line 1 (`# Signed intent to settlement iteration`), before "Active autonomous iteration":

> **Security currentness (2026-10-14, Opus).** This page describes the stage as of October 1. [DELIVERY.md](DELIVERY.md) records both required audits approving the scoped local candidate `fd9725e0`, so the "pending" and "armed" wording below is historical. The /1 protocol uses secp256k1 BIP340/ECDSA with a locally stipulated registry. It is not the ledger signed-v2 JubJub path, and it is not M7 or M8 evidence. See [security currentness](../SECURITY-CURRENTNESS-2026-10-14.md).

**2. `wiki-llm/midnight-crypto-2026-09-30/README.md`.** Append after line 3:

> **Security currentness (2026-10-14, Opus).** These are reviewed primitive-level local experiments (PR12, candidate `7e7f3861`). The k6 proof used an unsafe test SRS. Neither those results nor the secp256k1 signature tests qualify the current signed-v2 path, M8, or any target. See [security currentness](../SECURITY-CURRENTNESS-2026-10-14.md).

**3. `wiki-llm/native-financial-kernel-2026-10-01/README.md`.** Insert after line 7 (the paragraph under `## Latest reviewed result — R4 proof and R5 verification`), before "The next sections retain…":

> **Security currentness (2026-10-14, Opus).** R4/R5 is historical native evidence for one manually specialized kernel with a trusted development genesis. It does not transfer to the current compiler or signed-v2 path, to Preview, or to Fenrir. "Currently" and "active" statements on this page are as of October 1. See [security currentness](../SECURITY-CURRENTNESS-2026-10-14.md).

## Limits

- This was one finite pass over source and documents using only Read, Grep and Glob. It ran no commands or tests, and no SDK, VM, keygen, prove, verify, query or ledger action. It read no private, PK, SRS, cache or provider material and used no GPT security draft.
- It is not new security verification, not a full security or crypto audit, and not a milestone acceptance.
- SEC line references and "currently accepted" statements are source readings at `cb8280ef`. None of them has been executed.
- Hypotheses (SEC-03, SEC-08 completeness, SEC-17, SEC-18 double release) stay unknown.
- Metadata-only and uninspected files may hold security-relevant clauses that this pass did not find.
- The prior Opus review is authoritative only within its own scope.
- Native authority remains zero, and Fenrir target identity remains unknown.
