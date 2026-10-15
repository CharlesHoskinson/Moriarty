# Independent oracle/source check

**No normal functional source/oracle mismatches found.** Checked 2026-10-15T00:41:11.506298+00:00.

Candidate [core_test_report.rs](/home/charl/Moriarty/.worktrees/core-test-report-20261015/crates/mori/tests/core_test_report.rs) SHA-256 `0c10e7a6efe21b7f89b689722c2bb543c211cdab4102ebf8c87b34a169d62326` matches the requested exact source. The independent [expectation freeze](INDEPENDENT-EXPECTATIONS.json) remains unchanged at SHA-256 `3911ef8990bbcdf067f641bcae032f48828b2035ddae55f8687969ecc0cb2395`. This is a source-only check, with no product/build execution or security review.

The normal source bytes match exactly. Literal pre/post, ordered Transfer effect, preserved sixteen observer balances42 and duty11, both trace steps and expression indices match the frozen manual execution. Small compatibility pins exactly28,093 bytes and independent SHA-256 `b57043f1d8fd55e80cfde729ef8e85f78225ff29129bc8da6696c7f1163e5118`. Large256 invokes the actual public CLI and asserts exit0,256/0 summary, >1MiB stdout and the full ordered report. Mixed256 retains every full successful actual execution, with mismatches only at0/127/255, all names000..255, exit1 and253/3 summary.

FeePool filters the same87 accepted cases in original order, uses public source/scenario paths, and asserts complete expected poststate/ordered effects, trace presence, count/status/flags and >1MiB stdout. All164 original public source/scenario input hashes and the162-case corpus hash still match the independent freeze. The corpus contains87 accepted/75 rejected; this test selects accepted cases only.

Limitations: source agreement does not establish compilation or execution. The FeePool test reads the current tracked corpus without embedding the complete frozen hash inventory; identity was checked for this candidate. It asserts trace presence, while the simple fixture asserts complete independent trace contents. Logical work6/case remains a manual prediction because this command emits no work count. Security/resource review and exact command clearance remain with Opus. No expected values were tuned, and the original oracle/output artifacts were not modified.

Machine-readable check: [ORACLE-SOURCE-CHECK.json](ORACLE-SOURCE-CHECK.json), SHA-256 `5b02c2a067c6d007129f6ef918c97dabb7d7d418f54b22c47ffd0723ad25e5bf`.
