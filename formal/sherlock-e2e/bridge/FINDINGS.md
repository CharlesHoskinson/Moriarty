# Bridge findings (Formal-Lean-B, 2026-11-15)

Each finding was checked against the real code (openvm clone `~/.cache/m7-wrap/src/openvm-e3`, `git show <commit>:<path>`) and `~/.cache/rekey-t11/LIST.md` (sha256 6392209810…81db, mtime 10:31).

| ID | Requirements | Kind | Evidence | Proposed verdict |
|---|---|---|---|---|
| F-BR-1 | LTR-003 against CLB-002 (FP-3), ANC-018 | contradiction between requirements; spec bound differs from committed code | LTR-003 refuses an `L` key above 2,000 columns. FP-3 (CLB-002) replaces the bare `Call` AIR with `EffectAir` of 77 columns (MEASURED, LIST.md:44), so `L` becomes 1,995 - 1 + 77 = 2,071 columns (DERIVED, LIST.md:47). The two requirements cannot both hold on the T25 keys unless EffectAir shrinks by 72+ columns. The code already moved: openvm fp3 a2eeece and rekey-int 155d321 `crates/sherlock-lfr/src/frozen.rs:202` set `L` `max_columns: 2200` (LEAD PICK FP3-LCOLS, for T15), so code and spec text differ. Lean: `Bridge.ltr003_fp3_conflict`, `Bridge.ltr003_fp3_refused` (the oracle, which follows the spec, refuses the FP-3 `L` key). | CONFIRMED (spec/code divergence on recorded values). Fix: LTR-003 text to 2,200 with the FP3-LCOLS condition (T25 measures the width; k <= 21 and VK <= 50,000 B hold) or shrink EffectAir; spec owner Fable-B, LEAD PICK for T15 |
| F-BR-2 | CLB-005, ANC-002, ANC-017 against CLB-003/ANC-019 (FP-3 forward chain) | stale operand list against the merged FP-3 code; touches the x_j hard rule (D-013) | The sentences give `x_j = sponge(U_LOCK; c, pack(run_tag), k, d_in, d_out, j)` (6 operands; code t10q-ntfix e3b407d `iface.rs:281` `LOCK_LEN = 6`). FP-3 (fp3 a2eeece, merged in rekey-int 155d321) absorbs a seventh operand `link = pack(C_(j+1))` between `d_out` and `j` (`iface.rs:288` to `:290` `LOCK_LEN = 7`, `lock_x` at `:316`); LIST.md FP-3 row: "lock absorbs the chain link, LOCK_LEN 7". Read literally, CLB-005 refuses every honest FP-3 chunk (its x_j differs from the 6-operand sponge), and the capacity `cap(8, 6)` differs from `cap(8, 7)`. Lean: `Bridge.clb005_fp3_operands`. CLB-005 status scenario notes "FP-3 adds the link operand", but the requirement text does not. | OPEN DESIGN: the x_j form is an owner hard rule ("x_j lock form is canon", 2026-11-14, D-013); extending it by `link` needs owner confirmation at T15, then CLB-005, ANC-002, ANC-017 text update |

Checked and not a finding:
- CLB-001 12,166 words, 5 lane pairs: CODE 5b7ada8 `iface.rs:42`, `:1457`; the test still asserts 12,166 on rekey-int 155d321 (`iface.rs:1587`) after FP-3 and slot 15.
- CLB-004 slot 15: CODE t12-slot15 4ea3d05 and rekey-int 155d321 `iface.rs:112`.
- CLB-007 K_MAX = 2: MODEL t20-batched 5dc1be2ee `chunkCap.qnt:26`; no Rust constant (the spec status says so; GAP, not a finding).
- HDM-001 lock use code 8 = CODE `U_LOCK = 8` (e3b407d `iface.rs:277`).
- LTR-003 on the T10m keys: MEASURED 1,995 of 2,000 columns, 99 of 100 interactions (RESULTS.md:11, :49).

## Resolution (2026-11-15)

- F-BR-1: FIXED in the text. LTR-003 states 2,200 columns, the bound of openvm `rekey-int` 3826e38 `frozen.rs:202` (LEAD PICK FP3-LCOLS for T15). Lean `ltr003_fp3_fits`, `ltr003_fp3_accepted`.
- F-BR-2: FIXED in the text. The lock has 7 operands with the FP-3 link `pack(C_(j+1))` before `j` (D-038 OWNER; LEAD PICK for T15). Odin test `cx/fbbr2_test.rs`: the 6-operand lock is refused. Mutant `cx/fbbr2_link_constant_mutant.py`: a constant link lets a 3-slice run close as 2 chunks. Lean `clb005_fp3_operands`.
