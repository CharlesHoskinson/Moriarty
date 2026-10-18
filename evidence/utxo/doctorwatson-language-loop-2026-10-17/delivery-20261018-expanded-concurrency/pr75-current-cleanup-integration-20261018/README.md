# Current PR75 documentation cleanup, October 18

PR75 has already merged. Its actual head is `35a7275ad22fd8c13b37aba08842773e64ed41e8`, tree
`733c981de96ad6ac280a036355bd81be4bbdd1a3`, and merge commit is
`baf61ebd0cb49f2c4441cb7fa8de53b343e056dc` (2026-10-18T04:48:14Z).
Both original proposed documentation after-hashes equal the actual head files.
The October 17 cleanup is therefore delivered; do not replay that patch or
reopen the merged PR.

The remaining two-document correction is [pr75-current-provenance.patch](pr75-current-provenance.patch).
It replaces stale pre-merge lifecycle language with the observed merge, keeps
review bindings scoped to their exact historical candidates, labels original
UNRUN/NOT_GRANTED/native0 fields as capture-time history, and gives the full
historical fixture directory plus links to the actual owning selectors.
It supplies no execution-authority or security decision.

The live-main form is [live-main-current-provenance.patch](live-main-current-provenance.patch),
pinned to `e4d7a2b5709caa18a94f5256d03a0302364b8235`. Live main already added the shared
`outcome_report` consumer and 179 guide lines covering report-library checks
and adopted finite Core/2/M01/M03 observation clauses. Those additions remain
byte-identical in the candidate. Both patches reconstruct exactly against their
own pinned Git bytes. Neither was applied to a checkout or index.

The CLI requires DIRECTORY and reads DIRECTORY/mori-tests.json
(`crates/mori/src/commands/test.rs:8–11,39–43` at PR head). The stage consumer
uses only explicit manifest cases and member paths
(`crates/mori/src/commands/test/stage_test.rs:69–75,91–138`). The public test
copies only root regular files and skips subdirectories
(`crates/mori/tests/stage_test_public.rs:24–31`), then selects its temporary root
(43–48). The root manifest names two cases and six expected reports; no history
path is selected. Explicitly selecting the preserved history remains possible;
its invalidated expectations remain unusable for current positive validation.
External wrappers are outside this source finding.

Normal source checks performed: pinned API-content versus Git-byte equality;
all manifest references exist; the six reports and four canonical preimages
match their R3 copies; the independent-freeze SHA-256 remains
`ad44178b801dc51bfa6e254a2d9b48c18a4f7ea99e7b919be75730dc761cc34d`;
all frozen fixture inputs and expectations are unchanged (only maintained
PROVENANCE.md changes within the90-file directory); candidate relative links exist; both full
unified patches reconstruct in memory to their exact after-files. Checks and
before/after pins are in [RESULT.json](RESULT.json) and [SOURCE-PINS.json](SOURCE-PINS.json).
No new product-code defect is established by these source checks.

Before later integration, refresh actual head/main pins, inspect full candidate
contexts, repeat the source/link/frozen-data checks on that exact tree, and obtain
fresh applicable review under current routing. Product build, CLI/runtime and
actual-result review are separate. This task ran no product source, builds,
providers, reviews or approval decisions and changed no Git/index/shared/GitHub
state. All parent/EARS acceptance remains open.

The GitHub PR diff endpoint refused HTTP406 because its current changed-files
aggregate exceeds300 (API reports13225). That raw refusal is preserved.
`actual-merge-first-parent.diff` reconstructs the actual source integration
against first parent `7a25a75e6074ac6c9728864dd9b874bd00ef564e`:167 paths.
The API's base field `f7ff06ed05bcef5fc3540bdde70ddbf6402f5547` is preserved
separately from the live main ref; those values are not interchangeable.
