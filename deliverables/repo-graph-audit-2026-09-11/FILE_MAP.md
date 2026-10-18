# File-level repository map

This packet is a **file-level repository graph** companion to `graphify-out/graph.json` (semantic + AST).
It does not claim that every listed file was AST-parsed or semantically reviewed.

## Identity

- Requested extraction identity: `grok-4.6`
- Host-declared runtime identity: `Grok 4.6 released by xAI`
- Observed environment model fields: `None`
- Builder: `deliverables/repo-graph-audit-2026-09-11/scripts/build_file_map.py`
- Generated at: `2026-09-11T18:57:58.417708+00:00`
- Repository HEAD: `89b0c7b9809b501c091223c78f6dd2df8581936c`
- Wall-clock seconds: `20.064`
- Detect corpus: `deliverables/repo-graph-audit-2026-09-11/graphify-out/.graphify_detect.json` (read-only)

No LLM completion was used to invent nodes, edges, or symbol calls.
The measured builder time is 20.064 seconds of local Python work.
That is not a token invoice and not a claim of zero cost.

## Source coverage preserved from detect

- Supported files reported by detect: **42275**
- Unclassified files reported by detect: **16469**
- Union of unique repo-relative paths before this script's re-filter: **58744**
- Detect `total_files`: 42275
- Detect `total_words`: 46277977
- Detect skipped_sensitive: 231
- Detect ignored entries: 85
- Detect pruned_noise_dirs: 202
- Detect walk_errors: 0
- Union path collisions (same path in more than one detect list): 0
- Absolute paths outside the repo prefix: 0

Supported files by detect type:

| Type | Detect count |
| --- | ---: |
| code | 21161 |
| document | 19431 |
| image | 1554 |
| paper | 129 |
| video | 0 |

Detect warning (verbatim):

> Large corpus: 42275 files · ~46,277,977 words. Semantic extraction will be expensive (many Claude tokens). Consider running on a subfolder.

## In-scope file map after re-filter

- In-scope files: **58729**
- Directory nodes: **10320**
- Total nodes: **69049**
- Total links: **71653** (containment + EXTRACTED literal references)
- EXTRACTED literal reference edges: **2605**
- Bounded textual sources scanned: **795**
- Text sources skipped as too large: **1**

Kept files by detect type:

| Type | Kept |
| --- | ---: |
| code | 21156 |
| document | 19425 |
| unclassified | 16465 |
| image | 1554 |
| paper | 129 |

Kept files by first path segment:

| Segment | Kept |
| --- | ---: |
| `repos` | 40162 |
| `raw` | 10057 |
| `deliverables` | 5120 |
| `evidence` | 2329 |
| `experiments` | 316 |
| `graphs` | 206 |
| `corpus` | 100 |
| `openspec` | 83 |
| `inbox` | 71 |
| `.raw` | 57 |
| `site` | 50 |
| `wiki` | 44 |
| `docs` | 34 |
| `plugins` | 28 |
| `reports` | 20 |
| `scripts` | 17 |
| `.superpowers` | 13 |
| `(root)` | 12 |
| `.moriarty-dev` | 5 |
| `.foreman` | 2 |

## Exclusion policy

Authoritative input is the existing detect JSON. This script does not rescan the live tree for membership.
Each union path is re-checked and dropped on the first matching reason:

1. This deliverable packet (`deliverables/repo-graph-audit-2026-09-11/`), including this map's own outputs.
2. `.worktrees/` checkout duplicates.
3. Main-script `EXTRA_EXCLUDES` (secrets, witnesses, wallets, selected caches, this packet's `graphify-out` / `graphify-out-prior`, `.git/`).
4. Graphify builtin skip/noise directories (`_SKIP_DIRS` / `_is_noise_dir`), including `.git`, `.worktrees`, `graphify-out`, dependency and cache dirs.
5. Graphify `_SKIP_FILES` lockfiles.
6. Graphify `ignored_predicate(..., extra_excludes=EXTRA_EXCLUDES, gitignore=False)`.
7. Symlink targets that resolve outside `/home/charl/Moriarty`.
8. Non-regular files.
9. Graphify `_is_sensitive` builtin credential heuristics.

Dropped from the union by this re-filter:

| Reason | Count |
| --- | ---: |
| deliverable_packet | 11 |
| skip_dir | 4 |

- Total dropped from union: **15**

Detect already recorded skipped_sensitive, ignored, and pruned_noise_dirs. Those counts stay as source coverage.
They are not double-counted as in-scope files.

## Identifiers and edges

- File node `id` is the exact repo-relative POSIX path from detect. It is not a lossy slug and is not the resolved symlink target.
- Directory node `id` is that directory path with a trailing `/`. The repository root is `.`.
- `source_file` on a file node equals `id`.
- Directory containment edges use `relation: contains`, `confidence: EXTRACTED`.
- Literal Markdown links, wikilinks, and backtick file references from bounded textual sources use `relation: references`, `confidence: EXTRACTED`.
- No `calls` / symbol edges are created.

## Validation

- Duplicate node IDs: 0
- Path/ID mismatches: 0
- Links with missing endpoints: 0
- Kept-path vs file-node mismatch: 0
- Zero path/ID collisions: True
- Endpoints valid: True
- Validation ok: True

## Outputs

- `deliverables/repo-graph-audit-2026-09-11/repository-file-map.json` (55702822 bytes)
- `deliverables/repo-graph-audit-2026-09-11/repository-file-map.html` (5535502 bytes)
- `deliverables/repo-graph-audit-2026-09-11/FILE_MAP.md`

The JSON uses the graphify exported node-link shape: `directed`, `multigraph`, `graph`, `nodes`, `links`, `hyperedges`, `built_at_commit`.
Node IDs in this file are paths. They do not merge with slug IDs in `graphify-out/graph.json`.

## HTML

The HTML is a self-contained offline directory tree. It embeds a compact path index.
Directories render children only when expanded. Search lists at most 200 path matches.
Selecting a file shows parent containment and EXTRACTED literal cross-references.
It does not mount tens of thousands of DOM nodes at once and does not load a force-directed graph of the whole map.

## Limitations

- Membership is the detect snapshot, not a live `os.walk` of the dirty tree after detect.
- In-repo symlinks keep their own path IDs. Only symlink targets that resolve outside the repository are dropped.
- `repos/` is the largest first segment; pinned clones dominate file counts.
- Unclassified files are listed. Their bytes were not parsed as source.
- Literal references are extracted only from bounded first-party `.md` / `.mdx` / `.rst` files (not `repos/`, `raw/`, `evidence/`, `graphs/`, `corpus/`, `inbox/`, or this packet).
- Wikilinks resolve only when the target path or `path.md` is already in-scope. Missing targets are omitted, not invented.
- Sensitive-name heuristics can drop documents whose filenames look like credential stores.
- This map is not product acceptance, not financial evidence, and not an AST coverage claim.
