# Streaming JSON utility plan admission

Root read all491lines, including all four complete implementation/test blocks,
of plan SHA256a75dbb9283027d2937e663653289f3761a3813f82603cd99532042b0de571d95.
The utility is standard-library-only and supplies no schema, fixture or authority
judgment. Sharing it is explicitly disclosed; producer and checker semantics
remain independently implemented.

Review covered string/container scanning across UTF-8 chunks, escaped duplicate
keys, strict numeric conversion and trailing-data handling, lazy array events,
completion only after root EOF, short binary writes and explicit resource limits.
The reader preserves the JSON scalar types for later consumer validation. The
writer rejects non-string object keys, tuples, nonfinite values and unencodable
surrogates rather than normalizing them into accepted evidence. Partial staging
streams cannot be accepted. ResourceLimit must not be counted as a semantic kill.

Root ran a read-only synthetic feasibility probe of the planned reader on one
4194319-byte JSON document: terminal0, one item and complete end event, approximately
0.794s. This is not an implemented utility test, actual authority artifact or
production throughput measurement. The original probe command/result is retained
separately. Native largest-case and final Python memory measurements remain open.

Task1 reader and Task2 writer/hash are separately dispatched and admitted after
their actual importable RED/GREEN controls and original source/runtime receipts.
No consumer source edits or actual exports are authorized by this plan admission.
