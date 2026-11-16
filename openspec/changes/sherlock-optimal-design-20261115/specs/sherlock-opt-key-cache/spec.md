## ADDED Requirements

### Requirement: KCH-001 One universal key set
KCH-001: The Sherlock key manifest SHALL hold one universal key set of R_SL bucket app and leaf keys, the interpreter chip-set keys, the collector chip-set key, N1, N2, N3, MERGE-WORK, NW, NW2, FV, `L`, `L_Fr` and the wrap.

Source: FD-20. Hard rule: one universal key set.

#### Scenario: Manifest content
- **WHEN** the final manifest is read
- **THEN** it lists exactly these keys and one parameter set per layer

### Requirement: KCH-002 Tail keys independent of program images
KCH-002: The tail key set SHALL not depend on any program image including the collector's.

Source: FD-20, C-03. The tail key set is the `L`, `L_Fr` and wrap keys. Gate: E-KEYDEP.

#### Scenario: Collector image changed
- **WHEN** E-KEYDEP changes the collector image and regenerates the keys
- **THEN** the `L`, `L_Fr` and wrap VK bytes are equal

#### Scenario: Key cycle
- **WHEN** the collector reads its own `prog_d`
- **THEN** E-KEYDEP shows no key cycle through `key_root`

### Requirement: KCH-003 Key-derived public data ships with the keys
KCH-003: The Sherlock key manifest SHALL ship the key-derived public data with the keys: each child `X` of `L`, each child's full committed trace and opening data, the fixed `L` heights, the proving keys, the SRS and the wrap proving key.

Source: FD-21 (A-06).

#### Scenario: No recomputation
- **WHEN** a warm call runs on the resident service
- **THEN** zero child-VK commits and zero key loads happen inside the timed call (E-S1)

### Requirement: KCH-004 Binding of the loaded key data
KCH-004: When the resident service loads the key data, the resident service SHALL re-derive `key_root`, `keys_d` and the `L` vk hash from the full child-VK bytes, the parameter digest and the binary sha256.

Source: FD-41 (A-06).

#### Scenario: Load check
- **WHEN** the service starts with a correct manifest
- **THEN** the re-derived `key_root`, `keys_d` and `L` vk hash equal the manifest values

### Requirement: KCH-005 Refusal on a key-data mismatch
KCH-005: If a check of the loaded key data fails, then the resident service SHALL refuse to start without running a keygen.

Source: FD-41 (A-06).

#### Scenario: Altered child VK
- **WHEN** one byte of a child VK in the cache is changed
- **THEN** the service refuses to start and no timed keygen runs

### Requirement: KCH-006 Node key survival is measured
KCH-006: When the collection change is keyed, the rekey runner SHALL compare the serialized N1, N2 and N3 VKs before and after and record the result.

Source: FD-22 (C-06 (b), 5 of 6). The design does not require N1 to N3 unchanged; the `FOREIGN` head form of CSL-010 moves them. Dissent: fable-1 holds them unchanged as a requirement.

#### Scenario: E-COLL comparison
- **WHEN** E-COLL keys the collector and the extended DL-STEP node
- **THEN** the record states byte-equal or changed for each of N1, N2 and N3
