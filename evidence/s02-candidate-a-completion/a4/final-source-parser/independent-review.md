# A4 final-source parser package: independent review

**Spec verdict: PASS. Evidence-preservation verdict: PASS.** Reviewed `evidence/s02-candidate-a-completion/a4/final-source-parser/`. No package-evidence mismatch found. This is preservation of the admitted parser result, not native-pilot/full78/finalA4/H1 acceptance.

Read the entire audit/builder source, README, preservation report, archive index, artifact index and five original build/audit/rejection tool receipts. The audit rejects optimization before archive reading, uses only standard-library and packaged bytes, checks bounded canonical unique regular members and strict JSON, and uses AST literal inspection rather than executing archived source. It has no native subprocess, live group or external-runtime read. Builder source was inspected, not run.

Independent standalone audit from `/tmp`, `PYTHONOPTIMIZE=0 python3 -B <absolute audit.py>`: actual **a004db/exit0**. Verified290 archive members,242 original source copies, exact121-source closure,8,102 recorded runtime pins/four tree maps, original argv/transport/resource/admission bindings and raw38,527,047-byte IR with99 modules/122 unique typedefs/seven required types. Both old producer/runtime bases and parser source HEAD08e426c remain distinct from packaging HEAD4006244.

Independent live-byte comparison: actual **7bfabc/exit0**. Every290 archived original equals its retained live original:52,070,254bytes total, including the untouched parser IR. The archive is the exact263 original admission pins plus26 explicitly proposed additional originals and the reviewed proposal. Exact14 package files remain; all13 artifact-index entries match their current file lengths and hashes. Index self/later-review exclusions are preserved.

Primary pins:

| Artifact | SHA-256 |
| --- | --- |
| Original archive | `7233f85c03bfacf80d83d50a657a10f05a23d4863c157ecc81a4c37b13ee8119` |
| Original index | `19348e1089004e998e096b3c59dff2cbc7888a98a5b31fd7ae89db5dc6a00932` |
| Artifact index | `f1d813ec46583722c27c8c8b04e2f9131f670da2b626800ffce344fbb8893c4a` |
| Audit source | `a638058cb410594c53b26636a4a59d900be1a4ab8506117faf7f9421b937c977` |
| Preservation report | `21d69a0d465d043360b98df32995fb95f94dd021181909f7df0623c15ae3eb71` |

Authentic build launch d67bbd/session2692 and terminal c27173/exit0 remain separate; audit launch47f0f3/session81427 and terminal2b4a4e/exit0 remain separate. Their derived report objects agree with the original terminal output and chunk/exit fields. Deliberate optimization rejection999066/exit1 is preserved without rerunning it. Original parser session54358/terminal88c051, independent intake responses and root data-check7660c0 are present. The root81c702 interim-report refusal and other disclosed review/display failures remain original history; unavailable transcript/wire bytes are not reconstructed.

RH002 `inner_receipt_complete:false`, recorder `artifacts:{}`, Rust unsupported-version probe exit1, authentic parser/recorder/outer zero and scoped GNU-time metrics retain their meanings. No parser-specific CPU/RSS or total memory cap is invented. Both future exporter parser calls remain required. The package does not assert current live runtime/group state or continuous historical immutability.

One reviewer comparison initially assumed derived reports stored output keys at top level. Actual **e31341/exit1** raised `KeyError: 'ok'` at inline36 after original/member/artifact-pin comparisons. Inspection showed the correctly labeled reports nest native audit data under `output`. Only the reviewer data-check predicate was corrected to compare that object plus actual chunk/exit; the small comparison then passed. Both exact invocations/results follow. This was not a parser, package build or original-evidence failure.

No archived code/native/test/helper capture was executed; no large external runtime archive/tree was rehashed. No package original/source/runtime file was changed and no commit was made. Root owns final admission/commit.

## Standalone audit

```json
{
  "args": {
    "cmd": "PYTHONOPTIMIZE=0 python3 -B /home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-candidate-a-completion/a4/final-source-parser/audit.py",
    "workdir": "/tmp",
    "max_output_tokens": 1500
  },
  "result": {
    "chunk_id": "a004db",
    "wall_time_seconds": 1.105644135,
    "exit_code": 0,
    "original_token_count": 215,
    "output": "{\"ok\": true, \"scope\": \"offline preserved parser evidence; external runtime bindings only\", \"members\": 290, \"sourceCopies\": 242, \"sources\": 121, \"runtimePins\": 8102, \"runtimeTrees\": 4, \"irBytes\": 38527047, \"irSha256\": \"cbf57ad5a7f194ba62bf897553d3750ddba899190683ab8293cc633282886e1e\", \"modules\": 99, \"uniqueTypedefs\": 122, \"requiredTypes\": {\"A4Arguments\": \"candidate_a_integrated_observer\", \"A4Command\": \"candidate_a_integrated_observer\", \"A4Computation\": \"candidate_a_integrated_observer\", \"A4Event\": \"candidate_a_integrated_observer\", \"AAuthorityExecution\": \"candidate_a_authority_adapter\", \"AProgram\": \"candidate_a_types\", \"AState\": \"candidate_a_types\"}, \"actualOriginalOuterExit\": 0, \"archivedCodeExecuted\": false, \"currentRuntimeOrExternalArchivesRehashed\": false, \"nativePilotAccepted\": false, \"full78ExporterAccepted\": false, \"finalA4Accepted\": false}\n"
  }
}
```

## Initial independent comparison (reviewer schema error)

```json
{
  "args": {
    "cmd": "PYTHONOPTIMIZE=0 python3 -B - <<'PY'\nfrom pathlib import Path,PurePosixPath\nimport hashlib,json,tarfile,stat\nr=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');p=r/'evidence/s02-candidate-a-completion/a4/final-source-parser'\ndef need(v,n):\n if not v:raise ValueError(n)\ndef sha(b):return hashlib.sha256(b).hexdigest()\ndef read(n):return json.loads((p/n).read_bytes())\nneed(sha((p/'index.json').read_bytes())=='19348e1089004e998e096b3c59dff2cbc7888a98a5b31fd7ae89db5dc6a00932','index')\nneed(sha((p/'original-evidence.tar.gz').read_bytes())=='7233f85c03bfacf80d83d50a657a10f05a23d4863c157ecc81a4c37b13ee8119','archive')\npins={row['path']:row for row in read('index.json')['members']};seen=set();total=0\nwith tarfile.open(p/'original-evidence.tar.gz','r:gz') as tar:\n for member in tar:\n  name=member.name\n  need(name not in seen and name==str(PurePosixPath(name)) and not name.startswith('/') and '..' not in PurePosixPath(name).parts,'unsafe duplicate')\n  seen.add(name)\n  need(member.isfile() and member.size<=134217728,'regular bounded member')\n  f=r/name;before=f.lstat()\n  need(stat.S_ISREG(before.st_mode) and not any(x.is_symlink() for x in f.parents),'regular live original')\n  raw=tar.extractfile(member).read();live=f.read_bytes();after=f.lstat()\n  need((before.st_ino,before.st_size,before.st_mtime_ns)==(after.st_ino,after.st_size,after.st_mtime_ns),'original changed')\n  need(raw==live and len(raw)==pins[name]['bytes'] and sha(raw)==pins[name]['sha256'],'exact original '+name)\n  total+=len(raw)\nneed(seen==set(pins) and len(seen)==290 and total==52070254,'exact original membership')\nart=read('artifact-index.json');apins={row['path']:row for row in art['files']}\nneed(len(apins)==len(art['files'])==13 and art['selfExcluded'] is True and art['laterReviewsExcluded'] is True,'artifact scope')\nfiles={}\nfor f in sorted(p.iterdir()):\n need(f.is_file() and not f.is_symlink(),'package regular')\n raw=f.read_bytes();files[f.name]={'bytes':len(raw),'sha256':sha(raw)}\n if f.name in apins:need(files[f.name]=={k:apins[f.name][k] for k in ['bytes','sha256']},'artifact pin')\nneed(set(files)==set(apins)|{'artifact-index.json'} and len(files)==14,'14 package files')\nfor stage,sess,first,last in [('build',2692,'d67bbd','c27173'),('audit',81427,'47f0f3','2b4a4e')]:\n launch=read(stage+'-launch-tool-receipt.json');terminal=read(stage+'-terminal-tool-receipt.json')\n need(launch['result']['session_id']==terminal['args']['session_id']==sess and launch['result']['chunk_id']==first and terminal['result']['chunk_id']==last and terminal['result']['exit_code']==0,'authentic terminals')\n report=read(stage+'-report.json');value=json.loads(terminal['result']['output'])\n for key,v in value.items():need(report[key]==v,'derived report agreement')\nrejection=read('optimization-rejection-tool-receipt.json')['result'];need(rejection['chunk_id']=='999066' and rejection['exit_code']==1 and 'requires optimization disabled' in rejection['output'],'rejection')\nprint(json.dumps({'ok':True,'archivedOriginalsEqualLive':290,'originalBytesCompared':total,'artifactIndexPinsMatched':13,'packageFiles':files,'externalRuntimeOrTreesRehashed':False,'nativeExecuted':False},sort_keys=True))\nPY",
    "workdir": "/home/charl/Moriarty/.worktrees/s01-audit-start",
    "max_output_tokens": 2300
  },
  "result": {
    "chunk_id": "e31341",
    "wall_time_seconds": 0.126557174,
    "exit_code": 1,
    "original_token_count": 23,
    "output": "Traceback (most recent call last):\n  File \"<stdin>\", line 36, in <module>\nKeyError: 'ok'\n"
  }
}
```

## Corrected independent comparison and all14 package hashes

```json
{
  "args": {
    "cmd": "PYTHONOPTIMIZE=0 python3 -B - <<'PY'\nfrom pathlib import Path,PurePosixPath\nimport hashlib,json,tarfile,stat\nr=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');p=r/'evidence/s02-candidate-a-completion/a4/final-source-parser'\ndef need(v,n):\n if not v:raise ValueError(n)\ndef sha(b):return hashlib.sha256(b).hexdigest()\ndef read(n):return json.loads((p/n).read_bytes())\nneed(sha((p/'index.json').read_bytes())=='19348e1089004e998e096b3c59dff2cbc7888a98a5b31fd7ae89db5dc6a00932','index')\nneed(sha((p/'original-evidence.tar.gz').read_bytes())=='7233f85c03bfacf80d83d50a657a10f05a23d4863c157ecc81a4c37b13ee8119','archive')\npins={row['path']:row for row in read('index.json')['members']};seen=set();total=0\nwith tarfile.open(p/'original-evidence.tar.gz','r:gz') as tar:\n for member in tar:\n  name=member.name\n  need(name not in seen and name==str(PurePosixPath(name)) and not name.startswith('/') and '..' not in PurePosixPath(name).parts,'unsafe duplicate')\n  seen.add(name)\n  need(member.isfile() and member.size<=134217728,'regular bounded member')\n  f=r/name;before=f.lstat()\n  need(stat.S_ISREG(before.st_mode) and not any(x.is_symlink() for x in f.parents),'regular live original')\n  raw=tar.extractfile(member).read();live=f.read_bytes();after=f.lstat()\n  need((before.st_ino,before.st_size,before.st_mtime_ns)==(after.st_ino,after.st_size,after.st_mtime_ns),'original changed')\n  need(raw==live and len(raw)==pins[name]['bytes'] and sha(raw)==pins[name]['sha256'],'exact original '+name)\n  total+=len(raw)\nneed(seen==set(pins) and len(seen)==290 and total==52070254,'exact original membership')\nart=read('artifact-index.json');apins={row['path']:row for row in art['files']}\nneed(len(apins)==len(art['files'])==13 and art['selfExcluded'] is True and art['laterReviewsExcluded'] is True,'artifact scope')\nfiles={}\nfor f in sorted(p.iterdir()):\n need(f.is_file() and not f.is_symlink(),'package regular')\n raw=f.read_bytes();files[f.name]={'bytes':len(raw),'sha256':sha(raw)}\n if f.name in apins:need(files[f.name]=={k:apins[f.name][k] for k in ['bytes','sha256']},'artifact pin')\nneed(set(files)==set(apins)|{'artifact-index.json'} and len(files)==14,'14 package files')\nfor stage,sess,first,last in [('build',2692,'d67bbd','c27173'),('audit',81427,'47f0f3','2b4a4e')]:\n launch=read(stage+'-launch-tool-receipt.json');terminal=read(stage+'-terminal-tool-receipt.json')\n need(launch['result']['session_id']==terminal['args']['session_id']==sess and launch['result']['chunk_id']==first and terminal['result']['chunk_id']==last and terminal['result']['exit_code']==0,'authentic terminals')\n report=read(stage+'-report.json');value=json.loads(terminal['result']['output'])\n need(report['output']==value and report['actualToolChunk']==last and report['actualToolExit']==0,'derived report agreement')\nrejection=read('optimization-rejection-tool-receipt.json')['result'];need(rejection['chunk_id']=='999066' and rejection['exit_code']==1 and 'requires optimization disabled' in rejection['output'],'rejection')\nprint(json.dumps({'ok':True,'archivedOriginalsEqualLive':290,'originalBytesCompared':total,'artifactIndexPinsMatched':13,'packageFiles':files,'externalRuntimeOrTreesRehashed':False,'nativeExecuted':False},sort_keys=True))\nPY",
    "workdir": "/home/charl/Moriarty/.worktrees/s01-audit-start",
    "max_output_tokens": 2300
  },
  "result": {
    "chunk_id": "7bfabc",
    "wall_time_seconds": 0.054084455,
    "exit_code": 0,
    "original_token_count": 473,
    "output": "{\"archivedOriginalsEqualLive\": 290, \"artifactIndexPinsMatched\": 13, \"externalRuntimeOrTreesRehashed\": false, \"nativeExecuted\": false, \"ok\": true, \"originalBytesCompared\": 52070254, \"packageFiles\": {\"README.md\": {\"bytes\": 2667, \"sha256\": \"9dd10ec44d427ce35fa28cc61a8058889ae51874139e5bc1b96f4774b18a5f08\"}, \"artifact-index.json\": {\"bytes\": 2231, \"sha256\": \"f1d813ec46583722c27c8c8b04e2f9131f670da2b626800ffce344fbb8893c4a\"}, \"audit-launch-tool-receipt.json\": {\"bytes\": 526, \"sha256\": \"4508c359f3560d1c7f64ef9503f67ab3a472ad8de4aa1e3e335981e65eed7d39\"}, \"audit-report.json\": {\"bytes\": 1120, \"sha256\": \"4f10a636c15a936cfdd974d23a26fb31c91087062838d3eb6383726d195d5d44\"}, \"audit-terminal-tool-receipt.json\": {\"bytes\": 1265, \"sha256\": \"62df32af454afc39b61c6a104ca1fe11eb95be3aa6a10967961dce83f12ef80b\"}, \"audit.py\": {\"bytes\": 21833, \"sha256\": \"a638058cb410594c53b26636a4a59d900be1a4ab8506117faf7f9421b937c977\"}, \"build-launch-tool-receipt.json\": {\"bytes\": 518, \"sha256\": \"ae11e52dbd68e30e40e2d4a6205494bc1bdcc12b4afe1c0440f3ebc1b463b2d5\"}, \"build-report.json\": {\"bytes\": 471, \"sha256\": \"2fdf33e69265c5f61ca670a3700953d20cd16fe5affa37167e67ef62f2bd40a8\"}, \"build-terminal-tool-receipt.json\": {\"bytes\": 638, \"sha256\": \"e9ef5c680d197be351ee23d45bb9261a224325f0becb99e2127aacfd82381807\"}, \"build_archive.py\": {\"bytes\": 5133, \"sha256\": \"23f3e3787fbf180f2e6d58ea93ce2177b672f36c083121e9081bdbfceb98019f\"}, \"index.json\": {\"bytes\": 78831, \"sha256\": \"19348e1089004e998e096b3c59dff2cbc7888a98a5b31fd7ae89db5dc6a00932\"}, \"optimization-rejection-tool-receipt.json\": {\"bytes\": 851, \"sha256\": \"d7f5931e3143031649542ba8211ef79800d5e33ca5934bb52262df917f8b7a14\"}, \"original-evidence.tar.gz\": {\"bytes\": 5434195, \"sha256\": \"7233f85c03bfacf80d83d50a657a10f05a23d4863c157ecc81a4c37b13ee8119\"}, \"preservation-report.md\": {\"bytes\": 2366, \"sha256\": \"21d69a0d465d043360b98df32995fb95f94dd021181909f7df0623c15ae3eb71\"}}}\n"
  }
}
```


