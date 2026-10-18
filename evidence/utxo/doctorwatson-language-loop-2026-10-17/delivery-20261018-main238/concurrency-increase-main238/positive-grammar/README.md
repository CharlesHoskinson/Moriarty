# Positive grammar applicability at 71c22f8

Read-only source findings for the designated Sol6.1 HIGH author. My exact model and effort are not exposed, so this follow-up authors no patch or tests. `APPLICABILITY.json` contains exact edit locations, current consumer citations, the preserved 27-constructor inventory and still-open bounded-corpus gaps. `SOURCE-PINS.json` binds 44 source files to the requested commit.

All three prior proposal paths remain absent. Of 41 previous source pins, 39 are identical; only header diagnostics in the parser and appended diagnostic tests changed. Prior positive fixtures and test bodies still call the actual exported Core/1 and Core/2 consumers. This supports source applicability only; no execution occurred.

The frozen expectation stays at its original path with SHA256 `34ad90300bb91e5860de6efe324a360cd9243a28579488a14ac938d7f4cd9202`. It remains ordinary-author corroboration, not an independent oracle. No outputs were used to revise it.

After designated authorship and root adoption, the inspected bounded command is `cargo test --locked -j1 -p mori-core --no-default-features --test positive_grammar_edges -- --test-threads=1`. It has not run. All original gaps remain open; N3 staged liability reads and greater-than coverage are still specified only. Newly added repository tests were not exhaustively classified, so bounded corpus gaps are not whole-repository absence claims.
