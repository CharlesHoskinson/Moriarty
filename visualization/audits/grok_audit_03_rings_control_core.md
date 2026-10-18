# Audit 03 — Split-Arc Ring Design Language & 10-Sector Control Core

**Auditor:** Grok 4.7 xhigh #3 (Split-Arc Ring Design Language & 10-Sector Control Core)
**Stance:** Apple product design and horological engineering. One job per surface. Position carries identity. Color carries a family. Motion is a detent, or it is noise. A part that intersects another part is a rejected movement, whatever the render looks like from across the room.
**Date:** 2026-10-18
**Piece under inspection:** `src/designers/designer2_well.js` (`createWellSubsystem`)
**Studies:** `ring_study_02_frames.md` (algebra and families), `ring_study_03_control_core.md` (stator), `ring_study_04_split_arc_geometry.md` (machining), `ring_study_08_rings_impl.md` (integration note that the code follows)
**Ground truth:** `crates/mori-core/src/sherlock/reference.rs` and `reference_types.rs` in the Moriarty tree. `theme.js` `FRAME_KIND_META` / `CONTROL_KIND_META`.

---

## 1. Verdict

The split is the right movement. Twelve chronological half-arcs, three frame families, a ten-tooth control stator in place of the ruby, and a 39-rule floor are the correct instruments for `sherlock-k-a/1`. Chronological slotting is implemented correctly: `frames[frameCount - 1 - slot]` anchors the root on `R1.A` and the frontier on the leading arc. West and East are cut on the right meridian. The 14.4° bridges are real geometry, not an alpha clip. The floor grid’s UV matches the tiles. The stator tooth order puts `EVAL` at north and steps clockwise, which matches study 03.

It is not a finished movement. Four defects would send it back from the bench:

1. The transfer hand, the north–south strut, and the `R6` plaques occupy the same volume as the bore they are supposed to clear.
2. The sapphire “lens” is an opaque disc (`opacity 0.72`). The ruby was removed so the well could be seen. The disc puts the ruby back in blue.
3. The three semantic families and the ten control teeth have been collapsed into four hues, and those hues are shared across surfaces that do not mean the same thing. `Rule 177` (`NextArg`) twists an arc whose color does not change. `Return` and `Scope` are the same teal, so the `sum_list` signature (contract, scope, operand) cannot be read.
4. The floor engraves error codes that `reference.rs` does not use. `Error(1)` and `Error(2)` do not exist. `A111` is `Error(3)`. `A225` is `Error(10)` or `Error(11)`, and the floor says `Error(5)`, which is the non-boolean `if`.

Study 08 and the code took study 04’s staircase and study 03’s hand and shrank the bore around them without re-tolerancing. The next iteration is a clearance, semiotics, and engraving pass. It is not a new concept.

---

## 2. Which document wins

The four studies disagree with each other, and the code follows study 08 even where study 08 is the weak one. Freeze this authority before anyone edits a constant:

| Question | Authority | Reject |
| :--- | :--- | :--- |
| Which frame sits on which arc, and what a rewrite means | Study 02, checked against `reference.rs` | Study 08 §1, which states the `fact(4)` pair as if it were a law |
| Family hues on the arcs | Study 02’s two-step pairs (deep/bright inside one hue) | The single cyan and single violet in `designer2_well.js`; the sky-blue `Scope` in `theme.js` |
| What the ten teeth are | `Control` in `reference_types.rs`, order `Eval…Error` | Painting those teeth with frame-family hues |
| Cross-section, air gap, anglage, gap silence | Study 04 | Extruded 0.028 stock, 0.036 gaps, clasps sitting in the bridge |
| Where the hand is allowed to reach | The bore that was actually built | Study 03’s `r = 0.385` hand, drawn when `R6` inner was `0.418` |
| Error numerals on the floor | `reference.rs` | The numerals currently in `SHERLOCK_K_RULES` |

`theme.js` already has `FRAME_KIND_META` and `CONTROL_KIND_META`. The well ignores both and keeps a second, poorer table. One table, two consumers. Before adopting `FRAME_KIND_META`, fix the token split: `Return.color` is `0x2dd4bf` and `Return.css` is `#0d9488`. A pipe and a plaque built from those two fields will not match.

---

## 3. What already holds

**Slot algebra.** Six rings, twelve arcs, `slot = 2r` west and `slot = 2r + 1` east. Root stays on `R1.A` for the life of `fact(4)`. That is the difference between a chapter ring and a conveyor.

**Meridian.** In Three.js theta (0 at east, positive toward north after `rotation.x = -π/2`): west `[0.54π, 1.46π]`, east `[-0.46π, 0.46π]`, bridges centered at `0.50π` (north) and `1.50π` (south), each `0.08π = 14.4°`. Study 04 writes the same intervals in `atan2(z, x)`, which is the negation of Three.js theta. Both arcs are symmetric about the X axis, so the intervals describe the same metal. Do not “correct” the sign. The light-pipe `rotateZ` offsets (`-0.44π` east, `+0.56π` west) live in Three.js space and are not similarly symmetric; leave them.

**Machined span.** Each arc is `0.92π = 165.6°`, not 180°. The headers that say “180° half-arcs” are the nominal hemisphere. The metal is the hemisphere minus the bridge. Keep the gap. Rename the comments so the next pass does not close it.

**Two of the three rewrites are visible.** `A145` `Let → Scope` crosses violet to teal. `A149` `Left → Right` crosses amber to gold. The twist (`rewritePulse` decaying through a `sin`, peak about `0.042 rad ≈ 2.4°`) is the right kind of motion: a detent that returns to the bridge stops. `A177` is detected and twisted, and then fails chromatically (below).

**Stator indexing.** Sector `i` at `θ = π/2 - i·2π/10`, so `EVAL` is north and the order runs clockwise through `ERROR` just west of north. `resolveControlSectorIndex` maps the ten `Control` variants onto `0..9`, and `Done` shares the `RETURN` tooth, which is what rule 80 actually is: a return on an empty stack.

**Floor count and alignment.** `SHERLOCK_K_RULES` is 39 entries, `A080` through `A225`, lines matching `reference.rs`. The 6×7 grid leaves row 6, columns 3–5 for the cartouche, and the cartouche rectangle is exactly those three cells. `PlaneGeometry` UVs, `flipY`, and `toPxY(worldZ)` put north at the top of the texture. Labels sit on the tiles they name.

**Draw-call shape.** One instanced mesh each for fuel, stator teeth, clasps, and rule tiles, plus one static floor overlay. That budget is the right one. Spend any new signal inside it.

---

## 4. P0 — The bore is not clear

`R6` is the inner chapter. Code ladder (`lerp` from `y = 0.06, r = 0.95` to `y = -0.21, r = 0.31`, width `0.092` except `R6` at `0.078`):

| Ring | `baseY` | `outerR` | `innerR` | `midR` | Air gap to next outer |
| :--- | ---: | ---: | ---: | ---: | ---: |
| R1 | +0.060 | 0.950 | 0.858 | 0.904 | 0.036 |
| R2 | +0.006 | 0.822 | 0.730 | 0.776 | 0.036 |
| R3 | −0.048 | 0.694 | 0.602 | 0.648 | 0.036 |
| R4 | −0.102 | 0.566 | 0.474 | 0.520 | 0.036 |
| R5 | −0.156 | 0.438 | 0.346 | 0.392 | 0.036 |
| R6 | −0.210 | 0.310 | 0.232 | 0.271 | — (bore) |

Study 08’s table lists `R1`–`R5` inners 0.004 smaller (`0.854, 0.726, 0.598, 0.470, 0.342`). The code’s width is `0.092`, the table’s width is `0.096`. `R6` matches the table. Study 04 is a different movement: width `0.080`, air gap `0.048`, `R6` inner `0.260`, `R6` at `y = -0.280`. Nothing in the scene was rebuilt when the bore moved from `0.418` (study 03) to `0.232` (this code).

Hard interferences, measured off the constants:

**The hand lives inside `R6`.** Nominal tip `pointerRadialReach = 0.258`. The cone is `0.036` long, so the point sits near `0.276`. `PUSH` sets reach to `0.282 + 0.018 = 0.300`. `R6` inner metal is `0.232`. The stem (`length 0.11`, centered `0.064` behind the tip) still crosses `0.232` at the calm reach. A hand that cannot clear the chapter ring is not a hand. Study 03’s length `0.385` was drawn for an aperture of `0.418`. That aperture is gone.

**The active tooth swells into the same bore.** Instance scale `1.045` is about the origin, so the pad outer `(0.225 - 0.002) * 1.045 ≈ 0.233`, which is past `R6` inner `0.232`. Elevation is a translation. A scale changes the anglage and closes the kerf. Study 03 asked for `+0.006` in Y and a radial scale only if the kerf survives. This kerf is `sectorStep - 0.045 ≈ 2.58°`. Study 03 asked for `4°`. At mid-radius `0.185`, `2.58°` is about `0.008` of dark. That reads as a crack, not as an anodized isolation.

**The north–south strut bisects the lens.** `BoxGeometry(0.012, 0.008, innerR * 2)` with `innerR = 0.232` is a bar from `z = -0.232` to `z = +0.232` through the origin. The lens is a cylinder of radius `0.135` on that origin. The bar runs through the glass and through the readout plaque (`0.225 × 0.072`). Study 03 hung the stator on four hairlines at 45° so the cardinals stayed clear for the index. Study 08 put the supports on the cardinals and the code made one of them a diameter.

**`R6` plaques overlap the stator and kiss `R5`.** Plaque `0.16 × 0.058` is centered at `midR`. On `R6` it covers radius `[0.191, 0.351]`. The ring metal is `[0.232, 0.310]`. The plaque crosses the bore, overlaps the stator (outer `0.225`) by about `0.034`, and overlaps `R5` (inner `0.346`) by about `0.005`. On `R1`–`R5` the same plaque is wider than the `0.036` air gap and lands within `0.002` of the neighbor. They are stickers hovering at local `y = 0.0195` on a deck whose top is local `y = 0`, not the flush curved index study 04 specified. A `0.16` label cannot sit on a `0.078` ring.

**Outer collet is a zero-clearance stack.** Torus major radius `0.229`, tube `0.0045`, outer skin `0.2335` against nominal bore `0.232`. The extrude bevel (`0.004`) may give back a couple of thousandths. Do not depend on that. Specify `≥ 0.012` between every stator skin and `R6` inner metal, and test it as a number, not as a screenshot.

---

## 5. P0 — The lens does not open

Study 03’s reason for deleting the ruby is the sightline: `r < 0.088` clear, type in the annulus, camera looking down `x = 0, z = 0` sees wafers and the floor. The code builds `CylinderGeometry(0.135, 0.135, 0.006)` at `opacity 0.72` with a dark albedo, and lays an opaque plaque (`opacity 0.96`, background `rgba(11, 21, 40, 0.90)`) on top. From the hero camera this is a cabochon. The floor at `y = -2.18` is not in the optical path.

The `outputWave` is born at `r = 0.235`, already inside `R6`, and scales to `4.2`, so it travels through all six rings. `depthWrite: false` makes that a ghost rather than a solid crash. Its color is `HEX_ENV_CYAN` (`0x2dd4bf`), the same hue as a live `Return` arc. Rule 80 then looks like another scope frame. A finished watch does not strike the hour in the color of a running train.

---

## 6. P0 — Four hues are doing ten jobs

`getFrameSemanticHex` and `CONTROL_STATOR_SECTORS`:

| Surface | Code hue | What the algebra actually needs |
| :--- | :--- | :--- |
| `Return` and `Scope` | both `0x2dd4bf` | Study 02: `Return` `#0d9488`, `Scope` `#2dd4bf`. Same family, two values. |
| `Left`, `Unary`, `Args` (any cursor) | all `0xf59e0b` | `Right` is `#fbbf24`. `Args` after `A177` must join `Right`. `Unary` sits a step toward copper `#d97706`. |
| `If`, `Match`, `Let` | all `0x818cf8` | `Match` is `#a78bfa`. `If` and `Let` can share `#818cf8`. |
| `EVAL` and `PRIM` | gold `0xfbbf24` | Gold on an arc means “operand locked.” `EVAL` is the unlocked descent. |
| `PUSH`, `INVOKE`, `ENTER`, `CASES` | one amber | Four different teeth. Position can distinguish them only if the engraving is readable and the metal is quiet. |
| `RETURN`, `BOUND`, `MATCH` | one teal | `BOUND` has not pushed `Frame::Return` yet. `MATCH` (`CaseResult`) misses back to `CASES` as often as it pushes `Scope`. Teal here claims an environment frame that may not exist. |

The ruby was mute because every control looked the same. Mapping ten teeth onto four frame hues recreates that at one level up, and it makes the center tell lies about the stack. `BOUND` lighting teal does not mean a `Scope` or a `Return` is on a ring.

`theme.js` `CONTROL_KIND_META` gives each tooth a private hue (`Enter` sky, `Bound` violet, `Cases` indigo). That is a second palette fighting the arcs. Ten colors on a 36° tooth is a dashboard. A movement does not color-code every wheel.

**The rule that resolves both studies:**

- Family color lives on the twelve arcs only. That is where two different frames share a ring and west/east is not a family.
- The stator is one metal. Identity is the engraved index and the raised tooth. One active tooth, champagne, plus crimson on tooth 9 when it is actually the fault. Idle teeth stay `0x1e293b` with no 16% lerp toward their “semantic” color. The current lerp leaves `ERROR` faintly pink at rest, which teaches the eye to ignore a real fault.
- Do not paint `EVAL` gold. Gold is reserved for a locked operand.

**`A177` is the rewrite this palette drops.** `Frame::Args` does not change `kind`. The classifier never reads `values.length` or the `(0/` / `(1/` label test that study 02 specified. The arc twists and stays amber. In `cube_plus`, `S007` (`Args 0/1 → 1/1`) is the entire point of that preset, and the ring does not show it. The 36 hemisphere latches in studies 02 and 04 were the cursor. They were not built. The 24 bridge clasps that were built sit in the gap, take the family color, and close the silence the gap was machined to provide.

**West is not ENV and East is not OP.** Study 08 §1 states that as the architecture, using `fact(4)`. Study 02’s own traces contradict it:

- `sum_list` at `S034`: `R1.A = Return`, `R1.B = Scope`. Both ENV. The gold operand is on `R2.A`.
- `cube_plus` at `S013`: `R1.A = Scope`, `R1.B = Return`. East is the contract.
- `fact(4)` before `S021`: `R1.B` is `If`, a gate, and only later becomes `Right(4 * [?])`.

The geometry already allows any family on either half. Keep it that way. Do not engrave family names on the bridges. The `fact(4)` pair is a beautiful settled state. It is not a constraint.

---

## 7. P0 — The floor engraves the wrong faults

Counted against `reference.rs`. Codes `1` and `2` are not produced anywhere in that file.

| Tile | Engraved in `SHERLOCK_K_RULES` | `reference.rs` |
| :--- | :--- | :--- |
| `A111` VarMissing | `Error(1)` | `Error(3)` unbound variable |
| `A140` FrameErr | `Error(4)` | `Error(4)` correct |
| `A169` IfErr | `Error(2)` | `Error(5)` non-boolean condition |
| `A190` RetErr | no numeral | `Error(6)` return type |
| `A202` MissingFn | `Error(1)` | `Error(7)` unknown function |
| `A206` BindErr | no numeral | `Error(8)` arity or parameter type |
| `A212` NoMatchErr | `Error(3)` | `Error(9)` non-exhaustive match |
| `A225` PrimErr | `Error(5)` | `Error(10)` on `UInt` overflow, `Error(11)` on a domain miss |

`A225` saying `Error(5)` is the serious one: `Error(5)` is already `A169`. A person reading the floor will merge an arithmetic fault with a bad `if`. `A111` and `A202` both claim `Error(1)`, so two different faults share a fictional code. These strings are baked into the 2048 overlay. They are the engraving, not a comment.

`A223` and `A225` sit in the chapter labeled “Pattern Matching & Primitives.” They are the ALU. `A181`’s copy omits the pop; `FinishArgs` does `frames.remove(0)`.

The 6×7 pitch itself is sound, and the cartouche is in the right three cells. The six comment-groups are not visual groups. Row 0 is three terminal rules plus three dispatch rules. `A145` and `A149` end row 2; `A177` ends row 3. The three silent rewrites, which are the complication this whole piece exists to show, are not a set on the floor. Chapter ink can mark them without moving a tile. Moving a tile means re-proving the UV.

Active tiles scale Y by `1.35` and rise to `y = -2.174`. Half-height becomes `0.0054`, so the top is about `-2.1686`. The label plane is at `-2.171`. The lit cell, the one the reader came down to see, intersects its own type. Idle tiles clear the plane by about a thousandth. Drop the Y scale. Lift the overlay to `WELL_FLOOR_Y + 0.014` if a rise is kept, and keep the rise under `0.003`.

---

## 8. P1 — Escapement, chamber, and the way color breathes

**False beats on the hand.** `isPushRule` is true when `after.control.kind === 'Push'`. Rules `114`, `117`, `120`, `123`, `126`, `131`, `208`, and `220` schedule a `Push`; they do not commit it. The hand lunges on the schedule and again on `A135`. `rawControlKind === 'Return'` forces the inward, cyan “pop” on `A102` (literal), `A108` (variable hit), and `A223` (primitive ok). Those steps never took a frame off the stack. A remontoire has one beat per event.

Drive the hand, or its replacement, off the depth delta and the three rewrite lines only:

- Outward beat: `frameCount > beforeFrameCount` (the committed `A135`).
- Twist, no depth change: `145`, `149`, `177`.
- Inward beat: `frameCount < beforeFrameCount`.
- Otherwise the index rests on the active tooth and does not travel.

**The chamber and the rings are different machines.** Twelve arcs, eight terraces. `stackLimit` is clamped to `TERRACE_COUNT` (8). At `fact(4)` peak depth 11, terraces `0..7` show chronological frames `0..7`, and `isDeepestOccupied` breathes on frame 7. The real frontier is frame 10, which has no terrace. The rings, if left unclipped, still show it. Past depth 12 the ring buffer keeps the oldest twelve and drops `frames[0]`. The seconds hand falls off and the hour hand keeps a stale root. Always show the frontier. If the chapter only holds twelve, light an overflow pip on the `R6` bridges when `frameCount > 12`, and do not drop `frames[0]` to keep `frames[d-1]`.

**Held arcs are too hot.** Top pipe at emissive `1.45`, held at `0.85`, plus a sine breath on every occupied arc. Eight breathing arcs and a 14° gap read as one glowing washer. Held parents at `0.22`, frontier at `0.90`, idle at `0.04`. No breath on a held arc. The detent is the information; the throb is a toy.

**The light pipe is a hose.** `TorusGeometry(midR, 0.012, …)` has diameter `0.024` on an `R6` width of `0.078` (about a third of the ring) and it stands proud of a `0.028` deck. Study 04’s channel is a groove in the inner anglage, tube radius about `0.004`, below the chamfer line. The `0.88π` length, inset from the `0.92π` metal, is the right idea. The radius is not.

**Cross-section.** Study 04 specified a 6-vertex lathe profile, `0.052` thick, `0.011` of 45° anglage, roughness `0.34` on the idle metal. The code extrudes a rounded bevel of `0.004` on `0.028` stock at roughness `0.22`. Under a grazing key light that reads as a printed washer, not as a diamond-cut flank. The specular line on the inner chamfer is how a person tells `R3` from `R4` without reading a plaque. Restore the flank even if the radius ladder stays on the study 08 pitch.

**Idle stator tint and missing afterglow.** Inactive teeth lerp 16% toward their hue every frame. Study 03’s previous-tooth afterglow (`0.35`, decaying) was the right two-state read (`PUSH → EVAL` on `A135`) and it was not built. The floor already has a trail. The stator should have the same trail and no idle tint.

**Plaque copy budget.** `formatHalfArcPlaqueCopy` trims the expression to 15 characters and then wraps it as `R1A · RET:UInt (n=4)`. The string that hits the texture is often past 24 characters, on a plane that does not fit the ring. The arc gets a three-character badge (`RET`, `4×?`, `IF`, `ARG`). The lens gets the sentence. Study 03’s 14-character lens headline is the right cap; the lens title is currently unbounded (`EVAL ·` plus the raw control summary).

**`Done` and `Return` share a tooth and a hue.** Sharing the tooth is correct. Sharing the hue with every live `Return` arc is not. On rule 80, keep tooth 2 raised, change the lens word to `DONE`, and fire one pale paper wave (`#f4f0e8`, opacity ≤ 0.4) in a plane above the rings (`y ≈ 0.09`), one shot, not a loop tied to `ambient`.

---

## 9. P2 — Nomenclature, cache, and the claim of zero allocation

Comments and inspectable titles say “180°” beside `HALF_ARC_THETA_LENGTH = 0.92 * Math.PI`. Say “nominal 180° slot, machined 165.6°, bridges 14.4°.”

`update` builds a template string per arc per frame for the plaque key, and `formatSlotEnvCompact` concatenates on every call. The matrix math uses the scratch singletons and that part is real. The “zero heap” header is not. A GC hitch on a scrub is a beat error. Compute the key only from fields that change the pixels, and stop including `isTopOfStack` in the key: the copy function does not use it, so a frontier change reallocates two identical plaques.

`wallSide` on each rule is unread. The decision to drop the wall TSV clutter is the right Rams cut. Delete the field when the table is next touched so it does not grow a second, fictional bus.

Even rings share one metal, odd rings another. That alternation is a good counting stripe. Leave it. Do not also tint the metal by family; the pipe already has that job.

---

## 10. Next iteration

Do these in order. Each one is a finish criterion, not a direction.

**1. Clear the bore.** Delete `northSouthBridgeMesh`. If the stator needs hangers, use four hairlines of radius `0.0018` at 45°, 135°, 225°, 315°, ending at `R6` inner minus `0.012`. Delete the long stem and the cone. The active tooth is the index: it translates `+0.006` in Y, it does not scale, and its outer skin stays ≤ `0.220`. Inner collet, outer collet, dial, and lens backing all stay inside `r = 0.220`. `R6` inner metal stays ≥ `0.232`. Assert `0.232 - 0.220 ≥ 0.012` in a unit test next to the constants.

**2. Open the lens.** Replace the cylinder with `RingGeometry(0.088, 0.128)` plus a type plane whose background alpha is 0 outside the glyph ink. No disc at `opacity 0.72`. From a camera on `x = 0, z = 0`, the floor cartouche is visible through `r < 0.088`. Lens headline ≤ 14 characters. Full rule name stays on the floor and in `userData`.

**3. One family table, used by the arcs only.**

```text
Return  0x0d9488     Scope  0x2dd4bf
Left    0xf59e0b     Unary  0xd97706
Right   0xfbbf24     Args   0xf59e0b until a value has been pushed, then 0xfbbf24
If      0x818cf8     Let    0x818cf8
Match   0xa78bfa
idle    0x1e293b
```

`Args` gold flips when `A177` has fired or when the label is no longer `0/`. Fix `FRAME_KIND_META` so `color` and `css` are the same hue, then delete `HEX_ENV_CYAN` and the local classifier. `A145` and `A149` keep working. `A177` starts working.

**4. Stator metal, not a second palette.** Idle tooth `0x1e293b`, flat. Active tooth champagne `0xd4af37` except tooth 9, which is `0xe11d48` only while the control is `Error`, `Rejected`, or `Exhausted`. Engrave three-letter indexes (`EVL PSH RET INV ENT BND CAS MCH PRM ERR`), upright, no 100-tick track. A 10-state register is not a seconds scale. Previous tooth holds 35% of the active hue and decays. No idle lerp.

**5. Three latches, not twenty-four clasps.** Remove `bridgeClaspsInstancedMesh` from the bridge gaps. The gaps stay empty gunmetal. Add one instanced mesh of 36 hemispheres, radius about `0.008`, sitting on the arc at three stations per half, inside the metal, not in the kerf. On `Args`, light latch `k` gold when that argument is committed and amber when it is the live hole. On every other frame the three latches track the pipe color at low intensity so the arc still has a rhythm. This is the `A177` escapement study 02 already specified.

**6. Plaques that fit.** Arc badge width ≤ `0.55 * arcWidth` (`≤ 0.043` on `R1`–`R5`, `≤ 0.040` on `R6`), centered at `midR`, flush (`local y ≤ 0.002` above the deck). Three characters. Curved if the extrusion can take a second UV; a short flat plane is acceptable if it does not leave the metal. Delete the `0.16 × 0.058` plane and the plinth.

**7. Re-engrave the floor, do not re-grid it.** Correct the eight numerals in the table in §7. Split the `A225` line so the overlay can show `E10` and `E11` as the two prim faults, even though they share a tile: the tile copy should read `PrimErr · E10/E11`. Add a 6-pixel chapter tick and a family pip: teal, amber, violet, crimson, or metal, using the same hexes as the arcs. Put the same small gold triangle on `A145`, `A149`, and `A177`. Remove the Y scale on the active tile. Keep the cartouche where it is.

**8. One beat.** Implement the depth-delta table in §8. Delete `after.control.kind === 'Push'` and `rawControlKind === 'Return'` from the motion predicates. Held-arc emissive `0.22`, frontier `0.90`, idle `0.04`, breath amplitude `0`.

**9. Groove, not hose.** Light-pipe tube radius `0.004`, seated `0.002` below the deck, span `0.88π` kept. If this pass touches the cross-section at all, replace the extrude with study 04’s six-vertex lathe: thickness `0.040` is enough if `0.052` will not clear the stair, bevel `0.008` at 45°, idle roughness `0.34`. Do not change the radius ladder in the same pass as the bore clearance. One variable at a time.

**10. Frontier rule.** `topChronoIdx = frameCount - 1`, clamped into `0..11` for display, and if `frameCount > 12` the displayed window is the newest twelve (`frames[11] … frames[0]` mapped onto `R1.A … R6.B`) plus a crimson pip on both `R6` bridges. Document that choice next to the constant. The current window keeps the oldest twelve and hides the frontier. The chamber either grows to twelve terraces or it labels itself as an eight-deep section and lights the same indices the rings light, with the same overflow pip. Stop clamping `stackLimit` to 8 without an off-scale mark.

**11. `Done` wave.** One shot, `y = 0.09`, color `#f4f0e8`, peak opacity `0.4`, scale running from `1.0` to whatever clears `R1` outer in that higher plane. It must not be born at `r = 0.235`.

---

## 11. Acceptance, on the three traces study 02 already computed

Run these against the built scene. A screenshot of the idle well is not an acceptance.

**`fact(4)` `S021`.** Depth stays 2. `R1.A` deep teal `Return`. `R1.B` flips amber → gold in place and twists back to the bridges. No other arc twists. The hand, if any remains, does not cross `r = 0.220`. Tooth `EVAL` is the raised tooth after the step (the next control is `Eval`), and tooth `RETURN` is the decaying afterglow only if it was the previous tooth.

**`fact(4)` `S129` then `S131`.** Eight arcs, alternating deep teal and gold on `R1`–`R4`. `S131` pops `R5.A` only. The other seven do not flinch.

**`sum_list` `S034`.** `R1.A` deep teal, `R1.B` bright teal. They are not the same pixel. A person who has never read the plaque can tell contract from scope.

**`cube_plus` `S003`.** `R1.A` violet → bright teal, same depth, one twist. **`S007`.** `R1.B` stays `Args`, the first latch goes gold, the arc twists, the pipe leaves amber.

**Sightline.** Camera at `(0, 1.4, 0)` looking at the origin: the cartouche `SHERLOCK · K-A/1 · 39-RULE ROM` is readable through the lens hole. The north–south bar is absent. No plaque texel intersects a neighboring ring. Measure it: project each plaque’s four corners to radius and compare to that ring’s `innerR` / `outerR`.

**Floor.** Hit `A111`, `A169`, `A202`, `A212`, `A225`. The overlay numerals are `E3`, `E5`, `E7`, `E9`, `E10/E11`. The active tile’s short name is fully visible, not sliced by the label plane.

**Clearance test.** A pure function of the constants returns the minimum of (bore inner − stator outer skin), (each plaque’s inner edge − next ring outer), (each plaque’s outer edge − own ring outer). All ≥ `0.012` except the plaque-to-own-metal inset, which must be ≥ `0`. Fail the build on a miss. This is the horological equivalent of a timing check.

---

## 12. What not to reopen

The chronological index, the north–south meridian, the 165.6° / 14.4° split, the decision to instance the teeth and the tiles, the empty chamber walls, and the existence of a ten-tooth stator are settled. The ruby does not come back. A fourth color family does not get added to make `Enter` feel different from `Push`; the tooth’s position and its three letters do that. Study 04’s wider, higher staircase is a later pass, after the bore that exists today is actually clear.
