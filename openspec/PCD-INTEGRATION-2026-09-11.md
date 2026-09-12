# PCD integration amendment

Status: S2, specified-only planning revision, dated 2026-09-11. It grants no admission, dispatch, resource or acceptance. The machine twin is [pcd-integration.json](sprints/pcd-integration.json), checked by `openspec/sprints/verify.py`.

## Decision adopted

Moriarty builds a ledger-anchored certified state machine with bounded native certificates. The [decision report](../deliverables/pcd-midnight-native-2026-09-11/REPORT.md) gives the analysis, the [PCD roadmap](PCD-ROADMAP-2026-09-11.md) gives the stages, and the [PCD change package](changes/pcd-ledger-anchored-acceptance/README.md) holds the requirements.

- **Contract proofs cannot be inner proofs.** They use a Blake2b transcript, and `verify_proof` accepts only Poseidon zk-stdlib proofs.
- **The ledger already enforces history.** It verifies each call against the operation key in contract state and rejects a changed head read at application. With immutable keys and constrained genesis, every live head descends from genesis through accepted steps.
- **Recursion serves bounded certificates only.** Off-ledger segments, attestations and imports use `ledger-10` `verify_proof`, with pairings checked by the ledger.

The hard gate is unchanged: verification-enabled mandatory acceptance on Midnight Preview.

## Planning assumption

The user directed that Midnight recursion (pull request 738 on `ledger-10`) be assumed to reach production within a few months. Certificates therefore stay release scope, and `release` keeps `f2`. If recursion slips, certificate stages record `blocked`; the core and the Preview gate stand.

## Stage graph

Stage identifiers, owners and statuses are unchanged. `f3` is the only stage whose prerequisites changed: it previously required `atomic-accept`, `i2`, `f2` and `f1`.

| Stage | Owners | Requires | Purpose |
|---|---|---|---|
| `f0` | MC04, MC03 | — | PCD Stage 0: pinned verification seam (operation key in ContractState.operations checked by ledger well_formed), toolchain manifest per network generation, head-discipline checker and deploy-audit designs, and a certificate-route go/no-go conditional on ledger-10 |
| `native-path-freeze` | MC03, MC04 | f0 | Reviewed path and command-interface ownership for step-relation compiler output, head-discipline checker, deploy audit and certificate-relation roots, with source pins, before source authorship; actual commands freeze after F0a implementation |
| `f0a` | MC04, MC03 | f0, native-path-freeze | Certificate-relation authorship against pinned pull request 738 / ledger-10 sources: guard-constant lint, Collapsed decider constraints on vk_repr and state, frozen hashes and current source reviews |
| `f1-fixtures` | MC03, MC04 | f0a | Separately admitted bounded generation of independent non-loan certificate fixtures (Poseidon zk-stdlib inner proofs and a verifier-test accumulator) |
| `f1` | MC04, MC03 | f0a, f1-fixtures | E3: native certificate accepted on a ledger-10 devnet with its accumulator pairing; free or mismatched guard, substituted vk_repr, unbound inner instance, foreign-key and tampered inner proofs all reject; fee against measured validation work recorded |
| `f2` | MC03 | atomic-accept, rp01-mc03, f1, native-path-freeze | E5: off-ledger segment certificate over the Moriarty step at 1, 10 and 100 steps, retained and verified in a fresh process, and imported through the certificate entry point |
| `f3` | MC04 | atomic-accept, i2, f0, native-path-freeze | PCD Stages 1-3 for the atomic loan/swap profile: E2 fused step relation fit, E1 head read-then-write linearity on Preview, immutable-authority deploy audit and verification-enabled Preview acceptance of one step each |
| `mandatory` | MC05 | f3, rp01-full, successor-semantics, actus-semantics, defi-semantics | Versioned mandatory-claim extension under the fused step relation: claim discharge map, constrained genesis and termination, deploy-audited property certificate, block-time observation freshness and forward-declared migration (E4); requalify exactly changed atomic domains |
| `composition` | MC06 | mandatory, rp01-full | Versioned composition extension: ledger-atomic split/join, cross-contract Release/JoinFrom/Reclaim on ledger 9, recipient-keyed handoff and operator rules under head discipline, with corresponding requalification |
| `release` | MC08 | atomic-accept, i2, f2, f3, mandatory, composition, finance | Final release evidence under accepted versioned profiles |

- `mandatory` no longer depends, even transitively, on `f0a`, `f1-fixtures`, `f1` or `f2`. `release` still requires `f2`.
- `f2` keeps `f1`: E5's ledger import uses the certificate entry point whose soundness E3 establishes.
- `f2` keeps `rp01-mc03`, reinterpreted below.

## Record changes

- Package dependencies: MC04 now depends on MC01 and MC02; MC05 depends on MC04.
- MC03 has no planned commands; the `experiments/moriarty-native-ivc-r3/successor` root is retired.
- MC04 status is `specified-only`: the core seam is declared, pending RP02 review, E1 and E2.
- RP02 stays `blocked`, with the certificate-route conditions as its reason.
- Feasibility stage text now describes the Stage 0 seam, E3, E5 and the ledger-anchored core.
- SP09 completion no longer requires SP06. SP12 completion requires SP06 and SP11.

## PCD stages and experiments

| Id | Title | Stage | Tasks |
|---|---|---|---|
| PCD-S0 | Pin the verifier boundary | `f0` | SP01.4 |
| PCD-S0-paths | Path ownership before authorship | `native-path-freeze` | SP01.5 |
| PCD-S1 | One native transition proof | `f3` | SP09.1 |
| PCD-S2 | One authenticated predecessor extension | `f3` | SP09.1 |
| PCD-S3 | Ledger-enforced acceptance on Preview | `f3` | SP09.1 |
| PCD-S4 | Sequential history and termination | `mandatory` | SP09.2, SP09.5 |
| PCD-S5 | Cross-party successor | `composition` | SP10.2 |
| PCD-S6 | Branch, join, release and reclaim | `composition` | SP10.3 |
| PCD-S6-migration | Forward-declared migration | `mandatory` | SP09.3 |
| PCD-S7-authorship | Certificate-relation authorship | `f0a` | SP04.1 |
| PCD-S7-fixtures | Independent certificate fixtures | `f1-fixtures` | SP04.2 |
| PCD-S7-certificate | Native certificate on ledger-10 | `f1` | SP04.3 |
| PCD-S7-segment | Off-ledger segment certificate | `f2` | SP06.1, SP06.2, SP06.3 |
| PCD-S8 | Performance and profile freeze | `finance` | SP11.1 |
| E1 | Head read-then-write linearity on Preview | `f3` | SP09.1 |
| E2 | Fused step relation fit | `f3` | SP09.1 |
| E3 | Native certificate with negative controls on ledger-10 | `f1` | SP04.3 |
| E4-migration | Immutable keys and forward-declared migration | `mandatory` | SP09.3 |
| E4-reclaim | Cross-contract release and reclaim | `composition` | SP10.3 |
| E5 | Off-ledger segment certificate at 1, 10 and 100 steps | `f2` | SP06.2, SP06.3 |

## SP01 reinterpretation

The SP01 sprint document is hash-pinned by the loan design binding and consumed by in-flight work, so it is not edited. Its tasks are read as follows.

- **SP01.4 (`f0`).** PCD Stage 0: seam, toolchain manifest per network generation, head-discipline checker and deploy-audit designs, and a certificate-route go/no-go.
- **SP01.5 (`native-path-freeze`).** Path ownership for the step-relation compiler output, head-discipline checker, deploy audit and certificate-relation roots. It replaces the single MC03 successor namespace and the MC04 outer-port ownership.
- **SP01.7 (`rp01-mc03`).** The fixed native-statement subset becomes the reviewed statement subset of the Moriarty step that segment certificates fold.

## Reconciliation rows

The [September 7 reconciliation](REPORT-RECONCILIATION-2026-09-07.md) is hash-pinned and not edited. These rows are superseded or amended.

| Reconciliation row | Disposition | Replacement |
|---|---|---|
| F0: finalizer feasibility and ledger-family decision | Amended | Stage 0 seam and toolchain manifest per generation: ledger 8 core, ledger 9 cross-contract, `ledger-10` certificates |
| F0a: MC04 final pairing port and MC03 export | Superseded | Certificate-relation authorship; no outer finalizer port |
| I2: parallel uncertified integration | Retained | Deployed Preview contracts still use a one-key committee |
| F1: P1 transcript, P2 export, P3 constrained pairing | Superseded; no-waiver discipline retained | E3 controls on a `ledger-10` devnet |
| F2: two-step native IVC proof | Superseded | E5 segment certificates |
| F3: native verification inside the ledger proof, then Preview acceptance | Amended | Stages 1–3: one fused proof verified by the ledger, then verification-enabled Preview acceptance |
| F4: private composition and broader finance | Amended | Stages 5–6 and finance |
| RP02 pre-MC03 gate: P1/P2/P3 before MC03 | Superseded; no relaxation of mandatory history | E1 and E2 decide the core; E3 and E5 decide certificates |
| Retained candidate `moriarty_loan_r3.rs` | History only | — |
| F2/F3 native IVC checks: `vk_repr`, decider, accumulators, transcript EOF, pairing | Amended | Apply to certificates; the ledger discharges the pairing |
| Terminal-limb and outer-binding controls | Amended | Binding input, communication commitment, transcript `Popeq` and effect controls |
| P1–P3 fixture recipes | Partially reused | Inputs to E3 and E5 fixtures |
| P1/P3 in the exact outer circuit stack with a constrained finalizer | Superseded | Pairings are never computed in-circuit |
| Route: constrained complete native finalizer | Superseded | Ledger-side pairing from pull request 738 |
| Route: ledger-9 V3 source alignment | Amended | Cross-contract calls only |
| Route: ledger-native accumulator interface at a newer pin | Selected | Pull request 738, conditional on `ledger-10` release |
| Route: deterministic-only or host verification boolean | Retained as forbidden | — |
| Interface-blocked disposition when all routes fail | Amended | The core is not interface-blocked; removing mandatory PCD still needs user direction |
| History compliance connects predecessor states and proofs | Amended | On-ledger history needs no predecessor proofs |
| SP01.5 native-path-freeze with one MC03 successor root | Superseded | See the SP01 reinterpretation |

## Supersessions

MC requirement headings and locked task lines do not change. Each affected scenario carries an in-place "Amended by" note.

- **Mechanism replaced.** The rejection the MC text requires still holds; the PCD requirement supplies how it is enforced.
- **Extended.** The MC rejection list gains cases.
- **Removed.** The MC text described the retired recursive route; each removal is a decision below.

| Package | Requirement | Scenario | Disposition | Note | Governing PCD requirements |
|---|---|---|---|---|---|
| MC03 | Reviewed restart | Checked smaller encoding | removed | The fixed 54-limb relation and its smaller-encoding retry are retired (decision PD7). | Off-ledger segment certificate |
| MC03 | Real recursion and independent verification | Valid episode | mechanism-replaced | The fixed two-step episode becomes E5 segment certificates at 1, 10 and 100 steps. | Bounded native certificates; Off-ledger segment certificate |
| MC03 | Real recursion and independent verification | Invalid proof context | extended | Rejections add free or mismatched guards, substituted vk_repr and unbound inner instances. | Bounded native certificates |
| MC03 | Complete backend decision before native dispatch | — | mechanism-replaced | The backend decision is the certificate route under the dependency tracker. | Recursion dependency tracking and stop |
| MC04 | Actual verifier compatibility | Missing interface | mechanism-replaced | The core is not interface-blocked; host assertions and mocked verification still cannot substitute. | Declared ledger verification seam |
| MC04 | Durable one-time consumption | Duplicate authority | mechanism-replaced | Unique consumption uses head read-then-write discipline and E1. | Head read-then-write discipline |
| MC04 | Early complete verifier feasibility | — | mechanism-replaced | Early feasibility is the Stage 0 seam for the core and E3 for certificates. | Declared ledger verification seam; Bounded native certificates |
| MC05 | Mandatory acceptance | Downgrade attack | mechanism-replaced | Claims are compiled into immutable operation keys checked by the deploy audit. | Immutable authority deployment audit; Claim discharge map; Forward-declared migration replaces in-place revocation |
| MC05 | Intent refinement and complete effects | Authorized route choice | mechanism-replaced | Route choice uses outcome mode with a program-digest allowlist and consumed nonce. | Intent digest v2 and program digest |
| MC05 | No circular or simulated evidence | Bound outcome claims | mechanism-replaced | The signature binds digest v2 fields; the head read replaces predecessor lists. | Intent digest v2 and program digest |
| MC05 | Authority covers liabilities and all protected effects | Revoked verifier or inactive specification | mechanism-replaced | A defective or compromised program is stopped by Pause and migration, not key revocation (decision PD2). | Forward-declared migration replaces in-place revocation |
| MC05 | Authority covers liabilities and all protected effects | Oversized verification input | mechanism-replaced | The registered budget is the Π_P bound set, checked before proving. | Intent digest v2 and program digest; Measured bounds frozen into the program digest |
| MC06 | Bounded history composition | Compatible composition | mechanism-replaced | On-ledger branches need no branch proofs; off-ledger branches join through certificates. | Ledger-atomic split and join; Cross-contract release with reclaim |
| MC06 | Composition operators and witness ownership | Report requirement omitted | mechanism-replaced | On-ledger successors need no predecessor proof verification; handoff carries recipient-encrypted openings. | Recipient-keyed successor handoff; Composition operators under head discipline |

## Locked task dispositions

Task lines stay verbatim. The closing sprint tasks in `package-task-map.json` now do the following.

| Package | Task | Disposition | Note | Governing PCD requirements |
|---|---|---|---|---|
| MC03 | 1.1 | removed | Retired with the fixed relation (decision PD7); SP06.1 reviews the segment relation encoding instead. | Off-ledger segment certificate |
| MC03 | 2.1 | mechanism-replaced | Preimage, arithmetic, context and malformed-genesis tests apply to the segment relation; limb-boundary tests are removed. | Constrained genesis and termination; Off-ledger segment certificate |
| MC03 | 2.2 | retained | Applies to the Moriarty step folded by the segment. | Off-ledger segment certificate |
| MC03 | 3.1 | retained | The separate verifier process applies to certificates. | Bounded native certificates; Off-ledger segment certificate |
| MC03 | 3.2 | retained | Fresh deserialization, key and SRS identity and accumulator obligations apply to certificates. | Bounded native certificates; Off-ledger segment certificate |
| MC03 | 4.2 | mechanism-replaced | E5 runs 1, 10 and 100 steps under a reviewed resource amendment; stop on the first failed predicate is retained. | Off-ledger segment certificate; Recursion dependency tracking and stop |
| MC03 | D.1 | mechanism-replaced | The intake pins the released pull request 738 / ledger-10 formats. | Recursion dependency tracking and stop |
| MC04 | 1.1 | retained | Becomes the Stage 0 toolchain manifest and seam. | Declared ledger verification seam |
| MC04 | 1.2 | retained | Becomes the Stage 1 positive probe with mutated controls. | Fused step relation per entry point |
| MC04 | 4.1 | mechanism-replaced | Conflict and restart tests run against head discipline and E1. | Head read-then-write discipline |
| MC04 | 4.2 | mechanism-replaced | At most one finalized success follows from head discipline, shown by E1. | Head read-then-write discipline |
| MC04 | P.1 | mechanism-replaced | Compact step circuits proved by the proof server and verified by a Rust harness; no MC03 proof input. | Fused step relation per entry point |
| MC04 | P.2 | mechanism-replaced | Compact step circuits proved by the proof server and verified by a Rust harness; no MC03 proof input. | Fused step relation per entry point |
| MC04 | P.3 | mechanism-replaced | Compact step circuits proved by the proof server and verified by a Rust harness; no MC03 proof input. | Fused step relation per entry point |
| MC04 | P.4 | mechanism-replaced | Compact step circuits proved by the proof server and verified by a Rust harness; no MC03 proof input. | Fused step relation per entry point |
| MC04 | P.5 | mechanism-replaced | Compact step circuits proved by the proof server and verified by a Rust harness; no MC03 proof input. | Fused step relation per entry point |
| MC04 | P.6 | mechanism-replaced | Compact step circuits proved by the proof server and verified by a Rust harness; no MC03 proof input. | Fused step relation per entry point |
| MC04 | P.7 | mechanism-replaced | Compact step circuits proved by the proof server and verified by a Rust harness; no MC03 proof input. | Fused step relation per entry point |
| MC04 | D.1 | mechanism-replaced | The intake outcome is the declared seam; the core needs no wrapper alternative. | Declared ledger verification seam |
| MC04 | D.2 | retained | Adds immutable authority, the deploy audit and netTag. | Immutable authority deployment audit; Forward-declared migration replaces in-place revocation |
| MC05 | 1.1 | mechanism-replaced | Test vectors and mutations for Π_P, intent digest v2 and genesis body v2; certificate keys are circuit constants. | Intent digest v2 and program digest |
| MC05 | 1.2 | mechanism-replaced | Rejections cover every digest v2 field. | Intent digest v2 and program digest |
| MC05 | 3.1 | mechanism-replaced | Absent-proof and intent-invalid tests run against the fused step relation; no MC03 artifact is an input. | Fused step relation per entry point; Claim discharge map |
| MC05 | 4.1 | retained | Real evidence only for supported profiles. | Claim discharge map |
| MC05 | D.1 | mechanism-replaced | Signed intents are verified in the circuit with JubJub Schnorr; E2 measures fit. | Fused step relation per entry point; Intent digest v2 and program digest |
| MC06 | 3.1 | retained | Duplicate, policy, fan-in and amplification controls apply to heads. | Ledger-atomic split and join |
| MC06 | 3.2 | mechanism-replaced | Ledger-atomic split and join; branch proofs only for off-ledger branches. | Ledger-atomic split and join; Cross-contract release with reclaim |
| MC06 | 4.1 | retained | Recovery scenarios add Reclaim. | Cross-contract release with reclaim |
| MC06 | D.1 | mechanism-replaced | Same-contract realization on ledger 8, cross-contract on ledger 9. | Ledger-atomic split and join; Cross-contract release with reclaim; Recipient-keyed successor handoff |

## Decision register

Decisions PD1–PD5 and PD7 carry defaults awaiting the user's confirmation. None weakens a gate. PD6 has no default.

| Id | Question | Default | Owner | Blocks | Status |
|---|---|---|---|---|---|
| PD1 | Intent signing modes | Exact-head and outcome modes; cross-instance route choice owned by SP08 with MC05 | MC05, SP08 | Stage 1 digest | default-pending-confirmation |
| PD2 | Revocation and upgrade | Immutable keys, declared or unanimous migration and optional principal-threshold Pause; an all-principal committee with update delay only as a Π_P-declared option for long-lived profiles after review | MC05, SP09.3 | Stage 3 deploy configuration; E4 | default-pending-confirmation |
| PD3 | Where the ContractProperty certificate is checked | The deploy audit acts as governed registration | MC05, SP09.2 | Stage 0 audit scope | default-pending-confirmation |
| PD4 | Cross-contract calls in the Core | Allowed for Release, JoinFrom, Migrate, ImportFrom and Reclaim from Compact 0.33 on ledger 9; supersedes wiki/moriarty-architecture.md:143 for these only; external calls stay excluded | MC05, MC06 | Stage 6 | default-pending-confirmation |
| PD5 | Shared-state interleaving, asynchronous messaging and Pending | Redesign under head discipline in SP10 before any campaign advertises them | MC06, SP10.1 | Stage 6 scope | default-pending-confirmation |
| PD6 | Certificate entry point k bound | — | Program resource authority, MC03 | f1, f2 | user-decision-required |
| PD7 | Fixed MC03 54-limb relation | Retire it; its k17 failure stays failed evidence; segment certificates replace it | MC03, SP06 | Stage 7 | default-pending-confirmation |
| PO01 | Contract layout: one contract per instance or several heads per contract | — | MC04 | Stage 2 | open |
| PO02 | Single step entry point to hide the action type | — | MC06, SP10 with SP02 | Stage 1 circuit shape | open |
| PO03 | Join-summary content per profile | — | MC06, SP10 with RP01 | Stage 6 | open |
| PO04 | Migration of requiredClaimRoot to the compiled claim set | — | MC01 with MC05 | Stage 1 | open |
| PO05 | Per-head lifetime details across split, join and migration | — | MC01 with MC06 | Stage 6 | open |
| PO06 | now as a block-time-bounded value and accrual rounding | — | SP01, SP07 with MC05 | Stage 1 | open |
| PO07 | Unshielded versus contract-owned shielded settlement | — | MC02, SP05 with MC06 | Stage 3 | open |
| PO08 | State commitment hash: SHA-256 persistentHash unless a reviewed decision moves to Poseidon | — | SP04 with MC04 | E2 | open |
| PO09 | Network DUST fee caps: wallet policy versus proof | — | SP08 with MC08, SP12 | Stage 1 | open |
| PO10 | netTag residual for replicated deployments and a network-id request to Midnight | — | MC04 | Stage 1 | open |
| PO11 | Toolchain per network and the ledger-9 fork date | — | MC02, SP05 with RP03 | Stage 6 | open |
| PO12 | Compact versus hand-written zk-stdlib step relation | — | MC04, SP09 with SP04 | E2 | open |
| PO13 | Canonical claim vocabulary and site wording | — | MC01, SP01 with MC08, SP12 | Stage 0 | open |
| PO14 | Handoff encryption scheme, key directory and recovery owner | — | MC06, SP10.2 | Stage 5 | open |
| PO15 | Fallible-section placement and PartialSuccess handling | — | MC04 | E1 | open |
| PO16 | ledger-10 certificate soundness: parameter serving, format pin, subgroup and trailing-byte checks, unpriced accumulators | — | MC03, SP04 | Stage 7 | open |
| PO17 | Exported history proof for off-ledger auditors | — | MC08, SP12 | Outside acceptance | open |
| PO18 | Signature griefing by intervening steps | — | SP08 with MC06 | Stage 5 | open |

## Midnight dependency tracker

| Item | Needed by | Status on 2026-09-11 | Class |
|---|---|---|---|
| Pull request 738 merged and ledger-10 released and deployed | f0a, f1, f2 | Open; merge conditionally agreed | M3 (Midnight) |
| Fee accounting for accumulator public inputs and pairings | f1 exit, f2 | Absent at the pull request head | M3 (Midnight) |
| Guard lint for VerifyProof and InnerProof | f0a | Absent; Moriarty lint covers it meanwhile | M2 |
| Collapsed decider checking vk_repr and state | f0a, f2 | Absent; the Moriarty outer relation covers it meanwhile | M2 or M3 |
| k >= 18 parameters served through the proof server and data provider | f1, f2 | Ceremony files published; serving path unverified | M1 or M2 |
| Compact verifyProof frontend | f0a ergonomics | Draft MIP only; ZKIR hand emission meanwhile | M2 |
| midnight-js and proof-server inner-proof plumbing; Poseidon proving for zk-stdlib relations | f1, f2 | Absent; Moriarty Rust prover meanwhile | M2 (M0 workaround) |
| Published IVC module with instance and accumulator serialization | f2 | Unpublished | M2 |
| Recipient-keyed private-state handoff | composition | Absent; Moriarty encryption meanwhile | M2 (M0 workaround) |
| Network id in the contract statement | Residual replay risk | Absent; netTag separates networks but not byte-identical replicated deployments | M3 (optional) |
| Maintenance-authority threshold validation | Defense in depth | Absent; the deploy audit covers it | M3 (optional) |
| Ledger 9 on Preview | mandatory (SP09.3), composition (SP10.3) | Hard fork in node 2.1.0-beta.1; Preview runs ledger 8 | M3 (Midnight) |
| Reviewed resource amendment for certificate k | f1, f2 | Not requested; campaigns capped at k <= 17 | Moriarty decision |

## Resource ceiling

Every campaign keeps k ≤ 17, 8 GiB of process-group memory and two CPU jobs. The reproduced pull request 738 outer circuits needed k = 18 with a 4,168 MiB peak (about 4.1 GiB) and k = 19 with an 8,004 MiB peak (about 7.8 GiB). Certificate campaigns therefore need a reviewed `resourceAmendments` entry that the user approves (decision PD6). No such amendment exists.

## Stop conditions

- **E1 fails.** Stop Stages 2–6 until a reviewed consumption-set design replaces head discipline.
- **E2 fails.** Revisit relation decomposition within k ≤ 17; acceptance requirements stay unchanged.
- **`ledger-10` changes the pull request 738 formats.** Pin the released formats and rerun E3.
- **Recursion slips or the resource amendment is refused.** Certificate stages record `blocked`. The core and the Preview gate stand, and no scope is dropped without new user direction.

## Design additions beyond the report

These rules are not in the decision report. They ride with this revision's review.

- **`Terminate`.** Requirement "Constrained genesis and termination" adds an entry point for the report's `Terminated` lifecycle value, with a disposition for every residual obligation.
- **`Pause`.** Requirement "Forward-declared migration replaces in-place revocation" adds an optional principal-threshold pause that halts new steps without changing keys.
- **Exact-head nonce.** Requirement "Intent digest v2 and program digest" treats the exact-head nonce as informational, because the revision increment already stops replay. Outcome mode records its nonce in a per-instance consumed-nonce set.
- **Successor deploy state.** Requirements "Immutable authority deployment audit" and "Forward-declared migration replaces in-place revocation" record the predecessor address in the successor's `Uninit` state, and the audit checks it against the signed migration digest.

## Follow-ups outside this revision

| Follow-up | Owner |
|---|---|
| New semantic profile for the claim, ProofContext and genesis schemas in `typed-schemas.md` | MC01 through extension paths |
| New bounds profile in `bounds.json`: proof sizes within the 1 MiB transaction limit, k, memory and time | MC01, with requirement "Measured bounds frozen into the program digest" |
| Site copy: per-transaction recursive checking of all four claims, the fixed "thirteen trust boundaries" count and roadmap data | MC08, SP12 |
| README pipeline diagram that separates the authorization and history proof from Compact | MC08, SP12 |
| Security page threat rows: maintenance-authority capture, write-without-read, guard disabling, unclaimed release and signature voiding | Wiki maintainer |
| `docs/FOOTGUNS.md` digest drift in `report-lessons.json`, pre-existing | A reviewed reconciliation step |
| Untracked AFK change package without coverage rows, pre-existing, and its F0 record citing the declared seam | AFK package owner |
| Expired AFK routing in `AGENTS.md` and the stale SP01 digest in the loan design binding, both pre-existing | Program maintainer |
| Graphify rebuild and wiki canvas refresh | Wiki maintainer |
| Second Charter reviewer verdict on this revision | RP03 review routing |

`sourceDigests` in `pcd-integration.json` pin the decision report and the PCD roadmap. Any edit to either needs a digest refresh in the same change; otherwise `verify.py` fails with "Stale PCD integration source".

## Review record

- Design review: Fable 5.1 audit of the design, 2026-09-11. Verdict approve-with-changes; its two blocking, six major and nine minor findings were applied before this revision.
- Result review: Fable 5.1 audit of the implemented revision, 2026-09-11. Verdict approve-with-changes, with no blocking findings. Its four major findings and the minor wording, citation and consistency findings were applied; the digest-pinning trade-off and the SP09.3 placement of the ledger-9 note were accepted as designed.
- The Charter's second independent reviewer has not reviewed this revision.
