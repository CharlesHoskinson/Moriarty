# Sherlock leaf as a chip: CPU core, chiplets and one bus

Run `sherlock-processor-analogy-20261110`. Read-only study; nothing was built, run or proven.
Every row figure not quoted from a repository record is marked ESTIMATE with its arithmetic;
figures from the brief are marked MEASURED (brief). Scope: the dispatch facts block
(`c6_dispatch_step.rs:608-837`), the PROG walk and the checked value read were read. The step
fragment bodies were not, so the rule-to-chip map in section 2 is from the dispatch groups and
rule names, and the micro-operation claim in 2.2 is unproven.

Source keys: `MP` = `tools/profile/vendor/midnight-proofs-0.8.3/src`; `MC` =
`~/.cargo/registry/src/*/midnight-circuits-7.3.0/src`; `ZS` =
`~/.cargo/registry/src/*/midnight-zk-stdlib-2.4.0/src`; `ZKIR` =
`~/.cargo/git/checkouts/midnight-zkir-*/446c7fe/zkir/src`; `UP` =
`repos/midnightntwrk/midnight-zk` (upstream head `24a1eb5a`; same version strings as the pinned
crates, different code; cited only for "upstream has").

## 1. Architecture

### 1.1 Why a chip

The leaf today is a combinational circuit over the whole state: 1,726 lanes per side (MEASURED
(brief)), each step range-checks 343 lanes, walks all of PROG four times and multiplexes over
every frame, task and data slot (`c6_dispatch_step.rs:612-614,628,632-647,716,738-792`;
`c6_checked_value_read.rs:327,358,402-413`). One step is 1,794,327 to 5,670,286 rows (MEASURED
(brief)). A processor cycle sees a few registers and reaches everything else by address. The
design below does the same.

### 1.2 Block diagram

```
 pre_d --> [ CORE: S step slots; registers; issues micro-ops ] --> post_d
                |            one BUS: (chip, key, a, b, c)
     +----------+-----------+-----------+-----------+-----------+
   [DEC]      [PAGE]      [MEM]       [ALU]      [HASH]     (TRAP: gates in CORE)
   control    PROG rows   cells       128-bit    Poseidon
   words,     and old     appended    add, sub,  compress;
   fixed      cells,      in this     mul, cmp;  digests,
   table      opened vs   leaf;       RANGE      Merkle
              a root      stack       limbs      paths
                          links
```

All chips are row regions of one PLONK circuit, stacked in the same advice columns under a
fixed chip-selector column (as Miden stacks its chiplets). Stacking keeps the column count, and
so the node's cost, low (section 4.3).

### 1.3 The CPU core

Registers, as packed words (small fields share a word; a 128-bit value has its own):

| Register | Content today (`c6_phys.rs:48-70`, `c6_phys_helper.rs:62-70`) | Words |
|---|---|---|
| `ctl` | control record, 17 lanes, and control origin | 5 |
| `st` | outcome (3 lanes), fuel and five counters, stack limit, frame count | 8 |
| `env` | environment root | 1 |
| `fp`, `tp`, `len` | top-frame address, top-task address, memory log length | 3 |
| `top`, `task` | cached top frame (13 lanes) and top helper task (7 lanes) | 5 |
| `h`, `r` | helper record (23 lanes), result record (4 lanes) | 7 |
| `v0..v2` | value registers for rows returned by the bus in this step | 3 |

About 32 words (ESTIMATE: 5 + 8 + 1 + 3 + 5 + 7 + 3), against 1,726 lanes per side. `pre_d` and
`post_d` digest `ctl..r`, `mem_root` and `prog_root`.

The ISA the core issues, at most one micro-op per core row:

| Micro-op | Bus request | Effect on registers |
|---|---|---|
| `FETCH i` | PAGE `(PROG, i, ...)` | `v := PROG row i` |
| `LOAD a` | MEM or PAGE `(CELL, a, ...)`, with `a < len` | `v := cell a` |
| `APPEND x` | MEM `(CELL, len, x)` | `len := len + 1` |
| `PUSH`, `POP` | `APPEND` of `top` with link `fp`; `LOAD fp` | `fp`, `top` (or `tp`, `task`) |
| `ALU op` | ALU `(op, x, y, z)` | `v := z`, flag |
| `MOVE`, `SETC`, `INC` | none | copy a field, set a constant, add one |
| `HALT` | none | no-op after the outcome leaves `Running` |

A Sherlock step is: TRAP check, DEC lookup of the control word for the selected rule, then the
micro-ops that word names. One rule per step, as today.

### 1.4 The bus and why no balancing argument is needed

One tuple format for every chip: `(chip, key, a, b, c)`, five field elements. The core row puts
a request tuple in five columns; the chip row that serves it holds the same tuple in the same
five columns. The tie is one `lookup_any` of the column set on itself, gated by a request
selector, exactly the pattern of `MC:ecc/foreign/common.rs:204-225` ("a lookup of a column
(set) on itself"). Tag isolation follows `MC:field/decomposition/pow2range.rs:197-203`.

The pinned lookup is the halo2 permuted-column argument: set inclusion, not a multiset balance
(`MP:plonk/lookup.rs:40-66`). That is sufficient here because **every chip is a function of its
key**: DEC, ALU and HASH are stateless relations; PAGE rows are authenticated against a root;
MEM addresses are fixed by the circuit and no cell is overwritten. A request that matches any
provider row is therefore true, however often it matches. No send/receive balance, multiplicity
or timestamp is needed.

A LogUp or permutation-balanced bus and offline memory checking are **not available** on the
pinned stack: there is no shuffle argument in midnight-proofs 0.8.3, and the node's verifier
gadget accepts no user challenge phases (`MC:verifier/verifier_gadget.rs:16-17`; no code in
`MC:verifier/` reads `challenge_phase`). Section 3.

### 1.5 One key set

The circuit fixes the height of every chip region: `S x H_core` core rows, `N_page`, `N_mem`,
`N_alu`, `N_hash` chip rows, the DEC and RANGE tables. None depends on a program. A leaf closes
when S steps are done or when any chip region is full, whichever is first: the remaining slots
are disabled (`enable == 0`, the prefix rule of `design.md` 2.2(5),(7)), as zkVM shards close.
A step that needs more of one chip than a whole leaf has cannot be proven; the heights are
chosen above the per-step maximum of every rule (E0). Program size, memory size and run length
are behind `prog_root`, `mem_root` and the height, so the key set does not depend on them (GP-1).
The S steps of a leaf share every chip table: a PROG row or old cell is opened once per leaf
and read by any number of steps.

### 1.6 Size (ESTIMATE; arithmetic in section 4)

230 to 802 rows per step plus page-ins; a leaf of S = 16 steps is 15,488 to 42,240 rows,
**k = 14 to 16**. Node 768,708 to 813,708 rows at k = 20. G-C: fact(3) is 19 leaves and 19 node
steps, 19.3 to 21.2 minutes of node time.

## 2. The chips

Each chip has one job, the bus tuple as its only interface, a soundness obligation that can be
stated and model-checked without the others, and a row budget. "Quint" names the model to
write; composition is one more model that assumes each chip's obligation.

### 2.1 CORE

- Job. Carry the registers across S slots and issue micro-ops. Never reads a table directly.
- Interface. Issues `(chip, key, a, b, c)` requests; public statement as today.
- Rules. All 108.
- Constraints. Slot 0 registers open `pre_d`; slot S-1 registers close `post_d`; between
  micro-op rows each register is unchanged unless the control word names it as a destination;
  `enable` prefix; `steps` = number of enabled slots.
- Obligation. Frame condition: a register not named by the control word keeps its value.
  Range safety by induction: every written value is range-checked when written, and `pre_d`
  comes from a proven `post_d` or from genesis.
- Quint. Extends `models/sherlock/quint/leafSlots.qnt` (slot contract) with registers.
- Rows. H_core = 8 per step, ESTIMATE: 32 words / 12 columns = 3 rows of registers + up to 8
  requests at 2 per row = 4 rows + 1 decode row.

### 2.2 DEC (decoder and microcode ROM)

- Job. Map the dispatch class to the rule and its control word.
- Interface. `(DEC, class, rule_lane, ctrl0, ctrl1)`. `class` packs the trap bits, control tag,
  expression or frame tag, helper kind, phase and status, result tag, and the comparator flags.
- Rules. All 108: the table is `guard` and `path` of `c6_dispatch_step.rs:900-1314` in tabular
  form, in Core order, so first-match precedence is a property of the table.
- Constraints. Fixed table (in the key). The core computes `class` from registers and flags.
- Obligation. For every class there is at most one row (mutual exclusion,
  `c6_dispatch_step.rs:25-28`), and it is the rule Core selects. A class with no row refuses
  (SVM-004). The table is generated from, and tested against, the per-rule fragments.
- Quint. Table as a pure function; invariant: equals `owner_select` on all classes.
- Rows. Fixed, about 512 (ESTIMATE: 108 rules x up to 4 flag variants, rounded up).
- Open. Whether every rule body is a short sequence of the micro-ops of 1.3 is unproven
  (E0(c)). If not, the rule bodies stay as gated arithmetic over registers: about 21,500 more
  rows per step (ESTIMATE: 108 rules x 100 operations, a guess, + 108 x 99 lane selects).

### 2.3 PAGE (authenticated read-only rows)

- Job. Serve PROG rows and memory cells created by earlier leaves.
- Interface. `(PROG | CELL, index, a, b, c)`.
- Rules. PROG: the 13 Eval rules (`eval_row`), the 4 Cases rules, `BHelperMatchEntry` and
  `BHelperMatchArm` (`origin_row`), the 4 Find rules (function rows and count), the 9 Bind rules
  (parameters), and constant reads of Validate and Equal. CELL: any rule that reads a cell older
  than the leaf.
- Constraints. Every active PAGE row carries a Merkle path, issued as HASH requests, from its
  tuple to `prog_root` or to the `mem_root` in `pre_d`.
- Obligation. An active row is a member of the committed table. Duplicated rows are harmless
  (same root, same value). PROG canonicality is not checked here: it is a premise on
  `prog_root`, discharged by an admission claim (open item; today the parser walk checks it in
  every step).
- Quint. Membership against an abstract map; refinement to Merkle is a standard lemma.
- Rows. 1 row plus `depth x 25` HASH rows per distinct row per leaf: 200 at depth 8 (the
  qualification config has at most 403 PROG lanes, MEASURED (brief)), 500 at depth 20.

### 2.4 MEM (append log and stack links)

- Job. Hold the cells appended in this leaf: data nodes, field references, environment cells,
  value slots, and frame and task entries.
- Interface. `(CELL, addr, a, b, c)`; `addr = base + fixed row index`, `base` copied from `len`
  in `pre_d`.
- Rules. Appends: `BHelperEnvPutInsert`, `BHelperBindInsert`, the 5 Append rules,
  `BHelperScalarConstruct`, the construct and constructor-field rules, Materialize, the 2 Push
  rules, and every rule that rewrites the top frame or task. Reads: the 4 EnvGet rules, the
  Validate (7), Equal (5) and Match (9) groups, and the 17 Return rules (pop a frame).
- Constraints. An append requests `(CELL, len, x)` and increments `len`. A read requires
  `addr < len`. A frame or task entry stores the address of the entry below it; pop loads
  `fp` and sets `fp` to the stored link; a rewrite of the top appends with the same link.
- Obligation. Write-once: each address has one value (fixed address column), asserted by
  exactly one append (`len` advances by one). Reads never see an unallocated address. This
  rests on the tables being append-only: in `helper_machine/shared/` the data, data-field,
  environment and value-slot tables are only pushed to (`owner_runtime.rs:3514,3543-3564`,
  `owner_suspend.rs:473-478`; no indexed assignment, `truncate`, `retain`, `insert`, `remove`,
  `swap`, `pop`, `last_mut`, `get_mut` or `iter_mut` on them), `StepInv` says "data arena
  append-only" (`design.md` 2.1), and only frames and tasks are popped
  (`owner_runtime.rs:3575,3588`). The extension and financial owners were not searched.
- Quint. Log as a growing list; invariant: linked stack equals the reference stack.
- Rows. 1 per appended tuple; 6 to 12 per step (ESTIMATE). New cells are added to `mem_root`
  through HASH: about 8 permutations per step, 200 rows; 750 if records are not packed.

### 2.5 ALU and RANGE

- Job. 128-bit checked add, subtract, multiply, compare and equality; limb range checks.
- Interface. `(ALU, op, x, y, z)` with the overflow or comparison flag packed in `op`. RANGE is
  a fixed `(bits, value)` table used inside ALU rows and by writers, as `pow2range` is today.
- Rules. The 5 Scalar rules, the 2 Primitive rules, `KMachinePushAccept`/`Refuse` (depth
  against the stack limit), and the cursor comparisons of Find, Bind and Append
  (`c6_dispatch_step.rs:931,1094,1110-1147`). Every write uses RANGE.
- Obligation. Each ALU row satisfies its `op` over 128-bit operands with the stated flag;
  identical to the existing checked-arithmetic fragments, which are its differential test.
- Quint. Pure functions; checked against `word_arithmetic.rs` semantics.
- Rows. About 8 per operation (ESTIMATE: 3 values x 11 limbs of 12 bits = 33 limbs at 4 per
  row); 16 to 32 per step. RANGE table: 4,096 fixed rows.

### 2.6 HASH

- Job. One Poseidon compression per request.
- Interface. `(HASH, id, in0, in1, out)`.
- Rules. None directly. Serves PAGE paths, the MEM root update, the register digests at the
  leaf boundary, `rules_d` (`design.md` 2.2(6)) and the value digest at publication.
- Obligation. `out = Poseidon(in0, in1)` with the domain tag in `id`; the chip is the existing
  `MC:hash/poseidon` chip behind the bus.
- Rows. 25 per permutation (ESTIMATE: 18 round rows, the lower bound in `architecture.md` fact
  5, plus absorption and output; unmeasured).

### 2.7 TRAP (publication and halt)

- Job. The priority check P0, P1, P2 before decode, and the halted state after it.
- Interface. Gates on `st`, `ctl`, `h`, `r`; it feeds trap bits into the DEC class.
- Rules. `KAdminPublishValue`, `KAdminPublishError`, `KAdminPublishExhausted`
  (`c6_dispatch_step.rs:8-11,1246-1264`). Rules that raise an error only write `ctl`; the trap
  fires on the next step, as today.
- Obligation. A publishing step is the last enabled slot (`design.md` 2.2(8)); after it every
  slot is `HALT`.
- Rows. None beyond the core slot.

## 3. What the stack supports

| Need | midnight-proofs 0.8.3, circuits 7.3.0, stdlib 2.4.0 |
|---|---|
| Multi-column lookup, fixed table | Yes: `lookup` (`MP:plonk/circuit.rs:1860`), tuples compressed by `theta` |
| Lookup into advice columns (the bus) | Yes: `lookup_any` (`MP:plonk/circuit.rs:1889`); precedents `MC:ecc/foreign/common.rs:204`, `MC:parsing/scanner/mod.rs:471` |
| Argument type | halo2 `A'`, `S'`, `Z`: 3 commitments per argument, set inclusion (`MP:plonk/lookup.rs:40-66`) |
| Multiset or shuffle argument, offline memory checking | No (no shuffle in `MP`; permutation is copy constraints only) |
| User challenge phases | Prover yes (`MP:plonk/circuit.rs:2286`, `prover.rs:539-646`); in-circuit verifier no (`MC:verifier/verifier_gadget.rs:16-17`), so the node cannot verify a leaf that uses them |
| Node verifies a leaf with lookups | Yes, generically (`MC:verifier/verifier_gadget.rs:322,344,530-564`); one instance column, no phases |
| Lookups from ZKIR v3 | No: no table instruction (`ZKIR/ir.rs:396`) |
| Lookups from a stdlib `Relation` | No: `configure` is fixed by `ZkStdLibArch` (`ZS:lib.rs:486-548,1808`) |
| Upstream, unreleased | LogUp with batching (`UP/proofs/src/plonk/circuit.rs:1695-1741,2289`) and fflonk (changelog #555); both would cut the node cost of the bus |

Consequence: the chip design is buildable on the pinned proof system, but the leaf must be a
hand-written `Circuit` composed from midnight-circuits chips (native, pow2range, Poseidon) plus
the bus configuration. It cannot be emitted as ZKIR. Inside ZKIR only the register file with
Merkle openings is possible, at about 60,000 rows per step (ESTIMATE: 24 paths x 20 x 25 =
12,000; 99 lanes x 20 = 1,980 range rows, the 20 a guess; rule logic 21,492 instructions x 2.16
rows per instruction, from 2,747,489 / 1,270,000, = 46,423).

## 4. Sizing (all ESTIMATE)

### 4.1 Rows per step

| Chip | Low | High | Arithmetic |
|---|---|---|---|
| CORE | 8 | 8 | 2.1 |
| MEM | 6 | 12 | appended tuples per step |
| ALU | 16 | 32 | 2 to 4 operations x 8 |
| HASH for `mem_root` | 200 | 750 | 8 or 30 permutations x 25 |
| Total | 230 | 802 | 2^7.8 to 2^9.6 |

### 4.2 Leaf rows and k

`rows = F + S x step + P x page`, with F = RANGE 4,096 + DEC 512 + boundary digests 32
permutations x 25 = 800, total 5,408; P = distinct PROG rows and old cells opened in the leaf.

| S | Low: step 230, P = 2S at 200 | High: step 802, P = 3S at 500 | k |
|---|---|---|---|
| 16 | 5,408 + 3,680 + 6,400 = 15,488 | 5,408 + 12,832 + 24,000 = 42,240 | 14 to 16 |
| 64 | 5,408 + 14,720 + 25,600 = 45,728 | 5,408 + 51,328 + 96,000 = 152,736 | 16 to 18 |

Per step including page-ins and F at S = 16: 15,488 / 16 = 968 to 42,240 / 16 = 2,640 rows
(2^9.9 to 2^11.4). Page-ins dominate, and P is unmeasured: PROG rows repeat within a function
body, so 2 new rows per step is a ceiling, not an expectation (E0(b)). Against today's
5,670,286 rows per step the reduction is 5,670,286 / 2,640 = 2,148 (2^11.1) to 5,670,286 / 968
= 5,858 (2^12.5).

### 4.3 Node and decider at k = 20

Leaf proof commitments: 12 shared advice columns + 5 lookup arguments x 3 (bus x 2, RANGE x 2,
DEC) = 15 + 4 permutation products (12 columns in chunks of 3) + 6 quotient pieces = 37. Today's
stdlib leaf: 8 advice + 3 per range column + 3 + 5 = 19 with 1 range column (the stdlib default,
`ZS:lib.rs:227`) to 28 with 4 (the node's own setting, `crates/mori-recursion/src/parent.rs:566`);
the leaf's arch was not read. Difference 9 to 18 commitments at 3,000 to 4,000 node rows per
variable base (`research/sherlock-speed-replan-20261108/prover.md:155`): 27,000 to 72,000 rows.
Node: 741,708 + 27,000 = 768,708 to 741,708 + 72,000 = 813,708, under 1,048,576.

Decider (929,166 rows, headroom 119,410, MEASURED (brief)): the leaf's fixed and permutation
commitments are resolved there by name (`architecture.md` fact 4; `review.md` C1). Chip leaf:
about 6 selectors + 5 fixed tag, address and table columns + 4 constant columns + 12 permutation
columns = 27, against about 25 today (a guess). The cost per fixed base in the decider is not
recorded; this must be modelled (E3).

### 4.4 Gate G-C

`S_MAX = 16` is a node relation constant today (`design.md` 3.2(1)). fact(3): 293 / 16 = 19
leaves; list123: 303 / 16 = 19 leaves. Node time: 19 x 61 s = 19.3 min to 19 x 67 s = 21.2 min
(node step MEASURED (brief)), leaving about 98 minutes of the 2 h for 19 leaves at k <= 16 and
the decider. Leaf prove time at k = 16 for this circuit is unmeasured; leaves are independent
and can be proven in parallel within 48 GB.

## 5. Precedents and what was dropped

| System | What this design takes | Source |
|---|---|---|
| Miden VM | chiplets (hasher, bitwise, memory, ACE, kernel ROM) stacked in shared columns behind selectors, one chiplets bus | https://docs.rs/crate/miden-processor/0.28.0/source/src/trace/chiplets/mod.rs ; https://docs.rs/crate/miden-air/0.28.0/source/src/constraints/lookup/buses/mod.rs |
| Cairo | few registers; write-once nondeterministic memory; builtins | https://eprint.iacr.org/2021/1063 (abstract fetched; details from the paper body, not re-fetched) |
| RISC Zero | Merkle paging between segments: 1,094 to 5,130 cycles per page-in, about 1,130 on average | https://dev.risczero.com/api/zkvm/optimization ; https://dev.risczero.com/proof-system-in-detail.pdf |
| OpenVM | no central CPU; execution bus `(pc, t)`; "The memory bus is an adaptation of the offline memory checking argument" | https://openvm.dev/whitepaper.pdf (sections 4.5, 4.6) |
| SP1 | chips joined by LogUp interactions; shards of about 2^22 instructions; cross-shard memory consistency | https://docs.succinct.xyz/docs/sp1/hypercube/ ; https://hackmd.io/wztOd455QKWf-K8LXh_Fqw (community note) |
| PSE zkEVM | one halo2 circuit, tables joined by lookups (bytecode `(code_hash, index, opcode)`, rw table sorted by `rw_counter`) | https://ed255.github.io/zkevm-docs/print.html |
| Jolt | instruction as a lookup; Twist and Shout memory | https://eprint.iacr.org/2023/1217 ; https://eprint.iacr.org/2025/105 |
| halo2 lookup, LogUp | subset semantics of the pinned argument; the multiplicity argument upstream is moving to | https://zcash.github.io/halo2/design/proving-system/lookup.html ; https://eprint.iacr.org/2022/1530 |

The Miden design book pages could not be fetched (404 or empty); the Miden row rests on the
docs.rs source listings.

Dropped, with the reason:
- Balanced bus (LogUp or permutation) and offline memory checking with timestamps: need a
  challenge phase the node gadget lacks; not needed while every chip is a function of its key.
- Chiplets as separate proofs: a cross-proof bus needs a shared challenge, and the node has two
  slots and no room for a third (k = 21, F22).
- Lookup singularity (Jolt): needs sum-check and Lasso, not PLONK over KZG.
- Out-of-order execution: the prover has the full trace; nothing to reorder.
- Larger (CISC) steps: more cells per step, no saving per cell.

## 6. Experiments, cheapest first

| # | Experiment | Settles | Kill criterion |
|---|---|---|---|
| E0 | Census, native and emitter only. (a) Instructions of the dispatch block and of `KMachineReturnRight` and `BHelperMatchArm` by source (range checks, PROG walks, value reads, multiplexers, canonicality, transition). (b) On fact(3) and list123: per step, bus requests per chip, tuples appended, distinct PROG rows and old cells per window of 16 and 64 steps. (c) Per rule: registers written and the source of each value | chip heights, H_core, P, and whether the micro-op set of 1.3 covers the 108 rules | (b) more than 16 requests in a step at the 95th percentile, or P above 3S: redo section 4. (c) more than about 32 distinct micro-ops: DEC holds selection only and rule bodies stay gated arithmetic (2.2) |
| E1 | Inside ZKIR: one PROG walk shared by dispatch and the three value reads; each lane range-checked once. `cost_model` only | how much of the 2,747,489 selector rows is repetition (ESTIMATE 3x) | under 2x: the walk itself is the cost; nothing more to gain inside ZKIR |
| E2 | One chip alone: MEM with the linked stack, in the reference machine, differential against the owner engine; Quint model of 2.4 | write-once memory preserves semantics | any divergence that needs an overwritten cell |
| E3 | Bus spike: hand-written circuit with CORE, MEM, ALU and RANGE for `KMachinePushAccept`, `KMachineEvalLiteralValid`, `KMachineReturnRight`; model the leaf, then the node and decider over its constraint system | rows per step; that the gadget accepts the self-lookup bus; node and decider rows | node over 950,000 rows, decider over 1,000,000, the gadget refuses the constraint system, or over 2^11 rows per step before page-ins |
| E4 | Add PAGE and HASH to the spike; measure the Poseidon rows and one page-in | the two constants (25 and `depth x 25`) | page-in over 1,000 rows at depth 20 |
| E5 | DEC table generated from the fragments; Quint check against `owner_select` | decoder obligation of 2.2 | a class with two rows, or a rule body outside the micro-op set |

E0, E1 and E2 need no design decision and can start now. E3 is decisive.

## 7. Proposed design.md text (text only; no file was changed)

Written against `design.md` as amended 2026-11-10. It replaces the inputs of 2.2 item 1, the
fragment structure of 2.2 item 4 and the lane link of 2.3, and changes 2.4. It keeps 2.2 items
2, 3 and 5 to 9, the statement, and section 3.

> **2.7 Leaf as a chip (proposed 2026-11-10; not adopted; values wait for E0-E5 of
> `research/sherlock-processor-analogy-20261110/study.md`).**
>
> 2.7.1 The universal leaf is one circuit of a CPU core and the chips DEC, PAGE, MEM, ALU with
> RANGE, and HASH. The core carries the register file (control, outcome and counters,
> environment root, frame and task pointers, memory length, top frame, top task, helper and
> result records) across S step slots. In 2.3, `post_i == pre_{i+1}` is over the register file.
>
> 2.7.2 Every chip is reached only through bus tuples `(chip, key, a, b, c)`, tied by one lookup
> of the bus columns on themselves. Every chip is a function of its key. The leaf uses no
> challenge beyond those of the proof system and one instance column.
>
> 2.7.3 Memory is write-once. The data, data-field, environment and value-slot tables are
> append-only. Frames and helper tasks are entries linked to the entry below; push appends, pop
> follows the link, a rewrite of the top appends with the same link. This is a new layout
> version; it replaces "frames and helper slots zeroed on release" in `StepInv`.
>
> 2.7.4 PROG rows and cells created by earlier leaves are opened once per leaf against
> `prog_root` and the memory root in `pre_d`. PROG canonicality is a premise on `prog_root`,
> discharged by an admission claim [OPEN: its form].
>
> 2.7.5 Range checks apply to every value written and to the initial state, not to values
> carried from the previous `post_d`; SAS-010 states this as part of `StepInv` preservation.
>
> 2.7.6 Dispatch is the trap check (P0, P1, P2) and one DEC lookup. The rule's control word
> drives the core's micro-operations; registers it does not name are unchanged. [PENDING E0(c):
> otherwise 2.2 item 4 stands, applied to the register file.]
>
> 2.7.7 Chip heights are constants of the circuit. A leaf ends at S steps or when a chip is
> full; later slots are disabled. `prog_d` of 2.4 covers `prog_root`, the layout version and
> the profile, and no `Bounds` or host capacity.
>
> 2.7.8 Each chip has its own soundness obligation, Quint model and differential test against
> the per-rule fragments, which remain the reference. Gates [PENDING E3]: node at most 950,000
> rows and decider at most 1,000,000 over the leaf key; `k_leaf <= 16` at S = 16.

Open: the leaf leaves ZKIR, so the manifest of 2.4 names a circuit identity in place of ZKIR
and stdlib versions; the tree shape of the memory root; the admission claim; whether to wait
for upstream LogUp and fflonk.
