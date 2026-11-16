## ADDED Requirements

### Requirement: WRB-001 Wrap row gate at k = 21
WRB-001: The wrap circuit SHALL use at most 1,782,573 rows at k = 21 on the final key.

Source: FD-15 (A-15). Label: 1,782,573 is the gate, DERIVED; today 1,762,714 rows, MEASURED, with 19,859 free, DERIVED. Hard rule: never k = 22.

#### Scenario: Measured on the final key
- **WHEN** the final wrap key is generated
- **THEN** the row count is MEASURED and at or under 1,782,573

#### Scenario: Over the gate
- **WHEN** the wrap needs more than 1,782,573 rows
- **THEN** the rekey stops and no k = 22 circuit is made

### Requirement: WRB-002 Wrap VK size
WRB-002: The wrap verifying key SHALL be at most 50,000 bytes as measured on the final key.

Source: FD-15. Hard rule: ledger 9.1.0.1 verifier unchanged, wrap VK at most 50,000 B.

#### Scenario: VK read by the ledger
- **WHEN** the final wrap VK is serialized
- **THEN** its size is MEASURED at or under 50,000 bytes and ledger 9.1.0.1 accepts it natively

### Requirement: WRB-003 Row reserve for the fallback terminal forms
WRB-003: The wrap row budget SHALL keep at least 100,000 rows free after reserving 60,000 rows for the fallback terminal forms of CSL-009.

Source: FD-15 (K-3, C-02, C-07). Label: 60,000 rows ESTIMATE; 100,000 rows is the debate's bar. On the T10 fixture `b3k3p20n19` uses 1,160,737 rows, MEASURED (cost model).

#### Scenario: Budget at the final shape
- **WHEN** E-PCS step 1 prices the wrap at the T25 shape with the collection terminal change in
- **THEN** the free rows after the 60,000-row reserve are at least 100,000
