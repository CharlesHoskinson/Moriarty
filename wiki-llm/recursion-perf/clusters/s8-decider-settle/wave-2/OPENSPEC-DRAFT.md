# OpenSpec draft: persistent-prover-service

**ESTIMATE — Status:** design proposal, not applied, implemented, approved or deployed. This single document contains the proposed `proposal.md`, `tasks.md`, and one spec delta `specs/persistent-prover-service/spec.md`. SHALL statements below are proposed normative requirements, not claims about current code. No network action is authorized by this draft.

## Proposal

### Why

**SOURCE — Repeated setup has a measurable historical cost:** recl9 records 46.9 s D PK setup plus 586.4 s proving, and 104.5 s node PK setup plus 867.3 s proving. A4 requires D's VK to equal the installed contract VK. Pinned PK deserialization reconstructs polynomial transforms rather than loading a fully prepared object. Wallet registration's 435 s included 374.910 s sync. Sources: `raw/s8-decider-settle/w2-private-evidence-summary.md`, `raw/s8-decider-settle/w2-proofs-plonk.md`, `raw/s8-decider-settle/local-terminal-test.md`, `raw/s8-decider-settle/wallet-registration-timeline.md`.

**ESTIMATE — Proposed outcome:** retain immutable prepared parent/D keys and official SRS in one bounded worker; authorize every caller and bind every proof to its full approved VK identity; keep the existing deployer wallet synchronized and reconcile fee reservations durably. Remove warm-request setup while preserving the ledger-9 relation, transcript, proof format and finality/readback obligations. This proposal does not promise 10× latency: the supported D setup counterfactual is only 1.080× total invocation speedup.

### What changes

**ESTIMATE — Add:** a local authenticated coordinator and worker protocol; owner-approved artifact manifests; a two-key resident cache and bounded scheduler; a journal covering Consumed/Decided/Submitted/Finalized with attempt substates; a single-owner wallet lifecycle adapter; independent stock verification; phase/resource telemetry; and offline identity/recovery qualification.

**ESTIMATE — Compatibility:** preserve existing standalone compile/prove/deploy interfaces and CPU path. The service is optional maintainer tooling. Keep PLONK/KZG over BLS12-381, Blake2b outer transcript and the official SRS. A k19 relation is outside this change; any later changed relation/key requires a separate reviewed manifest/deployment activation. The service must never repair a VK mismatch by silently generating a new key, deploying a contract, or weakening verification.

**ESTIMATE — Deployment shape:** one parent+decider family resident at a time; initial single heavy job, 16-thread worker on this 20-core host; 16 GiB reserved per key; worker target ≤56 GiB and hard cap 60 GiB; total host plan ≤80 GiB; GPU disabled until qualified, then ≤24 GiB process allocations. These are provisional admission limits, not proof that the workload fits. Configuration may lower them after measurement; raising them requires a measured budget that leaves wallet/OS capacity. Exact key degree/retained allocation and pinned wallet SDK migration compatibility are OPEN QUESTION.

### Impact and rollout

**ESTIMATE — Sequence:** implement immutable identity/readiness and independent verification first, add residency and telemetry, qualify offline, then integrate wallet lifecycle and fault recovery. Promote only after E1 and E3 pass and an owner-approved deployment manifest exists. Roll back by closing admission, draining/reconciling submitted jobs and returning callers to the existing standalone path with the same artifact pins. Never delete wallet state, keys, contract private state or pending transaction evidence on rollback.

## Tasks

**ESTIMATE — All tasks are proposed and unchecked:**

- [ ] Define versioned canonical manifest/request/result encodings, hashing domains and principal scopes; define full `MidnightVK` versus inner VK and tagged-envelope hashes explicitly.
- [ ] Implement checked artifact loading, official SRS/parent/relation/template pins and PK-to-VK equality; authenticate manifests from the deployment authority; reject arbitrary request-supplied paths/keys.
- [ ] Implement Unix peer-credential authentication, private socket permissions, scoped admission, rate/byte limits, deadlines and durable idempotency conflicts.
- [ ] Retain prepared P/N and D objects plus shared SRS without deep clones; expose readiness only after validation; implement drain-before-rotation and single-heavy-job admission.
- [ ] Add key shape/degree/extended-domain and allocation instrumentation; enforce configurable RAM/VRAM/pinned-host caps and close admission under pressure.
- [ ] Independently verify every result through real stock ledger-9 against the approved full VK and exact public vector; authenticate result metadata without changing proof bytes.
- [ ] Implement durable state/attempt journal and recovery; distinguish ambiguous submission from rejection and Finalized from a SDK transaction type.
- [ ] Keep one existing deployer wallet owner per seed/network; restore validated private snapshots, maintain sync freshness, checkpoint atomically and preserve existing private-state stores.
- [ ] Implement serialized fee-input reservations, ordered contract-predecessor handling, TTL admission and rebuild/reproof invalidation; qualify exact deployed SDK/proof-server versions.
- [ ] Implement the proposed offline `s8-service-lab` interface and run E1/E3; record honest/negative/fault traces, memory and phase timing. Keep E2 k19 work separate.
- [ ] Document operational startup/health/drain/rollback and owner activation of a deployment manifest; verify standalone public developer workflows require none of this tooling.

## Spec delta: persistent-prover-service

### ADDED Requirements

**ESTIMATE — Normative design:** all requirements and scenarios in this section belong to one proposed capability, `persistent-prover-service`.

#### Requirement: Authenticate and authorize proof requests

The service SHALL authenticate callers before accepting witness data or enqueueing work. The initial local interface SHALL use a restricted filesystem Unix socket and kernel peer credentials mapped to permitted network, contract operation and circuit family. It SHALL bound request sizes, outstanding work and per-principal resource usage. An optional remote interface SHALL require mutually authenticated transport and equivalent scope checks.

##### Scenario: Unauthorized local caller

- **GIVEN** a process can reach the socket but its UID is not authorized for the requested deployment
- **WHEN** it requests a proof
- **THEN** the service refuses the request before witness intake or worker dispatch and records only nonsecret audit metadata.

#### Requirement: Bind every proof to an approved exact VK

Every request SHALL carry a nonempty `expected_vk_hash`, manifest identity and deployment generation. The VK hash SHALL cover canonical complete Processed `MidnightVK` bytes including architecture, k and public-input count; an inner-VK or filename hash SHALL NOT substitute. The authenticated manifest SHALL pin parent/child VKs, relation, public schema/template, official SRS, build identity, network/genesis, contract and operation. Before admitting work, the service SHALL compare expected VK with both its active manifest and the loaded key identity, including outer metadata and PK-embedded inner VK. It SHALL refuse mismatches without key generation or deployment fallback.

##### Scenario: A4 wrong installed key

- **GIVEN** the deployment manifest pins VK H1 and a prepared D slot belongs to H2
- **WHEN** an authorized caller asks for H1, or asks for H2 while H1 is installed/approved
- **THEN** the service returns `VK_MISMATCH` or `NOT_READY`, dispatches no proof, and never substitutes H2 for H1.

##### Scenario: Corrupt or substituted artifact

- **GIVEN** a relation, parent key, SRS, architecture or template differs from its authenticated pin
- **WHEN** the worker attempts to become ready
- **THEN** readiness fails and no job uses the artifact, even if its filename and nominal k match.

#### Requirement: Preserve ledger proof semantics

The service SHALL retain the approved relation and exact public-input encoding, PLONK/KZG BLS12-381 parameters, Blake2b outer transcript and official SRS. It SHALL use fresh cryptographic randomness for each new proving attempt. Before marking Decided, the coordinator SHALL independently verify the proof with stock ledger-9 proof verification enabled, using the approved VK and exact request public vector. The authenticated result envelope SHALL bind proof hash, public-input hash, full VK hash, manifest generation, SRS identity, parent artifacts and attempt ID. Metadata SHALL NOT alter the ledger proof encoding or replace verification.

##### Scenario: Valid proof for a different call

- **GIVEN** a proof verifies for a different binding input, communication commitment or public vector
- **WHEN** that result is returned for the current job
- **THEN** the coordinator rejects it and neither marks Decided nor allows the wallet to sign/submit it.

#### Requirement: Retain prepared keys within measured capacity

The worker SHALL retain prepared key objects and a shared official SRS across requests and SHALL NOT silently rebuild them on the warm request path. Initial admission SHALL permit at most one large proving or key-preparation operation and one resident parent/D family. It SHALL account for live keys, allocator retention, preparation/proving scratch, SRS, pinned host memory and GPU context/precompute allocations. Readiness SHALL require measured compliance with configured limits. CPU operation SHALL remain available independently of an optional GPU backend.

##### Scenario: New key exceeds reservation

- **GIVEN** warming a key exceeds its reserved capacity or approaches the worker memory limit
- **WHEN** admission evaluates readiness
- **THEN** it refuses new heavy work and reports the failed capacity phase; it does not spawn extra workers or retain an unbounded number of generations.

##### Scenario: Process-warm request

- **GIVEN** the exact manifest family is resident and the worker is healthy
- **WHEN** another authorized request for that family is admitted
- **THEN** it borrows the same immutable prepared objects, records zero SRS-decode/PK-preparation work for that request, and proves with new attempt randomness.

#### Requirement: Rotate artifacts without identity races

An attempt SHALL pin one immutable manifest/key generation for its lifetime. Activation SHALL require an owner-approved deployment mapping. Rotation SHALL close admission and drain or quarantine old attempts before replacing resident keys. The wallet SHALL revalidate deployment identity before signing/submitting; a change SHALL make an incompatible result stale. The service SHALL NOT auto-register a new VK.

##### Scenario: Deployment changes during proving

- **GIVEN** a request pins generation G1
- **WHEN** G2 becomes the approved deployment while its proof is running
- **THEN** the result retains G1 metadata, is not relabeled G2, and is barred from submission unless G1 remains explicitly valid for the intended operation.

#### Requirement: Durable idempotency and attempt recovery

The service SHALL persist request/attempt identity and transitions durably. An idempotency key SHALL be scoped to authenticated principal and a complete request digest. A repeated identical request SHALL return its existing job/result; reuse with a different digest SHALL conflict. Recovery SHALL reconcile ambiguous send outcomes before building a competing transaction or releasing its reserved inputs. A proof result SHALL be reusable only for the identical bound request.

##### Scenario: Crash after transaction send

- **GIVEN** a transaction hash and send intent are durable but acknowledgment was lost
- **WHEN** the coordinator restarts
- **THEN** it tracks the outcome as unknown, queries/replays authoritative transaction/effects evidence through the qualified adapter, retains reservations, and does not submit a new competing spend solely because a timeout elapsed.

#### Requirement: Keep one synchronized wallet owner

The deployer wallet SHALL have one exclusive owner for each existing seed/network and SHALL retain existing keys and contract private state. Snapshots SHALL be private and validated against address, network/genesis and SDK/ledger schema; snapshot/journal updates SHALL use atomic replacement and durable ordering. Readiness SHALL require healthy relevant sub-wallets, current indexer/head freshness and reconciliation of pending own transactions. A generic “synced” flag SHALL NOT alone establish spend readiness.

##### Scenario: Synced indexer has not applied own spend

- **GIVEN** the wallet reports synced but the previous transaction's fee input is still pending
- **WHEN** another job needs that input
- **THEN** the wallet blocks that spend until authoritative application/reconciliation, regardless of its displayed total balance.

##### Scenario: Wrong-network restore

- **GIVEN** a snapshot's network or derived address differs from the configured existing wallet
- **WHEN** it is restored
- **THEN** readiness fails and the snapshot, keys and private-state store are preserved for recovery.

#### Requirement: Reserve inputs and enforce validity deadlines

Fee reservation and recipe creation SHALL be serialized under wallet ownership and SHALL identify exact coins and job attempts. Admission SHALL account for remaining transaction TTL and DUST validity using qualified network parameters, queue/proving/submission estimates and clock uncertainty. Rebuilds that change any proof-bound field SHALL invalidate the old proof/result and require a new bound attempt. Reservations SHALL remain held through ambiguous submission until reconciliation justifies release.

##### Scenario: Concurrent jobs use one fee coin

- **GIVEN** two jobs choose the same fee coin
- **WHEN** they attempt to reserve it
- **THEN** at most one owns it; the other waits or fails admission without creating a competing spend.

##### Scenario: TTL expires while queued

- **GIVEN** remaining validity is insufficient for the configured completion allowance
- **WHEN** the job reaches admission or pre-submit revalidation
- **THEN** the service refuses or rebuilds it; it does not mutate signed/proved bytes or reuse a D proof whose call binding changed.

#### Requirement: Report the owned states truthfully

Consumed SHALL require the exact accepted parent artifact and durable job identity. Decided SHALL require independent terminal verification. Submitted SHALL require submission acknowledgment or reconciled inclusion evidence with transaction identity. Finalized SHALL require protocol finality evidence and independent readback of the intended ordered settlement effects. Timers SHALL separate queue, preparation, witness/proving, wallet sync/proving, submission, finality and observation delay. Logs SHALL exclude secrets, witnesses, private snapshots and blinding material.

##### Scenario: DUST becomes visible without settlement readback

- **GIVEN** an indexer shows a DUST balance update or the SDK returns a value named `FinalizedTransaction`
- **WHEN** contract finality and expected effects are not established
- **THEN** the job is not marked Finalized.

#### Requirement: Preserve the public developer boundary

Compiling, proving and deploying through the existing public workflows SHALL remain possible without the service, its authentication infrastructure, AgentBus, factory tooling or GPU backend. The service SHALL consume ordinary approved proof artifacts and SHALL NOT impose a new ledger protocol.

##### Scenario: Service unavailable

- **GIVEN** a developer uses the supported standalone CPU workflow
- **WHEN** the persistent service is stopped or absent
- **THEN** the developer can still compile/prove/deploy under the existing interfaces and artifact requirements.
