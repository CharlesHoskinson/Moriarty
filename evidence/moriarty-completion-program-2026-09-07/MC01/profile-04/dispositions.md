# MC01 profile04 normative-source dispositions

Scope: source specification correction only, provisional S2, not frozen or a proof/conformance/ledger claim. Worktree: /home/charl/Moriarty-wt-moriarty-mc01-20260907-plan-profile. Base commit: 5115d009fb4a679386593df22582040c96e06a8d. No commits were made. Owned edits are restricted to experiments/moriarty-language/spec and this record.

| Audit finding | Corrected source section / disposition |
|---|---|
| MC01-G6-03-01; F-MC01-06 Amount | numeric-profile.json types.amountCoreRepresentation and units; typed-schemas.md Types and values / Scalar and canonical JSON codec: exact tagged AmountValue distinguished from Source AmountLiteral; Bool/Text metadata is [], with explicit type distinguishing UInt128. Full materialization checks belong to the separate frontend/evidence work and are not claimed here. |
| MC01-G6-03-02 | typed-schemas.md Settlement and result records / Binding and acyclic proof acceptance: CompleteBody and ProofContext both carry recomputed exact EvaluationInput.action hash for both authority modes. Candidate context, backend evidence and trusted verdict bind the exact action. |
| MC01-G6-03-03 | typed-schemas.md Exact source spans and structural lowering: UTF-8 token boundaries, wrappers, literal children, nested parenthesis erasure/expanded expression spans, exact AST-to-Core SourceRef. Parenthesis worked interval example supplied; source/AST/Core/hash vector belongs to separate evidence work. |
| MC01-G6-03-04; F-MC01-07 | grammar.ebnf reserve_decl; typed-schemas.md ReserveDecl/ReserveRule/SemanticManifest; semantics.md Genesis and lifecycle: explicit reserve ACTION for CLOSURE declaration, exact root Gt(Remaining,Literal(UIntLiteral1)) guard, existing distinct actions, unique reserved actions, closure cannot itself be reserved. No implicit role or loan exception. Swap includes reserve swap for close. |
| F-MC01-01 | typed-schemas.md Authority and compact signing objects / Binding and acyclic proof acceptance; profile-proposal.md Transition judgment: EvaluationInput prechecks only, closed post-trace ProofAcceptanceInput plus resolver contract, exact context-bound closed verdict and numbered sign/derive/prove/verify/commit protocol. Backend proof count/per-proof/aggregate bytes bounded in bounds.json. |
| F-MC01-02 | semantics.md Binding equalities: exact genesis/source initialState, lifetime, horizon, bounds and program bindings; state program/genesis/instance and zero-revision genesis state; signed program/context/claims/predecessors and ExactPlan action equality; outcome allowed action. Every predicate has closed diagnostic. |
| F-MC01-03 | typed-schemas.md Typed program: zero-based action/statement indexes and expression preorder reset per statement; emit field source order, FloorDiv numerator before denominator. Exact loan a_0_s_5_e_0/1/2 worked example. |
| F-MC01-04 | semantics.md Authority, observations, and external assumptions: principal actor only for caps/goals; every principal outgoing Transfer/Fee requires cap, including zero; Due effects excluded; checked ledger-unit sums; credits >= checked(debits+minimum), overflow rejects. |
| F-MC01-05 | Historical evidence issue assigned to parent; no historical evidence rewritten by this source correction. |
| F-MC01-06 domains | bounds.json.domainRegistry is the complete profile registry, including outcome and all program/claims/genesis/state/action/observation/authority/proof-context/trace/signing domains and raw source/proof hashes. Historical domains preserved; typed schemas and bounds reference this registry. |
| F-MC01-08 | semantics.md Closed diagnostics: 89 unique code/stage/condition rows, including obligation zero/denomination mismatch, binding equalities and outcome refinements. Diagnostic selection and bounded result spelling fixed. |
| F-MC01-09 | typed-schemas.md Genesis, state, observations, and action: evidenceDigest and Signature.algorithm explicitly external wrapper-validated values; Scalar and canonical JSON codec reserves non-Source tags for MC02 and permits Source only in Core1. |
| F-MC01-10 | bounds.json programShape.sourceDeclarations=128; typed-schemas.md Counts; semantics.md Bounds: explicit effective total declaration limit and accepted lack of transition/closure representability near result cap. No blanket liveness guarantee. |
| F-MC01-11 | Historical locators assigned to parent. This disposition uses current section anchors, not stale line ranges. |

Verification performed after normative edits:

- JSON parse of bounds.json, numeric-profile.json and target-crosswalk.json: pass.
- Generic EBNF recognizer from evidence/.../profile-03/host_ebnf_check.py against current grammar and both full examples: exit 0, loan accepted 1056 tokens and swap accepted 950 tokens.
- Historical dependency-free source verifier evidence/.../profile-03/verify_profile.py: exit 1 at line 289, AssertionError: swap digest. It reached the pinned-vector digest checks after source static/effect/policy and crosswalk checks. The historical vector intentionally records the prior swap/bounds bytes; parent owns new evidence. No full verifier pass is claimed here.
- git diff --check for owned spec files: pass.
- target-crosswalk.json SHA256 remains 0cc5bddf6a10ea9a2df7b7b5686c148314eb8f9b48f97c54ba02ab32154de884; all 32 ACTUS and 72 DeFi rows preserved byte-identically.

Outstanding gates: fresh exact-byte Fable and GPT-6 audits; separate frontend materialization/aggregate representability and evidence vectors; MC05 actual proof backend; MC07 full target conformance. No historical audit or model check is promoted to those claims.
