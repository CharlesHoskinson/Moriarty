# Native validation and receipts draft

## Why

Twenty financial families still lack their own native ledger evidence.
The preserved public atomic prototype does not establish financial custody or general source correspondence.
Implementation18 has source/UPLC-unit evidence only.
This package specifies the missing ledger execution and immutable receipts.

## What Changes

Add native execution, measured runtime pins, source time correspondence, full financial readbacks, and receipt requirements.
Define prospective runtime scripts and explicit failure controls.
Retain unsupported families as distinct refusals until their dependent compilers exist.

## Capabilities

### New Capabilities
- `eutxo-native-validation`: task-owned Cardano execution with source-specific receipts and complete financial comparison.

### Modified Capabilities
None.

## Impact

Prospective implementation paths: `TASK/tooling/t1-custody-runtime/` and `TASK/tooling/native-validation/`.
Prospective evidence paths: `TASK/native-validation-runs/<runId>/` and approved revision02 receipt namespaces.
Compiler, ledger, finance, and conformance packages supply dependent predicates.
This draft changes no production code, nodes, wallets, keys, or chain state.
Structural validation does not approve native execution or semantic correctness.
Private, PCD, Complete, and Midnight Preview certification remain outside this native package.
