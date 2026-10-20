---
id: moriarty.research.sherlock.atlas.update020
title: UPDATE020 ScalarTag Selectors and Checked Witnesses
type: overview
status: active
created: 2026-10-20
updated: 2026-10-20
updated_at: 2026-10-20T14:18:31.764890+00:00
tags:
  - moriarty
  - sherlock
  - circuit-concept
sources:
  - src-313f8f460833576ac2a6
---

# UPDATE-020 — ScalarTag Selectors and Checked Witnesses

**Date:** 2026-10-20
**Author role:** Muse Wonder 1.0 HIGH, designated concept author under the standing Opus outage fallback route. Earlier Opus labels in this log are historical; this chapter is Muse prose and rewrites no earlier chapter.
**Internal scope:** This header and the closing note are outside the visitor copy. The visitor copy describes the finished Moriarty Sherlock website, which goes live only when Sherlock is fully finished and accepted. It is not a claim about current source, native, proof or target status.

## Finished visitor copy

Past the reading desk of UPDATE-019, the board narrows to a selector gallery: the ScalarTag station of the finished Sherlock interpreter. Here a checked value, already read from its actual referenced row, meets one selected Tag mode and is routed — honestly, visibly — to its next shape. Nothing here is waiting, half-wired or pending. Every lane on this board is complete, and the station exists so you can walk one selection at a time and see why it lands where it lands.

Three separations govern the gallery, and the drawing never blurs them. The reference cursor does not move, even while helper work is done around it. Allocating nothing is not the same as doing nothing. And a flattened display of a checked read is not the checking itself. Hold those apart and the whole station stays legible.

The Moriarty wordmark stays flat and unlit at weight 600. Identifiers are tabular monospace and never reflow. The layout sits on the measured 4 px spine, copper marks divergence only, and the thirty-nine semantic selector footprints remain the floor: a footprint selects a meaning, it is not an implemented chiplet, and its glow never sends anything down a lane.

## How the gallery is built

The scene continues the cumulative luminous Tron and printed-circuit board as an explorable Three.js place, with the UPDATE-018 board and the UPDATE-019 reading desk still visible underneath. I have chosen a five-stage selector gallery over a single dense mode chart, because ScalarTag is a chain of small exact choices — which reference, which checked kind, which mode, which outcome — and each choice should be met while the previous ones stay lit.

**Stage one — the cursor stays pinned.** At the gallery entrance the reference cursor is drawn as a fixed copper pin seated in the board, with its origin marker beside it. Through everything that follows the pin never travels. Beside it, two counters are kept visibly apart: an allocation counter, held at exact zero for data nodes, data fields and environment cells, with no value-slot writes; and a work counter, which is free to rise. The board states the lesson in words on a small plate: zero allocation is not zero work. When the helper-work limit is already reached, the reads still happen first, in full view, and then the step closes as exhausted: its outcome alone moves to Exhausted for helper work, while the work, cursor, allocation, helper, data and every unrelated state lane stay exactly as they were, and no charge is taken. Otherwise one unit of work is charged, one helper visit is added, and only the helper phase, register and staged lanes change.

**Stage two — the checked read arrives whole.** From the reading desk a trace carries the selected row into the gallery, and it carries the full checking instruction sequence with it: container chosen, index tested against the active prefix, tag unfolded, inactive payload sockets held at zero. At the side sits the shallow observation tray holding the flattened exported view of that read. The instruction trace is lit more brightly than the tray, on purpose, every time. A later consumer that composes the whole instruction sequence inherits the checks; a consumer that copies only the tray inherits only a display.

**Stage three — eight modes on one selector rail.** A single rail carries eight mode tablets in tabular capitals: ADD, SUB, MUL, LT, CONS, BRANCH, TESTBOOL, TESTTYPE. Two further names, LEAF and NOT, are drawn on a separate spur belonging to neighbouring stations, present so the map is honest, but not selectable here. Exactly one tablet is lit at a time. Beneath the rail, three lane families are drawn in three materials and never merged: the immutable source, configuration and table lanes, which enter and leave identical; the independently supplied before-and-after state lanes, which a checker compares; and the live owner-record lane, drawn as a sealed envelope whose seal is checked for internal consistency — the event matching its snapshots — without any claim that the envelope proves who sent it.

**Stage four — the routing floor.** The checked kind meets the lit mode on an open floor where each outcome is a physical destination. ADD, SUB, MUL and LT accept an unsigned integer and step forward to the Second phase carrying that value; any other kind closes at Done with a staged primitive error. CONS accepts an unsigned integer forward to Second and closes other kinds at Done with the same error. BRANCH accepts a leaf or a branch forward to Second and closes other kinds at Done. TESTBOOL always closes at Done, staging a selector that says true, false, or not-a-bool. TESTTYPE always closes at Done, staging a yes-or-no answer about the expected sort. A wrong kind is never quietly converted; it is routed, named and staged where the visitor can point to it.

**Stage five — the TestType gate stands apart.** At the far end, TESTTYPE alone passes through a distinct nominal gatehouse. For ordinary kinds its answer is a simple sort match — integer, bool, list, tree — and a marker answers no. For a constructor, the gatehouse checks the nominal record itself: the named type, its constructor row, the owning type, the constructor range and the field end. A constructor that fails that nominal check is refused at the gate; it does not slip through as a mere no. Every other mode treats a constructor by kind alone and never borrows the gatehouse colour. Beyond the gate, both destinations — Second and Done — flow back into the running board, the cursor pin still exactly where it began.

## Alternatives considered

A rotary dial of modes was considered and set aside: it hides the unchosen modes and makes kind-only routing and the nominal gate look like the same motion. A full eight-by-every-kind matrix wall was considered and set aside: it is honest but teaches badly, because the fixed cursor and the work counters disappear into the grid. A tray-first design leading with the flattened view was considered and set aside: it teaches exactly the confusion this station exists to prevent. The recommended arrangement is the five-stage gallery above — pinned cursor, whole checked read, eight-mode rail, routing floor, and the separate TestType gatehouse.

## Accessible and reduced-motion form

The flat form renders the same finished gallery as five stacked sections on the same 4 px spine. Each section lists its components in tabular text: the cursor stated as fixed; allocation and value-slot writes stated as exact zero while charged work and helper visits are counted separately; the eight modes listed with their accepted kinds and their Second or Done destinations and staged results; the TestType nominal gate stated as a refusal, distinct from a no answer; and the three lane families named as immutable lanes, independently supplied before-and-after lanes, and the internally checked owner-record envelope with no provenance claim. Keyboard order follows the build-up. A screen reader announces, for example: 'Stage four. Mode ADD. Checked kind is unsigned integer. Route is Second, carrying the value. Reference cursor unchanged.' No distinction is carried by hue, glow or motion alone.

## Scope

This is a concept chapter for the finished Moriarty Sherlock website, live only once the full Sherlock goal is finished and accepted. It adds no site code, assets or deployment. Engineering status is kept in the separate dated note, not in the visitor copy.