# Full39 semantic source review support

This directory preserves source-review evidence for the exact map one directory
above. It contains no execution result, invocation grant or publication approval.
The entire new publication candidate still needs fresh full owner and Muse review.

| Capture | Exact uncompressed source |
|---|---|
| `opus-full39-source.json.gz` | Raw full39 Opus `RESULT.json` bytes |
| `opus-full39-source.md.gz` | That JSON's `text` string encoded as UTF-8, unchanged |
| `muse-full39-source.json.gz` | Raw full39 Muse r2 `RESULT.json` bytes |
| `muse-full39-source.md.gz` | That JSON's `text` string encoded as UTF-8, unchanged |
| `ROOT-FULL39-SOURCE-ADOPTION.json` | Raw later Root semantic-adoption record |
| `INVENTORY-CLARIFICATION.json` | Raw Root 32-member-plus-manifest clarification |
| `original-full39-MANIFEST.json.gz` | Raw original local manifest, inventory only |

Each gzip has mtime=0, no filename header and one member. No newline, Unicode
normalization or JSON reserialization is applied to its uncompressed capture.
The [binding](publication-binding.json) records compressed and uncompressed
hashes/sizes and the actual source locator of each selected body. It also binds
the unchanged public L4 and normative Rust/K dependencies. The binding omits its
own hash; the separate publication-preparation manifest binds the binding.

Opus's original review reported a 32/33 discrepancy and described a manifest
self-reference concern. Those words remain raw evidence. The later Root record
and inventory clarification resolve the inventory: 32 members plus the separately
bound manifest is 33, and the manifest does not list its own hash. These are
bookkeeping clarifications, not rewritten review text or runtime observations.

The original manifest includes local artifacts not selected for publication.
The four auxiliary bodies `SEMANTIC-CROSSWALK.json`, `SOURCE.patch`,
`author_proposal.py` and `freeze_source_data.py` were identity-only in the Muse
transport and are not normal-approved. None of those bodies is published here.
The map's historical path to the companion crosswalk does not imply that its
body accompanies this publication. Source semantics and full R5/L4 dependencies
are the selected public bodies; this support directory is not the entire author
packet. Scripts, patches, local debug/log/prompt bodies and private source/SDK
content are not part of this selected publication.

Both votes are `ADOPT_FULL39_P1_SOURCE_WITH_CONDITIONS`, not unconditional
completion. The parent README and Root record preserve F1–F7 and the remaining
P1/P3/P2–P6/G4/F4/MD2 obligations verbatim. The raw Opus vote retains its original
12-row authorship independence limitation. Muse's complete semantic reading
and four identity-only auxiliary bodies retain their exact coverage limits.
Neither reviewer recomputed hashes or executed correspondence. The later local
hash/gzip checks are bookkeeping only.

Historical author labels and the old scoped12 record are not upgraded or erased.
Full39 conditions now also govern the old twelve arrays. No new resource,
physical mechanism, ABI/binding, circuit, proof, financial, ledger or stagenet
acceptance follows. Executable validation is UNRUN; native authority is zero.
