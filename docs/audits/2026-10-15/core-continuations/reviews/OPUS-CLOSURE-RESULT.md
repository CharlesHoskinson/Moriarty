Candidate `0e42392d` (tree `7cccf9c4`) is approved as a qualified closure of the whole local child feature. Both of the earlier Opus review's required findings are closed, and no required finding remains. I ran nothing; this rests on reading the source, the hashes and the captured results.

| Axis | Verdict |
|---|---|
| SOURCE | **APPROVE** |
| ACTUAL-RESULTS | **APPROVE**: 88 test functions (17 freshly captured, 71 carried over); not a rerun of all 88 |
| DOCS | **APPROVE** |
| INTEGRATED | **APPROVE**, qualified: `local-reference-stage` / `none` / `untracked` |

**F1/H18 (signed Target on `/2` compile): closed.**
- The new test is at `crates/mori/tests/core_continuations.rs:277-293`. Its Target text is an exact copy of the public `target_canonical_utf8` in the golden vector, with no key material, and the test first confirms that Target is signed.
- It checks exit code 1, empty stdout, the exact `mori::backend::unsupported` diagnostic, that no `--out` directory exists, and that only the two input files are present.
- In the unchanged production code, `lower.rs:23` refuses a signed Target before the program is loaded at `:29`. `compile.rs:25` creates the output directory only after compilation succeeds.
- Capture `cli-normal-05`: 12 passed, exit 0. I recomputed both stream hashes and every pinned file hash matches the current files.

**F2/H15 (`await`/`when` as `/1` names): closed.**
- The new test is at `crates/mori-core/tests/compat_v1.rs:124-143`. It compares the exported Program (881 bytes) and Execution (2,092 bytes) byte for byte against hand-written fixtures. It also checks that each fixture decodes back to the same value.
- The fixtures follow the field order of Program, Execution and Step in the base (`fe01bc8b`) `ir.rs`. `json::export` has not changed since base; only the parsing functions in `json.rs` changed.
- The fixture files were written before the test code (00:42–00:43 versus 00:43:46), and nothing generates expected bytes from the candidate's own output.
- Capture `core-compat-02`: 5 passed, exit 0, and its hashes are verified the same way.

**What I relied on from earlier reviews**
- **Unchanged production:** every crate `src` file, Cargo.lock, Cargo.toml, build file and the golden vector is byte-identical to `7d6c3965`. That carries over the dd4 Opus review's H1–H28 table and normative reads, and the dd4 Muse approval. Their copies in the dossier are byte-identical to the runtime originals.
- **Test edits are additions only:** both test files only gained lines at the end. Their earlier hashes match the dd4 pins.
- **Hashes and pins:**
  - Both checksum manifests pass `sha256sum -c`.
  - The six frozen snapshots and the oracle `76075b63…` are unchanged.
  - The pin comparison checks 1,086 recorded file hashes and 22 output streams. Every difference it lists is a separately compiled test file or a README that no code embeds.

**Limitations**
- I assumed the plain captures equal their `.gz` originals, because `zcat` was refused.
- I couldn't diff the dossier checksum manifest line by line, because that was refused too; the line counts match.
- The fixtures' match to base behaviour rests on the derivation and structure, not on a run of the base build.
- I didn't time the run, so it may have gone over 650 s.

The one remaining note is optional: a missing blank line before the comment at `core_continuations.rs:276`. The dd4 Opus report stays a historical NEEDS_CHANGES record. Nothing here approves V1, parent, native or event-authentication work. All ten `/4` gates stay unperformed and every U/M/MC/SP/G exit stays open.

I allow the separate metadata-only publication commit of these reports. It permits no source changes and no qualification upgrades.

Files are in `/home/charl/Moriarty/.moriarty-dev/runtime/core-continuations-20261015/`:
- OPUS-CLOSURE.md
- OPUS-CLOSURE-RESULT.md
