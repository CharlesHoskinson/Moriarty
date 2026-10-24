# Sherlock A102 literal source bridge

This source wraps the existing enabled Literal transition fragment with a complete logical statement: program tables, pre/post images, envelope and profile/layout/relation identity. It supplies local native prove/independent-verify entry points and a separate ledger-call adapter. Local mode exposes all lanes; ledger mode commits them and requires an off-chain opening.

The post image is a Step-mode Return containing a Constant-space reference, with arenas preserved. It is not a canonical-boundary image. The ledger adapter compares the full caller-expected raw intent and returns local call verification; it supplies no transaction, inclusion, finality, fee, state or settlement receipt.

Two fresh Opus 5.5 HIGH security reviews and one fresh ordinary source review approve corrected candidate02 within explicit limits. Root parser checks passed for three Rust files, and Cargo.lock parses as TOML. No Rust typecheck, native k/keygen/proof/verification, ledger acceptance or stagenet transaction has run. See [the factual review result](SOURCE-REVIEW-RESULT.json) and [the transition mapping](../literal-step/MAPPING.md).

The new dependency and declared source closure change existing build/keyset/descriptor identities. Old keys and descriptors are not inherited; no regeneration has run. The single lock edge is hand-authored and still needs Cargo confirmation. Source digests remain caller assertions. StepInv, F4, G4, uiValid, the Step-mode producer, canonical projection, complete-rule coverage and target interoperability remain open. Existing limits and native authority are unchanged.

The reviewed implementation is in [the backend wrapper](../../../crates/mori-backend/src/sherlock_machine/literal_bridge.rs) and [the ledger adapter](../../../crates/mori-ledger/src/sherlock_literal.rs). Full private reviews, provider logs, credentials and independent expected data are excluded.
