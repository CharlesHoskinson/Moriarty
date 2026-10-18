# Draft source and typed interface

This document is part of an unadopted design. No parser, implementation, public CLI command or source profile is added to main by these files.

The proposed join declaration is a separate source profile. Its ordinary child sources use existing `moriarty/2`. Module paths are literal keys into the explicit `(path, source_text)` map supplied to `join::load`; loading performs no filesystem or network lookup. The source map is not a discovery mechanism.

```ebnf
join_source = 'language "moriarty-join-draft/1";',
              'join', identifier, '{',
              module_decl, module_decl, module_decl,
              parent_decl, parent_decl,
              'availability all_observed;',
              'success all_complete;',
              'frame disjoint_union;',
              'successor', selector, ';', '}';
module_decl = 'module', identifier, 'from', string_literal, ';';
parent_decl = 'parent', identifier, '=', selector, ';';
selector = identifier, '.', identifier;
```

This finite profile has exactly three module declarations: two distinct parents and one distinct successor. Identifiers/string literals follow existing lexical conventions; action arguments remain in explicit Scenarios and successor inputs, not selector syntax. Declaration order fixes parent slots and module display order. The two parents each select exactly one existing action containing one Await. The successor selects one action; the example has its own genuine Await. The join introduces no Await or ExprId into any child Program. Static names, domain compatibility, unsupported-source diagnostics and admission/error ordering require separate implementation and owner review; this ordinary proposal does not define refusal precedence.

`API-TYPES-PROPOSAL.rs` describes the output DTOs and abstract callable signatures without function bodies. The future module name is `mori_core::stage::join_draft`. Its proposed concrete private types are `JoinProgram`, `PendingParent`, `JoinContinuation` and `JoinCompletion`; they correspond to the abstract trait's `Program`, `PendingParent`, `Continuation` and `Completion`. Its concrete `JoinOutcome` is `Outcome<JoinContinuation, JoinCompletion>`. The trait is a design notation, not a requirement to introduce a runtime trait or generic framework. Future public free functions have those concrete signatures. A nondefault `core2-join-draft` feature is proposed; the feature/name is also a review choice.

Each normal call consumes the actual owned control object and returns a normal outcome. `prepare_parent` is the only proposed creator of a `PendingParent`: it records one genuine first `stage::start` returning Waiting, including the full actual prefix. A report or bare normal Continuation cannot construct it. The proposed module may retain opaque local program references and private receipts internally; output DTOs cannot be used as an alternate input constructor. No new `Deserialize` is proposed for opaque join types or output views.

`load`/`plan_report` expose each declared module alias and literal source_key, plus all three complete existing Programs and filled StagePlans. A source_key is a supplied map key, not a new cryptographic identity. Nested module arenas, ExprIds, statement indexes, value encodings and stage identities are retained verbatim. The original source text is supplied by the caller and is not promoted into a new digest domain. Existing program/plan hashes are reported with their existing local meaning. This design adds a full versioned control record rather than a new cryptographic join hash.

`parent_report` returns the complete normal start envelope. `outcome_report` returns the complete `JoinReport` defined in the DTO file, including unchanged whole child envelopes in `imported_parent_reports` and `after_attach.core_report`. `view` and `completion_view` expose the same full current typed view. `parent_continuation` borrows only an actual currently pending normal child Continuation, permitting ordinary linkage assertions; it does not manufacture one from an expected report.

New report fields are mandatory, including `null` Option fields and empty history/residual arrays. New enums use the snake_case spellings shown by their derives. `Invocation.ordinal` is a UInt encoded by the existing UInt convention; Attach is ordinal0 and each subsequent actual wrapper operation appends the next ordinal. Ordinals describe join-call history only. They do not order the two independently imported starts. `after_attach[0]` is Attach with parent:null and core_report:null. A parent resume entry names its producing parent; enter and successor resume have parent:null and carry the complete actual successor stage report.

All output root fields and boundary choices are listed in DESIGN-PROPOSAL.md. The nested normal envelopes retain their own format and qualification labels. Every new join report also repeats qualification:`local-reference-stage`, authentication:`none`, consumption:`untracked`. These labels remain visible even though join objects are opaque and owned in-process. No persistence, signed policy, authenticated documentary check, global ledger transition or consumption proof is inferred. New join reports have no adopted stdout/byte ABI; proposed tests require complete JSON Value equality, not a new raw serialization contract.

The complete input examples are deliberately ordinary local observations. `document` and `acceptance` are source labels, and their Bool values are supplied by the caller. No document is parsed and no signature/proof is verified. `retainedDocumentDuty` deliberately survives both-parent completion and successor completion, demonstrating that local control completion does not delete residual obligations.
