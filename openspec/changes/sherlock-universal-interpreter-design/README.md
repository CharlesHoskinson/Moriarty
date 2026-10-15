# Sherlock universal interpreter: scope and evidence

**Name:** Sherlock is Moriarty’s universal interpreter, named by the user on 2026-10-15. Existing draft paths, model modules and requirement IDs retain their technical identifiers for traceability.

**DRAFT / SPECIFIED-ONLY.** This is a concrete review package for Candidate A, not an implemented interpreter, model result, semantic freeze or product release. It is staged only under runtime `spec-package/`. Eventual repository destination is `openspec/changes/sherlock-universal-interpreter-design/`; root owns registration, canonical links/index, security-draft copying, integration and publication. No repository files or stable source profiles are changed here.

- [Proposal](proposal.md): problem, requested change and integration boundary.
- [Design](design.md): supported machine, alternatives, financial extension constraints and open decisions.
- [EARS index](requirements.md) and [26 clauses/scenarios](specs/universal-interpreter/spec.md).
- [Twenty normal demonstration contracts](demonstrators.md).
- [Concrete K/Quint source inventory and gaps](model-coverage.md).
- [Required Lean/Clean stage](clean-stage.md), UIREQ-026; definitions, circuits, proofs and native lowering remain unimplemented/unproved.
- [Public Clean API inventory](public-clean-api-inventory.json); private integration pins/content remain separately retained.
- [Traceability](traceability.md): component coverage, inherited owners/exits and optimization evidence.
- [Unchecked tasks](tasks.md).
- [Hashed input inventory](inputs.json) and `SHA256SUMS`.

## Authorship and immutable inputs

Nonsecurity author: delegated GPT-6.1 Sol high lane 10. The ten first-wave proposals are research inputs, not independent design approval. Read-only source baseline: `feat/universal-interpreter-spec-20261015`, `9c63048a18315d7eb3cf114b983bd68c0d842dc6`, active isolated Moriarty checkout (runtime path omitted from public package).

Security clauses/judgments belong exclusively to Opus 5.5. The separate [OPUS-INTERPRETER-SECURITY-DRAFT.md](security-requirements.md), UISEC-001–012, is referenced unchanged; its input digest is in inputs.json. Root will copy the Opus-owned section verbatim separately and reconcile its references. No UIREQ clause replaces or assesses any UISEC/OSR/ZR/MNR security obligation. Functional arithmetic, failure and accounting descriptions here are requirements, not security conclusions.

Candidate contract: [MODEL-DRAFT-CONTRACT.md](../../../docs/research/sherlock/MODEL-DRAFT-CONTRACT.md). K author owns `../../../models/sherlock/k/`; Quint author owns `../../../models/sherlock/quint/`. This package neither writes nor executes those files. Their concrete source membership is reconciled in model-coverage.md and hashed in inputs.json. Model execution and correspondence remain unestablished; root owns any later classified checks. All private implementation details stay outside this public-text candidate.

## Evidence classes and remaining gap

| Artifact/class | Present status | Permitted conclusion |
|---|---|---|
| Ten public-source research briefs | Read as frozen inputs; their receipts retained in their own lanes | Sources motivate options; no Moriarty correspondence |
| This OpenSpec/EARS package | Source draft, internally checked for IDs/membership | Requirements/design are reviewable, not accepted or implemented |
| K pure semantics and fixtures | Generic function-table source evaluator; eight direct-term fixtures, unexecuted | No rewriting, determinism or preservation result asserted |
| Quint bounded lifecycle/control/data | Three algorithm-specific transitions, scalar15; eight unexecuted scenarios | No sampled/exhaustive/inductive result asserted |
| K-to-Quint adapter | Specified obligation | No refinement from coincident final values |
| Compiler and host implementation | Not delivered by this package | No general supported grammar/continuation implementation claim |
| Required Lean/Clean stage | Specified; no Sherlock Lean definitions/circuit/proof supplied | No imported theorem or native exporter assumed |
| Circuit/native correspondence | Not delivered or executed | No constrained execution/proof acceptance claim |
| Ledger-9 custom-decider route | User-supplied existing integration premise | Reuse is the intended route; no fresh reproduction here |
| Target ledger/financial qualification | No new observations | All native/target/U/M/MC/SP/G exits remain open |

Evidence records must name predicate, source membership/digests, profile, executable/consumer, representation, numeric and structural bounds, expected-observation source, tool versions/commands where actually run, evidence class, target tuple when established, premises, results and omissions. Unknown external fields remain unknown. Static source, finite traces, bounded properties, compiler correspondence, circuit correspondence, complete native verification and actual target observations remain distinct classes. No timing, rows, DUST, proof-size or capacity result is invented.

## Open decisions before freeze

1. Reconcile named K rule labels and helper costs, complete shape/capacity bounds and independent fuel literals; current K stack-limit error differs from the proposed capacity-exhaustion classification.
2. Extend dedicated UInt list/tree sorts to generic nominal declarations/nested data where claimed; define structural tree-to-arena projection.
3. Define common-domain K action mapping for the concrete scalar15 three-algorithm Quint companion; general-code/lexical/financial behavior remains absent.
4. Universal financial Await/Commit/Yield opcodes, constrained by existing Core/2 successful-prefix semantics and the separately owned security requirements.
5. Complete compiler/circuit/host inventory and native endpoint statement projection.
6. Required Lean semantic/representation definitions, Clean bounded step circuit and named unproved correspondence/native-backend gaps (UIREQ-026).
7. Designated exact-candidate reviews and recorded decisions among the listed alternatives.

Existing permissionless product rules remain: supported programs do not need project, registry or reviewer membership to compile/prove/deploy. Maintainer review/resource controls govern this development work only. There is no new execution permission, campaign, reset, target version, VerifyProof dependency or ledger10 prerequisite. The second optimization wave has started from the recorded Candidate A content snapshot. Recommendations do not adopt an architecture or establish a measured improvement.


Clean-stage destination: integration deliverables belong in the existing private Fenrir Clean fork/worktree. Sherlock language/OpenSpec stays in Moriarty. The required cross-repository pin/source/theorem/native-consumer correspondence is retained privately, with a shareable dependency/evidence reference here; no private paths/source or new public Clean fork are part of this package.

## Publication binding

This repository package registers the runtime Candidate A design. The original content freeze and document-check records describe their recorded runtime bytes. `publication-binding.json` maps those inputs to these published files; `SHA256SUMS` covers this package. Relative links were relocated for repository navigation. No model result is inferred from those document checks. Research inputs are retained under `docs/research/sherlock/`; raw public captures remain separately retained. Private integration inventories and source are excluded.

## Observed model checks

The [source-specific observations](../../../models/sherlock/observations.json) supersede the source-only status of this package’s recorded research snapshot. K completed its frontend check and produced syntax Kore plus compiled text; runtime `definition.kore` and rule execution were not observed. The repaired Quint source typechecked, all eight concrete tests passed, and 32 seed-1 traces of at most 32 steps showed no `allInv` violation; all seven named witnesses occurred. These are bounded model results, with no K/Quint refinement or product acceptance claim. The unchanged Candidate A snapshot and parser-repair lineage remain separately recorded.
