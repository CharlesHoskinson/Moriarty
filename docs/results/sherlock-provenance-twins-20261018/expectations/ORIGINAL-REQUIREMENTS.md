### Requirement: UIREQ-027 Indexed program-image provenance
WHEN preparation indexes a supported source program, it SHALL produce a ProgramImage preserving source membership, original function-table order and SourceOccurrenceId provenance.

#### Scenario: Distinct occurrences and recursive references
- **WHEN** Two source sites contain the same expression and a named function recursively calls itself.
- **THEN** They retain distinct SourceOccurrenceId values while recursive calls remain function-table references; expanded implementation-step IDs map back to those sites. Ordered child/parameter slices and first-match lookup remain as in the source until a separately admitted unique-function profile applies; preparation neither evaluates the selected path nor unrolls recursion.

### Requirement: UIREQ-020 Target phase mapping
WHEN the compiler maps a financial segment to target phases, it SHALL report the exact correspondence between source outcomes and guaranteed/fallible phase outcomes.

#### Scenario: Permitted retained fee
- **WHEN** The selected target phase policy permits a guaranteed fee to remain after a fallible action fails.
- **THEN** The adapter reports that policy-specific fee and remaining duties separately from local segment rollback; concrete target evidence remains pending.
