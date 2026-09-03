# Moriarty Compact DSL feasibility and SDK specification

<!-- markdownlint-disable MD013 MD060 -->

- Sprint: S00 instruction and feasibility sprint
- Feasibility status: S3 prototype, reproduced in a clean environment
- SDK status: complete architecture and component contracts; implementation is partial
- Active semantic scope: `0.0.0-e00.2`
- Scope snapshot SHA-256: `9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`

## Decision

A Compact-targeting financial DSL is possible at prototype scope. The evidence
supports a compiler architecture, not yet a production language decision.

One bounded Moriarty Core atomic swap compiles to valid Compact and four ZKIR
3.0 circuits. A clean archive reproduced the semantic artifacts, compiler
manifest, and ZKIR digests. The reference interpreter and an independent
manifest machine agree on 1,000 deterministic traces. Both invariant counters
are zero. Two Moriarty disclosure-correspondence negative controls and one
Compact compiler disclosure negative control fail closed.

This is sufficient to continue the language experiment. It is not sufficient
to claim general compilation, proof-system correctness, real proof feasibility,
ledger deployment, or acceptable cost.

## Evidence chain

| Claim | Preserved evidence | Result | Status |
|---|---|---|---|
| Finite financial semantics can lower to Compact | `moriarty/core.py`, `moriarty/compact.py`, `swap.compact` | bounded atomic swap generated | S3 |
| Reference and generated manifest behavior correspond | `translation-certificate.json` | 1,000 unique traces; zero divergence | S3 |
| Core invariants hold on the corpus | translation certificate | zero conservation and non-negativity failures | S3 |
| The Compact compiler accepts the output | `toolchain-results.json` | exit 0; TypeScript, metadata, and four ZKIR circuits | S3 |
| The ZKIR tool accepts all circuits | toolchain results | four mock compilations pass | S3 |
| Undeclared public control flow fails | `negative/compile-result.json` | Compact exits 255 with the expected diagnostic digest | S3 |
| Generated disclosure agrees with the visibility manifest | `validate_compact_disclosures` and its tests | missing and additional disclosure fail, including whitespace syntax; comments and strings do not satisfy the check | S3 |
| Compiler interface agrees with the Moriarty manifest | `contract-info.json` and `validate_compiler_metadata` | versions, complete circuits, witnesses, and ledger schemas match | S3 |
| The result reproduces outside the working tree | `evidence/wp01/reproduction-receipt.json` | 43 focused tests and all recorded digests match | S3 |
| Remediated validation is not confused with clean reproduction | `evidence/wp01/current-checkout-validation.json` | current sources and tests are SHA-256-bound; no commit or fresh-archive claim | S3 |

The clean reproduction pins Moriarty commit
`006c4d91ed09c0a89261861b6e7203b3efa3e2df`, Compact commit
`11e7ec5abeecb99297c4faa74d30ef9adc7b51f3`, and ZKIR commit
`2ffe2d17bbb736aec36fb300aeaca679a10d2278`.

## Reproduced footguns

1. Compact circuit parameters are private by default. A private choice that
   controls a public transfer requires explicit `disclose`. The compiler rejects
   the missing declaration.
2. A generator can declare one visibility policy and emit another. Moriarty now
   performs a separate manifest-versus-generated-Compact check. The check
   recognizes lexical whitespace, excludes comments and string literals, and
   rejects unnamed nested or compound disclosure expressions.
3. The Compact source-map digest depends on the spelling of the input path. An
   absolute input path changed the compiler-manifest digest. The recorded
   relative command reproduced it. Reproducible builds require a canonical
   sandbox path or source-map canonicalization.
4. The current path uses `--skip-zk` and ZKIR mock compilation. It supplies no
   evidence for parameter acquisition, key generation, real proof generation,
   or proof verification.
5. Constructor party identities, witness hashes, token colors, and deadlines are
   abstract. A deployment manifest must bind and verify them before a network
   action.

## Semantic boundary proved by E00

The active Core includes `Close`, `Pay`, `If`, `When`, `Deposit`, `Choice`,
`ChoiceEquals`, constant values, token-indexed accounts, partial-payment
warnings, canonical close refunds, and one absolute monotonic time domain.

The generated backend specializes that Core to fixed swap phases, sealed
constructor fields, witness-hash authorization, unshielded token receipt and
send operations, and a translation-validation certificate.

The scope explicitly excludes or defers Attest, action collections, bounded
mandates, conditional-token split and merge, Merkleized continuations, arbitrary
external calls, minting, modules, packages, bounded iteration, and state
compression. The immutable scope record is
`evidence/semantic-scope/moriarty-core-0.0.0-e00.2.json`.

## Complete SDK architecture

The SDK specification contains 65 component contracts and 28 shared data
contracts. Every component contract names its package, inputs, outputs, stable
failure codes, trust roots, version fields, security checks, conformance vectors,
and implementation scope.

```text
source + package lock
  -> parser -> typed, visibility, capability, and bound checks
  -> elaborator -> canonical Core + source map + resource certificate
  -> Compact generator -> backend validator -> compiler + ZKIR
  -> manifest + translation certificate + deployment manifest + SBOM
  -> typed user intent + verified chain state
  -> untrusted coin selection + transaction plan
  -> local artifact, state, intent, disclosure, capability, proof, and plan checks
  -> immutable signing request -> wallet or custody
  -> authorized submission -> verified events, rollback, payout, and explorer views
```

### Component packages

| Package | Components | Responsibility |
|---|---:|---|
| `@moriarty/compiler` | 11 | parse, check, elaborate, normalize, serialize, map, and generate Compact |
| `@moriarty/adapters` | 13 | wallet, custody, Runtime, oracle, identity, registry, continuation, chain, index, event, payout, and explorer boundaries |
| `@moriarty/deploy` | 5 | intent, state, coin selection, transaction planning, and deployment orchestration |
| `@moriarty/verify` | 4 | artifact, disclosure, capability, and transaction verification |
| `@moriarty/package` | 4 | resolution, lock, registry, and package signatures |
| `@moriarty/build` | 4 | reproducible builds, manifests, certificates, and SBOMs |
| `@moriarty/test` | 4 | differential, property, fuzz, and mutation runners |
| `@moriarty/analysis` | 4 | symbolic analysis, counterexample reduction, equivalence, and semantic difference |
| `@moriarty/assurance` | 3 | reference interpretation, simulation, and debugging |
| `@moriarty/assurance-ui` | 2 | trace and cost-envelope views |
| `@moriarty/backend-compact` | 2 | Compact generation and backend validation |
| `@moriarty/prover` | 2 | proof-parameter verification and untrusted prover access |
| Seven single-component packages | 7 | diagnostics, CLI, LSP, editor, client generation, conformance, and telemetry |

The normative inventory is
`openspec/changes/wp09-complete-development-sdk/sdk-component-inventory.json`.

### Canonical data and wire contracts

The 28 data contracts cover source units, dependency locks, typed programs,
canonical Core, source maps, diagnostics, resource certificates, capabilities,
visibility, translation certificates, build receipts, deployment manifests,
SBOMs, constructor bindings, user intent, chain-state evidence, transaction
plans, proof parameters, proof requests, proof receipts, verification results,
signing requests, signature and submission receipts, conformance results,
evidence manifests, Runtime requests, and external evidence.

Each contract fixes its schema identifier, required fields, canonical encoding,
producer, consumers, validation rules, unknown-field behavior, and privacy
classification. The normative contracts are in
`openspec/changes/wp09-complete-development-sdk/sdk-data-contracts.json`.
The instruction validator applies the Draft 2020-12 record schemas, requires
exact counts, checks identifier uniqueness, resolves every producer and
consumer to a component or an explicit external boundary, and recomputes the
SDK index statistics and digests.

## Minimum safety spine

WP09 must implement 17 components before WP10 proof-cost measurement or WP11
security audit can proceed:

- backend validator;
- manifest and certificate generators;
- intent builder and state verifier;
- coin selector and transaction planner;
- artifact, disclosure, capability, and transaction verifiers;
- proof-parameter verifier and prover client;
- wallet, Runtime, and chain adapters;
- deployment orchestrator.

The other 48 components are completely specified but remain `specified-only`.
They cannot satisfy an implementation or audit gate until implemented.

## Trust model

Moriarty does not silently trust the surface compiler, elaborator, Compact
generator, Compact compiler, ZKIR compiler, prover, proof-parameter source,
Runtime, chain provider, registry, oracle, identity provider, continuation store,
indexer, wallet adapter, or LLM.

The local safety path binds:

1. source and package lock to canonical Core;
2. Core to resource, capability, and visibility certificates;
3. Core and certificates to Compact, compiler metadata, and ZKIR digests;
4. deployment bindings to network, parties, tokens, authorities, and timeouts;
5. user intent to fresh verified state and a bounded transaction plan;
6. proof request to approved parameters, circuit, public input, and endpoint;
7. locally verified proof and transaction effects to an immutable signing
   request;
8. every signature and submission receipt to that request and network.

Only a `verified` result can create a signing request. `rejected` and
`unavailable` are distinct, stable outcomes. Provider failure never becomes
approval.

## Next evidence gates

1. WP04 must freeze the candidate Core through versioned semantic motions.
2. WP01 must rerun against that new semantic-scope digest.
3. WP06 must test conditional tokens, Attest, replay, and composition failures.
4. WP07 must compile seven canonical slices through shared interfaces.
5. WP09 must implement and attack the minimum safety spine.
6. WP10 must use approved real proving parameters and an independent verifier.
7. WP11 must audit only implemented artifacts and compare them with an eligible
   audited Compact-library baseline.

No testnet transaction is authorized by this specification. WP10 requires a
separate signed authority record. Mainnet submission is outside the sprint loop.
