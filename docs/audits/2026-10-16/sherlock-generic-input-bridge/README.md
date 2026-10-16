# Sherlock generic input bridge: metadata-only result

On 2026-10-16, one SDK-free Rust diagnostic compiled an exact 21-file closure containing the four [host client modules](../../../../models/sherlock/host-client-source-20261016/README.md), current public A type declarations and the separately prepared owner/type/family sources. `rustc 1.99.0` emitted dependency information and library metadata only, using edition 2024. Root independently verified the exact 21-file dependency set, all 28 bound members unchanged, and matching original/snapshot source hashes.

| Observation | Recorded result |
| --- | --- |
| Compiler version probe | Exit 0; 0.068 seconds |
| Metadata diagnostic | Exit 0; 1.381 seconds |
| Compiler diagnostics | Six warnings; zero errors |
| Source function, parser, bridge or engine execution | None |
| Tests | None built or run |
| Metadata linking/loading | None |

The unchanged `source_input_bridge.rs` therefore type- and borrow-checks against the supplied A/B declarations in this diagnostic crate. Its preceding fresh Muse source review approved the complete dedicated-sort constructor/field mapping within a source-only scope. Neither result qualifies another composition or registers this API. The existing cfg(test)-only/no-Stage-2 seam remains unresolved.

The four public modules are `execution_capture.rs`, `typed_inputs.rs`, `observation_json.rs` and `source_input_bridge.rs`. The remaining required owner/type/family closure and diagnostic root are outside this directory. This publication is not a standalone build, publication of the engine or source adoption.

Separate Opus source reviews found two unresolved defects in the engine's invariant diagnostic; correction is in progress. This metadata compile does not resolve those findings or establish engine correctness. They are source-review findings, not compiler errors. No application outcome, event, prefix, invariant preservation or A/B fuel correspondence was observed.

Retained evidence identities (SHA-256):

- Bridge source: `87be4cc95d27c74c806363f862ff9c032160af18274ba81bb03f7f8a150a9441`.
- Current A declarations: `4b4bb364a0c8182e117a9c36efb5d91cb7e405f653a6d3870f54dc03eefa317f`.
- Exact source-closure manifest: `0a8d7b6418a49d5dcc1ff00bd34845b0d7185b01ab37e61ad07415869915eea2`.
- Diagnostic receipt: `5bc90aed4b058a03e115af4f56313b2074b44c49785403d6ef3e7ffff41a2e94`.
- Dependency information: `c4622271691ab140477029114365628fa8b29adad0ca0e71bea1fe4b7495ab00`.
- Metadata artifact: `b0eb7aa0b5f72e3b75ba76eab255177bd256441f2a70659b191bf0134191c995`.

Full receipts and source-review context remain retained separately; runtime paths and provider streams are not published here. This local diagnostic makes no hermetic-toolchain or sandbox-isolation claim. It performs no SDK, model, circuit, proof, native or target work and closes no financial, product or roadmap exit. See the [facts, reasons and open questions](../../../../wiki-llm/sherlock-implementation-journal/GENERIC-INPUT-BRIDGE-20261016.md).
