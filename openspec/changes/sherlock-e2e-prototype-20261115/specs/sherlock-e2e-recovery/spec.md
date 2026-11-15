## ADDED Requirements

### Requirement: FRC-001 Only a passed step is skipped
FRC-001: When a runner step ends with exit status 0, the Preview runner SHALL write the marker file `<step>.ok` in the state folder.

Verification: T17 (a) reruns after the 18:40 reboot (`RESULTS.md:42`). MISSING a test of a failure between submit and readback; exists in part.

#### Scenario: A failed call leaves no marker
- **GIVEN** a `call` step that ends with status 4
- **WHEN** `t17.sh all` runs again
- **THEN** the runner skips `deploy` and runs `call` again

#### Scenario: Status of FRC-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: FRC-002, FRC-003. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/t17.sh:3,26,43,74,80,91,114`, `RUNBOOK.md:42`. Check: T17 (a) reruns after the 18:40 reboot (`RESULTS.md:42`). MISSING a test of a failure between submit and readback.

### Requirement: FRC-002 A deploy is never sent twice
FRC-002: If a deploy was sent and not confirmed, then the deploy runner SHALL read the recorded address back from the indexer and send no second deploy.

Verification: MISSING a fault-injection test; fix 4 of T17 (a); to build.

#### Scenario: A crash after the send
- **GIVEN** a state that holds a deploy address and no confirmation
- **WHEN** the runner starts again
- **THEN** the runner finds the contract at the address and continues

#### Scenario: Status of FRC-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:38`, `RUNBOOK.md:42,65`. Check: MISSING a fault-injection test; fix 4 of T17 (a).

### Requirement: FRC-003 A proven call is reused while its TTL lasts
FRC-003: While a proven, unrefused call that is not included has more than 5 minutes of TTL left, the ledger runner SHALL reuse the saved call on a rerun.

Verification: T17 (a) rerun; MISSING a test at 4 minutes of TTL; exists in part.

#### Scenario: Resume after a network loss
- **GIVEN** a saved unbound call A with 40 minutes of TTL left
- **WHEN** the runner starts again
- **THEN** the log reads "resumed the proven unbound A" and no proof is made

#### Scenario: Status of FRC-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. A call that was submitted and resolved as not included (FRC-013) is reused too: it carries the same claim, and LDG-002 makes a wrong resubmit cost a fee and no state (FAL-NETOPS-2, LEAD PICK for T15). Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:58,675-676,694`. Check: T17 (a) rerun; MISSING a test at 4 minutes of TTL.

### Requirement: FRC-004 Transport errors are retried
FRC-004: If wallet balancing fails with a transport error, then the ledger runner SHALL make at most 4 attempts in total, 30 s apart, with a wallet resync before each new attempt.

Verification: MISSING a fault-injection test of the bound; the T17 (a) second run needed 0 retries; to build.

#### Scenario: The 18:40 network loss
- **GIVEN** an indexer client that returns no HTTP response
- **WHEN** balancing fails 4 times
- **THEN** the runner stops the control after the fourth attempt and records each attempt

#### Scenario: Status of FRC-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: PVW-007, OBS-008. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:763-776` (`T10A_BALANCE_TRIES`, default 4); `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:42`. Check: MISSING a fault-injection test of the bound; the T17 (a) second run needed 0 retries.

### Requirement: FRC-005 A refused honest call is proved again
FRC-005: If the node refuses the honest call, then the ledger runner SHALL mark the saved call as refused and prove a new call on the next run.

Verification: MISSING a named test; to build.

#### Scenario: No resubmit of refused bytes
- **GIVEN** a saved call marked refused
- **WHEN** the runner starts again
- **THEN** the saved call is not reused

#### Scenario: Status of FRC-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: LVR-007. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:676,681,725`. Check: MISSING a named test.

### Requirement: FRC-006 The refusals step needs the readback of the honest call
FRC-006: If the readback of the honest call is absent, then the Preview runner SHALL refuse to start the `refusals` step.

Verification: T17 (a) step 4b; exists.

#### Scenario: Controls only
- **GIVEN** an included call A with its readback
- **WHEN** `t17.sh refusals` runs
- **THEN** the four controls run and A is not sent again

#### Scenario: Status of FRC-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RUNBOOK.md:30`; `tools/sherlock-sdk/t10a-ledger-run.mjs:689-692`. Check: T17 (a) step 4b.

### Requirement: FRC-007 Long runs are durable units
FRC-007: When a run is planned to last more than 30 minutes, the tested-design loop SHALL start the run as a `systemd-run --user` unit with an append-mode log file.

Verification: `systemctl --user status <unit>` for each running row (exists); exists.

#### Scenario: A closed terminal does not stop a run
- **GIVEN** unit `t17-preview-a`
- **WHEN** the terminal closes
- **THEN** the unit keeps running and `unit.log` keeps growing

#### Scenario: Status of FRC-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: loop rule (LOOP.md:99); `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RUNBOOK.md:34-40`; units `t23-partA` and `t23-partA-finalize` (LOOP.md:45). Check: `systemctl --user status <unit>` for each running row (exists).

### Requirement: FRC-008 Recovery after a reboot
FRC-008: When muninn starts after a crash, the tested-design loop SHALL restart each killed run from its resumable state as a new user unit.

Verification: the wiki `incident` entries (exist); no drill; exists in part.

#### Scenario: The 2026-11-14 crash
- **GIVEN** a reboot during the T17 controls
- **WHEN** the lead resumes
- **THEN** an `incident` entry is logged and the controls rerun from the saved state with 0 retries

#### Scenario: Status of FRC-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. Evidence: crash recovery rule (LOOP.md:104); `t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:42`. Check: the wiki `incident` entries (exist); no drill.

### Requirement: FRC-009 A gate that fails twice stops its row
FRC-009: If a gate fails 2 times in a row, then the tested-design loop SHALL stop that row and keep the other rows running.

Verification: MISSING (a rule of the lead loop; checked by the lead verdict); to build.

#### Scenario: A failing wrap gate
- **GIVEN** the ledger check failing twice on one key set
- **WHEN** the loop ticks
- **THEN** the row stops with the two logs and the other rows continue

#### Scenario: Status of FRC-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. Evidence: loop rule (LOOP.md:100). Check: MISSING (a rule of the lead loop; checked by the lead verdict).

### Requirement: FRC-010 The work is durable in git
FRC-010: When a row reaches DONE, the tested-design loop SHALL hold every file of the row in a commit on the row's branch.

Verification: `git status --short` per worktree and the T0 patch round trip (both exist as T0 records). A clean worktree alone does not show the round trip; exists in part.

#### Scenario: Row T0
- **GIVEN** the E-path worktrees
- **WHEN** T0 closes
- **THEN** `git status` is clean in each, and a fresh openvm clone plus the 14 patch series rebuilds the tip trees byte-identical

#### Scenario: Status of FRC-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: RKY-002 (every rekey list row has a commit, completion). Evidence: row T0 (LOOP.md:17: 14 patch series round-trip to the tip trees, MEASURED). Open today: `t23-args` staged, openvm `t12-slot15` dirty (AUDIT.md items D1 and A7a). Check: `git status --short` per worktree and the T0 patch round trip (both exist as T0 records). A clean worktree alone does not show the round trip.

### Requirement: FRC-011 Pushes wait for the freeze
FRC-011: If a push names a branch other than `t10q` before the owner signs T15, then the tested-design loop SHALL hold the push.

Verification: `git log origin/<branch>` unchanged (exists); exists.

#### Scenario: The e2e spec branch
- **GIVEN** branch `sherlock-e2e-spec` with its commit
- **WHEN** the lanes post DONE
- **THEN** no push happens

#### Scenario: Status of FRC-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. References: LPC-014, LPC-015 (sherlock-completion-20261115). Evidence: AFK rule (LOOP.md:95), lead decision on O11 (LOOP.md:89); BRIEF.md "Never push". Check: `git log origin/<branch>` unchanged (exists).

### Requirement: FRC-012 The suite settle resumes at the call
FRC-012: When the T23 settle run starts again after a stop, the T23 suite driver SHALL skip each call whose `runs[claim_pub]` readback equals its `outcome_pub`.

Verification: MISSING. GAP owner: row T23 driver; to build.

#### Scenario: A stop after 60 of 145 calls
- **GIVEN** 60 settled calls on Preview
- **WHEN** the driver restarts
- **THEN** the driver settles the other 85 and sends no call twice

#### Scenario: Status of FRC-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: STL-004; LDG-002 (a replay is refused on chain as well). Evidence: the contract refuses a settled claim, `t20-batched@5dc1be2ee:tools/sherlock-sdk/sherlock-ledger.compact:57`; no resume code in `t23-suite@16836929b:tools/t23-suite/t23.py`. Check: MISSING. GAP owner: row T23 driver.

### Requirement: FRC-013 An unknown inclusion state is resolved first
FRC-013: If the inclusion state of a submitted call is unknown, then the ledger runner SHALL resolve the state from the node and the indexer before the ledger runner builds another submission of the same claim.

Verification: MISSING: the states pending, included, refused and expired are not a defined machine; no crash or indexer-lag test; to build.

#### Scenario: Indexer lag after inclusion
- **GIVEN** a call included in a block and not yet indexed after 60 s
- **WHEN** the runner starts again
- **THEN** the runner finds the call by its transaction hash and does not mark the saved call refused

#### Scenario: Status of FRC-013
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: TO BUILD. References: FRC-002, FRC-003, FRC-005, FRC-012; LDG-002 (the contract refuses a second settle of one `claim_pub`, so a wrong resubmit costs a fee and no state). Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:683-726` (a call that is `included` and not indexed is marked refused, line 689). Check: MISSING: the states pending, included, refused and expired are not a defined machine; no crash or indexer-lag test.

### Requirement: FRC-014 Saved artifacts are checked before reuse
FRC-014: When the ledger runner reuses a saved proof or state file, the ledger runner SHALL check the SHA-256 of the file, the deployed key and the genesis hash against the run state before the reuse.

Verification: MISSING the hash check and a corruption test; the key check of the state folder exists; exists in part.

#### Scenario: A truncated proof file
- **GIVEN** a saved unbound call whose file was cut by a crash
- **WHEN** the runner starts again
- **THEN** the hash check fails and the runner proves a new call

#### Scenario: Status of FRC-014
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: FRC-003, SCL-004, LVR-008. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:676,694` (reuse by presence and TTL); `RUNBOOK.md:44` (key check of the state folder). Check: MISSING the hash check and a corruption test; the key check of the state folder exists.

### Requirement: FRC-015 Integration is recorded with ancestry and suites
FRC-015: When a lane tip merges into the integration branch, the tested-design loop SHALL record a true result of `git merge-base --is-ancestor` for the tip and the pass result of each required suite on the merged tree.

Verification: the T1 record (exists); MISSING for the merges still open: `t20-batched`, `t10q-ntfix`, `t11-zk`, `t22-governance`; exists in part.

#### Scenario: Row T1
- **GIVEN** the lane tips of fp8, e5-wrap, e6-int and x-exp
- **WHEN** `tested` is built
- **THEN** every tip is an ancestor and the suites pass, the ignored heavy gates i0, i5_l and i10 included

#### Scenario: Status of FRC-015
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: PARTIAL. References: LPC-010 (T20 merge), LPC-014 (T15 freeze presentation and owner sign-off), LPC-015 (pushes), completion. The freeze itself stays LPC-014. Evidence: row T1 (LOOP.md:18), `tested@52b71e250`. Check: the T1 record (exists); MISSING for the merges still open: `t20-batched`, `t10q-ntfix`, `t11-zk`, `t22-governance`.

### Requirement: FRC-016 Other balancing errors stop the call
FRC-016: If wallet balancing fails with an error other than a transport error, then the ledger runner SHALL record the attempt and stop the call without a retry.

Verification: MISSING a fault-injection test with a non-transport balancing error; to build.

#### Scenario: Insufficient funds
- **GIVEN** a wallet that cannot balance the fee
- **WHEN** balancing fails with that error
- **THEN** the runner records one attempt and stops the call

#### Scenario: Status of FRC-016
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT, not tested. Added for FAL-NETOPS-3 (LEAD PICK for T15). References: FRC-004, OBS-008. Evidence: `t17-preview@9e1e7f398:tools/sherlock-sdk/t10a-ledger-run.mjs:764-775` retries only transport-like errors and stops at once on any other error. Check: MISSING a fault-injection test.
