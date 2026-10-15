**Verdict for the whole child feature at `dd4e4298` (tree `f954cd00`): integrated NEEDS_CHANGES.** The production code has no defect. Two frozen hostile-test requirements are still not exercised by any test, so this is not an approval. Both fixes are test-only.

| Axis | Disposition |
|---|---|
| SOURCE | **NEEDS_CHANGES**: two required test gaps, F-1 and F-2 |
| ACTUAL-RESULTS | **APPROVE**: all 86 selected functions as captured |
| DOCS | **APPROVE** |
| INTEGRATED | **NEEDS_CHANGES** |

**What I verified myself**
- **No production change:** every crate `src` file is byte-identical to `7d6c3965`, and no Cargo or build file changed. The only code changes are three test files, and I read all of their changes.
- **Hashes:** both absolute manifests pass `sha256sum -c` (514 and 85 entries). I could not run a full line-by-line diff of the absolute dossier manifest against `SHA256SUMS` because the harness refused it. Line count, path prefix and the 12 new capture entries match. The five design documents and `FROZEN-DESIGN.json` match their pins.
- **Legacy projection:** Core and backend show no difference between `95df` and `eb2577`. For the 64 per-file hashes I relied on root's recorded projection. I did not read, stat or hash the baseline executable.
- **New captures:** I read all three in full: 10, 11 and 3 passes, each exit 0 with no native actions requested. I recomputed every stdout and stderr hash and each matches its `result.json`.
- **Retained captures:** the pin comparison covers 990 pins and 22 streams across 11 captures. None of the disclosed differences reaches a selected test's call path.
- **Counts:** 86 = 24 last-round + 62 retained. No rerun is summed and the three original failures are kept but not counted.
- **Quint:** accepted only as finite normal-example checks (11 tests), with no correspondence to the Rust code.

**Earlier required findings, all closed**
- **A-C1/H19:** the new CLI test shows `compile-call` on a `/2` program is refused before the deployment file is read and before any output exists. The ledger tests pin the exact `core1-required` refusal, using only the public curve generator. Ledger entry points no test reaches are safe by construction, either sealed off or refused by parsing first.
- **T4:** the same item as H19, closed with it and not counted twice.
- **A-C2/H16:** the three CLI refusals are pinned exactly.
- **A-D1:** both stored-cap paragraphs match `cost.rs:184-216`, and the cap is outside both digests.
- **B-T3/H20:** the new test decodes an honest `/4` artifact as a positive control. On the byte path the `/3` decoder refuses at `format` first; on the key-sorted `Value` path it refuses at `abi` first.

**Remaining required findings (Low)**
- **F-1 (H18):** no test shows that `mori compile` with a `/2` program and a signed Target leaves no `--out` directory. It holds by construction: `compile_staged` writes nothing, and `compile.rs:20-25` compiles before creating the directory. But the frozen text (`DESIGN-OPUS.md:232,295`) requires a test. The earlier A review assigned this part to B and the B review assigned it to A, so nobody tested it.
- **F-2 (H15):** for a `/1` program that uses `await` and `when` as names, only successful parsing is checked (`stage_static_cost.rs:505-507`). The required "unchanged parse and Execution bytes" comparison against frozen literals is missing (`DESIGN-OPUS.md:81,292`).

There are also two optional notes (O-1, O-2). The report has a complete H1–H28 table: everything other than F-1 and F-2 is covered, and the entries that rely on construction or earlier reviews are labelled as such.

**Publication.** My scope allows a metadata-only commit that publishes this report and its receipts verbatim, recorded as NEEDS_CHANGES, with the reviewed commit, tree and source hashes bound outside the frozen files. It would claim no approval or later-stage qualification, and I have not done it.

I ran nothing in the product, wrote only the report file, and Authority stays 0. I went over the 650 s target to finish the required reads.

**Identity:** Claude Opus 5.5 (`claude-opus-5-5`), high effort, fresh and non-author.

Report: `/home/charl/Moriarty/.moriarty-dev/runtime/core-continuations-20261015/OPUS-FINAL-INTEGRATION.md`
