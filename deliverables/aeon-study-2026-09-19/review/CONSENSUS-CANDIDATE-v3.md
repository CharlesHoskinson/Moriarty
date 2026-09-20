# Shared PL position — revision 3, 2026-09-19

This candidate supersedes v1/v2 after explicit user corrections and expanded APSS, ZKIRv3 and jet scope. It is advisory architecture consensus, not a proof, formal Council runtime receipt or deployment permission. Reviewers may disagree. Each must record a verdict, blocking objections, caveats and the observation that would change their vote against this exact file hash.

## 1. Product contract

Moriarty is a permissionless programming language for all DeFi developers on Midnight. Any developer can author, compile, prove and deploy supported programs without Moriarty maintainer permission, reviewer receipts, Foreman/RP03 records, hosted solver membership or a project program allowlist. Objective native validity, payment of costs, cryptographic asset-owner authorization and mandatory correctness proofs remain required. Application-specific governance does not become language-wide deployment approval.

Compiled contracts must target ZKIRv3 and execute on Midnight. TypeScript/K evaluation and generated Compact source are intermediate evidence. Compact may be a compiler route only with a bound emitted ZKIRv3 artifact. Pin source/Core semantics, compiler, ZKIR major/minor and instruction surface, circuit/key identity and deployed verifier independently.

Programs are general compositions within supported finite semantics; ACTUS/loan/swap/vault examples test coverage, not eligibility. The optional managed multichain SDK/router is an application/research design, not the mandatory public Moriarty interface. Correct authoritative documentation accordingly. This user-requested documentation package has independent review and write scope; Aeon feature tasks cannot edit roadmaps or legacy completion records.

## 2. Provable intention and APSS

Separate developer contract predicates, user-signed formal intent, proposed execution and settlement receipt. A proof cannot establish an unexpressed natural-language wish. Signing displays must derive from canonical signed objects. Bind program/semantics, property relation, intent, domain, predecessor/history, observations, complete effects and bounds. Contract correctness, intent refinement, transition validity and history compliance cannot be replaced by a prover-selected vacuous relation.

APSS supplies role boundaries. Applications express useful operations and constraints. Permission concerns keys and bounded user authority, not developer approval. Solvers propose candidate executions and may optimize stated preferences. Settlement verifies and applies effects under explicit ledger/finality assumptions. Moriarty supplies semantics and proof interfaces across these roles; it need not operate every service.

Intent binds exact assets or explicit substitution predicates; recipients; gross debits, fees and liabilities; lifetime/replay/revocation; and allowed partial outcomes/remedies. Solver reputation does not prove validity; correct execution does not prove global optimality, liquidity or eventual inclusion. NEAR direct submission/optional relays and CAKE heterogeneous solver markets are lessons, not Midnight guarantees. Current CoW solver whitelists illustrate why 'permissionless' must name the actor and surface.

Local atomic rejection does not imply global ledger rollback. Current Midnight documentation distinguishes guaranteed and fallible phases: failure of the latter can preserve earlier effects and fees. Proof statements and lowerers must account for the selected phase layout and all failure outcomes. External async execution needs pending/partial/unknown states, remaining liabilities and explicit recovery. Timeout alone is not evidence of nonexecution. An attestation is an assumption about an observation, not a replacement for program correctness. Private composition must state disclosures, witness assumptions and predecessor guarantees separately.

## 3. Certified basis and jets

Recover the September 13 Simplicity-inspired workstream rather than restarting it. Borrow the discipline of a reference meaning and certified optimized implementation, not the assertion that all native code is already proved. No Simplicity or Aeon implementation is imported by this plan.

Each candidate primitive certificate identifies type/domain/range preconditions, reference semantics, target ZKIRv3 constraint fragment, implementation/version hashes, theorem statement/evidence and resource model. Prove call-site preconditions and composition/frame obligations. Preserve successful values, rejection, effects and declared logical work. Logical work, host speed and circuit cost are distinct. A new cost model requires an explicit profile change; calling a fast path a jet does not silently change observable costs.

ZKIRv3 correctness needs valid-execution completeness and constraint soundness for arbitrary satisfying witnesses, not merely honest witness-generator tests. PR17 at ebb662c716fef2638ec1b0f42805a8bce23c75dd provides conditional Agda results for the older 34-instruction/13-type surface. It is open, has an explicit Assumptions record, and excludes newer branch instructions/types. Its producer obligations must be discharged; backend assumptions and deployed-version correspondence remain work. No '--safe/no postulates' wording erases module assumptions.

Start with a tractable exact arithmetic primitive and demonstrate certificate plus composition plus target artifact. Do not fix 'roughly fifteen primitives' as settled architecture. Constructor counts, source drift and claimed existing proofs in the old study require fresh evidence. Financial effects, crypto primitives and privacy have distinct harder obligations.

## 4. Aeon feature decisions

Adopt concepts in a Moriarty-native implementation, not Aeon source code, runtime, generated artifacts or native FFI. Use a separately pinned solver dependency if needed.

A0: expose versioned trust/obligation inventory with explicit extraction coverage and unestablished proof/ledger links. This renders known evidence; it creates no assurance itself.

A1: exact bounded advisory refinement/VC checker after A0. Freeze a useful pure Boolean/UInt fragment and judgment before implementation. Start literals, variables, checked add/subtract, comparisons, short-circuit and conditionals. Add multiplication/division only with exact intermediate bounds, rounding, zero and rejection semantics. Unsupported nodes cannot disappear. Separate successful-execution safety, guaranteed definedness and assumption feasibility. Use explicit encoding-established, counterexample-validated, unsupported, timeout, unknown and checker-error results with separate feasibility/vacuity status. Record solver/encoder versions and budgets. Formula discharge is not source/target correspondence.

Replay decoded counterexamples through the pinned evaluator before validation. If an established safety claim contradicts that evaluator on the same admitted assumptions and successful-execution judgment, mark the checker unsound and fail its corpus. A safety-only claim must not be confused with guaranteed definedness. Freeze positive/negative/mutation fixtures and publish unsupported/timeout coverage; defer an unhelpful checker rather than letting it displace the real compiler/proof path.

A2: improve counterexample explanations and classify consumable receipts, affine authority and persistent liabilities. Do not force unused authority to execute or allow debts to vanish.

A3: typed holes and bounded synthesis only after exact checks demonstrate usefulness against manual templates. Search cannot weaken hard constraints, costs, reserve or signed intent. Generated programs use the same validation path as manual programs. Reject unchecked native escapes, arbitrary reflection, general recursion and treating optimization fitness as correctness evidence.

Primary delivery remains general source-to-ZKIRv3-to-Midnight with mandatory proof verification. Certified-basis work and useful A0/A1 work support it; with one implementation track, advisory authoring conveniences do not preempt it.

## 5. Requirements and delivery

Use the new permissionless-provable-intention OpenSpec change, EARS requirements MOR-001..011, AEO-001..004 and DEV-001 with positive/negative scenarios and explicit evidence. They are specified-only until implemented. Preserve historical roadmap and results. Correct stale K/runtime and Preview status by annotation; no K/MC/SP/PCD/correspondence item becomes newly complete merely from this documentation change. Rescope or remove administrative program restrictions, rather than preserving all old gates indiscriminately.

Internal Pel: immutable task brief, declared paths, registered roles, real host verification, then independent exact-candidate review. Failed verification skips paid review. Permit at most one correction, then retain needs-action. Changed candidates require new verification/review. No publish or financial dispatch in the sample. A checker task owns only its declared source/tests/fragment/new deliverable paths; changes to plugin/custody/ledger/sprints/Preview records fail scope checks. Never fabricate host authority, keys or receipts. Public programs need none of these project records.

APSS literature uses Diataxis explanation/reference/how-to/tutorial notes with source provenance, exact PDF visual coverage and explicit design inference. Preserve source disagreements and partial readings. Consensus requires six explicit endorsements of this same candidate; no tally can hide unresolved objections.
