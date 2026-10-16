# Moriarty V1: security red-team of the V1 definition and sprint closure (Opus, planning only)

**Status.** This is a planning proposal from the Opus security owner. It is not product acceptance, a source or runtime audit approval, native authorization or a resource reset. I used no tools, shell, writes or web.

## Inspection scope and limits

**Read in full, as supplied text:**
- `ROADMAP.md` (0afd1af3…)
- `docs/MORIARTY-PRODUCT-CONTRACT.md` (d5674c65…)
- `openspec/.../requirements.md` (aa7b2243…)
- `spec.md` UNI-001–024 (66f85468…)
- `traceability.md` (d549f31a…)
- The heading index (a361071a…). It has headings only, no clause bodies.
- The Root phase-1 receipt.
- `SECURITY-REVIEW.md` (64cb033e…)
- `SECURITY-ROUTING.md`
- `MID-ROADMAP-REVIEW.md`
- `AUDIT-PLAN.md`

**Not supplied, so not inspected:**
- Any product source at `9bcbca52`.
- The ZR01–ZR16 and MNR01–MNR08 bodies (`MORIARTY-BACKEND-REQUIREMENTS.md`).
- The OSR-1014 addendum.
- SP10.1 operator definitions.
- MPLR notes.
- The frozen ACTUS/DeFi coverage manifest.
- PR51–PR101 diffs.
- `compile_reported` source.
- The Sherlock machine-circuit sources.
- The stagenet assignment file.

**Path:line references.** All path:line citations to Rust source come from the 2026-10-14 security review, frozen at `cb8280ef`. They must be re-anchored at `9bcbca52` before they are cited as current. Any SEC finding below may have been fixed or moved by PR51–101; I have not verified this.

**User-supplied facts taken as given:**
- `compile_reported` calls the backend compiler and then appends a Sherlock projection report.
- The production and `cfg(test)` draft frontends differ.
- The phase-1 receipt contents are as supplied.
- The K33 checks are UNRUN.
- A102 is not compiled.
- G4-BYTE is unadopted.
- Fenrir facts are unknown.
- Midnight stagenet is required.
- Lean/Clean is developer QA only.

---

## 1. Current evidence versus incomplete behavior

### 1.1 Evidence ledger (scope-qualified)

| Area | Strongest current evidence (class) | Incomplete behavior blocking V1 | Source |
|---|---|---|---|
| Core compiler | Bounded `moriarty/1` → ledger-9 ZKIR lowering for state, assets, transfers and liabilities. Mapped-role provenance metadata; one byte-identical specimen; ten focused tests (source-static/local) | Rule-level completeness and soundness over the admitted grammar; native verification; target effects. "ledger-9 ZKIR" is not yet pinned as "ZKIRv3 major/minor" per the product contract | ROADMAP compiler provenance; Product contract "Execution target" |
| `compile_reported` | Backend output plus an appended Sherlock projection report (user fact) | The report covers a scalar projection only. It is not correspondence evidence for the financial relation it is appended to | User brief |
| Signed native path | One local laboratory k13 proof on the backend stack at `ee963f61`; 6+6 hostile rejections (historical, native-local) | The ledger signed-v2 stack differs in which fields are public (SEC-02). Missing: account↔key binding (SEC-04), ledger-enforced time (SEC-01), token derivation (SEC-09), multi-payer authority, on-chain nonce uniqueness (SEC-06) | ROADMAP 2026-10-11; SECURITY-REVIEW SEC-01/02/04/06/09/15 |
| Core continuations `/2`, `/4` | 58 to 88 focused tests (mixed fresh and carried-forward); 11 Quint normals | Unmerged and unreviewed. `mori-ledger` native call does not support `/2`. No Rust↔Quint correspondence | ROADMAP continuation observations |
| Sherlock | K-A Quint (98+126 normals); typed host and image interpreters; `cfg(test)` draft frontend (11 tests); A102 APIs registered with 16 tests UNRUN | Not done: aux/2 phase 1 trace adjudication; phase 2; K33; A102 compile; G4; arena correspondence; image-digest binding; step circuit; native work; stagenet | Phase-1 receipt; ROADMAP Sherlock status |
| Phase-1 arithmetic/fuel | 18 named passing outputs, exit 0, rehash matched | "zero failed and ignored" is inferred, not printed. Trace adjudication is pending a whitespace regex fix. Phase 2 never started | Receipt |
| Economics | FeePool: 23 actions, 162 finite cases (local/static). Core fee/net semantics match the circuit (SEC-10) | Missing: fee recipient constrained by caps (§3 "Fees"); per-debtor liability sums (SEC-11); discharge coupling (SEC-12); phase and failure effects (SEC-21); DUST (M4) | SECURITY-REVIEW §3; ROADMAP table |
| Lifecycle (U3) | Reference escrow, duties and sponsor holds (local) | The 11 A / 20 B discriminator needs a counterparty debit. It cannot pass single-payer signed admission | SECURITY-REVIEW §5 item 5 |
| History (U4) | User-demonstrated ledger-9 recursion route without VerifyProof (mode unclassified) | Missing: MC03 native recursion; MC06 private handoff; all five composition operators; ZR/MNR | ROADMAP recursion; UNI-009 |
| Targets | Historical Preview loan and swap results | Stagenet E2E is unexecuted. Fenrir network ID, Anchor, committee and versioning are unknown. No 64 KB transaction ceiling (SEC-14) | ROADMAP stagenet requirement; UNI-022 |
| Developer | Rust local CLI; complete aggregate test report; Node beta (Source/6 → TS Core/5) | Rust LSP/MCP `todo!` panics (SEC-19). Two executables named `mori`. No clean install, no own-wallet submission, no two-builder reproduction | MID-ROADMAP-REVIEW table; SEC-16/19 |
| Conformance | Finite libraries | All of these remain open, with no column filled beyond finite samples: 277 ACTUS fixtures, 18 executable types, 32 dispositions, DS01–07, 72 DeFi rows, DA01–24, supplements, 3 held-outs, 8 intent cases, 8 regression classes, G01–G24 | UNI-023 |

### 1.2 Omissions that could falsely declare V1 complete

Each item below is a specific path to a false "V1 done". Each is closed by a gate in §3.3.

1. **Sherlock projection mistaken for compiler correspondence.** A green scalar Sherlock report appended by `compile_reported` could be read as covering the state, asset, transfer and liability constraints in the same artifact.
2. **`cfg(test)` frontend evidence counted for production.** The draft frontend's factorial and tree results do not exercise the production frontend. Without byte-identical emitted ZKIR they say nothing about it.
3. **Ledger induction relabeled as native recursion.** A route "without VerifyProof" is very likely ledger-checked per step. UNI-009 and the traceability conflict table forbid closing MC03 or MC06 that way.
4. **Single-payer V1.** Signed-v2 enforces one public payer and sole debtor (`signed/mod.rs:214-294` @cb8280ef). V1 cannot honestly claim swaps, escrow, the U3 discriminator or M14/M15 without multi-party signed authority.
5. **Hostile tests that never reached the verifier.** These include:
   - "rejections" caused by the honest prover refusing to build the witness;
   - rejections caused by envelope malformation (ROADMAP "Immediate delivery brief");
   - rejections with no constraint-ablation control showing that the test exercises the constraint it targets.
6. **Stack transfer.** Backend-stack native evidence cited for the ledger signed-v2 pipeline (SEC-02).
7. **Host-only or premise fields counted as enforced.** Examples: TTL inside the opaque payload digest (SEC-03), private `at` (SEC-01), unconstrained fee recipient, caller token mapping (SEC-09), a supplied predecessor.
8. **Local atomic failure substituted for ledger phase failure.** UNI-006 and MOR-011 require the actual retained guaranteed-phase effects.
9. **Target substitution.** Preview receipts standing in for stagenet, or stagenet for Fenrir M2/M3, or the reverse.
10. **Shared oracle.** Expected values derived from the evaluator, a static reopen or a wrapper (UNI-020).
11. **Model QA as implementation evidence.** Quint seed-1 single samples, K checks, or Lean/Clean results counted as native or target correspondence.
12. **Denominator shrinkage.** The 69-ID index used as the atomic denominator, or library batching that hides missing ACTUS/DeFi rows.
13. **Verifier non-independence.** The only verifier is Moriarty's own code over a caller-supplied predecessor (SEC-15).
14. **Release contents.**
    - The test-only witness-override harness ships in the release.
    - The unauthenticated `unsigned-mapped-roles` mode is reachable without explicit labeling.
    - Advertised commands panic.
15. **Unenforced resource ceilings.** The k ≤ 19 and 64 KB ceilings are enforced on some paths but not all. `generate_call_keys` accepts a caller ceiling (SEC-13), and no transaction byte cap was found (SEC-14).
16. **Interpreter program substitution.** A Sherlock proof whose statement does not bind the program-image digest. ROADMAP lists "Image/digest binding … remain open".
17. **U5 silently dropped.** U7 depends on U2–U6 (ROADMAP U7 row). Federation is optional for direct use, but its qualification is still in the V1 dependency set.

---

## 2. Prioritized capability sprints

### Conventions

- Each sprint delivers a public capability in existing crates or CLI, not process artifacts.
- Hostile-class IDs (H01–H28) and the hostile-test triple are defined in §3.
- Evidence classes:
  - **E-model:** Quint, K or Lean/Clean (developer QA).
  - **E-ref:** Rust reference evaluator.
  - **E-static:** compile, export or reopen.
  - **E-native:** local prove and verify.
  - **E-target:** stagenet, Fenrir or Preview, named separately.
- Native and target evidence always needs separate admission under the existing counters. Nothing here grants it.

### S01: Canonical statement, target tuple and compiler-emitted enforcement map (U0; MC01, M2, M6, UNI-002/018/019/022)

**Prerequisites.** None. This sprint is mostly source-only plus ordinary compiler code.

**Owned interfaces** (paths @cb8280ef, to be re-anchored):
- `mori-ledger/src/signed/{mod,context,constraints,preparation}.rs`
- `mori-backend/src/signed_relation.rs`
- `relation/mod.rs` and `relation/compiler.rs`
- `layout.rs` and `profile.rs` (target descriptor)
- The `compile_reported` output schema

**Capability.** `mori compile` emits a statement manifest from the same lowering pass that emits the constraints. The manifest maps every field to `{public | constant | constrained-private | host-only | premise}` and to its constraint IDs. It also emits a target-tuple record.

**EARS clauses:**
- **S01.R1.** WHEN the compiler emits a relation, it SHALL emit a statement manifest classifying every program, intent, state, effect, history, domain and cost field of the canonical statement, derived from the emitted constraint set rather than from documentation.
- **S01.R2.** IF a field constrained by any signed-intent clause is classified host-only or premise, THEN the compiler SHALL mark each dependent claim `not-enforced-in-relation` AND SHALL NOT emit a certified qualification for that claim.
- **S01.R3.** WHEN `compile_reported` appends a Sherlock projection report, the report SHALL list the compiled constraints it covers AND SHALL label every state, asset, transfer and liability constraint outside the scalar projection `not-covered`.
- **S01.R4.** WHERE network ID, Anchor, ledger/verifier version, ZKIR major/minor, instruction surface or SRS digest is not established for a target, the tuple record SHALL carry `unknown` for that field AND SHALL block qualification for that target only.
- **S01.R5.** WHEN a claim cites native evidence, it SHALL name exactly one stack (backend or ledger signed-v2). Two-stack claims SHALL require a field-level manifest diff over the claimed domain.

**Independent acceptance:**
- A non-author hand-derives the P-A manifest from source and constraints, and it matches.
- In a scratch branch, deleting the emitted `not_before ≤ at` constraint reclassifies `at` (E-static, ordinary).
- SEC-01 to SEC-21 are re-dispositioned at `9bcbca52` by Opus.

### S02: Certified primitive basis (U1; MC01/MC04, M5, M8, UNI-003/014, ZR06/ZR12/ZR13)

**Prerequisites.** The S01 field list. G4 is a required premise.

**Owned interfaces:**
- `relation/arithmetic.rs`
- `signed/schnorr.rs` and the Schnorr section of `signed/constraints.rs`
- `signed/nonce.rs` (Impact insert)
- Sherlock aux/2 arithmetic/fuel source
- G4 source-requirement record

**Basis:**
- Checked UInt64/UInt128 add, sub and mul.
- Comparisons and Boolean.
- Quotient/remainder witness relation (`q·d + r = n·w`, `r < d`, `d > 0`, inputs below 2^64).
- Hash/commitment.
- JubJub Schnorr: challenge reduction, response canonicality, R handling.
- Nonce insert.

**EARS clauses:**
- **S02.R1.** WHEN a primitive enters the basis, its certificate SHALL bind:
  - the specification and range preconditions;
  - the reference semantics;
  - the exact emitted constraint-fragment bytes and implementation version;
  - the cost relation;
  - the evidence class for completeness and for soundness, separately.
- **S02.R2.** IF a witness assigns an out-of-range value to any input, output or intermediate wire of a checked primitive (H01, H03), THEN the native verifier SHALL reject, shown by the §3 triple.
- **S02.R3.** WHEN host and circuit both implement the Schnorr reduction, then for these boundary inputs both SHALL produce identical accept/reject: challenge ≥ r_J, s = r_J − 1, s ≥ r_J, and R with a torsion component (H14, SEC-17).
- **S02.R4.** WHILE field primality and realization (G4) are unproved, every dependent certificate SHALL carry the G4 premise AND SHALL NOT be reported as unconditional.

**Independent acceptance:**
- Per primitive, a boundary-inclusive valid set is accepted (E-native).
- The hostile triple passes for each primitive.
- Opus disposition of SEC-17 (currently HYP): whether R subgroup membership is needed.

### S03: Full-Core rule-level correspondence and hostile-witness harness (U2; M1, MC01/MC04, UNI-003/019, MPLR-014/022/023)

**Prerequisites.** S01, and the S02 slice used by each rule.

**Owned interfaces:**
- `relation/compiler.rs` and the backend transition relation
- The production frontend admission matrix
- A new test-only witness-override crate, outside release

**EARS clauses:**
- **S03.R1.** WHEN a construct is admitted by the production frontend, its lowering rule SHALL have:
  - (a) completeness: each reference-valid execution has an accepted witness with identical complete observations; and
  - (b) soundness: each accepted witness decodes to a reference-valid execution with identical complete observations.
- **S03.R2.** IF evidence came from the `cfg(test)` draft frontend or an unregistered envelope, THEN it SHALL be recorded under that consumer AND SHALL NOT satisfy a production predicate unless the emitted ZKIR is byte-identical.
- **S03.R3.** WHERE the witness-override harness is used, it SHALL drive the byte-identical production circuit and keys, AND release build SHALL fail if the harness is linked.
- **S03.R4.** IF an unused effect, asset or duty slot carries a nonzero amount, a duplicate asset identity, or a present-zero duty treated as absent (H05), THEN verification SHALL reject.
- **S03.R5.** IF a witness reports success after logical-work exhaustion, or evaluates an eager Boolean differently from the reference (H18), THEN verification SHALL reject.

**Independent acceptance.** Every cell of the construct × H-class matrix (§3.4) has one of these dispositions:
- triple passed;
- not applicable, with a written reason;
- blocked.

The **mixed-payer refusal (N-1)** is recorded only as a negative control.

### S04: Sherlock interpreter circuit with bound program image (U0/U1/U2/U4 path; stagenet assignment)

**Prerequisites:**
- Phase-1 trace adjudication repaired. The fix is whitespace-aware syscall framing; this is an ordinary fix and needs no re-review.
- Phase 2.
- K33 checks run.
- A102 compiled.
- G4 adopted.
- P3 arena correspondence.
- S02 aux/2.

**Owned interfaces:**
- `literal_schema` and `emit_literal_step`
- The machine-circuit module
- aux/1 and aux/2 host packing
- The image-digest function

**EARS clauses:**
- **S04.R1.** WHEN a Sherlock proof is accepted, the statement SHALL bind the image digest, profile ID, initial and final state commitments, fuel budget and published result.
- **S04.R2.** IF a witness executes a different image (H12), THEN verification SHALL reject.
- **S04.R3.** IF any of the following occurs (H04), THEN verification SHALL reject:
  - an opcode selector is not one-hot;
  - a fetch reads outside image bounds;
  - an arena append overwrites a prior record;
  - an equality trace exceeds its declared capacity.
- **S04.R4.** IF fuel would underflow, THEN the circuit SHALL produce only the declared exhaustion outcome.
- **S04.R5.** WHILE Sherlock lacks certified financial suspension, Sherlock stagenet evidence SHALL be labeled scalar-scope. Financial V1 claims SHALL route through the Core compiler.

**Independent acceptance:**
- An explicit field adapter maps Quint/K traces to circuit step traces.
- Model counterexamples are replayed on the implementation.
- The hostile triple passes for H04, H12 and H18.
- One Sherlock program is deployed and called on stagenet, with readback. Deploy is separately admitted.

### S05: Authority, domain, time and replay (U0/U2; M7, M8, M9, M13, SEC-01/04/05/06/07/09)

**Prerequisites.** S01.

**Owned interfaces:**
- `signed/context.rs`, `signed/nonce.rs` and `signed/preparation.rs`
- `layout.rs` and `profile.rs`
- `mori-core/src/intent.rs` and `commands/verify_intent.rs`

**EARS clauses:**
- **S05.R1.** WHEN a signed intent states a validity window, the relation SHALL compare it to a time value that is public or ledger-read, not prover-chosen.
- **S05.R2.** IF the signed TTL exceeds `not_after` (H07), THEN admission SHALL reject.
- **S05.R3.** WHEN an asset is bound, its token identifier SHALL be computed from (network, issuer, reference) by the declared function.
- **S05.R4.** IF `asset.network` ≠ the deployment network (H09), THEN admission SHALL reject.
- **S05.R5.** WHEN authority is checked, the signing key SHALL be bound to an authority record supporting rotation and revocation.
- **S05.R6.** IF a revoked or unbound key signs (H13), THEN verification SHALL reject.
- **S05.R7.** WHEN an attachment valid for domain (network A, Anchor A) is submitted under domain B (H08), the target SHALL reject.
- **S05.R8.** WHEN a nonce is consumed on the target, a second submission, including a concurrent one (H10), SHALL be rejected by the ledger.
- **S05.R9.** IF the nonce map is full, THEN calls SHALL fail with a named error AND a declared successor path SHALL preserve duties (UNI-008).
- **S05.R10.** WHEN `verify-intent` reports `accepted`, its output SHALL enumerate non-signer effects as `unauthorized`.

**Independent acceptance:**
- Concurrent replay on stagenet.
- Cross-domain replay using stagenet versus a local domain. The Fenrir leg stays blocked-external.

### S06: Multi-party signed economics, phases and failure effects (U2/U3; M4, M11, UNI-004/005/006, MOR-011/012, MPLR-008/009/017/019)

**Prerequisites.** S01 and S05.

**Owned interfaces:**
- The signed policy (sole-payer and sole-debtor rules)
- The cap constraints
- `prepare.rs` phase layout
- `policy.rs` and `source_sponsor.rs`

**EARS clauses:**
- **S06.R1.** WHERE more than one party is debited or becomes a debtor, each party's own signed intent SHALL bound that party's gross debit, fees, liability and net floor.
- **S06.R2.** IF any effect debits a party without covering authority, THEN verification SHALL reject.
- **S06.R3.** WHEN liability caps are evaluated, sums SHALL be per debtor (SEC-11 guard; H15).
- **S06.R4.** WHEN a fee is charged, its asset, amount and recipient SHALL each be bound by the payer's intent (H06).
- **S06.R5.** WHEN a transaction uses guaranteed and fallible phases, the signed intent SHALL enumerate, for each failure point, the retained effects, fees, nonce consumption and residual duties.
- **S06.R6.** IF the fallible phase fails on target, THEN readback SHALL equal exactly the signed guaranteed-phase outcome.
- **S06.R7.** IF a plan relocates an effect between phases (H17), THEN verification SHALL reject.
- **S06.R8.** WHEN a program discharges a duty, the program SHALL declare its discharge rule (creditor consent or coupled payment).
- **S06.R9.** IF a discharge satisfies neither declared rule (H15), THEN verification SHALL reject.
- **S06.R10.** WHEN DUST sponsorship pays fees, the sponsor SHALL be a separately authorizing party.
- **S06.R11.** The sponsor-hold state SHALL record submission. IF `release_unused` follows a recorded submission (SEC-18), THEN it SHALL be refused.
- **S06.R12.** WHERE supply changes, mint and burn SHALL require the declared supply authority.

**Independent acceptance:**
- Stagenet induced fallible-phase failure with exact readback.
- The hostile triple passes for H06, H15, H16 and H17.

### S07: Stagenet single-stage E2E with two new contrasting programs (U2; M3, MC02/MC04, SP05, UNI-001/003/005)

**Prerequisites:**
- S01–S03.
- S05 R1–R6.
- S06 R3–R4.
- Stagenet tuple established.

**Owned interfaces:**
- Public CLI: `compile`, `compile-call --authorization signed-v2`, keygen, prove, `verify-signed-call`, deploy and readback.
- The offline readback helper.

**Programs:**
- **P-A, installment invoice:**
  - one asset;
  - a fee to a distinct treasury;
  - duty creation, accrual and partial discharge;
  - unrelated state preserved;
  - one `emit`.
- **P-B, conditional reserve allocation:**
  - two assets, with a limits row for each;
  - no fee;
  - a liability-only action plus a later discharge;
  - Bool state;
  - quotient/remainder weighted split.

Both programs are authored from public docs by a non-implementer. A third party freezes the oracle before any run.

**EARS clauses:**
- **S07.R1.** WHEN P-A and P-B execute on stagenet, authoritative readback SHALL equal the frozen oracle for:
  - every balance;
  - fee recipient;
  - custody and supply;
  - duty opening, accrual, discharge and closing;
  - nonce state;
  - preserved state.
- **S07.R2.** IF program, intent, predecessor, proof, fee, effect or domain bytes are mutated without valid corresponding bindings (H11, H26), THEN stagenet SHALL reject each mutation, with a valid unmutated positive control in the same deployment.
- **S07.R3.** WHEN verification is reported, it SHALL run in a process holding no prover key that rebuilds the verifying key from source.

**Independent acceptance:**
- Transaction identities.
- Readback diff against the oracle.
- N-1 and N-2 recorded but not counted as positives.

### S08: Conditional settlement, partial progress and recovery (U3; M13, M14, MC05/MC06, UNI-007/008, MPLR-001–010)

**Prerequisites.** S06 (multi-party) and S07.

**Owned interfaces:**
- Continuation `/2` and `/4` merged into the production path, with `mori-ledger` support.
- `policy.rs` recovery.

**Program P-C:**
- Spend at most 11 A, including at most 1 A in fees, to receive at least 20 B.
- Exact recipients and domains.
- A permitted partial fill.
- Recipient acceptance plus an authenticated document predicate.
- A separate recovery authority.
- Two independently authored routes.

**EARS clauses:**
- **S08.R1.** WHEN settlement and recovery are both submitted for one stage, at most one SHALL commit, AND the loser SHALL leave duties unchanged (H28).
- **S08.R2.** IF a document digest, a funding receipt or a timeout is presented in place of the bound delivery conditions, THEN release SHALL reject.
- **S08.R3.** WHEN a partial fill commits, the remainder obligation SHALL persist with the cumulative cap reduced by exactly the consumed amount.
- **S08.R4.** IF an ordinary path consumes reserved recovery work (H27), or rollover resets a spent budget, THEN verification SHALL reject.
- **S08.R5.** WHEN two independent routes satisfy the same signed constraints, both SHALL receive identical objective treatment.

**Independent acceptance.** On stagenet: both routes, the race, partial fill, recovery, and every hostile case listed in the ROADMAP "Immediate delivery brief".

### S09: Native recursion and private composition (U4; MC03, MC06, SP06/SP10, ZR02/04/05/08/09/14/16, MNR01–08, UNI-009/010/015/017)

**Prerequisites:**
- S07 and S08 lifecycle invariants.
- A released compatible Midnight interface (unknown).

**Owned interfaces:**
- The recursion lowering and the history statement.
- The private handoff API.

**EARS clauses:**
- **S09.R1.** WHEN a history claim is reported, it SHALL be labeled `native-recursive`, `ledger-induced` or another named mode.
- **S09.R2.** IF predecessor proofs are not verified in-circuit, THEN the claim SHALL NOT satisfy MC03 or MC06.
- **S09.R3.** IF any of the following is presented, THEN verification SHALL reject:
  - a fabricated genesis (H19);
  - an inner verifying key outside the bound compatible set (H20);
  - a parent resource used in two join branches (H21);
  - an omitted deferred or finalization check (H22).
- **S09.R4.** WHEN an inner proof is consumed, its public inputs SHALL equal the outer statement's referenced values (MNR verified-value binding).
- **S09.R5.** WHERE a private successor is claimed:
  - the successor SHALL run in a separately isolated party;
  - disclosure SHALL be confined to the declared set (H23);
  - a hidden reservation SHALL NOT be treated as absence (H16);
  - unavailable witness data SHALL route to a declared recovery path.
- **S09.R6.** WHEN composition is claimed, each of the five operators SHALL have positive and hostile evidence: sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging.

**Independent acceptance.** P-D (two native financial steps plus a bounded private join), on the target, with retained independent verification.

### S10: Federation, adapter and solver boundary (U5; M16, M17, UNI-011/012/013/015, ZR15)

**Prerequisites:**
- S08.
- The applicable S09 privacy evidence.
- Committee identity (unknown).

**EARS clauses:**
- **S10.R1.** IF proof for effect X is used to authorize Y, decoding is incomplete, or a fallback weakens policy (H24), THEN the adapter SHALL refuse.
- **S10.R2.** WHEN concurrent delegated tasks reserve funds, aggregate spent, pending and reserved amounts SHALL NOT exceed the attenuated budget across retry and failover (H25).
- **S10.R3.** WHEN an epoch migrates, duties and consumed authority SHALL persist.
- **S10.R4.** IF a migration resurrects consumed authority, THEN it SHALL be rejected.
- **S10.R5.** WHEN Moriarty is used directly on Midnight, no federation component SHALL be required.

**Independent acceptance:**
- Two independent solvers on P-C.
- One exact-byte external adapter.
- Payment, result and delivery kept separate.

### S11: ACTUS/DeFi complete conformance (U6; MC07, SP07/08/11, M15, UNI-014/020/023, MPLR-025/026)

**Prerequisites:**
- The S03 rules used by each row.
- The U0 numeric profile.

**EARS clauses:**
- **S11.R1.** WHEN conformance is reported, membership SHALL be validated against the frozen manifest for every retained count, with separate semantic, local, native, stagenet and Fenrir columns.
- **S11.R2.** IF held-out expectations were visible to implementers before the run, THEN the held-out cell SHALL be recorded as not held out.
- **S11.R3.** WHEN a port converts a price, it SHALL perform exact unit and orientation conversion before directed rounding. IF a reciprocal of an already rounded value is used (UNI-014 hostile), THEN qualification SHALL reject.
- **S11.R4.** WHEN netting is applied, gross debits, fees and liabilities SHALL remain individually observable (H16).

**Independent acceptance.** Every row has a disposition in every required column (see Disagreement D1). No family is skipped.

### S12: Independent developer release (U7; MC08, SP12, M10, G01–G24, UNI-001/016/024)

**Prerequisites.** S01–S11 at their declared scope.

**Owned interfaces:**
- Install packaging; one executable name.
- `commands/lsp.rs` and `commands/mcp.rs`.
- Release build configuration.

**EARS clauses:**
- **S12.R1.** WHEN a clean external environment installs the release, with network egress limited to the developer's Midnight node and wallet, the developer SHALL author, compile, prove, verify and submit a non-fixture program with no project registry, reviewer record or hosted service.
- **S12.R2.** WHEN two independent builders build the release from the tagged source, every compiler binary and every emitted ZKIR and verifying key for the release programs SHALL be byte-identical.
- **S12.R3.** IF an advertised command is unimplemented, THEN it SHALL exit with a defined refusal, not a panic (SEC-19).
- **S12.R4.** WHEN the release scope statement is published, it SHALL be generated from the predicate records, AND every open mandatory predicate SHALL be listed.

**Independent acceptance.** A developer outside the implementer pool completes the journey. Results are compared against the oracle.

---

## 3. QA strategy and fixture/requirement coverage

### 3.1 The hostile-test triple

A security rejection counts only with all three legs:

1. **Positive control.** The honest valid witness for the same circuit, key and deployment is accepted in the same run.
2. **Targeted hostile case.** The hostile witness, or the unchanged-proof statement mutation, differs from the positive control only in the targeted value. It is well-formed, passes envelope and deserialization checks, and is rejected by the verifier or ledger, not by the prover or parser.
3. **Necessity control.** On a scratch circuit with only the targeted constraint ablated, the same hostile witness is accepted. This shows the test discriminates. It requires native keygen and stays blocked until admitted. Until it runs, the cell is recorded as "two of three".

### 3.2 Hostile-class catalog

| ID | Hostile class |
|---|---|
| H01 | Range omission or field wraparound |
| H02 | Non-binary Boolean |
| H03 | Quotient/remainder violation: r ≥ d, d = 0, or q·d + r wrapping mod p |
| H04 | Selector, fetch or arena violation |
| H05 | Padding slot or duplicate asset; present-zero duty treated as absent |
| H06 | Fee recipient or fee asset substitution |
| H07 | Time outside window; TTL > `not_after` |
| H08 | Network or Anchor substitution |
| H09 | Token mapping mismatch |
| H10 | Nonce reuse, including concurrent reuse |
| H11 | Predecessor or state substitution |
| H12 | Program or image substitution |
| H13 | Key substitution or revoked key |
| H14 | Schnorr boundary (torsion R, s ≥ r_J, reduction mismatch) |
| H15 | Erased debt: unpaid discharge, expiry or rollover |
| H16 | Hidden reservation; netting that hides a fee or liability |
| H17 | Phase relocation or unsigned failure outcome |
| H18 | Exhaustion reported as success; eager-evaluation divergence |
| H19 | Fabricated genesis |
| H20 | Arbitrary inner verifying key |
| H21 | Repeated parent in a join |
| H22 | Omitted finalization or deferred check |
| H23 | Unauthorized disclosure |
| H24 | Proof for X used to sign Y |
| H25 | Copied budget under concurrency |
| H26 | Statement mutation with an unchanged proof |
| H27 | Reserve spent by an ordinary path |
| H28 | Late-success/refund double commit |

### 3.3 V1 release and QA gates (measurable)

- **RG-01 Denominator.**
  - WHEN V1 is declared, every retained heading SHALL be flattened into atomic predicates. Retained headings are U0–U7, M1–M17, L1–L8, MC01–MC08, SP01–SP12, G01–G24, ZR, MNR, MPLR, UNI, MOR, AEO, OSR, ACTUS/DeFi and composition.
  - Each predicate SHALL be `accepted` at its required evidence class.
  - IF any heading lacks atomic predicates, or any mandatory predicate is open, unknown or blocked, THEN V1 SHALL NOT be declared.
- **RG-02 Rule coverage.** Zero undispositioned cells SHALL remain in the production-construct × H-class matrix.
- **RG-03 Field coverage.** Zero intent-constrained fields SHALL be host-only or premise in the S01 manifest, unless an accepted premise is stated in the release scope.
- **RG-04 Hostile triple.** Every applicable H class SHALL have a complete triple on the canonical pipeline at E-native, and on stagenet where the class involves ledger state (H07, H08, H10, H11, H17, H28).
- **RG-05 New programs.** P-A and P-B SHALL pass S07.R1–R3 on stagenet.
- **RG-06 Failure effects.** At least one induced fallible-phase failure per distinct phase layout used by any release program SHALL match the signed outcome on stagenet.
- **RG-07 Recursion mode.** MC03 and MC06 SHALL be closed only by `native-recursive` evidence under S09.R1–R4.
- **RG-08 Composition.** All five operators SHALL pass S09.R6.
- **RG-09 Conformance.** All retained counts SHALL be in the S11 columns. Required columns are per D1.
- **RG-10 Targets.**
  - The stagenet tuple SHALL be pinned with no `unknown` fields.
  - Fenrir M2/M3 SHALL be qualified, or removed from V1 only by explicit user decision recorded in the scope statement.
  - Preview SHALL NOT substitute for either.
- **RG-11 Permissionless journey.** S12.R1 SHALL pass with an egress log showing no project, registry or hosted-service endpoints.
- **RG-12 Reproducibility.** S12.R2 SHALL pass.
- **RG-13 Verifier independence.** Proofs for P-A, P-B, P-C and P-D SHALL verify using the Midnight-native verifier, the published verifying key and the published statement encoding, without Moriarty verification code (SEC-15).
- **RG-14 Release contents.**
  - IF the release links the witness-override harness, the `cfg(test)` frontend, or a panicking command, THEN the release build SHALL fail.
  - Mapped-role mode SHALL emit its qualification string on every output.
- **RG-15 Test-result integrity.** Pass counts SHALL be parsed from the runner's printed structured output. An inferred "zero failed and ignored" result does not qualify as a gate input.
- **RG-16 Oracle independence.** Every scored expectation SHALL come from an author with no access to the implementation under test, frozen before the run, with shared-code dependencies listed.
- **RG-17 Resource ceilings.**
  - Every keygen path, including the library `generate_call_keys`, SHALL refuse k > 19 or an unpinned SRS.
  - The toolchain SHALL refuse encoded transactions above the target byte ceiling before submission.
  - Release program transaction sizes SHALL be measured on stagenet.
- **RG-18 Findings.** Every SEC and OSR finding SHALL carry a current Opus disposition: fixed with regression test, designed limitation stated in scope, or V1-blocking.

### 3.4 Coverage matrices and fixtures

**Matrices** (reusing the existing `AUDIT-PLAN.md` outputs, not new tooling):
- Production construct × H-class.
- Statement field × enforcement class × stack.
- Family row × evidence column.
- Requirement → atomic predicate → evidence (`predicates.jsonl`, `evidence.jsonl`, `coverage.json`).

**Fixtures:**

| Fixture | Role |
|---|---|
| P-A, P-B | U2 positives |
| P-C | U3, two routes |
| P-D | U4 history and join |
| P-S | One Sherlock scalar stagenet program |
| N-1 | Mixed-payer refusal |
| N-2 | Malformed envelope; never counted as semantic rejection |
| Held-outs | Custodied by an oracle author, not implementers |

**Formal / native / target distinction.**
- E-model evidence requires an explicit adapter and implementation replay of counterexamples before it informs correspondence. It never satisfies an E-native or E-target gate.
- Lean/Clean stays developer QA.
- ROADMAP wording calling it a "required Lean/Clean stage" should be factually corrected.

---

## 4. Factory workflow, parallel work and bottlenecks

**Workflow.**
- Ordinary PRs to `main` with frequent small integration.
- Ordinary non-native CI on every PR, using inspected filters only.
- One integration owner for shared IR, ABI and statement interfaces.
- Native and stagenet runs are batched per separately admitted request under the existing counters.
- No scheduler, campaign tooling or receipt-review layer.

**Note on stale text.** The ROADMAP still says "Foreman/Pel governs project implementation". Per the current override, that should be corrected as a factual edit.

**Roles:**
- Implementers per sprint.
- Independent oracle and fixture authors who do not implement.
- Opus for all security design, hostile catalogs and dispositions.
- Release QA drawn from outside the implementer pool. Gemini and Astra can contribute nonsecurity review and conformance work.

**Parallel lanes now** (source and ordinary work):
- S01 manifest.
- S02 certificate drafting.
- S04 Sherlock source and phase-1 regex fix.
- S05 and S06 designs.
- S11 semantic preparation and held-out custody.
- S12 refusal-not-panic, single executable name and release-build exclusion checks.
- P-A and P-B authoring with frozen oracles.

**Serial chain:**
1. S01
2. S03, with the S02 slice
3. S07
4. S08, which needs S06
5. S09
6. Then S10 and S12

**Bottlenecks:**
- Native admission and resource counters, especially for the necessity-control keygens.
- Stagenet tuple facts.
- Fenrir facts.
- Upstream Midnight recursion interface.
- Opus review throughput.
- The single IR integration owner.
- Availability of independent oracle authors.

---

## 5. Risks, disagreements, unknowns and next deliverables

### Risks
- Multi-party authority (S06) is a large redesign of signed-v2 and gates U3, U5, M14 and M15.
- The ledger-9 recursion route may not be native recursion. U4 then depends entirely on the upstream interface.
- The necessity controls multiply native keygen demand.
- SEC findings are unverified at `9bcbca52`.

### Disagreements

**D1. Native and target columns for conformance rows.** For each ACTUS/DeFi row, should native and stagenet evidence be required?
- **My recommendation:**
  - every row: E-ref oracle plus production compile;
  - E-native and stagenet for a representative set covering every lowering rule, effect kind and phase layout used by any row.
- **This is a scope decision for the user.** The MC07 heading "Extended native predicate evidence" suggests a stricter reading.

**D2. Fenrir as a V1 gate.**
- **My position:** Fenrir is a V1 gate per M2/M3.
- **Descoping** requires an explicit user decision.

**D3. Single-signer adequacy.** I upgrade Opus4's dissent: multi-party authority is V1-mandatory.

**D4. Canonical stack versus differential gate.** I recommend one canonical stack per claim, with the other stack as a differential leg only.

**D5. SEC-17 R subgroup membership.** Remains a hypothesis.

**D6. UNI-024 routing text.** It names GPT-6.1 Sol and Muse, which conflicts with the current reviewer set. A factual update is needed.

### Unknown facts
- Stagenet network ID, Anchor, ledger and verifier versions, and endpoints.
- All Fenrir identities.
- Whether ledger-9 ZKIR equals a pinned ZKIRv3 version.
- The actual transaction byte ceiling.
- SRS provenance beyond the SDK digest table.
- DUST mechanics on stagenet.
- The mode of the ledger-9 recursion route.
- The contents of OSR-1014, ZR/MNR bodies, SP10.1 and the frozen manifests.
- The state of SEC-01 to SEC-21 at `9bcbca52`.

### Next three concrete deliverables

All three are proposals for later execution. None is authorized here.

1. **S01 statement manifest emitted by `mori compile`, plus `compile_reported` coverage labeling.** Includes an Opus re-disposition of SEC-01 to SEC-21 at `9bcbca52`. Source and ordinary code only.
2. **Test-only witness-override harness over byte-identical production circuits, plus authored triple tests.** Covers H01, H03, H05, H06, H07, H14 and H26 on signed-v2, and H04, H12 and H18 on Sherlock aux/2. Tests stay unrun pending a separate native admission request.
3. **Opus multi-party signed-authority design for S06.** Covers per-party caps, the fee-recipient binding, the per-debtor liability guard (SEC-11) and the phase/failure-outcome signing format. It is the precondition for P-C.

Separately, the nonsecurity lane authors P-A and P-B with frozen independent oracles.

This plan creates no native execution authorization, resource reset or acceptance status.
