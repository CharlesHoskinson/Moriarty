# R405 — Designated Security Test-Delta Source Author Delivery (`test_depinfo_retained_audit.py`)

## 1. Identity, Role Change & Coverage Qualification

- **Conversation ID (`CID`)**: `3b833d1d-96eb-4d82-a33a-abc28e7d3533` (original cwd `/tmp/moriarty-r304-fresh-independent-disposition-_kt12zfw`)
- **Active Model / Route**: `gemini-4-argon-high` (`HIGH` thinking) under standing observed `claude-opus-5-5` (`HIGH`) outage fallback
- **Explicit Role in R405**: **Designated Security TEST-DELTA SOURCE AUTHOR only** for [`test_depinfo_retained_audit.py`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/404-native06-parser-only-test-independent-review/source/test_depinfo_retained_audit.py) (`10430` B, `sha256: 8c1c9cc1b68c1b5f9f266e42fa9f6e1a92fdeaf52c168f56a99930287c0e273b`).
  - Remains **non-author** of candidate production guard [`bounded_sdk_compile.py`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/398-native06-retained-audit-owner-candidate/afterimages/bounded_sdk_compile.py) (`79818` B, `sha256: de3cb754f3483e5100faf1a7313ae08e5dc2ac4dae27ee1d79a97910f4380a53`), which remains uninstalled and unexecuted as a native runner.
  - Because `3b833d1d-96eb-4d82-a33a-abc28e7d3533` authors this test delta, this response grants **no independent approval** of the test delta; Root will route the materialized test delta for independent non-author review.
- **Tools Invoked**: **ZERO (`0` tool calls)**.
- **Readable Coverage & Failure Diagnosis**:
  - Read 100% of the inline failed tuple anchor, `depinfo_path` definition, and R404 actual execution receipt (`actualExit: 1`, `0.07105704100104049` s, `stdout` `281696f9...`, `stderr` `ace4ddbf...`).
  - Both positive methods (`test_parse_depinfo_resolves_relative_and_crate_contained_parent_segments` and `test_parse_depinfo_preserves_make_continuations_and_escaped_filename_tokens`) passed (`ok`).
  - In `test_parse_depinfo_rejects_root_escapes_invalid_parents_and_cross_crate_traversals`, the subtest `'regular file traversed as parent directory before ..'` had 4 path components (`crates/mori-backend/src/lib.rs`) followed by 5 `../` segments (`../../../../../`), causing `depinfo_path` to underflow `stack` on the 5th `..` (`Refusal('non-normalized dep-info path')`) before reaching the `for d in ascended:` filesystem check. Retaining the 5-`../` case as a lexical-underflow subtest and adding a 4-`../` case (`../../../../`) exercises `d = self.lib_rs` (`d.is_dir() == False`) and raises the exact `Refusal(f'dep-info parent directory identity: {self.lib_rs}')`.

---

## 2. Single Literal JSON Edit for `test_depinfo_retained_audit.py`

```json
{
  "old": "            (\n                'regular file traversed as parent directory before ..',\n                (\n                    f'{self.out_rlib}: '\n                    'crates/mori-backend/src/lib.rs/'\n                    '../../../../../models/sherlock/machine-circuit/expected.json\\n'\n                ),\n                f'dep-info parent directory identity: {self.lib_rs}',\n            ),",
  "new": "            (\n                'five .. segments after four components underflow lexical stack',\n                (\n                    f'{self.out_rlib}: '\n                    'crates/mori-backend/src/lib.rs/'\n                    '../../../../../models/sherlock/machine-circuit/expected.json\\n'\n                ),\n                'non-normalized dep-info path',\n            ),\n            (\n                'regular file traversed as parent directory before ..',\n                (\n                    f'{self.out_rlib}: '\n                    'crates/mori-backend/src/lib.rs/'\n                    '../../../../models/sherlock/machine-circuit/expected.json\\n'\n                ),\n                f'dep-info parent directory identity: {self.lib_rs}',\n            ),"
}
```

---

## 3. Proposed Future Single-Method Ordinary Operation (Pending Independent Non-Author Review)

- **Proposed Selector Only (`count == 1`)**:
  `test_depinfo_retained_audit.DepinfoNormalizationTests.test_parse_depinfo_rejects_root_escapes_invalid_parents_and_cross_crate_traversals`
  *(The two passing positive methods in `DepinfoNormalizationTests` are already verified in `actual-sdkfree-parser-three-01` and are not selected for rerun.)*
- **Proposed Execution Envelope (Unchanged from R404)**:
  - Isolated 2-file SDK-free suite (`bounded_sdk_compile.py` `de3cb754...` + materialized `test_depinfo_retained_audit.py`).
  - `uid=1000`, `gid=1000`, clean environment (`PATH=/usr/bin:/bin`, `LANG=C.UTF-8`, `HOME=<suite_dir>`, `TMPDIR=<suite_dir>`).
  - `/usr/bin/timeout --signal=KILL 30s /usr/bin/prlimit --cpu=25:25 --as=1073741824:1073741824 --fsize=1048576:1048576 --core=0:0 -- /usr/bin/python3 -I -B -c ...` with `131072` B combined stream cap and `assert suite.countTestCases() == 1`.
- **Standing Boundaries Preserved**:
  - `native_authority = 0`; `R2 = UNSATISFIED`; `C4 = OPEN`; **Amendment 10 remains STOPPED**.
  - R399 `REVISE` items on `launch`/`retained_f_audit`/`sandbox_args` and the separate `output_check` / retained-audit lifecycle test cohorts remain pending.

