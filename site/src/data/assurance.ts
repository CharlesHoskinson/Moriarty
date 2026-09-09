/**
 * The assurance model: the layered vocabulary, the twelve properties, the four
 * mandatory proof claims and the trust boundaries.
 *
 * The layering is the substance. Establishing one layer does not establish the
 * next, and the three columns of a property never collapse into one.
 */

/**
 * Each layer is a separate obligation. Collapsing this chain is the standard
 * overclaim in this field.
 */
export const ASSURANCE_LAYERS = [
  {
    name: 'Semantic validity',
    holds: 'the agreement means what its operational semantics say it means',
  },
  {
    name: 'Translation validity',
    holds: 'generated Compact preserves the meaning of Moriarty Core',
  },
  {
    name: 'Compiler correctness',
    holds: 'compactc preserves the meaning of Compact in ZKIR',
  },
  {
    name: 'Circuit and proof correctness',
    holds: 'the proof establishes what the circuit claims',
  },
  {
    name: 'Ledger feasibility',
    holds: 'the ledger can actually accept the transaction',
  },
  {
    name: 'Construction and wallet correctness',
    holds: 'the transaction the principal signed is the one submitted',
  },
] as const;

export interface Property {
  name: string;
  /** The assumption that qualifies the existing evidence. */
  assumption: string;
  /** What Moriarty must do to carry the property into its own setting. */
  obligation: string;
}

/** Twelve properties. The count is fixed. */
export const PROPERTIES: readonly Property[] = [
  {
    name: 'Termination',
    assumption: 'V1 core only; no new recursive construct',
    obligation:
      'Core step decreases a well-founded measure; surface loops elaborate to finite Core',
  },
  {
    name: 'Finite maximum lifetime',
    assumption: 'every wait has a finite absolute bound',
    obligation: 'reject unbounded subscriptions; certify the bound in the manifest',
  },
  {
    name: 'Conservation of value',
    assumption: 'valid positive state and successful transaction',
    obligation:
      'prove abstract conservation, and separately prove correspondence to Midnight kernel effects',
  },
  {
    name: 'Positive accounts',
    assumption: 'valid initial state',
    obligation: 'make validity a constructor and transaction precondition',
  },
  {
    name: 'Closure and no residual internal value',
    assumption: 'a semantic execution path exists and can be submitted',
    obligation:
      'separate semantic closure from proving, ledger, witness and participant liveness',
  },
  {
    name: 'Quiescence and idempotence',
    assumption: 'pinned V1 evaluator',
    obligation:
      'preserve in Core; do not require one circuit to reduce an entire lifetime',
  },
  {
    name: 'Grouped and split input equivalence',
    assumption: 'only under stated valid transaction conditions',
    obligation:
      'reprove around atomic action sets — atomicity intentionally changes some equivalences',
  },
  {
    name: 'Determinism',
    assumption: 'canonical interval and serialization agreement',
    obligation: 'total deterministic step relation and canonical input ordering',
  },
  {
    name: 'Transaction count bound',
    assumption: 'abstract transactions, not ledger resource feasibility',
    obligation:
      'certificate carries maximum transitions and per-entry-point resource estimates',
  },
  {
    name: 'Authorization',
    assumption: 'does not validate arbitrary role-token policy',
    obligation: 'model credentials, capabilities and nullifier or replay rules explicitly',
  },
  {
    name: 'Continuation integrity',
    assumption: 'hash comparison does not imply availability',
    obligation:
      'bind continuation roots in zero knowledge; specify replicated availability separately',
  },
  {
    name: 'Cross-implementation correspondence',
    assumption: 'no single theorem covers every implementation lineage',
    obligation:
      'normative Core plus conformance vectors, differential tests and translation validation',
  },
] as const;

/**
 * Every accepted transaction carries all four. Fixed names, fixed order, none
 * optional. HistoryCompliance is the one that separates this from
 * contract-level verification.
 */
export const MANDATORY_CLAIMS = [
  { name: 'ContractInvariant', means: 'the agreement’s own rules hold' },
  {
    name: 'IntentRefinement',
    means: 'what executed refines what the principal authorized',
  },
  { name: 'TransitionValidity', means: 'this state transition is legal' },
  {
    name: 'HistoryCompliance',
    means: 'the predecessor history this extends is itself compliant',
  },
] as const;

export interface Threat {
  threat: string;
  path: string;
  control: string;
  /** The column nobody else publishes. */
  residual: string;
}

/** Thirteen trust boundaries. The count is fixed. */
export const THREATS: readonly Threat[] = [
  {
    threat: 'Malicious witness',
    path: 'local callback supplies fabricated private input',
    control:
      'circuit constrains commitments, signatures, units, ranges, freshness and authorization',
    residual: 'compromised local secrets and selective withholding remain possible',
  },
  {
    threat: 'Accidental disclosure',
    path: 'private-derived data reaches public state, output or a cross-call',
    control:
      'visibility types plus information-flow check; every generated disclosure reviewed in the manifest',
    residual: 'traffic analysis and intentionally disclosed correlations remain',
  },
  {
    threat: 'Compiler mistranslation',
    path: 'Core meaning changes in generated Compact or ZKIR',
    control:
      'independent Core interpreter, translation validation, differential traces, pinned reproducible toolchain',
    residual: 'compiler and validator bugs until proof and cross-check coverage is complete',
  },
  {
    threat: 'Invalid initial state',
    path: 'constructor violates positivity, bounds or accounting',
    control:
      'total well-formedness checker and constructor assertions; reject the deployment manifest',
    residual: 'a backend mismatch could bypass incomplete checks',
  },
  {
    threat: 'Token or unit confusion',
    path: 'the same integer used for different assets or for time',
    control:
      'token-indexed amounts and distinct time and duration types before lowering; runtime manifest checks',
    residual: 'downstream representations erase some domain distinctions',
  },
  {
    threat: 'Authorization replay',
    path: 'reuse of an old proof, capability or oracle statement',
    control:
      'contract and entry-point domain separation, sequence numbers, nonce or nullifier, deadline',
    residual: 'wallet or key compromise remains',
  },
  {
    threat: 'Oracle manipulation',
    path: 'stale, equivocated, replayed or unit-confused statement',
    control:
      'signed typed observation: source, feed, unit, timestamp, freshness, sequence, bounds, fallback',
    residual: 'the source can still lie; governance and dispute paths remain external',
  },
  {
    threat: 'Continuation loss',
    path: 'a hashed future branch becomes unavailable',
    control:
      'replicated content-addressed storage, preflight availability proof, participant export',
    residual: 'integrity does not create availability',
  },
  {
    threat: 'Runtime substitution',
    path: 'a planner offers an unintended transaction or artifact',
    control:
      'client recomputes Core hash, entry point, public effects, destinations, fees, versions and artifact hashes before signing',
    residual: 'a compromised signing interface can still deceive the principal',
  },
  {
    threat: 'Backend version skew',
    path: 'compiler, runtime, circuit IR, ledger or keys disagree',
    control: 'lock every version and artifact hash; conformance matrix; no implicit latest',
    residual: 'an emergency ledger upgrade can remove an execution path',
  },
  {
    threat: 'Resource exhaustion',
    path: 'bounded source expands into an impractical circuit or ledger state',
    control:
      'predeployment limits on Core size, transition rows, proof memory and time, state cardinality, transaction size',
    residual: 'estimates need a safety margin and live-network validation',
  },
  {
    threat: 'External contract call',
    path: 'an unreviewed contract violates value or liveness assumptions',
    control:
      'excluded from V0; later a capability manifest, allowlist, effect summary and separate audit evidence',
    residual: 'external behavior cannot inherit these guarantees',
  },
  {
    threat: 'Governance capture',
    path: 'specification or registry changed without review',
    control:
      'multi-party ownership, public proposals, reproducible releases, delayed activation, signed registries',
    residual: 'social-layer collusion cannot be eliminated technically',
  },
] as const;

/**
 * A single audit is not an adequate claim. These boundaries are audited
 * separately.
 */
export const AUDIT_BOUNDARIES = [
  'normative Core and proofs',
  'parser, type checker and elaborator',
  'Compact backend and translation validator',
  'generated circuits and artifact registry',
  'runtime and client verifier',
  'developer kit and interface',
  'optional oracle and composition protocols',
] as const;
