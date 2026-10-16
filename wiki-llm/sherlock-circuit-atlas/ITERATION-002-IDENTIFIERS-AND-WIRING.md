# Iteration 002: identifiers and wiring

- Date: 2026-10-16 UTC
- Author: Claude Opus 5.5 (`claude-opus-5-5`), high effort, as the user-assigned visual concept author. The attribution belongs to this planning note, not to proposed website copy. A security source review of the same bytes was produced separately; this note is not part of it and makes no security judgment.
- Source authorship: Astra high wrote the r2 source, tests and mapping. The two rejection-test requirements belong to the r1 Opus security review, G-D1(b,c). Root wrote `expected.json` independently.
- Iterations 000 and 001 stay frozen. This note extends 001 by reference.

## Source pins

Paths are relative to this folder, and `../../` is the repository root. The files are uncommitted additions in the Astra worktree on source base head `1fb794d1598e19fe34b9735f90996fc316b743fa`. No branch or commit is recorded, nothing is published and nothing is on `main`, so paths are shown as code rather than links. The concept checkout base `f7ff06ed05bcef5fc3540bdde70ddbf6402f5547` is not a source base.

| Path | SHA256 |
|---|---|
| `../../crates/mori-backend/src/sherlock_machine/fetch.rs` | `6e71e0dccf687bec051556eeefdc435cef1c61535323d62a3994dc7220cadc87` |
| `../../crates/mori-backend/src/sherlock_machine/identifier_rejection_tests.rs` | `3c5be80e5104827b287cc4f310f88f4b2ae4a084a0518d517d421bb4e3374851` |
| `../../crates/mori-backend/src/sherlock_machine/mod.rs` | `87fcffae76ebb1f4e19d7afe678f97ef5fd59a57fd65ca9e1781251181b16b5f` |
| `../../crates/mori-backend/src/sherlock_machine/normal_tests.rs` | `651a47138ab6a9bd37666508ca48de7c1ea9846840f1b56fee2b3a30e5400c8c` |
| `../../models/sherlock/machine-circuit/MAPPING.md` | `e0864af4a8cff8d3b2fb992cf22fc4069b5c67ed84d87727231b17341010c6ab` |
| `../../models/sherlock/machine-circuit/expected.json` | `1c0319aa6c447599eadbfb2020a60d66effc01001282470429d610e3d5867ee7` |

The author report bundle is held by Root and is not in the repository:

| File | SHA256 |
|---|---|
| `WORK_REPORT.md` | `035d23c95f0ea9f79c4b0f161e2753f15ab3113945faeca12ab327713c229995` |
| `SOURCE-MANIFEST.json` | `a7e1f1efe9b24eed2a13a73f2d0765c3d17e31d0af5596d86299128993da7fe0` |
| `SOURCE.patch` | `9e3e1d9a19c57ab24be356a2ac4d180331151d84a8dfec10dac2fbf7960923b3` |
| `STATIC-CHECKS.json` | `d440bc46d8f2bb6131a5dfbafc14a6e90d40e107b22dd1db7dce32de6fa0c168` |

The SDK pin is Midnight `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`.

**Earlier snapshots.** Iteration 001 describes the r1 bytes, and only `expected.json` is shared with r2:

| Path | r1 SHA256 (prefix) |
|---|---|
| `mod.rs` | `56e58bc0…` |
| `fetch.rs` | `df1d55b4…` |
| `normal_tests.rs` | `e4262f82…` |
| `MAPPING.md` | `40ece625…` |

The unprefixed names in 001, such as `row_fetch.input.4`, belong to that snapshot and are not what r2 emits. Iteration 000 describes a different module, `sherlock_step`, and says nothing about this gadget.

## Status at this iteration

- **Unregistered.** `crates/mori-backend/src/lib.rs` declares no `sherlock_machine`.
- **UNRUN:**
  - compilation;
  - all six tests (four normal, two rejection), which exist as source only;
  - serialization;
  - SDK load and check;
  - evaluation;
  - native checks;
  - proving.
- **Actual output:** UNRUN. Native authority is 0.
- **Executed:** only the text and metadata checks in `STATIC-CHECKS.json`.
- **Unchanged:** old failure, cap and counter records.

## What the source gives a board

These observations come from reading r2 and `SOURCE.patch`; no tool produced them.

- **Names carry `%`.** The wrapper emits `%row_fetch.input.0` through `%row_fetch.input.7` and `%row_fetch.temp.0` through `%row_fetch.temp.13`. The pinned SDK writes names verbatim, and its operand reader requires `%`. The backend's own builder already uses `%mori.{n}`.
- **Two refusals.** Before emitting anything, `emit_fetch_block` refuses:
  - a caller Variable without `%`, with `UnprefixedVariable`;
  - a caller Variable inside `%{scope}.temp.`, with `TemporaryNameCollision`.
- **Same relation.** These are unchanged from r1: the guards, decoder, lanes, output order, preserved aliases, `pack_witness` and the fixture.
- **Named nets.** The first normal test lists all 22 output identifiers in emission order:
  - the 8 inputs;
  - temp.0 and temp.1, the comparators;
  - temp.2 to temp.7, alternating selector and running sum;
  - temp.8 to temp.10, the amount lane;
  - temp.11 to temp.13, the enabled lane.
- **Specified edges.** The second normal test names every edge:
  - each `TestEq` immediate;
  - both `Add` operands;
  - the sum's equality to 1;
  - the capacity comparator's equality to 0;
  - each `CondSelect` `bit`, `a` and `b`, seed first.
- **Rejection tests.** `identifier_rejection_tests.rs` mutates count, then index, then one cell. It does this once with `caller.unprefixed` and once with `%fetch.temp.0`.
- **Open gaps.** `MAPPING.md` lists six G.4 gaps. The first is that no enable exists, so the capacity, prefix and sum-to-one obligations are unconditional.
- **Hand count.** The 3×2 wrapper's instruction vector has 57 entries: 24 in the public frame, 32 in the fragment and 1 `Output`. I recounted this from r2 for this note. It sizes a scene and says nothing about circuit constraints, rows or cost.

## Approaches

**A. Net-keyed array board.** This is the 001 board, with every node and net keyed by its exact operand string. Nothing moves, and identity becomes exact.

**B. Specification overlay.** A toggle layer on A marks each edge that a source test names, and the reserved namespace. It shows what the tests would check without showing a result.

**C. Netlist sheet.** A flat DOM table with one row per net: identifier, defining instruction, consumers, and input or output position. It is the most exact and the least luminous.

## Recommendation: net-keyed board with specification overlay

Proposed: A as the scene, B as a layer toggle, and C as the static and narrow-screen rendering, all generated from one scene record.

**Net identity.**

- A net's key is its operand string, `%` included. For example, `%row_fetch.input.4` is the pad for row 1's amount, and `%row_fetch.temp.9` is the amount lane after row 1.
- Immediates have no name, so they are keyed by value and use site.
- A preserved output is the same key at two terminals, so hovering either end lights both.
- Identifiers are silkscreened and shown literally. `%` is part of the name, not formatting, and a label is never shortened. Where a label does not fit, the mid distance tier hides it rather than abbreviating it.
- The scene record would be extracted from the emitted instruction list in a later, reviewed step. Its stamped source hash must match the page pin, or the board shows as stale.

**Keep-out.**

- The namespace `%row_fetch.temp.` is drawn as a hatched PCB keep-out around the fragment's internal parts, with the input pads outside it. In the embedded shape, the hatch reads `%embedded_fetch.temp.`.
- A static composition sketch shows why scopes must be distinct and must not nest: `%a.temp.` and `%a.temp.1.temp.` appear as overlapping hatches marked forbidden.
- That sketch is conceptual, because no composer source exists.

**Keyed sockets.** In the fragment-only shape, caller pins carry a small `%` key notch on the socket edge. This follows the source rule that caller Variables already carry the prefix. Immediates enter through a plain stamped fiducial.

**Specified-edge ticks.**

- With the overlay on, each edge named by a normal test gets a small silkscreen tick. Hover shows the test name and the field:
  - `TestEq b`, stamped with r;
  - `Add a`, from the previous sum;
  - `Add b`, from the selector;
  - the 1 and 0 terminators;
  - each `CondSelect` `bit`, `a` and `b`, including the edge from the seed diamond.
- A tick means only that a source test names this edge. Its status is UNRUN.
- Ticks are ink. They are never lit and never green.

**Refusal card.**

- The rejection tests appear as a DOM card beside the socket, not as an event on the board.
- Each row gives the mutated position (count, index or cell), the supplied name and the expected error variant.
- The card is captioned as an Opus-prescribed source expectation, UNRUN.
- Nothing flashes or animates a refusal.

**Hardwired obligations.**

- Copper emission from 001 still marks the capacity terminator 0, the Assert pad and the sum terminator 1.
- Because r2 has no enable, these are drawn hardwired: no switch glyph, no gate, and nothing that hints at an inactive branch.
- A future enabled fetch would get its own board, drawn from its own source.

**Light channels.**

- The channels from 000 and 001 are unchanged: ambient edge breath, cyan selection, example figures and reserved measured light.
- Selecting a net lights its terminals and opens the exact Rust lines from the pinned bytes.
- The instruction-order scrub walks emission order. It is labelled as source order, not execution.

**Result card.** As in 001, positions 0 through 9 each get three rows: Supplied, Expected and Actual. Expected copies Root's `expected.json`, and Actual reads UNRUN. Positions are listed explicitly.

## Mapping additions

| Source construct | Board element | Anchor |
|---|---|---|
| Operand `Identifier` with `%` | Net key and literal silkscreen | `Builder::fresh`, `emit_public_fetch` |
| `%{scope}.temp.` | Hatched keep-out | `emit_fetch_block` namespace |
| `UnprefixedVariable` | Key notch, refusal-card row | Caller check |
| `TemporaryNameCollision` | Keep-out edge, refusal-card row | Caller check |
| Normal-test assertions | Ink ticks on named edges | `normal_tests.rs` |
| Rejection tests | Refusal card, UNRUN | `identifier_rejection_tests.rs` |
| Unconditional obligations | Hardwired copper terminators | `emit_fetch_block` |

## Access and fallback

- **No WebGL or no scripts.** The page shows an SVG array and the netlist sheet, both generated from the same record.
- **Screen readers.** Identifiers are DOM text, so assistive technology reads `%row_fetch.temp.9` literally.
- **Keyboard.** Keys follow 001, with two additions: T toggles the overlay, and N cycles nets in emission order.
- **Not colour alone.** Ticks, keep-out and terminators differ by shape and pattern.
- **Reduced motion.** Parallax, scrub travel and fades are removed, and overlay changes cut.

## Brand

- The Moriarty wordmark stays flat on the page and is never extruded or lit.
- Identifiers and figures use tabular monospace.
- The ink grounds and the copper accent (`#9a3412`, `#e8724a`) carry over.
- Cyan near `#5ad1d8` and copper emission remain proposals, pending contrast measurement and design review.
- No franchise logos, typefaces or set designs are used.

## Boundaries

- This note holds ideas only: no website code, assets, packages, build or deploy.
- The page would never do any of these in the browser: build IR, evaluate an instruction, select a row, check an identifier or run a rejection.
- No light, tick or animation may imply compilation, SDK acceptance, constraint satisfaction or proving.
- Actual stays UNRUN until Root forwards a recorded, admitted run.

## Open notes

- Full identifiers are long at mid range. Hiding labels by tier needs testing at larger N×C.
- Whether ticks are on by default or only in the overlay needs design review.
- Because of the enable gap, any multi-branch machine imagery stays conceptual.
- A recorded run of the six tests would fill Actual for the first time and give measured light its first rule.
