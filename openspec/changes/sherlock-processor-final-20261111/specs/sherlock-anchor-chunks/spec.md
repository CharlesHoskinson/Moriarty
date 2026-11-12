## ADDED Requirements

### Requirement: ANC-001 The anchor cell holds a salted program value
The anchor cell `(0)` SHALL hold `C_P = H(SHK_PROGC, r_P, prog_d)`, with one salt `r_P` per deployment, or a lock value `A_j`. It SHALL NOT hold plain `prog_d`. User code SHALL NOT address cell `(0)`.

#### Scenario: User code writes the anchor
- **WHEN** an entry of user code writes cell `(0)`
- **THEN** the frontend refuses the contract at compile time

### Requirement: ANC-002 Anchor values of a chunked run
The anchor values SHALL be `A_0 = A_k = C_P` and `A_j = PoseidonFr_4(SHK_LOCK, pack(P2(DOM_LOCK, r_P, prog_d, run_tag, j)))` for `1 <= j < k`. `run_tag` SHALL be a fresh salted value per run (OD-16).

#### Scenario: Last chunk
- **WHEN** chunk `k` of a run completes
- **THEN** the anchor holds `C_P` again

### Requirement: ANC-003 The anchor statement per case
Every call SHALL carry the anchor statement `(mode, j, k, a_in, a_out)`. An ordinary call SHALL have `mode` ORD with `a_in = a_out = C_P` and no anchor write. Chunk `j` of `k` SHALL have `mode` CHUNK with `a_in = A_(j-1)` and `a_out = A_j`. It SHALL write `ins anchor := a_out` in the effect region (OD-16).

#### Scenario: Chunk with the wrong lock
- **WHEN** chunk `j` reads an anchor other than `A_(j-1)`
- **THEN** the `popeq` of the anchor fails on the ledger, and the wrap refuses an `a_in` that is not the `popeq` lane (LW-005)

### Requirement: ANC-004 A chunked call is one intent of one transaction
One logical call SHALL run as `k` contract calls in one intent of one transaction on the unmodified ledger. Chunk `j` SHALL claim chunk `j+1`. The anchor lock SHALL hold between chunks.

#### Scenario: Interleaving by another intent
- **WHEN** another intent places a call of the contract between two chunks
- **THEN** that call fails on the anchor lock

### Requirement: ANC-005 Mode 1F is the default
Mode 1F SHALL put all chunks in the fallible part of one segment. A failure of a middle chunk SHALL leave the contract state unchanged. The transaction SHALL be included with its fee paid.

#### Scenario: Middle chunk fails in 1F
- **WHEN** chunk 2 of 3 fails in mode 1F
- **THEN** no chunk is applied and the fee is paid (R-outcome)

### Requirement: ANC-006 Mode 1G for a guaranteed caller
Mode 1G SHALL put all chunks in the guaranteed part, for a callee of a guaranteed caller. A failure SHALL make the transaction invalid. A chain that mixes fallible and guaranteed chunks SHALL be refused.

#### Scenario: Mixed chain
- **WHEN** a chain has one fallible chunk and one guaranteed chunk
- **THEN** the frontend refuses the chain (R-phase)

### Requirement: ANC-007 Cut and context rules
The frontend SHALL cut a run only where the native run has stack depth exactly 3, at a boundary that the run certifies. The slice shard SHALL check the cut (R-cut). Every read of context slot 0 and effects slot 1 SHALL be hoisted into a prologue in chunk 1 (R-ctx).

#### Scenario: Cut at another depth
- **WHEN** a cut falls where the native stack depth is not 3
- **THEN** the slice shard refuses the cut

### Requirement: ANC-008 A chunked call is atomic
A chunked call SHALL have no dropped suffix, no dropped prefix and no interleaving inside or outside its intent. Replay SHALL be bound through `binding_input`. Fees SHALL be checked for the whole transaction before application.

#### Scenario: Dropped suffix or prefix
- **WHEN** a submitter drops the last chunks or the first chunk
- **THEN** the ledger refuses the suffix drop with `RealCallsSubsetCheckFailure` and the prefix drop at the `popeq` of `A_(j-1)`

### Requirement: ANC-009 Native-routed entries run the anchor prologue
A hybrid contract SHALL have a second cell `anchor_home` that deploy sets to `C_P` and that no entry writes. A native-routed entry SHALL start with the uncached reads `popeq anchor` and `popeq anchor_home`. Its circuit SHALL assert that the two values are equal (OD-8, OD-16).

#### Scenario: Native call between chunks
- **WHEN** a native-routed entry with the prologue is placed between chunks `c1` and `c2`
- **THEN** the call is refused (T2-N), and the same entry without the prologue is included with its write inside the run (control T2-N0)

### Requirement: ANC-010 Chunk tests and open items
The chunk tests SHALL run in order: T1 at ledger level with 13 cases and no proofs, T2 on the XC-9 devnet with `chunky.compact` and the controls T2-N and T2-N0, then T3 with universal-key chunks after frontend M5. Under placement V, `bind_label` SHALL compute the claim address and `ep_hash` of each chunk from the pinned windows (FPR-014, LEAD F-13), and the run SHALL write `(addr_c, ep_hash, comm_(j+1))` into the effect words of its `claimContractCall`.

#### Scenario: T3 with a full log
- **WHEN** T3 runs chunk 1, chunk `j` and chunk `k` whose discretionary operations fill the log
- **THEN** every chunk keeps its reservation `R` and spills the rest to tier E (FLG-006, FLG-007)

### Requirement: ANC-011 Lean obligations of the anchor and chunks
The anchor and chunk rules SHALL be reported proved only when CF-L2 is proved with an axiom report that holds only `propext`, `Quot.sound` and `Classical.choice`.

#### Scenario: Obligation status
- **WHEN** the status is reported
- **THEN** CF-L2 (anchor atomicity: no call other than chunk `j+1` of the same run is valid while the anchor holds `A_j`, `ClaimDag.lean`, proposed) is open
