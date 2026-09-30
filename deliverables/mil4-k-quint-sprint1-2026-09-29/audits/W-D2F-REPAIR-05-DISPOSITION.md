# W-D2F repair-05 disposition

**Decision, 2026-09-30:** retain the frozen W-D2F field map and schedule as a bounded specified-only candidate for the next implementation sprint. This is not adoption of W-D2 semantics or a successful consumer.

The exact [packet](w-d2f-grok-repair-05-candidate-packet.md) has SHA256 `13408bbbefd0b7925f956ef6851560853b16faaea7fdd89ae74d5950a1736b83`. GPT-6.1 Sol high [recomputed its 14 manifest hashes](w-d2f-grok-repair-05-gpt-6.1-sol-high-audit.md) and reported no high/medium defect. Grok was requested as 4.7 xhigh; its [raw result](w-d2f-grok-repair-05-grok-4.7-xhigh-raw.json) reports `grok-4.7-build` and no high/medium contradiction. Grok read only embedded bytes and did not independently recompute hashes. The two prior medium literal-oracle defects are resolved in this packet.

Scope: the header, 35 signed fields, 14 operation fields, F/D/M/H/C/I/E with optional L, B01–B17, six proposed positives, 152 hostile expectations and 11 inspections. The cases are written expectations, not executed consumer results. B01–B16 authenticators, complete B16 history transport, full semantic consumer and B17 atomic ledger application remain unavailable. W-D2 and Sprint 1 stay open.

Low review residue to track during integration: normalize `binding` versus `retainedFactBinding` keys; make tag-35 obligation indexing safe before unsigned-tail length checking; pin signed-versus-submitted action diagnostic paths in executable tests. These were not high/medium objections to the frozen design.
