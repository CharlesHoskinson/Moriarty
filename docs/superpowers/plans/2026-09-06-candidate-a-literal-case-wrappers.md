# Candidate A Literal Case Wrappers Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans. Execute the three gates below in order. Root owns source-freeze release and independent review; the implementation worker does not grant its own native expansion or final acceptance.

**Goal:** Produce the actual canonical78-case/1557-event A4 export from exact per-case literals, with unchanged driver/semantic rules and fresh authentic producer receipts.

**Architecture:** Replace the generated wrappers' table projection with the identical canonical record. Reuse the unchanged recorder, RH002, parser, exporter and independent replay interfaces. The exporter gets a fixed fresh stage prefix and one necessary generator-indexing fix; no runtime transformation or new recorder is introduced.

**Tech Stack:** Pinned Python3.13/pytest, Node24.18.1, Quint0.32.0/Rust evaluator, existing strict JSON/streaming and GNU-time capture.

## Constraints and exact paths

- Source-only plan. No command below has run while authoring. Wait for root's explicit release after the corrected A5 retained-data after snapshot before editing or executing source-dependent checks. The first failed retained256 after snapshot alone is not that release.
- Adopted design: `docs/superpowers/specs/2026-09-06-candidate-a-literal-case-wrappers-design.md`, SHA256 `2686ac05bc3d7053c4866f147b5ee75d2186d896d85e3d5ba2aaf74d0c1269ea`; root review `.superpowers/sdd/a4-literal-case-wrappers-design-root-review-20260906.md`, SHA256 `cf0d95c411d20badf43e2a4df8030a85f39cdaf0f0ba4a7c9f8fbe98442bae75`.
- ROOT `/home/charl/Moriarty/.worktrees/s01-audit-start`; E ROOT + `/.superpowers/sdd/a4-literal-v1-20260906`; P ROOT + `/.superpowers/sdd/a4-producer-receipts`; A ROOT + `/evidence/s02-candidate-a-completion/a4`. E is a fresh finite work/receipt directory, not a new generic harness. Commands use ROOT cwd.
- Python `/home/charl/Moriarty/.venv/bin/python`; Node `/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node`; CLI `/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js`; RH002 `evidence/s02-candidate-a-completion/a4/native-resources/runner.py`; recorder `scripts/record_s02_candidate_a_integrated.py`.
- Preserve the original recorder SHA `dc30b764e2603dfe3c39ef6e2063a7082d116dc9d3eb9a012072f468beb8f8e9`, RH002 SHA `d8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d`, canonical cases SHA `f42eedc0b0a092704c6760943e4d37242453dedcf548696639f22c1897ef7518` and driver SHA `dc2fa2c7a6bdad1912583a6470b71b69091a3867f668072d86bbc86e472c251a`.
- All78 identities/global ordinals and1557 events remain;32 installment/638 events,46 swap/919 events. Keep all original required test node IDs, all semantic triples and package mutants. No skip/xfail, reduced inventories, synthetic native replacements, increased heap/native limits or compiler retry.
- Use the admitted parent environment from `.superpowers/sdd/a5-no-flatten-diagnostic-20260906/root-dispatch-full.json`, SHA256 `878870cf1c9ea1cc3dcbe64e37c002011081e93a7adc494c4e2565fd09d98fba` (`parentRemoved`/`parentFixed`) when constructing each Python tool command; additionally set `PYTHONOPTIMIZE=0`. Preserve `-B`, fresh caches and exact runtime identities. No unchecked `PYTHONPATH`, `NODE_OPTIONS` or preload variables. The original runtime verification is reused, not recaptured under different tools. Every displayed Python command below also uses this prefix; its abbreviated `PYTHONOPTIMIZE=0` spelling does not authorize an unsanitized parent environment.
- Actual commit/source hashes are recorded when the implementation is frozen; no future HEAD or future test count is invented. Root may commit the reviewed source before dispatch, then keeps that source epoch stable through all qualifying inputs and final tests. Evidence-only commits cannot substitute for checking the actual121 source pins.

## Gate1 — Implement, check exact equality/bindings, then independent source review

**Modify only:** `scripts/s02_candidate_a_integrated_inventory.py` (`case_wrapper`),78 existing `specs/quint/s02/candidate_a_integrated_case_NNN.qnt`, `scripts/export_s02_candidate_a_integrated.py` (two stage names and selector indexing), `tests/test_s02_candidate_a_integrated_export.py` (representation/path tests and new controls). The independent checker and its existing test file remain unchanged. No new production module or source-closure path is needed.

- [ ] After root releases the freeze, create E exclusively. Before edits, collect the three test modules with `python -B -m pytest --collect-only -q -p no:cacheprovider tests/test_a4_json_stream.py tests/test_s02_candidate_a_integrated.py tests/test_s02_candidate_a_integrated_export.py`. Save actual output and terminal as E/collection-before originals. Extract every line beginning `tests/` with `::` into E/original-nodeids.json. Also require the114 `requiredPreNativeNodeids` plus `nativeDeferredNode` in `.superpowers/sdd/a4-task6-resumption-20260906/partition.json` are included. This collection is identity evidence, not a passing test result.
- [ ] Change `case_wrapper` to the following complete body. Leave every preceding generator function unchanged.

```python
def case_wrapper(global_index):
    from pathlib import Path
    rows = inventory()["cases"]
    if type(global_index) is not int or not 0 <= global_index < len(rows):
        raise ValueError("fixed case index")
    desc, steps = list(instructions())[global_index]
    if {**desc, "event_count": len(steps)} != rows[global_index]:
        raise ValueError("fixed case instruction correspondence")
    name = f"candidate_a_integrated_case_{global_index:03d}"
    template = (Path(__file__).resolve().parents[1] /
                "specs/quint/s02/candidate_a_integrated_driver.qnt").read_text(encoding="utf-8")
    header = "module candidate_a_integrated_driver {\n"
    if not template.startswith(header) or not template.endswith("}\n"):
        raise ValueError("driver template framing")
    body = template[len(header):-2]
    replacements = {
        "const CASE_A4: A4Case\n": f"pure val CASE_A4: A4Case = {quint_case(desc, steps)}\n",
        "const CASE_INDEX_A4: int\n": f"pure val CASE_INDEX_A4: int = {global_index}\n",
    }
    for original, literal in replacements.items():
        if body.count(original) != 1:
            raise ValueError("driver template parameter inventory")
        body = body.replace(original, literal)
    return f"module {name} {{\n" + body + "}\n"
```

- [ ] Add `PRODUCER_STAGE_PREFIX = "literal-v1-"` beside the exporter transport constants. In `native_command`, change only the output path to `f".superpowers/sdd/a4-producer-receipts/{PRODUCER_STAGE_PREFIX}case-{i:03d}-export/case-{i:03d}.itf.json"`. In `validate_receipt`, change `name` to `f"{PRODUCER_STAGE_PREFIX}case-{row['global_index']:03d}-export/receipt.json"`. Replace the single `selectors = instructions()[row["global_index"]][1]` with `selectors = list(instructions())[row["global_index"]][1]`. Root specifically approved this last fix: the original generator is unsubscriptable, so the old code would fail upon iteration before canonical export. Root reproduced that baseline at actual19cb3b/1, retained in `.superpowers/sdd/a4-selector-iterator-root-red-20260906.json`, SHA256 `446fb5ea047e80a1ca2ac7c67deed9bd3e3bedf6e06d184a27f1a35bad8053da`. No selector content or semantic validation changes.
- [ ] Add these source-oracle functions to the existing exporter test file and replace only the body of `test_all_literal_case_and_wrapper_bytes_match_renderer` as shown. The pinned table, not the new renderer, supplies the78 expected records. Preserve its existing test name and invalid-index parametrization.

```python
def literal_table_records(text):
    import hashlib, re
    assert hashlib.sha256(text.encode()).hexdigest() == "f42eedc0b0a092704c6760943e4d37242453dedcf548696639f22c1897ef7518"
    records = []
    for match in re.finditer(r"\{descriptor: \{caseId:", text):
        depth = 0; quoted = False; escaped = False
        for end in range(match.start(), len(text)):
            c = text[end]
            if quoted:
                if escaped: escaped = False
                elif c == "\\": escaped = True
                elif c == '"': quoted = False
            elif c == '"': quoted = True
            elif c == "{": depth += 1
            elif c == "}":
                depth -= 1
                if depth == 0:
                    records.append(text[match.start():end + 1]); break
        else: raise AssertionError("unterminated literal")
    assert len(records) == 78
    return records

def assert_literal_wrapper(text, index, records, template):
    header = f"module candidate_a_integrated_case_{index:03d} {{\n"
    assert text.startswith(header) and text.endswith("}\n")
    body = text[len(header):-2]
    value = f"pure val CASE_A4: A4Case = {records[index]}\n"
    ordinal = f"pure val CASE_INDEX_A4: int = {index}\n"
    assert body.count(value) == body.count(ordinal) == 1
    assert 'import candidate_a_integrated_cases' not in body
    body = body.replace(value, "const CASE_A4: A4Case\n")
    body = body.replace(ordinal, "const CASE_INDEX_A4: int\n")
    assert body == template.split("\n", 1)[1][:-2]

def test_all_literal_case_and_wrapper_bytes_match_renderer():
    from scripts.s02_candidate_a_integrated_inventory import case_wrapper
    qnt = Path(__file__).resolve().parents[1] / "specs/quint/s02"
    table = (qnt / "candidate_a_integrated_cases.qnt").read_text()
    assert table == quint_cases()
    records = literal_table_records(table)
    assert {p.name for p in qnt.glob("candidate_a_integrated_case_*.qnt")} == {
        f"candidate_a_integrated_case_{i:03d}.qnt" for i in range(78)}
    template = (qnt / "candidate_a_integrated_driver.qnt").read_text()
    for i in range(78):
        text = (qnt / f"candidate_a_integrated_case_{i:03d}.qnt").read_text()
        assert text == case_wrapper(i)
        assert_literal_wrapper(text, i, records, template)

def test_literal_oracle_rejects_changed_case_ordinal_step_profile_import_and_driver():
    from scripts.s02_candidate_a_integrated_inventory import case_wrapper
    qnt = Path(__file__).resolve().parents[1] / "specs/quint/s02"
    records = literal_table_records((qnt / "candidate_a_integrated_cases.qnt").read_text())
    template = (qnt / "candidate_a_integrated_driver.qnt").read_text()
    good = case_wrapper(14)
    lines = good.splitlines(keepends=True)
    steps = [i for i,s in enumerate(lines) if 'kind: "transition", instruction:' in s]
    assert len(steps) >= 2
    reordered = lines.copy();a,b=steps[:2];reordered[a],reordered[b]=reordered[b],reordered[a]
    omitted = lines.copy();omitted.pop(b)
    bads = [good.replace(records[14], records[15], 1),
            good.replace("CASE_INDEX_A4: int = 14", "CASE_INDEX_A4: int = 0", 1),
            "".join(reordered), "".join(omitted),
            good.replace("SignAfterResolve", "SignBeforeResolve", 1),
            good.replace("\n", '\nimport candidate_a_integrated_cases.* from "./candidate_a_integrated_cases"\n', 1),
            good.replace("cursor + 1", "cursor + 2", 1)]
    for bad in bads:
        assert bad != good
        with pytest.raises(AssertionError):assert_literal_wrapper(bad,14,records,template)
    assert_literal_wrapper(good,14,records,template)

def test_raw_events_advances_past_generator_selector_lookup(tmp_path):
    from scripts.export_s02_candidate_a_integrated import raw_events, shards, file_digest
    path = tmp_path / "empty.itf.json";path.write_text('{"states":[]}')
    # next() enters the real generator body. TypeError is not accepted as rejection.
    for row in shards():
        with pytest.raises(ExportError, match="raw state/event inventory"):
            next(raw_events(path,row,file_digest(path),None,{}))

def test_literal_epoch_command_paths_and_fixed_selectors():
    from scripts.export_s02_candidate_a_integrated import native_command, shards
    rows=list(instructions())
    for i,row in enumerate(shards()):
        desc,steps=rows[i]
        assert {**desc,"event_count":len(steps)} == inventory()["cases"][i]
        argv=native_command(row)
        assert argv[-1] == f".superpowers/sdd/a4-producer-receipts/literal-v1-case-{i:03d}-export/case-{i:03d}.itf.json"
        assert argv[1:-2] == ["run",row["entry"],"--backend=rust","--seed=42","--max-samples=1","--n-traces=1",f"--max-steps={row['event_count']-1}","--invariants","noDiagnosticA4","sourceInvariantA4","--witnesses","completeA4"]
```

The last command assertion deliberately includes `--out-itf` only in the excluded last-two tail. Keep existing receipt mismatch tests and update only their fixed path expectation where necessary; do not make a legacy receipt acceptable. New tests add names; they do not rename/remove old names.

- [ ] Run the renderer test before regenerating wrappers and retain its expected literal-mismatch failure (not an import/setup failure). Then regenerate exactly78 files using `for i in range(78): (qnt/f'candidate_a_integrated_case_{i:03d}.qnt').write_text(case_wrapper(i),encoding='utf-8')` in an inline Python command that imports only the reviewed inventory module. Do not regenerate the canonical table or driver.
- [ ] Run the four tests above plus the original invalid-index and exact-inventory tests. Then run the three complete Python modules with only these exact five actual-package nodes deselected until Gate3: `tests/test_s02_candidate_a_integrated.py::test_actual_complete_package_and_inventory_triples`, and exporter tests `test_actual_complete_sharded_corpus_matches_manifest`, `test_actual_case_omission_cannot_pass_fixed_inventory`, `test_actual_event_omission_cannot_pass_case_inventory`, `test_actual_reordered_computation_requests_rejected`. Use `-p no:cacheprovider --basetemp E/pre-tests --junitxml E/pre-tests.xml`; no skip injection. Collect again and require the exact original node list is a subset of the new collection, with each original parameter suffix intact. Preserve actual pytest parent/child command, streams, generated files and terminal; no dots-based success. The final full invocation is still mandatory.

### Fresh parse and exact binding comparison

- [ ] Freeze the121-entry source map as E/source-pins.json using the existing `required_sources(ROOT)` and streaming `file_digest`. Require equality of recorder/importer/checker membership; unchanged member names and all frozen Core/semantic pins. Record current HEAD and changed-source list in E/epoch.json together with plan/design/review pins, the actual collection lists and original parser admission pin. Source maps, not a fixed future HEAD, govern later checks.
- [ ] Obtain one fresh aggregate parse using the existing final-source-parser capture procedure, with only stage `literal-v1-aggregate-parse`, outer directory E/outer/aggregate-parse, and output P/literal-v1-aggregate-parse/aggregate-ir.json substituted. Exact inner argv: `NODE --max-old-space-size=4096 CLI parse specs/quint/s02/candidate_a_integrated_wrappers_typecheck.qnt --out ABSOLUTE_NEW_IR`; exact RH002 wall900, recorder cache P/literal-v1-aggregate-parse/python-cache. Preserve actual incremental transport in E/transport/aggregate-parse. Require authentic0, complete unforced cleanup, stable121/runtime maps, empty native stdout/stderr and IR<=128MiB. Reuse the published capture procedure; do not modify its old records or re-execute the old parse.
- [ ] Compare the admitted old IR at P/task6-final-source-aggregate-parse-20260906/aggregate-ir.json,38527047 bytes, SHA `cbf57ad5a7f194ba62bf897553d3750ddba899190683ab8293cc633282886e1e`, against the new IR. Authenticate the old pin through `.superpowers/sdd/a4-final-source-parser-admission-20260906.json`. Both have exact keys `stage,warnings,modules,table,errors`; require parsing stage, empty errors/warnings,122 unique typedefs and the seven original required types. Read with existing `bounded_load(...,PARSER_LIMIT)`, not an unbounded JSON read. The following is the exact comparison algorithm, implemented as a finite inline data command retained at E/bindings-command.json (no installed helper):

  1. Index each IR's modules by unique name and its top-level non-import declarations by `(module,name,kind)`. Index every declaration ID to that identity; index nested lambda parameters/local operator definitions by their owner declaration and structural path. For shared/repeated ID nodes, require compatible node structure after deleting `id`; prefer the top-level declaration as the identity of its own ID. A reference to a missing declaration fails.
  2. Delete only `id` and analysis-only `depth` from module declaration ASTs for structural comparison. Do not delete `name`, `qualifier`, type annotations, field labels, operator/constructor tags or ordered AST lists. Compare top-level declaration maps, not topologically sorted declaration order. The expected new module set is exactly the old set minus `candidate_a_integrated_cases`; all retained semantic declarations/types match. Source-file hashes for all unchanged defining modules must match the old admitted source map. Imports are checked from exact source bytes by the renderer test; they are not treated as new semantic declarations.
  3. Locate the old canonical `INSTALLMENT_CASES_A4` and `SWAP_CASES_A4` definitions. Require their expressions are `app/List` with32 and46 arguments. For wrapper i, compare its new `CASE_A4.expr` with the old table argument i or i-32. Compare its full declaration, including `pureval` and `A4Case` annotation, after that single expected replacement. Compare every other wrapper declaration with the corresponding original declaration. The unchanged driver module also matches in full.
  4. For each compared expression/type pair, traverse matched structure positions. For each `name`, `app` or type-`const` node, look up its ID in its own IR `table`. Require either both nodes are builtin/unbound in the resolver table or both resolve to the same defining module/declaration identity. For locals require identical owner and structural binder path. Table annotations such as `importedFrom`, `hidden`, `shadowing` and numeric IDs do not define identity. Every resolved top-level definition's normalized body has already been compared in steps2/3; do not recursively expand bodies through the resolver table. This checks actual bindings and avoids cycles/whole-table duplication.
  5. Require each required typedef, nested type application and all78 literal/driver comparisons pass. Save both input hashes, exact compared module/declaration/reference counts and the78 global identities to E/bindings.json, then rehash both IR inputs and all121 source pins. No zero-reference shortcut or unreviewed normalization is allowed. A mismatch stops Gate1; root reviews any necessary algorithm/source correction before another parser call. This data comparison is not a new compiler or runtime change.

- [ ] Run existing required Quint typecheck/test entries under the unchanged recorder/RH002900-second native envelope, with fresh stages `literal-v1-typecheck`, `literal-v1-export-tests`, `literal-v1-installment-tests`, `literal-v1-swap-tests`. Exact requested commands are `quint typecheck specs/quint/s02/candidate_a_integrated_wrappers_typecheck.qnt`; `quint test specs/quint/s02/candidate_a_integrated_export_test.qnt --backend=rust --seed=42`; `quint test specs/quint/s02/candidate_a_authority_installment_test.qnt --backend=rust --seed=42`; and `quint test specs/quint/s02/candidate_a_authority_swap_test.qnt --backend=rust --seed=42`. Use the same recorder/RH002 command construction as Gate2 with the named stage and E/outer/STAGE, replacing only its requested command. No test definition is removed. Original failures are retained and block dispatch.
- [ ] Nonauthor reviews the complete patch, original-node inclusion, source equality/negative controls, actual binding results and new source/runtime receipts. Root records one Gate1 decision in E/review-source.md. This is the first of three gates, not an additional design cycle. Root may commit the scoped reviewed patch and record the actual HEAD before Gate2; no native process overlaps source edits.

## Gate2 — Two qualifying cases, then the other76

No diagnostic launcher/observer is used. Each successful case is a canonical all78 input and will be reused. Cases014 and044 are the original largest-case pilots, now with new literal source/receipt names. Execute014 once; if it qualifies, execute044 once; root reviews the pair before the remaining76 in ascending ordinal order excluding14 and44.

- [ ] Build exact per-case commands from the reviewed exporter and save them in E/native-commands.json. For each i, the inner requested command is `native_command(shards()[i])`. Recorder argv is `[PYTHON,'-B','-X','pycache_prefix='+str(P/stage/'python-cache'),str(ROOT/'scripts/record_s02_candidate_a_integrated.py'),'--stage',stage,'--',*native_command(row)]`, where stage is exactly `literal-v1-case-%03d-export`.
- [ ] RH002 argv is `[PYTHON,'-B',str(ROOT/RH002),'--receipt-dir',str(E/'outer'/('case-%03d'%i)),'--inner-receipt',str(P/stage/'receipt.json'),'--wall-seconds','900','--cwd',str(ROOT),'--',*recorder_argv]`. Use the admitted clean parent environment and shell-quote argv with `shlex.join`; do not use multiline `python -c` inside GNU-time. No Node heap flag is added to a canonical run. Every stage/outer/transport path must be fresh. Pin the unchanged121 source map, runtime and commands before launch.
- [ ] Preserve each original tool response immediately in E/transport/case-NNN/response-NNN.json before the next poll; retain command.json and authentic terminal `{args,responses,actualOuterExit}`. Poll the same session through its actual terminal. Save failures, missing transport and takeover facts truthfully; no restart or inferred cleanup.
- [ ] Qualifying predicate: actual outer0, RH0020/eligible with complete unforced cleanup/errors[], absent owned group, no timeout/monitor/launch error, inner native/recorder0, complete stable source/runtime endpoints, exact new canonical requested and executed commands, exactly one ITF artifact with the expected case filename/hash, and unchanged tool identities. Use `validate_receipt` with that row and its actual pin; maintain the exact121 source map for every case. A missing/failed receipt or ITF is a preserved failure, not a qualified case.
- [ ] For each qualifying ITF, use Types from the new admitted aggregate IR and `raw_events(path,row,input_hash,types,initial_hashes)` to stream the one case into E/pilot-cases/case-NNN.json using `write_array_document` with the exact descriptor fields. Run unchanged `check_shard` on that case/raw pair with the fixed descriptor and global index. Do not load all raw events into memory. The two pilots' temporary converted cases are validation artifacts; final staging will regenerate the canonical case files from the unchanged original raw bytes. Their native executions are not repeated.
- [ ] Root reviews014/044 under E/review-pilots.md. On pass, execute the fixed76 remaining cases once each, sequentially, preserving the same evidence and stopping at the first nonqualifying result. No additional per-case approval loop or automatic retry is introduced. Each case's original after map must still equal E/source-pins.json. If any source must change, close this epoch; prior outputs cannot be silently mixed with the changed epoch.

## Gate3 — Complete export, independent replay, full tests and mutants

The canonical A/inventory.json already exists and must remain byte-identical. Currently A/raw, A/cases, A/cases.json and A/admission.json do not exist. Require that still holds before creating them; fail on any collision. Existing evidence subdirectories stay untouched. Use E/stage-admission.json for the temporary admission without case pins; only the later complete A/admission.json is the final package admission.

- [ ] Check all78 qualifying cases and exact source/runtime identities again. Copy each original ITF exclusively to A/raw/case-NNN.itf.json and its complete original recorder stage losslessly to A/literal-v1-case-NNN-export, using byte-preserving copies and streaming verification. Do not rewrite requested argv, source HEAD or any original receipt. Retain RH002 and actual transport under the existing E paths, bound by the final package review; do not fabricate these as inner artifacts.
- [ ] Build the schema3 staging admission from exact current data: `transport='case-sharded-itf-v1'`, `inventory_sha256=digest(canonical(inventory()))`, `source_pins=E/source-pins.json`, `entries=entries()`, all78 `input_pins` under their fixed raw paths, and `receipt_pins` for every copied original file under the78 prefixed producer directories. Verify existing A/inventory.json strictly equals `inventory()` and the independent checker's inventory. Write E/stage-admission.json exclusively with these exact seven fields (`schema_version,transport,inventory_sha256,source_pins,input_pins,receipt_pins,entries`). Do not include `case_pins` yet.
- [ ] Run exactly:

```bash
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B scripts/export_s02_candidate_a_integrated.py --mode stage-cases --input-root evidence/s02-candidate-a-completion/a4 --source-root . --inventory evidence/s02-candidate-a-completion/a4/inventory.json --admission .superpowers/sdd/a4-literal-v1-20260906/stage-admission.json --output .superpowers/sdd/a4-literal-v1-20260906/staged.json --parser-receipts .superpowers/sdd/a4-literal-v1-20260906/parser-stage
```

Preserve its actual command/streams/terminal and original parser receipts. Require actual0,78 staged cases/1557 events and the exact78 case-path pins. The existing exporter performs one fresh aggregate parse, bounded900 seconds/4096MiB/128MiB; the earlier source-binding parse does not replace it.

- [ ] Root makes the complete A/admission.json by preserving exactly the staging admission's fields and adding only `case_pins` from staged.json after streaming hash verification. This binds complete actual native/case files; it is not permission to manufacture a native receipt. Write exclusively. Run:

```bash
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B scripts/export_s02_candidate_a_integrated.py --mode seal-manifest --input-root evidence/s02-candidate-a-completion/a4 --source-root . --inventory evidence/s02-candidate-a-completion/a4/inventory.json --admission evidence/s02-candidate-a-completion/a4/admission.json --output evidence/s02-candidate-a-completion/a4/cases.json --parser-receipts .superpowers/sdd/a4-literal-v1-20260906/parser-seal
```

Require its separate fresh parse and actual terminal0. Then run unchanged independent replay:

```bash
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B scripts/check_s02_candidate_a_integrated.py --cases evidence/s02-candidate-a-completion/a4/cases.json --admission evidence/s02-candidate-a-completion/a4/admission.json --source-root . --input-root evidence/s02-candidate-a-completion/a4 --inventory evidence/s02-candidate-a-completion/a4/inventory.json --report .superpowers/sdd/a4-literal-v1-20260906/package-report.json
```

- [ ] Finally run all three Python test modules, without deselections, under optimization0 and fresh paths:

```bash
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B -m pytest -q -p no:cacheprovider --basetemp .superpowers/sdd/a4-literal-v1-20260906/final-tests --junitxml .superpowers/sdd/a4-literal-v1-20260906/final-tests.xml tests/test_a4_json_stream.py tests/test_s02_candidate_a_integrated.py tests/test_s02_candidate_a_integrated_export.py
```

Require actual terminal0, every collected original node exactly once, no skips/xfails, all newly added nodes, authentic sequential child exits and original generated semantic control files. Report actual JUnit count rather than predicting a future115 or larger total. The existing complete-package test must kill all seven manifest/admission mutants and accept the unchanged complete78-case package after each rejection; the existing semantic triples and positive counterparts remain. The actual exporter corpus test performs its own fresh parser; no test is replaced by a saved parser result.

- [ ] Nonauthor reviews the complete package, actual command/terminal/cleanup/source/runtime bindings, all78 raw/case linkage and1557-event result, final full tests and mutants. Root records E/review-final.md as the third gate. Preserve all originals, then scope any evidence commit to the completed reviewed result. Completion is complete finite-record agreement/authority export only; H1/model checking/cryptography/solver/correspondence remain outside that claim.

## Failure and handoff rules

No source/body/native execution is authorized by authoring this plan. Implementation follows root's freeze release and plan review, then the three gates. The original source investigation and failed native diagnostics remain unchanged. All later data and native calls retain authentic stdout/stderr, args, terminal, and generated artifacts. Unknown exit/cleanup remains unknown; never hash a live stage into a final admission. Stop at a pin, binding, test, parser or native failure. Root may review a concrete correction, but this plan supplies no hidden retry, source normalization exception, heap increase or omitted case.

Plan review must specifically check the admitted old IR binding algorithm against its actual resolver schema and the direct regression against deferred `raw_events` iteration. The only design supplement is the root-approved one-line generator-indexing correction; all other work implements the adopted literal-wrapper design. No new generic framework or redundant archive/admission hierarchy is part of this plan.
