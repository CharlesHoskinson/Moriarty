# Sherlock universal interpreter design

Status: **Final design Candidate B / proposed, specified-only**, 2026-10-15. This proposal integrates completed research for independent review. Adoption, implementation and release acceptance remain separate.

## Problem and result

Moriarty needs one execution relation that treats different supported programs as input, with inspectable recursion, recursive immutable data, return contexts, finite execution and eventual financial suspension. Sherlock is the proposed strict left-to-right first-order reference machine. Named self/mutual calls use explicit environments and frames; values use exact checked UInt128/Bool and typed constructors/matching. Source-visible pointer identity and cyclic values are outside the proposed fragment.

Candidate B turns the research into shared [engineering interfaces](interfaces.md): indexed ProgramImage and source occurrence identities; bounded MachineImage and representation relation; explicit helper microsteps under a new named profile; named carry/borrow/full-product APIs; separate witness and constraint plans with backend capability holes; deterministic uncached witness DAGs; ordered prefix-preserving slices; finite static residualization; and declaration-driven ordered ADT matching. See [architecture](design.md), [35 requirements/scenarios](specs/universal-interpreter/spec.md), [36 normal contracts](demonstrators.md) and the [implementation plan](../../../docs/SHERLOCK-IMPLEMENTATION-PLAN.md).

The proposed helper profile is not the frozen Candidate A K or Quint schedule. Existing source and observations remain unchanged. The K frontend check and Quint eight tests/32 bounded sampled traces retain their exact scope; they establish no B correspondence or circuit behavior. [Model coverage](model-coverage.md) records the missing mappings.

## Required Clean stage and integration

The development path is K reference semantics → Quint bounded design validation → Lean semantic/representation definitions and Clean bounded-step components/correspondence → compiler/native integration. Clean is required by UIREQ-026. A generic pure Lean foundation and normal examples have already merged into Fenrir's existing private fork after separate review. That foundation does not implement B helpers, bounded circuit encoding, Clean transition constraints or a Midnight exporter. [clean-stage.md](clean-stage.md) separates these obligations and the actual public API pin.

The language design remains Moriarty-owned; integration source belongs in the existing Fenrir Clean fork. Private source, paths and pins remain private. The route reuses the user's demonstrated Midnight ledger-9 custom-decider recursion capability. VerifyProof, ledger10 and a foreign proof-system port are not prerequisites. No fresh native reproduction is asserted.

## Financial behavior and retained scope

Core/2 typed-await successful-prefix behavior constrains the financial adapter. Complete ordered effects, fees, gross debit, residual duties and recovery survive cuts and optimizations. Exact universal Commit/Yield/Await opcodes and target-phase mappings remain open. A technical chunk is not a financial commit. Stable source profiles and public permissionless compile/prove/deploy behavior remain unchanged.

The proposal adds engineering work within U0–U7, M1–M17 and existing MC/SP/G/ZR/MNR obligations. It closes none of those full parent exits, creates no milestone system or resource authority, and preserves all ACTUS/DeFi and composition denominators.

## Research, choices and review

Ten first-wave studies, five Opus brainstorms, ten optimization studies and ten fresh online academic/project studies are inputs, not approval votes. The [concept adoption matrix](../../../docs/research/sherlock/CONCEPT-ADOPTION.md) maps concepts to independent rebuilds and required evidence. No external code import is selected. Licenses and backend differences do not disqualify independently rebuilt concepts.

Universal, per-program and fixed-family realizations remain unmeasured alternatives sharing one reference interface. Exact helper/allocation tables, bounds, admission, layouts, formal mappings and financial opcodes remain explicit work. Opus exclusively owns the [original security requirements](security-requirements.md) and [Candidate B security delta](security-candidate-b.md), including disposition of their recorded profile conflicts.

Superseded 2026-10-24: adoption and acceptance follow AGENTS.md (factory predicates + holdouts); no vote or reviewer approval is required. Passing document checks, merged foundation source and model samples do not replace implementation or target evidence.
