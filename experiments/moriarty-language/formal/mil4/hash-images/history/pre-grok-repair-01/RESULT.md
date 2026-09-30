# W-D2H bounded design result

**Status: proposed / specified-only, 2026-09-30.** This is an author design
result, not an approval, adoption, implementation or authentication result.
W-D2/W-D3 and B01–B17 remain open.

## Delivered scope

Only [SPEC](SPEC.md), [DECISION-MATRIX](DECISION-MATRIX.md) and this RESULT are
created in `formal/mil4/hash-images/`. W-D2F/W-D2G, source modules, fixtures,
protected records and historical evidence are unchanged by this sprint.

Repository inspection covers current W-D2F FIELD-MAP/PLAN, W-D2G SPEC,
Source/6 frontend/grammar/contract, Core/5 preparer/S0 contract, the Source
wrapper and wire/3 specification. The development skill was loaded and guarded
status inspected. Status still reports unresolved product execution/admission
gaps; no registered execution was dispatched. AGENTS' historical September10
ACTIVE-ROUTING/assignment paths are absent in this worktree; no execution or
model-routing substitution is inferred from that absence.

## Findings and recommendation

Repository observation: Source/6 is a builtin-selection stage proposal, not a
user-defined program body. Its embedded Source/policy claims are opaque and
currently copied unchanged. Whole-document hashing without exclusions would
self-reference `source_hash`; no current producer resolves that issue.

Recommendation: select the canonical instance-scoped Source definition
projection, an exact raw three-module Core implementation package, and an
exact scoped policy that commits computed Source/Core digests. SPEC gives
included/excluded bytes, fixed purposes, primitive widths, package roles,
static literals, action variants and acyclic generation order. A grammar-located
two-token document projection is the independently specified alternative.

Inference: this split makes Source presentation changes harmless to the
definition while preserving exact implementation identity and explicit
policy terms. It does not authenticate any artifact or prove that committed
source bytes were executed. B06 still needs independently verified loaded
artifact/toolchain/lowering correspondence, and B16 must replace the local
lowerer's incomplete replay projection with an adopted authenticated transport.

SPEC preserves W-D2F's tag order:8/11/12 compare authenticated artifact hashes,
while retained asset/scale/policy terms compare at their own later fields.
This avoids using an early candidate hash to move a cap/endpoint mismatch
ahead of its specified anchor. Upgrade consequences include fresh
signature evidence for changed wire claims, conservative Core hash changes
even on package comments, explicit suite/version binding and no implicit
state/replay/registry reset. Wire/3's existing content-digest/signature-message
boundary is unchanged.

The decision matrix defines2 positive construction oracles, presentation
invariance and18 hostile/invariance design cases, including self-reference,
selected-action mismatch, exact-code mismatch, policy/context mismatch,
zero-fee recipient, lowerer substitution, incomplete history and cross-purpose
digest collision. These are expected predicates, not frozen digest vectors
or executed outcomes.

## Evidence limits and next bounded work

Implementation changes: **0**. New encoders/consumers/verifiers: **0**.
Tests added/run: **0**. Computed Source/Core/policy digest vectors: **0**.
Authentication/correspondence proofs/native evidence/ledger submissions: **0**.
Independent audits/decision votes: **0**. No gate is closed.

The author performed a text consistency review of field inclusion, dependency
cycles, tag ordering, selector roles and claim-versus-computation language.
This is not an independent audit or an executed design-oracle result.

Next bounded work: independently review the proposed suite versus the document
alternative and the exact-code-versus-abstract-Core choice. Only after adoption
freeze independent image bytes/digests and hostile expectations, then implement
the smallest image encoder/comparator with its honest content-only scope.
Full authenticated consumption additionally needs actual B01–B17 definitions
and evidence at their existing gates; B17 remains a separate ledger result.
