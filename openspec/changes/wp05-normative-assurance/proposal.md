# Change: WP05 normative assurance

## Why

Moriarty needs one maintainable source of semantic authority. Parallel
formalizations without a correspondence policy would reproduce Marlowe's drift.

## What Changes

- Compare Isabelle, Agda, Lean, a reference interpreter, and differential semantics.
- Prototype the highest-risk theorem obligations in viable environments.
- Measure extraction, integration, contributor, and maintenance constraints.
- Select one normative strategy or the library-only fallback.

## Capabilities

### New Capabilities

- `normative-assurance`: Select and specify the authoritative semantics, proof,
  extraction, and conformance architecture.

### Modified Capabilities

None.

## Impact

This package governs semantic authority, proof artifacts, reference execution,
compiler validation, and release gates. It does not claim complete proofs.
