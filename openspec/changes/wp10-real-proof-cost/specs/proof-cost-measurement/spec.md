# Real proof-and-cost specification

## ADDED Requirements

### Requirement: real proof path

The package SHALL generate real keys and proofs with supported proving
parameters. It SHALL verify each proof through an independent verifier path.

#### Scenario: only a mock compiler is available

- WHEN no real proof can run
- THEN the package remains blocked and reports no proof-feasibility result.

### Requirement: complete measurements

Measurements SHALL include constraints, key sizes, proof size, peak memory,
compile time, proving time, verification time, state size, transaction size,
fees, and provider calls.

#### Scenario: one resource is not observable

- WHEN the toolchain does not expose it
- THEN the report marks it unavailable and states the measurement limitation.

### Requirement: representative distributions

The package SHALL record cold and warm distributions across approved hardware,
wallet states, canonical applications, and adversarial controls.

#### Scenario: one outlier exceeds the budget

- WHEN the approved percentile gate fails
- THEN the application or architecture fails its cost gate.

### Requirement: comparable fallback

The same workload and measurement schema SHALL compare Moriarty with audited
Compact libraries.

#### Scenario: library templates match assurance at lower cost

- WHEN the predefined advantage threshold is not met
- THEN the terminal recommendation selects library-only.

### Requirement: provenance

Every sample SHALL bind artifact, toolchain, parameters, hardware, network,
protocol settings, command, timestamp, and raw output.
It SHALL also bind the prover endpoint and independent verifier identity.

#### Scenario: an artifact digest is missing

- WHEN a sample cannot identify its circuit
- THEN it is excluded from aggregate results.

### Requirement: frozen protocol and budgets

The decision authority SHALL approve the measurement protocol, budgets,
percentiles, sample counts, exclusions, and comparison rule before measurements.

#### Scenario: a threshold changes after results exist

- WHEN its approved digest changes
- THEN prior measurements cannot satisfy the new gate without a new sprint run.

### Requirement: authorized testnet only

Testnet submission SHALL require a signed record naming authority, network,
wallet, limits, and allowed artifacts. Mainnet submission is prohibited.

#### Scenario: no authorization record exists

- WHEN an experiment requests submission
- THEN the SDK stops before wallet signing or network access.
