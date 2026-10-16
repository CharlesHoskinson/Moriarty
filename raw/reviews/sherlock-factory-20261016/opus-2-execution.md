# Moriarty V1 planning review: Opus 5.5, security-owner lens (execution and resource constraints, compiler/circuit and stagenet unblocking)

**Status of this document.** This is a planning proposal only. It is not product acceptance, a source audit, a runtime audit, a security approval or a native admission. No tools, shell, writes or web access were used. No tests, builds, models, SDK, native or ledger actions ran. No native command, resource reset or new counter is authorized or created here. Security judgments in this document are Opus-authored. Nonsecurity sprint content is offered as proposal input for the other reviewers.

## 0. Sources and inspection limits

**Read in full, as supplied text:**

| Source | Supplied SHA-256 |
|---|---|
| `ROADMAP.md` | `0afd1af3…` |
| `docs/MORIARTY-PRODUCT-CONTRACT.md` | `d5674c65…` |
| `openspec/changes/consolidated-language-kernel/requirements.md` | `aa7b2243…` |
| `openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md` (UNI-001–024) | `66f85468…` |
| `openspec/changes/consolidated-language-kernel/traceability.md` | `d549f31a…` |
| Complete heading index | original `a361071a…` |
| Latest Root phase-1 receipt | capture `4b20fae0…` |
| `docs/audits/2026-10-14/SECURITY-REVIEW.md` | `64cb033e…` |
| `docs/audits/2026-10-14/SECURITY-ROUTING.md` | `04699b07…` |
| `docs/audits/2026-10-14/MID-ROADMAP-REVIEW.md` | `6f2a8bc3…` |
| `docs/audits/2026-10-14/AUDIT-PLAN.md` | `9aaec5d0…` |

The user's message, including the baseline facts about `compile_reported`, the frontends, phase 1, K33, A102, G4-BYTE, Fenrir and stagenet, is also a source.

**Not read:**
- Any repository source at `9bcbca52…`.
- The linked dossiers.
- The `docs/MORIARTY-BACKEND-REQUIREMENTS.md` text (ZR01–16 and MNR01–08 bodies).
- The MPLR notes.
- The OSR-1014 addendum.
- SP10.1.
- The Sherlock READMEs.
- `docs/SHERLOCK-CORRECTNESS-LOOP.md`.
- `raw/assignments/moriarty-stagenet-acceptance-2026-10-15.md`.
- The PR95–101 diffs.

**Limits that follow:**
- I did not recompute any hash.
- All `path:line` references come from SECURITY-REVIEW and are pinned to `cb8280ef`, not `9bcbca52`. They must be re-located before anyone edits code.
- I cannot confirm whether the Core continuations candidate (`moriarty/2`), which the roadmap describes as "local, unpublished, unmerged", was among PR95–101.
- The supplied text does not define the "C4" label on A102. I carry it forward as given.
- Wherever I name a requirement body I did not read (ZR, MNR, OSR, MPLR), I reference it by ID only and make no claim about its contents.

---

## 1. Current evidence versus incomplete behavior

### 1.1 Evidence classes (all are supplied claims, none freshly executed)

| Area | Strongest supplied evidence | Class | Source |
|---|---|---|---|
| Rust Core `moriarty/1` / `moriarty-core/1` check, compile, simulate, test, intent, replay | Implemented local consumers | historical-local plus source | MID-ROADMAP table; ROADMAP "Current delivery observation" |
| `mori test` aggregate report | 4 CLI regressions: 256 ordered cases, 3-mismatch failure exit, 87 FeePool cases | historical-local | ROADMAP "Local tooling observation"; UNI-024 third scenario |
| Static all-action call export and reopen | PR47/49, byte reproducibility only (SEC-08: mapped mode unauthenticated) | source-static | SECURITY-REVIEW B5, SEC-08 |
| Mapped-role provenance metadata | 10 focused tests; one byte-identical specimen | historical-local | ROADMAP "Compiler provenance" |
| FeePool | 23 actions, 162 finite financial cases | historical-captured at `ce05a5c7` tree | SECURITY-REVIEW §5 item 11 |
| Native proof | One k13 signed payment, backend stack, tree `ee963f61`. 6 hostile-witness and 6 mutation rejections | historical native, other tree and stack | ROADMAP "Delivery observation 2026-10-11"; SEC-02 and SEC-15 |
| Preview loan and swap | Scoped historical Preview results | historical target (Preview only) | ROADMAP "What exists"; UNI-022 |
| Core continuations `/2` | 58→78→86→88 focused-test observations, partly carried forward rather than rerun | historical-local, candidate status unclear | ROADMAP continuation sections |
| Sherlock K-A Quint | 98 + 126 normal tests (separate cohorts) | formal model, historical | ROADMAP "Sherlock design work" |
| Sherlock host/image interpreters | 46 → 79 → 82 → 86 host checks | historical-local, private test-only | same |
| Draft frontend | 11 tests; **`cfg(test)` only**, differs from the production frontend | historical-local, not public | user baseline; ROADMAP |
| Offline readback helper | 16 assertions | historical-local | ROADMAP |
| Append model | Typecheck, 8 normal tests, short seed-1 simulation | formal model | ROADMAP "Sherlock source status" |
| aux/2 arithmetic/fuel Quint | **Latest receipt: 18 named tests passing, exit 0, no parent timeout, closures matched.** Zero failed/ignored is *inferred, not printed*. Trace adjudication is PENDING because of the regex bug | formal model, one run, adjudication open | Root phase-1 receipt |
| aux/2 K (33 checks) | UNRUN | none | user baseline |
| aux/1 (6 tests), A102 machine (16 tests) | UNRUN. A102 not compiled; native authority 0 | source only | ROADMAP; user baseline |
| G4 source requirements; G4-BYTE | Conditional source argument; **G4-BYTE draft unadopted** | source only | ROADMAP; user baseline |
| `compile_reported` | Calls the existing backend compiler, then appends a Sherlock projection report. **Sherlock's scalar-only restriction applies to the report, not to the Core compiler**, which already handles state, assets, transfers and liabilities | source fact supplied by the user | user baseline |

**Staleness to fix after adjudication.** ROADMAP still says "all 18 tests … remain UNRUN". The later receipt records them as passing. The roadmap should not be updated until the trace is adjudicated (Sprint S1).

### 1.2 Incomplete behavior (parent exits all open)

- **U0:** no single profile × consumer × semantics × enforcement × target matrix. The numeric profile (orientation, rounding, beneficiary per SEC-10) is not ruled on. Premise ledger: SECURITY-REVIEW §3, P column.
- **U1:** no native certificate for any primitive at the current tree. Signature, phase and recursion feasibility are unmeasured at the current tree. k for fraud circuits is unmeasured (SEC-13).
- **U2:** no two new positive programs through one declared pipeline. Canonical stack is unchosen (SEC-02). No current proof at `cb8280ef` or `9bcbca52`. No complete effect readback on any target from current source.
- **Midnight stagenet:** nothing executed. Target facts are not established in supplied text (§2, S6).
- **U3–U7, M1–M17, MC01–08, SP01–12, G01–G24, ZR, MNR, OSR, ACTUS, DeFi, composition:** all open. Denominators are retained (UNI-023).
- **Sherlock:** no complete step witness, no compiled fragment, no circuit correspondence, no native result, no stagenet result.

---

## 2. Assigned lens: execution and resource constraints, and how to unblock

### 2.1 Necessary physical or objective limits versus excessive process

| Constraint | Classification | Opus reasoning | Proposal (no reset) |
|---|---|---|---|
| SRS pinned digests for degrees 0–19; backend `MAX_K = 19` (SEC-13) | **Necessary** (cryptographic trust plus target limit) | SRS trust rests on the SDK digest table. Exceeding k changes the trust input | Keep. Extend the same refusal to library `generate_call_keys` (S3) |
| 64 KB transaction ceiling, fraud k ≤ 19 (M5) | **Necessary** (target-objective) | Ledger validity condition | Keep. Add measurement (S5) and pre-submission refusal (S3, S6) |
| Signed scope: ≤16 assets, ≤16 duties, sole payer and debtor | **Necessary for now** (implementation soundness boundary, SEC-11 coupling) | Relaxing it without a circuit change reopens SEC-11 | Keep for U2. U3 needs a designed extension (S8) |
| 1 MiB production-preparation refusal; 128-ID cap; nonce map of 12,153 entries | **Necessary** (implementation capacity) | Exhaustion behavior must be explicit | Keep. Document exhaustion as a refusal, never as success |
| Host memory and time for keygen and prove | **Physical, unmeasured** | Exhaustion can corrupt evidence or the host | Measure in the first admitted native packet. No limit change without a new decision |
| Isolation of private material (PK, wallet seed, signing key) | **Necessary security** | Leakage is irreversible | Keep. Strengthen structurally (§2.4) |
| Stagenet spend, DUST and wallet authority | **Necessary** (owner authorization; outward-facing, irreversible) | Publishing a transaction is irreversible | Requires the user's explicit admission per packet |
| Native authority = 0 | **Legitimate operator state**, not a physical limit | It protects the two rows above | Only the user can change it. §2.3 proposes a bounded *standing tier* the user may choose to adopt; nothing is adopted here |
| strace post-hoc trace adjudication for **pure** Quint/K model runs | **Excessive relative to its purpose** | The purpose is to show that no native, SDK, network or private access happened. Preventing that structurally is stronger and cheaper than detecting it after the fact. The current block is a parser bug, not a detected violation | Fix the parser and adjudicate the *existing* capture once (S1). Then propose structural isolation for future pure runs, subject to the user's decision |
| Per-child conditional Opus/Muse "adoption" votes before ordinary tests run | **Excessive** | Votes are not evidence (UNI-016 hostile scenario). They delay the cheap tests that would produce evidence | One independent non-author review per PR. Opus review only for security-touching PRs |
| Phase-1/phase-2 "P6" sequencing for pure models | **Excessive** for pure models | Pure model tests carry no resource or private-material risk once isolated | Run the whole pure suite as one job under isolation |
| "Carried forward on unchanged call paths" counting (71 of 88, 62 of 86) | **Excessive and weaker** | The projection argument costs more than rerunning cheap tests, and proves less | Rerun every pure test on every PR in CI |
| Receipts or Foreman/Pel as implementation governance | **Removed by user override** | Product contract already excludes it from public validity | Delete the roadmap's "Foreman/Pel governs project implementation" line; replace with §4 |
| Serialized build lock | **Necessary for the native lane only** | Contention matters only for heavy native jobs | Keep for T2/T3. T1 can parallelize |
| Unpinned `timeout` utility in the tool closure | **Genuine pinning gap** | An unpinned tool tripped the gate; its cause is not identified in supplied text | Add the utility to the pinned closure (S1) |

### 2.2 Test tiers

| Tier | Scope | Allowed operations | Authority required |
|---|---|---|---|
| **T0** | Reading, design, documentation | None executed | None |
| **T1** | Pure and ordinary work | Rust tests built **without** native entry points; Quint typecheck and tests; K checks; Core simulate, test and intent; source-static compile, export and reopen; JSON comparison | Existing ordinary authority. Becomes structurally safe after S1 |
| **T2** | Native-local | SDK `check`, `k()`, VM/transcript, keygen, prove, native verify, signed `verify-intent --profile`, `simulate --trace --target`, the replay generator, `prepare-call` / `prepare-signed-call` (SECURITY-REVIEW §4) | **Separate user admission per packet.** Not created here |
| **T3-stagenet** | Midnight stagenet deploy, call, readback | Outward, irreversible | Separate user admission, plus established target facts |
| **T3-Preview / T3-Fenrir** | Separate targets | Same | Separate. Fenrir stays blocked on unknown facts (UNI-022) |

A pass at one tier never transfers to another tier, tree, stack or target.

### 2.3 What a T2/T3 admission packet must contain (template for the user's decision; not an admission)

1. Exact source SHA and tree, plus Cargo.lock and toolchain identities (rustc, SDK crate versions, proof-server version if any, coreutils `timeout`).
2. The exact *existing* commands. The packet author enumerates them from source at that SHA. This plan does not list or authorize any.
3. Bounds:
   - maximum k, which must be ≤ 19 and come from the pinned table;
   - process count;
   - wall time per process;
   - memory ceiling;
   - number of attempts;
   - disk paths for keys and SRS;
   - for T3, maximum spend in test tokens and DUST, and maximum transaction count.
4. Private-material rules: proving keys and wallet seed stay outside the repository, are never printed, and are named only by digest.
5. The frozen oracle commit SHA, which must exist **before** the run (S4).
6. Abort conditions (§2.5) and the output location (`RESULT.md` holding raw outputs).
7. Counter charges under the *existing* counters. No reset.

### 2.4 Retry and failure semantics (proposed)

| Failure kind | Example | Semantics |
|---|---|---|
| Deterministic refusal | Compile refusal, admission error, verify reject on a positive control | **No retry.** Record verbatim, diagnose, fix on a new SHA, then submit a new packet |
| Expected hostile rejection | s+1 response, cap −1 | Counts only when its paired valid positive control passed in the same run |
| Environmental local failure | OOM kill, host time limit | Record. At most **one identical-input retry** if the packet pre-declares it. Never raise k, memory or time without a new user decision |
| Read-only target query failure | Indexer or RPC error | Bounded retry with backoff, as declared in the packet. Record each attempt |
| **Submission status unknown** | Timeout after submit | **Timeout is not nonexecution** (product contract). Do not re-sign and do not use a new nonce. First query by transaction identity and by nonce/state. Resubmit only the *identical bytes*, and only if the queries show the transaction absent and the nonce unconsumed. Otherwise record "unknown" and stop |
| Trace or adjudication indeterminate | Unparseable record | Report `indeterminate`. That is neither pass nor fail, and dependent phases do not start |

**Stop and escalate to Opus** on any apparent soundness break. Record it; do not build an exploit (SECURITY-REVIEW §7).

### 2.5 Missing facts and owners

**Midnight stagenet.** The external Midnight operator supplies these; the user obtains them. None are invented here.

| # | Fact | Why it matters |
|---|---|---|
| F1 | Stagenet network ID string, exactly as used in addresses and transaction domain | M7, SEC-05 |
| F2 | Node RPC, indexer and proof-server endpoints, with their versions | Deployment and readback |
| F3 | Deployed ledger version (confirm ledger-9), accepted ZKIR major/minor, on-chain runtime version | M1, M2, UNI-003 pins |
| F4 | Verifier parameters: SRS identity and maximum k accepted by stagenet | Must match the pinned digest table |
| F5 | Encoded transaction size limit and fee/DUST parameters | M4, M5 |
| F6 | Whether stagenet deployment accepts verifier keys from non-`compactc` ZKIR, and the deployment interface format | **The decisive interface question for Moriarty** |
| F7 | Block-time semantics exposed to contracts (if any), and TTL semantics | SEC-01 fix design |
| F8 | Native token-type derivation on stagenet | SEC-09, M9 |
| F9 | Finality and indexer lag semantics for readback | Retry semantics |
| F10 | Funded test wallet: who holds the seed, faucet, DUST registration | T3 spend authority |
| F11 | Whether "Anchor" applies to stagenet. If it does not, record `N/A` explicitly | Avoids inventing values |

**Host (operator):**
- F12: host RAM, cores and disk. Prove/keygen time and peak memory at the relevant k is **unmeasured**; the first T2 packet measures it.
- F13: availability of a process-isolation mechanism, such as a network namespace or read-only bind mounts, on the real execution host. The environment shown to me is not evidence about that host.

**Interface and design:**
- F14 (compiler integration owner; Opus security recommendation): canonical U2 pipeline. **Recommend ledger signed-v2 installed**, with the backend stack as a differential leg only (SEC-02).
- F15 (toolchain owner; T1 source reading): whether `mori-ledger` local deployment output can be turned into a stagenet deployment transaction.
- F16 (user ruling): whether the Oct 15 stagenet requirement mandates the **Sherlock circuit**, or accepts the existing Core compiler path for the first stagenet evidence. I did not read that file.
  - **Recommendation:** the first stagenet row uses the Core compiler, labeled "Core-compiler scope". Sherlock stagenet is a separate later row.
  - Reason: `compile_reported` already wraps the Core compiler, which handles financial state. The Sherlock circuit has no compiled fragment yet.

**Fenrir:** version, network ID, Anchor, committee, registry and deployed tuple stay **unknown**. M2, M12, M16 and M17 Fenrir rows stay blocked. Language work continues regardless (UNI-022).

### 2.6 Safe direct repo-native delivery without Foreman

- **Structural T1 isolation (S1).** Put every T2 entry point behind a Cargo feature (`native`, off by default) so that T1 builds *do not compile them*. This replaces per-run "inspected filter" judgments with a property CI can check. Linking pure SDK field or IR types for source-static construction remains allowed; it matches the existing vocabulary in AUDIT-PLAN "Execution boundary".
- **CI runs T1 only.** It has no secrets, no network beyond fetching pinned dependencies, and no private paths.
- **The native lane is one human-run job per admitted packet.** It is a plain checked-in script that lists exactly the admitted commands, with no scheduler. Its output is a `RESULT.md` with raw outputs, merged by PR.
- **Evidence equals the committed test files, the CI log and RESULT.md.** No receipt review, no campaign and no vote counts as acceptance.

---

## 3. Prioritized capability sprints (12)

Every sprint keeps the U/M/MC/SP/G/ZR/MNR/OSR denominators. "Accept" means the observable result that an independent party can check. Parent exits close only when all of their rows close.

### S1: T1 structural isolation and phase-1 adjudication repair (assigned lens; highest priority)

- **Parents:** UNI-016, UNI-018; enables U0–U2 and Sherlock.
- **Prerequisites:** none.
- **Owned interfaces:**
  - `Cargo.toml` features for `mori-ledger`, `mori-backend` and `mori`;
  - the native entry points named in SECURITY-REVIEW §4 (`relation/mod.rs` SDK `check`, `ir.k()`, VM, keygen, prove, verify);
  - the trace parser;
  - the pure-model runner script;
  - the pinned tool closure.

**EARS:**
- **WHERE** a build is classified T1, the workspace build **SHALL** be produced with the `native` feature disabled, and the SDK `check`, `k()`, VM, keygen, prove and verify entry points **SHALL NOT** be compiled into any T1 artifact.
- **IF** a T1 job's resolved build contains any gated native entry point, **THEN** the job **SHALL** fail before running any test and **SHALL** print the dependency path that enabled it.
- **WHEN** a T1 job runs, it **SHALL** have no network after dependency fetch and no read access to private-material paths. It **SHALL** record the identities of rustc, cargo, quint, the K toolchain, node and `timeout`.
- **WHEN** a syscall trace is adjudicated, the parser **SHALL** frame records at syscall boundaries, including unfinished/resumed pairs, and **SHALL** decode argv with whitespace preserved.
- **IF** any trace record cannot be parsed, **THEN** adjudication **SHALL** report `indeterminate` and dependent phases **SHALL NOT** start.

**Accept:**
1. Parser fixture tests:
   - a reconstructed unfinished/resumed `execve` with spaces in argv parses to the exact argv (positive);
   - a fixture trace containing a forbidden `execve` is flagged (negative);
   - a truncated record yields `indeterminate`.
2. The **existing** phase-1 capture is adjudicated once with the repaired parser. Outcome is recorded as clean, violation or indeterminate. This needs no model rerun.
3. A deliberate test branch that enables `native` under T1 fails before running tests (negative). Main T1 passes (positive control).
4. The roadmap is updated only after item 2.
5. Separate user decision: whether future pure-model runs use structural isolation in place of strace adjudication.

**Tier:** T1. **Formal/native/target:** none claimed.

### S2: U0 profile, admission and field-enforcement matrix (generated, not hand-written)

- **Parents:** U0, UNI-018/019/021/022, M1, M6, MC01, SP01–03.
- **Prerequisites:** S1.
- **Owned interfaces:** `crates/mori/src/commands/check.rs` and `compile.rs`; `packages/moriarty-beta/src/bridge.ts`; the production and `cfg(test)` draft frontends; `compile_reported`; the premise ledger.

**EARS:**
- **WHEN** the matrix is generated, every Core construct **SHALL** have one row per consumer (Core evaluator, backend lowering, ledger signed-v2, ledger unsigned, Sherlock projection, production frontend, `cfg(test)` draft frontend, Node beta) marked admitted, refused-with-reason or unknown.
- **IF** a construct is added to Core without a row for every consumer, **THEN** the T1 matrix test **SHALL** fail.
- **WHEN** `compile_reported` runs on a program containing constructs outside the Sherlock scalar projection, the Core compile output **SHALL** be byte-identical to plain `compile`, and the Sherlock report **SHALL** list each excluded construct as `projection-unsupported`.
- **WHERE** a frontend exists only under `cfg(test)`, no public capability record **SHALL** cite it.
- **WHERE** the numeric profile is declared, it **SHALL** state price orientation, unit dimensions, per-primitive rounding direction and the owner ≠ beneficiary fee policy (SEC-10).
- Every field row **SHALL** state C, H or P enforcement per SECURITY-REVIEW §3, and every P row **SHALL** name a closing predicate and owner.

**Accept:**
- Positive: a scalar program yields both outputs with an empty exclusion list.
- Contrast: a program with assets, transfers and liabilities compiles through Core while the Sherlock report lists its exclusions.
- Negative: adding a stub construct without matrix rows fails T1.

**Tier:** T1. **Distinction:** this is a source and admission fact, not correspondence.

### S3: Signed-v2 host-surface security hardening (Opus-owned; pure)

- **Parents:** M7, M8 (partial), M9, M10, M13, U2, UNI-003/004/005.
- **Prerequisites:** S1. For SEC-01, a design ruling on the TTL relation; my recommendation is below.
- **Owned interfaces:** `signed/mod.rs` (`admit`, `from_source`), `layout.rs`, `mori-core/src/intent.rs`, `commands/verify_intent.rs`, `mori-core/src/policy.rs`, `mori-ledger/src/proving.rs` (`generate_call_keys`), `commands/lsp.rs` and `mcp.rs`. All are as of `cb8280ef`; re-locate them before editing.

**EARS:**
- **SEC-01 (recommended ruling).**
  - **IF** a signed call request's TTL exceeds its signed `not_after`, **THEN** admission **SHALL** refuse with a named error before preparation.
  - Rationale: this makes the ledger-enforced TTL carry the owner's upper bound. The in-circuit `not_before ≤ at` remains prover-asserted and **SHALL** be labeled as such in output until F7 resolves.
- **SEC-09:** **IF** signed bindings name an asset whose network differs from the deployment network, **THEN** `from_source` **SHALL** refuse.
- **SEC-07:** **WHEN** `verify-intent` evaluates effects that debit a party other than the signer, the output **SHALL** include `unauthorized_non_signer_effects` listing each such effect and **SHALL NOT** present `accepted` without that field.
- **SEC-11:** **IF** any duty's debtor differs from the owner under signed-v2, **THEN** compilation **SHALL** refuse, so the circuit's all-duty cap cannot silently diverge from Core's signer-debtor cap.
- **SEC-13:** **IF** `generate_call_keys` receives parameters whose digest is not in the pinned table, or a ceiling above `MAX_K`, **THEN** it **SHALL** refuse before decoding.
- **SEC-18:**
  - **WHEN** a sponsor hold is submitted, its state **SHALL** become `Submitted`.
  - **IF** `release_unused` is called on `Submitted`, **THEN** it **SHALL** refuse.
- **SEC-19:** **WHEN** `mori lsp` or `mori mcp` is invoked, it **SHALL** exit nonzero with an `unsupported` diagnostic and **SHALL NOT** panic.
- **SEC-14 (static part):** the target transaction byte ceiling **SHALL** be a named constant whose value comes from F5 and is never invented. Until F5 is known, it **SHALL** read `unknown` and submission **SHALL** refuse.

**Accept (each negative paired with a passing positive in the same pipeline):**
- TTL: TTL = `not_after` and TTL < `not_after` are accepted; TTL = `not_after` + 1 is refused.
- Asset network: same network accepted; different network refused.
- Third-party effects: signer-only effects give an empty list; a third-party debit is listed.
- Owner debtor: owner-only debtor compiles; a second debtor is refused.
- Key ceiling: pinned digest at k ≤ 19 is accepted; unpinned digest or k = 20 is refused before decode.
- Sponsor hold: `Reserved → release_unused` succeeds; `Submitted → release_unused` is refused.
- LSP/MCP: exit code and message, with no panic.

**Tier:** T1. **Distinction:** these are host checks only; no circuit claim.

### S4: Two new positive U2 programs with independently frozen oracles

- **Parents:** U2, M1, M3 (prep), MC01, MC04, UNI-002/005/020, SP02/03.
- **Prerequisites:** S2 admission rows; the S3 refusals.
- **Owned interfaces:** `examples/core/` (new directories), oracle directory, `mori test` manifests.
- **Roles:** the program author, oracle author and implementer are three different people or models. The oracle is committed, with its SHA recorded, **before** any run of the program. The oracle does not import Core, the compiler or evaluator code.

**P-A (installment invoice; one asset):**
- Pre-state: owner holds 1000 A.
- Action `pay(a=100, f=3)`.
- Effects, in order: owner→beneficiary 100 A; owner→treasury 3 A; owner duty opened 50 A to the beneficiary; one `emit`.
- Signed caps: gross 103, fee 3, net floor per the U0 profile, liability cap 50.
- All other state is unchanged.

**P-B (conditional reserve allocation; structural contrast):**
- Two assets, each with its own limits row. No fee.
- Action `open_liability` creates a duty of 40 B with no transfer.
- Action `discharge` is coupled to a 40 B owner→creditor transfer in the same action. Answers SEC-12 at program level.
- Boolean state flag.
- Weighted split with n = 1000, w = 3, d = 7, giving witness q = 428, r = 4. Guards:
  - `requires q*d + r == n*w`;
  - `requires r < d`;
  - `requires d > 0`;
  - all inputs < 2^64.
- Branches are separate actions, since Core has no conditional statement or division.

**EARS:**
- **WHEN** `P-A.pay(100, 3)` executes from the declared pre-state, the complete ordered effects, changed and preserved state, gross debit 103, fees 3, net receipt, liability opening 0 / creation 50 / closing 50, and logical work **SHALL** equal the frozen oracle.
- **IF** the signed gross cap is 102, **THEN** acceptance **SHALL** reject. The positive control is the same program and inputs with cap 103.
- **IF** P-B receives (q = 427, r = 11), **THEN** it **SHALL** refuse at `r < d`. The positive control is (428, 4).
- **IF** P-B receives d = 0, **THEN** it **SHALL** refuse.
- **IF** P-B attempts `discharge` without the coupled transfer, **THEN** it **SHALL** refuse, and the duty **SHALL** remain 40 B.
- A mixed-payer FeePool-style program (N-1) **SHALL** refuse with `one-public-payer` before the signed sole-payer check and with no output directory. It **SHALL NOT** count as a U2 positive.

**Accept:**
- T1: simulate and `mori test` against the oracles; static `compile-call --authorization signed-v2` export and exact reopen for both programs.
- The oracles diverge structurally between P-A and P-B (assets, effects, duty evolution, arithmetic).

**Tier:** T1 only here. Proof is S5.

### S5: Native-local U1/U2 slice (T2; requires user admission, not granted here)

- **Parents:** U1, U2, M3, M5, M8 (controls), MC01/04, ZR01/05/06, MNR05/06 (by ID), UNI-003/009.
- **Prerequisites:** S3, S4, F12, F14, an admitted packet (§2.3).
- **Owned interfaces:** existing ledger signed-v2 compile, keygen, prove and cold-verify paths (SEC-15).

**EARS:**
- **WHEN** P-A and P-B are proven under the admitted packet, cold verification in a separate process holding no PK **SHALL** accept. Local post-state readback **SHALL** equal the S4 oracles field by field.
- **IF** a hostile witness is presented, **THEN** proving **SHALL** fail or verification **SHALL** reject. The hostile cases are:
  - response s + 1;
  - wrong key;
  - each cap set one below the honest value;
  - nonce reuse;
  - R with a torsion component (C3);
  - challenge at the reduction boundary (C3).
- Each hostile case **SHALL** be paired, in the same run, with an unmodified valid witness that verifies, and with a cap-plus-one variant that verifies.
- **WHEN** cold verify receives a mutated VK, public input, proof bytes, keyset or predecessor (C4), it **SHALL** reject, and the unmutated tuple **SHALL** accept.
- **WHEN** any circuit is proven, the run **SHALL** record k (≤ 19), peak memory, wall time, proof bytes, VK identity and encoded call size.
- C2 (`at` inside the window while real time is outside) **SHALL** be recorded as a *known accepted limitation demonstration*, not a pass.

**Accept:** `RESULT.md` with raw outputs, hashes and measurements. An independent re-verification by a second operator using the published artifacts is desirable. **Tier:** T2. **Distinction:** local native only; not stagenet.

### S6: First Midnight stagenet end-to-end (T3; requires facts F1–F11 and user admission)

- **Parents:** M1, M2 (stagenet tuple only), M3, M4 (partial), M5, M7, M9, MC02/04, UNI-003/022; the Oct 15 stagenet requirement under the F16 ruling.
- **Prerequisites:** S5 for P-A at minimum; F1–F11; an admitted packet.

**EARS:**
- **WHEN** P-A is deployed to Midnight stagenet, the deployment record **SHALL** bind:
  - the network ID;
  - the contract address;
  - VK identities equal to VKs regenerated locally from source;
  - the ledger, ZKIR and runtime versions as reported by stagenet.
- **WHEN** the signed call is submitted and final, readback from an indexer or node query independent of the submitting client **SHALL** show complete effects (balances, fee recipient, DUST spent, duty state, nonce consumed) equal to the frozen oracle plus the declared fee model.
- **IF** the identical signed call is resubmitted, **THEN** it **SHALL** be rejected and no balance **SHALL** change.
- **IF** an intent signed with a different network ID is submitted to stagenet, **THEN** it **SHALL** be rejected. The positive control is the stagenet-bound intent.
- **IF** the encoded transaction exceeds the F5 ceiling, **THEN** the client **SHALL** refuse before submission.
- **WHILE** the submission status is unknown, the client **SHALL** follow §2.4 and **SHALL NOT** re-sign.

**Accept:** stagenet transaction identities, readback output and an oracle diff. A second person repeats the readback independently. **Tier:** T3-stagenet. It is labeled "Core-compiler scope" unless F16 rules otherwise. Preview and Fenrir are not implied.

### S7: Sherlock circuit track to a first native-checked fragment

- **Parents:** U0/U1/U2/U4 preparation; P1, P3, P4; G4; IO1–IO8 (by ID); stagenet Sherlock row.
- **Prerequisites:** S1 (adjudication and K33 run); a G4-BYTE adoption decision.
- **Owned interfaces:** `models/sherlock/*`, `literal_schema`, `emit_literal_step`, aux/1 and aux/2 sources.
- **Lean/Clean** is developer QA only: no public build, prover or acceptance dependency.

**EARS:**
- **WHEN** the aux/2 suite runs under T1, all 18 Quint tests and 33 K checks **SHALL** report per-test outcomes, and failed and ignored counts **SHALL** be *printed*, not inferred.
- **IF** a Quint test and a K check share an input, **THEN** their outputs **SHALL** agree, and **SHALL** match independently authored UInt128 expectations.
- **WHEN** `emit_literal_step` emits the A102 fragment, two builds **SHALL** produce byte-identical IR, and the existing IR reader **SHALL** reopen it.
- **WHERE** a Sherlock fragment claims circuit correspondence, T2 **SHALL** show a valid-witness acceptance alongside adversarial-witness rejections (wrong carry, fuel underflow, out-of-range limb) before the claim is recorded.
- **WHERE** Lean/Clean artifacts exist, the public compiler's dependency graph **SHALL** contain none of them (T1 check).

**Accept:** T1 for the models and emitter; T2 for the fragment under admission; stagenet as a later row. **Distinction:** formal, then native, then target, each recorded separately.

### S8: U3 conditional settlement and recovery discriminator

- **Parents:** U3, M11 (partial), M13, M14 (partial), MC04–06, MPLR-001–010/017/019, UNI-005–008, L1/L2/L4/L5/L7/L8.
- **Prerequisites:** S4 and S5. Design decision on stage decomposition (below), owned by language design with Opus review.
- **Design proposal (Opus):** the 11 A / 20 B contract needs a counterparty debit, which signed-v2 sole-payer blocks (SECURITY-REVIEW §5 item 5). Decompose into single-payer stages:
  1. the counterparty funds B escrow under its own signed intent;
  2. the owner funds A escrow under its own;
  3. conditional release from contract custody.

  Release from contract custody needs a new custody payer kind. That is unimplemented and must be designed, not assumed.

**EARS:**
- **WHILE** escrow is funded and recipient acceptance or the authenticated document predicate is unmet, release **SHALL** refuse, and the funds **SHALL** remain in custody.
- **WHEN** both conditions hold, release **SHALL** deliver ≥ 20 B to the owner. The owner's total outlay **SHALL** be ≤ 11 A including ≤ 1 A fees, with the exact recipients and domain.
- **IF** the deadline passes, **THEN** only the separate recovery authority **SHALL** refund, and a release racing a committed refund **SHALL** be rejected.
- **IF** a fallible phase fails, **THEN** the guaranteed-phase fee **SHALL** be recorded, and the duties **SHALL** be retained.

**Hostile cases**, each paired with a positive control: extra fee, wrong asset, wrong recipient, wrong domain, a document digest substituted for the predicate, timeout treated as delivery, erased debt, a hidden reservation, and rollover resetting a budget. Two independently authored candidates use different authorized routes and receive identical treatment (UNI-004).

**Accept:** T1 reference first; T2 and T3 under admission. Recovery state-cap tests use the existing caps.

### S9: U6 financial conformance and libraries

- **Parents:** U6, M11, M14, M15, MC07, SP07/08/11, UNI-014/020/023.
- **Prerequisites:** S2 numeric profile.
- **Owned interfaces:** library examples, the conformance inventory, manifests.

**EARS:**
- **WHEN** the conformance inventory is generated, it **SHALL** validate membership against frozen manifest hashes: 277 ACTUS fixtures and fields, 18 types, 32 dispositions, DS01–07, 72 DeFi rows, DA01–24, supplements, 3 held-outs, 8 intent cases, 8 regression classes.
- **IF** any member is missing, **THEN** the inventory **SHALL** name it, and no total **SHALL** be reported as complete.
- **WHEN** a library port converts price orientation, it **SHALL** convert exactly before directed rounding (UNI-014).
- A port that rounds the reciprocal of an already-rounded value **SHALL** be rejected by the oracle comparison.

**Accept:** T1 semantic evidence per cell, with the native and target columns kept separate. Capacity Exchange (M11) and batch clearing (M14/M15) are explicit rows.

### S10: U4 history and composition (scoped first, full later)

- **Parents:** U4, MC03, MC05, MC06, SP06, SP09, SP10, ZR02/04/05/07–10/14–16, MNR02/04/07 (by ID), the five composition operators, UNI-009/010/015/017.
- **Prerequisites:** S5, S8 lifecycle invariants.

**EARS:**
- **WHERE** history is ledger-induced, every report **SHALL** label it ledger-induced and **SHALL NOT** mark MC03 or MC06.
- **IF** a fabricated genesis, arbitrary verifier, repeated parent resource or finalization omission is presented, **THEN** verification **SHALL** reject. The positive control is a legitimate two-step chain.
- **WHEN** released recursion interfaces are inspected, a feasibility record **SHALL** list the actual instructions and costs. **IF** they are unavailable, **THEN** MC03 **SHALL** remain blocked, with no relabeling.
- Each of the five operators (sequential, disjoint parallel, shared-state interleaving, atomic synchronization, asynchronous messaging) **SHALL** have a formal model with a positive trace and an interference-violating trace that is rejected.

**Tier:** formal (T1); native (T2) only on released interfaces.

### S11: U5 optional federation and solvers (never required for direct use)

- **Parents:** U5, M16, M17 (Fenrir committee blocked), K3–K5, MPLR-030–035, UNI-011–013, ZR15.

**EARS:**
- **WHEN** concurrent delegated requests of 6 and 4 units including fees share a 10-unit budget, both **SHALL** reserve.
- **IF** requests of 6 and 5 units use copied parent budgets, **THEN** at least one **SHALL** be rejected or deferred.
- **IF** proof for X is used to sign Y, **THEN** the adapter **SHALL** refuse. The positive control is X signed with proof for X.
- **WHEN** a paid request retries result retrieval, no second charge **SHALL** occur.
- An epoch migration **SHALL** preserve duties and consumed authority.
- Two independent solvers on the S8 program **SHALL** receive identical objective treatment, with no federation required.

**Tier:** T1 reference first. Committee identity stays unknown.

### S12: U7 independent developer release

- **Parents:** U7, M10, MC08, SP12, G01–G24, UNI-001/016/024.
- **Prerequisites:** S2, S3 (SEC-19), S6.

**EARS:**
- **WHEN** a developer installs from public artifacts on a clean machine, a single resolved `mori` executable **SHALL** identify its profile and consumers. The two-executables-named-`mori` ambiguity **SHALL** be resolved.
- **WHEN** two independent builders build the same tag, the compiler and emitted artifacts **SHALL** be byte-identical, or the differences **SHALL** be listed.
- **WHEN** an external developer authors a non-fixture program and submits it from their own wallet to stagenet, effects **SHALL** match their signed intent.
- **IF** only a reviewer receipt is removed, **THEN** validity **SHALL** be unchanged (UNI-001).
- Rust LSP and MCP **SHALL** either work or refuse explicitly.

**Accept:** an external developer's transcript and readback. G01–G24 are each recorded individually.

**M-row coverage:**

| M row | Covered by |
|---|---|
| M1 | S4/S5/S6 |
| M2 | S6 (stagenet only; Fenrir open) |
| M3 | S5/S6 |
| M4 | S6 partial (actual DUST); exchange in S9 |
| M5 | S3/S5/S6 |
| M6 | S2/S7 |
| M7 | S3/S6 |
| M8 | S5 controls; external audit still owed |
| M9 | S3/S6 |
| M10 | S12 |
| M11 | S9 |
| M12 | open, needs Fenrir |
| M13 | S8 |
| M14, M15 | S9 |
| M16, M17 | S11; Fenrir blocked |

OSR-1014 is unread; it maps through S3/S5 once read.

---

## 4. QA strategy and coverage

- **Role separation per sprint:** author, oracle author and implementer are distinct. Opus authors every hostile and security case.
- **Oracle independence:** hand derivations plus an exact-arithmetic oracle that imports no Moriarty code, committed before the run (git order is the freeze). Wrapper or API equality counts only as internal agreement (UNI-020).
- **Paired controls:** a rejection counts only when its positive control in the same pipeline and run passed. Malformed-envelope rejections are recorded but never counted as semantic rejections (ROADMAP "Immediate delivery brief").
- **Boundary pairs:** every cap and range has an at-bound accept and a one-past reject.
- **Coverage records:**
  - `predicates.jsonl` with separate implementation, evidence-class and acceptance axes (AUDIT-PLAN);
  - grammar × consumer cells (S2);
  - field × C/H/P cells (SECURITY-REVIEW §3);
  - frozen-manifest membership (S9);
  - composition operators (S10).

  Unknown cells stay visible. No percentage is computed.
- **Formal, native and target layers stay separate:**
  - Quint, K and Lean/Clean results are *formal developer QA*.
  - Rust T1 results are *local*.
  - T2 results are *native-local*.
  - T3 results are *target-observed* per network.
  - Model traces count toward implementation correspondence only after being replayed through Rust.
- **Regression:** all T1 tests run on every PR. Nothing is "carried forward".

## 5. Factory workflow, parallelism and bottlenecks

- **Flow:** a small PR goes to `main`. T1 CI runs. One independent non-author review follows; Opus also reviews any PR touching authority, signing, keys, nonces, domains, custody or native paths. The integration owner merges. `main` stays T1-green.
- **The native lane** is serialized: packet, then user admission, then a plain script run, then a `RESULT.md` PR.
- **Parallel lanes (disjoint files):**
  - (a) S1 tooling;
  - (b) S3 security hardening;
  - (c) S4 programs and oracles;
  - (d) S2 matrix;
  - (e) S7 Sherlock models;
  - (f) S9 inventory;
  - (g) S12 developer tooling;
  - (h) native/stagenet packet preparation, which is documents only until admitted.

  One integrator owns the shared IR, ABI and statement interfaces (SEC-02 surface).
- **Bottlenecks:**
  - the stagenet facts F1–F11 (external latency, so request them first);
  - native admission and host capacity (one operator; F12 unmeasured);
  - Opus security review capacity;
  - the shared-interface integrator;
  - the decision on canonical stack F14.
- **Not built:** no scheduler, campaign framework or receipt-review layer, and no application-specific compiler.

## 6. Risks, disagreements, unknowns and next three deliverables

**Risks:**
- The stagenet ledger, ZKIR or SRS may differ from the pinned SDK (F3/F4). This can invalidate S5 artifacts for S6, so request the facts before S5.
- Non-`compactc` deployment may be unsupported (F6). That would block S6 regardless of compiler quality.
- P-B's k is unknown and could approach 19.
- Signing-key or seed leakage during T3.
- SEC-01 time: the in-circuit window is prover-asserted.
- Over-claiming from historical evidence across trees or stacks.

**Disagreements to resolve:**
1. ROADMAP's line "Foreman/Pel governs project implementation only" conflicts with the NO FOREMAN override. Recommend replacing it with §4.
2. ROADMAP says "Lean is not a … roadmap workstream", while the Sherlock design calls Lean/Clean a "required stage". Recommend stating that Lean/Clean is optional developer QA and that no acceptance depends on it.
3. The UNI-024 routing names GPT-6.1 Sol and Muse; the current request uses Gemini4Argon, Opus and Astra. The routing clause needs updating. Security stays with Opus.
4. Whether stagenet must use Sherlock (F16).
5. Canonical stack F14.
6. The stale aux/2 "UNRUN" wording.
7. The unclear merge status of continuations.
8. The undefined "C4" label.

**Next three concrete deliverables:**
1. **S1 PR:** feature-gated native entry points, the T1 CI job, the whitespace-aware trace parser with fixtures, the pinned `timeout`, and a one-time adjudication record for the existing phase-1 capture.
2. **Stagenet facts request and draft T2/T3 packet (documents only):**
   - questions F1–F16 addressed to the named owners;
   - a draft packet per §2.3 for P-A, containing no invented values;
   - submitted to the user for decision.
3. **S4 PR, with S3 in parallel:** P-A and P-B sources, independently frozen oracles committed first, T1 simulate, test and static export/reopen, and the N-1 refusal control.

No native execution, resource reset or acceptance is created by this proposal.
