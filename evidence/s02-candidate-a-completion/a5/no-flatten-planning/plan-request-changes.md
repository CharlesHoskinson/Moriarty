REQUEST_CHANGES
# A5 no-flatten implementation plan: independent review

Reviewed `docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-diagnostic.md` SHA-256 `c8df95776d75b2702d1d41d78f66ed3cb7ceea95e4918802c7192b108885e59d` against adopted design `f8b6abf816fea5ab58bbe217f847b6eb8e3dbaa1a9e8aaa5fce0cc15c1b8c061`.

**Spec verdict: REQUEST CHANGES. Quality verdict: REQUEST CHANGES.** Two narrow source/provenance defects must be corrected and re-reviewed before data preparation or native dispatch. No implementation or native/helper execution occurred.

## NF-R1 — incomplete fixed source/archive closure

NF001 describes the212-path union as exhaustive (line262), but NF002's view-copy verification reads `ROOT/row['original']` for all24 view-manifest rows. Three actual original inputs are absent from every fixed group:

- `specs/quint/s02/factored_verification/candidate_a_funding_pilot.qnt`
- `specs/quint/s02/factored_verification/candidate_a_funding_pilot_f.qnt`
- `specs/quint/s02/factored_verification/candidate_a_funding_pilot_test.qnt`

The54 group contains24 copied views and the30 earlier task2 originals, which do not include these later pilot modules. Their bytes influence the `copy==original` checks but are not in `expected`, not rechecked by the final expected-pin loop, and not retained in the endpoint source archive. This contradicts the claimed exhaustive byte closure.

Add these three original file hashes as support pins and update the support/union counts consistently (38 support entries,215 unique fixed paths), or supply those reads from an explicitly pinned and archived original source. Keep the54 identity group unchanged and retain exact original-byte comparisons. Do not regenerate the view. Source-only checking found all currently declared212 hashes valid; this is omitted coverage, not a mismatch among declared pins.

## NF-R2 — failed intake can promote an unvalidated receipt exit

NF005 sets `result['actualCompilerExit']=receipt['exit_code']` before validating the original inner receipt hash, then its catch block at lines794–803 reads any present receipt and unconditionally copies an integer `exit_code` into `actualCompilerExit`. The catch can be reached because the receipt hash, source, argv or other success binding failed. A present JSON file alone is not enough to label that value an authentic exit for the intended compiler invocation.

In failed-preservation intake, keep `actualCompilerExit` null unless RH002 reports the original receipt present, its original receipt SHA matches the settled file, and the receipt's requested/executed compiler argv and cwd match this dispatch. Preserve any observed untrusted exit separately with its failed-binding reason. Ensure success-branch assignment cannot survive a failed prerequisite hash/argv validation as an authenticated value. Preserve malformed or mismatched originals; do not repair a receipt or infer Node exit from RH002's time-wrapped return code.

## Other reviewed interfaces and boundaries

All declared pin groups currently match live originals:121 integrated sources,54 A5 view/original paths,4 alias files,35 support entries; union212. The integrated map equals the admitted original parser closure. Seven complete anchored Python code blocks pass syntax compilation without executing their bodies. All native commands remain specified only.

The two fixed compiler argv arrays and historical comparison preserve the selected roots/main/invariant/default init/step, adding explicit4096MiB Node argv and `--flatten=false` while retaining original stdout as JSON. Pinned CLI/serializer/ToIrListener source supports the intended compiling-stage fields, sole main module, imports, type application/record structure and generated selections. Tiny comparison is deliberately structural against the old flattened result, not whole-output equality. Empirical capability still requires the one admitted tiny invocation.

The planned RH002/integrated-recording composition, sanitization/cache semantics, source-before and separate dispatch archive ordering, full independent runtime endpoints, current versus bootstrap bases, original incremental transport, scheduler predecessor cleanup and separate tiny/full admission paths are otherwise coherent. Native timeouts may lack inner snapshots/receipt, and RH002's bound includes recorder setup. These remain failed diagnostic preservation with no phase localization, causal subtraction, solver input, H1 or A5-completion promotion.

The original public no-flatten route remains unexecuted under this plan. A failed tiny predicate closes the gate. Actual control/intake results must not be silently weakened or retried.

## Actual source-only checks and reviewer extraction correction

Actual check51dbb9/exit0 verified the212 pins and identified the three uncovered original paths. Its initial fence extraction regex was unanchored and truncated Python block0 at an embedded Markdown-fence string, so it reported an extraction-induced unterminated-string SyntaxError. This was not a syntax failure in the actual plan. The corrected column-anchored extraction checked all seven full blocks; actual5eadff/exit0 reports each syntax valid. No helper imports or block bodies ran. Both exact invocations/responses are preserved below; no code was materialized.

### Initial pin/closure and unanchored syntax extraction

```json
{
  "args": {
    "cmd": "python3 -B - <<'PY'\nfrom pathlib import Path\nimport hashlib,json,re\nr=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');p=r/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-diagnostic.md';s=p.read_text();f=chr(96)*3\ngroups=json.loads(s.split('<!-- noflat-pins -->\\n'+f+'json\\n')[1].split('\\n'+f)[0]);union={}\nfor name,m in groups.items():\n for n,h in m.items():\n  if n in union and union[n]!=h:raise ValueError('conflict '+n)\n  union[n]=h\nmismatches=[]\nfor n,h in union.items():\n with Path(n).open('rb') as stream:actual=hashlib.file_digest(stream,'sha256').hexdigest()\n if actual!=h:mismatches.append(n)\nview=json.loads((r/'.superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/view-manifest.json').read_text())\nuncovered=[str(r/v['original']) for v in view['files'] if str(r/v['original']) not in union]\nclosure=json.loads((r/'.superpowers/sdd/a4-producer-receipts/task6-final-source-aggregate-parse-20260906/before/closure.json').read_text())['sources']\nif {str(r/k):v for k,v in closure.items()}!=groups['integrated121']:raise ValueError('integrated group')\nsyntax=[]\nfor i,b in enumerate(re.findall(f+'python\\n(.*?)'+f,s,re.S)):\n try:compile(b,'<plan-block-%d>'%i,'exec');syntax.append({'block':i,'syntax':'ok'})\n except SyntaxError as e:syntax.append({'block':i,'error':str(e)})\nprint(json.dumps({'planSha256':hashlib.sha256(p.read_bytes()).hexdigest(),'groupCounts':{k:len(v) for k,v in groups.items()},'union':len(union),'mismatches':mismatches,'viewOriginalsReadButNotFixed':uncovered,'syntax':syntax,'helpersImported':False,'planBodiesExecuted':False},sort_keys=True))\nPY",
    "workdir": "/home/charl/Moriarty/.worktrees/s01-audit-start",
    "max_output_tokens": 1700
  },
  "result": {
    "chunk_id": "51dbb9",
    "wall_time_seconds": 0.000005567,
    "exit_code": 0,
    "original_token_count": 239,
    "output": "{\"groupCounts\": {\"a5viewAndOriginal54\": 54, \"alias4\": 4, \"integrated121\": 121, \"support\": 35}, \"helpersImported\": false, \"mismatches\": [], \"planBodiesExecuted\": false, \"planSha256\": \"c8df95776d75b2702d1d41d78f66ed3cb7ceea95e4918802c7192b108885e59d\", \"syntax\": [{\"block\": 0, \"error\": \"unterminated string literal (detected at line 65) (<plan-block-0>, line 65)\"}, {\"block\": 1, \"syntax\": \"ok\"}, {\"block\": 2, \"syntax\": \"ok\"}, {\"block\": 3, \"syntax\": \"ok\"}, {\"block\": 4, \"syntax\": \"ok\"}, {\"block\": 5, \"syntax\": \"ok\"}, {\"block\": 6, \"syntax\": \"ok\"}], \"union\": 212, \"viewOriginalsReadButNotFixed\": [\"/home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/factored_verification/candidate_a_funding_pilot.qnt\", \"/home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/factored_verification/candidate_a_funding_pilot_f.qnt\", \"/home/charl/Moriarty/.worktrees/s01-audit-start/specs/quint/s02/factored_verification/candidate_a_funding_pilot_test.qnt\"]}\n"
  }
}
```

### Corrected anchored syntax extraction

```json
{
  "args": {
    "cmd": "python3 -B - <<'PY'\nfrom pathlib import Path\nimport hashlib,json,re\np=Path('docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-diagnostic.md');s=p.read_text();f=chr(96)*3\nblocks=re.findall('^'+f+'python\\n(.*?)^'+f+'$',s,re.S|re.M)\nout=[]\nfor i,b in enumerate(blocks):\n try:compile(b,'<anchored-plan-block-%d>'%i,'exec');out.append({'block':i,'syntax':'ok','lines':len(b.splitlines())})\n except SyntaxError as e:out.append({'block':i,'error':str(e)})\nprint(json.dumps({'planSha256':hashlib.sha256(p.read_bytes()).hexdigest(),'anchoredPythonBlocks':out,'planBodiesExecuted':False}))\nPY",
    "workdir": "/home/charl/Moriarty/.worktrees/s01-audit-start",
    "max_output_tokens": 700
  },
  "result": {
    "chunk_id": "5eadff",
    "wall_time_seconds": 0.000005967,
    "exit_code": 0,
    "original_token_count": 111,
    "output": "{\"planSha256\": \"c8df95776d75b2702d1d41d78f66ed3cb7ceea95e4918802c7192b108885e59d\", \"anchoredPythonBlocks\": [{\"block\": 0, \"syntax\": \"ok\", \"lines\": 151}, {\"block\": 1, \"syntax\": \"ok\", \"lines\": 63}, {\"block\": 2, \"syntax\": \"ok\", \"lines\": 29}, {\"block\": 3, \"syntax\": \"ok\", \"lines\": 202}, {\"block\": 4, \"syntax\": \"ok\", \"lines\": 43}, {\"block\": 5, \"syntax\": \"ok\", \"lines\": 12}, {\"block\": 6, \"syntax\": \"ok\", \"lines\": 48}], \"planBodiesExecuted\": false}\n"
  }
}
```


