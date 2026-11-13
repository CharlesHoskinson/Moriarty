# Narration script: steps 1 to 3

## Step 1: The program

Arrives: A source sheet and its input value at the left edge of the workbench.
Leaves: The program image as a ROM rack, with `prog_d` and `input_d` on the handoff plate.
Scene length: 113 s (target 80 s)

### Beat 1.1: source-and-input (22 s)

On screen: A violet source sheet slides into the first station beside a blue input cube. The sheet contains the symbolic program below, with increment and ceiling as named program constants. The source and input hold still together.

```text
fn lift(value) =
  value + increment

fn main(input) =
  let saved = input in
  let raised = lift(saved) in
  let below = raised < ceiling in
  let selected = if below then lift(raised) else saved in
  let carried = lift(selected) in
  let settled = lift(carried) in
  if below then settled + saved else saved
```

Narration: Moriarty source enters with an input value named input. The program saves that value, calls lift to add increment and compares the result with ceiling. A conditional selects the next value. Further named calls create intermediate values, and the selected branch then adds the saved input. The main run passes every arithmetic and resource check.

Labels:

- Moriarty source | on: the source sheet
- Input value | on: the blue cube
- Named call | on: the lift definition
- Conditional | on: the selected expression

Numbers used: none.

### Beat 1.2: source-to-rom (22 s)

On screen: The source sheet moves through a ceramic compiler housing and emerges as a violet ROM rack. Its shelves distinguish expressions, functions, argument references and supporting type and constant tables. A verification sweep crosses the rack, which stops beside the source sheet.

Narration: The compiler turns the source into a program image: the machine language, stored in ROM (read-only memory). Its tables hold expressions, function entries, argument references, types, constructors and constants. A rule reads the table and row that its instruction names. Each row is authenticated against this image, so nobody can substitute another operation or constant.

Labels:

- Compiler | on: the compiler housing
- Program image | on: the ROM rack
- Code tables | on: the expression and function shelves
- Types and constants | on: the supporting shelves

Numbers used: none.

### Beat 1.3: program-digest (22 s)

On screen: The image’s encoded cells line up in their canonical order and slide through the gold Poseidon block. A violet-rimmed hexagonal token emerges with an engraved face and settles into the front carrier. The ROM rack remains beside it, connected to the token by a fine line.

Narration: Poseidon, the hash function of Sherlock, digests the program image into `prog_d`. The image holds the code tables and their entry information in a fixed encoding. Changing any encoded operation, constant, function body or entry changes what the digest commits to. Later program reads prove membership under this commitment, so substituted code cannot run.

Labels:

- Poseidon | on: the hash block
- Program image | on: the ROM rack
- `prog_d` | on: the violet-rimmed token

Numbers used: none.

### Beat 1.4: canonical-input (22 s)

On screen: The input’s canonical value cells slide through the same Poseidon block and produce a blue-rimmed token. A stationary inset shows shared and separately stored versions of the same structure, with matching engraved digest faces. Those matching tokens slide into physical coincidence and hold as a single outline.

Narration: The input becomes `input_d` under `sherlock-value/1`, the canonical value digest. Canonical means that equal values hash from one encoding, fixed by type, value form and ordered contents. A reference contributes the digest of its target, so moving records or sharing equal substructures keeps the digest. A different value gives a different committed input.

Labels:

- Canonical value | on: the ordered input cells
- `sherlock-value/1` | on: the input hashing port
- `input_d` | on: the blue-rimmed token
- Equal value | on: the coincident inset tokens

Numbers used: `1` in `sherlock-value/1`: BRIEF.md §3, step 1.

### Beat 1.5: fixed-key-handoff (25 s)

On screen: The wall rack holds the leaf, node and decider key cards. A second program image rests beneath the same rack with a different violet token, while the key cards remain unchanged. The main program’s ROM rack and token carrier slide together onto the handoff plate and hold.

Narration: The ROM rack and the tokens `prog_d` and `input_d` identify the run. Three key cards serve it: leaf, node and decider. A key fixes a proof circuit, the constraints that check its witness. Every program and input uses this key set, because the circuit reads authenticated program data from ROM. A second program changes the program token and leaves the keys unchanged.

Labels:

- Leaf key | on: the leaf key card
- Node key | on: the node key card
- Decider key | on: the decider key card
- Program image | on: the main ROM rack
- `prog_d` | on: the violet token
- `input_d` | on: the blue token

Numbers used: Three keys: BRIEF.md §3, step 1.

## Step 2: The run

Arrives: The program image as a ROM rack, with `prog_d` and `input_d` on the handoff plate.
Leaves: The witness tape, with its Halted outcome frame at the end.
Scene length: 232 s (target 110 s)

### Beat 2.1: machine-arrival (22 s)

On screen: The ROM rack arrives in the same pose as the preceding handoff. It slides into the machine’s program socket while the token carrier remains at the front edge. The machine rests ready to execute, with an empty witness-tape path beside it.

Narration: `mori simulate` receives the ROM program and the input that `input_d` identifies. It runs a CEK-style machine, which keeps the current computation, the variable bindings and the saved continuations explicit. The reference machine `sherlock-k-a/1` defines every transition. The run produces the detailed evidence that the leaf checks. The simulator's own record certifies nothing.

Labels:

- `mori simulate` | on: the machine housing
- Program image | on: the ROM socket
- `sherlock-k-a/1` | on: the machine’s reference plate
- Witness stream | on: the empty tape path

Numbers used: `1` in `sherlock-k-a/1`: BRIEF.md §2, Semantics.

### Beat 2.2: machine-state (24 s)

On screen: The machine housing lifts to reveal its six state compartments. Register cubes occupy control, environment and fuel positions; linked tokens represent the stacks, and ceramic tiles represent memory. The exploded assembly holds with every compartment visible and no data moving.

Narration: The machine state has six parts. Control names the expression, return or helper action in progress. The environment maps variable names to values. The frame stack holds suspended computation and return context, and the task stack holds unfinished helper work. Memory stores immutable records. Fuel counters track available and consumed work, beside the logical resource counters that execution updates.

Labels:

- Control | on: the control compartment
- Environment | on: the environment compartment
- Frame stack | on: the continuation chain
- Task stack | on: the helper chain
- Memory | on: the ceramic record trays
- Fuel counters | on: the resource register compartment

Numbers used: Six state parts: BRIEF.md §3, step 2, enumerated state list.

### Beat 2.3: enabled-rule (24 s)

On screen: The rule table sits beside the current state, with 120 rule positions. A verification sweep crosses the state and instruction inputs, leaving the selected rule in heat orange. Only that rule connects to the next-state cells in the final pose.

Narration: Each step fires exactly one rule out of 120. A rule is enabled when its conditions hold for the current state and authenticated instruction, under the reference priority. Exactly one means that the selected rule fixes every permitted update and other state stays unchanged. A claimed transition cannot mix writes from two rules or pick a convenient branch.

Labels:

- 120 rules | on: the rule table
- Current state | on: the incoming register bank
- Instruction | on: the ROM row
- Selected rule | on: the active rule position
- Next state | on: the outgoing register bank

Numbers used: One selected rule and 120 rules: BRIEF.md §3, step 2.

### Beat 2.4: named-call-context (24 s)

On screen: At the call to lift, the caller’s continuation moves into the frame stack. The source sheet remains visible with the call, addition and comparison connected to their state positions. The saved-input record stays in memory, with its environment reference intact.

Narration: The call to lift saves the caller's continuation in a frame, so returning restores the environment and next expression. The addition inside lift produces the raised value. The comparison produces the Boolean that selects the conditional branch, and named calls continue from that choice. The saved input stays reachable through its environment reference until the final addition reads it.

Labels:

- Named call | on: the active call expression
- Saved continuation | on: the pushed frame
- Raised value | on: the addition result cube
- Branch condition | on: the Boolean cube
- Saved input | on: the referenced memory record

Numbers used: none.

### Beat 2.5: resource-accounting (24 s)

On screen: The rule’s required charge aligns beside the remaining-fuel register. A verification sweep crosses the comparison and its resulting resource registers. The final pose retains the initial budget, remaining allowance and consumed work as distinct cubes.

Narration: Fuel starts at the run's initial work budget. Each rule applies its specified charge to remaining fuel and to consumed work. Helper counters track traversal and construction, and resource limits bound stack growth and allocation. Each check compares the requested work with the remaining allowance, so no free computation, hidden allocation or counter wrap can extend the authorized run.

Labels:

- Initial budget | on: the initial-fuel register
- Required charge | on: the rule’s charge cube
- Remaining fuel | on: the remaining-fuel register
- Consumed work | on: the work register
- Resource limits | on: the logical-limit register group

Numbers used: none.

### Beat 2.6: witness-frame (23 s)

On screen: A completed witness frame slides onto the tape with its rule plate between before-state and after-state cells. The preceding after-state and the new before-state slide into physical coincidence. The tape holds with matching boundary cells sharing a single outline.

Narration: Each fired rule records one frame on the witness stream, the private execution tape. A frame holds the rule id, the state before and the state after. Each frame begins in the state where the previous frame ended. An unexplained change between recorded steps breaks that equality. The leaf checks this continuity and the rule's actual transition.

Labels:

- Witness stream | on: the tape
- Rule id | on: the rule plate
- State before | on: the incoming state cells
- State after | on: the outgoing state cells
- Equal boundary | on: the coincident cells

Numbers used: none.

### Beat 2.7: step-counter (23 s)

On screen: The witness tape extends behind a break mark as the step counter advances with the active frame. A separate ruler marks the counter range without expanding the machine geometry. The final pose shows `n_steps` beside the tape and the counter limit beside the ruler.

Narration: A 64-bit counter counts the fired steps, terminal publication included. A run has up to 2^64 - 1 steps, and its total is `n_steps`. The value 2^64 is the counter's range, not the size of a circuit. Counter checks stop a long execution from wrapping into a smaller claimed count. Fuel charges and step counts stay separate.

Labels:

- 64-bit counter | on: the step counter
- `n_steps` | on: the tape’s total-count plate
- 2^64 − 1 steps | on: the counter limit
- Counter range | on: the range ruler

Numbers used: 64-bit counter, 2^64 − 1 maximum steps and 2^64 counter range: BRIEF.md §3, step 2.

### Beat 2.8: arithmetic-rejection (21 s)

On screen: An inset of the addition’s overflow branch sits beside the stationary main tape. Its arithmetic result stops, moves slightly back and acquires a red hatch. The inset holds on a Rejected outcome frame carrying code 10, while the main run remains unchanged.

Narration: An addition that overflows ends the run as Rejected with code 10, the payload for arithmetic overflow. The proof checks that the arithmetic really overflows, and the wrapped result never becomes a successful return value. This ending uses the same rule system and the same proof key as the successful one.

Labels:

- Arithmetic overflow | on: the overflow check
- Rejected | on: the terminal tag
- Code 10 | on: the terminal payload

Numbers used: Rejection code 10: BRIEF.md §3, step 2.

### Beat 2.9: fuel-exhaustion (23 s)

On screen: A resource-check inset aligns a required charge with an insufficient remaining allowance. The operation stops, moves slightly back and receives a red hatch. The inset holds on an Exhausted frame with Fuel as its payload.

Narration: When the next required work charge exceeds the remaining fuel, the run ends as Exhausted with the kind Fuel. The payload names the resource. The check compares the required charge with the remaining allowance, so no prover can fake exhaustion. A leaf that fills its proving capacity only moves the tape cut and never produces this outcome.

Labels:

- Required charge | on: the charge cube
- Remaining fuel | on: the allowance cube
- Exhausted | on: the terminal tag
- Fuel | on: the terminal payload

Numbers used: none.

### Beat 2.10: halted-tape-handoff (24 s)

On screen: The main tape ends with a Halted frame and its teal result cube. The completed tape slides onto the handoff plate, preserving the outcome frame at its right end. The program and input tokens remain visible in their front carrier throughout the transfer.

Narration: The main run reads the saved input, completes its final addition and returns the selected value. Each return through the frame stack restores a suspended context. At the outer return, publication records Halted with the result value as its payload. The witness tape ends at that outcome frame and carries `n_steps`, the run's step count, into leaf proving.

Labels:

- Witness stream | on: the completed tape
- Halted | on: the final outcome tag
- Result value | on: the teal payload cube
- `n_steps` | on: the tape’s count plate
- `prog_d` | on: the violet token
- `input_d` | on: the blue token

Numbers used: none.

## Step 3: The leaf

Arrives: The witness tape, with its Halted outcome frame at the end.
Leaves: Leaf proof slabs, each with seven pins `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`, on the handoff plate.
Scene length: 856 s (target 240 s)

### Beat 3.1: tape-to-slices (23 s)

On screen: The witness tape arrives in its preceding handoff pose. A slice slides into a leaf housing with 16 step positions, beneath the unchanged leaf key card. The terminal slice remains visible farther along the tape, with an inactive suffix after its outcome frame.

Narration: The witness tape arrives with its Halted outcome frame attached. The leaf cuts the tape into slices of `S = 16` step slots, each entering the same circuit under one leaf key. A full slice uses every slot. A shorter slice keeps its unused suffix inactive, so state holds still and the proof covers the recorded steps.

Labels:

- Witness stream | on: the incoming tape
- `S = 16` | on: the slot ruler
- Leaf key | on: the key card
- Active slice | on: the inserted tape segment
- Inactive suffix | on: the unused terminal positions

Numbers used: `S = 16`, one circuit and one leaf key: BRIEF.md §3, step 3.

### Beat 3.2: core-register-chain (23 s)

On screen: The leaf housing exposes a floor plan with CORE beside the provider modules and the BUS route between them. CORE’s register bank expands into consecutive rule slots. The after-state of a selected slot slides onto the following before-state, ending in physical coincidence.

Narration: CORE, the module that checks machine transitions, holds the persistent registers and the rule slots. Its registers encode control, environment reference, cached stack tops, helper work, result, outcome and resource counters. Each slot's final register cells equal the next slot's initial cells. Registers outside the selected rule's write set keep their values and hide no change.

Labels:

- CORE | on: the core housing
- Persistent registers | on: the register bank
- Rule slot | on: the inspected slot
- Equal state | on: the coincident boundary cells
- Unchanged registers | on: the cells outside the write set

Numbers used: none.

### Beat 3.3: decoder-control-word (25 s)

On screen: The current state’s dispatch cells align with DEC’s fixed table. A verification sweep selects the matching control-word row, whose cells connect directly to CORE’s operation and write controls. The final pose retains those connections and dims the unselected table rows.

Narration: DEC, the decoder, derives a rule control word from a fixed table. It checks the current control and outcome, the authenticated instruction, stack tags and comparison results against each rule's conditions, and reference priority selects exactly one matching rule. The returned word fixes the rule id, operation order and write set. Bounded fields stop any other word passing for that entry.

Labels:

- DEC | on: the decoder housing
- Dispatch facts | on: the incoming state cells
- Fixed table | on: the table rack
- Control word | on: the selected row
- Write set | on: CORE’s permitted-write cells

Numbers used: Exactly one selected rule: BRIEF.md §3, step 2.

### Beat 3.4: active-prefix (22 s)

On screen: A verification sweep crosses the slice from its first active slot through terminal publication. The following slots retain identical before-state and after-state cells, with blank request sockets. The final pose shows a continuous active prefix and an inactive suffix.

Narration: Active slots form one continuous prefix. Each active slot counts as one step, and its internal operations add no steps. An inactive slot keeps every persistent register and makes no request. Terminal publication ends the active prefix. A prover cannot skip a transition, pad the step count or keep executing after Halted, Rejected or Exhausted.

Labels:

- Active prefix | on: the enabled slots
- Terminal publication | on: the final active slot
- Inactive suffix | on: the remaining slots
- State preserved | on: the suffix register cells

Numbers used: One counted step per active slot: BRIEF.md §3, steps 2 to 3.

### Beat 3.5: bus-tuple (24 s)

On screen: A bar of cubes leaves CORE and enters the BUS route, with positions for `(chip, key, a, b, c)`. The provider housings display their fixed chip ids beside the route’s request channels. The packet stops at the junction, with its destination and payload positions legible.

Narration: BUS connects CORE requests to checked provider tuples `(chip, key, a, b, c)`. The chip names the operation family, the key selects the operation or record, and the other cells carry the data. Chip ids are 1 for ALU, 2 for MEM, 3 for HASH and 4 for ROM. CORE alone issues requests, through the two request channels.

Labels:

- BUS | on: the shared route
- `(chip, key, a, b, c)` | on: the packet bar
- ALU 1 | on: the ALU provider
- MEM 2 | on: the MEM provider
- HASH 3 | on: the HASH provider
- ROM 4 | on: the ROM provider

Numbers used: Chip ids 1, 2, 3 and 4; two request channels: BRIEF.md §3, step 3.

### Beat 3.6: request-meets-provider (24 s)

On screen: The ADD packet travels from CORE to the ALU provider row. Its complete tuple slides onto the provider tuple, merging their outlines when they coincide. The checked packet returns along the same route to CORE and holds beside the receiving cells.

Narration: CORE puts the addition's operation, operands and claimed result into a BUS request. The whole tuple must equal a checked ALU provider tuple. Every provider satisfies its operation constraints, even when no request uses it. Several requests can match the same provider, but requests cannot authenticate each other. An invented answer therefore has no valid computation to match.

Labels:

- CORE request | on: the outgoing packet
- ADD | on: its operation cell
- Checked provider | on: the ALU row
- Equal tuple | on: the matched packet outlines

Numbers used: none.

### Beat 3.7: checked-addition (29 s)

On screen: The ALU exposes operand, result and overflow-flag cubes. A verification sweep crosses the main run’s ADD equation and leaves the clear overflow flag connected to CORE’s successful transition. A stationary inset connects the set flag to a red-hatched Rejected payload carrying code 10.

Narration: ALU, the arithmetic module, proves the exact unsigned addition of the value and increment. It constrains both operands, the result and the overflow flag, and a carry past the permitted integer width sets the flag. CORE connects a set flag to Rejected with code 10. In the main run the flag is clear, and the ordinary result passes unwrapped. The same ALU checks subtraction, multiplication and equality under their machine rules.

Labels:

- ALU | on: the arithmetic housing
- ADD | on: the active operation
- Operands | on: the operand cubes
- Result | on: the output cube
- Overflow flag | on: the flag cube
- Code 10 | on: the rejection payload

Numbers used: Rejection code 10: BRIEF.md §3, step 2.

### Beat 3.8: checked-comparison (24 s)

On screen: The LT packet aligns the raised value and ceiling with ALU’s comparison cells. A verification sweep crosses the comparison and its Boolean output. The output cube holds in physical coincidence with CORE’s conditional-control cell.

Narration: The LT request asks ALU whether the raised value is less than ceiling. ALU constrains its Boolean result to that comparison of the authenticated operands, and CORE uses the same result cell to choose the conditional branch. A Boolean changed without a change to the operands fails the comparison check, so the prover cannot pick the branch it prefers.

Labels:

- LT | on: the comparison operation
- Raised value | on: the first operand
- Ceiling | on: the comparison bound
- Boolean result | on: the output cube
- Branch condition | on: CORE’s matching control cell

Numbers used: none.

### Beat 3.9: integer-range-checks (31 s)

On screen: A checked integer cube expands into limb cubes beside RANGE’s fixed table. A verification sweep crosses the limb membership checks, the narrower highest piece and the reconstructed value. The reconstructed cube settles onto the original caller’s cube, while distinct table tags separate RANGE from DEC.

Narration: RANGE proves that integer cells fit their required widths. It reconstructs each checked cell from 12-bit limbs, small integer pieces checked against a fixed table, and the highest piece also obeys the exact width of the original integer. The reconstruction equals the caller's cell. Oversized operands and modular arithmetic tricks that imitate valid integer equations fail here. Fixed table tags keep RANGE queries apart from DEC queries, so neither check can borrow the other's table entries.

Labels:

- RANGE | on: the range-check housing
- 12-bit limbs | on: the limb cubes
- Exact top width | on: the highest piece
- Equal reconstruction | on: the coincident original and reconstructed cubes
- Fixed table tags | on: the table’s separated query ports

Numbers used: 12-bit limbs: BRIEF.md §3, step 3.

### Beat 3.10: poseidon-relations (34 s)

On screen: HASH exposes its ordered input port and gold Poseidon block. Caller cells slide into the block in order, producing an engraved digest token at the output. Direct input connections remain visible for the longer hash, alongside the separate BUS provider socket.

Narration: HASH computes Poseidon over the caller's ordered input cells. It constrains the initial state, absorption, permutation rounds and final output. Fixed domain tags tell memory records, tree levels, stacks and state bodies apart, so a stack link cannot pass for a memory node. Public digest formats keep their specified encoding. Each input copies the caller's cell, so substituted data cannot be hashed. Longer hashes use direct ports with copied input cells in a fixed order. Chained short BUS hashes differ from the whole-message hash.

Labels:

- HASH | on: the hash housing
- Ordered inputs | on: the incoming cells
- Domain tag | on: the fixed domain cell
- Poseidon | on: the gold block
- Digest | on: the output token
- Direct port | on: the longer input connection

Numbers used: none.

### Beat 3.11: append-address-chain (25 s)

On screen: CORE’s append events form a line from the incoming length cube to the outgoing length cube. A verification sweep follows consecutive addresses and the MEM request attached to each event. The final pose shows every new record connected to its own address and provider row.

Narration: CORE owns the append chain, the ordered sequence of new memory addresses. The first append starts at the incoming length, each active append advances the length to the next address, and the final length equals the outgoing memory length. Every appended record also matches a MEM provider through BUS. No address is skipped or allocated twice, and no record's contents are swapped.

Labels:

- Append chain | on: CORE’s event line
- Incoming length | on: the initial length cube
- Append address | on: the inspected event
- MEM request | on: its packet
- Outgoing length | on: the final length cube

Numbers used: none.

### Beat 3.12: memory-window (24 s)

On screen: The MEM window separates inherited tail tiles from newly appended tiles in a continuous tray. A new record slides into the next unused position while padding remains visually distinct and disconnected from BUS. The tray holds with its incoming and outgoing length markers aligned to the corresponding record boundaries.

Narration: The MEM window holds the imported partial tail and this leaf's new records, in a single arena. Each logical record occupies one cell. Active records form a continuous prefix, and the imported records end exactly at the incoming length. Rules append at most 3 cells per step. Padding serves no read. Unused capacity therefore holds no invented program data.

Labels:

- MEM window | on: the window housing
- Imported tail | on: the inherited tiles
- New records | on: the appended tiles
- Padding | on: the unused positions
- At most 3 cells | on: the per-step append port

Numbers used: One arena, one cell per logical record and at most 3 appended cells per step: BRIEF.md §2, Arenas, Tree depth and Appends per step.

### Beat 3.13: authenticated-tail (24 s)

On screen: The example’s call and binding records occupy the tail tray in append order. The inherited tiles and their fixed padding feed the Poseidon block, whose output token slides onto the incoming tail token. The tray holds full with 8 cells, while the coincident tokens certify the inherited portion.

Narration: The named calls and intermediate bindings append records in execution order until the 8-cell tail is full. The dense MEMORY BACKEND authenticates the imported tail: it recomputes the tail's digest and matches the incoming tail token. Unused tail positions hold fixed padding. At a leaf boundary, inherited records cannot be replaced and a partial tail cannot take a second encoding.

Labels:

- MEMORY BACKEND | on: the backend housing
- 8-cell tail | on: the full tray
- Imported records | on: the inherited portion
- Fixed padding | on: the tail-opening padding cells
- Equal tail digest | on: the coincident tokens

Numbers used: 8-cell tail: BRIEF.md §2, Flush.

### Beat 3.14: segment-flush (24 s)

On screen: The full tray aligns beneath the next segment position in the tree. A verification sweep follows the empty-position path and the replacement path, with their address bits and sibling tokens visibly shared. The new segment token holds at the selected position with the outgoing root above it.

Narration: The dense MEMORY BACKEND turns the full 8-cell tail into a segment. Its flush proves that the next segment position is empty under the current tree root. The new segment follows the same address path and sibling hashes to compute the new root. The matching paths stop the flush from overwriting an occupied segment or placing records elsewhere.

Labels:

- Segment flush | on: the flush unit
- Empty position | on: the old segment position
- Same address path | on: the shared path
- Shared siblings | on: the sibling tokens
- New root | on: the outgoing root token

Numbers used: 8-cell segment: BRIEF.md §2, Tree depth and Flush.

### Beat 3.15: memory-outlasts-counter (22 s)

On screen: The tree column uses a break mark and a depth ruler instead of repeated literal levels. A verification sweep crosses the address bound and the capacity inequality beside the tree. The final pose compares the memory-capacity ruler with the step-counter ruler.

Narration: The memory tree has depth 63 over segments, a capacity of 2^66 cells. Rules append at most 3 cells per step, and 3 x 2^64 < 2^66, so memory outlasts the step counter even at the maximum append rate. Address checks keep each record inside that space, so field arithmetic cannot disguise an outside address.

Labels:

- Depth 63 | on: the tree ruler
- 2^66 cells | on: the memory-capacity ruler
- 64-bit counter | on: the step-counter ruler
- Capacity inequality | on: the arithmetic inset
- Address bound | on: the checked address cube

Numbers used: Depth 63, 2^66 cells, at most 3 cells per step and `3 x 2^64 < 2^66`: BRIEF.md §2, Tree depth and Appends per step. The 64-bit counter: BRIEF.md §3, step 2.

### Beat 3.16: capacity-closes-slice (22 s)

On screen: The current leaf’s flush unit and remaining window capacity stay visible beside the tape. The cut marker moves to the boundary before a step that needs more capacity. The waiting step rests at the next slice’s entrance, with the machine-state tokens unchanged across the cut.

Narration: A leaf has one flush unit. Its remaining capacity depends on the imported tail and the work already in the slice. When the next step exceeds that capacity, the leaf ends before it, and execution resumes in the next leaf with no extra charge. The unused suffix stays inactive and preserves the machine state.

Labels:

- Flush unit | on: the leaf’s flush housing
- Remaining capacity | on: the unoccupied window
- Slice boundary | on: the cut marker
- Next step | on: the waiting frame
- State preserved | on: the matching state tokens

Numbers used: One flush unit per leaf: BRIEF.md §2, Flush.

### Beat 3.17: read-address-and-time (24 s)

On screen: CORE’s requested address, expected record kind and current append-chain position align at the read port. A verification sweep checks the address source, kind equality and running-length bound. The final pose shows the permitted record behind the current-length marker and later appends beyond it.

Narration: Every read goes through CORE's authenticated read port. The requested address equals the address in the requesting register or preceding operation, and the record kind equals the kind the rule requires. The address must lie below the running memory length at that exact read point. A read cannot reach a later append, take another reference or confuse record kinds.

Labels:

- Read port | on: CORE’s read interface
- Requested address | on: the address cube
- Expected kind | on: the kind cube
- Running length | on: the append-chain marker
- Later append | on: a record beyond that marker

Numbers used: none.

### Beat 3.18: scalar-old-read (24 s)

On screen: The saved-input record sits in a completed segment behind the current leaf. Its record hash follows a Merkle path through sibling tokens to a computed root, using the standard Poseidon motion. That root slides onto the pre-state root token and holds in physical coincidence; the record connects directly to CORE’s read port.

Narration: The last use of the saved input requests an old record from a completed segment. A scalar cold read, one isolated authenticated memory read, hashes the record and follows its Merkle opening: the sibling hashes along the address path. The computed root must equal the leaf's pre-state root, which rules out a forged payload, wrong kind or substituted address.

Labels:

- Saved input | on: the old record
- Scalar cold read | on: the direct read interface
- Merkle opening | on: the sibling path
- Computed root | on: the reconstructed token
- Pre-state root | on: the matching incoming root

Numbers used: none.

### Beat 3.19: authenticated-cache-line (24 s)

On screen: PAGE/ROM presents an 8-record line from an old completed segment beside the stationary scalar-read diagram. A verification sweep crosses the line’s record hashes, its path to the pre-state root and its position before the imported tail. The authenticated line holds with its record cells connected to MEM provider sockets on BUS.

Narration: PAGE/ROM also serves old memory as an 8-record cache line. The dense MEMORY BACKEND hashes the line and checks its path against the same pre-state root. The line's segment must lie before the imported tail. Its checked record cells become MEM providers on BUS. Repeated reads are authenticated, and the cache can neither invent contents nor overlap the window.

Labels:

- PAGE/ROM | on: the cache housing
- 8-record cache line | on: the line tray
- Pre-state root | on: the authentication target
- Old segment | on: the segment-position marker
- MEM providers | on: the line’s BUS sockets

Numbers used: 8-record cache line: BRIEF.md §2, Old reads.

### Beat 3.20: authenticated-rom-line (24 s)

On screen: A ROM line rests between the violet program rack and the ROM provider socket. A verification sweep follows its table and row position through the authentication path to the program root. The final pose shows the requested row coincident with its checked provider cells and `prog_d` still in the front carrier.

Narration: ROM lines authenticate program table rows against the program root that `prog_d` binds. CORE fixes the requested table and range-checks the row index. PAGE/ROM derives each provider key from the authenticated path and row position, and the served cells equal the hashed cells. A row from another table or another program cannot supply an instruction, type or constant.

Labels:

- ROM line | on: the program-row tray
- Table and row | on: the requested key
- Authenticated path | on: the ROM path
- Program root | on: the root token
- `prog_d` | on: the violet token

Numbers used: none.

### Beat 3.21: frame-chain-push (24 s)

On screen: The named call’s previous frame top, depth cube and preceding chain token line up at the Poseidon block. They slide through the block to produce a new FRAMES link beneath the incoming cached top. TASKS remains visible as a separate stationary chain with its own domain marker.

Narration: STACKS keeps FRAMES and TASKS as Poseidon hash chains with cached top records. FRAMES stores continuations and TASKS stores helper work. Pushing during the named call hashes the previous top, its depth and the preceding chain into a new link, and the new frame becomes the cached top. Separate hash domains stop a task record posing as a frame.

Labels:

- STACKS | on: the stack housing
- FRAMES | on: the continuation chain
- TASKS | on: the helper chain
- Previous top | on: the spilled frame
- Stack depth | on: the depth cube
- Cached top | on: the newly installed frame

Numbers used: none.

### Beat 3.22: frame-chain-pop (24 s)

On screen: The supplied restored frame, its depth and predecessor token feed the Poseidon block. The computed link slides onto the stored FRAMES link and merges into a single outline. The restored frame holds at the cached-top position, with the predecessor chain exposed below it.

Narration: Returning from lift pops FRAMES. STACKS recomputes the current chain link from the restored frame supplied, its depth and its predecessor digest. That hash must equal the stored link before the frame becomes the cached top. Counts and empty-stack constants fix the endpoints. A forged return context, wrong predecessor or frame restored at another depth fails this equality.

Labels:

- Frame pop | on: the active stack operation
- Restored frame | on: the restored record
- Predecessor | on: the preceding chain token
- Stored link | on: the authentication target
- Equal link | on: the coincident tokens

Numbers used: none.

### Beat 3.23: task-rewrite-before-push (24 s)

On screen: A TASKS helper inset shows the updated parent cursor already in its top record, with the former cursor marked as superseded. The updated parent feeds the Poseidon block as the child becomes the cached top. The final pose preserves the updated parent beneath the child, connected to the same machine-step frame.

Narration: Helper work uses the same checked chain protocol on TASKS. When a helper advances a parent cursor and pushes a child, the top rewrite precedes the push, so the chain commits to the updated parent. Popping the child restores that updated record. CORE checks this order as one machine event, so no work repeats and no cursor update is lost.

Labels:

- TASKS | on: the helper chain
- Updated parent | on: the rewritten task
- Child task | on: the new cached top
- Cursor | on: the updated progress cube
- Same machine step | on: the associated tape frame

Numbers used: One counted machine event: BRIEF.md §3, step 2.

### Beat 3.24: value-digest-at-creation (23 s)

On screen: VALUE DIGEST receives the new value’s form and authenticated contents through CORE’s read ports. The ordered cells slide through Poseidon and produce a named engraved value token. That token settles onto the digest position of the newly appended record.

Narration: VALUE DIGEST computes `sherlock-value/1` whenever a rule creates a value. The value form, scalar contents and authenticated child digests determine the hash. CORE supplies the child records and ROM data through its checked read ports. The resulting digest equals the digest stored with the new record, so a correct-looking record cannot carry an invented semantic value.

Labels:

- VALUE DIGEST | on: the value-digest housing
- `sherlock-value/1` | on: its format plate
- Authenticated contents | on: the incoming cells
- Value digest | on: the output token
- New record | on: the appended tile

Numbers used: `1` in `sherlock-value/1`: BRIEF.md §3, step 1.

### Beat 3.25: structured-value-checks (24 s)

On screen: A mechanism inset presents ordered child digest tokens beneath a constructor record. A verification sweep crosses the constructor identity, field progression and completion check. Shared and relocated representations hold beside it with equal value-token faces, while their memory arrangements remain different.

Narration: VALUE DIGEST checks scalar bounds, Boolean values, constructor identity, field order and field count. A structured value combines its children's digests, and constructor processing carries checked fold state across steps. Completion confirms that every field contributes before the value is used. A reference keeps its target's digest. Sharing and relocation keep value identity, and a missing or substituted field fails.

Labels:

- Constructor identity | on: the constructor metadata
- Ordered fields | on: the child digest row
- Fold state | on: the progress register group
- Completion check | on: the final comparison
- Equal value digest | on: the matching value tokens

Numbers used: none.

### Beat 3.26: boundary-body (24 s)

On screen: BOUNDARY COMMITMENT receives ordered cells from CORE, STACKS and MEMORY BACKEND through direct connections. The body cells slide through Poseidon and produce `body_d`. The final pose retains visible connections from the memory length, root and tail digest to the ordered body.

Narration: BOUNDARY COMMITMENT binds the state that execution needs to continue. Its body hashes the persistent registers, constructor progress, stack hashes and counts, and memory `(len, root, tail_d)`. Here len is the allocated length, root commits completed segments and tail_d commits the partial tail. Each input equals its owning module's checked cell, so the commitment cannot describe another state.

Labels:

- BOUNDARY COMMITMENT | on: the boundary housing
- Persistent registers | on: CORE’s body contribution
- Stack state | on: STACKS’ contribution
- `(len, root, tail_d)` | on: the memory contribution
- `body_d` | on: the output token

Numbers used: none.

### Beat 3.27: boundary-header (24 s)

On screen: The header fields align in their stated order above the stationary `body_d` token. They slide through Poseidon with the body token in its final header position. The resulting state token holds beside the still-readable header fields.

Narration: The boundary header holds `outcome_tag`, `outcome_payload`, `input_d`, `deps_d` and `body_d`. The outcome fields carry the execution status and its value digest, rejection code or exhaustion kind. The dependency digest binds consumed claims. Poseidon hashes the header in this fixed order. Its digest binds the readable outcome fields to the complete body, and checking the header leaves the registers hidden.

Labels:

- `outcome_tag` | on: the header’s outcome-tag cube
- `outcome_payload` | on: the header’s payload cell
- `input_d` | on: the header’s input token
- `deps_d` | on: the dependency token
- `body_d` | on: the body token
- State digest | on: the header hash output

Numbers used: none.

### Beat 3.28: endpoint-digests (24 s)

On screen: The entry and exit state tokens align with the leaf’s corresponding public-pin sockets. Each computed token slides onto its public value in physical coincidence. A stationary scratch inset shows only within-slot producer-to-consumer connections, with no scratch connection crossing the leaf boundary.

Narration: The entry header digest becomes `pre_d` and the exit header digest becomes `post_d`. Each public token equals its computed hash output. By Poseidon's collision resistance, equal boundary tokens bind equal committed states. Temporary scratch stays outside the boundary because each use depends on a value produced earlier in the same slot. Incoming scratch is unconstrained and cannot change persistent execution.

Labels:

- `pre_d` | on: the entry digest pin
- `post_d` | on: the exit digest pin
- Computed digest | on: the matching hash output
- Within-slot scratch | on: the temporary-cell inset
- Producer connection | on: the scratch dependency line

Numbers used: none.

### Beat 3.29: genesis-initialization (25 s)

On screen: The first slice rests beside INIT with its height displayed as `h_pre = 0`. A verification sweep crosses its entry control, outcome, counters, empty stack chains and empty memory tray. The initial-state digest holds connected to those checked cells and to the program and input tokens.

Narration: INIT checks the initial state only when `h_pre = 0`. It proves `Init(prog_d, input_d)`: the entry control, Running outcome, resource counters, empty stacks and empty genesis memory match their prescribed initial values. It also authenticates the program and input commitments. The height check drives the INIT selector, so a prover cannot switch initialization off and start from a convenient intermediate state.

Labels:

- INIT | on: the initialization housing
- `h_pre = 0` | on: the first slice’s height cell
- Running | on: its initial outcome tag
- Empty genesis memory | on: the empty tray
- Empty stacks | on: the initial chain endpoints
- Initial state | on: the checked entry registers

Numbers used: Genesis height 0: BRIEF.md §2, Input admission, and §3, step 3.

### Beat 3.30: input-by-digest (24 s)

On screen: The input token remains beside empty genesis memory, with a digest reference in the machine state. An inspected input node’s canonical contents pass through Poseidon, and the resulting token slides onto the expected value digest. The authenticated node holds beside the append port, ready to enter memory through the ordinary record path.

Narration: Input admission keeps the value as a digest in that empty genesis state. When execution inspects an input node, VALUE DIGEST recomputes its canonical digest and matches it to the expected digest before use. Records enter memory through the checked append chain. The input enters without loading its whole structure at genesis, and unauthenticated input contents cannot influence any transition.

Labels:

- Input by digest | on: the initial input reference
- Input node | on: the inspected node
- Expected digest | on: the authentication target
- Equal value digest | on: the coincident tokens
- Append port | on: CORE’s record-entry port

Numbers used: none.

### Beat 3.31: rules-and-step-interval (24 s)

On screen: The slot rule plates line up in execution order, including the fixed inactive entries. They slide through Poseidon to produce `rules_d`, beside the checked `h_pre` and `steps` cubes. Those public values hold at their pin positions on the emerging slab.

Narration: CORE computes `rules_d` by hashing the selected rule identifiers in slot order, fixed inactive entries included. It also constrains `steps` to the active slot count and `h_pre` to the slice's starting height, and the resulting height stays within the step counter's range. With these bindings a prover cannot claim another rule sequence or an inconsistent interval for the slice.

Labels:

- Ordered rule ids | on: the slot plates
- Inactive entries | on: the suffix plates
- `rules_d` | on: the rule digest pin
- `h_pre` | on: the starting-height pin
- `steps` | on: the active-count pin

Numbers used: none.

### Beat 3.32: seven-public-pins (24 s)

On screen: A titanium slab with seven front pins holds above the completed leaf floor plan. A verification sweep crosses the slab and gives its gold edge a brief bright finish. Labels identify the binding, program and endpoint pins; the remaining pins keep the order established in the preceding beat.

Narration: The seven public pins are `[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]`. The binding value connects the proof to its transcript, the data the proof protocol authenticates. The program digest fixes the executed image, and the boundary digests fix the endpoints. The rule digest fixes each slot's choice, and the height and step count place the slice in the run.

Labels:

- Seven public pins | on: the complete pin row
- `binding` | on: the transcript-binding pin
- `prog_d` | on: the program pin
- `pre_d` | on: the entry-state pin
- `post_d` | on: the exit-state pin

Numbers used: Seven public pins: BRIEF.md §3, step 3.

### Beat 3.33: partition-independence (24 s)

On screen: Matching copies of the same witness tape rest under different slice boundaries. Each ends beside the same completed-segment arrangement and partial tail. Their final memory tokens slide into physical coincidence, leaving a single outline while the differing cuts remain visible.

Narration: Changing the cuts leaves memory unchanged after the same executed steps. Only real appends advance its length, full segments enter the tree at their sequential positions, and partial tails carry forward unchanged. Closing a leaf adds no record or charge. Cache choices change no state. Every valid partition reaches the same final memory and supports the same run claim.

Labels:

- Same execution | on: the matching tapes
- Different cuts | on: the differing slice markers
- Same memory | on: the matching segment and tail arrangements
- Equal final digest | on: the coincident tokens
- No closing charge | on: the unchanged resource registers

Numbers used: none.

### Beat 3.34: bounded-leaf-cost (24 s)

On screen: The flush unit and scalar old-read unit sit over calibrated row rulers. A verification sweep crosses their cost plates and the complete leaf’s row-capacity frame. The long witness tape remains behind a break mark, while the leaf housing keeps its fixed footprint.

Narration: At depth 63 a flush costs 8,821 rows and an old-memory read costs 4,761 rows. Rows are positions in the circuit's constraint layout, and the complete leaf fits 2^16 rows. That fixed capacity covers one slice's checks, and successive leaves cover the long run. Neither the tape's length nor the memory tree's capacity makes a single leaf circuit larger.

Labels:

- Depth 63 | on: the memory-tree ruler
- 8,821 rows | on: the flush-cost plate
- 4,761 rows | on: the old-read cost plate
- 2^16 rows | on: the complete leaf’s capacity ruler
- Leaf circuit | on: the fixed housing

Numbers used: Depth 63, flush cost 8,821 rows, old-memory read cost 4,761 rows and leaf capacity 2^16 rows: BRIEF.md §3, step 3.

### Beat 3.35: leaf-slabs-handoff (24 s)

On screen: The completed leaf slabs slide together onto the handoff plate, each with the same footprint and seven front pins. Their endpoint tokens and height cells remain aligned with their own slabs. The final establishing pose holds on the slabs, with the program and input carrier at the front edge and the node station ahead.

Narration: The checked slices leave as leaf proof slabs, each with its seven public pins. Each proof binds the executed rules, authenticated data and initial and final state of its slice. Neighboring slabs carry the endpoint digests and step intervals that linking needs. The handoff plate takes the slabs to the node, and `prog_d` and `input_d` stay with the run.

Labels:

- Leaf proofs | on: the slabs
- Seven public pins | on: the front pin rows
- Boundary digests | on: the endpoint tokens
- Step intervals | on: the height and count cells
- `prog_d` | on: the violet carrier token
- `input_d` | on: the blue carrier token

Numbers used: Seven public pins per leaf: BRIEF.md §3, step 3.

## Notes for the director

- **Timing.** Durations use whitespace-separated narration words, including identifiers and mathematical tokens, divided by 2.5 and rounded up. All narration passages contain 30 to 90 words. The scene totals are **113 s**, **232 s** and **856 s**. Every scene exceeds its target by more than a quarter. The additional time carries the requested mechanisms, checks and consequences of omitted checks.
- **Display conventions.** Render powers with superscripts. Every digest is an engraved hexagonal token, never a hexadecimal string. The token carrier remains visible throughout, but its text labels disappear during close inspections when necessary to keep the visible label count within six.
- **Symbolic example.** The source sheet uses symbolic functional notation. Increment and ceiling are named program constants. The main input satisfies the comparisons, arithmetic bounds and resource allowances that lead to Halted. The Rejected and Exhausted beats are separate branch insets, not additional endings attached to the main tape. The task-stack and structured-value insets explain module behavior without adding those operations to the scalar source program.
- **Arithmetic width.** The requested example uses unsigned 128-bit arithmetic, as the architecture’s ALU kit specifies. BRIEF.md §§2 to 3 does not include that width. The visible script therefore says “unsigned” and “permitted integer width” without displaying the number.
- **Input identity disagreement.** `research/sherlock-final-design-20261111/ARCHITECTURE.md`, the VALUE DIGEST and INIT kits, and `openspec/changes/sherlock-vm-consensus-20261107/design.md` define `input_d` as an input-envelope hash containing a canonical value digest, fuel components and a stack limit. BRIEF.md §3, step 1 calls `input_d` the input value’s digest under `sherlock-value/1`. The script follows the brief and does not display the conflicting envelope formula. Resource initialization remains described as a field-by-field INIT check; the script does not invent a replacement resource-parameter commitment.
- **Program identity disagreement.** The older consensus design and requirements include layout, qualification bounds and profile identity in the program-digest preimage. The architecture’s INIT kit also describes those opening fields. BRIEF.md §3, step 1 defines `prog_d` as the Poseidon digest of the program image. The script follows that definition and shows the image’s code tables and entry information without profile names or qualification-bound commitments.
- **Genesis disagreement.** The older consensus design and requirements describe input records already present in initial data tables. The architecture discusses several admission forms and leaves parts conditional. BRIEF.md §2 selects admission by digest with empty genesis memory. Beats 3.29 to 3.30 use that selected mechanism.
- **Memory-record disagreement.** The architecture’s MEM window kit discusses logical records spanning consecutive physical cells. BRIEF.md §2 states that one logical record is one cell. Beat 3.12 and every memory tray use the brief’s cell interpretation.
- **Memory configuration disagreement.** The architecture and README retain alternatives, conditional tree-depth wording and older per-leaf append descriptions. The script uses only the brief’s single arena, depth-63 dense tree, 8-cell tail and one flush unit per leaf. It does not equate one flush unit with a fixed total number of appends per leaf: imported-tail occupancy and cumulative capacity determine the cut.
- **Leaf-size disagreement.** The older consensus design discusses other slice sizes and larger circuit limits, and the README gives prototype sizes. The script uses `S = 16` step slots and the complete leaf’s 2^16-row capacity from BRIEF.md §3, step 3. Partial slices preserve the same circuit and key.
- **Partition disagreement.** The README says execution-state commitments depend on leaf cuts, while its dense-memory discussion separately describes partition independence. BRIEF.md §3, step 3 requires the final memory state to be independent of cuts. Beat 3.33 compares the same executed prefix or completed run under different valid partitions; it does not equate intermediate endpoints representing different execution positions.
- **Public statement disagreement.** Older README and consensus passages expose initial or final state digests, use a sealed program identity, or describe a contract per program. The script uses the brief’s shared key set and seven leaf pins. It leaves final publication and contract behavior to the later scenes.
- **Earlier leaf visualization.** `visualization/src/leaf/leafChip.js` supplies only the floor-plan arrangement, BUS-packet route and tail-tray layout. Its stand-in rules, register schema, stack records and populated genesis do not enter this script.
- **Quantities deliberately unnumbered.** The script gives no numeric program-table dimensions, ROM tree depth, program-size bound, constructor arity, resource allowance, initial fuel, stack limit, register count, scratch count, arithmetic width, limb count, top-limb width, digest width, Poseidon round count, hash input length, stack-record width, domain-tag value, table-tag value, physical row encoding width, scalar cold-read-unit count, cache-line count, ROM-line count, cumulative append capacity, number of slices or number of leaves. It also gives no numeric input, intermediate value, result or example step count. The example total is always `n_steps`.
- **Fixed read capacities.** Each leaf has a fixed number of scalar cold-read units, heap cache lines and ROM lines. Show their installed housings without a numeric capacity label or a geometry count that asserts an unspecified quantity.
- **Mechanism detail absent from the supplied sources.** The architecture treats canonical program admission, including canonical type and constant digests, as a premise rather than giving a complete admission-proof mechanism. The narration explains authentication to the committed image and does not claim that a ROM membership check independently proves source typing or canonical admission. The sources identify `binding` as transcript-bound but do not provide its complete transcript derivation in the requested sections; beat 3.32 explains its supported role without inventing a formula.
- **Hash security.** Digest equality carries the collision-resistance assumption stated by the architecture. Beat 3.28 names that basis explicitly; physical coincidence depicts a verified equality, not a claim that finite digests are mathematically injective.