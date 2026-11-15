# Proposal: completion of the tested Sherlock design (`sherlock-completion-20261115`)

Status: S2 (written requirements). This change builds, keys, proves and measures nothing. Each requirement states its status on 2026-11-15 and its evidence. Audit: `research/sherlock-spec-audit-20261115/AUDIT.md`.

## Why

The owner ordered on 2026-11-15: "Audit this checklist to confirm all work items. Make sure the openspec is accurate and we have EARS for each item." The lead checklist of 2026-11-15 (`~/.cache/lead/CHECKLIST-20261115.md`) lists the work that remains before the interface freeze (row T15). Many of its items have no requirement in `sherlock-processor-final-20261111` or `sherlock-modular-leaf-20261111`: the R_SL chip set and its kits, the single rekey, slot 15, FP-3, the T11 guards, the T12 legs, the T23 settlement and the smaller rows. This change gives each item one or more EARS requirements with an ID and a verification line. It can move the terminal decision because T15 counts a row only with its done-when evidence, and this change names that evidence per item.

## What Changes

- New EARS requirements, one spec per module. Each requirement holds one EARS sentence, a source (owner words, a built design or a LEAD PICK), a verification line and a status.
- `sherlock-processor-final-20261111` and `sherlock-modular-leaf-20261111` are corrected in place to the built design and to the owner rulings: ANC-001 (slot 15), ANC-002 (`x_j` lock form), HDM-001 (use code 8), HDM-007 (`DOM_LOCK` out of the lock), SPL-002 and SPL-011 (setup mode, F-3 replaced), ZK-001, ZK-003, ZK-008, ZK-009, ZK-010 (case A, blinder lifetimes, `test-seed`, ZK-L2 built), CF-005 (R_SL chip set 3), LW-018 (typed MEM_BUS), SQG-013 and SHP-008 (MERGE-WORK 5,400 owner-confirmed with the lead condition).

## Scope

| Spec folder | Module | ID prefix | Checklist items |
|---|---|---|---|
| `sherlock-rsl-chipset` | R_SL chip set 3 and its kits | RSL | A1 to A4, rekey rows |
| `sherlock-compact-frontend` | Mycroft lowering of secp256k1 values | CF | A4 |
| `sherlock-anchor-chunks` | slot 15, NT fixes, FP-3 | ANC | A6, A7, A8 |
| `sherlock-l-and-wrap` | typed MEM_BUS, WHIR shape check | LW | A8 |
| `sherlock-zero-knowledge` | T11 blinders, guards, EQ-ZK, slots, GPU, case B | ZK | A5 |
| `sherlock-rekey` | the single rekey and its measurements | RKY | B |
| `sherlock-security-gate` | T12 legs and gates on the new keys | SQG | C |
| `sherlock-suite-settlement` | T23 on Preview | STL | D |
| `sherlock-loop-completion` | rows T8b, T8c, T13, T14, T16, T17, T19 to T22, Mycroft gates, T15, pushes, missing items | LPC | E, F, missing |

IDs continue the series of the processor-final change. ANC-012 is taken on branch `t20-batched`, so this change starts at ANC-013.

## Canon rule

Only the owner's words are canon. A requirement that cites an owner ruling quotes it or names the wiki log entry that quotes it. Every other choice reads LEAD PICK or built design.

## Non-goals

- No change to a hard rule (LOOP.md line 86).
- No edit of lane branches: `t20-batched`, `t22-governance`, `t16-elastic`, `prover-gui`, the mycroft branches and the openvm trees keep their text. AUDIT.md lists the text that those lanes must change.
- No Mycroft spec edit: `mycroft/` is not on main or on the integration branch. AUDIT.md lists the notes for the Mycroft lane.
