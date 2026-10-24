# Selected source and receipt reference

The new K entry is `NIWindowObservation.niRunWindow(NIJSON, NGProgram, Int)`. The received DATA root is `niWindowObserved(NIJSON, NGTape)`. NGTape fields are `initial`, `after_each_event`, `final`; the observer stops at terminal control without fabricated stutter. Selected cut4 comes from the exact public contract.

The exact receiver API is `render_source11(source_document, observation_cut, *, budget=DEFAULT_BUDGET)` and `decode_source11(source_document, raw_kprint_text, *, budget=DEFAULT_BUDGET)`. Source and test bytes are pinned in `SOURCE-IDENTITY-AND-REUSE.json`; the tested source file is 100099 bytes, SHA256 b89ddb216310ef3289e41839af4958affb10834c0b2bc1b74d2479cffd5482cb.

`actual-pure-tests/` contains one command/result and two full streams. Each `actual-windows/<case>/<phase>/` contains an exact command/result and stdout/stderr. The process records include actual exit, timing, timeout and group/token cleanup observations. Existing F03 limits were 30-second children, 3-second sweep, core 0, 512 MiB per-file fsize and JVM 64..512 MiB heap / 32 MiB stack / 2 processors, with no aggregate CPU/RSS/disk guarantee.

The source-specific compiled 211 registry, compiled 15, raw whole ASTs, decoded observations and independent expected bodies remain original archive artifacts, represented here only by selected provenance pins and Root's comparison receipt. No wrapper equality, file hash or test count replaces their semantics.

The observer also exists byte-for-byte in the older nominal-full-state subtree. Its new adjacent Source10/source placement is needed by the exact new wrapper’s relative requires path; the four unchanged adjacent K dependencies are reused rather than copied.
