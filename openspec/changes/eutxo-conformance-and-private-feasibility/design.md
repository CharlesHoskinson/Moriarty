# Conformance design and evidence gates

Status: S2, written proposal or design. All implementation and native tasks are specified-only.
This draft makes no semantic, native, private, PCD, or Complete approval.

## Inputs and provenance

Base commit: `84f64b30df53ff0b8ccaf475f57324d3d7c6e594`.
Task evidence root: `/home/charl/moriarty-eutxo-validation-2026-10-12`.
Research root: `/home/charl/moriarty-utxo-research-2026-10-12`.
`PATTERN-INVENTORY.json` names twenty families; no family is removed.
`PATTERN-RESULTS.md` and `ASSESSMENT.md` preserve source limits and Stage00 scope.
All five designer source suites, expected outputs, and verifiers remain evidence inputs.

Reviewed design: `next-backend-plan/revision03/CANDIDATE-SHA256.json`, SHA-256 `dcc62e21e9e351843bd04a3be537ebe260f8c1d0ec606cfcc5e15fd83ca7aff0`.
Both revision03 design reviews approve only scoped T1 design and representation.
Revision01/revision02 dissent remains historical evidence.
The receipt-helper revision02 approval covers its exact helper, not a native driver.

Independent source packet: `t1-independent-oracles-19/`, packet digest `9d71e736e3d3c159a83ff3bb21dbb3f5b2a9661dd90ed1e70414c2690b6552d6`.
Its RESULTS envelopes are source evidence; compiled templates are specified-only.
Implementation18 frozen candidate digest reported by primary: `aed4090af8f1e94e3904140ac2b23be9aa79e85141806841f1066115e5386680`.
That digest is context, not this draft's code approval.
Superseded 2026-10-24: adoption and acceptance follow AGENTS.md (factory predicates + holdouts); no vote or reviewer approval is required.
Observed backend file hashes are partial moving observations unless their full candidate manifest verifies.

## Prospective typed interfaces

These interfaces do not claim currently available APIs.
Implement them under `experiments/moriarty-eutxo/src/conformance/types.mjs` with matching JSON schemas.

```typescript
type Layer = 'source' | 'representation' | 'admission' | 'uplc' | 'transport' | 'ledger' | 'private';
type Artifact = { path: string; sha256: string; bytes: number };
type LayerOutcomeV1 = {
  version: 'LayerOutcome/1'; layer: Layer; status: 'accepted' | 'rejected' | 'unrun' | 'partial';
  fullEnvelope: unknown; rawOutput: Artifact; failureCode: string | null;
  reachedPredicate: string; comparedExpected: Artifact | null;
};
type ConformanceCaseV1 = {
  version: 'ConformanceCase/1'; inventoryId: string; requirementIds: string[]; taskIds: string[];
  sourceProfile: string; action: string; source: Artifact; input: Artifact; expected: Artifact;
  ordinaryPre: unknown; financialPre: unknown; arguments: unknown; observations: unknown;
  generator: { version: string; seed: string; bounds: unknown; shrinkTrace: Artifact | null } | null;
  compileManifest: Artifact; runtimePins: Artifact | null; outcomes: LayerOutcomeV1[];
};
type PrivateCapabilityProbeV1 = {
  version: 'PrivateCapabilityProbe/1'; route: 'cardano' | 'midnight-ledger-anchored';
  proverExport: string | null; verifierExport: string | null; ledgerExport: string | null;
  toolchain: Artifact[]; relation: Artifact | null; publicInputOrder: string[];
  verificationKey: Artifact | null; validFixture: Artifact | null; negativeFixtures: Artifact[];
  missingCapabilities: string[]; measuredRefusal: LayerOutcomeV1 | null;
  disposition: 'engineering-gap' | 'measured-limit' | 'bounded-feasible' | 'unverified';
};
type FinalAssessmentV1 = {
  version: 'FinalAssessment/1'; inventory: Artifact; candidate: Artifact; cases: Artifact[];
  families: { inventoryId: string; owner: string; requiredTasks: string[]; outcomes: LayerOutcomeV1[] }[];
  reviews: { identity: string; candidateSha256: string; scope: string; verdict: string; receipt: Artifact }[];
  nativeFull20: boolean; privateVerified: boolean; pcdVerified: boolean; completeVerified: boolean;
};
```

## Dependency boundaries

Compiler draft supplies exact versioned lowering, source/Core/schema pins, ABI vectors, and spending UPLC hashes.
Ledger draft supplies authenticated context, identity, authority, and injective effect-output allocation.
Finance draft supplies independently specified complete economic envelopes and gross/net policy checks.
Native draft supplies runtime pins, POSIX witness, immutable transaction receipts, readbacks, and reached failure phases.
This package consumes their artifacts; it does not accept a producer's success flag as independent evidence.
Package dependencies are artifact dependencies, not permission to skip missing behavior.

The ledger proposal removes a compile-policy hash cycle using the own ScriptContext credential first.
That is an unapproved amendment beyond frozen revision03, not an adopted T1 design correction.
Compiler, ledger, and native consumers need matching reviewed pin-generation versions before integration.

## Full correspondence

Compare whole acceptance objects: status, complete ordinary post, complete financialPost, ordered effects, and workRemaining.
Compare whole rejection objects, including code, diagnostic details, spans, failedSourceText, action index, and early workUsed where present.
Retain list order, unrelated rows, allowances, obligations, used IDs, closure reserve, and cumulative liabilities.
Never use a field subset, fixture label, allPassed flag, or observed output as an independent expected value.
Some historical expected files assign effect values from observed positives; retain them with this limitation.
Freeze new expected full effects before candidate observation.

Preserve revision03 HeadDatum's thirteen fields and LifecycleState/1 representation.
Transport revision, transport remaining, ordinary revision, and lifecycle work are distinct.
Redeemer Release/Refund/Cancel tags select actions; actor and time cannot substitute for the selector.
E12 source acceptance retains outstanding 40; T1 admission refuses NONEMPTY_OBLIGATIONS.
Source /6 interval-boundary is not the selected T1 /5 compile input.

Exact source codes remain distinct from other layers:

| Source observation | Different boundary |
| --- | --- |
| GUARD_FAILED | phase-two script failure |
| WORK_EXHAUSTED, workUsed 48 | kernel INSUFFICIENT_WORK |
| INVARIANT | ABI arithmetic OVERFLOW |
| INVALID_AMOUNT | ABI AMOUNT_BODY |
| SCHEMA | ABI SCHEMA_VERSION |
| INVALID_IDENTIFIER | ABI IDENTIFIER |
| UNKNOWN_FIELD | deployment-domain mismatch |
| PROFILE_MISMATCH | rejected /6 admission |
| E12 FundedExpressionPrepared | T1 NONEMPTY_OBLIGATIONS |

## Property and metamorphic examples

Proposed tests live under `experiments/moriarty-eutxo/tests/conformance/`.
Freeze each case before implementation and retain complete expected envelopes.

```javascript
// Proposed harness calls; these are not existing repository APIs.
assert.deepStrictEqual(caseResult.source.fullEnvelope, frozenExpected);
assert.deepStrictEqual(decodeAcceptedHead(caseResult.uplc), frozenExpected.financialPost);
assert.deepStrictEqual(projectCompleteEffects(caseResult.ledger), frozenExpected.effects);
assert.deepStrictEqual(replay(seed, frozenCaseBytes), retainedRun);
```

Use unsigned bounds 0, 1, maximum-minus-one, maximum, and maximum-plus-one.
Use identifier lengths 0, 1, 64, and 65 with malformed UTF-8 and leading digits.
Mutate source bytes, action tags, args, observations, program/domain/instance/head/nonce, roles, policy IDs, and body bytes independently.
Test wrong genesis, forged thread, wrong address, reference-versus-spent token, and substituted asset names.
Test same-output double satisfaction, fee omission, refund gross-cap evasion, missing residual duty, and replay.
Test ordinary/work/transport counter substitution and constructor omission.
Record sequential stale spend separately from concurrent fill/fill and cancel/fill races.

Permitted metamorphic control: add an unrelated admitted Witness balance row and preserve it in the complete successor.
Permitted differential control: Lender pre-balance 7 implies release post-balance 107.
Forbidden assumption: reorder balance rows or effects without semantics that authorize that transformation.
Short-circuit and eager work costs need separate traces; structural node counts alone cannot prove dynamic semantics.

## Actual AP04 source seam

Inspected factories expose `elaborate(source)`, `check(source)`, and `evaluate(source, actionName, snapshotCanonicalJSON, financialPreStateJSON)`.
Source files: `experiments/moriarty-language/src/successor/financial-agreement-source-v5.ts` and `financial-agreement-source-v6.ts`.
These exports do not provide a prover or certificate verifier.
The designer's actual unsupported sources are `designers/05-authority-private/unsupported-certificate.mori` and `unsupported-private.mori`.
The preserved outcomes are OPERATION_BINDING and UNKNOWN_DECLARATION.
This draft's bounded read-only probe records fresh complete check envelopes without rewriting designer evidence.

Reproduction uses real `/6` `check` calls:

```javascript
const api = createFinancialAgreementSourceV6();
api.check(readFileSync('unsupported-certificate.mori', 'utf8'));
api.check(readFileSync('unsupported-private.mori', 'utf8'));
```

Outputs: `05-conformance/AP04-SOURCE-PROBE.json`, command log, exit status, and input hashes.
No private witness, proof, key, wallet, node, or submission is required for this refusal probe.

## Private feasibility and canonical PCD

The canonical Moriarty model is ledger-anchored certified state transitions with bounded native certificates.
`openspec/PCD-INTEGRATION-2026-09-11.md` specifies the Midnight ledger-10 verify_proof seam.
Contract proofs use Blake2b transcripts; that seam accepts Poseidon zk-stdlib proofs.
This is a distinct platform path, not an existing Cardano whole-transition verifier.
Do not transplant ledger-10 exports into a Cardano API claim.
BLS support alone does not implement Nova, SlimNova, commitment openings, or ledger history compliance.

AP04 closure needs a pinned whole-relation prover and native verifier.
The relation binds program/key/domain, instance/head/predecessor/nonce, full pre/post states, effects, intents, caps, and invariants.
Its public-input order and encoding must be closed and independently reviewed.
Measure valid proof verification plus bit flips, substituted keys, wrong domain/head, missing openings, arity changes, aliases, and malformed points.
Require actual ledger acceptance of the same proof-bound transition.
A local verifier pass alone does not demonstrate private ledger settlement.
An unrelated arithmetic proof does not establish the financial relation.

Where no concrete callable verifier exists, record the inspected export path and absent relation as an engineering gap.
Do not fabricate a callable API, mock proof, or successful negative test.
A measured resource limit needs exact code, parameters, raw output, and a bounded experiment.
Missing implementation alone cannot establish infeasibility.
New source proof syntax requires a versioned profile and reviewed source/compiled semantics.
Public compatibility, private correspondence, mandatory Preview, PCD, and Complete remain separate assessment fields.

## Runtime pins and randomness

Consume the native package's measured RuntimePins/1 manifest.
Generate public fixture identifiers from a pinned deterministic seed where suitable.
Record random native key/policy generation commands and public output bytes in a later isolated authorized run.
Do not record private seeds or credentials.
Compile actual policies and spending scripts, hash their bytes, and verify independent hash parity.
Reject zero samples, equal policies, malformed policy lengths, wrong networks, or unbound instance hashes.
Freeze a generation-specific manifest before UPLC/native comparisons.
Changed genesis, source calibration, scripts, or policies invalidate dependent cases.

## Final assessment and graph update

Prospective output: `experiments/moriarty-eutxo/artifacts/conformance/final-assessment.json`.
Require every family requirement-to-scenario-to-test-to-receipt chain.
Require source semantics, canonical ABI, generated spending UPLC, and actual ledger readbacks where claimed.
Require fresh independent exact-candidate code and actual-result reviews.
Preserve original Luna/Grok/Sol identities, prior failures, and dissent.
Graph extraction identifies structural relationships; semantic review requires separate scoped evidence.
A later graph update must consume the reviewed assessment and preserve source locations and evidence tiers.
No graph mutation occurs now.

## Verification and limits

Run only this draft's structural validator during drafting:

```text
openspec validate eutxo-conformance-and-private-feasibility --strict --json --no-interactive
```

Save stdout, stderr, exit code, and hashes under `05-conformance/`.
Planned tests and native/private probes remain unchecked until actually performed.
Twenty-family native compatibility, private/PCD, mandatory Preview, and Complete remain unverified.
