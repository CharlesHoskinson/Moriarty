# A4 public run-phase diagnostic design: independent review

**Spec verdict: PASS. Quality verdict: PASS for design admission only.** No concrete design contradiction found. The exact-plan gate remains open; no implementation, materialization or native execution is authorized by this review.

Reviewed `docs/superpowers/specs/2026-09-06-candidate-a-run-phase-diagnostic-design.md`, SHA-256 `e969fc1004d1cfb4e9928604d52acc6c863c996bad58284b0ce76cc02c794e4a`, against the pinned CLI/compiler/helpers/Rust wrapper, prior observer and unchanged capture interfaces.

- **Source grounding and changed question** (design9–46): correct. Real CLI run chains load/parse/typecheck/runSimulator/outputResult, without the compile/full-flatten branch. toExpr parses/resolves expressions. Rust request serialization precedes spawn; that ordering does not locate the original OOM or exclude later Rust/output work. Matching historical aggregate/typecheck evidence supports coverage but not the failed invocation's phase. The proposed question concerns a new observed public interval.
- **Observer preservation** (50–71): the five fixed hooks and reuse of non-async mechanics preserve receiver/args, exact return/Promise identity and thrown/rejected values. Real Either Left skips downstream callbacks; outputResult's process exit need not produce a completion marker. Bounded synchronous writes/classification/failure74 remain separate from OOM/signals. No direct stage invocation or yargs reconstruction is proposed.
- **Fresh controls** (108–132): fresh identity/throw/reject/Left/receiver/argument and all listed sink faults are required; old compile controls do not transfer. Genuine deterministic success/transition-violation direct/observed pairs check actual runSimulator Right/Left chains. The exact plan must enumerate permissible timestamp/duration/path differences and compare every remaining relevant field and complete state sequence. No arbitrary normalization or inferred equivalence is permitted.
- **Capture and provenance** (75–104,136–151): unchanged RH002 and integrated recorder retain group ownership and honest launcher argv; their receipts cannot satisfy canonical native_command acceptance. Separate diagnostic code/control/invocation freezes close the gap in the121-source map. Missing-inner outcomes require independent settled endpoint capture, not fabricated inner receipts or Node exits. Fresh paths, no heap/verbosity change,900-second outer bound, serialization with A5 and authentic response-before-next-poll retention are correctly scoped.
- **Meaning of output** (26–30,139–156): incomplete runSimulator covers expression resolution, discovery, request serialization, Rust execution and output/ITF work. It cannot identify an inner operation or the historical OOM phase. Observer failure or malformed/missing trace supplies no phase conclusion. Any ITF remains diagnostic; no retry,044,all78,final115,solver/Council/H1 promotion follows.

The following are concrete checks for the later exact-plan review, already required by this design's full validation/closure contract; they are not design approval to execute:

1. Before loading real CLI, explicitly restore `process.argv` to the validated effective argv, as the predecessor launcher does at its line35. Preserve both actual launcher argv and effective CLI argv, and verify execPath/execArgv/cwd. Checking an invocation JSON alone cannot make yargs consume those arguments.
2. Keep the predecessor's Promise observation callbacks non-forwarding: return undefined from both handlers so the unused observation Promise does not become a new unhandled rejection; retain original Promise identity. No new global handlers.
3. Enumerate tiny Quint fixtures as frozen originals alongside JS controls. Capture root-owned before endpoints for all extra code/control/invocation inputs and the complete runtime before dispatch, and original after endpoints only once writers are confirmed absent. A missing inner receipt must not leave the before provenance inferred retrospectively.
4. Specify exact Right/Left event sequences, stage/file ownership, comparison predicates, code/plan/runtime pins, controls and full stop gates. An unexpected failed control must block dependent launches; root admission must precede the single full slot. Reconcile source-archive/dispatch ordering without self-hash cycles. Preserve failed/missing transport honestly.

Read-only pin verification actual chunk **62238a**, exit **0**, matched ten selected design/source/evidence pins. Exact invocation/result follows. No helper/project import, native/mock/test run, original/source/runtime edit or commit occurred. No archive or full runtime-tree audit was repeated.

```json
{
  "args": {
    "cmd": "python3 -B - <<'PY'\nfrom pathlib import Path\nimport hashlib,json\nr=Path('/home/charl/Moriarty/.worktrees/s01-audit-start')\npkg=Path('/home/charl/.npm-global/lib/node_modules/@informalsystems/quint')\npins={\n'docs/superpowers/specs/2026-09-06-candidate-a-run-phase-diagnostic-design.md':'e969fc1004d1cfb4e9928604d52acc6c863c996bad58284b0ce76cc02c794e4a',\n'.superpowers/sdd/a4-pilot014-failure-root-admission-20260906.json':'f0676a056c57464478954ecc9ae622acfee14102bfcec8079b6900584dc3f758',\n'.superpowers/sdd/a4-pilot014-next-diagnostic-source-proposal-20260906.md':'4258fbfc7cb9b32574f4cda14ad44829dd323dbb50150abf3b8d171cc9cf495a',\n'.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/observer.cjs':'59f706006a73c795c303d84e7c1e3368acd1cf8a7547bfb0d46821f0d69fbd49',\n'scripts/record_s02_candidate_a_integrated.py':'dc30b764e2603dfe3c39ef6e2063a7082d116dc9d3eb9a012072f468beb8f8e9',\n'evidence/s02-candidate-a-completion/a4/native-resources/runner.py':'d8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d'}\ninstalled={'dist/src/cli.js':'ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501','dist/src/cliCommands.js':'b18672d656782aefde254fe1021dc13fdffdcf8f4ad8709e3126fb5fa293362a','dist/src/cliHelpers.js':'4ad6b6364d2a5d07389f2518aa80013bfd01dbdbab6201b53fd6b042eb8a91dc','dist/src/rust/commandWrapper.js':'1f26d3f1c31529572b5be32276d2d94c19a3d0298dbf444891802f744e337e1c'}\nfor base,items in [(r,pins),(pkg,installed)]:\n for name,expected in items.items():\n  if hashlib.sha256((base/name).read_bytes()).hexdigest()!=expected:raise ValueError('pin mismatch '+name)\nprint(json.dumps({'ok':True,'reviewedDesignSha256':pins[next(iter(pins))],'sourceAndEvidencePinsMatched':10,'nativeOrHelperExecuted':False,'archivedCodeImported':False}))\nPY",
    "workdir": "/home/charl/Moriarty/.worktrees/s01-audit-start",
    "max_output_tokens": 500
  },
  "result": {
    "chunk_id": "62238a",
    "wall_time_seconds": 0.000004496,
    "exit_code": 0,
    "original_token_count": 52,
    "output": "{\"ok\": true, \"reviewedDesignSha256\": \"e969fc1004d1cfb4e9928604d52acc6c863c996bad58284b0ce76cc02c794e4a\", \"sourceAndEvidencePinsMatched\": 10, \"nativeOrHelperExecuted\": false, \"archivedCodeImported\": false}\n"
  }
}
```

