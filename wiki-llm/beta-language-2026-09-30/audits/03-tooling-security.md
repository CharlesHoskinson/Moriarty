# Independent PL audit 3: tooling, architecture and security

Candidate: `BETA-DESIGN.md`, SHA-256 `6af4ca5356ed3fd58818b5717401d5b218b672b65769e342f708d4a4e0cd061a` (recomputed and matched `candidate-v1.sha256`). Baseline identified by candidate: `870f998b36ecda04622fa4274132e74902942d0b`.

Requested reviewer: GPT-6.1 Sol, medium effort, as supplied in delegation. This audit is written by the fresh Codex sub-agent assigned seat 3. The runtime has not exposed a returned model identifier or an effort receipt to this agent; the requested identity is not asserted as independently observed. No peer audit or convergence discussion was consulted.

Verdict: **conditional agreement with the architecture; changes required before freezing implementation contracts**. Findings below are design defects or missing specifications, not claims of observed beta implementation exploits. No production changes, executable beta tests, provider activation, deployment or native-proof experiment were performed.

## Evidence inspected

Repository observations: loaded `AGENTS.md` and the checked-in `plugins/moriarty-dev/skills/develop/SKILL.md`; ran guarded status. Historical loan/swap dispatch remains blocked by stale bindings and missing accounting/resource evidence; no pending transactions were listed. This read-only review did not dispatch that work.

Read the frozen candidate; toolchain/editor/AI/meta-tool memos (domain 05–08, with relevant detailed sections inspected); `docs/language/PROGRAMMER-FACING-MOCKUP-REQUIREMENTS-2026-09-30.md`; actual Source/6 lexical, parsing and lowering paths in `financial-agreement-source-v6-frontend.ts`; actual preparation wrapper `mil4-s0-source-v6.ts`; and relevant Core/5 preparation checks in `mil4-s0-core-v5.ts`. Protocol/version facts in memos are treated as their reported evidence; no fresh normative protocol or package-advisory acquisition occurred. Abstain on current dependency vulnerabilities, licensing and actual editor activation.

## Findings and exact repairs

### TS-01 — High: scenario JSON closure must exist before serialization

Design observation: the candidate calls scenarios bounded JSON and requires rejection of duplicate/extra inputs, but does not define a closed JSON transport schema, duplicate object-member handling, numeric spelling, nesting limit or exact optionality. The strict Source/6 parser validates the resulting fixed-order source; it cannot discover an unknown JSON field already discarded by a projection, or a duplicate member overwritten by ordinary `JSON.parse`.

Inference: passing through Source/6 is necessary but insufficient to enforce the advertised scenario closure. This is especially consequential for source intent overrides and fabricated authentication fields: silently ignoring them makes the tool appear to have checked a statement it never interpreted.

Repair: freeze `moriarty-local-scenario/1` with exact required/optional keys at every level; use decimal strings for financial integers; prohibit unknown members, duplicate decoded member names, noncanonical integers and invalid Unicode scalars. Specify one replay tag compatible with Source/6 rather than general history. Validate incoming raw JSON before any lossy object conversion, then copy approved fields into a closed internal representation. For object-input library APIs, accept only plain data properties and validate closure rather than invoking caller getters. Reject unknown `intent`, `signatureVerified` and policy fields rather than silently discarding them. A bound scenario cannot overwrite authored intent.

Discriminators: duplicate `work_spent` with differing values; `"head"` plus `"\u0068ead"`; unknown nested `authenticated: true`; missing spent counter; number `9007199254740993` instead of a decimal string; transfer balances with a fourth row; repayment obligation with an extra creditor alias. CLI and MCP must return named formation errors and no published effects. Repeated duplicate keys must reject even when both values agree.

### TS-02 — High: source limits do not bound server or expansion work

Design observation: numerical source/AST/depth caps exist, but scenarios and stdio frames are only described as bounded, expression/expansion counters have no limits, and the candidate does not commit to open-document, queue, output or aggregate-memory caps. Domain 06 and 07 recommend those additional controls; the frozen contract does not select them.

Inference: a stream of individually valid maximum-size edits or MCP previews can exceed aggregate resources. A huge `Content-Length` can consume resources before the language byte limit runs. Returning many origin maps can amplify small requests. Source/6 also caps its generated source at 65536 bytes, so a valid 65536-byte beta input need not have a valid generated transport.

Repair: publish separate numerical limits for header bytes, transport body bytes, scenario bytes/depth/collections, stored document bytes/count, queued/running jobs, expansion operations/generated Source/6 bytes, origins and serialized result bytes. Apply frame caps before buffering the body and scenario caps before recursive parsing. Use latest-version replacement for queued full-sync edits and bounded admission for MCP requests. Charge each expression edge and expansion step; memoize declaration values once. Document deterministic failure codes and no-partial-artifact behavior. Set a generated-source cap consistent with the existing Source/6 parser, or reject expansion distinctly when that cap is crossed. Do not weaken existing Source/6 limits invisibly.

Discriminators: declared frame length above cap without delivering a body; header with no terminator; escaped JSON whose transport is larger than decoded source; hundreds of documents or queued max-size changes; repeated constant reuse; maximum source producing an oversized Source/6 image. Assert bounded memory/work, named failure, continued handling of a subsequent valid request where protocol recovery is supported, and no stale candidate publication.

### TS-03 — High: UInt128 sugar and S0 signed bound range are different

Repository observation: Source/6 parses `gross_cap`, `fee_cap`, `net_floor`, obligation principal/accrued/outstanding with `S128 = 2^127 - 1`; Core/5 validates those signed bounds against the same maximum. Source/6 quantities, ordinary balance/counter fields and transfer amounts use UInt128 in other positions. The candidate promises checked UInt128 intermediates and rejects out-of-range atoms, without saying when `Qty` expressions are narrowed to S0's smaller admissible bound range.

Inference: an apparently supported, diagnostically clean intent can later fail Source/6 formation despite satisfying the advertised quantity range. The beta must not redefine the old S0 arithmetic contract or label this as a Core economic rejection.

Repair: define UInt128 value arithmetic separately from field-specific S0 range constraints. Validate each supported intent and obligation field against its actual Source/6 range during strict checking/expansion; name the failing field and its source origin. Preserve the full-language horizon independently. Inspection should list these operative limits. Avoid describing a UInt128 `Qty` as universally usable in every signed field.

Discriminators: use `atoms(asset: USD, value: 170141183460469231731687303715884105728)` as `gross_cap`; use the same value as a balance where otherwise permitted; test exactly `2^127 - 1` and `2^128 - 1`. Expect the field constraint to reject deterministically without changing the value or coercing it into signed arithmetic. Boundary tests must traverse beta → Source/6, rather than only the expression evaluator.

### TS-04 — Medium: shared analysis needs immutable version identity and a strict gate

Agreement: one text-based analysis service for CLI/LSP/MCP and tolerant lexical assistance separate from strict lowering is the correct decision. The candidate needs a concrete gate and snapshot contract to make that decision reviewable.

Repair: return distinct partial editor analysis and strict eligible analysis states. Lowering should recheck exact input bytes or accept an opaque internal result produced only by strict analysis of those same bytes; public library callers cannot provide a hand-built AST or forge an eligibility flag. Bind cached results and origins to profile, exact source digest and scenario digest, plus document version/generation for editor jobs. Reset eligibility on every edit, including syntax failure. Server-side discard must precede publication, not depend on client version filtering. Never read disk to substitute for an open buffer. Format edits must carry the version from which they were computed and be discarded if it changed.

Discriminators: a formerly valid intent edited to omit `fee_cap`; a delayed old analysis completing after a new version; decreasing versions and duplicate opens; close/reopen with the same URI; a Unicode astral character preceding an error; CRLF and lone CR line endings; escaped versus literal Unicode. Old diagnostic/candidate/formatting results must not become current. All hover/definition/symbol/diagnostic ranges must use the same UTF-8 ↔ UTF-16 converter. Test split/coalesced frames and body lengths measured in bytes, not characters.

### TS-05 — Medium: generic named records are insufficient to award `SpecifiedOnly`

Design observation: the grammar allows many declaration kinds and qualified named calls; the candidate promises name/quantity checks for non-S0 operations but also rejects unknown constructs. The eight-family table does not define a closed set of recognized operation names, references, field schemas or construct-level support labels.

Inference: a misspelled `amm.swpaExactIn` or an arbitrary record can look like a successfully checked specified financial profile. A whole-action `SpecifiedOnly` flag does not identify an unrecognized nested construct or distinguish structural checks from absent financial validation.

Repair: choose a small explicit authoring registry for horizon examples with operation names, nominal reference roles, required fields and checked quantity dimensions. Report `SpecifiedOnly` only for recognized forms; unknown forms receive named authoring diagnostics. Expose separate recognized/syntax/name/quantity/lowering/local-preparation statuses, with construct-level unsupported obligations. Full financial validation remains open. The complete eight-family examples, composition trace and coverage table required by the mockup must accompany the eventual beta documentation; the family table alone does not satisfy those requirements.

Discriminators: typo in a horizon operation; unknown named argument; reference to an undeclared instrument; amount in another nominal asset with the same symbol; composed stage relying on an unavailable operation. Check must describe exactly what was checked; expand/simulate must reject unsupported execution with no effect vector.

## Substantive design votes

| Choice | Vote | Reason and acceptance discriminator |
| --- | --- | --- |
| External bounded brace language, existing TS frontend, deferred workbench | Agree | Lowest semantic migration scope; parser corpus and measurable editor latency still required. No independent grammar becomes the financial authority. |
| Existing Source/6 plus Core/5 as strict local preparation route | Agree with TS-01/03 repairs | Reuses actual field/order/range checks and `PreparedUnqualified`. Independent fixtures must compare every effect, identity, counter and first failure, not invoke the same elaborator twice. |
| Partial edit help excluded from strict financial lowering | Agree with TS-04 repair | Useful incomplete editing must preserve the absence of a financial candidate; one old valid tree cannot authorize changed bytes. |
| Compiled JS package with clean installed CLI/library | Agree | Pack existing strict modules into the release artifact. Test outside checkout without global TS tooling or maintainer files; import library without CLI side effects; invoke check, expand, simulate and public API. Node minimum, exports and VSIX server path must be tested from the actual artifact. |
| Bounded source/scenario text only, read-only local MCP | Agree with TS-01/02 repairs | Reject paths, shell, caller ASTs, network module loading, wallet operations and unknown parameters at the server. Hints alone do not enforce access. A real protocol-client test must compare complete CLI/MCP results and stdout purity. Inert provider templates do not establish activation or host zero-egress. |
| Formatter changes whitespace while retaining token/comment spelling | Agree with additional executable contract | Compare raw token and comment lexemes and their sequence, reparse, check idempotence. Do not move a line-comment newline in a way that swallows the next token. Keep original digest distinct from formatted digest; literal `source_hash` remains an opaque authored claim and is never rewritten to manufacture agreement. Reject unsafe incomplete formatting. |
| Exact nominal quantity sugar; explicit `atoms` alternative | Agree with TS-03 repair | Good reduction of atom-count mistakes; reject rounding, scale mismatch and nominal cross-asset arithmetic. Pure sugar requires boundary and independent explicit-form comparisons. |
| Statuses distinguish authoring, local preparation and qualification | Agree with TS-05 repair | The current wrapper explicitly retains four unverified bindings and four external premises. Every success-shaped CLI/MCP output must retain them. No `verified: true`, syntax pass or test success can promote qualification. |

## Disposition required for convergence

Incorporate TS-01 through TS-05 into the selected contracts or record explicit, testable alternative dispositions. The existing architecture is implementable without another framework migration. This seat does not approve unperformed distribution/editor/protocol tests, general sugar correspondence, eight-family financial semantics, K/Quint equivalence, native proof or Midnight settlement. Those remain separate evidence obligations.
