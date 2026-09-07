```json
{
  "verdict": "BLOCKED",
  "candidate_commit": "5115d009fb4a679386593df22582040c96e06a8d",
  "candidate_sha256": "fa7ba0b44a5f886120b40e120ba041faf9d66ede7ea5f905da43f3e6232c42ec",
  "scope": "Independent specification-freeze review of the MC01 1.1/1.2/D.1/D.2 source-profile packet for moriarty-bounded-atomic/1, judged on the supplied bytes only. Not an approval of any frontend, compiler, evaluator, native proof, PCD, ledger acceptance, MC07 conformance, or whole-MC01 closure. No tools were executed; digests, recognizer output, and checker output are taken as reported in the packet and were not recomputed.",
  "confirmed": [
    "grammar.ebnf is internally consistent for both complete examples: every declaration, statement, policy target, status form, effect schema, emit, and expression in loan.moriarty and swap.moriarty maps to a production; keyword list has no duplicates; only asset/amount are contextual labels; no plain division and no chained comparison.",
    "Numeric typing and nominal units hold on the examples: loan interest numerator types as Amount<USD_micro>, denominator UInt128, floor_div yields Amount<USD_micro>; swap numerator is the local-only Quantity {AssetA:1,AssetB:1}, denominator Amount<AssetA>, and floor_div yields Amount<AssetB>, so (A*B)/A = B without unit erasure.",
    "Independent arithmetic: 5,000,000,000*8*31 / (100*365) = 33,972,602 remainder 27,000/36,500 = 54/73, exact 2,480,000,000/73; swap 19,940,000,000,000 / 1,009,970,000 floors to 19,743 (exact 1,994,000,000/100,997), reserves 1,010,000 / 1,980,257; all intermediates fit UInt128.",
    "Policy coverage is exhaustive and disjoint by occurrence: loan 14 targets (5 accrue, 9 settle), swap 12 targets; FloorDiv provenance is a singleton at every floor-rounded target and empty at every rounding-none target under the stated propagation rules.",
    "Loan closure does not equate to discharge: after settle the source guard, the status agreement remaining_notional rule, and the vectors all retain 4,500,000,000 micro-USD with AgreementStatus Outstanding while EpisodeStatus is Closed; PR and IP obligation tombstones are retained.",
    "Lifetime/horizon: revision + remaining == lifetime holds across both example traces (2/0,1/1,0/2 and 8/0,7/1,6/2); horizon is a genesis-frozen exclusive bound with explicit authenticated now; resets, split/join, and Pending are rejected with UNSUPPORTED_PENDING.",
    "Quantum orientation and exact conversion (20/10 -> 2, 15/10 -> SETTLEMENT_NON_DIVISIBLE) and unique one-to-one asset/unit bindings are specified; DueSettled-to-Transfer conservation matches the loan's combined 533,972,602 transfer.",
    "Hash preimages (source, bounds, program, claimRoot, genesis, state, action, observations, statement, authority, proofContext, trace) and the CountBody count preimage are noncircular; ProgramRef is mandatory in signed statements, genesis, and results; historical MORIARTY-SIGN-v1 and MORIARTY-OUTCOME-SIGN-v1 domains are preserved unchanged.",
    "Crosswalk: 32 ACTUS row IDs and 72 DeFi row IDs are present, distinct, all mc07_mandatory=true, each with source locator, supported/unsupported/extension/evidence fields; signed/calendar/convergence/composition semantics are listed as versioned mandatory extensions, not excluded targets. DS-01..DS-07 foundation-versus-MC07 split is stated."
  ],
  "findings": [
    {
      "id": "F-MC01-01",
      "severity": "high",
      "blocking": true,
      "locator": "typed-schemas.md sections 'Authority and compact signing objects' (ExternalChecks, EvaluationInput) and 'Binding and acyclic proof acceptance'; semantics.md 'Deterministic validation and transition' stages 10 and 13; profile-proposal.md 'Transition judgment and correctness scope' evaluate() signature",
      "problem": "The closed EvaluationInput record requires checks.requiredClaimsValid, checks.historyProofValid, and claimEvidence[] (whose publicInputDigest must equal proofContextHash) to be present before evaluation, yet the same document defines those values as backend attestations computed only after deterministic execution produces traceHash and ProofContext (stage 13). No record or function signature carries the ProofContext to the backend or returns its verdict, and proof bytes (needed for proofDigest) appear in no schema. The proposal's evaluate(program,state,action,authority,observations) signature omits genesis, checks, and claim evidence entirely. An implementer must invent a two-phase protocol to build an input that cannot exist before the output it attests.",
      "required_fix": "Split the interface: (1) EvaluationInput carries only pre-execution trusted checks (signatureValid, nonceFresh, genesisValid, observationsAuthentic, predecessorSetValid, stateCurrentAndUnconsumed, authenticatedPrincipal); (2) define a closed post-trace ProofAcceptanceInput {proofContext:ProofContext, claimEvidence:[ClaimEvidenceRef], proofs:[{claimId,kind,bytes:OpaqueBytes}] or an explicit external-resolution rule} and a closed ProofAcceptanceVerdict {requiredClaimsValid, historyProofValid} returned by the profile-bound backend for exactly that ProofContext; (3) state the exact sequence as a numbered protocol with the objects passed at each step; (4) align profile-proposal.md's evaluate() signature with EvaluationInput. MC05 implements the backend; this gate must fix the interface."
    },
    {
      "id": "F-MC01-02",
      "severity": "medium",
      "blocking": true,
      "locator": "typed-schemas.md 'Genesis, state, observations, and action' (GenesisBody), 'Binding and acyclic proof acceptance'; semantics.md 'Genesis and lifecycle' and stage 9/10; numeric-profile.json genesisVersusDynamic",
      "problem": "GenesisBody duplicates initialState, lifetime, horizon, and bounds that also live in the SemanticManifest, but no equality predicate between them is stated, so a genesis could bind a different notional, allowance, or horizon than the authenticated source literals. Likewise the ExactPlanStatement.action is never required to equal EvaluationInput.action, and equality of genesis.body.program with EvaluationInput.program and StateBody.programHash is only implied by 'compact ProgramRef equality'. These are financial-meaning bindings that an implementer would have to invent.",
      "required_fix": "Add an explicit binding-equality list to stage 9/10: genesis.body.initialState == manifest.initialState (exact canonical bytes), genesis.body.lifetime == manifest.lifetime, genesis.body.horizon == manifest.horizon, genesis.body.bounds == program.bounds, genesis.body.program == EvaluationInput.program, state.body.programHash == program.programHash, state.body.genesisHash == genesis.genesisHash, authority.statement.action == EvaluationInput.action (ExactPlan) and action.name in allowedActions (IntentRefinement). Assign each a diagnostic code."
    },
    {
      "id": "F-MC01-03",
      "severity": "medium",
      "blocking": true,
      "locator": "typed-schemas.md 'Typed program: moriarty-typed-program/1' (nodeId scheme) and 'Core, policies, schemas, and semantic manifest' (statementId, RoundingNode.coreNodeId)",
      "problem": "nodeId a_<action-index>_s_<statement-index>_e_<preorder-index> and statementId a_<i>_s_<j> feed the program hash and the rounding coreNodeId, but the index base (0 or 1) is unspecified, and the preorder-index scope is unspecified (per statement or per action, and ordering across the multiple field expressions of one emit). Two conforming implementations can produce different programHash values for identical source, breaking deterministic implementability and all downstream identifiers.",
      "required_fix": "State zero-based action, statement, and expression indexes; define the expression preorder scope as per statement, with an emit's field expressions traversed in field order as one sequence; give a worked example listing the IDs for one loan statement (e.g., the accrue floor_div let) so the scheme is testable."
    },
    {
      "id": "F-MC01-04",
      "severity": "medium",
      "blocking": true,
      "locator": "semantics.md 'Authority, observations, and external assumptions' (IntentRefinement paragraph); typed-schemas.md OutcomeStatement, DebitCap, NetGoal",
      "problem": "IntentRefinement financial semantics are incomplete: it is not stated whether a Transfer/Fee debit for an (actor,asset) pair with no grossDebitCap rejects or passes; whether caps and goals for actors other than the principal-bound actor are enforced or ignored; whether DueCreated/DueSettled count as debits or credits; and how 'credits minus debits' is evaluated in unsigned arithmetic when debits exceed credits. A permissive default would let the signing principal's assets be debited without any cap.",
      "required_fix": "Define: for the principal-bound actor A, every Transfer/Fee with from==A requires a matching cap or rejects (e.g., INTENT_DEBIT_UNCAPPED); caps/goals naming other actors are either forbidden or explicitly ignored (choose one); Due effects are excluded from ledger debit/credit sums; a net goal is satisfied iff checked(credits) >= checked(debits + minimumLedgerAmount) with overflow rejecting; all sums in ledger units after quantum conversion."
    },
    {
      "id": "F-MC01-05",
      "severity": "medium",
      "blocking": true,
      "locator": "evidence/.../profile-03/verification.json specFileSha256; CANDIDATE MANIFEST sha256 entries; FOREMAN_REPORT.md",
      "problem": "verification.json records SHA-256 values for bounds.json, grammar.ebnf, profile-proposal.md, semantics.md, and typed-schemas.md that do not match the candidate manifest digests for those files (e.g., bounds.json 631aa2e5... vs cffd9a1e...; typed-schemas.md b3eadb79... vs e2fb0680...). The pass record therefore describes pre-host-correction bytes, is not labeled as such, and the host record carries no per-file digests of the frozen bytes. A freeze packet cannot contain an unflagged evidence record for different spec bytes.",
      "required_fix": "Either regenerate verification.json from verify_profile.py against the frozen bytes and re-record all digests, or relabel it as the pre-correction worker record and add the frozen per-file SHA-256 set (matching the manifest) to host-verification.json together with the checker output that was run on those bytes."
    },
    {
      "id": "F-MC01-06",
      "severity": "low",
      "blocking": true,
      "locator": "numeric-profile.json types.amountCoreRepresentation; bounds.json signingEnvelope.schemaDomainNew; typed-schemas.md AmountValue and IntentRefinementAuthority",
      "problem": "numeric-profile.json describes the wire Amount as closed {value,unit} while typed-schemas.md defines AmountValue as closed {tag:'Amount',unit,value}; bounds.json registers only MORIARTY-SIGN-bounded-atomic/1 as the new signing domain while typed-schemas.md also defines MORIARTY-OUTCOME-bounded-atomic/1. semantics.md states that any conflict among the normative files is an error in the candidate, so these must be resolved before freeze even though the intended shape is clear.",
      "required_fix": "Change amountCoreRepresentation to the {tag,unit,value} record and add MORIARTY-OUTCOME-bounded-atomic/1 (and the MORIARTY-AUTHORITY/PROOF-CONTEXT/TRACE/CLAIMS/GENESIS/STATE/ACTION/OBSERVATIONS domains) to a single domain registry in bounds.json or typed-schemas.md, referenced from both."
    },
    {
      "id": "F-MC01-07",
      "severity": "medium",
      "blocking": false,
      "locator": "semantics.md 'Genesis and lifecycle' closure-reserve paragraph; bounds.json lifecycle.closureReserve",
      "problem": "The rule that each repeatable action 'must' contain a remaining > 1 guard when 'a distinct terminal resource-return action' exists has no source marker or decidable predicate identifying such an action, and it is absent from the 13 validation stages. It is stated as mandatory but cannot be checked deterministically.",
      "required_fix": "Either add an explicit source marker (e.g., an action modifier) and a stage-6 check with a diagnostic code, or restate the rule as documentary developer guidance that is not a validation predicate, and say which."
    },
    {
      "id": "F-MC01-08",
      "severity": "low",
      "blocking": false,
      "locator": "semantics.md 'Obligation transition and status derivation'; typed-schemas.md Diagnostic",
      "problem": "Diagnostic codes are not fully enumerated: DueCreated denomination-unit mismatch has no code (OBLIGATION_ZERO/DUPLICATE/CAPACITY cover only three of four rules), DueSettled zero amount says 'zero rejects' without a code, and the binding-equality and intent-refinement failures have none. Diagnostic determinism is part of conformance testing.",
      "required_fix": "Add a closed diagnostic code table (code, stage, condition) covering every rejection named in semantics.md and typed-schemas.md."
    },
    {
      "id": "F-MC01-09",
      "severity": "low",
      "blocking": false,
      "locator": "typed-schemas.md ObservationValue.evidenceDigest, Signature.algorithm, GeneratedTag; Core instruction set",
      "problem": "evidenceDigest has no defined preimage or domain; Signature.algorithm has no registry; GeneratedTag values other than Source have no Core carrier (CoreInstruction has no generated-check variant), so it is unclear whether generated nodes exist in this profile or only in later lowering.",
      "required_fix": "State that evidenceDigest and algorithm are opaque external values validated only by the wrapper (or define them), and state that in moriarty-core/1 all nodes carry tag Source and the other tags are reserved for MC02 lowering."
    },
    {
      "id": "F-MC01-10",
      "severity": "low",
      "blocking": false,
      "locator": "bounds.json astEncoding.arrayLength=128 versus programShape per-category maxima; resultEncoding 65536 bytes versus evaluationEncoding StateEnvelope 65536 bytes",
      "problem": "SourceAST.declarations can exceed 128 entries while every per-category maximum is satisfied (e.g., 64 state + 64 const + 64 actions), and a Complete wrapper shares its 65,536-byte cap with the embedded after-state, so a bounds-valid near-limit state could have no representable transition, including closure. Both are acknowledged as joint limits, but the first is an effective cap not stated anywhere and the second is a liveness risk.",
      "required_fix": "Document the effective declarations cap explicitly, and either raise resultEncoding above the embedded state bound or state the liveness consequence as an accepted profile limitation."
    },
    {
      "id": "F-MC01-11",
      "severity": "low",
      "blocking": false,
      "locator": "worker-report.json dispositions locators; worker-report.md; FOREMAN_REPORT.md",
      "problem": "Worker locators use line ranges from pre-correction files and MC01-N8 cites 'FOREMAN_REPORT.md:Audit finding-Disposition-MC01-N8', which does not exist in the supplied FOREMAN_REPORT.md. The host notes old locators but the packet does not say which dispositions were re-verified against the frozen bytes.",
      "required_fix": "Refresh locators to section anchors in the frozen files or mark the worker reports as historical and add a host disposition table keyed to the frozen bytes."
    }
  ],
  "residual_limitations": [
    "No tools were executed. SHA-256 digests, the host EBNF recognizer results, canonical codec/action vector hashes, and verify_profile.py output are accepted as reported and were not recomputed.",
    "Lossless traceability of the 104 crosswalk rows to actus-32-requirements.csv and defi-72-requirements.csv could not be checked because the source CSVs are not in the packet; only row counts, distinct IDs, field presence, and the unchanged crosswalk digest were checked.",
    "Grammar acceptance of both examples was checked by manual derivation and by the reported recognizer; unambiguity was assessed by inspection, not by a parser-generator conflict check.",
    "Representability of the complete compiled SemanticManifest/BoundProgram for either example under bounds.json was estimated, not materialized; the packet itself states no frontend exists.",
    "Line-number locators in worker reports could not be verified; findings cite section and record names instead.",
    "Financial correctness is judged only for the two fixed vectors; no ACTUS fixture or DeFi conformance claim is assessed, consistent with the gate."
  ]
}
```