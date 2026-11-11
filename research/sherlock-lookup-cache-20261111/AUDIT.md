# Audit of CONSENSUS.md (run `sherlock-lookup-cache-20261111`)

Date: 2026-11-11. Target: `research/sherlock-lookup-cache-20261111/CONSENSUS.md`. Checkout: `/home/hoskinson/Projects/Moriarty` at `9973d633b`. Read-only audit: no build, test, proof or model run. Paths are relative to the repository root. `C:n` is line n of CONSENSUS.md. `O/` is `openspec/changes/sherlock-vm-consensus-20261107/`. `MC/` and `MP/` are `src/` of `midnight-circuits-7.3.0` and `midnight-proofs-0.8.3` in the cargo registry. `FD/` is `research/sherlock-final-design-20261111/`. `RL/` is `research/sherlock-runlength-consensus-20261111/`.

The target was still being written when the audit started (24,591 bytes at 02:14, sections up to 1.4). This audit is of the file of 84,457 bytes and 524 lines, last written 2026-11-11 02:27:52 local time. It did not change again during the audit.

## 1. Verdict

**Adopt with changes.**

Adopt the direction. Memory, PROG and the type tables stay on the dense tail-buffered Merkle design of the run-length study (R-3), with one new part: a small number of 8-record cache lines per leaf, opened against the pre-state root and served on the existing bus. The design stays one-pass with Poseidon boundary digests. Run-wide write-once LogUp does not replace it. I found no stale-line, aliasing, partial-segment, forgery or overlap attack on the block cache, on the conditions listed in section 5.1. The reasons against LogUp as the default are correct and are the strongest part of the report: it cannot stop and resume without reloading live memory, it cannot be proved while the run executes, and its segment proofs depend on a run-wide challenge.

Do not adopt sections 2.2 and 3 as written. Three points block that:
1. The cache sizes `K_page`, `K_rom` and "4 bus requests per step" are fixed with no per-rule maximum, and the scalar read path is removed. A step that needs more lines than the leaf has is unprovable, which narrows GP-1 (finding 1).
2. The value-digest module as specified does not compute `sherlock-value/1` (finding 2).
3. Section 3 presents a module interface that is not the landed C1 interface, cites lines that do not hold it, and its memory bus key is not injective at TOP = 63 (finding 3).

Profile B (ZmLogUp with Horner boundary) is sound in outline and needs no fork of the two dependencies. It is not a two-trait swap, and it can stop and resume only by reloading all live memory. Keep the interface seam (it exists, `crates/mori-chip/src/membackend.rs:95-128`). Do not schedule the build of B before M4, and record B as benchmark-only (finding 9).

Counts: 3 blocker, 11 major, 9 minor.

## 2. Findings

### Blockers

**1. Cache capacity per step has no bound, and the fallback is removed (GP-1).**
Evidence: `C:103-104` freezes `K_page = 4`, `K_rom = 2` for M4; `C:181` replaces the scalar `cold_read` (`crates/mori-chip/src/dense.rs:716-753`) with block lines; `C:26` and `C:220` fix 4 bus requests per step. Chip heights are circuit constants, and a step whose requests do not fit cannot be enabled (`O/design.md:90`). So a rule that reads records from more than `K_page` distinct old segments, or ROM rows from more than `K_rom` distinct ROM blocks, in one step has no proof even at one step per leaf. Program row, constructor row, type row and constant row are in different tables and so in different blocks; `K_rom = 2` is below that count if one rule reads three of them. Lane D lists the per-rule request count as not checked (`lanes/D/round2.md:497`) and uses `K_page = 2` (`lanes/D/round2.md:181`); lane B uses 3 to 4 (`lanes/B/round2.md:162`). The run-length audit asked for this table before adoption (`RL/AUDIT.md:44-46`); `FD/EXPERIMENTS.md` (section "Not covered and open") says it is still not measured. `C:517-524` does not list it.
Required change: before any of `K_page`, `K_rom` or the lane count is frozen, add a per-rule table over the full rule set: bus requests, distinct old-memory segments, distinct ROM blocks (per table), appends and stack spills in one step. Require each maximum to fit the chip heights at S = 1. Keep `LeafShape` parameters open until then.

**2. The value-digest module does not compute `sherlock-value/1`.**
Evidence: (a) `C:28`, `C:122` and `C:354` hardwire one constant state `[C_0, C_1, C_2] = PoseidonPerm(0, SHK_VAL, 1)`. The hash of `sherlock-value/1` has the input length in its initial state (`O/design.md:175`; `MC/hash/poseidon/poseidon_cpu.rs:131-133`), and the chip sponge starts its capacity at the input length (`crates/mori-chip/src/poseidon.rs:8,109,315`). Value preimages have 4, 5, 6 or 7 words (`O/design.md:177-186`, cost table `:198-203`). The state after the first block is therefore one constant per length, not one universal constant. A digest built from the formula in the report differs from `sherlock-value/1` and so changes `outcome_d` and `input_d`. (b) `C:204` and `C:351` route the digest through bus tuples `(CHIP_HASH, key, in0, in1, out)`. A sponge state has three words; a chain that passes one output word is a different hash. (c) `C:350` names `crates/mori-core/src/sherlock/value_digest.rs` as the reference; the file does not exist. (d) `C:354` and `C:151-153` add tuples, records and arrays; `sherlock-value/1` has seven forms and none of these (`O/design.md:177-186`).
The saving itself is real and larger than stated: the header `[V, 1, tag, arity]` is constant for each builtin form, so two permutations can be folded into constants for `UInt`, `Bool`, `Leaf`, `Cons` and `Branch`.
Sharing and layout independence hold on these conditions: the stored `vd` is computed from the tag and the `vd` lanes of the children; each child `vd` is read from a record whose kind the rule constrains; `b` is boolean before the `VD_TRUE`/`VD_FALSE` selection; `UInt n` is below 2^128; `ty_d` and constant digests in ROM are part of the PROG canonicality premise, whose form is open (`O/design.md:84`).
Required change: state that `sherlock-value/1` is unchanged bit for bit. Replace the single prefix by a table of constants indexed by preimage length (or by form). Give VALUE DIGEST whole-sponge slots in HASH with copy-constrained inputs, not 2-input bus chains. Remove the reference file name and the three new value forms. Add a differential test of every form against the specification text.

**3. Section 3 is not the landed interface, and its memory key is not injective.**
Evidence:
- `C:201-202` and `C:315` define the memory bus key as `(arena << 64) | addr`. With TOP = 63 addresses run to 2^66 - 1 (`C:38`), so `(1, 0)` and `(0, 2^64)` give the same key. An address of 2^64 or more exists only after 2^64 real appends, so this is not a practical attack; it is the loss of an invariant that the landed design checks. The landed key `8 addr + kind` with `kind` in `0..=4` is injective at any TOP (`crates/mori-chip/src/membackend.rs:55-58,265`) and is a checked property (`FD/EXPERIMENTS.md`, Quint property 4, mutant `kind_out_of_range`).
- `C:201-202` splits MEM into two chip ids (append and read). The landed window row holds one tuple that serves both (`membackend.rs:258-259`).
- `C:212-213`, `C:272` and `C:290-308` describe `WindowBoundary`, `FlushPort`, `StateTok` and a one-method trait `assign_backend`, and cite `membackend.rs:159-163`, `:172-178`, `:32`, `:33`, `:396-415` and `:95-148`. None of these names is in `crates/mori-chip/src`. Those lines are selector constants, `low_bit`, `use` lines and row assignment. The landed trait has `persist` and `cold_read` (`membackend.rs:95-128`). In the landed code the backend computes the record hashes, the tail opening and `Tree8` and owns `tail_d` in its token (`membackend.rs:12-16`, `dense.rs:644-671`); section 3.4 moves them into MEM (`C:275,277`). `FlushPort` holds one flush, but `C:106` needs W = 2 to 3 at S = 64.
- `C:23`, `C:35` and `C:272` say that the design does not alter `WindowCfg` and that a backend swap changes zero lines of CORE and MEM. Section 3 changes the key, the chip ids, the trait and the module boundary.
- Chip ids `0, 2..6` (`C:198-204`, `C:457`) differ from the code (`crates/mori-chip/src/bus.rs:30-31`: ALU 1, MEM 2; `poseidon.rs:53`: HASH 3).
Required change: the architecture merge takes the landed C1 interface as the base (`membackend.rs`, `dense.rs`, `FD/EXPERIMENTS.md`) and adds to it. Keep the key `8 addr + kind` and one MEM chip id. Add one method to `MemBackend` for block lines; keep `persist`. Delete the citations that do not hold. If a boundary is to move (for example `tail_d` into MEM), state it as a change with its reason.

### Major

**4. The block-cache contract in the report is incomplete.**
Evidence: `C:19` and `C:315` state three rules: open against `pre.root`, `blk_idx < q_pre`, disabled lines publish zero. The soundness argument needs the rules of section 5.1, which the lanes partly state (`lanes/B/round2.md:62,483-489`, `lanes/A/round2.md:466-471`) and the report drops. Two matter most. (a) `C:18`, `C:118` and `C:272` give the hot range as `base <= addr < len_post`. The rule is `addr < len_cur` at the step (`O/design.md:103`; mutant `future_record_read` in `FD/EXPERIMENTS.md`); `C:202` and `C:276` say it correctly. With the cache the request no longer names its provider, so the check is needed on every read. (b) `C:315` gives 86 permutations per line, which includes the 8 record hashes, but its text names only `Tree8` and the walk, and `C:317` budgets 70. If the leaves of `Tree8` are not `H_REC(kind, a, b, c)` as in `persist` (`dense.rs:67-69,650-651`), the line cannot match the stored segment and the kind is not bound.
Required change: put the rules of section 5.1 in the module text and give each a named mutant.

**5. The ALU contract loses the overflow flag, and the multiply rule makes overflow unprovable.**
Evidence: `C:200` and `C:255` give the tuple `(ALU, op, x, y, w)`. The landed key is `op + 8 flag` (`crates/mori-chip/src/alu.rs:3-6,118`). `C:258` "enforces `x_hi * y_hi == 0`" and a carry below 2^64. Overflow of a checked `u128` operation is a program outcome (`Rejected(10)`, `lanes/C/round1.md:63`; `crates/mori-core/src/sherlock/image_reference.rs:658`), so it must be provable with flag 1. As written an overflowing multiply has no proof, which narrows "every outcome" of GP-1. Lane C's form is right: 64-bit limbs, `x0 y0 = c0 2^64 + r0`, `x0 y1 + x1 y0 + c0 = c1 2^64 + r1`, overflow iff `x1 y1 + c1 != 0` (`lanes/C/round1.md:163-166`), with every limb range-checked and `c1 < 2^65`. `RangeConfig` has only the 128-bit shape (`crates/mori-chip/src/range.rs:1-10`); a 64-bit shape is new. The op list `1..=9` (`C:255`) adds `DIV_MOD`, signed compare and `BITOPS`; lane C read the machine as having no division and no bitwise operation (`lanes/C/round1.md:23`). "+0 `k_leaf` growth" (`C:27`) is an estimate: two more 128-bit decompositions per operation are 24 more rows in running-sum mode (`range.rs:12-17`), and `C:259` (4 rows per operation) cannot hold with one limb column.
Self-ranging itself is correct and worth doing: `alu.rs:8-9` leaves the operand range to the requester.
Required change: keep `key = op + 8 flag`; specify result and flag for the overflow case of each operation (unique, so the chip is a function of its key); list the limb range checks; restrict the op list to operations of the reference machine.

**6. The tagged DEC + RANGE lookup is described in two forms with different soundness and savings.**
Evidence: `C:25` and `C:120` say two columns and "-1 fixed column". `C:206-209` and `C:247` define three columns `(tag, key, val)` with the tag taken from the fixed selector `q_dec`. The three-column form is sound on both sides without any condition, because the prover does not control the tag. It keeps three table columns, so it saves one lookup only: about -22,600 node rows (ESTIMATE), not -29,062. The two-column form of lane C (`lanes/C/round1.md:177-186`) adds the tag to advice values. Its RANGE side is sound. Its DEC side is sound only if `ctrl` is below 2^54 or `syn` is below 2^64 by gates that do not depend on the lookup; otherwise `ctrl = 1 - 2^64`, `syn = v - 2^64` matches a RANGE row. `q_rng * q_dec == 0` is a property of fixed columns and is not that condition. In both forms the packing of `packed_key` from `phase`, `op` and flags (`C:238`) needs range checks on each part, as `C:241` requires for `ctrl_word`.
Required change: choose one form. Recommended: the three-column form, with the saving restated. If two columns: state the bound on `ctrl` or `syn` as a DEC contract item with a mutant.

**7. The row figures are not of this design and ignore the C1 measurements.**
Evidence: the report does not cite `FD/EXPERIMENTS.md` or `FD/DECISIONS.md`. R-10 says study row figures are not results.
- Leaf, one arena (`C:135`): 42,100 to 45,600 rows. From measured parts: the composed leaf at S = 16, W = 1, TOP = 63 with no old read is 15,226 rows; a line is 86 permutations (6,020 rows); a ROM line with its row hashes is about 55 permutations at depth 32, not 39 (`C:316`). Sum (ESTIMATE): 15,226 + 4 x 6,020 + 2 x 3,850 + 1,120 to 2,240 (value digest, `C:355`) + 1,400 (stack spills, `lanes/B/round2.md:211`) + 384 (two more decompositions for 16 ALU operations) + 768 (a 12-row bound check for up to 64 reads) = about 50,700 to 51,800 rows. It fits `k_leaf = 16`; the margin is about 14,000 rows, not 19,900.
- `C:317` gives 491 permutations for the backend; it omits 30 window record hashes, 14 tail compressions and 48 line record hashes. Measured `persist` at W = 1 is 12,417 rows (177 permutations), not 133.
- Node (`C:25,135`): 823,466 is the node over the M3 spike leaf. The dense backend adds about 4 fixed columns, about +24,016 rows, to about 847,494 (ESTIMATE, `FD/EXPERIMENTS.md` "Node rows"). The cache, ROM and hash-stack providers add more code columns. The lookup saving must be applied to that base.
- An isolated old read costs more with a line (6,020 rows) than with the scalar walk (4,761 rows, measured). "735 to 752.5 rows per record" (`C:19`) is the case where all 8 records are used. No locality census exists (`C:521`).
- `C:19` and `C:35` label the measured 4,621 and 4,761 rows as ESTIMATE; "4,480" omits the record hash.
Required change: recompute the tables from `FD/EXPERIMENTS.md`; mark every composed figure ESTIMATE with its inputs; state the line cost as a fixed cost per leaf and the per-record figure as a best case.

**8. The four-arena default does not fit the leaf gate with the proposed cache sizes.**
Evidence: `C:501` defaults to four arenas with two shared flush units until the non-interleaving check passes. That default follows the run-length audit (`RL/AUDIT.md:29-31`: until the invariant is mechanised, four arenas are the default) and R-6 keeps the choice open; E-3 leans to one arena after C1 (`FD/DECISIONS.md`). The fit claim is the problem. Shared flush units are not built. From measured parts the four-arena leaf with no old read is 53,341 rows (one flush unit each, TOP = 63) or 35,699 (two shared units, ESTIMATE). Adding 4 heap lines and 2 ROM lines (about 31,800 rows, finding 7) gives about 85,100 or 67,500 rows, both above 65,530. `C:136` and `C:499` (51,800 to 56,400 rows, "fits") are not supported. The executive summary (`C:39`) and table row `C:135` assume one arena.
Required change: state the arena count as open (R-6, decision in C4), give both leaf sizes from measured parts, and say that four arenas with `K_page = 4` and `K_rom = 2` need `k_leaf = 17` or fewer lines. Stage 1 of `C:103-104` (`k_leaf = 16`) holds only for one arena.

**9. Profile B: sound in outline, not a two-trait swap, and not GP-1 complete.**
Evidence and assessment:
- No fork: correct for the dependencies. `committed-instances` is enabled (`crates/mori-chip/Cargo.toml:14`, `crates/mori-recursion/Cargo.toml:45`); the gadget absorbs committed instances before the plain instances and the advice (`MC/verifier/verifier_gadget.rs:298-316`), evaluates instance queries (`MC/verifier/expressions/mod.rs:48`, `verifier_gadget.rs:435-436,627-634`), and upstream tests a variable committed instance (`verifier_gadget.rs:930-939`). The challenges are public inputs of the leaf, so the single-phase assertion (`:310`) and the `Challenge` panic (`expressions/mod.rs:49`) are not touched. Nothing is built (`FD/EXPERIMENTS.md`: "The LogUp backend ... is not built").
- Soundness: the two-pass order is right. It holds only if every term of the sums and of the Horner fingerprint is a function of cells bound to the committed instance column, all leaf commitments enter `d_root`, the seal checks that the challenges equal the hash of `d_root` and that the sum is zero, each address is written once (dense counter), and every read keeps `addr < len_cur`. The report states the first four.
- Not a swap of two traits: B adds an instance column, changes the leaf statement, the node (variable point, subgroup check, `d_root`), the seal and the continuation module, and replaces the `digest_inj` premise (`clean/fork/Clean/Sherlock/Aggregation.lean:201`) by a probabilistic one. `C:23` lists only the modules that do not change.
- GP-1: B has no memory commitment across a stop (`C:54,75`). GP-001 allows the proving of a run to stop and resume (`O/requirements.md:56`). B can resume only by reloading all live memory in loader leaves, a cost that grows with the live state, or by the address-free checkpoint mode that R-7 does not adopt. B is therefore not suited as the production backend.
- Hardening 4 (`C:61,325`): the 2^-125.86 figure is a union bound over pairs of tuples. The Schwartz-Zippel bound on the joint identity in the two challenges is about (W + R) x arity / r, near 2^-187 at 2^64 accesses. One challenge pair in the base field meets 128 bits; the second pair is optional and costs rows. "< 2^-250" is not derived.
- Hardening 3 (`C:324`): a negative `m_w` does not forge a read, because the characteristic is far above the read count. The range check is harmless and not needed.
- Hardening 6 (`C:56`): 11 divides the G1 cofactor (checked, `MC/ecc/curves.rs:287`), and `assign` does the subgroup check (`MC/ecc/foreign/weierstrass_chip.rs:343-360`). A torsion offset does not change the committed polynomial; it gives several encodings of one commitment. Keep the check; the described loss of binding strength (`lanes/C/round2.md:15`) is overstated.
Required change: describe B as benchmark-only for single-session runs; list the modules it changes; give the bound model; keep the interface seam; do not put `P5` (`C:184`) before M4.

**10. The 2-leaf `RunLift` figures are of an unbuilt circuit and show no margin.**
Evidence: `C:135`, `C:413` and `C:416` give 994,820 to 1,022,000 rows and call it "safe" against 1,048,570. The only adopted node gate is 950,000 rows (`O/design.md:105`); the GP architecture asks that a binary node fit k = 20 "with margin" and lists that as an experiment (`research/sherlock-gp-architecture-20261110/architecture.md:632`). The circuit does not exist. Its baseline "about 1,046,000" (`C:133`) has no source in the report, and the dense backend adds about 24,000 node rows per leaf key (finding 7), which the figures omit. With that term the upper end is at or above 2^20.
Required change: mark the 2-leaf node as unmeasured; state which gate applies to it; keep it out of the M4 plan and out of the wall-clock figures until `cost_model` rows exist.

**11. Row 6 is not a passive replica.**
Evidence: `crates/mori-chip/src/rfcore.rs:198-200` places copies of `L0`/`L1` on rows 6 and 10 (correct). The gates on rows 5 and 7 can reach lanes only at rotation +-1, and the registers on those rows read them: `R0`, `V0`, `V1`, `V2` (`rfcore.rs:184-185,533-537`). The `Cell` type has no `L2`/`L3` (`rfcore.rs:66-77`). Promotion needs a new layout plan (registers moved or operands routed). "+0 node rows" (`C:26`) holds only if that plan needs no new column. The spike has 6 rule flags, not the full rule set. `C:194` and `C:220` name rows 5 and 6; `C:26` names row 6 with the canonical row 2 (`rfcore.rs:217`); `C:225` gives 7 rows per step against a 13-row slot (`rfcore.rs:215`).
Required change: state the promotion as a layout experiment with its acceptance (no new column, all rules placed), fix the row numbers, and include lane E's rule that unused lanes are zero in all five cells (`lanes/E/round2.md:121`).

**12. ROM lines are under-specified, and the tree depth bounds the program size.**
Evidence: `C:203` and `C:316` give depth 32 and `row_idx < 2^32`; lanes use 16 and 29 (`lanes/D/round2.md:373`, `lanes/B/round2.md:211`). A fixed depth is a circuit constant that limits the program, and GP-001 says every program (`O/requirements.md:56`). `C:155` says "`prog_root` handles arbitrary program size". The report gives no leaf hash domain for ROM rows, no padding rule for a last partial line, and no statement of how `(tbl_id, row)` maps to the path. The rule and table counts (120 + 2 rules, 14 tables, `C:120,508`) cite `crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs:70-108`, which is the disabled-rule and extension-rule check, not a rule or table list.
Required change: state the ROM depth and show that it is at least the index width of the language's program tables, or ask the owner. Add the ROM rules of section 5.1. Give a correct source for the rule and table counts.

**13. Requirements, value forms and files that do not exist.**
Evidence: `SRC-025` (`C:151,153`) is not in `O/requirements.md` (the last is SRC-023). `Value::Tuple`, `Value::Record`, `Value::Array`, `ArrayGet` and `ArrayLen` (`C:151-153`) are not in the reference machine; SI-12 to SI-14 concern field slices (`O/dissent.md:198-202`; `C:152` cites `:27`). `specs/quint/` (`C:226` and each module) does not exist; models are in `models/sherlock/quint/`. `crates/mori-chip/src/rfcore.rs:283` is `fn flags`, not a zero tuple (`C:195,199`). `crates/mori-chip/src/hashchip.rs:88-100` is `configure`, not a rule that every permutation runs (`C:123,336`). The "consensus proposed resolution" of D-1 (`C:153`) is a proposal on an item that R-9 keeps open.
Required change: remove or correct each. D-1 of `O/dissent.md` stays open; the report may propose, the lead or the owner changes the status.

**14. "5/5" in the executive summary overstates the lane files.**
Evidence: `C:15` and `C:32` say all five lanes converge on the recommended design "incorporating all soundness corrections" of the run-length audit. Only lane A cites that audit or uses TOP = 63. Lanes B, C, D and E use TOP = 61 (`lanes/B/round2.md:15`, `lanes/C/round2.md:13`, `lanes/D/round2.md:373`, `lanes/E/round2.md:15`). Lanes C and E keep the slot-0 scratch zeroing that R-8 rejects. Section 4 records these splits correctly (`C:487-488,491,502-505,510`). The core claims are true for all five: Merkle with block cache as default (`lanes/A/round2.md:12-18`, `B:15-21`, `C:12-15`, `D:112-115`, `E:15-18`), compatible with the run-length consensus (`A:266-268`, `B:174`, `C:182`, `D:193`, `E:185`), LogUp kept behind the interface, b2 rejected.
Required change: in the summary, limit "5/5" to those four claims and mark TOP = 63, write-before-read, Mode 1 only and the claim-counter rule as the orchestrator's alignment with the audit (which is the right choice and matches R-4, R-5, R-7, R-8).

### Minor

**15.** `C:5` gives the baseline as `9ffc2fe76`. `membackend.rs` and `dense.rs` are not in that commit; they landed in `6a92ba3d7` and `b6f49f99a`. HEAD is `9973d633b`.

**16.** `C:21` and `C:89` cite `crates/mori-recursion/src/ivc.rs:530-535` for the fixed identity commitment. Those lines verify the previous node proof. The leaf's is `crates/mori-recursion/src/inner.rs:181-182`. The claim holds.

**17.** `C:20` cites `owner_runtime.rs:3566-3591` for `EMPTY_STACK != 0`. The lines show push, pop and rewrite-top on vectors. The empty-stack rule is a design rule, and one direction of it is a collision assumption (`RL/AUDIT.md`, finding 8). Hash-chain stacks are not built.

**18.** `C:386` says the Poseidon boundary "directly discharges" `digest_inj`. It is a premise (collision resistance); the digest instantiates it.

**19.** The G-C times (`C:76,104,135`) assume 16 steps per leaf and are marked so. With `K_rom = 2` and poor program-row locality a leaf closes after a few steps: at 2 steps per leaf, 303 steps need about 152 leaves, about 160 minutes of serial node time at 63 s (ESTIMATE), above the 2 h gate. Say that the gate margin depends on the unmeasured miss rate.

**20.** `C:106` uses W = 2 to 3 at S = 64. The Lean theorem covers the one-flush rule with at most 8 appends per leaf (`FD/EXPERIMENTS-C3.md`, `partition_independent_flush`, `FitsLeaf`). Multi-flush has a MockProver test (`c1_two_flush_units`) but no theorem.

**21.** The report's dissents D-1 to D-6 (`C:496-514`) reuse the names of `O/dissent.md` D-1 and D-2. Rename them (for example LC-1 to LC-6).

**22.** `C:411` gives the leaf public lanes as `[prog_root, pre_tok, post_tok, steps, halt_flag, outcome_d]`. The landed lanes are `[binding, prog_d, pre_d, post_d, rule, h_pre, steps]` (`crates/mori-recursion/src/ivc.rs:70-74`), and the genesis gate is `h_pre == 0` (`O/design.md:172`). The merge must reconcile them.

**23.** Prior-art table (`C:164-173`) and the external advisories were not re-checked here. `C:168` dates CVE-2026-62333 to April 2026; the run-length audit found 2026-10-30 at OSV (`RL/AUDIT.md:104`).

## 3. Claims verified

- `crates/mori-chip/src/membackend.rs`: `WindowCfg` at `:152-460`; window and chain gates `:248-305`; `kfix` `:28-30,156`; trait `:95-128`; `top` parameter `:381`. `dense.rs`: `iv`, `pad_d`, `empty_seg`, height-tagged nodes `:59-98`; `tail_d` `:112-119`; `impl MemBackend` `:620-753`; `cold_read` `:716-753`. `bus.rs:40-68`: two `lookup_any` lanes.
- Measured rows in `FD/EXPERIMENTS.md`: flush unit 8,541 / 8,821, old read 4,621 / 4,761 at TOP 61 / 63.
- Line arithmetic: 16 + 7 + 61 = 84 permutations (5,880 rows), 86 at TOP = 63 (6,020 rows).
- `rfcore.rs:198-200`: lane copies on rows 6 and 10. `alu.rs:8-9,111-125`: operands are not range-checked by the chip.
- `MC/verifier/verifier_gadget.rs`: committed instances absorbed `:298-300`, instances `:302-306`, `assert_eq!(cs.phases().count(), 1)` `:310`, advice read `:314-316`, challenges `:319,327,328,341`, rotations -1, 0, +1 only `:604-611`. `MC/verifier/expressions/mod.rs:48-49`. `MP/plonk/prover.rs:32-49`: `commit_to_instances` behind the feature.
- `MC/ecc/foreign/weierstrass_chip.rs:343-360` (`assign`, cofactor check) and `:936-952` (`assign_without_subgroup_check`); `MC/ecc/curves.rs:287`: cofactor, 125.84 bits, equal to (x - 1)^2 / 3, divisible by 121.
- `1,032,467 = 2^0 + 2^1 + 2^4 + 2^8 + 2^14 + ... + 2^19`; headroom 16,103; unqueried tags sum to 16,108; table loader `MC/field/decomposition/pow2range.rs:215-236`; measured figure at `research/sherlock-c6-outcome-ledger9-review-20261110/review.md:213`.
- `O/design.md:105` (gates 950,000 / 1,000,000, `k_leaf <= 16`); `O/requirements.md:36,39,56-59`; `S_MAX = 16` at `ivc.rs:77`; `digest_inj` at `Aggregation.lean:201`; E2 (+4 node rows per leaf k) and E4 at `research/sherlock-recursion-design-review-20261110/experiments/e2-e4.md:7-35`.
- Citations of `RL/AUDIT.md` at `C:36-43` match its findings 1, 2, 3, 4, 5, 8 and 9 (finding 4 makes four arenas the default until the contiguity invariant is mechanised). The design agrees with R-3, R-4, R-5, R-7, R-8 and E-2 of `FD/DECISIONS.md`. `MC/hash/poseidon/poseidon_cpu.rs:131-133`: the sponge puts the input length in the capacity word.
- -29,062 = 22,600 (one lookup) + 6,462 (one table column, upper fit) from `lanes/C/round1.md:13-14,175`. It is derivable for the two-column form over the spike node only (findings 6, 7).
- GP-2, GP-3 and GP-4 are not edited or narrowed by Profile A. The claim-counter rule follows R-5.

## 4. Claims not verified

- Every leaf, node, `RunLift` and decider row figure of sections 1.4, 2.2 and 3 beyond the measured parts named above; -51,204 to -51,662 rows; +18,000 to +40,400 rows for the subgroup check.
- All wall-clock, memory and hit-rate figures (`C:54,71-76,104-108,135-138`).
- The per-rule maxima of requests, old reads, ROM reads and appends; the 54-bit control word and the 2,048-row DEC table; the rule and table counts.
- That 4 bus requests per step serve every rule; that a layout with a second request row needs no new column.
- The one-arena non-interleaving claim (open, R-6).
- E8 column counts at `O/design.md:547-579` and the processor-analogy experiment lines (`C:129`).
- Lane line citations in section 4.1 other than those named in finding 14; round-1 files except the lines cited in this audit.
- The b2 analysis (`C:78`) and its row figures; the prior-art table and URLs.
- `INIT_PROG` and reload time figures (`C:53-54`). The argument does not depend on their size.

## 5. Module interface items for the architecture merge

### 5.1 Block cache (MEMORY BACKEND, Profile A)

1. A heap line is opened against the leaf's `pre.root`, never the running root. Records of segments flushed in this leaf are served by the window.
2. The 8 leaves of the line are `H_REC(kind, a, b, c)` with the capacity tags of `dense.rs:54-83`; `Tree8` uses heights 1 to 3 and the walk heights 4 to TOP + 3, as in `persist`.
3. The provider cells are the hashed cells (copy constraints). Row `j` of a line serves `(MEM, 8 (8 blk + j) + kind_j, a, b, c)`, with `j` fixed by the row.
4. The walk bits are the TOP-bit decomposition of `blk` (last accumulator zero).
5. `en` is boolean. With `en = 0` all five provider cells are zero and the root equality is off (as the gated equality at `dense.rs:743-749`).
6. `blk < q_pre`. Domain separation of `EMPTY_SEG` already prevents opening an unflushed slot; keep the check so that window and cache are disjoint by a local rule.
7. `kind` in `0..=4` holds for stored records by induction from `persist`. This needs a genesis root that is the empty tree or is proved by Init. Otherwise repeat the kind gate on line rows. In a four-arena variant a line names its arena root and pins the kind.
8. Deduplication and ordering of lines are not soundness conditions: the lookup is set inclusion and a provider is a function of its key (`bus.rs:18-20`). Add no constraint for them.
9. A line changes no boundary value. `(len, root, tail_d)` and the public statement do not depend on which lines a leaf opens (SRC-023).
10. Capacity: `K_page`, `K_rom` and the lane count are at least the per-step maxima of finding 1.

Why the five attacks fail under these rules:
- Stale line: memory is write-once and the tree changes only at slots that hold `EMPTY_SEG`. A segment below `q_pre` is the same under `pre.root` and under every later root (rules 1, 6), so a line cannot go stale inside a leaf or across leaves.
- Aliasing: `blk` is the walk index (rule 4), `j` is fixed by the row and `kind` is hashed into the leaf (rules 2, 3, 7), so the key `8 addr + kind` names one record. Two lines for one block serve equal tuples (rule 8).
- Partial segment: the tail segment `q_pre` is not in the tree; its slot holds `EMPTY_SEG`, which no `Tree8` of record hashes equals under the height and domain tags (rules 2, 6). A flushed segment always has 8 active records (`dense.rs:25-26`), so no pad record is under the root.
- Line forgery: an enabled line must reach `pre.root` from the cells it serves (rules 1 to 3); a disabled line serves only the zero tuple, which no active request matches (rule 5, `bus.rs:15-17`).
- Tail and window overlap: window rows have addresses at or above `8 q_pre` and lines below it (rule 6). The remaining risk is a read of a window row appended later in the same leaf; rule 11 closes it, not the cache.

### 5.2 CORE obligations toward memory

11. Every read takes its address from a register, constrains the kind that the rule expects, and checks `addr < len_cur` at that step.
12. Appends and new window rows correspond one to one (CORE append chain); `len` advances only by real appends.
13. Unused request lanes are zero in all five cells; an active request fixes a nonzero chip id.
14. Scratch `v0..v2`: write before read in the same step (R-8).

### 5.3 ROM

15. Own chip id. Key holds table id and row; row and table bits come from the path bits.
16. ROM leaf hash and node tags differ from the heap domains and carry heights.
17. Padding of unused rows and of the last line is canonical and part of the `prog_d` normal form.
18. `ty_d` and constant `vd` in ROM are trusted only through the PROG canonicality premise (`O/design.md:84`, open).
19. Depth and the program-size bound stated; no GP-1 narrowing without the owner (finding 12).

### 5.4 DEC and RANGE, ALU, HASH, VALUE DIGEST

20. Tagged table: tag from fixed selectors (three columns), or the stated bound on `ctrl` or `syn` (two columns). Zero row present. One DEC row per key. Range checks on every packed part of key and control word.
21. ALU: tuple `(ALU, op + 8 flag, x, y, z)`; `x`, `y` and the witness below 2^128 inside the chip; unique `(z, flag)` for every input, including overflow; 64-bit limb shape for multiply.
22. HASH: tags in the capacity (E-2); no provider row with an unconstrained output; a disabled permutation is off the bus.
23. VALUE DIGEST: `sherlock-value/1` unchanged; constants per preimage length; whole-sponge slots; conditions of finding 2.

### 5.5 Backend and boundary seams

24. `MemBackend` keeps `persist` and gains one block-line method; the token stays opaque (`membackend.rs:93-94`). `W` flush units are a parameter.
25. BOUNDARY COMMITMENT default: Poseidon digest over registers, stack hashes and `(len, root, tail_d)` per arena, capacity-tagged.
26. Profile B, if built: committed instance column bound cell by cell to the records it sums; challenges as public inputs equal to the hash of `d_root` at the seal; subgroup-checked commitment points; sum zero at the seal; no claim of stop and resume; listed as changing the leaf statement, node, seal and continuation modules.
27. Leaf public lanes reconciled with `ivc.rs:70-74` and the `h_pre == 0` genesis gate.
28. `LeafShape { s, w_flush, k_page, k_rom }` and the arena count stay parameters until the per-rule table, the locality census and the non-interleaving check exist.
