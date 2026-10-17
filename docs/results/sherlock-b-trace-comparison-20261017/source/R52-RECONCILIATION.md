# Independent expected positions: source reconciliation only

Read unchanged R52 EXPECTED.json only after CANDIDATE-FREEZE-01. Its SHA256 is `06e851db225437474707f39cd8a586bb9195801d918a43cc897a9170462a6e24`. Root did not read the new implementation/test source before freezing its 16 cases. The test source now contains `independently_frozen_positions_match_exact_supplied_contrasts`, with explicit E01–E16 checks against those positions. This is authored test source, not observed execution.

| IDs | Exact supplied contrast | Expected location |
| --- | --- | --- |
| E01–E02 | Equal present empty/nonempty | None |
| E03 | Initial layout metadata | Initial |
| E04 | Record0 before metadata | Record0 before |
| E05 | Record0 event work delta | Record0 event |
| E06 | Record0 after remaining work | Record0 after |
| E07 | Equal prefix, right tail | Length common1/left1/right2 |
| E08 | Equal prefix, left tail | Length common1/left2/right1 |
| E09 | Before/event/after differ | Record0 before |
| E10 | Initial + event differ | Initial |
| E11 | Common event differs, right shorter | Record0 event |
| E12 | Initial widths None/Some(empty) | Initial |
| E13 | Initial constant marker_free | Initial |
| E14 | Event source occurrence | Record0 event |
| E15 | Initial names same members/opposite order | Initial |
| E16 | Aliased same object and structural clone | None |

All 16 have literal checks in the revised test. Before revision02, several rules were covered only by equivalent contrasts at different boundaries/directions; exact source fixtures were added without changing the independent table or comparator. Other tests add borrowed-value identity, field priority, ascending record index, zero/one length, every top-level raw event field and selected concrete snapshot groups.

No case was executed, and position-label presence is not a Rust typecheck. A nonzero/distinct SegBase remains outside this fixed public genesis fixture; no private constructors or expected-data substitution were added. Entire snapshot equality still compares the stored SegBase via existing Eq.
