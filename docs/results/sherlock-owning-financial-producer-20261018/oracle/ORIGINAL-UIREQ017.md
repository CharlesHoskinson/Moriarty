WHEN the future financial extension reports a segment outcome, it SHALL expose the complete ordered financial observation defined by its adapter.

#### Scenario: First paid prefix
- **WHEN** Reserve 100/liability 40 pays 30, charges fee 2 and discharges liability 30 at an admitted prefix boundary.
- **THEN** The observation reports reserve 68, liability 10, gross debit 32 and recipient receipt 30, with residual duty 10.
