# DESIGN011 — Child binding and anchored return

- **Date:** 2026-10-19 UTC
- **Kind:** Creative concept companion for the future Sherlock circuit site. It adds no circuit-source iteration, website code, scene, asset, build or deployment. It runs alongside implementation and blocks nothing.
- **Author:** Opus 5.5 (`claude-opus-5-5`), the requested Opus HIGH concept author for this entry. Root records the returned identity, reply and note hashes separately after capture. Earlier companions keep their own authors: Designs 008/009 are Muse contributions, and Design010 is a separate Opus reply. This note does not claim any of them.
- **Launch condition:** Unchanged. The site goes live once the full Sherlock goal is complete and accepted.

## Continuity

The board language is unchanged:

- ink ground and a flat, unlit Moriarty wordmark;
- copper accent (`#9a3412` light, `#e8724a` dark), with no new hue;
- a measured 4px grid and tabular monospace labels that never reflow or abbreviate;
- Source, Idea and Status kept as visually distinct channels. Light means selection or decoration only. Only a dated, recorded result becomes a history card.

The Lt switchyard and the earlier bays stay where they are. This note adds a new east–west spine north of them, the **return spine**. Along it, one proved step is handed upward to the recursion that consumes it.

## Recommendation: the return spine

The spine has four board layers and one separate bench. In the cumulative Three.js assembly they mount in source order, and each layer is clickable through to its file and symbol.

### Layer 1: Statement ribbon (`sherlock_child.rs`)

- **Ribbon.** The child instance is drawn as one long pin header.
  - Pin 0 is the **binding**. It has a notched, keyed head so it reads by shape and never by colour alone.
  - Every statement lane follows in positional order, one pin per lane.
  - The pin count comes from the source constant at build time (`EXPECTED_STATEMENT_LANES + 1`), not from this note.
- **No-ellipsis rule.** The ribbon never collapses into a "…" glyph, a hash puck or a summary tile. The source says it plainly: "nothing is truncated or digested." Zooming out lowers mesh detail only, and the DOM table beside the ribbon always lists every lane.
- **Self-reference trace.** A silkscreen trace runs from the binding pin back to a file card for `sherlock_child.rs`, because "the child binding commits this file's own bytes."
- **Fence.** The nine relation-hash sources sit inside a dashed fence labelled *unchanged*. The child carrier is mounted outside that fence, matching the header: "this file is outside them, so `relation_hash` and the context are unchanged."
- **Carrier tray.** The witness carrier is a four-compartment tray: `PublicInput` outputs (one per positional lane), the binding, `pis`, and the SDK-returned `pi_skips`.
  - A plain gate labelled `ProofPreimage::check` stands in front of the prove socket. The source runs it first, with the real binding.
  - The tray carries a stamped plate reading `ledger 04c9 only`, because the carrier type is documented unstable.
  - A caption under the tray quotes the source: no instruction is evaluated here.

### Layer 2: Phase shelf (`literal_witness_native_tests.rs`, T1)

- **Sealed module.** `ChildKey` is drawn as a sealed black module with sockets only at its return boundary and no interior lights. The phase marker "cannot attest internal stages that `ChildKey` does not expose", so the scene shows none.
- **Shelf.** A shelf above the module holds create-once tiles in this order:
  - **Before prove:** `vk`, `instance`, `claim` and `child.phase.1-reconstructed.json`. All are written when reconstruction returns and before `prove` is called.
  - **After prove returns:** `proof`.
  - Each tile has a lock glyph, because each is written once with `create_new` and never rewritten.
- **Match bracket.** A bracket spans the pre-prove tiles and the returned child. It stands for the `pre_prove_files_match_returned_child` comparison: the promise that was written down, set against what came back.
- **Negatives variant.** A second, narrower shelf holds the three phase tiles of the frozen-negatives path: `1-reconstructed`, `2-joined` and `3-control-returned`. Three tamper probes rest at instance lanes 0, 1 and the last lane, the positions the source enumerates.
- **Population rule.** Tiles and probe outcomes populate only from the files of an actual recorded run. With no recorded run the shelf shows empty locked slots, and the scene never fabricates a tile.

### Layer 3: The splice (A102 child → A80 linked return child, `sherlock_link.rs`)

- **Two ribbons, one seam.** Two statement ribbons lie end to end: the A102 literal child, then the A80 return-publication child.
  - Where they meet, a seam bar carries the shared program and midpoint identity as through-pins.
  - The pins are placed by `LinkShape::offsets` and checked by `check_link_lanes`.
  - Which lanes cross the seam, and how each pin is labelled, is taken from final reviewed source, not from this note.
- **Genesis locator.** A small ring-shaped locator plate (`GenesisAnchor`, height 0) sits at the west end of the spine. It is drawn deliberately unlike the terminal anchor, so a visitor never confuses where the chain is located with where it is anchored.
- **Reading the seam.** The seam is the visual point of this layer: two separately proved steps agree on exactly the state they hand from one to the other.

### Layer 4: Parent bay and terminal anchor

- **Parent bay.** The recursive parent is a deep chassis bay east of the splice. Its sockets are labelled with the obligation names the source records: `keys_id`, roots, `trace_c`, accumulation, deferred pairing, and subgroup checks.
  - Both child proofs enter the bay as two separate cables. They are never merged before it.
  - Interior gates are drawn only from actual parent source. No interior is invented for this note.
- **Terminal anchor.** A solid block at the east end of the spine, labelled by its source file `sherlock_anchor.compact`. It is the one place where the return chain is fixed.
- **No loop.** No wire runs from the anchor back to the children, and no light travels along the spine. Proving is never animated.

### Separate rail: the K bench

- **Placement.** A light table runs parallel to the spine on its own rail. It is separated by a visible air gap with the silkscreen label *no wire*.
- **Vellum sheets.** The bench shows the finite K-state record as translucent vellum sheets: two window frames and three registered positions. Phase tick marks are counted from the record.
- **Registration.** Agreement is drawn as registration marks lining up. A slider overlays the sheets.
- **Rhyme, not a connection.** The bench echoes the splice, two windows meeting at a midpoint, but it is never cabled to it.
- **Caption.** The bench caption lists what this rail does not show, using the record's own no-claim list:
  - a finished Add;
  - terminal stutter;
  - charged A/B steps;
  - K work or fuel;
  - recursive or native proof;
  - a production endpoint;
  - host/K correspondence;
  - EARS closure.
- **Literal-proof history.** Literal-proof history cards stay in the A102 gallery and are not cross-wired to this rail or to the spine.

## Alternatives

- **B. Hash puck.** The child statement is collapsed into one glowing disc. This is rejected: it depicts the exact digest replacement the source refuses.
- **C. Proof tower.** The children are stacked vertically under the parent. It is dramatic, but it hides the seam and invites a recursion animation. Keep it only as a static overview thumbnail.

**Choose the spine**, with C as the overview and the ribbon's lane table as the deepest zoom.

## Interactive explanations

**"Every lane, by name."** Scrub along the ribbon. The DOM table follows the cursor, and the binding pin opens its file card. The control navigates only and computes no values.

> The step is handed up whole. First comes a binding to the exact code that carried it, then every value of its statement, in order. Nothing is summarised on the way.

**"Write it down first."** This is a three-step walk-through:
1. Reconstruction returns, and the key, instance, claim and phase tiles lock into place.
2. `prove` is called.
3. The proof tile locks, and the match bracket closes over both rows.

> Before asking for a proof, the system writes down exactly what it is about to prove. What comes back is checked against that record.

**"Meet at the seam."** Highlight the A102 ribbon, then the A80 ribbon, then the shared through-pins.

> Two steps, proved separately, agree on the exact state they pass between them. The parent takes both, and the anchor fixes the result.

## History cards (dated, separate gallery)

- **2026-10-19:** Child carrier source and phase persistence installed (source only).
- **2026-10-19:** A80/link module source delivered (source only).
- **2026-10-19:** Independent K trace equality over two windows.
- **Earlier:** P14 verify-only run ended without a verifier verdict. The card shows an empty verdict slot, neither a tick nor a cross, and no duration.

Later compile, check, proof and verification records each add their own dated card. A record never adds glow to a board.

## Motion, keyboard, reduced motion, mobile, fallback

- **Motion.** The camera dollies along the spine between layers (400 ms ease). The assembly control drops the layers in source order. Selection glow is static. Nothing pulses, flows or implies proving.
- **Keyboard.** Tab follows the spine:
  1. ribbon: binding first, then lanes, with arrow keys moving between lanes;
  2. carrier tray and check gate;
  3. phase shelf tiles and match bracket;
  4. splice: A102, seam pins, A80, genesis locator;
  5. parent sockets, then terminal anchor;
  6. K bench as its own group, reached last.

  Enter opens the source panel with file, symbol and line anchor.
- **Reduced motion.** Cuts replace the dolly. The layers are drawn fully assembled, and walk-throughs step only on keypress.
- **Mobile.** The spine stacks vertically and the ribbon becomes a paged lane table. Tap targets are at least 44px, and explicit zoom buttons are provided.
- **Static fallback.** Each layer gets an SVG plate and a DOM table that mirrors every glyph with source links. It needs no WebGL. Shape, notch, label and position always carry meaning alongside tone.

## Next source events

1. A reviewed, applied and compiled A80/link module mounts the splice and its seam labels.
2. Fenrir parent source fills the parent bay's interior from actual bodies.
3. A compiled terminal anchor and its harness bolt the anchor block to the spine.
4. Each actual check, proof or verification record adds a dated history card.

## Visitor copy (finished release)

```text
Every step Sherlock proves is handed upward whole. Its public instance opens with a binding to the exact code that carried it, then lists every lane of the statement in order — nothing summarised, nothing dropped. Before a proof is requested, the key, the instance and the claim are written down and sealed, and what comes back is checked against them. A literal step and the step that returns its result are joined at the seam they share; a recursive parent takes both proofs, and a terminal anchor fixes the outcome.
```

```text
The bench beside the boards is a different kind of record. It lays the machine's states over one another, position by position, and shows them registering exactly. That makes the trace legible and consistent; it is not a proof, and nothing on the bench is wired to the boards.
```

## INTERNAL: provenance and gap register (not visitor copy)

**Supplied scope.**
- I was supplied the child header (`sherlock_child.rs` lines 1–19 only), the full T1 patch, selected Root metadata, the atlas index, and Design010 with its corrections.
- I did not see:
  - the runtime guard, circuit or native bodies;
  - `sherlock_child.rs` beyond line 19;
  - the body of `sherlock_link.rs`;
  - the body of `sherlock_anchor.compact`;
  - any K trace or oracle body.
- Source metadata does not prove behaviour. This note is design only, not a compile, security or result opinion.

**Child and T1 (installed source only).**
- Root reports the five files installed at HEAD `40d4186894c93d120cf32e2ca1cb372dce902b6b`, with full manifest `f29043248788f83bf772cba5971aa572a46478183fd7afadffe80df69b1038e2`.
- Pins:
  - `sherlock_child.rs` 18,176 B `c193b5e5…d75738`
  - `lib.rs` 46,396 B `76a9e085…3c492c`
  - `Cargo.toml` 784 B `495dfdee…8bc54`
  - `literal_witness_native_tests.rs` 84,787 B `a0fd2f9d…b2a67`
  - `Cargo.lock` 117,174 B `946b097d…e3993`
  - T1 patch 12,696 B `6d674acd…70b9`
- T1 source authorship is a separate actual Opus source call, per its MATERIALIZATION record. It is not this note.
- No native or Cargo run for C4 occurred. The one-shot was not started, the execution root is empty and no cleanup is owed.
- No phase artifact, proof, verification or resource figure has been observed.
- Root's local publication commit `3dd96307` has push pending. Remote success is not inferred.

**Parent and terminal delivery.**
- At the historical broad cut, delivery was partial. It contained one complete source object, `sherlock_anchor.compact` (2,618 B, `16efe3b6…261746839`). The anchor compile and transcript template remain open.
- Still missing:
  - Fenrir parent prefix (`keys_id`, roots, `trace_c`, accumulation, deferred pairing and subgroup checks);
  - decider and terminal;
  - anchor plus existing-harness integration.
- The intended path A102 → A80 → parent → terminal anchor is a source contract, not an executed proof.

**A80/link (delivered source, unapplied).**
- `sherlock_link.rs`: 905 lines, 42,539 B, `2657a5d8…ac00da`. Inner JSON is `8db75e9e…fa67d6a`.
- Status is COMPLETE_SOURCE_UNCOMPILED. It is not applied, independently reviewed, compiled, keyed, proved or verified. Its `lib.rs` preimage and registration anchor are unverified.
- It was written by a separate actual Opus source-author call, not by this note's author.
- Its own unresolved list stands:
  - `anchor_genesis` returns a height-0 locator plus an Init host-subset check, not a root.
  - Init equations are constrained by neither child.
  - The A80 context hashes a caller-asserted `LiteralSourceIdentity`, which is a review item.
  - The `midnight-proofs` 0.8.3/0.8.2 closure is unestablished.
  - k, rows, sizes, times and memory are unmeasured.
- No complete, coherent recursive endpoint exists.

**K (finite windows, not proof).**
- Result: PASS_COMPLETE_INDEPENDENT_K_TRACE_EQUALITY at 2026-10-19T13:12:11Z. It covered 10 actual phases, 2 compiled windows and full three-position equality after the freeze `3d25bd0a…2d2`.
- This is K-only, uncharged, complete-state link equality. Its no-claim list applies in full.
- The editor read no protected oracle or trace body.

**P14 history.** Compiled. The verify-only run hit terminal exit 124 at about 300 s wall time, with verifier verdict NONE. No unchanged retry was selected, and the saved literal bundle is preserved. No duration is shown on the site.

**Current index (internal provenance only).**
- The historical cached build is 18,266 B (`25283ab3…`) and the current public one is 20,024 B (`389cca5b…`).
- A metadata-only rebind is insufficient, because the cached helper pins the historical A0 freeze and file map.
- The owner H1–H6/G1 proposal has three files, materialized under separate pins. It is unadopted and unexecuted. Fresh reviews are ready but not launched, and no Cargo current-index success has been observed.
- This is not a visitor flow and has no counterpart in the scene.

**Not claimed.** This note claims none of the following:
- a proof, verifier result or recursive relation implementation;
- an endpoint, performance figure or resource figure;
- host/K equivalence;
- stagenet or Fenrir-derived rollup results;
- that any source is machine-ready or that Sherlock is complete.

The full goal remains a Sherlock stagenet recursive proof followed by Fenrir-derived rollup k/performance.
