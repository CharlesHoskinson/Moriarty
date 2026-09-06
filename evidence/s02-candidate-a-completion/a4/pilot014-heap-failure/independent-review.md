PASS

Spec and evidence-preservation review: PASS for the frozen failed014 compact package, with the separately retained HEAD-observation supplement. This admits preservation only. No pilot acceptance, case044 dispatch, structural intake, retry, native execution, or H1 conclusion follows.

Reviewed all of `audit.py` and `build_archive.py`, README, preservation report, package indices, and actual build/audit/optimization-rejection receipts. The standalone audit uses only standard-library data operations and AST literal inspection; it does not import or execute archived code, access original worktree paths, or read external runtime archives. Its non-assert optimization guard rejects optimized execution before archive reads.

Independent standalone invocation from `/tmp`, with `PYTHONOPTIMIZE=0 python3 -B`, returned actual chunk `95edc0`, exit 0. Independent data comparison returned actual chunk `425218`, exit 0. All 325 regular archived originals match retained live bytes: 303 indexed originals + 21 proposal additions + the proposal; 14,842,431 uncompressed bytes. All 11 artifact-index entries match the 12-file package (self excluded). The archive, index, and support-file membership are closed. Exact commands, actual returned objects, and all 12 package hashes are preserved below.

The audit verifies both sets of 121 source copies (242 files), the recursive 104-Quint/17-Python closure, dispatch and parser-admission source bindings, 8,102 recorded runtime pins and four tree inventories, executable/version bindings, all 37 original transport responses, requested and executed argv, RH002 sidecars and original inner receipt, and authentic failure outcomes. Recorded Node exit -6, recorder/RH002/actual outer 134 remain distinct. Original 1,510-byte JavaScript heap-OOM stderr, empty stdout, no ITF, eligible=false, timed_out=false, and unforced complete cleanup are preserved. GNU-time 5:56.80 and maximum RSS 4,371,180 KiB concern the whole recorded invocation. No explicit heap override or internal failure phase is established. Requested invariants and witnesses did not acquire passing evidence.

The independent original audit's truncated chunk `62b47c` and the separate complete indexing chunk `5b70df` remain exact; no missing output is reconstructed. Author build `91caa6`/0 and audit `367a6d`/0 agree with their separately labeled derived summaries. Deliberate optimization rejection `83fa1b`/1 is preserved as such.

Historical native source HEAD is `08e426c7163880b9312f1f1f029a4dde9d0e7593`. Package field `packagingHeadObserved=4006244885c7cf34562fe54cbb76856142b0d244` and chunk `f8db22` refer only to the earlier observation before builder preparation, not the later build or freeze. The authentic supplement `.superpowers/sdd/a4-pilot014-package-head-observation-tool-20260906.json`, SHA256 `8ef486b32b470c6aa8b429dbb7cf551879c9c2301bc932e243a8c47cff9d3b82`, explicitly records this distinction and is outside the frozen archive; root must retain it with adoption. Package bytes were not changed.

External runtime archives and the parser archive/38,527,047-byte IR were not rehashed or duplicated. Their recorded hash references and archived manifests/parser index are checked; this review relies on their prior independent admissions for external byte identity. Recorded endpoint agreement does not prove continuous historical immutability. Original no-ITF/no-044 and cleanup observations are not a new present-day process or filesystem claim.

Key frozen hashes:
- Archive: `31023b784fa91b797bd604269cd9e09f4def7d11f8cb8d9075999f2a8c8dcec1`.
- Original index: `bd025f3ea24847ace97f0cdd025a262dd2382da0984a1776e21982dfbd1fceb3`.
- Audit source: `072a920e7028d8d01b10195a2bdb985f27ff5a355ef6304bdcfa60246591cfdb`.
- Artifact index: `7d61bb222cb374748bd1f3e1483a4147aa59b608b0677f09b1020b7bac7fff86`.

Exact independent tool receipts (new preservation encoding of actual argument/return objects, not original wire bytes):

```json
{
  "scope": "Independent source inspection, standalone offline audit, and retained-original comparison only; no native or archived code execution",
  "audit": {
    "args": {
      "cmd": "PYTHONOPTIMIZE=0 python3 -B /home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-candidate-a-completion/a4/pilot014-heap-failure/audit.py",
      "workdir": "/tmp",
      "yield_time_ms": 1000,
      "max_output_tokens": 2000
    },
    "result": {
      "chunk_id": "95edc0",
      "wall_time_seconds": 0.028370219,
      "exit_code": 0,
      "original_token_count": 153,
      "output": "{\"ok\": true, \"scope\": \"offline failed014 preservation; recorded external dependencies only\", \"members\": 325, \"sourceCopies\": 242, \"sources\": 121, \"runtimePins\": 8102, \"runtimeTrees\": 4, \"transportResponses\": 37, \"actualExits\": {\"Node\": -6, \"recorder\": 134, \"RH002\": 134, \"outerTool\": 134}, \"failure\": \"JavaScript heap out of memory\", \"noITF\": true, \"eligible\": false, \"timedOut\": false, \"internalPhase\": \"unknown\", \"explicitNodeHeapFlag\": false, \"pilotAccepted\": false, \"case044Authorized\": false, \"structuralIntakePerformed\": false, \"externalArchivesOrRuntimeRehashed\": false, \"archivedCodeExecuted\": false}\n"
    }
  },
  "comparison": {
    "args": {
      "cmd": "PYTHONOPTIMIZE=0 python3 -B - <<'PY'\nimport hashlib,json,tarfile\nfrom pathlib import Path\nroot=Path('/home/charl/Moriarty/.worktrees/s01-audit-start')\np=root/'evidence/s02-candidate-a-completion/a4/pilot014-heap-failure'\nsha=lambda b:hashlib.sha256(b).hexdigest()\nindex=json.loads((p/'index.json').read_bytes())\nrows={r['path']:r for r in index['members']}\nassert len(rows)==len(index['members'])==325\nseen=set(); total=0\nwith tarfile.open(p/'original-evidence.tar.gz','r:gz') as tar:\n    for member in tar:\n        assert member.isfile() and member.name not in seen\n        seen.add(member.name)\n        data=tar.extractfile(member).read()\n        live=root/member.name\n        assert live.is_file() and not live.is_symlink()\n        assert live.read_bytes()==data\n        assert rows[member.name]=={'path':member.name,'bytes':len(data),'sha256':sha(data)}\n        total+=len(data)\nassert seen==set(rows) and total==14842431\nartifact=json.loads((p/'artifact-index.json').read_bytes())\nassert artifact['selfExcluded'] is True and artifact['laterReviewsExcluded'] is True\nassert len(artifact['files'])==11\nassert {r['path'] for r in artifact['files']}=={x.name for x in p.iterdir()}-{'artifact-index.json'}\nfor row in artifact['files']:\n    data=(p/row['path']).read_bytes()\n    assert len(data)==row['bytes'] and sha(data)==row['sha256']\nfor kind,receipt,chunk in [('build','build-launch-tool-receipt.json','91caa6'),('audit','audit-tool-receipt.json','367a6d')]:\n    tool=json.loads((p/receipt).read_bytes())\n    report=json.loads((p/(kind+'-report.json')).read_bytes())\n    assert report['actualToolChunk']==tool['result']['chunk_id']==chunk\n    assert report['actualToolExit']==tool['result']['exit_code']==0\n    assert report['output']==json.loads(tool['result']['output'])\nreject=json.loads((p/'optimization-rejection-tool-receipt.json').read_bytes())\nassert reject['result']['chunk_id']=='83fa1b' and reject['result']['exit_code']==1\nassert 'Archive audit requires optimization disabled' in reject['result']['output']\nobservation=root/'.superpowers/sdd/a4-pilot014-package-head-observation-tool-20260906.json'\nob=observation.read_bytes()\nassert sha(ob)=='8ef486b32b470c6aa8b429dbb7cf551879c9c2301bc932e243a8c47cff9d3b82'\no=json.loads(ob)\nassert o['result']['chunk_id']=='f8db22' and o['result']['exit_code']==0\nassert o['result']['output'].strip()==index['packagingHeadObserved']=='4006244885c7cf34562fe54cbb76856142b0d244'\nprint(json.dumps({'ok':True,'liveOriginalMembersCompared':len(seen),'originalBytes':total,'packageFiles':{x.name:{'bytes':x.stat().st_size,'sha256':sha(x.read_bytes())} for x in sorted(p.iterdir())},'authorBuildChunk':'91caa6','authorAuditChunk':'367a6d','expectedOptimizationRejectionChunk':'83fa1b','headObservation':{'path':str(observation),'sha256':sha(ob),'chunk':'f8db22','scope':'earlier observation before builder preparation, not later build/freeze HEAD'},'nativeOrArchivedCodeExecuted':False,'externalRuntimeRehashed':False},sort_keys=True))\nPY",
      "workdir": "/tmp",
      "yield_time_ms": 1000,
      "max_output_tokens": 3000
    },
    "result": {
      "chunk_id": "425218",
      "wall_time_seconds": 0.000008873,
      "exit_code": 0,
      "original_token_count": 510,
      "output": "{\"authorAuditChunk\": \"367a6d\", \"authorBuildChunk\": \"91caa6\", \"expectedOptimizationRejectionChunk\": \"83fa1b\", \"externalRuntimeRehashed\": false, \"headObservation\": {\"chunk\": \"f8db22\", \"path\": \"/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-pilot014-package-head-observation-tool-20260906.json\", \"scope\": \"earlier observation before builder preparation, not later build/freeze HEAD\", \"sha256\": \"8ef486b32b470c6aa8b429dbb7cf551879c9c2301bc932e243a8c47cff9d3b82\"}, \"liveOriginalMembersCompared\": 325, \"nativeOrArchivedCodeExecuted\": false, \"ok\": true, \"originalBytes\": 14842431, \"packageFiles\": {\"README.md\": {\"bytes\": 2414, \"sha256\": \"3b6c49383518abcb67bb1a9bc50d37cd956dfc300a1902ed0c175667597bf69e\"}, \"artifact-index.json\": {\"bytes\": 1862, \"sha256\": \"7d61bb222cb374748bd1f3e1483a4147aa59b608b0677f09b1020b7bac7fff86\"}, \"audit-report.json\": {\"bytes\": 858, \"sha256\": \"7ce51eaf86c1439fc1f4779491729b0262253bc114c1563113bde854930a0d88\"}, \"audit-tool-receipt.json\": {\"bytes\": 1185, \"sha256\": \"304af0a4017bdfa6c176e1de65311844f8e75e0691152609e6e4f328a5c017a5\"}, \"audit.py\": {\"bytes\": 24526, \"sha256\": \"072a920e7028d8d01b10195a2bdb985f27ff5a355ef6304bdcfa60246591cfdb\"}, \"build-launch-tool-receipt.json\": {\"bytes\": 823, \"sha256\": \"395b8b225c676eafbe393ecbd939e6f5da482aec0c7e22cc88334c2a3ab33550\"}, \"build-report.json\": {\"bytes\": 471, \"sha256\": \"d2eb0ffa75ac56c0a56218e0bdd63348077e0641cf4a58ffa4ec7b65e18b4672\"}, \"build_archive.py\": {\"bytes\": 4950, \"sha256\": \"75033bb9dd40ab55c5bf55cd7bf2d72d3b13e96b9d4afe270ef9c6cbbfecf65b\"}, \"index.json\": {\"bytes\": 79186, \"sha256\": \"bd025f3ea24847ace97f0cdd025a262dd2382da0984a1776e21982dfbd1fceb3\"}, \"optimization-rejection-tool-receipt.json\": {\"bytes\": 831, \"sha256\": \"ba13c5f9c456f9bee6d566babbb9639ff9b504e677450dcc553d9b5218bc756c\"}, \"original-evidence.tar.gz\": {\"bytes\": 2590403, \"sha256\": \"31023b784fa91b797bd604269cd9e09f4def7d11f8cb8d9075999f2a8c8dcec1\"}, \"preservation-report.md\": {\"bytes\": 2494, \"sha256\": \"21b086ba7ee5ecebe1da6255aa4041e162a9851d5796c08e6bedcc2f70b2f6eb\"}}}\n"
    }
  }
}
```

