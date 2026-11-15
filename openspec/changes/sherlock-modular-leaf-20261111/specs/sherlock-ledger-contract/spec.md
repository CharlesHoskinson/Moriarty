## ADDED Requirements

### Requirement: LDG-001 One contract holds one ledger of settled claims
The LEDGER CONTRACT SHALL be one Compact contract with one shared state: `runs: claim_pub -> outcome_pub` (FPR-010, FPR-013; amendment of `n_steps` out of the map, owner item F-12), `foreign: fclaim_d -> ()`, `verified_roots` and the registry root. Every entry point SHALL read and write this one state. Two contracts with separate ledgers SHALL NOT be used.

#### Scenario: Same claim through two entry points
- **WHEN** a run is accepted by `acceptRun` and the same `claim_d` is later opened from a collection by `openClaim`
- **THEN** `runs` holds the claim once and the second call changes nothing (positive `ledger_accept_run_then_open_same_claim_is_noop`), and a contract with two maps is refused by the control test `ledger_double_settle_two_entries`

### Requirement: LDG-002 `acceptRun` settles a run claim at most once
`acceptRun(C_P, input_pub, outcome_pub)` SHALL verify the wrap proof whose effect words carry the three values (FPR-010). Plain `prog_d` and `n_steps` SHALL NOT be call data (ZK-002, FPR-013); `C_P` is LEAD F-7 under N-11, OWNER. It SHALL compute `claim_pub = transientHash([C_P, input_pub, outcome_pub])` and SHALL refuse a `claim_pub` that is already in `runs`.

#### Scenario: Replay of a settled run
- **WHEN** a second `acceptRun` call carries a `claim_d` that is already in `runs`, from any partition
- **THEN** the call is refused, and the Quint invariant `ClaimSettledOnceAcrossEntries` holds

### Requirement: LDG-003 `acceptCollection` accepts a root and writes no claim
`acceptCollection(reg_pub, set_pub)` SHALL verify the wrap proof of the collection statement `[reg_pub, set_pub]` (LW-009 and DCD-004, with no `shape_d`, owner ruling O9 2026-11-13, and no claim count), SHALL check `reg_pub` against the registry state (history per N-8), and SHALL insert `set_pub` into `verified_roots`. It SHALL make no per-claim write. `shape_d` SHALL NOT be call data and SHALL NOT key any write.

#### Scenario: Collection settlement
- **WHEN** a collection of three runs is accepted
- **THEN** no claim is in `runs` until it is opened (test `ledger_collection_writes_no_claim`), the inputs are exactly `[reg_pub, set_pub]` (a control entry that takes `shape_d` is refused by the same check), and a stale or foreign registry root is refused (Quint invariant `StaleOrForeignRegRootRefused`)

### Requirement: LDG-004 A claim of any depth opens in one call; the depth is in the proof
There SHALL be no `descend` entry (ARCHITECTURE section 1.11, FP-3 fact 3, owner ruling FP3-3 (a)). The path from `set_root` to the claim SHALL be proved inside the opening proof by DESCEND nodes of the GP-2 hook, so the number of contract calls and contract operations to open a claim SHALL NOT depend on its depth. No `K_open` bound SHALL exist.

#### Scenario: Path deeper than any fixed bound
- **WHEN** a claim sits many levels below a verified root
- **THEN** it is opened by one `openClaim` call whose proof carries the DESCEND path (positive `ledger_open_depth_beyond_one_call`), and the Quint invariant `EveryJoinedClaimOpens` holds

### Requirement: LDG-005 `openClaim` inserts a head claim once and refuses non-claims
`openClaim(set_pub, C_P, input_pub, outcome_pub)` SHALL verify an opening proof whose root statement is `SUB(set_root, x)` reached by DESCEND nodes, with `x` opened by `L` as a `LEAF` or `DEP` head, SHALL require `set_pub` in `verified_roots`, SHALL compute `claim_pub = transientHash([C_P, input_pub, outcome_pub])`, and SHALL refuse a `claim_pub` already in `runs`; else it SHALL insert it. The proof SHALL refuse a `JOIN` root and an `ADM` or `VIEW` head. The opening of a `FOREIGN` head into `foreign` SHALL keep this shape with `fclaim_d` in place of the run entries; its public form is set with the GP-3 hook.

#### Scenario: Diamond-shared claim
- **WHEN** one claim has two paths in `set_root` and both are opened
- **THEN** it settles once (Quint invariant `DiamondAncestorOpenedAtMostOnce`), and O35 (LDG-L1, `ClaimDag.lean`) is open

#### Scenario: Seal premise or JOIN offered as a claim
- **WHEN** an `ADM` or `VIEW` head, or a `JOIN` root, is offered to `openClaim`
- **THEN** the call is refused (tests `ledger_open_premise_as_claim`, `ledger_open_join_as_claim`), and a foreign claim opens into `foreign` (positive `ledger_foreign_claim_open`)
