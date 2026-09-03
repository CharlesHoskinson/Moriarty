# Protocol-coverage specification

## ADDED Requirements

### Requirement: exact roster coverage

The dataset SHALL contain exactly one reviewed row for each of the 72 pinned
protocol identifiers.
The input roster SHALL match the SHA-256 pinned in `design.md`.

#### Scenario: a name occurs in two product categories

- WHEN one organization supplies distinct products
- THEN stable product identifiers prevent accidental row collapse.

### Requirement: closed dispositions

Each row SHALL use `bounded-instance`, `partial-kernel`, `outside-kernel`,
`library-preferred`, or `unsupported`.

#### Scenario: only a conserved-value submachine fits

- WHEN protocol operation also needs off-chain discretion
- THEN the row uses `partial-kernel` and names the external capability.

### Requirement: evidence-complete rows

Each row SHALL name source evidence, canonical pattern, supported obligations,
unsupported obligations, trust assumptions, proof duties, SDK duties, and confidence.

#### Scenario: no public evidence supports one behavior

- WHEN the behavior cannot be verified
- THEN the row states absence of evidence and lowers confidence.

### Requirement: legacy regression preservation

The package SHALL retain thirteen Marlowe-style canonical regressions alongside
the 72 product rows.

#### Scenario: a semantic upgrade changes one regression

- WHEN an expected trace changes
- THEN migration impact and the approving semantic motion are required.

### Requirement: independent reviewer privacy

Each row SHALL have an independent reviewer pseudonym. A private custodian SHALL
retain the identity mapping and independence evidence under the research-data
protocol.

#### Scenario: a reviewer helped author one row

- WHEN independence is materially impaired
- THEN another qualified reviewer decides that row.

### Requirement: no protocol overclaim

A bounded instance SHALL NOT imply complete protocol reproduction. Reports
SHALL aggregate coverage by obligation and capability, not only protocol count.

#### Scenario: all rows receive a disposition

- WHEN many rows are partial or outside-kernel
- THEN the report cannot state that Moriarty implements all 72 protocols.
