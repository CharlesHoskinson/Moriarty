## ADDED Requirements

### Requirement: CF-001 The frontend taps `Lflattened`
The frontend SHALL read `Lflattened` after `optimize-circuit2`, the exact input of `reduce-to-zkir`. The manifest and the TypeScript runtime SHALL come from `analyzed-ir` unchanged. One analysis pass `add-sherlock-anchor` SHALL follow `determine-ledger-paths`. The frontend SHALL replace `reduce-to-zkir`, the `zkir-v3 compile-many` key step and the per-circuit key fingerprint.

#### Scenario: Tap point drift
- **WHEN** the compiler version changes the input of `reduce-to-zkir`
- **THEN** the golden vectors of CF-003 fail, and the frontend is not released on that compiler

### Requirement: CF-002 The frontend commands and `abi.json`
The frontend SHALL provide `sherlock compile`, `sherlock plan`, `sherlock simulate`, `sherlock deploy`, `sherlock prove` and `sherlock call`. `abi.json` SHALL give per entry the step count `N_e`, the effect words `L_e`, the tape length and the count per builtin.

#### Scenario: Plan without a witness
- **WHEN** `sherlock plan` runs on an image
- **THEN** it reads no witness and outputs the per-program keys and `prog_d` (SPL-002)

### Requirement: CF-003 The effect vector equals the native public-input vector
For every entry and preimage, `effect_vector(preimage)` SHALL equal the native public-input vector word for word.

#### Scenario: Golden vectors at M0
- **WHEN** milestone M0 runs the golden vectors of every example entry
- **THEN** every effect vector equals the native vector, four mutants are caught, and CF-L1 (`CompactEffect.lean`, proposed) is open

### Requirement: CF-004 The step count of an entry does not depend on the tape
`N_e` SHALL be the same for any two tapes of one entry.

#### Scenario: Reference machine at M1
- **WHEN** milestone M1 runs 100 random preimages per entry on the reference machine over `R_SL_*`
- **THEN** every run equals native, `N_e` is equal for two tapes, and `spend` takes at most 550 steps

### Requirement: CF-005 Lowering of Compact builtins
Stage 1 SHALL lower in straight-line mode over `R_SL_*`. Compact `Field` SHALL stay emulated on FIELD-EMU (OD-14). `transientHash` SHALL be `SPONGE(n)` in the log or POSEIDON-FR-EMU inline. `persistentHash` SHALL run on the `sha2` chip. `ecMul` and `ecMulGenerator` SHALL be log entries while `E` slots last, else JUBJUB-EMU.

#### Scenario: Native call with no kit
- **WHEN** an entry calls a native builtin that no kit covers
- **THEN** compilation fails with an error that names the missing kit

### Requirement: CF-006 Tape and effect layout
The tape SHALL be `Cons(salt, Cons(selector, args ++ comm_rand ++ witness, read and callee results in program order))`, static in length and layout per entry. Ledger reads SHALL be `popeq` values from the tape. Writes SHALL be effect words at static indices. A guarded-off impact SHALL write zeros. The host SHALL insert the `Noop` merges of the native builder.

#### Scenario: Stale read
- **WHEN** a ledger value changes between proving and inclusion
- **THEN** the ledger refuses the stale `popeq` read at frontend M3

### Requirement: CF-007 Entries above `L_TX`
An entry with `L_e > L_TX - A` SHALL be a compile error that names the entry and its `L_e` until the chunked mode exists. With the chunked mode it SHALL run as a chunked call (ANC-004). `L_TX` SHALL be 1,024 (OD-7).

#### Scenario: Large entry before the chunked mode
- **WHEN** an entry needs more effect words than the call vector holds and the chunked mode is not built
- **THEN** compilation fails with the entry name and `L_e`

### Requirement: CF-008 E-1 placement V binds the entry label
`bind_label(e)` SHALL read `m` from the private tape, pin `m[0 .. 26) = "midnight:binding-input[v1]"` and `m[58 .. 58 + |enc(e)|) = enc(e)`, compute SHA-256 on the `sha2` chip, and write `bind = le31(digest)` into the outcome. The wrap SHALL require lane 0 to equal `bind` (LW-005). This placement SHALL follow OD-5.

#### Scenario: Cross-entry attack
- **WHEN** a proof of one entry's transcript is placed in a call labelled with a second entry that holds the same key
- **THEN** the bind gadget refuses the witness, as the prototype does in MockProver and in a real prove and verify at k = 13

#### Scenario: Bind collision bound
- **WHEN** the real label `e'` differs from `e`
- **THEN** `m' != m`, and an accepted call needs a collision of SHA-256 truncated to 248 bits, about 2^124 hash calls

### Requirement: CF-009 V+ pins the label layout to a key constant
`bind_label` SHALL read the domain bytes, their length and the label offset from the tape. It SHALL use exactly those values for the window checks. It SHALL write `layout_d = P2(DOM_LAYOUT, |domain|, domain, offset)` as one statement word. The wrap SHALL pin `layout_d` to a key constant (OD-5).

#### Scenario: Ledger layout change
- **WHEN** the ledger changes the binding layout
- **THEN** one wrap key constant changes and no `prog_d` changes

### Requirement: CF-010 E-1 evidence gates the design
T0 SHALL gate placement V. The frontend SHALL record T0 to T6. The open items SHALL stay open until measured: the remaining T2 mutants, the `label_d` link, T3 to T6, T0 on calls with a fallible transcript or non-empty effects, and the window position under a variable-length SHA-256.

#### Scenario: T0 and T1 on the local devnet
- **WHEN** T0 rebuilds `m` from public transaction bytes and T1 runs the attack on a local ledger 9.1.0.1 devnet
- **THEN** T0 passes with the window at offset 58 and T1 shows that E-1 is real (MEASURED, `PR` section 7.2), and every other E-1 test is PENDING

### Requirement: CF-011 Frontend milestones
The frontend SHALL pass the milestones in order: M0, M1, M2a, M2b, M3, M4 and M5. M2a SHALL give byte-equal internal-recursive keys across two chip sets. M2b SHALL give one wrap proof that the ledger 9 verifier accepts with `2 + L_TX` public inputs. M3 SHALL deploy and call on the local devnet.

#### Scenario: Local devnet refusals at M3
- **WHEN** a call has a tampered proof bit, another `C_P` or a stale read
- **THEN** the ledger refuses each call

### Requirement: CF-012 Lean obligations of COMPACT FRONTEND
The kit SHALL be reported complete only when CF-L1 is proved with an axiom report that holds only `propext`, `Quot.sound` and `Classical.choice`.

#### Scenario: Obligation status
- **WHEN** the kit status is reported
- **THEN** CF-L1 (effect equivalence, `clean/fork/Clean/Sherlock/CompactEffect.lean`, proposed) is open, and CF-L2 belongs to the anchor spec (ANC-011)
