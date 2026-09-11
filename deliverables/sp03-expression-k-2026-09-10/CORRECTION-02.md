# K String transport correction02

Candidate01 is superseded before independent source/resource reviews. Its 89 pinned files were verified and copied unchanged; preservation-01.json resolves every original path and digest. Original proposal, manifest, README and logs remain historical.

The original codec used JSON quoting for K String tokens. JSON emits unsupported K `\b` and paired surrogate escapes, and its decoder rejects K `\x08` and `\U0001f600`. The retained root structural probe reproduces these failures; it is not captured K execution. Pinned domains.md lines1680–1710 defines the token grammar. Read-only javap of installed StringUtil confirms codepoint-based enquoteKString/getUnicodeEscape (retained bytecode lines719 onward) and surrogate rejection in unquoteKString (lines502 onward). No compiler or backend was invoked.

The codec now quotes and decodes the distinct K token grammar over Unicode scalars. Outer canonical JSON remains unchanged. Independent literal token tests cover control bytes, BMP, astral and maximum scalar; malformed escapes, surrogate pairs and out-of-domain values reject. The failing tests and corrected output are retained separately. Six new LitText fixtures retain independent full expected results for backspace, astral, NUL, formfeed, BMP and maximum scalar.

Validation: 38 Python tests and 318 published-reference/fixture tests pass, including all113 complete expected K fixtures. Python syntax checks and git diff --check pass. No K compile, kast, krun, proof or operational dispatch occurred. The builtin documentation includes a historical backend Unicode warning; actual installed backend scalar correspondence remains a required result of the proposed traces, not established by these source tests.

Proposal02 keeps one compile and the same memory/CPU/cleanup restrictions. Six added traces increase the ceiling from107 to113, aggregate from2392 to2512 seconds (180+113*20+72), and outer envelope from2428 to2548 seconds (30+2512+5+1). No attempts have been consumed or reset. Fresh independent source/resource approvals remain required.
