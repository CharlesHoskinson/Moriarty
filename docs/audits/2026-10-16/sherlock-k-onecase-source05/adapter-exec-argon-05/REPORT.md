### Review Identity & Verdict

- **Reviewer Identity:** Gemini 4 Argon high via AGY (user-authorized security outage fallback; non-author; **not** Opus; independent of Muse high).
- **Review Mode:** Fresh non-author full-source **PRE-EXECUTION** eligibility review only. Zero tools, browsing, commands, file actions, delegations, SDK, K engine, compile, native, circuit, proof, key, or network actions were invoked.
- **Verdict:** **`APPROVE_WITH_SCOPE`** (bounded strictly to pre-execution source eligibility for one execution of the exact proposed pure-Python assembler command below; **not** a test pass claim, **not** an actual-result approval, and **not** a comparison/equivalence pass).

---

### 1. Approved Command & Pinned Source Digests

**Exact eligible command (for one-shot execution after required parallel review):**
```bash
/usr/bin/python3 -I -B /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/build_actual_n01_bundle.py --approved-self-sha256 d3fc5ec8d6680eaddd30d9dcb9b329b1e6c52d83f0bacbb7db3bbe72c5e7a92f --approved-module-sha256 5df990ed9c8b54ca1e090e80883f483f2e745138a740cdbcbbfc80fd00ce77d1
```

| File / Artifact | Bytes | SHA-256 | Status in Review |
|---|---:|---|---|
| [build_actual_n01_bundle.py](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/build_actual_n01_bundle.py) | `26173` | `d3fc5ec8d6680eaddd30d9dcb9b329b1e6c52d83f0bacbb7db3bbe72c5e7a92f` | Full source reviewed; matches `--approved-self-sha256` |
| [k_capture_observations.py](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_capture_observations.py) | `41183` | `5df990ed9c8b54ca1e090e80883f483f2e745138a740cdbcbbfc80fd00ce77d1` | Full source reviewed; matches `--approved-module-sha256` and `ADAPTER` |
| [k_observations.py](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_observations.py) | `7686` | `540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07` | Full source reviewed; matches `ADAPTER` |
| [k_terms.py](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_terms.py) | `5666` | `c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca` | Full source reviewed; matches `ADAPTER` |
| [receipt.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/attempt-engine-N01-01/receipt.json) | `295790` | `d6123acfdfd16128d533701dae3316bf166c08e5ae393fccbb958657f38d02cc` | Top-level, complete `postflight`, and initial `preflight` reviewed (see §4.1 on transport truncation) |
| `COMPILED-MAIN-RULE-SPANS-01.json` | `85954` | `9e77ddfe74acaa1e2deccaa1fcde70794494dd3583362b3f9a6f58473d9e941c` | Pinned in `INVENTORY` (113 rows, schema `sherlock-k-compiled-source-attribute-inventory/1`) |
| `compile-input/models/sherlock/k/main.k` | `16249` | `b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41` | Pinned in `SOURCE` |
| `inputs/N01-fuel-001.ui` | `43` | `e5fe7b26e9565fdafdc97da0fb25fb81c6171afd69e99eaa6dfb38b5409a2e0f` | Pinned in `RUN_INPUT` |

---

### 2. Code & Schema Fit Verification (`k_capture_observations.py` + `build_actual_n01_bundle.py`)

1. **Frozen Inner Bundle & Outer Envelope Separation:**
   - [`observation_bundle`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_capture_observations.py#L465-L522) preserves `BUNDLE_FORMAT = 'sherlock-k-normal-observations/1'`, `PRODUCER = 'sherlock-k-reference'`, and the exact six-key `observations` object (`{'producer', 'profile', 'source_sha256', 'boundary', 'programs', 'runs'}`).
   - [`_assemble`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/build_actual_n01_bundle.py#L244-L355) asserts `set(observations) == OBSERVATION_KEYS` and nests `bundle` unchanged inside the outer envelope (`schema = 'sherlock-k-actual-n01-fuel1-envelope/1'`). Any downstream consumer must explicitly select `envelope['bundle']`.

2. **Observed Installed K Log Layout & Span Binding:**
   - [`decode_log`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_capture_observations.py#L248-L315) and [`_step`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_capture_observations.py#L318-L347) claim every line of raw ASCII `stderr` bytes or record a blocking gap:
     - **Rewrite steps (`04`, `05`, `06`):** require `_REWRITE_LAYOUT = ('DebugAppliedRewriteRules', 'WarnDepthLimitExceeded', 'InfoExecDepth')`, `variant == 'applied'`, exactly 1 location span in `source_path`, `WarnDepthLimitExceeded` matching `--depth` (`1`), and `InfoExecDepth` equal to `1`.
     - **Terminal replay (`07`):** requires `_NO_RULE_LAYOUT = ('DebugAppliedRewriteRules', 'InfoExecDepth')`, explicit `'    No rules were applied.'` (`variant == 'no-rules'`), zero location rows, no depth-limit warning, and `InfoExecDepth` equal to `0`.
   - [`_spans`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_capture_observations.py#L197-L218) indexes the 113-row compiled inventory by full 4-tuple `(start_line, start_column, end_line, end_column)` for `source_path` and rejects duplicate spans or duplicate `unique_id` digests. [`_attributes_agree`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/build_actual_n01_bundle.py#L123-L129) in the assembler verifies that all 113 rows carry matching `Location(...)`, `UNIQUE'Unds'ID{}("...")`, and `Source(...)` substrings in `raw_attributes`. Rule identities are never inferred from states or from `receipt['expectations']`.

3. **Startup Pre-State, Depth-0 Initializer, and Post-Startup Initial Pairing:**
   - Stages `01-kast`, `02-initializer`, `03-exec-depth0` (942 B `state.kore`, 152 B `stderr`), and `08-kprint-03` (91 B `stdout`, which is an unnormalized initializer term rather than the 9 cells) are retained raw in `retained['artifacts']` and listed in `pairing['retained_only']`; `08-kprint-03` is never passed to `_configuration` or `decode_capture`.
   - Startup (`04-exec-depth1-1` / `09-kprint-04`) is decoded via [`decode_startup_probe`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_capture_observations.py#L401-L430): `before` is `None`, `before_kore` retains the exact `On configuration:` Kore text and SHA-256 from `04`'s log untyped, `before_projection` explicitly states it is not established, and `after` is checked for raw and snapshot equality with `captures[0]['configuration']` (`09-kprint-04`).
   - Post-startup edges (`05/10` semantic, `06/11` publication) and terminal replay (`07/12`) enforce [`_logged_input_gaps`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_capture_observations.py#L350-L358), requiring the logged `On configuration:` term to equal the preceding stage's `state.kore` bytes re-indented by 10 spaces inside the observed `\and{SortGeneratedTopCell{}}` / `\top` wrapper.

4. **Unit-Tail Reduction & `<generatedCounter>` Boundary:**
   - [`_configuration`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_capture_observations.py#L106-L130) bounds size (`<= 8192` B), character set (`[\x20-\x7e\n]*`), parenthesis nesting (`<= 32`), and cell tags (`generatedTop` + the 9 declared `main.k` cells) before calling [`decode_configuration`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_observations.py#L64-L81).
   - Only a single `<k>` item of the form `X ~> .K` (single `~>`, single-line non-empty `X != '.K'`, `.K` unit tail) is reduced to `X` (applicable to `09` and `10`; `11` and `12` have `<k> .K </k>` with no `~>`). Original unreduced texts and SHA-256 digests are retained in `original_configurations`, `raw_captures`, and `decoded_configurations`.
   - `Lbl'-LT-'generatedCounter'-GT-'` in raw `state.kore` is tracked per edge via `raw_kore_has_generated_counter_label` and disclosed in `projection_boundary` (`printer_omitted=['generatedCounter']`).

5. **Stage Receipt & Command Chain Checks:**
   - [`_stages`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/build_actual_n01_bundle.py#L132-L162), [`_artifacts`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/build_actual_n01_bundle.py#L165-L189), and [`_commands`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/build_actual_n01_bundle.py#L192-L210) verify all 12 stages (`02-initializer` driver-written plus 11 child command receipts with `returncode == 0`, `timed_out is False`, `containment_breach is False`, and empty problem/residue lists), all 5 `postflight` problem/change lists empty (verified against the supplied `receipt.json` `postflight`), exact `attempt_tree` record equality, and the unbroken path chain `N01-fuel-001.ui -> 01-kast -> 02-initializer -> 03..07 -> 08..12`.
   - Distinctness of `engine_output_kore_sha256` and `print_stdout_configuration_sha256` is enforced on every edge.

6. **Side-Effect & Isolation Controls:**
   - [`main`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/build_actual_n01_bundle.py#L387-L433) requires `sys.flags.isolated` (`-I`), sets `sys.dont_write_bytecode = True` (`-B`), loads sibling modules via `importlib.util.spec_from_file_location` using explicit paths (unaffected by `-I` omitting the script directory from `sys.path`), reads files with `O_RDONLY | O_NOFOLLOW | O_CLOEXEC` and `realpath` verification, and writes exclusively via `O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW | O_CLOEXEC` (`0o600`) to `R/adapter-actual-source-05/ACTUAL-N01-FUEL1.json` and `R/adapter-actual-source-05/ACTUAL-N01-FUEL1-OPERATOR-STATUS.json`.

---

### 3. Preconditions for Execution

1. **Materialization & Sibling Presence Before First Run:**
   - `build_actual_n01_bundle.py` (26,173 B, SHA-256 `d3fc5ec8...`) must be placed at `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/build_actual_n01_bundle.py`.
   - All three adapter files (`k_capture_observations.py`, `k_observations.py`, `k_terms.py`) must already be present in `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/` with their exact pinned sizes and SHA-256 hashes **before** invoking the command. Because `_load_adapter()` runs inside `_assemble()`, a missing sibling file would raise `FileNotFoundError` (`OSError`), causing `_blocked()` to write `ACTUAL-N01-FUEL1-OPERATOR-STATUS.json` (`outcome='blocked'`) and thereby consume the one-shot status file slot.
2. **Absence of Output Files:**
   - Neither `ACTUAL-N01-FUEL1.json` nor `ACTUAL-N01-FUEL1-OPERATOR-STATUS.json` may exist in `R/adapter-actual-source-05` prior to execution (`os.path.lexists` check refuses with exit code `2` before any write).
3. **Realpath Invariant:**
   - `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01` and all referenced paths under it must have no symlink components (`os.path.realpath(text) == text`), or `_bytes_at` / `OUT_DIR` validation will refuse/block.
4. **Parallel Review Requirement:**
   - Execution requires both this pre-execution source approval and the separate applicable review (Muse high) per owner governance.

---

### 4. Disclosed Limitations, Source Gaps & Security Boundaries

1. **Prompt Transport Truncation of Trailing `receipt.json` Bytes:**
   - The prompt stream truncated the trailing 399,390 bytes inside `receipt.json`'s `preflight.inventory`. All four Python source files, both Opus `REPORT.md` files, and `receipt.json`'s top-level fields, complete `postflight.attempt_tree` (including all stage file sizes and SHA-256 digests), and initial `preflight` fields were received in full. At runtime, `build_actual_n01_bundle.py` verifies the full 295,790-byte `receipt.json` against `d6123acfdfd16128d533701dae3316bf166c08e5ae393fccbb958657f38d02cc` and enforces all stage/preflight checks fail-closed.
2. **Consistency vs. Unauthenticated Supplied Receipt Trust:**
   - Byte sizes, SHA-256 hashes, command-line option templates, path chains, and `On configuration:` input Kore embeddings establish **internal consistency only**. The attempt receipt is unsigned; `driver_sha256` and `identity_sha256` are compared against constants, not cryptographically attested; producer authenticity remains `'not-established'`.
3. **Import-Time / Load-Time TOCTOU Window (A5/A7/A9/A10):**
   - `build_actual_n01_bundle.py` verifies the sizes and SHA-256 hashes of `k_capture_observations.py`, `k_observations.py`, and `k_terms.py` before loading, compares `module.ADAPTER_SOURCES` (computed at import time), and re-verifies all three files after assembly. However, `importlib.util.spec_from_file_location` re-opens the files by path rather than executing from the verified in-memory byte buffers, leaving a residual filesystem TOCTOU gap if an untrusted process had concurrent write access to `R/adapter-actual-source-05`.
4. **Compiled `definition.kore` Not Opened Directly:**
   - `DEFINITION_SHA256` (`d795e328...`) is checked against `COMPILED-MAIN-RULE-SPANS-01.json` and `receipt['preflight']['protected']['compile_attempt']`; the `definition.kore` artifact itself is not read from disk by the assembler (explicitly disclosed in `not_opened`).
5. **Rewrite Axioms vs. Equations & Corpus / Comparison Scope:**
   - The 113-row inventory binds source spans to compiled `UNIQUE'Unds'ID` attributes but does not syntactically distinguish rewrite axioms from equations beyond `kore-exec` emitting them under `DebugAppliedRewriteRules`.
   - This source/assembler covers **only** the single observed N01 fuel-1 capture (`attempt-engine-N01-01`). The other 215 fixed inputs remain unobserved.
   - The independent Root expected comparison is withheld; no test pass claim is supplied (the tests-only continuation remains live); and a full final `source + tests + actual-result` audit remains required after assembly before any reliance is permitted.

