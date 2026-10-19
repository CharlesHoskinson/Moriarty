# DESIGN008: The Actual Sub5 Composer and Audited Parent Backplane

- **Kind:** Creative concept companion only. It adds no circuit iteration.
- **Status:** This note is not site publication or implementation. The site launches only after the whole Sherlock goal is finished and accepted.

## Source pins

- `b_admin_publication.rs` — `fd96b0c381172c44bed233b09b7f7e934f2b59628b1177d3ea91baa5701505ad`
- `b_scalar_subtraction_completion.rs` — `bb08d2f25dcced0a4c12bb18b09a7898b20b91a21182e983114b5283f1db571d`
- `mod.rs` — `b26df00c78ea030c779f31105bb9f0fd393502faf5c715c4ea2ca332c558fcf4`
- `b_admin_publication_normal_tests.rs` — `ea5e09cd64cf353fe874dbfe79a5a9d3e139e48bf9d29717276366867d3d49a6`

## Continuity from DESIGN007

DESIGN007 drew the sixteen-byte borrow ladder, the staged value and errors, the completion fork, and a via-free typed handoff ribbon. At that older freeze, the wiring beyond the handoff was still an absent-wiring sketch: no Sub return-right, no P1 integration, and unregistered modules.

DESIGN008 keeps that ladder as the opening movement and extends the cumulative Moriarty Tron-like PCB and Three.js atlas to what the current source now contains: the actual Sub5 composer, `emit_b_sub_to_publication_chain`, its typed handoff, and the audited parent wiring. DESIGN007 remains truthful for its freeze and is not rewritten. What changes here is the board beyond the ladder — a real five-stage chain and a source-audited backplane that can now be depicted as connections in the source, rather than as a hoped-for exit.

Add and Sub remain distinct types even where their board silhouettes rhyme. The atlas preserves that distinction visually: shared geometry may suggest kinship, but keyed sockets and labelled lanes never imply a shared carrier.

## Visual alternatives

- **A. Five-station composer spine.** A single horizontal 4px spine carries five stations in source order. The borrow ladder opens the board, followed by the exit/failure partition and publish mux, the return-right and typed handoff, the common primitive value/error stage, and finally P1 publication. Keyed sockets mark where each station receives only the lanes its type allows.
- **B. Backplane-first board.** The parent wiring dominates: a rear backplane with three explicit Sub module sockets and twenty explicit named exports rendered as labelled edge connectors. The five stations sit as daughter boards plugged into it. This foregrounds the newly audited `mod.rs` wiring.
- **C. Branch-fan board.** The board centres on the exactly-one exit/failure partition. Three source-described branch traces fan out — value toward Halted, underflow toward Rejected, wrong sort toward Rejected — with per-branch complements drawn as guard rings around each trace.

**Recommendation: A, with B's backplane as its mounting layer.** The composer spine follows the actual staged source order, so a visitor reads the machine as it is composed. Laying it over B's backplane lets the parent wiring be shown honestly as existing source connection without letting the connector count upstage the computation. C's fan is retained inside station two, where the partition belongs, rather than becoming the whole board's claim.

## Cumulative assembly, interaction, and source clickthrough

The assembled board reads in five stations:

1. **Sub arithmetic.** Sixteen byte lanes run as ladder rungs, carrying the borrow from byte to byte. The staging socket holds three pads: the value, error 10, and error 11.
2. **Exit/failure partition.** Exactly one of exit or failure is selected, each with its complement and the publish mux. The cap-refusal path is drawn as a held pad preserving `Exhausted(DataNodes)`, its trace stuttering unchanged through the later stages.
3. **Sub return-right and typed handoff.** The return-right consumes RightF/Sub and the typed handoff. The handoff ribbon stays via-free and unlit in character: the typed adapters move the existing lanes unchanged and emit no instructions.
4. **Common primitive value/error.** Value and error arrive on their own lanes, still visibly typed.
5. **P1 publication.** At depth zero a value can reach Halted; error branches reach Rejected.

Interaction is selection, not simulation. Choosing a station, rung, pad, socket, or ribbon segment highlights that element and opens its source clickthrough: file path, symbol, and pinned SHA256. The three newly appended branch test bodies appear as source panels beside the board — 25−7→18→P1 Halted(Data(1)); 7−25→underflow10→P1 Rejected(10); Bool operand B→sort11→P1 Rejected(11) — presented as source text to read, never as a run result. Any glow is proposed decoration for selection only, never fabricated measured activity. The Moriarty wordmark stays flat and unlit, in copper and ink, with tabular labels throughout. There is no browser evaluator and no proving animation.

## Visitor copy (finished release)

> Subtraction enters the board one byte at a time. Sixteen lanes pass a single borrow hand to hand, each asking whether its byte covers what it owes. At the end of the ladder the answer settles into one of three seats: a clean difference, an underflow, or a value of the wrong sort. From there the board makes its one decision — exit or failure, never both — and the result crosses a connector that changes nothing it carries. Follow it through the return path, through the common value and error lanes, and out through publication. A finished value arrives halted, its data in hand. An error arrives rejected, its code intact. Every trace on this board leads back to the source that drew it: touch any lane, and the code behind it opens.

## Accessibility, DOM, and reduced motion

- **DOM equivalent.** A structured table mirrors the board: one row per station, one row per byte lane, and labelled outcome pads for value, 10, 11, Halted, and Rejected. Every interactive glyph has an equivalent DOM entry with the same label and the same source clickthrough.
- **Keyboard order.** Focus follows the source order — station one to five, byte 0 to byte 15 within the ladder — so the keyboard reads the machine as the eye does.
- **Not colour alone.** Borrow, refusal, and rejection are carried by notch glyphs and text labels as well as tone.
- **Reduced motion.** Nothing ripples or travels. Selection is a static highlight, layer changes cut rather than animate, and the cap-refusal stutter is shown as a fixed stepped trace, not a flicker.

## INTERNAL — Designer/provenance gap register

*This register is not visitor copy.*

- **Source-reviewed; new SDK tests UNRUN.** The three branch test bodies above are approved only at normal Rust/API/positive-branch source review for their three literal positive branches. Their SDK/evaluator execution is unrun.
- **Guard isolation not established.** The same review rejects claimed exact guard isolation: the three forged-lane negatives assert only some `ConstrainEq` rejection, not the exact intended guard or first failure. They must not be drawn as measured test success.
- **Older P9 is not the new thirteen.** Native P9 compilation covers the old W2 source and none of the new thirteen source hashes. It cannot light this board as executed.
- **Wiring PASS is source-only.** The parent wiring in `mod.rs` — three explicit Sub modules and twenty explicit named exports — passed independent Muse source review as source-wiring compatible, with no mandatory fix. That does not establish compilation. The P1/composer source-scoped APPROVE historically awaited wiring, compile, and full-chain tests; the wiring part is now supplied, the compile and test parts are not. The new test append is a separately pinned source overlay, not part of the earlier thirteen-source publication.
- **Typed completion is source, not a proof.** No claim of full Sherlock, constraint success, proof, native integration, deployment, or benchmark is made anywhere in this note.
- **Authorship.** Opus 5.5 was the requested author; an actual Opus outage led Root to select a fresh Muse under the standing fallback. This note is therefore an actual Muse contribution, not an Opus-authored one, and must never be relabelled. Any later Opus addition would be recorded separately, without overwriting this original text. This gap does not block implementation and creates no new approval or scheduler framework.