# Opus brainstormer 3: verification strategy and adversarial end-to-end audit proposal

This is an advisory audit design. It is not an audit approval and not native qualification. I read only static source at `cb8280ef`. Native authority stayed zero. I ran no tests or commands. Luna maps were used as leads and were checked against source wherever they are cited below.

---

## 1. Approach

### Recommended: a predicate ledger with independent-oracle differential audit

1. **Break every retained exit into atomic predicates.** This covers U0–U7, M1–M17, MC01–08, SP01–12 and G01–G24. Each predicate records:
   - the profile and public consumer it applies to;
   - its oracle and where that oracle came from;
   - its evidence class;
   - a status.
2. **Use the predicates as the denominator.** Progress is reported as predicates proven under a named evidence class. It is never a test count, line count, percentage estimate or vote.
3. **Run checks in three rings:**
   - **R0:** static inspection, safe now.
   - **R1:** ordinary local, non-native deterministic checks, safe once the audit run is authorized.
   - **R2:** native, target or ledger checks. These are blocked until separate admission.

Every R2 predicate stays `blocked` or `unknown`. It is never inferred from an R0 or R1 result.

### Alternative A: one slice in depth (FeePool or the U3 discriminator end to end)
- **Gain:** fast, concrete findings.
- **Cost:** the denominators stay untouched. That repeats the known hazard of copying narrow receipts upward.

### Alternative B: formal correspondence first (Quint/K trace adapters into Rust)
- **Gain:** the strongest semantic oracle.
- **Cost:** large Quint sampling already timed out with an unknown verdict. K covers defined subsets only. Neither reaches ZKIR or native verification.
- **Use it as:** a workstream inside the recommended approach, not as the approach itself.

---

## 2. Required audit outputs

All outputs are machine-readable and bound to the frozen commit and tree.

| Artifact | Contents |
|---|---|
| `audit/predicate-ledger.jsonl` | One row per atomic predicate (schema below) |
| `audit/oracle-register.json` | Each oracle, how it was produced, the code it shares with the subject, and an independence verdict |
| `audit/mutation-matrix.json` | Mutation × stage × expected refusal layer/code × observed result × run-id |
| `audit/conservation.json` | Per case: gross debits, fees, refunds, net, liabilities created/discharged/residual, held custody, and an invariant verdict |
| `audit/atomicity.json` | Per refused case: whether state, effects or output directory were byte-identical before and after |
| `audit/denominators.json` | Per obligation family: total, then counts by status. Unknowns are explicit. No percentages without a predicate denominator |
| `audit/premise-ledger.json` | Every trusted caller input (clock, authority, predecessor, SRS, network/Anchor), the consumer that relies on it, and the closing predicate |
| `audit/repro.json` | Toolchain, Cargo.lock digest, source-closure value, and two-builder byte comparison (or `not-run`) |
| `AUDIT-REPORT.md` | Human roadmap report: findings by risk, the obligation table, changes recommended now and deferred, dissent, and non-claims |

**Candidate row schema:**

```json
{"predicate_id":"M1.p3.target-vs-independent-reference",
 "parents":["M1","U2","MC01","MC04"],
 "profile":"native-language/1",
 "consumer":"mori compile-call --all-actions",
 "stage":"core->zkir",
 "oracle":{"id":"hand-derived-feepool-162","shares_code_with_subject":false},
 "evidence_class":"source-inspected|local-run-current|local-run-historical|native-local|target|externally-accepted",
 "status":"proven|refuted|unknown|blocked|specified-only|out-of-scope-dispositioned",
 "blocking_premises":["authenticated-predecessor"],
 "citations":["crates/mori-ledger/src/static_compile.rs:612"],
 "run":{"id":null,"command":null,"exit":null,"artifact_digest":null},
 "non_claims":["no native proof","no ledger settlement"]}
```

---

## 3. The end-to-end chain to test

Each stage needs its own predicate, oracle and hostile mutation. Rings are R0, R1 and R2 as defined in section 1.

| # | Stage | Independent oracle | Hostile mutations | Ring |
|---|---|---|---|---|
| 1 | Author source → parse | A grammar reference or hand-built AST for small programs | Unicode/confusables, duplicate declarations, size and token bounds | R1 |
| 2 | Beta/Core routing and typing | Profile matrix: `check` routes `language`-prefixed files to Core and others to Beta | Beta-only syntax passed to `compile` | R0/R1 |
| 3 | Core simulate (reference) | Hand-derived post-state and ordered effects | Overflow, rounding direction, zero denominators | R1 |
| 4 | Signed intent / verify-intent | Independent signature vectors | Wrong domain, wrong time, replayed nonce, wrong authority | R1 |
| 5 | Core → ZKIR, two emitters | **Differential:** both emitters vs Core on the same witnesses | Witness that satisfies the constraints but breaks the source semantics | R1 (SDK `check`) |
| 6 | Static bundle export and reopen | Byte identity (reproducibility only) | Tampered or extra leaf, symlink, mode swap | R1 |
| 7 | Keygen | Independent SRS provenance | — | **R2 blocked** |
| 8 | Prove | — | Six hostile witnesses (re-run) | **R2 blocked** |
| 9 | Independent verify in a separate process | Real ledger verifier | Statement or byte mutation | **R2 blocked** |
| 10 | Deploy and call on the target | Authenticated readback | Wrong network/Anchor | **R2 blocked** |
| 11 | Complete effects and history readback | Conservation invariant | Erased debt, hidden reservation | R1 local / R2 ledger |
| 12 | Recovery, late outcome, refund race | Time-ordered model | Late success racing recovery | R1 local / R2 |

### Programs

The audit needs newly authored programs. Existing fixtures are not enough.

- **P-A (new):** the U3 discriminator from `ROADMAP.md:62`. Spend at most 11 A, including at most 1 A in fees, to receive at least 20 B. Includes partial fill, a persistent duty and recovery.
- **P-B (new, structurally contrasting):**
  - no transfers;
  - mostly Liability/Discharge/Emit statements;
  - two debtors;
  - nondivisible weighted split;
  - empty-action edge;
  - a different state shape.
- **P-N (negative):** a program outside the admitted grammar or profile. It must be refused with a named code and no output directory.
- **Existing control:** FeePool `fee-pool.mori`. Its `contrasting.mori` is **not** structurally contrasting. It reuses the same 55/10/5/15 floor template (`examples/core/fee-pool/README.md:147-158`), so it cannot satisfy U2 or M1 "structurally contrasting" on its own.

---

## 4. Highest-risk gaps found in inspected source

Labels: **O** = observed in source, **I** = inferred, **U** = unknown. None of these implies runtime success.

1. **Two separate Core→ZKIR emitters (O; correspondence risk I).**
   - `mori-backend` emits `moriarty-zkir-relation/3` (`crates/mori-backend/src/lib.rs:29-36`). `mori compile` uses it (`crates/mori/src/commands/compile.rs:23-27`), as do `prove` and `verify`.
   - `mori-ledger` emits `moriarty-native-call-relation/1` (`crates/mori-ledger/src/relation/mod.rs:16`) for `compile-call`.
   - Whether they agree semantically is U. This is the main differential target for M1 and U2.

2. **Self-referential admission oracles (O).**
   - `reopen` recompiles and byte-compares (`crates/mori-ledger/src/static_compile.rs:612-619`).
   - `relation::admit` does the same (`crates/mori-ledger/src/relation/mod.rs:207-211`).
   - These show reproducibility within one build, not semantic correctness.

3. **Verification runs inside the proving process (O).** `prove_prepared_call` calls `verify_proven_call` in-process (`crates/mori-ledger/src/proving.rs:716`). Independent verification needs a separate process and binary. Whether that happened for the Oct 11 result is U here.

4. **Mapped-role debits are relaxed and unauthenticated (O).**
   - `DebitRule::MappedRoles` accepts any bound role as the debit source (`crates/mori-ledger/src/layout.rs:215-223`).
   - It uses the same `compile_inner` with `signed=None` (`crates/mori-ledger/src/relation/compiler.rs:624-636`).
   - Non-promotion is enforced at the type level by compile-fail doctests (`crates/mori-ledger/src/static_compile.rs:252-273`, `crates/mori-ledger/src/relation/mod.rs:232-237`).
   - Byte-level non-promotion is I. Mapped IR with a non-payer debit, fed to a SinglePayer consumer, should be refused by `admit_bindings`, but that needs an R1 test.

5. **Build identity is a declared source hash (O).** `source_closure` hashes the `include_str!` sources plus `Cargo.lock` (`crates/mori-ledger/src/proving.rs:149-217`). It does not cover rustc, the SDK binary or SRS.

6. **Target identity is unauthenticated (O).**
   - `Target.network` and `Target.anchor` are free strings (`crates/mori-backend/src/lib.rs:39-46`).
   - The ledger bindings anchor must equal `"local-unqualified"` (`crates/mori-ledger/src/layout.rs:154`).
   - The dependency pin is a source revision only (`Cargo.toml:23-32`).
   - So M2, M7 and M9 target binding is open.

7. **Time and authority are caller-supplied (O).**
   - `verify-intent --at` and `--authority` (`crates/mori/src/commands/verify_intent.rs:17-21`).
   - Sponsor finality is reconciled from caller-authenticated observations (`crates/mori-core/src/source_sponsor.rs:302-303`).

8. **The structural graph cannot serve as the coverage denominator (O).**
   - It excluded `experiments/**` (`structural-graph/coverage.json:58`), which holds the Quint language models and the successor path.
   - `.mori` and `.qnt` files are not AST-parsed (`structural-graph/coverage.json:45-49`).

9. **M1–M17 are missing from traceability (O).**
   - `traceability.md` mentions neither M rows nor Fenrir (0 matches).
   - `ROADMAP.md` mentions them only at line 26.
   - The only enumerated table is the Oct 10 design (`docs/superpowers/specs/2026-10-10-fenrir-integration-design.md:76-92`).

10. **CLI state for M10 (O).**
    - `simulate`, `test` and `verify-intent` are implemented. `test` compares exact post-state and effects (`crates/mori/src/commands/test.rs:60-62`).
    - Rust `lsp` and `mcp` call `todo!` and panic rather than refuse (`crates/mori/src/commands/lsp.rs:9`, `mcp.rs:9`).
    - M10 "differential beta fixtures" is U.

11. **No general Quint→Rust replay (I).** Only two crate files mention Quint. Quint's own README treats verifier booleans as premises.

12. **Oracle provenance (U).** The FeePool expectations are claimed to be independent (`examples/core/fee-pool/cases/COVERAGE.md:3`). Provenance must be verified, not assumed. The 1 MiB aggregate report limit is retained (`examples/core/fee-pool/cases/COVERAGE.md:13`).

---

## 5. Preserving obligations

- **No obligation may be dropped.** Every U, M, MC, SP and G row gets a ledger entry, with `out-of-scope-dispositioned` requiring a cited user decision.
- **Child-capability status vocabulary:** `specified-only`, `source-present`, `local-run-historical`, `local-run-current`, `native-local`, `target`, `externally-accepted`, `blocked`, `unknown`.

Examples of the useful splits:

| Parent | Children (proposed status from this review) |
|---|---|
| U2 / M1 | (a) admitted-grammar compile: source-present. (b) two contrasting new programs: specified-only. (c) emitter-vs-Core differential: unknown. (d) native verify: blocked. (e) complete-effect readback: blocked |
| M3 | Prove: historical native-local, one case (Oct 11, per `ROADMAP.md:15-26`; not rerun). Independent verify: unknown. Deploy/call/effects on Fenrir: blocked (no target identity) |
| M10 | simulate/test/verify-intent: source-present. Beta differential: unknown. LSP/MCP Rust: stub |
| M11 | FeePool finite source component: source-present, plus PR49 captured local results. Mapped-role static export: source-present, unauthenticated. Capacity Exchange: specified-only. Native tests/readback: blocked |
| M13 / M16 | Local sponsor holds and metering: source-present. Authenticated finality and cost planes: specified-only |
| M2 / M7 / M9 | Source pin: source-present. Fenrir network/Anchor/registry: blocked on external facts. Network ID and Anchor must not be invented |
| G01–G24 | Retain `unassessed`; primary closure under SP12.3 |

---

## 6. Workstreams

| WS | Work | Depends on | Ring | Can run in parallel? |
|---|---|---|---|---|
| W0 | Freeze commit/tree; build the obligation inventory from ROADMAP, the product contract, traceability, the Fenrir design and the MC specs | — | R0 | — |
| W1 | Write the predicate ledger and denominators | W0 | R0 | — |
| W2 | Register oracles; audit independence (shared code, who authored the expectations) | W0 | R0 | Yes, with W1 |
| W3 | Differential and metamorphic tests: two emitters vs Core; statement-reordering invariance; scaling and rounding laws | W2 | R1 (SDK `check`, not proving) | Yes |
| W4 | Hostile mutations: JSON duplicate keys and nulls, mode swap, leaf tamper, symlink, mapped→single-payer byte replay, witness-satisfies-but-violates | W2 | R1 | Yes |
| W5 | Conservation and atomicity on FeePool, P-A and P-B (gross = claimed + duties + backstop + held + carried, per `examples/core/fee-pool/README.md:41`) | W3 | R1 | Yes |
| W6 | Reproducibility: two clean builders; source-closure stability | W0 | R1 | Yes |
| W7 | Native keygen/prove/separate-process verify; Fenrir target | Native admission and target facts | **R2 blocked** | — |
| W8 | Synthesis into the report and roadmap edit proposal | All of the above | R0 | — |

### Safe now (R0)
- W0, W1, W2, and the predicate ledger.
- Checking every claim in this report against source.

### Safe once the audit run is authorized (R1)
- `cargo test`, including the FeePool component and mapped-role I/O tests.
- `mori compile-call` export, reopen and inspect.
- SDK `check` on preimages.
- Quint deterministic tests.
- K fixture runs.
- Two-builder reproducibility.

### Blocked
- Keygen, proving, native verification, deploy/call/ledger.
- SRS/PK/cache reads.
- Fenrir target.
- Large Quint sampling. This may be retried only under a bounded budget, and a timeout is recorded as `unknown`.

### Done when
- Every obligation has a ledger row with a status and at least one citation.
- Every R1 predicate has a current run artifact.
- Every oracle has an independence verdict.
- Every refusal case has an atomicity verdict.
- All unknowns and blocked items are counted in the denominators.

---

## 7. Roadmap updates

### Recommended now, as wording only, with no promotion

1. **Add a dated "What exists" entry for PR45–49.** Keep each item's qualification:
   - PR49 FeePool is a finite source component.
   - Mapped-role export is `source-static-mapped-roles-unproven-uninstalled-unauthenticated` (`crates/mori-ledger/src/static_compile.rs:20-22`).
2. **Add an M1–M17 crosswalk** to `traceability.md` and the roadmap: each row mapped to its U/MC rows and child predicates, all open.
3. **Remove the subjective 60–70% and 30–40% estimates.** Point to the predicate denominators instead.
4. **Record the two-emitter fact as an open U0/M1 correspondence item.**
5. **Record that `contrasting.mori` does not satisfy U2/M1 structural contrast.**
6. **Keep these as explicit unknowns:** the Quint 1000×40 timeout and the 1 MiB aggregate report limit.

### Deferred until there is evidence
- Any status above `source-present`.
- M3/M11 native claims.
- Any M2/M7 target identity.
- U2 generality.

---

## 8. Dissent, open questions and non-claims

### Dissent
- **"Reopen admitted" is not a correctness signal.** The report should say so in plain words; it is not stronger than byte reproducibility.
- **Panicking stubs are a release defect (U7 negative controls).** Rust `lsp`/`mcp` should refuse rather than `todo!`.
- **The roadmap may need a choice of canonical emitter.** It should either pick one or require a differential gate between the two.

### Open questions
1. Which emitter is canonical for U2?
2. Is there a user-approved record that makes the Oct 10 M table controlling?
3. Which Fenrir network/Anchor facts exist? The user says Fenrir has no versioning yet.
4. Were the FeePool expectations authored by someone who never saw execution output?

### This review cannot support
- Any runtime success, test pass or native result.
- Correspondence between source, Core, ZKIR and ledger.
- Fenrir qualification or authenticated state, time or history.
- Any milestone acceptance.
- Any score.

Counts such as 162 cases, 84 refusals and 14 Quint tests describe captured inputs, not correct behavior.
