# Moriarty evidence-gated sprint program

<!-- markdownlint-disable MD013 MD060 -->

- Date: 2026-09-03 UTC
- Status: active
- Decision target: build a Moriarty vertical slice, ship audited Compact
  libraries, or stop

## Progress model

Moriarty does not count calendar time. A sprint completes only when its evidence
gate passes. A failed stop gate selects the recorded fallback. It does not add
time, features, or undocumented exceptions.

Each sprint begins with pinned inputs and an acceptance predicate. Each sprint
ends with preserved outputs, deterministic verification, claim reconciliation,
and a journal entry. Partial work remains partial regardless of elapsed time.

## Sprint sequence

| Sprint | Package | Evidence output | Exit gate |
|---|---|---|---|
| S00 | Compact DSL feasibility | finite Core, generated Compact, ZKIR, manifest, certificate, differential traces | no unbounded path, no undeclared disclosure, zero divergence |
| S01 | Evidence and taxonomy | complete metrics, 689 residue rows, genuine-rater protocol | calculations reproduce and simulated ratings remain excluded |
| S02 | Marlowe semantic delta | construct and proof-premise matrix | every inheritance claim has exact evidence or new proof work |
| S03 | Core semantic scope | versioned motion ledger and Core grammar | one disposition per motion and no unresolved Core ambiguity |
| S04 | Normative assurance | executable strategy comparison and theorem inventory | choose one maintainable specification authority or library-only |
| S05 | High-risk composition | conditional tokens with exchange and adversarial paths | conservation, authorization, replay, and deterministic-build gates pass |
| S06 | Canonical applications | F1-F6 and prediction source-to-backend evidence | seven bounded slices compile without full-protocol overclaim |
| S07 | Protocol coverage | one reviewed disposition for each of 72 rows | 72 of 72 rows have bounded or outside-kernel results |
| S08 | Complete development SDK | full SDK architecture, contracts, schemas, trust boundaries, and conformance plan | every SDK component has a specification and local verification duty |
| S09 | Real proof and cost | proving keys, proofs, verification, resource distributions, ledger costs | measured budgets pass or the project selects a fallback |
| S10 | Independent evaluation | security audit, user comparison, and two pilot specifications | no open critical or high finding and demand evidence exists |
| S11 | Terminal council decision | blinded critique, dissent, scorecards, and decision record | choose language, libraries, or stop without an exploratory outcome |

## Semantic scope ledger

Each sprint records the active semantic version and scope. The ledger contains:

- Core constructors and types.
- Surface-only constructs.
- Backend-only operations.
- External capabilities and trust assumptions.
- Accepted, revised, deferred, outside-core, and rejected motions.
- New proof obligations.
- New serialization or migration obligations.
- The evidence that changed each disposition.

S00 freezes only the experimental E00 Core: `Close`, `Pay`, `If`, `When`,
`Deposit`, bounded `Choice`, typed parties, typed tokens, positive internal
accounts, explicit timeout, and canonical atomic swap. It does not freeze
Attest, action sets, bounded mandates, conditional-token split or merge,
Merkleized continuations, external calls, minting, packages, or the surface
language.

S03 owns the first candidate Core freeze. Later sprints can upgrade the scope
only through a new semantic-motion record. The record must state compatibility,
proof, backend, privacy, resource, security, and demand effects.

## Compact DSL feasibility evidence

The current evidence establishes a narrow possibility result. A Python
programmatic Core builds a finite atomic swap. Its restricted lowerer emits
readable Compact. The pinned compiler emits four ZKIR 3.0 circuits and generated
TypeScript. The mock ZKIR compiler accepts all four circuits. An independent
transition machine matches the Core on 1,000 unique traces. The compiler rejects
an intentionally undeclared disclosure.

This result shows that one bounded Marlowe-shaped financial DSL slice can target
Compact. It does not establish a general source language, a complete compiler,
a real proof, ledger execution, economic cost, universal interpretation, or
production safety. S00 remains open until a fresh pinned environment reproduces
the complete artifact chain.

## Complete SDK boundary

S08 specifies the entire development system, not only a TypeScript wrapper. It
must cover:

1. Canonical textual syntax, parser, formatter, and linter.
2. Type checker, visibility checker, capability checker, and bound checker.
3. Surface elaborator, Core normalizer, canonical serializer, and source maps.
4. Compact generator, manifest generator, certificate generator, and backend validator.
5. Reference interpreter, simulator, debugger, trace viewer, and static analyzer.
6. Property runner, differential tester, fuzz interfaces, and conformance vectors.
7. Package resolver, lockfile, registry protocol, signing, and reproducible builds.
8. CLI, language server, editor protocol, and machine-readable diagnostics.
9. Transaction planner, coin selection boundary, cost estimator, and intent model.
10. Artifact, state, intent, disclosure, capability, and transaction verifiers.
11. Wallet, custody, hardware signer, and partial-transaction adapters.
12. Continuation, oracle, identity, validator, and script-registry interfaces.
13. Chain, indexer, explorer, event, rollback, payout, and provenance clients.
14. SDK versioning, compatibility, telemetry boundaries, SBOM, and release conformance.

Every component needs inputs, outputs, failure types, trust assumptions, version
rules, and local verification duties. Generated clients cannot silently trust a
Runtime, compiler, registry, oracle, wallet, or LLM.

## Armed loop

1. Read the goal, wiki index, open questions, contradictions, graph, and journal.
2. Select one decision-changing evidence gap.
3. Record its acceptance predicate in the journal.
4. Pin every source, repository, toolchain, and test input.
5. Use Scrapling for all web search and acquisition.
6. Add a failing test before behavioral code changes.
7. Preserve commands, raw outputs, versions, digests, and limitations.
8. Update the semantic scope ledger and smallest relevant wiki pages.
9. Run focused checks and the applicable full suite.
10. Apply the current sprint gate and record the fallback on failure.
11. Request independent review only at a commitment boundary.
12. Checkpoint the exact state before selecting the next gap.

The loop does not authorize deployed transactions, external messages,
unbounded network acquisition, or silent Core expansion.
