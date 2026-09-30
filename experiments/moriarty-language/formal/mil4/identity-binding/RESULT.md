# W-D2G identity design sprint result

**Status: proposed / specified-only.** Completed deliverable is a bounded
design draft for B01–B04. No authenticated implementation, empirical identity
test result, accepted independent audit, adopted decision or acceptance-gate closure is
claimed.

## Findings and output

Repository observation: Source agreement `programId` and Core selected-builtin
`programId` have different roles. Source's selected action lowers to Core's
program selector; stage/episode have no Source/Core slots. W-D2E commits a
supplied image with neither those identity roles nor a Source registry relation.
Existing W-D2E `actionId=Action` fixtures cannot satisfy the existing W-D2F
direct action comparison with closed Source/6 selectors.

- [SPEC.md](SPEC.md) inventories present identities, proposes five nominal
  sorts and a required stage/episode wrapper, defines contextual registry
  obligations and names open authentication/migration questions.
- [DECISION-MATRIX.md](DECISION-MATRIX.md) compares strict selectors and explicit
  alias mapping, recommends strict selectors provisionally, and specifies
  exact collision, namespace, stale-anchor, duplicate-registration, alias and
  multiple-defect controls. Every test is specified-only.

Recommendation: preserve Source/6's current builtin selectors and introduce
explicit contextual stage/episode claims. Register each nominal role under a
component-wise scoped key. Same spelling across nominal sorts can be valid,
but evidence for one sort cannot prove another. An alias design would require
an explicit successor to the current W-D2F literal contract.

## Evidence and limits

Inspected repository HEAD:
`983a4bb49e3ccae399ee2514da2f6fa3f03593fd`.
This shared worktree includes uncommitted and untracked inputs; HEAD does not
commit the inspected Source/6/Core/5 or MIL/4 experiment files. No immutable
review freeze or source hash receipt is asserted for this proposal.

Primary repository inputs: effect-consumer FIELD-MAP.md and PLAN.md;
financial-agreement-source-v6-frontend.ts (AST, selected builtin constraint,
lowerer); mil4-s0-core-v5.ts (types, builtin/kind checks); mil4-s0-source-v6.ts
(unverified wrapper result); wire SPEC.md; effect-wire SPEC.md and fixtures.json.
No external sources were acquired. No runtime tests or experiments were run.
Document inspection checked the proposed expectations against parser diagnostics
and W-D2F anchor order. The actual wrong-selector parser code is
`SOURCE6_PROFILE_UNSUPPORTED`; stage/episode claims remain absent from Source.
The parent supplied Grok 4.7 xhigh review findings on the prior frozen packet.
The prior three documents are preserved byte-for-byte under
history/pre-grok-repair-01/. That review identified B02's competing head
authority and gaps in later association/image comparisons; it did not approve
the candidate. The next reviewed three-file version is preserved byte-for-byte
under history/pre-grok-repair-02/. Its fresh review reported the previous high
head-custody issue resolved, with five medium design/diagnostic findings;
GPT independently reported the wire path-channel mismatch. These findings are
recorded as review scope, not acceptance. The following reviewed version is
preserved under history/pre-grok-repair-03/. Its Grok xhigh review reported no
high findings and three remaining medium findings about missing baseline hash/
head fields and an unnumbered G03 positive. Those findings prompted the preceding
repair. That reviewed version is preserved byte-for-byte under
history/pre-grok-repair-04/. Its Grok 4.7 xhigh review identified a medium G29
ordering inconsistency: the changed retained current-action fact would reject
at tag6 before the claimed tag10 result. This repair keeps G29's action key and
retained current-action/builtin-selector fact as TransferLiteralFee, changing
only expected Core program to RepayAccrualFirst and constructor to repayment.
SPEC now states the selectedTarget tuple order and tag6 comparison explicitly;
the combined G18/G29 oracle retains its tag10 Source-literal precedence.
The present bytes have not received fresh approval.

The proposal now keeps head custody with B11: B02 retains a stage-association
reference, and B11 must authenticate registry/stage-to-snapshot linkage along
with its complete snapshot fact. Missing linkage is unavailable B11; invalid
declared snapshot/linkage evidence is invalid B11; a complete genuine
other-head fact mismatches signed preHead only after B11 authentication.
G12/G46–G48 specify these distinct cases. G30 remains an invalid B02 association
proof-content case with no head comparison.

B01 forward association checks now occur at tags4/5/6/10/16. B03 authenticates
exact prior-stage membership. B04's retained image references must link to
B05–B07 artifacts at8/11/12; its valid incompatible program/constructor facts
mismatch at10/35. G32–G45 pin those links, constructor-only mismatch and present
wrapper sourcePath errors. Wire-only subtype failures now preserve W-D2F's
sourcePath=null and inputPath=wire.field; context wrapper sourcePath remains an
explicit proposed extension. The impossible mutated actual-lowerer case was
replaced by G21, now restricted to the reachable fragment through tag6 under
hypothetical genuine same-view proofs. Tag10/16 comparisons are separate
conditional checks requiring their actual earlier gates, never part of G21's
positive fragment.

B01 associations are explicitly authenticated finite sets; the selected ID
must be a member, with no whole-set equality or ban on multi-action agreements.
Tag6/tag10 now enumerate B01 and B04 comparisons in their exact order. G49
isolates B02's retained episode association as the first mismatching fact;
G50 specifies a positive current-action membership check in a two-action set.
G38–G40 each have one wrong-image mutation and one evidence-invalid oracle.
Evidence-invalid registration/proof failures are disjoint from authentic B01
association omissions, which yield field mismatch at the selected ID's tag.
A repayment positive control is explicitly outside this bounded packet.

Baseline wire sourceHash/coreHash/policyHash and Source selected hash/policy
claims are now exact explicit values, with separate conditional expected
artifact hashes. G51–G53 specify genuine selected-artifact hash mismatches at
tags8/11/12; G38–G40 remain wrong-image linkage evidence failures only. Hash
values are symbolic expectations, not calculated artifact results. Source
intent.preHead and authenticated.head are both explicit `5`×64 claims, and
G12/G46–G48 require both literal comparisons to pass before their B11 result.
G03's extra unnumbered positive was removed; G21/G50 remain the two numbered
positive fragment/membership controls.

There are 53 numbered cases: two specified positive fragment/membership controls
and 51 rejection/control cases. G01–G50 retain their numbering; G51–G53 add the
three required hash-mismatch oracles. None were executed. Concrete proof bytes await the
selected provider format. No identity-only positive discharges B05–B16 or B17;
no new normative adoption or acceptance is claimed.

The repaired schedule was inspected against current effect-consumer FIELD-MAP.md
and PLAN.md for the parent's current grok-repair-04 packet, including
provider-fact order, V-S0-01, B11 integrity followed by core/domain/asset/head
fact checks, authenticated-head diagnostics and wire-only path channels.
This does not establish new adoption or implementation.

The development CLI `status --json` was inspected in this checkout. It reported
SP01.6 loan-swap-subset blocked by stale admission inputs, missing current
accounting and unavailable resource live state, with no pending transactions.
That operational state does not establish or deny any identity relation; this
authorized document task dispatches no registered campaign.

The graphify skill was inspected; this checkout has no graphify-out graph.
A new graph build would write outside the delegated directory and was not
performed. Primary source inspection provides the observations above.
Brainstorming was applied as context inspection and candidate comparison; the
AFK authority and delegated output scope select a draft, without user questions,
implementation, commits or a claim of adopted approval.

## Remaining work

Independent consequential review is required before adopting A or B. Real
registry authority/view selection, snapshot association and verifier behavior
are missing. B05–B07 hash images/lowering, B08–B16 authentication and B17 ledger
consumption remain separate obligations. Source template/instance reuse,
episode genesis, stage allocation, identity fields in the effect image and
cross-instance replay policy remain explicitly open.

All created files are inside identity-binding/. No effect-consumer inputs,
Source/Core modules, wire fixtures, historical evidence or gate records were
modified by this task.
