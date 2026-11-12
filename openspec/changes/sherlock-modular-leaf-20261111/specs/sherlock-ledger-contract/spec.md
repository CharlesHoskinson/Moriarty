## ADDED Requirements

### Requirement: LDG-001 One contract holds one ledger of settled claims
The LEDGER CONTRACT SHALL be one Compact contract with one shared state: `runs: claim_d -> (outcome_d, n_steps)`, `foreign: fclaim_d -> ()`, `verified_roots` and the registry root. Every entry point SHALL read and write this one state. Two contracts with separate ledgers SHALL NOT be used.

#### Scenario: Same claim through two entry points
- **WHEN** a run is accepted by `acceptRun` and the same `claim_d` is later opened from a collection by `openClaim`
- **THEN** `runs` holds the claim once and the second call changes nothing (positive `ledger_accept_run_then_open_same_claim_is_noop`), and a contract with two maps is refused by the control test `ledger_double_settle_two_entries`

### Requirement: LDG-002 `acceptRun` settles a run claim at most once
`acceptRun(prog_d, input_d, outcome_d, n_steps)` SHALL verify the wrap proof whose effect words carry the four values with `prog_d` as call data (SRC-007; N-11, OWNER). For a private program, plain `prog_d` SHALL NOT be call data (ZK-002); the public form of the program identity in the call data and in `claim_d` is owner item N-11 and freeze prerequisite FP-3. It SHALL compute `claim_d = H(SHK_RUN, 1, prog_d, input_d, outcome_d)` and SHALL refuse a `claim_d` that is already in `runs`.

#### Scenario: Replay of a settled run
- **WHEN** a second `acceptRun` call carries a `claim_d` that is already in `runs`, from any partition
- **THEN** the call is refused, and the Quint invariant `ClaimSettledOnceAcrossEntries` holds

### Requirement: LDG-003 `acceptCollection` accepts a root and writes no claim
`acceptCollection(reg_root, set_root, shape_d)` SHALL verify the wrap proof of the collection statement, SHALL check `reg_root` against the registry state (history per N-8), and SHALL insert `set_root` into `verified_roots`. It SHALL make no per-claim write and no write keyed by `shape_d`.

#### Scenario: Collection settlement
- **WHEN** a collection of three runs is accepted
- **THEN** no claim is in `runs` until it is opened (test `ledger_collection_writes_no_claim`), and a stale or foreign registry root is refused (Quint invariant `StaleOrForeignRegRootRefused`)

### Requirement: LDG-004 A claim is opened in bounded steps from a verified root
`descend(root, openings)` SHALL require `root` in `verified_roots`, SHALL check at most `K_open` one-hash openings of `JOIN` or `DEP` nodes, and SHALL insert the reached sub-root. `K_open` SHALL keep one call within `MAX_CONTRACT_OPERATIONS = 256`. A path of any depth SHALL be opened by several calls.

#### Scenario: Path deeper than one call
- **WHEN** a claim sits below more than `K_open` levels of a verified root
- **THEN** it is opened by successive `descend` calls (positive `ledger_open_depth_beyond_one_call`), the Quint invariant `EveryJoinedClaimOpens` holds, and a call from an unverified root or with more than `K_open` openings is refused (tests `ledger_descend_unverified_root`, `ledger_descend_over_op_cap`)

### Requirement: LDG-005 `openClaim` inserts a head claim once and refuses non-claims
`openClaim(sub_root, head)` SHALL require `sub_root` in `verified_roots` and SHALL check one head hash of the `LEAF`, `DEP` or `FOREIGN` form. It SHALL insert `claim_d` into `runs` or `fclaim_d` into `foreign`, and SHALL change nothing if the key is present. It SHALL refuse a `JOIN` root and an `ADM` or `VIEW` head.

#### Scenario: Diamond-shared claim
- **WHEN** one claim has two paths in `set_root` and both are opened
- **THEN** it settles once (Quint invariant `DiamondAncestorOpenedAtMostOnce`), and O35 (LDG-L1, `ClaimDag.lean`) is open

#### Scenario: Seal premise or JOIN offered as a claim
- **WHEN** an `ADM` or `VIEW` head, or a `JOIN` root, is offered to `openClaim`
- **THEN** the call is refused (tests `ledger_open_premise_as_claim`, `ledger_open_join_as_claim`), and a foreign claim opens into `foreign` (positive `ledger_foreign_claim_open`)
