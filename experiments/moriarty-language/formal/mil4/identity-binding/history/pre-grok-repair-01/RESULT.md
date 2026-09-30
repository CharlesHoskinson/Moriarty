# W-D2G identity design sprint result

**Status: proposed / specified-only.** Completed deliverable is a bounded
design draft for B01–B04. No authenticated implementation, empirical identity
test result, independent audit, adopted decision or acceptance-gate closure is
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
The proposal separates B02 stage-fact authentication at tag4 from signed
preHead comparison at tag18, and B04 registry-fact authentication at tag6 from
signed/actual Core-program comparison at tag10. G12/G29 specify valid different
provider facts followed by field mismatches at tag18/tag10. G30/G31 separately
specify claimed facts that disagree with their unchanged proof content, which
reject as invalid evidence at tag4/tag6. G18/G21 retain tag10 mismatches with
valid earlier evidence. This aligns with a pending W-D2F repair and claims
neither its final review nor adoption. All 31 cases remain specified-only;
concrete proof bytes await the selected provider format.

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
