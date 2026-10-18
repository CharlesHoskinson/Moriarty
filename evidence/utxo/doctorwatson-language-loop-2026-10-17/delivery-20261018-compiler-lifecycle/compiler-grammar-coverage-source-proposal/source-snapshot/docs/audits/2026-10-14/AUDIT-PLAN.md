# End-to-end Moriarty audit plan

Status: audit design and static reconnaissance; execution and release acceptance are not complete. Source baseline: `cb8280ef54082853dd8bb6c6a52050328d0fc6fa`, tree `ce05a5c777e5e60f9bebfb1f5a80df78a2556096`. Six Luna mapping passes and ten independent initial proposals informed this plan. The individual reports and dissent remain preserved. GPT covers nonsecurity work. Opus owns all security analysis, findings, attack-test design and security judgments under the user's latest instruction.

## Recommended approach

Use a requirement/evidence register together with two independent developer walkthroughs. The register prevents omissions; the walkthroughs expose broken integration. Add grammar-by-consumer and field-by-boundary coverage so two successful examples cannot stand in for generality. Use the existing language, CLI and test harnesses; create no scheduler or application-specific compiler.

Two alternatives remain useful subordinate techniques. A component-first review is easier to divide among reviewers but misses seams between profiles and consumers. A single golden end-to-end example finds integration failures quickly but does not cover the admitted grammar, financial families or release obligations. Neither is a sufficient replacement for the combined approach.

## Required outputs

Keep one dossier rather than a new operational framework:

| Output | Required content |
|---|---|
| `scope.json` | Frozen commit/tree, reviewed profiles, actual inspected paths, exclusions, reviewer/model identities, public input hashes and claim limits |
| `profiles.json` | Public executable/install path, source and semantics identifiers, parser, checker, evaluator, lowering, ABI, supported grammar, qualification and refusal boundaries |
| `predicates.jsonl` | Atomic falsifiable predicates linked to every retained requirement; owner, target/profile, oracle, evidence, status and blocker |
| `enforcement.jsonl` | Origin and representation of each intent/state/effect/history field; each enforcement/verification consumer and its unresolved premises; security portions authored by Opus |
| `oracles.jsonl` | Independently authored financial, work and semantic expectations, derivations, common-code dependencies and freeze time |
| `chains.jsonl` | Every program/stage attempt, actual input/output identities, command, result, complete financial observations and first blocked/divergent boundary |
| `security/` | Opus threat model, independently substantiated findings, security test matrix and their scopes; never GPT security verdicts |
| `evidence.jsonl` | Exact source/receipt/run identity, observation method, source tree, target, scope, digest, limitations and fresh/historical status |
| `coverage.json` | Grammar/consumer cells, field mappings, financial families and retained requirement IDs; explicit unknown and unflattened clauses |
| `REVIEW.md` and roadmap patch | Usable capabilities, confirmed gaps, hypotheses, blockers, disagreements, next demonstrations and unchanged parent exits |

`obligation-index.json` in this discovery package indexes 69 top-level U/M/MC/SP/G IDs. It is not the finished atomic predicate ledger. The linked complete clauses, L/ZR/MNR/MPLR/UNI/MOR/AEO requirements, financial fixtures and composition operators remain required. Do not calculate an overall completion percentage from this index.

Candidate predicate record (a specification, not a run):

```json
{"schema":"moriarty-audit-predicate/1","id":"U2.compiler.shared-subset",
 "parents":["U2","M1","MC01","MC04"],"profile":"moriarty/1",
 "consumer":"mori compile-call","target":"local-unqualified",
 "predicate":"each admitted construct preserves the independently specified reference observation",
 "implementation_status":"partial","evidence_class":"source-inspected",
 "acceptance":"open","evidence_ids":[],"oracle_id":null,
 "blocked_by":["required-native-evidence"],"owner":"compiler-correspondence",
 "security_review_owner":"Opus","not_claimed":["native proof","ledger settlement"]}
```

Separate implementation status (`implemented`, `partial`, `stub`, `specified`, `unknown`) from evidence class (`source-inspected`, `historical-local`, `current-local`, `native-local`, `target-observed`) and acceptance (`open`, `accepted`, `refuted`). A source inspection, captured result, or review approval never automatically advances another axis. No pass is inferred downstream of a blocked stage.

## Independent program walkthroughs

Commission two positive programs within the same explicitly claimed supported pipeline. One author writes source from public documentation; another freezes independent expectations before seeing execution output. Record the required contrast in syntax, state, arithmetic, assets, effects and duties. Prefer:

- **Installment invoice:** one debit owner, fees to a distinct recipient, creation/accrual/partial discharge of a duty, continuation and complete unrelated-state preservation.
- **Conditional reserve allocation:** the same single-payer admission where native success is claimed, but two assets, state transitions selected by action guards, separately specified quotient/remainder arithmetic, several recipients and a different duty/effect structure.

These are proposed specimens, not implemented or executed programs. Use only constructs actually admitted by the selected grammar. Core has no general division primitive: an appropriate finite quotient/remainder construction must specify its ranges and arithmetic contract. Do not silently replace unsupported source with hand-written ZKIR or host logic.

Mixed-payer mapped-role compilation is an additional static capability/negative-control track. A correct installed/native refusal is useful evidence but cannot count as the second successful U2 program. The broader two-asset partial escrow/recovery discriminator remains U3's lifecycle target, including the retained spend-at-most-11-A/fees-at-most-1-A/receive-at-least-20-B contract. Neither the new specimens nor current FeePool fixtures replace grammar coverage or full conformance.

For each program retain every boundary:

1. Clean installation and executable resolution; source profile, diagnostics and typed declarations.
2. Source-to-Core embedding, numeric units and evaluation order, admitted grammar and complete independent reference outcomes/work.
3. Canonical displayed intent and its exact program/action/parameter/state/domain binding; security enforcement is reviewed by Opus.
4. Actual public compiler, target descriptor, emitted ZKIR, ABI/statement field mapping and the reason for any lowering refusal.
5. Separately admitted native proof production and independent verification of the same claimed source/statement; no mock or static-export substitution.
6. Target installation/deployment and call, independent authoritative readback, and reconciliation of all balances, custody, fees, supply changes, duties and residual liabilities.
7. Successor history, partial progress, restart, unknown finality and recovery, with each field's authenticated or supplied origin recorded.

Compare both lowerings only on their documented common subset. Different consumer restrictions need a reasoned admission lattice; do not require identical bytes or infer that two relations with different statements must be globally equivalent. Generality requires rule-level preservation/completeness and field coverage, not just two demonstrations.

## Oracle and conformance requirements

Use independent hand-derived small examples plus an independently implemented exact arithmetic oracle for larger cases. Expected values must not import the evaluator or compiler under audit. Wrapper/API equality is a useful integration check, not an independent semantic oracle. Count logical work from a versioned schedule separately from target circuit cost, DUST, service fees and recovery reservations.

Check complete ordered effects, all affected and preserved state, gross debit, fees, net receipts, reserve dust, opening/accrued/discharged/closing liabilities and present-zero versus absent duties. Ordinary conformance includes documented overflow, eager evaluation, work exhaustion and unsupported constructs. Opus supplies all threat models, hostile cases, cryptographic/authorization/security tests and judgments.

K and Quint comparisons require explicit profile and field adapters, premise inventories, original traces and implementation replay of meaningful counterexamples. Model booleans or sampled traces do not establish implementation or native correspondence. Retain the timed-out large Quint sample as unknown and the aggregate test-report size failure as unresolved engineering work.

Preserve ACTUS all 277 fixtures and expected fields, 18 executable types, 32 taxonomy dispositions and DS01–07. Preserve DeFi all 72 rows, DA01–24, adopted supplements, three held-outs, eight intent cases and eight regression classes. Validate membership against frozen manifests before scoring. Preserve sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging composition. The [canonical traceability document](../../../openspec/changes/consolidated-language-kernel/traceability.md) retains the complete requirements.

## Workstreams and execution boundary

| Workstream | Owner | Dependency and output |
|---|---|---|
| Profiles, semantics and admission lattice | GPT/nonsecurity | Source baseline; profile/consumer and construct matrix |
| Financial/work oracle and formal adapters | GPT/nonsecurity | Independent of source author; freeze expectations before runs |
| Native, authority, custody and all security review | Opus | Own security artifacts, source observations, hypotheses and required controls |
| Installation, tooling and conformance inventory | GPT/nonsecurity | Public packages/docs plus complete retained family membership |
| Toolchain and evidence reconciliation | GPT/nonsecurity | Exact build/target identities and stage-bound evidence |
| Integration and roadmap reconciliation | Maintainer | Combine findings without voting missing behavior into existence |

Disjoint source inspection and oracle preparation can run in parallel. One integrator owns shared IR, ABI and result interfaces. Serialize builds and model executions under the existing lock and limits; avoid broad native test suites. No new campaign, native retry, contract, budget reset or execution loop is created by this plan.

Current native authority is zero. Static inspection, graph construction, ordinary reference work, and approved source-static public IR construction/serialization are distinct from executing the SDK. SDK relation/circuit checks, model/degree queries, VM/transcript execution, native key generation, proving, native verification and ledger actions remain blocked. Source-static compilation is not permission to invoke those operations. Opus additionally classifies signed `verify-intent --profile`, `simulate --trace --target` and the CLI replay generator as blocked where they invoke SDK checks or degree queries; pure Core reference APIs are a separate surface (see `SECURITY-REVIEW.md`, section 4). Use only inspected safe ordinary checks under their existing authority; this planning review ran no product tests. Actual Fenrir target identity/configuration and other unavailable external facts are separate blockers. Unknown target facts are not invented and do not block independent language work.

## Completion criteria

The planning phase is complete when all requested proposals and their dissent are preserved, outputs and work boundaries are explicit, each retained family has a reconciliation owner, security is Opus-owned, and the roadmap reports observed child capabilities without weakened exits.

The full end-to-end audit is complete only after every required predicate/profile/target cell has a disposition, both positive programs have traversed every claimed stage with independent complete outcomes, the grammar and field coverage arguments hold, every adopted security finding has Opus evidence/disposition, and all required current native/target/release results and independent reviews exist. Blocked or uninspected mandatory cells prevent a full completion claim. This dossier does not close U0–U7 or any M/MC/SP/G exit.
