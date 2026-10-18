### Requirement: UIREQ-021 Declared model projection
WHEN a K trace is compared with Quint, the adapter SHALL identify its common domain and rule-to-action projection.

#### Scenario: Finite call-return comparison
- **WHEN** A pure arithmetic/list trace is compared with a bounded Quint instance.
- **THEN** The report names numeric bounds, field maps, hidden steps, state representation differences and unmatched finance/wait behavior; trace agreement is not called refinement.

