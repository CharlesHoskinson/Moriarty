# Family-demonstration specification

## ADDED Requirements

### Requirement: seven bounded slices

The package SHALL provide one bounded canonical application for F1, F2, F3,
F4, F5, F6, and Prediction.

#### Scenario: a family depends on off-chain facts

- WHEN the slice requires custody, legal, oracle, or validator facts
- THEN its manifest names those assumptions outside the Core.

### Requirement: complete artifact chain

Each slice SHALL produce source, canonical Core, Compact, ZKIR, bounds,
manifest, certificate, and client-verification evidence.

#### Scenario: one backend artifact lacks provenance

- WHEN its source or toolchain digest is missing
- THEN the slice fails the artifact-chain gate.

### Requirement: reusable language evidence

At least two non-swap slices SHALL pass through shared surface, elaboration,
Core, and backend interfaces.

#### Scenario: every slice needs a custom lowerer

- WHEN no shared language path exists
- THEN the result supports template libraries, not a general Moriarty language.

### Requirement: bounded claim language

Each report SHALL distinguish a canonical slice from a complete protocol. It
SHALL list every unsupported behavior.

#### Scenario: a slice omits liquidation auctions

- WHEN the source protocol uses liquidation auctions
- THEN the report names the omission and its capability boundary.

### Requirement: per-slice stop gate

Any invariant, authorization, boundedness, disclosure, or artifact-integrity
failure SHALL fail that slice.

#### Scenario: six slices pass and one fails

- WHEN one slice fails
- THEN the package does not claim seven-family feasibility.
