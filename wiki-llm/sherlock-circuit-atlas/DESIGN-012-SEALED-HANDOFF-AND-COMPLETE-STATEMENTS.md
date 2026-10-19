
# DESIGN012 — Sealed handoff and complete statements

- **Date:** 2026-10-19 UTC. The facts cut is 2026-10-19T15:22:05Z.
- **Kind:** Creative concept continuation of DESIGN011 for the future Sherlock circuit site. It adds no circuit-source iteration, website code, scene, asset, build or deployment. It runs alongside implementation and blocks nothing.
- **Author:** Opus 5.5 (`claude-opus-5-5`), the requested fresh Opus 5.5 HIGH creative author for this entry. This author made no tool calls, delegated nothing and read no file beyond the supplied packet. Root records the returned identity, dates and hashes separately.
- **Unchanged neighbours:** DESIGN011 and its separate status record are untouched and keep their own authorship.
- **Launch condition:** Unchanged. The site goes live once the full Sherlock goal is complete and accepted.

## Continuity

The board language stays exactly as before:

- ink ground and a flat, unlit Moriarty wordmark;
- copper accent only (`#9a3412` light, `#e8724a` dark), with no new hue;
- a 4px grid and tabular monospace labels that never reflow or abbreviate;
- Source, Idea and Status as distinct channels. Light means selection or decoration only. Only a dated, recorded result becomes a history card.

DESIGN011's return spine keeps its four layers and its K bench. This note adds three things:

- a **handoff deck** of six mounts, laid along the spine between the splice and the parent bay;
- a split of the spine's east end into two different objects, the **terminal stage** and the **anchor block**;
- an **inspection rack** south of the spine. The rack has four separately sourced rails, and the existing K bench becomes the west end of the K rail.

In the cumulative Three.js assembly, each mount drops in source order after DESIGN011's splice. Every glyph clicks through to its file and symbol.

## Recommendation: the sealed handoff deck

### Mount 1: Admission gauge (`LinkShape::offsets`)

- **Gauge.** Two layout blocks, `LiteralLayout` and `ReturnPublicationLayout`, slide through a machinist's go/no-go gauge before they reach the H1 encoder hopper.
- **Ruler.** The gauge face carries a ruler with seven ticks named for the `LinkOffsets` fields: `binding`, `context`, `program`, `pre`, `post`, `rule_tag`, `charge`. Tick positions come from final reviewed source and export, never from this note.
- **Two outcomes.** The offsets are computed with checked arithmetic and return a `Result`.
  - On success, both blocks pass into the hopper together.
  - On error, the block stops at the gauge and an error card drops into a tray beside it. The block is never trimmed, clipped or drawn partly inside.
- **Scope plate.** A small plate on the gauge reads *shape and offsets, before encoding*. The gauge is not drawn as a limit on Program allocation or on H1 recursion, and no allocation meter appears near it.

### Mount 2: The seal die and the glass cabinet (`link_seal::Seal`, `LinkVerification`)

- **Sealed housing.** A closed copper housing contains a die. The housing has no exterior handle, lever or port to the die, because the seal type is private to its module.
- **Three feeds.** The exact requests, the two children and the verification record converge on the die. The impression it leaves is drawn as a fine registration lattice.
- **Glass cabinet.** The verification record sits in a glass-fronted cabinet beside the housing.
  - Each readable face of the cabinet is an accessor. Visitors can read every face through the glass.
  - The cabinet has no hinge, latch or handle, because its fields are private.
- **No lamp.** The die has no lamp, gauge needle or tick of any colour. A seal selects an exact byte relation; it never acts as an approval light.
- **Export shelf.** Downstream of the die, the export shelf reuses DESIGN011's create-once tiles.
  - Each file tile carries a lock glyph (`create_new`) and a small sync glyph (`sync_all`).
  - The shelf's back wall, which stands for the directory, deliberately carries no sync glyph.
  - The `link.json` tile looks exactly like its siblings. It has no flag, crown or commit lamp, because its presence is not a durable commit marker or an authentication.
  - A failed export stays on the shelf exactly as recorded, with its open slot visible. It is never swept away or tidied.
- **Population rule.** Tiles populate only from actual export records. With none, the shelf shows empty locked slots.

### Mount 3: Manifest /2 board (`LinkManifest`, `LinkChild`, `GenesisAnchor`, link constants)

The manifest is one engraved board. Every declared field appears in declaration order, as text, with no ellipsis and no summary tile.

- **Header strip:**
  - `format`, which shows the `LINK_FORMAT` string;
  - `transcript`;
  - `field_encoding`, which shows the `LINK_FIELD_ENCODING` string;
  - `provenance`, which shows the `LINK_PROVENANCE` string verbatim;
  - `program_lanes`, `state_lanes`, `instance_lanes`;
  - the `offsets` ruler, mirrored from Mount 1.
- **Links rail.** The five `LINK_EQUALITIES` sentences are silkscreened verbatim along the board edge.
- **Child cards.** Two card slots sit in fixed order. Slot 0 holds A102 and slot 1 holds A80, and neither slot swaps or reorders.
  - Each card lists every `LinkChild` field as one row: name, the four file/sha256 pairs, `qualification`, `claim_format`, `binding`, `instance_lanes`, `proof_bytes`, `context_hash`, `zkir_hash`, `adapter_source_sha256`, `carrier_source_sha256`, `circuit_k`, `rule_tag`, `charge` and `open_obligations`.
  - Values come from an actual exported manifest only.
  - File digest rows carry a small tag, *identifies bytes*, taken from the disclosure text.
  - The `vk_sha256` row carries the `LINK_PIN_AUTHORITY` sentence as its tag.
- **Carrier row.** This is the one row drawn with three distinct shapes.
  - **Null:** a solid blanking plate screwed over the port, engraved *this claim has no carrier field*. This is slot 0 (A102).
  - **Present:** an open port showing the exact characters of the A80 claim's own field. This is slot 1, and it populates only from export.
  - **Empty string or invented digest:** the scene has no rendering path for either. It never shows a placeholder hash, a zero string or a greyed digest.
- **Open obligations.** Each child card carries a stack of engraved strips. The stack is exactly the sealed `ChildVerification` list for that child, verbatim and in order.
- **Footer.** Two more stacks close the board:
  - the manifest-level `open_obligations`;
  - `disclosures`, the five `LINK_MANIFEST_DISCLOSURES` sentences.
- **Genesis ring.** DESIGN011's ring locator now shows `height`, `child`, `state_offset`, `state_lanes` and the `root_authority` sentence.
  - Its centre stays hollow, because no Poseidon root is computed here.
  - It locates a state; it does not root one.
- **Validator loupe.** A magnifier represents `from_json` and `validate`. It is confined to the board inside a dashed edge labelled *reads this document only*.
  - No trace runs from the loupe to the export tiles, keys, proofs or source cards.
  - The loupe has no lamp.

### Mount 4: The keyed reader connector

- **Plug and socket.** The manifest board's east edge ends in a keyed plug. The parent bay's west wall carries a keyed socket.
- **Keyways from source.** Each keyway is generated only from a parsed source constant.
  - The plug's keyway comes from `LINK_FORMAT` (`/2`).
  - The socket's keyway comes from the format that the parent reader's own source accepts.
  - The version number sets the notch count, so `/1` and `/2` differ physically.
- **No relabelling.** A sticker or label on the socket never changes its key.
- **At the dated cut.** The reviewed parent reader reads `/1`. The `/2` plug therefore rests in a holster beside the socket, with a dated paper tag reading *reader reconciliation requested*. It is never drawn seated, half-seated or bridged with an adapter.
- **Finished release.** The plug seats only when final reviewed parent source reads `/2`. Seating is a static state change, with no spark, glow or click animation.

### Mount 5: The 28-lane parent route (inside DESIGN011's parent bay)

- **Header.** The parent's own public instance is one square-pinned header, deliberately distinct from the round pins of the child ribbons.
  - Pin count and labels are read from source at build time.
  - The first group is the 16 statement lanes, labelled from `FIELD_NAMES` in order.
  - A keyed divider follows.
  - The second group is the 12 lanes of the collapsed accumulator, each labelled from source.
- **Two kinds of record.** The divider is shaped, not coloured, so a visitor can read by touch alone that a statement and an accumulator are different things.
- **Stock routes.** Beside the header stand the existing stock CLI route headers, drawn at the widths that current CLI source declares.
  - At the cut, no trace joins the Sherlock 28 header to any CLI route.
  - A trace appears only when actual CLI source wires the 28 route.
- **No invented interior.** DESIGN011's rule stands: interior gates come only from actual parent bodies.

### Mount 6: Terminal stage and anchor block

DESIGN011 drew one *terminal anchor*. This note separates it into two objects, each with its own shape.

- **Terminal stage.** The closing wall of the parent chassis, on the board side, where the recursive chain is decided.
- **Anchor block.** A solid block on its own ledger plinth across a visible gap. It is labelled by its source file, `sherlock_anchor.compact`, and it is where the result is fixed.
- **Three-shape legend.** Pinned at the spine's west end, the legend teaches the three ends of the chain apart:
  - **ring**, the genesis locator, which locates a state;
  - **wall**, the terminal stage, which decides;
  - **block**, the anchor, which fixes the result.
- **At the dated cut:**
  - The terminal wall is an outline only, and its interior is empty.
  - The anchor plinth is empty.
  - Beside the plinth, the historical anchor proposal is pinned as a paper drawing, not a block. No current reviewed or compiled anchor matches it.

## The inspection rack: four rails, four sources

The rack is four parallel rails south of the spine. Visible air gaps labelled *no wire* separate them from each other and from the spine. Each rail opens with a source-origin plate at its west end and closes with its own claim plate and no-claim plate at its east end.

- **No shared clock.** The rack has no common time axis, bar chart or duration readout. Each check's wall time stays in that rail's own record and is never set beside another rail's.

### Rail N: Nat, kernel-checked

- **Frames.** Four frames, *Program*, *Init*, *middle* and *Done*, each list every represented field.
- **Child views.** Two child-view windows overlap on the shared Program and middle frames. This deliberately rhymes with DESIGN011's seam, but it is not cabled to it.
- **Supplied contexts.** Context tiles are plain cards stamped *supplied*. They are never hash pucks.
- **Done frame.** The frame shows the same `ConstantRef` and every represented control row cleared.
- **Rust lane order.** A dotted bracket with a reviewer tag stands for the manual Rust-order correspondence. Kernel-checked relations are drawn solid.
- **Claim plate:** *finite natural-number positions and values; kernel-checked*.
- **No-claim plate:** field elements, hashes, cryptography, parent, accumulator and native work.

### Rail K: definition compiled

- **West end.** DESIGN011's vellum windows stay as they are.
- **Drafting plate.** A new drafting plate titled *complete images definition* carries an inventory drawer for each compiled output member. Drawers are labelled by name only and stay closed, because payloads are omitted.
- **No registration marks.** The plate shows no registration marks, overlay or slider. Those appear only after an actual runtime comparison exists.
- **Rust consumer tag.** A small *ABSTAIN* tag hangs on the Rust-consumer drawer.
- **Claim plate:** *source compiles to this definition*.
- **No-claim plate:** runtime, comparison, context, oracle, refinement and native work.

### Rail Q: Quint, directed scenarios

- **Stations.** Four operation stations, each with its scenario tags. The tags are labelled *directed, same author*.
- **Snapshot frames.** The executable source and tool checks appear as photograph pairs, a *before* frame and an *after* frame with a visibly empty gap between them. They are never drawn as a continuous trace.
- **Model tray.** The model source directory is a three-slot tray marked *exact membership*. It is the only membership check on the rail.
- **Expectation record.** The expectation record has its own frame, stamped once at post-readback. That frame is absent from the eight phase snapshots.
- **Capture excerpt.** The capture excerpt is a cut-off connector footprint, an exact prefix that stops mid-declaration. The full helper sits beside it as a separate pinned card.
- **Claim plate:** *parametric finite model; sample scenarios*.
- **No-claim plate:** Rust, cross-layer, native, recursive, target and EARS evidence.

### Rail C: circuit and native

- **Mirror of the spine.** The rail mirrors the spine glyph for glyph. Each object carries a status pin coded by shape:
  - an outline for source only;
  - a filled outline for a reviewed source;
  - solid for a compiled object;
  - solid with a notch for an object with a recorded proof or verification.
- **Verdict slots.** Each slot stays empty, neither a tick nor a cross, until an actual record fills it.
- **Gap markers.** Gap markers sit exactly where Mounts 4–6 show holsters, outlines or empty plinths.

## Three readings (interactive explanations)

**"The whole footprint."** The source panel shows two connector footprints side by side.
- One declaration is drawn through its closing delimiter, and every pad is present.
- One excerpt stops at an opening brace. Its pads beyond the cut are blank board, never ghosted, guessed or filled from a digest.

> Every declaration is shown whole, from its first line to its closing brace. An excerpt that stops short is drawn as a cut-off footprint, and nothing beyond the cut is assumed.

**"Two photographs, not a film."** Scrub between the before and after frames on Rail Q. The gap between them stays empty.

> A before-and-after check compares two moments. It says nothing about what happened between them, so the gap between the frames stays blank.

**"Selected, not approved."** Shift the lattice overlay one byte along an export tile. It misregisters, and nothing lights either way.

> A seal is cut for exact bytes. It tells you which bytes belong together; whether they are right is a job for the proof.

**"Blank plate, open port."** Toggle between the two carrier rows.

> The literal step's claim has no carrier field, so its card carries a blanking plate. The return step has one, so its card shows that value exactly. No stand-in digest is ever drawn.

**"Key, not sticker."** Try to relabel the parent socket. The sticker changes, and the keyway does not.

> A reader accepts the format its own code was written for. Renaming the socket does not change its shape.

## Source clickthrough

Enter on any glyph opens the source panel. The panel shows the repository-relative file, the symbol, the line range and the file hash, and then the complete declaration through its closing delimiter. A partial excerpt is shown as a cut-off footprint marked *body not shown*.

| Glyph | Symbol | File (repository-relative) |
|---|---|---|
| Gauge, ruler | `LinkShape::offsets`, `LinkOffsets` | `crates/mori-backend/src/sherlock_link.rs` |
| Die, cabinet | `link_seal::Seal`, `LinkVerification` | same |
| Manifest board | `LinkManifest`, `LinkChild`, `GenesisAnchor`, `LINK_*`, `RETURN_OPEN_OBLIGATIONS` | same |
| Socket | parent reader's accepted format | Fenrir parent `sherlock_link.rs` |
| 28 header | `FIELD_NAMES`, collapsed accumulator | Fenrir parent source |
| Anchor block | anchor contract | `sherlock_anchor.compact` |
| Rail N | complete literal statement module | `clean/fork/docs/results/sherlock-clean-complete-literal-statement-20261019/` |
| Rail K | complete-images definition | `docs/results/sherlock-k-literal-complete-images-compile-20261019/` |
| Rail Q | parametric literal-images model | `docs/results/sherlock-quint-parametric-literal-images-20261019/` |

The scene binds to final reviewed source, never to this table or to this note's prose.

## Alternatives

- **B. Signal-lamp panel.** Each seal, digest and validator gets a green lamp. *Rejected:* it turns byte selection into approval.
- **C. Exploded isometric drawing.** The deck is drawn as one static engineering sheet with leader lines. *Kept* as the overview thumbnail and the static fallback plate.
- **D. Merged rail chart.** All four rails share one timeline. *Rejected:* it invites a clock or performance comparison that their separate checks do not support.

**Choose the deck and rack**, with C as the overview and the child-card field table as the deepest zoom.

## History cards (dated, separate gallery)

Cards carry a date, the record kind and the outcome in the record's own words. They show no durations.

- **2026-10-19:** Complete literal statement and child views kernel-checked over finite natural numbers.
- **2026-10-19:** K complete-images definition compiled (definition generation only).
- **2026-10-19:** Quint parametric model: four operations and eight directed tests passed.
- **2026-10-19:** Link manifest /2 source frozen with seven owner tests (source only).
- **By 2026-10-19:** Parent link source received scoped source approval (source only, reader /1).

A record never adds glow to a board.

## Motion, keyboard, reduced motion, mobile, fallback

- **Motion.** A 400 ms camera dolly runs between mounts. The assembly control drops the mounts in order. Plug seating, tile locking and gauge passage are discrete cuts. Nothing pulses, flows, sparks or implies proving.
- **Keyboard.** Tab follows the deck, then the rack:
  1. gauge, then its ruler ticks, with arrow keys moving between ticks;
  2. die housing, then cabinet faces;
  3. export shelf tiles;
  4. manifest: header, links, child card 0 rows, child card 1 rows, genesis ring, obligations, disclosures, loupe;
  5. connector, holster tag and socket;
  6. the 28 header (arrow keys move between pins; Home and End jump between groups), then the stock routes;
  7. terminal wall, then anchor plinth, then the legend;
  8. rails N, K, Q and C, each as its own group.

  Enter opens the source panel. Escape returns to the glyph.
- **Reduced motion.** Cuts replace the dolly, the deck is drawn fully assembled, and every reading steps only on a keypress.
- **Mobile.** The deck stacks vertically. Child cards become paged full-field tables, and the 28 header becomes a two-page table split at the divider. Tap targets are at least 44px, with explicit zoom buttons.
- **Static fallback.** Plate C plus one SVG per mount and rail, each with a DOM table mirroring every glyph and source link. No WebGL is needed. Shape, notch count, plate and position always carry meaning alongside tone.

## Next source events

1. The reviewed and compiled manifest /2 module mounts the gauge, die and board populated from final source.
2. Parent reader source that reads `/2` seats the plug.
3. Actual CLI source wiring the 28 route draws its trace.
4. Reviewed terminal and anchor source fills the wall and bolts the block to its plinth.
5. Each actual check, proof or verification record adds its own dated card.

## Visitor copy (finished release)

```text
When two proved steps are joined, Sherlock checks the join once and seals exactly what it checked: both requests, both children and the record of the check. The seal is cut for those bytes and no others. Anyone can read the record; nobody can change it. A seal is not a verdict — it says which bytes belong together, and the proof says whether they are right.
```

```text
Before a single lane is encoded, both layouts pass a gauge. Every offset is computed with arithmetic that refuses to overflow, and a layout that does not fit is turned away whole, never trimmed to fit.
```

```text
The manifest gives each child its own card: what it is, how its claim is written, its binding, the size of its proof and the obligations it still carries. Where a field does not exist, the card says so with a blanking plate — the literal step's claim has no carrier field, so none is shown and no stand-in takes its place. The manifest also states, in its own words, what it does not check.
```

```text
A manifest meets its reader through a keyed connector. The version is cut into the key, so the parent accepts only the format its own reader was written for. Changing a label never changes the key.
```

```text
The parent's public statement has twenty-eight lanes: sixteen that state the result and twelve that carry the recursive accumulator. Three shapes mark the chain's ends. A ring locates where it begins, a wall marks the terminal step that decides, and a separate block — the anchor contract — fixes the outcome.
```

```text
Beside the boards stand four inspection rails, each with its own source and its own check. One counts the machine in natural numbers and has a proof kernel confirm that a complete statement reads back field by field; its numbers are positions and values, not hashes. One describes the complete states in K. One walks a Quint model through directed scenarios. One is the circuit and its proofs. They are not wired together and their clocks are not compared: each answers its own question.
```

```text
Every declaration in the source panel is shown whole, from its first line to its closing brace. If an excerpt stops short, it is drawn as a cut-off footprint, and nothing beyond the cut is guessed.
```

## INTERNAL: dated source, status and provenance register (cut 2026-10-19T15:22:05Z — not visitor copy)

**Supplied scope.**
- **Seen:** DESIGN011 and its status record in full, Root's current facts JSON, the B22, B28 and K result summaries, and complete named declarations from the manifest /2 candidate.
- **Declarations seen:** `LINK_FORMAT`, `LINK_FIELD_ENCODING`, `LINK_PIN_AUTHORITY`, `LINK_PROVENANCE`, `GENESIS_ROOT_AUTHORITY`, `LINK_MANIFEST_DISCLOSURES`, `LINK_EQUALITIES`, `RETURN_OPEN_OBLIGATIONS`, `LINK_OPEN_OBLIGATIONS`, `LinkOffsets`, `GenesisAnchor`, `LinkChild` and `LinkManifest`.
- **Not seen:** the bodies of `from_json`, `validate`, `LinkShape::offsets`, `link_seal::Seal`, `LinkVerification`, export, the parent reader, the terminal or the anchor.
- These are named declarations, not implementation or audit coverage. No unseen body is certified.
- **S1/S2 wording.** The S1 and S2 descriptions, the private seal, the checked offsets and admission before H1 encoding, are Root summaries, not code read here.

**Declaration source and current freeze.**
- The declarations came from candidate-06 (56,680 B, `5ce07beb…298c23`).
- The current freeze is candidate-07: A80 v2 composed source plus seven owner tests. Its module is 56,754 B (`f13358e1…1e935e`) and its freeze record 3,929 B (`bd5a4458…ef`).
- This author did not compare the two. The scene must bind to final reviewed source.
- Candidate-07 status: COMPLETE_COMPOSED_A80_V2_SOURCE_PLUS7OWNER_TESTS_UNREVIEWED_UNCOMPILED. The tests are unreviewed, uncompiled and unrun.
- The earlier link module received CHANGES_REQUIRED in review 15050. Its status record holds that history.

**S2 scope.** Checked offsets admit both layouts before H1 encoding. They are not a claim to bound Program allocation or H1 recursion, and Mount 1 shows no allocation meter for that reason.

**Validation and export qualifications.**
- `from_json` and `validate` are intrinsic only. They read no sibling file and verify no digest, key, claim, proof or source.
- Export is per-file `create_new` plus `sync_all`. The directory is not fsynced and failed exports are retained.
- `link.json` presence is not a durable commit or an authentication.
- Binding lanes are SHA-256 claim commitments that are not recomputed in circuit.
- Genesis is an Init host subset only, and no Poseidon root is computed.
- All link equalities are host tooling.

**Carrier.**
- For A102, `children[0].carrier_source_sha256` is null because `ChildClaim` lacks a carrier field.
- For A80, `children[1]` carries the A80 claim's own field.

**Parent.**
- Source: reviewed Fenrir parent `sherlock_link.rs`, 37,710 B, `b0443be3…6a`.
- Review: MF01–MF06 scoped SOURCE APPROVE plus typed API ABSTAIN. It carries no execution eligibility.
- Its reader is `/1`. The `/2` reader reconciliation is requested (handoff 1,823 B, `f9079ba3…40`) and is not integrated.
- Its instance is exactly 16 `FIELD_NAMES` followed by a 12-field collapsed accumulator, 28 lanes in all.
- The current CLI stock path is 17/29, with no wired Sherlock 28 route.

**Terminal and anchor.**
- A historical 2,618 B anchor proposal exists.
- No current matching reviewed or compiled anchor, Template capture or target result exists.
- The terminal decider and anchoring are listed as unobserved consumer work.

**Rail N, Clean B28 (projectfenrir).**
- Source: 11,106 B, `6e7bf504…250`.
- Kernel check: Lean 4.33.1, one module check, exit 0, wall 3.379900310013909 s.
- It covers six theorem facts. Postcheck matched 174 unique pins, and the five outputs total 526,120 B.
- Root adoption record: 7,718 B, `0244bc9c…95`.
- Scope: finite Nat values, supplied contexts and a manual Rust-order correspondence. It makes no field, hash, cryptographic, parent or target claim.

**Rail K.**
- Result: PASS_FRESH_DEFINITION_GENERATION_ONLY, wall 14.951 s.
- Definition: 1,068,521 B, `df05e8d6…ac9`, with 15 outputs.
- Outcome record: 4,615 B, `57258dbc…07`.
- Scope: no runtime, full comparison, context, oracle or refinement work. The overall and Rust-consumer dispositions remain ABSTAIN.

**Rail Q, Quint B22.**
- Result: PASS_FOUR_OPERATIONS_EIGHT_NEW_TESTS.
- Operation times: 3.972359826992033, 5.409273496974492, 4.854895883006975 and 4.012205305014504 s.
- Raw outcome: 4,986 B, `797c3541…33`. Qualifications: 3,137 B, `30a9f579…51`.
- **Q1:** The raw `sourceAndExpectationsUnchanged=true` field is preserved as a literal. The EXPECTATIONS JSON was matched only at post-readback, not in the phase snapshots.
- **Q2:** The capture excerpt is the 6,800 B prefix of a 13,595 B helper. It is not a complete closure.
- **Q3:** The 4,678 checks are before/after snapshots of declared files. Only the three-file model directory had exact membership checking.
- The D1–D6 strengthenings and coverage gaps are retained. The tests are same-author, source-exposed and finite.

**Timings.** The Lean, K and Quint wall times above are separate records of separate operations. No A, K, B, helper, fuel or circuit-degree equality and no performance comparison follows from them. The site shows none of these times side by side.

**Delivery.** Root reports Moriarty commit `34a8593b` and Fenrir commit `9250d9ed` pushing at the cut. PR numbers are pending, and this author infers no remote success or merge.

**DESIGN011 pins.** DESIGN011 is unchanged: 16,134 B, `b7253cec…1621`. Its status record is 4,414 B, `5114b816…6ab4db`.

**Not claimed.** This note claims none of the following:
- a proof, verifier verdict, native child, parent or terminal result;
- recursive acceptance, an anchor compile, a ledger or stagenet result, or an endpoint;
- resource or performance figures;
- that the `/2` reader is integrated;
- that the 28 route is wired;
- host/K equivalence or EARS closure;
- that any source is machine-ready, that Sherlock is complete, or that this note is a new circuit-source iteration.

Visitor copy describes the finished release. It must be rechecked against final accepted source before launch, and any mechanism that changes rewrites its copy. The full goal remains a Sherlock stagenet recursive proof followed by Fenrir-derived rollup k/performance.

