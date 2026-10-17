# Additive EARS index

DRAFT; original35 UIREQ and all retained requirements remain authoritative. New SCON IDs refine implementation coverage; they are not a completion denominator. Every clause includes an exact EARS condition/response and two observable scenarios in [spec.md](specs/sherlock-completion/spec.md).

| ID | Refinement | Existing requirements | Batch |
|---|---|---|---|
| SCON-001 | Explicit execution profile | UIREQ-001, UIREQ-002, UIREQ-012, UIREQ-013, UIREQ-021, UIREQ-025 | B03 |
| SCON-002 | Explicit observation adapters | UIREQ-003, UIREQ-004, UIREQ-009, UIREQ-010, UIREQ-013, UIREQ-021, UIREQ-025, UIREQ-028, UIREQ-029 | B03 |
| SCON-003 | One public compile path | UIREQ-001, UIREQ-002, UIREQ-005, UIREQ-022, UIREQ-023, UIREQ-025, UIREQ-027, UIREQ-031 | B04 |
| SCON-004 | Program grammar and identity | UIREQ-001, UIREQ-002, UIREQ-004, UIREQ-005, UIREQ-007, UIREQ-008, UIREQ-012, UIREQ-022, UIREQ-027, UIREQ-029, UIREQ-034 | B04 |
| SCON-005 | Full39 A implementation map | UIREQ-003, UIREQ-004, UIREQ-005, UIREQ-006, UIREQ-007, UIREQ-008, UIREQ-009, UIREQ-010, UIREQ-011, UIREQ-012, UIREQ-013, UIREQ-023, UIREQ-026 | B05 |
| SCON-006 | B runtime and guard map | UIREQ-012, UIREQ-013, UIREQ-027, UIREQ-028, UIREQ-029, UIREQ-034 | B06 |
| SCON-007 | Complete trace continuity | UIREQ-011, UIREQ-013, UIREQ-016, UIREQ-021, UIREQ-024, UIREQ-025, UIREQ-032, UIREQ-035 | B01 |
| SCON-008 | Observer and semantic terminal precedence | UIREQ-010, UIREQ-011, UIREQ-012, UIREQ-013, UIREQ-016, UIREQ-021, UIREQ-032 | B01 |
| SCON-009 | Persistent boundary export and import | UIREQ-009, UIREQ-010, UIREQ-012, UIREQ-013, UIREQ-014, UIREQ-015, UIREQ-016, UIREQ-018, UIREQ-029, UIREQ-032 | B07 |
| SCON-010 | Financial continuation complete observations | UIREQ-014, UIREQ-015, UIREQ-017, UIREQ-018, UIREQ-019, UIREQ-020 | B10 |
| SCON-011 | Explicit financial versus technical boundaries | UIREQ-014, UIREQ-015, UIREQ-016, UIREQ-017, UIREQ-018, UIREQ-019, UIREQ-020, UIREQ-023 | B10 |
| SCON-012 | Separate constraint witness and exporter plans | UIREQ-022, UIREQ-023, UIREQ-026, UIREQ-030, UIREQ-031, UIREQ-035 | B08 |
| SCON-013 | Deterministic witness baseline | UIREQ-009, UIREQ-013, UIREQ-024, UIREQ-031, UIREQ-035 | B08 |
| SCON-014 | Required bounded residualization | UIREQ-005, UIREQ-010, UIREQ-012, UIREQ-013, UIREQ-024, UIREQ-027, UIREQ-033 | B08 |
| SCON-015 | Required earliest-stop grouping | UIREQ-010, UIREQ-012, UIREQ-013, UIREQ-016, UIREQ-024, UIREQ-032 | B08 |
| SCON-016 | Generic declarations and ordered matches | UIREQ-004, UIREQ-007, UIREQ-008, UIREQ-009, UIREQ-027, UIREQ-029, UIREQ-034 | B06 |
| SCON-017 | Finite interpreter Quint model | UIREQ-010, UIREQ-012, UIREQ-013, UIREQ-014, UIREQ-015, UIREQ-016, UIREQ-017, UIREQ-018, UIREQ-019, UIREQ-021, UIREQ-026, UIREQ-028, UIREQ-029 | B09 |
| SCON-018 | Finite model reachability and sensitivity | UIREQ-021, UIREQ-024, UIREQ-025, UIREQ-026 | B09 |
| SCON-019 | Per-slice iterative correctness | UIREQ-003, UIREQ-004, UIREQ-005, UIREQ-006, UIREQ-007, UIREQ-008, UIREQ-009, UIREQ-010, UIREQ-011, UIREQ-021, UIREQ-023, UIREQ-024, UIREQ-025, UIREQ-026 | EVERY |
| SCON-020 | Finite local DoctorWatson batches | UIREQ-022, UIREQ-024, UIREQ-025 | EVERY |
| SCON-021 | Documentation on every code batch | UIREQ-002, UIREQ-022, UIREQ-024, UIREQ-025 | EVERY |
| SCON-022 | Scoped first native and target demonstration | UIREQ-022, UIREQ-023, UIREQ-025, UIREQ-026 | B11 |
| SCON-023 | Full retained v1 completion | UIREQ-001, UIREQ-002, UIREQ-003, UIREQ-004, UIREQ-005, UIREQ-006, UIREQ-007, UIREQ-008, UIREQ-009, UIREQ-010, UIREQ-011, UIREQ-012, UIREQ-013, UIREQ-014, UIREQ-015, UIREQ-016, UIREQ-017, UIREQ-018, UIREQ-019, UIREQ-020, UIREQ-021, UIREQ-022, UIREQ-023, UIREQ-024, UIREQ-025, UIREQ-026, UIREQ-027, UIREQ-028, UIREQ-029, UIREQ-030, UIREQ-031, UIREQ-032, UIREQ-033, UIREQ-034, UIREQ-035 | B12 |
| SCON-024 | Measured optimization claims | UIREQ-013, UIREQ-024, UIREQ-025, UIREQ-031, UIREQ-032, UIREQ-033, UIREQ-035 | B08 |
| SCON-025 | First parameter-free public compiler increment | UIREQ-001, UIREQ-002, UIREQ-006, UIREQ-022, UIREQ-025, UIREQ-027, UIREQ-031 | B04 |

Final03 reconciliation preserves25 SCON IDs and35 original UIREQ; 68 total scenarios (18 additive R02 scenarios plus50 original). New subclauses/scenarios are under existing SCON-007/009/010/011/013/014/016/017/023, retaining original requirements. Exact final03/loop/annex/source/result bindings are in final03-binding.json.
