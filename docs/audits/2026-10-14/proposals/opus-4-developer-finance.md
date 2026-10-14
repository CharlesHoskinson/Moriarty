# Opus brainstormer 4: Developer product and financial conformance audit design

**Advisory proposal only.** This designs an audit. It is not a completed audit, a product approval, native qualification or milestone acceptance. Frozen source: `cb8280ef`. Native authority stayed at zero. I used Read/Grep/Glob only and executed no commands, tests or native actions. I read no peer proposals. Every runtime statement below is labelled **observed-in-source** (I read the code), **recorded** (a historical receipt says so; not rerun) or **unknown/inferred**.

## 0. Anchoring observations from inspected source

These shape the proposal. Each one was checked in source, not taken from the Luna maps.

- **Four authoring entry points; only one reaches ZKIR.**
  - `mori check` sends a file to Core only if its first token is `language` (`crates/mori/src/commands/check.rs:29`). Anything else goes to the Beta parser/checker.
  - `mori compile` loads Core `moriarty/1` only and calls `compile`/`compile_signed` (`compile.rs:19-27`).
  - The Node package bridges Beta source to Source/6 by importing straight from `experiments/` (`packages/moriarty-beta/src/bridge.ts:5-6`). That path ends in `PreparedUnqualified` (`cli.ts:18`) and produces no ZKIR.
  - README describes yet more profiles: source/5 and bounded-atomic/1 (`README.md:67`).
- **Editor/MCP tooling does not cover the compile-capable profile.**
  - Rust `mori lsp` and `mori mcp` are `todo!()` (`lsp.rs:9`, `mcp.rs:9`). The help text still advertises them (`cli.rs:151-154`).
  - The Node LSP/MCP tools only check, inspect, expand and preview Beta source (`servers.ts:116-119`).
  - Both the Node package and the Rust crate install a binary named `mori` (`package.json:9-10`; `crates/mori/Cargo.toml:2` plus `src/main.rs`).
- **Core's effect vocabulary is narrower than the contract's definition of complete effects.**
  - `Effect` has only State, Transfer, Fee, Liability and Emit (`crates/mori-core/src/ir.rs:214-244`).
  - The contract requires authorized mint/burn supply changes and phase-specific failure accounting (`docs/MORIARTY-PRODUCT-CONTRACT.md:43,47`).
- **Owner-intent authority covers one signer only.** `check_effects` caps gross, fee, net and liability only for effects whose `from` or `debtor` is the signer (`crates/mori-core/src/intent.rs:264-277, 286-347`). Debits from other parties are not authority-checked there. The PR49 mapped-role mode is labelled `...-unauthenticated` (`crates/mori-ledger/src/static_compile.rs:22`), and its README disclaims debit authority (`examples/core/fee-pool/README.md:111-115`).
- **The numeric policy does not cover the native profile.** The U0 numeric decision covers only `experiments/.../successor/` primitives (`docs/decisions/u0-numeric-profile-decision.md:26`). Its canonical price orientation is base-per-quote (`:7`). `examples/core/batch-clear/batch.mori:13-18` computes quote = qty × price, a quote-per-base usage. Core has no `Price` type, so whether D1 even applies there is **unknown**.
- **Recovery accounting rests on trusted input.** Sponsor settlement takes "an observation authenticated by the trusted caller" and does not query a network (`crates/mori-core/src/source_sponsor.rs:302-310`). Unknown finality keeps the full hold (`:294-297`).
- **Library inventory.**
  - No Capacity Exchange or ACTUS code exists under `crates/` or `examples/`. Capacity Exchange is only mentioned in the FeePool README.
  - ACTUS material appears only under `experiments/`.
  - Retained denominators: 277 ACTUS fixtures across 18 executable types, 32 taxonomy dispositions, 72 DeFi rows and DS-01–07 (`openspec/changes/mc07-.../spec.md:11,41,92`).
- **The M1–M17 exit text lives only in a proposed design.** It is in the Oct 10 Fenrir design (`docs/superpowers/specs/2026-10-10-fenrir-integration-design.md:76-92`). `ROADMAP.md` mentions the rows once (`:26`), and its U-table (`:45-52`) does not map them.
- **The structural map skipped shipped code.** It excluded `experiments/**` from content reads (`structural-graph/coverage.json:57`), but the published Beta package imports code from there. The maps' frontend coverage of the shipped package is therefore incomplete.
- **A known output limit.** The aggregate `mori test` JSON overflowed its 1 MiB bound after evaluating 123 cases (`examples/core/fee-pool/cases/COVERAGE.md:13`) (recorded).

## 1. Recommended approach and alternatives

**Recommended: a predicate ledger, driven by two fresh programs that a developer would actually write.**

1. Freeze one obligation register. It must cover U0–U7, M1–M17, MC01–08, SP01–12, G01–G24, the 277/32/72 ACTUS/DeFi denominators and DS-01–07.
2. Decompose each row into child predicates. Each child carries an independent oracle and an evidence class.
3. Commission two fresh `moriarty/1` programs from an author with no fixture access. Drive them through every stage of the public CLI, from install and editor use through to the native boundary.
4. Each stage either produces an observed result or records a named block (`blocked:native-authority-zero`, `blocked:external-target`).

Completion is reported as per-row fractions of child predicates, with unknowns as an explicit third bucket. This replaces the subjective 60–70%/30–40% estimates.

- **Tradeoff:** It surfaces integration defects that per-PR audits miss, such as the profile split, missing LSP/MCP and effect-vocabulary gaps. The cost is a register maintainer, and much of it will honestly read "blocked".

**Alternative A: an audit organized by component** (frontend, Core, backend, ledger, libraries, tooling), each audited against its own spec.

- **Tradeoff:** It is easier to parallelize and lines up with code ownership. It misses seams between components, such as Beta → Source/6 never reaching ZKIR, and it encourages counting tests.

**Alternative B: a pure documentation and obligation reconciliation** (roadmap/README against traceability), with no fresh programs.

- **Tradeoff:** It is cheap and immediately useful, since it would catch the stale PR49 entry and the M-row authority gap. It cannot observe any behavior and risks the "consensus as evidence" failure the brief forbids.

## 2. Exact audit outputs

1. **`audit/obligations.jsonl`**: one record per child predicate (schema below). Every U/M/MC/SP/G row must have at least one child, and no row may be dropped.
2. **`audit/chain-runs/<program>/<stage>.json`**: the chain stage records from §3, including exact command, exit code, stdout/stderr digests, artifact digests and the profile used.
3. **`audit/surface-matrix.json`**: install path × command × profile → status. Install paths are npm pack, cargo build and the docs tutorial. Commands are check, compile, simulate, test, intent, verify-intent, compile-call, lsp, mcp and editor activation.
4. **`audit/accounting-ledger.json`**: per program and action, the gross, fee, net, custody, liability, mint/burn and phase-failure columns. Each column records its oracle and the reason for any omission.
5. **`audit/conformance-denominators.json`**: for each of the ACTUS 277/18/32, DeFi 72 and DS-01–07 counts, which items are semantic, native, local or Preview covered, which are absent, and which are only specified.
6. **`audit/ROADMAP-AUDIT-REPORT.md`**: the human report. It covers per-row child fractions, observed defects, blocked experiments, recommended edits and dissent.

**Child predicate schema.** Required fields: `id`, `parent_rows[]`, `predicate` (testable text), `profile` (`beta|moriarty/1|source/5|source/6|native-signed|...`), `stage`, `evidence_class` (`source-inspected|specified-only|historical-run|current-run|native-run|externally-accepted`), `status` (`satisfied|refuted|partial|unknown|blocked`), `block_reason`, `oracle` (`{kind, provenance, independent_of_impl: bool}`), `citations[]` (`path:line@commit`), `adversarial_controls[]`, `external_dependency` (for example `fenrir-network-id`), `not_claimed[]`.

Candidate record:

```json
{"id":"M10.c3","parent_rows":["M10","MC08","U7"],
 "predicate":"`mori mcp` serves read-only tools for the moriarty/1 profile over stdio",
 "profile":"moriarty/1","stage":"tooling","evidence_class":"source-inspected",
 "status":"refuted","block_reason":null,
 "oracle":{"kind":"source-inspection","provenance":"crates/mori/src/commands/mcp.rs:9","independent_of_impl":true},
 "citations":["crates/mori/src/commands/mcp.rs:9@cb8280ef","packages/moriarty-beta/src/servers.ts:116@cb8280ef"],
 "adversarial_controls":[],"external_dependency":null,
 "not_claimed":["runtime panic observed","Node beta MCP correctness"]}
```

## 3. The end-to-end chain with fresh and contrasting programs

**Program authorship.** An independent author receives only public docs (`docs/NATIVE-CORE.md`, the language reference and the CLI `--help`). The author gets no access to `examples/` or tests. The author writes two programs:

- **P1 (fresh): a single-payer collateralized credit line.** Actions: open, draw (partial), accrue (ceil), repay (partial), close, and recover after deadline. It has one owner-debit asset, a liability with opening + accrual − discharge = closing, and a fee to a distinct recipient. It is designed to fit the existing single-payer legacy and signed paths, so it can travel the furthest down the native-blocked chain.
- **P2 (structurally contrasting): a two-asset, multi-party conditional exchange with a rebate.** It differs from P1 on at least five machine-checked axes:
  - payer count >1 (exercises the `one-public-payer` refusal and mapped-role mode);
  - no liabilities;
  - witness quotient/remainder division;
  - Bool phase state;
  - a refusal branch that must leave no effects.

  Record the contrast vector in the run record. P2 must also differ from FeePool's contrasting example, which reuses the same arithmetic (`fee-pool/README.md:145-158`).

The independent oracle author fixes expected states, effects and refusals by hand-derived arithmetic **before** any execution. Expected numbers must not be produced by `mori simulate`.

Stages and their pass predicates:

| # | Stage | Command / API | Pass predicate | Current expectation |
|---|---|---|---|---|
| 1 | Install | npm pack (no network), cargo build, docs tutorial | Which `mori` ends up on PATH; does the packed tarball include the `experiments/` imports? | **Unknown.** Collision inferred |
| 2 | Author/editor | Node LSP, Rust `mori lsp` | Diagnostics for `moriarty/1` | Rust: `todo!()` (observed). Node: Beta only |
| 3 | Typing/Core | `mori check --json` | Accepts the program; one hostile mutation per judgment is rejected | Runnable locally |
| 4 | Intent | `mori intent` / `verify-intent` | Network, anchor, nonce and limits are bound; wrong-signer, expired, high-S and asset-swap variants are rejected (`intent.rs:204-249`) | Runnable locally |
| 5 | Effects | `simulate`, `test` | Full post-state and ordered effects equal the independent oracle; check gross/fee/net per `check_effects` | Runnable locally |
| 6 | ZKIR | `compile --target` and `compile-call` (unsigned, signed-v2, mapped) | Artifacts reopen exactly; P2 is refused by legacy and signed modes | Runnable locally (source-static) |
| 7 | Prove | `prove-call` / `prove-signed-call` | Real proof; hostile witnesses rejected | **blocked: native-authority-zero** |
| 8 | Verify | `verify-call` (Real verifier) | Tampered statement, key or proof rejected | **blocked** |
| 9 | Deploy/call | Fenrir/Preview | Readback of ledger effects equals stage 5 | **blocked: external target.** Do not invent network ID or anchor |
| 10 | History/recovery | Sequence replay; unknown finality, late success, timeout | Residual duties survive; holds persist | Local reference only. Authenticated history open |

The source-to-ZKIR correspondence check at stage 6 compares the stage-5 Core trace against the reference replay produced by `simulate --trace --target` (`docs/NATIVE-CORE.md:70`). This is a differential check, not a correspondence proof.

## 4. Highest-risk gaps

| Gap | Class | Basis |
|---|---|---|
| No single "author → ZKIR" product path. Beta/Node tooling stops at `PreparedUnqualified`; the compile-capable `moriarty/1` has no LSP or MCP | **Observed** | `check.rs:29`, `bridge.ts:5-6`, `lsp.rs:9`, `mcp.rs:9`, `servers.ts:116-119` |
| Core effect vocabulary lacks mint/burn and guaranteed/fallible phase effects that the contract requires | **Observed** (vocabulary). Impact on any specific program: **inferred** | `ir.rs:214-244`, product contract `:43,:47` |
| Multi-party debit authority is absent. Intent covers the signer only; mapped roles are unauthenticated | **Observed** | `intent.rs:264-277`, `static_compile.rs:22` |
| Numeric policy (orientation, rounding direction, dust) is scoped to experiments/source/5, not `moriarty/1` | **Observed** scope; violation **inferred** | decision `:26`; `batch.mori:13-18` |
| M1–M17 exits rest only on a proposed design | **Observed** | Fenrir design `:76-92`; `ROADMAP.md:26` |
| Published npm package depends on `experiments/` source | **Observed** import; tarball completeness **unknown** | `bridge.ts:5-6`, `package.json:18-21` |
| Two `mori` binaries can collide on PATH | **Inferred** | `package.json:9-10`, `Cargo.toml:2` |
| Recovery and unknown-finality settlement trust caller observations | **Observed** | `source_sponsor.rs:302-310` |
| Aggregate test report overflows 1 MiB, which limits user-facing qualification at scale | **Recorded** | `COVERAGE.md:13` |
| ACTUS/DeFi: none in the compile-capable tree | **Observed** absence in `crates/` and `examples/`. Coverage elsewhere: **unknown** | grep result; mc07 spec `:92` |
| Large Quint sampling verdict | **Unknown** (timed out) | FeePool README `:170-171` |

I make no claim that any test passes today. The PR49 counts (162 cases, 47 tests and so on) are **recorded**, not rerun.

## 5. Preserving retained obligations, with child statuses

Every row stays in the register, and the parent status is derived from its children. A parent never reads "satisfied" while any child is unknown or blocked. Child statuses I can support from source:

- **M1:**
  - `M1.c1` compile command exists for `moriarty/1`: source-present.
  - `M1.c2` two independently authored programs, no fixture lookup: not yet done (§3).
  - `M1.c3` outputs checked against an independent reference: unknown.
- **M10:**
  - `simulate`, `test` and `verify-intent` are source-present (`cli.rs:141-150`). The historical "all stubs" wording should be retired once a current run confirms them.
  - `mcp` and `lsp` are refuted in Rust.
  - Differential Beta fixtures: unknown.
- **M11:**
  - FeePool finite component: recorded local evidence, partial.
  - Capacity Exchange: absent.
  - Native tests and integration readback: blocked.
- **M4 and M16:**
  - Local sponsor and metering exist: partial, unauthenticated.
  - DUST: blocked by an external dependency.
- **M13:** `escrow-recovery` covers local partial fill, recovery and residual duty (`escrow.mori:110-136`) through supplied observations: partial. Authenticated recovery authority: open.
- **M14/M15:** `batch.mori` shows a uniform price with a caller-supplied oracle (`:10`): partial, with no oracle authority, liquidation or solver bound beyond a cap input.
- **M2/M3/M5/M7/M9/M12/M17:** target-qualified children are blocked by an external dependency (Fenrir has no versioning yet). Only local binding children can be assessed. For example, `Authorization` already carries `network`, `anchor` and `target_hash` (`intent.rs:30-43`).
- **U6 and MC07:** the full denominators stay open. Partial FeePool or escrow work does not reduce them.
- **G01–G24:** stay `unassessed`, pointing to SP12.3 (`openspec/sprints/legacy-release-gates.json`).

## 6. Workstreams, dependencies and done criteria

| WS | Scope | Depends on | Parallel-safe |
|---|---|---|---|
| W1 | Register freeze: all rows → children; M-row authority disposition | — | Yes |
| W2 | Independent oracle authoring for P1/P2 (hand arithmetic, hostile list) | W1 skeleton | Yes, but isolated from W3 |
| W3 | Fresh author writes P1/P2 from public docs only | — | Yes, isolated |
| W4 | Surface audit: install/pack, PATH, docs tutorial, editors, LSP/MCP, help text | — | Yes |
| W5 | Local chain stages 3–6 for P1, P2, FeePool and escrow; differential reference replay | W2, W3 | After W2/W3 |
| W6 | Accounting ledger: custody, gross/net, liability, mint/burn, phase columns | W5 | Partly |
| W7 | Conformance denominators: ACTUS, DeFi, DS inventory across experiments and the native tree | W1 | Yes |
| W8 | Native stages 7–10: **specified-only** run plans with exact predicates | Separate admission | Blocked |

**Done** means:

- every row has at least one child with an evidence class and citation;
- P1 and P2 have stage records for 1–6, plus explicit blocked records for 7–10;
- every accounting column is populated or carries a reasoned omission;
- denominators are reported with unknowns listed separately;
- two fresh non-author audits have reviewed the exact report.

**Immediately achievable** without native authority (ordinary local commands, run by the root or another session with execution rights): `npm pack --dry-run` file listing; `cargo build`; `mori --help`; `mori check`, `compile`, `compile-call`, `simulate`, `test`, `intent` and `verify-intent` on P1 and P2; running `mori lsp` to confirm the panic.

**Blocked:** prove, verify, keygen, deploy, call, DUST, Fenrir binding, and authenticated history.

## 7. Roadmap updates

**Recommended now** (small, factual):

1. Add the PR49 entry to "What exists". Describe it as a finite FeePool source component plus unauthenticated mapped-role source-static export, with no native, custody or M11 claim. Luna evidence flagged this as stale by omission.
2. Add an M1–M17 → U crosswalk table to `ROADMAP.md` with the status "exit text from the Oct 10 proposed design; controlling authority pending user confirmation". Do not change the exits.
3. Add one "Developer surface" line. Name the canonical compile-capable profile (`moriarty/1`), state that Rust `lsp`/`mcp` are unimplemented, and state that Node tooling covers Beta/Source/6 preparation only.
4. Replace the percentage estimates with "per-row child-predicate fractions pending audit".

**Deferred pending evidence:**

- Choosing a single public authoring profile. This needs a design decision with two agreeing votes.
- Adding mint/burn and phase effects to Core.
- Extending the numeric profile to `moriarty/1`.
- Changing M10 status.
- Any re-sequencing of M11/M13–M15.
- Any change to the U2 status.

## 8. Dissent, open questions and claims this review cannot support

- **Dissent.** "U2 via a fresh program" is ill-posed until one public profile is designated. Today a developer following the editor/MCP path cannot reach ZKIR. Make profile designation a U0 child, not a U7 polish item.
- **Dissent.** The single-signer intent model (`intent.rs:264`) may be structurally inadequate for M14/M15 and FeePool-style multi-payer programs. Treating mapped-role export as progress toward custody risks a dead end. Rule on this before M11/M14 work continues.
- **Open questions.**
  - Does the packed npm tarball bundle the `experiments/` imports?
  - Which `mori` wins on PATH under the tutorial?
  - Does `mori compile` (as opposed to `compile-call`) accept multi-payer `batch.mori`?
  - Is D1 price orientation meant to bind `moriarty/1` UInt arithmetic?
  - Is there a user-approved record making the Oct 10 M-exits controlling?
- **This review cannot support** any runtime success, the absence of any test failure, native proof or verification soundness, ledger or Fenrir acceptance, ACTUS/DeFi coverage levels, security of the signature path, or that any U/M/MC/SP/G row is closed or nearer closure. Test counts, hashes, Luna graphs, PR approvals and the 74 archived drafts are not evidence of correct behavior here.
