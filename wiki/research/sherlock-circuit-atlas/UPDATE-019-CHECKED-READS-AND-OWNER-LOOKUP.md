---
id: moriarty.research.sherlock.atlas.update019
title: UPDATE019 Checked Reads and Owner Lookup
type: overview
status: active
created: 2026-10-20
updated: 2026-10-20
updated_at: 2026-10-20T12:41:29.760342+00:00
tags:
  - moriarty
  - sherlock
  - circuit-concept
sources:
  - src-22ee37d52f566663f20e
---

# UPDATE-019 — Checked Reads and Owner Lookup

**Date:** 2026-10-20
**Author role:** Muse Wonder 1.0 HIGH, designated concept author under the standing Opus outage fallback route. Earlier Opus labels in this log are historical; this chapter is Muse prose. It is append-only and rewrites no earlier chapter.
**Internal scope:** This header and the closing scope note are outside the visitor copy. The visitor copy below is written for the finished Moriarty Sherlock website, which goes live only when Sherlock is fully finished and accepted. It describes the completed interpreter as a place to explore, not work in progress.

## Finished visitor copy

Welcome beyond the guard gantry, to the reading desk of the Sherlock interpreter — the place where the finished machine answers the most ordinary question in computing: this name, this reference, what value does it actually mean?

On the long board from UPDATE-018 the run already moves left to right through typed values, bounded records, fenced yards, a single step joining before, event and after, and guards at every seam. This station does not replace that board. It plugs into its centre and lets you watch two readings happen slowly enough to see every joint: a checked read, where a small reference selects one row and unfolds its typed payload, and an owner lookup, where a name saved in an environment is followed to its cell and the value held there is returned. Follow either trace and you arrive back on the same complete board, with nothing merged and nothing hidden.

Sherlock keeps three ideas apart at this desk, and the drawing keeps them apart too. A reference is not the row it selects. A row is not the payload it exposes. And a displayed projection of that payload is not the checking work that made the reading safe. Hold those three apart and the whole station stays legible.

The thirty-nine semantic selector footprints remain the floor. Stand on any one and the board shows the lane, the register and the name beneath your feet. A footprint selects a meaning; it is not a chiplet underneath the floor, and a steady selection glow never sends anything down a lane and never counts as proof.

## How this station is drawn

The station continues the atlas language: luminous routes on dark layered boards in the Tron and printed-circuit tradition, rendered as an explorable Three.js scene on the finished site. Source is the unlit solid hairline with a path anchor. Every drawing here is labelled `PROPOSAL` where geometry goes beyond a declaration. Observation is a shallow tray with a tabular identifier and a flat rim. Evidence is a hatched static fill only where an independent result exists. Selection is glow and thicker stroke only.

The Moriarty wordmark stays flat, unlit and unextruded, in its existing sans at weight 600 with tight tracking. Identifiers and figures are tabular monospace and never reflow. Layout sits on the measured 4 px spine. Copper marks divergence only, and every divergence is named in copper rather than hidden.

## The reading desk, built up in sight

The scene extends the completed board of UPDATE-018 at its centre, keeping its six layers visible underneath. Over them it builds the reading desk in five explanatory stages. Each stage adds one kind of relation and locks it in; traces are never erased. These stages explain the finished system. They are not a timeline, and no part of the board is waiting or half-connected.

I have chosen this cumulative desk, rather than a single dense diagram of every read at once, because a checked read is a chain of small exact choices — which container, which row, which tag, which fields — and the visitor should meet each choice while the previous ones are still lit.

**Stage one — a reference arrives as two small facts.** At the left edge of the desk a reference lights as a pair: a kind bit and an index. The kind bit chooses the container — the constant table carried in the program, or the DATA carrier beside it. The index chooses nothing yet; it is only a number travelling beside its kind. The board draws the reference as a short hairline ending in an anchor, deliberately smaller than everything it is about to touch, so the eye learns the first lesson of the desk: the pointer is tiny, the responsibility is not.

**Stage two — the container shows its shape: capacity, active prefix, and the selected row.** Two long containers open. Each is drawn in three nested extents that never share a colour. Outermost is the container's full capacity, the whole padded carrier, drawn as a faint ruled rectangle. Inside it, brighter, is the active row prefix — the rows that count, ending at a hard end stop. Beyond the stop the padded lanes remain visible and are drawn plain, counted in tabular figures, and never selectable. When the arriving index is placed against the prefix, one of two things happens in full view: an index inside the prefix lights exactly one row, or an index at or beyond the end stop meets the rail and the read closes with no row selected. An index that lands in the padded capacity is treated exactly like an index beyond it. Room reserved is not content present, and the drawing refuses to pretend otherwise.

**Stage three — the selected row unfolds by its tag.** The chosen row rises as a tagged strip. Its tag sits at its head in tabular capitals — a word for an unsigned integer, for a boolean, for nothing-at-all, for a linked pair, for a leaf, for a branch, for a constructor carrying its type and constructor identifiers, or, in the constant table only, for a marker. Beneath the tag the row unfolds into labelled payload sockets, and here the desk shows its most deliberate habit: only the sockets belonging to that tag light. An integer row lights its integer socket and leaves its boolean socket dark at exact zero. A boolean row does the reverse. A constructor row lights its two identifier sockets and its field span; a pair, leaf or branch lights the span alone. The dark sockets are not missing and they are not errors — they are drawn, counted and held at zero on purpose, so that no value is ever invented from the wrong shape. What a row does not say, the board shows it not saying.

Beneath the unfolded row a narrower copper-outlined span marks the ordered fields the tag points to: a start and a length selecting a consecutive run of field entries, in order. Three lengths are now visible at once and never merged — the container's capacity, the active prefix, and this selected field span — each with its own extent line and tabular count, so a visitor can point to each and say which is which.

**Stage four — a name is followed home through the saved environment.** Behind the desk the environment stands as a saved wall of named cells, built by the let-bindings the run has already made, each cell engraved with its name and holding one value. A lookup begins when a variable name lights on the run's trace. The trace follows the name along the wall — past the cells it does not mean, without lighting their contents — to the one cell that name was bound to, and the value held there is lifted out and carried back to the waiting trace. The returned thing is the value itself, seated in the cell since its binding, not a fresh copy conjured at the wall. Two bindings of related names sit side by side on this wall — a name bound to a number, a later name bound by reading the first — and the stage replays the follow twice so the visitor sees that lookup is always the same motion: name, cell, held value, return. A name with no cell meets the end of the wall and the read closes; the wall never invents a value for a name it does not hold.

**Stage five — the two readings join the running board.** Both traces now flow back into the completed interpreter from UPDATE-018. The checked read delivers its unfolded payload — kind, the one lit payload socket, the dark sockets at zero, the constructor identifiers where they belong, and the ordered field span — into the hands of the step that asked for it. The lookup delivers its returned value into the same step. Guards at the seam light as each delivered thing is confirmed to be the kind that seam admits, the before panel fills, the event fires, the after panel fills, and the relation gate spans all three. The visitor can replay the desk stage by stage in reverse, folding the row back into its container and the value back into its cell, because the desk was built in exactly the order it can be taken apart.

## Two distinctions the desk refuses to blur

First, the raw marker lane and the held-value condition are drawn as two separate rails and never wired together. In the constant table a marker row carries a raw lane, and the desk shows that lane exactly as it stands. Whether a held value satisfies the interpreter's marker condition is a different question, asked on the second rail in its own colour. Lighting the raw lane never lights the second rail. Beside them, nominal sort validation stands as its own checkpoint with its own frame: knowing a row's kind as a pure classification is one thing, and the desk shows that classification; confirming that a constructor truly belongs to its named sort is the checkpoint's separate work, drawn separately and never borrowed from the classification's colour.

Second, a projection is a display, not a substitute for the checking instructions. At the side of the desk a shallow observation tray can hold an exported view of a finished read — kind, payload, identifiers and span, neatly flattened for the next consumer to look at. The tray is useful and the board shows it proudly, but the trace that matters runs underneath it: the full sequence of checking instructions that chose the container, tested the index against the active prefix, unfolded the tag and zeroed the inactive sockets. A later consumer that composes that whole instruction sequence inherits the checks. A consumer that merely copies the tray inherits only the display. The board lights the instruction trace more brightly than the tray, on purpose, every time.

## Alternatives considered

A single wall chart of every row, tag and socket was considered and set aside: it shows the finished desk honestly but teaches it badly, because capacity, prefix and span arrive at once and blur into one rectangle. A tray-first design, leading with the flattened exported view, was considered and set aside: it makes the display look like the mechanism and teaches exactly the confusion this station exists to prevent. An animated progress sequence, in which containers would fill as if being completed, was considered and set aside: the desk explains a finished system, and its stages are layers of explanation, not instalments of work.

The recommended arrangement is the five-stage reading desk above: reference, container, tagged row, environment wall, and the join back into the running board, each layer locked in before the next is revealed, with the UPDATE-018 board continuing underneath as its foundation.

## Accessible and reduced-motion form

The reduced-motion form is a flat rendering of the same finished desk on the same 4 px spine. The five stages become five stacked sections in reading order. Each section lists, in tabular text, the components it introduces, the exact extent of each container and span with its count, which payload sockets are lit and which are held at exact zero, and the full sentence written on every frame it adds. Trays become rows naming what was placed in them. The two distinctions are given as complete sentences, never as colour or position alone.

Keyboard order follows the build-up: reference, containers, tagged row and field span, environment wall, and the join into the running board. A screen reader announces, for example: 'Stage two. Container capacity, active row prefix and padded lanes shown as three separate extents. The selected index falls inside the active prefix; one row is selected.' No distinction is carried by hue, glow, hatching or motion alone; every visual distinction has a word-for-word counterpart in the flat form, and the flat form describes the same finished, fully connected interpreter as the scene.

## Scope

This is a concept chapter for the finished Moriarty Sherlock website, which goes live only once the full Sherlock goal is finished and accepted. It adds no site code, assets, shaders or deployment. Engineering status, limits and attribution are kept in the separate status note, not inside the visitor copy.