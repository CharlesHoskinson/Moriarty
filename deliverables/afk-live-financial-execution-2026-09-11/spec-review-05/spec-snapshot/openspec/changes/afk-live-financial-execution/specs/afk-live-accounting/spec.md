## ADDED Requirements

### Requirement: Item 1.3 concrete source producer and existing caller
After both fresh specification approvals, Grok SHALL implement import-safe `plugins/moriarty-dev/scripts/moriarty_dev/accounting.py`, called by existing `cli.py::cmd_run` for the exact `finalized-financial-settlement` profile. It SHALL provide preparation, current-projection validation and one-shot launch-claim operations within existing runner/store/record modules. It SHALL not create a new service, scheduler or alternate launcher. Tests SHALL use explicit isolated master, budget lock, projection, admission records and SQLite files including WAL/SHM; live stores/locks/registries and identity adoption remain untouched.

Input SHALL be the selected registered action, frozen hash-bound runner plan, canonical-store anchor and history reconciliation, contained current accounting reference, actual master path, exact reviewed resource amendment and both vote references, and current public observation references. Output SHALL be either a named denial with no child, or a durable debit/current projection and a single matching plugin/master launch claim consumed by the existing guarded run. Missing or malformed authority/identity/projection input SHALL reject; tests may supply anchored fixture arguments, while live authority must come only from separately reviewed contained admission evidence.

The existing runner-plan validation SHALL be factored so preflight can verify exact executable/launcher/argv/action/candidate bytes before a debit exists. Sequence SHALL be source/resource authorization and canonical-history/primary/retry preflight; lock and commit debit/projection; final existing prepaid `load_snapshot`/policy and `load_runner` validation; transactional `store.reserve`; post-reservation checks; master one-shot claim; existing `execute`. No final-admission-before-debit cycle and no skip of final admission is allowed. Changed source after freeze needs both new source reviews and new current admission.

#### Scenario: Real integrated source path
- **WHEN** isolated real producer/runner/CLI code receives complete current fixture authority, accounting and both claim owners
- **THEN** it commits exactly one debit before one stubbed child call through the actual caller, with independently recomputable output and no live writes.

#### Scenario: Missing instance evidence
- **WHEN** a concrete grant, canonical anchor/history, observation or final executor is absent
- **THEN** the source emits a named unavailable result before live mutation/launch; deterministic source repair/testing may proceed under its own approvals without inventing those instance values.

### Requirement: Closed existing accounting fields and bounded successor semantics
The producer SHALL preserve the existing `moriarty.supervised-accounting/1` top-level key family and `currentAccounting`'s closed `schema,path,passBinding,passLedger` fields. Original authority SHA SHALL remain `f6b7f17d1bda437d5f83abc656ccf4a0d9c7a57a83995668387c29bd0328f492`. Observation/identity/claim metadata SHALL not be added as unknown top-level keys. Preserve all original ledger bytes and historical rows; new charge/reservation entries may carry exact scoped provenance validated by this producer. Unknown historical costs remain unknown in referenced immutable dispositions, never a synthetic numeric zero.

An explicit two-vote successor amendment MAY close historical residual credit through a new `reserved` row and place exclusively new bounded credit in one `successorEnvelopes` entry. A new closure row SHALL contain exactly `id,seconds,basis,sourceSha256,amendmentSha256`, where basis is exactly `historical-credit-closure-not-paid-cost`, seconds is a nonnegative integer and digests bind the original/current grant evidence. Use conservative ceiling for fractional represented remainder (7 seconds for the inspected approximately 6.95267 seconds), with no rounding refund or assertion of actual historical payment. A deficit is not erased. Old envelopes SHALL have no spendable current remainder; their originals remain immutable. No historical remainder may be borrowed into a new grant.

The current envelope SHALL use existing `remainingReservedSeconds,workerDispatches,workerDispatchLimit` plus exact immutable grant identity/provenance committed by resource evidence. The selected grant/charge counters are new-grant usage, not reset lifetime counts. A projection SHALL explicitly preserve original owner attribution in `charges` versus `externalPackageCharges`, planning/overhead and package/master limits. Relabeling the MC01 master as MC02 or silently treating MC01 counts as MC02 is prohibited. Until a concrete reviewed MC02 projection disposition specifies inherited rows/counters and exact active-grant semantics, the producer SHALL reject with `resource-package-projection-unresolved`. Supporting this deterministic rejection and isolated valid scoped examples is source scope; live projection adoption is separate.

The concrete amendment SHALL state exact seconds, partitions, master/package/aggregate dispatch increments, current envelope identity and public attempt/submission/fee bounds. It SHALL disposition the unestablished 197630/71 ancestry and the two later direct delegated amendments honestly, without inventing retrospective majority approval. The September 7 successor authority permits two substantive votes on the exact new grant; specification approval and the eight-hour clock are not these votes. Immutable vote paths/digests SHALL be checked before application; each amendment applies once.

#### Scenario: New grant only
- **WHEN** a reviewed concrete successor closes all historical credit, preserves unknowns and supplies an exact enforceable active MC02 projection
- **THEN** spendable credit is at most the new grant minus its actual committed usage, with old limits/counters preserved as history and no lifetime-global total claim.

#### Scenario: Shape-valid fabricated credit
- **WHEN** the input omits external charges, guesses unknown costs, relabels package ownership, lacks the exact projection disposition, repeats an amendment, uses missing votes or presents invalid numeric/extra top-level fields
- **THEN** the actual producer/consumer rejects before debit or launch, even if the old reader alone would accept its shape.

### Requirement: Single allocation, all-kind limits and prepaid boundary
For the live action, its resource allocation SHALL occur once in `resourceAmendment`; binding `resources.allocationSeconds` SHALL be absent or zero and the same grant SHALL not recur in `successorAmendments`. Hash identity, not different filenames, SHALL identify duplicates. The producer SHALL enforce current-envelope, selected-package and aggregate-master time/dispatch and Preview attempt/submission bounds for every live kind, including report/admin/verify/review/reproduce. A new resource vote must expressly extend exhausted aggregate 83/83 or package ceilings where applicable; it cannot reset the retained 52/52 MC01 counter.

At the limit a genuinely prepaid invocation may claim and launch once. Any other caller or replay remains denied. Reservations/fees/attempts are consumed before use and failures do not refund them; actual paid fees remain separately observed.

#### Scenario: Last authorized resource race
- **WHEN** two real isolated callers compete for one remaining dispatch or the same final seconds
- **THEN** existing lock/store serialization lets at most one commit and launch; both kind-switch and duplicated allocation paths reject.

### Requirement: Master projection and global one-shot claim
The master remains at its existing external location, accessed by the explicit producer under `budget.lock`; `records.py` and `runner.py` continue to read only contained projection paths. Write/fsync temporary master/projection/admission correspondence files, replace and fsync their directories. Separate replacements SHALL not be called one atomic transaction. Current correspondence evidence SHALL bind exact master/projection generations/digests and be checked under the lock before final launch. Interrupted/mismatched correspondence yields unavailable credit until explicit reconciliation.

New debit rows SHALL contain the existing exact `id,seconds,actionId,candidateHash,runnerDigest` plus `storeIdentity,reservationId,launchClaim`. Before reservation, `reservationId` may be null and `launchClaim` exactly `unclaimed`; afterward the unique matching reservation ID is recorded and launchClaim transitions once to `claimed`. The persisted in-store identity must positively match the authorized canonical repository anchor. The master claim and existing SQLite unique charge claim both bind the same ownership; a claim token is not just a filename. No second `claimed` transition or launch is allowed. Recovery never clears/rewrites consumed rows.

The canonical inventory `canonical-store-before-source.json` records no persisted token and only SP01 design/report events. Concrete token adoption and loan-history reconciliation SHALL be independently reviewed against that original inventory; no ordinary initializer/status/review call or override can create authority. Normalize checkout/common-directory event identity through that reviewed reconciliation and retain every linked-worktree/external failure; no unrelated seeded event or renamed capability establishes zero failures. These live-instance mutations remain pending after source repair.

Observation/correspondence digests SHALL be mutable evidence outside candidate inputs, binding inputs and runner plan, bound in final admission. Refreshing them SHALL not change candidate, runner digest, binding hash or committed debit.

#### Scenario: Persistence and claim crash matrix
- **WHEN** isolated fault injection interrupts before debit, after debit before projection, between file replacements, between SQLite/master claim, or after master claim before child launch
- **THEN** no unauthorized child starts; committed charges remain consumed, correspondence/claim disagreement denies, and an unlaunched committed invocation receives an explicit failed/unknown disposition without retry credit.

#### Scenario: Store rollback or clone replay
- **WHEN** an already committed debit is presented with another clone/default-resolved store, a new/fallback store, a restored same-token pre-claim backup or conflicting ownership
- **THEN** canonical identity and master one-shot claim deny another launch while preserving the existing SQLite guard and historical charge.

### Requirement: Exact attempts and actual submission consumer
Retained loan allocations/runs SHALL establish exactly two prior consumed loan attempts before the proposed cumulative attempt 3; zero master Preview counters and cumulative reservation totals are not attempt evidence. The new invocation SHALL have at most four public submission calls for the actual deploy → initialize → accrue → settle path, subject to both resource votes and confirmation that the final executor uses that path. A charged but unlaunched attempt consumes attempt 3 with zero observed submissions plus its reserved maximum; no attempt 4 follows automatically.

The actual producer SHALL bind and reconcile the existing `ledger/providers.mjs` durable reservation path, `bindingFor` and submission guard with the invocation/accounting identity. Every SDK submission/retry must reach that guard; a Python launch counter alone is insufficient. Exact DUST SPECK fee ceilings, selected token/domain gross ceilings and actual paid indexer fee units SHALL remain distinct. No tNight/USD conversion, refund against gross debit or unknown-unit equivalence is inferred. Final concrete financial bounds remain pending resource votes.

#### Scenario: Retry and attempt accounting
- **WHEN** four allowed submissions are consumed or attempt 3 commits then fails before launch
- **THEN** a fifth submission, counter reset, reservation-path switch or fourth invocation is denied and every real transaction/failed attempt remains retained.

### Requirement: Direct nonpersisting SDK funding observer
Source scope includes `experiments/moriarty-midnight-financial/ledger/observe-preview-funding.mjs`, invoked as a bounded child by accounting preflight. It SHALL directly use the pinned SDK `DustWallet(configuration).restore` and `UnshieldedWallet(configuration).restore`, their child `start`, `waitForSyncedState(0n)` and `stop` APIs. Reuse strict original snapshot/identity checks from `preview-bootstrap.mjs:76–89`, not `launchPreviewFinancialCase`, `createWallet`, check-balance or the facade's submission/proving services. Use NoOp transaction history and an outer 90-second maximum with 5-second stop grace. This permits only synchronization of a disposable restored in-memory copy; canonical wallet files/keys/identity and chain/contract state remain unchanged.

The observer privately reads only the existing seed and dust/unshielded snapshots necessary to derive and verify the original role identity, with original owner-only/no-symlink validation. It SHALL not create a wallet, fallback on failed restore, read role secrets/contract passwords, serialize/persist wallet state, register, transfer, balance, sign, prove, finalize or submit. Clear temporary sensitive buffers where supported; log no private state/error objects. Stop every partially initialized child; containment failure makes observation unavailable. Independent tests SHALL compare canonical input bytes/digests before/after and trap every forbidden API/write.

Unshielded funding SHALL use exact selected-token available balances. DUST funding SHALL sum `availableCoins[*].generatedNow` at native sync time, excluding pending entries; raw `balance(Date)` and hypothetical generation are not substitutes. SDK restore resets `pendingDust`, so independent old-allocation/finality/persistence history SHALL rule out unresolved prior spend before declaring funds unreserved. Missing that evidence yields unavailable funding. Require connected/strictly-complete progress, exact original public key/address/network, canonical network/protocol reference, sync/observation age at most 120 seconds at launch, no future timestamps and explicit units. Output only public aggregate funding/identity/freshness metadata to a separate evidence destination.

Pending-use reconciliation SHALL cover the actual retained allocation/reservation/persistence/finality lineage for the two identified Preview loan allocations, intervening swaps and any other material observed use of this wallet, together with exclusive current wallet ownership and fresh SDK sync. Unresolved material observed use SHALL block funding. Exhaustive proof against hypothetical external use, or host-global lifetime accounting of unrelated accounts, SHALL NOT be added as a funding predicate; retain those limits honestly without treating known pending ambiguity as resolved.

#### Scenario: Strict observed funds
- **WHEN** isolated real observer logic restores the original identity without fallback, syncs within bounds, has independently cleared pending-spend lineage, and all canonical files remain unchanged
- **THEN** it returns exact timestamped available token/DUST quantities for admission comparison; observation is not spend reservation or a promise of finality.

#### Scenario: Unavailable or mutating observer
- **WHEN** identity differs, restore fails, old pending spend is unresolved, units/timestamps are unknown, sync/stop times out, a forbidden API/write is attempted or snapshot bytes change
- **THEN** funding is unavailable and live admission fails. Empty restored pending state, a recent indexer tip or green fixture alone cannot pass.

### Requirement: Source approval and live adoption stay separate
These requirements authorize no concrete grant, live wallet observation or adoption. Both fresh reviewers must approve this exact source specification before implementation; source tests then exercise real producer/caller/observer code entirely on isolated fixtures and transports, followed by both frozen full-source audits. A later concrete grant, exact MC02 projection disposition, canonical history/token adoption, prior-attempt reconciliation, final executor and live observations are separately reviewed prerequisites to the existing guarded run. Their absence SHALL be reported precisely while eligible deterministic source work proceeds; no new record service or broad historical survey is required.

#### Scenario: Source-ready but instance-blocked
- **WHEN** actual isolated producer/observer tests and both source reviews pass but concrete resource/adoption evidence remains pending
- **THEN** record only source readiness, preserve named live blockers and continue their concrete preparation without claiming admission or financial acceptance.
