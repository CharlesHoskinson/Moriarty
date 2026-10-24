# Main consolidation — October 24, 2026

The integration combines the current Moriarty language on main with Sherlock source and documentation, including the published October 24 source cut. It resolves 18 original conflicts, fixes the SDK error-type migration and preserves the existing CLI error contracts alongside the richer library API. PR #374 fixes a case-insensitive filename collision; PR #298 publishes the reviewed optimization proposal without adopting it.

Local validation passed 953 ordinary Core/CLI tests with one ignored Core test, workspace Rust formatting and the default-feature all-target SDK/CLI type check. The optimization OpenSpec passed strict validation with 16 requirements and 32 scenarios. See [the validation record](../../deliverables/main-consolidation-2026-10-24/VALIDATION.json) for review identities, raw-log digests and scope.

The DoctorWatson corpus check reports stale requirement evidence. Source literal transcriptions match; current requirement closure is not established by this merge. Historical raw snapshots and receipts remain unchanged. SDK proof tests, genuine recursive proving, stagenet settlement/readback and synthetic-rollup benchmarking are separate outstanding acceptance work.

The user requested one branch, retaining the existing default name main, and removal of Claude Co-authored-by commit trailers. The metadata rewrite preserves source trees, human attribution and commit ancestry correspondence. Unmerged branch experiments are retained as archive tags before deleting branch refs; an archive is preserved source, not qualified production code. The full pre-cleanup repository bundle remains outside the repository. The accompanying commit map and archive manifest identify retained histories.
