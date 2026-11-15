## ADDED Requirements

### Requirement: LDG-001 One contract holds one ledger with no per-claim state
The LEDGER CONTRACT SHALL be one Compact contract with one shared state: `runs: claim_pub -> outcome_pub` for runs that their own users submit (FPR-010, FPR-013; amendment of `n_steps` out of the map, owner item F-12), `claim_roots`, one `claims_root` per accepted collection, and the registry root. It SHALL hold no map or set with one entry per collection claim (Midnight write rule, owner ruling 2026-11-14, row T20). Two contracts with separate ledgers SHALL NOT be used.

#### Scenario: Same claim through a run and a collection
- **WHEN** a run is accepted by `acceptRun` and the same claim is a member of an accepted `claims_root`
- **THEN** a consumer uses it once (test `W-2 one use across sources` in `tools/sherlock-sdk/t20-ledger.test.mjs`; Quint `UsedOncePerConsumer`), and a consumer whose nullifier is keyed by the root uses it twice (control `consumer-control-root-nullifier`)

### Requirement: LDG-002 `acceptRun` settles a run claim at most once
`acceptRun(C_P, input_pub, outcome_pub)` SHALL verify the wrap proof whose effect words carry the three values (FPR-010). Plain `prog_d` and `n_steps` SHALL NOT be call data (ZK-002, FPR-013); `C_P` is LEAD F-7 under N-11, OWNER. It SHALL compute `claim_pub = transientHash([C_P, input_pub, outcome_pub])` and SHALL refuse a `claim_pub` that is already in `runs`.

#### Scenario: Further rules of LDG-002
- **WHEN** LDG-002 is applied
- **THEN** Under the Midnight write rule (owner ruling 2026-11-14, row T20) `acceptRun` SHALL be the entry for one run that its own user submits and pays for. A job that an operator proves and settles SHALL NOT be settled by `acceptRun`; it SHALL settle through the epoch collection of LDG-006.

#### Scenario: Replay of a settled run
- **WHEN** a second `acceptRun` call carries a `claim_d` that is already in `runs`, from any partition
- **THEN** the call is refused, and the Quint invariant `ClaimSettledOnceAcrossEntries` holds

### Requirement: LDG-003 `acceptCollection` accepts one root and writes no claim
`acceptCollection(reg_pub, claims_root)` SHALL verify the wrap proof of the collection statement `[reg_pub, claims_root]` (LW-009 and DCD-004 as amended by T20, with no `shape_d`, owner ruling O9 2026-11-13, and no claim count), SHALL check `reg_pub` against the registry state (history per N-8), SHALL refuse a `claims_root` already in `claim_roots`, and SHALL insert it. It SHALL make no per-claim write. `shape_d`, `set_root` and `set_pub` SHALL NOT be call data.

#### Scenario: Collection settlement
- **WHEN** a collection of three runs is accepted
- **THEN** `claim_roots` grows by one and `runs` is unchanged (test `W-2 collection`), a second acceptance of the same root and a stale or foreign registry root are refused (Quint invariant `StaleOrForeignRegRootRefused`)

### Requirement: LDG-004 A collection claim is used by membership, with no ledger write
There SHALL be no `descend` entry and no `openClaim` entry. A Sherlock consumer SHALL discharge a collection claim in circuit from the producer's SET proof (GP-2 hook: DESCEND, DL-STEP, SEAL-WITH-DEPS) and SHALL read no ledger state.

#### Scenario: Further rules of LDG-004
- **WHEN** LDG-004 is applied
- **THEN** A Compact consumer SHALL prove, inside its own call, a path of `D_C` = 20 siblings from the claim's leaf to a root, SHALL read `rootAccepted(root)` from the LEDGER CONTRACT by a cross-contract call, and SHALL keep its own nullifier keyed by the claim alone. The number of ledger writes SHALL NOT depend on the number of claims used or on their depth in `set_root`.

#### Scenario: Member of an accepted root
- **WHEN** a consumer offers a valid path to an accepted `claims_root`
- **THEN** the call succeeds, the LEDGER CONTRACT state is unchanged, and a second use by the same consumer is refused (test `W-2 consumer`; Quint `UsedOncePerConsumer`, `LedgerWritesBounded`, `OneCallAnyDepth`)

#### Scenario: Not a member
- **WHEN** the path leads to a root that was never accepted, the outcome is changed, the path belongs to another slot, or an index bit is flipped
- **THEN** `rootAccepted` refuses the call (test `W-2 refusals`; Quint `UsedOnlyVerified`), and a consumer with no root read accepts the forged path (control `consumer-control-no-root-check`)

### Requirement: LDG-005 The claims tree and its availability
`claims_root` SHALL be the root of a binary Merkle tree over Fr of fixed depth `D_C` = 20, built by `L` from the member heads of the collection's `set_root`: a run claim's leaf SHALL be its `claim_pub`, an empty slot SHALL be 0, a node SHALL be `transientHash([l, r])`, and an `ADM` or `VIEW` head SHALL NOT be a leaf. The foreign leaf is set by LDG-007.

#### Scenario: Further rules of LDG-005
- **WHEN** LDG-005 is applied
- **THEN** The claim list and the paths SHALL stay off chain: the operator SHALL publish the epoch's leaf list to a data-availability store at epoch close and SHALL give each job's submitter its slot and path. A withheld list SHALL NOT make a non-member usable. A claim SHALL be usable only after its epoch's collection is accepted.

#### Scenario: Withheld list and epoch wait
- **WHEN** the leaf list of an accepted epoch is withheld, or a claim is proved in the open epoch
- **THEN** no consumer can use the claim until the list is published or the epoch's collection is accepted (Quint `claimsRoot.qnt` tests `neg_withheld`, `pos_published_later`, `neg_use_before_close`; invariant `UseAfterEpochClose`)

### Requirement: LDG-006 Operator-settled jobs settle by one collection per epoch
When an operator settles proved jobs, the operator SHALL JOIN the run SETs of one epoch and SHALL submit one `acceptCollection` call for the epoch. An epoch with no job SHALL write nothing. The operator SHALL make at most one ledger write per epoch, and the number of ledger writes SHALL NOT grow with the number of jobs.

#### Scenario: Many jobs in one epoch
- **WHEN** an operator proves four jobs in one epoch and a user submits its own run
- **THEN** the ledger receives two writes, one `acceptCollection` for the four jobs and one `acceptRun` for the user's run (Quint `epochSettle.qnt`, `pos_four_jobs_one_write`, invariants `OperatorWritesPerEpochAtMostOne` and `LedgerWritesBounded`), and a job never enters `runs` (`NoJobInRuns`)

### Requirement: LDG-007 A foreign claim is used by membership, with no foreign map
The LEDGER CONTRACT SHALL hold no `foreign` map and SHALL have no FOREIGN opening entry. A foreign claim SHALL stay a member of the collection's `set_root` as a `FOREIGN(fclaim_d)` node, and its leaf in the claims tree SHALL be `transientHash([SHK_CLAIM_FGN, fclaim_pub])` with `fclaim_pub = PoseidonFr_4(SHK_FCLAIM, pack(fclaim_d))`. A consumer SHALL use it with the path check, the `rootAccepted` read and its own nullifier keyed by the leaf.

#### Scenario: Further rules of LDG-007
- **WHEN** LDG-007 is applied
- **THEN** A foreign leaf SHALL NOT pass as a run claim, and a run leaf SHALL NOT pass as a foreign claim.

#### Scenario: Foreign claim in a collection
- **WHEN** a collection holds run claims and one foreign claim and its root is accepted
- **THEN** a consumer uses the foreign claim once with `useForeignClaim` (test `W-3 foreign claim`), and a run leaf offered as a foreign claim is refused, while a consumer with no kind tag accepts it (control `consumer-control-no-kind-tag`; Quint mutant `no_kind_separation`)

### Requirement: LDG-008 One registry root update per registry epoch
When registry updates exist (N-8), the registry root SHALL be updated at most once per registry epoch, with one new root over the whole entry list, and SHALL NOT be written once per registered or revoked key. A registry epoch with no change SHALL write nothing. The entry list SHALL stay off chain, and a key SHALL be admitted only by a membership path against the registry root proved in circuit (GP-3 hook). `acceptCollection` SHALL refuse a `reg_pub` other than the current root.

#### Scenario: Three key changes in one registry epoch
- **WHEN** two keys are registered and one is revoked in one registry epoch, and a later collection uses the new keys
- **THEN** the registry root is written once, the collection under the new root is accepted, a collection under the replaced root is refused, and a collection that uses an unregistered key is refused (Quint `registryEpoch.qnt`: `RegWritesPerEpochAtMostOne`, `StaleOrForeignRegRootRefused`, `KeysUsedAreRegistered`; mutants `per_key_write`, `no_reg_check`, `unbound_membership`)
