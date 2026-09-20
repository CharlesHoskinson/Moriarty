# Independent delivery review of consolidation candidate v1

Verdict: **changes_requested — DESIGN CONSOLIDATION ONLY**.

I read all eleven files in `review-candidate-v1.txt`. Each reconstructed file payload matches its byte count and SHA-256 in `candidate-v1-manifest.json` after removing packet separators. This review is bound to that frozen manifest, not a live checkout. I refreshed read-only repository status; no builds, tests, proofs, deployments or candidate edits were performed. An initial comparison to the older main checkout was inapplicable; the complete frozen-packet/manifest comparison passed for all eleven files.

The proposal honors the user's architecture and is substantively ready except for one narrow acceptance-dependency correction. Future implementation, proof and backend availability gaps explicitly marked open are not reasons to withhold design approval.

## Required correction DREV-01: preserve full successor acceptance dependencies

**Severity:** medium; required before describing the crosswalk as preserving every SP acceptance contract.

**Files/sections:** `openspec/changes/consolidated-language-kernel/traceability.md`, Prior plan aliases, SP09 row (`U2 subset, U4 full`) and MC05 row (`full closure U4 with MC03/MC04`); `ROADMAP.md`, One implementation sequence, U4 dependencies and U6 qualification exit.

**Problem:** The inherited `openspec/sprints/README.md` distinguishes early atomic/subset acceptance from full successor mandatory promotion. The latter requires accepted successor semantics and ACTUS/DeFi semantic inputs; SP09's full completion depends on SP07/SP08. The new crosswalk unconditionally assigns full SP09 to U4, but U4 explicitly depends only on U2 and U3 lifecycle invariants while SP07/SP08 are assigned to U6. This leaves a misleading route to reporting full SP09 before the required extended semantic inputs are accepted. The generic statement that inherited gates survive helps, but does not reconcile this explicit closure placement.

**Minimal correction:** Keep U4 as the owner of recursion/private composition and its qualified acceptance subset. State explicitly that full SP09/successor MC05 promotion additionally requires the accepted SP07/SP08 semantic inputs; those may be prepared and accepted ahead of U6's final conformance gate. Add that U6 requalifies affected compiler, complete-effect, consumption and mandatory-acceptance predicates for each semantic/library extension, with full supported-language closure reported only after its required domains are covered. Update the two crosswalk rows accordingly. Do not require all U6 conformance to block early U4 work and do not create a second queue.

**Requirement links:** UNI-002/003/016; ZR03/06/11; retained SP09 completion contract and MC05 acceptance lineage. This is a planning consistency correction, not a request for new proof implementation or a change to the native-recursion horizon.

## Minor editorial correction DREV-02

`openspec/changes/consolidated-language-kernel/requirements.md` identifies `UNI-001–016` as the normative consolidation clauses, but the supplied spec also contains `UNI-017 Next native backend contract`. Change the range to `UNI-001–017`. UNI-017 and ZR01–ZR16 are otherwise explicit and appropriately scoped. This item alone would not block approval.

## Positive scope findings

- **One queue:** ROADMAP U0–U7 is explicitly governing. P/C/K become aliases; the task register forbids separate dispatch. Independent source preparation is permitted without inventing parallel compiler programs or another scheduler.
- **MPLR/financial preservation:** All MPLR-001–035 have rows, linked detailed behavior and future evidence status. All MC01–MC08/SP01–SP12 are mapped. The crosswalk retains G01–G24 and the ACTUS/DeFi fixture, field, taxonomy, source-gap, held-out and composition denominators. DREV-01 concerns closure timing, not an omitted family.
- **Permissionlessness:** UNI-001, the design boundary and workflow forbid public use of project metadata as validity inputs. U5 is optional federation functionality. U7's requirement to finish the full requested project agenda does not itself make federation membership a prerequisite for a developer's direct Midnight program; the text expressly preserves direct use.
- **Recursion:** The approximately March 2027 horizon is a dated user planning assumption. Full native recursive financial history/private bounded multi-parent scope remains required; ledger induction cannot close MC03/MC06. ZR02/04/05/07/08/09/14/16 make the target materially more precise without claiming current availability.
- **Backend honesty:** ZR01–ZR16 separate version identity, binding, arbitrary-witness soundness, deferred verification, actual ledger consumption, private continuation and cost evidence. Historical PR/native observations remain pin-bound. Tests do not stand in for soundness or compilation correspondence.
- **Financial boundary:** UNI-004/005/014 preserve gross versus net economics, explicit liability and authority accounting, exact price orientation and directed rounding. The design correctly treats narrow DeFiFormal models as reference work, not a native backend or a deployed federation.
- **Partiality/recovery:** Retained phases can create only authorized duties; request receipt alone cannot impose recipient obligations. Ordinary expiry and scoped recovery rights are separate. Timeout cannot manufacture nonexecution. Work/state caps have explicit recovery/rollover obligations.
- **Pel:** The supplied template has an initial candidate and one correction, skips substantive review after failed verification and returns a bounded non-completion result. The workflow labels symbolic roles, artifact references and `candidate-full` unbound until real commands, resources, schemas and identities are supplied. It creates no campaign or public gate. No new implementation is needed merely to approve this specified-only workflow.

## Re-review condition

After DREV-01 is corrected and the UNI range fixed, I expect to approve the design consolidation, provided no unrelated semantic or scope changes are introduced. This expectation is not an approval of uninspected changed bytes. No implementation, release, native proof or ledger acceptance is approved by this report.
