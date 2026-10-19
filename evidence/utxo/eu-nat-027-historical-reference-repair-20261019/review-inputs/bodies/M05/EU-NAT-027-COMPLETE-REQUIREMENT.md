### Requirement: EU-NAT-027 Immutable reproduction and stop

IF two cycles repeat the same defect class, THEN the runner SHALL stop retries and retain one minimal reproducer.

#### Scenario: EU-NAT-027 positive
- **WHEN** a changed hypothesis and reviewed bounded repair resolves the defect
- **THEN** the runner shall resume only the affected case under existing authority

#### Scenario: EU-NAT-027 negative
- **WHEN** the next retry only widens timeouts or relabels evidence
- **THEN** the runner shall refuse the retry and preserve failures

