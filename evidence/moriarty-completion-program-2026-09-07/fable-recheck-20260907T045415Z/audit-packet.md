# Independent Moriarty completion PLAN audit
You are the independent Fable reviewer requested by the user. Audit the frozen S2 plan below. You are not its author. No other reviewer verdict is supplied.
User requires plans and execution addressing all seven gaps: Preview financial operation vs local evaluator; bounded DSL syntax/types/semantics/compiler mapping; real native recursive proof after reviewed k17 encoding/resource decision; contract/intent/transition/history proofs in actual acceptance; private witness handoff and split/join; complete ACTUS/DeFi including held-outs; compiler-ledger correspondence and durable authorization/replay.
Assess completeness, feasible dependency ordering, actual proof and target acceptance, numeric/type/finite guarantees, private composition, complete coverage, reproducibility, cumulative resource/attempt limits, and honest completion. These are plans, not implementation results. Native circuit fit and ledger verifier compatibility are unproven. No tools, implementation, network, proofs or model calls are permitted.
Return only JSON with kind=independent-planning-audit, verdict=APPROVED or BLOCKED, candidate_sha256, findings=[{id,severity,blocking,title,locators,observation,correction}], coverage_assessment, residual_limits. A substantive verdict is mandatory; identity is checked from CLI metadata, not your self-description. Distinguish blocking contradictions from refinements and empirical gates appropriately left to execution.

Frozen manifest:
{
  "kind": "specified-only-plan-candidate",
  "created_at": "2026-09-07T04:17:06.927548+00:00",
  "base_commit": "b55eca82b5218b4760abd4edf6127b7c3c485514",
  "files": [
    {
      "path": "openspec/MORIARTY-COMPLETION-PROGRAM.md",
      "sha256": "b3ff52e21a254e444aefc5ea3ddfe1c25ac377f250e845ba7ef26780fad67082"
    },
    {
      "path": "openspec/moriarty-completion-program.json",
      "sha256": "3dd20c612f3f87b8af70750e38e8841e704db6d3e9337a6074bbf7d285d0571a"
    },
    {
      "path": "raw/assignments/moriarty-completion-loop-2026-09-07.md",
      "sha256": "3a3ea8f6e45d2c453fba3745d0fdf3bc35002b17a3548eb198bb61277ea5be0c"
    },
    {
      "path": "openspec/changes/mc01-bounded-language/README.md",
      "sha256": "6cb214b135bb06927af84ee85ee7fceaeaae7174f0d49647122669dedf630705"
    },
    {
      "path": "openspec/changes/mc01-bounded-language/design.md",
      "sha256": "557f8441a11fa1fcc19fe3e8100c76251933b64abff8ce88ff4e56ea08fb83bd"
    },
    {
      "path": "openspec/changes/mc01-bounded-language/proposal.md",
      "sha256": "2d8c087044b20abb8a4aef9a4f4f32543c02ce15856080ce9c482e11f4e3e6b0"
    },
    {
      "path": "openspec/changes/mc01-bounded-language/specs/mc01-bounded-language/spec.md",
      "sha256": "390653695228ca7e03fa4cd2fe573b02eb0d74f70384ee2fc7330db12da947ed"
    },
    {
      "path": "openspec/changes/mc01-bounded-language/tasks.md",
      "sha256": "0550923825a19aa6a52c9bc97ceff4f6b27c890ea7c74608fb3a854ffe2eaf4c"
    },
    {
      "path": "openspec/changes/mc02-preview-financial-operation/README.md",
      "sha256": "17bf896435de1237b63458e1e1bf6c07215968f34c7f79fcf324f9f11d63d4f0"
    },
    {
      "path": "openspec/changes/mc02-preview-financial-operation/design.md",
      "sha256": "43a155a4fabfd9cbbdfc8303efb3400b498195dcfbf17280ff2beda04dd512f2"
    },
    {
      "path": "openspec/changes/mc02-preview-financial-operation/proposal.md",
      "sha256": "b74c2badb0023d2b10b625ba3aac6ae97e63f24a16e4a8c5f34e8cdc9940dae9"
    },
    {
      "path": "openspec/changes/mc02-preview-financial-operation/specs/mc02-preview-financial-operation/spec.md",
      "sha256": "cfd0008bee46f4746a0193248f590d0993076c98da94c024d3849320cb06da35"
    },
    {
      "path": "openspec/changes/mc02-preview-financial-operation/tasks.md",
      "sha256": "ab67f66e9293e986330b0e54bbca313c5c409589d399b4b1f3aef1fedb13e858"
    },
    {
      "path": "openspec/changes/mc03-native-recursive-proof/README.md",
      "sha256": "b1428f0c440d4d1c0c4489d592c0a13e3266966cfcd85e847c3605b8c5582c9c"
    },
    {
      "path": "openspec/changes/mc03-native-recursive-proof/design.md",
      "sha256": "99852894f7345863c691a935e8a9aa4564497491c65ac46961e841c9a32c48b4"
    },
    {
      "path": "openspec/changes/mc03-native-recursive-proof/proposal.md",
      "sha256": "72edeb26a64142e0efd5492931ae2d492827d00ce09ea8ee1ca181fc5654b43c"
    },
    {
      "path": "openspec/changes/mc03-native-recursive-proof/specs/mc03-native-recursive-proof/spec.md",
      "sha256": "8646e60d369ca6086b3cef3f247a8d06d4362237f2587ca7d3cd2bfd5f0f4b4e"
    },
    {
      "path": "openspec/changes/mc03-native-recursive-proof/tasks.md",
      "sha256": "ebc8434218e35cd1a69e0bfbd6e7d08b87c47945603ee84125a194bd72e9f7b1"
    },
    {
      "path": "openspec/changes/mc04-ledger-correspondence-and-consumption/README.md",
      "sha256": "8225bac494bc4a78bafef22488bd572eb4cbdc04658b5cdd1add86fa1df78692"
    },
    {
      "path": "openspec/changes/mc04-ledger-correspondence-and-consumption/design.md",
      "sha256": "4b7e8f18c47389f50735257de42a8dfdeaa73063eb21f1e732227171818d657c"
    },
    {
      "path": "openspec/changes/mc04-ledger-correspondence-and-consumption/proposal.md",
      "sha256": "4ec8f7688ebdafaf8dbdce6163a20eab0a2e0023f424efcfe99c57e834fd1c24"
    },
    {
      "path": "openspec/changes/mc04-ledger-correspondence-and-consumption/specs/mc04-ledger-correspondence-and-consumption/spec.md",
      "sha256": "bb13858f4f61c44710e17676fab930713696fafb2bd953e277687875b107ede4"
    },
    {
      "path": "openspec/changes/mc04-ledger-correspondence-and-consumption/tasks.md",
      "sha256": "d28dc4d61179e57ada731f65b9005bf63ca25adb28052c4b11026e32252f2ab5"
    },
    {
      "path": "openspec/changes/mc05-mandatory-claim-acceptance/README.md",
      "sha256": "d15d7f439f525b17a9d2faaf433601070eb90d169e55d3af427aa2bf61eb4858"
    },
    {
      "path": "openspec/changes/mc05-mandatory-claim-acceptance/design.md",
      "sha256": "3d8b10f241fc1734723ece66dfb6610eacfe318b4b51072f48e2304bed05fbe0"
    },
    {
      "path": "openspec/changes/mc05-mandatory-claim-acceptance/proposal.md",
      "sha256": "f010a84a6c2cc242491e8207006b05ccff5f3a2f03ee993b5143f3fb12a23579"
    },
    {
      "path": "openspec/changes/mc05-mandatory-claim-acceptance/specs/mc05-mandatory-claim-acceptance/spec.md",
      "sha256": "ade80f5b0c2250c4ce1c8523e60cab4cbf156bd3b082e83df7b953fa09c7d5b7"
    },
    {
      "path": "openspec/changes/mc05-mandatory-claim-acceptance/tasks.md",
      "sha256": "bbe489c24248dd389eb65bb200d8304436fe4468367fbfb3cb619641f6b4e91e"
    },
    {
      "path": "openspec/changes/mc06-private-handoff-and-composition/README.md",
      "sha256": "b360378420fb4b1387a13f71dfa808b166e5bbfd094aae97249c34916815367c"
    },
    {
      "path": "openspec/changes/mc06-private-handoff-and-composition/design.md",
      "sha256": "e20c0199d46835db7442ed7bac78b3aff3506271135a1835b54f242f2c966e35"
    },
    {
      "path": "openspec/changes/mc06-private-handoff-and-composition/proposal.md",
      "sha256": "990bbf1f74d07a410d765fecbe2980a39daded033af36cc4f2cb6de9771cf99c"
    },
    {
      "path": "openspec/changes/mc06-private-handoff-and-composition/specs/mc06-private-handoff-and-composition/spec.md",
      "sha256": "4f304884d517d75e0f913e7f9d7898552527085df63c838cef987261db63487f"
    },
    {
      "path": "openspec/changes/mc06-private-handoff-and-composition/tasks.md",
      "sha256": "0c83bb08c82fe80fc44d04a6b810c2bf0d4c660512a88d97fea542dbab66158e"
    },
    {
      "path": "openspec/changes/mc07-complete-financial-conformance/README.md",
      "sha256": "f2bd1bc608948dcd531eccaaaae53933f775316ec33eef7b8e5050c56925c981"
    },
    {
      "path": "openspec/changes/mc07-complete-financial-conformance/design.md",
      "sha256": "52f31084849bcc56650a659301dfec0f09514e98df3212c96debcace559aa35b"
    },
    {
      "path": "openspec/changes/mc07-complete-financial-conformance/proposal.md",
      "sha256": "e6238cc0da458d6e9176021d02dbf8ff84298b61c59f49cf4ff4b17bc204c78b"
    },
    {
      "path": "openspec/changes/mc07-complete-financial-conformance/specs/mc07-complete-financial-conformance/spec.md",
      "sha256": "5090a74d39195371d111af9acb182337c88f0d40338f40632f2a640d95d0dcca"
    },
    {
      "path": "openspec/changes/mc07-complete-financial-conformance/tasks.md",
      "sha256": "62d0aec928d71d45756d7d4a9d89f8d1288c2878811280c06cc36a8804c356f0"
    },
    {
      "path": "openspec/changes/mc08-release-evidence-and-developer-flow/README.md",
      "sha256": "b9ee7882e389dddd98e6f05c3f562f88ba03e9b69c66df7752798d33b8923aa9"
    },
    {
      "path": "openspec/changes/mc08-release-evidence-and-developer-flow/design.md",
      "sha256": "3219873585b0e816069cd821418e2a8a901ef0551ab5fa800af9ca186808d5b7"
    },
    {
      "path": "openspec/changes/mc08-release-evidence-and-developer-flow/proposal.md",
      "sha256": "d413794eea9064a34b1dd7a1abff81411d47257a81334b88fea7a2b0682acb98"
    },
    {
      "path": "openspec/changes/mc08-release-evidence-and-developer-flow/specs/mc08-release-evidence-and-developer-flow/spec.md",
      "sha256": "378f8cf3be215a8b0b8b36a7b739b0a1f77923cf6271f11cd61c5d67628407fb"
    },
    {
      "path": "openspec/changes/mc08-release-evidence-and-developer-flow/tasks.md",
      "sha256": "cca82f871ff966278ea85d2ffe16389554444e84362f32634ed92309b5eb4387"
    }
  ],
  "candidate_sha256": "99bbf6aca17ffa702820ac8925a10ad4bab4b25ef7e6e25de843ee1223998e02",
  "predecessor": "plan-candidate-02.json"
}

--- FILE openspec/MORIARTY-COMPLETION-PROGRAM.md ---
# Moriarty target-first completion program

Status: S2, specified-only. The plan files do not establish an active execution loop.
Authority: [user request](../raw/assignments/moriarty-completion-loop-2026-09-07.md).
Machine register: [moriarty-completion-program.json](moriarty-completion-program.json).
Execution and review receipts: [program evidence](../evidence/moriarty-completion-program-2026-09-07/).

## Intended result and current foundation

Developers will author bounded financial contracts, inspect outcomes, sign authority, prove transitions, and submit them on Preview.
Acceptance must enforce contract properties, intent refinement, transition validity, and compliant predecessor history.
Every guarantee names its assumptions, supported profile, complete effects, and finite bounds.
Turing incompleteness establishes neither financial correctness nor practical proof feasibility by itself.

The retained local evaluator supports loan and swap examples with local outcome authorization.
Its nonce history is not durable, and its mandatory native proofs remain unavailable.
[Preview evidence](../evidence/midnight-preview-2026-09-07/README.md) records a finalized deployment, call, and exact message readback.
It does not record a financial transfer comparison or Moriarty PCD acceptance.
[Native R3 evidence](../evidence/moriarty-native-ivc-r3-2026-09-07/README.md) records row exhaustion at k17.
No recursive proof exists at that boundary.
The target study inventories ACTUS and DeFi; inventory is not full conformance.

## Work packages and checklist mapping

Each link contains a proposal, design, unchecked tasks, and normative acceptance scenarios.
Implementation paths and verification commands are planned entry points unless retained evidence says otherwise.

| Package | Reviewable outcome | Dependencies | Requested gap |
|---|---|---|---|
| [MC01](changes/mc01-bounded-language/README.md) | Grammar, typing, bounded semantics, canonical encoding, initial Compact lowering | Existing target study | DSL definition |
| [MC02](changes/mc02-preview-financial-operation/README.md) | Real loan and swap transfers; complete finalized effects match independent expectations | MC01 | Preview financial operation |
| [MC03](changes/mc03-native-recursive-proof/README.md) | Reviewed encoding; two recursive financial steps; independent retained-proof verification | MC01 | Native recursion |
| [MC04](changes/mc04-ledger-correspondence-and-consumption/README.md) | Proof/ledger compatibility, compiler correspondence, durable authority and consumption | MC01–MC03 | Correspondence and replay protection |
| [MC05](changes/mc05-mandatory-claim-acceptance/README.md) | All four mandatory claims enforced in actual acceptance | MC03, MC04 | Mandatory proof acceptance |
| [MC06](changes/mc06-private-handoff-and-composition/README.md) | Separate-party witness handoff; valid private split/join and residual obligations | MC05 | Handoff and composition |
| [MC07](changes/mc07-complete-financial-conformance/README.md) | Full ACTUS/DeFi and held-out behavioral coverage | MC01, MC04–MC06 | Financial conformance |
| [MC08](changes/mc08-release-evidence-and-developer-flow/README.md) | Reproducible developer workflow and independently audited completion dossier | MC01–MC07 | Combined acceptance |

```mermaid
flowchart LR
  Targets[ACTUS + DeFi + PCD + intents] --> MC01
  MC01 --> MC02
  MC01 --> MC03
  MC02 --> MC04
  MC03 --> MC04
  MC04 --> MC05
  MC05 --> MC06
  MC06 --> MC07
  MC07 --> MC08
```

The machine register contains every direct dependency; the diagram shows the main path.
Read-only compatibility inspection and conformance source-gap resolution can begin alongside MC01.
Package dependencies gate accepted implementation, not independent source inspection.
Inspect the native-to-ledger verifier interface before spending on a production adapter.
If no compatible interface exists, record the exact missing boundary and stop that adapter.
Do not replace it with a host-computed verification bit.

## Design decisions and checkpoints

Use the existing unified semantic proposal as the starting point.
Trace Core operations to financial behaviors before freezing the language profile.
Do not turn financial product names into primitive correctness claims.
Freeze the smallest common profile through MC01, then extend it through explicit reviewed versions.
Resolve foundational numeric and lifecycle ambiguities before dependent execution.
Keep unresolved target-specific source questions visible until MC07 resolves them.
Do not let an MC01 freeze exclude a required later target.

This sequence supplies early developer and financial feedback while preserving explicit proof gates.
A backend-first rewrite would postpone the target and authoring decisions that caused the prior detour.
A single full-corpus implementation would delay evidence about proof and ledger feasibility.
The selected sequence uses bounded slices, then requires full coverage before completion.

MC02 transactions are integration experiments until MC05 completes mandatory acceptance.
MC03 proves the retained financial episode under its fixed authority assumptions.
MC04 and MC05 must extend that evidence to actual authorization and ledger acceptance.
MC06 must prove its additional split/join relations; the MC03 proof cannot substitute for them.
Each extension requires a reviewed relation, valid feasible examples, and meaningful rejection controls.

Formal outputs use pinned Lean toolchains and dependencies where the package specifies Lean.
Name every theorem, input domain, assumption, and connection to executable code.
Reject `sorry`, `admit`, unchecked axioms, and assumed compiler or verifier correctness as correspondence evidence.
Record the theorem axiom audit and the remaining trusted computing base.
A successful Lean build alone does not establish the requested correspondence.

## Execution algorithm

Use a new Codex runtime goal bound to this charter and the machine register.
Never resume or falsely complete the superseded A4/A5 runtime goal.
A manifest, checkpoint, shell background process, or task list cannot establish that the runtime loop is armed.
Record the actual goal creation result and runtime state before reporting arming.

1. Recover the latest user authority and checkpoint.
2. Verify the register, candidate digests, remaining limits, and prerequisite evidence.
3. Select the first eligible unchecked task in dependency order.
4. Create an isolated implementation worktree from the accepted candidate.
5. Freeze its five-part brief, exact commands, owned paths, and terminal predicates.
6. Verify the existing Foreman runtime and required worker readiness.
7. Create the immutable external Endstop contract before the first actionful worker dispatch.
8. Dispatch through the contract-bound queue with durable process ownership.
9. Add failing behavioral tests before implementation changes.
10. Run the package checks and retain failures with actual resource receipts.
11. Request independent Fable and GPT-6 result audits of the frozen candidate.
12. Correct blocking findings within the existing contract limits.
13. Recheck affected predicates and obtain updated candidate-bound verdicts.
14. Admit only the owned, tested, audited candidate through the repository gate.
15. Update task status, evidence, source links, and the typed checkpoint.

The latest user request authorizes this sequence without another generic design-confirmation round.
It does not override a failed acceptance gate or authorize unlimited retries.
Prefer the configured cross-vendor worker after readiness succeeds.
The user-required Fable and GPT-6 audits replace the default auditor pairing for these packages.
No author may approve their own result.
If the worker route fails, retain that failure and disclose any proposed route substitution.
Do not repair Foreman as a side task.

Allowed package states are `specified-only`, `ready`, `working`, `pending-audit`, `resource-stopped`, `interface-blocked`, and `complete`.
Only recomputed acceptance and both substantive audits permit `complete`.
Independent source inspection can continue when an implementation dependency is blocked.
Stop dependent dispatch when a required audit or runtime interface is unavailable.

## Resource and terminal contract

These are ceilings, not spending targets or automatic evidence of feasibility.
Freeze concrete commands and smaller task limits before dispatch.
The external Endstop contract must enforce cumulative counters across restarts and worktrees.
Use one heavy build, proof, or conformance process group at a time.
Keep memory usage within available host capacity, even below these ceilings.

| Scope | Initial ceiling | Terminal condition |
|---|---|---|
| Program actionful subprocess work | 8 cumulative hours; 24 worker dispatches | Either ceiling stops new actionful dispatch |
| One worker round | 30 minutes; 8 GiB process group; 2 CPU jobs | Timeout, memory breach, or failed terminal predicate |
| One task | Initial candidate plus at most 2 justified corrections | Third failed candidate stops the task |
| One substantive audit | 10 minutes; 1 initial review plus at most 2 correction reviews per reviewer and task | Unavailable identity, timeout, exhausted credit, or exhausted rounds |
| Native revision-01 preflight | 10 cumulative minutes; 8 GiB; 2 CPU jobs; no proving | First failed encoding or fit predicate |
| Native revision-01 proving | 20 cumulative minutes; 8 GiB; 2 CPU jobs; k at most 17; 256 MiB retained outputs | First synthesis, proof, control, time, memory, or output failure |
| MC03 revision-01 positive campaign | One campaign containing exactly two original R2 loan transitions | No restart after a failed campaign |
| Public financial cases | At most 2 submissions per case; 20 minutes per attempt | Failed finality/effect gate stops that attempt |
| Program Preview submissions | At most 24 submissions, including deploys and failed submissions | No attempt-counter reset across packages |
| Test assets | At most 1,000 tNIGHT total gross external debit; 100 per case | Balance, fee, closure reserve, or denomination check fails |
| Program retained generated evidence | 2 GiB, excluding already retained dependencies/SRS | Stop before the limit; preserve failures |

Count all program worker, verification, correction, and substantive-audit process time in the eight-hour ceiling.
Maintain public-submission and gross-debit counters outside disposable worktrees.
Do not net refunds against gross debit.
Measure DUST separately in its actual units.
Freeze the maximum DUST spend and closure reserve from fresh read-only estimates before each public campaign.
Reject submission if those estimates or required token identities are unavailable.
Use the existing dedicated Preview wallet and externally stored secrets.
Never print seeds, secret snapshots, or private witnesses in tracked evidence.
Use local Docker before public attempts; make no mainnet submission.

Revision-01 is new work explicitly requested after the failed original R3 experiment.
Retain the original resource record and link it as the predecessor.
Both auditors must approve the changed encoding, complete relation, SRS, exact command, and revised contract before native launch.
Plan review alone cannot approve an encoding that has not been implemented and checked.
The k17 ceiling allows a smaller checked encoding; it does not establish that one will fit.
If k17 remains infeasible, record the decision evidence and stop.
A larger k, unallocated campaign, or exhausted terminal contract requires a new reviewed proposal and explicit user authorization.
Do not hide state in unchecked commitments or spend the original unused budget.

### Allocated native extension campaigns

The two-transition restriction applies only to MC03 revision-01.
This program separately allocates the following extensions; it does not treat MC03 proofs as their evidence.
Each campaign uses owned `proof/` artifacts listed in its package design and tasks.
Each contract links its predecessor and has separate persistent counters under the common program ceiling.
All campaigns retain k at most 17, 8 GiB process-group memory, and two CPU jobs.
Each allows ten cumulative preflight minutes and 256 MiB retained outputs.
All preflight, proving, and verification time consumes the eight-hour program ceiling.
Each campaign has one frozen positive-case manifest, one execution allocation, and no automatic failed-campaign retry.
Both auditors must approve its implemented relation, checked encoding, exact commands, SRS, and contract before launch.
A failure stops that campaign and dependent acceptance without converting earlier proofs into extension evidence.

| Package campaign | New relation and positive evidence | Proving and retained-verification ceiling |
|---|---|---|
| MC04 adapter-profile-01 | Compiled loan and swap transition relations; at most 6 positive transitions; exact ledger verifier and complete effects | 20 cumulative minutes |
| MC05 mandatory-claims-01 | Actual contract, intent, transition, and history claims; dynamic signed authority; at most 8 positive transitions | 20 cumulative minutes |
| MC06 composition-01 | Private successor, split, two branch histories, and join; at most 8 positive transitions | 20 cumulative minutes |
| MC07 financial-coverage-01 | Required extended profiles and row-specific certificates; at most 349 positive episodes across pinned cases | 120 cumulative minutes |

MC04 first probes the retained MC03 proof interface without claiming swap or dynamic-authority correspondence.
MC04 then implements its own loan/swap relation before asserting correspondence for either supported compiled family.
MC05 proves its additional signed-authority and mandatory-claim predicates and requalifies affected MC04 correspondence.
MC06 proves composition predicates and requalifies affected acceptance and correspondence.
MC07 freezes every episode, transition count, and profile bound before dispatch.
Its episode ceiling permits coverage; it does not allow an unbounded number of transitions within one episode.
Freeze independent expected fields and invalid proof/context controls for every extended profile.
Use bounded batches when a campaign exceeds one worker round.
Each batch stays within the 30-minute worker ceiling and consumes the same persistent campaign counters.
Do not restart successful batches or replace required cases to fit the remaining budget.
The 349-episode cap is not the conformance denominator; all 277 ACTUS fixtures and 72 DeFi rows remain mandatory.
If required cases cannot fit, retain resource-stopped status and propose the next bounded decision.
No allocation guarantees circuit fit, ledger compatibility, or completion within its ceiling.

After an external failure, retry only after evidence of an external state change.
Do not poll a credit-limited model or restart an exhausted proof campaign.
Record blockers immediately; apply the runtime's three-consecutive-turn rule before marking a goal `blocked`.
Preserve completed work when a ceiling prevents full completion.

## Independent audit protocol

Required reviewers are exact `claude-fable-5-1` and a fresh `gpt-6-astra` agent.
The current request controls these eight packages; it does not close older three-provider Council obligations.
Keep prior Grok, Fable, and GPT review requirements visible in the MC08 crosswalk.

Before each audit, freeze the candidate commit, file manifest, source pins, acceptance predicates, and evidence digests.
Use a cold packet containing the relevant diff and reproducible commands.
Each reviewer works independently and receives no other review verdict before their first assessment.
Retain a structured verdict, severity, exact locators, missing evidence, and residual limitations.
The author cannot act as the independent GPT-6 reviewer.
Record tool-selected GPT-6 identity and the agent identifier; model self-identification is insufficient.

For Fable, use the installed bounded tool-free readiness canary from a temporary directory.
Require `modelUsage["claude-fable-5-1"].canonicalModel` to equal `claude-fable-5-1`.
Require the same verified identity in the substantive result receipt.
A canary, alias, authentication status, or empty model usage cannot substitute for a substantive audit.
No alternate Claude model may silently replace Fable.

Audit grammar, numeric semantics, target traceability, complete effects, proof binding, ledger enforcement, and durable consumption.
Audit private witness boundaries, composition, held-outs, provenance, and resource claims where relevant.
Reviewers must distinguish specified-only predicates from independently recomputed results.
Any unresolved blocking finding prevents acceptance.
Source changes invalidate affected reviews and checks until reviewers bind updated verdicts to the corrected candidate.
Preserve disagreements and rejected approaches with their reasons.

## Completion and broader release scope

Close this program only after MC01–MC08 satisfy their actual predicates and both required result audits.
Require finalized financial effects, real recursion, mandatory acceptance, composition, complete conformance, and correspondence evidence.
Require a fresh-checkout reproduction and a developer demonstration using the accepted implementation.
Do not substitute checkboxes, model approval, mock proofs, or expended resources for those results.

MC08 maps all seven requested gaps and legacy G01–G24 requirements.
Unperformed pilots, licensing work, baselines, or additional release assurances remain explicit release blockers.
Completing these seven gaps does not automatically establish production release readiness.
The final report must state the proved scope and every broader remaining blocker.


--- FILE openspec/moriarty-completion-program.json ---
{
  "schemaVersion": 1,
  "program": "moriarty-target-first-completion-2026-09-07",
  "status": "planned-not-armed",
  "authority": "raw/assignments/moriarty-completion-loop-2026-09-07.md",
  "charter": "openspec/MORIARTY-COMPLETION-PROGRAM.md",
  "runtimeGoalId": null,
  "auditModels": [
    "claude-fable-5-1",
    "gpt-6-astra"
  ],
  "packages": [
    {
      "id": "MC01",
      "change": "mc01-bounded-language",
      "dependencies": [],
      "status": "specified-only",
      "acceptance": "openspec/changes/mc01-bounded-language/specs/mc01-bounded-language/spec.md",
      "tasks": "openspec/changes/mc01-bounded-language/tasks.md",
      "evidenceRoot": "evidence/moriarty-completion-program-2026-09-07/MC01",
      "commands": [
        "npm --prefix experiments/moriarty-language ci",
        "npm --prefix experiments/moriarty-language run build",
        "npm --prefix experiments/moriarty-language test",
        "npm --prefix experiments/moriarty-developer-mock run build",
        "npm --prefix experiments/moriarty-developer-mock test"
      ]
    },
    {
      "id": "MC02",
      "change": "mc02-preview-financial-operation",
      "dependencies": [
        "MC01"
      ],
      "status": "specified-only",
      "acceptance": "openspec/changes/mc02-preview-financial-operation/specs/mc02-preview-financial-operation/spec.md",
      "tasks": "openspec/changes/mc02-preview-financial-operation/tasks.md",
      "evidenceRoot": "evidence/moriarty-completion-program-2026-09-07/MC02",
      "commands": [
        "npm --prefix experiments/moriarty-midnight-financial ci",
        "npm --prefix experiments/moriarty-midnight-financial run build",
        "npm --prefix experiments/moriarty-midnight-financial test",
        "npm --prefix experiments/moriarty-midnight-financial run preview -- --case loan --max-attempts 2",
        "npm --prefix experiments/moriarty-midnight-financial run preview -- --case swap --max-attempts 2"
      ]
    },
    {
      "id": "MC03",
      "change": "mc03-native-recursive-proof",
      "dependencies": [
        "MC01"
      ],
      "status": "specified-only",
      "acceptance": "openspec/changes/mc03-native-recursive-proof/specs/mc03-native-recursive-proof/spec.md",
      "tasks": "openspec/changes/mc03-native-recursive-proof/tasks.md",
      "evidenceRoot": "evidence/moriarty-completion-program-2026-09-07/MC03",
      "commands": [
        "python3 experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json --preflight-only",
        "python3 experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json --execute",
        "python3 experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json --verify-retained"
      ]
    },
    {
      "id": "MC04",
      "change": "mc04-ledger-correspondence-and-consumption",
      "dependencies": [
        "MC01",
        "MC02",
        "MC03"
      ],
      "status": "specified-only",
      "acceptance": "openspec/changes/mc04-ledger-correspondence-and-consumption/specs/mc04-ledger-correspondence-and-consumption/spec.md",
      "tasks": "openspec/changes/mc04-ledger-correspondence-and-consumption/tasks.md",
      "evidenceRoot": "evidence/moriarty-completion-program-2026-09-07/MC04",
      "commands": [
        "npm --prefix experiments/moriarty-ledger-adapter ci",
        "npm --prefix experiments/moriarty-ledger-adapter run build",
        "npm --prefix experiments/moriarty-ledger-adapter test",
        "lake -d experiments/moriarty-ledger-adapter/formal build",
        "npm --prefix experiments/moriarty-ledger-adapter run verify-preview",
        "python3 experiments/moriarty-ledger-adapter/proof/run-reviewed.py --contract experiments/moriarty-ledger-adapter/proof/resource-contract.json --preflight-only",
        "python3 experiments/moriarty-ledger-adapter/proof/run-reviewed.py --contract experiments/moriarty-ledger-adapter/proof/resource-contract.json --execute",
        "python3 experiments/moriarty-ledger-adapter/proof/run-reviewed.py --contract experiments/moriarty-ledger-adapter/proof/resource-contract.json --verify-retained"
      ]
    },
    {
      "id": "MC05",
      "change": "mc05-mandatory-claim-acceptance",
      "dependencies": [
        "MC03",
        "MC04"
      ],
      "status": "specified-only",
      "acceptance": "openspec/changes/mc05-mandatory-claim-acceptance/specs/mc05-mandatory-claim-acceptance/spec.md",
      "tasks": "openspec/changes/mc05-mandatory-claim-acceptance/tasks.md",
      "evidenceRoot": "evidence/moriarty-completion-program-2026-09-07/MC05",
      "commands": [
        "npm --prefix experiments/moriarty-acceptance ci",
        "npm --prefix experiments/moriarty-acceptance run build",
        "npm --prefix experiments/moriarty-acceptance test",
        "lake -d experiments/moriarty-acceptance/formal build",
        "npm --prefix experiments/moriarty-acceptance run verify-preview",
        "python3 experiments/moriarty-acceptance/proof/run-reviewed.py --contract experiments/moriarty-acceptance/proof/resource-contract.json --preflight-only",
        "python3 experiments/moriarty-acceptance/proof/run-reviewed.py --contract experiments/moriarty-acceptance/proof/resource-contract.json --execute",
        "python3 experiments/moriarty-acceptance/proof/run-reviewed.py --contract experiments/moriarty-acceptance/proof/resource-contract.json --verify-retained"
      ]
    },
    {
      "id": "MC06",
      "change": "mc06-private-handoff-and-composition",
      "dependencies": [
        "MC05"
      ],
      "status": "specified-only",
      "acceptance": "openspec/changes/mc06-private-handoff-and-composition/specs/mc06-private-handoff-and-composition/spec.md",
      "tasks": "openspec/changes/mc06-private-handoff-and-composition/tasks.md",
      "evidenceRoot": "evidence/moriarty-completion-program-2026-09-07/MC06",
      "commands": [
        "npm --prefix experiments/moriarty-composition ci",
        "npm --prefix experiments/moriarty-composition run build",
        "npm --prefix experiments/moriarty-composition test",
        "lake -d experiments/moriarty-composition/formal build",
        "npm --prefix experiments/moriarty-composition run verify-isolated",
        "npm --prefix experiments/moriarty-composition run verify-preview",
        "python3 experiments/moriarty-composition/proof/run-reviewed.py --contract experiments/moriarty-composition/proof/resource-contract.json --preflight-only",
        "python3 experiments/moriarty-composition/proof/run-reviewed.py --contract experiments/moriarty-composition/proof/resource-contract.json --execute",
        "python3 experiments/moriarty-composition/proof/run-reviewed.py --contract experiments/moriarty-composition/proof/resource-contract.json --verify-retained"
      ]
    },
    {
      "id": "MC07",
      "change": "mc07-complete-financial-conformance",
      "dependencies": [
        "MC01",
        "MC04",
        "MC05",
        "MC06"
      ],
      "status": "specified-only",
      "acceptance": "openspec/changes/mc07-complete-financial-conformance/specs/mc07-complete-financial-conformance/spec.md",
      "tasks": "openspec/changes/mc07-complete-financial-conformance/tasks.md",
      "evidenceRoot": "evidence/moriarty-completion-program-2026-09-07/MC07",
      "commands": [
        "npm --prefix experiments/moriarty-conformance ci",
        "npm --prefix experiments/moriarty-conformance run build",
        "npm --prefix experiments/moriarty-conformance test",
        "npm --prefix experiments/moriarty-conformance run actus -- --all --all-fields",
        "npm --prefix experiments/moriarty-conformance run defi -- --all-rows",
        "npm --prefix experiments/moriarty-conformance run verify-coverage",
        "python3 experiments/moriarty-conformance/proof/run-reviewed.py --contract experiments/moriarty-conformance/proof/resource-contract.json --preflight-only",
        "python3 experiments/moriarty-conformance/proof/run-reviewed.py --contract experiments/moriarty-conformance/proof/resource-contract.json --execute",
        "python3 experiments/moriarty-conformance/proof/run-reviewed.py --contract experiments/moriarty-conformance/proof/resource-contract.json --verify-retained"
      ]
    },
    {
      "id": "MC08",
      "change": "mc08-release-evidence-and-developer-flow",
      "dependencies": [
        "MC01",
        "MC02",
        "MC03",
        "MC04",
        "MC05",
        "MC06",
        "MC07"
      ],
      "status": "specified-only",
      "acceptance": "openspec/changes/mc08-release-evidence-and-developer-flow/specs/mc08-release-evidence-and-developer-flow/spec.md",
      "tasks": "openspec/changes/mc08-release-evidence-and-developer-flow/tasks.md",
      "evidenceRoot": "evidence/moriarty-completion-program-2026-09-07/MC08",
      "commands": [
        "npm --prefix experiments/moriarty-release-check ci",
        "npm --prefix experiments/moriarty-release-check run build",
        "npm --prefix experiments/moriarty-release-check test",
        "npm --prefix experiments/moriarty-release-check run verify -- --program openspec/moriarty-completion-program.json"
      ]
    }
  ]
}


--- FILE raw/assignments/moriarty-completion-loop-2026-09-07.md ---
# Moriarty completion plans and execution authority

Source: user message in the active Codex conversation.
Recorded: 2026-09-07 UTC (2026-09-06 America/Denver).
Authority: normative user instruction. Lifecycle: S2 planning and execution authorization.
This record preserves the request; it does not establish implementation or audit completion.

## Exact request

> draft an series of openspec plans to resolve all of these and then arm a loop to execute, use fable and gpt 6 to audit the results   - [ ] Next: run a bounded financial operation on Preview and compare its finalized effects with the local evaluator.
>   - [ ] Finalize the DSL’s authoring syntax, typing rules, semantics, and compiler mapping.
>   - [ ] Produce a real native recursive proof. Blocked: the current R3 experiment exhausted rows at k17; review the encoding/resource
>     decision before restarting.
>
>   - [ ] Enforce contract properties, intent refinement, transition validity, and predecessor-history compliance in the actual acceptance
>     path.
>
>   - [ ] Demonstrate private witness handoff and split/join composition.
>   - [ ] Complete ACTUS/DeFi conformance coverage, including the held-out financial cases.
>   - [ ] Establish compiler-to-ledger correspondence and durable authorization/replay protection.

## Scope reconciliation

This request authorizes plans followed by execution within their recorded limits.
It retains the ACTUS/DeFi target-first reset and Preview steering.
It does not resume historical A4/A5 work or authorize Foreman development.
Fable and GPT-6 are the required auditors for this new program.
Older Council evidence and unresolved release requirements remain preserved.


--- FILE openspec/changes/mc01-bounded-language/README.md ---
# MC01: Bounded language and semantic contract

Status: specified-only.

- [Proposal](proposal.md)
- [Design](design.md)
- [Requirements](specs/mc01-bounded-language/spec.md)
- [Tasks](tasks.md)
- [Program charter](../../MORIARTY-COMPLETION-PROGRAM.md)


--- FILE openspec/changes/mc01-bounded-language/design.md ---
# Design: Bounded language and semantic contract

## Inputs and dependencies

Dependencies: Existing approved target-first design and retained evidence.

- `docs/superpowers/specs/2026-09-06-moriarty-unified-semantics-design.md`
- `docs/research/2026-09-06-intents-report-integration.md`
- `experiments/moriarty-developer-mock/src/language/core.ts`
- `experiments/moriarty-developer-mock/src/language/outcome.ts`

## Interfaces

parse(source) -> Result<AgreementAST, Diagnostic[]>; check(ast, profile) -> Result<TypedAgreement, Diagnostic[]>; elaborate(typed) -> CoreProgram; evaluate(program, state, action, authority, observations) -> Rejected | Complete | Pending. All wire encodings carry schema and semantic versions.

## Outputs and ownership

- `experiments/moriarty-language/spec/grammar.ebnf`
- `experiments/moriarty-language/spec/numeric-profile.json`
- `experiments/moriarty-language/spec/semantics.md`
- `experiments/moriarty-language/spec/bounds.json`
- `experiments/moriarty-language/src/ast.ts`
- `experiments/moriarty-language/src/parser.ts`
- `experiments/moriarty-language/src/typecheck.ts`
- `experiments/moriarty-language/src/elaborate.ts`
- `experiments/moriarty-language/src/evaluate.ts`
- `experiments/moriarty-language/src/codec.ts`
- `experiments/moriarty-language/src/lower-compact.ts`
- `experiments/moriarty-language/src/errors.ts`
- `experiments/moriarty-language/examples/loan.moriarty`
- `experiments/moriarty-language/examples/swap.moriarty`
- `experiments/moriarty-language/tests/frontend.test.mjs`
- `experiments/moriarty-language/tests/semantics.test.mjs`
- `experiments/moriarty-language/package.json`

## Trust boundaries

Treat source text, solvers, indexers, remote provers, generated code, and supplied receipts as untrusted inputs.
Keep oracle truth, ledger uniqueness, semantic validity, and confidentiality claims separate.
Bind all outputs to the semantic profile and the exact implementation candidate.

## Decisions and failures

Use the shared bounded Core rather than financial family names as primitive proof claims.
Preserve complete effects and positive feasibility when refining an adapter.
Record a concrete changed hypothesis before any correction.
Stop a dependent package when its prerequisite fails.
Do not reset exhausted contracts or replace a missing proof with a mock.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc01-bounded-language/proposal.md ---
# Change: Bounded language and semantic contract

## Why

Define and implement the Moriarty authoring language over a finite typed Core.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Freeze the language profile.
- Implement the frontend.
- Implement semantic elaboration.
- Implement the initial Compact mapping.

## Impact

- `experiments/moriarty-language/spec/grammar.ebnf`
- `experiments/moriarty-language/spec/numeric-profile.json`
- `experiments/moriarty-language/spec/semantics.md`
- `experiments/moriarty-language/spec/bounds.json`
- `experiments/moriarty-language/src/ast.ts`
- `experiments/moriarty-language/src/parser.ts`
- `experiments/moriarty-language/src/typecheck.ts`
- `experiments/moriarty-language/src/elaborate.ts`
- `experiments/moriarty-language/src/evaluate.ts`
- `experiments/moriarty-language/src/codec.ts`
- `experiments/moriarty-language/src/lower-compact.ts`
- `experiments/moriarty-language/src/errors.ts`
- `experiments/moriarty-language/examples/loan.moriarty`
- `experiments/moriarty-language/examples/swap.moriarty`
- `experiments/moriarty-language/tests/frontend.test.mjs`
- `experiments/moriarty-language/tests/semantics.test.mjs`
- `experiments/moriarty-language/package.json`
- `evidence/moriarty-completion-program-2026-09-07/MC01/`

## Non-goals

No A4/A5 continuation, mainnet funds, new consensus, or Foreman repair belongs to this package.
No downstream requirement becomes complete from this package's narrower result.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc01-bounded-language/specs/mc01-bounded-language/spec.md ---
## ADDED Requirements

### Requirement: Defined authoring and canonical representation
The frontend SHALL define grammar, units, asset domains, typing, diagnostics, canonical bytes, and versioned elaboration.

#### Scenario: Valid loan and swap programs
- **WHEN** the frontend receives valid loan and swap source programs
- **THEN** both parse, typecheck, elaborate, roundtrip canonically, and match independent expected evaluator traces.

#### Scenario: Invalid authoring
- **WHEN** source contains recursion, ambiguous assets, unbounded collections, malformed encodings, or unsupported operations
- **THEN** the frontend rejects it before signing and reports the source location.

### Requirement: Finite semantic work
Every execution SHALL have explicit arithmetic, allocation, schedule, horizon, nesting, fold, transaction, proof-size, and predecessor bounds.

#### Scenario: Finite closure
- **WHEN** an execution follows ordinary work, pending progress, closure, or bounded epoch migration
- **THEN** each transition decreases the declared lifecycle measure and respects every declared local bound.

#### Scenario: Bound evasion
- **WHEN** a transition overflows, exhausts work, leaves intermediate arithmetic unchecked, or resets a continuation budget
- **THEN** the evaluator and proof relation reject that transition.

### Requirement: Target-driven semantic freeze
Every Core operation SHALL trace to an ACTUS behavior, DeFi behavior, or explicit developer requirement.

#### Scenario: Supported profile
- **WHEN** loan and swap source programs elaborate into Core
- **THEN** dues and settlement remain distinct, and gross debit and net delivery checks match independent expected effects.

#### Scenario: Taxonomy shortcut
- **WHEN** a proposed Core operation cites only a financial family label or convenient numeric tolerance
- **THEN** the semantic-freeze gate rejects that unsupported operation.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Fable or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.


--- FILE openspec/changes/mc01-bounded-language/tasks.md ---
# Tasks: Bounded language and semantic contract

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Define and implement the Moriarty authoring language over a finite typed Core.

**Dependencies:** Program charter.

**Implementation root:** `experiments/moriarty-language/`.

**Interfaces:** parse(source) -> Result<AgreementAST, Diagnostic[]>; check(ast, profile) -> Result<TypedAgreement, Diagnostic[]>; elaborate(typed) -> CoreProgram; evaluate(program, state, action, authority, observations) -> Rejected | Complete | Pending. All wire encodings carry schema and semantic versions.

## 1. Freeze the language profile

- [ ] 1.1 Write the grammar, numeric profile, transition judgments, and bounds. Classify DS-01 through DS-07 dependencies. Resolve foundational ambiguities before freezing; retain target-specific gaps for MC07.
- [ ] 1.2 Review traceability against the complete target matrices. Require both named auditors before freezing this profile.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement the frontend

- [ ] 2.1 Add positive roundtrip fixtures and negative syntax/type/bounds tests before parser and checker changes. Implement canonical AST and diagnostics.
- [ ] 2.2 Run the frontend suite, including source-location and canonical-encoding mutations.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Implement semantic elaboration

- [ ] 3.1 Add independent loan/swap expected traces before implementing elaboration and evaluation. Preserve exact-plan and outcome modes.
- [ ] 3.2 Compare every state field, obligation, authority delta, and effect against the existing evaluator.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Implement the initial Compact mapping

- [ ] 4.1 Specify representable Core operations and rejection diagnostics. Add local lowering tests and source maps.
- [ ] 4.2 Compile generated supported examples. Record unsupported mappings as failures; do not claim general correspondence.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-language ci
npm --prefix experiments/moriarty-language run build
npm --prefix experiments/moriarty-language test
npm --prefix experiments/moriarty-developer-mock run build
npm --prefix experiments/moriarty-developer-mock test
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC01/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Fable and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc02-preview-financial-operation/README.md ---
# MC02: Preview financial operation and differential settlement

Status: specified-only.

- [Proposal](proposal.md)
- [Design](design.md)
- [Requirements](specs/mc02-preview-financial-operation/spec.md)
- [Tasks](tasks.md)
- [Program charter](../../MORIARTY-COMPLETION-PROGRAM.md)


--- FILE openspec/changes/mc02-preview-financial-operation/design.md ---
# Design: Preview financial operation and differential settlement

## Inputs and dependencies

Dependencies: MC01.

- `experiments/moriarty-developer-mock/src/language/core.ts`
- `experiments/moriarty-midnight-network/preview-call.mjs`
- `experiments/moriarty-midnight-network/preview-verify.mjs`
- `evidence/midnight-preview-2026-09-07/README.md`

## Interfaces

FinancialRunInput binds program, initial state, bounded action, authority, observations, and explicit test-asset mapping. DifferentialReceipt binds local expected trace, complete observed ledger effects, transaction identifier, block hash, finality, and readback.

## Outputs and ownership

- `experiments/moriarty-midnight-financial/fixtures/loan-preview.json`
- `experiments/moriarty-midnight-financial/fixtures/swap-preview.json`
- `experiments/moriarty-midnight-financial/asset-mapping.json`
- `experiments/moriarty-midnight-financial/contracts/financial.compact`
- `experiments/moriarty-midnight-financial/src/run-preview.ts`
- `experiments/moriarty-midnight-financial/src/compare-effects.ts`
- `experiments/moriarty-midnight-financial/tests/differential.test.mjs`
- `experiments/moriarty-midnight-financial/package.json`

## Trust boundaries

Treat source text, solvers, indexers, remote provers, generated code, and supplied receipts as untrusted inputs.
Keep oracle truth, ledger uniqueness, semantic validity, and confidentiality claims separate.
Bind all outputs to the semantic profile and the exact implementation candidate.

## Decisions and failures

Use the shared bounded Core rather than financial family names as primitive proof claims.
Preserve complete effects and positive feasibility when refining an adapter.
Record a concrete changed hypothesis before any correction.
Stop a dependent package when its prerequisite fails.
Do not reset exhausted contracts or replace a missing proof with a mock.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc02-preview-financial-operation/proposal.md ---
# Change: Preview financial operation and differential settlement

## Why

Settle bounded loan and swap examples on Preview and compare all effects with independent local expectations.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Freeze feasible test fixtures.
- Implement differential local tests.
- Run the bounded Preview campaign.
- Reconcile integration evidence.

## Impact

- `experiments/moriarty-midnight-financial/fixtures/loan-preview.json`
- `experiments/moriarty-midnight-financial/fixtures/swap-preview.json`
- `experiments/moriarty-midnight-financial/asset-mapping.json`
- `experiments/moriarty-midnight-financial/contracts/financial.compact`
- `experiments/moriarty-midnight-financial/src/run-preview.ts`
- `experiments/moriarty-midnight-financial/src/compare-effects.ts`
- `experiments/moriarty-midnight-financial/tests/differential.test.mjs`
- `experiments/moriarty-midnight-financial/package.json`
- `evidence/moriarty-completion-program-2026-09-07/MC02/`

## Non-goals

No A4/A5 continuation, mainnet funds, new consensus, or Foreman repair belongs to this package.
No downstream requirement becomes complete from this package's narrower result.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc02-preview-financial-operation/specs/mc02-preview-financial-operation/spec.md ---
## ADDED Requirements

### Requirement: Explicit financial meaning
The experiment SHALL distinguish contractual denomination, test-token identity, dues, and actual transferred value.

#### Scenario: Feasible funded episode
- **WHEN** the dedicated wallet executes the frozen small loan fixture with its explicit asset mapping
- **THEN** real supported ledger assets move, contractual dues update correctly, and independent expected values match.

#### Scenario: Fictitious settlement
- **WHEN** a result supplies only a message update, synthetic balance field, or undisclosed currency conversion
- **THEN** the financial-settlement gate rejects it as transfer evidence.

### Requirement: Independent complete comparison
The checker SHALL compare complete local financial effects with finalized Preview effects and contract state.

#### Scenario: Two target families
- **WHEN** the generated loan and pool-swap programs settle on Preview
- **THEN** complete effects and state match local expectations, with indexed success, canonical finality, and exact readback.

#### Scenario: Partial comparison
- **WHEN** observed effects contain a wrong recipient, wrong domain, excess fee, missing debit, extra approval, or undeclared write
- **THEN** the complete-effect comparator rejects the result.

### Requirement: Integration scope
This package SHALL label its transactions uncertified until MC05 enforces all mandatory claims.

#### Scenario: Honest integration result
- **WHEN** MC02 produces a valid financial integration receipt before MC05 acceptance
- **THEN** the receipt names the financial predicate and marks mandatory proof claims unavailable.

#### Scenario: Promotion shortcut
- **WHEN** network success is offered as evidence of history compliance or compiler correspondence
- **THEN** the package gate rejects the unsupported proof claim.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Fable or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.


--- FILE openspec/changes/mc02-preview-financial-operation/tasks.md ---
# Tasks: Preview financial operation and differential settlement

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Settle bounded loan and swap examples on Preview and compare all effects with independent local expectations.

**Dependencies:** MC01.

**Implementation root:** `experiments/moriarty-midnight-financial/`.

**Interfaces:** FinancialRunInput binds program, initial state, bounded action, authority, observations, and explicit test-asset mapping. DifferentialReceipt binds local expected trace, complete observed ledger effects, transaction identifier, block hash, finality, and readback.

## 1. Freeze feasible test fixtures

- [ ] 1.1 Derive small loan and swap cases from existing packages. Pin denomination and token mappings. Reserve closure costs.
- [ ] 1.2 Check independent expected values and the available dedicated test balance before public work.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement differential local tests

- [ ] 2.1 Write local expected-effect and adverse-effect tests first. Implement generated Compact financial entry points and complete effect decoding.
- [ ] 2.2 Run positive and negative comparisons against local Docker before public submission.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Run the bounded Preview campaign

- [ ] 3.1 Use the existing Preview wallet. Preserve keys externally. Retain before-state, transaction bytes, all effects, and failed attempts.
- [ ] 3.2 Submit at most two public attempts per case. Require indexed SUCCESS, canonical node finality, and exact readback.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Reconcile integration evidence

- [ ] 4.1 Record actual commands, pins, source hashes, resource use, and the funded asset domain.
- [ ] 4.2 Require independent expected results and both audit verdicts before closing this integration package.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-midnight-financial ci
npm --prefix experiments/moriarty-midnight-financial run build
npm --prefix experiments/moriarty-midnight-financial test
npm --prefix experiments/moriarty-midnight-financial run preview -- --case loan --max-attempts 2
npm --prefix experiments/moriarty-midnight-financial run preview -- --case swap --max-attempts 2
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC02/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Fable and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc03-native-recursive-proof/README.md ---
# MC03: Reviewed native encoding and recursive proof

Status: specified-only.

- [Proposal](proposal.md)
- [Design](design.md)
- [Requirements](specs/mc03-native-recursive-proof/spec.md)
- [Tasks](tasks.md)
- [Program charter](../../MORIARTY-COMPLETION-PROGRAM.md)


--- FILE openspec/changes/mc03-native-recursive-proof/design.md ---
# Design: Reviewed native encoding and recursive proof

## Inputs and dependencies

Dependencies: MC01.

- `experiments/moriarty-native-ivc-r3/README.md`
- `experiments/moriarty-native-ivc-r3/harness/moriarty_loan_r3.rs`
- `experiments/moriarty-native-ivc-r3/run-native.py`
- `evidence/moriarty-native-ivc-r3-2026-09-07/README.md`

## Interfaces

NativeEpisode binds the original R2 financial episode, all public contexts, fixed authority scope, and the remaining lifecycle bound. ProofArtifact binds proof bytes, VK, SRS, backend pin, statement encoding, and final accumulator verification.

## Outputs and ownership

- `experiments/moriarty-native-ivc-r3/revision-01/encoding-decision.md`
- `experiments/moriarty-native-ivc-r3/revision-01/relation-spec.md`
- `experiments/moriarty-native-ivc-r3/revision-01/encoding-fixtures.json`
- `experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json`
- `experiments/moriarty-native-ivc-r3/revision-01/harness.rs`
- `experiments/moriarty-native-ivc-r3/revision-01/verify-retained.rs`
- `experiments/moriarty-native-ivc-r3/revision-01/tests/encoding-controls.rs`
- `experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py`

## Trust boundaries

Treat source text, solvers, indexers, remote provers, generated code, and supplied receipts as untrusted inputs.
Keep oracle truth, ledger uniqueness, semantic validity, and confidentiality claims separate.
Bind all outputs to the semantic profile and the exact implementation candidate.

## Decisions and failures

Use the shared bounded Core rather than financial family names as primitive proof claims.
Preserve complete effects and positive feasibility when refining an adapter.
Record a concrete changed hypothesis before any correction.
Stop a dependent package when its prerequisite fails.
Do not reset exhausted contracts or replace a missing proof with a mock.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc03-native-recursive-proof/proposal.md ---
# Change: Reviewed native encoding and recursive proof

## Why

Produce and independently verify a real two-step financial recursive proof under a reviewed resource contract.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Review the encoding and resource decision.
- Implement encoding and arithmetic controls.
- Implement retained proof verification.
- Run one reviewed proof campaign.

## Impact

- `experiments/moriarty-native-ivc-r3/revision-01/encoding-decision.md`
- `experiments/moriarty-native-ivc-r3/revision-01/relation-spec.md`
- `experiments/moriarty-native-ivc-r3/revision-01/encoding-fixtures.json`
- `experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json`
- `experiments/moriarty-native-ivc-r3/revision-01/harness.rs`
- `experiments/moriarty-native-ivc-r3/revision-01/verify-retained.rs`
- `experiments/moriarty-native-ivc-r3/revision-01/tests/encoding-controls.rs`
- `experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py`
- `evidence/moriarty-completion-program-2026-09-07/MC03/`

## Non-goals

No A4/A5 continuation, mainnet funds, new consensus, or Foreman repair belongs to this package.
No downstream requirement becomes complete from this package's narrower result.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc03-native-recursive-proof/specs/mc03-native-recursive-proof/spec.md ---
## ADDED Requirements

### Requirement: Reviewed restart
Native execution SHALL require a changed encoding hypothesis and both named audits before consuming a new bounded resource contract.

#### Scenario: Checked smaller encoding
- **WHEN** both auditors review the implemented smaller encoding and its resource contract
- **THEN** they require checked ranges and commitment preimages preserving every original financial and authority binding before launch.

#### Scenario: Unjustified retry
- **WHEN** dispatch reuses the failed 54-limb setup, unchecked state hashes, or an unreviewed larger k
- **THEN** the execution gate refuses dispatch and preserves the original failed campaign.

### Requirement: Real recursion and independent verification
The harness SHALL prove both financial steps and verify retained bytes in a separate process.

#### Scenario: Valid episode
- **WHEN** the native harness proves the original accrual and due-settlement transitions
- **THEN** a separate process verifies retained proofs, constrained genesis, expected fields, and the discharged final accumulator.

#### Scenario: Invalid proof context
- **WHEN** retained input is missing, truncated, altered, wrong-key, wrong-state, wrong-domain, wrong-intent, forged-genesis, or invalid-predecessor
- **THEN** the separate verifier rejects each mutant for its declared reason.

### Requirement: Hard resource stop
The runner SHALL enforce cumulative time, process-group memory, CPU, output, SRS, k, and attempt limits.

#### Scenario: Recorded bounded run
- **WHEN** a reviewed native campaign terminates
- **THEN** its receipt records actual peak usage, command, input digests, terminal status, and the exact proved predicate, if any.

#### Scenario: Resource failure
- **WHEN** synthesis, proving, a control, timeout, memory, or output enforcement fails
- **THEN** the native contract stops immediately without automatic escalation or a fresh attempt-counter reset.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Fable or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.


--- FILE openspec/changes/mc03-native-recursive-proof/tasks.md ---
# Tasks: Reviewed native encoding and recursive proof

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Produce and independently verify a real two-step financial recursive proof under a reviewed resource contract.

**Dependencies:** MC01.

**Implementation root:** `experiments/moriarty-native-ivc-r3/revision-01/`.

**Interfaces:** NativeEpisode binds the original R2 financial episode, all public contexts, fixed authority scope, and the remaining lifecycle bound. ProofArtifact binds proof bytes, VK, SRS, backend pin, statement encoding, and final accumulator verification.

## 1. Review the encoding and resource decision

- [ ] 1.1 Compare direct limbs with checked commitment encoding. Inspect constraint costs without proving. Select a justified representation.
- [ ] 1.2 Obtain Fable and independent GPT-6 approval of relation equivalence, exact command, k, SRS, and limits.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement encoding and arithmetic controls

- [ ] 2.1 Write independent preimage, limb-boundary, arithmetic, context, and malformed-genesis tests before changing the relation.
- [ ] 2.2 Compare every original episode field with R2. Require native and in-circuit relations to agree.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Implement retained proof verification

- [ ] 3.1 Add a separate verifier process and proof/context mutations before launching the positive proof campaign.
- [ ] 3.2 Require fresh deserialization, expected VK/SRS identity, transcript exhaustion, and discharged accumulator obligations.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Run one reviewed proof campaign

- [ ] 4.1 Adapt the existing cgroup wrapper to the new immutable contract. Preserve old counters and failures.
- [ ] 4.2 Run exactly two positive steps and retained negative controls. Stop immediately on the first failed predicate.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
python3 experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json --preflight-only
python3 experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json --execute
python3 experiments/moriarty-native-ivc-r3/revision-01/run-reviewed.py --contract experiments/moriarty-native-ivc-r3/revision-01/resource-contract.json --verify-retained
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC03/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Fable and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc04-ledger-correspondence-and-consumption/README.md ---
# MC04: Compiler correspondence and durable ledger consumption

Status: specified-only.

- [Proposal](proposal.md)
- [Design](design.md)
- [Requirements](specs/mc04-ledger-correspondence-and-consumption/spec.md)
- [Tasks](tasks.md)
- [Program charter](../../MORIARTY-COMPLETION-PROGRAM.md)


--- FILE openspec/changes/mc04-ledger-correspondence-and-consumption/design.md ---
# Design: Compiler correspondence and durable ledger consumption

## Inputs and dependencies

Dependencies: MC01, MC02, MC03.

- `docs/research/2026-09-06-midnight-native-recursion.md`
- `experiments/moriarty-language/src/lower-compact.ts`
- `experiments/moriarty-native-ivc-r3/revision-01/relation-spec.md`
- `experiments/moriarty-midnight-financial/src/compare-effects.ts`

## Interfaces

LedgerAdapter consumes a versioned compiled program, typed proof artifact, full authority envelope, and predecessor identifiers. It returns Rejected, Unavailable, or FinalizedReceipt. Consumption identity is domain/principal/nonce plus ledger-consumed predecessor/output identities.

## Outputs and ownership

- `experiments/moriarty-ledger-adapter/verifier-compatibility.json`
- `experiments/moriarty-ledger-adapter/correspondence-spec.md`
- `experiments/moriarty-ledger-adapter/formal/Correspondence.lean`
- `experiments/moriarty-ledger-adapter/contracts/acceptance.compact`
- `experiments/moriarty-ledger-adapter/src/adapter.ts`
- `experiments/moriarty-ledger-adapter/src/consumption.ts`
- `experiments/moriarty-ledger-adapter/src/effect-projection.ts`
- `experiments/moriarty-ledger-adapter/tests/adapter.test.mjs`
- `experiments/moriarty-ledger-adapter/tests/recovery.test.mjs`
- `experiments/moriarty-ledger-adapter/package.json`
- `experiments/moriarty-ledger-adapter/formal/lakefile.toml`

## Native relation extension: adapter-profile-01

Implement compiled loan and swap transition relations, complete effects, and durable authorization bindings.
Establish the retained MC03 verifier interface before extending the relation. Requalify correspondence using the new loan/swap proofs.
Freeze at most 6 positive transitions and meaningful invalid proof/context controls.
The charter allocates one campaign with 20 cumulative proving/verification minutes.
Require both audits before launch and independent retained-byte verification after proving.
The original fixed-loan MC03 proof cannot discharge this extended predicate.

Owned outputs:

- `experiments/moriarty-ledger-adapter/proof/relation-spec.md`
- `experiments/moriarty-ledger-adapter/proof/harness.rs`
- `experiments/moriarty-ledger-adapter/proof/verify-retained.rs`
- `experiments/moriarty-ledger-adapter/proof/encoding-controls.rs`
- `experiments/moriarty-ledger-adapter/proof/campaign-cases.json`
- `experiments/moriarty-ledger-adapter/proof/resource-contract.json`
- `experiments/moriarty-ledger-adapter/proof/run-reviewed.py`

## Trust boundaries

Treat source text, solvers, indexers, remote provers, generated code, and supplied receipts as untrusted inputs.
Keep oracle truth, ledger uniqueness, semantic validity, and confidentiality claims separate.
Bind all outputs to the semantic profile and the exact implementation candidate.

## Decisions and failures

Use the shared bounded Core rather than financial family names as primitive proof claims.
Preserve complete effects and positive feasibility when refining an adapter.
Record a concrete changed hypothesis before any correction.
Stop a dependent package when its prerequisite fails.
Do not reset exhausted contracts or replace a missing proof with a mock.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc04-ledger-correspondence-and-consumption/proposal.md ---
# Change: Compiler correspondence and durable ledger consumption

## Why

Connect compiled semantics and native proof verification to durable Midnight effects and consumption.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Establish the verifier adapter boundary.
- Implement a complete effect projection.
- Mechanize correspondence.
- Implement durable consumption and recovery.

- Implement the separately allocated native relation and retained-proof campaign.

## Impact

- `experiments/moriarty-ledger-adapter/verifier-compatibility.json`
- `experiments/moriarty-ledger-adapter/correspondence-spec.md`
- `experiments/moriarty-ledger-adapter/formal/Correspondence.lean`
- `experiments/moriarty-ledger-adapter/contracts/acceptance.compact`
- `experiments/moriarty-ledger-adapter/src/adapter.ts`
- `experiments/moriarty-ledger-adapter/src/consumption.ts`
- `experiments/moriarty-ledger-adapter/src/effect-projection.ts`
- `experiments/moriarty-ledger-adapter/tests/adapter.test.mjs`
- `experiments/moriarty-ledger-adapter/tests/recovery.test.mjs`
- `experiments/moriarty-ledger-adapter/package.json`
- `experiments/moriarty-ledger-adapter/formal/lakefile.toml`
- `evidence/moriarty-completion-program-2026-09-07/MC04/`

## Non-goals

No A4/A5 continuation, mainnet funds, new consensus, or Foreman repair belongs to this package.
No downstream requirement becomes complete from this package's narrower result.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc04-ledger-correspondence-and-consumption/specs/mc04-ledger-correspondence-and-consumption/spec.md ---
## ADDED Requirements

### Requirement: Actual verifier compatibility
The adapter SHALL demonstrate that the selected Midnight acceptance path verifies the exact native proof, VK, and public-input relation.

#### Scenario: Compatible verifier
- **WHEN** a real native positive proof and its mutants reach the selected target acceptance path
- **THEN** the exact native proof, VK, and public-input relation verify there, and each invalid mutant rejects there.

#### Scenario: Missing interface
- **WHEN** the target lacks an interface that checks the required native relation
- **THEN** the adapter remains interface-blocked; host assertions and mocked verification cannot substitute.

### Requirement: Named compiler correspondence
The package SHALL state and mechanically check source-to-Core-to-proof-to-ledger correspondence for its supported finite domain.

#### Scenario: Positive realization
- **WHEN** loan and swap programs compile within the supported finite domain
- **THEN** mechanically checked correspondence and positive realized traces cover the complete effect projection.

#### Scenario: Unsound projection
- **WHEN** an execution adds transfers, approvals, hidden calls, changed rounding, reordered events, or omitted writes
- **THEN** the correspondence checker rejects it and the package cannot close.

### Requirement: Durable one-time consumption
The acceptance path SHALL enforce authorization and predecessor consumption across restart, concurrency, and competing branches.

#### Scenario: Crash recovery
- **WHEN** a client restarts after finalized acceptance or crashes at each consumption boundary
- **THEN** durable ledger identities prevent repeated effects and recovery reconciles finality without issuing duplicate authority.

#### Scenario: Duplicate authority
- **WHEN** proposals share a consumed nonce or predecessor, including changed-intent-hash and concurrent valid-branch cases
- **THEN** at most one conflicting proposal settles; restart does not restore consumed authority.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Fable or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.

### Requirement: Extended native predicate evidence
The package SHALL implement, review, prove, and independently verify its allocated native relation before proof-dependent acceptance.

#### Scenario: Reviewed extension executes
- **WHEN** both auditors approve the implemented relation and contract, and the allocated campaign passes
- **THEN** retained proofs and separate-process verification establish only the extended predicate and requalified dependent correspondence.

#### Scenario: Fixed proof substituted for extension
- **WHEN** the package supplies only an earlier fixed-loan proof, exceeds its campaign allocation, or lacks a required extension proof
- **THEN** package acceptance remains blocked and no earlier proof or counter can substitute.


--- FILE openspec/changes/mc04-ledger-correspondence-and-consumption/tasks.md ---
# Tasks: Compiler correspondence and durable ledger consumption

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Connect compiled semantics and native proof verification to durable Midnight effects and consumption.

**Dependencies:** MC01, MC02, MC03.

**Implementation root:** `experiments/moriarty-ledger-adapter/`.

**Interfaces:** LedgerAdapter consumes a versioned compiled program, typed proof artifact, full authority envelope, and predecessor identifiers. It returns Rejected, Unavailable, or FinalizedReceipt. Consumption identity is domain/principal/nonce plus ledger-consumed predecessor/output identities.

## 1. Establish the verifier adapter boundary

- [ ] 1.1 Pin application, aggregation, Compact, ZKIR, ledger, and key formats. Identify the supported verification entry point.
- [ ] 1.2 Run one positive compatibility probe and mutated proof/public-input controls before broad adapter work.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement a complete effect projection

- [ ] 2.1 Write effect omission and target mutation tests first. Map every supported source effect to actual ledger behavior.
- [ ] 2.2 Compare independent traces and reject any unrepresented effect or unverifiable external call.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Mechanize correspondence

- [ ] 3.1 State assumptions and the finite supported domain. Implement the source/Core/target relations and proof.
- [ ] 3.2 Run Lean without sorry or undeclared axioms. Keep trace tests separate from the theorem claim.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Implement durable consumption and recovery

- [ ] 4.1 Write crash-point, duplicate nonce, competing branch, and concurrent proposal tests first. Implement atomic ledger-backed consumption.
- [ ] 4.2 Require at most one finalized success for conflicting proposals and consistent restart recovery.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## P. Implement and review the native extension

These tasks precede any proof-dependent acceptance or correspondence claim in this package.
Establish the retained MC03 verifier interface before extending the relation. Requalify correspondence using the new loan/swap proofs.

- [ ] P.1 Write `experiments/moriarty-ledger-adapter/proof/relation-spec.md` and `experiments/moriarty-ledger-adapter/proof/campaign-cases.json` for adapter-profile-01.
- [ ] P.2 Add independent expected-field and rejection controls in `experiments/moriarty-ledger-adapter/proof/encoding-controls.rs` before changing the relation.
- [ ] P.3 Implement `experiments/moriarty-ledger-adapter/proof/harness.rs` and the separate verifier `experiments/moriarty-ledger-adapter/proof/verify-retained.rs`.
- [ ] P.4 Implement `experiments/moriarty-ledger-adapter/proof/run-reviewed.py` and freeze `experiments/moriarty-ledger-adapter/proof/resource-contract.json` under the charter allocation.
- [ ] P.5 Obtain both implemented-relation and resource-contract audits before executing the frozen campaign.
- [ ] P.6 Retain actual proofs and verifier controls. Stop on first failure without automatic retry.
- [ ] P.7 Recheck affected earlier package predicates and obtain updated candidate-bound audits.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-ledger-adapter ci
npm --prefix experiments/moriarty-ledger-adapter run build
npm --prefix experiments/moriarty-ledger-adapter test
lake -d experiments/moriarty-ledger-adapter/formal build
npm --prefix experiments/moriarty-ledger-adapter run verify-preview
python3 experiments/moriarty-ledger-adapter/proof/run-reviewed.py --contract experiments/moriarty-ledger-adapter/proof/resource-contract.json --preflight-only
python3 experiments/moriarty-ledger-adapter/proof/run-reviewed.py --contract experiments/moriarty-ledger-adapter/proof/resource-contract.json --execute
python3 experiments/moriarty-ledger-adapter/proof/run-reviewed.py --contract experiments/moriarty-ledger-adapter/proof/resource-contract.json --verify-retained
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC04/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Fable and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc05-mandatory-claim-acceptance/README.md ---
# MC05: Mandatory contract, intent, transition, and history claims

Status: specified-only.

- [Proposal](proposal.md)
- [Design](design.md)
- [Requirements](specs/mc05-mandatory-claim-acceptance/spec.md)
- [Tasks](tasks.md)
- [Program charter](../../MORIARTY-COMPLETION-PROGRAM.md)


--- FILE openspec/changes/mc05-mandatory-claim-acceptance/design.md ---
# Design: Mandatory contract, intent, transition, and history claims

## Inputs and dependencies

Dependencies: MC03, MC04.

- `experiments/moriarty-developer-mock/src/language/claims.ts`
- `experiments/moriarty-developer-mock/src/language/policy.ts`
- `docs/research/2026-09-06-pcd-report-integration.md`
- `experiments/moriarty-ledger-adapter/correspondence-spec.md`

## Interfaces

ClaimSpec -> MandatoryManifest -> signed Intent -> BoundClaim -> ProofArtifact. VerifiedClaim records trusted verifier identity, domain, program/spec versions, dependencies, assumptions, and applicability. accept checks ContractInvariant, IntentRefinement, TransitionValidity, and HistoryCompliance.

## Outputs and ownership

- `experiments/moriarty-acceptance/claim-policy.json`
- `experiments/moriarty-acceptance/src/claim-codec.ts`
- `experiments/moriarty-acceptance/src/certificates.ts`
- `experiments/moriarty-acceptance/src/verify-intent.ts`
- `experiments/moriarty-acceptance/src/accept.ts`
- `experiments/moriarty-acceptance/formal/ContractProperties.lean`
- `experiments/moriarty-acceptance/formal/IntentRefinement.lean`
- `experiments/moriarty-acceptance/formal/lakefile.toml`
- `experiments/moriarty-acceptance/contracts/mandatory-claims.compact`
- `experiments/moriarty-acceptance/tests/acceptance.test.mjs`
- `experiments/moriarty-acceptance/package.json`

## Native relation extension: mandatory-claims-01

Implement contract invariants, dynamic signed authority, intent refinement, valid transitions, and authenticated history.
Require actual signature and mandatory-claim checks in the proof/acceptance boundary. Requalify the affected MC04 theorem and adapter.
Freeze at most 8 positive transitions and meaningful invalid proof/context controls.
The charter allocates one campaign with 20 cumulative proving/verification minutes.
Require both audits before launch and independent retained-byte verification after proving.
The original fixed-loan MC03 proof cannot discharge this extended predicate.

Owned outputs:

- `experiments/moriarty-acceptance/proof/relation-spec.md`
- `experiments/moriarty-acceptance/proof/harness.rs`
- `experiments/moriarty-acceptance/proof/verify-retained.rs`
- `experiments/moriarty-acceptance/proof/encoding-controls.rs`
- `experiments/moriarty-acceptance/proof/campaign-cases.json`
- `experiments/moriarty-acceptance/proof/resource-contract.json`
- `experiments/moriarty-acceptance/proof/run-reviewed.py`

## Trust boundaries

Treat source text, solvers, indexers, remote provers, generated code, and supplied receipts as untrusted inputs.
Keep oracle truth, ledger uniqueness, semantic validity, and confidentiality claims separate.
Bind all outputs to the semantic profile and the exact implementation candidate.

## Decisions and failures

Use the shared bounded Core rather than financial family names as primitive proof claims.
Preserve complete effects and positive feasibility when refining an adapter.
Record a concrete changed hypothesis before any correction.
Stop a dependent package when its prerequisite fails.
Do not reset exhausted contracts or replace a missing proof with a mock.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc05-mandatory-claim-acceptance/proposal.md ---
# Change: Mandatory contract, intent, transition, and history claims

## Why

Enforce all four required claims in the real transaction acceptance path.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Freeze canonical claim and policy bindings.
- Prove bounded contract properties and refinement.
- Wire required evidence into ledger acceptance.
- Replace unavailable demo claims honestly.

- Implement the separately allocated native relation and retained-proof campaign.

## Impact

- `experiments/moriarty-acceptance/claim-policy.json`
- `experiments/moriarty-acceptance/src/claim-codec.ts`
- `experiments/moriarty-acceptance/src/certificates.ts`
- `experiments/moriarty-acceptance/src/verify-intent.ts`
- `experiments/moriarty-acceptance/src/accept.ts`
- `experiments/moriarty-acceptance/formal/ContractProperties.lean`
- `experiments/moriarty-acceptance/formal/IntentRefinement.lean`
- `experiments/moriarty-acceptance/formal/lakefile.toml`
- `experiments/moriarty-acceptance/contracts/mandatory-claims.compact`
- `experiments/moriarty-acceptance/tests/acceptance.test.mjs`
- `experiments/moriarty-acceptance/package.json`
- `evidence/moriarty-completion-program-2026-09-07/MC05/`

## Non-goals

No A4/A5 continuation, mainnet funds, new consensus, or Foreman repair belongs to this package.
No downstream requirement becomes complete from this package's narrower result.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc05-mandatory-claim-acceptance/specs/mc05-mandatory-claim-acceptance/spec.md ---
## ADDED Requirements

### Requirement: Mandatory acceptance
The real acceptance path SHALL require valid evidence for all four claim types under a fixed trusted policy.

#### Scenario: Certified action
- **WHEN** a loan or swap action carries all mandatory evidence under the fixed trusted policy
- **THEN** actual acceptance checks contract, intent, transition, history, and ledger-consumption predicates before applying effects.

#### Scenario: Downgrade attack
- **WHEN** an action omits claims, strips mandatory roots, selects arbitrary verifiers, or supplies stale certificates, false guards, or unsatisfied dependencies
- **THEN** actual acceptance rejects it before applying effects.

### Requirement: Intent refinement and complete effects
The proof relation SHALL connect the concrete plan and complete effects to signed bounded authority and net goals.

#### Scenario: Authorized route choice
- **WHEN** different permitted plans refine the same signed outcome intent
- **THEN** each accepted route satisfies the original gross authority limits, net goals, and complete-effect relation.

#### Scenario: Ledger-valid intent-invalid action
- **WHEN** a ledger-valid action has wrong recipients, refunds hiding excess gross debit, fees violating net goals, or undeclared approvals
- **THEN** the mandatory refinement relation rejects it in actual acceptance.

### Requirement: No circular or simulated evidence
The certificate and proof construction SHALL use the non-circular commitment order and actual checked evidence.

#### Scenario: Bound claims
- **WHEN** the system builds and consumes the non-circular signed claim envelope
- **THEN** program, policy, predecessors, observations, resulting state, and effects bind identically across signature, proof, and ledger.

#### Scenario: Fake compliance
- **WHEN** an action supplies hash-linked receipts, mock proofs, signatures alone, or unchecked certificate labels
- **THEN** actual acceptance rejects the missing correctness evidence.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Fable or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.

### Requirement: Extended native predicate evidence
The package SHALL implement, review, prove, and independently verify its allocated native relation before proof-dependent acceptance.

#### Scenario: Reviewed extension executes
- **WHEN** both auditors approve the implemented relation and contract, and the allocated campaign passes
- **THEN** retained proofs and separate-process verification establish only the extended predicate and requalified dependent correspondence.

#### Scenario: Fixed proof substituted for extension
- **WHEN** the package supplies only an earlier fixed-loan proof, exceeds its campaign allocation, or lacks a required extension proof
- **THEN** package acceptance remains blocked and no earlier proof or counter can substitute.


--- FILE openspec/changes/mc05-mandatory-claim-acceptance/tasks.md ---
# Tasks: Mandatory contract, intent, transition, and history claims

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Enforce all four required claims in the real transaction acceptance path.

**Dependencies:** MC03, MC04.

**Implementation root:** `experiments/moriarty-acceptance/`.

**Interfaces:** ClaimSpec -> MandatoryManifest -> signed Intent -> BoundClaim -> ProofArtifact. VerifiedClaim records trusted verifier identity, domain, program/spec versions, dependencies, assumptions, and applicability. accept checks ContractInvariant, IntentRefinement, TransitionValidity, and HistoryCompliance.

## 1. Freeze canonical claim and policy bindings

- [ ] 1.1 Write hash-order vectors and mutation controls before changing claim codecs. Specify allowed verifiers and certificate applicability.
- [ ] 1.2 Reject altered domain, program, policy, dependency, predecessor, and mandatory-root fields.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Prove bounded contract properties and refinement

- [ ] 2.1 Implement explicit loan/swap properties and their relation to signed authority. Keep oracle assumptions separate.
- [ ] 2.2 Mechanically verify properties and positive feasibility. Reject vacuous guards and the wrong-recipient falsifier.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Wire required evidence into ledger acceptance

- [ ] 3.1 Write absent-proof and valid-but-intent-invalid end-to-end tests first. Connect MC03 artifacts through the MC04 adapter.
- [ ] 3.2 Require target rejection for every mandatory-claim mutation, including stripped or downgraded evidence.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Replace unavailable demo claims honestly

- [ ] 4.1 Expose real verified evidence only for supported profiles. Keep unsupported certificates and profiles unavailable.
- [ ] 4.2 Run browser and SDK flows against actual acceptance results and preserve every limitation.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## P. Implement and review the native extension

These tasks precede any proof-dependent acceptance or correspondence claim in this package.
Require actual signature and mandatory-claim checks in the proof/acceptance boundary. Requalify the affected MC04 theorem and adapter.

- [ ] P.1 Write `experiments/moriarty-acceptance/proof/relation-spec.md` and `experiments/moriarty-acceptance/proof/campaign-cases.json` for mandatory-claims-01.
- [ ] P.2 Add independent expected-field and rejection controls in `experiments/moriarty-acceptance/proof/encoding-controls.rs` before changing the relation.
- [ ] P.3 Implement `experiments/moriarty-acceptance/proof/harness.rs` and the separate verifier `experiments/moriarty-acceptance/proof/verify-retained.rs`.
- [ ] P.4 Implement `experiments/moriarty-acceptance/proof/run-reviewed.py` and freeze `experiments/moriarty-acceptance/proof/resource-contract.json` under the charter allocation.
- [ ] P.5 Obtain both implemented-relation and resource-contract audits before executing the frozen campaign.
- [ ] P.6 Retain actual proofs and verifier controls. Stop on first failure without automatic retry.
- [ ] P.7 Recheck affected earlier package predicates and obtain updated candidate-bound audits.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-acceptance ci
npm --prefix experiments/moriarty-acceptance run build
npm --prefix experiments/moriarty-acceptance test
lake -d experiments/moriarty-acceptance/formal build
npm --prefix experiments/moriarty-acceptance run verify-preview
python3 experiments/moriarty-acceptance/proof/run-reviewed.py --contract experiments/moriarty-acceptance/proof/resource-contract.json --preflight-only
python3 experiments/moriarty-acceptance/proof/run-reviewed.py --contract experiments/moriarty-acceptance/proof/resource-contract.json --execute
python3 experiments/moriarty-acceptance/proof/run-reviewed.py --contract experiments/moriarty-acceptance/proof/resource-contract.json --verify-retained
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC05/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Fable and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc06-private-handoff-and-composition/README.md ---
# MC06: Private witness handoff and bounded split/join

Status: specified-only.

- [Proposal](proposal.md)
- [Design](design.md)
- [Requirements](specs/mc06-private-handoff-and-composition/spec.md)
- [Tasks](tasks.md)
- [Program charter](../../MORIARTY-COMPLETION-PROGRAM.md)


--- FILE openspec/changes/mc06-private-handoff-and-composition/design.md ---
# Design: Private witness handoff and bounded split/join

## Inputs and dependencies

Dependencies: MC05.

- `docs/research/2026-09-06-pcd-report-integration.md`
- `docs/research/2026-09-06-intents-report-integration.md`
- `experiments/moriarty-acceptance/src/accept.ts`

## Interfaces

HandoffPackage lists each successor artifact, recipient, confidentiality, availability, and recovery owner. Split/Join bind distinct predecessor and output identities, compatible policies, residual authority, outstanding obligations, and a conserved global work budget.

## Outputs and ownership

- `experiments/moriarty-composition/handoff-schema.json`
- `experiments/moriarty-composition/leakage-and-recovery.md`
- `experiments/moriarty-composition/src/handoff.ts`
- `experiments/moriarty-composition/src/split.ts`
- `experiments/moriarty-composition/src/join.ts`
- `experiments/moriarty-composition/src/residuals.ts`
- `experiments/moriarty-composition/tests/alice-bob.test.mjs`
- `experiments/moriarty-composition/tests/split-join.test.mjs`
- `experiments/moriarty-composition/formal/Composition.lean`
- `experiments/moriarty-composition/formal/lakefile.toml`
- `experiments/moriarty-composition/package.json`

## Native relation extension: composition-01

Implement private successor handoff, split, independent branches, join, residual authority, and conserved global bounds.
Require private successor and real branch/join proofs. Requalify affected MC04 correspondence and MC05 acceptance.
Freeze at most 8 positive transitions and meaningful invalid proof/context controls.
The charter allocates one campaign with 20 cumulative proving/verification minutes.
Require both audits before launch and independent retained-byte verification after proving.
The original fixed-loan MC03 proof cannot discharge this extended predicate.

Owned outputs:

- `experiments/moriarty-composition/proof/relation-spec.md`
- `experiments/moriarty-composition/proof/harness.rs`
- `experiments/moriarty-composition/proof/verify-retained.rs`
- `experiments/moriarty-composition/proof/encoding-controls.rs`
- `experiments/moriarty-composition/proof/campaign-cases.json`
- `experiments/moriarty-composition/proof/resource-contract.json`
- `experiments/moriarty-composition/proof/run-reviewed.py`

## Trust boundaries

Treat source text, solvers, indexers, remote provers, generated code, and supplied receipts as untrusted inputs.
Keep oracle truth, ledger uniqueness, semantic validity, and confidentiality claims separate.
Bind all outputs to the semantic profile and the exact implementation candidate.

## Decisions and failures

Use the shared bounded Core rather than financial family names as primitive proof claims.
Preserve complete effects and positive feasibility when refining an adapter.
Record a concrete changed hypothesis before any correction.
Stop a dependent package when its prerequisite fails.
Do not reset exhausted contracts or replace a missing proof with a mock.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc06-private-handoff-and-composition/proposal.md ---
# Change: Private witness handoff and bounded split/join

## Why

Demonstrate independent private successor proving and bounded branch composition with residual authority and obligations.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Specify residual and handoff semantics.
- Implement independent private handoff.
- Implement split and join.
- Validate ledger conflicts and recovery.

- Implement the separately allocated native relation and retained-proof campaign.

## Impact

- `experiments/moriarty-composition/handoff-schema.json`
- `experiments/moriarty-composition/leakage-and-recovery.md`
- `experiments/moriarty-composition/src/handoff.ts`
- `experiments/moriarty-composition/src/split.ts`
- `experiments/moriarty-composition/src/join.ts`
- `experiments/moriarty-composition/src/residuals.ts`
- `experiments/moriarty-composition/tests/alice-bob.test.mjs`
- `experiments/moriarty-composition/tests/split-join.test.mjs`
- `experiments/moriarty-composition/formal/Composition.lean`
- `experiments/moriarty-composition/formal/lakefile.toml`
- `experiments/moriarty-composition/package.json`
- `evidence/moriarty-completion-program-2026-09-07/MC06/`

## Non-goals

No A4/A5 continuation, mainnet funds, new consensus, or Foreman repair belongs to this package.
No downstream requirement becomes complete from this package's narrower result.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc06-private-handoff-and-composition/specs/mc06-private-handoff-and-composition/spec.md ---
## ADDED Requirements

### Requirement: Independent successor witness
A successor SHALL prove using only its permitted private inputs and the declared handoff package.

#### Scenario: Alice to Bob
- **WHEN** Bob receives only his permitted private inputs and the declared handoff artifacts
- **THEN** isolated processes and secret stores produce a valid successor proof without access to Alice secrets.

#### Scenario: Unavailable witness
- **WHEN** a required handoff artifact is missing
- **THEN** the successor reports unavailable and cannot fabricate evidence or disclose Alice secrets to force acceptance.

### Requirement: Bounded history composition
Split and join SHALL preserve obligations, authority limits, output uniqueness, and the original global lifecycle measure.

#### Scenario: Compatible composition
- **WHEN** two independently proved compatible branches undergo the declared split and join
- **THEN** their new proofs discharge dependencies and preserve obligations, residual authority, uniqueness, and global lifecycle bounds.

#### Scenario: Composition attack
- **WHEN** composition duplicates predecessors, reuses outputs, mixes incompatible policies, exceeds fan-in, or resets bounds
- **THEN** proof and acceptance gates reject the invalid composition.

### Requirement: Ledger and confidentiality boundaries
The evidence SHALL distinguish history compliance, ledger uniqueness, oracle truth, and confidentiality assumptions.

#### Scenario: Competing valid branches
- **WHEN** two otherwise valid branches conflict on ledger consumption
- **THEN** both may have valid proofs, but only permitted unique consumption settles.

#### Scenario: False privacy claim
- **WHEN** a demonstration relies on a public proof alone or shared access to both parties secret directories
- **THEN** the private-handoff evidence gate rejects the demonstration.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Fable or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.

### Requirement: Extended native predicate evidence
The package SHALL implement, review, prove, and independently verify its allocated native relation before proof-dependent acceptance.

#### Scenario: Reviewed extension executes
- **WHEN** both auditors approve the implemented relation and contract, and the allocated campaign passes
- **THEN** retained proofs and separate-process verification establish only the extended predicate and requalified dependent correspondence.

#### Scenario: Fixed proof substituted for extension
- **WHEN** the package supplies only an earlier fixed-loan proof, exceeds its campaign allocation, or lacks a required extension proof
- **THEN** package acceptance remains blocked and no earlier proof or counter can substitute.


--- FILE openspec/changes/mc06-private-handoff-and-composition/tasks.md ---
# Tasks: Private witness handoff and bounded split/join

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Demonstrate independent private successor proving and bounded branch composition with residual authority and obligations.

**Dependencies:** MC05.

**Implementation root:** `experiments/moriarty-composition/`.

**Interfaces:** HandoffPackage lists each successor artifact, recipient, confidentiality, availability, and recovery owner. Split/Join bind distinct predecessor and output identities, compatible policies, residual authority, outstanding obligations, and a conserved global work budget.

## 1. Specify residual and handoff semantics

- [ ] 1.1 Define Complete, Pending, and Rejected outcomes with exact residual authority and obligations. Inventory witness ownership and leakage.
- [ ] 1.2 Review positive feasibility and global work-budget conservation before implementation.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement independent private handoff

- [ ] 2.1 Write missing-secret and forbidden-artifact tests first. Run Alice and Bob in isolated private directories.
- [ ] 2.2 Record artifact inventories and prove Bob cannot read Alice secrets through the harness.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Implement split and join

- [ ] 3.1 Write duplicate-predecessor, incompatible-policy, fan-in, and authority-amplification controls first. Implement bounded composition.
- [ ] 3.2 Verify real branch proofs, joined history, and residual conservation.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Validate ledger conflicts and recovery

- [ ] 4.1 Run competing-branch and unavailable-handoff recovery scenarios through actual acceptance.
- [ ] 4.2 Require durable consumption, explicit unavailable outcomes, and scoped confidentiality evidence.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## P. Implement and review the native extension

These tasks precede any proof-dependent acceptance or correspondence claim in this package.
Require private successor and real branch/join proofs. Requalify affected MC04 correspondence and MC05 acceptance.

- [ ] P.1 Write `experiments/moriarty-composition/proof/relation-spec.md` and `experiments/moriarty-composition/proof/campaign-cases.json` for composition-01.
- [ ] P.2 Add independent expected-field and rejection controls in `experiments/moriarty-composition/proof/encoding-controls.rs` before changing the relation.
- [ ] P.3 Implement `experiments/moriarty-composition/proof/harness.rs` and the separate verifier `experiments/moriarty-composition/proof/verify-retained.rs`.
- [ ] P.4 Implement `experiments/moriarty-composition/proof/run-reviewed.py` and freeze `experiments/moriarty-composition/proof/resource-contract.json` under the charter allocation.
- [ ] P.5 Obtain both implemented-relation and resource-contract audits before executing the frozen campaign.
- [ ] P.6 Retain actual proofs and verifier controls. Stop on first failure without automatic retry.
- [ ] P.7 Recheck affected earlier package predicates and obtain updated candidate-bound audits.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-composition ci
npm --prefix experiments/moriarty-composition run build
npm --prefix experiments/moriarty-composition test
lake -d experiments/moriarty-composition/formal build
npm --prefix experiments/moriarty-composition run verify-isolated
npm --prefix experiments/moriarty-composition run verify-preview
python3 experiments/moriarty-composition/proof/run-reviewed.py --contract experiments/moriarty-composition/proof/resource-contract.json --preflight-only
python3 experiments/moriarty-composition/proof/run-reviewed.py --contract experiments/moriarty-composition/proof/resource-contract.json --execute
python3 experiments/moriarty-composition/proof/run-reviewed.py --contract experiments/moriarty-composition/proof/resource-contract.json --verify-retained
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC06/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Fable and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc07-complete-financial-conformance/README.md ---
# MC07: Complete ACTUS and DeFi conformance

Status: specified-only.

- [Proposal](proposal.md)
- [Design](design.md)
- [Requirements](specs/mc07-complete-financial-conformance/spec.md)
- [Tasks](tasks.md)
- [Program charter](../../MORIARTY-COMPLETION-PROGRAM.md)


--- FILE openspec/changes/mc07-complete-financial-conformance/design.md ---
# Design: Complete ACTUS and DeFi conformance

## Inputs and dependencies

Dependencies: MC01, MC04, MC05, MC06.

- `evidence/moriarty-design-sprint-2026-09-06/actus-32-requirements.csv`
- `evidence/moriarty-design-sprint-2026-09-06/defi-72-requirements.csv`
- `evidence/moriarty-r2b-heldouts-2026-09-06/manifest.json`
- `docs/research/2026-09-06-actus-defi-design-study.md`
- `evidence/actus-public-source-acquisition-2026-09-03.json`

## Interfaces

CoverageRow binds stable source/fixture identity, package implementation, independent oracle, complete expected fields, positive feasibility, negative controls, certificate, compiler mapping, and actual acceptance evidence. Totals derive from immutable inventories.

## Outputs and ownership

- `experiments/moriarty-conformance/source-gap-dispositions.json`
- `experiments/moriarty-conformance/coverage.json`
- `experiments/moriarty-conformance/numeric-comparison-policy.json`
- `experiments/moriarty-conformance/src/actus-runner.ts`
- `experiments/moriarty-conformance/src/defi-runner.ts`
- `experiments/moriarty-conformance/src/coverage-gate.ts`
- `experiments/moriarty-conformance/tests/coverage.test.mjs`
- `experiments/moriarty-conformance/tests/heldouts.test.mjs`
- `experiments/moriarty-conformance/fixtures/nam19.json`
- `experiments/moriarty-conformance/fixtures/maple-refinance.json`
- `experiments/moriarty-conformance/fixtures/huma-redemption.json`
- `experiments/moriarty-conformance/package.json`

## Native relation extension: financial-coverage-01

Implement extended financial profiles, row-specific properties, intent refinement, and held-out pending obligations.
Keep all 277 ACTUS fixtures and 72 DeFi rows mandatory. Requalify affected earlier semantic, proof, compiler, and acceptance predicates.
Freeze at most 349 positive episodes with individually frozen transition bounds and meaningful invalid proof/context controls.
The charter allocates one campaign with 120 cumulative proving/verification minutes.
Require both audits before launch and independent retained-byte verification after proving.
The original fixed-loan MC03 proof cannot discharge this extended predicate.

Owned outputs:

- `experiments/moriarty-conformance/proof/relation-spec.md`
- `experiments/moriarty-conformance/proof/harness.rs`
- `experiments/moriarty-conformance/proof/verify-retained.rs`
- `experiments/moriarty-conformance/proof/encoding-controls.rs`
- `experiments/moriarty-conformance/proof/campaign-cases.json`
- `experiments/moriarty-conformance/proof/resource-contract.json`
- `experiments/moriarty-conformance/proof/run-reviewed.py`

## Trust boundaries

Treat source text, solvers, indexers, remote provers, generated code, and supplied receipts as untrusted inputs.
Keep oracle truth, ledger uniqueness, semantic validity, and confidentiality claims separate.
Bind all outputs to the semantic profile and the exact implementation candidate.

## Decisions and failures

Use the shared bounded Core rather than financial family names as primitive proof claims.
Preserve complete effects and positive feasibility when refining an adapter.
Record a concrete changed hypothesis before any correction.
Stop a dependent package when its prerequisite fails.
Do not reset exhausted contracts or replace a missing proof with a mock.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc07-complete-financial-conformance/proposal.md ---
# Change: Complete ACTUS and DeFi conformance

## Why

Complete every required ACTUS fixture and DeFi target row with independently checked financial and proof evidence.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Freeze the full coverage oracle.
- Implement held-outs before broad expansion.
- Complete financial packages incrementally.
- Attach certificates and target evidence.

- Implement the separately allocated native relation and retained-proof campaign.

## Impact

- `experiments/moriarty-conformance/source-gap-dispositions.json`
- `experiments/moriarty-conformance/coverage.json`
- `experiments/moriarty-conformance/numeric-comparison-policy.json`
- `experiments/moriarty-conformance/src/actus-runner.ts`
- `experiments/moriarty-conformance/src/defi-runner.ts`
- `experiments/moriarty-conformance/src/coverage-gate.ts`
- `experiments/moriarty-conformance/tests/coverage.test.mjs`
- `experiments/moriarty-conformance/tests/heldouts.test.mjs`
- `experiments/moriarty-conformance/fixtures/nam19.json`
- `experiments/moriarty-conformance/fixtures/maple-refinance.json`
- `experiments/moriarty-conformance/fixtures/huma-redemption.json`
- `experiments/moriarty-conformance/package.json`
- `evidence/moriarty-completion-program-2026-09-07/MC07/`

## Non-goals

No A4/A5 continuation, mainnet funds, new consensus, or Foreman repair belongs to this package.
No downstream requirement becomes complete from this package's narrower result.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc07-complete-financial-conformance/specs/mc07-complete-financial-conformance/spec.md ---
## ADDED Requirements

### Requirement: Complete ACTUS evidence
Coverage SHALL include all 18 executable types and 277 fixtures with every present result field.

#### Scenario: Full comparison
- **WHEN** the conformance harness evaluates all 277 pinned ACTUS fixtures across all 18 executable types
- **THEN** all nine present result-field kinds match explicit per-field rules for units, event ordering, calendars, and numbers.

#### Scenario: Coverage shortcut
- **WHEN** coverage skips fixtures or fields, uses a Float32 oracle shortcut, applies an unjustified global tolerance, or lacks results
- **THEN** the conformance gate fails without reducing its required denominator.

### Requirement: Complete DeFi evidence
Each of the 72 DeFi rows SHALL have row-specific behavior, refinement, feasible positives, and meaningful negative evidence.

#### Scenario: Target implementation
- **WHEN** the harness tests all 72 DeFi rows against their source-defined behaviors
- **THEN** each row has implemented refinement, feasible positives, meaningful negatives, certificates, and acceptance evidence.

#### Scenario: Taxonomy substitution
- **WHEN** a row provides only a family classification, keyword certificate, or aggregate passing count
- **THEN** the individual row remains incomplete and the full conformance gate fails.

### Requirement: Held-out semantic coverage
The package SHALL implement NAM19 capitalization, accepted refinance, and pending redemption with carried obligations.

#### Scenario: Three held-outs
- **WHEN** the harness executes NAM19 IPCI, accepted refinance, and pending redemption cases
- **THEN** IPCI capitalizes interest with zero payoff, refinance binds accepted terms and debt identity, and redemption separates request from claim.

#### Scenario: Lost obligations
- **WHEN** a held-out action makes premature payoff, unilaterally refinances, forgets unfilled redemption, or loses residual authority
- **THEN** semantic and acceptance checks reject the action.

### Requirement: Source gaps remain explicit
All 32 ACTUS taxonomy dispositions and DS-01 through DS-07 SHALL remain visible until justified resolution.

#### Scenario: Independent resolution
- **WHEN** a source decision addresses ANN initialization, CLM schedule, absent CSMP fixtures, or FXOUT event identity
- **THEN** primary-source evidence and executable tests support it while all 32 taxonomy dispositions and DS-01 through DS-07 remain visible.

#### Scenario: Fabricated conformance
- **WHEN** a source-gap disposition is submitted as a passing fixture or permission to reduce coverage
- **THEN** the coverage gate rejects the substitution and preserves the unresolved requirement.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Fable or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.

### Requirement: Extended native predicate evidence
The package SHALL implement, review, prove, and independently verify its allocated native relation before proof-dependent acceptance.

#### Scenario: Reviewed extension executes
- **WHEN** both auditors approve the implemented relation and contract, and the allocated campaign passes
- **THEN** retained proofs and separate-process verification establish only the extended predicate and requalified dependent correspondence.

#### Scenario: Fixed proof substituted for extension
- **WHEN** the package supplies only an earlier fixed-loan proof, exceeds its campaign allocation, or lacks a required extension proof
- **THEN** package acceptance remains blocked and no earlier proof or counter can substitute.


--- FILE openspec/changes/mc07-complete-financial-conformance/tasks.md ---
# Tasks: Complete ACTUS and DeFi conformance

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Complete every required ACTUS fixture and DeFi target row with independently checked financial and proof evidence.

**Dependencies:** MC01, MC04, MC05, MC06.

**Implementation root:** `experiments/moriarty-conformance/`.

**Interfaces:** CoverageRow binds stable source/fixture identity, package implementation, independent oracle, complete expected fields, positive feasibility, negative controls, certificate, compiler mapping, and actual acceptance evidence. Totals derive from immutable inventories.

## 1. Freeze the full coverage oracle

- [ ] 1.1 Import immutable inventories and every expected result-field kind. Resolve source gaps using primary evidence.
- [ ] 1.2 Add missing-row, missing-field, altered-oracle, and permissive-tolerance rejection tests before the coverage gate.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Implement held-outs before broad expansion

- [ ] 2.1 Implement NAM19, accepted refinance, and pending redemption with independent expected traces.
- [ ] 2.2 Check zero-payoff capitalization, old/new debt identity, and carried unfilled obligations.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Complete financial packages incrementally

- [ ] 3.1 Implement each remaining ACTUS type and DeFi row through the shared Core. Preserve all numeric and source assumptions.
- [ ] 3.2 Run deterministic fixture batches; stop a failing family without marking it complete.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Attach certificates and target evidence

- [ ] 4.1 Bind each row to checked properties, compiler mapping, mandatory claims, and acceptance evidence.
- [ ] 4.2 Require no skipped or unimplemented required row. Recompute totals from the frozen inventories.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## P. Implement and review the native extension

These tasks precede any proof-dependent acceptance or correspondence claim in this package.
Keep all 277 ACTUS fixtures and 72 DeFi rows mandatory. Requalify affected earlier semantic, proof, compiler, and acceptance predicates.

- [ ] P.1 Write `experiments/moriarty-conformance/proof/relation-spec.md` and `experiments/moriarty-conformance/proof/campaign-cases.json` for financial-coverage-01.
- [ ] P.2 Add independent expected-field and rejection controls in `experiments/moriarty-conformance/proof/encoding-controls.rs` before changing the relation.
- [ ] P.3 Implement `experiments/moriarty-conformance/proof/harness.rs` and the separate verifier `experiments/moriarty-conformance/proof/verify-retained.rs`.
- [ ] P.4 Implement `experiments/moriarty-conformance/proof/run-reviewed.py` and freeze `experiments/moriarty-conformance/proof/resource-contract.json` under the charter allocation.
- [ ] P.5 Obtain both implemented-relation and resource-contract audits before executing the frozen campaign.
- [ ] P.6 Retain actual proofs and verifier controls. Stop on first failure without automatic retry.
- [ ] P.7 Recheck affected earlier package predicates and obtain updated candidate-bound audits.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-conformance ci
npm --prefix experiments/moriarty-conformance run build
npm --prefix experiments/moriarty-conformance test
npm --prefix experiments/moriarty-conformance run actus -- --all --all-fields
npm --prefix experiments/moriarty-conformance run defi -- --all-rows
npm --prefix experiments/moriarty-conformance run verify-coverage
python3 experiments/moriarty-conformance/proof/run-reviewed.py --contract experiments/moriarty-conformance/proof/resource-contract.json --preflight-only
python3 experiments/moriarty-conformance/proof/run-reviewed.py --contract experiments/moriarty-conformance/proof/resource-contract.json --execute
python3 experiments/moriarty-conformance/proof/run-reviewed.py --contract experiments/moriarty-conformance/proof/resource-contract.json --verify-retained
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC07/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Fable and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc08-release-evidence-and-developer-flow/README.md ---
# MC08: Completion evidence and developer acceptance

Status: specified-only.

- [Proposal](proposal.md)
- [Design](design.md)
- [Requirements](specs/mc08-release-evidence-and-developer-flow/spec.md)
- [Tasks](tasks.md)
- [Program charter](../../MORIARTY-COMPLETION-PROGRAM.md)


--- FILE openspec/changes/mc08-release-evidence-and-developer-flow/design.md ---
# Design: Completion evidence and developer acceptance

## Inputs and dependencies

Dependencies: MC01, MC02, MC03, MC04, MC05, MC06, MC07.

- `openspec/moriarty-completion-program.json`
- `deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml`
- `docs/FOOTGUNS.md`

## Interfaces

CompletionRecord maps each requested checklist item and legacy G01-G24 gate to exact evidence and an explicit scoped status. A release record cannot inherit approval from a different source digest or a planning audit.

## Outputs and ownership

- `experiments/moriarty-release-check/src/verify-release.ts`
- `experiments/moriarty-release-check/tests/release.test.mjs`
- `experiments/moriarty-release-check/release-gate-crosswalk.json`
- `experiments/moriarty-release-check/developer-scenarios.json`
- `experiments/moriarty-release-check/package.json`

## Trust boundaries

Treat source text, solvers, indexers, remote provers, generated code, and supplied receipts as untrusted inputs.
Keep oracle truth, ledger uniqueness, semantic validity, and confidentiality claims separate.
Bind all outputs to the semantic profile and the exact implementation candidate.

## Decisions and failures

Use the shared bounded Core rather than financial family names as primitive proof claims.
Preserve complete effects and positive feasibility when refining an adapter.
Record a concrete changed hypothesis before any correction.
Stop a dependent package when its prerequisite fails.
Do not reset exhausted contracts or replace a missing proof with a mock.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc08-release-evidence-and-developer-flow/proposal.md ---
# Change: Completion evidence and developer acceptance

## Why

Close the seven requested workstreams only after reproducible evidence, independent audits, and a usable developer workflow.
The current local prototypes and network receipts do not satisfy this package's terminal predicate.

## What Changes

- Implement the final evidence gate.
- Exercise the developer workflow.
- Run both independent final audits.
- Publish the local completion dossier.

## Impact

- `experiments/moriarty-release-check/src/verify-release.ts`
- `experiments/moriarty-release-check/tests/release.test.mjs`
- `experiments/moriarty-release-check/release-gate-crosswalk.json`
- `experiments/moriarty-release-check/developer-scenarios.json`
- `experiments/moriarty-release-check/package.json`
- `evidence/moriarty-completion-program-2026-09-07/MC08/`

## Non-goals

No A4/A5 continuation, mainnet funds, new consensus, or Foreman repair belongs to this package.
No downstream requirement becomes complete from this package's narrower result.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- FILE openspec/changes/mc08-release-evidence-and-developer-flow/specs/mc08-release-evidence-and-developer-flow/spec.md ---
## ADDED Requirements

### Requirement: Evidence-gated completion
Completion SHALL require every dependency gate, resolved blocking findings, and admissible Fable and GPT-6 audits.

#### Scenario: Complete supported language
- **WHEN** a fresh checkout runs the complete program verification and both audits accept the exact candidate
- **THEN** the dossier records reproduced parser, semantic, proof, compiler, conformance, acceptance, recovery, and composition predicates.

#### Scenario: False completion
- **WHEN** closure relies on checkboxes, missing reports, stale digests, abstention, exhausted resources, or mock evidence
- **THEN** the program remains incomplete.

### Requirement: Usable developer flow
Developers SHALL author, simulate, inspect, sign, prove, submit, and diagnose representative loan and swap operations.

#### Scenario: End-to-end workflow
- **WHEN** a developer authors, simulates, inspects, signs, proves, and submits the representative loan and swap
- **THEN** the interface shows exact signed authority, verified claim predicates, finalized effects, and durable receipt status.

#### Scenario: Misleading proof status
- **WHEN** a profile is unsupported, required witness data is unavailable, or mandatory evidence rejects
- **THEN** the interface reports unavailable or rejected and cannot show proved or settled success.

### Requirement: Scope and release claims
The final dossier SHALL reconcile all seven requested workstreams and the legacy G01-G24 requirements.

#### Scenario: Explicit scope
- **WHEN** the final dossier reconciles the requested checklist with legacy G01-G24 requirements
- **THEN** each additional pilot, baseline, license, leakage, and assurance requirement has evidence or an explicit remaining release blocker.

#### Scenario: Scope laundering
- **WHEN** an unperformed legacy release requirement is marked complete solely because the seven-item checklist passed
- **THEN** the scope reconciliation gate rejects that unsupported release claim.

### Requirement: Independent audit and provenance
The package SHALL bind acceptance evidence to exact sources, commands, environment, outputs, and both required audit identities.

#### Scenario: Audited result
- **WHEN** deterministic checks pass and both independent reviewers have no unresolved blocking finding
- **THEN** the package records accepted scope with the exact reviewed candidate digest.

#### Scenario: Missing or stale audit
- **WHEN** Fable or GPT-6 is unavailable, substituted, stale, or lacks a substantive identity-bound verdict
- **THEN** the package remains pending audit and cannot promote dependent acceptance.

### Requirement: Failed predicate stops promotion
The package SHALL remain incomplete if any required positive or rejection predicate fails.

#### Scenario: Negative control incorrectly accepts
- **WHEN** a required invalid input is accepted or its rejection lacks evidence
- **THEN** verification fails and dependent acceptance remains blocked.


--- FILE openspec/changes/mc08-release-evidence-and-developer-flow/tasks.md ---
# Tasks: Completion evidence and developer acceptance

Status: specified-only. All implementation tasks remain unchecked.

**Goal:** Close the seven requested workstreams only after reproducible evidence, independent audits, and a usable developer workflow.

**Dependencies:** MC01, MC02, MC03, MC04, MC05, MC06, MC07.

**Implementation root:** `experiments/moriarty-release-check/`.

**Interfaces:** CompletionRecord maps each requested checklist item and legacy G01-G24 gate to exact evidence and an explicit scoped status. A release record cannot inherit approval from a different source digest or a planning audit.

## 1. Implement the final evidence gate

- [ ] 1.1 Write missing-package, stale-source, absent-auditor, fake-proof, and omitted-target tests before the release checker.
- [ ] 1.2 Require exact source, command, receipt, and audit digests for every closed predicate.
- [ ] 1.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 1.4 Commit only owned changes in an isolated implementation worktree.

## 2. Exercise the developer workflow

- [ ] 2.1 Run loan, swap, rejected intent, unavailable witness, pending redemption, restart, and conflict scenarios.
- [ ] 2.2 Keep local simulation, proof verification, submission, and finality visibly distinct.
- [ ] 2.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 2.4 Commit only owned changes in an isolated implementation worktree.

## 3. Run both independent final audits

- [ ] 3.1 Prepare a sealed candidate bundle after deterministic checks. Request exact Fable and fresh GPT-6 review.
- [ ] 3.2 Resolve blocking findings and recheck changed evidence. Unavailable or substituted reviewers cannot approve.
- [ ] 3.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 3.4 Commit only owned changes in an isolated implementation worktree.

## 4. Publish the local completion dossier

- [ ] 4.1 Write deliverables/moriarty-completion-program-2026-09-07/README.md and the G01-G24 crosswalk. Preserve scope limitations.
- [ ] 4.2 Close the runtime goal only when all requested predicates pass. Keep broader release blockers explicit.
- [ ] 4.3 Preserve the failing result and the corrected result with source hashes.
- [ ] 4.4 Commit only owned changes in an isolated implementation worktree.

## Verification commands

The implementation tasks create these entry points. They are not currently passing commands.
Each command requires exit 0 and the package-specific positive and negative predicates.
Commands alone cannot certify properties outside their declared scope.

```sh
npm --prefix experiments/moriarty-release-check ci
npm --prefix experiments/moriarty-release-check run build
npm --prefix experiments/moriarty-release-check test
npm --prefix experiments/moriarty-release-check run verify -- --program openspec/moriarty-completion-program.json
```

## Package closure

- [ ] 5.1 Write `evidence/moriarty-completion-program-2026-09-07/MC08/manifest.json`.
- [ ] 5.2 Bind inputs, outputs, commands, resource receipts, and the exact candidate digest.
- [ ] 5.3 Obtain independent Fable and GPT-6 result audits under the program protocol.
- [ ] 5.4 Resolve every blocking finding without widening the accepted predicate.
- [ ] 5.5 Recompute acceptance and update the program register.

## Execution contract

Status: S2, specified-only. No implementation task is complete by this plan's existence.
The program charter in `openspec/MORIARTY-COMPLETION-PROGRAM.md` controls execution and audit gates.
Every behavioral implementation task requires a failing test before code changes.
Each acceptance predicate requires an independently recomputable result.
Each result audit requires exact Fable and fresh GPT-6 identities.
Source-only review does not prove the implemented predicate.
Missing evidence, incompatible interfaces, resource stops, and unavailable audits remain explicit blockers.
Keep unrelated changes, old failed runs, and private wallet material intact.


--- SUPPORTING SOURCE docs/FOOTGUNS.md ---
# Moriarty footguns

Standing instructions adopted by the [2026-09-06 user reset](../raw/assignments/moriarty-target-first-reset-2026-09-06.md).
Read with [the postmortem](postmortems/2026-09-06-moriarty-verification-detour.md).
These rules apply to research, planning, implementation and recovery.

1. **Recover the purpose before the work queue.** Read the latest user directive
   and current roadmap notice before checkpoint obligations. A stale loop is
   not authority to continue. A4/A5 are unfinished historical experiments; the
   current task is the ACTUS/DeFi/PCD design cycle. Never report them complete to
   clear an obsolete goal.

2. **Start semantics with implementation targets.** Every proposed semantic
   operation must trace to an ACTUS fixture, a named DeFi behavior or an explicit
   developer requirement. Give the smallest example that requires it. Do not
   defer both target families until after selecting and verifying a Core.
   Product types and taxonomy categories do not automatically become Core
   constructors. Keep the complete target coverage matrix visible.

3. **Show what a developer can do.** Before a large formalization campaign,
   show representative authoring, simulation, failure diagnosis, signing and
   proof-consumption flows. Name what works, what is mocked and what is missing.
   A test-count checklist cannot replace this demonstration.

4. **Name the correctness claim.** Separate language metatheorems,
   contract-specific properties, a transaction's valid execution, PCD history
   compliance, and compiler/ledger correspondence. State assumptions, input
   domains and bounds. A passing model check, cryptographic proof or compiler
   invocation establishes only its actual predicate.

5. **Turing-incomplete does not mean automatically correct.** Require explicit
   sizes for values, intermediate arithmetic, collections, schedules, horizons,
   nesting, transaction work and predecessor fan-in. Specify rounding, overflow,
   rejection and termination. Finite state may still be too large to enumerate.
   Prefix checking needs a completeness argument before becoming a lifetime
   claim. Continuations must not silently reset a promised lifecycle bound.

6. **PCD is part of transaction acceptance.** The design must bind a transaction
   to its semantic/program versions, predecessors, authorization, observations,
   resulting state and effects. Specify constrained genesis, multi-input
   composition and the final verification decision. Hash-linked receipts and
   simulated certificates are not PCD. Missing or invalid required proofs must
   fail closed in the real acceptance path. A signed mandatory-claim root cannot
   be stripped or downgraded by a relay. An optional acceleration fallback must
   preserve the same required claim/history predicate.

7. **History compliance is not global uniqueness or oracle truth.** Define
   ledger consumption/nullifiers, ordering, finality and external-input trust
   separately. Test duplicate consumption and competing valid branches, as well
   as altered proof bytes and public inputs. State what a signature attests.

8. **Check backend compatibility early and narrowly.** Recursive verification,
   folding, accumulation, compression and zero knowledge are different
   properties. Compact's ban on recursive source functions does not rule out
   Midnight-native recursive proofs. Inspect the actual native implementation
   and its application/ledger interface separately. Name the construction and
   final verifier. Do not infer Nova–Compact compatibility from either project's existence. After specifying the
   required relation, use one small positive proof and meaningful rejection
   controls to test the actual pinned deployment interface.

9. **Bound investigations by a decision.** Before an expensive run, record the
   question, smallest decisive input, expected distinguishing outcomes, command,
   resource ceiling and stop condition in the task's existing record. Track
   cumulative effort against that ceiling. After a failure, change a justified
   hypothesis before repeating work. Do not automatically increase heaps,
   widen matrices, rerun consumed stages or arm an open-ended completion loop.
   Use an existing authorized resource budget; if none exists, propose a bounded
   experiment as part of planning rather than inventing unlimited authority.

10. **Keep process proportional.** Preserve authentic inputs, outputs and failed
    runs. Reuse those receipts. Add review or infrastructure only when it closes
    a named correctness or reproducibility gap; do not turn tooling repair,
    provenance, checkpointing or Foreman development into the product. A
    source-only review does not approve product suitability or a native result.

11. **Reuse evidence with its original scope.** E00 generated a narrow Compact
    atomic-swap specialization; its proof compilation was mock. Candidate A
    contains useful interpreter, authorization and lifecycle experiments, with
    incomplete A4/A5 acceptance. Neither is a finished general DSL. Do not erase
    useful code, promote it automatically, or force the new semantics to fit it.

12. **Report progress in user terms.** Lead checkpoints with the current goal,
    usable capability, material gaps and next reviewable deliverable. Include
    ACTUS, DeFi, proofs and developer interface status. Cite test counts only as
    supporting evidence. Report runner counters as counters, not a billing
    invoice or a measured total of wasted compute.

The current [design-cycle plan](superpowers/plans/2026-09-06-actus-defi-pcd-replanning.md)
applies these rules. Changing the plan requires preserving the reason and its
effect on target coverage, not adding another layer of approval machinery.

13. **Separate authority from outcomes and concrete plans.** Exact-plan signing
    is a restricted profile, not solver-independent intent. A refund cannot
    conceal gross over-spending, and a gross receipt before fees is not the net
    promised delivery. Check permitted intermediate recipients and calls.
    Pending progress carries residual authority and obligations; it does not
    establish a terminal goal. A valid signature, keyword certificate or
    taxonomy classification proves none of these semantics. See the
    [intents amendment](research/2026-09-06-intents-report-integration.md).


--- SUPPORTING SOURCE docs/research/2026-09-06-actus-defi-design-study.md ---
# ACTUS and DeFi requirements for the Moriarty design

Date: 2026-09-06. Status: S2 source study and design requirements; not language
implementation, protocol certification or all-vector conformance. Two bounded
research lanes inspected ACTUS and DeFi independently; the lead agent reconciled
the findings, checked decisive source passages and generated complete row
inventories. No compiler, model checker, prover or native campaign ran.

The study supports a shared bounded transition semantics with typed financial
packages. It also establishes why a transfer-only Core demonstration is not
enough: interest, loss allocation, authority, ordering and external dependencies
change the meaning of otherwise plausible balance movements.

## Coverage and source identity

| Inventory | Actual scope |
| --- | --- |
| [ACTUS matrix](../../evidence/moriarty-design-sprint-2026-09-06/actus-32-requirements.csv) | All 32 dictionary taxonomy rows have one Moriarty disposition. All 18 executable types have inspected techspec sections and representative fixtures. 276 type fixtures plus one ANN analysis-date fixture are inventoried; none was executed in this sprint. |
| [DeFi matrix](../../evidence/moriarty-design-sprint-2026-09-06/defi-72-requirements.csv) | All 72 original product rows have exact source JSON pointers, family requirements, individual deltas and explicit model omissions. Representative family source was inspected; this is not a full independent source-code audit of all 72 protocols. |
| [Source manifest](../../evidence/moriarty-design-sprint-2026-09-06/source-manifest.json) | Full repository pins, file hashes, read-only inventory scope and worked-example provenance. |

ACTUS tests are pinned at `f7a8064872b69db1f0beabac771c99dc3ce0c397`,
dictionary at `356f7663f26091105cc4fef4ae3496942dcf0ebf`, techspec at
`94ef09e4992f79d573f84f41d8480f557365870e`, and comparative Haskell at
`42451170dc61c5c11c4144bd5a69046f149e8016`. DeFiFormal is pinned at
`8ae0bbfaa3193078d1cabf6999db1382985b7f95`. Claims concern these source versions,
not the current behavior of live named protocols.

The DeFi crosswalk's inherited 60 included/12 absent construction classifications
and 36 yes/33 no/3 conditional kernel flags remain separate from proposed
language coverage. No row is marked implemented or verified. The shared
`quint-models-v2/kernel.qnt` provides arithmetic helpers; it is not an implemented
semantic kernel for every financial product.

## ACTUS requirements by executable type

The matrix contains exact type-specific source locators, fixture counts,
capabilities, pitfalls and remaining gaps. The following groups explain the
common machinery they require:

| Types | Financial behavior that must survive elaboration |
| --- | --- |
| PAM, LAM | Accrual, resets, principal redemption, interest base, caps/floors, derived maturity and final residuals. |
| LAX, NAM, ANN | Array schedules and changing principal; negative amortization without clamping; fixed installments versus recalculated annuities. |
| CLM, UMP | Calls/notices and unscheduled principal events, with explicit finite horizons and observation bounds. |
| CSH, STK, COM | Zero-payoff analysis events; dividends; quantity and price; signed purchase versus termination. |
| FXOUT, SWPPV | Currency/leg orientation, gross versus net settlement, distinct accruals and fixed/floating events. |
| SWAPS, CAPFL | Bounded child composition; stable merge and identity; paired child evaluation with controlled term overrides. |
| OPTNS, FUTUR | Exercise versus settlement; option positive-part versus signed futures payoff; observed underlying and fixing time. |
| CEG, CEC | Referenced exposure, credit triggers, collateral value, coverage caps, valuation time and settlement lag. |

Calendar functions must preserve scalar/array cycles, stubs, end-of-month rules,
business-day adjustment, distinct calculation/payment dates and event precedence.
Numeric functions must preserve scale, rounding, sign, year fraction, annuity,
quantity and conversion rules. A supplied schedule, observation or underlying
value is not an unconstrained witness that can decide the result arbitrarily.

All nine present result-field kinds matter: date, event type, payoff, currency,
principal, interest rate, accrued interest, exercise amount and exercise date.
The largest expected result list has 241 rows; that source observation is not a
universal event bound. The comparative Haskell implementation is useful evidence,
but its excluded cases and narrower/Float32 comparator prevent using its test
harness as a complete oracle.

The 14 non-vector taxonomy nodes retain their own dispositions. BCS has a
retained descriptive specification/example without a reference vector and with
completeness gaps. NAX and REP are planned. CDSWP, MAR, BNDCP, CLNTE, EXOTi,
ANX, PBN, SCRCR, SCRMR, TRSWP and BNDWR remain taxonomy-only in this inspected
executable corpus. An upstream “Implemented” label does not establish Moriarty
implementation or supply a missing executable vector.

## Source discrepancies and dispositions

These are source observations followed by explicit proposed dispositions. An
interpretation supported by a fixture is not yet an executed conformance result.

| ID | Evidence | Disposition and remaining acceptance obligation |
| --- | --- | --- |
| DS-01 | TEX schedule section says non-working day; dictionary and Haskell `DateShift.hs:33–103` shift to business days. `pam08` CSF and `pam09` SCF move payment to April 1 but report interest 26.6666666666667 versus 27.5. | Propose business-day semantics and preserve calculation-before/after-shift. Use the contrast as a required regression, then independent all-field comparison. |
| DS-02 | Dictionary `calculateShiftModifiedPreceding` repeats SCMP; TEX and Haskell distinguish SCMP/CSMP. No CSMP fixture was found in the bounded pass. | Preserve raw tokens and typed identifiers; propose CSMP for that identifier without reinterpreting existing SCMP fixtures. Add independently sourced convention tests before conformance. |
| DS-03 | ANN initial `Prnxt` TEX formula around line1688 contains `todo/todo`. | Public TEX is incomplete at this point. Derive an explicit initialization rule from the annuity definition, pinned comparative implementation and fixtures, and check independently before implementation acceptance. No invented completed formula in this sprint. |
| DS-04 | COM TD table points to PRD/STK; `com01` and Haskell `Payoff.hs:319/387` include quantity and opposite purchase/termination signs. | Propose signed quantity × unit price with purchase/termination direction. Preserve erratum and independently check all COM cases. |
| DS-05 | CLM references PR/PAM although PAM has no PR schedule; surveyed Haskell excludes CLM cases. | Keep call/redemption semantics unresolved where this matters. A complete callable package needs an explicit schedule/state rule and all-case comparison. |
| DS-06 | `fxout01` uses two MD result records where TEX names settlement-event variants. | Preserve vector event identities; reconcile the representation with the package rule before claiming FXOUT compatibility. No silent event renaming. |
| DS-07 | FUTUR taxonomy describes margining breadth beyond sampled TEX/vector lifecycle; BNDCP name and callable/puttable description differ. | Preserve taxonomy breadth separately. Futures settlement does not certify a margin-account package; do not label BNDCP a proved convertible-note implementation. |

Lossless import and numeric profiles are an additional cross-cutting obligation.
For example, the first `lam01` interest expression is an exact rational, whereas
the expected trace uses a finite decimal. A chosen representation/tolerance
must be justified per field before running a compatibility gate. The design
does not choose a convenient global tolerance to make the corpus pass.

## DeFi behavior families and their limits

| Family | Required state and properties | Boundary exposed by source inspection |
| --- | --- | --- |
| F1 exchange | Reserves, LP positions, fees, slippage, orders and fills | Uniswap constant product cannot stand in for Curve convergence, concentrated positions, hooks or order auctions. |
| F2 credit | Debt/supply shares, accrual, collateral, liquidation and loss bearer | Morpho's supplied interest increment does not establish a correct interest-rate model. Fluid combines exchange and debt in one position. |
| F3 derivatives | Signed positions/PnL, funding or borrow fees, scenario margin, expiry and reservation | The named Hyperliquid model covers Bridge2 only. Derive takes an empty-option path. Hegic's supplied payoff does not implement its pricer. |
| F4 consensus claims | Operator allocation, slash liability, rewards and exit queues | Lido/EigenLayer abstractions omit parts of validator, delay and proof behavior; custody and consensus remain named dependencies. |
| F5 external claims | Issuer/obligor, eligibility, backing statements, redemption and registry | WBTC separates requests from custodian confirmation. A model variable called reserve is not proof of external reserves. |
| F6 management | Investor claims, role-scoped mandates, caps, strategy accounting and ordered redemption | MetaMorpho omits cap timelock/fees/guardian/underlying accrual; Huma requires junior-loss and senior-redemption ordering. |
| P conditional claims | Conditions, partitions, payout rules, resolution and single redemption | Polymarket's model does not automatically cover Kalshi clearing or Azuro pool liabilities, and has its own dispute/negative-risk omissions. |
| Infrastructure | Domain-separated receipts, asset topology, consumption, timeout and finality | Bridge settlement and relayer reimbursement can finalize separately; they are not one local atomic transfer. |

Each of the 72 CSV rows adds its own requirement delta and exact source pointer.
These deltas are inherited corpus evidence interpreted for design, not newly
verified current protocol facts. Shared families are an index for study and
packages; they are not seven generic correctness proofs.

The strongest cross-family findings are that rounding changes the beneficiary
of value, ordering changes policy outcomes, and authority checks can disappear
inside abstractions. The language must make those distinctions observable.
Examples include first-minimum index selection in Derive, ordered maker fills
in Polymarket and senior-before-junior redemption in Huma. A set or aggregate
balance cannot replace the ordered computation.

## Proposed boundary and design consequences

Use bounded typed data, checked operations, state transitions and exact effects
as shared machinery. Put amortization, pricing, liquidation, allocation, exercise
and calendar rules in versioned packages where expressible. Represent external
prices/rates, outcome resolution, custody, registry, validator events, execution,
governance and cross-domain finality as explicit capability profiles.

PCD should certify execution of those exact package rules from admissible
predecessors. It must not certify a simplified model while presenting the omitted
financial behavior as covered. A contract's finite analysis/execution envelope
must also be explicit: open-ended accounts, perpetuals and pools need finite
epochs or another reviewed bounded scope. Renewal cannot silently extend a
lifetime theorem.

The [semantic proposal](../superpowers/specs/2026-09-06-moriarty-unified-semantics-design.md)
and [developer interface](../superpowers/specs/2026-09-06-moriarty-developer-interface-design.md)
apply these requirements. The row inventories are complete; package algorithms,
source-gap resolutions, all-field numeric conformance and deployment feasibility
remain the explicitly identified work needed before implementation acceptance.


--- SUPPORTING SOURCE experiments/moriarty-native-ivc-r3/README.md ---
# R3 native financial IVC experiment — blocked at k17

The fixed loan harness is implemented and was run under the specified resource
ceiling. It compiled and passed the independent financial/application-circuit
checks, but recursive VK synthesis exhausted rows at k17. **No recursive proof
was produced.** See [actual results](../../evidence/moriarty-native-ivc-r3-2026-09-07/README.md).
The specification below records the intended predicate and controls; controls
after key setup remain unexecuted. Docker/network settlement is a separate test.

Decision: can the inspected Midnight IVC interface prove and extend one bounded
Moriarty financial episode? At the fixed k17, this specialization did not fit.

## Fixed input and inspected interface

Use `midnightntwrk/midnight-zk` commit
`695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7` from SRC-0045.
`aggregation/src/ivc/mod.rs` exposes `IvcContext`, `IvcState`, `IvcIO` and
`IvcTransition`; `setup.rs:24`, `prover.rs:62` and `verifier.rs:49` expose
`setup`, `prove_step` and `verify`. Implement matching native and circuit
transitions. `IvcState::decider` must validate any full-data commitments rather
than blindly return true. `format_public_input` and `as_public_input` must bind
the same state in the same order. The verifier checks its fixed VK identity,
the application decider, proof transcript exhaustion and final KZG accumulator.
The pinned implementation uses `BlstrsEmulation` and
`CircuitTranscript<PoseidonState<F>>`; use its types rather than inventing a
Compact recursion API. SRS origin, bytes/hash, degree and setup assumptions must
be recorded when the harness exists.

Use the R2 loan first-period episode: genesis notional 5,000,000,000 micro-USD,
borrower cash 20,000,000,000, revision zero and two remaining steps. Step one
creates principal due 500,000,000 and interest due 33,972,602 while reducing
notional to 4,500,000,000. Step two transfers 533,972,602, discharges those dues
and closes this episode. It does not discharge the remaining loan notional.
Export expected before/action/after/effects from R2 and compare every field
against a separately written native relation before proving.

The proposed public state encoding is a fixed ordered vector of domain,
program, specification and intent commitments; predecessor/output identity;
revision and remaining bound; phase; notional/due/paid/cash fields; effect
commitment; and authority/obligation commitment. Encode every UInt128 as two
range-constrained 64-bit limbs and every 256-bit digest as four limbs. No
modular reduction may silently alias distinct wire values. Pin byte order and
hash preimages in fixtures. Bind the context in genesis and transition
constraints; carrying a digest does not prove its preimage's semantics.

The first native result may establish only the financial transition/history
relation with explicitly fixed authority. Label that predicate narrowly.
Dynamic Ed25519 authorization, general IntentIR refinement and all-domain
contract certificates remain required before full Moriarty acceptance. Do not
label a narrower IVC success `HistoryCompliance` for a stronger signed profile.

## Harness and stopping contract

Create a local isolated checkout at that pin and add a single proposed example
named `moriarty_loan_r3`. The harness is preserved under `harness/`. It accepts no
unbounded workload. Its intended run performs exactly two positive proving steps,
serializes a receipt with its predicate and source/input hashes, and runs the negative checks
below using retained proof bytes. Do not run the upstream example as a substitute:
it performs 1,000 Poseidon iterations per step and demonstrates a different task.

Original resource ceiling for the bounded experiment: 20 minutes cumulative
wall time including build/setup/proving, two build jobs, 8 GiB process-group
memory, one fixed `k = 17`, two proving steps and at most 256 MiB of retained
outputs. This ceiling did not establish that the circuit would fit. The executed result
above records the failure; no automatic retry or increase is authorized by it. Confirm available resources and enforce the group
limit before the command; do not approximate it with a JavaScript heap flag.

Once the example, lockfile, SRS receipt and resource wrapper are reviewable, the
inner candidate command is:

```sh
cargo run --locked --release --jobs 2 -p midnight-aggregation --features truncated-challenges --example moriarty_loan_r3
```

Record the complete outer resource-wrapper command before execution. Stop after
the first build/setup/proof failure, timeout, memory limit or failed negative
control. Preserve stdout/stderr, status and peak resource use. Do not change k,
fetch a different SRS, retry or widen the workload without recording a changed
hypothesis and remaining authorized budget. A blocked setup is a useful result.

Positive predicates: verify each step and the final two-step instance; reproduce
the R2 financial fields; reject further ordinary work after the episode closes.
Negative predicates: absent/truncated/altered proof; altered final cash or due;
wrong domain/program/intent binding; different VK; forged genesis; mismatched
predecessor; excessive authority; and an unsatisfied recursive dependency.
Final verification must discharge accumulator obligations.

## Separate deployment and witness gates

Native success does not establish ledger acceptance. SRC-0045 records
aggregation dependencies proofs 0.8/circuits 7.0 versus the inspected ledger's
0.7/6.2. Separately map the final proof, VK and public inputs to the pinned
ledger's verifier and transaction path. If no mapping exists, record the needed
adapter/version change; never substitute a mock proof provider or acceptance bit.

R4 must additionally test independent successor witnesses and split/join
histories. IVC's stateful prover API alone does not demonstrate safe cross-party
handoff. Ledger duplicate consumption and competing valid branches remain
separate checks. The [intents amendment](../../docs/research/2026-09-06-intents-report-integration.md)
adds residual authority and obligations to that relation.

## Executable artifacts and reproduction boundary

`export-episode.mjs` performs a fresh R2 build and emits `episode.json` and
`harness/episode.rs`, including original preimages and source/executable hashes.
`harness/moriarty_loan_r3.rs` supplies the finite native/application relation.
Copy the generated module to `aggregation/examples/moriarty_r3/episode.rs` and
the harness to `aggregation/examples/moriarty_loan_r3.rs` in an isolated checkout
of the pinned backend. The source catalog identifies the required local SRS.

`run-native.py` wraps the recorded command in verified cgroup limits. Its
`--correction` path requires an explicit changed hypothesis tied to a prior failed
terminal receipt and subtracts that run from the original cumulative ceiling.
The exact three invocations and source snapshots are in the evidence package.
The diagnostic-only backend patch is separate from the pinned financial relation.
Do not repeat these runs or change k based on this reproduction description;
first resolve the recorded public-state-size/resource decision.


--- SUPPORTING SOURCE evidence/midnight-preview-2026-09-07/README.md ---
# Preview public-network settlement — 2026-09-07

**Verified capability: public Preview deployment, contract call and exact state
readback.** The [combined receipt](settlement-2026-09-07T03-46-27-714Z.json) records
both transactions as SUCCESS, matches their indexer block hashes to node RPC,
and confirms both heights are at or below the finalized height of 755891.
The call settled in block **755889**, with transaction hash
`f79580fe0075dc26ae3f97f10557706f340cdf7a3b3118cd65a72b4fd870110a`.
Readback: `Moriarty Preview settlement test`.

**Original deployment observation.** The indexer returned
`SUCCESS` for transaction hash
`b38849c16cb1b628b2d6c88dcaa4ab5ec1d571ba1dbfd47547462f45a9798b7d`
in block755639. A separate node RPC read confirmed that block's canonical hash
and a later finalized height of755701 at03:27:23 UTC. See
[deployment-finality.json](deployment-finality.json) and the original
[indexer response](initial-contract-indexer.json). This is a historical
experiment observation, not a claim about current endpoint availability.

The initial deploy-and-call acceptance test **did not pass**. The first
`storeMessage("Moriarty Preview settlement test")` attempt was rejected by the
node with `Custom error: 170`. The official
[error table](https://docs.midnight.network/nodes/error-codes) identifies this as
`InvalidDustSpendProof`. Its documented regeneration remedy justified one fresh
attempt; that attempt failed locally with `could not balance dust`. At that point no call
receipt existed. The CLI read back an empty message, and the combined verifier
exited1 because the latest indexed action was still `ContractDeploy`.

**Recovery observation.** Read-only diagnostics found the DUST coin hidden by
an internal reservation even though the SDK pending list was empty. Processing
the grace period on a temporary copy exposed the coin; live timers were not
changed. Wallet and chain DUST parameters agreed. The
[inspected SDK sources](dust-source-inspection.json) show automatic revert on
submission failure, pending-list filtering through exposed ledger UTXOs, and
snapshot restoration with an empty SDK pending list. This explains why an empty
pending list alone was insufficient evidence of released DUST. The exact original
race was not captured; no upstream SDK bugfix is claimed.

Replayed only DUST in a separate private directory, retaining the same seed and
copies of the shielded/unshielded snapshots. The first scan reached its 240-second
cap and saved offset171818; one bounded continuation completed the remaining
history. The [recovered view](diagnose-2026-09-07T03-43-57-686Z.ndjson) exposed the
sequence1 coin. One instrumented submission then succeeded; see
[call output](call-2026-09-07T03-45-46-311Z.ndjson). The runner waits for fresh sync
before balancing and retains finalized transaction bytes privately before
submission. No proof-server, SDK version, key or deployed contract changed.
The cause of the original error170 is still unconfirmed; recovery plus a fresh
call succeeded, which does not prove that future proof rejection is impossible.

After finality passed, the recovered DUST snapshot was promoted to the original
runtime directory. The [promotion receipt](dust-state-promotion.json) names the
preserved original snapshot; no private state is in this repository.
A [fresh restore](diagnose-2026-09-07T03-47-02-821Z.ndjson) reported
all three children restored, a spendable sequence2 DUST coin, and no pending coin.
See the [bounded follow-up record](dust-followup-plan.md).

Completed observations:

- Wallet received5,000tNIGHT (5,000,000,000 smallest units); see
  [funding-observation.json](funding-observation.json). No faucet txID was captured.
- All three persisted child wallets restored; full sync then completed.
- DUST registration submission returned, positive DUST became available, and
  deployment completed. The registration txID was not separately retained.
- Contract address:
  `ddc676b1bfb36f29665caf17b4ae5016595af74c3dc4165e638b6af77ef40b4f`.
- [deploy.log](deploy.log) is the complete deployment console output.
  `call-cli-partial.log` retains the first rejection;
  `call-retry-cli-partial.log` retains that output plus the fresh-attempt failure.
  These are partial PTY captures, not complete independent runs.
- [deploy-call-verification.log](deploy-call-verification.log) records the failed
  combined gate. The subsequent combined receipt above records the passing call path.

This test used the pinned scaffold and local proof server from the
[Docker package](../moriarty-midnight-network-2026-09-07/README.md), with the public
Preview RPC/indexer from the wallet receipt. The bounded public experiment
initially stopped after deployment and two failed call attempts. The authorized
follow-up completed the call and finality gate. Native R3 was not rerun.
Neither this deployment nor the local application checks establish Moriarty PCD.

The user selected Preview after the official documentation/network review.
The public test now uses the dedicated unshielded address in
[wallet-public.json](wallet-public.json). Its SDK Bech32m checksum, network/type
decoding and encode/decode roundtrip passed. The observation helper independently
re-derives the address through the hello-world wallet code and requires equality
before using its state stream. Keys remain in the external private seed file
referenced by that public receipt, with mode600; no secret is in this package.

Use https://faucet.preview.midnight.network/ for this attempt. The preceding
[network review](../midnight-network-review-2026-09-07/README.md) observed its
health endpoint reporting SERVING, while the Nethermind Preview URL linked by
the documentation returned503. A normal human CAPTCHA is required. No CAPTCHA
bypass or token request was performed by the observer.

`observe-*.ndjson` preserves timestamped sub-wallet progress and unshielded
balance independently of overall `isSynced`. This directly addresses the prior
Preprod helper's invisible-funding issue. Observation lasts180seconds and sends
no transactions. It uses the installed, tested scaffold's wallet persistence
functions with external storage and an owner-only umask. Persistence and restore
results must be read from actual run receipts; a helper call alone is not proof
that all three child wallets were saved or restored.

From the main repository root:

```sh
timeout --signal=TERM --kill-after=30s 210s \
  .worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/.bin/tsx \
  experiments/moriarty-midnight-network/preview-observe.mjs
```

The existing R3 worktree supplies the installed dependencies and compiled
contract tooling; keep it while this runtime is in use. Preview wallet state
is outside the repository under
`/home/charl/.local/share/moriarty/test-wallets/preview-runtime-20260907`.
The original two Preprod seeds are preserved. Their addresses are not Preview
addresses and must not be pasted into the Preview faucet.

The funded wallet and finalized deployment now satisfy public transaction
submission. The successful contract call, exact readback and combined finality gate also passed.

For a subsequent call to the existing deployment, run the existing CLI from the
external Preview runtime directory, set `MIDNIGHT_WALLET_SEED_FILE` to the
Preview seed path in the receipt, and explicitly pass `--network preview`.
Set `MIDNIGHT_INDEXER_URL`, `MIDNIGHT_INDEXER_WS_URL`, `MIDNIGHT_NODE_URL` and
`MIDNIGHT_PROOF_SERVER_URL` to the receipt's matching configuration. Do not rely
on the scaffold's default `undeployed` network or a previous sticky network.

Read-only combined check (passed on the recorded chain state):

```sh
node experiments/moriarty-midnight-network/preview-verify.mjs \
  ddc676b1bfb36f29665caf17b4ae5016595af74c3dc4165e638b6af77ef40b4f
```

Deployment finality was checked by querying `system_chain`,
`chain_getFinalizedHead`, `chain_getHeader(finalizedHash)`,
`chain_getBlockHash(755639)` and `chain_getBlock(canonicalBlockHash)`. Acceptance
required Preview identity, indexer `SUCCESS`, equal node/indexer block hashes,
the matching node block height and height no greater than finalized height.
The retained indexer query is in `initial-contract-indexer.json`.


To submit this same test message again when authorized, run the instrumented
helper with the existing runtime as the working directory. It requires
`--submit`; no automatic retry is performed. The installed R3 worktree remains
a runtime dependency. The separate `preview-diagnose.mjs` helper is read-only
with respect to chain transactions and saves wallet sync state privately.

```sh
cd /home/charl/.local/share/moriarty/test-wallets/preview-runtime-20260907
/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/.bin/tsx \
  /home/charl/Moriarty/experiments/moriarty-midnight-network/preview-call.mjs --submit
```
