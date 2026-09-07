{
  "candidate_commit": "1c2616efbec89791c12a249bb99fef24d96a034d",
  "candidate_sha256": "1aa997e30f5d6e79e5d9d237eab83f9b63023a4bb9896505272c5729e47cf376",
  "base_candidate_commit": "352dfdb284e3c021cf1a4c53d896eeab336787fc",
  "verdict": "APPROVED",
  "scope": "MC01 1.1/1.2/D.1/D.2 source specification freeze for profile moriarty-bounded-atomic/1 only: grammar.ebnf, semantics.md, typed-schemas.md, numeric-profile.json, bounds.json, profile-proposal.md, target-crosswalk.json, and the two developer examples, judged from the supplied packet bytes. Not approval of the experimental frontend, evaluator, materialized encodings as implementation evidence, native proof, PCD, MC04/MC05 ledger acceptance, MC07 corpus conformance, or whole MC01 closure. No tools were executed by this reviewer; arithmetic and count cross-checks below were recomputed by hand from the source constants and files in the packet, not taken from verify_profile.py output or the test log.",
  "acceptance_assessment": {
    "ebnf_internal_consistency": "PASS. Declaration, statement, and primary alternatives are keyword-distinguishable; longest-match operators, no chained comparison, no plain division, contextual asset/amount labels, and reserved prototype names are explicit. Both examples, including reserve/status/policy/effect/action forms, are derivable from the grammar by inspection.",
    "numeric_typing_and_nominal_units": "PASS. UInt128/Text/Amount<U> stored, Bool/Quantity expression-only, exact-vector assignment, same-unit add/sub/compare, exponent add on Mul and subtract on FloorDiv. Swap numerator Amount<A>*Amount<B> over denominator Amount<A> classifies as Amount<B> and is let-bound as a Quantity intermediate as required.",
    "literals": "PASS. uint(), text(), amount(n,U), true/false are distinct; numeric-looking text never coerces; canonical decimal tokens with staged UINT_RANGE at stage 6.",
    "checked_intermediates": "PASS. All intermediates UInt128, overflow rejected before a later division (explicit MAX*2 width boundary), zero divisor and underflow reject the whole action, no reassociation or host widening accepted.",
    "envelopes_and_limits": "PASS. Closed records throughout, canonical-json/1 defined, single domain registry, compact ProgramRef only in signed statements, depth-16/65536-byte public envelopes checked jointly; computed maximum nesting of signed statement (~8), evaluation input (~8), manifest (~28 of 40) and typed program (~25 of 40) fit the declared depths; typedProgram arrayLength 16384 equals 64 actions x 256 nodes.",
    "lifetime_horizon": "PASS. Positive genesis allowance, one consumption per Complete, revision+remaining==lifetime invariant, exclusive UTC horizon from authenticated now, resets/split/join/continuation forbidden, explicit ReserveRule with exact Gt(Remaining, Literal 1) shape test.",
    "result_variants": "PASS. Rejected exposes nothing; Complete is episode-scoped with separate agreement status and retained obligation tombstones; Pending is not a variant and rejects UNSUPPORTED_PENDING.",
    "bindings_and_schemas": "PASS. Genesis/state/authority/claim-root/predecessor/action equalities enumerated with closed codes; ExternalChecks precheck contract, mandatory local ProgramBinding source/bounds recheck, exact ActionCall hash bound in CompleteBody and ProofContext, acyclic six-step sign/derive/prove/verify/commit protocol, CountBody omitting resourceCounts removes count/hash circularity; ObligationDelta snapshots and empty-action zero maxima are now specified with exact vectors.",
    "traceability": "PASS at inventory level. 32 ACTUS rows (ANN through BNDWR) and 72 sequential DeFi rows each carry source locator/pointer, supported behavior, unsupported behavior, necessary extension, evidence status, and mc07_mandatory=true; only LAM and DEFI:1 are marked partial-pilot. This is inventory, not behavioral conformance.",
    "ds01_07_split": "PASS. Foundation versus MC07 obligation table present; signed values, calendar/year fraction, convergence, quantity-price, composition/Pending are listed as extensions requiring a new version, not excluded targets.",
    "example_arithmetic": "PASS. Recomputed: 5,000,000,000*8*31 = 1,240,000,000,000; /36,500 floors to 33,972,602 with remainder 27,000/36,500 = 54/73; total due 533,972,602; notional after 4,500,000,000. Swap: 9,970,000*2,000,000 = 19,940,000,000,000 over 1,009,970,000 floors to 19,743 (remainder 162,290,000), reserves 1,010,000 and 1,980,257. Loan settle ends with episode Closed, both dues Settled, agreement Outstanding, remaining notional 4,500,000,000; the final source guard cannot equate closure with discharge.",
    "policy_coverage": "PASS. Loan: 9 Amount set occurrences + 5 Amount effect-field occurrences = 14 targets across 3 policies, disjoint and exhaustive; floor provenance a_0_s_5_e_0 reaches interest_due and emit ordinal 1 only. Swap: 8 sets + 4 effect fields = 12 targets; output_calculated provenance reaches reserve_b, trader_b and emit ordinal 1; all other targets are provenance-empty with rounding none. Static counts in the receipt (accrue 14/42/3, settle 19/58/0, swap 24/76/4, close 10/26/0) match a hand count of the sources.",
    "effects_and_pcd": "PASS. Transfer/Fee/Due effects are declared proposals with mandatory exact settlement resolution; asset authenticity, movement, and all contract/intent/transition/history claims are external fail-closed predicates; documentary policy strings are not predicates; historical MORIARTY-SIGN-v1 and MORIARTY-OUTCOME-SIGN-v1 bytes are never relabeled."
  },
  "findings": [
    {
      "id": "F-01",
      "severity": "low",
      "blocking": false,
      "locator": "semantics.md 'Arithmetic and evaluation order' and 'Closed diagnostics' TYPE_MISMATCH row; grammar.ebnf semantic obligation 8",
      "problem": "The requirement that a guard condition have type Bool is only implicit ('Guards false reject'); the TYPE_MISMATCH row enumerates operators, initializers, sets, arguments, effect fields and stored/local types but not guard conditions.",
      "required_fix": "Add one sentence: a Guard condition must type as Bool with unit metadata []; any other type rejects TYPE_MISMATCH at stage 6."
    },
    {
      "id": "F-02",
      "severity": "low",
      "blocking": false,
      "locator": "typed-schemas.md 'Types and values' BareAmountEffectType; numeric-profile.json types.amountCoreRepresentation and units.quantityClassification",
      "problem": "AmountFromOperand 'records the operand unit' but does not state in one place that the operand must classify as Amount<U> (single declared unit, exponent one). A UInt128 or Quantity operand is rejectable by derivation (empty/compound vector is not an Amount), but the rejection code and stage (TYPE_MISMATCH at 6 versus later OBLIGATION_MISMATCH/SETTLEMENT_BINDING_MISSING) are not spelled out.",
      "required_fix": "State explicitly that an emit operand for a BareAmount field must have classification Amount<U>; UInt128, Bool, Text and Quantity operands reject TYPE_MISMATCH at stage 6."
    },
    {
      "id": "F-03",
      "severity": "low",
      "blocking": false,
      "locator": "typed-schemas.md 'Counts, ordering, and malformed inputs' first sentence versus 'Authority and compact signing objects' ExactPlan comparison",
      "problem": "Writes are said to follow 'declaration or statement order as applicable' in one place and 'statement order' in the ExactPlan comparison. Both readings are present; only statement order is consistent with the WriteRecord projection.",
      "required_fix": "State once that CompleteBody.writes and exactWrites are in set-statement execution order, and that after.body.values remain in declaration order."
    },
    {
      "id": "F-04",
      "severity": "low",
      "blocking": false,
      "locator": "semantics.md 'Closed diagnostics' rows DECLARATION_SCHEMA (stage 4) and SETTLEMENT_DECLARATION (stage 6); typed-schemas.md SettlementDecl",
      "problem": "grammar.ebnf accepts any literal for settlement asset/quantum, but the AST record requires TextLiteral/AmountLiteral. Both 'malformed declaration' (stage 4) and 'malformed settlement declaration' (stage 6) could apply to a uint() quantum, leaving the rejection code for that invalid program ambiguous. Diagnostic bytes only; no effect on accepted programs or hashes.",
      "required_fix": "Assign AST-shape failures (wrong literal kind in SettlementDecl, EpisodeStatusDecl literal, Const/State initializer literal kind) to DECLARATION_SCHEMA at stage 4 and reserve SETTLEMENT_DECLARATION for semantic failures of a well-shaped declaration."
    },
    {
      "id": "F-05",
      "severity": "low",
      "blocking": false,
      "locator": "semantics.md 'Closed diagnostics' preamble ('failing source expression/statement/declaration span') and GUARD_FAILED row",
      "problem": "For GUARD_FAILED it is not stated whether primarySpan is the Guard statement span or the condition expression span; both are 'available'. Rejected objects are not identities or certificates, so impact is limited to Rejected byte determinism.",
      "required_fix": "State that GUARD_FAILED uses the Guard statement span, arithmetic codes use the failing expression node span, and POLICY_PROVENANCE uses the Set/Emit statement span."
    },
    {
      "id": "F-06",
      "severity": "low",
      "blocking": false,
      "locator": "semantics.md 'Closed diagnostics' POLICY_ROUNDING row",
      "problem": "Row text duplicates its own clause ('unknown, wrong-action, ambiguous, unknown, wrong-action, ambiguous, ... or its root is not FloorDiv'). Editorial only; the condition set is unambiguous.",
      "required_fix": "Deduplicate the row text without changing the condition set."
    },
    {
      "id": "F-07",
      "severity": "low",
      "blocking": false,
      "locator": "semantics.md 'Effect schemas and settlement' conservation paragraph",
      "problem": "The group key is written as (asset,from,to,unit) for DueSettled effects, which have debtor/creditor rather than from/to; the following clause ('from=debtor and to=creditor') makes the intent recoverable but the key naming is loose.",
      "required_fix": "Restate the key as (asset, debtor, creditor, unit) for DueSettled and (asset, from, to, unit) for Transfer, with the two keys compared component-wise."
    },
    {
      "id": "F-08",
      "severity": "low",
      "blocking": false,
      "locator": "numeric-profile.json integer.reassociation ('forbidden-if-observable-behavior-changes') versus semantics.md ('No reassociation ... is permitted')",
      "problem": "One file permits unobservable reassociation and the other forbids all reassociation. Semantically equivalent for accepted behavior, but the wording differs.",
      "required_fix": "Align both to a single statement: evaluation order is the recorded Core tree; any implementation optimization must be unobservable in results, diagnostics, and resource counts."
    },
    {
      "id": "F-09",
      "severity": "low",
      "blocking": false,
      "locator": "profile-proposal.md 'Developer surface' illustrative fragment",
      "problem": "The fragment omits the mandatory actor parameter and uses a DueCreated field named event that the fixed schema forbids. It is labeled 'not a valid fixture', so it is not normative, but it can mislead an implementer reading the proposal first.",
      "required_fix": "Either mark each nonconforming line explicitly or replace the fragment with the first lines of loan.moriarty."
    }
  ],
  "residual_limitations": [
    "This review judged the packet text only; no tool, hash recomputation, parser, or test was executed by the reviewer. Reported SHA-256 digests, host_ebnf_check acceptance, verify_profile/verify_materialized results, and the 20-test edge-runtime log are author-supplied and were checked only for internal consistency with the sources.",
    "Lossless correspondence of the 32/72 crosswalk rows to the original actus-32-requirements.csv and defi-72-requirements.csv was not verifiable from the packet; the crosswalk is an inventory and establishes no behavioral conformance. All ACTUS fixtures and DeFi rows remain MC07 obligations.",
    "The materialized AST/Core encodings show representability of the two examples under an experimental frontend whose receipt discloses gaps (undeclared diagnostic protocol admission, caller-supplied validator dependency, a parser nesting ceiling of 128 that the current semantics.md now explicitly forbids as an implementation-specific rejection). These are implementation matters for a later gate, not part of this freeze.",
    "ExternalChecks, ProofAcceptanceInput/Verdict and the resolver contract define the trusted backend interface only. MC05 must implement signature, nonce, currentness, observation authentication, predecessor, and proof verification; no PCD, native proof, or verifier exists in this candidate.",
    "Aggregate 65536-byte limits on EvaluationInput and Complete mean an individually representable state, authority and program can still be unevaluable or unclosable (INPUT_BOUNDS/RESULT_BOUNDS without consumption). The profile discloses this as an accepted liveness limitation; deployments must size instances accordingly.",
    "Signed values, calendar/year-fraction, convergence, quantity-price, partial settlement, Pending/split/join, and composition remain mandatory later versions; nothing in this profile may be reinterpreted to supply them.",
    "Freeze status is an external approval record over the exact file digests in the manifest; the 'proposed-not-frozen' strings inside bounds.json and the spec headers are part of the hashed bytes and must not be edited to record approval."
  ]
}