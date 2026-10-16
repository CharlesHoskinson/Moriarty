# Iteration 003: enabled fetch

- Date: 2026-10-15, the author session date. Root's receipt timestamp is authoritative for UTC ordering.
- Author: Claude Opus 5.5 (`claude-opus-5-5`), high effort, as the user-assigned visual concept author. The same session produced a separate fresh security source review of these bytes. That review is a different document, it is not part of this note, and this note makes no security judgment.
- Source authorship: Astra high wrote the G3 constructor, tests and mapping. The construction requirements and rejection-test prescriptions belong to the Opus enabled-fetch owner requirements, SHA256 `21ab71fa53a05dc9c041d128c02517c84b0a9b1ad3dff2583776bf812a2a1423`. Root wrote `enabled-fetch/expected.json` independently, frozen before the constructor.
- Iterations 000 to 002 stay frozen. This note extends 002 by reference.

## Source pins

Paths are relative to the repository root. The files are uncommitted in the Astra worktree on source base `1fb794d1598e19fe34b9735f90996fc316b743fa`. No branch or commit is recorded, so paths are shown as code. The concept checkout base `f7ff06ed05bcef5fc3540bdde70ddbf6402f5547` is not a source base. The SDK pin is Midnight `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`.

| Path | SHA256 | G3 change |
|---|---|---|
| `crates/mori-backend/src/sherlock_machine/enabled_fetch.rs` | `6c45ab9190ad829db29b039d91050bfd8b82d1ccb66b3dd990538c18415dc8c1` | new |
| `crates/mori-backend/src/sherlock_machine/enabled_fetch_tests.rs` | `b6afffebff3c3d1bed4e2a6b94baa2f410e25a573c31756e2e90c4c27a820973` | new |
| `crates/mori-backend/src/sherlock_machine/mod.rs` | `e2af2fe03d327403224ae983d56672f3f19822d1fa7b3e9477886faf6b817ac7` | exports, test module |
| `crates/mori-backend/src/sherlock_machine/fetch.rs` | `6e71e0dccf687bec051556eeefdc435cef1c61535323d62a3994dc7220cadc87` | none |
| `crates/mori-backend/src/sherlock_machine/identifier_rejection_tests.rs` | `3c5be80e5104827b287cc4f310f88f4b2ae4a084a0518d517d421bb4e3374851` | none |
| `crates/mori-backend/src/sherlock_machine/normal_tests.rs` | `651a47138ab6a9bd37666508ca48de7c1ea9846840f1b56fee2b3a30e5400c8c` | none |
| `models/sherlock/machine-circuit/MAPPING.md` | `e0864af4a8cff8d3b2fb992cf22fc4069b5c67ed84d87727231b17341010c6ab` | none |
| `models/sherlock/machine-circuit/expected.json` | `1c0319aa6c447599eadbfb2020a60d66effc01001282470429d610e3d5867ee7` | none |
| `models/sherlock/enabled-fetch/MAPPING.md` | `c06e4867e79849036d6527da628a1f345ff1a19b0f59cd577e15759ee2ff6b9c` | new |
| `models/sherlock/enabled-fetch/expected.json` | `53045ce217e8abbfff8eeb6996db996290d20fc058bee28e002f70808e3c2d63` | new |

Root holds the author bundle:

| File | SHA256 |
|---|---|
| `WORK_REPORT.md` | `692998f405ea80f24c2e9a39b9f84ba6105cc9220d02e120b7cbcaa2902d4822` |
| `SOURCE-MANIFEST.json` | `d5443f78f5cac94fab05462b39a9bc97c17702e71f94a6ce3effc5f841931507` |
| `SOURCE.patch` | `09585ce40491dc5ceaf4244d3fd1f819d27bd6b9433084eeca571352e9244e7a` |
| `STATIC-CHECKS.json` | `732160f386ef4125a047b770b05ae1dfb0a1fc570d524f41a2bddebee0a978a4` |

**Publication at freeze.** G2 and notes 000 to 002 are in final publication review in a separate isolated checkout and are not merged. This note changes nothing in that package. Root materializes this note into the concept working tree only, then reviews and publishes it with G3.

## Status at this iteration

- **Unregistered.** `lib.rs` declares no `sherlock_machine`.
- **UNRUN:**
  - compilation;
  - five new constructor tests and six inherited tests;
  - serialization;
  - SDK load, check and evaluation;
  - model and k;
  - native checks;
  - proving.
- **2^128 case.** The disabled case with raw index 340282366920938463463374607431768211456 is UNRUN. No host evaluator or witness adapter exists, and the index stays decimal text.
- **Actual output:** UNRUN. Native authority is 0, and old failure, cap and counter records are unchanged.
- **Executed:** only the text, JSON and hash checks in `STATIC-CHECKS.json`.

## What the source gives a board

These observations come from reading the forwarded source; no tool produced them.

- **A sibling constructor.** `emit_enabled_fetch_block` is a separate constructor, and `fetch.rs` is byte-identical to G2. The two are separate boards, not two revisions of one board.
- **No wrapper.** No PublicInput or Output is emitted, so names come from the constructor test: `%dispatch.selected`, `%state.active_rows`, `%branch.raw_index`, `%state.row.{r}.amount` and `.enabled`, and `%enabled_fetch.temp.0` to `.18`. These are test operands, not a machine ABI.
- **One enable operand.** It reaches 4+N instruction terminals: its Boolean check, the bit of temp.0, the bit of temp.3, the bit of each selector, and the final sum equality. It also appears C+1 times in host structs, as each selected tag and the preserved alias. The tags are Rust fields, not instructions.
- **The raw index** is an operand of exactly one instruction, the true arm of temp.0, and is then preserved unchanged.
- **Unconditional checks:** enable Boolean, count UInt128, temp.1 equal to 0, and every cell typed.
- **Guarded prefix:** temp.2 feeds the temp.3 select against immediate 1, then Assert.
- **Gated selectors.** Each eq net (temp.4+3r) ends at its Boolean check and its selector select. Lanes tap only the selectors (temp.5+3r). The sum terminator is the enable net, not immediate 1.
- **Disabled-zero.** The fixture's disabled cases expect UInt 0 and Bool false. MAPPING defines this as a construction value: never a fetched, default or padding row.
- **Hand counts.** I recounted 41 instructions and 19 temporaries for 3×2, matching `12+2NC+5N+C` and `4+N(C+3)`. They size a scene and say nothing about constraints, rows, cost or k.

## Approaches

**A. Gated array board.** The 002 board grammar plus an enable bus and select glyphs.

**B. Sibling comparison.** The G2 board ghosted beside A, marking where gates enter. It risks reading as version history.

**C. Fan-out sheet.** A DOM table listing every terminal of each multi-terminal net.

## Recommendation: gated board with a fan-out sheet

Proposed: A as the scene and C as the static and narrow-screen rendering, from one scene record. B is optional, captioned as a sibling constructor and never as a replacement.

**Enable bus.** A single trace on its own inner layer, keyed by its exact operand string, with vias only at real instruction terminals. Selected tags are silkscreen flags beside the output pads, not vias, because they are host data.

**Select glyph.** Every CondSelect is a trapezoid footprint with the bit pin on its short edge. Which net enters that pin tells the select kinds apart: enable for index_eff, guard and selectors; a selector net for lanes.

**Raw index.** One tap into the temp.0 true arm, then a straight run to its preserved terminal. The silkscreen reads: raw; no range check in this fragment.

**Obligations.**

- Unconditional terminators keep 001's proposed solid copper emission.
- The guarded Assert pad is a copper outline behind its select glyph.
- The sum terminator is a via on the enable bus.

Shape carries each difference, not colour alone.

**Disabled-zero.**

- The zero seed diamond from 002 stays.
- A DOM card quotes the MAPPING definition and the fixture's expected zeros.
- Disabled-zero is never drawn as a table row, a padding hatch or a darkening lane.
- No switch animates, and the board is identical for every fixture case.

**Off-board obligations.** Two dashed, unlit ink sockets:

- upstream of enable: enable provenance, dispatch owner, no source;
- downstream of the outputs: gate every consumer with enable, no consumer source.

**Result card.** For each of the four fixture cases, show three rows:

- Supplied: enable, count, raw index, as verbatim decimal text.
- Expected: copied from `expected.json`.
- Actual: UNRUN.

Selecting a case outlines only its input pads, for inspection. No net changes.

**Light.**

- Hovering any enable terminal lights every terminal of that net in the proposed cyan. This is inspection, not SDK activity.
- The instruction scrub is labelled source order.
- No measured light exists yet.

**Keep-out.**

- `%enabled_fetch.temp.` is hatched as in 002.
- A composition sketch shows enabled and unconditional scopes as separate, non-nested hatches. It is conceptual, because no composer exists.

**Branches.**

- A dispatch backplane, with several enabled cards on one selector bus, appears only as a dashed sketch captioned conceptual until dispatch source exists.
- Recursion imagery stays conceptual as well.

## Mapping additions

| Source construct | Board element | Anchor |
|---|---|---|
| `enable` operand | Inner-layer bus with vias | `emit_enabled_fetch_block` |
| `index_eff` select | Trapezoid, raw index on true arm | temp.0 |
| Guarded Assert | Copper outline behind trapezoid | temp.3 |
| Gated selector | Trapezoid between eq stub and lanes | temp.5+3r |
| `sum == enable` | Terminator via on enable bus | final `ConstrainEq` |
| `EnabledTypedOperand.enable` | Silkscreen flag, not a via | host struct |
| Disabled-zero | Seed diamond and DOM card | MAPPING, fixture |
| Caller obligations | Dashed unlit sockets | doc comments |

## Access and fallback

- **No WebGL or no scripts.** An SVG board and the fan-out sheet, both generated from the record.
- **Screen readers.** Operand strings are DOM text and are never abbreviated.
- **Keyboard.** E cycles enable terminals in source order, I jumps to the raw index, and C cycles fixture cases. T and N work as in 002.
- **Reduced motion.** Scrub travel and fades are removed, and changes cut.

## Brand

- The Moriarty wordmark stays flat and is never extruded or lit.
- Identifiers use tabular monospace.
- Ink grounds and copper (`#9a3412`, `#e8724a`) carry over.
- Cyan near `#5ad1d8` and copper emission remain proposals pending contrast measurement. This iteration adds no new hue.
- No franchise marks are used.

## Boundaries

- Ideas only: no website code, assets, packages, build or deploy.
- The browser never builds IR, evaluates, selects a row or toggles enable.
- No light may imply compilation, acceptance, satisfaction or proving.
- Nothing here says G3 is machine-ready or that Sherlock is complete.

## Open notes

- The enable bus density at larger N needs layout testing.
- Whether the off-board sockets show by default needs design review.
- An admitted run of the eleven tests and the owner SDK checks would fill Actual and give measured light its first rule.
