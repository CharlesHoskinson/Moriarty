# Opus brainstormer 2: native compiler, cryptography and ledger boundary audit design

**Status:** advisory proposal only. It is not an audit, an approval, native qualification or milestone acceptance. Native authority was zero. I used Read/Grep/Glob on frozen tree `cb8280ef` and the runtime maps, and I executed nothing. Luna maps served as pointers. I checked each claim below against source and cite it by path:line.

---

## 1. Approach

**Recommended: a predicate ledger keyed to obligations, walked stage by stage along the real production path.**
- Break every retained obligation (U0–U7, M1–M17, MC01–08, SP01–12, G01–24) into child predicates small enough to test.
- Walk two newly authored programs through every stage from source to recovery. At each stage, record the exact consumer function, the evidence class and the authority needed to go further.
- Status comes only from evidence attached to a predicate. It never comes from test counts, hashes or votes.
- **Tradeoff:** this costs more up front. It is the only design that gives honest denominators and keeps "observed in source" separate from "observed at runtime".

**Alternative A: a threat-model-first adversarial audit.** Enumerate attacker capabilities (forged witness, cross-domain replay, key substitution, fee siphon, mixed-payer custody, time forgery) and try each against each stage.
- Strength: it finds deep defects quickly.
- Weakness: it does not produce obligation denominators, and obligations nobody threat-modeled get dropped silently.
- Best used as a workstream inside the recommended approach.

**Alternative B: a differential cross-stack audit.** Compile the same sources through both native stacks and the reference evaluators (`mori-backend` transition relation and `mori-ledger` contract-call relation; see gap R5), then compare outputs.
- Strength: it is cheap and mechanical, and it finds correspondence drift.
- Weakness: if both stacks share a wrong premise, it confirms the error. It also says nothing about authority, settlement or target facts.

---

## 2. Required outputs

1. **`audit/predicates.jsonl`**: one row per child predicate. Fields:
   - `id`, `obligation[]` (U/M/MC/SP/G IDs), `child_capability`, `predicate` (a falsifiable sentence)
   - `stage` (S0–S13 below)
   - `profile` (`beta` | `core5` | `source6` | `backend-transition` | `ledger-call` | `ledger-static` | `ledger-mapped-static`)
   - `evidence_class` (`source-inspected` | `specified-only` | `historical-run@<tree>` | `current-ordinary-run` | `current-native-run` | `external-target-observed`)
   - `status` (`observed` | `partial` | `absent` | `unknown` | `refused-by-design` | `blocked-native` | `blocked-external` | `contradicted`)
   - `refs[]` as `{path, line, blob_oid}`
   - `oracle {kind, author, frozen_before_run, independent_of_impl}`
   - `hostile_controls[]`, `required_authority`, `blocker`, `not_claimed[]`, `dissent[]`
2. **`audit/denominators.json`**: per obligation, counts by status. It must report `unknown` and `blocked-*` explicitly. It must contain no percentages and no weighted scores, which replaces the earlier 60–70% / 30–40% subjective estimates.
3. **`audit/chain-<program>.json`**: for each program and stage, the consumer symbol, input and output identities, whether the stage was reached, where the chain stopped, and why.
4. **`audit/identity-matrix.json`**: every identity that appears in a manifest or proof context (source, program, ABI, IR, compiler hash, source closure, toolchain, SRS, PK/VK, network, Anchor, address, asset, policy). For each one: which artifacts bind it, whether a circuit constraint or only a host check binds it, and whether its value is real or a sentinel.
5. **`audit/ROADMAP-AUDIT-REPORT.md`** (human-readable):
   - findings by risk
   - status changes for each obligation, with evidence
   - contradictions needing a user decision
   - the proposed roadmap diff
   - a "claims not supported" section

Candidate row:

```json
{"id":"M7.b","obligation":["M7","MC05","U2"],"child_capability":"real Fenrir network ID and Anchor bound in signed context",
 "predicate":"Installed signed deployment binds a non-sentinel Anchor and target network ID that the target ledger checks",
 "stage":"S9","profile":"ledger-call","evidence_class":"source-inspected","status":"blocked-external",
 "refs":[{"path":"crates/mori-ledger/src/profile.rs","line":236},{"path":"crates/mori-ledger/src/signed/context.rs","line":164}],
 "oracle":{"kind":"external-target-fact","author":null,"frozen_before_run":false,"independent_of_impl":true},
 "hostile_controls":["cross-network replay","cross-Anchor replay"],"required_authority":"external-target",
 "blocker":"Fenrir has no versioning, network ID or Anchor facts; installed profile requires anchor == \"local-unqualified\"",
 "not_claimed":["domain separation on any deployed network"],"dissent":[]}
```

---

## 3. Testable end-to-end chain

**Programs.** Both must be authored fresh, not taken from fixtures or FeePool. A non-author must freeze hand-derived expected effects, refusals and duties before anything runs.

- **P-A, the positive slice:** a two-asset owner payment.
  - Owner pays a beneficiary in asset X, with a protocol fee in X.
  - A residual duty in asset Y is left open.
  - One public payer, signed-v2 consuming authorization, guaranteed phase only, fee and net caps in a signed `limits` row.
- **P-B, the structural contrast:**
  - Two debit roles, unsigned-mapped-roles.
  - A branch, checked multiply/divide with floor/ceil rounding toward the receipt, and a duty created without a transfer.
  - Expected result: it compiles on the mapped-static path, and the installed/native path refuses it at `one-public-payer` (`crates/mori-ledger/src/layout.rs:208-212`).
  - A correct refusal at that point counts as evidence. A refusal caused by a malformed envelope does not (`ROADMAP.md:64`).
- **P-A′:** the P-A source compiled through `mori-backend` with a signed Target (`crates/mori/src/commands/verify_intent.rs:140`). This is the differential leg.

**Stages.** Each stage has an oracle, and auditability is split into **now** (static, or ordinary local work with zero native authority) and **blocked** (needs native or external authority).

| Stage | Consumer | Now | Blocked |
|---|---|---|---|
| S0 author | `.mori` text | Freeze the oracle | — |
| S1 parse/route | `mori check` Beta-vs-Core router (Luna frontend; verify `commands/check.rs`) | Which profile accepts P-A/P-B | — |
| S2 Core/typing | `mori_core::parse`, `layout::validate_static` / `validate_source_static_roles` (`static_compile.rs:95-105`) | Refusal codes, limits | — |
| S3 intent | `OwnerPayload` (`signed/context.rs:227-266`), context hash (`:151-186`) | Field coverage against the U0 enforcement map | — |
| S4 ZKIR | `relation::compile` (`relation/mod.rs:86-148`), `compile_signed` (`signed/context.rs:187`), static bundle (`static_compile.rs:274`) | Byte determinism, compile-fail promotion barriers (`:252-273`) | — |
| S5 ordinary check | `relation::check` (`relation/mod.rs:149-188`) | Hostile-witness mutation on preimages | — |
| S6 keygen | `generate_call_keys` (`proving.rs:319-345`) | Source review of degree and SRS checks | SRS provenance, real keygen |
| S7 prove | `prove_prepared_call` (`proving.rs:697-721`) | — | Native |
| S8 verify | `verify_proven_call`, `ProofVerificationMode::Real` (`proving.rs:658-682`) | — | Native; independent re-keygen verifier |
| S9 deploy | none observed for Fenrir; installed profile is local-only (`profile.rs:149-150, 232-243`) | — | External target tuple |
| S10 call | Intent submission | — | External |
| S11 effects | Readback (`proving.rs:603-625`; `prepare.rs:805-826`) | Local readback against the oracle | Ledger-applied effects, fees, DUST |
| S12 history | Nonce Impact (`relation/compiler.rs:708-745`; `signed/nonce.rs:121-139`) | Supplied-state replay refusal | On-chain unique consumption, reorg, race |
| S13 recovery | Sponsor and recovery accounting (`mori-core/src/source_sponsor.rs:294`, per Luna) | Local accounting | Unknown finality on a real chain |

---

## 4. Highest-risk gaps found in source

**Observed** means I saw it in source. It says nothing about runtime success.

- **R1. In-circuit time is chosen by the prover (observed).**
  - Signature-binding constraints check `not_before ≤ at ≤ not_after` (`signed/constraints.rs:137-138`).
  - `at` is witness input `ranged[7]` (`signed/preparation.rs:312`). It is not part of the signed message preimage (`constraints.rs:79-104`).
  - The real expiry therefore depends on the native intent `ttl_hex`, which the owner signs through the payload (`signed/context.rs:258`).
  - **Unknown:** whether `ttl ≤ not_after` is enforced anywhere.
  - **Inferred:** `not_before` cannot be enforced against ledger time.
- **R2. Anchor and network are placeholders (observed).**
  - The installed profile requires `anchor == "local-unqualified"` (`profile.rs:236`; `signed/deployment.rs:45`). The network may be any non-empty string (`profile.rs:238`).
  - Both values are hashed into the signing context and the nonce domain (`signed/context.rs:164-165`; `signed/nonce.rs:26-27`). The domain-separation structure therefore exists, but nothing is bound to a real domain.
  - M7 cannot advance until Fenrir supplies these facts. Do not invent them.
- **R3. The k ≤ 19 limit is not a product constant (observed).**
  - `NativeWorkLimits::validate` rejects only `max_degree == 0` (`profile.rs:268`). The value 19 appears only in fixtures and tests (`prepare.rs:1022`, `call_artifacts.rs:553`).
  - The search found no 64 KB transaction ceiling in `mori-ledger`. The 64 KiB constants it did find cover the backend envelope (`mori-backend/src/authorization.rs:12`) and source size (`mori-lexer/src/lib.rs:29`), not transactions. Status: absent from search, so **unknown**.
  - Effect on M5: the caller sets these limits and nobody has measured them.
- **R4. JubJub Schnorr splice correspondence is unknown.**
  - The in-circuit verifier exists (`constraints.rs:105-136`). The public key is a compiled constant (`:75-77`), so key rotation and account authority are absent (`schnorr.rs:88`).
  - The non-identity checks test only `x ≠ 0` (`constraints.rs:45-49`). Whether a ZKIR `JubjubPoint` input enforces prime-order subgroup membership is **unknown**.
  - The host computes the challenge with `Scalar::from_bytes_wide` (`schnorr.rs:95-97`). The circuit uses `JubjubScalarFromNative` (`constraints.rs:112`). Whether those reductions are equivalent is **unknown**. A mismatch would break either completeness or soundness, which is exactly the M8 question.
- **R5. Two native stacks, two Schnorr copies, two compiler-identity notions (observed).**
  - The stacks are `mori-backend` (transition relation, Target, `prove`/`verify`/`verify-intent`; `mori-backend/src/lib.rs:1-5`) and `mori-ledger` (contract call).
  - The Schnorr copies are `mori-backend/src/schnorr.rs` and `mori-ledger/src/signed/schnorr.rs`.
  - The ordinary manifest's `compiler_hash` covers 5 files (`relation/mod.rs:117-125`). Signed and mapped-role manifests use the much wider `source_closure()` (`proving.rs:150-204`). Neither is a certificate of the build or rustc version.
  - The Oct 11 proof ran the `moriarty/1` transition circuit at tree `ee963f61` (`deliverables/native-owner-intent-2026-10-11/RESULT.md:11,20`). It does not carry over to `cb8280ef` or to the deploy/call path.
- **R6. Verification is not independent (observed).**
  - `prove_prepared_call` verifies with the public key from the same derived material (`proving.rs:703-716`).
  - `PublicDerivedCallKey` cannot be built from imported bytes (`:297`). A third-party verifier therefore has to repeat keygen, which pushes SRS provenance back onto the caller (`:316-318`).
- **R7. Replay and liveness.**
  - The nonce read and write happen in-circuit through an Impact on the authority map (`relation/compiler.rs:708-745`) (observed).
  - On-chain uniqueness relies on the ledger applying that transcript (inferred, never observed).
  - The authority map is append-only with capacity 12,153 (`signed/nonce.rs:8,131`). Once full, signed authority stops permanently. No recovery path for this was observed.
- **R8. Fee and net semantics (inferred question).**
  - Fees paid by the owner are subtracted from the beneficiary's net (`constraints.rs:153-177`).
  - `Sub` results are range-checked as UInt (`relation/arithmetic.rs:170-177`), so a negative beneficiary net makes the circuit unsatisfiable.
  - When owner ≠ beneficiary, U0 needs a semantic ruling here. Not proven either way.
- **R9. DUST and phases (observed absence).**
  - `dust_actions: None` (`prepare.rs:587`), `phase_policy` is `guaranteed-only` (`profile.rs:235`), and fees and settlement are recorded as `not-performed` (`prepare.rs:823-825`).
  - M4 has no native path in this tree.
- **R10. Asset identity (partial).**
  - Core `AssetId` carries network, issuer, reference and scale (`mori-core/src/ir.rs:106-111`).
  - The native `token_hex` is a caller-supplied binding (`profile.rs:70-73`). The circuit hashes the Core identity as constant limbs (`constraints.rs:94`).
  - No observed check derives `token_hex` from that identity, so M9 is unknown past S4.
- **R11. Custody.**
  - Mixed-payer installs are refused (`layout.rs:212`).
  - The mapped-role path describes itself as unauthenticated (`relation/mod.rs:215, 297-304`; `static_compile.rs:21-22`).
  - PR49 adds no native custody.
- **R12. Target contradiction (observed).**
  - AGENTS.md says Preview is the sole public execution target (`AGENTS.md:100`).
  - M2/M3 require a qualified Fenrir devnet (`docs/superpowers/specs/2026-10-10-fenrir-integration-design.md:77-78`).
  - The user needs to decide this before any S9 work.

---

## 5. Keeping every obligation

- No obligation is merged, weakened or closed.
- Each U, M, MC, SP and G ID gets at least one predicate row. The audit fails if an ID ends up with zero rows.
- The original MC/SP/G denominators carry through unchanged. Historical statuses stay attached to their trees: G01–G24 stay `unassessed` until SP12.3 closes them.

Child gates for my specialty (current status, read statically):

| Child gate | Status |
|---|---|
| M2.a Workspace SDK and package pin (`profile.rs:7-41`) | observed |
| M2.b Deployed compiler/ZKIR/key/verifier/ledger tuple (product contract `:21`) | blocked-external |
| M2.c Compatibility tests against the target | absent |
| M3.a Local Real verify API | partial |
| M3.b Independent verifier | absent (R6) |
| M3.c Deploy/call and complete effects on target | blocked-external |
| M4.a Own DUST | absent |
| M4.b Sponsored allowance | local partial |
| M4.c Concurrency and timeout | blocked |
| M5.a Enforced k ceiling | absent (R3) |
| M5.b Measured transaction bytes | unknown / blocked-native |
| M5.c Replay certificate | absent |
| M7.a Domain fields hashed into the signed context | observed |
| M7.b Real values | blocked-external |
| M7.c Rejection of cross-domain replay on the target | blocked |
| M8.a In-circuit splice present | observed |
| M8.b Host/circuit/reference correspondence | unknown |
| M8.c Subgroup and cofactor | unknown |
| M8.d Reconcile the two copies | unknown |
| M8.e Independent audit | absent |
| M9.a Core identity | observed |
| M9.b Token derivation | unknown |
| M9.c Identity in ledger effects | blocked |
| M10.a `simulate`/`test`/`verify-intent` commands exist | observed |
| M10.b Behaviour correct | unknown |

M10 note: `lsp` and `mcp` are `todo!` (`commands/lsp.rs:9`, `commands/mcp.rs:9`). The old "all stubs" wording is out of date.

U1, U2 and U4 keep every exit listed in `ROADMAP.md:46-49`.

---

## 6. Workstreams

| WS | Content | Depends on | Authority |
|---|---|---|---|
| W1 | Extract the obligation register from ROADMAP, traceability and the Fenrir rows | — | static |
| W2 | Stage audit of the `ledger-call` path, S1–S5 | — | static |
| W3 | Crypto audit: R1, R4, R7; both Schnorr copies; context-hash field coverage | — | static |
| W4 | Identity matrix: R2, R5, R10 | — | static |
| W5 | Custody, fee/net, DUST, phase: R8, R9, R11 | — | static |
| W6 | Author P-A/P-B/P-A′; a non-author freezes the oracle | W1 | static |
| W7 | Ordinary local runs: compile, static export, `relation::check` hostile mutations, differential P-A against P-A′ | W2, W6 | root authorization; native zero |
| W8 | Keygen, prove and independent verify with SRS provenance | W7, native admission | **blocked** |
| W9 | Target tuple, deploy/call, DUST, replay, finality | Fenrir facts, R12 decision | **blocked** |
| W10 | Synthesis, denominators, roadmap diff | all available | static |

- W1–W6 can run in parallel because their scopes do not overlap.
- W7 serializes after the oracle freeze.

**Done means:**
- Every obligation ID has rows.
- Every row has a status, and every `observed` row has a citation that a second, non-author auditor has reproduced. All high-risk rows get reproduced, not a sample.
- Every `blocked-*` row names its exact missing fact or authority.
- Both programs have chain files that stop at a named stage for a named reason.

**Achievable now:** all static rows, the oracle freeze, and the R-items as source findings.

**Blocked:**
- S6–S10 on the current tree
- any statement that a proof is valid
- on-chain replay and finality

---

## 7. Roadmap changes

**Now (minimal and textual):**
1. Add a dated "What exists" entry for PR45–49 that is explicitly source/local only. Wording: finite FeePool component; mapped-role static path marked unauthenticated; no native custody, no M11.
2. Add an M1–M17 table to `ROADMAP.md` that references the Fenrir design rows. Mark every row open and keep local and external-target predicates separate.
3. Record R12 as an open contradiction for the user to decide, along with these unknowns: Fenrir network ID, Anchor and versioning.
4. Replace the subjective percentages with "predicate denominators pending audit".
5. State that two native stacks exist, and that the Oct 11 evidence belongs to `backend-transition` at `ee963f61`.

**Deferred until there is evidence:**
- any change to M2/M3/M4/M5/M7/M8/M9 status
- a U2 claim
- choosing the canonical stack
- making k ≤ 19 and the transaction ceiling product constants (a design decision first, then code)
- the owner ≠ beneficiary fee ruling (U0)

---

## 8. Dissent, open questions and claims this review cannot support

**Dissent:**
- Choose one canonical native stack, or require a differential certificate between the two, before U2. Two silently diverging stacks and two Schnorr copies are an M8 risk.
- A fixed-key signer policy should not count toward M7 or M13 authority.
- PR49's 162 cases count only as a finite local component, never as fractional M11 credit.

**Open questions:**
- Does ZKIRv3 `JubjubPoint` enforce subgroup membership?
- Does the ledger reject a second consumption of the nonce map under concurrent submission?
- What is the intended semantic of `not_before`?
- Is Preview or Fenrir the M3 target?

**This review cannot support:**
- that any proof verifies at `cb8280ef`
- compiler soundness or completeness
- that any key or SRS is genuine
- any settlement, DUST or replay safety on a ledger
- any percentage of completion
- any milestone acceptance

Test counts, hashes, Muse or consensus approvals, and archived drafts do not substitute for any of these.
