## ADDED Requirements

### Requirement: Authenticated financial state and actual language consumer
The ledger adapter SHALL bind the selected agreement/Core/profile, contract identity, network, accepted head/revision, authority and complete financial state to authenticated ledger observations. It SHALL execute the reviewed source-to-financial path through the actual transaction caller. Arbitrary JSON snapshots, host flags and hardcoded loan initialization SHALL not establish authenticated lifecycle execution. Witness material and credentials SHALL remain private.

#### Scenario: Stale or substituted state
- **WHEN** a caller changes debt, allowances, period history, program identity or head revision outside the authenticated state
- **THEN** the actual acceptance path SHALL reject without financial settlement.

### Requirement: Docker financial lifecycle before Preview
The existing local Midnight environment SHALL execute the supported lifecycle through the real caller before public submission. Evidence SHALL include transaction status, complete decoded states, asset balances, allowances, obligations, periods, fees and the raw command exit. Negative transactions SHALL include independent before/after state readback. Mock transport results SHALL remain labeled unit evidence.

#### Scenario: Local complete result
- **WHEN** deployment/initialization and all lifecycle actions finalize under Docker
- **THEN** independent readback SHALL match the complete oracle and the driver SHALL exit zero with owned resources contained.

### Requirement: Current guarded Preview admission
Every public action SHALL use the Moriarty plugin guarded run with current candidate-bound authority, reviewed resource limits, current accounting, funding evidence and remaining attempts. Historical exhausted attempts or missing accounting SHALL not be reset or fabricated. A bounded resource amendment SHALL cite actual predecessor evidence and the required substantive votes before dispatch. Public transaction IDs and observed statuses SHALL be reported through the plugin notification flow.

#### Scenario: Admission unavailable
- **WHEN** candidate inputs, accounting, authority, funding or attempts cannot be positively validated
- **THEN** no public financial child SHALL launch. Independent source, Docker or K work SHALL continue when eligible.

### Requirement: Independent public financial acceptance
Preview completion SHALL require finalized lifecycle transactions, complete independent financial readback, actual exit zero, fee accounting and current result audit. Existing MC04/MC05 mandatory correctness, intent, transition and history requirements SHALL remain open unless separately discharged. General certificates, private composition and full ACTUS/DeFi coverage SHALL not be inferred from this bounded loan.

#### Scenario: Partial or unverified public outcome
- **WHEN** a transaction is submitted but finality, financial effects, exit status or required verification is absent
- **THEN** the stage SHALL retain the observed partial status and SHALL not claim completion.
