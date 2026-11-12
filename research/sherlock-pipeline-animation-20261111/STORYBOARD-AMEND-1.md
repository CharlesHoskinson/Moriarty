# Storyboard amendment 1: off-chain and on-chain

This file amends `STORYBOARD.md`. Where the two differ, this file wins. It makes one thing plain in every frame: what runs off-chain on the user's machine and what is on-chain on Midnight. The format, the register and the rules of `STORYBOARD.md` section 0 and of `SLOP-RULES.md` apply. Every title, narration and label below is final.

Running time: 14 beats change length (80 s more) and one beat is new (56 s). The piece goes from 2,738 s to 2,874 s (47:54), with 116 beats.

## 1. The location system

Two names are used everywhere and nowhere varied: **Off-chain: your machine** and **On-chain: Midnight**. The short forms, where space is tight, are **Off-chain** and **On-chain**. The crossing is **Crossing: the transaction**.

### 1.1 Which beat is where

Each beat carries a field `where` with one of three values: `'off'`, `'cross'`, `'on'`. The scene files set it; the shell reads it.

| Beats | `where` | Reason |
|---|---|---|
| Scenes 1 to 5, all beats | `'off'` | Compiler, native run, leaf proofs, node proofs and the decider seal run on the user's machine. |
| 6.1, 6.2, 6.3 | `'off'` | The wallet builds the transaction on the user's machine. |
| 6.4 | `'cross'` | The transaction leaves the machine. |
| 7.1 to 7.6 | `'on'` | `acceptRun`: verify, look up, store, refuse. |
| 7.7 to 7.20 | `'off'` | A later run's seal and the foreign-proof path are proving work. They stand on `ptray`. |
| 7.21 | `'on'` | The stored record. |
| 8.1, 8.2, 8.3 | `'on'` | Midnight nodes verify; the claim is a fact. |
| 8.4, 8.5 | `'off'` | JOIN and the shape digest are computed in node proofs. |
| 8.6, 8.7 | `'on'` | The collection settles in the contract. |
| 8.8 | `'on'` | The pull-back ends at the claim. |
| 8.9 (new) | `'on'` | The inventory; its card shows both sides. |

### 1.2 Shell element: the location chip (owner: `shell.js`, `pipeline.html`, `m3.css`)

- **What.** One M3 assist chip, `#whereChip`, inside the stage at its top left, directly to the right of `#stageBadge` with an 8px gap; on a stage narrower than 480px it sits under the badge with a 4px gap. Height 32px, corner `--md-sys-shape-corner-small`, 8px left padding, 16px right padding, an 18px Material Symbols Rounded icon and a `label-large` text. It is real DOM text with `aria-live="polite"`, so a change of location is announced.
- **States.**

| `where` | Icon | Text | Container role | Text and icon role |
|---|---|---|---|---|
| `'off'` | `computer` | Off-chain: your machine | `--md-sys-color-secondary-container` | `--md-sys-color-on-secondary-container` |
| `'cross'` | `send` | Crossing: the transaction | `--md-sys-color-primary-container` | `--md-sys-color-on-primary-container` |
| `'on'` | `hub` | On-chain: Midnight | `--md-sys-color-tertiary-container` | `--md-sys-color-on-tertiary-container` |

- **Change.** When `where` changes, the container colour and the text cross-fade over 300 ms with `--md-sys-motion-easing-emphasized`. In stepped mode the change is immediate. The chip never moves.
- **Narration card.** The line under the beat title ("The node, scene 4 of 8, beat 1 of 17") gains the short location name as its first item: "Off-chain. The node, scene 4 of 8, beat 1 of 17".
- **Scene rail and scene chips.** Two section labels in `label-medium`, `--md-sys-color-on-surface-variant`: "Off-chain" above Program, and "On-chain" above Contract. Transaction stays in the first group. On the phone layout the scene chip row gets the same two words as non-interactive text before the first and the seventh chip.
- **Scrubber.** A 4px band under the track, full width, in three runs that follow the timeline: `secondary-container` for `'off'` time, `primary-container` for `'cross'`, `tertiary-container` for `'on'`. The band is built from the beats' `where` values, so the off-chain stretches inside scenes 7 and 8 show on it.
- **Colour key.** Two entries are added after "Refusal": "Off-chain" with a `--md-sys-color-secondary` dot and "On-chain" with a `--md-sys-color-tertiary` dot.

### 1.3 Stage: two zones and a boundary (owner: `shell.js`, `theme.js`, `m3palette.js`)

Two canvas values are added to `M3_CANVAS` and mapped in `PALETTE`: `zoneOff` (the secondary role, tone 80) and `zoneOn` (the tertiary role, tone 80). The zone colours are structural. They appear only on the bench rail, the boundary gate and the rim of an off-chain tray. No token, cell, slab or label chip ever takes them, so `zoneOn` cannot be read as the outcome family.

- **Ground.** As built: stations 1 to 5 and the left half of station 6 on `paper` with `paperWarm` slabs; the right half of station 6 and stations 7 and 8 on `wellSlate` with `graphite` slabs.
- **Rail.** The shell's front rail (0.35 by 0.45 in section, at z 7.2) is split at world x 200 (station 6, local x 0). The off-chain part uses a material of colour `PALETTE.zoneOff`; the on-chain part uses `PALETTE.zoneOn`. Roughness 0.6, metalness 0.1, no emissive.
- **Boundary gate.** At world x 200 the shell draws a gate across the bench: a floor strip 0.4 by 0.08 by 14 at y 0.04, split along its length (the -x half `zoneOff`, the +x half `zoneOn`, each 0.2 wide); two posts 0.3 by 7 by 0.3 at z -7 and z 7; a lintel 0.3 by 0.3 by 14.3 at y 7. Posts and lintel are `materials.titanium()`. There is no translucent plane. The gate is always drawn; it is in frame in scene 6 and in beats 8.8 and 8.9.
- **Zone labels.** The shell shows two labels anchored to the top of the front post: "Off-chain" 2 units to the -x side and "On-chain" 2 units to the +x side, kind chips in the two zone colours. They show whenever the gate is inside the frame and they do not count toward a scene's six labels.
- **Off-chain trays on on-chain stations.** A scene that shows proving work at station 7 or 8 puts it on a tray with a `paperWarm` floor and a rim of `PALETTE.zoneOff`, and shows the label "Off-chain" at the tray's back left corner for as long as the tray is up. This applies to `ptray` in scene 7 (beats 7.7 to 7.20) and to a new tray `colTray` in scene 8 (section 2).

### 1.4 Kit addition: the decider verifier key card

The proving keys (leaf, node, decider) are the gold key cards of the kit. They stay off-chain in every scene. The one key on-chain is a different object: the **decider verifier key card**, a kit key card at scale 0.6 inside a titanium frame 0.12 thick, with one ink notch. It appears beside `door` in scene 6, as `vk` in scene 7 and behind each node in scene 8. No gold proving key card is ever drawn on the on-chain side.

Scene 6 no longer draws its own threshold `bar`: the shell's gate replaces it.

## 2. Changed beats

For each beat: the owner file, the first words of the old narration, the changes to objects and labels, and the complete new narration. Durations follow the rule of section 0.1. A beat that gets longer keeps the start times of its motions; the added seconds extend the final hold. Every beat also gets its `where` value from section 1.1. In the Labels lines of this section, backticks only quote the label text: write as `<code>` only the identifiers of `BRIEF.md`.

### Beat 1.1: `source-and-input` (22 s becomes 29 s)

**Owner.** `scenes/scene1.js`.

**Old narration begins.** "Moriarty source enters with an input"

**New narration, complete.**

> The work starts off-chain, on the user's machine or on a proving service the user picks. Moriarty source enters with an input value named input. The program saves that value, calls lift to add increment and compares the result with ceiling. A conditional selects the next value. Further named calls create intermediate values, and the selected branch then adds the saved input. The main run passes every arithmetic and resource check.

### Beat 2.1: `machine-arrival` (23 s becomes 28 s)

**Owner.** `scenes/scene2.js`.

**Old narration begins.** "`mori simulate` receives the ROM program,"

**New narration, complete.**

> The run executes off-chain, as native execution on the user's machine. `mori simulate` receives the ROM program, the input value and the resource policy that `input_d` binds. It runs a CEK-style machine, which keeps the current computation, the variable bindings and the saved continuations explicit. The reference machine `sherlock-k-a/1` defines every transition. The run produces the detailed evidence that the leaf checks. The simulator's own record certifies nothing.

### Beat 3.1: `tape-to-slices` (23 s becomes 34 s)

**Owner.** `scenes/scene3.js`.

**Old narration begins.** "The witness tape arrives with its"

**New narration, complete.**

> Leaf proving runs off-chain. The Sherlock chip is this leaf circuit with its proving key, and it lives on the prover's machine and never runs on Midnight. The witness tape arrives with its Halted outcome frame attached. The leaf cuts the tape into slices of `S = 16` step slots, each entering the same circuit under one leaf key. A full slice uses every slot. A shorter slice keeps its unused suffix inactive, so state holds still and the proof covers the recorded steps.

### Beat 3.35: `leaf-slabs-handoff` (24 s becomes 29 s)

**Owner.** `scenes/scene3.js`.

**Old narration begins.** "The checked slices leave as leaf"

**Objects and motion.** Before the handoff slide, at 4.4 to 6.0: three verification sweeps run at the same time, one over each of the three slabs at `dock` (parallel work is the subject of these 1.6 s). The handoff slide and the camera key keep their times.

**Labels.** Replace `Step intervals` with `CPU cores` | on: the three slabs at `dock`, from 4.4 to 8.0 | kind: neutral. After 8.0 show `Step intervals` again.

**New narration, complete.**

> The checked slices leave as leaf proof slabs, each with its seven public pins. Each proof binds the executed rules, authenticated data and initial and final state of its slice. Neighboring slabs carry the endpoint digests and step intervals that linking needs. Independent leaves let the machine prove them in parallel on its CPU cores. The handoff plate takes the slabs to the node, and `prog_d` and `input_d` stay with the run.

### Beat 4.1: `leaf-arrival` (22 s becomes 27 s)

**Owner.** `scenes/scene4.js`.

**Old narration begins.** "Leaf proofs arrive with their authenticated"

**New narration, complete.**

> Node proving runs off-chain as well, on the same machine or proving service. Leaf proofs arrive with their authenticated public values. The node treats each proof as a segment, a consecutive interval of the run. Its self-recursive circuit verifies leaf proofs and proofs of its own circuit under pinned keys: keys that the verifier fixes. The circuit size setting is `k = 20` throughout the tree.

### Beat 5.1: `root-arrival` (21 s becomes 24 s)

**Owner.** `scenes/scene5.js`.

**Old narration begins.** "The root slab and its accumulator"

**New narration, complete.**

> The decider is the last off-chain step. The root slab and its accumulator enter the decider, the circuit that completes verification and publishes the run statement. It verifies the root proof under pinned node keys and authenticates its boundary values and carry. With pinned keys, no prover can swap in a node circuit that accepts an invalid run.

### Beat 5.13: `publish-only-the-statement` (24 s becomes 29 s)

**Owner.** `scenes/scene5.js`.

**Old narration begins.** "The decider publishes exactly `[prog_d, input_d,"

**New narration, complete.**

> The decider publishes exactly `[prog_d, input_d, outcome_d, n_steps]`. No state digest, memory root, tail digest, memory length or address becomes public. Those fields describe physical execution boundaries and storage. Leaf cuts choose which boundaries appear inside the proof. The private input, the witness stream and the execution trace stay on the machine. With those fields outside the public statement, the claim is independent of those cuts and of the value's storage layout.

### Beat 6.1: `pack-the-call` (30 s becomes 32 s)

**Owner.** `scenes/scene6.js`.

**Old narration begins.** "The prover places the decider proof"

**New narration, complete.**

> Still off-chain, the wallet on the user's machine places the decider proof and its four public values in a Midnight transaction, an envelope submitted for ledger validation. `prog_d` is the program image digest, and `input_d` binds the input value digest, the fuel and the stack limit. `outcome_d` is the outcome digest under `sherlock-value/1`, the canonical value encoding. `n_steps` gives the machine step count. The envelope calls `SherlockRun.acceptRun`. Its proof is about 6.4 KB, whatever the length of the run.

### Beat 6.3: `keep-the-contract` (20 s becomes 27 s)

**Owner.** `scenes/scene6.js`.

**Old narration begins.** "The contract receives `prog_d` as call"

**Objects and motion.** The key rack with its three proving key cards moves to the off-chain side: rail centre (-9, 4.2, -5.4), and no connector leaves it. On the on-chain side, one decider verifier key card (kit addition, section 1.4) stands on a post at (13, 3.0, -2.6) beside `door`. The one connector from the call-data position still crosses the boundary to `door`; a second, short connector joins the verifier key card to `door`. The three connectors from the rack to `door` are deleted.

**Labels.** Replace `Shared key set` | on: the gold key rack with two labels: `Proving keys stay` | on: the rack | kind: state, and `Decider verifier key` | on: the card beside `door` | kind: state. Five labels in all.

**New narration, complete.**

> The contract receives `prog_d` as call data, a value carried by the invocation. One contract and one decider verifier key, deployed once on Midnight, serve every program and every input. The program identity selects no contract and no verifier key, and a new program deploys nothing. The leaf and node keys never go on-chain. The proof binds the supplied program digest to the execution it proves.

### Beat 6.4: `cross-to-slate` (20 s becomes 30 s)

**Owner.** `scenes/scene6.js`.

**Old narration begins.** "The prover pays the transaction fee"

**Objects and motion.** Add a row of four objects that stay, on the off-chain side at z -3.2, built by this scene: a leaf housing with its lid (scene 3's `lid` and board outline at scale 0.12) at (-12, 0.3, -3.2); the key rack of beat 6.3 behind it; an input cube (0.6, `family('input')`) at (-8.5, 0.3, -3.2); a tape strip of 4 frames at scale 0.5 with a break mark at (-5.5, 0.1, -3.2). They appear at 0.4 and never move. When the envelope starts its slide at 6.0, a titanium stop bar (0.2 by 0.5 by 4) rises in front of the row at (-3.2, 0.25, -3.2) (6.0 to 6.6). The follow camera leaves the row behind: it is out of frame by 8.0, which is the point of the shot. The envelope passes through the shell's boundary gate at local x 0.

**Labels.** The six labels of this beat are now, with times: `DUST` | on: the fee disc | to 6.0 | kind: neutral. `Stays: Sherlock chip` | on: the leaf housing | to 8.0 | kind: neutral. `Stays: proving keys` | on: the rack | to 8.0 | kind: state. `Stays: input, witness, trace` | on: the tape strip | to 8.0 | kind: input. `Crosses: decider proof` | on: the seal in the envelope | from 6.0 | kind: state. `Crosses: four public values` | on: the socket row of the envelope | from 6.0 | kind: neutral.

**New narration, complete.**

> The prover pays the transaction fee in DUST, Midnight's fee resource. Two things cross from off-chain to on-chain, carried in the call to `SherlockRun.acceptRun`: the decider proof and the four public values `[prog_d, input_d, outcome_d, n_steps]`. Everything else stays on the machine: the Sherlock chip, the leaf, node and decider proving keys, the private input, the witness stream and the execution trace. The contract needs none of them to check the run claim.

### Beat 7.1: `check-the-installed-key` (25 s becomes 33 s)

**Owner.** `scenes/scene7.js`.

**Old narration begins.** "The transaction reaches `acceptRun`, the contract"

**Objects and motion.** `vk` is the decider verifier key card of section 1.4 (the same object as beside `door` in beat 6.3), not a proving key card.

**Labels.** Change `Verifier key` to `Decider verifier key`.

**New narration, complete.**

> From here the work is on-chain. The transaction reaches `acceptRun`, the contract operation that accepts a proved run. It checks the decider proof against the contract's verifier key, the public key that defines the accepted proof relation, and the call transcript. A failed check refuses the call, so a sender cannot pass off a proof of an unrelated relation. The same decider verifier key serves every program and input. The contract verifies this one proof and never runs the Sherlock chip.

### Beat 7.4: `store-the-outcome` (22 s becomes 29 s)

**Owner.** `scenes/scene7.js`.

**Old narration begins.** "For a fresh `claim_d`, the contract"

**New narration, complete.**

> For a fresh `claim_d`, the contract stores `outcome_d` and `n_steps` under that key. These are the values the verified proof binds, so the record preserves both the result identity and the authenticated step count. The contract stores no state digest. One proof verification, the computation of `claim_d`, one lookup and one insert are the whole on-chain work of a call. Memory roots, tail digests, memory lengths and addresses remain inside the proof.

### Beat 8.1: `check-at-each-node` (23 s becomes 26 s)

**Owner.** `scenes/scene8.js`.

**Old narration begins.** "The stored claim reaches network validation"

**New narration, complete.**

> Every Midnight node does this on-chain work itself. The stored claim reaches network validation as the result of the transaction's contract state transition. Midnight nodes on ledger 9.1.0.1 each verify the transaction independently. Each uses the contract's verifier key and checks the bound public values and contract operations against ledger state. Each node rejects an invalid proof or a replay on its own checks.

### Beat 8.2: `keep-verification-fixed` (24 s becomes 26 s)

**Owner.** `scenes/scene8.js`.

**Old narration begins.** "Each node uses the same verifier"

**New narration, complete.**

> Each node uses the same verifier key for every program. It checks the final decider proof. It does not execute the witness tape, check each leaf or run the Sherlock chip. The proof remains about 6.4 KB. The cost of checking that proof does not depend on the length of the run. Longer execution adds prover work and leaves the node's run-verification work unchanged.

### Scene 7, beats 7.7 to 7.20: `ptray` (no narration change)

**Owner.** `scenes/scene7.js`.

**Objects and motion.** The rim of `ptray` takes `PALETTE.zoneOff`. The label "Off-chain" shows at (-4.6, 0.3, 0.5) while the tray is up. These beats have `where: 'off'`, so the chip reads "Off-chain: your machine" from 7.7 and returns to "On-chain: Midnight" at 7.21. In 7.20 the mini node housing and both of its children stand on the tray: a node proof is off-chain work.

### Scene 8, beats 8.4 and 8.5: `colTray` (no narration change)

**Owner.** `scenes/scene8.js`.

**Objects and motion.** A new tray `colTray`, 8.8 by 0.1 by 9.0 at (11.2, 0.05, -0.8), `paperWarm` floor, `zoneOff` rim, rises at the start of 8.4 and carries the hash blocks, the node slab and `graph`. The label "Off-chain" shows at its back left corner. At the start of 8.6 (0 to 0.6) the tray lowers and hides, with the hash blocks; the collection seal, its three pin tokens and the registry post then stand on the on-chain bench.

### Scene 8, beats 8.1 and 8.2: key cards

**Owner.** `scenes/scene8.js`.

**Objects and motion.** The card behind each of the six `nodes` is the decider verifier key card of section 1.4. **Labels.** In 8.1 and 8.2 change `Shared verifier key` to `Decider verifier key`.

## 3. New beat

### Beat 8.9: `what-lives-where` (index 8, 56 s, scene time 208 to 264)

**Title.** What lives where

**Owner.** `scenes/scene8.js` for the beat and its 3D; `shell.js`, `pipeline.html` and `m3.css` for the inventory card.

**Camera.** `WIDE`, no move: the beat holds the last pose of 8.8. `ctx.showAll(true)` for the whole beat.

**Objects and motion.** The whole bench is in frame with the boundary gate and the two rail colours. 2.0 to 3.2: the off-chain rail is emissive (`zoneOff`, 0.5) and the first column of the card appears. 6.0 to 7.2: the gate's floor strip is emissive (`gold`, 0.5) and the middle column appears. 10.0 to 11.2: the on-chain rail is emissive (`zoneOn`, 0.5) and the third column appears. 16.0 to 17.2: the comparison row appears. Each emissive value returns to 0 after 1.2 s. `trace` stays as 8.8 left it. Still pose: the bench in two rail colours with the gate between them, and the complete card.

**Inventory card (shell element `#whereCard`).** A beat can carry a field `inventory`; the shell shows the card while such a beat is current. It is an M3 card of real DOM text over the lower 62 percent of the stage, 16px inside its edges, container `--md-sys-color-surface-container-high`, corner `--md-sys-shape-corner-medium`, text `--md-sys-color-on-surface`, `body-medium`. Three columns on a wide stage, in the ratio 5 to 3 to 4; on a stage narrower than 600px the columns stack and the card scrolls. Each column has a header chip in the form of the location chip (same icon, text and roles as section 1.2) and a list. Items appear column by column at the times above (`inventory.columns[i].from`); in stepped mode all show at once. The exact text:

| Off-chain: your machine | Crossing: the transaction | On-chain: Midnight |
|---|---|---|
| Moriarty compiler | Decider proof | One ledger contract with `acceptRun` |
| Program and input | `prog_d` | One decider verifier key |
| Native execution trace | `input_d` | Claim set `runs`, keyed by `claim_d` |
| Witness stream | `outcome_d` | Collection roots |
| Private input | `n_steps` | Registry root |
| Sherlock chip circuit | | For each call: one proof verification |
| Leaf, node and decider proving keys | | For each call: `claim_d` check and insert |
| Leaf and node proofs | | |
| CPU cores | | |

Under the columns, one comparison row of two cells with a hairline `--md-sys-color-outline-variant` above it:

| Native Compact | Sherlock |
|---|---|
| One verifier key on-chain for each exported circuit of each contract | One decider verifier key for every program; a new program deploys nothing |

**Labels.** One scene label: `Accepted claim` | on: `plinth` | kind: state. The shell's two zone labels show at the gate.

**Carrier and handoff.** Carrier: all 0, as in 8.8. No handoff.

**Narration.**

> Everything off-chain is on the user's machine or a proving service the user picks: the compiler, the program and its input, the native execution trace, the witness stream, the private input, the Sherlock chip circuit, the leaf, node and decider proving keys, the leaf and node proofs, and the CPU cores that do the proving. One transaction crosses the boundary. It carries the decider proof and `[prog_d, input_d, outcome_d, n_steps]`.
>
> On-chain, Midnight holds one ledger contract with `acceptRun`, one decider verifier key, the claim set `runs` keyed by `claim_d`, the collection roots and the registry root. For each call, the nodes verify one proof, then check and insert `claim_d`. A native Compact contract puts one verifier key on-chain for each circuit it exports. Sherlock keeps one decider verifier key for all programs, and a new program deploys nothing.

Beat 8.8 keeps its text and its length. Its still pose is no longer the last frame of the piece; 8.9 is.

## 4. Who changes what

| File | Change |
|---|---|
| `visualization/src/pipeline/shell.js` | Read `beat.where` (default by scene: 1 to 6 `'off'`, 7 and 8 `'on'`). Drive `#whereChip`, the location word in the narration card, the scrubber band, the rail section labels. Split the front rail in two colours. Draw the boundary gate and its two zone labels. Read `beat.inventory` and drive `#whereCard`. |
| `visualization/pipeline.html`, `visualization/src/pipeline/m3.css` | Markup and styles for `#whereChip`, `#whereCard`, the two rail section labels, the scrubber band and the two colour-key entries. |
| `visualization/src/pipeline/m3palette.js`, `theme.js` | `zoneOff` and `zoneOn` in `M3_CANVAS` and `PALETTE`. |
| `visualization/src/pipeline/SCENE-API.md` | Document `beat.where`, `beat.inventory`, the gate, the zone labels and the rule for off-chain trays. |
| `visualization/DESIGN.md` | Add the location chip, the inventory card and the two zone roles. |
| `scenes/scene1.js`, `scene2.js`, `scene4.js`, `scene5.js` | New narration and duration of beats 1.1, 2.1, 4.1, 5.1 and 5.13; `where: 'off'` on every beat. |
| `scenes/scene3.js` | Beats 3.1 and 3.35 (narration, duration, the three parallel sweeps, the label); `where: 'off'`. |
| `scenes/scene6.js` | Beats 6.1, 6.3, 6.4; remove `bar`; move the rack off-chain; add the decider verifier key card and the row of objects that stay; `where` per section 1.1. |
| `scenes/scene7.js` | Beats 7.1 and 7.4; `vk` as the decider verifier key card; `ptray` rim and label; `where` per section 1.1. |
| `scenes/scene8.js` | Beats 8.1 and 8.2; the cards behind the nodes; `colTray`; new beat 8.9 with its `inventory` data; `where` per section 1.1. |

`SCENE-CONTRACT.md` section 6 changes with this amendment: scene 8 has 9 beats, and the durations of the 14 changed beats are those given above.
