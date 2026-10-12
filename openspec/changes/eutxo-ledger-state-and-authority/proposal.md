# Ledger state and authority draft

## Why

The 20-family native matrix lacks authenticated genesis, durable outcome authority, and lifecycle settlement evidence.
The stage00 public atomic prototype supplies real isolated Cardano receipts for its narrower predicate.
Those receipts do not approve financial source /5, source /6, private execution, PCD, or Complete.
This package defines measurable state and authority acceptance predicates for the missing implementation.
It can change the terminal evidence decision only after compiler, UPLC, native inclusion, and complete readbacks agree.

## What Changes

- Add EARS requirements EU-LDG-001 through EU-LDG-031.
- Define constrained genesis and authenticated consumed-head protocols.
- Define nonce, role, value-frame, exclusive-output, and lifecycle conservation interfaces.
- Propose an acyclic RuntimePins generation amendment.
- Specify positive, negative, and concurrent-race acceptance evidence.

These are new DRAFT amendments, not previously approved T1 bytes.
The reviewed T1 candidate and two votes remain unchanged.

## Capabilities

### New Capabilities

- `eutxo-ledger-state`: Authenticated genesis, continuations, authority, custody, lineage, migration, retirement, and evidence boundaries.

### Modified Capabilities

None. This draft does not modify an existing approved specification.

## Impact

Prospective source paths appear in `tasks.md`.
The compiler package supplies admitted Core and complete semantic envelopes.
The runtime package supplies measured contexts and independent native readbacks.
The financial package supplies obligation and economic invariants.
The conformance/private package owns complete proof verification and final compatibility claims.

Production implementation, commits, wallet actions, nodes, seeds, signing, submission, and external messaging are outside this draft scope.
The source /5 and /6 profiles retain their separate version tuples.
Existing finite financial economics needs no new source grammar.
Split, Join, and Migrate need versioned semantics only where source operations are absent.
