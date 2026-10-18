# Ordinary Core/1 compiled artifact publication (PROPOSED)

Status: unadopted normal-positive wire proposal, not implementation or runtime evidence. Base 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb / tree 2f85c0862dffecc57edd26b558c8c0ce0aa3a35f. This document proposes observable metadata and complete comparisons; it does not qualify any proof/financial/ledger predicate.

The existing public path is `mori compile FILE --target TARGET --out OUT` with native enabled, through `source_projection::compile_reported`, production `compile`, pretty serde serialization and the existing publisher. Current transport checks already compare complete publication bytes to an Artifact returned by the same compiler. The next independent comparison must replace that shared oracle for its claimed fixtures; it must not add another DTO transport test or call the compiler to manufacture expected values.

For the finite ordinary unsigned Core/1 scope, a complete expected artifact SHALL be frozen independently before the actual consumer. It SHALL include every top-level field in the public Artifact: `format`, `abi`, `relation`, `authorization_mode`, `program_hash`, `semantics`, `target`, `circuits`, `resources`, `gates`. Every nested circuit field, exact IR string, complete ordered public ABI, effect/frame descriptors, null/absence distinction, resource count and target member SHALL be retained. A matching selected projection or an internal typed reopen alone SHALL NOT count as this full expectation. Exact manifest UTF8 bytes SHALL have no trailing newline; exact numbered circuit files SHALL equal the complete corresponding IR strings.

First new normative wire premise proposed for review: `/gates/*/predicate`. For this finite normal compile scope `gates` SHALL be exactly, in order:

1. `native-proof-generation` with status `unperformed`.
2. `circuit-cost-and-k-measurement` with status `unperformed`.
3. `fraud-proof-cost-and-k-measurement` with status `unperformed`.
4. `authenticated-ledger-frame-and-phase-correspondence` with status `unperformed`.
5. `signed-intent-and-predecessor-history-verification` with status `unperformed`.
6. `live-ledger-deployment` with status `unperformed`.

These are report labels, not a security classification or execution permission. A normal compile reports these obligations without upgrading any predicate to performed. The proposal preserves the current producer list; order/status requirements must be adopted or revised by the applicable reviewers before independent goldens are frozen. It does not change decoder acceptance or author rejection tests.

The complete positive expectation contract also fixes `moriarty-zkir-artifact/3`, `moriarty-public-abi/3`, `moriarty-zkir-relation/3`, unsigned mode and `moriarty-core/1`. This scope omits Target.signed_policy; public binding constant/asset and static effect Option members remain present with explicit null where absent. Normal unmeasured resources retain present-null circuit_k/fraud_max_k and cost_measured=false. Exact positive fixtures must independently freeze the complete Program/canonical bytes, hashes/context preimages, exact instruction/ABI/effect layouts and all full-wire byte counts; none is supplied by compiler output, reference execution output or this prose.

Initial specimen pair for preparation: PublicationFlag (Bool state/input, capture, state write and event) and PublicationPair (two UInt states, captured old value, ordered writes and event). Their source is only statically proposed; parse/compile/publication remain unrun. Full expected JSON/IR files are a separate custodian prerequisite. Until those bytes and their premises are frozen, no exact full-artifact result exists and the comparison cannot be scored.

This is a finite nonsecurity publication slice. Signed intention, financial relation enforcement, the admitted grammar generality argument, native verification and complete ledger effect readback remain separate U2 requirements. Core/2, hostile/refusal semantics, proof/VM/model/k/key/SRS/ledger/Preview/Fenrir qualification, C4/k17 and all U0-U7/M/MC/SP/G exits remain unchanged. Existing consumed allocations cannot be reused.
