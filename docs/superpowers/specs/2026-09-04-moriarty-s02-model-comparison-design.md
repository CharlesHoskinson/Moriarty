# S02 bounded architecture comparison

Status: independently reviewed experiment design. Classification: recommendation and specification.
This preparation does not pass S02, select an architecture, or change Core.
Execution requires the completed S01 gate.

## Authority and purpose

XML v1.3 W1 requires four architecture alternatives. S02 requires their
specification, model checking, and an evidence-backed selection or stop decision.
The frozen S01 judgment supplies the common authorization interface.
Its candidate status does not imply any model or implementation theorem.

Compare where execution meaning, authorization, and correspondence obligations
reside. Equal authorized traces are desirable. Four labels over one execution
function do not constitute four architectures.

The original prompt, S01 design, audit supplement, and Core scope remain immutable.
The active semantic scope remains `0.0.0-e00.2` throughout S02.
Hypothetical extensions are model experiments, not accepted semantic motions.

## Approaches considered

One shared interpreter with four configuration labels would simplify execution.
It would not expose differences in language semantics or correspondence obligations.
That approach does not satisfy this design.

Four complete production prototypes would permit direct deployment comparisons.
They would require unapproved Core motions and premature compiler work.
They also would confound architecture selection with unequal implementation maturity.

The proposed experiment uses four distinct finite execution representations.
It applies one observable workload, one authorization contract, and comparable attacks.
Independent trace checks connect the existing-Core slice to repository behavior.
Production correspondence and cost remain later obligations.

## Participants and communication

The model includes Alice, Bob, and Mallory as opaque participant identifiers.
Roles include the authorizer, solver, execution mechanism, and local verifier.
Participants act through shared abstract transaction and authorization state.
The experiment does not model a distributed messaging protocol or consensus.
Therefore, plain Quint shared-state models are appropriate. Choreo is not required.

The solver can select an enabled operation or propose a substituted artifact.
The environment can change the abstract state anchor or implementation version.
The trusted local checker evaluates actual candidate effects and signed bounds.
It does not accept a solver-supplied boolean as proof of effect authorization.

Cryptographic authenticity and complete effect extraction remain explicit premises.
The model distinguishes available, unavailable, and invalid evidence.
A premise flag represents an external assumption, not a mechanized cryptographic result.

## Scope and finite domains

Two workloads provide the minimum comparison.

1. The canonical swap covers both deposits, acceptance, voluntary refund,
   deadline refund, input rejection, and timeout priority.
2. A two-installment obligation covers partial consumption, residual authorization,
   cancellation, and both outcomes of the cancellation race.

The swap uses the actual example quantities and assets from `moriarty/swap.py`.
The installment obligation divides one bounded ten-unit authorization into two
five-unit payments. Its initial authorization explicitly permits both installments.
Residual authorization must bind that parent and its remaining budget.
An unsigned or substituted residual does not acquire authority.

Use two assets, two installments, two nonce values, and two implementation versions.
Use fresh and stale state anchors. Use time classes before, at, and after deadline.
Asset and participant identifiers have no interpreted string structure.
Where conservation depends on quantities, retain exact integer arithmetic.
These bounds limit the experiment. They are not universal coverage claims.

The E00-compatible slice excludes new Core constructors and credentials.
The lifecycle-extension slice identifies every behavior absent from frozen Core.
In particular, a Core partial-payment warning is not a signed partial-fill policy.
Minting, external calls, confidential cryptography, and cross-domain recovery remain excluded.

## Four execution representations

### A: agreement Core with an intent envelope

Agreement state contains a continuation, accounts, choices, and minimum time.
Authorization state contains signed bounds, nonce consumption, and residual authority.
The agreement machine computes legal transitions independently of the envelope.
The envelope checks the resulting complete effects before an authority boundary.

The bounded agreement representation contains finite constructor nodes and successor
indices. It preserves `Close`, `Pay`, `If`, `When`, `Deposit`, and `Choice` behavior.
Reduction steps remain distinct from externally observable transaction commits.
An input error restores the original transaction state and financial effects.
At the deadline, timeout reduction precedes input matching.

An agreement-legal transition can violate the envelope and must fail authorization.
An envelope-approved effect cannot bypass the agreement continuation.
The agreement interpreter and envelope predicate must remain separate definitions.

### B: intent Core with agreement libraries

Native state contains a finite obligation graph, satisfied dependencies, balances,
available capabilities, residual quantities, and nonce consumption.
The solver can choose an enabled native obligation.
A native transition discharges that obligation under the signed hard predicates.

The agreement package translates scheduling, order, and refund behavior into graph
dependencies and guards. Native execution does not call an agreement AST interpreter.
Package-to-intent correspondence is a separate check from native authorization safety.
Removing a prerequisite must expose premature settlement in a negative control.
Changing package timeout or refund rules must expose a correspondence failure.

### C: two calculi with a refinement bridge

State contains independent agreement and intent states, plus pending bridge evidence.
Each calculus computes its proposed successor independently.
The bridge binds predecessor states, versions, effects, and intended successor states.
An unresolved or invalid bridge cannot authorize financial commitment or consumption.

The bridge checks the declared correspondence relation between both results.
It is not an alias for the shared envelope acceptance predicate.
Agreement-only advancement, intent-only consumption, and stale bridge evidence require
separate negative controls. Successful correspondence commits the paired result.

### D: Compact library with a local verifier

State contains application-specific library phases, balances, artifact identity,
exported-call inputs, actual call effects, and local verification state.
The swap operational table comes from the pinned specialization's phase behavior.
The installment library is a proposed bounded model, not a deployed implementation.
No generic agreement or intent language interpreter runs in this alternative.

The verifier compares actual call effects against the signed intent and advertised
summary. Changing call effects while retaining a safe summary must fail.
The library model does not establish actual Compact or ledger correspondence.
D receives the same independent-checking allowance as the other alternatives.

## Authority boundaries and transition granularity

The lifecycle distinguishes drafting, resolution, authorization, verification,
financial commitment, partial completion, cancellation, refund, and terminal outcome.
Service labels are not substitutes for these semantic states.
Failed checks may record a rejection reason but cannot change financial state
or consume authorization.

`SignAfterResolve` binds a concrete plan before signing. Execution rechecks state
freshness and the applicable authorization. A later substituted plan must fail.
`SignBeforeResolve` binds bounded intent and the enforcement mechanism before resolution.
The later plan must pass execution-state verification before assets or authority
are consumed. The model never labels that unresolved plan pre-sign verified.

Resolve, sign, change environment, verify, and commit are separate actions where
their interleaving can invalidate authority. Internal agreement reductions remain
inside a transaction until its commit or rollback decision.
Cancellation and a fill compete for the same applicable consumption state.
The losing operation cannot consume the same authority again.

Every action has an explicit guard and assigns all state variables.
Every map starts with all required keys. Genuine terminal states stop.
A blanket no-op transition cannot conceal an unexpected deadlock.

## Properties and witnesses

The common safety floor contains these predicates:

- Executed effects satisfy the complete signed envelope, including multiplicity.
- Balances remain nonnegative and conserve each asset across deposits and payouts.
- Refund and change destinations match their signed predicates.
- Consumption requires applicable authorization and rejects replay.
- Cancellation excludes subsequent consumption of the same authorization.
- Residual amounts conserve the initial budget and cannot expand authority.
- State anchors, versions, and artifact identities match at authority boundaries.
- Rejection preserves financial state and authorization consumption.
- Display or disclosure permission cannot add an authorized effect.
- A settlement claim does not exceed its declared evidence level.

Every safety property must reference actual state, not a constant truth value.
Positive witnesses establish settlement, voluntary refund, and deadline refund.
They also establish rejected extra effects, first installment with residual,
both cancellation-race outcomes, and both signing profiles.
At least one authorized execution must follow verification in every candidate.
A candidate that rejects every operation fails the nonvacuity requirement.

## Negative controls and correspondence

Use each control against the representation whose protection it tests.

1. Remove the second-deposit prerequisite.
2. Preserve an acceptable summary but substitute the executed artifact.
3. Reverse timeout priority at the deadline.
4. Repeat the first installment or enlarge its residual authority.
5. Corrupt only package elaboration, bridge mapping, or effect extraction.
6. Substitute a plan after signing or omit execution-state verification.

Each security-critical mutation must produce a reachable counterexample under
unchanged remaining premises. Record the path and violated predicate.
Independent guards can make a mutation redundant. Classify redundant or equivalent
mutations explicitly, with evidence of the remaining protection.
A critical mutant survivor blocks the package. A justified redundant mutation
does not penalize stronger defenses. Trivial rejection of an unreachable attack
is not mutation evidence.
Generator and checker cannot share the code whose correctness they compare.

For the E00 slice, compare model traces with the pinned Python Core's actual
accepted transactions, errors, warnings, ordered payments, accounts, choices,
continuation, and minimum time.
Record any abstraction map used for amounts, time, or internal reduction steps.
The correspondence check must detect an independently mutated abstraction map.

Include two paired deadline witnesses. With no supplied input, timeout reduction
commits its refunds. With a supplied input, reduction can reach `Close` before
input matching. The resulting failure restores the original transaction state,
including continuation, minimum time, and refund effects.

## Execution evidence and package gate

Build each model incrementally. Typecheck, execute, and check reachability before
adding the next transition. Keep scenario tests in separate `_test.qnt` modules.
Instantiate every model constant explicitly.

Use reproducible sampled runs during construction. Record seeds and domain bounds.
The final S02 result also requires actual model checking, not only simulation.
Use `.qnt` source and Quint's `typecheck`, `run`, `test`, and `verify` commands.
Use `quint verify --backend apalache` for final model checking.
Pin the Quint and Apalache versions and binary hashes.
Follow the modeling and language skills from the
[Quint LLM Kit](https://github.com/quint-co/quint-llm-kit).
The user's 2026-09-04 direction excludes a direct TLA+/TLC workflow.
Record whether coverage is bounded-depth exploration or complete finite-state exploration.
Record the exact invariant, initializer, step operator, and termination behavior.

The OpenSpec package must define immutable inputs, exact outputs, acceptance
predicates, negative controls, failure outcomes, rollback, and scope transition.
Its validator must recompute package-specific evidence and reject stale receipts.
No S02 gate passes while a candidate lacks a required check or positive witness.
The package does not pass any unrelated XML release gate.

## Decision rule and stop conditions

Apply the common safety and nonvacuity floor before comparing other properties.
Then compare preserved semantics, required semantic motions, trusted components,
correspondence obligations, representation size, and tested workload coverage.
Separate measurements from architectural judgments and preserve dissent.
Model-state counts and source length alone do not measure production costs.

A passing model cannot establish real proofs, compiler correspondence, ACTUS
coverage, production resource budgets, clean-builder reproducibility, or human preference.
S02 can select a research candidate with those later obligations explicitly open.
Do not mark unperformed work as a failed experiment to manufacture a stop decision.

If no candidate meets the floor, preserve the counterexamples and decide whether
a justified semantic motion can repair the cause. Otherwise stop the language path.
If evidence leaves a tie, identify the discriminating experiment before selection.
Do not relax mandatory terminal obligations or accept a new product trust assumption
without user direction.

## Design review

An independent architecture review requested two corrections before execution.
The mutation criterion now distinguishes critical controls from demonstrated
redundant defenses. The E00 projection now includes the complete transaction result
and both deadline commit and rollback witnesses. The re-review approved this design.
That approval is a design-review result, not evidence that any model passed.
