/**
 * Intents, multichain settlement and the safety mechanisms that follow from
 * them.
 */

/** Five artifacts most systems conflate into one. */
export const INTENT_ARTIFACTS = [
  {
    name: 'Intent',
    is: 'principal, scope, authority, requirements, assumptions, lifecycle, validity and replay data, semantic versions',
    fixes: 'what outcome you authorize, before any route is chosen',
  },
  {
    name: 'Permission',
    is: 'the authority the intent grants, scoped by asset, recipient and cap',
    fixes: 'how far that authorization reaches',
  },
  {
    name: 'Plan',
    is: 'intent identity, bounded steps, adapter identities, dependencies',
    fixes: 'one chosen route',
  },
  {
    name: 'Execution',
    is: 'the trace actually performed against the agreement',
    fixes: 'what happened',
  },
  {
    name: 'Receipt',
    is: 'intent and plan identities, checked effects, status, residual resources',
    fixes: 'what remains owed',
  },
] as const;

export const SIGNING_PROFILES = [
  {
    name: 'Exact-plan signing',
    means: 'you sign the concrete plan',
    tradeoff: 'narrower meaning; no solver choice after signing',
  },
  {
    name: 'Outcome-intent signing',
    means:
      'you fix authority, goals, permitted agreement programs, validity and nonce before a plan exists',
    tradeoff:
      'solvers search outside the finite checker; ranking cannot excuse invalid authority or a failed goal, and there is no global best-price claim',
  },
] as const;

/** The rules that make outcome authorization safe. */
export const AUTHORITY_RULES = [
  {
    rule: 'Authority is affine',
    detail:
      'Partial completion consumes it. Residual authority cannot grow or reset an epoch.',
  },
  {
    rule: 'Gross, not net',
    detail:
      'Sum gross outgoing transfers and fees by asset across all recipients. A refund never restores allowance. Fees have their own cap and still consume gross authority.',
  },
  {
    rule: 'Every recipient is enumerated',
    detail: 'Including refunded movements.',
  },
  {
    rule: 'Goals compare net credits',
    detail: 'Final minus initial. Fees count against the net goal.',
  },
  {
    rule: 'Asset identity is the whole tuple',
    detail:
      'domain, issuer, reference and kind. No ticker aliases. A receipt token is not the delivered asset.',
  },
  {
    rule: 'Validity is bounded and bound',
    detail:
      'notBefore <= now < expiresAt, with a nonce and a domain separator. An envelope’s key cannot appoint itself authoritative; verification uses the caller’s independent trust record.',
  },
  {
    rule: 'Pending progress has its own judgment',
    detail:
      'A pending receipt retains obligations. It can never show a terminal goal as settled.',
  },
  {
    rule: 'Anti-vacuity',
    detail:
      'A compiler that always rejects is not a working compiler. Require trace inclusion and a feasible positive witness.',
  },
] as const;

/** What signing surfaces render, derived from canonical signed meaning. */
export const SIGNING_SUMMARY_FIELDS = [
  'asset domains',
  'gross budgets',
  'allowed recipients',
  'net goals',
  'fees',
  'validity and nonces',
  'assumptions',
  'liabilities',
] as const;

/** The gap between what cross-chain systems claim and what they document. */
export const MULTICHAIN_CONTRADICTIONS = [
  {
    claim: 'Atomic cross-chain execution with automatic refunds',
    documented:
      'External calls complete asynchronously, simulation excludes them, and deposit, withdrawal, storage and indexer paths include detached, nonrefundable or manually recovered steps',
  },
  {
    claim: 'Non-custodial throughout',
    documented:
      'Contract-held internal balances, temporary transfers to a trusted swapping agent, and a treasury plus proof-of-authority bridge',
  },
  {
    claim: 'Tiered confidentiality labels',
    documented: 'No public normative leakage definition for the labels',
  },
] as const;

export const MULTICHAIN_RULES = [
  'Define atomicity per layer and per route. Never lift an internal ledger-batch atomicity up to bridge fulfillment.',
  'Publish a route-specific custody and authority manifest before approval.',
  'Model confidentiality as a named adapter profile with explicit disclosure and custody assumptions.',
] as const;

export const MULTICHAIN_CHALLENGE =
  'Construct a cross-chain swap trace in which the internal batch succeeds but the bridge withdrawal fails or is rolled back. Any model that cannot represent that trace, and reject it, is not modelling cross-chain settlement. Operator, relay, treasury and bridge attestations cannot discharge a proof obligation.';

/** Bridges split by trust model. Merging these is a category error. */
export const BRIDGE_TRUST = ['message-verified', 'custodial'] as const;

/** Four mechanisms, each answering a category of real failure. */
export const SAFETY_MECHANISMS = [
  {
    mechanism: 'Typed asset identity and units',
    detail:
      'Token-indexed amounts, distinct time and duration types, Debt<T> separate from Amount<T>, and the full domain, issuer, reference and kind tuple.',
    kills: 'unit confusion and ticker aliasing, at the type level',
  },
  {
    mechanism: 'Declared rounding and remainder policy',
    detail:
      'Every conversion names its direction, its remainder disposition and its proof obligation.',
    kills: 'the rounding-direction class of vault and share-conversion exploits',
  },
  {
    mechanism: 'Affine authority with gross accounting',
    detail:
      'Refunds do not restore allowance, fees consume authority, every recipient is enumerated.',
    kills: 'the authorization-scope class',
  },
  {
    mechanism: 'History compliance as a proof obligation',
    detail: 'A valid transition against an invalid predecessor is rejected.',
    kills: 'the class where each step looks locally fine',
  },
] as const;

/** How an adversarial fixture must be specified to be useful. */
export const ADVERSARIAL_FIXTURE_FIELDS = [
  'actor capability',
  'vulnerable layer',
  'precondition',
  'ordered effects',
  'violated predicate',
  'loss outcome',
] as const;

export const ADVERSARIAL_RULES = [
  'ABI shape does not establish semantic compatibility.',
  'A valid oracle signature does not establish economic truth.',
  'Flash borrowing is a capability with legitimate uses, not a vulnerability.',
] as const;
