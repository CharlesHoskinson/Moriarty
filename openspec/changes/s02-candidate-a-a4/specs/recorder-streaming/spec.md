## ADDED Requirements

### Requirement: RH001 bounded emitted-artifact hashes

WHEN the recorder hashes an emitted native ITF, it SHALL use positive reads
bounded to 1 MiB and preserve the existing digest and receipt map schema.

#### Scenario: Whole-read regression is detected

- **WHEN** the original whole-file helper receives a guarded native-artifact path
- **THEN** the test fails behaviorally on the whole read
- **AND** after the streaming correction the exact digest matches known bytes
- **AND** the complete producer Python suite passes, including the bounded-read test
- **AND** a structural test requires main's published artifact map to call the helper

### Requirement: RH002 resource evidence remains a pre-pilot gate

WHEN the largest-case native pilot runs, its evidence SHALL retain a declared
wall budget, original resource output and the measured metric's exact scope.
Timeouts and missing terminal/resource evidence SHALL not count as successful
completion or semantic counterexamples.

#### Scenario: Resource runner is not yet validated

- **WHEN** only the outer resource-runner proposal has been reviewed
- **THEN** no measured pilot result or tested cleanup behavior is claimed
- **AND** its implementation and resource-content checks remain separately required
- **AND** GNU time maximum RSS is not presented as simultaneous Node-plus-Rust RSS

RH001 is adopted experimentally at `9cea581`; RH002 remains a separately
assigned pre-pilot implementation gate. Neither requirement changes Candidate A
authority rules or permits rewriting original receipts.
