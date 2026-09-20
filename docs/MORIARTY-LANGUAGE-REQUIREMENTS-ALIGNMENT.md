# Language syntax and semantics requirements reconciliation

Status: documentation and design reconciliation. No new language profile, implementation capability or theorem is accepted. Compared the current source/Core/lifecycle contracts with MPLR-001–035, UNI-001–017, ZR01–16 and MNR01–08 after publishing the consolidated research.

## Findings and corrections

1. The page combined current /5 grammar with detailed older repayment/expression rules and only a short current lifecycle summary. Expanded the /5 boundary with admission order, explicit action selection, immutable financial PRE, Ensure-only financial POST, ordered protected operations, local rejection, full work accounting, output re-admission and the distinction between local JSON and authenticated ledger state.
2. The current same-name source-unit/settlement-asset restriction is narrower than the target's domain-qualified financial model. Made that current limitation explicit rather than claiming future asset semantics are implemented.
3. An E + N work sentence could be read as applying to every profile. Scoped it to its financial PRE-read profile and identified /5 suffix work separately. Existing equations and canonical grammar remain unchanged.
4. The page linked the requirements agenda but gave no complete comparison. Added all 35 MPLR rows and all 24 backend/recursion rows, with local foundations and open obligations. No full requirement is marked satisfied by a partial local mechanism.
5. Current implemented profiles do not define the full planned authenticated continuation, evidence, conditional settlement, phase, recovery, delegation or private history contracts. Added the required semantic interfaces as specified-only design. Their concrete syntax, typing/effect rules, Core encodings and proof obligations must be frozen under U0; labels on the page are not source keywords.
6. Native proof, compiler, ledger, federation and service obligations require different enforcement points. Retained their ownership and required evidence rather than treating syntax additions or finite tests as backend assurance.

## Current sources inspected

- [Source /5 contract](../experiments/moriarty-language/spec/successor/financial-agreement-source-v5.md)
- [Canonical /5 grammar](../experiments/moriarty-language/spec/successor/financial-agreement-source-v5-grammar.ebnf)
- [Source /5 public API](../experiments/moriarty-language/src/successor/financial-agreement-source-v5.ts)
- [Shared compiler and evaluator dispatch](../experiments/moriarty-language/src/successor/financial-agreement-source-compiler.ts)
- [Source /5 tests](../experiments/moriarty-language/tests/financial-agreement-source-v5.test.mjs)
- [Local lifecycle tests](../experiments/moriarty-language/tests/successor-lifecycle.test.mjs)
- [Loan lifecycle tests](../experiments/moriarty-language/tests/loan-lifecycle.test.mjs)
- [Consolidated design](MORIARTY-CONSOLIDATED-DESIGN.md), [backend contract](MORIARTY-BACKEND-REQUIREMENTS.md), [MPLRs](../wiki/research/mplr/index.md), [EARS](../openspec/changes/consolidated-language-kernel/requirements.md) and [traceability](../openspec/changes/consolidated-language-kernel/traceability.md)

## Target semantic interfaces

### One authenticated stage contract

The target acceptance relation must connect the program and semantic profile to the signed intention, current state and authority, predecessor commitments, observations, complete effects, fees, resource limits and residual duties. Contract properties, intent refinement, transition validity and history compliance are separate required judgments over this connected statement. Source, Core, ZKIR, proof keys and ledger effects have distinct identities linked by pinned correspondence, not equal hashes by assumption. Every mandatory predicate must govern every accepting path, for every accepted witness. A host Boolean, an unused assertion or a verifier chosen by the prover cannot replace that enforcement.

### Conditions, authority and obligation formation

The language needs typed evidence policies and condition combinations, including conjunction and threshold conditions, bound to issuer, domain, request, freshness, finality and consumption rules. Recording a request, funding escrow, establishing release eligibility and establishing delivery denote different facts. Stage authority must distinguish initiation, completion, reconciliation, recovery, disclosure and amendment. Creating or increasing an obligation requires the affected party’s applicable consent; receiving positive value alone need not require interaction. Exact-plan and outcome authorization already have local atomic-profile checks; that does not establish signed native authority for the /5 lifecycle.

### Persistent continuations and partial outcomes

A continuation must preserve the program, causal predecessors, awaited evidence, remaining choices, cumulative spending and outstanding duties. A successful prefix of a distributed workflow can survive later failure. The result model therefore needs to distinguish an uncommitted rejected candidate, an accepted partial stage, an unresolved external outcome and terminal delivery or recovery. These are semantic distinctions, not newly accepted source keywords. Availability and success policies for joins are separate, and late or unselected branches retain their duties. Local all-or-nothing rejection cannot implement those distinctions by itself.

### Complete financial and resource accounting

Each stage must bind the complete asset and custody frame, authorized supply changes, gross debits, fees, net receipts, liability changes and residual duties. Assets require domain-qualified identity; quantities require explicit units, conversion, price orientation and rounding. Refunds do not reset spent authority. Netting requires a gross-to-net preservation relation, including liabilities. Concurrent solvers must reserve against the same authenticated budget. Local numeric and lifecycle checks are useful foundations, but local arrays do not establish authenticated absence or completeness of private positions.

### Recovery, state limits and evolution

Recovery must distinguish refund of controlled assets from compensation after committed effects. A timeout is an observation about time, not proof of nonexecution. Refund and late success need exclusive terminal consumption. Reserve work is unavailable to ordinary progress; a claimed recovery guarantee needs a viable supported path under stated assumptions, or must be rejected as a guarantee. Finite episodes may continue only through authenticated successor rules preserving lifetime budgets, duties and replay records. Changes to semantics, verifier, policy or federation epoch need preservation or applicable amendment consent; expiry and revocation do not erase existing debt.

### Native history, private handoff and composition

Proof recursion verifies earlier proofs while each source stage stays bounded. The target requires authenticated genesis, authorized relation and key identities, well-founded predecessor composition, bounded multi-parent joins and current-state consumption. Harmless shared ancestry is different from spending the same resource twice. The fan-in bound does not impose one fixed global history depth. Private composition must prove completeness of the relevant state domain and provide the authorized successor with needed witness data under a stated disclosure and availability policy. Supported private handoff must work without federation membership. Ledger induction and proof aggregation do not discharge full native recursive compliance, deferred verification or private split/join.

### External effects, wallet delegation and service delivery

A settlement implementation must satisfy its financial postconditions, not just an interface shape. An adapter binds exact submitted bytes, decoded effects and observations to the same intention, stage, domain and epoch. ZK, threshold signing and TEE evidence have separate trust assumptions; a destination accepting only a threshold signature remains exposed to threshold compromise. OWS delegation must preserve scoped authority across wallet, solver and signer boundaries. x402 workflows must distinguish payment finality, result availability and recipient delivery, retaining one authenticated logical request across retries and reconciling before a new authorized charge.

### Certified operations and honest authoring tools

A primitive or library certificate must preserve reference values, preconditions, failure behavior, complete effects and the declared cost relation for the admitted artifact. U0 must freeze the numeric profile: exact units, price orientation, rounding direction and beneficiary rules, bound into primitive and library qualification. In particular, adapting DeFiFormal’s quote-per-base prices to Moriarty’s current base-per-quote convention requires exact dimensioned conversion and directed rounding; renaming a type or taking the reciprocal of an already rounded value does not establish that correspondence. Lowering must exclude invalid adversarial witnesses, including bad ranges, non-Boolean selectors and unconstrained outputs. Resource certificates cover late-bound components under the pinned cost model. Solver completion and future typed-hole synthesis preserve every signed constraint and existing commitment; neither implies optimality or eventual completion. Tool output must distinguish specified behavior, local predictions, verified statements and finalized effects. An unsupported check or dummy proof cannot become acceptance evidence.

## Requirement coverage

The [published comparison](https://charleshoskinson.github.io/Moriarty/docs/language.html#requirement-coverage) records each MPLR’s local basis and missing full-scope obligation, plus every ZR/MNR contract. [U0–U7](../ROADMAP.md) remains the single delivery sequence. U0 must turn these interfaces into a versioned stage relation, source forms, static/effect judgments, result cases and target embeddings. Later phases implement and qualify them; this audit does not dispatch those phases or invent a source /6.

Maintain the current local grammar and formal rules until a separately versioned implementation and its evidence justify a successor. Source examples must declare their implemented profile; proposed syntax must be visibly non-executable. Ledger induction cannot close the full recursive/private-history obligations. No project approval becomes a public deployment condition.
