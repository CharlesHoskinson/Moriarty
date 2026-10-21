# Current canonical source-cut compile

The canonical Sherlock normal Core source closure at `f73784fbd5c0c22a3f0d16ae31769948b6729674`, with exactly the three reviewed Core1 publication files overlaid, compiled successfully once in 7.157268001 seconds. The actual dep-info contains 121 source inputs: 118 byte-equal canonical inputs and three exact reviewed overlays. The canonical additive nominal runtime handoff is included.

This is a changed-input check, separate from the dated normal05 build. It uses the retained SDK-free direct-rustc normal-library command and qualified dependencies, with no `cfg(test)`, replacement modules, Cargo, SDK or native operation. The unchanged limits were 30 seconds wall, 25 seconds CPU, 2 GiB address space, 64 MiB output-file size, zero core files and 131072 bytes per captured stream. Full stdout is empty; full stderr contains 34 warnings and no errors. All 373 source/tool pins and the three complete library-directory inventories matched before and after. The normal rlib and dep-info were pinned and retained read-only locally; compiled payloads are excluded here.

The first caller preparation stopped at a missing result parent before any subprocess or compiler launch. Its transcript-only failure is retained; no original stderr file is claimed. After creating that output parent, the compiler child was launched exactly once.

This adds compilation evidence for this exact source cut. It does not establish runtime correctness, whole Cargo-tree compatibility, a full CLI build, eligible145 export, a complete independent image oracle, native acceptance or EARS closure. Existing source/runtime reviews retain their scopes. The original eleven-member publication package and its historical failures remain unchanged.

The archive includes the complete actual command, result, streams, dep-info, pre/post checks and exact source/tool bindings. Source identity and result checks were repeated by the preparing agent; Root adoption remains a separate action.
