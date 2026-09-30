# W-D2 integration repair-01 result

**Status: proposed / specified-only, 2026-09-30.** This unreviewed successor
reconciles current F repair-05, G repair-04 and H repair-03 source bytes plus
named review evidence. [SPEC](SPEC.md) and [ISSUE-MATRIX](ISSUE-MATRIX.md) preserve
provider/proof/ledger limits and specify remaining integration refinements.
B01–B17, normative W-D2/W-D3 and Sprint1 remain open.

## Current findings and disposition boundaries

Repository observation: G04 fixes I01/I02. G29 now keeps the authenticated
current action/builtin selector TransferLiteralFee while changing only expected
Core ID/constructor. Its tag6 current-action check can pass; the expected Core
ID fails at10 only after earlier gates. G18+G29 explicitly retains the same current
action, so its direct Source-versus-signed Core ID failure at10 is reachable
under the stated prerequisites. Historical contradictions remain in the preserved
original integration packet and are not rewritten.

H03 now incorporates exact total order at 16/17 and Source authenticated
debtor/creditor before B11 facts at 35, with explicit diagnostic paths and
multiple-defect controls. It also defines six Source-body checks at 8 before
hash, keyRef admission at 12 before policy body/hash checks, authenticated suite
selection without search/fallback, and exact B14 prior grant-scope diagnostics.
SPEC aligns with those current candidate rules, replacing the earlier integration's
five-check Source tuple grouping, profile-error reinterpretation and snapshotHead
projection. Explicit tag11 package-body diagnostic/order refinements and additional
integration controls remain new proposals requiring audit.

F05 and G04 dispositions accept **bounded specified-only candidates**. Their
GPT-6.1 Sol high reviewers independently recomputed 14 and 11 manifest hashes,
respectively, and reported no high/medium defects. Grok was requested 4.7 xhigh,
returned grok-4.7-build, read embedded bytes without independently recomputing
hashes, and reported no high/medium contradiction. These dispositions establish
neither implemented authentication nor successful consumer/ledger behavior.

H03 frozen packet is `9a459b021554e26917a1bebb8a6d87a23f583d6d7f09a4accc071060482c8efa`.
GPT-6.1 Sol high independently recomputed 14 manifest hashes and gave bounded
specified-only approval. **Grok review remains pending in this successor.** H03
is not reported accepted; this integration successor is also unreviewed.

F low residue: normalize binding/retainedFactBinding, safely index obligation
rows at 35 before the later unsigned-tail length check, and pin signed/submitted
Source diagnostic paths. G low residue: tag10 step5 is unreachable as a first
failure, add an isolated genuine current-action fact difference, and pin exact
binding/fact/Source paths for abbreviated Core-program rows. These are open
implementation expectations rather than design gate closures.

The separate content codec has a high GPT Proxy admission/coherence finding.
Its descriptor values are validated then discarded; later property reads may
invoke get traps and substitute a selector/kind or cross-phase asset after
validation. The audit reports 72 original tests passed and independently recomputed
finite vectors, but those results do not establish adversarial object admission.
No codec repair result is accepted by this reconciliation. The smallest current
implementation recommendation is a bounded recursive snapshot/coherence repair
with hostile controls and fresh result audits, followed separately by a Source
AST-to-purpose1 adapter. These are future owner tasks; this agent ran no tests.

## Preservation and exact source snapshot

Worktree: `/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929`.
Inspected HEAD: `983a4bb49e3ccae399ee2514da2f6fa3f03593fd`.
Untracked/uncommitted inputs are identified by their exact hashes, not HEAD.
The three original integration documents were copied byte-for-byte before any
successor edit to `integration/history/pre-integration-repair-01/`.

| Preserved document | SHA256 |
| --- | --- |
| SPEC.md | 1dbb77f5202707a30368bb22cd0ac771d3f7a024d20a036e329e994b49e5fc46 |
| ISSUE-MATRIX.md | 7efffe616708eda034547f667b99ea18ef7281d616a3e4c89dfac2a8e811cf6d |
| RESULT.md | 43867ce6544673ccb25cba18d2c96c877dbac521b9c8dcc71fcd00453b6bcf6c |

Source paths below are relative to `experiments/moriarty-language/formal/mil4/`.
These are file-byte digests, not Source/Core/policy image commitments.

| Current input | SHA256 |
| --- | --- |
| effect-consumer/PLAN.md | c2cc802de3ae5c9a4985f2676daa1d0ba71259d513488d049c2b7a70f40c791d |
| effect-consumer/FIELD-MAP.md | 52f9a795c8a56c1474132fcaad7a39af18d6f642a237d825f9b270043c7c4182 |
| effect-consumer/EXPECTATIONS.json | e91934a5801343d98727d1881a0b0e249d143a65f13f7af8a660a7c4eb84603b |
| effect-consumer/RESULT.md | 09ef42d43fcd90b91beb7bd6f23428cd0baf260f5aa8171551faa8d9d8ee205d |
| effect-consumer/freeze-receipt.json | 0dfcbafd6c421f9afd2b7bd9d860bb71a3c45cd78e44f465ce79e0b871d80f16 |
| identity-binding/SPEC.md (G04) | 6e179d9881cf86e861e79c70760470d98c146c55966d6bf9f3a0ee283148fd88 |
| identity-binding/DECISION-MATRIX.md (G04) | e5a1789ceed91365696c2935b06f7a5721f5038dc465222ab4af7691acc7e51b |
| identity-binding/RESULT.md (G04) | e5052be84978a9bdcd122bfe76e3833db6a1fcdeaef0c63137cc1373e20b1c0f |
| hash-images/SPEC.md (H03) | 7f6ebf64002d8cbae45de22ed759eb71dce919735c4160deb014bc0673e7232d |
| hash-images/DECISION-MATRIX.md (H03) | c49e2be1dcef52d57198b6abd464ceabd6d65e0de396e2c8d3fb61ab0202d6fb |
| hash-images/RESULT.md (H03) | 3c01a688bd2751a192e4df540858f01277cb74a03ba18dfa65f98b775b33bb22 |

The unchanged F source freeze still reports six specified positive candidates,
152 hostile expectations and 11 inspections. G04 preserves 53 symbolic cases,
including two positive fragments. H03 preserves two construction positives, one
presentation invariant and 28 hostile/invariance controls. These source design
cases were not executed by this integration agent. The separate codec's empirical
result/review scope must not be conflated with their full-path expectations.

## Exact review evidence

The following files were inspected and hashed, with paths relative to
`deliverables/mil4-k-quint-sprint1-2026-09-29/audits/`.

| Record | SHA256 |
| --- | --- |
| [W-D2F-REPAIR-05-DISPOSITION.md](../../../../../deliverables/mil4-k-quint-sprint1-2026-09-29/audits/W-D2F-REPAIR-05-DISPOSITION.md) | 5a8893f4315563ae74cc5756d8fb1b566c17ad19db127abaabcaa42be568817e |
| [W-D2G-REPAIR-04-DISPOSITION.md](../../../../../deliverables/mil4-k-quint-sprint1-2026-09-29/audits/W-D2G-REPAIR-04-DISPOSITION.md) | 37f39a1da6ec443675f42929ef4b0c2bc7da6f8726520424d25d04479fcfb4f8 |
| w-d2f-grok-repair-05-candidate-packet.md | 13408bbbefd0b7925f956ef6851560853b16faaea7fdd89ae74d5950a1736b83 |
| w-d2g-grok-repair-04-candidate-packet.md | acf54f334d395390c4293efcabde84ce7880672b54a5b643c1821db21253d7ed |
| w-d2h-grok-repair-03-candidate-packet.md | 9a459b021554e26917a1bebb8a6d87a23f583d6d7f09a4accc071060482c8efa |
| w-d2h-grok-repair-03-gpt-6.1-sol-high-audit.md | 2000ea3f725e616d3b91d70c2ff04bad104699af6be5f08e6ae8902ddc26b68f |
| w-d2h-image-codec-candidate-packet.md | 34e34595aaab4094ee4735ce81b511b873f82fe3ad7f89648ebcfecb7757d332 |
| w-d2h-image-codec-gpt-6.1-sol-high-audit.md | fef909b55e47957b50237f26d6c4d5b6c2f17d8a08f6d05c6d58bba4cdf3d81d |

Historical H01/H02 source bytes and original integration assertions remain in
their existing history directories. Current H03 source bytes, rather than H01,
control this repair. H03 incorporation is a candidate design fact; one-provider
bounded approval is not its required complete acceptance disposition.

## Authorized scope and evidence limits

Moriarty development skill remained loaded. Refreshed `status --json` reports
SP01.6 execution blocked by stale admission, missing current accounting,
unavailable live resource state and unresolved operational history; no pending
transactions. No execution dispatch was made for this document task. No external
web, graph creation, new registry/proof assumption or campaign expansion occurred.

Writes by this agent are only the three current integration documents and their
three exact history copies. Other agents' shared-worktree edits are not attributed
to this task. Current output hashes are returned externally so RESULT need not
contain its own digest. This successor is the new unreviewed audit target.

Implementation changes: **0**. Tests added/run: **0**. Consumer executions: **0**.
Generated image vectors/providers/proofs/native results/ledger submissions: **0**.
New design adoptions or product gate closures: **0**. Read-only document/hash
inspection establishes provenance of the stated evidence only.
