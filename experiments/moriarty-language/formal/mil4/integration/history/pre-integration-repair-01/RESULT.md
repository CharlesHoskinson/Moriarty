# W-D2 integration reconciliation result

**Status: independent document reconciliation / specified-only proposal,
2026-09-30.** Delivered [SPEC](SPEC.md) and [ISSUE-MATRIX](ISSUE-MATRIX.md) specify
cross-layer issues, new proposed comparison order and the smallest conditional
executable sprint. This result is neither adoption nor acceptance. The scope
is the exact F repair05/G repair03/H repair01 document inputs listed below.

## Findings

Independent inspection found two exact first-failure contradictions in G's
repair03 packet: G29 and its G18+G29 paragraph claim tag10 despite a changed
retained current action that must fail tag6. The parent separately reports
repair04 frozen, with bounded GPT approval reported and Grok review pending. The
parent also reports four medium H repair01 findings and a frozen repair02 with
bounded GPT approval and Grok pending:
tag8 definition-body order, suite selection, B14 scope diagnostics and policy
keyRef admission at12. F repair05 Grok review is also pending. Successor
bytes/full fresh review are unverified here;
they cannot erase these pinned packet findings or pre-approve the successor.

The major remaining integration gaps are F's missing wrapper schema, the
unmerged tag16/17 multi-provider order, artifact/body comparison diagnostics,
and explicit preservation of F repair05's Source debtor/creditor literal loci
inside H's operation-order wording. SPEC makes concrete successor proposals
with first-failure examples, including a new B14 scope projection/diagnostic
order and a keyRef-domain diagnostic at12. They require fresh exact-candidate
audit. Updated G/H candidates require a later exact-byte reconciliation.

The inspected proposals agree on the outer phases, sole B11 current-head
authority, prior predecessor versus current head versus independent successor,
complete B16 replay transport, no truncated/deduplicated history, and separate
B17 ledger mode. A genuine policy-body mismatch and wrong selected artifact
link are distinct cases. The local three-file package cannot establish the
unimplemented full-history adapter; H explicitly records this gap.

Every B01–B17 obligation is checked in ISSUE-MATRIX. None closes. Recommend
the smallest conditional next sprint: a standalone purpose1 Source-definition
encoder/comparator with new independent Transfer/Repay byte/digest vectors
after exact purpose1/identity-mapping adoption. Its result would establish
content only. Purpose3/4, real registry/snapshot/signature/history authority,
runtime correspondence, full consumer and actual ledger application remain
separate delivery requirements.

## Input snapshot

Worktree: `/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929`.
Inspected HEAD: `983a4bb49e3ccae399ee2514da2f6fa3f03593fd`.
The proposals and MIL4 implementation inputs are untracked in this shared
worktree; HEAD alone does not freeze them. Hashes below identify the inspected
document bytes, not computed Source/Core/policy commitments or audit receipts.

Paths are relative to `experiments/moriarty-language/formal/mil4/`.

| Input | SHA256 |
| --- | --- |
| effect-consumer/PLAN.md | c2cc802de3ae5c9a4985f2676daa1d0ba71259d513488d049c2b7a70f40c791d |
| effect-consumer/FIELD-MAP.md | 52f9a795c8a56c1474132fcaad7a39af18d6f642a237d825f9b270043c7c4182 |
| effect-consumer/EXPECTATIONS.json | e91934a5801343d98727d1881a0b0e249d143a65f13f7af8a660a7c4eb84603b |
| effect-consumer/RESULT.md | 09ef42d43fcd90b91beb7bd6f23428cd0baf260f5aa8171551faa8d9d8ee205d |
| effect-consumer/freeze-receipt.json | 0dfcbafd6c421f9afd2b7bd9d860bb71a3c45cd78e44f465ce79e0b871d80f16 |
| identity-binding/SPEC.md (repair03) | ac7427a5ed4855f608ff45d00e97ed9d43616c08a208f5d6cd21ffa112a4413a |
| identity-binding/DECISION-MATRIX.md (repair03) | 02b3f06df69a5bc89bd35d74f041e60a5e8202c6b5abff913b11db9cb2a4d45d |
| identity-binding/RESULT.md (repair03) | 4b3c2611e9cd235adb18f7563e89e89181143501c24ea7a7ecf39d67bed20f52 |
| hash-images/SPEC.md | 5fdec8d214ec5e51ad2a77e0bcfbaf9078de95cb0b9bfa21ad630b55c1f8990f |
| hash-images/DECISION-MATRIX.md | e611ba6e160686488faa6e9c1abfec9568788908952bd5ef4a7a494dd8972658 |
| hash-images/RESULT.md | 4bd14da24c0e2b9a14aab29ccc44d6689daaa293850f1b5b0acf9d8e3fbf3072 |

Supporting read-only inspection included Source/6 parser/lowerer, Core/5
prepareMil4S0, Source wrapper, wire/effect specifications, six effect fixtures,
F expectation metadata and replay/operation comparison order. Relevant code
observations: lowerer.state.consumedReplay is []/[selectedTuple], Core intent
programId is selected.actionId, wrapper calls Core with lowerer.state, and the
three module imports are local to the declared package. No execution or
correspondence theorem follows from these observations.

F records six specified positive candidates,152 hostile cases and11 literal
inspections; G records53 symbolic cases, including two positive fragments; H
records image construction/invariance/hostile oracles with no actual digest
vectors. Those counts and claims remain their own packet scope. Integration
neither executes them nor promotes them to full-path successes.

## Scope and procedure

Loaded the Moriarty development skill and repository AGENTS instructions.
Read README/ROADMAP/FOOTGUNS/WIKI_SCHEMA/wiki index, current reviewer routing
and AFK assignment. Startup `status --json` reported SP01.6 blocked by stale
admission inputs, missing current accounting, unavailable resource live state
and unresolved operational history; no pending transactions. No `next/run`
dispatch was needed for this authorized document review.

Graphify instructions were inspected. There is no checkout graphify-out graph;
building one would violate the output boundary. Source inspection was used
without graph creation. External sources/web were not used. There is no
implementation plan execution or automatic expansion into a formal campaign.

Only integration/SPEC.md, ISSUE-MATRIX.md and RESULT.md were written by this
agent. Source proposals, review packets, other docs, code and fixtures were
read-only. Shared-worktree changes by other agents are not attributed to this
review. Exact output hashes are returned externally, avoiding a self-containing
RESULT digest. This three-document packet is the proposed audit target.

Implementation changes: **0**. Tests added/run: **0**. Consumer executions: **0**.
Generated image/digest vectors: **0**. New providers/proofs/native results/ledger
submissions: **0**. New design adoptions or product gate closures: **0**.
Document reads and SHA256 file inspection identify evidence bytes only.
