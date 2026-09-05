# S02 common-foundation delegated design decision

Date: 2026-09-05. Status: adopted design with preserved dissent; implementation
and acceptance remain open. Classification: recommendation and local design
decision under the user's delegation. No A–D alternative is selected.

## Evidence and decision rule

All three requested experts independently read the same frozen inline sources.
Their [public proposals and receipt](../../../raw/reviews/s02-common-design-2026-09-05/receipt.json)
are SRC-0035. All recommend amendment. Astra and Grok prefer a combined package;
Fable prefers a separately reviewed observation prerequisite. Adopt the majority
combined boundary while keeping observations independently importable and tested
before authorization. This is not unanimous agreement on every detail. The
dispositions below preserve substantive disagreements and use the frozen design
to resolve them. Model identity limitations remain in the receipt.

This decision supersedes the old type sketch where they conflict. Its complete
reviewed bytes remain in the immutable prompt. No further direct human approval
is required. The requested Council implementation gates are not waived or
satisfied by these proposal-authoring outputs.

## Adopted structure

1. `observations.qnt` imports only `effects.qnt`. It defines neutral, complete
   carriers and structural/emission validators, not a Core interpreter. Keep
   `NoCoreProjection` distinct from an applicable complete result. Keep Core
   rejection separate from envelope rejection: a legal proposed Core transition
   can be rejected by authorization without becoming a fabricated Core error.
2. `authorization.qnt` imports observations and the unchanged effects/consumption
   foundations. Signed policies contain finite alternative clauses. Each clause
   binds observable inputs, times, effects, capabilities, and parent dependencies.
   Conditions within a clause are conjunctive; alternative clauses are disjunctive.
   A candidate's outcome label or display cannot authorize a clause.
3. Complete opaque candidate payloads remain equality-bound. A neutral plan view
   additionally exposes each planned operation's input, full effects, artifact,
   expected predecessor/successor, applicable projection, and slot dependencies.
   A common checker cannot inspect an opaque plan by magic. Candidate adapters
   must later establish view fidelity independently; this is not a common A–D
   execution interpreter.
4. Concrete harness payloads and all maps have finite domains. Instantiate both
   profiles explicitly. Pre-sign checks inspect unsigned policy content; signing
   records exactly that content; execution verification requires all applicable
   signatures. No pre-sign check depends on its own future signature or settlement.
5. Store per-key signing records and per-operation attempts/verification records.
   At least first fill, second fill, cancellation, and recovery can retain their
   own snapshots. First fill is not a terminal state. Both competing prepared
   fill/cancel observations bind the same revision and the loser records rejection.
6. Every operation uses the same execution verification and commit boundary.
   Required authority is debit-owner authority plus operation-specific authority.
   Cancellation requires the exact parent's signed cancellation capability even
   with no transfers. Recipients do not become signers merely by receiving money.
7. Financial and authority updates are atomic. First parent use claims nonce 0
   and moves its registry cell to `AuthorityConsumed`, retaining the exact signed
   parent. Further use is authorized only by that retained parent and valid
   residual, not by a general consumed-policy exception. Registry consumption
   revision equals the parent entry revision. Unused cancellation also claims and
   consumes that key. Recovery signs and consumes a separate Alice nonce 1.
8. Recovery reads actual escrow and requires it to equal the expected remaining
   budget in the ten-unit fixture. It refunds exactly that balance to Alice and
   leaves cancelled parent accounting unchanged. It cannot bypass evidence checks,
   re-open nonce 0, or use a cancellation flag as financial settlement evidence.
9. Successful authorized fixture commits use `transactionTime == physicalTime`,
   with finite times `{1,2,100,101}`. Adversarial stale or future proposals remain
   representable and reject without commitment. Fresh signature evidence may be
   obtained at execution for an existing signed policy; it is not a new signature.
   All evidence payloads and execution environments match the actual context.
10. Terminality is explicit: completed payment or completed recovery with no
    unresolved attempt. A separate classified rejected/aborted attempt does not
    claim successful financial completion. No blanket stutter is permitted.
    Nonterminal enabledness is the disjunction of the actual pure action guards;
    finite-domain exhaustion cannot be hidden by unrelated environment actions.

## Dispositions and dissent

| Issue | Expert input | Disposition and test obligation |
| --- | --- | --- |
| Package boundary | Astra combined; Grok combined; Fable split | Combined majority; test and hash the standalone observation module before integration. Preserve Fable's split preference. |
| Flat policies | Astra R3; Fable RC-01/02/11; Grok RC-INCOMING-ONE | Adopt branches over neutral input and state facts, with a single canonical policy body. Reject one-legged settlement and label-only refund. |
| Opaque plan check | Astra R1; Grok RC-BIND-PLAN-ARTIFACT | Adopt an inspectable neutral plan view plus complete opaque identity. Slot-2 and deposit mutations must invalidate an after-resolve check. Before-resolve may accept a different resolution only when it actually refines the signed bounds. |
| Optional Core result | Astra R2; Fable RC-03; Grok RC-HARNESS-EVIDENCE | Adopt optional projection and separate rejection reasons. A lifecycle-only cancellation has no invented Core result. |
| Parent nonce state | Astra R6 and Fable RC-15 consume on first use; Grok RC-AUTH-RESIDUAL retains Registered | Adopt the two-expert consumed-plus-exact-residual design. Grok correctly identifies the unreachable-second-fill bug if consumed means universally unusable; explicit residual applicability fixes it. Registered-through-residual is not adopted. |
| Exact signed parent | Astra R6; Grok RC-SIGNED-PARENT | Instantiate unchanged generic consumption types with the complete signed policy. Match both policy and revision at every use. |
| Effects-free cancellation | Astra R4; Fable open actor question; Grok RC-COMMIT-GLUE | Require signed operation authority, not an empty conjunction. Model unauthorized callers/proposals explicitly in a shared-state fixture; shared state does not prevent representing an actor. |
| Lifecycle and races | Astra R5/R10; Fable RC-07; Grok RC-PROFILE-GUARDS/RC-PREPARED-RACE | Per-key signing plus per-operation attempts supports two in-flight verifications, both race orders, slot 2, and retry. |
| Pre-sign freshness | Astra conservative serialized checks; Fable RC-13 and Grok RC-PRESIGN-FRESHNESS key-local checks | Bind candidate state, ledger, environment, the signing key, and all parent dependencies. Unrelated signature registration need not invalidate a check. Relevant parent consumption always does. |
| Recovery pipeline | Astra R6/R7; Fable RC-04/08; Grok RC-COMMIT-GLUE/RC-SIGNED-PARENT | Full resolve/check/sign/verify/commit under nonce 1. Initial positive fixtures sign recovery after the cancellation result is known, avoiding a premature concrete refund-10 binding. No registered-policy replacement is introduced in this package; Fable's supersession extension is deferred, not silently accepted. |
| Recovery amount | Fable ledger balance; Astra exact remaining refund; Grok balance equals allowance | Read ledger balance and require equality with expected residual for the declared fixture. A corrupted ledger is rejected, not used to relax conservation. |
| Execution time | Astra equality; Fable RC-05 permits past dates; Grok binds record time only | Equality for successful authorized commits blocks both future-dating and backdated successful fills. Preserve stale proposals for Core rejection tests. Fable's less restrictive inequality is not sufficient against post-deadline backdating. |
| Signature evidence freshness | Fable RC-06 weakens environment fields | Retain full current evidence bindings. A signature token and freshly obtained verification evidence about that token are distinct. Refreshing evidence does not require signing again or weakening policy version binding. |
| Finite domains and initialization | Astra R10; Fable RC-12; Grok RC-FINITE-DOMAINS/RC-ESCROW-FIXTURE | Concrete aliases, twelve authority keys, finite times/versions, bounded attempt records, and direct prefunded escrow initialization in the harness. Do not modify effects.qnt merely to add fixture convenience functions. |
| Core encoding | Astra R8; Fable RC-10; Grok RC-CORE-VALIDATOR | Structural validators preserve all fields, sparse-account decoding, absent versus zero choices, and ordered projections. Candidate A's later fixture starts with a When boundary before each installment; separate cancellation has no Core transaction. |
| Order and display | Astra R3/R8; Fable multiset authorization; Grok RC-DISPLAY | Retain ordered effects and multiplicity. Clauses may constrain order explicitly. Display cannot expand authority; substituting a verified observation still requires fresh binding, so canCommit need not ignore a changed signed/bound display. |
| Recovery coverage | Astra R9; Fable and Grok mandatory subscenarios | Preserve eleven existing witness IDs; encode both additional recovery subscenarios under both profiles and later every candidate. Missing traces fail coverage. |
| Terminal deadlocks | Astra explicit terminal stop; Fable RC-09 guarded terminal stutter; Grok terminal-only stutter | Use real terminal predicates and explicit guard-based nonterminal enabledness. Any backend-only terminal stutter must be terminal-guarded and tested; do not claim it as a lifecycle witness. |
| Tool claims | All proposals | Typecheck, deterministic tests, sampled runs, and authorized Quint/Apalache checks remain empirical obligations, not facts supplied by model advice. |

## Immediate implementation sequence

The first executable unit is the observation carrier with finite concrete tests:
exact error/warning validation, absent versus zero choice, accepted deposit
without a Core payment, rejection restoring original minimum time and state,
and optional projection. Then implement branch policies and neutral plan views,
both signing lifecycles, current evidence checks, and atomic parent/recovery
commits. Each action receives a deterministic test and sampled reachability
check before the next action. Later A–D execution and independent correspondence
remain separate plans. No source-level proposal proves their feasibility.

## Deferred empirical questions

Apalache tractability of complete records is unknown. Optimize only through
explicit checked abstractions. Fixed mechanism/version changes may permanently
invalidate a policy; rejection is evidence of that boundary, not a guarantee of
recovery availability. Candidate A's timeout authorization, all error/warning
fixtures, complete independent codecs, mutation adequacy, and final Council
acceptance remain required later work. Missing work is not an architecture stop.
