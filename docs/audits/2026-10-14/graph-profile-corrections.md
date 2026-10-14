# Graph profile corrections

Base commit: `cb8280ef54082853dd8bb6c6a52050328d0fc6fa`

The prior graph mislabeled the Rust `mori-core` crate as Core/5. Its actual identities are `moriarty/1` source and `moriarty-core/1` semantics. Core/5 belongs to the separate experimental TypeScript preparation adapter imported by the Node beta bridge. The two profile families must remain separate in the graph. The original Luna reports are unchanged.

| Prior graph item | Corrected label | Evidence |
|---|---|---|
| Rust Core/5 parser, IR, evaluator | Rust Moriarty/1 parser/IR with moriarty-core/1 semantics | `crates/mori-core/src/ir.rs:4-5`; `crates/mori-core/src/lib.rs:13,15` |
| Rust Core/5 source parser | Rust Moriarty/1 source parser | `crates/mori-core/src/parser.rs:1`; `crates/mori-core/src/lib.rs:13` |
| Rust Core/5 validation/runtime | Rust moriarty-core/1 validation, simulation and replay | `crates/mori-core/src/ir.rs:5`; `crates/mori-core/src/lib.rs:15` |
| Beta Source/6 preparation | Experimental TypeScript Source/6 adapter to experimental TypeScript Core/5 local S0 preparation | `experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts:3,5`; `mil4-s0-source-v6.ts:1` |

The Rust CLI loads source with `mori_core::parse` through `core_io::program` (`crates/mori/src/commands/core_io.rs:240-243`). The Node bridge has separate imports for its TypeScript beta frontend and experimental Source/6 modules (`packages/moriarty-beta/src/bridge.ts:3-6`).

## Experimental source import closure

The Node beta bridge reaches exactly three local modules under `experiments/moriarty-language/src/**` through its two direct experimental imports: the Source/6 frontend, Source/6 preparation adapter, and shared MIL/4 S0 Core/5 implementation. The adapter imports the frontend and core; the frontend imports the core; the core has no local imports. The machine-readable file records paths, SHA-256 values and edges. This is static source inspection only.

No test, build, evaluator, native, cache, or security work was performed.
