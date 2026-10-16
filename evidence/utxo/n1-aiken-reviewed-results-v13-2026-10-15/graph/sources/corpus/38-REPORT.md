# REPORT: P3S1-R1 W0/W1 source (production gate split and per-component cost benches)

**Author:** `claude-opus-5-5` (runtime-stated; high effort requested, not self-observable). Fresh context. Security, cost and ledger judgments here are Opus-owned per `docs/audits/2026-10-14/SECURITY-ROUTING.md`.
**Tools:** Read, Write and Edit only. I compiled, ran and hashed nothing. Every outcome below is a source prediction.

## Bottom line

| Axis | Status |
|---|---|
| Implementation | **W1 and W0 are written at source.** C2 and S2 evaluate unnamed Boolean gates. Each condition is defined once and shared with test-only diagnostics. 128 W0 benches and 66 W1 gate tests are new. |
| Evidence | **None yet.** Nothing has been compiled or run. The predictions, falsifiers and attribution rules were frozen before any run. |
| Acceptance | **None.** This is first measurable source work on PB-01/PB-02, not a fix. W1 is **predicted not to close** either blocker (WS-06, WS-07). |

## 1. What I read and the consumer path

I read the startup skill, the execution-focus reference, the full-integration `AGENTS.md`, the product contract and the security routing. I read the full actual-result review packet (REPORT, NEXT-CAPABILITY, FINDINGS, ADOPTION-RECOMMENDATION, ROOT-REVIEW-MANIFEST) and the root ADOPTION.json. I also read the frozen 146-test index, `EXPECTED-CASES-R1.json`, every root run record I used (A1-TEST-RESULTS in full) and the Preprod projection. Before changing anything I traced the actual consumers:

- `validators/s2.ak` → `thread.genesis`, `thread.burn_only`, `thread.settle`, `thread.revoke`, `thread.close`.
- `validators/c2.ak` → `claim.issue`, `claim.burn_only`, `claim.release`.

In R1, `settle` → `settle_checks` → `settle_facts(..).gate`, and `release` → `release_parts`. The named lists, with their ByteArray labels, were on the production path. That is where AR-07's C2 strings came from. Both validator files are unchanged in W1: they call the same functions, which are now the unnamed gates.

## 2. W1: what changed

- **One definition per condition.** Each component is a `pub fn … -> Bool` in `thread.ak` or `claim.ak`: genesis 17, settle 19, revoke 8, close 9, release 5. Each body is the R1 expression (table in `DIFFS/thread.ak.md`, `DIFFS/claim.ak.md`).
- **Unnamed production gates.** `genesis`, `revoke`, `close` and `release` are `and { … }` over those functions, in R1 order. `settle_gate` returns `Some(sigs)` or `None` through a nested `if` chain over the 19 settle components in R1 order. `settle_checks` and `settle` wrap it.
- **Diagnostics moved out of reachable code.** `pvc2/diag` (new) holds `all_parts`, `false_parts`, `SettleFactsV2`, `settle_facts` and the five `*_parts` lists. Each list entry is a call to the production function on stage values built by the production stage functions. Neither validator nor any production module imports `diag`, so no label is reachable from C2 or S2.
- **Parse once, build each stage once.** Settle work is staged: B (parse), SD, E (effects), S (snapshot), C (claim), A (signer accounts), SC (scenario hash), M (messages), ST (statement), N (next datum). The production gate builds a stage only after every earlier component holds, and passes it to every later component that needs it. No full facts are built and then picked apart.
- **Shared intent fragments.** The signer-independent OwnerIntent JSON values and their checks are now rendered once per step instead of once per signer. The message bytes are unchanged; the R1 `facts_match` tests compare them with the frozen `msg_*` fixtures.
- **Small pure refactor.** The first-match cell and duty index search no longer allocates an `indexed_map` per datum row.

The R1 test modules keep all 146 test names, bodies, expected lists and `fail` flags. Only the helper call targets move to `diag` (`DIFFS/settle_r1_test.ak.md`, `DIFFS/unit_r1_test.ak.md`).

## 3. Why the accepted set is unchanged (source argument, not a formal proof)

1. Production accepts only if every stage and every component evaluates True without halting. On a positive input nothing short-circuits.
2. The diagnostics evaluate exactly that set of expressions, and so did R1 `settle_facts` and the R1 lists. W1 moved conjuncts; it removed none (component tables in DIFFS).
3. So production accepts ⇔ the diagnostics evaluate without halting and every part is True ⇔ R1 accepted.
4. Where the diagnostics halt, production either halts or returns None/False. It cannot accept, because accepting evaluates the halting expression.

**Refusal class.** Short-circuiting can turn an R1 halt into a False/None refusal. This changes how a transaction is refused, never whether it is. I recorded the R1 class of every one of the 85 cases before any run (`OLD-CASE-CLASS-CROSSWALK.json`): 18 POSITIVE, 57 BOOLEAN, 10 ERROR-PATH.

- **57 BOOLEAN cases:** the new non-`fail` tests assert that production refuses *without halting*. That is a falsifiable consequence of point 4.
- **9 settle ERROR-PATH cases:** the new tests are `fail` tests, so they are refusal evidence only. I claim no isolation and no ERROR→False conversion. My per-case predictions of where production halts are labelled unverified.
- **One theoretical diagnostics shift:** a later signer's `check_effects` overflow after an earlier False is now False instead of a halt. No fixture has this shape.

## 4. W0: the cost benches and how they will be read

- **Contexts:** P01, P04, and P07.0 and P07.1 (the two threads of the atomic transaction). Per context there are 32 benches:
  - decode only;
  - 10 cumulative stage closures;
  - the production gate;
  - the diagnostics gate;
  - one bench per settle component (19).
- **Bench shape.** Each bench builds only its component's stage closure. No bench builds facts or a parts list, except the one that measures exactly that.
- **Expected truths.** Component truths are frozen from R1 `falseParts: []`. The stage forcing predicates are my own and labelled separately (`EXPECTED-COMPONENTS.json`).
- **Attribution rule (frozen in `COST-ATTRIBUTION-PLAN.json`).** Stage and component marginals are differences of cumulative closures, so shared setup and cached work count once. On these positive inputs every component is evaluated, so short-circuit does not bias the sum.
- **Additive model.** gate = D + Σ stage marginals + Σ component marginals + r, with |r| ≤ max(1% of (gate − D), 300,000) mem and max(1%, 100,000,000) cpu. A residual outside tolerance falsifies the model for that context. A marginal below the sanity floor marks that bench shape invalid. Neither is refitted.
- **W1 deltas.** These come from the unchanged R1 test bodies, compared against the adopted R1 values, with `sequential_chain_reserve_r1` as an exact-equality control.
- **Decode caveat.** D is the harness decode (`aiken/cbor.deserialise` + `test_ctx`). It is not the ledger wrapper. X1 stays a separate future measurement.
- **Not ledger figures.** All values are host CEK under aiken's default cost model, not ExUnits.

## 5. Predictions (directional; not gold values)

| ID | Prediction | Confidence |
|---|---|---|
| T-1 | All 340 tests pass, with names and fail flags as `candidate/expectations/EXPECTED-W0W1-TESTS.json` (46 fail) | moderate (I cannot compile; WS-10) |
| PD-1 | Every R1 `*_accept` test listed in the plan costs less than in R1 | moderate |
| PD-2 | `p01`/`p04` `facts_match` cost less than in R1 | low (stage-record overhead, WS-05) |
| PD-3 | `sequential_chain_reserve_r1` is exactly equal to R1 | high |
| PS-1 | No R1 gate label in C2 or S2 compiled code | high |
| PS-2 | C2 < 2,911 B | moderate |
| PS-3 | S2 unapplied < 30,728 B | low |
| PS-4 | S2 applied stays > 16,384 B | high |

## 6. Honest remaining gap

- **Size (PB-01).** W1 removes labels, lists, a record and one per-signer render. It keeps every check, so S2 is very unlikely to drop from 30.7 KB to under 16 KB. PB-01 needs a split (W3 or equivalent), with its ten adversarial cases, and M-SZ measurement of each script and each deploy transaction.
- **Cost (PB-02).** On an accepted settle every component runs, so W1's short-circuit saves nothing on the accept path. The R1 host signal is about 2.8–3.7× per-tx memory after decode removal. W1 is not expected to close that. W0 shows where the cost sits (the share of JSON rendering versus recomputation versus statement versus per-thread work). That decides whether W2, W2b or W3 is worth its two-vote, security-reviewed proposal.
- **Still unmeasured:** C2 Groth16 `issue` (K2), valid-signature cost (FX1), the wrapper (X1), the Preprod cost model and parameter mapping (PB-04), and per-transaction totals including the C2 release.

## 7. Execution proposal (request only)

`EXECUTION-PROPOSAL.json` reuses the R1 literal steps with path deltas. It keeps the same aiken v1.1.24 binary, stdlib v2.2.0 and `--trace-filter all --trace-level silent`. There are three substantive deltas:

1. Placement by `cp -R` + `sha256sum -c`. The Sol helper pins the R1 manifest hash and a 16-file count, so it cannot place this 19-file package.
2. A new read-only label-search step, A2s.
3. A corrected A2r label: 16,384 is maxTxSize.

It needs a fresh full Opus source/effect review, two resource votes and the root's exact authorization. It is a new run, not a retry. My positions are in `VOTES.md` and do not count as votes.

## 8. Preserved and open

Every financial, authority, frame, residual, recursion/claim-binding and multi-parent check of P3S1-R1 remains on the production path (`CHANGES.json` `preservedObligations`). Also unchanged: the RR-16 cross-version and RR-08 close residuals, the signed format and the ABI.

Still open: the original bridge 42/52 gap, all 20 pattern families, ACTUS 277/18/32/DS01–07, DeFi 72/DA01–24/held-outs/intents/regressions and general financial obligations, recursion/PCD, private handoff, multi-parent composition, PB-01..PB-12, Preprod, and every U0–U7, M1–M17, MC01–MC08, SP01–SP12 and G01–G24 exit. No funding, key, signature, proof, ledger, network or deployment action was taken or approved. The public language stays permissionless; this is maintainer source work, not program admission.

## 9. Method limits

- **Hashes.** I copied them from root seed and records; I recomputed none. Changed and new files have no hash until the root hashes them after my terminal state.
- **Input reading.** I read `EXPECTED-CASES-R1.json` in two pages, covering all of it. I did not read `fixtures_r1.ak` (1 MB, root-seeded and unchanged); the fixture constant names I use all appear in the R1 test module.
- **Output files.** The R1 run directory is untouched. I wrote only inside my own directory. I did not list, stat or open any root-owned prompt, process, provider, stderr or wrapper file.
- **Coverage record:** `READ-COVERAGE.json`.
