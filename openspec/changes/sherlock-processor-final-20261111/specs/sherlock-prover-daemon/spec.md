## ADDED Requirements

### Requirement: PRD-001 The Sherlock prover is a resident daemon
The Sherlock prover SHALL be a resident daemon that listens on a local socket (OD-13). It SHALL serve only a peer of the same user on the same machine.

#### Scenario: Remote peer
- **WHEN** a peer that is not a local socket of the same user connects
- **THEN** `serve_local` refuses the connection

### Requirement: PRD-002 One-use blinding slots
The daemon SHALL hold a pool of one-use blinding slots with fresh CSPRNG blinders. Each slot SHALL be consumed by exactly one `wrap_prove`. A slot SHALL NOT be serialized off the machine. A slot made by a helper SHALL be refused.

#### Scenario: Reused slot
- **WHEN** a second `wrap_prove` takes a slot that was used
- **THEN** the daemon refuses the call (daemon test)

#### Scenario: Helper-made slot
- **WHEN** a slot arrives from another host
- **THEN** the daemon refuses the slot

### Requirement: PRD-003 Proofs and checkpoints stay in the trust domain
No STARK proof, prover checkpoint or shard plan of a private program SHALL leave the daemon's trust domain. The trust domain SHALL be the user's own devices and GPUs (OD-9).

#### Scenario: Export request
- **WHEN** a client asks the daemon for its outputs
- **THEN** only the wrap proof and the call vector leave the process

### Requirement: PRD-004 Path C and the daemon come together
The daemon and its slot pool SHALL be built with path C in build step 15. A one-leaf Compact call SHALL take path C: one leaf, one leaf layer at internal parameters, no internal node, then `L` and the wrap.

#### Scenario: One-leaf call
- **WHEN** a one-leaf Compact call is proved through the daemon
- **THEN** the daemon runs path C, and D12 measures its STARK part

### Requirement: PRD-005 Lean obligations of the prover daemon
The design lists no Lean obligation for the prover daemon. Its zero-knowledge role SHALL rest on ZK-L1 and premise A7 of kit ZERO KNOWLEDGE (ZK-010).

#### Scenario: Obligation status
- **WHEN** the status is reported
- **THEN** it reads "no Lean obligation in the design", and the daemon tests of PRD-001 to PRD-003 are the evidence
