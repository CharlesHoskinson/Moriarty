### Requirement: UIREQ-006 Exact scalar arithmetic
WHEN a scalar primitive executes, the interpreter SHALL produce its declared checked UInt128 or Bool outcome.

#### Scenario: Normal arithmetic
- **WHEN** add(2,3), sub(7,2), mul(3,4), and lt(2,3) execute.
- **THEN** Their results are 5, 5, 12, and true under the selected primitive table.

### Requirement: UIREQ-027 Indexed program-image provenance
WHEN preparation indexes a supported source program, it SHALL produce a ProgramImage preserving source membership, original function-table order and SourceOccurrenceId provenance.

#### Scenario: Distinct occurrences and recursive references
- **WHEN** Two source sites contain the same expression and a named function recursively calls itself.
- **THEN** They retain distinct SourceOccurrenceId values while recursive calls remain function-table references; expanded implementation-step IDs map back to those sites. Ordered child/parameter slices and first-match lookup remain as in the source until a separately admitted unique-function profile applies; preparation neither evaluates the selected path nor unrolls recursion.
