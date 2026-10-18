# Grok 4.7 xhigh Audit 01 — CEK Engine, Ring-Slot Provenance, Control Core

Auditor: Grok 4.7 xhigh Auditor #1 (Sherlock CEK Engine & Ring-Slot Provenance).
Pinned reference: `sherlock-k-a/1`, `K_SOURCE_SHA256 = b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41` (`reference_types.rs`).
Subjects: `src/engine/sherlockEngine.js`, `rust-runner/main.rs`, `src/designers/ring_study_01_compiler.md`.
Oracle: `crates/mori-core/src/sherlock/reference_types.rs`, `reference.rs`, and the image machine the runner actually steps (`image_reference.rs`).

Method: line audit against the frozen machine, then execution. Browser path: `compileAndRunInBrowser` + `buildRingTelemetry` on all eight presets (fuel/stack from each preset) and on `fact(6)` at fuel 400 / stack 24. Native path: installed `rust-runner/sherlock-runner` on Factorial (fuel 200, stack 16) and LexicalCube (fuel 120, stack 16). Both engines emit 145 factorial steps and 46 lexical steps, and they agree on the base-case index below.

## Verdict

The chronological coordinate system is right, and on every preset the in-place rewrite detector is sound. `R1` is the oldest pair and the live hole is `chronoFrames[depth - 1]`, which is `frames[0]` after the top-first reversal. Rules 145, 149, and 177 each light exactly one `REWRITTEN` slot, and that slot is the chrono top. Five presets produced zero false `REWRITTEN` marks.

Three gaps will mis-draw the next iteration. Past depth 12 the active hole is clamped onto `R6.B` and painted `HELD`. A `POPPED` slot is published as `IDLE` with a null origin. The study's sector rule column describes the control that a rule consumes, while `controlCore` publishes the control the rule left behind, so the dial and the ring are one transition apart. Separately, the browser fallback retargets `originExprId` on rule 145; the native runner keeps the Let expression, matching `ImageMachine`.

## What matches the reference

`Snapshot.frames` is top-first. `reference.rs` rule 135 does `frames.insert(0, frame)`; the browser does `frames.unshift`; `ImageMachine` does the same and records `frame_origins` in that order. `buildRingTelemetry` reverses both the current and previous stacks, so `chronoIdx 0` stays the root for the life of that frame. Pairs `(2r, 2r+1)` are `R1.A`..`R6.B`. The nine `Frame` variants land in the families the study names (`ENV` / `OP` / `GATE`). The ten sectors cover the ten `Control` variants, with the study's aliases `CaseResult → MATCH`, `PrimitiveResult → PRIM`, `Done → RETURN`, and `Rejected | Exhausted → ERROR`.

Native in-place retention matches `image_reference.rs` 337–392. Rules 145, 149, and 177 assign `frames[0]` and leave `frame_origins[0]` in place. Failed `If` / return-type tests (169, 190) do not pop. Rule 111 is `Error(3)`.

Measured pictures that match the study's tables, at this engine's indices:

| Program | Step index | Rule | Depth | Slots |
| --- | ---: | ---: | ---: | --- |
| `fact(4)` base case | 125 | 149 | 11 | `R1.A RET:UInt (root)`, `R1.B 4 * [?]`, `R2 (n=4, 3 * [?])`, `R3 (n=3, 2 * [?])`, `R4 (n=2, 1 * [?])`, `R5.A RET (n=1)`, `R5.B IF [?] (n=0)`, `R6.A 0 == [?] REWRITTEN`, `R6.B` empty |
| `is_even(4)` base case | 109 | 149 | 7 | `R1 RET+RET (root, n=4)`, `R2 RET+RET (n=3, n=2)`, `R3 RET (n=1) + IF (n=0)`, `R4.A 0 == [?]` |
| LexicalCube rewrites | 3, 7, 23, 33 | 145 / 177 | same | Let→Scope and Args→Args on the chrono top only |

The `(n=k)` notes on `Return` frames are the saved caller environment from rule 208, not the callee binding. That reading matches both traces. Unwind arithmetic in study §4.1 (1, then 1, 2, 6, 24) matches `Done(24)`.

Rewrite predicate (`sherlockEngine.js` 1785–1796), confirmed counts: factorial 13/13 rule 149; list fold 6/6 rule 149; tree fold 4/4 rule 149; parity 9/9 rule 149; lexical 3×145 + 1×177 + 3×149; underflow 177+149; stack trap 5×149; fuel 1×149. No other rule produced `REWRITTEN`.

## Findings

### F1. Depth above 12 hides the live hole on R6.B — high

`activeSlotIndex` is `min(11, depth - 1)` (`sherlockEngine.js` 1904). `R6` is forced active whenever `depth > 12` (1821–1825). Slots past chrono 11 are built and dropped. On `fact(6)` the maximum is depth 15 at step 179 (rule 135). Telemetry reports `overflowFrames = 3`, `activeSlotIndex = 11`, and both `R6` slots `HELD` and not top (`RET:UInt` and `1 * [?]`). The real top is chrono 14, `Left` of `[hole] == 0`. Push, rewrite, and pop flags for that hole never appear. Default `stackLimit` is 16, so this is inside the supported machine, not a hypothetical.

Next iteration: keep the twelve root-anchored slots for `R1`..`R6`, and add an explicit `trueTop` record `{ chronoIndex, kind, family, transition, shortLabel, fullLabel, originExprId }` taken from the unclamped chrono top. Set `isActiveRing` only from `isTopOfStack`. Drive the chip highlight from `trueTop` when `overflowFrames > 0`.

### F2. A popped slot is published as IDLE — high

On a pop, `buildSlot` reads the missing current frame and returns `kind: Idle`, `shortLabel: IDLE`, `fullLabel: Idle`, `originExprId: null`, `occupied: false`, with `transition: POPPED` (1777–1812). Factorial step 3 (rule 181) pops chrono 0 from `Args` to that empty payload. Step 15 (rule 153) does the same for a `Right` frame. Designers that read the slot label cannot show what left the ring. `ImageMachine::return_value` sets `control_origin` from `frame_origins[0]` before `remove_top_frame` (image_reference.rs 342–414), so the identity exists at the moment of the pop.

Next iteration: if `transition === 'POPPED'`, ghost `prevFrame` into `kind`, `family`, labels, and `originExprId`, and set `ghost: true`, `occupied: false`. Put `beforeKind` / `afterKind` on `REWRITTEN` slots the same way.

### F3. The dial and the study disagree about which control a sector names — high (spec), engine is internally consistent

Study §5 lists, under each sector, the rules that **consume** that variant. `RETURN` includes 145, 149, 153, 157, 161, 165, 169, 173, 177, 181, 185, 190, 195. `PUSH` includes 135 and 140. `EVAL` includes 102 and 108.

Both runners set `controlKind` from the **after** snapshot (`main.rs` 311–335, browser 1366). `buildRingTelemetry` maps that kind (1847–1876). The subline still names the rule that just fired. The register and the rule are one step apart.

Factorial, both engines:

| Index | Rule that fired | After control | Sector | Stack action | Depth |
| ---: | --- | --- | --- | --- | ---: |
| 0 | 131 Call(args) | Push | PUSH | EVAL | 0 |
| 1 | 135 Push frame | Eval | EVAL | PUSH | 1 |
| 2 | 102 Literal | Return | RETURN | EVAL | 1 |
| 125 | 149 Right operand | Eval | EVAL | REWRITE | 11 |
| 126 | 102 Literal | Return | RETURN | EVAL | 11 |

Rule 149 does not light `RETURN`. It leaves `Control::Eval` (`reference.rs` 234–237). Rule 135 does not light `PUSH`. It leaves the pushed `next`, here `Eval`. Rule 140 leaves `Error`, so the engine files it under `ERROR` (stack-trap trace: one rule 140, sector `ERROR`, action `ERROR`). The post-state reading is a faithful view of `Snapshot.control`. It is not the table in §5.

Next iteration: emit both `register` (after control, today's `variant`) and `fired` (the variant the rule consumed, or the rule id itself). Point the dial at one of them and rewrite §5 so the rule column matches that choice. Suggested fired-side map, from `reference.rs`: 145/149/161/165/177 → the step's after-control is `Eval`, so a "what just happened" dial should key off the rule id (`REWRITE` / `POP` / `PUSH`), which `controlCore.action` already does. Keep `action` as the stack axis. Stop describing `action` and `variant` as the same fact.

### F4. Browser rule 145 retargets the frame origin — medium, native path is clean

`reference.rs` 228–232 and `image_reference.rs` 344–348 assign `frames[0] = Scope` and do not touch origins. The browser pops and re-pushes (`sherlockEngine.js` 1153–1160), then `frameOrigins.unshift(controlOrigin)`.

LexicalCube rule 145, same steps in both traces:

| Index | Native `originExprId` (kept) | Browser `originExprId` (rewritten) |
| ---: | --- | --- |
| 3 | 0 `entry/` | 1 `entry/0` |
| 23 | 5 `fn[0]:cube_plus/` | 8 `fn[0]:cube_plus/0.1` |
| 33 | 9 `fn[0]:cube_plus/1` | 12 `fn[0]:cube_plus/1.0.1` |

Rules 149 and 177 keep the origin on both paths (binary expr, call expr). With `preferRust` the UI shows the native origin. The offline fallback highlights the value expression as the owner of the `Scope` slot.

Next iteration: `frames[0] = { kind: 'Scope', env: top.env }` and leave `frameOrigins[0]` alone. The saved env must stay the pre-binding environment; the active env receives the new binding. That split is already correct on both paths.

### F5. Non-Eval returns keep the wrong source highlight in the browser — medium

`ImageMachine::step` overwrites `control_origin` only when the successor is `Eval` (image_reference.rs 178–181). `return_value` first copies `frame_origins[0]` (line 342). On rule 185 the successor is `Return`, so the origin stays the consumed `Return` frame, which rule 208 stamped with the call expression.

First factorial rule 185 (index 131): native `controlOrigin` is expr 9, text `fact(n - 1)`, path `fn[0]:fact/2.1`. Browser `controlOrigin` is expr 6, text `1`. The browser updates `controlOrigin` only when `next.kind === 'Eval'` (1358–1360) and drops the frame origin on the pop.

Next iteration: before a pop, copy `frameOrigins[0]` into `controlOrigin`, then let an `Eval` successor overwrite it. Covers 153, 157, 173, 181, 185, and 195.

### F6. Args arity label is short by the open hole — medium, both runners

The frame exists only while one argument is in flight. That argument is in neither `evaluated` nor `remaining` (`reference.rs` 267–276). Both labels use `evaluated / (evaluated + remaining)` (`sherlockEngine.js` 1464–1467, `main.rs` 419–424).

Measured: `cube_plus(seed, 272)` shows `cube_plus(0/1 args)` then, after rule 177, `cube_plus(1/1 args)`. A one-argument call shows `sum_list(0/0 args)` for the whole spine. Short labels collapse to `ARGS:name`, so the ring chip hides the error and the detail line does not.

Next iteration: denominator `evaluated + remaining + 1`. Rule 177 detection does not depend on this: kind stays `Args`, the label still changes, and the `ruleLine === 177` arm is a backstop. Keep that arm.

### F7. `Right` and `Unary` slots report environment `root` — medium

`formatBindingsText([], 'root')` (`1757–1759`) treats a missing binding list as the root environment. `Frame::Right` and `Frame::Unary` carry no environment (`reference_types.rs` 105–106). Factorial step 125 therefore prints `R1.B 4 * [?]` with `envText: root`, the same string as the genuine empty caller env on `R1.A RET:UInt`. The `lhs = 4` fact lives in `detail` and is not what `envText` shows.

Next iteration: for `Right` and `Unary`, leave `envText` empty. Reserve `root` for a real empty `Environment` on `Return`, `Scope`, `Let`, `Left`, `If`, `Match`, and `Args`.

### F8. The runner drops `before.frames` — medium

The JSON `before` object is only `fuel` and `frameCount` (`main.rs` format string; confirmed on the binary: step 0 keys `['fuel','frameCount']`). `compileAndRunSherlock` fills `before.frames` by aliasing the previous step's `after.frames` (1644–1646). `buildRingTelemetry` also falls back to `steps[i-1].after.frames` (1771–1775). The stitch is correct for a contiguous trace, which is why F1–F3 measurements agree across engines. The alias means a later in-place edit of `before.frames` mutates the previous snapshot. `action: REWRITE` keys off the rule id (1898) and would still fire if the stitch broke; the slot flag would not.

Next iteration: emit `before.frames` as its own array of `{ index, kind, label, originExprId }` from `record.before`. Copy on the client. Keep the fallback.

### F9. Rule 111 is `Error(3)`; two captions say error(1) — low, typed path never reaches it

`reference.rs` 168 and `image_reference.rs` 281 return `Error(3)`. `main.rs` 94 names the rule `Var missing -> error(1)`. The browser machine returns code 1 (`sherlockEngine.js` 1085) and the same caption (line 206). The draft typechecker rejects an unknown variable before either machine runs (line 723), so none of the eight presets contain rule 111. The caption is still what the dial subline will show the day a trace carries 111, and the browser code would publish `Rejected(1)`.

Next iteration: caption and browser code both become 3.

### F10. Browser rules 169 and 190 pop frames the reference keeps — low, typed path never reaches them

`image_reference.rs` 337–341: failing if-tests and return-type tests keep every frame and the saved environment. The browser shifts the frame off before the test (`1211–1222`, `1242–1249`). The typechecker rejects a non-Bool condition and disagreeing branch types, so current programs do not hit this. A future untyped or residual step would draw a false `POPPED` and a short stack on `Error(5)` / `Error(6)`.

Next iteration: pop only on the success arms (161, 165, 185), matching `reference.rs` 250–287.

### F11. Study §4.1 names the base case as step 72 — doc defect

Both engines, fuel 200, 145 steps (indices 0..144): the configuration in the §4.1 table is step **125**, rule 149, after-control `Eval`. Depth 11 first appears at step 123, rule 135, while `R6.A` is still `Left` of `[?] == 0`. Step 125 rewrites that slot to `0 == [?]`. There is no indexing of this trace in which that picture is step 72. §4.4's `is_even(4)` picture is accurate and is step 109. §4.2's cons spine is the real nesting order (outer `Right` frames stay under the inner `Left`); the outermost call's full label at that moment is the F6 string `sum_list(0/0 args)`.

### F12. Smaller telemetry nicks — low

- `clampLabel14` runs inside the telemetry object (1709–1712). Tree-fold chips become `leaf7 branch …` and `[?] + sum_tre…`. `fullLabel` still has the text. Clamp at draw time.
- Sector 9 merges `Rejected(code)` and `Exhausted`. The step has `outcomeKind`; `controlCore` does not. Add `reason` (`exhausted` or the rejection code) without adding an eleventh sector.
- Unknown `controlKind` becomes sector 0 via `Math.max(0, indexOf)` (1878). Surface it as a distinct `variant: 'EVAL'` only when the kind is actually `Eval`.
- `==` in the runner's aux/2 path is materialized as `ArithmeticOp::Sub` (`main.rs` 580–582). The displayed primitive result is the Bool; the limb bytes are the difference. Out of scope for the ring map, in scope for the next runner pass.
- `frame.index` is K order (0 = top). `slotIndex` is chrono order (0 = `R1.A`). Designers must use `ringTelemetry`.

Heterogeneous `==` in the browser falls through to error 11; the reference returns `Bool(false)` (`reference.rs` 344). The typechecker requires equal types first (line 760), so presets do not reach it.

## Next iteration, in order

1. Add `trueTop` and stop marking `R6` active merely because the stack is taller than 12 (F1).
2. Ghost the previous frame on `POPPED`, and record before/after kind on `REWRITTEN` (F2).
3. Split `controlCore.register` from `controlCore.fired`, and correct study §5 so rule 149 is not a `RETURN` rule and rule 135 is not a `PUSH` rule under the post-state dial (F3). Replace "step 72" with step 125 (F11).
4. Make browser rule 145 a true `frames[0]` store that preserves `frameOrigins[0]` (F4). Copy the popped frame origin into `controlOrigin` on non-Eval returns (F5).
5. Fix the Args denominator in both runners (F6). Stop calling an absent `Right`/`Unary` environment `root` (F7). Emit a copied `before.frames` from the runner (F8).
6. Correct the rule 111 caption and the browser error code (F9). Pop on 169/190 only after the test succeeds (F10).

A single node check is enough to lock the regression: for every preset step, a rule in `{145, 149, 177}` has exactly one `REWRITTEN` slot and it is the chrono top; every other step has none; when `depth <= 12`, `activeSlotIndex === depth - 1` (or 0 when empty); factorial step 125 has `R6.A` label matching `0 ==`; `fact(6)` reports `trueTop.chronoIndex === 14` at the depth-15 step; browser and native `originExprId` on lexical steps 3, 23, and 33 are equal.

## Residual risk

The rewrite detector ors a rendered label difference with the rule id. It is sound on the eight presets because 145 changes `Let → Scope`, 149 changes `Left → Right`, and 177 changes the Args fraction. A later formatter that restyles a held top frame without a store to `frames[0]` would false-trigger on the label arm. Prefer the rule id plus a structural key `(kind, op, value summary, arg counts, originExprId)`, and treat the label as display only.
