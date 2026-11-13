# X10: the state header opening inside the node

Date: 2026-11-12. Loop item X-EXP of `research/sherlock-e-path-20261112/LOOP.md`. Branch `x-exp` (from `final-design-2` at `efb0eb78a`). Experiment definition: `research/sherlock-final-design-20261111/ARCHITECTURE.md` section 5.4, row X10: "the state header opening inside the NODE seal modes: `open_state_header` with the 42-word header sponge of section 1.11, FP-5 ... with node and decider rows and table rows". It decides AD-19 and N-2 (does the seal-time opening fit; else the fallback leaf statement, AD-13), and gates I-12 and I-35.

Labels: MEASURED is read from `out/x10-run.jsonl` (one run, shared machine, no `--exclusive`, so the times are not timings). DERIVED is computed from MEASURED numbers by a stated rule. READ is a code or text reading.

## Result

The opening fits the node with a large margin. One AIR of 8 rows does it, and the node keeps its stacked width.

| Quantity | Node without opening | Node with `HeaderOpenAir` | Bound (internal set) | Label |
|---|---:|---:|---:|---|
| AIRs per node | 39 | 40 | 50 | MEASURED |
| `HeaderOpenAir` height, width | | 8 rows (2^3), 307 columns (299 Poseidon2 + 8 selectors) | 2^19 rows | MEASURED |
| `HeaderOpenAir` constraints, interactions, degree | | 362, 0, 3 | 1,000, 100 | MEASURED |
| largest constraints in one AIR | 282 | 362 (`HeaderOpenAir`) | 1,000 | MEASURED |
| largest interactions in one AIR | 77 | 77 | 100 | MEASURED |
| widest AIR | 301 | 307 | | MEASURED |
| sum of AIR widths | 1,543 | 1,850 | 2,000 | MEASURED |
| stacked width needed | 69 | 69 | | MEASURED |
| tallest AIR | `FinalPolyQueryEvalAir`, 2^18 | same | 2^19 | MEASURED |
| main trace cells | 35,334,472 | 35,336,928 (+2,456 = 8 x 307) | | MEASURED |
| prove and verify | yes | yes | | MEASURED |

The node is arity 2 over two W1 app leaves of 2^16 rows (one Poseidon2 AIR each), at the OD-19 internal set (`internal_ud()`, the s10x parameters). Prove times were 23.9 s and 23.0 s on a shared machine (not timings).

The column bound is 2,000 trace columns in total across all AIRs, unstacked (stark-sdk `config/mod.rs`, "Assumptions for 100-bit security"; PROPOSAL line 200). The sum of AIR widths in the tables is that count.

Checks:

- **Parity with the reference sponge (MEASURED).** `header_trace` gives the same 8-word digest as stark-sdk `PaddingFreeSponge<Poseidon2, 16, 8, 8>` over the 42 words for a random header, a header with a zero payload, and a header that is zero after the domain word (3 of 3 equal).
- **Mutant (MEASURED).** A trace with the `view_root` word forged (word 34 + 1) and the honest `post_d` as public values does not verify (`BatchConstraintError(SumClaimMismatch)`).
- **Gemini Quint lane (lead run).** `qa/gemini-quint/x10-header/REPORT.md`: SOUND and COMPLETE with conditions. Lead results: both typechecks pass, 13 of 13 tests pass, `invOpen` holds on `hdr_sound` (200 samples, seed 0x10), both non-vacuity reach invariants are violated as expected, and all six mutants M1 to M6 are caught. Lane finding: the sum-to-one selector constraint is required. Without it, padding row 6 can switch on `s[0]` a second time, because the shift constraint never sets column 0 on later rows (M5).

What this gives the open rows (no row is frozen by a new choice here):

- **AD-19 (b) and N-2 (a)**: the seal-time opening fits. The fallback `sherlock-leaf-stmt/2` (N-2 (b), AD-13 fallback) is not needed on this evidence.
- **I-12** (54-word leaf public values) and **I-35** (seal read of `view_root`): the X10 gate is met for the opening AIR. Two node parts are still open (next section).
- **N-14**: the header layout of FP-5 (domain, tag, payload, `input_d`, `deps_d`, `view_root`, `body_d`; 42 words, 6 permutations) gives a 6-permutation opening, as the ESTIMATE in the BOUNDARY COMMITMENT kit says (ARCHITECTURE line 2040). It is now MEASURED.
- **Decider rows**: not applicable. Under OD-24 option E, `L` opens the sealed SET head, not a state header (ARCHITECTURE section 1.11, FP-3), so there is no decider opening to measure.

## Not in this measurement

- **The link of the public values to the child's `post_d` (DERIVED, not wired).** In the bench, `HeaderOpenAir` takes its 8 public values from the host. The production node must make them equal to the authenticated `post_d` lane of child slot 0 (FP-2 payload words 16..24). This is one equality of 8 words, or one bus message, in the node's public-value AIR. It adds at most 8 columns or 1 interaction, so it does not change the fit.
- **Export of the opened words to SEAL.** `HeaderOpenAir` has 0 interactions. SEAL and SEAL-WITH-DEPS need the opened `outcome_tag`, `outcome_payload`, `input_d`, `deps_d` and `view_root` (`body_d` stays private). One bus message per row on rows 0..5, gated by `s[0..6]`, stays far below 100 interactions per AIR (DERIVED). The SEAL predicates themselves (terminal tag, `deps_d` check, `VIEW` claim, `claim_d`, SET root) belong to NODE INTEGRATION and are not measured here.
- **The leaf chip's header digest (`boundary_header_leaf_vs_std`).** No BabyBear leaf kit exists yet (CORE is a halo2 spike), so the parity check is against the stark-sdk sponge, which a leaf chip must match.
- **`SHK_SD2`** is a stand-in constant. ENC-1 does not give the BabyBear value; any fixed constant gives the same shape.

## Method

- Code: `src/x_exp_node.rs` (a copy of the bench example in the s10x stark-bench checkout, OpenVM `05f3b73`, crate `stark-bench-ovm`, entry `[[example]] name = "x_exp_node"`). `HeaderOpenAir` is the stock `Poseidon2SubAir` (SBOX_REGISTERS = 1) through `SubAirBuilder`, as the recursion `DagCommitSubAir` uses it, plus 8 one-hot row selectors. Row 0 fixes the domain word and zero capacity, rows 1..5 chain capacity, row 5 keeps rate words 2..8 (overwrite mode, a 2-word last block), and the row-5 rate output equals the 8 public values. Rows 6 and 7 are padding.
- Node: `VerifierSubCircuit<2>` over two app-leaf proofs, `HeaderOpenAir` appended as AIR 39, keygen, trace generation, prove, verify. `shape()` reads widths, constraints, interactions and the stacked layout from the proving key, as s10x does.
- Build: `cargo build --release --locked -p stark-bench-ovm --example x_exp_node` under `heavy-slot` (Rust 1.91.1 from the checkout, `-C target-cpu=native`). Run: `x_exp_node x10 16 2` under `heavy-slot --mem 40G`, rc 0, 126 s.

## Files

- `out/x10-run.jsonl`: the header-AIR record with parity vectors, the leaf, the node without opening, the node with opening, the mutant.
- `out/x10-run.err`: run log.
- `out/x10-quint.txt`: the lead's quint commands and exit codes.
- `src/x_exp_node.rs`: the bench source (also used by X7 and X14).
- `qa/gemini-quint/x10-header/`: lane report, model `header_open.qnt`, tests `header_open_test.qnt`.
