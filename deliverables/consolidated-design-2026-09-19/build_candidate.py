from pathlib import Path
import json,shutil,re
R=Path(__file__).parent; V=Path('/home/charl/Moriarty-aeon-study'); C=R/'candidate'; C.mkdir(exist_ok=True)
def put(p,s):
 q=C/p;q.parent.mkdir(parents=True,exist_ok=True);q.write_text(s)
design=(R/'design-draft.md').read_text()
design=design.replace('## Native proof-carrying history\n','''## Native proof-carrying history

Planning assumption supplied by the user on 2026-09-19: Midnight will have comprehensive recursion in about six months, approximately March 2027. The target architecture therefore includes full native recursive compliance, portable history, private handoff and bounded multi-parent composition. Prepare these relations, interfaces and tests now; qualify the actual released interfaces before reporting support. This is a planning assumption, not a verified delivery date. Current limitations determine the early implementation subset, not the ultimate language scope.

''')
design += '''
## Backend evolution contract

[Next ZKIR and recursion requirements](MORIARTY-BACKEND-REQUIREMENTS.md) separate mandatory correctness contracts, required capabilities and measured usability targets. They assign obligations to ZKIR, compiler, proof runtime, ledger and Moriarty rather than assuming new instructions alone solve authorization, privacy or settlement. The initial execution target remains ZKIRv3; requests for the next version do not claim a ZKIRv4 release or prescribe its numbering.

## Source basis

- [Whole-language audit](../deliverables/whole-language-review-2026-09-19/REVIEW.md): integration, reserves, lifetime state, expiry and roadmap conflicts.
- [DeFiFormal](../deliverables/defiformal-study-2026-09-19/RESULT.md): composition and reusable financial reference contracts.
- [Aeon consensus](../deliverables/aeon-study-2026-09-19/review/FINAL-CONSENSUS.md) and [Anoma study](../deliverables/anoma-study-2026-09-19/SYNTHESIS.md): refinements, certified basis and constraint-preserving solver completion.
- [MPLR register](../wiki/research/mplr/index.md): NEAR, Daml, Simplicity, APSS, OWS/x402 and Anoma requirements and source provenance.
- [Six independent proposals and synthesis](../deliverables/consolidated-design-2026-09-19/CONSENSUS.md): exact scope, disagreement dispositions and final review.

Historical native-interface findings remain tied to their original pins and dates. No fresh backend build or network execution is claimed by this consolidation.
'''
# Clarify all subtle proposal conflicts.
design=design.replace('A workflow may be submitted without funding;', 'An uncommitted candidate alone changes no ledger state or obligations. Any accepted phase, including a retained guaranteed phase, can create or preserve only explicitly authorized duties. Recording a request does not impose a duty on an unconsenting recipient.\n\nA workflow may be submitted without funding;')
design=design.replace('Ordinary authority can expire', 'Initiate, complete, reconcile, recover, disclose and amend rights have separate scopes; no fixed default expiry or perpetual spending right is implied. Knowledge reconciliation does not itself authorize a new transfer.\n\nOrdinary authority can expire')
design=design.replace('A TEE attestation cannot replace', 'A foreign destination that accepts only a threshold signature can be bypassed if that threshold is compromised, even when honest signers check proofs. State that conditional enforcement boundary and the federation’s membership, threshold, ordering, equivocation, availability and epoch-change rules explicitly. Signed fallback policies must not become an unsigned evidence downgrade.\n\nA TEE attestation cannot replace')
put('docs/MORIARTY-CONSOLIDATED-DESIGN.md',design)
road=(R/'roadmap-draft.md').read_text().replace('## One implementation sequence','''## Recursion planning horizon

The user supplied a six-month planning assumption on 2026-09-19: comprehensive Midnight recursion around March 2027. Full native recursion and private multi-parent composition are target capabilities, not optional replacements. Prepare U4 and the [next ZKIR/recursion requirements](docs/MORIARTY-BACKEND-REQUIREMENTS.md) during U0/U1. Current interfaces can support an early scoped milestone; their limitations do not reduce MC03/MC06. Revalidate actual released interfaces and costs before claiming support.

## One implementation sequence''')
put('ROADMAP.md',road)
base='openspec/changes/consolidated-language-kernel/'
put(base+'proposal.md','''# Consolidated Moriarty language and kernel proposal

## Why

Research produced overlapping language, partial-transaction and kernel plans. Current source profiles, native evidence and financial examples do not yet form one general source-to-ledger relation. Ambiguous history, authority expiry and model/runtime boundaries can weaken the intended product.

## What changes

Adopt one [vision and design](../../../docs/MORIARTY-CONSOLIDATED-DESIGN.md), [U0–U7 roadmap](../../../ROADMAP.md), [backend requirements](../../../docs/MORIARTY-BACKEND-REQUIREMENTS.md), and [traceability register](traceability.md). Preserve permissionless public deployment, native ZKIRv3 and full recursive/private-history scope. The federated kernel coordinates optional services under signed intention; it cannot authorize programs or replace native proofs. No Lean dependency is introduced.

## Impact and status

Documentation and requirements only. All implementation exits remain open. Existing P/C/K plans become historical detail/provenance; MC/SP/G acceptance and resource histories remain. This proposal creates no public validity gate, native proof, deployment or new resource budget. The six-month recursion horizon is user-supplied planning context, not a verified release promise.

## Acceptance

One governing schedule, every MPLR-001–035 and inherited package mapped, explicit EARS positive/negative scenarios, bounded Pel workflow with readiness limits, backend requests with owners/tests, recorded six-expert synthesis, and working navigation. See [tasks](tasks.md) and [workflow](workflow.md).
''')
put(base+'design.md','''# Design authority

The single architecture is [Moriarty consolidated vision and design](../../../docs/MORIARTY-CONSOLIDATED-DESIGN.md). The public [product contract](../../../docs/MORIARTY-PRODUCT-CONTRACT.md) remains controlling. This file deliberately routes to the design rather than maintaining a competing copy.

The [backend contract](../../../docs/MORIARTY-BACKEND-REQUIREMENTS.md), [EARS scenarios](specs/consolidated-language-kernel/spec.md) and [traceability](traceability.md) supply its testable obligations. Native recursive history and private composition remain full-scope targets under the user's approximately March 2027 planning horizon.
''')
reqs=[
('UNI-001','Permissionless public pipeline','When a developer presents a supported program and valid required evidence, the public pipeline SHALL decide validity without project, registry, council, Foreman or hosted-service membership.','A clean developer authors and submits a novel supported program without project records.','Objective checks accept it under owner and ledger rules.','Only a reviewer receipt is removed from an otherwise identical valid candidate.','Validity is unchanged; a receipt cannot substitute for a missing proof.'),
('UNI-002','One semantic relation','When an accepted stage changes state, the implementation SHALL bind program, profile, signed intention, current authority, history, observations, complete effects, costs and residual duties through one versioned relation.','Two source constructs elaborate into the supported Core with checked representations.','Their target behaviors satisfy the same scoped relation.','A legacy lowering path drops authority or a mandatory predicate.','The path cannot claim certified acceptance.'),
('UNI-003','Native target and correspondence','When a program claims certified execution, the toolchain SHALL establish valid-execution completeness and adversarial-witness soundness for its pinned ZKIRv3 or explicitly qualified successor target and actual ledger interface.','A source-valid execution has a witness under the bound target relation.','Native verification and actual effects agree with its specified outcome.','An adversarial witness omits a range or assertion constraint.','It is rejected or the missing soundness obligation prevents certification.'),
('UNI-004','Intent and solver completion','When a solver completes or combines intentions, acceptance SHALL preserve every applicable signed constraint, including allowed choices, gross caps, fees, net outcomes, liabilities, disclosures and recovery.','Two independent solvers choose distinct permitted routes.','Both receive the same objective acceptance treatment.','A plan changes recipient, fee asset or obligation consent while preserving net balance.','Acceptance rejects the unauthorized effect.'),
('UNI-005','Complete financial state','When a stage commits effects, acceptance SHALL account separately for assets and authorized supply, gross debits and fees, authority consumption, liabilities and residual duties over the authenticated complete state domain.','A partial fill consumes a proportional authorized amount and retains its remainder.','Accounting and cumulative limits remain correct.','Netting hides a fee or an omitted liability/reservation.','Acceptance rejects the incomplete or unauthorized accounting.'),
('UNI-006','Phases and partiality','When a target permits retained effects after failure, the compiler SHALL enforce the signed phase-specific effects, fees, authority consumption and remaining duties without representing them as global rollback or workflow completion.','The fallible phase fails while an authorized guaranteed fee remains.','The result records that fee and unresolved duties.','An uncommitted incomplete candidate is presented as a completed workflow.','No ledger completion or unconsented duty is inferred.'),
('UNI-007','Conditional settlement','When delivery is conditional, acceptance SHALL require the authorized evidence combination and distinguish request recording, funding, eligibility, in-flight effects and actual delivery.','Recipient consent and the bound document predicate become valid for funded escrow.','The permitted release can execute and be recorded after actual delivery evidence.','A document digest, funding receipt or mere timeout is substituted for delivery conditions.','Release or completion is rejected and unresolved duties remain.'),
('UNI-008','Recovery and state limits','When a workflow reaches expiry, work or state limits, supported recovery or successor transitions SHALL preserve applicable consent, cumulative budgets, replay state and duties under their own authority and resource rules.','Ordinary work is exhausted but an authorized funded recovery path remains.','Only that path may consume the reserved closure work.','Rollover resets a spent budget or expiry erases debt.','Acceptance rejects the reset or erasure.'),
('UNI-009','Full history scope','When compliant history is claimed, evidence SHALL establish authenticated origin, compatible well-founded predecessors and unique resource use under the stated mode; ledger induction SHALL NOT count as native recursive or private split/join evidence.','Two native financial steps and a bounded private join verify under the released compatible interface.','Their recursive and composition claims have retained independent verification.','A fabricated base, arbitrary verifier, repeated parent resource or finalization omission is supplied.','Verification rejects the invalid history.'),
('UNI-010','Privacy and completeness','Where private continuation is claimed, acceptance SHALL bind the authenticated completeness domain and permitted observations, while the implementation supplies the stated witness-handoff and availability mechanism.','A separately isolated successor gets authorized private inputs.','It continues under the declared privacy and history relation.','A hidden reservation is treated as absence or unauthorized information is disclosed.','The claimed completeness or privacy property is not accepted.'),
('UNI-011','Federation and adapter boundary','When a kernel requests an external effect, its enforcing signer or destination SHALL bind exact decoded bytes and effects to the same intention, stage, domain, epoch and evidence policy, with ZK, MPC, TEE and finality assumptions separately stated.','Proof and threshold authorization bind the same permitted transfer.','The adapter signs or executes only those exact effects under its declared assumptions.','Proof for X is used to sign Y, decoding is incomplete, or a fallback weakens the policy.','The request is rejected; compromised bare-threshold enforcement remains an explicit trust boundary.'),
('UNI-012','Delegated AI spending','When concurrent delegated tasks commit funds, acceptance SHALL enforce attenuated owner authority and durable aggregate spent/pending/reserved budgets across retry and failover.','Requests for six and four units including fees share a ten-unit budget.','Both can reserve without exceeding ten.','Concurrent requests for six and five units use copied parent budgets.','At least one is rejected or deferred.'),
('UNI-013','Service lifecycle','When payment or result retrieval is retried, the workflow SHALL preserve authenticated logical request identity and distinguish payment finality, result availability and recipient delivery.','A paid request retries result retrieval.','Existing payment is reconciled without an unauthorized second charge.','A finalized payment or TEE-local result is claimed as recipient delivery.','The delivery duty remains unresolved.'),
('UNI-014','Certified libraries','When an optimized primitive or financial library is substituted, correspondence SHALL preserve values, refusal, complete effects, declared costs and discharged preconditions, including exact units, price orientation and rounding.','A native port converts an exact positive quote/base ratio before directed rounding.','The declared economic predicate and target constraints hold.','A port reuses opposite price type spelling, rounded reciprocal or a certificate for another implementation.','Qualification rejects the mismatch.'),
('UNI-015','Semantic evolution','When program, verifier, policy, federation epoch or persistent state evolves, acceptance SHALL preserve signed semantics, consumption, recovery, privacy and obligations or require applicable amendment consent.','An authorized migration retains duty and replay commitments.','The successor continues under the bound version.','An interface-compatible upgrade changes beneficiary or resurrects consumed authority.','Migration is rejected.'),
('UNI-016','Assurance and release','When tools or release reports claim capability, they SHALL distinguish specified, inspected, tested, proved-under-assumptions, locally accepted and Preview-finalized evidence for the exact scope, retaining all required conformance and release rows.','All required rows carry exact artifacts, commands and supported predicates.','Only demonstrated capabilities are reported complete.','Six advisor votes, finite samples or a static Pel check are supplied as proof of implementation.','Implementation and full-release status remain open.'),
]
spec='## ADDED Requirements\n\n'
for id,title,rule,pos,then,neg,nthen in reqs:
 spec+=f'### Requirement: {id} {title}\n{rule}\n\n#### Scenario: Positive witness\n- **WHEN** {pos}\n- **THEN** {then}\n\n#### Scenario: Hostile witness\n- **WHEN** {neg}\n- **THEN** {nthen}\n\n'
put(base+'specs/consolidated-language-kernel/spec.md',spec)
put(base+'requirements.md','''# Consolidated EARS requirements

[UNI-001–016](specs/consolidated-language-kernel/spec.md) are the normative consolidation clauses with positive and hostile scenarios. Existing MOR-001–012, AEO-001–004 and MPLR-001–035 requirements remain incorporated as detailed behavioral obligations; DEV-001 applies only to internal delivery. [Traceability](traceability.md) names their single phase owners and retained MC/SP lineage. Grouping does not erase an individual scenario or theorem obligation.

The [next ZKIR and recursion requirements](../../../docs/MORIARTY-BACKEND-REQUIREMENTS.md) add backend-specific capability and correctness requests. All are specified-only unless a row identifies scoped observed evidence. Their implementation belongs to the named component; no backend document substitutes for Moriarty financial semantics.
''')
# Complete aliases and MPLR owner table.
alias={'P0':'U0','P1':'U1','P2':'U2, U4','P3':'U0–U2 authoring support','P4':'U3, U4','P5':'Optional authoring support after U2','P6':'U6','P7':'U7','C0':'U0','C1':'U2, U3','C2':'U3','C3':'U3, U4, U5','C4':'U6, U7','K0':'U0','K1':'U1, U4','K2':'U2, U3, U4','K3':'U5','K4':'U5','K5':'U5, U7','MC01':'U0, U1, U2','MC02':'U2, U7','MC03':'U4; feasibility U0/U1','MC04':'U2, U3, U4','MC05':'U2 subset; full closure U4 with MC03/MC04','MC06':'U4 with U3 lifecycle','MC07':'U6','MC08':'U7','SP01':'U0, U1','SP02':'U0, U2','SP03':'U0, U1, U2','SP04':'U0, U1, U2, U4','SP05':'U2','SP06':'U4; feasibility U1','SP07':'U6','SP08':'U6','SP09':'U2 subset, U4 full','SP10':'U3, U4','SP11':'U6','SP12':'U7'}
trace='''# Traceability and dispositions

This is a scheduling crosswalk, not an acceptance waiver. All inherited objective requirements, source gaps, positive/hostile witnesses and MC/SP/G denominators survive. The [earlier detailed reconciliation](../../../docs/ROADMAP-RECONCILIATION-2026-09-19.md) retains the individual requirement inventory. Its P/C/K owner labels now resolve through this table. Frozen reports and machine campaign registers remain historical; they do not independently select a competing queue.

## Prior plan aliases

| Retained ID | Canonical owner | Disposition |
|---|---|---|
'''
for k,val in alias.items(): trace+=f'| {k} | {val} | Preserve objective predicates and evidence; use U schedule |\n'
trace+='''
G01–G24 remain individually required under U7; ACTUS all 277 fixtures and expected fields, 18 executable types, 32 taxonomy dispositions, DS01–07; DeFi all 72 original rows, DA01–24, adopted supplementary cases, three held-outs, eight intent cases and eight regression classes remain U6 scope. All five retained composition operators remain U4 scope. Actual source gaps remain open; no denominator is reduced by library batching. Validate these inherited counts against the frozen coverage manifest before implementation and record any additive revisions.

## MPLR behavioral owners

Each row retains the entire linked research requirement and its existing detailed EARS scenarios, not only the short title. Full research-to-proof closure remains open. UNI clauses consolidate cross-cutting contracts, and do not merge away individual tests.

| MPLR | Required behavior | Primary owner | UNI | Evidence status |
|---|---|---|---|---|
'''
maprows={1:('U3','006'),2:('U3','002,008'),3:('U3','007'),4:('U3','005'),5:('U3','008'),6:('U3','007,011'),7:('U3/U4','009'),8:('U3','004,008'),9:('U3','005,008'),10:('U3','007,008'),11:('U4','005,009'),12:('U2','016'),13:('U4','010'),14:('U2','003,006'),15:('U2/U7','001'),16:('U0/U7','001'),17:('U2/U3','005'),18:('U2','004,007'),19:('U2/U3','004,005'),20:('U1','014'),21:('U1/U3','008,014'),22:('U2/U4','002,003,009'),23:('U2','002,004'),24:('U3/U5','006,011'),25:('U6','005,014'),26:('U5/U6','011,014'),27:('U4','009'),28:('U4','015'),29:('U4','005,010'),30:('U5','011'),31:('U5','012'),32:('U5','012'),33:('U5','013'),34:('U5','013'),35:('U2/U5','004')}
idx=(V/'wiki/research/mplr/index.md').read_text()
for n,(owner,uni) in maprows.items():
 id=f'MPLR-{n:03}'; m=re.search(r'\['+id+r'\].*?\| ([^|]+) \|',idx);title=m.group(1).strip() if m else id
 trace+=f'| [{id}](../../../wiki/research/mplr/{id}.md) | {title} | {owner} | UNI-{uni} | specified-only |\n'
trace+='''
MOR-001/002/005 → UNI-001 (U0/U2/U7); MOR-003/004/009/010/011/012 → UNI-002–006/009/014 (U0–U4); MOR-006/007/008 → UNI-005–010 (U3/U4). AEO-001–004 remain obligation reporting, exact supported checks, replayed counterexamples and bounded synthesis; they never supply ledger evidence. DEV-001 remains internal.

## Conflict dispositions

| Conflict | Controlling disposition |
|---|---|
| Project gates vs permissionless language | Project review/resource controls only govern maintainer work. Objective proofs and owner consent remain public validity conditions |
| Optional Lean bridge and planned Lean paths | Superseded as active delivery work; K reference and native Midnight stack remain; original paths/receipts retained as history |
| Older categorical rejection of DAG/recursive history | Superseded for target scope. Full native recursion/private multi-parent composition planned around user-assumed March 2027 horizon; actual interface qualification remains open |
| Ledger induction vs MC03 recursion | Distinct evidence modes. No renaming of MC03 or closure through ledger induction |
| P/C/K overlapping schedules | Superseded as schedules by U0–U7; detailed requirements remain incorporated |
| Local atomicity vs Midnight phases vs multichain | Separate relations; explicit retained effects and saga/recovery; no global rollback claim |
| Reserve unavailable to ordinary work | Preserve rule; add separately scoped recovery or admission viability, never indiscriminate reserve spend |
| 128-ID cap vs persistent service | Explicit finite episode or authenticated successor with all duties/cumulative limits; no proof-compression shortcut |
| Expiry vs late evidence | Separate initiative, completion, reconciliation, recovery/disclosure/amendment authority; no automatic perpetual spending right |
| DeFiFormal registry/grants/net clearance | Reference assumptions, not deployment licensing, owner consent or full obligation discharge |
| Same price type name, opposite orientation | Exact unit/orientation conversion before directed rounding and native qualification |
| Six-month recursion forecast | Dated user planning assumption; not independently verified release fact |

The six-expert synthesis additionally disposes of proposal errors: phase-partial outcomes can create authorized duties; merely recording a request cannot impose recipient duties; kernel actors can supply authenticated state/history artifacts but cannot redefine them; different representation hashes need correspondence, not literal equality; fresh backend inspection must precede present-tense capability claims.
'''
put(base+'traceability.md',trace)
put(base+'tasks.md','''# Single implementation task register

All product tasks below remain unchecked. This documentation proposal does not dispatch them.

- [ ] U0 — Freeze one stage/intent/history relation, profile embeddings, target tuple and next-backend requirement matrix.
- [ ] U1 — Certify the minimum exact primitive basis and measure native signature/phase/recursion feasibility.
- [ ] U2 — Demonstrate the public signed source-to-ZKIRv3-to-Midnight path with contrasting programs and complete effects.
- [ ] U3 — Implement conditional escrow, partial fills, persistent continuations, late-result recovery and work/state boundary behavior.
- [ ] U4 — Deliver full native recursive financial history, bounded private composition, completeness and isolated handoff.
- [ ] U5 — Integrate the optional federated kernel, exact-effect adapter, constrained OWS delegation and x402 lifecycle.
- [ ] U6 — Qualify all retained ACTUS/DeFi libraries, held-outs and conformance rows.
- [ ] U7 — Complete independent public developer flow and every retained release obligation.

For each item, use the [roadmap exit](../../../ROADMAP.md), [EARS](requirements.md), [traceability](traceability.md) and [bounded workflow](workflow.md). P/C/K task lists are supporting detail; do not dispatch them as separate queues.
''')
put(base+'workflow.md','''# Internal bounded delivery workflow

This workflow governs maintainer work only. Public developer tools never read Pel, reviewer or campaign records as validity inputs.

Select one missing U0–U7 acceptance predicate. Freeze a five-part task with objective, exact owned files, interfaces, constraints and verification. Include positive/hostile expected results from an independent reviewer. Bind the base and candidate bytes, semantic/target profile, requirements, existing executable paths and argv, tool versions, input hashes, output artifacts, timeout/memory/CPU/submission/debit ceilings and current cumulative charges. Missing command or target interface means specified-only or interface-blocked, not ready.

Reconcile existing internal accounting before its dependent action. Use the repository development plugin for registered dispatch. Verify the actual caller path; retain failed observations. Request non-author review of the complete frozen result, preserve dissent and correct within the authorized bound. A successful local test or reviewer vote does not establish native proof or Preview financial acceptance. Do not build new orchestration infrastructure.

## Known executable checks and limits

- `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` reports internal state only.
- `npm --prefix experiments/moriarty-language run typecheck` and that package's test commands check their existing local scope; inspect scripts before selection.
- `openspec validate consolidated-language-kernel --strict` checks this specification's structure.
- The installed Pel `check` and `plan` commands check the internal template's structure and symbolic plan.

General source-to-native-proof/ledger commands do not yet exist as a qualified full path. U0 must bind actual inspected commands and their supported predicates. This document does not invent `prove-all` or equivalent readiness.

## Pel status

[implementation.pel](implementation.pel) is a bounded symbolic internal template, not a dispatched or ready production lane. Bind `artifact:consolidated-execution-packet`, the exact implementer/reviewer identities, candidate schema and `candidate-full` verification policy to real commands and artifacts before use. The packet must include the above limits and acceptance predicate. Static checking cannot provide those bindings.

The template permits an initial candidate and one correction, skips review when verification fails, and stops without claiming completion after the bound. A no-change result requires diagnosis rather than another process-only loop. Native recursion requirements and cost measurements do not grant new proving budget. Resource and public transaction notifications follow existing project rules.
''')
# Reuse existing bounded Pel semantics; preserve explicit non-ready labeling.
pel=(V/'openspec/changes/permissionless-provable-intention/implementation.pel').read_text().replace('artifact:approved-spec','artifact:consolidated-execution-packet')
put(base+'implementation.pel',pel)
print('candidate files',len(list(C.rglob('*.*'))))
