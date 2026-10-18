# Simulate615 whole-opinion mechanical fidelity

**PASS_WITH_SCOPE_QUALIFICATIONS**: 263 offline comparisons, no source/log
literal or digest mismatch. [RESULT.json](RESULT.json) records every comparison,
exact source/log quote location, full raw capture pins, manifest roots and the two
scope passages requiring careful reading. The original 37,454-byte opinion is
unchanged, SHA-256 `7e737b9a6347e97e4bafeddc4bac9bf221c721cf4cfd84470262370eeadb0e7f`.
This is neither a new review vote nor a corrected/spliced opinion. Root adopts
separately.

The exact two candidate sources, 10 source quote anchors and 9 real log quotes
match the packet and committed `615eceee9c6250d2f203cb5807d5c75482dbdb5d`, tree
`4a78114c06d5fbb858a4bf97b922c0446fabe978`. The actual runtime schema validates
the complete opinion. All 156 mandatory paths agree with its coverage list and
schema enum; every mandatory body is literally present in FULL-INPUT. The
308-file manifest, 304-file packet manifest and mandatory manifest validate
completely. Input manifest/schema/prompt digests bind the complete public input
set in RESULT.

FULL-INPUT is exactly 329,878 bytes and 8,865 splitlines (8,866 with the terminal
empty EOF entry). The 62,389-byte schema similarly has 1,887 splitlines or 1,888
EOF-inclusive entries. The opinion's line counts use the latter convention;
there are no omitted bytes. Model-read completeness and actual backend identity
remain unverified declarations, not conclusions of these comparisons.

Both 902-entry source-pin sets are corroborated: all after bytes match candidate
Git; 901 tracked before files match parent 605, and the already frozen new test
is unchanged. simulate.rs is the sole production mutation, containing only the
stated header-error propagation hunk. All mapped source-unit hashes and original
line ranges match Git or the physically supplied public renderer source.
Post-commit binding matches the tested after bytes; historical phase START
records retain parent 605 rather than claiming a new execution at 615.

All four full phase logs, exits and FINAL records match: RED exit 101, GREEN exit 0,
Core1-positive exit 0 and Core2-positive exit 0. All eight raw capture directories and frozen
case expectations were compared directly, including input/scenario bytes, argv,
process status, producer Error JSON, stderr and stdout. Explicit escaped-newline
narrative quotations are recorded as display representations; formal source/log
anchors match literal bytes.

RED captures four processes before assertions. Its single aggregate Rust test
passes the first unknown producer/exit/stdout assertions, then fails the human
stderr assertion at line 200. Missing, commented and empty-control assertions
are not reached on RED; their raw captures are checked separately by the root
comparison. GREEN reaches the complete aggregate. Empty-Core2 asserts exit 0,
empty stderr, status and segment.execution only. Other captured wrapper fields
remain unasserted. Existing positive controls retain their shared-library and
helper-selected-field limitations; all 27 constructor obligations stay open.

Two exact opinion passages require their detailed context:

- The phrase “program/plan hashes unasserted” is overbroad as a statement about
  every comparison: core_continuations.rs:173 compares the complete observed
  continuation with the prior actual continuation, indirectly checking carried
  hash preservation. It still asserts no independent expected hash value.
- The summary saying all digests match records including EXPECTED-FREEZE must
  not be read as universal repair-freeze equality. All three RED stderr files
  intentionally fail that equality, as the detailed opinion and actual root
  comparison explicitly state.

These qualifications preserve the original opinion and its detailed limits.
They do not replace its vote or add financial/security/target acceptance.

Repeat only the offline fidelity comparison:

```sh
python3 -B /home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/simulate-review-fidelity/validate.py --repo /home/charl/Moriarty/.worktrees/utxo
```

No private capture, provider, product/native/security execution, or shared/Git
write occurred. No source-authenticity/backend attestation, independent financial
oracle, whole-report agreement or parent/148/292/20 closure is inferred.
