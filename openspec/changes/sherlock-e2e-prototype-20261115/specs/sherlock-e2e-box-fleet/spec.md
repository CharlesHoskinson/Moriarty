## ADDED Requirements

### Requirement: BXF-001 Only boxes in the owned-box roster prove
BXF-001: If a proving job names a host that is not in the owned-box roster of the user, then the job runner SHALL refuse the job.

Verification: NEW: a roster file that the user writes, read by the job runner; tests: a host outside the roster is refused, a fourth box added to the roster is admitted. To build.

#### Scenario: A rented host
- **GIVEN** a job for host `gpu-rental-1` that is not in the roster
- **WHEN** the runner reads the roster
- **THEN** the runner refuses the job

#### Scenario: A fourth owned box
- **GIVEN** a roster of muninn, huginn, odin and a new owned box
- **WHEN** a job for the new box starts
- **THEN** the runner admits the job

#### Scenario: Status of BXF-001
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: GAP. Owner lane: box tooling (lead). Evidence: Owner 2026-11-15 09:05 (`~/Projects/Moriarty/llm-wiki/wiki/log.md:4111` to `~/Projects/Moriarty/llm-wiki/wiki/log.md:4113`): case A is any box the user owns; case B later through Veritee. ZK-018 has no enforcing code. The factory fixtures are muninn, huginn and odin (LOOP "Compute boxes" rule). Corrected after Astra-B review R-05: the roster is the user's list of owned boxes, not three fixed host names.

### Requirement: BXF-002 Heavy jobs on muninn take a slot
BXF-002: When a prover, keygen or gate job starts on muninn, the job runner SHALL start the job through `heavy-slot`.

Verification: NEW: the T25 and T23 unit files call `heavy-slot`; checked by `rg heavy-slot` over the run scripts. To build.

#### Scenario: Four jobs
- **GIVEN** 3 running jobs and a fourth request
- **WHEN** `heavy-slot` is called
- **THEN** the fourth job waits for a slot

#### Scenario: Status of BXF-002
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (the tool). GAP: LOOP states 90G in one rule and 64G in another; the script is 64. Owner lane: lead. Evidence: `~/.local/bin/heavy-slot` lines 27 to 28: `SLOTS=3`, `BUDGET=64` (GB); lock file `~/.cache/cost-opt/measure/heavy.lock` (lines 23 to 24). `heavy-run` starts the job in `heavy.slice` (`MemoryHigh=64G`, `MemoryMax=72G`). LOOP lead decision "heavy-slot budget: aligned to the slice". The LOOP loop rule still reads "3 shared slots, 90G".

### Requirement: BXF-003 A shared job over the budget is refused
BXF-003: If a shared job asks for more memory than the heavy-slot budget, then the `heavy-slot` tool SHALL refuse the job.

Verification: NEW: `heavy-slot --mem 80G true` exits non-zero; `heavy-slot --exclusive --mem 80G true` runs. To build.

#### Scenario: The T25 keygen
- **GIVEN** a keygen that needs 70 GB
- **WHEN** the job starts as a shared job
- **THEN** `heavy-slot` refuses it and the runner restarts it with `--exclusive`

#### Scenario: Status of BXF-003
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (the tool). The named check is a gap. Owner lane: lead. Evidence: `~/.local/bin/heavy-slot` lines 115 to 116; `--exclusive` takes the exclusive lock (lines 103, 148 to 152).

### Requirement: BXF-004 A GPU job holds the GPU lock
BXF-004: While a GPU proving job runs on a box, the box runner SHALL hold an exclusive `flock` on `~/sherlock/gpu.lock`.

Verification: the T16 run logs; NEW: two runners started together on one box, the second waits. Partly to build.

#### Scenario: Two jobs on huginn
- **GIVEN** a leaf job and a timing job
- **WHEN** both start
- **THEN** the second waits until the first releases the lock

#### Scenario: Status of BXF-004
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT in the T16 runners. Other runners (T23, T25) are not checked. Owner lane: t23-suite. Evidence: Moriarty `t16-elastic` a7a7b8867 `research/sherlock-t16-elastic-20261114/run-huginn.sh:11` and `:24`; the sampler starts without the lock descriptor (`run-huginn.sh:26`).

### Requirement: BXF-005 An orphan lock holder is ended under the narrow rule
BXF-005: If `gpu.lock` is held for more than 10 minutes with no progress by the sampler or monitor of an ended job whose parent is gone, then the box agent SHALL log the holder on the bus and end it with SIGTERM.

Verification: the bus log line before each signal; a review of `~/.cache/box-bus/all.log` at T15. Exists.

#### Scenario: The sampler of a dead run
- **GIVEN** sampler pid 592115 on huginn with no parent
- **WHEN** 10 minutes pass with no progress
- **THEN** the agent logs the pid, then sends SIGTERM

#### Scenario: Status of BXF-005
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT as a rule for the box agents. No script enforces it. Evidence: Owner rule 2026-11-15 (LOOP Finish plan, "Coordination"; huginn `~/sherlock/tooling/ORPHAN-LOCK-RULE.md`).

### Requirement: BXF-006 A prover is never ended by the lock rule
BXF-006: If the holder of `gpu.lock` is a prover or a `wrapd` process, then the box agent SHALL leave the holder running.

Verification: the bus log: no kill line names a prover or `wrapd`. Exists.

#### Scenario: A slow prover
- **GIVEN** a leaf prover that holds the lock for 40 minutes
- **WHEN** the agent checks the holder
- **THEN** the agent takes no action and posts the state on the bus

#### Scenario: Status of BXF-006
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT as a rule. Evidence: The same owner rule: it covers a sampler or monitor only.

### Requirement: BXF-007 Files move with a digest
BXF-007: When a result file moves between boxes, the receiving box SHALL check the sha256 of the file against the sha256 in the bus message.

Verification: the bus messages of the T25 key transfer and the T23 proofs; UKS-004 manifest check. Exists.

#### Scenario: A truncated proof file
- **GIVEN** an `L_Fr` proof of odin that arrives short
- **WHEN** muninn checks the digest
- **THEN** the digest differs and the wrap does not start

#### Scenario: Status of BXF-007
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT as a rule. GAP: no transfer script enforces the check. Owner lane: box tooling (lead). Evidence: LOOP "Compute boxes" rule: "files move only through muninn with path + sha256"; LOOP Finish plan: "every box result returns with paths + sha256 and is checked on muninn".

### Requirement: BXF-008 One timeline of both buses
BXF-008: When a box writes a line to its outbox, the `box-relay` unit SHALL append the line to `~/.cache/box-bus/all.log`.

Verification: `~/.cache/box-bus/all.log` holds the lines of both boxes in time order; the muninn unit `box-relay` is active. Exists.

#### Scenario: A message for odin
- **GIVEN** huginn posts a line with `to:odin`
- **WHEN** `box-relay` reads the outbox
- **THEN** the line is in `all.log` and the odin agent receives it

#### Scenario: Status of BXF-008
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT. Evidence: `~/.local/bin/box-relay` line 7 (`LOG`), line 22 (forwards a message that names `to:odin`, `to:huginn` or `to:all`). LOOP "Compute boxes" rule.

### Requirement: BXF-009 Long runs are durable units
BXF-009: When a run is planned to take more than 30 minutes, the job runner SHALL start the run as a `systemd-run --user` unit with a persistent log.

Verification: `systemctl --user list-units` during T25 and T23 shows the units; the logs survive a reboot (LOOP crash recovery rule). Exists.

#### Scenario: A reboot during keygen
- **GIVEN** the T25 keygen unit
- **WHEN** muninn reboots
- **THEN** the lead restarts the unit from its resumable state and the log of the first part exists

#### Scenario: Status of BXF-009
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT as a rule and in use. Evidence: LOOP loop rule: "runs over 30 minutes in a systemd-run --user unit". Units in use: `t23-partA`, `t16-cde-run6` (LOOP T23, T16).

### Requirement: BXF-010 A dropped box hands its jobs over
BXF-010: If the link to a box drops while the owner is away, then the lead agent SHALL move the queued jobs of that box to another box.

Verification: the bus timeline and the LOOP Finish plan table after a drop. Exists.

#### Scenario: odin drops at night
- **GIVEN** odin with 71 queued suite calls
- **WHEN** the link drops
- **THEN** the lead gives the calls to huginn and logs the move

#### Scenario: Status of BXF-010
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT as a rule. Evidence: LOOP "Compute boxes" rule: "overnight a dropped link waits for the owner and its jobs move to another box"; relink needs the owner PIN (`remote-link --window BOX`).

### Requirement: BXF-011 Speed claims compare one box with itself
BXF-011: The timing runner SHALL state a speed gain from two runs on the same box.

Verification: the T14 and T14b records name the box in each row; LPC-005. Exists.

#### Scenario: odin against huginn
- **GIVEN** a run of 27.77 s on odin and 71.59 s on huginn
- **WHEN** a speed claim is written
- **THEN** the claim compares each run with the baseline of its own box

#### Scenario: Status of BXF-011
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT as a rule. Evidence: LOOP "Compute boxes" rule: "Each box keeps its own baseline: a speed claim compares runs on one box only". LOOP lead decision: "Timing claims come from huginn only; muninn is not a timing box" (GPU clock cap 754 of 3,003 MHz).

### Requirement: BXF-012 No confidential-compute query on muninn
BXF-012: The Sherlock prover SHALL read GPU data with `nvidia-smi --query-gpu` and the CUDA memory call and with no `nvidia-smi conf-compute` call.

Verification: NEW: a text search for the confidential-compute subcommand over the openvm tree and the run scripts gives no hit. To build.

#### Scenario: Setup on muninn
- **GIVEN** muninn with the GB10 driver
- **WHEN** the setup mode runs
- **THEN** the process list shows one `--query-gpu` call and no `nvidia-smi conf-compute` call

#### Scenario: Status of BXF-012
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT (no call exists). The check is a gap. Owner lane: lead. Evidence: openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs:184` to `:185`, `crates/sherlock-leaf/src/sched.rs:144` to `:161`, `sched.rs:184`. Host note 2026-11-14: the `nvidia-smi conf-compute` query crashes the GB10 driver and hangs muninn.

### Requirement: BXF-013 A holder that ignores SIGTERM is killed after a recheck
BXF-013: If the same eligible orphan still holds `gpu.lock` 30 seconds after SIGTERM, then the box agent SHALL end it with SIGKILL.

Verification: the bus log: each SIGKILL line follows a SIGTERM line for the same pid by at least 30 s and repeats the eligibility check. Exists.

#### Scenario: A sampler that traps SIGTERM
- **GIVEN** the orphan sampler of BXF-005 still alive after 30 s
- **WHEN** the agent checks pid, parent and role again
- **THEN** the agent sends SIGKILL and logs it

#### Scenario: Status of BXF-013
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT as a rule. No script enforces it. Evidence: Owner rule 2026-11-15 (`~/Projects/Moriarty/llm-wiki/wiki/log.md:4117`): log first, SIGTERM, then SIGKILL, under the conditions of BXF-005; the 30 s wait is in the rule file on the boxes (`~/sherlock/tooling/ORPHAN-LOCK-RULE.md`). Added after Astra-B review R-28.

### Requirement: BXF-014 Factory files move through muninn
BXF-014: If a file transfer between huginn and odin does not pass through muninn, then the box bus SHALL refuse the transfer.

Verification: NEW: a transfer wrapper on the boxes that accepts muninn as the only peer; a direct odin-to-huginn copy fails. To build.

#### Scenario: A direct copy
- **GIVEN** a leaf proof on odin wanted on huginn
- **WHEN** odin copies it to huginn directly
- **THEN** the wrapper refuses; the file goes odin to muninn to huginn with its sha256

#### Scenario: Status of BXF-014
- **WHEN** the status is reported on 2026-11-15
- **THEN** it reads: BUILT as a rule. No script enforces it. Owner lane: box tooling (lead). Evidence: LOOP "Compute boxes" rule: "files move only through muninn with path + sha256". This is a rule of the factory fleet, apart from the owned-box rule of BXF-001. Added after Astra-B review R-29.
