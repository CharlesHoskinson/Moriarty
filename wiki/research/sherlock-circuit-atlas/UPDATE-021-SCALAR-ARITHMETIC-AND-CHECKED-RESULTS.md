---
id: moriarty.research.sherlock.atlas.update021
title: UPDATE021 Scalar Arithmetic and Checked Results
type: overview
status: active
created: 2026-10-20
updated: 2026-10-20
updated_at: 2026-10-20T15:35:20.930358+00:00
tags:
  - moriarty
  - sherlock
  - circuit-concept
sources:
  - src-e7299984b7b3131318a9
---

# UPDATE-021 — Scalar Arithmetic and Checked Results

**Date:** 2026-10-20
**Author role:** Muse Wonder 1.0 HIGH, designated concept author under the standing Opus outage fallback route. Earlier Opus labels in this log are historical; this chapter is Muse prose and rewrites no earlier chapter.
**Internal scope:** This header and the closing note are outside the visitor copy. The visitor copy describes the finished Moriarty Sherlock website, which goes live only when Sherlock is fully finished and accepted. It is not a claim about current source, native, proof or target status.

## Finished visitor copy

Beyond the selector gallery of UPDATE-020, the board opens into the arithmetic room of the finished Sherlock interpreter. The Tag station has already done its work: a first value was read from its actual referenced row, checked, and — for the numeric modes — carried forward in the register. Here that cached value meets its second operand, or a lone Boolean is negated, and the exact answer is staged where you can inspect it before anything is built from it.

Five modes share this room, and the drawing keeps their reading paths visibly different. NOT arrives from the First phase and reads one operand only. ADD, SUB, MUL and LT arrive from the Second phase and read a second operand to meet the first. One room, two entrances, five exact outcomes — and no lane ever borrows another lane's operand.

The Moriarty wordmark stays flat and unlit at weight 600. Identifiers are tabular monospace and never reflow. The layout sits on the measured 4 px spine, copper marks divergence only, and the thirty-nine semantic selector footprints remain the floor: a footprint selects a meaning, it is not an implemented chiplet, and its glow never sends anything down a lane.

## How the room is built

The scene continues the cumulative luminous Tron and printed-circuit board as an explorable Three.js place, with the UPDATE-018 board, the UPDATE-019 reading desk and the UPDATE-020 selector gallery still visible underneath. I have chosen a split-rail arithmetic loom over a calculator face, because the lesson of this room is not keying numbers in — it is watching two differently-sourced values meet, watching the full mathematical result exist for a moment, and watching only its checked projection go forward.

**Stage one — the cursor stays pinned, the counters stay apart.** The reference cursor is the same fixed copper pin from the gallery entrance; it does not move in this room either. Two counters stand beside it, never merged: an allocation counter held at exact zero, and a work counter free to rise. The plate from the gallery still reads true here: zero allocation is not zero work. When the helper-work limit is already reached, the reads below still happen first, in full view, and then the step closes exhausted: its outcome alone changes to exhausted for helper work and no charge is taken, while the work, cursor, helper, data and allocation lanes stay exactly as they were. Otherwise one unit of work is charged and one helper visit is added.

**Stage two — NOT reads one operand and no other.** From the First-phase entrance, a single trace runs to the actual operand-A row and nowhere else. There is no operand-B reference on this path: no second socket is lit, no absent lane is interpreted as an address. If the checked value is a Boolean, its negation is staged as a Boolean value. If it is any other kind, a primitive error numbered 11 is staged instead. The phase closes at Done. A wrong kind is never treated as false; it is named, numbered and staged.

**Stage three — the numeric modes meet two checked sources.** From the Second-phase entrance, two traces converge on a meeting bridge. The first carries the cached register value, and beside it the bridge re-reads the actual operand-A row and binds the two together: the register must equal the unsigned integer that the checked read of its own source row produces. That binding belongs to this station and this selected step. The second trace reads the actual operand-B row. Only when both sources check out does the arithmetic core receive a pair of unsigned integers: the cached first value, proven against its source, and the freshly read second value.

**Stage four — the full result exists before the checked answer.** The core is drawn as three open workbenches, each retaining more than it forwards. At the ADD bench, the exact sum is formed and split at the 128-bit boundary; the carry beyond it stays on the bench as a visible copper tab. At the SUB bench, one comparison does double duty: it is both the borrow and the less-than answer, and the difference is reconstructed with the borrow before its own split. At the MUL bench, each operand is separated into 64-bit limbs, the partial products are laid out in a grid, and the complete high half of the product is retained on a raised tray before any projection. Then, and only then, the checked answer is taken: if the exact mathematical result lies inside the unsigned 128-bit range, it is staged as that value; if ADD overflows, SUB underflows, or MUL's retained high half is non-zero, a primitive error numbered 10 is staged. A wrapped residue is never presented as a success. LT takes a different exit on the same bridge: it stages a Boolean answer, true or false, and never an integer. Every applied branch — value, Boolean or error — closes at Done with the register itself preserved.

**Stage five — staged is not allocated.** Past the benches, the staged result rests on a lit shelf: a tag and a payload, nothing more. No data node has been allocated, no result has been issued, and the helper is still present. Two later stations, drawn downstream and deliberately unlit from this room's traces, do that separate work: the exit station turns a staged value into an allocated result, and the failure station turns a staged primitive error into its own distinct error result. This room ends at the shelf. What is staged here is exact and inspectable; what happens to it next belongs to those stations.

## Alternatives considered

A calculator keypad was considered and set aside: it suggests typed-in numbers and hides the two different operand-reading paths that this room exists to teach. A single formula wall listing all five modes as equations was considered and set aside: it states the mathematics but shows neither the retained carry, borrow and high half, nor the moment where the full result is refused projection. A residue dial showing wrapped field values was considered and set aside: it teaches exactly the confusion this room prevents, that a residue could pass as an answer. The recommended arrangement is the split-rail loom above — pinned cursor, single-operand NOT path, two-source meeting bridge, three workbenches retaining the full result, and the staged shelf before any allocation.

## Accessible and reduced-motion form

The flat form renders the same finished room as five stacked sections on the same 4 px spine. Each section lists its components in tabular text: the cursor stated as fixed; allocation stated as exact zero while charged work and helper visits are counted separately; NOT stated as reading operand A only, with no operand-B reference, staging a negated Boolean or primitive error 11; the numeric modes stated as binding the cached register value to the checked operand-A read before meeting the checked operand-B value; ADD, SUB and MUL stated as staging the exact result inside the unsigned 128-bit range or primitive error 10 outside it, with carry, borrow and high half retained before projection; LT stated as staging a Boolean; and the staged shelf stated as preceding, and distinct from, later allocation at exit and later error issue at failure. Keyboard order follows the build-up. A screen reader announces, for example: 'Stage four. Mode MUL. Full product formed. High half is non-zero. Staged result is primitive error 10. Nothing allocated.' No distinction is carried by hue, glow or motion alone.

## Scope

This is a concept chapter for the finished Moriarty Sherlock website, live only once the full Sherlock goal is finished and accepted. It adds no site code, assets or deployment. Engineering status is kept in the separate dated note, not in the visitor copy.