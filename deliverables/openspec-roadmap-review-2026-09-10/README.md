# Moriarty OpenSpec and sprint review — September 10, 2026

**Recommendation: continue SP05, Financial integration on Preview. Finish SP05.2's local adverse-transaction evidence before SP05.3's separately admitted Preview campaign.**

This is a source and evidence review at commit `dd0897e878d34f6357c48efa90278f72551b034f`. It changes no sprint status, campaign admission, resource allocation or historical receipt. No blockchain transaction was submitted for this review. The graph is navigation and source provenance, not an acceptance certificate.

The [review inventory](review-inventory.json) covers all 76 OpenSpec files: 65 Markdown documents, eight JSON records, two Python files and one YAML configuration. Root and two bounded review assignments read their full contents. Their recorded SHA-256 values match the reviewed checkout. This includes all twelve sprint plans and all nine change packages. [Validation observations](validation-observations.json) and the [independent scoped synthesis check](SYNTHESIS-CHECK.md) retain their separate limits.

## Repository graph

The refreshed [interactive graph](../../graphify-out/graph.html) opens with 4,193 communities. The [graph report](../../graphify-out/GRAPH_REPORT.md) and [coverage manifest](../../graphify-out/coverage-2026-09-10.json) describe 103,069 nodes and 298,898 directed edges across 15,775 inventoried files, including retained dependency snapshots. Counts are not unique concepts. All 76 reviewed OpenSpec files are present with matching hashes and fresh review nodes.

The full detected corpus received structural indexing; existing semantic extractions were reused and OpenSpec semantics refreshed. This was not a new semantic reading of all 14.1 million words. There are 7,700 uncached document/media files without new semantic extraction. The importer excluded 5,489 inherited relationships with unresolved endpoints while preserving their original snapshots, and retained eight self-loops. The exported graph has unique node IDs and zero dangling endpoints. The graph report lists the 24 detector-sensitive exclusions and metadata-only files. Generated graph outputs remain local and Git-ignored.

## Why SP05 comes next

The authoritative `i2` stage requires only `atomic-accept` and `rp01-mc02`. Both are recorded complete for their exact bounded profiles. Whole-SP01 completion is not a task-entry barrier. Full successor language and native recursion remain separate tracks.

The latest reviewed local loan and swap traces supply useful positive evidence. Loan partial repayment preserves remaining debt. The swap exchanges 10,000 A net for 19,743 B; closing returns the remaining reserves and leaves both contract balances zero. These are local `undeployed` results, not Preview results. Their incomplete inner cleanup diagnostics and independently established outer containment remain separate observations.

Sources:

- [Authoritative stage register](../../openspec/moriarty-completion-program.json), `reportReconciliation.stageAdmission.stages`.
- [SP05 tasks](../../openspec/sprints/sp05-financial-integration-on-preview.md), especially SP05.1–SP05.3 and the report refinement.
- [Reviewed local loan](../sp05-financial-integration-2026-09-09/local-continuation-02/REVIEWED-RESULT.md).
- [Reviewed local swap and close](../sp05-financial-integration-2026-09-09/local-swap-continuation-01/REVIEWED-RESULT.md).
- [Asset-study implementation order](../../openspec/ASSET-STUDY-INTEGRATION-2026-09-09.md): finish the currently admitted loan/swap work before expanding its financial profile.

## Next demonstrable capability

A rejected financial operation leaves the contract's financial state and assets intact. Evidence identifies the rejection boundary and accounts separately for any charged network fees.

1. Complete SP05.2 adverse controls through the callable production path. Cover wrong payer, wrong recipient and omitted fees where required.
2. Distinguish a pre-submit source rejection, node rejection and included transaction rollback. One does not establish the others.
3. Complete SP05.1 identity, custody and funding evidence. The current single-controller result does not establish independently funded or signed counterparties.
4. Retain successful local loan/swap evidence; do not rerun consumed campaigns merely to refresh a green result.
5. Admit SP05.3 separately against current source, identities, tool versions, resources and both required reviews.
6. Finalize both financial cases on Preview. Compare complete state, assets, recipients, fees, change and surviving duties with independent expectations.
7. Post every actual transaction ID and observed status. Keep I2 integration receipts uncertified until the later mandatory-acceptance lineage is demonstrated.

Current review routing is GPT-6 implementation, fresh independent GPT-6 Astra and Grok 4.6 high audits. Historical Fable and Opus receipts retain their identities. A source-review recommendation is not execution permission.

## Planning discrepancies

| Finding | Evidence | Disposition |
| --- | --- | --- |
| TX03 contains an incorrect financial expectation | `openspec/sprints/report-lessons.json:385` states `100 × 10/100 × 30/360 = 5`; exact rational evaluation gives `5/6` | Correct the adopted expectation before SP07/SP11 use it. Preserve the historical report and record the discrepancy; source fidelity does not establish arithmetic correctness. |
| Sprint verification fails on a stale source binding | `python3 openspec/sprints/verify.py` exits 1: `Stale report source: docs/FOOTGUNS.md` | Review the changed source and update its binding through ordinary reviewed maintenance. Do not replace the hash solely to make the check green. |
| Asset-study crosswalk is outside the current validator's checks | `openspec/sprints/verify.py` does not load `asset-study.json` | Preserve AS01–AS11 and AT01–AT08. Add meaningful source, ownership and omission controls in a scoped maintenance task. |
| SP05 execution prose predates actual local settlement | `openspec/sprints/execution/SP05.md:23` says no local financial transaction was submitted | Add a current disposition pointing to the reviewed results. Preserve historical failure evidence. |
| Maintained reviewer instructions disagree | Charter/register and MC package boilerplate still select Fable; September 9 assignment and AGENTS select Grok | Latest user routing controls. Reconcile maintained instructions without rewriting historical reviews. |
| Old proof-path descriptions conflict with the K-first contract | MC04 design lists Lean outputs; newer task/formal contract makes a supporting proof assistant conditional | Use executable K and required correspondence claims. Do not create an unnecessary second semantics. |
| Plugin queue reports old SP01 operational history | `status` and `next` select `sp01-loan-report` and identify unresolved campaign fields | Preserve the affected dispatch stop. This does not override user-directed review or create a whole-SP01 barrier to the I2 product track. |

OpenSpec strict validation passed all nine change packages. That validates their OpenSpec structure; it does not establish implementation, financial acceptance or complete coverage. The sprint validation failure remains visible.

The validator test suite ran eight tests: four passed and four failed. The stale source binding masks several expected mutation diagnostics. An audit-only collector found no other failing validator predicate; its instrumented output is not a canonical pass. Manual checks found all 11 AS requirements and eight AT cases intact, with valid source binding and task ownership. A controlled absent-file view confirmed that the canonical validator never reads the asset-study map.

## Remaining sprint sequence

| Sprint | Next required outcome |
| --- | --- |
| SP01 | Complete financial/signing design, RP01-MC03, full RP01 and the bounded native route decision. |
| SP02 | Complete the successor lexical, EBNF, static, canonical and authoring contract, including the matched syntax study. |
| SP03 | Extend beyond the tested K projection and discharge the required base-domain semantic and correspondence claims. |
| SP04 | Establish complete outer-verifier feasibility with independent fixtures and all P1/P2/P3 controls. |
| **SP05** | **Finish local adverse and custody evidence, then finalize loan and swap on Preview.** |
| SP06 | Produce a real two-step recursive financial proof and independently verify retained bytes and rejection controls. |
| SP07 | Implement full ACTUS semantics, including all 277 fixtures, 18 executable types and 32 dispositions. |
| SP08 | Implement all 72 historical DeFi rows, DA01–DA24, intent/request behavior and report-derived cases. |
| SP09 | Join I2 and native proof evidence at atomic F3, then qualify the full mandatory successor acceptance lineage. |
| SP10 | Demonstrate isolated private handoff, split/join and all five composition operators. |
| SP11 | Close complete financial qualification across semantics, proofs, local acceptance and required Preview evidence. |
| SP12 | Deliver the reproducible developer workflow, two clean builders, two substantial pilots and every release gate. |

If SP05 public admission is blocked, continue eligible SP01 source/design work. SP02's full stage waits for full RP01; SP04 and SP06 cannot bypass the native feasibility gates. Parallel preparation remains permitted within its actual scope.

See [the MC01–MC04 review](MC01-MC04-REVIEW.md) for exact requirement locators and SP05 entry conditions, and [the MC05–MC08 review](MC05-MC08-REVIEW.md) for the financial expectation, asset-study checks and later dependencies. [Raw validator test output](sprint-validator-tests.txt) and [instrumented review output](review-checks.txt) preserve their different scopes. The accompanying file inventory and graph report record review and indexing coverage separately.
