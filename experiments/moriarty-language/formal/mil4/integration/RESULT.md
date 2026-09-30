# W-D2 integration repair-02 result

**Status: proposed / specified-only, 2026-09-30.** This unreviewed successor
reconciles current F repair-05, G repair-04 and H repair-04 source bytes plus
named bounded design dispositions. [SPEC](SPEC.md) and
[ISSUE-MATRIX](ISSUE-MATRIX.md) retain all provider/proof/ledger limits.
B01–B17, normative W-D2/W-D3 and Sprint1 remain open.

## Current findings and disposition boundaries

G04 still fixes I01/I02: G29 retains Transfer current action/builtin selector
and changes only expected Core ID/constructor. G18+G29 preserves the same current
action, with conditional first mismatch at10 after intervening gates.
Those fixes do not supply an executed registry proof.

H04 repairs the two H03 medium findings. At8 an asset/scale-only change with
compatible prior facts and old hash claims fails computed-source-hash equality.
A permitted selector/constructor switch instead changes selectedActionId and
fails the fourth prior-body comparison before hash; an unpaired kind fails
purpose1 image-domain admission even earlier. H-H14C now distinguishes direct
Source literal, B10 missing/invalid, genuine prior scope, and B01/B10/B05/B07
retained asset failures with their exact diagnostic owners and paths. Current
SPEC mirrors these repaired source bytes, not the older H03 constructor wording.

H04 retains exact total order 16/17 and Source authenticated debtor/creditor
before B11 facts at35. It retains six Source-body checks at8, keyRef admission before
nine policy-body checks/hash12, authenticated suite selection without any
search/fallback, and exact B14 prior grant-scope paths23. Extra integration
refinements, including explicit tag11 package-label ordering/diagnostics and
B06 constructor placement within35, remain unreviewed proposals.

F05/G04/H04 dispositions accept **bounded specified-only design candidates**.
GPT-6.1 Sol high independently recomputed 14/11/14 manifest hashes respectively
and reported no high/medium defects. Grok was requested 4.7 xhigh, returned
grok-4.7-build, read embedded bytes without independently recomputing hashes,
and reported no high/medium contradiction. H04 exact packet is
`1e47bfc15567b12630da95814807caeffe739776a42cb9041fcfebf1b5fe1a3b`.
Its disposition supersedes H03's pending design status; neither design approval
nor this integration establishes implemented image/provider semantics or W-D2.

F low residue remains diagnostic key normalization, safe obligation indexing
before 35 without relocating unsigned-value checks, and exact signed/submitted
Source paths. G low residue remains unreachable tag10 step5 as first failure,
an isolated genuine current-action control, and exact abbreviated Core-program
binding/fact/Source paths. H04 low residue is package unpaired-kind failure at11,
purpose4 actionId/kind self-admission separately from signed equality at35, and
keeping keyRef image-domain admission explicit in every tag12 summary.

The earlier 72-test codec Proxy high finding remains historical evidence. The
repaired 87-test packet and the separate 50-test Source adapter subsequently
received two bounded content approvals each. See the codec and adapter
review dispositions in the audit directory. These reviews do not change this
specified-only integration result or supply selected-artifact authority.

The next implementation task is to freeze B01–B07 decisions and implement
fail-closed provider boundaries. Content encoding and Source projection still
establish no selected-artifact authority, loaded-code correspondence or full
semantic consumer. Actual B01 authority,
B05–B07 artifact authentication, B08–B16 providers/history transport and B17
atomic ledger application remain independent requirements.

## Preservation and exact source snapshot

Worktree: `/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929`.
Inspected HEAD: `983a4bb49e3ccae399ee2514da2f6fa3f03593fd`.
Untracked/uncommitted inputs are identified by their exact hashes, not HEAD.
The original integration documents remain in pre-integration-repair-01. The
three immediately preceding repair-01 documents were copied byte-for-byte
before any repair-02 edit to `integration/history/pre-integration-repair-02/`.

| Preserved original document (pre-integration-repair-01) | SHA256 |
| --- | --- |
| SPEC.md | 1dbb77f5202707a30368bb22cd0ac771d3f7a024d20a036e329e994b49e5fc46 |
| ISSUE-MATRIX.md | 7efffe616708eda034547f667b99ea18ef7281d616a3e4c89dfac2a8e811cf6d |
| RESULT.md | 43867ce6544673ccb25cba18d2c96c877dbac521b9c8dcc71fcd00453b6bcf6c |

| Preserved repair-01 document (pre-integration-repair-02) | SHA256 |
| --- | --- |
| SPEC.md | 4789eff7e59646fc4270a8dc040fe5fd06b785dd568ed7ff61fc71fe236d1939 |
| ISSUE-MATRIX.md | 032fdf00950d36b409a50b7ee43f5668020755504531c4d4037dfea6a5ac9d20 |
| RESULT.md | aea386acc947e3312efbc84c3eaea42eefafb21f85fca68e9e6508efeed1d36c |

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
| hash-images/SPEC.md (H04) | 7adb21b56b75e3246154552de198291bddf38154ba855c4a07f79d85bae60933 |
| hash-images/DECISION-MATRIX.md (H04) | 4f4020d588c0bd541185c387d2679c57271973774895b1c4f1631e898ebb2507 |
| hash-images/RESULT.md (H04) | 8eb9a6137871dd6cf59a894cd036071d2acb8eb3a0ec46d4027634201b1e899d |

The unchanged F source freeze still reports six specified positive candidates,
152 hostile expectations and 11 inspections. G04 preserves 53 symbolic cases,
including two positive fragments. H04 preserves two construction positives, one
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

Additional current evidence, with the same audits/ path root:

| Record | SHA256 |
| --- | --- |
| W-D2H-REPAIR-04-DISPOSITION.md | d4cb1a7bf3f90c9eacadc5311ccfe82a3fb3fccf75ffa0018ad919dbab6e06cb |
| w-d2h-grok-repair-04-candidate-packet.md | 1e47bfc15567b12630da95814807caeffe739776a42cb9041fcfebf1b5fe1a3b |
| w-d2h-grok-repair-04-gpt-6.1-sol-high-audit.md | a923d737b01bb57a14f5104e286ae5e72db885874f771053e15982e9f621260c |
| w-d2h-image-codec-proxy-repair-01-candidate-packet.md | 3876143201ceaa93d378a6125a899233893b88d55629580a2d967eeaf4bf5973 |
| w-d2h-image-codec-proxy-repair-01-gpt-6.1-sol-high-audit.md | 765f08ad877077b2a9dfb716264cb119b38c712ac40fa138a666caa1c7a70da8 |
| w-d2h-source-image-adapter-candidate-packet.md | aa9e9fa530447376d583750c073ee43b24d27856ff65293c524bde5b4b973582 |

H03 packet/GPT and original codec review rows above remain historical evidence,
not current acceptance. Current H04 source bytes control this repair and its
bounded design disposition records both required reviewers. Codec87 and Source adapter50 subsequently received bounded two-review dispositions. Older integration and H proposal histories
remain intact and are not rewritten. Live paths can change after this snapshot.

## Authorized scope and evidence limits

Moriarty development skill remained loaded. Refreshed `status --json` reports
SP01.6 execution blocked by stale admission, missing current accounting,
unavailable live resource state and unresolved operational history; no pending
transactions. No execution dispatch was made for this document task. No external
web, graph creation, new registry/proof assumption or campaign expansion occurred.

Writes by this agent are only the three current integration documents and their
three exact repair-02 history copies. Other agents' shared-worktree edits are not attributed
to this task. Current output hashes are returned externally so RESULT need not
contain its own digest. This successor is the new unreviewed audit target.

Implementation changes: **0**. Tests added/run: **0**. Consumer executions: **0**.
Generated image vectors/providers/proofs/native results/ledger submissions: **0**.
New design adoptions or product gate closures: **0**. Read-only document/hash
inspection establishes provenance of the stated evidence only.
