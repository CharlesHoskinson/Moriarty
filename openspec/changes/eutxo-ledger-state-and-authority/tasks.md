# Ledger state and authority tasks

Status: DRAFT. Every task is prospective and unchecked.
Paths below are proposed implementation targets, not existing interfaces.
Commands below are reviewable future checks, not commands executed for this draft.
Run commands from the worktree root after the specified target exists.
Use the exact admitted source tuple and independent full oracle envelopes.
Keep the immutable revision03 and oracle19 files unchanged.
Grok4.7 remains the active implementation provider.

## Requirement-sized local tasks

The proposed public function is `checkLedgerRelation({semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor})`.
Its result is `RelationDecision/1`, as defined in `design.md`.
The proposed pin helper is `deriveRuntimePins({sourceFreeze, genesisCommitment, cashArtifact, roleHashes, spenderArtifact, policyTemplate})`.
The proposed final-body function is `checkFinalBodyWalletPolicy({bodyBytes, resolvedCollateral, signedExposureLimits})`.
The test adapter imports these prospective exports from the targets listed below.

### LDG-T001: EU-LDG-001

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/001.json` with the positive and negative scenarios from EU-LDG-001.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/001.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: compiler admitted Core and oracle19 complete envelopes.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/001.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T002: EU-LDG-002

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/002.json` with the positive and negative scenarios from EU-LDG-002.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/002.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer01 S01 and constrained mint-policy UPLC/native readback.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/002.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T003: EU-LDG-003

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/003.json` with the positive and negative scenarios from EU-LDG-003.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/003.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer01 S01 identity mutations and actual mint/input/output readback.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/003.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T004: EU-LDG-004

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/004.json` with the positive and negative scenarios from EU-LDG-004.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/004.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: actual policy hashing, mint-policy templates and stage00 genesis limitation.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/004.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T005: EU-LDG-005

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/005.json` with the positive and negative scenarios from EU-LDG-005.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/005.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: stage00 ASSESSMENT and revised T1 scoped reviews.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/005.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T006: EU-LDG-006

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/006.json` with the positive and negative scenarios from EU-LDG-006.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/006.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/runtime-pins.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: deriveRuntimePins -> RuntimePins/1 or refusal.
Evidence input: design proposed generation protocol and runtime compile/hash parity.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/006.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T007: EU-LDG-007

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/007.json` with the positive and negative scenarios from EU-LDG-007.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/007.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: revision03 script-own distinction and proposed ScriptContext projection.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/007.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T008: EU-LDG-008

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/008.json` with the positive and negative scenarios from EU-LDG-008.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/008.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer01 head-step source plus UPLC/native exact outref readbacks.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/008.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T009: EU-LDG-009

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/009.json` with the positive and negative scenarios from EU-LDG-009.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/009.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer01 replay observations and ledger resolved-input envelope.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/009.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T010: EU-LDG-010

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/010.json` with the positive and negative scenarios from EU-LDG-010.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/010.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: oracle19 varied balances/allowances/IDs/revision/work and designer01 complete oracle.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/010.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T011: EU-LDG-011

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/011.json` with the positive and negative scenarios from EU-LDG-011.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/011.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer05 well-formed marker substitutions and exact commitments.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/011.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T012: EU-LDG-012

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/012.json` with the positive and negative scenarios from EU-LDG-012.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/012.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: CIP0031 Reference inputs and actual reference/consumed input projections.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/012.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T013: EU-LDG-013

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/013.json` with the positive and negative scenarios from EU-LDG-013.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/013.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: revision03 correspondence and oracle19 E13/E11/cancel native template.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/013.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T014: EU-LDG-014

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/014.json` with the positive and negative scenarios from EU-LDG-014.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/014.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer03 cases and proposed output-index allocation table.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/014.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T015: EU-LDG-015

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/015.json` with the positive and negative scenarios from EU-LDG-015.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/015.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: native input/output/effect joint readback and compiler composition boundary.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/015.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T016: EU-LDG-016

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/016.json` with the positive and negative scenarios from EU-LDG-016.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/016.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer05 AP02 and proposed AuthorityState with lifetime bound.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/016.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T017: EU-LDG-017

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/017.json` with the positive and negative scenarios from EU-LDG-017.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/017.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: actual concurrent pair receipts and state readbacks distinct from sequential replay.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/017.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T018: EU-LDG-018

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/018.json` with the positive and negative scenarios from EU-LDG-018.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/018.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer05 scope limitation and bounded registry size policy.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/018.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T019: EU-LDG-019

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/019.json` with the positive and negative scenarios from EU-LDG-019.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/019.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: cardano-ledger Conway TxInfo and native signatory/witness negative.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/019.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T020: EU-LDG-020

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/020.json` with the positive and negative scenarios from EU-LDG-020.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/020.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer05 AP03 and designer02 joint-role native controls.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/020.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T021: EU-LDG-021

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/021.json` with the positive and negative scenarios from EU-LDG-021.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/021.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/wallet-body-policy.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkFinalBodyWalletPolicy -> RelationDecision/1.
Evidence input: pinned V3 Contexts and whole-body wallet policy measurements.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/021.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T022: EU-LDG-022

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/022.json` with the positive and negative scenarios from EU-LDG-022.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/022.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer01 S03 partition oracle and proposed LifecycleDescriptor.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/022.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T023: EU-LDG-023

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/023.json` with the positive and negative scenarios from EU-LDG-023.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/023.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer01 S03 join oracle plus bounded fan-in coverage.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/023.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T024: EU-LDG-024

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/024.json` with the positive and negative scenarios from EU-LDG-024.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/024.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer01 S04 oracle and proposed migration descriptor.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/024.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T025: EU-LDG-025

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/025.json` with the positive and negative scenarios from EU-LDG-025.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/025.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: designer01 S04 termination and E12 outstanding40 nontermination.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/025.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T026: EU-LDG-026

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/026.json` with the positive and negative scenarios from EU-LDG-026.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/026.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: unsupported-split source and SOURCE_ACTION_UNKNOWN controls.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/026.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T027: EU-LDG-027

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/027.json` with the positive and negative scenarios from EU-LDG-027.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/027.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: oracle19 templates and stage00 source/node/readback distinctions.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/027.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T028: EU-LDG-028

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/028.json` with the positive and negative scenarios from EU-LDG-028.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/028.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: existing pcd-ledger-anchored spec and designer05 failclosed certificate controls.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/028.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

### LDG-T029: EU-LDG-029

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/029.json` with the positive and negative scenarios from EU-LDG-029.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/029.test.mjs` before changing relation behavior.
- [ ] Implement the required relation in `experiments/moriarty-eutxo/src/ledger-state.mjs`.
- [ ] Connect the relation through the admitted compiler output and measured runtime context.

Interface: checkLedgerRelation -> RelationDecision/1.
Evidence input: oracle19 templates and stage00 InvalidWitnessesUTXOW versus PlutusFailure versus BadInputsUTxO.
Fixture fields: exact semanticEnvelope, head, context, allocation, pins, lifecycleDescriptor, and expected decisions.
Positive expectation: the complete positive projection equals the independent scenario oracle.
Negative expectation: the scenario refusal occurs without published financial state or executable native artifacts.

Prospective verification:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/029.test.mjs
```

Expected: exit0, positive accepted, negative refused, full fields compared, no synthetic native approval.

## Concrete proposed test shape

The following snippet is a prospective test, not an existing API call.
Each requirement fixture supplies independently constructed expected fields and refusal codes.

```js
import { checkLedgerRelation } from '../../src/ledger-state.mjs';
import { strict as assert } from 'node:assert';
const accepted = checkLedgerRelation(fixture.positive.input);
assert.equal(accepted.accepted, true);
assert.deepEqual(accepted.projection, fixture.positive.expectedProjection);
const refused = checkLedgerRelation(fixture.negative.input);
assert.equal(refused.accepted, false);
assert.equal(refused.code, fixture.negative.expectedCode);
assert.equal(refused.publishedFinancialState, false);
```

Native projections use independently decoded actual outputs, not `accepted.projection` as their expected oracle.

## Cross-package integration and measurements

### LDG-I01: Exact compiler envelope integration

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/compiler-envelope.test.mjs`.
- [ ] Compare all oracle19 accepted envelopes and original rejected envelopes without renaming source codes.
- [ ] Integrate `AdmittedSemanticEnvelope/1` at the compiler peer's agreed public export.

Prospective command:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/compiler-envelope.test.mjs
```

Expected: all46 cases and both supplemental action-tag probes match complete fields.
E12 remains source-prepared and admission-refused NONEMPTY_OBLIGATIONS.
The /6 profile remains refused at the /5 boundary.
No fixed E13 post substitutes for varied balances, allowances, IDs, revisions, or work.

### LDG-I02: PolicyHashWitness bounded feasibility

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/policy-hash-parity.test.mjs`.
- [ ] Add `experiments/moriarty-eutxo/eutxo-ledger/lib/moriarty/policy_hash.ak`.
- [ ] Measure exact template application bytes and actual Cardano hash-domain parity.
- [ ] Measure the reconstruction relation under the pinned PlutusV3 cost model and committed byte-size limits.

Proposed interface: `reconstructPolicyHash(template, ownCredential, seedOutRef, genesisCommitment, witness)`.
Expected positive: on-chain reconstruction equals independent CLI and Lucid actual policy identity.
Expected negative: changed parameter offset, noncanonical encoding, foreign script credential, or arbitrary policy bytes refuse.

Prospective commands:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/policy-hash-parity.test.mjs
/home/charl/.aiken/bin/aiken check experiments/moriarty-eutxo/eutxo-ledger
```

Expected: exit0 with named positive and negative tests, plus retained memory/CPU measurements within the committed limits.
A failed feasibility measurement leaves the implementation gap open.
It does not authorize unchecked policy IDs or an unreviewed registry fallback.

### LDG-I03: Full constrained mint and spend UPLC contexts

- [ ] Add `experiments/moriarty-eutxo/eutxo-ledger/validators/genesis.ak`.
- [ ] Add `experiments/moriarty-eutxo/eutxo-ledger/validators/head.ak`.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/uplc-contexts.test.mjs`.
- [ ] Generate exact positive and hostile consumed/reference/mint/signer/value context fixtures.

Prospective commands:

```sh
/home/charl/.aiken/bin/aiken check experiments/moriarty-eutxo/eutxo-ledger
/home/charl/.aiken/bin/aiken build --uplc experiments/moriarty-eutxo/eutxo-ledger
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/uplc-contexts.test.mjs
```

Expected: both constrained mint and spend execute their full positive predicates under the pinned cost model.
Expected: every hostile context fails its intended predicate, with the full expected state unchanged.
The generated output remains separate from `eutxo-public` and the preserved T1 candidate.

### LDG-I04: Native goldens and inclusion readback

- [ ] Add `tooling/eutxo-ledger-runtime/native-ledger-families.mjs` under a fresh task-owned runtime directory.
- [ ] Integrate the runtime peer's receipt and resolved-context interfaces.
- [ ] Record signed CBOR, independent TxId, inclusion, and complete input/output/effect readbacks.

The exact prospective full driver path is `/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/eutxo-ledger-runtime/native-ledger-families.mjs`.
The driver contract accepts `--family`, `--pins`, `--oracle`, and `--receipt-dir`.
It refuses an existing receipt namespace and any nativePins=false manifest.
Driver creation and execution belong to future authorized implementation/runtime work.
No node or wallet command is authorized by this draft task.

Prospective command template:

```sh
node /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/eutxo-ledger-runtime/native-ledger-families.mjs --family S01 --pins RuntimePins.json --oracle LedgerGoldens.json --receipt-dir receipts/new-S01
```

Expected S01: constrained seed consumption, exact quantity1, correct full datum and address, and counterfeit mint refusal.
Repeat separately for S02, S03, S04, AP02, and AP03 after their implementation predicates exist.
Expected readback: full native fields equal their independent goldens, with source/adapter/node codes recorded separately.
A native row stays unverified until all required measurements exist.

### LDG-I05: Concurrent head and authority races

- [ ] Add `tooling/eutxo-ledger-runtime/native-ledger-races.mjs` under the same fresh runtime directory.
- [ ] Build separate valid fill/fill and cancel/fill bodies sharing only the declared head or authority input.
- [ ] Allocate independent fee and collateral inputs for each body.
- [ ] Submit both without awaiting first confirmation in the authorized future runtime phase.

Prospective command:

```sh
node /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/eutxo-ledger-runtime/native-ledger-races.mjs --pins RuntimePins.json --oracle LedgerGoldens.json --receipt-dir receipts/new-races
```

Expected: at most one canonical outcome, determinate readback, preserved consumed nonce, and explicit loser class.
Sequential replay is a separate BadInputsUTxO control.
Indeterminate observation remains partial.

### LDG-I06: Private and terminal conformance handoff

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/proof-boundary.test.mjs`.
- [ ] Integrate the conformance peer's per-family evidence gate without changing historical approvals.
- [ ] Compare complete relation commitments with the private peer's proposed verifier inputs.

Prospective command:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/proof-boundary.test.mjs
```

Expected: required private/PCD modes fail closed until the actual complete verifier exists and is measured.
Public native acceptance does not close private, PCD, Complete, or unrelated inventory families.

## Draft schema check

The installed schema check is the only executable validation authorized for this package.

```sh
openspec validate eutxo-ledger-state-and-authority --strict --json --no-interactive
```

Expected: package valid true. Schema validity is not semantic approval or ledger acceptance.

### LDG-T030: Source lifecycle invariants

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/030.json` with complete positive and negative contexts.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/030.test.mjs` before changing behavior.
- [ ] Implement EU-LDG-030 in `experiments/moriarty-eutxo/src/ledger-state.mjs`.

Interface: `checkLedgerRelation -> RelationDecision/1`.
Evidence input: existing ledger-anchored specification and independent complete source/native projections.

Prospective command:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/030.test.mjs
```

Expected: exit0, exact positive claim boundaries, missing principal/import/certificate negative refusal, and no private/native overclaim.

### LDG-T031: Mandatory claim loci

- [ ] Add `experiments/moriarty-eutxo/tests/ledger/fixtures/031.json` with complete positive and negative contexts.
- [ ] Add `experiments/moriarty-eutxo/tests/ledger/031.test.mjs` before changing behavior.
- [ ] Implement EU-LDG-031 in `experiments/moriarty-eutxo/src/ledger-state.mjs`.

Interface: `checkLedgerRelation -> RelationDecision/1`.
Evidence input: existing ledger-anchored specification and independent complete source/native projections.

Prospective command:

```sh
node --experimental-strip-types --test experiments/moriarty-eutxo/tests/ledger/031.test.mjs
```

Expected: exit0, exact positive claim boundaries, missing principal/import/certificate negative refusal, and no private/native overclaim.
