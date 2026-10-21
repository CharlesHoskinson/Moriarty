# Fresh Independent Non-Author Source/API/Formal Review — K `source10` (`k-next-public-nominal-identity-source10`)

## 1. Reviewer Identity, Routing & Execution Attestation
- **Role**: Fresh independent non-author source/API/formal reviewer under the standing October 16 observed Opus outage fallback to **Gemini 4 Argon (High)** via AGY, with the October 18 beta automation override active.
- **Requested / Returned Identity**: Requested `gemini-4-argon-high` at `high` effort via AGY (`IdentityrequestedHIGH/backendunattested`: model selection is Gemini 4 Argon (High) in the active AGY session; underlying provider runtime remains cryptographically unattested by host receipt).
- **Startup & Read-Only Discipline**: Loaded `plugins/moriarty-dev/skills/develop/SKILL.md`, ran `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`, read the complete 349,953-byte review packet `FULL-SOURCE-API-AND-EXACT1-REVIEW-PROMPT.txt` (`e4dd5b77d23bd5f32801258d8c676533acf0df19729cb64a065e3e87713fa881`), verified all 34 inverse-pinned inline bodies against disk files, verified `ROOT-HANDOFF-REF-VERIFICATION.json` (`485` bytes, `40b2d3ce974557997d2c5b9a276950fa86011863dcb1635bb5091282b33523e8`), and inspected the Rust declaration readset (`adt.rs`, `adt_storage.rs`, `nominal_declaration_order.rs`, `shared.rs` lines 315–370, `owner_ext_source_program.rs` lines 1–155, `owner_ext_t3_uses.rs` lines 1–150, `openspec/changes/sherlock-nominal-text-source/design.md`).
- **Zero Subject Execution**: No candidate modules were imported; no Python tests, K `kompile`/`kore-exec`, Quint, Lean/Clean, Cargo, SDK, native, proof, provider, or Git mutation commands were run. No private Clean bodies or protected cells were read.

---

## 2. Scoped Verdicts & Summary
- **Overall Scoped Verdict**: **`APPROVE_SOURCE_WITH_LIMITS`**
- **Source / API Adequacy (`k-next-public-nominal-identity-source10`)**: **`APPROVE`** (positive `ProgramView23` table-identity bridge and `SHERLOCK-PROGRAMVIEW-IDENTITY-SOURCE10` K observer extension).
- **Prospective Single Compile Proposal (`EXACT1` / `00-programview-identity-source10-compile`)**: **`APPROVE`** (eligible for Root selection via unchanged `F03.launch` after Root rechecks current pins and fresh directories; runtime execution remains **`UNSELECTED`** pending a separately owned/reviewed receiving decoder seam).
- **Mandatory Fixes**: **None (`0`)**.
- **Optional Improvements**: **None (`0`)**.

---

## 3. Verified Source Pins (`SOURCE-FREEZE.json`: `8897` B, `e51ead0dfd0ab1b26bcfdb81ad9f4f76067f44e14634285e145ba275d7cd96b4`)
1. **Unchanged Reused Base Sources** (`REUSED-SOURCE-BYTE-EQUALITY.json`: `2922` B, `4a66d63e66c42e3c5c74e5d02584624f4ac0381759a36cdbb30c8147c79b2c13`):
   - `source/nominal-product-syntax.k`: `5090` B, `750f9b820a672865b2a1abc50bef4f417f2c6c02e7b77f661a0431388dacab8c`
   - `source/nominal-product.k`: `21856` B, `29da74d7ab2b89cb23155521b94ffb6880920befcbf9806941ba97826d47b197`
   - `source/public-nominal-source09.k`: `3530` B, `6e961f2c8efc205fc83e16d0e6448753266ffd6c6ebf18b960e99d080cca08c1`
   - `source/program_view_to_ngprogram.py`: `26468` B, `ab7408993da8b1e46e8460b8d2a13ed8d436af1da735093ece9627db5f469fa0`
2. **New Candidate Sources & Tests (UNRUN)**:
   - `source/positive_identity_port.py`: `10293` B, `fc1dcadf0e3e8b18167fd41cd1ee9773e1dc2293161b575ab035f6148772debc`
   - `source/programview-identity-source10.k`: `1308` B, `d0dfcb4e1ee97011d3611ef409810d10357548fdad7d9a88a311b562287fd094`
   - `tests/programview-identity-source10-laws.k`: `1670` B, `a6d82079e15d5f51051fff3cfd18abeebaffc06c49fc15c7e2796e2f2f085448`
   - `tests/test_positive_identity_port.py`: `3563` B, `0553fa01bab822c2164a86abc97b113c48ea352ab7a8b5344158bd1595bd2f3b`
3. **Frozen Inputs & Independent Expectations**:
   - `inputs/map-queue.json`: `15399` B, `70cac9aa9b43dddd6c8361ed909a04a22b9778d816ee20f7b874d3a14a59491d`
   - `inputs/map-queue.mori`: `638` B, `b8b30eb7b66280065092d4e8590c8c798b640e64b3c748153a997107f8c8010d`
   - `inputs/mirror-arbor.json`: `17236` B, `fda25dc7346264988cb9189246aa446ce2261c57f7435c00ff433e2b477d8052`
   - `inputs/mirror-arbor.mori`: `735` B, `504fd3f85c354b778f4f5c4d49cd75155c21a62d087c24530f9a2fa51e93bfa9`
   - `EXPECTED-COMPLETE-LOGICAL-RETURNS.json`: `10000` B, `cb147037577d1a0ce8b467b1b4349d83cad20a6dc3a96b84c1f40bc56547e469`
   - `EXPECTED-map-queue-BINDINGS.json`: `41505` B, `b18fa616ddffa603083c2b3dc047090df09eb82cc3d74785283a5a1730bd9e3a`
   - `EXPECTED-mirror-arbor-BINDINGS.json`: `47484` B, `103c1b560c1c41b618921f2f033e52a81f4b7ac8cbbbf5f3f9879a842422c523`

---

## 4. Technical Audit Findings

### A. Source & API Correctness (`positive_identity_port.py` & `programview-identity-source10.k`)
1. **Runtime Table Index Semantics & Generic Derivation**:
   - Consistent with `crates/mori-core/src/sherlock/helper_machine/shared/adt.rs` (lines 4–23, 30) and `adt_storage.rs` (lines 4–16), `BUILTIN_TYPES` (`positive_identity_port.py` line 11) maps `"UInt": 0, "Bool": 1, "List": 2, "Tree": 3`, and `source_numbers` (lines 14–34) translates `{"TypeId": "<usize>"}` (lines 24–25) and decimal index/length strings (lines 32–33) to Python `int` while leaving `"names"`, `"decl_templates"`, and `U128` `"n"` untouched via `copy.deepcopy(value)` (lines 21–22).
   - No fixture-switch or hardcoded `source09` dummy IDs exist in producer code. In `previous.Port.declaration` (`program_view_to_ngprogram.py` lines 320–336), `type_id` is the runtime `types` index (`2..5`) and `ctor_id` is `row["ctor_start"] + local` (yielding global constructor IDs `4, 5` for `Queue`/`Arbor` and `6` for `Envelope`/`Stamp`), with `source_order` and `field_types` derived from `constructors` and `ctor_fields` slices (lines 338–351).
   - `type_metadata` (`positive_identity_port.py` lines 49–57) slices `view["type_args"]` by `[start:start+len]` in exact declaration order (`[4, 1]` for `Envelope<Queue<UInt>, Bool>`; `[4, 0]` for `Stamp<Arbor<Bool>, UInt>`) without sorting, and resolves user type names from `view["decl_templates"][row["decl"]]["name"]`.
   - `constructor_metadata` (lines 59–70) slices `view["ctor_fields"]` in exact field order (`["z_item", "a_tail"]`, `["z_items", "a_marker"]`, `["z_left", "a_right"]`, `["z_tree", "a_code"]`) and resolves user constructor and field names via `view["names"][...]`. Builtin constructors (`owner < 4`, lines 62–66) use `BUILTIN_CTORS[(owner, row["source_order"])]` and explicit ordinal labels `field0`/`field1` because builtin `ctor_fields` carry placeholder `name_id = 0`.
2. **Whole-Document `ProgramView23` Retention & `NIJSON` Fidelity**:
   - `port_positive` (lines 72–90) stores `raw = copy.deepcopy(document)` before normalizing `document["program_view"]`, preserving all 23 `PROGRAM_FIELDS` (`program_view_to_ngprogram.py` lines 15–21), raw string vs `null` distinctions (`types[4].decl = "0"` vs `types[0].decl = null`, `Match.plan = "0"`, `Construct.constant = "4"`/`"5"` vs `null`), unnormalized empty slices (`constructors[4].field_start = "5", field_len = "0"`), and duplicate constants (`constants[4]` and `constants[5]` in `map-queue.json`).
   - `json_ast` (lines 107–116) maps `None`, `bool`, `int`, `str`, `list`, and `dict` losslessly into `NIJSON` constructors (`niNull`, `niBool`, `niInt`, `niString`, `niArray`/`NIJItems`, `niObject`/`NIJFields`), matching `SHERLOCK-PROGRAMVIEW-IDENTITY-SOURCE10` (`programview-identity-source10.k` lines 6–9). `json_from_ast` (lines 118–131) is clearly scoped as a positive-image inverse for local test verification, not an untrusted K output parser.
3. **Delegation to Existing `Port`, `source09` `Not`, and `npRunReference`**:
   - `PositivePort.expression` (lines 36–44) intercepts only `Unary` with `op == "Not"`, validating `("op", "operand")` via `previous.record` and `self.ref("expressions", ...)` and emitting `{"id": index, "occurrence": index, "kind": {"Not": {"operand": operand}}}`, while delegating all other forms to `super().expression(...)`.
   - `expression_ast`, `program_ast`, `np_type_ast`, and `query_ast` (lines 150–198) construct a closed constructor-name AST whose sort and constructor arities match `nominal-product-syntax.k` (lines 7–27), `public-nominal-source09.k` (lines 5, 20–28), and `programview-identity-source10.k` (lines 6–11).
   - In `programview-identity-source10.k` (lines 10–20), `niRun(Source, P, Ts, Cs, Cuts)` rewrites to `niObserved(Source, npRunReference(P, Ts, Cs, Cuts))`, and `niSource`, `niReferenceObservation`, `niCompleteState`, and `niOrderedEvents` project the complete carried `NIJSON`, `NPObservation`, `NGState`, and `NGEvents` without dropping frames, environments, or event ordering.
   - Static trace verification confirms that under `npRunReference(..., 512)`, the ported `map-queue.json` and `mirror-arbor.json` programs evaluate using runtime `TypeIdC` `4, 5` and global `CtorIdC` `4, 5, 6` to the exact logical trees in `EXPECTED-COMPLETE-LOGICAL-RETURNS.json` (`Wrap(Push(5, Push(10, Empty())), true)` and `Seal(Fork(Fork(Tip(false), Tip(true)), Tip(false)), 41)`).

### B. Test Meaning & Correlation Qualification
- **8 Python Tests (`tests/test_positive_identity_port.py` lines 13–58, UNRUN)**: Recorded prior to implementation (`TEST-FIRST-RECORD.json`, `590` B, `f7fd6985f136b6c26b767aa97f3f19fc4d8f0cc7d0b6f51e68ceeaf215432c2e`), though unrun (no observed RED/GREEN). They verify raw identity preservation (`test_complete_raw_identity`), runtime global IDs (`[2, 3, 4, 5]`, ctors `[4, 5]`, `ctor_id: 6` in `test_runtime_global_ids_and_order`), unsorted generic type args (`[4, 1]` and `[4, 0]` in `test_generic_arguments_are_not_sorted`), declaration order of same-typed fields (`["z_left", "a_right"]` in `test_equal_typed_fields_keep_declaration_order`), preservation of `"5"`/`"0"`, `"0"` vs `None`, and duplicate constants (`test_empty_slice_and_some_zero_preserved`), `Not` porting (`test_not_uses_existing_semantics`), `NIJSON` data roundtrip (`test_whole_source_ast_and_query_are_data`), and input-table name derivation under mutation (`test_names_are_input_derived`). Fixture-specific literals `4, 5, 6` appear only in test assertions, never in producer logic.
- **3 K Structural Laws (`tests/programview-identity-source10-laws.k` lines 4–24, UNRUN)**: Same-author post-implementation diagnostic equations checking `niSource`, `niCompleteState`, and `niOrderedEvents` on synthetic `niObserved` terms. They do not execute `niRun` on the two fixtures or establish compiler/Core prefix equivalence.
- **Oracle Independence**: `nominal-programview-identity-independent-expectations01` was frozen from input-only `ProgramView` tables and `.mori` declarations before `source10`. Because the `source10` author read those authorized expectations and older `source09` outputs, the author is not counted as an independent oracle vote.

### C. Adequacy of Prospective Single Compile (`PROPOSED-ONE-EXISTING-F03-COMPILE-DATA.json`)
- **Pin & Resource Closure**: `PROPOSED-ONE-EXISTING-F03-COMPILE-DATA.json` (`7634` B, `2ebe602848f103e17c1d4acabd54b1d752648e7f1956618c06d79ad718832756`) proposes a single `kompile` phase (`00-programview-identity-source10-compile`) of `tests/programview-identity-source10-laws.k` (`--backend haskell --no-haskell-binary --main-module SHERLOCK-PROGRAMVIEW-IDENTITY-SOURCE10-LAWS --syntax-module SHERLOCK-PROGRAMVIEW-IDENTITY-SOURCE10-LAWS`) through the unchanged `F03.launch` callable (`k_engine_attempt_f03_01.py`, `73962` B, `f1bcf0a0d21b4a7aaedd2f2aae1a1e9e5b62219271eb41b3f71ed97d5a0bc24d`) and unchanged tool pins (`TOOL-ENTRY-PINS.json` `f93c73ffbb5b0cca5244b4c61aaa14172eaa357e4f29fb978e03b0d0cc8d628b`, `EXISTING-K-TOOL-CLOSURE-PINS.json` `b88dfeb16de28599e485fa1cc7fd4a7ec9987fa16daa712e0256067acde6418a`).
- **Resource & Product Policy**: Retains unchanged per-child bounds (`wall_seconds = 30`, `sweep_seconds = 3`, `rlimit_core = 0`, `rlimit_fsize = 536870912`, `umask = 0o077`, JVM `-Xms64m -Xmx512m -Xss32m -XX:ActiveProcessorCount=2`, process-group and `SHERLOCK_K_ATTEMPT_TOKEN` sweep), requires `definition.kore` nonempty, and inventories/hashes all actual outputs without guessing a product count.
- **Copy Map & Inverse**: `SOURCE-COPY-MAP.json` (`12844` B, `f5d355c3e5bef9f736b6d6c77629edb9d413aaf9a44846e2d230c13c49fa9639`) and `FORWARD-INVERSE-READBACK.json` (`1317` B, `edd8e51ec24dc895023a768fb0229dc0cedb69bddac6966dd56646448876979a`) specify 13 `ADD_ONLY` targets under `models/sherlock/k/public-nominal-identity-source10-20261020/`, all verified absent in `PW`.

### D. Required Open Gaps & Exact Next Step
- **Receiving Seam Required Before Runtime**: The existing NG K-output decoder predates `source09` `Not` (`ngNotArgument`, `ngNotStarted`, `ngNotReturned`), `NPObservation`/`NPType`/`NPValue`, and `NIJSON`/`NIObservation`. No untrusted parser or symbol-binding adapter is authored here. Consequently, **`source10` runtime execution is NOT eligible yet**.
- **Non-Claims Preserved**: Old public compiler captures (`event_capture = not-retained`) have no ordered event stream, so forwarded counts cannot establish Core prefix correspondence. Source/compile approval does not establish compiler table-generation correctness, work/fuel clocks, `StepInv`, admission, security, refinement, native proofs, or EARS closure.
- **Exact Next Source / Check**:
  1. Root rechecks current integration base and pins, then selects the single `00-programview-identity-source10-compile` operation via unchanged `F03.launch`, retaining/fsyncing raw outputs and inventorying all actual products and compiled Kore labels in `definition.kore`.
  2. Separately author and freshly review the designated-owner K-output receiving/decoder adaptation (binding actual compiled Kore labels and covering `NIJSON`, `NIObservation`, `NP*`, and `ngNot*`) and run the 8 positive Python port tests before selecting any `source10` runtime evaluation capture.
