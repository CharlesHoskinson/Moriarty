# Moriarty: permissionless language of provable intention

Status: target architecture, specified-only where not linked to implementation. Authority: user clarification, 2026-09-19.

Moriarty is a programming language for all DeFi developers on Midnight. Any developer may author, compile, prove, publish and deploy programs in the supported language without approval from Moriarty maintainers, an agent council, Foreman, a hosted solver or a program registry. Objective Midnight validity rules, resource costs and asset-owner authorization still apply.

This document supersedes conflicting product-scope and administrative-deployment requirements in earlier designs. Historical receipts remain evidence of their original scope. It does not declare missing proof or ledger capabilities implemented.

## Consolidated design and delivery

The [consolidated vision and design](MORIARTY-CONSOLIDATED-DESIGN.md), [single U0–U7 roadmap](../ROADMAP.md), and [next ZKIR/recursion requirements](MORIARTY-BACKEND-REQUIREMENTS.md) integrate the research agenda. P/C/K identifiers remain requirement lineage; they are not independent delivery queues. The user supplied a planning assumption on 2026-09-19 that comprehensive Midnight recursion arrives in about six months. Plan full native recursive compliance and private multi-parent composition accordingly; current interface limitations do not remove that scope. The timing is not a verified release claim.

## Product boundary

The product is the language, semantics, compiler, proof interfaces, verifier integration and developer tooling. ACTUS, loans, swaps, vaults and other DeFi applications test expressivity and conformance; they are libraries and examples, not an exhaustive list of programs developers may deploy. Supported constructs compose into new programs. New language primitives require sound semantics and implementation; they do not justify approval of individual programs built from existing constructs.

A managed multichain router, identity provider, adapter catalog or pricing service can be an optional application. None is required to use Moriarty. Application-specific access control remains expressible without becoming a language-wide deployment restriction.

## Execution target and certified basis

Compiled Moriarty contracts must target **ZKIRv3** and run on Midnight. A local TypeScript evaluator, K execution, generated Compact source or a non-Midnight proof is not the terminal artifact. Compact can be an intermediate route only when the resulting pinned ZKIRv3 artifact and actual Midnight acceptance are established. Pin compiler, ZKIR major/minor version, instruction surface, circuit/key identity and ledger/verifier deployment separately.

Use Midnight's native proof infrastructure for proof production, aggregation and ledger verification, according to the pinned target's supported interfaces. Midnight's proof implementation derives from Halo2 and uses PLONK with KZG commitments; see the [native stack](https://github.com/midnightntwrk/midnight-zk). Lean is not a Moriarty compiler, developer-tool, proof-production or deployment dependency. Juvix's Lean integration is comparative research only. The user confirmed this architectural boundary on 2026-09-19.

Moriarty's responsibility is to express authenticated intention, financial invariants, authority and residual duties in the constraints enforced by that native stack, and establish that compilation preserves their meaning. Native cryptographic proof verification establishes the encoded relation; the compiler and language must ensure that relation includes the intended conditions. This obligation does not prescribe Lean or a separate proving backend.

Restore the Simplicity-inspired jet workstream: define a small semantic basis and permit optimized implementations only with explicit equivalence/correspondence obligations. A jet must preserve values, rejection behavior, effects and the declared resource semantics under its preconditions. Host speed, logical work and circuit cost are different quantities. Do not infer certification from a hash, tests, a shared invariant or a reference expression alone.

PR #17 at `ebb662c716fef2638ec1b0f42805a8bce23c75dd` is an open conditional Agda development for an older 34-instruction/13-type surface. Its `statement-sound` needs both source producer obligations and substantive witness-side `WShape` conditions. Those conditions must be derived or enforced for every adversarial accepted witness; checking only the honest producer witness is insufficient. Concrete chip assumptions and correspondence to newer instructions and the deployed ledger remain open. See [PR #17](https://github.com/midnightntwrk/midnight-zkir/pull/17).

For ZKIRv3, witness generation correctness is insufficient: constrained execution must exclude invalid witnesses as well as admit source-valid executions. Each primitive certificate binds its specification, input/range preconditions, reference semantics, target constraint fragment, implementation/version, theorem evidence and cost model. Composition must prove call-site preconditions and framing. Hash recognition selects a certified meaning; it never approves a developer. Changing semantics or cost requires an explicit profile/version decision, not an invisible optimization.

The September 13 jet study remains historical analysis on the local architecture branch. Its proposed basis size and proof-status claims require fresh reconciliation with current source and ZKIRv3 surfaces. No certified backend is established by this design.

## What provable intention means

Developers specify contract invariants and transition rules. Users authorize canonical, machine-readable outcome and authority constraints. Candidate execution must satisfy both. Natural-language wishes require explicit formalization and an inspectable signing display; a proof cannot establish that a misunderstood specification matches an unexpressed wish.

A proof statement binds versioned program and semantics, property specification, signed intent, execution domain, predecessor state/history, observations, complete effects and bounds. The acceptance relation verifies contract correctness, intent refinement, transition validity and compliant history. Compilation and ledger correspondence remain separate obligations until established. Arbitrary prover-selected predicates or keys cannot replace the bound relation.

Signed intent includes exact asset identity or an explicit substitution predicate; recipients; gross debit, fee and liability bounds; minimum net outcomes; validity, replay and revocation rules; and allowed partial completion/recovery. Plans may optimize preferences only within these hard constraints. The proposer can be a developer, wallet, independent solver or synthesizer.

Complete effects include an authenticated domain/frame and per-asset accounting for transfers, fee recipients, custody/reserves and authorized mint/burn supply changes. Liabilities have a separate typed evolution: opening plus creation/accrual minus explicit discharge equals closing. Debt is not token supply.

Resource semantics distinguish consumed receipts, affine spending authority and persistent liabilities. Unused authority need not be exercised. Debt and pending duties cannot disappear through weakening or a local success result. Composition preserves constraints, cumulative costs and residual duties.

Local evaluator rejection can be atomic. Midnight ledger phase semantics must be modeled separately: a failed fallible phase can retain guaranteed-phase effects and fees. Bind the chosen phase layout and account for every permitted failure outcome. Signed intent must state per-phase authority/nonce consumption, fees and remedies. Each failure outcome is an explicit transition retaining residual duties; the lowerer cannot invent a charged failure policy. External asynchronous effects require explicit pending, partial, unknown, settled and recovered states. Timeout is not evidence of nonexecution. Safety proofs do not supply liquidity, inclusion, oracle honesty, bridge security or unconditional liveness. External observations and attestations remain named assumptions; they cannot substitute for mandatory program/transition proofs.

Private composition states which data is hidden, what the public statement reveals and how predecessor proofs and authority compose. Confidentiality, noninterference and history correctness require explicit arguments; use of a zero-knowledge backend alone does not establish all three.

## Verification versus project process

| Condition | Product meaning |
|---|---|
| Program typing, bounds and supported semantics | Objective program validity |
| Signature, authority, nonce and current revocation | Owner authorization |
| Bound proof relation, verifier and state/history | Objective transaction acceptance |
| Native fees, finality and ledger validity | Midnight execution conditions |
| Reviewer identity, campaign budget, RP03, Foreman/Pel receipt | Internal project work only; never a public toolchain prerequisite |
| Optional relay/adapter service policy | That service only; no universal deployment authority |

Per-contract policy commits to exact claim and verifier semantics. Protocol upgrades and application governance must have explicit scope. Neither is a license for project reviewers to admit individual Mori programs.

## Aeon lessons and ordering

Borrow refinement typing, explicit definedness/termination assumptions, trust reports, counterexample explanations and typed holes. Implement these against Moriarty semantics rather than importing Aeon runtime, native escape hatches or solver results as ledger proofs.

First expose proof obligations and missing evidence. Develop the general supported-program proof path as the main delivery priority. An exact bounded static checker can improve authoring in parallel when capacity permits. Counterexamples require evaluator replay. Distinguish formula discharge, semantic correspondence, finite tests and actual ledger verification. Unknown, timeout, unsupported and inconsistent assumptions cannot become success.

Synthesis follows useful validated obligations. Generated code remains a proposal checked by the same public pipeline; hard constraints and signed intent are immutable during search. General recursion, unchecked native code and synthesis fitness cannot bypass finite execution or proofs.

## Demonstrable completion

A clean external developer installation, without project metadata or reviewer records, must author a new supported program outside the fixed loan/swap fixtures, inspect its claims, produce real proofs and submit it through the developer's own Midnight environment. Valid effects must match the signed intent and semantics. Tampering with program, intent, predecessor, proof, fees or effects without valid corresponding bindings must reject. Independently valid alternative plans within the same signed constraints remain admissible. Two proposal sources must receive the same objective verification treatment.

This end-to-end capability is unfinished. Current local source profiles, finite K comparisons and scoped Preview financial demonstrations are foundations, not completion of this contract.

## Partial transactions and conditional settlement

Moriarty must support staged transactions and conditional delivery, including submission to a destination that waits for a specified combination of signatures, documents, proofs, recipient acceptance or other supported predicates. The condition policy binds the request before authorization. Recorded submission or reserved funding is distinct from final delivery. The proposed main term is conditional settlement; contingent settlement remains an alias, and programmable escrow names the funded variant.

Partial fills, persistent continuations, joins, late results, cancellation and compensation retain complete effects, per-stage authority, fees and residual obligations. Documents and external evidence carry explicit identity, predicate, trust and freshness assumptions. The compiler must map these behaviors to Midnight ZKIRv3 and the actual ledger phases; it cannot assume NEAR semantics or global rollback. See the [requirements](../openspec/changes/partial-and-conditional-transactions/proposal.md) and [MPLR theory log](../wiki/research/mplr/index.md).
