# Fresh Independent Non-Author Security Review: F03 Complementary Offline Guard Tests & Prospective Operation

- **Reviewer Role**: Fresh independent non-author security reviewer (human-readable runtime model identity: **Gemini 4 Argon (High)**, standing Opus outage route; did not author the four production modules or `test_f03_general_adoption_guards.py`).
- **Tools & Readset Compliance**: All 21 supplied lossless pages (`01-production-assembler.txt` through `21-RETAINED-PASSING-SCOPE.txt`) received a full `DONE` read via `view_file({"AbsolutePath": ...})` only. Zero terminal commands, zero Python/test/model executions, zero filesystem writes, zero network calls, and zero reads outside the 21 listed page files were performed.
- **Honest Readset & Omission Disclosure**:
  - **Inspected in full (100% of bytes)**:
    - `assemble_k_f03_observations.py` (`63,706 B`, SHA-256 `83a6424f9896345fbbd22e51842af2561eb9aba17de7be6c34bb54507a9510eb`, pages `01`–`05`)
    - `k_capture_observations.py` (`42,838 B`, SHA-256 `85e4e79a64752ee2b6b928e7dd1f749d0a9cedfacfbac0307f63d63107b62a1a`, pages `06`–`08`)
    - `k_observations.py` (`7,686 B`, SHA-256 `540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07`, page `09`)
    - `k_terms.py` (`5,666 B`, SHA-256 `c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca`, page `10`)
    - `test_f03_general_adoption_guards.py` (`45,058 B`, SHA-256 `eb2a7804092475fdfa7075e9c8f7276c443704c8ee921818c8bf555f82746ea9`, pages `11`–`14`)
    - Requirement/scope/fixture/operation metadata (`15`–`21`).
  - **Not in reviewer readset**: The raw on-disk capture payloads (`251-f03-paired-model-operation-proposal/actual/k/**` logs, `.kore` files, `kprint` stdout files, full `receipt.json` beyond the selected stage receipts in `18-FIXTURE-BINDINGS.txt`, `K-COMMANDS.json`, `COMPILED-MAIN-RULE-SPANS-01.json`, `main.k`, and external pin manifests `FLAT-INPUT-PINS.json` / `PROPOSED-OFFLINE-FIXTURE-READ-PINS.json`) and independent oracle files (`fixture-normal-programs.json`, `fixture-normal-prefixes.json`, which must never be read). This omission does **not** prevent a definitive static source and prospective-operation review because the complete producer/consumer source modules, pinned constants/hashes, selected stage receipts (`kast`, `exec-000`, `exec-001`, `exec-002`, `exec-009`, `exec-010`, `exec-095`, `exec-096`, `print-000`, `print-001`, `print-002`, `print-096`), `INITIALIZER-RECIPE.json`, and prior `passive_three` / `direct_seven` receipts are supplied in pages `01`–`21`.

---

## 1. Exact Actual Helper Signatures & Per-Category Concrete Assertions/Mutations (Pre-Verdict)

### 1.1 Exact Actual Production Signatures (`pages/01`–`10`)

- **Assembler (`assemble_k_f03_observations.py`, `pages/01`–`05`)**:
  - `def _require_bytes(value: object, label: str, *, allow_empty: bool = False, max_bytes: int | None = None) -> bytes:` (`01:L320-L326`)
  - `def _require_ascii_text(value: object, label: str, *, max_bytes: int, require_printable: bool = False) -> tuple[str, bytes]:` (`01:L349-L355`)
  - `def _reject_prospective_or_oracle_dict(mapping: object, label: str) -> dict:` (`01:L368`)
  - `def _engine_options(argv: list[str]) -> dict[str, str | bool] | None:` (`02:L8`)
  - `def _engine_command_template(argv: list[str]) -> tuple[str, ...]:` (`02:L33`)
  - `def _validate_receipt_shape(receipt: object, expected_keys: frozenset[str], label: str, *, expected_argv: list[str] | None = None, require_zero_exit: bool = True) -> dict:` (`02:L41-L48`)
  - `def validate_source_k_bytes(source_k_bytes: bytes | str, *, source_path: str = F03_COMPILED_MAIN_K_PATH) -> dict:` (`02:L79-L83`)
  - `def validate_f03_run_input(run_input_bytes: bytes | str) -> dict:` (`02:L105`)
  - `def _validate_rule_span_rows(rules: object, *, source_path: str) -> list[dict]:` (`02:L137`)
  - `def load_rule_span_inventory(raw_inventory: bytes | str | dict, *, source_path: str = F03_COMPILED_MAIN_K_PATH, require_pinned_sha256: bool = True) -> tuple[list[dict], dict]:` (`02:L181-L186`)
  - `def load_prospective_k_commands(k_commands_bytes: bytes | str, *, require_pinned_sha256: bool = True) -> dict[str, list[str]]:` (`02:L286-L290`)
  - `def _validate_kast_stdout_kore(value: object) -> tuple[str, bytes]:` (`02:L352`)
  - `def _validate_exec_stage(stage_id: str, stage: object, *, expected_depth: str, expected_argv: list[str] | None = None, strict: bool = True) -> dict:` (`03:L66-L73`)
  - `def _validate_print_stage(stage_id: str, stage: object, *, expected_argv: list[str] | None = None, strict: bool = True) -> dict:` (`03:L146-L152`)
  - `def _validate_prestartup_stages(stage_artifacts: dict, *, expected_argv_by_id: dict[str, list[str]] | None = None, exec_001_receipt: dict | None = None, print_001_receipt: dict | None = None) -> dict:` (`03:L193-L199`)
  - `def assemble_f03_capture_payload(stage_artifacts: dict, *, run_input_bytes: bytes | str, rule_span_inventory_bytes: bytes | str | dict, source_path: str = F03_COMPILED_MAIN_K_PATH, require_pinned_inventory_sha256: bool = True, k_commands_bytes: bytes | str | None = None, strict_stage_validation: bool = False) -> dict:` (`04:L77-L86`)
  - `def decode_f03_observations(stage_artifacts: dict, *, run_input_bytes: bytes | str, rule_span_inventory_bytes: bytes | str | dict, source_k_bytes: bytes | str, source_path: str = F03_COMPILED_MAIN_K_PATH, require_pinned_inventory_sha256: bool = True, k_commands_bytes: bytes | str | None = None, strict_stage_validation: bool = False) -> dict:` (`04:L190-L200`)
  - `def _verify_safe_subdir(base_dir: Path, resolved_root: Path, subdir_name: str) -> Path:` (`04:L274`)
  - `def _read_checked_file(path: Path, label: str, *, max_bytes: int = MAX_LOG_BYTES, allow_empty: bool = True, resolved_root: Path | None = None) -> bytes:` (`04:L299-L306`)
  - `def load_stage_artifacts_from_actual_dir(actual_k_dir: str | Path, *, receipts_by_stage_id: dict[str, dict]) -> dict:` (`04:L344-L348`, `05:L1`)

- **Decoder (`k_capture_observations.py`, `pages/06`–`08`)**:
  - `def _single_k_item(head):` (`06:L171`)
  - `def _configuration(text):` (`06:L182`)
  - `def _decode(text, entry_k):` (`06:L210`)
  - `def _engine_options(argv):` (`06:L236`)
  - `def _engine_gaps(receipt, *, raw_log, state_kore):` (`06:L261`)
  - `def _print_gaps(receipt, *, configuration, engine):` (`06:L288`)
  - `def _logged_map_key_line(line):` (`07:L205`)
  - `def _logged_input_gaps(entry, input_kore):` (`07:L218`)
  - `def _engine_template(argv):` (`07:L254`)
  - `def _chain_gaps(engines, printers):` (`07:L259`)
  - `def retained_run_input(text):` (`07:L282`)
  - `def decode_terminal_probe(probe, *, terminal_configuration, terminal_state_kore, entry_k, source_path):` (`08:L13-L14`)
  - `def decode_startup_probe(probe, *, post_startup_configuration, entry_k, source_path, rule_spans=None):` (`08:L50-L51`)
  - `def decode_capture(case_id, entry_k, captures, *, source_path, rule_spans=None, terminal_probe=None, startup_probe=None, run_input=None, max_captures=MAX_CAPTURES):` (`08:L83-L85`)
  - `def observation_bundle(decoded_runs, *, source_sha256):` (`08:L178`)

- **Observations & Terms (`k_observations.py` `page/09`, `k_terms.py` `page/10`)**:
  - `def decode_configuration(raw_text, *, entry_k):` (`09:L68`)
  - `def decode_run(case_id, entry_k, configurations, *, rule_start_lines=None, terminal_has_next=None):` (`09:L89-L90`)
  - `def parse_term(text: str):` (`10:L114`)
  - `def render_term(node) -> str:` (`10:L123`)
  - `def normalize_term(text: str) -> str:` (`10:L134`)

- **New Test Module Helpers (`test_f03_general_adoption_guards.py`, `page/11`)**:
  - `def _sha256(data: bytes) -> str:` (`11:L68`)
  - `def _load_candidate_modules():` (`11:L72`)
  - `def _clone_stages(stages: dict) -> dict:` (`11:L103`)
  - `def _clone_assembled(assembled: dict) -> dict:` (`11:L120`)
  - `def _run_decode_capture(self, assembled: dict) -> dict:` (`13:L201`)

### 1.2 Concrete Assertion/Mutation Quotes Across All Six R328 Categories, Multiline, and Full Chain

1. **R328 Category 1 (Positive 195-child + initializer JVM unpinned/pinned + direct `kast`, `11:L233-L273`)**:
   ```python
   direct_stages = _clone_stages(self.stages)
   kast_flag_idx = kast_argv.index('-kast')
   direct_stages['kast']['receipt']['argv'] = ['/usr/bin/kast', *kast_argv[kast_flag_idx + 1 :]]
   ...
   for res in (unpinned, pinned, direct):
       self.assertTrue(res['bundle']['comparison_ready'])
       self.assertEqual(len(res['decoded_run']['raw_captures']), 95)
       run = res['bundle']['observations']['runs'][0]
       self.assertEqual(len(run['records']), 94)
       self.assertEqual([r['charge'] for r in run['records']], [1] * 93 + [0])
       self.assertIs(run['terminal_has_next'], False)
   ```
2. **R328 Category 2 (Negative `kast.receipt.argv` refusals, `11:L320-L329`)**:
   ```python
   check_bad_argv(
       orig_argv[:main_idx] + ['-server'] + orig_argv[main_idx:],
       'kast.receipt.argv java launcher has unadmitted JVM token',
   )
   bad_cp_flag = list(orig_argv)
   bad_cp_flag[cp_idx + 1] = '-Xss8m'
   check_bad_argv(
       bad_cp_flag,
       'kast.receipt.argv java launcher has invalid -cp/-classpath argument',
   )
   ```
3. **R328 Category 3 (Negative `_validate_kast_stdout_kore` single-term refusals, `12:L37-L44`, `12:L86-L98`)**:
   ```python
   (
       b'LbluiRun{}() Lblother{}()\n',
       r'kast\.stdout must be a single KORE term \(root application closed before end of line\)',
   ),
   (
       b'LbluiRun{"str_before_lparen"}()\n',
       'kast.stdout string literal outside root KORE application',
   ),
   ```
4. **R328 Category 4 (Negative pre-startup chain & template refusals, `12:L149-L160`)**:
   ```python
   check_prestartup(
       mutate_flag('exec-001', '--pattern', '/tmp/wrong-exec000-output.kore'),
       'pre-startup path chain mismatch: exec-001 --pattern does not match exec-000 --output path',
   )
   ...
   check_prestartup(
       lambda st: st['exec-000']['receipt']['argv'].extend(['--smt', 'none']),
       'pre-startup command template mismatch between exec-000 and exec-001',
   )
   ```
   *(See **Defect 1** below: the second mutation above hits `_engine_options` duplicate-flag rejection instead of line 24 of `04-production-assembler.txt`.)*
5. **R328 Category 5 (Post-startup `strict_stage_validation=False` canonical gaps vs `True` `ValueError`, `12:L240-L252`)**:
   ```python
   over_kore = b'a' * (self.dec.MAX_KORE_BYTES + 1)
   over_kore_sha = _sha256(over_kore)
   run_both(
       lambda st: (
           st['exec-002'].__setitem__('state_kore', over_kore),
           st['exec-002']['receipt'].__setitem__('output_sha256', over_kore_sha),
       ),
       r'exec-002\.state_kore exceeds bound',
       (
           'edge-0-state-kore-exceeds-retention-bound',
           'edge-1-logged-configuration-is-not-input-kore',
       ),
   )
   ```
6. **R328 Category 6 (On-disk 488-file `lstat`/symlink/pre-read size/oracle/span guards, `13:L13-L20`, `13:L43-L57`)**:
   ```python
   os.truncate(base_dir / 'kast/stdout', self.asm.F03_KAST_STDOUT_MAX_BYTES + 1)
   try:
       with self.assertRaisesRegex(ValueError, r'kast\.stdout file size .* exceeds bound'):
           self.asm.load_stage_artifacts_from_actual_dir(
               base_dir, receipts_by_stage_id=min_receipts
           )
   finally:
       (base_dir / 'kast/stdout').write_bytes(min_kast_out)
   ```
7. **Multiline `<k>` Head Guards (`13:L152-L160`, `13:L177-L195`)**:
   ```python
   reduced, note = self.dec._configuration(multiline_cfg)
   self.assertIsNotNone(note)
   self.assertNotIn('~>', reduced)
   decoded = self.dec._decode(multiline_cfg, self.asm.F03_ENTRY_K)
   self.assertEqual(decoded['original_configuration'], multiline_cfg)
   ...
   overbound_head = 'uiEval(\n' + (' ' * (self.dec.MAX_TERM_BYTES + 10)) + 'uiVar(0)\n)'
   ```
   *(See **Defect 3** below regarding `self.assertNotIn('~>', base_cfg)` at `13:L138`.)*
8. **Whole `decode_capture` Chain Propagation & Tampering Guards (`13:L274-L287`, `14:L5-L26`)**:
   ```python
   tampered_pred_asm = _clone_assembled(self.assembled_control)
   bad_pred_kore = tampered_pred_asm['captures'][8]['state_kore'].replace(
       b'\\dv{SortInt{}}("85")', b'\\dv{SortInt{}}("84")', 1
   )
   ...
   res_pred = self._run_decode_capture(tampered_pred_asm)
   self.assertFalse(res_pred['comparison_ready'])
   self.assertNotIn('edge-7-engine-receipt-output-kore-sha256-mismatch', res_pred['gaps'])
   self.assertIn('edge-8-logged-configuration-is-not-input-kore', res_pred['gaps'])
   ```
   *(See **Defect 2** below regarding `e5_argv.extend(['--smt', 'none'])` at `14:L14`.)*

---

## 2. Technical Audit of Module Loading, Fixtures, Mocks, and Concrete Defects

### 2.1 Verified Sound Design Elements
1. **Exact Four-Module Loading Without Old Worktree Imports (`11:L39-L100`, `11:L195-L206`)**:
   - `_load_candidate_modules()` verifies the byte length and SHA-256 of all four production files in `CANDIDATE_QA_DIR` against `PRODUCTION_PINS` before loading.
   - `k_capture_observations.py` loads `k_observations.py` and `k_terms.py` via `Path(__file__).with_name(...)` (`06:L36-L44`), and `k_observations.py` loads `k_terms.py` via `Path(__file__).with_name('k_terms.py')` (`09:L25-L28`).
   - `_load_candidate_modules()` binds `sys.modules['k_capture_observations']` and `sys.modules['models.sherlock.qa.k_capture_observations']` to `dec_mod` inside a `try...finally` block while executing `assemble_k_f03_observations.py`, restoring `sys.modules` immediately afterward. No `sys.path` mutation or old worktree import occurs.
2. **Shared Fixture Isolation & Raw-Byte Retention (`11:L103-L189`)**:
   - `setUpClass` loads the 488 on-disk stage files and `receipt.json` once into `_F03GuardTestBase`.
   - `_clone_stages` and `_clone_assembled` copy all mutable dicts and `argv` lists before each negative test mutation; immutable `bytes`/`str`/`int` values are shared safely without cross-test leakage.
   - Raw byte retention (`raw_captures`, `original_configuration`, `state_kore_b64`) is preserved and asserted.
3. **Scoped `mock.patch.object` for `FORBIDDEN_ORACLE_SHA256S` (`13:L43-L58`)**:
   - Because reading actual independent oracle files (`fixture-normal-programs.json`, `fixture-normal-prefixes.json`) is prohibited and SHA-256 is preimage-resistant, `test_cat6` uses `with mock.patch.object(self.asm, 'FORBIDDEN_ORACLE_SHA256S', self.asm.FORBIDDEN_ORACLE_SHA256S | {_sha256(min_kore)}):` to test the exact `_sha256_bytes(raw) in FORBIDDEN_ORACLE_SHA256S` branches in `_read_checked_file` (`04:L339-L340`) and `_require_bytes` (`01:L344-L345`) without bypassing production functions or reading forbidden files.

### 2.2 Concrete Test-Source Defects Requiring Minimal Revision

#### Defect 1 (High — Wrong Earlier Exception in Category 4 `exec-000`/`exec-001` Template Mismatch)
- **Location**: `test_cat4_negative_prestartup_chain_and_template_refusals` (`12-NEW-TEST-SOURCE.txt` lines 157–160).
- **Root Cause**: The test mutates `exec-000`'s `argv` with:
  ```python
  lambda st: st['exec-000']['receipt']['argv'].extend(['--smt', 'none']),
  ```
  However, as shown in `18-FIXTURE-BINDINGS.txt` (`L57-L58`), `exec-000`'s `argv` **already contains** `"--smt", "none"`. Extending `argv` with `['--smt', 'none']` creates a duplicate `'--smt'` flag. In `assemble_k_f03_observations._engine_options` (`02:L16-L17`), `if flag in options: return None` rejects duplicate flags and returns `None`. Consequently, `_validate_prestartup_stages` (`03:L346-L348`) raises:
  `ValueError: K F03 offline assembly error: exec-000.receipt.argv is not an admitted kore-exec option set`
  and **never reaches** the target guard `_fail('pre-startup command template mismatch between exec-000 and exec-001')` (`04:L21-L24`). `self.assertRaisesRegex` will fail at runtime.
- **Minimal Fix**: Mutate the existing `--smt` value on `exec-000` (`mutate_flag('exec-000', '--smt', 'z3')`), which keeps `_engine_options(exec_000['receipt']['argv'])` non-`None`, satisfies `_ENGINE_REQUIRED`, and reaches `04:L24`.

#### Defect 2 (High — Bypassed `_chain_gaps` Checks in Full-Chain Path/Template Tampering Test)
- **Location**: `test_full_chain_engine_and_printer_path_and_template_tampering_refusals` (`14-NEW-TEST-SOURCE.txt` line 14).
- **Root Cause**: The test mutates `e5_argv` (`asm_chain['captures'][6]['engine_receipt']['argv']`, i.e., `edge-5` / `exec-007`) with:
  ```python
  e5_argv.extend(['--smt', 'none'])
  ```
  Because every F03 `exec-*` receipt already includes `"--smt", "none"`, appending `['--smt', 'none']` duplicates `--smt`, causing `k_capture_observations._engine_options(e5_argv)` (`06:L243-L244`) to return `None`. In `_chain_gaps` (`07:L263-L271`):
  ```python
  options = _engine_options(engine['argv']) if engine is not None else None
  if options is None:
      previous = None
      continue
  ```
  When `options is None` for `edge-5`, `_chain_gaps` executes `continue` **before** `outputs.append(options.get('--output'))` (`07:L269`) and **before** `templates.add(_engine_template(engine['argv']))` (`07:L270`). As a result, neither `'chain-engine-output-paths-not-distinct'` nor `'chain-engine-command-template-differs'` is appended to `res['gaps']`, and both `self.assertIn` checks at `14:L22-L26` will fail at runtime.
- **Minimal Fix**: Replace `e5_argv.extend(['--smt', 'none'])` with `e5_argv[e5_argv.index('--smt') + 1] = 'z3'`, which keeps `_engine_options(e5_argv)` valid so `_chain_gaps` records both `'chain-engine-output-paths-not-distinct'` and `'chain-engine-command-template-differs'`.

#### Defect 3 (High — False Fixture Assumption on `print-001/stdout` in Multiline `<k>` Positive Test)
- **Location**: `test_multiline_single_k_item_positive_normalization_and_retention` (`13-NEW-TEST-SOURCE.txt` lines 137–151).
- **Root Cause**: Line 138 asserts `self.assertNotIn('~>', base_cfg)` on `base_cfg = self.stages['print-001']['stdout'].decode('ascii')`. In K 7.1.337 `kprint`, non-empty `<k>` cells (`kseq(X, dotk())`) are pretty-printed with `' ~> .K'` (`06:L17-L18`, `06:L195-L207`). The test immediately replaces the `<k>...</k>` interior via `base_cfg[:k_start] + '\n    ' + multiline_head + ' ~> .K\n  ' + base_cfg[k_end:]`, so only `base_cfg[:k_start] + base_cfg[k_end:]` (the configuration outside `<k>...</k>`) must be free of `'~>'` to guarantee `multiline_cfg.count('~>') == 1` and `self.assertNotIn('~>', reduced)`.
- **Minimal Fix**: Move the assertion below `k_start`/`k_end` and assert `self.assertNotIn('~>', base_cfg[:k_start] + base_cfg[k_end:])`.

---

## 3. Minimal Uniquely-Anchored Literal `OLD` / `NEW` Test Source Fixes

Applying the three minimal edits below to `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/F03-general-offline-guard-tests-owner-materialization-20261018/afterimages/test_f03_general_adoption_guards.py` resolves all three test-source defects without altering any production module or weakening any guard:

### Fix 1 (`12-NEW-TEST-SOURCE.txt` lines 157–160)
- **OLD**:
```python
        check_prestartup(
            lambda st: st['exec-000']['receipt']['argv'].extend(['--smt', 'none']),
            'pre-startup command template mismatch between exec-000 and exec-001',
        )
```
- **NEW**:
```python
        check_prestartup(
            mutate_flag('exec-000', '--smt', 'z3'),
            'pre-startup command template mismatch between exec-000 and exec-001',
        )
```

### Fix 2 (`13-NEW-TEST-SOURCE.txt` lines 136–151)
- **OLD**:
```python
    def test_multiline_single_k_item_positive_normalization_and_retention(self):
        base_cfg = self.stages['print-001']['stdout'].decode('ascii')
        self.assertNotIn('~>', base_cfg)
        multiline_head = (
            'uiEval(\n'
            '    uiCall(\n'
            '      3,\n'
            '      uiExpr(uiLiteral(uiCons(uiU(1), uiCons(uiU(2), uiCons(uiU(3), uiNil())))), uiNoExprs())\n'
            '    )\n'
            '  )'
        )
        k_start = base_cfg.index('<k>') + 3
        k_end = base_cfg.index('</k>')
        multiline_cfg = (
            base_cfg[:k_start] + '\n    ' + multiline_head + ' ~> .K\n  ' + base_cfg[k_end:]
        )
```
- **NEW**:
```python
    def test_multiline_single_k_item_positive_normalization_and_retention(self):
        base_cfg = self.stages['print-001']['stdout'].decode('ascii')
        multiline_head = (
            'uiEval(\n'
            '    uiCall(\n'
            '      3,\n'
            '      uiExpr(uiLiteral(uiCons(uiU(1), uiCons(uiU(2), uiCons(uiU(3), uiNil())))), uiNoExprs())\n'
            '    )\n'
            '  )'
        )
        k_start = base_cfg.index('<k>') + 3
        k_end = base_cfg.index('</k>')
        self.assertNotIn('~>', base_cfg[:k_start] + base_cfg[k_end:])
        multiline_cfg = (
            base_cfg[:k_start] + '\n    ' + multiline_head + ' ~> .K\n  ' + base_cfg[k_end:]
        )
```

### Fix 3 (`14-NEW-TEST-SOURCE.txt` lines 12–16)
- **OLD**:
```python
        e5_argv[e5_argv.index('--output') + 1] = e4_argv[e4_argv.index('--output') + 1]
        asm_chain['captures'][6]['print_receipt']['argv'][2] = e5_argv[e5_argv.index('--output') + 1]
        e5_argv.extend(['--smt', 'none'])
        asm_chain['captures'][6]['print_receipt']['argv'][0] = '/usr/local/bin/kprint-custom'
```
- **NEW**:
```python
        e5_argv[e5_argv.index('--output') + 1] = e4_argv[e4_argv.index('--output') + 1]
        asm_chain['captures'][6]['print_receipt']['argv'][2] = e5_argv[e5_argv.index('--output') + 1]
        e5_argv[e5_argv.index('--smt') + 1] = 'z3'
        asm_chain['captures'][6]['print_receipt']['argv'][0] = '/usr/local/bin/kprint-custom'
```

---

## 4. Independent Assessment of Prospective Offline Operation (`pages/17`, `20`, `21`)

### 4.1 Discovery Failure of Author's Literal `argv` Under `python3 -I -m unittest`
- The author's proposed `literal_author_argv` (`20-EXACT-OPERATION-PROPOSAL.txt` `L3-L20`) is:
  ```json
  ["/usr/bin/timeout", "--signal=TERM", "--kill-after=2s", "20s", "/usr/bin/prlimit", "--cpu=25", "--as=1073741824", "--fsize=67108864", "--core=0", "python3", "-I", "-B", "-m", "unittest", "-v", "test_f03_general_adoption_guards"]
  ```
  with `proposed_cwd` set to `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/F03-general-offline-guard-tests-owner-materialization-20261018/afterimages`.
- **Why this fails**: Python's `-I` (isolated mode) sets `sys.flags.isolated = 1` and `sys.flags.safe_path = True`, which **omits the current working directory (`''`) from `sys.path`** when `-m` is used. `unittest`'s named-module loader (`TestLoader.loadTestsFromNames(['test_f03_general_adoption_guards'])`) calls `__import__('test_f03_general_adoption_guards')`, which searches only stdlib `sys.path` entries and fails immediately with `ModuleNotFoundError: No module named 'test_f03_general_adoption_guards'`.
- **Exact Minimal Compatible Corrected Invocation**:
  Because `test_f03_general_adoption_guards.py` already ends with `if __name__ == '__main__': unittest.main()` (`14:L29-L30`) and loads all four production modules by absolute path without relying on `sys.path`, invoking the materialized script file directly by absolute path under `/usr/bin/python3 -I -B` (matching the proven `direct_seven` pattern in `21-RETAINED-PASSING-SCOPE.txt` `L26-L40`) runs **only the 11 methods in `__main__`** without putting `cwd` on `sys.path` and without scanning any directory:
  ```json
  [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=2s",
    "20s",
    "/usr/bin/prlimit",
    "--cpu=25:25",
    "--as=1073741824:1073741824",
    "--fsize=67108864:67108864",
    "--core=0:0",
    "--",
    "/usr/bin/python3",
    "-I",
    "-B",
    "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/F03-general-offline-guard-tests-owner-materialization-20261018/afterimages/test_f03_general_adoption_guards.py",
    "-v"
  ]
  ```

### 4.2 Resource Bounds, Temporary Disk, and Controller Stream Quotas
- **Per-process / per-file `prlimit` bounds**:
  - `wall=20s` (`SIGTERM` + `2s` `SIGKILL`), `CPU=25:25s`, `AS=1073741824:1073741824` (`1 GiB`), `FSIZE=67108864:67108864` (`64 MiB`), `core=0:0`:
  - `setUpClass` reads the 517 pinned read-only fixture files once (~30–40 MiB total in memory); `_clone_stages` and `_clone_assembled` share immutable `bytes` references without duplicating Kore/log payloads; `test_cat5` allocates one `4 MiB + 1 B` byte string (`over_log`) and one `1 MiB + 1 B` byte string (`over_kore`); `test_cat6` allocates two `1 MiB + 2 B` byte strings. Peak RSS/AS is well within `1 GiB`, and expected CPU time for ~15 full 95-capture decode passes is ~2–5 seconds (well within `20s` wall / `25s` CPU).
- **Aggregate temporary disk usage vs per-file `FSIZE=64 MiB`**:
  - `RLIMIT_FSIZE` (`64 MiB`) bounds only individual file offsets, **not** aggregate directory size or inode count.
  - In `test_cat6`, `tempfile.TemporaryDirectory()` creates 1 root dir, 196 stage subdirs, and 488 tiny files (<100 B each, ~25 KiB total actual bytes), plus two temporary `os.truncate` calls (`65,537 B` on `kast/stdout` and `1,048,577 B` on `exec-000/state.kore`, restored immediately in `finally:`), and 2 empty forbidden-name subdirs, all unlinked on `with tempfile.TemporaryDirectory()` exit. Peak logical size in `TMPDIR` is ~1.14 MiB across ~687 inodes, with zero persistent writes.
- **Explicit Root Controller Facts Required**:
  - Because `RLIMIT_FSIZE` and `RLIMIT_AS` do not cap parent-captured stdout/stderr pipes or enforce isolated temporary directory cleanup if `SIGKILL` fires mid-test, Root's execution controller must explicitly:
    1. Provide an isolated empty `TMPDIR` (as in `direct_seven` `21:L45`) and verify post-run that `TMPDIR` is empty and all 517/518 input + production + test pins are unchanged pre/post.
    2. Bound captured `stdout` and `stderr` streams in the Root controller (e.g., `<= 65,536 B` per stream; expected `unittest -v` output for 11 methods is `< 4 KiB` on `stderr` and `0 B` on `stdout`).
    3. Enforce clean environment (`PATH=/usr/bin:/bin`, `LANG=C.UTF-8`, isolated `TMPDIR`) with zero subprocesses (`author_source_subprocesses_network=False` verified by static inspection of `test_f03_general_adoption_guards.py` and the 4 production modules).

---

## 5. Coverage Matrix & Actionable Verdicts

### 5.1 Obligation & Category Coverage Matrix

| Obligation / Category | Method(s) in `test_f03_general_adoption_guards.py` | Status Before Fix | Status After Fixes 1–3 |
| :--- | :--- | :--- | :--- |
| **R328 Cat 1**: Positive 195-child + initializer (`k_commands_bytes=None` & pinned + direct `kast`) | `test_cat1_positive_195_child_initializer_jvm_pinned_unpinned_and_direct_kast` | Covered | **Covered** |
| **R328 Cat 2**: Negative `kast.receipt.argv` refusals | `test_cat2_negative_kast_receipt_argv_refusals` | Covered | **Covered** |
| **R328 Cat 3**: Negative `_validate_kast_stdout_kore` single-term refusals | `test_cat3_negative_validate_kast_stdout_kore_single_term_refusals` | Covered | **Covered** |
| **R328 Cat 4**: Negative pre-startup chain & template refusals (`k_commands_bytes=None`) | `test_cat4_negative_prestartup_chain_and_template_refusals` | **Defective** (`exec-000` template mismatch subcase hits `_engine_options` duplicate `--smt` error) | **Covered** (after Fix 1) |
| **R328 Cat 5**: Post-startup `strict_stage_validation=False` canonical gaps vs `True` `ValueError` | `test_cat5_poststartup_strict_false_gaps_vs_strict_true_value_error` | Covered | **Covered** |
| **R328 Cat 6**: On-disk 488-file `lstat`/symlink/size/oracle + unpinned `>1MiB` JSON + span rows | `test_cat6_ondisk_488_file_symlink_size_oracle_and_span_inventory_guards` | Covered (with honest `mock.patch.object` qualification for oracle SHA-256 preimage) | **Covered** |
| **Multiline `<k>` Head**: Positive single-`KItem` normalization/retention vs multi-item/wrong-root/`>MAX_TERM_BYTES` | `test_multiline_single_k_item_positive_normalization_and_retention`, `test_multiline_multiple_items_wrong_root_and_overbound_term_refusals` | **Defective** (`assertNotIn('~>', base_cfg)` on `print-001/stdout` in positive test) | **Covered** (after Fix 2) |
| **Whole `decode_capture` Chain**: 95-capture/94-edge positive, edge-008 inline/wrapped equivalence, logged-input/predecessor `state_kore`/terminal/path/template tampering | `test_full_chain_positive_control_and_edge008_inline_equivalence`, `test_full_chain_logged_input_and_predecessor_state_kore_tampering_refusals`, `test_full_chain_engine_and_printer_path_and_template_tampering_refusals` | **Defective** (`e5_argv.extend(['--smt', 'none'])` causes `_engine_options` to return `None`, skipping `outputs` and `templates` in `_chain_gaps`) | **Covered** (after Fix 3) |

### 5.2 Actionable Verdicts

1. **Test Source Verdict**: **`REVISE`**
   - Apply the **3 minimal uniquely-anchored literal `OLD`/`NEW` test-source fixes** in Section 3 (`Fix 1`, `Fix 2`, `Fix 3`) to `test_f03_general_adoption_guards.py`. No production module changes are needed or permitted.
2. **Prospective Offline-Operation Eligibility**: **`CONDITIONALLY ELIGIBLE AFTER EXACT SOURCE AND ARGV FIXES`**
   - **As literally proposed**: **Ineligible** because `python3 -I -B -m unittest -v test_f03_general_adoption_guards` fails module discovery under `-I` (`ModuleNotFoundError`), and the unrevised test source has 3 runtime assertion failures (Defects 1–3).
   - **After applying Fixes 1–3 and the corrected direct-script `argv` in Section 4.1**: **Eligible** for Root-selected offline execution under the `20s TERM / +2s KILL`, `CPU=25:25`, `AS=1 GiB`, `FSIZE=64 MiB`, `core=0:0` envelope with an isolated Root-managed `TMPDIR`, bounded `stdout`/`stderr` capture (`<= 64 KiB`), and pre/post SHA-256 pin verification across all read-only inputs.
   - **Scope & Gate Preservation**: Nothing has been executed (`PASS` count for this 11-method suite remains `0` until Root materializes the fixes and runs the command). Eligibility of this offline unittest operation does not authorize K/Quint/Cargo/SDK/native execution, does not alter existing `606+46` public observations or `passive_three` / `direct_seven` results, and does not itself grant general tool adoption prior to actual passing execution and owner acceptance.

