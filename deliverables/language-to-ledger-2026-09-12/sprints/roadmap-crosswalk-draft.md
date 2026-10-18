# Proposed roadmap requirement crosswalk delta

**The actual verifier failed on 69 missing requirement mappings.** There are 148 requirement identities in the current spec directories and 79 rows in `coverage.json`; none of the 79 points to an absent requirement. The preserved earlier failure had 58 missing identities. The session-completion package adds the remaining 11.

Run once from `/home/charl/Moriarty`: `python3 openspec/sprints/verify.py`, exit1, message `Requirement crosswalk differs`. The [JSON draft](roadmap-crosswalk-draft.json) retains the exact tool result, current source hashes, every missing requirement/scenario identity, complete proposed rows, closing tasks, LL identities and stage prerequisites. This is navigation repair only; nothing has been applied or approved.

## Exact proposed edit

Append the 69 `proposedCoverageRow` objects in JSON order to [`openspec/sprints/coverage.json`](../../../openspec/sprints/coverage.json) at `/requirements`. Keep all 79 existing rows unchanged. Each addition uses the existing fields `spec`, `requirement`, `package`, `sprints`, `primaryClosingSprint` and `contributingSprints`. Preserve exact source-path/heading identity, including punctuation.

No change is proposed to the verifier, scanned directories, original MC task crosswalk, SP ownership/dependency graph, program register, PCD supersessions, source requirement text, historical checkmarks or resource records. The current single-package schema can represent the primary acceptance owner. Additional cross-owner task contributions are recorded explicitly in the draft; they do not become new prerequisite gates.

| Missing package | Additions | Earlier baseline |
|---|---:|---:|
| afk-live-financial-execution |37|37|
| language-to-ledger-lifecycle |21|21|
| session-completion-sprints |11|0|

## Requirement destinations

Each heading below is exact; the linked file and source line plus that heading form its identity. `MC/SP` is the proposed coverage owner. Existing closing tasks, cross-owner contributions, rationale and actual stage prerequisites are retained in JSON. Completed PR5–PR7 local predicates stay historical; these entries do not reschedule unchanged source work.

### [openspec/changes/afk-live-financial-execution/specs/afk-live-accounting/spec.md](../../../openspec/changes/afk-live-financial-execution/specs/afk-live-accounting/spec.md)

| Line and exact requirement | MC/SP primary | Existing closing tasks | LL destination |
|---|---|---|---|
| 69: Authenticated MC02 envelope on the preserved MC01 master | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 18: Authenticated context discovery and closed evidence payloads | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 52: Closed existing accounting fields and bounded successor semantics | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 189: Direct nonpersisting SDK funding observer | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 167: Durable debit intent and rollback-resistant first application | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 181: Exact allocation arithmetic without duplicate credit | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 126: Exact attempts and actual submission consumer | MC02/SP05 | SP05.3 | LL07.1, LL08.1, LL08.2 |
| 135: Independently bound observer launch and resource identities | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 3: Item 1.3 concrete source producer and existing caller | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 109: Master projection and global one-shot claim | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 41: Separately charged bounded read-only preflight | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 94: Single allocation, all-kind limits and prepaid boundary | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 151: Single preflight caller and occupied-primary handover | MC02/SP05 | SP05.3 | LL05.2, LL07.1, LL08.1 |
| 206: Source approval and live adoption stay separate | MC02/SP05 | SP05.3 | LL05.8, LL07.1, LL08.1 |

### [openspec/changes/afk-live-financial-execution/specs/afk-live-bindings/spec.md](../../../openspec/changes/afk-live-financial-execution/specs/afk-live-bindings/spec.md)

| Line and exact requirement | MC/SP primary | Existing closing tasks | LL destination |
|---|---|---|---|
| 16: Current source reviews and source-only acceptance | MC02/SP05 | SP05.1, SP05.3 | LL05.2, LL05.8, LL07.1, LL08.1 |
| 31: Digest-bound single-path SP01 prerequisite bridge | MC02/SP05 | SP05.1, SP05.3 | LL05.2, LL05.8, LL07.1, LL08.1 |
| 46: Existing I2 action/gates remain authoritative | MC02/SP05 | SP05.1, SP05.3 | LL05.2, LL05.8, LL07.1, LL08.1 |
| 3: Item 1.2 source repair scope and exact live labels | MC02/SP05 | SP05.1, SP05.3 | LL05.2, LL05.8, LL07.1, LL08.1 |

### [openspec/changes/afk-live-financial-execution/specs/afk-live-financial-execution/spec.md](../../../openspec/changes/afk-live-financial-execution/specs/afk-live-financial-execution/spec.md)

| Line and exact requirement | MC/SP primary | Existing closing tasks | LL destination |
|---|---|---|---|
| 43: Item 1 — Authenticate the SP01 prerequisite without rewriting history | MC02/SP05 | SP05.1, SP05.3 | LL05.1, LL05.2, LL07.1, LL08.1 |
| 20: Item 1 — Current binding identity with preserved history | MC02/SP05 | SP05.1, SP05.3 | LL05.1, LL05.2, LL07.1, LL08.1 |
| 60: Item 1 — Genuine cumulative accounting and live observations | MC02/SP05 | SP05.1, SP05.3 | LL05.1, LL05.2, LL07.1, LL08.1 |
| 135: Item 1 — Operational history is evidence | MC02/SP05 | SP05.1, SP05.3 | LL05.1, LL05.2, LL07.1, LL08.1 |
| 164: Item 2 — Complete callable loan lifecycle | MC02/SP05 | SP05.2 | LL05, LL07.6 |
| 179: Item 3 — Exact live admission and one bounded invocation | MC02/SP05 | SP05.3 | LL07.1, LL08.1, LL08.2 |
| 198: Item 4 — Actual financial result and original MC02 acceptance | MC02/SP05 | SP05.3 | LL07.7, LL08.6, LL09.1 |
| 215: Item 5 — Reviewed publication and honest handoff | MC08/SP12 | SP12.3, SP12.4 | LL09.3, LL09.5, LL09.6, LL09.7 |
| 226: Item 6 — Optional bounded F0 diagnosis | MC04/SP01 | SP01.4 | LL09.1 |
| 3: Per-item specification approval before execution | MC02/SP05 | SP05.1, SP05.3 | LL05.1, LL05.2, LL07.1, LL08.1 |

### [openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md](../../../openspec/changes/afk-live-financial-execution/specs/afk-loan-executor/spec.md)

| Line and exact requirement | MC/SP primary | Existing closing tasks | LL destination |
|---|---|---|---|
| 138: Complete isolated acceptance and separately pending live instance | MC02/SP05 | SP05.2, SP05.3 | LL05 complete inherited contract; LL07.6 |
| 29: Concrete one-shot runner invocation handoff | MC02/SP05 | SP05.2, SP05.3 | LL05 complete inherited contract; LL07.6 |
| 129: Durable result and consuming CLI semantics | MC02/SP05 | SP05.2, SP05.3 | LL05 complete inherited contract; LL07.6 |
| 53: Exact loaded-main classification before evidence-driven stop | MC02/SP05 | SP05.2, SP05.3 | LL05 complete inherited contract; LL07.6 |
| 42: Fixed deadlines with a retained outer margin | MC02/SP05 | SP05.2, SP05.3 | LL05 complete inherited contract; LL07.6 |
| 74: Identity-safe external containment and deterministic fault disposition | MC02/SP05 | SP05.2, SP05.3 | LL05 complete inherited contract; LL07.6 |
| 16: Immutable plan and exact current authority boundary | MC02/SP05 | SP05.2, SP05.3 | LL05 complete inherited contract; LL07.6 |
| 3: Item 2 complete callable source boundary | MC02/SP05 | SP05.2, SP05.3 | LL05 complete inherited contract; LL07.6 |
| 100: Prover lifetime established before daemon startup | MC02/SP05 | SP05.2, SP05.3 | LL05 complete inherited contract; LL07.6 |

### [openspec/changes/language-to-ledger-lifecycle/specs/bounded-loan-lifecycle/spec.md](../../../openspec/changes/language-to-ledger-lifecycle/specs/bounded-loan-lifecycle/spec.md)

| Line and exact requirement | MC/SP primary | Existing closing tasks | LL destination |
|---|---|---|---|
| 10: Complete conservation observation | MC01/SP03 | SP03.1, SP03.2, SP11.2 | LL01.1, LL03.4, LL03.5, LL04.5, LL07.4, LL08.4 |
| 3: Complete source-driven lifecycle | MC01/SP03 | SP03.1, SP03.2, SP11.2 | LL01.1, LL03.4, LL03.5, LL04.5, LL07.4, LL08.4 |
| 17: Repeatable developer example | MC08/SP02 | SP02.3, SP12.1 | LL01.1, LL09.2 |

### [openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md](../../../openspec/changes/language-to-ledger-lifecycle/specs/financial-postconditions/spec.md)

| Line and exact requirement | MC/SP primary | Existing closing tasks | LL destination |
|---|---|---|---|
| 25: Exact staged work and error precedence | MC01/SP03 | SP03.2, SP03.3 | LL01.1, LL03.3, LL03.4, LL04.3, LL04.5 |
| 14: One staged action and atomic publication | MC01/SP03 | SP03.2, SP03.3 | LL01.1, LL03.3, LL03.4, LL04.3, LL04.5 |
| 36: Public tooling and regression evidence | MC08/SP02 | SP02.3, SP12.1 | LL01.1, LL03.7, LL09.2 |
| 3: Versioned typed financial post-state access | MC01/SP02 | SP02.2, SP03.2 | LL01.1, LL03.3, LL04.3 |

### [openspec/changes/language-to-ledger-lifecycle/specs/language-delivery-reconciliation/spec.md](../../../openspec/changes/language-to-ledger-lifecycle/specs/language-delivery-reconciliation/spec.md)

| Line and exact requirement | MC/SP primary | Existing closing tasks | LL destination |
|---|---|---|---|
| 17: Persistent evidence and honest loop state | MC08/SP12 | SP12.3, SP12.4 | LL09.1, LL09.3, LL09.4, LL09.6, LL09.7 |
| 10: Preserve wider acceptance and user changes | MC08/SP12 | SP12.3, SP12.4 | LL09.1, LL09.3, LL09.4, LL09.6, LL09.7 |
| 3: Scoped merged capability status | MC08/SP12 | SP02.3, SP12.4 | LL01.1, LL09.4 |

### [openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-k-agreement/spec.md](../../../openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-k-agreement/spec.md)

| Line and exact requirement | MC/SP primary | Existing closing tasks | LL destination |
|---|---|---|---|
| 10: Complete differential observations | MC01/SP03 | SP03.1, SP03.2, SP03.3 | LL01, LL02, LL03 |
| 3: Executable supported Core in K | MC01/SP03 | SP03.1, SP03.2, SP03.3 | LL01, LL02, LL03 |
| 17: Explicit proof scope and bounded execution | MC01/SP03 | SP03.1, SP03.2, SP03.3 | LL01, LL02, LL03 |

### [openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-ledger-integration/spec.md](../../../openspec/changes/language-to-ledger-lifecycle/specs/lifecycle-ledger-integration/spec.md)

| Line and exact requirement | MC/SP primary | Existing closing tasks | LL destination |
|---|---|---|---|
| 3: Authenticated financial state and actual language consumer | MC02/SP05 | SP05.1, SP05.2 | LL05 complete inherited contract; LL07.6 |
| 17: Current guarded Preview admission | MC02/SP05 | SP05.3 | LL07.1, LL08.1, LL08.2 |
| 10: Docker financial lifecycle before Preview | MC02/SP05 | SP05.2, SP05.3 | LL07.2, LL07.3, LL07.4, LL07.5, LL07.6, LL07.7 |
| 24: Independent public financial acceptance | MC02/SP05 | SP05.3 | LL08.3, LL08.4, LL08.5, LL08.6 |

### [openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md](../../../openspec/changes/language-to-ledger-lifecycle/specs/loan-origination-accrual/spec.md)

| Line and exact requirement | MC/SP primary | Existing closing tasks | LL destination |
|---|---|---|---|
| 14: Bounded integer interest with explicit rounding | MC01/SP03 | SP03.2, SP09.3 | LL01.1, LL03.3, LL03.4, LL04.3, LL04.5 |
| 3: Explicit funded origination and authority boundary | MC01/SP03 | SP03.2, SP09.2, SP09.3 | LL01.1, LL03.3, LL04.1, LL04.4 |
| 25: Monotone period and duplicate protection | MC01/SP03 | SP03.2, SP09.3 | LL01.1, LL03.3, LL03.4, LL04.3, LL04.5 |
| 32: Whole-stack operation support | MC01/SP03 | SP02.2, SP03.2 | LL01.1, LL03.3, LL04.4 |

### [openspec/changes/session-completion-sprints/specs/session-completion-sprints/spec.md](../../../openspec/changes/session-completion-sprints/specs/session-completion-sprints/spec.md)

| Line and exact requirement | MC/SP primary | Existing closing tasks | LL destination |
|---|---|---|---|
| 73: Actual compiler artifact acceptance | MC04/SP05 | SP03.2, SP05.2, SP09.4 | LL06 |
| 62: Authenticated generated lifecycle consumer | MC02/SP05 | SP05.1, SP05.2 | LL04 |
| 84: Complete inherited executor and lifetime contract | MC02/SP05 | SP05.2, SP05.3 | LL05 |
| 47: Complete scoped K acceptance | MC01/SP03 | SP03.1, SP03.2, SP03.3 | LL01, LL02, LL03 |
| 99: Docker before separately admitted Preview | MC02/SP05 | SP05.2, SP05.3 | LL07, LL08.1 |
| 3: Identity-preserving sprint scope | MC08/SP12 | SP12.3, SP12.4 | LL01.1, LL09 |
| 110: Independent finalized Preview acceptance | MC02/SP05 | SP05.3 | LL08 |
| 14: Independent preparation with one implementation writer | MC08/SP12 | SP01.2, SP12.3 | LL01, LL04, LL05, LL09 |
| 36: One separately admitted diagnostic | MC01/SP03 | SP03.1, SP03.2, SP03.3 | LL01, LL02, LL03 |
| 25: Retained K author and exact source repair | MC01/SP03 | SP03.1, SP03.2, SP03.3 | LL01, LL02, LL03 |
| 121: Truthful completion and wider gates | MC08/SP12 | SP12.3, SP12.4 | LL01.1, LL09 |

## Mapping defects versus semantic conflicts

The observed verifier failure is purely a missing set crosswalk: all 69 source headings exist and no existing row needs removal. The following inherited contract issues affect later execution or closure and cannot be solved by appending navigation rows.

**expired-afk-window** (substantive execution-authority incompatibility; mapping-only repair cannot solve it). Keep September11 deadline and consumed interval historical. Future runtime must bind currently reviewed allocation/deadline under latest full-branch authority; no clock/resource reset from append-only mapping. Preserve relative executor cutoffs and margins.

**four-submission-old-loan-versus-full-lifecycle** (substantive runtime/coverage incompatibility). Historical attempt3/four-call deploy-initialize-accrue-settle proposal is not authority for source5 originate/partial repayment. Freeze actual full lifecycle stage grouping and reviewed schema/resource delta before dependent execution; preserve original attempts and charges.

**immutable-existing-financial-schema** (unresolved inherited caller/schema compatibility). Retain immutable preview-financial-launch/1 original and permitted deadline-only runtime copy. Independently review whether source5 consumer fits that reviewed schema; if not, specify exact versioned change under existing task rather than silently relaxing closed fields.

**review-routing-scope** (historical exact reviewer contract; current scope must be explicit). AFK executor/source contracts name both Opus and Astra. Lifecycle task routing is Grok/fresh Astra with retained extra auditors where inherited contract requires both. Preserve historical identities and use explicit scoped later authority; missing auditors never approve.

**pcd-old-finalizer-wording** (substantive mechanism supersession already explicitly recorded). Map to SP01.4/f0 with existing amendment; core uses ledger seam, certificates use ledger-side pairings. No old outer-finalizer or f2-before-f3 campaign is revived; release still requires f2.

**six-item-existing-goal-wording** (scope supersession under current user instruction). Keep six session outcomes as subset and preserve historical goals; current active thread requires full branch roadmap. Mapping must not make six outcomes sufficient for closure.

**diagnostic-stage-prospective** (separate pending registration/admission; not a missing requirement mapping). Current canonical SP03 has SP03.1–SP03.3. SP03.4/k-retained-trace106-diagnostic lives in prospective reviewed drafts, not current stage graph. Coverage row may assign SP03 ownership but grants neither that registration nor diagnostic execution.

The original PCD amendment remains controlling: atomic `f3` uses the Stage 0 seam and requires `atomic-accept`, `i2`, `f0`, `native-path-freeze`; it does not wait for certificates. `mandatory` has no certificate-stage dependency; `release` retains `f2`. An AFK F0 mapping cannot revive the retired constrained-finalizer route or make native certificates optional for release.

## Verification and next use

Prospective consistency checks confirm that the append yields exact 148-identity set equality, with no duplicate, unknown MC/SP owner or invalid existing closing-task/stage reference. Every proposed package owns every sprint listed in its row, and contributor sets match existing row rules. All current spec directories remain in the inventory. Source hashes and source-line/scenario identities were checked. These checks do not establish that the full verifier passes: its single actual run stopped at requirement equality, and downstream checks have not been executed on an applied delta.

After independent review, the sole writer can append these rows to the existing coverage file and run the existing verifier normally. Any later failure should retain its exact evidence and be repaired under its original owner. LL01 offline source repair remains independently eligible. This draft grants no resource, source/result approval, stage registration, diagnostic admission or financial execution.
