# Quint check of the chiplet leaf design

Run `sherlock-processor-analogy-20261110`, 2026-11-10. Owner request: "Check your design using
quint". Design under test: `study.md` in this folder (CORE with about 32 register words, chips
DEC, PAGE, MEM, ALU/RANGE, HASH, TRAP as core gates, one bus of `(chip, key, a, b, c)` tuples tied
by `lookup_any` set inclusion, write-once memory with linked frames, S slots, fixed chip heights).
Context: GP-1 in `openspec/changes/sherlock-vm-consensus-20261107/general-purpose.md`; conventions
from `models/sherlock/quint/leafSlots.qnt` and `aggregation.qnt`.

Model: `models/sherlock/quint/chipLeaf.qnt`, tests `models/sherlock/quint/chipLeaf_test.qnt`.
Independent Gemini model: `qa/gemini-quint/chipleaf/`. Quint 0.32.0, Rust backend; every command
ran as `heavy-run --mem 4G --name chip-quint -- quint ...`.

## 1. Answer

Set inclusion without multiplicities is sound for this design, under eight constraints that
`study.md` does not state (D1 to D8 below). Each constraint has a counterexample when it is
missing. The study's claim in 1.4 ("every chip is a function of its key") is the right
condition, but it holds only if the table side of the lookup contains provider rows alone, if no
two providers can answer the same key, and if every MEM row that reaches the memory root was
requested. With D1 to D8 in place, no sampled witness broke refinement, bus soundness, memory
consistency, frame consistency, capacity or shape (20,000 samples per invariant per instance;
section 4). Nothing in the model needs a use-once (multiplicity) argument: no row is consumed,
because memory is write-once and `len` names every append address exactly once.

Where multiplicities would matter, and why they do not here:

- MEM: an append is a lookup of `(CELL, len, x)`. It does not "write"; it asserts that the
  prover-chosen row at the fixed address `len` holds `x`. That is sound because `len` advances by
  one per append (each address is asserted by one step), the address column is `base + fixed
  index` with a single `base` (one row per address), and a read needs `addr < len` at the time of
  the read (the row was asserted earlier). Drop any of the three and a counterexample appears
  (`no_len_inc`, `base_per_row`, `no_read_bound`, `read_bound_final`).
- Frames: pop and rewrite never free or overwrite a row; they move `fp`. A stale frame is
  reachable only through a wrong address, which the core fixes from `fp` (`load_key_free`,
  `rewrite_no_move`).
- DEC, ALU, PAGE, HASH: rows are facts, and a fact can be used any number of times. Unrequested
  rows are harmless if they satisfy the row gate (`orphan_ok` accepted and sound).
- The one place where an unrequested row did harm is MEM: a MEM row above `post.len` that enters
  `mem_root` is a cell nobody appended (`root_all_rows`). D3 closes it.

## 2. What the model contains

- CORE (study 2.1, 2.7): registers `pc, r0, r1, len, fp, depth, status, out` over S slots, with the
  slot contract of `leafSlots.qnt` (one enable per slot, enable prefix, chain, `steps`, publication
  last). A step is FETCH, an ALU `LT` request for LOAD (the `addr < len` check), the DEC lookup of
  the class, then the rule's micro-ops. The core recomputes every query field from the registers
  and earlier answers; the prover supplies only answer fields.
- Abstract ISA (programs are data behind `prog_root`): LIT, ADD, ALLOC, LOAD, PUSH, POP, SETTOP
  (rewrite of the top frame), JNZ, HALT. LOAD out of range and POP or SETTOP on an empty stack set
  `status := Err`; the TRAP class fires on the next step and publishes, as in study 2.7.
- Chips, each its own row gate plus the bus interface:
  PAGE (2.3) PROG rows equal the committed program and CELL rows equal the pre-leaf memory; the
  memory tree is sparse (an unallocated leaf opens to a default cell), the worst case of the open
  tree shape in study section 7. MEM (2.4) has `N_MEM` rows with key `base + k` and a free payload.
  ALU (2.5) rows satisfy `z = f(op, x, y)`. DEC (2.2) is a fixed table, one row per class.
  HASH (2.6) is folded into exact maps: PAGE membership and the memory root are modelled as the
  committed lists (premise: Poseidon collision resistance; Merkle openings bind index and value).
- Bus: requests are the tuples of enabled slots; the table side is the union of PAGE, MEM, ALU and
  DEC rows plus the zero tuple that pads every fixed-height region. Inclusion is set membership.
- Memory: one append log; frames are entries `(value, link)`. PUSH appends `(v, fp)`; POP loads
  `mem[fp]` and follows the link; SETTOP loads `mem[fp]`, appends `(v, link)` and moves `fp`. The
  core does not cache the top frame (study 1.3 caches `top`; the uncached form puts every frame
  read on the bus, which is the case to check).
- Statement: post registers and the post memory commitment (pre cells plus MEM rows below
  `post.len`).
- Reference: `refStep`, a machine with a mutable list stack and the same append-only layout.
- Witnesses: an honest leaf from a random 6-row program after 0 to 4 warm-up steps, with a random
  close (prover choice, chip full, publication), plus at most one tamper. Tampers: `forged` (a
  bus answer from the wrong PROG, CELL, ALU or DEC value), `page_forge`, `wrong_alu`, `dec_lie`,
  `pad_match` (answer equal to the padding tuple), `wrong_chip` (CELL answer served by a row in the
  ALU region), `page_overlap` (a cell appended in this leaf answered by PAGE with the default
  leaf), `dup_mem` (a second MEM row for an address), `stale` (read of an older cell), `replay` (a
  row of an earlier step), `mid_step` (last micro-op row dropped), `wrong_pc`, `overflow` (a step
  past MEM capacity), `after_publish`, `orphan_ok`, `orphan_bad`, `orphan_mem` (unrequested rows),
  `append_via_page` (an append whose payload equals the empty leaf answered by PAGE, the MEM row
  left free; added after the Gemini lane's F3).
  A read of a never-written cell is the honest behaviour of the `no_read_bound` and
  `read_bound_final` mutants.

Not modelled: field arithmetic and range checks (integers stand for range-checked values), HASH
rows and path layout, the cached `top` and `task` registers, the 108 real rules (E0(c) is open),
the node and decider, and the admission claim on `prog_root` (premise: the committed program is
canonical).

## 3. Invariants

| Invariant | Statement |
|---|---|
| `invRefine` | An accepted leaf has `1 <= steps <= S`, enabled slots form a prefix, and post registers and the post memory commitment equal `steps` reference steps from the pre state. |
| `invFrames` | The stack read from committed memory through `fp` and links equals the reference stack. |
| `invTerminal` | Every enabled step starts from a steppable reference state (publication is last). |
| `invBus` | Every request is in the region of its tag (PROG and old CELL: PAGE; new CELL: MEM; ALU; DEC) and equals that chip's function of its input columns. |
| `invMem` | Every read addresses the reference's cell (LOAD: the address register; POP, SETTOP: the reference top frame), that cell is below `len` at the read, and the value is its latest write (for frames, the reference top value). |
| `invCapacity` | Region heights are the constants, appends fit in MEM, every enabled step records all its micro-op rows and every append is a MEM row, a disabled slot records nothing and keeps its registers. |
| `invShape` | The key constants (S, heights, DEC table, tags) do not depend on the program or memory. |
| `invFunctional` | Two provider rows (PAGE, MEM, ALU, DEC, padding) that agree on tag and input columns are equal. This is the condition under which set inclusion needs no multiplicity. |
| `invTamperRefused` | Base only: an accepted witness carries no tamper except the harmless ones (`orphan_ok`, `orphan_mem`). |
| `invComplete` | Base only: every honest witness is accepted. |

`invShape` is structural: it checks that no key constant is computed from the witness's program
or memory. It does not prove GP-1; it shows that the design puts nothing program-dependent into
the key.

## 4. Results

`quint typecheck` is clean for both files. `quint test`: 4 tests pass in each of the 14 test
modules (`chipLeaf_test` and one per mutant scenario).

Every cell below is one `quint run --invariant <inv> --max-samples 20000 --max-steps 1 --seed
20261110` (20,000 traces). `pass` = no violation in 20,000 samples; `VIOL (t)` = violation found,
with `t` the tamper of the counterexample (`none` = an honest witness of the mutated circuit).
Instances: `chipLeafBase` S = 3, `N_MEM` = 2, `N_PAGE` = 4, `N_ALU` = 2; `chipLeafS1` S = 1;
`chipLeafS5` S = 5, `N_MEM` = 4, `N_PAGE` = 8, `N_ALU` = 3. Each `chipLeaf_<mutant>` is the base
instance with one constraint dropped (or, for `mem_unbounded`, the MEM height freed).
`invTamperRefused` and `invComplete` apply to the unmutated instances only.

| Module | Refine | Frames | Terminal | Bus | Mem | Capacity | Shape | Functional | TamperRefused | Complete |
|---|---|---|---|---|---|---|---|---|---|---|
| `chipLeafBase` | pass | pass | pass | pass | pass | pass | pass | pass | pass | pass |
| `chipLeafS1` | pass | pass | pass | pass | pass | pass | pass | pass | pass | pass |
| `chipLeafS5` | pass | pass | pass | pass | pass | pass | pass | pass | pass | pass |
| `chipLeaf_no_inclusion` | VIOL (page_overlap) | VIOL (dec_lie) | pass | VIOL (page_overlap) | VIOL (page_overlap) | VIOL (overflow) | pass | pass | - | - |
| `chipLeaf_table_ungated` | VIOL (page_overlap) | VIOL (dec_lie) | pass | VIOL (page_overlap) | VIOL (page_overlap) | VIOL (overflow) | pass | pass | - | - |
| `chipLeaf_tag_unbound` | VIOL (wrong_chip) | pass | pass | VIOL (wrong_chip) | VIOL (wrong_chip) | pass | pass | VIOL (wrong_chip) | - | - |
| `chipLeaf_tag_zero` | VIOL (page_overlap) | pass | pass | VIOL (page_overlap) | VIOL (page_overlap) | pass | pass | VIOL (none) | - | - |
| `chipLeaf_page_unauth` | VIOL (page_forge) | VIOL (page_forge) | pass | VIOL (page_forge) | VIOL (page_overlap) | VIOL (append_via_page) | pass | VIOL (page_overlap) | - | - |
| `chipLeaf_page_no_base_bound` | VIOL (page_overlap) | VIOL (append_via_page) | pass | VIOL (page_overlap) | VIOL (page_overlap) | VIOL (append_via_page) | pass | VIOL (page_overlap) | - | - |
| `chipLeaf_base_per_row` | VIOL (dup_mem) | pass | pass | VIOL (dup_mem) | VIOL (dup_mem) | pass | pass | VIOL (dup_mem) | - | - |
| `chipLeaf_dec_dup` | VIOL (dec_lie) | VIOL (dec_lie) | pass | VIOL (dec_lie) | pass | pass | pass | VIOL (none) | - | - |
| `chipLeaf_alu_gate_off` | VIOL (wrong_alu) | pass | pass | VIOL (wrong_alu) | pass | pass | pass | VIOL (wrong_alu) | - | - |
| `chipLeaf_pc_unbound` | VIOL (wrong_pc) | pass | pass | pass | pass | pass | pass | pass | - | - |
| `chipLeaf_load_key_free` | VIOL (stale) | VIOL (stale) | pass | pass | VIOL (stale) | pass | pass | pass | - | - |
| `chipLeaf_no_read_bound` | VIOL (none) | VIOL (none) | VIOL (none) | VIOL (none) | VIOL (none) | VIOL (none) | pass | pass | - | - |
| `chipLeaf_read_bound_final` | VIOL (none) | VIOL (none) | VIOL (none) | VIOL (none) | VIOL (none) | VIOL (none) | pass | pass | - | - |
| `chipLeaf_no_len_inc` | VIOL (none) | VIOL (none) | pass | pass | VIOL (none) | VIOL (none) | pass | pass | - | - |
| `chipLeaf_rewrite_no_move` | VIOL (none) | VIOL (none) | pass | VIOL (none) | VIOL (none) | pass | pass | pass | - | - |
| `chipLeaf_per_row_enable` | VIOL (mid_step) | VIOL (mid_step) | pass | pass | pass | VIOL (mid_step) | pass | pass | - | - |
| `chipLeaf_root_all_rows` | VIOL (none) | pass | pass | pass | pass | pass | pass | pass | - | - |
| `chipLeaf_mem_unbounded` | pass | pass | pass | pass | pass | VIOL (overflow) | VIOL (overflow) | pass | - | - |
| `chipLeaf_trap_off` | VIOL (none) | pass | VIOL (none) | pass | pass | pass | pass | pass | - | - |
| `chipLeaf_push_link_free` | VIOL (none) | VIOL (none) | pass | VIOL (none) | VIOL (none) | pass | pass | pass | - | - |
| `chipLeaf_prog_in_key` | pass | pass | pass | pass | pass | pass | VIOL (none) | pass | - | - |

Coverage in the base instance (`--witnesses`, 20,000 traces, seed 20261110): full S steps 23.1%,
early close by prover choice 66.7%, close because a chip is full 2.0%, publication `Halted`
16.6%, publication of an error 25.2%, LOAD from MEM 1.6%, LOAD from PAGE 8.4%, POP after a
rewrite 0.9%, POP of a frame from an earlier leaf 11.9%, unrequested rows accepted 15.9%, tamper
refused 41.7%.

Mutants: 21 seeded, 21 caught, each on its target invariant: `no_inclusion` (Refine),
`table_ungated` (Bus), `tag_unbound` (Bus), `tag_zero` (Functional), `page_unauth` (Bus),
`page_no_base_bound` (Functional), `base_per_row` (Functional), `dec_dup` (Functional),
`alu_gate_off` (Refine), `pc_unbound` (Refine), `load_key_free` (Mem), `no_read_bound` (Mem),
`read_bound_final` (Mem), `no_len_inc` (Refine), `rewrite_no_move` (Frames), `per_row_enable`
(Capacity), `root_all_rows` (Refine), `mem_unbounded` (Capacity), `trap_off` (Terminal),
`prog_in_key` (Shape), `push_link_free` (Frames). `invTamperRefused` is the mutant check for the
base relation itself: every one of the 16 non-harmless tampers is refused.


## 5. Counterexamples

Each trace is the counterexample of the target-invariant run in section 4 (seed 20261110).
Tuples are `(chip, key, a, b, c)` with PROG = 1, CELL = 2, ALU = 3, DEC = 4; cells are
`(value, link)`; NIL = -1. "Ref" is the reference machine.

The inclusion-without-multiplicity question:

1. **Two providers for one CELL key** (`page_no_base_bound`, tamper `page_overlap`; design gap D2).
   Genesis, PUSH r0 appends `(0, NIL)` at address 0 in MEM. Next step SETTOP loads `fp = 0`; the
   prover answers with `(CELL, 0, 0, 0)`, a PAGE opening of the empty leaf 0 of the sparse
   pre-leaf tree (base is 0, so the leaf is unallocated there). The link read is 0 instead of NIL,
   the new top is appended as `(0, 0)` and the committed stack is wrong. Both rows
   `(CELL, 0, 0, NIL)` (MEM) and `(CELL, 0, 0, 0)` (PAGE) are on the table: `invFunctional` fails.
   Gemini F3 is the write-side variant (an append of the default value matched by PAGE, the MEM
   row left free); it is tamper `append_via_page` here and is caught on the same mutant.
2. **Unrequested MEM row in the root** (`root_all_rows`, honest witness; D3). From genesis: PUSH
   (address 0), POP. `len = 1`, but MEM row 1 `(CELL, 1, 0, 0)` was never requested and enters
   `memOut = [(0, NIL), (0, 0)]`; Ref memory is `[(0, NIL)]`. No tamper is needed: padding rows
   alone corrupt the next leaf's memory.
3. **Self-match on an ungated table** (`table_ungated`, tamper `page_overlap`; D1). POP from an
   old frame at address 0 is answered `(CELL, 0, 0, 0)`, a tuple no provider holds; with requests
   on the table side it matches itself. Ref pops `(0, NIL)`; the leaf ends with the wrong link.
   `no_inclusion` gives the same trace (inclusion dropped altogether).
4. **Two MEM rows for one address** (`base_per_row`, tamper `dup_mem`; D6). SETTOP appends
   `(0, NIL)` at address 1; a padding row also carries key 1 with `(3, NIL)`. The next POP of
   address 1 is answered by the second row: `r0 = 3`, Ref `r0 = 0`.
5. **Padding tuple as an answer** (`tag_zero`, honest witness; D8). With `CELL = 0`, MEM row 0
   `(0, 0, 0, NIL)` and the padding tuple `(0, 0, 0, 0, 0)` share the input `(0, 0)`; any load of
   address 0 may take the padding value. `invFunctional` fails on the first PUSH.
6. **Mid-step close** (`per_row_enable`, tamper `mid_step`; D5). PUSH r0 records FETCH and DEC
   but not its append; the slot's post still has `len = 2, fp = 1`. MEM row 1 is free and holds
   `(5, 5)`, which enters the root: Ref memory has `(0, 0)` at address 1.
7. **Stale frame after a rewrite** (`rewrite_no_move`, honest witness; D7). SETTOP appends the
   new top `(1, 0)` at address 3 but leaves `fp = 2`. The committed stack reads the old copy at
   address 2 (value 0); Ref top is 1.
8. **Wrong chip** (`tag_unbound`, tamper `wrong_chip`; D4). SETTOP's load of `fp = 1` is
   answered `(CELL, 1, 4, 0)` by a row placed in the ALU region with tag CELL; the true cell is
   `(1, 0)`. Not detectable without a fixed tag column.

Confirmed constraints (already in the study):

9. `no_read_bound` / `read_bound_final`: `LOAD r1` with `r1 = 1 = len`. Ref errors (LOADOOB);
   the mutant reads MEM row 1, never written, value `(0, 0)`. With the bound at `len + N_MEM`
   the LT row `(ALU, LT, 1, 3, 1)` is true and the same unwritten read follows.
10. `no_len_inc`: PUSH appends at address 1 without moving `len`; the following SETTOP reads
    address 1 and appends at address 1 again. All three requests `(CELL, 1, 0, 0)` match one MEM
    row, two writes collapse into one cell and `len` stays 1 (Ref 3). This is the only use-once property the design needs, and `len + 1` provides it.
11. `load_key_free`: POP reads address 1 (an ALLOC data cell `(0, NIL)` appended in the same
    leaf) instead of the frame at `fp = 0`. Here the two cells hold equal values, so only
    `invMem` sees it; in the `invRefine` counterexample the stale value reaches a register.
12. `push_link_free`: PUSH stores link 0 instead of `fp = 3`; the committed stack skips two frames.
13. `alu_gate_off`: ADD answers `(ALU, ADD, 0, 0, 1)`; `r0 = 1`, Ref `r0 = 0`.
14. `dec_dup`: DEC rows `(70 -> 70)` and `(70 -> 71)` both exist; `invFunctional` fails on the
    table, and the `dec_lie` tamper turns a valid SETTOP into SETTOPEMPTY (an error).
15. `pc_unbound`: LIT with `pc` 1 -> 3 instead of 2.
16. `trap_off`: a leaf from an `Err` state executes SETTOPEMPTY three times instead of
    publishing; Ref publishes in its first step and stops.
17. `mem_unbounded`: three appends with `N_MEM = 2` accepted with a third MEM row; the key height
    changes (`invShape` and `invCapacity`).
18. `prog_in_key`: the PROG table in the key differs per program (`invShape`).
19. `page_unauth`: a PROG row with the wrong register operand (`(PROG, 3, 5, 1, 0)` for
    `PUSH r0`) is accepted; the pushed value is `r1 = 1`, Ref pushes `r0 = 2`.


## 6. Gemini lane

An independent Gemini 4 Argon lane (`argon-run`, `gemini-4-argon-high`, quint-modeling and
quint-lang skills, conversation `b2634d33-9b0c-4b30-aee3-1d80ea8fd47a`, 6 turns, ended `DONE`)
modelled the design from `study.md` alone in a detached worktree under
`~/.cache/gemini-lanes/chipleaf/`. It did not see this model. Its files are copied to
`qa/gemini-quint/chipleaf/` (`chipleaf.qnt`, `chipleaf_test.qnt`, `REPORT.md`). The lane's report
is input; the evidence is the quint runs below.

Repairs needed before its model ran (mechanical, no change of meaning): `coreTransitionsValid`
lacked a closing brace and needed braces around a lambda body with `val`; `range(..).forall` became
`0.to(..).forall`; the test file's import got its `from "./chipleaf"` path; one test's
`then(all { assert(..) })` became `expect(..)`.

Runs on the Gemini model (`quint typecheck` clean for both files; `quint test --main chipleaf_test`:
19 passing; `quint run --max-samples 20000 --max-steps 1 --seed 20261110`):

| Module | Invariant | Result |
|---|---|---|
| `chipleafHardened`, `chipleafS2`, `chipleafS4` | `invSound` | pass (20,000 each) |
| `chipleafHardened` | `invTamperRefused` | pass |
| `chipleafHardened` | `reachShort`, `reachTerminalShort`, `reachFull`, `reachPushPop` | violated as intended (shapes reachable) |
| `chipleafAsWritten` | `invSound` | violation (design as written is unsound) |
| 12 of 13 `chipleaf_gap_*` mutants | `invSound` | violation (caught) |
| `chipleaf_gap_dup_mem_addr` | `invSound` | pass: mutant NOT caught |

Two limits of the Gemini evidence. (1) Its sampler draws from 13 hand-built counterexamples and
honest witnesses over 5 start values and 3 lengths, so 20,000 samples revisit about 28 fixed
witnesses; the passes are close to exhaustive over that list and say nothing beyond it. (2) Its
`gap_dup_mem_addr` mutant drops only `memAddressFixed`, but the hand-built witness is also
refused by the active-prefix and root-fold constraints, so the mutant survives. Its unit test
`cex11_duplicateMemAddrTest` passes because it checks `acceptsAsWritten`, not the mutant. The
finding itself (F11) is real: this model's `base_per_row` mutant is caught on `invFunctional`
and `invRefine` by the `dup_mem` tamper.

Verdicts of the lane: as written, SOUND: no; with its constraints C-BUS-1 to C-CORE-1, sound.
COMPLETE: no (capacity per step, root geometry, GP gaps). This matches section 1.

Reconciliation, every finding of either model:

| Gemini | Finding | This model | Resolution |
|---|---|---|---|
| F1 | Ungated self-lookup: a request matches itself | `table_ungated` caught | Agreed: D1 |
| F2 | Unrequested MEM rows enter `mem_root` | `root_all_rows` caught | Agreed: D3 (Gemini states it as an active prefix of MEM rows; same constraint) |
| F3 | PAGE answers an append whose payload equals the empty leaf; the MEM row stays free | not in the first version; added as tamper `append_via_page`, refused by base, caught on `page_no_base_bound` (test `mutantCaughtTest`, scenario P3) | Agreed: D2 |
| F4 | Stale read: a cell appended in this leaf read through PAGE as the empty leaf | `page_overlap` on `page_no_base_bound` caught | Agreed: D2 |
| F5 | Read of a never-written cell (len advances by more than one, or `a < len` against the end-of-step or end-of-leaf `len`) | `no_read_bound`, `read_bound_final` caught | Agreed; already in the study, keep explicit |
| F6 | Wrong ALU result or flag | `wrong_alu` on `alu_gate_off` caught | Agreed. Note: study 2.5 packs the flag into `op`, so the flag sits in the key column; the ALU gate must fix it from `(op, x, y)`, otherwise two rows differ only in the flag |
| F7 | Wrong next `pc` | `wrong_pc` on `pc_unbound` caught | Agreed |
| F8 | MEM overflow with PAGE answering the overflowed append | `overflow` refused by base; with D2 an append above `base + N_MEM` has no provider | Agreed: D2 plus fixed heights |
| F9 | Two appends at one address share one MEM row (len not chained) | `no_len_inc` caught | Agreed; already in the study |
| F10 | Leaf ends mid-step | `mid_step` on `per_row_enable` caught | Agreed: D5 |
| F11 | Two MEM rows for one address | `dup_mem` on `base_per_row` caught | Agreed: D6 |
| F12 | Study 1.3 (cached `top`, `fp` = frame below) and 2.4 ("a rewrite appends with the same link", `fp` to the new entry) disagree; mixing them makes POP return the rewritten top | `rewrite_no_move` caught for the uncached form | Agreed: D7 must pick one protocol and state it |
| F13 | Forged frame link on PUSH | not in the first version; added as mutant `push_link_free` (link from the bus), caught | Agreed: the link is the core's `fp` |
| F14 | Completeness: per-step demand above a chip height has no witness; the `mem_root` update budget (about 8 permutations per step) does not match a depth-20 Merkle tree that PAGE opens by path | not modelled (HASH folded into maps) | Accepted as an open cost and consistency item, not a soundness gap: the root must be an append-only Merkle tree whose frontier update costs about `depth` permutations per appended batch; section 4.1 of the study undercounts it. E4 must measure it |
| - | GP-1..GP-4 not met by the leaf alone (admission claim open, one outcome type, no GP-2..GP-4) | `invShape` only | Agreed, out of scope of the leaf check |
| this model only | Padding tuple vs chip tag 0 | `tag_zero` caught | D8; Gemini uses a non-zero `NO_REQ` tag, so it did not meet it |
| this model only | Chip tag of a region must be fixed (a CELL row in the ALU region) | `tag_unbound` caught | D4; Gemini states it inside C-BUS-1 |
| this model only | DEC one row per class; read address from a register; TRAP class; fixed heights; program not in the key | `dec_dup`, `load_key_free`, `trap_off`, `mem_unbounded`, `prog_in_key` caught | Already in the study; confirmed |
| this model only | Inclusion without multiplicity needs no use-once argument for frames | `invFunctional`, `invFrames` hold in base | Gemini reaches the same conclusion (its section 2 proof sketch) |

No finding of either model is left open as a soundness gap once D1 to D8 are adopted.


## 7. Required design changes

Text for `study.md` 1.4, 2.3, 2.4, 2.7 and the proposed `design.md` 2.7. D1 to D8 are needed
for soundness; the model has a counterexample for each when it is missing. The last group is
already in the study and the model confirms that each item is load-bearing.

New constraints:

- **D1 Provider-gated table side** (1.4). The `lookup_any` table expression is
  `q_provider * tuple`, with `q_provider` a fixed selector that is 1 only on active chip rows and 0
  on core request rows; the input side is `q_request * tuple`. Without it a request matches
  itself (`table_ungated`).
- **D2 One provider per key** (2.3, 2.4, 7). PAGE serves `CELL` only for `index < base`, with
  `base = len` from `pre_d`; or PAGE and MEM use different tags (`OLD`, `NEW`); or the memory tree
  commits its length and refuses openings at or above it. With a sparse tree and no bound, PAGE
  opens the default leaf for a cell appended in this leaf and the read gets the default
  (`page_no_base_bound`), or an append of the default value is answered by PAGE and its MEM row
  stays free (`append_via_page`, Gemini F3). This also settles the open "tree shape of the memory root" in part: the
  tree must bind the length, or D2 must be a gate.
- **D3 The root covers appended rows only** (2.4). The `mem_root` update hashes MEM rows
  `0 .. post.len - base - 1` and no others; rows above are padding outside the root. Otherwise a
  MEM row that no append requested enters the next leaf's memory (`root_all_rows`).
- **D4 Fixed chip tag per region** (1.2, 1.4). The `chip` column of a provider row is a fixed
  column (the region's tag), not advice. Otherwise a row in the ALU region tagged `CELL` answers a
  load (`tag_unbound`).
- **D5 One enable per step** (1.5, 2.1). All micro-op rows of a slot share the slot's `enable`;
  a slot either records all requests of its rule or none, and a leaf closes only between steps.
  With per-row enables, a step that is cut after its register update but before its append
  publishes `len + 1` over an unconstrained MEM row (`per_row_enable`).
- **D6 One base for the MEM region** (2.4). `base` is one cell copied from `pre_d.len` and every MEM
  row's address is `base + k` with `k` a fixed column. A per-row base gives two rows for one
  address and the read picks either (`base_per_row`).
- **D7 One frame protocol** (1.3, 2.4, 7 item 2.7.3). The study mixes two: 1.3 caches `top` in
  registers with `fp` at the frame below; 2.4 appends every rewrite of the top "with the same
  link". Pick one and state it. Uncached (this model): a rewrite appends `(v, link)` and sets
  `fp` to the new entry; otherwise the next POP reads the old copy (`rewrite_no_move`). Cached
  (Gemini's choice): PUSH spills the old `top` with link `fp`, POP loads `mem[fp]`, and a rewrite
  changes only the register; appending a rewrite under the cached reading makes POP return the
  rewritten top (Gemini F12). In both, the stored link is the core's `fp`, never a bus value
  (`push_link_free`, Gemini F13).
- **D8 No tag equals the padding tuple** (1.4). Disabled and padding rows put `(0, 0, 0, 0, 0)` on
  the table side, so no chip tag is 0 and no request has an all-zero tuple. With `CELL = 0`, a
  load of address 0 matches padding (`tag_zero`).

Confirmed load-bearing (already in the study; keep them explicit in `design.md`):

- `len` advances by exactly one per append (`no_len_inc`); the study's "write-once" argument rests
  on this alone.
- A read checks `addr < len` against the current `len` of the step, not the leaf's final `len` or
  the MEM region's end (`no_read_bound`, `read_bound_final`).
- The read address comes from a register (`fp`, or the address operand), never from the bus
  (`load_key_free`).
- Every active ALU row satisfies its relation, whether requested or not (`alu_gate_off`); every PAGE
  row opens against its root (`page_unauth`); DEC has one row per class (`dec_dup`); the next `pc`
  and every named register are core constraints (`pc_unbound`); TRAP gates the class and a
  publishing step is the last enabled slot (`trap_off`); heights are constants (`mem_unbounded`);
  no program row is a fixed table (`prog_in_key`).

Clarifications that the model shows are not needed:

- No multiset, shuffle, LogUp or timestamp argument is needed for this design (section 1).
- "A leaf ends ... when a chip fills" is a prover rule, not a circuit gate. The circuit needs no
  per-chip counter: a step whose requests do not fit has an unmatched request and cannot be
  enabled, and an early close is a shorter, still correct segment. Because set inclusion
  deduplicates, ALU and PAGE fill by distinct rows, so the honest prover counts distinct rows.
- Range checks on `len` and addresses (so that `addr < len` is an integer comparison in the field)
  are outside the model and stay an obligation of RANGE (2.5).


## 8. Commands

```
cd models/sherlock/quint
heavy-run --mem 4G --name chip-quint -- quint typecheck chipLeaf.qnt
heavy-run --mem 4G --name chip-quint -- quint typecheck chipLeaf_test.qnt
heavy-run --mem 4G --name chip-quint -- quint test chipLeaf_test.qnt --main chipLeaf_test
heavy-run --mem 4G --name chip-quint -- quint test chipLeaf_test.qnt --main chipLeaf_test_<mutant>
heavy-run --mem 4G --name chip-quint -- quint run chipLeaf.qnt --main <module> --invariant <inv> \
  --max-samples 20000 --max-steps 1 --seed 20261110
heavy-run --mem 4G --name chip-quint -- quint run chipLeaf.qnt --main chipLeafBase --invariant invAll \
  --max-samples 20000 --max-steps 1 --seed 20261110 --witnesses reachFull reachShortChoice \
  reachCapacityEnd reachPublishHalt reachPublishErr reachLoadMem reachLoadPage \
  reachPopAfterRewrite reachPopOldFrame reachOrphanAccepted reachTamperRefused
```

`--max-steps 1`: each trace is one fresh witness (`init` and `step` both sample), as in
`leafSlots.qnt`; 20,000 samples are 20,000 traces of two witnesses each.
