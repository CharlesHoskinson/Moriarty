# Draft financial pattern validation

## Why

Twenty inventory families lack complete native financial correspondence. Source-only envelopes cannot establish authenticated custody, liability retention, or wallet outcomes. This proposal defines measurable finance predicates for compiler and ledger validation. It can change the terminal decision only through future exact-source compiled and ledger receipts.

This document is S2: written proposal or design. It does not approve implementation, tests, native spending, private execution, or release.

## What Changes

- Add complete EARS requirements for escrow, intervals, observations, roles, swaps, orders, batches, loans, fees, and outcome authority.
- Define independent full-envelope oracles and parameterized prestate controls.
- Define cross-package interfaces for lowering, context authentication, effect allocation, and monetary projection.
- Preserve unsupported admissions and all twenty family distinctions.
- Define future conformance files and unchecked implementation tasks.

## Capabilities

### New Capabilities

- `eutxo-financial-patterns`: complete financial semantics and tier-specific acceptance requirements.

### Modified Capabilities

None. Existing source /5, source /6, and atomic /1 semantics remain versioned inputs.

## Impact

Prospective artifacts: `experiments/moriarty-eutxo/fixtures/financial/`, `experiments/moriarty-eutxo/tests/financial-patterns.test.mjs`, and versioned backend finance modules. These paths describe future work. No files at those paths are created by this proposal.

Dependencies: `eutxo-compiler-correspondence`, `eutxo-ledger-state-and-authority`, and sibling native/conformance/private drafts.

Non-goals: changing grammar without a demonstrated deficiency, implementing source/backend/test code, starting nodes, generating keys, submitting transactions, replacing active Grok escrow work, or closing Midnight release gates.

## Evidence

Base: `84f64b30df53ff0b8ccaf475f57324d3d7c6e594`. Task input digests are in `INPUT-SHA256.json` under the finance draft evidence directory.

Revision03 candidate: `dcc62e21e9e351843bd04a3be537ebe260f8c1d0ec606cfcc5e15fd83ca7aff0`. Both actual design-review files vote `APPROVE_SCOPED_T1_DESIGN`. Their votes do not approve all finance or native execution.

Independent source packet: `9d71e736e3d3c159a83ff3bb21dbb3f5b2a9661dd90ed1e70414c2690b6552d6`. Its compiled/native controls are templates, not executions. Implementation18 was not inspected.
