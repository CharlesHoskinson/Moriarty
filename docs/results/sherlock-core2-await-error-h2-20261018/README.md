# Core Error primitive representation: H2 slice

The Muse-authored `_error` conversion now preserves null and exact unsigned integer `offset:u32` and `step:u16`, rejecting bool, float, numeric strings, negatives and overflow. This selected consumer is 12,712 bytes, a 307-byte delta from the **exact 12,405-byte H2 Bool source**, not directly from the installed 12,301-byte H1 source.

The [fresh Muse review](independent-source-review/FINAL-RESPONSE.txt) accepted the materialized source and exact one new two-test operation. Root ran only the two new private `_error` methods once: exit 0 in 0.060130516 seconds, both named `ok`, `Ran 2 tests`, `OK`. The [complete receipts](actual-new-two-01/RESULT.json) retain full 318-byte stderr, empty stdout, exactly two environment keys and before/after three-entry/source/Python checks. No earlier tests were replayed.

Both author and reviewer used the standing Muse Wonder 1.0 high route after Root observed an actual Opus 429 outage. They are not relabeled Opus. The original author diff has a defective new-line count (10 declared, 12 explicit). Acceptance applies to the materialized literal body and normalized selected diff, not the raw patch as an applicable patch. Author RESULT/COMMAND also retain terminal literal backslash-n after JSON; those raw bytes and their qualification remain disclosed.

[Qualified result](QUALIFIED-RESULT.json) retains remaining H2 advanced/numerics/Quint and H3–H6 gaps. Direct `_error` tests do not establish integration through the Rejected result branch, full consumer compatibility, CLI or real Core2/B06 traces. No financial, program, effects, admission, native, B-charge or EARS closure follows.
