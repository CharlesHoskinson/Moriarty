---
id: moriarty.research.sherlock.atlas.update018
title: UPDATE018 Math Guards and Execution Layers
type: overview
status: active
created: 2026-10-20
updated: 2026-10-20
updated_at: 2026-10-20T11:25:15.020766+00:00
tags:
  - moriarty
  - sherlock
  - circuit-concept
sources:
  - src-67304c4a1660d522d9a0
---

# UPDATE-018 — Math, Guards and Execution Layers

**Date:** 2026-10-20
**Author role:** Muse Wonder 1.0 HIGH, designated concept author under the standing Opus outage fallback route. Earlier Opus labels in this log are historical; this chapter is Muse prose. It is append-only and rewrites no earlier chapter.
**Proposed destination:** `wiki/research/sherlock-circuit-atlas/` as a new linked continuation. That placement is proposed here, not claimed applied.
**Internal scope:** This header and the scope note at the end are outside the visitor copy. The visitor copy below is written for the finished Moriarty Sherlock website, which goes live only when Sherlock is fully finished. It describes the completed interpreter as a place to explore. It is not a report on work in progress.

## Finished visitor copy

Welcome to the layer room, where the Sherlock interpreter stops looking like a pile of parts and starts behaving like one connected machine.

Until now the atlas has walked you station by station: records padded whole, temporary names kept inside their own fenced yards, and a single step read as before, event and after. This station puts those boards together. You will watch a program enter at one edge of a long dark board, take shape as typed values and bounded records, pass through guarded execution layers, and arrive at the other edge as a finished run — every connection lit, every layer still readable on its own.

Nothing here merges into a blur. Sherlock keeps its kinds of knowing apart even in operation. A name is not a value. A value is not a record. A record is not a step. A guard does not do the work it guards; it checks the work as it passes. The pleasure of this board is that you can follow one bright trace all the way through and, at any point, say exactly what kind of thing you are looking at.

The thirty-nine semantic selector footprints of the atlas remain its floor throughout. Stand on any one and the board shows the lane, the register and the name beneath your feet. A footprint is a place to stand and a meaning to select. It is not, by itself, a claim that a separate physical chiplet exists beneath it. The board is honest in the same way at every scale: what you can point to is what the drawing is claiming, and no more.

## How this station is drawn

The station continues the language built across the atlas: luminous routes on dark layered boards in the Tron and printed-circuit tradition, rendered as an explorable Three.js scene when the finished site opens. Four channels stay visually distinct everywhere, and they never borrow one another's clothes.

Source is the unlit solid hairline with a path anchor. Idea — every drawing in this chapter — is labelled `PROPOSAL` where geometry goes beyond a declaration. Observation is a shallow tray with a tabular identifier and a flat rim: things placed in trays were seen, not derived. Evidence is a hatched static fill that appears only where an independent result exists. Selection is a steady glow and a thicker stroke only; a selected glow is not proof, and selection never sends anything down a lane.

The Moriarty wordmark stays flat, unlit and unextruded, in its existing sans at weight 600 with tight tracking. Identifiers and figures are tabular monospace and never reflow. Layout sits on the measured 4 px spine. Copper is held for divergence: where this station departs from the boards before it, the divergence is named in copper rather than hidden.

## The finished board: one interpreter, built up in sight

The scene is a single long board, read left to right. Instead of showing five separate stations, it builds one working interpreter in six explanatory stages. Each stage adds a layer of structure on top of what is already there and locks it in place. Traces are never erased as the board grows. By the final stage the visitor is looking at the complete connected system, with every earlier layer still visible underneath the operation it makes possible.

These stages explain how the finished system works. They are not a timeline. Nothing in the scene is waiting, pending or half-connected. At every stage the portion of the board that has been introduced is whole and internally connected; the next stage simply reveals the next kind of relation that the completed machine contains.

I have chosen this cumulative build-up, rather than a single finished diagram shown all at once, for one reason: a completed interpreter is hard to read in one glance, but easy to understand when each kind of connection arrives while the previous kinds are still on the board. The visitor learns the machine the way the machine itself is organised — values first, then the records that carry them, then the names that refer to them, then the step that changes them, then the guards that watch the change, and finally the whole run.

**Stage one — words become typed values.** The board begins almost empty: a dark field on the 4 px spine with entry pads along its left edge. As the stage opens, source hairlines feed named entry points and resolve into typed values, each drawn as a short luminous segment seated in a labelled socket. A value that carries a recursive shape unfolds once on the board, parent socket above and child sockets below, so a visitor can see that a tree-shaped value is one value with internal structure, not a crowd of unrelated parts. Nothing moves yet. This stage answers only one question: what is here, and what kind is it?

**Stage two — values take their places in bounded records.** Frames rise around the values. The seventeen-lane control frame from the previous station returns, entered through its paired `org` and `nxt` registers, with every unused lane driven plainly to zero and shown as deliberate copper. Data terraces continue underneath, their lanes extended by the derived apron where a start and a length are added. Selecting a tag lights the lanes that form uses and leaves the padded lanes visible, counted in tabular figures. The visitor now sees the second kind of relation: not what a thing is, but where it sits and how much room its record guarantees it.

**Stage three — names stay inside their fences.** Five enclosed yards open behind the record frame: one for the record itself and one for each composed block the atlas already knows — data, helper, metadata, and program-and-context. Each yard mints its temporary names only inside its own fence, with its own serial counter at its gate, all standing on a parent ground that is itself checked before any yard opens. An arriving name that already carries a reserved yard marker stops at the checkpoint rail and is refused, never quietly renamed. Two emitter paths are drawn and never merged: the retained path that carries each yard's serial forward across repeated visits, and the shorter single-visit spur that builds its emitter, uses it once, and ends. The board now has its third kind of relation: who may refer to what, and where a name is allowed to live.

**Stage four — one step joins before, event and after.** At the centre of the board the three panels of the boundary station lock into place as the working heart of the interpreter: a before snapshot, an event panel, and an after snapshot, spanned by a bounded relation gate with a hard end stop. When the board runs in this stage, a single trace enters the before panel, crosses the event, and leaves through the after panel, and the gate checks that the three agree. Beneath the panels the caption from the previous station remains, now reading as a property of the running machine: consistency of a supplied record is not authenticated provenance, and the single owner step on the far side of the ruled line carries its non-rollback marker. The visitor now understands the fourth relation: what it means for one step to claim that this before became that after.

**Stage five — guards take their posts at the execution layers.** Guard frames now descend over the connected board like a transparent gantry. Each guard sits across a lane or a layer boundary it is responsible for, drawn in its own steady colour from end to end of the path it watches. Math guards span whole frames, because their rules speak about whole records at once; narrower guards sit inside them where a single lane carries a tighter holding, drawn as a rule within a rule. Execution-layer guards stand at the seams between the record layer, the namespace layer and the step layer, checking that what crosses a seam is the kind of thing that seam admits. A guard never carries a value itself. Its trace lights only when the traffic beneath it satisfies the condition written on its frame in tabular capitals. The fifth relation is now visible: what is checked, where, and by a rule the visitor can read at the point of checking.

**Stage six — the whole interpreter runs.** In the final stage the entry pads at the left edge open together and a complete run crosses the board. Typed values seat themselves in bounded records. Records enter their yards under names minted on the spot and refused at fences when they arrive pre-marked. A before snapshot fills, an event fires, an after snapshot fills, and the relation gate spans all three. Guards light in sequence as each condition is met at its own post. The run leaves at the right edge as a finished record resting in an observation tray, stamped with its tabular identifier, and the evidence hatch appears only on the portions of the trace where an independent result exists. Selection still only glows. It sends nothing. The visitor can now replay the run stage by stage in reverse, lifting one layer of relation at a time, because the board was built up in exactly the order it can be taken apart.

Beneath the running board, out of focus, stand the paired 22-lane rulers from UPDATE-016 — admission above, layout below, never merged — and the DATA terraces, apron and gantry from UPDATE-015 continue underneath unchanged. Earlier stations are not replaced by this one. They are the lower storeys of it.

## Alternatives considered

A single static master diagram of the completed interpreter was considered and set aside: it shows the finished machine honestly but teaches it badly, because every kind of relation arrives at once and the eye cannot tell values from names from checks. A free-roam scene with no staged build-up was considered and set aside for the same reason; freedom to wander is not the same as a path to understanding. An animated progress sequence, in which parts of the board would arrive as if being completed one by one, was considered and set aside: this board explains a finished system, and its stages are layers of explanation, not instalments of work.

The recommended arrangement is the six-stage cumulative board: values, records, namespaces, the single step, the guards, and the whole run, each layer locked in before the next is revealed, with the earlier atlas stations continuing underneath as its foundation. Each separation the design keeps is a separation the drawing keeps, and each connection the drawing shows is one a visitor can point to and name.

## Accessible and reduced-motion form

The reduced-motion form is a flat rendering of the same completed board on the same 4 px spine. The six stages become six stacked sections in reading order. Each section lists, in tabular text, the components it introduces, the relation each component has to the sections before it, and the exact words written on every guard frame it adds. The final section is a complete component-by-component description of the running interpreter: every value, record, yard, panel and guard named in the order a run crosses it. Trays become rows naming what was placed in them. Guard conditions are given as full sentences, never as colour or position alone.

Keyboard order follows the build-up: entry values, record frame, yards and checkpoint rail, boundary panels, guard gantry, and the finished run. A screen reader announces, for example: 'Stage four, single step. Before snapshot, event, after snapshot. Relation gate spans all three panels. Consistency of a supplied record is not authenticated provenance.' No distinction is carried by hue, glow, hatching or motion alone; every visual distinction on the board has a word-for-word counterpart in the flat form, and the flat form describes the same finished, fully connected interpreter as the scene.

## Scope

This is a concept chapter for the finished Moriarty Sherlock website, which goes live only once the full Sherlock goal is finished and accepted. It adds no site code, assets, shaders or deployment, and it asserts no physical circuit beyond what its own evidence channel shows. Engineering status, limits and attribution are kept in the separate status note, not inside the visitor copy. This asynchronous log never gates, blocks or speaks for the native programme.