# ValidateEntry emitter01: fresh independent security source/API review (review02)

**Reviewer:** Claude Opus 5.5 (High) through AGY, as reported by the host model setting. I am not the author and did not read any earlier Gemini review conversation. Author prose and hashes were not used as evidence.
**Candidate:** emitter `c6_validate_entry_step.rs` (50079 B, sha256 `6bd8e745a336adef45d66b92ff06993e0f8b2be6d7edc94ea401d4d254e46c8f`) and parent `mod.rs` (21537 B, sha256 `c6df833b3e29665567a27607d1a7f1634ac4586a5d9791cdbf742febcb723cd7`). I recomputed both hashes with read-only `sha256sum`, and both match `SOURCE-FREEZE.json`/`FILE-MAP.json`.
**Method:** I only read files. I used `sha256sum`, `wc`, `diff`, `ls/find` and `grep/awk/sed` on the packet text, all read-only. I did not import, compile, run tests, use circuits, SDK, native code, provider, Git or network, call subagents, or edit source. I did not load the `moriarty-dev:develop` skill or run its CLI, because this assignment restricts execution. Factual notes are in `NOTES.md` next to this file.

## Verdict: **APPROVE_SOURCE_WITH_LIMITS** (exact frozen hash only)

I found no defect in the emitted relation. The two issues below do not change any constraint, so I recorded them as binding limits rather than REVISE defects. If Root treats contract-doc inaccuracy as blocking, the exact fixes are given. Any edit changes the hash and needs a fresh review.

### Binding limits and non-relation issues
1. **False receipt-contract sentence.** Module doc L80–82 says `const_found` is "gated by `enable` inside `emit_prog_const_capture`". The actual source sets `nxt.const_found = sel(lane_active, 1, …)` and `const_count` with no gate (packet PROMPT.txt L7151, L7159). Consumers must treat `const_found`, `const_count` and `const_row` as **ungated**, like every other receipt. Exact fix: drop the parenthetical "(unless noted by `const_found` …)".
2. **Unused import.** `IMAGE_SECTION_HEADER_LANES` is imported at emitter L106 and never used. I counted one occurrence. This gives an `unused_imports` warning and fails only under `-D warnings`. I saw no deny lint in the crate root (L19402–19444) or in Cargo (L19447–19477).
3. **Imprecise comment, no code impact.** The comment at L922–923 links the carrier check `pre.task_live < caps.max_helper_tasks` to `owner_physical_check`. Core's physical check actually tests the same *admission* `b.helper_tasks` (L11899), so it can never fail for this row once `cap_first` is `None`. The carrier check is an encoding requirement: the applied post must fit the padded carrier. The constraint is correct.
4. **Cost.** PROG is walked three times: the private scanner, the full gated walk inside `emit_prog_const_capture`, and the canonical `emit`. That is correct but costs rows. Circuit degree k is unmeasured.

## Relation assessment against actual Core

| Obligation | Core source | Emitter | Result |
|---|---|---|---|
| Selector: P0 Running, P2 fuel>0, no helper/result, `Eval{expr}` with a Literal row ⇒ `BHelperValidateEntry`, helper-eligible, 0 visits | L12083–12103, L11256–11263, L11282–11284, L11407–11424, L11355–11371, L11324–11333 | L795–811 (helper/result/task-count=0, outcome [0,0,0], `0<remaining`, control tag 0, lanes 2..17 zero); L861–869 (scanner, `expr<count`, found, tag 0) | ✓ |
| `control_origin == expr` | `RecordingPreImageView::expr` records `Const(c)` only when `id == control_origin` (L11577), and `check_copied_refs` (L11600–11613) rejects otherwise | L814–818 | ✓ matches Core exactly |
| Root = `RefC::Const(c)` taken from the actual selected row, `c < |constants|` | L34190–34201, SC-05 L11728–11731 | `expr_lane1` from the bounded scan; const capture `found==1`, `c<const_count`, `c<max_constants` (L870–892). No host hint and no fixed program. | ✓ |
| All 8 const kinds, no early kind walk | `validate_entry_payload` reads no kind | `const_row` is only a receipt and is never constrained by kind | ✓ |
| Helper `Validate{common{EvalLiteral, pre.reference_cursor, None}, Const(c), Pending}` | L34174–34177, L34196–34197; codec L16478–16480, L16553, L16440, L16450 | L984–1003: tag 7, site 0, origin=pre cursor, pr [0,0], root [0,c], status 0, lanes 12..26 zero | ✓ |
| Task `ValidateTask{Const(c), Unvisited, 0}` pushed at slot 0, count+1 | L34198–34199, L12035; codec L16799, L16463; header lane 3 = count (L16346) | pre count 0 enforced; L1004–1015 and lane 3 = 1 | ✓ |
| P3: `helper_work == admission max_helper_work`, uncharged `Exhausted{HelperWork}`, everything else unchanged, overrides P6 | L12106–12111; exhaust tags L15080–15082; outcome codec L15488–15501 | `is_p3` uses the retained `source_params.admission_bounds()` (u128, L9229); outcome [3,5,0]; counters and helper preserved; no overflow assert on the P3 path | ✓ |
| P4 checked charge: remaining−1, consumed+1, helper_work+1 | L12113–12117 (u128 checked) | `charged_gate` only: `post+1==pre` with `pre>0`; `pre<u128::MAX` ⇒ +1 | ✓ |
| P6 HelperTasks: `task_live+1 > helper_tasks`, charged, no helper or task write | L11886, L12119–12126 | `!(task_live < helper_tasks)`; [3,4,0]; `post_h == pre_h` | ✓ |
| Physical check before P3 | L11891–11900, using the same admission bound | Cannot diverge for this row (see limit 3) | ✓ |
| P7 preservation: visits +0, cursor unchanged (not LegacyComplete), control, origin, env, frames, alloc, active, seg, DATA, META, PROG, CTX, result | L11994–12046 | Mutable image lanes are exactly outcome[3], remaining, consumed, helper_work (L957–969). DATA/META/PROG/CTX equality is unconditional. Helper lanes outside the record/count/slot0 are preserved. | ✓ |
| Physical task cap 0 | — | Helper loop only spans `0..helper_total_lanes` (=31). Applied becomes unsatisfiable through `glt_index(...,0)`. P3/P6 stay satisfiable. | ✓ |
| Pre-image canonicality | `decode_image_section`/`root_caps` L15448–15583 | env, stack_limit, alloc/active/seg counts (strict index vs inclusive count, usize-fit) at L820–846; all 10 frames line-equivalent to push step L8541–8624; inactive frame slots zero | ✓ |
| Disabled / gate | — | `cbool(enable)` rejects 2; enable=0 gives raw-128 `post==pre` on every section; disposition lanes are only boolean | ✓ |
| Statement dispositions | — | `dp_hw==is_p3`, `dp_ht==is_p6` under enable; Applied ⇔ both 0 | ✓ |

I checked the API and the Rust code against actual definitions. Every import and called item exists with the signature used: `C6PhysCaps` (27 fields, L14917), `C6StateCaps` (Copy, L16025), `C6PaddedCarrierGeometry` (Copy, L7962), `validate_geometry`, `admission_bounds -> BoundsC`, `limits`, `meta_binding`, `expected_ctx` (L8325–8345), the meta/data/helper/progctx input structs and emit fns (L5813, L7437, L5259, L6674–6732, L7227–7309), and the SDK `Instruction` variants (L19998–20460). On the Rust side:
- Partial moves out of `ExprCapture` and the const-capture block are legal; neither type implements Drop.
- The disjoint field borrows (`&self.scope` alongside `&mut self.progctx_em`) follow the same pattern as the in-tree ScalarEntry template (L34764–34994).
- Every CondSelect bit is boolean.
- Every LessThan operand fits its bit width (128 bits, or 129 for slice ends).

The parent `mod.rs` changes in exactly two hunks against the frozen 21314 B parent: the `pub mod` line and the `pub use` of five public names.

## Retained premises (not discharged; caller/owner obligations)
- **Empty task stack.** This emitter requires an empty task stack as a declared stronger valid pre-state. In source that is SI07 (L12469). The presource's "SI23" label is a slip, since SI23 is the value-slots clause.
- **Core snap lookup.** Core needs `pre.occurrence(control_origin)` and `occurrence_path` to succeed in `snap` (L11909–11916). The emitter does not encode this. It sits under the program-admission/membership premise, the same as the ScalarEntry template.
- **AB binding.** `PreImage::bind` and admission/program membership, plus the requirement that `source_params.admission_bounds()` equals the actual `Admitted.body.bounds`. That equality is a consumer/witness obligation.
- **Not established here:** full StepInv, SI08/SI17/SI21/SI22, V6, P1, digest authentication, and native issuance.

## Not established by this review
- No consumer, owner-record, host-pair, witness, builder or checker was delivered, for either LocalPublic or LedgerCall.
- No genuine or raw negative tests, compile, MockProver, native proof, measured k or performance results exist.
- The downstream rows are not covered: Validate Occurrence/Field/Empty/Invalid/Exit/Failure and KMachineEvalLiteral Valid/Invalid.
- Full-39 / grouping / financial / recursive / stagenet goals are outside this review.

This approval covers only the bounded source relation of this one entry row. It does not close the leaf, the consumer or the full goal.

## Coverage
- **Read completely:** emitter (1129 lines, plus a reread of L860–936 before writing this); parent `mod.rs` and its diff; `FILE-MAP.json`; `SOURCE-FREEZE.json`. Packet units 006 (progctx canonical), 080 (structural family), 081 (family dispatch), 082 (ScalarEntry template), 083, 084, 025, 026.
- **Read in part (the cited ranges):**
  - 011 owner driver: L11140–11440, L11540–12140, L12448–12475.
  - 008 push step: L7808–8420, L8486–8630.
  - 018 c6_phys: L14848–14950, L15300–15600.
  - 019 c6_phys_data: L16022–16035, L16071–16110.
  - 020 c6_phys_helper: L16336–16560, L16874–16934.
  - 021 c6_phys_program: L16936–17100, L17282–17345, L17578–17650.
  - 004 helper canonical: L5255–5466.
  - 005 meta: struct/sig only.
  - 007 data: struct/sig only.
  - 009 BoundsC.
  - 030/034 SDK: definitions and VM/in-circuit arms for the instructions used.
- **Not read:** units 001–003, 010, 012–017, 022–024, 027–029, 031–033, 035–079 (SDK P2R/Fr/field internals beyond the instruction contracts above); the author FINAL/RESULT prose (deliberately); consumer/test folders; all prior Gemini reviews.
