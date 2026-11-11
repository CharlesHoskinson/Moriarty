---
title: "Dense tail-buffered memory (replacing D3)"
type: concept
tags: [memory, d3, run-length, chip-leaf, lead-decision]
sources: [runlength-consensus, runlength-consensus-audit, final-design-decisions, astra-area1-processor-draft, astra-area2-memory-draft, dissent-c6-section, design-2-7-chip-leaf, c1-dense-memory-experiment, lookup-cache-consensus, lookup-cache-audit]
last_updated: 2026-11-11
---

# Dense tail-buffered memory (replacing D3)

The memory design direction that replaces D3's padded aligned segments. Status: **lead decision R-3, pending owner review** ([[final-design-decisions]]); not an owner ruling. The backend is built and measured as a spike chip in experiment C1 ([[c1-dense-memory-experiment]]), not in the leaf. The chip leaf itself stays proposed, not adopted ([[ChipLeaf]]). The lookup-cache study keeps this design and adds a block cache of 8-record lines on top (lead L-1, [[BlockCache]], [[lookup-cache-audit]]).

## Why D3 is replaced

- D3 sets `len_post = base + 8` on every leaf close, uses a depth-17 top tree and reserves segment 0, so one run has fewer than 2^17 leaves and 2^20 cells; a leaf with no appends still burns a segment ([[runlength-consensus]], [[dissent-c6-section]]).
- Padding makes the physical state at height h depend on the leaf partition. The consensus calls this "falsifying" SRC-023 and the Lean theorem; the audit corrects that: SRC-023 constrains only `outcome_d` and `n_steps`, and padding makes the `det` premise impossible to discharge at digest level, so parallel provers must fix the partition first ([[runlength-consensus-audit]] finding 10). The rejection of D3 stands on the 2^17 limit and on SI-12.

## Mechanism (the direction all five lanes share)

From [[runlength-consensus]], with the audit's conditions ([[runlength-consensus-audit]] finding 8):

- Dense memory: `len_post = len_pre + used`, with no padding at leaf close; `len = 8q + r`.
- The `r` (0 to 7) unflushed tail cells are carried across leaves in `pre_d` and `post_d` and opened into the leaf MEM window with up to 8 new cells.
- A full 8-cell segment is flushed into a fixed-depth Poseidon Merkle tree iff `floor(len_post / 8) > floor(len_pre / 8)`, into a slot that holds `EMPTY_SEG`; otherwise the root is unchanged.
- Reads at or above the tail base come from the leaf window with no Merkle walk; older reads open against `root_pre` (PAGE), by the scalar `cold_read` (kept, lead L-2) or by an 8-record cache line ([[BlockCache]]). Every read checks `addr < len_cur` at its step ([[lookup-cache-audit]] item 11).
- The MEM interface is the landed C1 interface: `membackend.rs`, `dense.rs`, bus key `8 addr + kind`, trait methods `persist` and `cold_read`; the lookup consensus's section 3 interface (key `(arena << 64) | addr`, not injective at TOP = 63) is not adopted (lead L-4, [[final-design-decisions]]).
- FRAMES and TASKS leave random-access memory: register-cached top frame and task, deeper entries in Poseidon hash-chain stacks (`fp_hash`, `tp_hash`). Stack traffic uses no heap addresses.
- Audit conditions taken into R-3 as MEM contract items: canonical `tail_d` (positions at or above `r` are a fixed pad); a leaf that touches the tail opens it against `pre_d.tail_d`, otherwise passes it through; both lengths decomposed with range checks; kind-tag checks on reads (one-arena variant); a domain tag on every hash. The audit also says the hash-chain stacks are not D7 (D7 links frames by an address in memory, and only that form is modelled) and that new chip models (tail, flush, hash stack) are a condition of adoption.

## Open parts

- **Tree depth (R-4, OWNER).** The consensus picks `TOP = 61` (at most `2^64 - 1` cells). The audit (blocker 3) shows the address space can then bind before the step counter, since a step may append up to 3 cells. Lead recommendation: TOP >= 63; experiment C1 measures the cost. The owner confirms TOP >= 63 or accepts a memory bound below the step bound, which would narrow GP-1 ([[final-design-decisions]], [[OpenOwnerQuestions]]). Astra's area 1 and 2 drafts raise the same gap ([[astra-area1-processor-draft]], [[astra-area2-memory-draft]]).
- **Arena count (R-6; lead L-9: one arena, pending owner review).** One domain-tagged arena with one flush unit (run-length lanes C, D, E) against four arenas DATA, FIELDS, ENV, SLOTS with two flush units (run-length lanes A, B). C1 measured one arena at 12,281 backend rows against 49,172 for four (E-3). Lead L-9 picks one arena: four arenas with 4 + 2 cache lines do not fit `k_leaf <= 16` (about 67,500 to 85,100 rows, ESTIMATE), one arena is about 50,700 to 51,800 (ESTIMATE) ([[final-design-decisions]], [[lookup-cache-audit]]). The non-interleaving check over the rule set is still required before the spec freezes this. Against it: the lookup audit keeps the count open until C4, the run-length audit would make four arenas the default meanwhile, the lookup consensus defaults to four arenas, and the area 2 draft recommends four initially; area 1 keeps four logical namespaces at the processor interface.
- **Per-step maximum.** No table yet of the maximum cold reads, appends and arenas one step touches; the leaf must fit the largest single step at S = 1 ([[runlength-consensus-audit]] finding 6; [[astra-area1-processor-draft]]). The lookup audit (blocker 1) extends it to distinct old segments, ROM blocks per table and stack spills; cache-line counts are not frozen until it exists (lead L-2, [[BlockCache]]).
- **Scratch registers.** Slot-0 zeroing does not replace a write-before-read obligation (R-8, audit finding 9).
- **D-1.** The consensus rejects the linked-slice form; D-1 stays open until measured (R-9, task 4.18). The dense design is a third candidate form ([[runlength-consensus-audit]] finding 13, [[dissent-c6-section]]).
- **Backend.** Run-wide write-once LogUp (Profile B) does not replace dense Merkle; it stays behind the `MemBackend` seam as benchmark-only, not built before M4, because it cannot stop and resume without reloading live memory and changes the leaf statement, node, seal and continuation (lead L-5, [[lookup-cache-audit]] finding 9). The run-length consensus rejected offline LogUp on the pinned stack; the area 2 draft keeps it as a measured alternative ([[runlength-consensus]], [[astra-area2-memory-draft]]).

## Figures

Row and throughput figures in the study are ESTIMATE and not of this design (R-10) ([[final-design-decisions]]). They are not quoted here as results.

Experiment C1 built the dense backend as a spike chip behind a `MemBackend` interface and measured it ([[c1-dense-memory-experiment]]):

- measured: flush unit 8,541 rows at TOP = 61, 8,821 at TOP = 63; cold read 4,621 / 4,761; window slot 203; `persist` 12,137 rows at one flush unit (W = 1), +10,161 per extra unit;
- measured: TOP = 63 adds 280 rows per flush and 140 per cold read and no column, so the node does not grow with TOP;
- measured: four arenas with one flush unit each, 49,172 harness rows against 12,281 for one arena;
- estimated: leaf rows 14,942 / 36,543 / 58,565 at S = 16 / 64 / 128 (W = 1 / 3 / 5) without cold reads, +4,751 per cold read; node about 847,500 rows (+4 fixed columns);
- estimated: four arenas at S = 16 cost 52,205 rows with a flush unit per arena (2 cold reads fit `k_leaf <= 16`) or 35,123 with two shared units (6 fit), against 14,942 for one arena (10 fit);
- not done: the LogUp backend, hash-chain stacks, wiring into the real leaf, the non-interleaving check.

The lookup consensus labels the C1 cold-read rows (4,621 to 4,761) ESTIMATE; C1 and the lookup audit label them measured. The audit composes a one-arena leaf with 4 heap and 2 ROM lines at about 50,700 to 51,800 rows (ESTIMATE) from the measured parts ([[lookup-cache-consensus]], [[lookup-cache-audit]]).

## Contradictions

- Consensus against audit on "5/5" adoption of one design, on slot-0 zeroing and on D-1; the lead adopted the audit's corrections (R-3, R-6, R-8, R-9).
- Consensus against audit on D3 "falsifying" SRC-023 (finding 10); no separate lead decision is recorded.
- Memory key encodings differ between [[astra-area1-processor-draft]] and [[astra-area2-memory-draft]]; the lookup consensus proposes a third, `(arena << 64) | addr`, which the audit finds not injective at TOP = 63; lead L-4 keeps the landed `8 addr + kind`.
- Arena count: lead L-9 (one arena) against the lookup audit and the run-length audit (keep open or four by default) and the lookup consensus (four by default).

See also [[RunLengthCeiling]], [[Continuations]], [[ChipLeaf]], [[MoriChip]].
