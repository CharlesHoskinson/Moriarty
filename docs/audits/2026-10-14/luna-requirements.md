# Luna graph pass 3/6: requirements and traceability

Base: `cb8280ef54082853dd8bb6c6a52050328d0fc6fa`. Scope: read-only acceptance/requirement mapping. Plugin status reported capability `SP01.6 loan-swap-subset` with operational-history/binding/accounting/resource evidence unresolved; no new dispatch was performed.

## Normative reading

- `ROADMAP.md` is the single delivery sequence. U0-U7 control current scheduling; MC01-MC08, SP01-SP12 and G01-G24 remain objective obligations. P/C/K labels remain lineage, not queues.
- The product contract controls permissionless public product scope and ZKIRv3/Midnight target. Internal review/campaign/resource process cannot become developer compile/prove/deploy prerequisites.
- Traceability is a scheduling crosswalk, not a waiver. Detailed package specs, positive/hostile evidence and original denominators remain in force.
- The Fenrir M1-M17 detailed source is the Oct 10 integration design. The Oct 14 routing assignment confirms M1-M17 remain in scope; no separate M-row assignment was found under `raw/assignments/`.

## Acceptance and dependency findings

- Every U0-U7 exit is explicitly stated in `ROADMAP.md:45-52`; roadmap text says all are still specified-only until demonstrated (`:11`) and all U/M exits remain open (`:24-29`).
- SP07/SP08 can prepare independently, but full SP09 requires accepted semantic inputs; U4 can accept a bounded subset earlier without closing full SP09/MC05. This is an artifact dependency, not a whole-sprint cycle (`ROADMAP.md:54-58`).
- Crosswalk ownership is uneven by design: MC05 spans U2 subset, U4 native base, SP07/SP08 inputs and U6 requalification alongside MC03/MC04; MC02 remains U2/U7. Preserve those partial scopes.
- G01-G24 each retain a historical `unassessed` status and point to SP12.3 primary closure, with individual contributing tasks in `legacy-release-gates.json`. G18 retains stop/reduce wording, but refinement says current user scope forbids autonomous reduction: block affected release and preserve the gate.
- Broad milestone closure must not erase scoped shipped behavior: local K/evaluator, Preview loan/swap, authoring beta, and Oct 11 local proof are foundations only. Roadmap explicitly keeps general source-to-ledger, mandatory PCD, recursion/private history and full conformance open.

## Broad milestones that hide subcapabilities

- U2 contains several distinct accomplishments: local signed intent, native relation checks, complete effects/duties and bounded hostile witnesses. The Oct 11 result advances one local laboratory case but lacks authentic account/history/time/nonce and Preview financial settlement.
- U4 separates native recursion, legitimate genesis, bounded multi-parent compatibility, private witness handoff and five composition operators. Ledger induction does not close MC03/MC06.
- U7 is not synonymous with SP12 documentation or M10 CLI availability; it requires clean external installation, new non-fixture authoring, own-wallet submission, two-builder reproducibility and complete release/pilot/license/privacy evidence.
- Fenrir rows M1-M17 mix Moriarty-local capabilities with target-qualified proof/deploy/call and external contract/library behaviors. Report each row with its exact local/native/live predicate; no single local proof closes a whole M row.

## Structural issue audit

- No acceptance dependency cycle appears in U0-U7: U1→U0→U2→U3→U4→U5→U6→U7 is acyclic. SP07/SP08 semantic inputs join full SP09; early subset acceptance is expressly permitted.
- The main stale-claim hazard is historic completion labels or narrow receipts copied upward. Roadmap and traceability explicitly prohibit inheritance of broad completion from atomic acceptance, hello-world Preview, scoped loan/swap results, ledger induction, or one bounded proof.
- Potentially unclear authority: M1-M17 detailed exits are in an Oct 10 proposed design marked for review, while Oct 14 routing only affirms scope. Audit should identify a current user-approved assignment/decision record before treating design choices as controlling; do not infer a weaker exit.

## Source limitations

See JSON `coverage_gaps` and `inspected_paths` for hashes. All eight MC package specs and coverage map were recorded, but their every EARS scenario was not flattened into this capped graph. The graph is an obligation map, not proof of implementation or completion.
