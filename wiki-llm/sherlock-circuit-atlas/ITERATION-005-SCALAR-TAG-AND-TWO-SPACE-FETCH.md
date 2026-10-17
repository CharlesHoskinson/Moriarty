# Iteration 005: scalar tag and two-space fetch

This note responds to the handwritten source freeze of the B-core positive leaf `emit_b_scalar_tag_add`. It is a real circuit iteration and follows [iteration 004](ITERATION-004-FULL-IMAGE-LITERAL-AND-BORROW.md) on the assembly rail. It supersedes 004 as the latest iteration by reference only, and every earlier note stays as written. [Design 005](DESIGN-005-ARENA-SLICES-AND-BYTE-LANES.md) and [Design 006](DESIGN-006-SOURCE-BOARDS-AND-HOST-REPLAY.md) remain companions. They are not iterations.

Passages marked *Visitor copy* use the single internal assumption that Design 006 established. They are written for the public launch, after Sherlock v1 has been accepted and every rendered element resolves to a source, circuit, execution or activity binding. The current state of the source appears only in the author-facing grounding section near the end. Author identity and requested effort are recorded in [`AUTHORSHIP.json`](AUTHORSHIP.json). This note claims no returned provider model or effort. No tools were used, and every SHA256 is as supplied, not recomputed.

## What this board does

In the interpreter, adding two scalars is a helper that runs in phases. `b.helper.Scalar.tag` is the step that opens the `Add` helper on site `RetRight` in phase `First`. It looks up the value that `operandA` refers to, checks that the value is an unsigned 128-bit integer, and copies its sixteen bytes into the helper register `regA`. It then sets the phase to `Second` and charges one unit of work. It adds nothing yet. Its job is to prepare the first operand so that a later phase can use it.

The subject of the 005 board is therefore a register write fed by a typed read. The read is interesting because a `ValueRef` can point into either of two tables. The program's read-only `Const` table holds literals, and the machine image's `Data` table holds values the run has built. The board reads exactly one of them. It calls the enabled fetch from iteration 003 once for each space and joins the two results with a router that the space bit drives.

> *Visitor copy:* Before Sherlock adds two numbers, it fetches the first one. A value reference names a table, either the program's constants or the machine's data, and a row in it. This board reads from whichever table the reference names, confirms that the row holds an unsigned 128-bit integer, and copies its sixteen bytes into the helper's first register. Then it moves the helper to its second phase and charges one unit of work, byte by byte. Every other field of the helper passes through unchanged.

## Source reading

### Branch identity and the two-space read

Before the function emits any instruction, it runs host construction checks. The scope must be nonempty. Each table must supply exactly `capacity` rows. Pre and post `rest` must have equal lengths. Every variable operand must carry the `%` prefix and must stay clear of three reserved namespaces: `%{scope}.local.temp.`, `%{scope}.const_fetch.temp.` and `%{scope}.data_fetch.temp.`. These checks emit no instructions, so the board draws them as silkscreen keep-out borders around the input pads rather than as components. The scan includes `pre_helper.reg_a`. No instruction ever reads that field, so its pads have a keep-out border and no traces.

K01 constrains `enable` to a bit. K02 forms the effective space `s = CondSelect(enable, operand_a.space, 0)` and constrains it to a bit. K03 forms `en_const = CondSelect(s, 0, enable)`. The data enable is `s` itself, an alias rather than a new instruction, so the board gives it a net junction and no part. Under `enable = 1` exactly one space is enabled, and under `enable = 0` both are 0.

K04 and K05 route through the local helper `space_read`. For a space with nonzero capacity, it builds a `TableShape` with a `tag` column and columns `n.0` to `n.15`, all `Type::UInt`. It then calls the existing `emit_enabled_fetch_block` with that space's enable, its `active_rows`, the raw `operand_a.index` and the projected rows, under scope `{scope}.const_fetch` or `{scope}.data_fetch`. The reused fragment forms its own effective index, asserts the active-prefix bound under its enable, and selects one row. For a space with zero capacity, `space_read` emits `ConstrainEq(enable, 0)` for that space's enable and returns sixteen immediate zeros. It builds no row and calls no fetch. K06 applies `gated_eq` to the fetched `tag` against `CONST_TAG_U128` or `DATA_TAG_U128`, both ordinal 0. K07 forms each of the sixteen payload lanes as `read_n[i] = CondSelect(s, n_data[i], n_const[i])` and constrains it to 8 bits.

The two fetch calls share a constructor but not their parameters. The Const and Data tables are supplied independently, so their capacities may differ, and each call receives its own rows, active length, enable and scope. The reused fragment emits one selector rank per row of its own shape, so the size and wiring of each instance follow its own table.

The disabled value of `read_n` is construction data. When `enable = 0`, both fetch enables are 0, the reused fragment's selectors are all 0, and every lane resolves to the zero seed. That zero is not a fetched row, a default or padding, which matches the rule that iteration 003 drew.

Disabling does not leave the board unconstrained. Each nonzero-capacity call to `emit_enabled_fetch_block` still constrains, unconditionally, every projected cell to 128 bits and `active_rows` to at most `capacity`. Nor does disabling impose a canonical zero post-state, because the leaf emits no zeroing rule.

### Writes, preservation and the counters

K08 gates five identity checks on `enable`. The pre-helper must have `present = 1`, `kind = 0` (Scalar), `site = 11` (RetRight), `mode = 0` (Add) and `phase = 0` (First). K09 writes the two fields the row changes: post `phase = 1` (Second) and each post `reg_a[i] = read_n[i]`. K10 holds `present`, `kind`, `site`, `mode`, `operand_a.space`, `operand_a.index` and every paired `rest` lane equal from pre to post, all gated on `enable`.

The post helper and post counters are independent supplied witnesses, and these relations constrain them. The function never computes a post image. The returned `preserved` field is `input.clone()`, an alias of the operands, not a post-state and not a proof.

The counters are four local UInt128 carriers, each made of sixteen little-endian byte lanes. K11 passes every lane of the four pre and four post carriers through `CondSelect(enable, lane, 0)` and constrains the result to 8 bits. K12 decrements `remaining_work`. The borrow starts at `enable`, and each byte satisfies `pre + 256·next == post + borrow`, with 9-bit intermediates and `next` taken from `TestEq(pre, 0)`. K13 increments `consumed_work`, `helper_work` and `helper_visits`. The carry starts at `enable`, and each byte satisfies `post + 256·next == pre + carry`, with `next` taken from `TestEq(pre, 255)`. Every chain ends with `ConstrainEq` of the outgoing borrow or carry to 0, so nothing wraps at 2^128. These widths are logical byte widths. They are not a serialized B counter width, a physical gate count or a cost.

### What the leaf leaves to its caller

The source names its seam in comments, and the column mapping lists it as OBL-01 to OBL-12. The caller must derive `enable` from constrained dispatch, with exactly one enable per step. It must supply a complete `rest` partition in identical order, the true Const and Data projections with their true active lengths, and equality for every cell outside the leaf. It must also connect the local carriers to serialized encodings, and it owns StepInv, statement binding and the event envelope. The board draws these seam points as dashed, unlit sockets on its perimeter, as iteration 003 did, because no composer source exists to wire to.

## Place in the assembly history

The 005 entry adds a new lineage, the B-core leaf, beside the boards that already exist. It has two kinds of relationship to earlier work, and the site must keep them visually apart.

- **Source call.** `space_read` calls `emit_enabled_fetch_block` once for each nonzero-capacity space. The candidate pins `enabled_fetch.rs` at `6c45ab9190ad829db29b039d91050bfd8b82d1ccb66b3dd990538c18415dc8c1`, which is the iteration 003 pin. This is an *embeds* edge. Both calls use the same constructor revision, but each receives its own capacity, rows, active length, enable and scope, so each socket holds a parameterized instance of the same footprint family, bound to the accepted netlist's actual capacity for that space. The two instances need not be identical.
- **Copied idiom.** The local `Emitter`, `gated_eq` and the bytewise borrow chain repeat the design of `literal_step.rs`. They are not calls into it. The caller contract says the originals are private and A-specific and directs that `literal_schema`, `LiteralImage` and `emit_literal_step` must not be reused. This is an annotation of analogous design. It is drawn as a dotted silkscreen callout and never as a trace.

Neither relationship joins 004 and 005 into one working transition. The A102 board remains its own historical freeze. The relations below are computed by pin.

| Field | 005 entry |
|---|---|
| Revision | Candidate manifest `90db5144e24d05ab05712260bca4ec85d4672e995b2c1809f9c01b34359129e5`; source `b_scalar_tag.rs` `af82e70192b34f6bd5a95ec1008fe412a5294057b2becd149cab24aef60285cb` (13854 B), target path `crates/mori-backend/src/sherlock_machine/b_scalar_tag.rs` |
| Relation | New lineage (B core); embeds G3 through two calls; idiom annotation to 004 |
| Added | `b_scalar_tag.rs` and its column mapping `05dfcbe2744dc407788bd41d42935db1ecbc11c08f24931a962e64d98170a56a` |
| Retained (pin-identical) | `enabled_fetch.rs` `6c45ab91…` (003 pin); `fetch.rs` `6e71e0dc…` (002/003 pin); `literal_step.rs` `b5d2ca73…` (004 pin) |
| Replaced by this candidate | None |
| Shared `mod.rs` | Builder input `5f193ee1…` (8703 B) differs from the last log pin `e2af2fe0…` (003, 8328 B). The 004 manifest pinned no `mod.rs`. The supplied bytes declare `mod literal_step` and its re-exports. The difference predates this candidate, and the rail attributes it only from actual pins. |
| Reason (quoted) | "set h.regA := u128(a); set h.phase := Second" |
| Reader path | A typed read from one of two spaces, a register write, and work charged byte by byte |

The board delta follows from those pins. Before the 005 stop, the bench holds the 000 module, the G2 row fetch, the G3 sibling and the A102 board. After the stop, the bench holds the same boards plus the B leaf, with one capacity-parameterized instance of the G3 footprint family inside it for each nonzero-capacity space.

The G2 `fetch.rs`, the G3 `enabled_fetch.rs` and the A102 `literal_step.rs` are pin-identical to their earlier pins. Pin identity of an implementation file is necessary for a board to stay still, but it is not sufficient, because each board also depends on symbols in the shared `mod.rs`, and `mod.rs` has changed pins. Under L1 below, stillness at each stop is checked against each board's actual transitive parent-symbol dependencies and its instantiated graph, by comparing those symbol bodies between the two pinned `mod.rs` revisions at build time, and never by how similar the boards look. The new leaf, for example, uses `immediate`, `TableShape`, `Column` and `ConstructionError` directly and reaches `constrain` only through the enabled fetch.

> *Visitor copy:* This step starts a new branch of the machine: the helpers that carry out arithmetic. The new board reuses the enabled fetch twice, once for each table a value can live in, and you can see the same fetch design seated in both sockets, each sized to its own table. Its byte-by-byte counters follow the same pattern as the literal step's fuel borrow, written fresh for this board.

## Visual approaches

### I. Switchyard

The B leaf is laid out as a rail switchyard. The Const bank sits on the left and the Data bank on the right. Each bank feeds a G3 socket. The two sockets converge on a row of sixteen lane routers (K07), and the router row runs up into the post `regA` pads. The space router sits at the head of the yard, where `s` and `en_const` split. Counter strips run along the lower edge, and the helper register is a cartridge across the top.

This approach shows the two-space read most clearly, which is the board's new idea. The cartridge is compact, so the preservation straps need a zoom level of their own.

### II. Register cartridge

The `reg.helper` cartridge is the hero. It is a long connector with named pins (`present`, `kind`, `site`, `mode`, `phase`, `operand_a`, `reg_a`) followed by the `rest` lanes. Pre is on the bottom face and post on the top. Through-vias carry the K10 preservation straps. The K09 writes drop in from a read daughterboard mounted above.

This approach teaches what changes and what holds better than any other. The two-space read shrinks to a module, however, and the reuse of the G3 footprint is less visible.

### III. Lineage overlay

This approach adds a ghost layer to approach I. The A102 borrow daughterboard floats translucent above the B counter strips, so that the copied idiom can be compared stage by stage.

It makes the design lineage tangible. It also risks reading as a wire between 004 and 005, which the source does not contain, so it would need a hard visual break between the two boards.

### Recommendation

The nonbinding recommendation is **approach I, with approach II's cartridge as the top layer**. The cartridge flips open on `P` to show the pre and post faces with their straps. Approach III's comparison survives only as a DOM side-by-side in the source drawer, so that the bench never implies a connection.

## Storyboard for the 005 stop

The caption block opens before any control appears.

1. **Arrival.** The rail reaches 005. The cumulative 004 bench stays still where L1 confirms it. A new substrate sheet rises into its own layer, silkscreened `B · Scalar.tag · Add/First/RetRight`. The relation map gains an *embeds* edge to 003 and a dotted idiom callout to 004.
2. **Inputs.** The input pads populate in the classes that the mapping records. Read-only aliases are hairline-ringed. Independent pre operands are solid. Post witnesses are open squares, and constants and immediates are silkscreen numerals. Keep-out borders mark the host checks, and the `pre.reg_a` pads carry no trace.
3. **Enable and router (K01–K03).** The enable via lights first on the inner layer. Two `CondSelect` trapezoids form `s` and `en_const`. The `en_data` net continues from `s` through a junction dot.
4. **Sockets (K04–K06).** Two G3 instances lower into their sockets. Both come from one pinned `enabled_fetch.rs` revision and belong to the same footprint family, but each is sized by its own table: its row pads and selector ranks follow that space's capacity in the accepted netlist, so the two meshes may differ. Each carries a pin label `emit_enabled_fetch_block` with its scope suffix. A tag gate follows each socket. A *Construction* toggle in the source drawer shows the zero-capacity variant as a flat schematic inset: a grounded enable pad and sixteen silkscreen zero stubs, with no row. The bench itself shows the accepted netlist's actual shape.
5. **Payload (K07).** Sixteen lane routers form a row, with byte 0 at the left and byte indices etched below. Each lane runs into its 8-bit termination.
6. **Writes (K08–K09).** The identity comparators sit against ordinal numerals on the cartridge's pre face. The `phase` write and the sixteen `regA` write vias rise to the post face.
7. **Preservation (K10).** Straps run straight from pre to post for the named fields and for each `rest` lane, in order.
8. **Counters (K11–K13).** Four strips, each with sixteen byte stages. The `remaining_work` strip ripples right on borrow, and the others ripple right on carry. Each chain ends on a ground pad labelled with its final `ConstrainEq(…, 0)`.
9. **Seam.** Dashed perimeter sockets labelled OBL-01 to OBL-12 settle at the board edge, unlit.
10. **Compare.** `B` toggles the before state, the 004 bench without this layer, and `C` isolates the delta.

## Source-to-mesh map

| Anchor | Source | Mesh element | Line style |
|---|---|---|---|
| S1 | host checks before `Emitter` | silkscreen keep-out borders | none (not a part) |
| K01 | `b.boolean(enable)` | enable via | control |
| K02–K03 | `select(enable, space, 0)`; `select(s, 0, enable)` | space router trapezoids; `en_data` junction | control |
| K04–K05 | `space_read` → `emit_enabled_fetch_block` | two G3 socket instances (one footprint family, per-space capacity) | data inside, control enable |
| K04–K05 (capacity 0) | `b.eq(enable, immediate(0))` | grounded enable pad, zero stubs (inset only) | construction |
| K06 | `gated_eq(&enable, selected[0], tag)` | tag gate | control |
| K07 | `select(s, n_data[i], n_const[i])`, `bits(8)` | 16-lane router row | data |
| K08 | five `gated_eq` against ordinals | identity comparators | control |
| K09 | post `phase`, `reg_a[i]` | write vias | data |
| K10 | `gated_eq(post, pre)` | through-straps | data |
| K11 | `eff_bytes` | lane gates on strips | control |
| K12 | `decrement` | borrow strip, ground pad | data and control |
| K13 | `increment` (three carriers) | carry strips, ground pads | data and control |

Line anchors come from the published bytes at build time. The mapping's recorded anchors (`K01-K03` 136, `K04-K06` 143, `K07` 151, `K08-K09` 157, `K10` 171, `K11` 182, `K12-K13` 191) refer to the frozen candidate only.

## Materials, light and glyphs

The substrates are matte, in the ink grounds `#14130f` and `#0e0d0a`. Traces are tinned ink `#a9a396` in high-roughness `MeshStandardMaterial`. Pads are `InstancedMesh`, so the reusable pad, selector and router components of both G3 instances can share geometry buffers. Each complete fetch mesh is assembled from its own netlist instance, so the two need not be identical. The B lineage is set apart by silkscreen lettering and a notched board edge, not by a new hue.

Control nets use a dashed pattern, and data nets are solid. `CondSelect` keeps the trapezoid glyph from iteration 003. `TestEq` stages are small diamonds. `ConstrainBits` terminations are bars etched with their width (`8`, `9`), and `ConstrainEq` ground pads are filled circles.

Copper `#e8724a` (dark) or `#9a3412` (light) is proposed for the write terminals only: the `phase` via, the `regA` vias and the counter post pads, where post diverges from pre. The preservation straps stay ink. This extends the divergence rule from the Execution rulers to the board, which is an open question from Design 006. Edge emission on the active layer and a cyan for selected nets remain internal proposals pending design review and contrast measurement. Each keeps a non-colour twin in dash, glyph or word. The wordmark stays flat, unlit, in the sans at weight 600.

## Interaction, accessibility and exports

**Inspection.** Selecting any pad opens a DOM drawer showing the source path, full SHA256, commit, symbol anchor, an excerpt copied from the pinned bytes at build time, and the operand class recorded in the mapping. *Follow read* traces `operand_a` through the router, the enabled socket and the lane row to `regA`. *Follow write* shows the K09 targets. *Follow preserve* shows the straps.

**Keyboard.** Shortcuts are active only while the board region has focus: `R` (read path), `W` (writes), `P` (preservation and cartridge flip), `K` with `[` and `]` (step the K anchors), `B` (before), `C` (changes), `S` (source drawer) and `Esc`. Every control has a visible `:focus-visible` ring.

**Mouse, touch and reduced motion.** Fixed orthographic views are the default, and orbit is opt-in. Touch targets are at least 44 px, and swiping steps the anchors. Under `prefers-reduced-motion`, the layer rise, socket docking and strip ripple become cuts.

**Without WebGL or scripts.** The board renders as a K01–K13 table, an operand-class table (read, write, preserve), an obligations list, and a static build-time SVG schematic with text-node labels. On mobile, the yard collapses to a vertical sequence: router, sockets, lanes, cartridge, strips.

**Exports.** The page offers the published source bundle with its manifest, the column mapping and the netlist binding as exact bytes, served as attachments with their filename, byte count and SHA256 preserved. This stop has no capture export.

## Launch bindings carried forward

The Design 006 publication review set conditions L1–L7. They are the owner's existing launch conditions, and this note carries them forward without rewriting Design 006:

- **L1. Stillness.** Board stillness requires that no mesh derive from a symbol that changed between the `mod.rs` pins. Here that means `e2af2fe0…` against `5f193ee1…`, and the check applies to every board at every stop.
- **L2. Host records.** Host record replay is an interpreter run. "Running on the board" requires an Activity binding under an accepted correspondence. Rule identifiers such as `b.helper.Scalar.tag` and capture `rule_id` strings are joined only through a published mapping.
- **L3. History versus membership.** The rail shows superseded pieces. Membership in the final interpreter comes only from the Integration binding.
- **L4. Lossless integers.** Every integer keeps its original text, with values and key order taken from the bytes. JavaScript `Number` never parses a u128.
- **L5. Passive rendering.** Source, data and identifiers reach the DOM only as text nodes. There is no `innerHTML`, no Markdown or Mermaid evaluation of data and no `eval`, the CSP is restrictive, and downloads are non-executable attachments.
- **L6. Stack.** Plates derive from the recorded `frames` array diff, with the depth fields shown verbatim beside them.
- **L7. Example programs.** No ungrounded superlatives about example programs.

This iteration adds four 005-specific bindings:

- both sockets bind to the pinned `enabled_fetch.rs` revision, and each instance binds to its own space's capacity, active length and wiring in the accepted netlist;
- the bench shows the capacities of the accepted netlist;
- the copper write terminals are confirmed against the netlist's write set;
- an optional activity layer for this board appears only for records that an accepted correspondence ties to this leaf.

This leaf implements no recursion. Recursion views stay with the finished release's own source bindings.

## Grounding and provenance (author-facing)

| Subject | Recorded fact |
|---|---|
| Source | `b_scalar_tag.rs` `af82e70192b34f6bd5a95ec1008fe412a5294057b2becd149cab24aef60285cb`, 13854 B; callable `emit_b_scalar_tag_add`. A separate unregistered runtime freeze, unpublished, with no verified public locator and no `mod` declaration. |
| Manifest | Candidate `90db5144e24d05ab05712260bca4ec85d4672e995b2c1809f9c01b34359129e5`; column mapping `05dfcbe2744dc407788bd41d42935db1ecbc11c08f24931a962e64d98170a56a`; static inventory `e0bd581dffa498afdbb105bc13a1ea169da1e16e301dc3573f2d9201e4d5efd0`; implementer report `696ddaf8b99644461acbae575cb5f2c6fc83c824ef336b37cd70482e718a9c4e` |
| Caller contract | `eff9da91e02019722908deff03fd4925b189e807e86d7b3f43ee6308dc2adf98`, Opus-authored, `PROPOSED_SOURCE_DESIGN_ONLY`. It adopts no ABI, width, layout, field or row. The row overlay's own status reads `PROPOSED_UNADOPTED`. |
| Builder inputs (unchanged by this candidate) | `literal_step.rs` `b5d2ca73b0e6b561f554902c3f4e50ec7743cc7ca04b9e7e9acb0c3aca10b839`; `enabled_fetch.rs` `6c45ab9190ad829db29b039d91050bfd8b82d1ccb66b3dd990538c18415dc8c1`; `fetch.rs` `6e71e0dccf687bec051556eeefdc435cef1c61535323d62a3994dc7220cadc87`; `mod.rs` `5f193ee16986094b9706c7dedc7c8ac091d29496b83e46e71fa7c4553f6eea2f` (8703 B) |
| Reviews | A separate Muse source-reading review returned `APPROVE_WITH_SCOPE` for this manifest. It is a source-reading result only. At the initial creative request freeze, the designated Opus source and constraint review was pending. Its later result is recorded in the dated grounding update below. This creative note is not that review. |
| Execution | Compile, SDK, model, test, proof and native checks all UNRUN. There is no accepted netlist, constraint result, post-StepInv proof, or source-to-circuit correspondence. |
| Inputs not read | No D01, corpus, capture, O or QA tuple. No example values appear here. |
| Authority | Native authority 0; C4 unchanged; brainstorm only. |

**Grounding update, 2026-10-17 UTC.** The designated Opus source and constraint review later returned `APPROVE_WITH_SCOPE` on candidate manifest `90db5144e24d05ab05712260bca4ec85d4672e995b2c1809f9c01b34359129e5`. The source bytes were unchanged, and the review required no source-construction correction. Its scope is only the positive `RetRight / Add / First / UInt` leaf: the operand read and the stage write of `regA` and `phase`. It does not cover numeric addition, which belongs to a later phase. It abstains where the full normative body was outside its packet, leaves OBL-01 to OBL-12 with the caller, and keeps the SDK semantics and field-size facts as unproven assumptions. Compile, SDK, model, test, proof and native checks remain UNRUN. This update changes no design content in the note, and the review is not a design, website, integration or execution acceptance.

Source currentness is scoped to the recorded freeze and to the dated update above. Any later change to the source, registration or reviews is recorded as a separate dated revision. Private review bodies, prompts, provider streams and private paths are excluded.

## Open design notes

1. Should the rail introduce an *idiom* annotation beside the Design 006 relation vocabulary, or should copied design live only in source drawers?
2. When the accepted netlist fixes a zero capacity for one space, should the bench show the grounded pad as a real component, since it is then an emitted `ConstrainEq`?
3. Is copper at write terminals an acceptable reading of the divergence rule?
4. How should the later Sub, Mul and Lt modes, the `PrimError` negative branch and the Second-phase leaf join this board without implying integration edges before the architecture supplies them?
5. Which published mapping, if any, joins `b.helper.Scalar.tag` to the HOST capture's `rule_id` strings?