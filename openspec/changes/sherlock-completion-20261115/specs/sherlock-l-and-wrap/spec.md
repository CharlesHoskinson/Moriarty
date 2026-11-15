## ADDED Requirements

### Requirement: LW-020 Typed MEM_BUS messages
LW-020: The `L_Fr` circuit SHALL carry the type element `TY_BB` = 0 or `TY_FR` = 1 as the sixth element of every MEM_BUS message.

#### Scenario: Typed MEM_BUS messages
- **WHEN** a tape reads an Fr value as a BabyBear operand
- **THEN** the MEM_BUS argument does not balance and the proof fails

#### Scenario: Verification and status of LW-020
- **WHEN** the status is reported
- **THEN** the source is C-A2c, PK1 (LEAD PICK in the single rekey list that the owner confirmed on 2026-11-15), the verification is `t13` F3a and F3c Poseidon cases with bus 2 and bus 3 (openvm `rekey-chips` fe3601c, 4437a74), and the L_Fr positive controls of odin r1, and the status on 2026-11-15 is: DONE on openvm `rekey-chips`, keys change at the rekey.

### Requirement: LW-021 Ill-typed tapes are refused
LW-021: If a tape has an ill-typed operand, then the `L_Fr` host SHALL refuse the tape in `Machine::new`.

#### Scenario: Ill-typed tapes are refused
- **WHEN** a tape passes an Fr cell to `BbAlu`
- **THEN** `Machine::new` returns an error

#### Scenario: Verification and status of LW-021
- **WHEN** the status is reported
- **THEN** the source is C-A2c (LEAD PICK), the verification is `Machine::try_new` refusal test (openvm `rekey-chips` fe3601c), and the status on 2026-11-15 is: DONE.

### Requirement: LW-022 WHIR shape check
LW-022: If a verifying-key WHIR shape has `log_rs < k`, then the wrap host SHALL refuse the shape before it synthesizes a cell.

#### Scenario: WHIR shape check
- **WHEN** a key gives `log_rs = 3` and `k = 4`
- **THEN** `WhirShape::check` refuses it and the cs loop does not run

#### Scenario: Verification and status of LW-022
- **WHEN** the status is reported
- **THEN** the source is wrap-levers C3.1 (LEAD PICK, no key change), the verification is `wl_c31_c52_shape_guards` (Moriarty `t10q-ntfix` 95990a6d3), and the status on 2026-11-15 is: DONE on `t10q-ntfix`. `LIST.md` still says NOT committed.
