# Independent MC01 source-profile audit 03

Verdict: **BLOCKED**

Candidate commit: `5115d009fb4a679386593df22582040c96e06a8d`
Candidate SHA-256: `fa7ba0b44a5f886120b40e120ba041faf9d66ede7ea5f905da43f3e6232c42ec`

Independent source-profile freeze review for MC01 1.1/1.2/D.1/D.2 of the exact frozen candidate. This is not frontend implementation, MC01 package completion, MC07 conformance, native proof, PCD execution, or ledger acceptance.

## MC01-G6-03-01 (high; blocking)

Locator: experiments/moriarty-language/spec/numeric-profile.json:38,64,73; experiments/moriarty-language/spec/typed-schemas.md:68-74,168-169,188-204; experiments/moriarty-language/spec/semantics.md:4-7

The jointly normative encodings conflict. numeric-profile.json requires the closed Amount record {value,unit} on every AST/manifest/input/state/effect/result object, while typed-schemas.md requires {tag:"Amount",unit,value} and different Source AmountLiteral records. The closed-record rules cannot both hold. Also Bool/Text unit vectors are explicitly not-applicable, while every TypedAnnotation and Core expression requires unitVector:UnitVector; no not-applicable encoding or Bool/Text empty-array convention exists, and numeric-profile.json says the empty array is UInt128. Both supplied examples necessarily contain Text expressions and Bool guards. Consequently their full typed/Core encodings cannot be derived without choosing or repairing a normative rule.

Required fix: Make the numeric and typed schemas agree on the exact tagged Amount wire value, distinguishing Source AmountLiteral from evaluated AmountValue. Define one exact representation for nonnumeric unit metadata in TypedAnnotation/Core, including Bool comparisons, Text references/literals, and Not. Apply it consistently to the numeric profile and closed schemas, then provide/recheck complete typed/Core example encodings against the aggregate bounds.

## MC01-G6-03-02 (high; blocking)

Locator: experiments/moriarty-language/spec/typed-schemas.md:341,418-424,452-456,508-515,539-549; experiments/moriarty-language/spec/examples/swap.moriarty:64,79

The exact ActionCall is absent from the proof public input in IntentRefinement mode. OutcomeStatement signs allowedActions but no selected action or arguments; CompleteBody records no action/actionHash; ProofContext contains neither. The defined actionHash is unused in these bindings. A concrete witness is the supplied swap with amount_in=10000 and min_out=0 versus min_out=1 under the same outcome authority and input state: both pass the only min_out guard, execute identical counts, and yield identical writes/effects/after state, hence identical traceHash and ProofContext despite different ActionCalls. The backend therefore cannot bind or distinguish the exact evaluated request, and evidence for one call can be reused for the other context-equivalent call. This is a binding omission, not a claim that the local evaluator accepts a failing guard.

Required fix: Include the recomputed actionHash or complete ActionCall in the unsigned candidate trace and/or ProofContext for both authority modes. Require equality to EvaluationInput.action and require backend checks and proof public inputs to bind it. Preserve the existing acyclic signature/trace/proof sequence; no actual proof implementation is required at this gate.

## MC01-G6-03-03 (medium; blocking)

Locator: experiments/moriarty-language/spec/typed-schemas.md:44-45,105-122,176-180,189-204,337; experiments/moriarty-language/spec/grammar.ebnf:107-118; experiments/moriarty-language/spec/semantics.md:39-43,295-303

The candidate requires deterministic source spans and program hashes but specifies only legal byte ranges, not the source-to-AST span derivation. In particular the grammar permits parenthesized expressions while SourceExpression has no parenthesis variant and gives no rule for retaining or excluding those delimiters in the child expression span. For source (uint(1)), both the inner literal-expression interval and the interval including parentheses meet the published range constraints. Core sourceRef retains that interval inside SemanticManifest, so the two choices produce different canonical program hashes for the same source/profile. Similar token-versus-wrapper ambiguity remains for literal spans. Deterministic lowering is named as a stage rather than resolving this byte-level choice.

Required fix: Specify exact token-boundary spans for declarations, statements, type syntax, literal wrappers/tokens, and expressions, including parentheses and whitespace. Define the direct SourceExpression-to-Core sourceRef mapping so equivalent implementations produce identical manifest bytes. Add a small source/AST/Core/hash vector with nested parentheses and literals; a full frontend or proof implementation is unnecessary.

## MC01-G6-03-04 (medium; blocking)

Locator: experiments/moriarty-language/spec/semantics.md:241-246; experiments/moriarty-language/spec/bounds.json:lifecycle.closureReserve; experiments/moriarty-language/spec/grammar.ebnf:12-83

Closure-reserve validation depends on unrepresented semantic categories: a distinct terminal resource-return action, a repeatable financial action, and the loan fixed-two-step exception. The source grammar and manifest have no action-role declarations, and no syntactic or semantic decision procedure classifies arbitrary actions into those categories. Thus an implementation must invent when the mandatory source guard is required. Naming the two supplied programs as examples does not define deterministic acceptance for another valid straight-line agreement, renamed cleanup action, or equivalent guard spelling.

Required fix: Define a decidable, exact reserve policy in this profile. For example, add explicit hash-bound action-role/closure metadata with validation and a precise required guard form, or specify a uniform mechanical reserve rule with explicit exceptions represented in source. State how equivalent guard expressions are treated and retain the nonresetting lifetime invariant.

## Verification performed

- Verified HEAD equals 5115d009fb4a679386593df22582040c96e06a8d and git status --porcelain=v1 is empty before and after read-only checks.
- Recomputed the candidate SHA-256 from compact sorted JSON excluding candidate_sha256: fa7ba0b44a5f886120b40e120ba041faf9d66ede7ea5f905da43f3e6232c42ec. Rehashed all 20 manifest files; zero mismatches.
- Read the actual grammar, numeric profile, bounds, semantics, typed schemas, proposal, and both full loan/swap sources. Inspected verification-model code before execution; no reviewer verdict was consulted.
- Compared complete crosswalk source dictionaries against retained CSV rows using multisets: exact equality for all 32 ACTUS and all 72 DeFi rows. All 104 rows retain mc07_mandatory=true. Reviewed DS-01 through DS-07 foundation/extension separation and the explicit future-version boundaries.
- Ran the candidate generic EBNF recognizer against actual grammar and source: loan accepted at 1056 tokens; swap accepted at 945 tokens. This is a source recognizer, not an implemented Moriarty frontend.
- Reran the dependency-free source verification model: exact effect schemas and Amount-policy occurrence coverage passed (loan 14 targets, swap 12), canonical/action/bounds vectors passed, quantum 20/10 succeeds and 15/10 rejects, obligation-model checks passed.
- Independently recomputed financial values with fractions.Fraction: loan interest 2480000000/73, floor 33972602, remainder 54/73, settlement 533972602, remaining notional 4500000000; swap output 1994000000/100997, floor 19743, reserves 1010000 and 1980257. Inspected nominal A*B/A typing and checked UInt128 intermediate rules.
- Measured actual source sizes: loan 5994 UTF-8 bytes, swap 5377. Maximum source string lengths are 68 and 63 UTF-8 bytes respectively. Reviewed separate finite AST/typed/manifest/signing/evaluation/result limits and the non-self-referential CountBody definition.
- Checked the signature/trace/proof dependency order manually: no direct digest cycle remains; claim requirements are generated from fixed built-ins plus distinct policy claims and evidence is outside signed/trace preimages. Trusted checks explicitly reject unauthenticated caller booleans, but exact outcome ActionCall context binding remains finding MC01-G6-03-02.
- Confirmed the concrete min_out witness has different action hashes: min_out=0 gives 8e1574b9a59407733294ff6b2c4bbb81290b76d5e8622769f5803d5c9d70870f; min_out=1 gives a0bbde157dd66029f786886fa383eee618e4cddee0e0529db63460fdde9722f5. Source inspection confirms min_out appears only in the parameter declaration and one passing guard.
- Reviewed obligation identity/tombstones, exact full settlement, transfer matching, independent episode/agreement status, authenticated principal/provider/genesis interfaces, nonce/current-state fail-closed requirements, and historical domain preservation.

## Residual limitations

- This bounded source review does not establish frontend/compiler execution, actual proof verification, PCD, custody, ledger movement/consumption, or full target conformance. None was required as a prerequisite for this source-profile review.
- Did not construct complete canonical AST/TypedProgram/BoundProgram/EvaluationInput/Complete artifacts or certify their aggregate representability; conflicting unit/value encodings and underspecified source spans prevent a unique normative construction. Syntax acceptance and source-model checks do not substitute for that construction.
- The reused recognizer and source-check model were inspected and rerun, but are candidate provenance rather than independent frontend or proof implementations. Arithmetic and CSV equality were additionally computed independently.
- No network, installs, builds, native proofs, wallets, repository edits, other model consultations, or new Fable verdict inspections were performed. Only requested audit reports were written outside the worktree.
- Other source-profile defects may remain; findings are bounded to the inspected frozen bytes and the stated review interval.
