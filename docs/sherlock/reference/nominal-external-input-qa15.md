# Nominal external-input QA observations

The public consumer is `crates/mori/tests/nominal_external_input_compile_handler.rs`. It contains 15 designated-owner tests using the real Args, selected handler and public nominal compiler APIs. Fourteen unchanged bodies passed in QA4; corrected test 12 passed in QA5. QA5 listed 15 but executed only 1, with 14 filtered.

Test 12 preserves three distinct refusal routes. StepCap is `owner-admission`, with `CoreTooling/CapacityNotAdmitted` and `Field{name:"bounds.stepCap"}`. Excessive fuel is `genesis`, with direct `CapacityNotAdmitted` and a null locator. The typed layout mismatch preserves `OwnerAdmission/CoreTooling/AdmissionRejected` reason and detail. That layout pattern ignores its locator; it is not full serialized equality. Body checking had completed in each route.

The compile-only test resolves declared type and constructor rows. It checks a 23-key ProgramView object and selected joins, not independent equality of all 23 fields. Test 3 propagates the strict parser's own errors; tests 13/15 compare the current handler with the current public producer.

The [archive](../../results/sherlock-nominal-external-input-qa15-20261020/README.md) retains failed assumptions, owner corrections, complete raw observations and actual identity projections. The exact frozen normal Core 189 basis was reused. No whole-current-tree, Cargo, backend CLI, native, independent-oracle or EARS qualification follows.
