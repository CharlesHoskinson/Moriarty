# A5 compiler phase preservation package: independent review

**Spec verdict: PASS. Evidence-preservation verdict: PASS.** Reviewed package `evidence/s02-candidate-a-completion/a5/compile-phase-diagnostic-preservation/`. This admits preservation quality only; compiler acceptance remains false and H1 unresolved. No concrete package blocker was found.

I read the complete standalone audit, builder, README, index schema/bindings and original packaging/audit/optimization-rejection receipts before running the audit. The audit uses only Python standard-library and packaged bytes, rejects optimization before reading the archive, checks safe canonical unique regular/directory members and bounded nested members, and never extracts to disk, imports or executes archived code. The builder was inspected, not executed. The audit has no native call, live group check or external runtime-tree read.

Standalone execution from `/tmp` with `PYTHONOPTIMIZE=0` and `-B` returned actual chunk **f49518**, exit **0**. A separate read-only byte comparison returned actual chunk **764f9e**, exit **0**. Both authentic arguments/results are retained below.

- Exact package remains10 regular files. Archive SHA-256 `9b3e13c42c4942f0d2440b49c2340054a38b1811b8165d49721e95b9e3858158`; index SHA-256 `2ff06fc6e52a30103cab79dfe8b689fbdfca2e8e19e87414e7657559d97bd64c`.
- All282 archived regular originals equal the retained live originals byte-for-byte:33,410,745bytes total. All25 original directories match, including the intentional mock `trace.jsonl` directory. Live closed diagnostic membership remains259files. The archive is the exact255 intake pins plus intake and its3 subsequent transport files,22 proposal-listed external originals and the original proposal.
- The standalone audit verified12 unchanged nested source archives/1,253 nested members,102 preparation pins and exact expanded stage sets, original four plan-source blocks,24 view files/two reversible import additions/30 original pins, recorded8,083 unique runtime pins and all external manifest/archive references.
- The seven mock children retain expected identity/failure74 behavior; four alias children retain0/0/1/1 exits, raw stream-pair equality and exact observed Right/Left/skip sequences. The full child remains-15 at900seconds, recorder/outer0 for preservation, complete forced SIGTERM cleanup, and empty stdout/stderr/resources. Its seven trace rows support only the incomplete observed public compile interval in that invocation, including observer I/O and scheduling margins.
- All100 returned transport responses across modes remain bound, including91 full responses and the genuine terminal. Source/design/plan reviews, control adoptions, root dispatch, A4 prerequisite and final intake bindings pass. The alias prelaunch save failure and unavailable nested response, original independent-review orchestration error, A4 polling gap and unknown historical parent remain preserved.
- Original author packaging98bf43/exit0 and audit351b74/exit0 agree with their report data. Original deliberate optimization rejection262a4f/exit1 is preserved separately. The independent output agrees with those admitted audit findings. Its raw stdout SHA-256 is `ea1c7eeee8735b189aa010ed1dc063a0ec963b6e94311bea92fe6127db2f4504`.

The two external runtime archives and installed runtime trees were not rehashed here; the package retains their original endpoint/manifests and independently admitted references. No live process state is claimed by this package audit. No developing case014 ITF was read. No source/runtime/original file was edited, no native/test/helper capture was run, and no commit occurred. Historical compiler phase remains unknown; no causal hotspot, new Node exit, GNU wall/CPU/RSS metric, solver acceptance, full A4/A5 completion or H1 result is inferred. Root owns final preservation admission and any later scoped commit.

## Actual standalone audit

```json
{
  "args": {
    "cmd": "PYTHONOPTIMIZE=0 python3 -B /home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-candidate-a-completion/a5/compile-phase-diagnostic-preservation/audit.py",
    "workdir": "/tmp",
    "max_output_tokens": 1600
  },
  "result": {
    "chunk_id": "f49518",
    "wall_time_seconds": 0.356643209,
    "exit_code": 0,
    "original_token_count": 215,
    "output": "{\"A4PollingGapPreserved\": true, \"H1\": \"unresolved\", \"aliasChildren\": 4, \"aliasPrelaunchFailurePreserved\": true, \"archivedCodeExecuted\": false, \"childRecords\": 12, \"compilerAcceptance\": false, \"continuousHistoricalImmutabilityProved\": false, \"externalRuntimeRehashed\": false, \"fullActualChildExit\": -15, \"fullActualOuterExit\": 0, \"fullActualRecorderExit\": 0, \"fullChildren\": 1, \"fullReturnedResponses\": 91, \"historicalCompilerPhase\": \"unknown\", \"intakeActualOuterExit\": 0, \"intakeOriginalPins\": 255, \"mockChildren\": 7, \"nativeExecuted\": false, \"nestedSourceArchives\": 12, \"nestedSourceMembers\": 1253, \"newIncompleteObservedInterval\": \"compile\", \"ok\": true, \"optimization\": 0, \"originalDirectories\": 25, \"postindexOriginals\": 4, \"preparationSourcePins\": 102, \"regularArchiveMembers\": 282, \"returnedTransportResponses\": 100, \"runtimeUniqueRecordedPins\": 8083}\n"
  }
}
```

## Actual independent original-byte comparison and complete package hashes

```json
{
  "args": {
    "cmd": "PYTHONOPTIMIZE=0 python3 -B - <<'PY'\nfrom pathlib import Path, PurePosixPath\nimport hashlib,json,stat,tarfile\nr=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');p=r/'evidence/s02-candidate-a-completion/a5/compile-phase-diagnostic-preservation'\ndef need(ok,msg):\n if not ok:raise ValueError(msg)\ndef sha(b):return hashlib.sha256(b).hexdigest()\nidx=json.loads((p/'index.json').read_bytes())\nneed(sha((p/'index.json').read_bytes())=='2ff06fc6e52a30103cab79dfe8b689fbdfca2e8e19e87414e7657559d97bd64c','index')\nneed(sha((p/'original-small-evidence.tar.gz').read_bytes())=='9b3e13c42c4942f0d2440b49c2340054a38b1811b8165d49721e95b9e3858158','archive')\npins={row['path']:row for row in idx['members']}\nseen=set();directories=set();total=0\nwith tarfile.open(p/'original-small-evidence.tar.gz','r:gz') as tar:\n for member in tar:\n  need(member.name not in seen,'duplicate');seen.add(member.name)\n  need(not member.name.startswith('/') and '..' not in PurePosixPath(member.name).parts,'unsafe')\n  original=r/member.name\n  before=original.lstat()\n  if member.isdir():\n   need(stat.S_ISDIR(before.st_mode) and member.size==0,'original directory');directories.add(member.name);continue\n  need(member.isfile() and stat.S_ISREG(before.st_mode) and member.size<=4000000,'small regular original')\n  b=tar.extractfile(member).read();live=original.read_bytes();after=original.lstat()\n  need((before.st_ino,before.st_size,before.st_mtime_ns)==(after.st_ino,after.st_size,after.st_mtime_ns),'changed original')\n  need(b==live and len(b)==pins[member.name]['bytes'] and sha(b)==pins[member.name]['sha256'],'original equality '+member.name);total+=len(b)\nneed(seen-directories==set(pins) and len(pins)==282 and directories==set(idx['directories']) and len(directories)==25,'membership')\nout=r/'.superpowers/sdd/a5-compiler-phase-diagnostic-20260906'\nneed({str(f.relative_to(r)) for f in out.rglob('*') if f.is_file()}=={n for n in pins if n.startswith(str(out.relative_to(r))+'/')},'live closed diagnostic files')\nneed({str(f.relative_to(r)) for f in out.rglob('*') if f.is_dir()}==directories,'live closed diagnostic directories')\nfiles={}\nfor f in sorted(p.iterdir()):\n need(f.is_file() and not f.is_symlink(),'package regular file')\n files[f.name]={'bytes':f.stat().st_size,'sha256':sha(f.read_bytes())}\nneed(len(files)==10,'10 original package files')\nar=json.loads((p/'audit-tool-receipt.json').read_bytes());pr=json.loads((p/'packaging-tool-receipt.json').read_bytes());rej=json.loads((p/'optimization-rejection-tool-receipt.json').read_bytes())\nneed(ar['responses'][0]['exit_code']==pr['responses'][0]['exit_code']==0,'author actual exits')\nneed(json.loads(ar['responses'][0]['output'])==json.loads((p/'audit-report.json').read_bytes()),'author audit output')\nneed(json.loads(pr['responses'][0]['output'])==json.loads((p/'packaging-report.json').read_bytes()),'author package output')\nneed(rej['responses'][0]['exit_code']==rej['expectedExitCode']==1 and 'requires Python optimization disabled' in rej['responses'][0]['output'],'original rejection')\nprint(json.dumps({'ok':True,'archivedOriginalsEqualLive':282,'originalDirectoriesEqualLive':25,'liveOriginalBytesCompared':total,'closedDiagnosticFiles':259,'packageFiles':files,'externalRuntimeRehashed':False,'developingNativeITFRead':False},sort_keys=True))\nPY",
    "workdir": "/home/charl/Moriarty/.worktrees/s01-audit-start",
    "max_output_tokens": 2000
  },
  "result": {
    "chunk_id": "764f9e",
    "wall_time_seconds": 0.014823809,
    "exit_code": 0,
    "original_token_count": 358,
    "output": "{\"archivedOriginalsEqualLive\": 282, \"closedDiagnosticFiles\": 259, \"developingNativeITFRead\": false, \"externalRuntimeRehashed\": false, \"liveOriginalBytesCompared\": 33410745, \"ok\": true, \"originalDirectoriesEqualLive\": 25, \"packageFiles\": {\"README.md\": {\"bytes\": 3535, \"sha256\": \"049b12d45cf2bbed010c9042227f7f7b4e52877acb9702eb4039b5bbee3f36de\"}, \"audit-report.json\": {\"bytes\": 919, \"sha256\": \"b2e3f82ff33ec5e66a42b456038fbc7e35d5c2fe3901a01771a5c9faea4e177c\"}, \"audit-tool-receipt.json\": {\"bytes\": 1419, \"sha256\": \"c0401cef3862775754cd873e9651dfe2fca3764aa515682aef1cc1496789d5bf\"}, \"audit.py\": {\"bytes\": 29776, \"sha256\": \"a5bc7cbc1a23c61bc6cfd88ca8b167720837ede8b66837bbd0d1f9c4a35600d7\"}, \"build_archive.py\": {\"bytes\": 5661, \"sha256\": \"f24b3c39f3ce15cdc612901116b07fc917f3aefdf969187ee05787417fe247ca\"}, \"index.json\": {\"bytes\": 71523, \"sha256\": \"2ff06fc6e52a30103cab79dfe8b689fbdfca2e8e19e87414e7657559d97bd64c\"}, \"optimization-rejection-tool-receipt.json\": {\"bytes\": 882, \"sha256\": \"37bd8216c7f5860f0cdec026f3e8bbc176f4761157438327db1fbea790cde0c5\"}, \"original-small-evidence.tar.gz\": {\"bytes\": 17213019, \"sha256\": \"9b3e13c42c4942f0d2440b49c2340054a38b1811b8165d49721e95b9e3858158\"}, \"packaging-report.json\": {\"bytes\": 243, \"sha256\": \"7d8eefad36c630c1799eca9006a0fde860c463ca5ca16daabc00d2c279303b65\"}, \"packaging-tool-receipt.json\": {\"bytes\": 736, \"sha256\": \"e34da978c896fedf70f646decb0175a1f2a2557bad54b9ef48c9b67965251995\"}}}\n"
  }
}
```

