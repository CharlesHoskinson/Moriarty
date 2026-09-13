## ADDED Requirements

### Requirement: Versioned typed financial post-state access
The implementation SHALL introduce agreement source /4 and expression Core /3. Existing profiles SHALL retain their exact behavior. The six post_outstanding, post_principal, post_accrued, post_balance, post_allowance_remaining and post_allowance_spent intrinsics SHALL be legal only within ensures expressions. Each SHALL take one declared unit or asset and one dynamic Text identity. Result types SHALL match their corresponding existing financial PRE read. Existing unprefixed reads SHALL keep immutable PRE meaning throughout the action.

#### Scenario: Actual debt result
- **WHEN** an action repays 30 against outstanding debt 100 and ensures requires post_outstanding<Cash>("Due100") equal to quantity 70
- **THEN** the action SHALL prepare successfully with actual outstanding debt 70, not a caller-supplied result.

#### Scenario: Invalid post-read scope
- **WHEN** a post read appears in requires, let, next or emit, including a dead branch
- **THEN** checking SHALL reject with the original source location before execution.

### Requirement: One staged action and atomic publication
Evaluation SHALL type-check every action, select one action, admit owned financial input, validate snapshots, execute the prefix once, prepare the existing kernel once, and execute the entire ensures suffix once. The suffix SHALL see preserved locals, staged ordinary post-state, immutable financial PRE and tentative financial POST. No public API SHALL accept a post-state, callback or resumable execution token from the caller. Publication SHALL include ordinary post, financial post and effects only after all checks succeed.

#### Scenario: Financial postcondition fails
- **WHEN** the kernel successfully prepares repayment30 but an ensure requires outstanding69
- **THEN** evaluation SHALL reject and expose no post-state, financial post, effects or reusable continuation.

#### Scenario: Complete validation and identity ownership
- **WHEN** an unselected action is ill-typed, the projection contains an invalid unrelated entry, or a caller injects financial POST through snapshots
- **THEN** evaluation SHALL reject before publishing any result.

### Requirement: Exact staged work and error precedence
Successful work SHALL equal executed prefix reductions plus executed kernel actions plus executed suffix reductions. Short-circuited nodes SHALL consume no reductions. Each entered expression constructor SHALL retain its existing unit cost. Closure reserve SHALL remain separate. Postcondition rejection after successful kernel preparation SHALL report total attempted work through the failing suffix node. Existing kernel rejection envelopes SHALL retain their defined action index. Prior spent work SHALL survive continuation without reset or refund.

#### Scenario: Exact work boundary
- **WHEN** the successful action receives spendable work equal to its exact total cost, with a separately retained closure reserve
- **THEN** it SHALL succeed; one fewer spendable unit SHALL reject atomically without consuming the reserve.

#### Scenario: Kernel failure precedes postconditions
- **WHEN** the body is valid but funding fails and an ensure would also fail
- **THEN** the kernel rejection SHALL occur and no ensure SHALL execute.

### Requirement: Public tooling and regression evidence
Source /4 SHALL expose primitive-string elaborate, check and four-argument evaluate APIs. Core /3 SHALL expose contextless checking and integrated funded action evaluation. Missing financial context SHALL reject execution. Standalone Core expressions SHALL not claim funded preparation. Normal check, format and simulate CLI paths and README commands SHALL execute the same implementation. Original /1–/3 source and /1–/2 Core fixtures SHALL retain complete results and diagnostics.

#### Scenario: Cross-entry-point result
- **WHEN** the same action runs through source API, Core API and CLI
- **THEN** complete financial observations, ordinary post, effects, work and rejection scope SHALL agree.
