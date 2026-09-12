# Large corpus warning

Graphify detect on `/home/charl/Moriarty` (2026-09-11, gitignore disabled, extra excludes applied) reported:

- **42,275 supported files**
- **~46,277,977 words**
- Detector warning: `Large corpus: 42275 files · ~46,277,977 words. Semantic extraction will be expensive (many Claude tokens). Consider running on a subfolder.`

The skill would normally stop here and ask which first-level subdirectory to scan. The user overrode that narrowing prompt and required entire-repository scope. This run continued on the full detected set.

## Top first-level directories (detected supported files)

| Rank | Directory | Detected files |
| ---: | --- | ---: |
| 1 | `repos/` | 24,964 |
| 2 | `raw/` | 9,954 |
| 3 | `deliverables/` | 4,309 |
| 4 | `evidence/` | 2,060 |
| 5 | `experiments/` | 287 |

## Detected categories

| Category | Files |
| --- | ---: |
| code | 21,161 |
| document | 19,431 |
| paper | 129 |
| image | 1,554 |
| video | 0 |

This warning is retained as an audit fact. It is not a claim that every detected file was semantically read.
