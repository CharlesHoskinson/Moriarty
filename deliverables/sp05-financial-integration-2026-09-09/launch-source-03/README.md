# Reject a private-input FIFO without hanging

The launcher opened a private input with blocking `O_RDONLY` before checking the descriptor's file type. A real owner-only FIFO with no writer therefore hung before regular-file rejection. The CLI reads its plan before installing its timer, so that rejection path also needed the correction.

The repair adds `O_NONBLOCK` to the existing open. Descriptor-based regular-file, ownership, permissions and size checks remain in place. This is a Linux/POSIX FIFO correction, not a universal filesystem latency guarantee.

- [Original failing regression](fifo-red.txt): the reader child reached its two-second timeout; the later CLI assertion was not reached.
- [Launcher regression](launcher-green.txt): all 12 tests pass, including the reader and CLI FIFO rejection.
- [Broader regression](ledger-regression-with-artifacts.txt): all 169 tests pass using the existing retained custody artifacts; no compilation, proof or network execution. The [first invocation](ledger-regression.txt) omitted `MORIARTY_CUSTODY_ARTIFACTS` and failed that explicit prerequisite. Its failure is retained.
- [Independent GPT-6 review](gpt6-review.json): `PASS_SCOPED` for the correction, with no blockers.
- [Grok correction attempt](grok-observation.json): local termination at 90.034 seconds, continuing progress events but no final verdict. Token usage and upstream cancellation are unknown. The user corrected the short deadline; a [same-scope attempt with 600 seconds](grok-02-start.json) preserves the exact prompt and candidate. No approval is inferred before a substantive terminal result.

The [candidate](candidate.json) pins the changed source and test. Existing source-02 reviews and resource plans retain their original bytes and scope; they do not approve this changed source. Full launcher review, current resource admission, financial settlement, Preview validation and mandatory proof acceptance remain open. This candidate may be retained on GitHub for recovery without being promoted as accepted execution code.
