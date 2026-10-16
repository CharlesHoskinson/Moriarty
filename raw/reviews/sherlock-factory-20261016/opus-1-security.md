# Moriarty V1 planning review: security-owner lens (Opus 5.5)

**Status.** This is a planning proposal by the designated Opus security owner. It is not product acceptance, not a security approval, and not a source or runtime audit. It creates no native execution authorization, no resource reset, no gate and no new process.

## Inspection limits

**Read in full (the supplied text only):**
- `ROADMAP.md` (`0afd1af3…`)
- `docs/MORIARTY-PRODUCT-CONTRACT.md` (`d5674c65…`)
- consolidated `requirements.md` (`aa7b2243…`)
- `spec.md` UNI-001–024 (`66f85468…`)
- `traceability.md` (`d549f31a…`)
- the complete heading index (`a361071a…`)
- the latest Root phase-1 receipt
- `SECURITY-REVIEW.md` (`64cb033e…`), `SECURITY-ROUTING.md` (`04699b07…`), `MID-ROADMAP-REVIEW.md` (`6f2a8bc3…`) and `AUDIT-PLAN.md` (`9aaec5d0…`)

The hashes are as you supplied them. I did not recompute them.

**Not read (bodies not supplied):**
- The OSR-1014 security addendum. I cite it by ID only and cannot check overlap with the clauses proposed below.
- The ZR01–ZR16 and MNR01–MNR08 bodies.
- The MPLR notes and SP10.1.
- The Sherlock design and model sources, A102 mapping, aux/1, aux/2, the G4 and arena records, and the stagenet assignment.
- The PR95–101 diffs and the location of `compile_reported`.
- All product Rust/TypeScript source.

**Line references.** `path:line` references are carried from `SECURITY-REVIEW.md` at frozen tree `cb8280ef`. They have not been re-anchored to baseline `9bcbca52`, and lines may have moved. Re-anchoring is part of deliverable 1.

**No tools.** I used no tools, shell, writes or web, and ran nothing. Every test and result count below is source text.

**Formal/native/target labels used throughout:**
- **[F]** model or formal: Quint, K, or Lean/Clean as developer QA only.
- **[L]** local Rust/host ordinary execution.
- **[N]** native SDK check, VM, keygen, prove or verify. Each item needs the existing separate admission; none is created here.
- **[T]** target. Midnight stagenet is required. Preview and Fenrir are separate targets with no transfer between them.

---

## 1. Current evidence versus incomplete behavior

### 1.1 Corrections to the supplied roadmap text

| # | Roadmap text | Supplied newer fact | Disposition |
|---|---|---|---|
| C-1 | "all 18 tests, the sampled run and all K checks remain UNRUN" | The Root phase-1 receipt shows 18 named normal tests passed once: exit 0, no parent timeout, closure post-rehash MATCH. Failed and ignored counts were inferred, not printed. | Update the roadmap. The sampled run and the K33 checks remain UNRUN. **Security note:** all 18 are *Normal* tests. No adversarial or overflow model test ran. |
| C-2 | Sherlock "scalar-only" restriction, read broadly | `compile_reported` calls the existing backend compiler, then appends a Sherlock projection report. The Core compiler already lowers state, assets, transfers and liabilities. | Any claim that Core compilation is scalar-only is wrong. The converse hazard is real too: the appended report could be misread as evidence that Sherlock produced the artifact (OSR-1016-S1.1). |
| C-3 | Draft frontend results (factorial 24, tree fold 7) | The production and `cfg(test)` draft frontends differ. | Test-only frontend results are not production-profile evidence (OSR-1016-S1.2). |
| C-4 | "Foreman/Pel governs project implementation only"; the OpenSpec "Pel plan" | User override: no Foreman. | Strike these from forward planning. UNI-001's statement that the public pipeline needs no Foreman stays valid. |
| C-5 | "The design includes a required Lean/Clean stage" | The product contract says Lean is not a dependency. Per your clarification, Lean/Clean is developer QA only. | Reword to "optional developer-QA model". No public compiler, runtime or prover dependency. |
| C-6 | Phase-2 hold in the receipt | The trace adjudication failed on a whitespace regex bug in the query, not in the model. | Fix the parser once, as an ordinary defect (§5). I add no adjudication layer. |

### 1.2 Evidence by class

| Capability | Strongest evidence | Class | Incomplete behavior |
|---|---|---|---|
| Rust `moriarty/1` / `moriarty-core/1` check, simulate, test, intent | Captured local results; 4 CLI report regressions (2026-10-15) | [L] historical-captured | U0 matrix; U2 correspondence; M10 |
| Core compile to ledger-9 ZKIR | Source; bounded lowering | [L] source-static | Rule-level correspondence; native; target tuple |
| Static all-action export/reopen (PR47/49) | Captured source-static | [L] | Mapped mode is unauthenticated (SEC-08) |
| Compiler provenance metadata | 10 focused tests; one byte-identical specimen | [L] | Full source/result audits |
| Core continuations (`/2` await, `/4` N+1 circuits) | 58/78/86/88-function observations, partly carried forward rather than rerun; unmerged | [L] partial | Native through `mori-ledger` unsupported for `/2` |
| Oct 11 signed payment | One genuine local proof; 6 hostile-witness and 6 mutation rejections | [N] HIST at `ee963f61`, backend stack only | Does not cover ledger signed-v2 (SEC-02); no account authority (SEC-04) |
| FeePool | 23 actions, 162 cases | [L] captured | Full M11, Capacity Exchange, custody |
| Sherlock K-A Quint | 98 + 126 normal tests | [F] | Refinement, circuit, native |
| Sherlock arithmetic/fuel Quint | 18 normal tests, run once | [F] | Adversarial model cases; K33 UNRUN; circuit |
| Sherlock append model | 8 normal tests plus a short simulation | [F] | P3 correspondence |
| Host image/helper/checkpoint interpreters | 46/79/82/86 host checks | [L] private, test-only | Canonical checkpoint serialization; B profile |
| A102 literal step | 16 registered tests, UNRUN; fragment not compiled | source only | Everything downstream. The "C4" label's meaning was not supplied. |
| G4 field and instruction record; G4-BYTE draft | Conditional source argument; draft unadopted | source only | Modulus primality unproved; field realization; native |
| Offline readback helper | 16 normal assertions | [L] | No target result |
| Midnight stagenet | None | — | Required, unexecuted |
| Fenrir | None | — | Version, network ID, Anchor, committee, tuple all unknown |

### 1.3 Soundness reconciliation across the seven dimensions (security lens)

Enforcement classes: **C** = circuit constraint, **H** = host check, **P** = caller premise.

| Dimension | What is enforced today, and where | Soundness gap | Owner |
|---|---|---|---|
| **Source** | Lexer and parser per profile (H) | Two `mori` executables. A beta Source/6 path is separate. A test-only draft frontend exists. Source-to-statement identity is unbound beyond hashes (SEC-16). | S1, S11 |
| **Core** | Typing and checked arithmetic (H); range-checked Sub/Add in the circuit (C) | No division operator. Eager booleans. Present-zero vs absent duties are profile-specific. No rule-level compiler correspondence. | S1, S3, S5 |
| **Intent** | Signature-bound payload digest P (C); interior fields bound by host equality (H) (SEC-03) | TTL and `not_after` are unrelated (SEC-01). `token_hex` is not derived (SEC-09). Legacy `verify-intent` reports `accepted` despite third-party debits (SEC-07). | S1, S2 |
| **Effect** | Gross, fee, net and liability caps (C) on the single-payer path | Liability cap coupling (SEC-11). Discharge is not coupled to payment (SEC-12). No mint/burn or custody beyond the sole payer. | S2, S5, S7 |
| **Authority** | Schnorr equation under a *compiled constant* key (C) | No account/key link, rotation or revocation (SEC-04). R subgroup is HYP (SEC-17). The two stacks expose different public fields (SEC-02). | S2 |
| **History** | Nonce Impact insert over a *supplied* map (C over P) | Chain uniqueness, reorg, predecessor authentication and capacity (SEC-06) are all P. No recursion. | S6, S8 |
| **Failure** | Local atomic rejection (H) | Guaranteed phase only. DUST is `None` (SEC-21). Sponsor release does not record submission (SEC-18). No fallible-phase model. | S7 |
| **Interpreter (new)** | None in a circuit | The program image digest is not yet in any statement. Arena consistency, fuel and terminal publication are unconstrained beyond models. | S3, S4 |

**Three-path warning.** There are now three potential ZKIR producers:
- the backend transition relation,
- the ledger native-call relation,
- a future Sherlock step circuit.

Evidence from one path does not transfer to another (extending SEC-02). Every evidence record must name its path (OSR-1016-S1.1).

---

## 2. Prioritized capability sprints

Each sprint must ship a working public or library capability and the tests that prove it. Proposed clause IDs are `OSR-1016-Sn.m`; they extend and do not replace OSR-1014. Every rejection test is paired with a valid positive control, under the rule in §3.2.

**Ordering.** The critical path is S1 → S2 → S5 → S6 → S7. S3 and S4 run in parallel and join at S5's stagenet stage only if you require Sherlock to be the stagenet path (see unknown U-5). S8–S11 keep their U dependencies.

### S1: U0 security-bound semantic contract and honest reporting

- **Public capability:**
  - Machine-readable profile, frontend and lowering provenance in `compile`, `check` and `test` output.
  - `verify-intent` output that lists unauthorized parties.
  - `lsp`/`mcp` refuse instead of panicking.
  - An enforcement map (field × path × C/H/P).
- **Prerequisites:** none. Ordinary source work.
- **Owned interfaces (from SECURITY-REVIEW, to re-anchor):**
  - `crates/mori/src/commands/{check,compile,verify_intent,lsp,mcp}.rs`
  - `compile_reported` (path not supplied)
  - `mori-core/src/intent.rs`
  - the U0 numeric/intent profile document
- **Retained IDs:** U0, UNI-002/016/018/019/021, M6, M10, MOR-007, AEO-001; SEC-07, SEC-10, SEC-19.

**EARS:**
- **S1.1** WHEN any command emits a ZKIR artifact or report, the toolchain SHALL record the producing lowering (backend transition, ledger native-call or Sherlock circuit), source profile, frontend build identity and admitted subset. IF a Sherlock projection report is appended to an artifact Sherlock did not produce, THEN the report SHALL carry a field stating that it did not produce the artifact.
- **S1.2** IF an input was accepted only by a `cfg(test)` frontend, THEN no public command or evidence record SHALL report acceptance under the production profile.
- **S1.3** WHEN `verify-intent` validates a signature over effects that debit or obligate any party other than the signer, the output SHALL enumerate those effects as unauthorized by this signature and SHALL NOT emit an unqualified `accepted`.
- **S1.4** IF `mori lsp` or `mori mcp` is invoked while unimplemented, THEN the command SHALL exit nonzero with an "unsupported" diagnostic and SHALL NOT panic.
- **S1.5** WHERE a statement field is enforced only by a host check or caller premise, the enforcement map and every report SHALL label it H or P and SHALL NOT describe it as proved.
- **S1.6** WHEN the U0 numeric profile is versioned, it SHALL define fee beneficiary policy when owner ≠ beneficiary, price orientation, unit dimensions, per-primitive rounding direction, and the authoritative time source and TTL relation.

**Tests and acceptance [L]:**

| Test | Mutation or input | Expected | Positive control |
|---|---|---|---|
| S1.3 | Program adds a third-party `from` | Third-party effect listed as unauthorized; no unqualified `accepted` | Owner-only program: `accepted` with an empty unauthorized list |
| S1.1/S1.2 | Draft-frontend-only input | Refused or labeled | Same source through the production frontend: label matches |
| S1.4 | Invoke `mori lsp` | Exit nonzero, no panic | — |

Opus reviews the enforcement map against `9bcbca52` source.

### S2: Signed-intent authority binding on the ledger signed-v2 path (M7, M8, M9)

- **Public capability:** signed-v2 preparation that refuses inconsistent time, asset and network bindings, and reports that state the authority scope truthfully.
- **Prerequisites:** S1.6 ruling on time and TTL.
- **Owned interfaces:** `mori-ledger/src/signed/{mod,context,preparation,constraints,schnorr,nonce}.rs`, `layout.rs`, `profile.rs`.
- **Retained IDs:** M7, M8, M9, MC05/L3, UNI-003/004, MOR-003/004; SEC-01, 03, 04, 09, 11, 17.

**EARS:**
- **S2.1** IF a signed request's `ttl` and `not_after` violate the S1.6 relation, THEN `SignedCallRequest::admit` SHALL refuse with a named reason.
- **S2.2** IF any bound asset's network differs from the deployment network, THEN signed preparation SHALL refuse before constraint construction.
- **S2.3** WHERE `token_hex` is bound to (network, issuer, reference), the toolchain SHALL either verify it under a stated derivation or emit it as an owner-signed premise in the enforcement output.
- **S2.4** WHILE the sole-debtor rule holds, the circuit's per-asset liability sum and Core's signer-debtor sum SHALL be equal. IF the sole-debtor rule is relaxed, THEN compilation SHALL refuse until the circuit cap ranges over signer-debtor duties only.
- **S2.5** IF `at` is a private prover choice, THEN reports SHALL state that expiry is enforced by owner-signed TTL and SHALL NOT claim circuit- or ledger-time enforcement.
- **S2.6** WHERE a compiled constant key is used, reports SHALL NOT claim account authority, rotation or revocation.
- **S2.7** WHEN the in-circuit Schnorr check accepts, the host and circuit challenge reductions SHALL be equal for all inputs, and R SHALL be on-curve. IF either condition fails for a witness, THEN no accepting proof SHALL exist.

**Tests:**
- [L] B2 (`ttl > not_after`) and B4 (network mismatch) flip from "currently accepted" to refused. Positive controls: `ttl == not_after` and matching network, both admitted.
- [L] S2.4 guard test: relax sole-debtor in a test fixture and expect refusal. The unmodified fixture compiles.
- [N] blocked pending admission: WS-C1 (s+1, wrong key, each cap ±1, nonce reuse) and WS-C3 (torsion R; challenge at or above r_J). Each runs only after the honest proof verifies on identical keys.
- [F] optional developer QA: a written statement of the SEC-17 argument with the on-curve premise named.

### S3: U1 certified arithmetic and fuel basis (aux/2 checked UInt128, G4)

- **Public capability:** a primitive certificate record and a ZKIR fragment for checked UInt128 add, sub and mul plus the fuel charge, usable by both Core lowering and Sherlock.
- **Prerequisites:** none for source and [F] work. [N] needs admission.
- **Owned interfaces:**
  - `relation/arithmetic.rs`
  - `models/sherlock/machine-circuit/arithmetic-fuel-source-20261016/*`
  - `g4-source-requirements-20261016/*`
  - the G4-BYTE draft (adoption decision)
- **Retained IDs:** U1, M5, MC01, SP01/03, ZR06/12/13, MNR01/07, UNI-014, MPLR-020/021, MOR-010.

**EARS:**
- **S3.1** WHEN a checked UInt128 operation is lowered, the fragment SHALL constrain every byte limb to [0, 255] and every carry or borrow to {0, 1}, such that no witness with an out-of-range limb or carry satisfies the fragment.
- **S3.2** IF the exact integer result lies outside [0, 2^128 − 1], THEN the evaluator SHALL refuse, and no satisfying witness SHALL exist for any in-range claimed result.
- **S3.3** WHERE soundness relies on the ledger-9 field, the certificate SHALL cite the pinned source modulus and SHALL bound every intermediate below it. It SHALL NOT rely on unproved primality.
- **S3.4** WHEN fuel is charged, the fragment SHALL bind `fuel_out = fuel_in − charge` with `charge ≤ fuel_in`. IF `charge > fuel_in`, THEN the step SHALL yield the declared exhaustion outcome and SHALL NOT yield success.
- **S3.5** WHEN a primitive certificate is claimed, it SHALL bind specification, preconditions, reference semantics, constraint fragment, implementation version, theorem evidence and the cost relation. Logical lanes SHALL NOT be reported as measured circuit cost.

**Tests:**

| Class | Positive controls (all accepted) | Rejection tests |
|---|---|---|
| [F] | `0xFF…FE + 1`; `0 − 0`; max × 1; full 16-limb carry chain | `0xFF…FF + 1` and `0 − 1` refused. Add adversarial Quint and K cases for these; the run 18 are Normal only. Run K33. |
| [N] blocked | Valid limbs and carries for each positive operand | Limb = 256 with compensating lower limb; carry = 2; result `a + b − p`; fuel claim with charge > fuel_in |

### S4: Sherlock step-circuit statement binding (A102 → step circuit → P3 arena)

- **Public capability:** an interpreter proof statement that binds program image, versions, state commitments, fuel and outcome, with production-frontend input.
- **Prerequisites:** S3 fragments; A102 compiled under [L].
- **Owned interfaces:** `literal_schema`, `emit_literal_step`, `models/sherlock/machine-circuit/*` (the full39 map, arena policy, aux/1 and aux/2), checkpoint serialization.
- **Retained IDs:** U0/U1/U2/U4, M6, ZR01/03/05, MNR04/05, UNI-003/019, MPLR-022.

**EARS:**
- **S4.1** WHEN an interpreter proof is produced, its public statement SHALL bind the program image digest, `sherlock-*` profile and schema versions, initial and final state commitments, fuel bound and outcome tag. IF the witness image differs from the bound digest, THEN no accepting witness SHALL exist.
- **S4.2** WHEN an arena read occurs, it SHALL equal the most recent append to that address. IF a witness supplies another value, THEN the equality trace SHALL fail. WHERE capacity is symbolic, a concrete bound SHALL be fixed, and overflow SHALL refuse.
- **S4.3** IF execution has not reached a terminal frame, THEN terminal publication SHALL NOT be accepted. The free K-A publication rule SHALL apply only to terminal states.
- **S4.4** WHEN execution resumes from a checkpoint across proofs, the resumed state SHALL match a canonical serialization commitment. IF no canonical serialization exists, THEN cross-proof resume SHALL NOT be claimed.
- **S4.5** IF a decoded instruction is outside the adopted full39 set, THEN the step SHALL reject.

**Tests:**
- Positive pair through the *production* frontend: factorial(4) = 24 (recursive calls) and tree fold = 7 (ADT traversal). Contrasting third case: a two-asset Core program, compiled by Core and projected by Sherlock and compared only on the common domain (UNI-019).
- Rejection tests: swapped image digest; flipped branch selector; stale arena read; fuel claimed without charge; non-terminal publication; an opcode outside the set.
- Classes: [F] model, [L] host interpreter, [N] blocked, [T] only after S5's pipeline exists.

### S5: U2 two new positive programs to Midnight stagenet (M1, M3, MC01/02/04/05)

- **Public capability:** a developer compiles, proves, verifies, deploys and calls two genuinely new programs on stagenet, with full readback.
- **Prerequisites:**
  - S1 and S2.
  - S3 for the arithmetic used.
  - Stagenet network ID, Anchor and tuple obtained from the operator, never invented.
  - Native admission under the existing restrictions.
- **Recommended pipeline:** ledger signed-v2 installed. The backend stack is a differential leg only.
- **Owned interfaces:** `compile-call --authorization signed-v2`, keygen, prove, `verify-signed-call`, a new stagenet submit/readback consumer reusing the offline readback helper.
- **Retained IDs:** U2, M1, M3, M9, MC01/02/04/05, SP02/03/05, UNI-001–005/018–020, MPLR-012/014/017/023.

**Positive programs** (oracle values below are illustrative; the independent oracle author freezes the real ones):
- **P-A, installment invoice.** One asset A. The owner pays beneficiary 40 A plus a 1 A fee to a distinct treasury. Opening duty 100; a discharge of 40 is *coupled in the same action* to the 40 transfer; closing duty 60; one `emit`.
- **P-B, conditional reserve allocation.** Two assets, A and C. Separate create-liability and discharge actions. Bool state. Quotient/remainder split with `requires q*d + r == n*w`, `r < d`, `d > 0`, inputs below 2^64. For example `n = 1000, w = 3, d = 7` gives `q = 428, r = 4`. Two recipients; the remainder 4 is retained and recorded as reserve dust.

**EARS:**
- **S5.1** WHEN U2 acceptance is claimed, both positive programs SHALL traverse the same declared pipeline from public source to stagenet call and authoritative readback.
- **S5.2** WHEN a stagenet call is accepted, readback SHALL reconcile every complete-effect field against the frozen oracle: per-asset transfers, fees and recipients, custody, mint/burn, liability opening/creation/accrual/discharge/closing, duties, nonce consumption, events and preserved state. IF any field differs or is unread, THEN the record SHALL be "divergent", not "passed".
- **S5.3** IF the stagenet network ID, Anchor or tuple is unknown, THEN the toolchain SHALL refuse to sign a target-bound intent and SHALL NOT substitute `local-unqualified`.
- **S5.4** WHEN verification is claimed independent, the verifier process SHALL re-derive the VK from source and pinned SRS without reading the PK, in a process separate from the prover.
- **S5.5** IF a discharge reduces a liability without the program's declared payment coupling or creditor consent, THEN Core and the circuit SHALL refuse.

**Rejection tests:**
- N-1: mixed payer expects `one-public-payer`. This is a control and never counts as the second positive program.
- N-2: malformed envelope. Recorded, but not counted as a semantic rejection.
- Single-field mutations of P-A: fee +1, recipient, asset, net −1, liability cap −1, uncoupled discharge, and program, intent, proof, VK and PI bytes. Each runs only after the unmutated P-A verifies.

### S6: History, replay, nonce and registry anchoring (M6, M7, M12; scoped U2 history)

- **Public capability:** on-chain replay protection, cross-domain refusal, and authenticated predecessor checking on stagenet.
- **Prerequisites:** S5 pipeline; stagenet facts.
- **Owned interfaces:** `signed/nonce.rs`, `context.rs` predecessor handling, the verifier predecessor input, the M12 registry binding (design open).
- **Retained IDs:** M6, M7, M12, MC04, UNI-008/009/015, MPLR-022/027, SEC-05, SEC-06.

**EARS:**
- **S6.1** WHEN an intent is consumed, its nonce SHALL be inserted into the authenticated ledger map in the same transaction. IF the nonce is present, THEN the transaction SHALL fail with no effects beyond declared guaranteed-phase fees.
- **S6.2** WHEN the nonce map reaches capacity, signed consumption SHALL refuse with a named reason, and any successor migration SHALL preserve every consumed nonce.
- **S6.3** IF an intent signed for (network, Anchor) X is presented under Y, THEN verification SHALL reject.
- **S6.4** IF a predecessor is caller-supplied rather than checked against authenticated ledger state, THEN the record SHALL label it a premise.
- **S6.5** WHERE ledger-induced history is used, records SHALL label it as such and SHALL NOT count it toward MC03 or MC06.
- **S6.6** IF a reorg removes a consuming block, THEN nonce state SHALL follow ledger state, and no off-chain cache SHALL be authoritative.

**Tests:**
- [T] Replay the identical P-A transaction and expect rejection; the control is the first submission accepted.
- [T] Cross-network replay, using two operator-provided domains only.
- [L] Capacity refusal at the 12,153-entry bound. Control: the entry before the bound is accepted.

### S7: U3 conditional settlement, partiality, recovery, phases, DUST and sponsorship (M4, M11, M13, M14)

- **Public capability:** the discriminator "spend at most 11 A including at most 1 A fees, receive at least 20 B", with escrow, recipient consent, a document predicate, partial fill, recovery and a late-success/refund race, runnable by a developer.
- **Prerequisites:**
  - S5 and S6.
  - **A multi-party authority design.** The single-signer sole-payer rule cannot admit the counterparty B debit; this is Opus4's dissent, adopted as a blocker.
  - The phase layout model.
- **Owned interfaces:** `mori-core/src/policy.rs`, `source_sponsor.rs`, the `prepare.rs` DUST and phase paths, a new multi-signer intent schema.
- **Retained IDs:** U3, M4, M11, M13, M14, L1/2/4/5/7/8, MC04/05/06, SP10, UNI-005–008, MPLR-001–010/017/019, ZR10/11/12.

**EARS:**
- **S7.1** WHEN the discriminator completes, acceptance SHALL require gross A ≤ 11 including fees ≤ 1, and net B ≥ 20 to the exact signed recipient and domain. WHEN a permitted partial fill commits, A consumed SHALL be proportional, and the remainder and duty SHALL persist.
- **S7.2** WHEN a fallible phase fails, the outcome SHALL record guaranteed-phase effects, fees, nonce consumption and unresolved duties, and SHALL NOT be reported as rollback or completion.
- **S7.3** IF recipient consent and the authenticated document predicate are not both satisfied, THEN release SHALL reject. A digest, a funding receipt or a timeout SHALL NOT substitute for either.
- **S7.4** WHILE settlement is pending, recovery SHALL require its separately signed authority. IF late success and refund race, THEN exactly one SHALL consume the escrow.
- **S7.5** IF a sponsor submission was recorded, THEN `release_unused` SHALL refuse.
- **S7.6** WHEN ordinary work is exhausted, only the authorized recovery path SHALL consume reserved closure work.
- **S7.7** IF a successor or rollover resets a spent budget or erases debt, THEN acceptance SHALL reject.

**Rejection tests** (from the roadmap list): extra fees, wrong asset/recipient/domain, fabricated genesis, wrong relation or key, hidden reservation, erased debt, unauthorized disclosure, replay, state or work exhaustion, and late settlement racing recovery.
- Two independently authored routes must both be accepted (route neutrality, without U5 infrastructure).
- Uncommitted solver candidates must be distinguished from committed partiality.

### S8: U4 native recursion and private composition (MC03, MC06, SP06, SP10)

- **Prerequisites:**
  - S5 and S7 lifecycle invariants.
  - Accepted SP07/SP08 inputs for each claimed successor scope.
  - Released interface qualification. The user-assumed horizon is not a fact.
- **Owned interfaces:** the recursion route (the "demonstrated ledger-9 route without VerifyProof" was not supplied to me), the Sherlock checkpoint commitment, and the composition operators.
- **Retained IDs:** U4, MC03, MC05, MC06, SP06, SP09, SP10, ZR02/04/05/07/08/09/14/15/16, MNR01–08, L6, UNI-009/010/017, MPLR-011/013/027/029, all five operators.

**EARS:**
- **S8.1** IF a recursive history's base is not the committed legitimate genesis, THEN verification SHALL reject.
- **S8.2** WHEN a predecessor proof is consumed, its key SHALL belong to the authorized compatible key set bound in the statement. IF an arbitrary verifier key is used, THEN the step SHALL reject.
- **S8.3** IF a multi-parent join reuses any parent resource, THEN the join SHALL reject.
- **S8.4** IF any deferred obligation remains undischarged at finalization, THEN the history SHALL NOT be accepted.
- **S8.5** WHERE private handoff is claimed, the public statement SHALL reveal only declared observations. IF a hidden reservation is treated as absence, THEN completeness SHALL NOT be accepted.
- **S8.6** For each operator (sequential, disjoint parallel, shared-state interleaving, atomic synchronization, asynchronous messaging), acceptance SHALL require one valid composition and one interference or reuse rejection.

### S9: U5 federation adapter, solvers, delegated budgets and x402 (M16, M17)

- **Prerequisites:** S7; applicable S8 scope. Fenrir committee identity remains unknown, so direct Midnight use must remain functional without any federation.
- **Retained IDs:** U5, M16, M17, K3–K5, UNI-011/012/013/015, MPLR-024/026/030–035, ZR14/15, MNR03/06.

**EARS:**
- **S9.1** IF a proof bound to effect X is presented to authorize decoded bytes Y, or decoding is incomplete, THEN the adapter SHALL refuse to sign.
- **S9.2** WHEN concurrent delegated requests share a budget, the aggregate of spent, pending and reserved SHALL NOT exceed the cap. Requests of 6 and 4 against a cap of 10 SHALL both reserve; 6 and 5 SHALL have at least one rejected or deferred, across retry and failover.
- **S9.3** WHEN the federation epoch changes, duties and consumed authority SHALL be preserved.
- **S9.4** IF finalized payment or a TEE-local result is presented as delivery, THEN the delivery duty SHALL remain unresolved.
- **S9.5** WHERE ZK, MPC, TEE or bare-threshold trust is used, reports SHALL state it as a named trust boundary.

### S10: U6 composable libraries and full conformance (M11 full, M15, MC07, SP07/08/11)

- **Retained IDs:** U6, M11, M15, MC07, SP07, SP08, SP11, UNI-014/020/023, MPLR-025/026.
- **Retained denominators:**
  - ACTUS: 277 fixtures, 18 executable types, 32 dispositions, DS01–07.
  - DeFi: 72 rows, DA01–24, supplements, 3 held-outs, 8 intent cases, 8 regression classes.

**EARS:**
- **S10.1** WHEN a price is converted, exact orientation conversion SHALL precede directed rounding. IF a port reciprocates an already-rounded value, reuses an opposite-orientation type name, or cites another implementation's certificate, THEN qualification SHALL reject.
- **S10.2** WHEN netting is applied, gross debits, fees and liabilities SHALL remain individually accounted.
- **S10.3** IF an oracle observation lacks the bound identity, freshness or trust assumption, THEN the dependent liquidation or clearing step SHALL refuse.
- **S10.4** WHEN FeePool multi-payer custody is claimed, every debit SHALL carry its payer's authorization. Mapped-role output SHALL NOT be cited as custody.

### S11: U7 independent developer release (MC08, SP12, G01–G24, M2, M5, M10)

- **Retained IDs:** U7, M2, M5, M10, MC08, SP12, G01–G24, UNI-001/016/024.

**EARS:**
- **S11.1** WHEN a clean external installation without project metadata authors a non-fixture program and submits it through the developer's own wallet, the pipeline SHALL decide validity without reviewer, registry or service membership.
- **S11.2** WHEN two builders reproduce a release, the record SHALL compare both artifact bytes and the derived proof statement (relation and VK). A matching build hash alone SHALL NOT count.
- **S11.3** IF an encoded transaction exceeds 64 KB, or any keygen API (including library `generate_call_keys`) receives k > 19 or unpinned parameters, THEN the toolchain SHALL refuse.
- **S11.4** The release scope statement SHALL list every C/H/P premise from S1.5 that remains open.

---

## 3. QA strategy and fixture/requirement coverage

### 3.1 Principles
1. **Independent oracles.** The oracle author is never the implementer. Expected effects and work are frozen before the run and must not import the evaluator or compiler under test (UNI-020).
2. **Model ≠ implementation ≠ native ≠ target.** A Quint or K pass is [F]. It counts toward a Rust or circuit claim only through an explicit field adapter plus implementation replay of any counterexample.
3. **Adversarial coverage in every layer.** All existing Sherlock model counts are Normal-only. Each sprint adds hostile model cases alongside the normal ones.

### 3.2 Positive-control rule for rejection tests (mandatory)
A rejection counts as semantic evidence only when all three hold:
- (a) the unmutated control is accepted by the same pipeline, build and keys in the same evidence record;
- (b) exactly one declared field or witness value differs;
- (c) the rejection names the intended check, or for native tests, the envelope is well-formed (proof decodes, PI length is correct) and the verifier itself rejects.

**Witness tampering versus statement mutation.** A hostile witness produced by a prover that skips a constraint is different from an unchanged proof with a mutated statement. Both are required where native work is admitted.

### 3.3 Per-field mutation matrix (applies to S2, S5–S8)

| Field | Mutations |
|---|---|
| Authority | Key, s + 1, R off-curve or torsion |
| Domain | Network, Anchor |
| Asset | Identity, `token_hex` |
| Gross | Cap + 1 |
| Fee | Cap + 1; recipient |
| Net | Floor − 1 |
| Liability | Cap − 1; uncoupled discharge |
| Time | Out-of-window TTL |
| Nonce | Reuse |
| Predecessor | Fabricated |
| Custody | Third payer |
| Recovery | Unauthorized refund |
| Program image | Digest swap |
| Fuel | Under-charge |
| Arena | Stale read |

### 3.4 Coverage map

| Sprint | Requirements | Fixtures and denominators |
|---|---|---|
| S1 | UNI-016/018/019/021, M6, M10 | CLI report suites; profile matrix |
| S2 | M7–M9, SEC-01/03/04/09/11/17 | WS-B B2/B4; WS-C C1/C3 |
| S3 | U1, M5, ZR06/12/13 | Quint 18 + new hostile; K33 |
| S4 | ZR01/03/05, MNR04/05 | Factorial, tree fold, two-asset projection |
| S5 | U2, MC01/02/04/05, SP05 | P-A, P-B, N-1, N-2; loan/swap requirements retained |
| S6 | M12, UNI-009 | Replay and cross-domain set |
| S7 | U3, M4/M11/M13/M14, L1–L8 | Discriminator; 10 roadmap hostile classes |
| S8 | MC03/06, 5 operators | Composition set |
| S9 | UNI-011–013 | 6 + 4 / 6 + 5 budgets; X/Y adapter |
| S10 | ACTUS, DeFi and all retained denominators | Frozen manifests validated before scoring |
| S11 | G01–G24, SP12 | Clean-install journey |

---

## 4. Direct factory workflow, parallel work and bottlenecks

**What ships.** Every sprint ends with merged, working code on `main` (or a precisely labeled refusal) plus its tests and a short evidence note in the existing roadmap and evidence files. There is no scheduler, campaign tool or new orchestration software.

**Roles, kept minimal:**
- **Implementer.** Any model for nonsecurity code.
- **Independent oracle and QA author.** A different author, who freezes expectations first.
- **Opus security review.** Applies only to diffs touching security surfaces: statement or PI layout, signature, nonce, domain, custody, liability caps, phase or recovery, keygen parameters, verifier, Sherlock statement binding, and adapter signing. Gemini and Astra may implement or QA these surfaces but do not issue security judgments.
- **Integrator.** One integrator owns shared IR, ABI and statement interfaces to avoid merge conflicts.

**Parallel lanes:**
- (a) S1 tooling and reporting
- (b) S2 host checks
- (c) S3/S4 Sherlock source and models
- (d) oracle and fixture authoring for S5/S7/S10
- (e) S7 multi-party authority design
- (f) S10 library source

Builds and model runs stay serialized under the existing lock and limits.

**Integration cadence.** Merge each child capability as soon as its tests pass on `main`. No batching behind parent exits.

**Bottlenecks:**
1. Native authority is zero. All [N] and [T] work waits for separate admission.
2. Stagenet network ID, Anchor and tuple.
3. The multi-party authority design, which gates S7.
4. Throughput of independent oracle authors.
5. The single integrator on statement interfaces.
6. The serialized build and model lock.

---

## 5. Risks, disagreements, unknowns and next deliverables

### Risks
- **R-1.** Claim transfer across the three ZKIR paths (SEC-02, extended).
- **R-2.** The Oct 11 native result is cited for the ledger path.
- **R-3.** Normal-only model QA is read as soundness evidence.
- **R-4.** Placeholder domains leak into target-bound intents. S5.3 guards against this.
- **R-5.** A single-signer intent cannot express U3, M14 or M15.
- **R-6.** Symbolic arena capacity and the absence of canonical checkpoint serialization leave interpreter cut/resume unsound.

### Disagreements, preserved
- **D-1, phase-2 hold.** I treat the regex fix as an ordinary defect. Whether phase 2 must wait on full trace adjudication is your decision, since you ruled out receipt-review bureaucracy while keeping genuine restrictions. I neither create nor remove the hold.
- **D-2, critical path.** I put the existing ledger signed-v2 path on the U2/stagenet critical path and Sherlock in parallel. If the stagenet requirement means Sherlock must be the stagenet path, S4 becomes critical.
- **D-3, stale roadmap text.** C-1, C-4 and C-5 need roadmap edits.

### Unknowns
- **U-1.** Stagenet and Fenrir identities and tuples.
- **U-2.** The SEC-17 on-curve premise; G4 modulus primality and field realization.
- **U-3.** PR95–101 contents and the location of `compile_reported`.
- **U-4.** Meaning of the "C4" label and the bodies of OSR-1014, ZR and MNR.
- **U-5.** The recursion-route mechanism.
- **U-6.** Whether the failed and ignored test counts were actually zero.

### Next three concrete deliverables
1. **Security enforcement and premise map at `9bcbca52`.**
   - Re-anchor SEC-01–21.
   - Map field × {backend, ledger signed-v2, Sherlock} × C/H/P.
   - Record the `compile_reported` and frontend provenance labeling (S1.1/S1.2) and the S1.6 numeric/time rulings needed from you.
2. **P-A/P-B source with an independently frozen oracle.**
   - Include N-1, N-2 and the full single-field mutation matrix with positive controls.
   - Compile source-static, ready for stagenet once native admission and stagenet facts exist.
3. **Safe-static security unit controls and fixes.**
   - B2–B6 as pure-Rust tests with inspected filters.
   - Fixes for SEC-07, SEC-09 and SEC-19, plus SEC-01 once ruled.
   - Hostile Quint and K cases for S3.1–S3.4 to accompany the 18 normal tests.
