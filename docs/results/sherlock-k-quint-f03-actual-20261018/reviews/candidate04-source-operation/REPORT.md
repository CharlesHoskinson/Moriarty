# Follow-Up Non-Author Source & Three-Operation Review (Candidate 04)

## 1. Reviewer Identity & Non-Author Attestation

- **Conversation ID (`reviewer_cid`)**: `7d931b5d-fc0c-4a84-927e-2b7af395d63b` (same independent reviewer context as Turns 1 and 2)
- **Working Directory (`cwd`)**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ021-F03-K-caller-materialized-review/F03-next-capture-and-projection/optional-lf-candidate-02/fresh-nonauthor-review-transport/fresh-provider-cwd`
- **Model**: Gemini 4 Argon (High)
- **Independence**: Disjoint from excluded original assembler authors (`dfa2b7b7-a887-472a-a285-dc24c628728b`, `7356947d-bd19-4cc5-a58b-643f0d3bf399`), optional-LF / Candidate 03 / Candidate 04 source author (`8b5575c5-d1b9-4add-bcb1-5dd7c719e100`), and failed negative-unit author (`85cf...`).
- **Execution / Mutation Discipline**: Zero tools, zero commands, zero provider calls, zero file edits, zero fixes/tests authored, and zero model/evaluator executions performed.

---

## 2. Exact Body Coverage Inventory

| Coverage Tier | Artifact / Path | Size & Digest | Status in This Review |
| :--- | :--- | :--- | :--- |
| **`PROVIDED_FULL_BODY` (Read Inline in Turn 3)** | `.../logged-input-chain-candidate-04/afterimages/models/sherlock/qa/k_capture_observations.py` (BODY 1) | `42838B` | Complete Candidate 04 decoder read and audited inline |
| **`PROVIDED_FULL_BODY` (Read Inline in Turn 3)** | `.../logged-input-chain-candidate-04/SOURCE.diff` (BODY 2) & `CONSUMER.diff` (BODY 4) | Exact diffs from Candidate 03 decoder (`42034B`, `70db66d9...`) and Candidate 03 consumer (`6240B`, `34285adf...`) | Complete diffs read and verified against BODY 1 and BODY 3 |
| **`PROVIDED_FULL_BODY` (Read Inline in Turn 3)** | `.../logged-input-chain-candidate-04/test_actual_f03_decode_only.candidate04.py` (BODY 3) | `6248B` | Complete Candidate 04 decode-only positive consumer read and audited inline |
| **`PROVIDED_FULL_BODY` (Read Inline in Turn 3)** | `.../review-inputs/ALL94-READABLE-BYTE-SPAN-PROJECTION.json` (BODY 5) | All 94 post-startup edges (`0..93`) byte/LF/diff-offset/inventory-span projection + 20 referenced inventory rows | Complete 94-row matrix and 20 inventory span rows read inline (2 SHA256 columns omitted from readable projection only; pinned in `ALL94-BYTE-SCHEMA-SPAN-MAP.compact.json`, `22020B`, `993582a2...`) |
| **`PROVIDED_FULL_BODY` (Read Inline in Turn 3)** | `.../inputs/FIVE-COMPLETE-RAW-PAIR-REPRESENTATIVES.lossless.json` (BODY 6) | `46806B`, SHA256 `904872559d6920f99fb33438aacfac31a9405865d3da4c7707344cde714cf531` | Complete lossless line-factored raw `(input_kore, actual_logged_config)` pairs for edges `0, 8, 10, 40, 91` read inline |
| **`PROVIDED_FULL_BODY` (Read Inline in Turn 3)** | `.../logged-input-chain-candidate-04/OWNER-NARRATIVE-QUALIFICATIONS.json` (BODY 7) & `.../review-inputs/THREE-OPERATIONS-EXACT.json` (BODY 8) | Owner report qualifications and exact 3-operation schedule | Read and verified inline |
| **`PRIOR_QUALIFIED_SOURCE` (Read Inline in Turns 1–2 of Same CID `7d931b5d...`)** | `.../logged-input-chain-candidate-04/afterimages/models/sherlock/qa/assemble_k_f03_observations.py` (`63706B`, `83a6424f...`), `k_observations.py` (`7686B`, `540128fd...`), `k_terms.py` (`5666B`, `c79a852c...`) | Byte-identical copies of modules read in full inline in Turns 1 and 2 | Audited in Turns 1–2 and externally pinned in `FLAT-INPUT-PINS.json` (not reprinted in Turn 3) |
| **`SOURCE_AVAILABLE_NOT_FRESHLY_READ` (Externally Pinned)** | Other 89 raw `(input_kore, actual_logged_config)` edge pairs, `ALL94-BYTE-SCHEMA-SPAN-MAP.compact.json` (`22020B`, `993582a2...` with the two SHA256 columns), `models/sherlock/qa/quint_observations.py` (`16774B`), `models/sherlock/qa/check.py` (`81758B`), `quint-single-run-manifest.proposed.json`, 95-state Quint ITF capture, full 113-row `COMPILED-MAIN-RULE-SPANS-01.json` (`85954B`), `K-COMMANDS.json` (`413065B`), `main.k` (`16249B`), `definition.kore` (`1338414B`), `F03-fuel093-stack064.ui` (`359B`), parent `receipt.json` (`1642368B`), and frozen F03 oracle fixtures | Pinned in external 517-entry `FLAT-INPUT-PINS.json` (`138328B`, SHA256 `4da35ee16179782e21a602dda954e8e228caf64c6028ec70a0b6a23b76375607`) | Externally pinned machine evidence; bodies not provided or read inline in Turn 3 |

---

## 3. Mandatory Literal Readbacks

1. **`_INLINE_MAP_KEY_INJ` Declaration** (`k_capture_observations.py`, lines 88–90 / `SOURCE.diff` Hunk 1):
   ```python
   _INLINE_MAP_KEY_INJ = re.compile(
       r'( *)/\* Inj: \*/ inj\{SortInt\{\}, SortKItem\{\}\}\(\\dv\{SortInt\{\}\}\("(-?[0-9]+)"\)\),'
   )
   ```

2. **Full New Helper `_logged_map_key_line` and Updated `_logged_input_gaps` (Including `expected` Inline/Wrapped Membership Check)** (`k_capture_observations.py`, lines 503–530 / `SOURCE.diff` Hunk 2):
   ```python
   def _logged_map_key_line(line):
       """Render an inline SortInt map-key injection in kore-exec's wrapped On-configuration form."""
       match = _INLINE_MAP_KEY_INJ.fullmatch(line)
       if match is None:
           return ' ' * 10 + line
       indent = ' ' * (10 + len(match[1]))
       return (
           f'{indent}/* Inj: */ inj{{SortInt{{}}, SortKItem{{}}}}(\n'
           f'{indent}    \\dv{{SortInt{{}}}}("{match[2]}")\n'
           f'{indent}),'
       )


   def _logged_input_gaps(entry, input_kore):
       """Require the logged On-configuration term to match the re-indented input Kore term."""
       if entry is None or len(input_kore) > MAX_KORE_BYTES or not input_kore.isascii():
           return ['logged-configuration-is-not-input-kore']
       lines = input_kore.decode('ascii').split('\n')
       inline_term = [' ' * 10 + line for line in lines]
       inline_term[-1] += ','
       wrapped_term = [_logged_map_key_line(line) for line in lines]
       wrapped_term[-1] += ','
       expected = {
           '\n'.join(_LOGGED_OPEN + tuple(inline_term) + _LOGGED_CLOSE) + '\n',
           '\n'.join(_LOGGED_OPEN + tuple(wrapped_term) + _LOGGED_CLOSE) + '\n',
       }
       return [] if entry['configuration_text'] in expected else [
           'logged-configuration-is-not-input-kore']
   ```

3. **Unchanged Rule-Span Matching & Non-Inference in `_step` and `decode_capture`** (`k_capture_observations.py`, lines 490–496 and 638–647):
   ```python
           elif _span_key(entry['locations'][0]) not in spans:
               gaps.append('applied-rule-span-not-in-compiled-inventory')
           else:
               location = entry['locations'][0]
               rule = dict(location, unique_id=spans[_span_key(location)])
   ```
   ```python
           step = _step(capture['incoming_log'], capture['state_kore'], capture['configuration'],
                        engine, printer, source_path=source_path, spans=spans, rewrite=True)
           edge = list(step['gaps'])
           edge.extend(_logged_input_gaps(step['applied'], previous) if previous is not None
                       else ['input-kore-not-supplied'])
           edge = list(dict.fromkeys(edge))
           logs.append(step['decoded_log'])
           gaps.extend(f'edge-{index}-{gap}' for gap in edge)
           rules.append(None if edge else step['rule']['start_line'])
           identities.append(None if edge else step['rule'])
   ```

4. **Candidate 04 Consumer Path/Root Binding & Source-Origin Assertion** (`test_actual_f03_decode_only.candidate04.py`, lines 11–18 / `CONSUMER.diff`):
   ```python
   ASSEMBLER=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ021-F03-K-caller-materialized-review/F03-next-capture-and-projection/logged-input-chain-candidate-04/afterimages/models/sherlock/qa/assemble_k_f03_observations.py')
   CANDIDATE_ROOT=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ021-F03-K-caller-materialized-review/F03-next-capture-and-projection/logged-input-chain-candidate-04/afterimages')
   sys.path.insert(0,str(W))
   sys.path.insert(0,str(CANDIDATE_ROOT))
   spec=importlib.util.spec_from_file_location('f03_r328_actual_api',ASSEMBLER)
   a=importlib.util.module_from_spec(spec)
   spec.loader.exec_module(a)
   assert Path(a.decode_capture.__code__.co_filename).resolve()==CANDIDATE_ROOT/'models/sherlock/qa/k_capture_observations.py'
   ```

---

## 4. Technical Findings & Evidence

### 4.1 Diagnosis of Attempt 03 Non-Ready Return (`ALL94-READABLE-BYTE-SPAN-PROJECTION.json`, BODY 5 & `FIVE-COMPLETE-RAW-PAIR-REPRESENTATIVES.lossless.json`, BODY 6)
- **What Succeeded in Attempt 03**: Candidate 03 parsed all 95 post-startup configurations, the startup probe (`001`), and the terminal probe (`096`) without raising any exception (`startup` and `terminal` gaps were `[]`, `original_decoded_log_gaps_empty: true`, `execution_depth: [1]`, `warning_depth: [1]`, and every edge `0..93` matched an exact `inventory_span_row` `0..19` in `COMPILED-MAIN-RULE-SPANS-01.json`).
- **Why `comparison_ready` Was `False` on Edges `8..92`**:
  - In `input_kore` (`exec-{i+1}/state.kore`), `kore-exec`'s output serializer writes `SortInt` map-key injections on a single line:
    `            /* Inj: */ inj{SortInt{}, SortKItem{}}(\dv{SortInt{}}("0")),`
  - In `actual_logged_config` (`exec-{i+2}/stderr` `On configuration:`), `kore-exec`'s log serializer wraps that exact construct across 3 lines:
    ```kore
                          /* Inj: */ inj{SortInt{}, SortKItem{}}(
                              \dv{SortInt{}}("0")
                          ),
    ```
  - Across all 94 edges in `ALL94-READABLE-BYTE-SPAN-PROJECTION.json` (BODY 5), the exact partition of `(actual_minus_expected_bytes, actual_minus_expected_LF)` is:
    - **`(0, 0)` (9 edges: `0..7` and `93`)**: State contains only empty `.Map` (`Lbl'Stop'Map{}()`), so 0 map keys exist and `current_literal_byte_match` was already `true`.
    - **`(50, 2)` (30 edges: `8, 9, 12..16, 29, 30, 33..37, 50, 51, 54..58, 71, 72, 75..77, 80, 84, 88, 92`)**: 1 `SortInt` map key at 12-space indent (`+50B`, `+2 LF`).
    - **`(116, 4)` (10 edges: `10, 11, 31, 32, 52, 53, 73, 74, 78, 79`)**: 2 `SortInt` map keys—one at 12-space indent (`+50B`, `+2 LF`) and one inside a stack frame at 20-space indent (`+66B`, `+2 LF`), totaling `+116B`, `+4 LF`.
    - **`(174, 6)` (33 edges: `19..28, 40..49, 61..70, 81, 85, 89`)**: 3 `SortInt` map keys (`+174B`, `+6 LF`).
    - **`(240, 8)` (12 edges: `17, 18, 38, 39, 59, 60, 82, 83, 86, 87, 90, 91`)**: 4 `SortInt` map keys (`+240B`, `+8 LF`).
  - Inspecting all five complete raw pairs in BODY 6 (edges `0, 8, 10, 40, 91` spanning all five delta families), every difference between `' ' * 10 + line` and `actual_logged_config` is accounted for **solely** by `_logged_map_key_line(line)`.
  - Secondary `rule_start_lines-not-supplied` in Attempt 03 occurred only because `decode_capture` sets `rules[i] = None` whenever `edge` is non-empty (`rules.append(None if edge else step['rule']['start_line'])`), causing `all(rule is not None for rule in rules)` to evaluate to `False`.

### 4.2 Soundness, Anchoring, Bounds, and Refusal Behavior of Candidate 04 (BODY 1 & BODY 2)
- **Strict Full-Line Regex Anchoring**: `_INLINE_MAP_KEY_INJ.fullmatch(line)` matches **only** an entire line consisting of leading spaces `( *)`, literal `/* Inj: */ inj{SortInt{}, SortKItem{}}(\dv{SortInt{}}("(-?[0-9]+)")),`.
  - Sorts other than `SortInt{}` -> `SortKItem{}` (e.g. `SortUIUInt{}`, `SortUIListValue{}`, `SortUIControl{}`), comments other than `/* Inj: */ `, non-decimal domain values, or lines with trailing tokens cannot match.
  - Indentation is strictly preserved: `indent = ' ' * (10 + len(match[1]))` for the outer lines and `f'{indent}    \\dv{{SortInt{{}}}}("{match[2]}")\n'` (`+4` spaces) for the inner line.
- **All-Inline vs. All-Wrapped Membership (`expected` Set of Size $\le 2$)**:
  - `_logged_input_gaps` does **not** perform whitespace stripping or per-key optional matching. It builds exactly two complete strings: `inline_term` (every key inline) and `wrapped_term` (every `_INLINE_MAP_KEY_INJ` line wrapped into 3 lines).
  - Any partially wrapped mixture (some `SortInt` keys wrapped and some inline within the same step), any arbitrary single-line vs. 3-line formatting on non-map-key lines, or any modification to comments, literals, sorts, constructors, or indentation is **refused** (`entry['configuration_text'] not in expected`), preserving the canonical `'logged-configuration-is-not-input-kore'` gap and `comparison_ready=False`.
- **Input Limits, Memory Bounds, and Raw Preservation**:
  - `if entry is None or len(input_kore) > MAX_KORE_BYTES or not input_kore.isascii(): return ['logged-configuration-is-not-input-kore']` runs before `.decode('ascii').split('\n')`. With `MAX_KORE_BYTES = 1024 * 1024` (max observed F03 `input_kore` is `27150B` on edge `74`), memory overhead is bounded to a few MiB within the `1GiB` AS cap.
  - No rule-line inference or bypass was added: `_step` and `decode_capture` are untouched, and `raw_captures`, `decoded_incoming_logs`, `edge_links`, and `ADAPTER_SOURCES` remain intact.

### 4.3 Disposition of Owner Narrative Inaccuracies (`OWNER-NARRATIVE-QUALIFICATIONS.json`, BODY 7)
- The owner's narrative prose (`3c0c1550...`) is **not** relied upon and carries zero audit authority. Specifically:
  1. Supplied complete raw pair representatives in BODY 6 are the **5** edges **`0, 8, 10, 40, 91`** (not `0, 8, 22, 40, 41, 91`).
  2. Edge `10` (`exec-011 -> exec-012`) has delta **`+116B / +4 LF`** (not `+50B / +2 LF`), as shown in BODY 5 and BODY 6.
  3. The actual candidate path is `.../logged-input-chain-candidate-04/afterimages/models/sherlock/qa/k_capture_observations.py` (`42838B`), using actual source symbols `MAX_CAPTURES`, `F03_MAX_CAPTURES`, `MAX_NESTING`, `MAX_KORE_BYTES`, and `entry['configuration_text']`.
- These narrative errors in the author's report do not affect the executable Python code in BODY 1 (`SOURCE.diff` in BODY 2 is clean, minimal, and exact).

### 4.4 Findings Summary
- **Mandatory / Blocking Findings**: **None (`0`)**.
- **Non-Blocking Findings**:
  1. **`F03-DEC-CAND04-NB1` (`LOW_NONBLOCKING` — Dual String Construction in `_logged_input_gaps`)**: `_logged_input_gaps` constructs both `inline_term` and `wrapped_term` strings on each step. At max `27,150B` (`<= 1 MiB` bound) across 94 steps + terminal probe, total runtime and memory cost is negligible.
  2. **`F03-DEC-CAND03-NB1..2` & `F03-ASM-CAND02-NB1` (`LOW_NONBLOCKING` — Carried Over from Turns 1–2)**: Unchanged `_single_k_item` double-parse on 15 multiline `<k>` heads, historical N01 docstring/evidence note in `k_capture_observations.py`, and cosmetic `"newline-terminated"` error string in `assemble_k_f03_observations.py`.

---

## 5. Weighing Direct Guard-Unit Dependency vs. Narrow Actual Decode & General Adoption

- **Absence of New Offline Guard Units in This Packet**: Focused offline guard unit tests for `_logged_map_key_line` / `_logged_input_gaps` have been separately requested from a fresh designated author and are **not** present or executed here.
- **Why Narrow Actual Decode (`failedDecodeOnlyAndKArtifacts`) Is Eligible Now**:
  1. Operation 1 executes against a **fixed, hash-pinned 517-file input readset** (`FLAT-INPUT-PINS.json`, SHA256 `4da35ee1...`) whose 94 edges and 5 representative full raw pairs have been directly inspected in BODY 5 and BODY 6.
  2. `_logged_map_key_line` is a pure 11-line string transformation gated by `_INLINE_MAP_KEY_INJ.fullmatch` and `MAX_KORE_BYTES`, with no I/O, no subprocesses, and no relaxation of receipt, path-chain, or rule-span checks.
- **Why General Tool Adoption Remains Blocked on Offline Guard Units**:
  - Before `assemble_k_f03_observations.py` or `k_capture_observations.py` can be installed or adopted into the general QA toolchain, **all** of the following offline guard suites must be authored and observed passing:
    1. All six R328 independent offline assembler guard categories,
    2. Multiline `<k>` single-item vs. multiple-item / non-`KItem` root / `> MAX_TERM_BYTES` decoder guard tests, and
    3. Direct `_logged_map_key_line` / `_logged_input_gaps` guard unit tests (covering exact all-inline match, exact all-wrapped match, refusal of partially wrapped mixed keys, refusal of non-`SortInt` or malformed `/* Inj: */` lines, refusal of indentation/literal tamper, and `> MAX_KORE_BYTES` refusal).

---

## 6. Verdicts & Separate Three-Operation Eligibility

### 6.1 Source Verdict
- **Verdict**: **`APPROVE_WITH_LIMITS`**
- **Approved Candidate 04 Files**:
  - `.../logged-input-chain-candidate-04/afterimages/models/sherlock/qa/k_capture_observations.py` (`42838B`, exact 2-hunk diff in BODY 2 over Candidate 03 `70db66d9...`)
  - `.../logged-input-chain-candidate-04/afterimages/models/sherlock/qa/assemble_k_f03_observations.py` (`63706B`, SHA256 `83a6424f9896345fbbd22e51842af2561eb9aba17de7be6c34bb54507a9510eb`, unchanged copy from Turn 1)
  - `.../logged-input-chain-candidate-04/afterimages/models/sherlock/qa/k_observations.py` (`7686B`, SHA256 `540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07`, unchanged copy from Turn 2)
  - `.../logged-input-chain-candidate-04/afterimages/models/sherlock/qa/k_terms.py` (`5666B`, SHA256 `c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca`, unchanged copy from Turn 2)
  - `.../logged-input-chain-candidate-04/test_actual_f03_decode_only.candidate04.py` (`6248B`, exact 1-hunk path/root diff in BODY 4 over Candidate 03 consumer)
- **Limits**:
  1. Scoped strictly to passive execution of the three operations in `THREE-OPERATIONS-EXACT.json` (BODY 8) on the pinned F03 capture.
  2. Zero general installation or QA adoption authority; all offline guard suites listed in Section 5 remain mandatory before general adoption.

### 6.2 Separate Eligibility for the Exact Three Passive Operations (BODY 8)

| Operation ID | Scope & Resource Envelope | External / Read Coverage & Dependencies | Eligibility Verdict |
| :--- | :--- | :--- | :--- |
| **1. `failedDecodeOnlyAndKArtifacts`** | Runs `test_actual_f03_decode_only.candidate04.py` (1 selected test: `test_actual_decoding_has_complete_eleven_field_94_record_contract` + 3 exclusive `open('x')` writes to `passive-outputs-01/` only if test passes; retains prior Attempt 02 byte-preservation pass without replay). Wall `30s` (`KILL`), CPU `25s`, AS `1GiB`, FSIZE `64MiB`, CORE `0`, `python3 -I -B`, minimal env, `stdin=DEVNULL`, stream cap `131072B`, `serialFailStopNoRetry=true` | All 4 candidate modules and consumer read in full across Turns 1–3; all 96 print configs (Turn 2), all 94 edge byte/span rows (BODY 5), and 5 full raw `(input_kore, actual_logged_config)` pairs (BODY 6) audited inline; remaining 89 raw pairs and 2 SHA256 columns externally pinned in `FLAT-INPUT-PINS.json` (`4da35ee1...`) | **`ELIGIBLE`** |
| **2. `projectActualQuintITF`** | Runs unchanged `quint_observations.py` on `quint-single-run-manifest.proposed.json`, writing fresh `passive-outputs-01/quint-observations.json` (`64MiB` cap, no truncation, fail-stop on cap/nonzero) and stderr (`131072B` cap). Wall `10s` (`KILL`), CPU `25s`, AS `1GiB`, FSIZE `64MiB`, CORE `0`, `python3 -I -B`, minimal env, `stdin=DEVNULL`, `serialFailStopNoRetry=true` | `quint_observations.py` (`16774B`) and `quint-single-run-manifest.proposed.json` are `SOURCE_AVAILABLE_NOT_FRESHLY_READ` inline (pinned in `FLAT-INPUT-PINS.json`); still unrun because Attempt 03 fail-stopped at Operation 1 | **`ELIGIBLE` (Subject to Post-Run Output & Result Review)** |
| **3. `compareActualPairAndIndependentOracle`** | Runs unchanged `check.py --repo ... --paired-k-observations .../k-observations.json --paired-quint-observations .../quint-observations.json --selected-case F03 --selected-initial-fuel 93`, writing fresh `passive-outputs-01/paired-projection-report.json` (`64MiB` cap, no truncation, fail-stop on cap/nonzero) and stderr (`131072B` cap). Wall `10s` (`KILL`), CPU `25s`, AS `1GiB`, FSIZE `64MiB`, CORE `0`, `python3 -I -B`, minimal env, `stdin=DEVNULL`, `serialFailStopNoRetry=true` | `check.py` (`81758B`) is `SOURCE_AVAILABLE_NOT_FRESHLY_READ` inline (pinned in `FLAT-INPUT-PINS.json`); consumes Phase 1 (`k-observations.json`) and Phase 2 (`quint-observations.json`) generated outputs under `serialFailStopNoRetry=true` | **`ELIGIBLE` (Subject to Post-Run Whole-Clause Result Review)** |

### 6.3 Retained Boundaries & Non-Claims
- **Mandatory Guard Units Retained**: All six R328 assembler guard categories, multiline single-item `<k>` head guards, and new inline/wrapped map-key and representation-tamper guards remain **mandatory** before general assembler/decoder adoption.
- **Unauthenticated Receipts & No Predictive Pass**: Receipt/hash/path checks verify internal consistency of supplied bytes only (`producer_authenticity='not-established'`). No predictive pass claim is made prior to running the three operations.
- **Original UIREQ-021 Closure Pending**: UIREQ-021 closure remains open pending actual materialization of all five output artifacts (`k-observations.json`, `k-decoded-run.json`, `k-assembled-provenance.json`, `quint-observations.json`, `paired-projection-report.json`) with verified hash joins and a fresh independent whole-clause result review against `UIREQ021-ORIGINAL-CLAUSE.md`.
- **Gate & Authority Preservation**: Zero formal semantic refinement, theorem, native proof, target, or global H1 claim is made. `UIREQ-021`, `C4 OPEN`, `R2 UNSAT`, `60460 HOLD`, and stopped `Amendment 10` remain unchanged.

