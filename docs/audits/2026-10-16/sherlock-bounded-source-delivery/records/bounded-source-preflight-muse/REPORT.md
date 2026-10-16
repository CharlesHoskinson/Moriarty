**APPROVE_WITH_SCOPE** — nonsecurity half only.

This bounded job satisfies H1 sufficiently for its stated purpose: two hash-pinned, fixed Core source fixtures (183/204 bytes), fixed scalar arguments (4,9) and (7,10), fuel capped at 35/33, stack limit 8, and caller-stated occurrence/nesting/record/slot counts checked before preparation or machine construction. Terminal outcomes, including the retained H1 warning and A `Rejected(4/8/10)` behavior, are unchanged.

Approval is strictly limited to:

- The exact manifest snapshot and steps (0)–(6) as proposed, in a fresh directory outside the worktree, stopping at the first failure.
- Hash verification of the snapshot, selected artifacts, and `rustc -vV` showing 1.99.0 before step (1); no retry, fallback, added optimization, or limit increase.
- The 14 named Opus tests, the fixed no-argument producer, and comparison through [check_source_observations.py](/tmp/models/sherlock/qa/check_source_observations.py) against the independently frozen oracle.
- Local diagnostic use only: no merge, push, publication of raw captures, installation, generic or untrusted input path, network use, or native/symbolic authority.

Retained non-blocking gaps for this scope: the counts are not byte, wall-clock, memory, disk, or native-stack guarantees; recursive third-party artifacts and compiler proc-macros remain an opaque trust boundary; and there is no host memory or disk cap. Those gaps do not defeat this fixed, timeout-bounded local run, but they prevent any broader H1 or untrusted-API claim.

The actual new Core build must wait for Opus’s separate security agreement with this same scope.
