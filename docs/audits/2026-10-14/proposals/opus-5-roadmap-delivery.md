# Opus brainstormer 5: audit design for roadmap and delivery structure (advisory)

**Scope.** This is an audit design, not a completed audit. It is not milestone acceptance, native qualification or product approval. I used Read, Grep and Glob only on frozen tree `cb8280ef` and the Luna maps. I ran no tests, native actions or commands. Every claim below is labelled **observed** (I read the source at the cited line), **inferred**, or **unknown**. Nothing here shows that anything succeeds at runtime.

---

## 1. Approach

### Recommended: build an obligation register first, then walk two new programs through the chain

1. **Build one register of every retained obligation.**
   - It covers U, M, MC, SP, G, L, ZR, MNR and MPLR obligations plus the ACTUS/DeFi denominators.
   - Each parent obligation is split into child capability predicates. Each child names one stage, one profile, one target and one evidence class.
2. **Test the register against a real chain walk.** Two newly authored programs with contrasting structure go through every stage of author source → Core → intent → ZKIR → proof → verification → deploy/call → effects/history/recovery.
3. **Derive roadmap edits from the register.** Edits come only from register differences, never from narrative or from vote counts.

Why this fits the problem: broad milestones hide child capabilities (Luna requirements map, lines 20–25). The roadmap has no item-by-item M1–M17 status (see R1 below).

### Alternative A: walk the chain only

- **For:** fastest way to find integration breaks such as profile splits or two compile paths.
- **Against:** it never touches U5/U6/G obligations or L-row duties that sit off the chain. A green local walk also invites over-crediting whole M rows.

### Alternative B: re-review each milestone in turn

- **For:** it maps directly onto the existing tables, so reviewers find it familiar.
- **Against:** each milestone is reviewed as a whole, which repeats the "percent of a milestone" mistake. It duplicates the MC package specs and runs slower. It also cannot cleanly separate shipped capability from accepted milestone.

---

## 2. What the audit must produce

### Files

| File | Purpose |
|---|---|
| `obligations.json` | One record per parent obligation and per child predicate (schema below) |
| `chain-walk.json` | One record per (program, stage, variant): command, inputs digest, the oracle's expected result, observed result, evidence class |
| `evidence-index.json` | Each cited receipt or source location → evidence class (`source-observed`, `specified-only`, `historical-run`, `current-local-run`, `native-local`, `externally-accepted`) and the exact tree it describes |
| `blockers.json` | Each blocker: external vs. implementable, owner, the predicate it blocks, and what would unblock it |
| `progress.json` | Per-parent counts of children by status. **No percentages, no weights.** A parent is accepted only when every child is accepted. |
| `ROADMAP-AUDIT.md` | Human report. Sections: findings by risk, stale statements with line references, roadmap diff (applied now vs. deferred), unknowns, dissent |
| `roadmap-diff.patch` | The proposed edits only. Applied separately, after review. |

### Candidate schema example

```json
{"id":"M7.c2","parent":"M7","u_owner":["U1","U2"],"mc":["MC05"],
 "predicate":"signed envelope binds Fenrir networkID+Anchor; cross-Anchor replay rejects",
 "profile":"moriarty-jubjub-owner-intent/1","target":"fenrir-devnet",
 "stage":"intent","status":"blocked-external",
 "evidence_class":"source-observed",
 "evidence":[{"path":"crates/mori-ledger/src/layout.rs","line":154,
   "observation":"static bindings require anchor == \"local-unqualified\""}],
 "blocker":"no authenticated Fenrir network ID/Anchor (do not invent)",
 "oracle":null,"claims_not_supported":["M7 acceptance","replay safety"],
 "source_tree":"ce05a5c7…","auditor":"…","reviewed_by":[]}
```

### Allowed status values

`accepted` · `local-evidence-captured` · `source-observed` · `stub-observed` · `specified-only` · `blocked-native-admission` · `blocked-external` · `unknown` · `contradicted`

`accepted` requires the exact roadmap exit predicate plus fresh independent result audits. An approval vote alone never sets it.

---

## 3. The end-to-end chain test

### The two programs

The programs must be newly authored, outside the FeePool and loan/swap fixtures. Program text and an independent expected-outcome oracle come from different agents. The oracle is frozen before anything runs (develop skill, `SKILL.md:67`).

| | P-A: two-party conditional escrow | P-B: single-payer metered stream |
|---|---|---|
| Payers | Two debit roles | One payer |
| Duties | Fee, persistent liability | None |
| Flow | Partial fill, recovery branch, late-success-vs-refund race | Repeated settlements |
| Arithmetic | — | Directed floor/ceil rounding at boundaries; base-per-quote price |
| Assets | — | Distinct asset identity triple |

P-B is chosen to contrast with P-A on payer count, duties, recovery and arithmetic. Each program also gets hostile variants:

- wrong asset, recipient or domain
- erased debt
- extra fee
- replayed nonce
- stale predecessor
- reordered or omitted effects
- a valid proof bound to the wrong source or key

### Stages

| Stage | Entry point (observed in `crates/mori/src/cli.rs:105-155`) | What counts as evidence | Can run now? |
|---|---|---|---|
| S0 author/check | `mori check` | Diagnostics match the oracle | Static/local: yes |
| S1 Core | `expand` | Core IR equals the oracle's lowering | Local: yes |
| S2 simulate/test | `simulate`, `test` | Full post-state, ordered effects and liabilities equal independent expectations | Local: yes |
| S3 intent | `intent`, `verify-intent` | Signed bytes bind the complete scope. Hostile variants reject. | Local: yes |
| S4 ZKIR | `compile` **and** `compile-call` | The relation constrains what the oracle requires | Local static: yes |
| S5 keygen/prove | `keygen-*`, `prove*` | A real proof exists | **Blocked: native admission** |
| S6 verify | `verify*` | Real verification, plus hostile-witness rejection | **Blocked: native admission** |
| S7 deploy/call | (none) | Ledger acceptance | **Blocked externally; no CLI verb** |
| S8 effects readback | — | Chain effects equal the reference | Blocked |
| S9 history/recovery | `replay` | First-divergence check on public records | Local diagnostic only |

**Rule:** a stage blocked upstream records `unknown` downstream, never `pass`. A rejection caused only by a malformed envelope does not count as a semantic rejection (`ROADMAP.md:64`).

---

## 4. Highest-risk gaps found in source

**R1. M1–M17 are missing from the delivery sequence and the crosswalk. (Observed)**
- `ROADMAP.md` mentions M1–M17 once, as "remain open" (line 26). Its U-table (lines 43–52) has no M rows.
- `traceability.md` maps P/C/K/MC/SP (lines 9–47) but no M rows.
- The detailed M exits live in `docs/superpowers/specs/2026-10-10-fenrir-integration-design.md:74-92`. That file is marked "implementation design for user review" (line 12), but also carries an "Approved scope clarification" (lines 1–8). Which one controls is **unknown**. M rows can therefore drift with no owner.

**R2. L1–L8 have no owner in the crosswalk. (Observed)**
M11/M13/M16 cite L-rows. Those rows are defined only in `docs/superpowers/specs/2026-09-11-defi-kernel-multichain-design.md:105-112`, and `traceability.md` has none.

**R3. The execution target is contradictory. (Observed)**
- `AGENTS.md:100` says "Preview is the sole public execution target".
- MC02 is defined as Preview (`openspec/changes/mc02-preview-financial-operation/README.md:1-3`).
- The Fenrir design says M1–M17 select its scope "over … Preview-only target restriction" (line 12), and M3 maps to MC02.
- No disposition says whether Preview evidence and Fenrir-devnet evidence substitute for each other. **Inferred:** they cannot.

**R4. There are two compile paths with unknown equivalence. (Observed)**
- `compile` goes through `mori_backend::compile` / `compile_signed` (`compile.rs:23-27`).
- `compile-call` goes through the `mori-ledger` static path (`static_compile.rs:18-22`).
- Which one M1/U2 "general" refers to is **unknown**. So is which source profile U2 admits: the Luna frontend map shows Beta, Core and Source/6 entry points.

**R5. State and time are supplied by the caller. (Observed)**
- `verify-intent` takes `--at` "logical time" as an argument (`verify_intent.rs:20-21`).
- Scenario JSON supplies local state (`cli.rs:63-65`).
- The signed path says "not … key-to-account authority or ledger acceptance" (`verify_intent.rs:156`).
- Authenticated state, time and history are **absent** on the inspected paths.

**R6. The toolchain pin likely differs from the target. (Inferred)**
- The workspace pins ledger rev `04c9c5d9…` (`Cargo.toml:23-32`, `crates/mori-ledger/src/profile.rs:7`).
- The design records the devnet ledger as `6abe9b16…` (line 19). That is a document claim; I did not verify it live.
- M2 is blocked until a target tuple is established.

**R7. Mapped-role mode exists only on the static path. (Observed)**
- Relaxed debit rules are private to "the distinct source-static view" (`layout.rs:143-148`, `215-222`).
- The proving path keeps the single-payer rule (`layout.rs:206-213`).
- Static bindings require anchor `"local-unqualified"` (`layout.rs:154`).
- So generic mixed-payer native custody and Fenrir Anchor binding are absent on the inspected path.

**R8. Process documents give conflicting review rules. (Observed)**
- `PARALLEL-DEVELOPMENT.md:260` and `SKILL.md:21` still require Sol+Grok final audits.
- `AGENTS.md:5-6` and `PARALLEL-DEVELOPMENT.md:3-4` say Muse replaces Grok.
- Acceptance records could be ruled invalid on routing grounds alone.

**R9. Roadmap status is stale. (Observed)**
- The latest delivery observation is dated 2026-10-11 (`ROADMAP.md:13-29`). PR45–49 are not mentioned.
- The design's M10 baseline says simulate/test/signing are "placeholders" (line 16). The CLI now dispatches `simulate`, `test` and `verify-intent` (`cli.rs:142-150`). Rust `lsp`/`mcp` are `todo!()` (`lsp.rs:9`, `mcp.rs:9`).
- Whether M10 behaves correctly is **unknown**.

**R10. Coverage and oracle blind spots. (Observed)**
- `crates/mori-core/src/lib.rs` yields no graph symbols, and `.mori`/`.qnt` files are not parsed (`coverage.json:23-24, 45-50`).
- The aggregate `mori test` report overflows its 1 MiB limit (`examples/core/fee-pool/cases/COVERAGE.md:13`).
- Some successor tests compare the CLI with the same API they wrap (Luna verification map). That shows CLI/API agreement, not correct semantics.

---

## 5. Keeping every obligation

The register must list every item below verbatim, with source path and line:

- **U0–U7:** `ROADMAP.md:45-52`
- **M1–M17:** the brief's rows, plus design lines 76–92
- **MC01–MC08:** package READMEs and their EARS specs
- **SP01–SP12:** plus `openspec/sprints/package-task-map.json`
- **G01–G24:** 24 IDs in `openspec/sprints/legacy-release-gates.json`, each "recordedHistoricalStatus: unassessed", primary closer SP12.3
- **L1–L8**
- **ZR01–16 and MNR01–08:** `traceability.md:118-150`
- **MPLR-001–035**
- **ACTUS/DeFi denominators and the five composition operators:** `traceability.md:49`

**Rules:**
- No row is deleted, merged or weakened without a user decision record.
- A parent status is derived from its children, never asserted.
- Preview and Fenrir targets get separate child rows.

### Proposed initial child statuses

These are to be confirmed by the audit; they are not verdicts.

| Child | Proposed status |
|---|---|
| M10.simulate / test / verify-intent | `source-observed` (behavior unknown) |
| M10.lsp / mcp (Rust) | `stub-observed` |
| M11.feepool-finite-source | `local-evidence-captured` (PR49 receipts; source-scope audit only) |
| M11.capacity-exchange | `specified-only` |
| M7.network-anchor-binding | `blocked-external` |
| M2.target-tuple | `blocked-external` |
| M8.jubjub-audit | `blocked-external` |
| M12.per-block-registry | `specified-only` (the design found no authoritative Bus Registry interface, line 24) |
| M17.committee-identity | `blocked-external` (no Fenrir versioning or committee identity; do not invent) |
| M6.reference-replay | `source-observed` (`replay` verb exists) |
| M1.general-compile | `unknown` (R4) |

---

## 6. Workstreams, dependencies and done criteria

| ID | Work | Depends on | Parallel-safe? |
|---|---|---|---|
| W0 | Build the register skeleton from the sources in §5 | — | Root only; serial, first |
| W1 | Crosswalk M/L/target rows | W0 | With W3, W6 |
| W2a | Author P-A/P-B (author agent) | W0 | With W2b, but separate sessions |
| W2b | Write the independent oracle and hostile matrix (separate agent) | W0 | Must never see W2a outputs before freezing |
| W3 | Classify PR45–49 controlled public receipts only (evidence branch `882669a1`) | W0 | Yes |
| W4 | Walk S0–S4 and S9 locally | W2a, W2b frozen; root authorizes ordinary non-native execution | One build process, `-j2` (`PARALLEL-DEVELOPMENT.md:220`) |
| W5 | Write specified-only experiment packets for S5–S8 | W4 | Yes. Packets only; no admission. |
| W6 | Reconcile process and status documents (R8, R9) | W0 | Yes |
| W7 | Draft the roadmap diff and the report | W1, W3, W4, W6 | Serial, last |
| — | Fresh Muse full-candidate audit of the outputs | W7 | — |

### What can run now vs. what is blocked

- **Possible now:** static source reading; register building; crosswalk gaps; authoring the programs and oracle. Ordinary local `check`, `expand`, `simulate`, `test`, `compile` and `compile-call` plus Quint deterministic tests can run *if root authorizes them*.
- **Blocked:** keygen, prove, verify, deploy, call, Preview, Fenrir devnet, live target-tuple checks, k≤19 and 64 KB measurements, recursion.

### Done when

- Every row from §5 is present, with zero orphan IDs. This is checked mechanically by comparing the register's ID set with grep counts over the source documents.
- Every child has a status and an evidence pointer, or is marked `unknown`.
- S0–S4 have been walked for both programs and their hostile variants, compared against the frozen oracle.
- Every blocker is classified as external or implementable.
- `progress.json` is derived purely from the register.
- A fresh audit of the outputs has been completed.

### Critical path

1. U0 target matrix together with the M2 target tuple (external)
2. U1 primitive certificate (needs native admission)
3. U2/M1 general compile and M3 prove/verify/deploy (native + external)
4. U3/M13
5. U4 (recursion; user assumption of around March 2027)
6. U6
7. U7

**Work that can proceed off the critical path:** M1 static, M6, M9, M10, M11-FeePool source, local M13–M16 source, and L-row source work.

---

## 7. Roadmap edits

### Minimal edits recommended now

These are bookkeeping only; none promotes a status.

1. Add an **M1–M17 crosswalk table** to `traceability.md`. Columns: U owner, MC/L, local predicate, target predicate, external dependency. Every status `specified-only` unless evidence exists. Use the user's controlling rows verbatim.
2. Add **L1–L8** rows to `traceability.md`.
3. Add a dated **"Delivery observation — 2026-10-14"** to `ROADMAP.md`. List PR45–49 with their exact qualification strings, e.g. `source-static-mapped-roles-unproven-uninstalled-unauthenticated` (`static_compile.rs:21-22`). State what each does *not* establish.
4. Add a **target disposition note:** Preview (MC02/SP05) and Fenrir devnet (M2/M3) are separate targets, and evidence does not transfer between them. Flag `AGENTS.md:100` for a user decision.
5. Add an **external-blockers list:**
   - Fenrir network ID, Anchor, versioning and committee identity
   - devnet target tuple
   - JubJub audit
   - recursion interfaces
   - Bus Registry interface
6. State that progress is reported **only as per-predicate counts from the register**. Withdraw the subjective 60–70% / 30–40% figures.
7. Replace the historical M10 "all stubs" wording with the observed verb inventory, marked behavior-unverified.
8. Reconcile the review routing in `PARALLEL-DEVELOPMENT.md:260` and `SKILL.md:21` with the October 14 routing. This is a governance document edit.

### Deferred until there is evidence

- Any child or parent promotion.
- Choosing the U2 source profile and compile path.
- Changing the order of the critical path.
- U2 subset labels.
- Any M-row exit rewording.
- Numeric-profile changes.

---

## 8. Dissent, open questions, and claims this review cannot support

- **Dissent:** I would *not* fold M1–M17 into the U table as new milestones. They mix local and external predicates. Putting them in the crosswalk keeps U as the single sequence. Another reviewer may prefer M rows as first-class roadmap rows; that choice needs a user decision.
- **Open questions:**
  - Which document controls the M exits: the Oct 10 design or an approved assignment?
  - Is the canonical U2 path `compile` or `compile-call`?
  - Is Preview still a required target alongside Fenrir?
  - Should the 1 MiB aggregate report limit be fixed, or bounded manifests accepted?
- **Cannot be claimed from this review:**
  - that any stage works at runtime, or that any test passes now
  - native soundness, key or SRS provenance, or settlement
  - completion of M11 or any other M row, or acceptance of any U, MC, SP or G item
  - completeness of Quint sampling
  - that PR49's approval extends beyond its source scope
  - any percentage of progress
