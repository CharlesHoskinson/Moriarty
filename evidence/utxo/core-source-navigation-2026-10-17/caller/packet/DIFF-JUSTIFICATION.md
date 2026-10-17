# Citation repair justification

This new candidate applies four source-attribution repairs to the preserved prior graph. It does not alter the prior output directory.

| Finding | Prior graph issue | Repair in v2 | Packet evidence |
|---|---|---|---|
| F1 | `source:N01-lib.rs -> draft-bridge-module` cited lines 12/13, which declare `sherlock_draft`, not the image bridge. | Module node cites declaration line 23; edge aggregates cfg/path/module lines 21–23. Neighboring test-module declarations use their own correct lines. | N01 lines 21–23: `#[cfg(test)]`, `#[path = "sherlock/draft_image_bridge.rs"]`, `mod sherlock_draft_image;`. |
| F2 | `call-context-new -> helper-profile` cited N07 line 157, which is outside `CallContext::new`. | Added `CallContext::proposed_profile` node and moved the helper-profile edge to that method. | N07 line 157: `pub(super) fn proposed_profile(&self) -> &'static str { helper::PROPOSED_PROFILE }`. |
| F3 | `core-reference-profile` visibility was incorrectly marked cfg(test). | Corrected it to public through the public Sherlock module and unconditional `reference_types` glob re-export; added module/re-export nodes and citations. | N01 line 8 `pub mod sherlock;`; N02 line 12 `mod reference_types;` and line 16 `pub use reference_types::*;`; N09 line 4 `pub const PROFILE`. |
| F4 | Helper-adapter and draft-source-test module path edges repeated the cfg/path line as the module declaration. | Aggregated distinct cfg, path, and declaration citations. | N02 lines 60/61; N01 lines 19/20 (and all adjacent test modules similarly corrected). |

The existing `TYPE_COMPATIBLE` relation remains separate from test caller edges. Actual test invocations added in the caller graph remain marked `test_calls`. No runtime behavior or security conclusion is inferred.
