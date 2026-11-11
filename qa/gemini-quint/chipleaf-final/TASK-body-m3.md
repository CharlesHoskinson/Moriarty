## Lane m3: the M3 memory commitment as wired

IMPORTANT (second start of this lane): the first attempt spent its whole output budget on one response (65,536 output tokens, almost all of it thinking) and wrote no file. Work in small steps: write `m3.qnt` in several parts (types and pure hash/tree functions first, then the gates and acceptance, then the reference and refinement, then the state machine and the instances), appending with separate edits, and keep every single response short. Write a first small working model early and extend it.

### What is built (read the code; this is a summary only)

- MEM segment (`mem.rs` 96-242): 8 rows from `o`. Row columns: 0..5 the bus tuple `(chip, key, a, b, c)`, 5 `addr`, 6 `len_end`, 7 `act`; one code column (code 1 row 0, 2 rows 1..6, 3 row 7). Gates `mem row` (every row: `act` boolean, `chip = act * MEM`, `key = act * addr`, `(1 - act) * x = 0` for the data words), `mem chain` (rows 0..6: `addr' = addr + 1`, `len_end' = len_end`, `act' <= act`, and `act * (1 - act') * (addr + 1 - len_end) = 0`), `mem first` (row 0 padding: `addr = len_end`), `mem last` (row 7 active: `addr + 1 = len_end`). Every row is a bus provider row (`q_prov = 1`).
- Wiring (`rfcore.rs` 1199-1243): row 0's `addr` is a copy of slot 0's LEN cell (`base`); row 0's `len_end` is a copy of slot `s`'s LEN cell (the core's running `len` after the last slot). The core increments LEN by one per PUSH/TPUSH (`rfcore.rs` 509-515), and each PUSH/TPUSH requests `(MEM, len, ...)` on lane 1; a POP/TPOP requests `(MEM, fp or tp, ...)` on lane 1 and an ALU less-than `fp < len` (or `tp < len`) on lane 0 (`rfcore.rs` 352-431).
- Records (`m3.rs` 304-311): `H(act, a, b, c)`, a 4-input sponge whose inputs are copies of the row's `act` cell and its bus data cells. Segment root: 7 two-input compressions (312-323), inputs copied from the outputs below.
- Top tree, depth 17 (`m3.rs` 325-417): an empty segment slot is 0; level-`l` compression `H(in0, in1)`. The old walk starts at `x = 0`; the new walk at the segment root. The new walk's `s` (sibling), `b` (bit), `acc_in`, `acc_out` cells are COPIES of the old walk's cells at the same level (362-373). Level `l + 1`'s `x` is a copy of level `l`'s output, and the old walk's level `l + 1` `acc_in` is a copy of level `l`'s `acc_out` (345-353). The walker gates sit on the absorb row of each compression (`m3.rs` 103-145): `m3 walk` (every level, both walks): `b (1 - b) = 0`, `in0 = x + b (s - x)`, `in1 = s + b (x - s)`, `acc_in = 2 acc_out + b`; `m3 walk first` (old walk, level 0): `x = 0`, `8 acc_in = base` (column 10, a copy of slot 0's LEN), `len_post = base + 8` (column 11); `m3 walk last` (old walk, level 16): `acc_out = 0`. The compression inputs `in0`, `in1` are free advice (`HIn::Val`) fixed only by the walker gate.
- Digests (`m3.rs` 419-442): `pre_d = H(w_0 .. w_28, old_root)` over slot 0's register cells; `post_d = H(w_0 .. w_28, new_root)` over slot `s`'s register cells with word LEN (16) replaced by the close cell `len_post`. `pre_d` and `post_d` are the leaf's public statement. Genesis `len = 8` (`rf.rs` 92-110), the first segment (with `NIL = 0`) reserved.
- The sponge (`poseidon.rs` 268-416): `HIn::Cell` inputs are copy-constrained; `HIn::Val` are free advice. The previous lane (`chipleaf-hash`) found the sponge sound; take the hash as a collision-free function.

### Questions (Q3)

Compare the code with the constraints C1-C5 of `qa/gemini-quint/chipleaf-pairrow-m3/verdict.md` (and C6, which is open and out of scope here except to note it) and with the earlier model `m3seg.qnt`. In particular:
3a. The shared walk cells: is every cell that must be shared between the two walks actually copied (sibling, bit, accumulator chain), and is `x` of each level a copy of the level below? Can the new walk use a different path or different siblings from the old walk?
3b. The 17-bit index: does `8 acc_in = base` with the accumulator chain and `acc_out = 0` at level 16 force `base / 8 = sum b_l 2^l` with boolean `b_l`, so that the walked slot is exactly `base / 8` and `base` is a multiple of 8 below 2^20? Mind the field: `acc_in = 2 acc_out + b` is a field equation.
3c. The active flag at the boundary: is the active prefix exactly the addresses below `len_end`, for 0, partial and 8 active rows? Is `len_end` bounded by `base + 8`? Can an append be skipped, an unrequested row be active, or padding be non-zero or readable?
3d. The tag on records: `act` as the tag word. Can a written record collide with padding, can a padding row carry data into the root, and what does the tag NOT distinguish (frame, task and data records)?
3e. The close cell: `post_d.len = base + 8` while slot `s`'s `len` feeds `len_end`; `pre_d.len` is slot 0's `len = base`. Do two chained leaves line up (leaf 2's `pre_d` = leaf 1's `post_d`)?
3f. The state digest: are all 29 words and the root copies, and is the old root the one the old walk computes?

### Model

Scale down: segment of C = 4 rows (and one instance with C = 8 if it runs), top-tree depth D = 2 or 3 (so `base / C < 2^D`), a field modulo a small prime large enough for the honest values (say whether a small prime lets the accumulator wrap). Hash: an injective function (a tuple or an injective integer encoding). Model at least TWO chained leaves, with leaves of 0, partial and C active rows. The prover controls every advice cell named above (MEM row cells, `len_end`, every walker cell: `x`, `s`, `b`, `acc_in`, `acc_out`, `in0`, `in1`, `len_post`, the core's slot-0 and slot-`s` LEN, the requests); copy constraints and gates restrict it. Let the nondeterministic step choose one or two cell tampers from a small domain AND recompute the leaf's public `pre_d` / `post_d` from the forged witness (as the Rust negatives do), so only the circuit can refuse it. Reference: an abstract memory (map address -> (a, b, c), write-once, allocated in aligned segments of C), the core's appends and reads in order, and `mem_root` = the commitment of that memory with this leaf's segment written at slot `base / C`, all others as before. `invRefine`: accepted implies (i) `pre_d` and `post_d` are the digests of the reference states before and after the leaf, with `post_d.len = base + C`; (ii) every read is served the value written at that address earlier; (iii) the leaf's segment holds exactly the appended cells, padding zero.

Mutants (each caught, each with an honest-refines test):
- M1 `sib_not_shared`: the new walk's sibling is its own advice (no copy from the old walk).
- M2 `bit_not_shared`: the new walk's bit is its own advice.
- M3 `acc_not_chained`: the old walk's `acc_in` at level `l + 1` is not a copy of `acc_out` at level `l`.
- M4 `walk_last_dropped`: no `acc_out = 0` at the top level.
- M5 `bit_not_boolean`: no `b (1 - b) = 0`.
- M6 `x_zero_dropped`: no `x = 0` on the old walk's level 0.
- M7 `base_unaligned`: no `8 acc_in = base` (the walk index is free).
- M8 `close_running`: `post_d.len` is slot `s`'s `len` (the running `len_end`), not `len_post`.
- M9 `first_dropped`: no `mem first` (0 active rows).
- M10 `last_dropped`: no `mem last` (C active rows).
- M11 `prefix_end_dropped`: no `act (1 - act') (addr + 1 - len_end)` in `mem chain`.
- M12 `tag_dropped`: records are `H(a, b, c)` without the `act` word.
- M13 `chip_unscaled`: `chip = MEM` on every row (not `act * MEM`), or `key = addr` (not `act * addr`).
- M14 `len_end_free`: `len_end` is not a copy of slot `s`'s LEN.
- M15 `digest_skips_root`: a digest leaves out its root (or one word).
Also list every difference between this as-built model and the earlier `m3seg.qnt` (tag = `act`; no PAGE, so reads below `base` cannot be served; genesis `len = 8`; empty slot literal 0; the same 2-input hash for segment and top-tree nodes; `in0`/`in1` free advice fixed by the walker gate).
