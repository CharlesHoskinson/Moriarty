# Candidate A final-source parser capture Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to execute this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Retain and independently admit one aggregate parse IR matching the current complete A4 source freeze, as a prerequisite for later pilot-only structural inspection.

**Architecture:** Reuse the admitted RH002 CLI around the unchanged integrated recorder and one explicit pinned Node/Quint parse command. The recorder captures the full source/runtime endpoints; root separately binds the original `--out` IR and authentic outer transport. No new recorder, wrapper, parser API, source mutation, or helper-global override is introduced.

**Tech Stack:** Existing Python virtual environment, RH002 GNU-time capture, integrated recorder, pinned Node 24.18.1 and Quint 0.32.0, and the exporter's existing strict JSON/Types interfaces.

## Global constraints and evidence basis

- Status: **specified-only**; this plan is not parser execution, root dispatch, pilot acceptance, final A4 completion, or H1 evidence. No native/parser/test/helper import was performed while writing it.
- Worktree: `/home/charl/Moriarty/.worktrees/s01-audit-start`. Planning-time HEAD was `07c3a5462154d45e733d01d8a8bae856cb0f63c4`; record the actual later dispatch HEAD separately.
- Preserve the historical aggregate IR and all earlier receipts. The historical parse has 122 unique typedefs but differs in two current aggregate dependencies; it does not meet the final-source requirement. Controlling intake: `.superpowers/sdd/a4-pilot-parser-evidence-intake-20260906.md`, SHA-256 `4a25fae90aebe186b5cac2060509612a252100fad63593ee4e46a70612f38e96`; scope and later pilot requirements: `.superpowers/sdd/a4-largest-pilot-readiness.md`.
- The preliminary gate is root-admitted at `.superpowers/sdd/a4-task6-resumption-20260906/root-admission114.json`, SHA-256 `2b112e8273b7f2a1b12adf9cc1404dc7c3da5fb933e14a972fc12f35519aba08`. It comprises 17 retained controls plus 97 fresh tests. It supplies neither the unknown historical parent exit nor final native/final-115 acceptance.
- Exactly one aggregate parse attempt, outer wall 900 seconds, explicit Node `--max-old-space-size=4096`. No retry, increased limit, pilot run, source change, factored input, compile, typecheck, or solver is supplied by this plan.
- Acceptance cap: original IR at most 134217728 bytes (128 MiB), strict JSON and exact parser schema, 122 unique typedef names and all seven required types. This is an admission bound, not a streaming disk cap; retain oversized/partial bytes without truncation.
- Existing recorder runtime verification and four tool-version probes remain part of its unchanged behavior. They are not extra aggregate parses. The 900-second outer interval includes recorder setup, runtime checks, version probes, source snapshots, the parse, and receipt completion; it is not a fresh 900 seconds solely for parsing.

| Required unchanged file | SHA-256 |
| --- | --- |
| `scripts/record_s02_candidate_a_integrated.py` | `dc30b764e2603dfe3c39ef6e2063a7082d116dc9d3eb9a012072f468beb8f8e9` |
| `evidence/s02-candidate-a-completion/a4/native-resources/runner.py` | `d8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d` |
| `scripts/export_s02_candidate_a_integrated.py` | `51028d697cd9be91ebfa97b5d7d6f69127ca700fd47313648cb4138877dddcd5` |
| `scripts/run_s02_candidate_a_factoring_pilot.py` | `816c3ad79dc3a67ca9a03729af56d74188c161a7546b9c43e821c222a4bce7f2` |
| `/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node` | `f3432a45b03b2da0d270095fdd8813dc34cbea73f5fc8b18c7a384b7cf9b333a` |
| `/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js` | `ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501` |

## Task FP001: Independently review and freeze the dispatch

**Files:** Create only the new root data receipt `.superpowers/sdd/a4-final-source-parser-dispatch-20260906.json`. Keep producer dispatch `.superpowers/sdd/a4-producer-dispatch.json` unchanged: SHA-256 `cc8ab817598c84a4d44c6464d9ce341d4ea7959461370634488c995858dc9147`, producer base `386bf0ae10f767e051b414a7231be105cc4b0f71`. Shared runtime bootstrap remains `900bb2051225b4a3d99bf422c3b2e5e386e3e7bc`.

**Interfaces inspected:** RH002 `capture(argv, receipt_dir, inner_receipt, wall_seconds=900, cwd=None, grace_seconds=5, cleanup_seconds=5)` requires absent outer directory and absent inner receipt outside that directory. Integrated recorder `--stage NAME -- COMMAND` requires an absent stage and exact `-B -X pycache_prefix=STAGE/python-cache`; it resolves only a command whose first item is literal `quint`. Therefore supply the explicit Node command directly, retaining the heap option in its original `command` and `executed_command` fields. `snapshot()` and exporter `required_sources(root)` agree on the 121-file full closure.

- [ ] Root independently adopts the exact reviewed plan, checks the six source/tool pins above, and freezes the complete current 121-source map using the existing `required_sources(root)` interface. Require all sources present, no symlinks, and no factored-module substitution. The two corrected aggregate inputs remain `candidate_a_integrated_case_032.qnt` SHA `e9f4d1e63b4ca0d19725e26317a878fbb9f6dc949d0609bdb265b7b5805b267c` and `candidate_a_integrated_lowering.qnt` SHA `a6dac06db7bb10cab7b33dc8ac63e2b18f88c12e4fd52066d0e2614e15b7c322`.
- [ ] Require actual prior A4 fresh97 and A5 diagnostic terminals plus their actual outer terminals. Inspect their recorded process groups with read-only liveness checks; absent/clean is required, and a reused/uncertain group ID blocks dispatch. The A5 timeout's forced but completed cleanup is sufficient for non-overlap, not compiler success. The relevant A5 process terminal is `.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/full/full-observed/process-terminal.json`, SHA `8d00cbc294996d789c7815e5c8590e159f3224a5a27775bcf2283719582f4e35`; its outer terminal SHA is `3938090a70a53149dd1c437ee939e05a439f7e1053fcd6aea391f47015b44cc8`.
- [ ] Confirm these destinations are absent and their parents already exist. Do not precreate the inner/outer directories or caches:

| Role | Exact worktree-relative path |
| --- | --- |
| Inner stage | `.superpowers/sdd/a4-producer-receipts/task6-final-source-aggregate-parse-20260906/` |
| Original IR | `.superpowers/sdd/a4-producer-receipts/task6-final-source-aggregate-parse-20260906/aggregate-ir.json` |
| Inner receipt | `.superpowers/sdd/a4-producer-receipts/task6-final-source-aggregate-parse-20260906/receipt.json` |
| Inner cache | `.superpowers/sdd/a4-producer-receipts/task6-final-source-aggregate-parse-20260906/python-cache/` |
| RH002 outer directory | `.superpowers/sdd/a4-final-source-aggregate-parse-20260906/` |
| RH002 parent cache | `.superpowers/sdd/a4-final-source-aggregate-parse-20260906-parent-cache/` |
| Original tool transport | `.superpowers/sdd/a4-final-source-parser-transport-20260906/` |
| Root parser admission | `.superpowers/sdd/a4-final-source-parser-admission-20260906.json` |

- [ ] Only after those reviews, root may run this data-only freeze/authorization command. It imports the existing source-enumeration interface at execution time; it performs no parser/helper/native capture. Its resulting JSON is a separate root dispatch, not an alteration of the original producer/runtime bases.

```sh
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B - <<'PY'
from pathlib import Path
import hashlib, json, os, subprocess, datetime
root = Path('/home/charl/Moriarty/.worktrees/s01-audit-start')
os.chdir(root)
def digest(p):
    with p.open('rb') as f: return hashlib.file_digest(f, 'sha256').hexdigest()
exporter = root / 'scripts/export_s02_candidate_a_integrated.py'
if digest(exporter) != '51028d697cd9be91ebfa97b5d7d6f69127ca700fd47313648cb4138877dddcd5':
    raise SystemExit('Exporter changed')
from scripts.export_s02_candidate_a_integrated import required_sources
names = sorted(required_sources(root))
if len(names) != 121: raise SystemExit('Expected complete 121-source closure')
sources = {name: digest(root / name) for name in names}
prior = [root / '.superpowers/sdd/a4-task6-resumption-20260906/fresh97/terminal.json',
         root / '.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/full/terminal.json']
groups = []
for path in prior:
    terminal = json.loads(path.read_text())
    for item in terminal.get('commandLifecycles', terminal.get('lifecycles', [])):
        if not item['cleanup']['complete']: raise SystemExit('Prior cleanup incomplete')
        pgid = item['ownedPgid']
        if pgid is not None:
            try: os.killpg(pgid, 0)
            except ProcessLookupError: pass
            else: raise SystemExit('Prior group present; root must resolve identity')
            groups.append(pgid)
plan = root / 'docs/superpowers/plans/2026-09-06-candidate-a-final-source-parser-capture.md'
admission = root / '.superpowers/sdd/a4-task6-resumption-20260906/root-admission114.json'
if digest(admission) != '2b112e8273b7f2a1b12adf9cc1404dc7c3da5fb933e14a972fc12f35519aba08':
    raise SystemExit('Preliminary admission changed')
bindings = prior + [plan, admission,
    root / '.superpowers/sdd/a4-task6-resumption-20260906/fresh97/outer-tool-receipt.json',
    root / '.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/transport/full/terminal.json',
    root / '.superpowers/sdd/a4-producer-dispatch.json',
    root / '.superpowers/sdd/a5-factoring-receipts/dispatch.json',
    root / '.superpowers/sdd/a5-factoring-receipts/tool-store/manifest.json',
    root / 'evidence/s02-candidate-a-completion/a4/native-resources/runner.py',
    root / 'scripts/run_s02_candidate_a_factoring_pilot.py']
record = {'rootDecision': 'Authorize exactly one final-source aggregate parse under this reviewed plan',
    'createdAt': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'actualDispatchHead': subprocess.check_output(['git','rev-parse','HEAD'], text=True).strip(),
    'sources': sources, 'sourceCount': 121, 'priorOwnedGroupsAbsent': groups,
    'bindings': {str(p): digest(p) for p in bindings},
    'outerWallSeconds': 900, 'nodeHeapMiB': 4096, 'parserAcceptanceBytes': 134217728,
    'scope': 'parse evidence and later pilot-only structural inspection; no native pilot or final A4 acceptance'}
destination = root / '.superpowers/sdd/a4-final-source-parser-dispatch-20260906.json'
with destination.open('x') as f: json.dump(record, f, indent=2); f.write('\n')
print(json.dumps({'dispatch': str(destination), 'sha256': digest(destination), 'sourceCount': 121}))
PY
```

The preceding manual root checks are mandatory; this small data command is not a substitute for their review. Recheck the frozen 121 source hashes, six fixed pins, plan hash and actual HEAD immediately before launch. A drift blocks the command rather than rewriting the dispatch.

## Task FP002: Execute the one existing capture stack

**Files:** Only the new destinations above. No executable source is created or edited.

- [ ] From the exact worktree cwd, invoke this command once through `tools.exec_command` with `yield_time_ms:1000`, `max_output_tokens:2500`. The single line is the entire native capture command:

```sh
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-final-source-aggregate-parse-20260906-parent-cache /home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-candidate-a-completion/a4/native-resources/runner.py --receipt-dir /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-final-source-aggregate-parse-20260906 --inner-receipt /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-producer-receipts/task6-final-source-aggregate-parse-20260906/receipt.json --wall-seconds 900 --cwd /home/charl/Moriarty/.worktrees/s01-audit-start -- /home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-producer-receipts/task6-final-source-aggregate-parse-20260906/python-cache /home/charl/Moriarty/.worktrees/s01-audit-start/scripts/record_s02_candidate_a_integrated.py --stage task6-final-source-aggregate-parse-20260906 -- /home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node --max-old-space-size=4096 /home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js parse specs/quint/s02/candidate_a_integrated_wrappers_typecheck.qnt --out /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-producer-receipts/task6-final-source-aggregate-parse-20260906/aggregate-ir.json
```

RH002 creates its own new process group around `/usr/bin/time -v -o OUTER/resources.txt` and the unchanged inner recorder. The inner Node argv stays exactly as supplied because its first item is the absolute Node path. The recorder clears inherited NODE_OPTIONS, so the explicit Node option carries the heap setting. Outer Python uses optimization zero; RH002 removes inherited Python option variables, and the inner explicit `-B`/cache arguments preserve the existing recorder's no-bytecode contract.

- [ ] Before requesting each next tool response, save the exact original response in a fresh sequential `response-000.json`, `response-001.json`, etc. under the transport directory. Preserve exact exec arguments in `command.json`; poll only the actual returned session ID with `tools.write_stdin`, empty chars, `yield_time_ms:10000`, `max_output_tokens:2500`. Do not start another exec to replace that session. When an actual `exit_code` arrives, save `terminal.json` with the exact command/cwd, original ordered `toolResponses`, and `wrapperExitCode` equal to that actual exit. No absent terminal is synthesized. These transport writes remain outside both runner-owned receipt directories.
- [ ] On transport interruption, retain the last authentic session/response and any original process listing used by root to establish ownership. The existing RH002 runner does not persist its PGID until its terminal and has no recorder-level signal handler. Do not externally terminate its controlling process as a routine stop mechanism; let its owned 900-second timeout/cleanup finish. Host loss or external termination may leave missing ownership/terminal records and requires explicit root takeover, with no retry supplied here.

Expected successful capture: actual tool exit 0; RH002 `actual_exit=0`, `return_code=0`, `eligible=true`; inner recorder and Node parse exits 0; empty inner stdout/stderr; original IR exists; full 121 before/after source snapshots and complete runtime endpoint maps remain stable. These are expected predicates, not results already obtained.

## Task FP003: Independently admit the original parser IR

**Interfaces:** Existing exporter `bounded_load(path, PARSER_LIMIT)` performs strict duplicate-key/nonfinite/EOF JSON decoding. `fields(parsed, expected, label)` checks exact top-level fields; `Types(parsed)` rejects duplicate typedef names. Calling these data interfaces does not call `parser_types` or launch Quint.

- [ ] First establish that all writers are gone and intake is eligible. Require authentic outer tool zero, RH002 zero/eligible, complete unforced cleanup with no errors or remaining group, no timeout/launch/monitor error, unchanged outer source/interpreter/time pins, and complete positive-RSS GNU-time resource data with exact command/23-field binding. GNU-time measures the entire recorder invocation; report that scope rather than parser-only timing or a hard total-RSS bound.
- [ ] Require inner `exit_code=recorder_exit_code=0`, `source_stable=runtime_stable=true`, both missing-source lists empty, and sourceCommit/sourceCommitAfter equal the separate root dispatch HEAD. Both 121-entry source maps must equal the root freeze. Rehash every original before/after source copy against those maps. Runtime-before/runtime-after and shared-runtime-before/after must agree with the admitted manifests and original producer/shared-dispatch pins; retain the complete maps and original runtime-helper copy. Do not relabel producer base386bf0a or bootstrap900bb20 as current HEAD.
- [ ] Check exact inner requested/executed Node parse argv, including heap flag and fresh `--out` path. Require empty original inner stdout/stderr and exact their receipt hashes. The recorder's `artifacts` must be `{}`: it indexes only `*.itf.json`. Root separately records raw `aggregate-ir.json` bytes and SHA-256; never invent an ITF artifact entry or mutate the receipt. Pin all original inner/outer files, both source-copy trees, root dispatch and transport files in the external root admission.
- [ ] After those complete source/runtime/terminal gates, use this existing-interface data check. It does not invoke a parser, and it leaves the original IR unchanged. Retain the actual checker stdout/error and tool terminal as additional root intake evidence; a failure blocks admission.

```sh
PYTHONOPTIMIZE=0 /home/charl/Moriarty/.venv/bin/python -B - <<'PY'
from pathlib import Path
import hashlib, json
root = Path('/home/charl/Moriarty/.worktrees/s01-audit-start')
source = root / 'scripts/export_s02_candidate_a_integrated.py'
if hashlib.sha256(source.read_bytes()).hexdigest() != '51028d697cd9be91ebfa97b5d7d6f69127ca700fd47313648cb4138877dddcd5':
    raise SystemExit('Exporter source changed')
from scripts.export_s02_candidate_a_integrated import bounded_load, fields, Types, PARSER_LIMIT, file_digest, require
raw = root / '.superpowers/sdd/a4-producer-receipts/task6-final-source-aggregate-parse-20260906/aggregate-ir.json'
require(raw.is_file() and not raw.is_symlink(), 'original parser IR regular file')
before = file_digest(raw)
parsed = bounded_load(raw, PARSER_LIMIT)
fields(parsed, ('stage', 'warnings', 'modules', 'table', 'errors'), 'parser IR')
require(parsed['stage'] == 'parsing' and parsed['warnings'] == [] and parsed['errors'] == [], 'successful parse stage')
require(type(parsed['modules']) is list, 'parser modules')
types = Types(parsed)
required = {'A4Event', 'AAuthorityExecution', 'A4Arguments', 'A4Command', 'A4Computation', 'AProgram', 'AState'}
require(len(types.types) == 122 and required <= set(types.types), '122 unique typedefs and required seven')
require(file_digest(raw) == before, 'original IR changed during intake')
print(json.dumps({'scope': 'parser data validation only; source/runtime/terminal root admission separate',
    'path': str(raw), 'sha256': before, 'bytes': raw.stat().st_size, 'uniqueTypedefs': 122,
    'requiredTypes': sorted(required), 'types': sorted(types.types)}))
PY
```

- [ ] Only after independent PASS on every gate, create `.superpowers/sdd/a4-final-source-parser-admission-20260906.json` exclusively. Required fields: root decision/scope and observation time; actual dispatch HEAD and dispatch SHA; exact 121 source pins; original inner receipt and both closure hashes; all original inner/outer/transport file hashes; original IR path/bytes/SHA; actual child/recorder/RH002/tool exits; source/runtime stability and admitted shared archive references; exact strict-IR/122/seven-type predicates and their original data-check response pins. Label this **final-source parser evidence for pilot-only structural inspection**, with `nativePilotAccepted:false`, `full78ExporterAccepted:false`, and `finalA4Accepted:false`. Independent review, not the data check alone, supplies this admission.

On any timeout, nonzero exit, missing inner receipt, missing after snapshot, oversized/invalid IR, tool/source drift, cleanup uncertainty or failed data predicate: preserve all originals and stop. RH002 can kill the inner recorder before its post-command snapshots/receipt exist; those missing files remain missing. RH002 also computes sidecar hashes even when cleanup is incomplete: preserve those reported hashes as original statements, but do not admit them as settled bytes while a writer is uncertain. No new complete index or success receipt is fabricated. Existing capture suffices for a successful parse admission and honest failed-attempt preservation; it does not promise complete inner before/after evidence on every failure path.

## Later pilot use and boundaries

The original admitted raw IR is later loaded unchanged through `bounded_load(original_ir_path, PARSER_LIMIT)` and passed to `Types(parsed)`. For each separately authorized and source/tool-matched genuine pilot014/044, use existing `shards()[14]` or `shards()[44]` and exhaust `raw_events(original_itf_path, row, original_itf_sha256, types, initial_hashes)` to strict EOF. Keep the actual ITF path/hash and the row's canonical future input-path mapping explicit in that pilot's structural receipt; do not reserialize the parser IR or substitute expected checker states. Require the resulting 28/27 event counts and all existing raw metadata/counter/type/domain/computation checks. Exhaustion matters because final metadata and EOF checks run after the final yielded event.

This is structural validation of raw carriers under the admitted type IR, not independent semantic correspondence, model checking, or a two-case export package. No pilot command or `raw_events` invocation is dispatched by this parser-only plan. The full exporter still requires all 78 raw inputs/receipts before `export_shards`; it must invoke its own `parser_types` with a fresh required `--parser-receipts` directory for stage-cases and again for seal-manifest. This capture is not a cache, replacement terminal in that API's schema, allow-subset route, or waiver of either later parse. Full78, final115/native, independent checker/package, Council and architecture decisions remain separate.

## Source-only handoff

- [x] Inspected actual recorder/RH002/exporter interfaces and current small-file pins, including the 114 admission and prior terminal evidence. No helper was imported or executed while planning.
- [ ] Root obtains independent review of this exact plan SHA before data freeze/dispatch. No source feature, wrapper, helper mutation, test, parser/native run, commit, or permission prompt is part of this plan-writing task.
