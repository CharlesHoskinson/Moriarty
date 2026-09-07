# Candidate A harness archive audit

Scope: root-created swap/installment archive, manifests and root receipts at
`a4677cd985a540d9c560de2ad7d882495fc92b78`. Native reviewer
`execution_adversarial_tests` authored the original harness source and author
receipts; this is an independent archive-integrity check, not independent source
review or Council acceptance.

All 58 manifest pins matched exact source-commit or archive bytes. Neither bundle
had an unpinned artifact. Both eight-file RED closures are import-complete. All
six ITFs and both author reports are byte-identical to their original scratch
files. The swap pins program bytes b443fc0b at source 3c648ec; the installment
pins bf814bce at source c002a84. Subsequent worktree changes are not substituted.

Root receipts preserve terminal zero exits for 19 swap tests, 23 installment
tests, 19 swap regression tests, and both 1,000-trace runs. All action witnesses
are positive and both terminal witnesses count 1,000. Outcome partitions match
the raw counts: swap 52+48+109+260+531; installment 59+246+62+511+122.

The reviewer found one minor wording issue: the installment manifest said its
author receipt recorded the tool version, but it records only the entrypoint
hash. Version 0.32.0 appears in the adjacent swap receipt for that same hash.
The manifest now makes that cross-reference explicit; immutable author reports
were not changed. No source or runtime failure was inferred from this correction.

No new tests were executed during this archive audit. Bounded sampling, local
consistency checks and these archive checks do not establish independent Python
correspondence, exhaustive model checking, authority integration, or S02 completion.
