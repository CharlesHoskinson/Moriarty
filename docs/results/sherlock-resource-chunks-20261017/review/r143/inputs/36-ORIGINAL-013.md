### Requirement: UIREQ-013 Versioned work observations
WHEN a semantic transition consumes work, tooling SHALL report its resource observations under the selected versioned ResourceManifest.

#### Scenario: Same computation in chunks
- **WHEN** The same admitted computation is collected in one trace and in two technical chunks.
- **THEN** Its cumulative reference work is the sum of the same semantic charges; physical transitions, helper visits, logical frame depth, physical frame/environment bytes, preparation/serialization bytes, host time/memory, circuit rows/degree, proof/key bytes and ledger fees remain separate fields. sherlock-k-a/1 and sherlock-quint-a/1 retain their unequal terminal charges; sherlock-helper-microstep/1 is a new proposed schedule, without same-fuel equivalence or a measured cost winner.

