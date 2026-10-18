# Audit 07 — Technical prose, CEK pedagogy, and architectural honesty

Auditor: Grok 4.7 xhigh Auditor #7
Date: 2026-10-18
Scope: editorial cards and CEK table in `index.html` (the masthead line that frames them, the `.prose-deck` section, and the inspector labels those cards name); designer spec `src/designers/ring_study_10_ui_prose.md`.
Baseline: `docs/SHERLOCK-REFERENCE-STATUS.md` (2026-10-15, with the October 16 delivery update) and `openspec/changes/sherlock-universal-interpreter-design/design.md` (Candidate B, specified-only). Enum shapes, rule lines, and the 11-frame peak below were checked against `crates/mori-core/src/sherlock/reference_types.rs` and `reference.rs`. Display behavior was checked against `src/app.js` and `src/designers/designer2_well.js`.

Verdict: do not polish this copy in place. Card 01 is the one passage that already teaches the machine. Card 02 states an undelivered image as the circuit realization and skips how an environment is saved and restored. Card 03 and ring study 10 teach a false invariant: one recursive call pushes `Return` on the west arc and `Right` on the east arc, and `fact(4)` peaks at 8 of 12 slots with rings 5 and 6 idle. The CEK table’s first two columns are worth keeping. Its third column repeats that invariant and over-describes the center needle.

The next iteration should make a reader able to answer three questions from the page alone:

1. What does one `step()` change, and what does it charge?
2. Where does a saved local live, and what restores it?
3. Why does the root call stay on ring 1, and what is a ring a picture of?

## What to keep

- Card 01’s opening is already aligned with the status note and the design. Sherlock is one reference transition over an explicit snapshot. `step()` returns the before state, the after state, the source-rule line, and a charge. `run()` repeats that relation. `depth()` is `frames.len()`.
- “No operand stack” matches the design sentence that factorial’s pending multiplication and a tree recursion’s pending right walk are explicit state, never a host-stack assumption.
- The CEK row for `Control` is right to say the control carries the expression or the value, not an address. That sentence should survive any rewrite.
- The ten names in the Control cell, and the nine names in the frames cell, match `reference_types.rs`. Keep the Rust names in the teaching text.
- Reversing `frames` so index 0 is the oldest frame is the correct picture. `reference.rs` inserts at `frames[0]`. The inspector list “Saved frames, top first” is the machine order. The rings are the reverse. Ring study 10 explains that reversal clearly, and then its table throws the explanation away.
- Source lines 145 and 149 are real, and they do what the prose says they do, once the sentence is limited to those two rules. Line 145 overwrites `Let` with `Scope` at the same depth. Line 149 overwrites `Left` with `Right` at the same depth. `Right` is never inserted by the push rule.
- Dropping the passive “Ruby” from the user-facing cards matches the stator the die now draws. The study’s claim that every leftover “Ruby” string is gone is still false: `src/stage/SherlockStage.js` and `designer7_cinema.js` still use it in stage copy.

## Findings

### 1. High — “every recursive call pushes Return west and Right east” is not the machine

`index.html` card 03 says every recursive call pushes `Frame::Return` into slot A and `Frame::Right` holding `4 × [?]` into slot B, and that calls walk `R1 → R6`. Ring study 10 repeats this as the ring table and labels the 8-slot picture “Peak Depth,” with `R5` and `R6` idle. The same study then says `fact(3)`, `fact(2)`, and `fact(1)` step inward through `R6`. Those two sentences in section 2 disagree with each other.

What the transition does:

- A call with arguments pushes `Args` (source line 131, then the push at 135). A zero-argument call does not (line 129). `Args` is popped at line 181 before `Invoke`. It is not the east arc of the suspended call.
- `Frame::Return` is pushed later, when `Bound` schedules the body (line 208, then 135). It saves the caller’s environment and the declared result type.
- The pending multiplication is born as `Frame::Left` (line 117). Line 149 rewrites that same slot to `Frame::Right`. There is no push of `Right`. Ring study 02 already records this. Study 10 and the HTML contradict it in the same breath as they mention the rewrite.
- While `fact(k)` tests `n == 0`, the second slot of that call’s ring is `If`, not `Right`. The comparison occupies the next ring. After the test fails, `If` is popped (line 165) and `Left` is pushed into the vacated slot, then rewritten to `Right`. The Return-plus-Right pair is the settled interval of a non-tail `fact` call. It is not the call, and it is not every program.
- Tail calls such as `is_even` / `is_odd` pop `If` and `Args` before the callee is entered, so a suspended caller leaves only `Return`. Two returns then share one ring. `sum_list` uses `Match`, then `Scope`, then a binary frame. Ring study 01 already walks both. The editorial guide dropped them.
- Peak depth of `fact(4)` is the base-case comparison, not the four suspended multiplications. Four outer `Return`/`Right` pairs, plus `fact(0)`’s `Return`, plus its `If`, plus the equality `Right`, is 11 frames. Chronological slots: `R1` = `RET` root + `4 × [?]`; `R2` = `RET` (`n = 4`) + `3 × [?]`; `R3` = `RET` (`n = 3`) + `2 × [?]`; `R4` = `RET` (`n = 2`) + `1 × [?]`; `R5` = `RET` (`n = 1`) + `IF [?]`; `R6.A` = `0 == [?]`; `R6.B` idle. Ring study 01’s table matches this shape. Ring study 09’s “peak” of 9 frames stops at `fact(0)`’s `Return`, before `If` and the comparison. Study 10’s 8-slot table is the settled state before `fact(0)` is entered. Call that state “suspended at the multiplications.” Do not call it the peak.

The 8-slot table is still useful as one captioned moment. Shipping it as the meaning of the rings teaches the wrong rule.

### 2. High — `MachineImage` is written as the circuit this page is

Card 02 says the circuit-facing realization encodes state as `MachineImage` fixed-capacity code, data, environment, frame, and helper vectors, with an append-only child-before-parent data arena, and that physical frame bytes, logical depth, and fuel are reported as separate quantities.

The design document’s own status is “specified-only.” Its delivery table marks bounded `MachineImage`, the helper schedule, and any circuit correspondence as required work with no complete implementation in that document. The status note says the same about the fixed-capacity image, the B helper schedule, and the canonical checkpoint codec. The October 16 update adds a flat-ID image observation under PR #104. That observation is not Candidate B’s image, and the update leaves compiler and circuit correspondence open. The Rust file `machine_image.rs` calls itself an unadopted, private, caller-bounded host snapshot codec proposal and says it selects no B schema, helper clock, or capacity policy.

The design does separate logical depth, physical frame bytes, fuel, helper visits, active and padded cells, and serialization, as distinct `ResourceManifest` fields, and it says unknown is not zero. The card collapses that list to three quantities and says they “are reported.” This inspector reports frame count, a 12-slot window, remaining fuel, and the step charge. It does not report physical frame bytes.

Write the image in the future tense, in one sentence, and say this page runs the A reference. Leave the arena and the manifest for a design note, not the memory lesson.

### 3. High — list and tree identity is the architecture’s sentence with the caveat removed

Card 02: “recursive `List` / `Tree` data with nominal constructor identity.”

The design uses “nominal constructor/type identity” for the generic ADT extension, and the next sentence says the current models’ dedicated list and tree types are narrower than that extension. The status note says dedicated lists and trees are not yet general declaration-driven ADTs. In `reference_types.rs`, `Value` is `UInt`, `Bool`, `List(Vec<u128>)`, and `Tree(Leaf | Branch)`. `Cons` builds the vector. It is not a user-declared nominal constructor.

The part of the design a reader needs, and the card omits, is the next clause of that same paragraph: sharing is storage only, two references are two traversals, and pointer equality and cycles are outside this fragment.

### 4. High — six rings are drawn as if they were the frame limit

The rings are a view of at most twelve frames. `Snapshot::depth` is `frames.len()`. `stack_limit` is an independent field. The page’s own slider runs from 2 to 20 and defaults to 16. `app.js` already computes `overflowFrames` when the chronological vector is longer than 12, and the gauge badge can show `+N`. The prose never says the twelve slots are a window.

“6 Split-Arc Rings (`R1`..`R6`, 12 slots)” in the CEK table, and “12 Half-Ring Slots total” in the study, read as machine capacity. A reader who then hits the frame-limit slider, or a program that passes 12 frames, has been taught that the picture ran out of machine.

The outer fuel ring is a fourth ring-shaped object and a fair picture of `remaining_fuel`. Name it as the fuel picture once, so “ring” does not mean frame arc and fuel segment in the same paragraph.

### 5. High — the center dial is asked to mean more than it can point at

The ten variants in the HTML match the `Control` enum of `sherlock-k-a/1`: `Eval`, `Return`, `Push`, `Invoke`, `Enter`, `Bound`, `Cases`, `CaseResult`, `PrimitiveResult`, `Error`. That list is the right scope. The design’s proposed `Control.ScheduleNext`, and the helper controls of `sherlock-helper-microstep/1`, are not this enum. The status note treats Quint’s fuel counter as a different profile. Say “the ten controls of `sherlock-k-a/1`.”

Two abbreviations in the die column collide with frame readouts the inspector already prints:

| Badge on the dial | Control it stands for | Frame readout a reader already sees |
| --- | --- | --- |
| `RET` | `Control::Return` | `RET:UInt` for `Frame::Return` |
| `MATCH` | `Control::CaseResult` | `MATCH [?]` for `Frame::Match` |

`PRIM` for `PrimitiveResult` is only a shortening. `RET` and `MATCH` teach two different objects as one word. The stator metadata in `designer2_well.js` uses badge `RET` and label `RETURN`, and badge `MATCH` for `CaseResult`. The prose copies the badges into the sentence that is supposed to define the sectors.

The needle does not aim at “the active half-ring arc.” In `designer2_well.js` it has two angles: west for an even chronological index, east for an odd one. The index is the newest frame, clamped at 11. A radial needle from the center crosses the same hemisphere of every concentric ring. It selects slot A versus slot B. The active ring is a separate highlight. Write that. The current sentence describes a selection the geometry cannot make.

The same resolver parks `Done` on the Return sector and both `Rejected` and `Exhausted` on the Error sector. Outcomes are not control variants. After publication, `Snapshot.control` is `None` (the status of K’s `.K` in the type comment). The dial has no sector for empty control. Say that the lit sector is the control while a step is in progress, and that a finished run is an outcome.

“Rule 140 → `Rejected(error 4)`” skips a step. Line 140 fires when a push finds `frames.len()` already at `stack_limit`. It charges 1 and yields `Control::Error(4)`. The following step, line 82, publishes `Outcome::Rejected(4)` at charge 0. The design’s phrase “stack-limit breach is `Rejected(error4)`” is the outcome. The card’s arrow makes 140 look like the publication. Quint refuses with `StackBound`, not error 4. The page badge already says `sherlock-k-a/1`; the sentence about error 4 should name that profile too.

### 6. Medium — the fuel sentence is the right skeleton and the wrong completeness

“1 per charged rule and 0 for terminal publication” matches the K-A schedule in the design: one unit per machine-section rule; startup, final, and error publication cost zero. In this host, startup has already happened. The free publications a reader can see are line 80 (`Done`), line 82 (`Rejected`), and line 84 (`Exhausted`). The charge shown in the inspector is the right place to point.

Both documents insist that lookup, binding, argument append, validation, matching, equality, and copying are unmetered host helpers inside a charged step. Card 01’s “every step checkable in isolation” is still fair, because the snapshot is complete. It is not fair as a cost claim. Add one clause: the fuel counter charges the machine rule, not the helpers inside it. The outer fuel ring may stay, labeled as the picture of `remaining_fuel`.

### 7. Medium — card 01 omits `program`, and “no mutable cells” fights card 03

`Snapshot` also holds `program`. `control` is `Option<Control>`. The card’s inventory is otherwise the right list.

“No mutable cells” is the right claim about the language: the design’s source forms are literals, variables, let, checked primitives, conditionals, named calls, constructors, and ordered match. There is no assignment. It is the wrong claim to leave next to an in-place frame rewrite. The resolution a reader needs in one sentence: values are immutable and locals are replaced by installing another map; three rules overwrite the newest frame without changing depth (145, 149, 177).

`Right(op, value)` does not hold the string `"4 × ?"`. It holds a `BinaryOp` and the already evaluated left `Value`. The string is the die’s drawing of `Right(Mul, 4)`. The `Args` clause matches the type and the design (completed values, remaining expressions, caller map). Keep it.

The card never says the two facts the design puts first: arguments and fields run strictly left to right, and only the selected conditional or match body runs; a recursive call is a table lookup plus a saved return frame, not a program-specific rule and not an unrolling of the source. Fuel bounds the attempt. The source recursion does not have to be structurally decreasing. Those four clauses are the execution lesson. The JVM/Forth contrast can stay as one short sentence. It is doing work.

“Rule 149” is a line number in the pinned K source (`K_SOURCE_SHA256`), which `StepRecord::rule_start_line` documents as a line number rather than an executed-K ordinal. The status note’s “36 charged rule branches” and “113-case source catalog” are different counts. The inspector prints `Rule A102`. Gloss this once, on first use: “source line 149, shown in the inspector as Rule A149.” Then use one spelling.

### 8. Medium — card 02 does not teach memory

The design’s memory paragraph is the lesson, and the card does not tell it:

- The live environment is a whole map (`BTreeMap<Name, Value>` in the host; “full lexical map” in the design). Naming `BTreeMap` once is honest. The teaching word is map.
- Call entry saves the caller map on `Return` and installs a fresh parameter map.
- `Return` restores that map after the result type checks (line 185). A type mismatch is line 190, `Error(6)`, and does not pop.
- `Let` saves the outer map. Line 145 writes `Scope` into the same slot, inserts the binding, and evaluates the body at the same depth. `Scope` later restores the outer map (line 195).
- `If`, `Match`, `Left`, and `Args` also carry a map. `Right` and `Unary` do not.
- The pending `4` of `fact(4)` lives in the `Right` frame during the recursive call. It is not a mutable local named `n`. `n` is in the environment the callee’s `Return` frame saved.

The CEK environment row says the map is “displayed on each saved frame card.” `resolveFrameEnvText` in `app.js` borrows an enclosing frame’s bindings when the frame itself has none, and the comment there names `Right` as the example. Either show only maps the frame actually stores, or say the card borrows the enclosing map for `Right` and `Unary`. The current sentence makes the borrowed line look like a field of that frame.

“There is no addressable RAM in the semantics” is the right first sentence. The second paragraph should be the restore story. The image sentence should not be the second paragraph.

### 9. Medium — card 03 is a legend written as a paragraph

The bold lead-ins (“Chronological Outermost-to-Innermost Mapping … & Two 180° Split-Arcs…”, “Central 10-Sector Control Stator & Live Readout Lens”) name the diagram. They do not tell a reader what to look at. “Stator” and “live readout lens” are never defined. One clause is enough: a ten-sector dial at the center, one sector lit for the current control, plus a short text lens.

“Colored by its `reference_types.rs` semantic family” over-attributes the legend. That file has nine frame variants and no family enum. Cyan / amber / gold / violet is an editorial grouping on this page. Say “grouped here.”

Scope the color transitions:

- Amber to gold is `Left` to `Right` at line 149. `Args` stays amber across its in-place cursor step (line 177). `Unary` stays amber. Gold means “the left value is now in the frame.”
- Violet to cyan is only `Let` to `Scope` at line 145. `If` and `Match` pop. A matching arm also pushes a fresh `Scope` at line 220. That `Scope` was never a violet `Let`.
- The inspector legend “OP (Left / Right / Args)” drops `Unary`, which card 03 and study 10 section 1 include. Study 10 section 3 drops it again. Put `Unary` on the legend. If the legend has no room for a fourth clause, the gold-versus-amber split is the clause worth adding, because that is the change the reader can watch.

West and east need one definition the first time they appear: slot A is the older frame of the pair, drawn on the west arc; slot B is the newer, drawn on the east arc. “180°” can go. The split is the fact; the angle is the drawing.

### 10. Medium — tone and the masthead’s “universal”

The deck under the title is the right voice: “Choose a program. Follow its calls, inspect the saved state, and see how the result is computed.” Card 03 leaves that voice. It is a parts list.

The title and the H1 call Sherlock a universal recursion interpreter. The design’s “universal” is one of three unrealized realization choices, and no winner is selected. The admitted language is a strict first-order fragment. The status note is an A-profile host component: finite observations, H1 still excluding untrusted input, dedicated data rather than general ADTs, no circuit acceptance. The product name can stay. The deck should not inherit “universal” as a technical claim. A sufficient deck addition is the profile the badge already shows: `sherlock-k-a/1`.

Ring study 10’s “Less, but better” is not a description of card 03. The editorial change is fewer coined names and one causal story per card.

### 11. Low — small precisions that keep the table from drifting

- `push` at the limit does not push and then fail. The insert is skipped.
- Line 80 publishes `Done` only when `Return` meets an empty frame vector. A `Return` with frames still up is an ordinary charged return into `return_value`.
- `Frame::Return` checks `has_type` against the declared result type. “Return-type contracts” in the study is fair if the card says the check can reject (error 6).
- Wafers in the well are one plaque per chronological frame, the same index as the half-arc (`chronoIdx` even = slot A). The CEK phrase “3D Continuation Wafers” is fine once the rings paragraph has defined the index. Do not introduce wafers as a second memory.
- Study 10’s badge formula matches `app.js` (`N/12 SLOTS · FUEL p% · CTRL: KIND`). The static HTML placeholder omits fuel until the first sync. No prose change required. The study should stop saying `R1·A` is always `RET:UInt` and `R1·B` is always `4 × [?]`. That is `fact(4)` after the root test has failed and the left factor has been stored.

## Next iteration

Replace the three cards and the third column of the table. Do not add a fourth card. Keep the CEK table at four rows.

### Card 01 — How Sherlock executes

Sherlock applies one transition to one explicit state. A snapshot holds the program, a control or none, the environment, the frame vector, the frame limit, initial and remaining fuel, and an outcome. Depth is the number of frames. Each `step()` records the before state, the after state, the pinned K source line, and a charge. `run()` repeats that step. This page follows profile `sherlock-k-a/1`. The inspector’s “Rule A149” is source line 149 in that pinned file, not the 149th of the 36 charged branches.

There is no anonymous operand stack and no assignment. A half-finished calculation is a frame. `Right(Mul, 4)` stores the operator and the value 4; the die draws it as `4 × [?]`. `Args` stores the function name, the values already computed, the expressions still waiting, and the map to restore. Three source lines overwrite the newest frame without changing depth: 145 turns `Let` into `Scope`, 149 turns `Left` into `Right`, and 177 advances `Args`.

A call looks the name up in an immutable function table, saves the caller’s map on a `Return` frame, and evaluates the body in a fresh parameter map. It is not a private rule for that function. Arguments run left to right. Only the taken branch of `if` or `match` runs. Fuel charges the machine rule: 1 for a charged rule, 0 when `Done`, `Rejected`, or `Exhausted` is published. Lookup, binding, matching, equality, and copying sit inside the step and are not charged again. Fuel only bounds the attempt. A recursion does not have to get smaller to be admitted.

### Card 02 — How memory works

The semantics have no addressable store. A value is a `UInt128` with checked addition, subtraction, and multiplication, a `Bool`, a list of machine integers, or a tree of leaves and branches. Those lists and trees are the dedicated types of this reference. They are not yet a general declaration of nominal datatypes. Sharing, where an image has it, is storage only: two uses of one node are two traversals. Pointer equality and cycles are outside this language.

The environment is a map from names to values, a `BTreeMap` in the host. Entering a call saves the whole caller map on a `Return` frame and installs the parameter map. When the call returns, that frame puts the caller map back, after checking the declared result type. `let` saves the outer map on a `Let` frame. When the right-hand side returns, that same slot becomes `Scope`, the name is bound, and the body runs. `Scope` restores the outer map when the body returns. `If`, `Match`, `Left`, and `Args` carry a map too. `Right` and `Unary` do not. If a `Right` card still shows bindings, it is borrowing the enclosing map, not displaying a field of `Right`.

The `4` in `4 × fact(3)` sits in the `Right` frame for the whole recursive call. It is not stored by mutating `n`.

A future bounded image, Candidate B’s `MachineImage`, would keep code, data, environments, frames, and helpers in fixed tables, with an append-only data arena between canonical boundaries. That image is specified. It is not this page, and it is not a delivered circuit. The counters this inspector reports are the reference ones: frame count, remaining fuel, and charge.

### Card 03 — What the rings and the control core mean

Frames are stored newest first. “Saved frames, top first” is that order. The rings reverse it, so the oldest frame stays on ring 1 while newer frames appear inward and leave outward. A push does not move the root call onto another ring.

Each ring is two arcs of that reversed vector. Slot A, west, is the older frame of the pair. Slot B, east, is the newer. Six rings are a window of twelve frames, not a register file and not one ring per call. The frame limit is the slider, which starts at 16. Past twelve frames the gauge marks overflow. The machine has not run out of rings.

Color is a legend for this page, grouped from the nine frame variants. Cyan is `Return` and `Scope`. Amber is `Left`, `Args`, and `Unary`. Gold is `Right` only, so source line 149 is the amber-to-gold change. Violet is `If`, `Match`, and `Let`. Source line 145 is the violet-to-cyan change, and it is the only one. `If` and `Match` are popped. A successful `match` arm also pushes a fresh `Scope` that was never violet.

On `fact(4)`, the familiar picture — ring 1 holding `RET` and `4 × [?]`, rings 2–4 holding the same pair for 3, 2, and 1 — is the moment after each test has failed and each left factor has been stored, before `fact(0)` is entered. Eight slots, rings 5 and 6 idle. It is not the peak, and it is not the rule. At the base-case test `0 == 0` the window holds eleven frames: ring 5 is `fact(0)`’s `Return` plus `If`, and ring 6’s west arc is the equality. A tail call leaves only `Return`, so two calls share a ring. `Right` is never itself pushed.

The center is a ten-sector dial for the controls of `sherlock-k-a/1`, one sector lit while a step is in progress: `Eval`, `Push`, `Return`, `Invoke`, `Enter`, `Bound`, `Cases`, `CaseResult`, `PrimitiveResult`, `Error`. Read those names. The short badges `RET` and `MATCH` collide with the frame readouts `RET:UInt` and `MATCH [?]`. The needle points west or east, the hemisphere of the newest visible slot. It does not select one ring. `Done`, `Rejected`, and `Exhausted` are outcomes. After publication the control is empty. A push that finds the frame count already at the limit takes source line 140 to `Error(4)`; the next step publishes `Rejected(4)` at no charge. That code is this profile’s. Quint’s refusal is a different spelling.

The outer segmented ring is remaining fuel, not a seventh frame ring.

### CEK table

Keep the headers. Change the third header to “On this page,” so the die is not given the authority of the semantics.

| Sherlock part | Conventional analogue | On this page |
| --- | --- | --- |
| `Control` of `sherlock-k-a/1`: `Eval`, `Return`, `Push`, `Invoke`, `Enter`, `Bound`, `Cases`, `CaseResult`, `PrimitiveResult`, `Error`. Empty after publication. | Program counter plus the current instruction. The analogy stops at “where the machine is.” | The control carries the expression or the value, not an address. The center dial lights one of those ten names. The needle points west or east for the newest visible frame. Outcomes (`Done`, `Rejected`, `Exhausted`) are not sectors. |
| `Environment`, a map of names to values (`BTreeMap` in the host). | Locals. There is no fixed register set. | The live map is replaced wholesale. Frames that store a map keep a copy. `Right` and `Unary` do not store one; a card that shows bindings for them is borrowing the enclosing map. |
| `frames`, newest first. Nine forms: `Let`, `Left`, `Right`, `Unary`, `If`, `Match`, `Args`, `Return`, `Scope`. | A call stack. Many of these frames are pending operands, not activation records. | The inspector list is machine order. The six rings are the same frames reversed, two per ring, twelve at most, with overflow marked past that. Cyan, amber, gold, and violet are the legend above. Wafers in the well use the same chronological index as the arcs. |
| `remaining_fuel`, `stack_limit`. | A step budget and a frame guard. | Under `sherlock-k-a/1`, a charged machine rule costs 1 and publication of `Done`, `Rejected`, or `Exhausted` costs 0. Helpers inside a step are unmetered. The outer fuel ring draws remaining fuel. A push at the current frame count takes source line 140 to `Error(4)`, then line 82 publishes `Rejected(4)`. The slider, not the twelve arcs, is the guard. |

### Ring study 10

Replace section 2’s table. Keep the reversal bullets above it. Suggested rows for two captioned moments, not one “peak”:

Suspended multiplications, before `fact(0)` is entered (8 frames):

| Ring | Slot A (west, older) | Slot B (east, newer) |
| --- | --- | --- |
| R1 | `Return` of `fact(4)`, caller map | `Right(Mul, 4)`, drawn `4 × [?]` |
| R2 | `Return` of `fact(3)`, map `n = 4` | `Right(Mul, 3)` |
| R3 | `Return` of `fact(2)`, map `n = 3` | `Right(Mul, 2)` |
| R4 | `Return` of `fact(1)`, map `n = 2` | `Right(Mul, 1)` |
| R5 | idle | idle |
| R6 | idle | idle |

Base-case test `0 == 0`, after line 149 has rewritten the equality (11 frames):

| Ring | Slot A | Slot B |
| --- | --- | --- |
| R1–R4 | unchanged from the table above | unchanged |
| R5 | `Return` of `fact(0)`, map `n = 1` | `If` of `fact(0)` |
| R6 | `Right(Eq, 0)`, drawn `0 == [?]` | idle |

Add one line under the tables: `is_even` / `is_odd` leave only `Return`, and `sum_list` places `Match` and `Scope` on arcs. Delete “always” from “R1·A = RET:UInt and R1·B = 4 × [?]”.

In section 1, replace “Every element … Ruby … have been eliminated” with the user-facing surfaces that were actually cleared (`index.html` cards, inspector header, legend). Point the leftover “Ruby” strings at `SherlockStage.js` and `designer7_cinema.js` instead of claiming they are gone.

In section 3, add `Unary` to the OP legend, and split the OP swatch note into amber (`Left`, `Args`, `Unary`) and gold (`Right`).

## Consistency checks for the editor who applies this

- The same `fact(4)` moment is not called the peak in the HTML, the study, the wafers note, and the call-tile note. Study 09’s “9 frames” and study 10’s “8/12” need the same two-moment caption, or a pointer to it.
- Inspector legend, card 03, and the CEK frames row name the same three families and include `Unary`.
- Control is spelled with the enum names in prose. Badges, if they remain on the die, are defined as abbreviations once.
- `MachineImage` does not appear in the present tense.
- “Universal” does not appear as a claim about the admitted language.
- The first use of a source line says it is a pinned K line and shows the inspector’s `Rule A…` spelling.
- A reader can point at ring 1 during a later recursive step and say why it did not move.

## Sources read

- `index.html` masthead, inspector labels, `.prose-deck`, CEK table.
- `src/designers/ring_study_10_ui_prose.md` (entire).
- `src/designers/ring_study_01_compiler.md` §3–§4, for the trace this guide contradicted.
- `src/designers/ring_study_09_wafers_calls.md` §1–§3, for the conflicting “9 frames” peak.
- `src/app.js` ring gauge, overflow, frame-card environment borrow, legend metadata.
- `src/designers/designer2_well.js` stator sectors and needle angles.
- `reference_types.rs` `Value`, `Frame`, `Control`, `Outcome`, `Snapshot`, `StepRecord`.
- `reference.rs` `step`, `transition`, `evaluate`, `return_value`.
- `machine_image.rs` file header and `MachineImageDraft`.
- `docs/SHERLOCK-REFERENCE-STATUS.md`.
- `openspec/changes/sherlock-universal-interpreter-design/design.md`, especially “One machine,” “Helper microsteps,” “Values, environments and bounded machine images,” and the delivery table.
