TP-S1 corrected; narrow independent re-review pending.

Preserved original audit8f3b7211157e79ddfb3ba9680d3bd3921b58173a6ee44a1f80f36aba851d5780 at `.superpowers/sdd/a5-no-flatten-tiny-package-audit-original-20260906.py` and original REQUEST_CHANGES review7782552e2564fccf802761c351567a33f68fbf4e2162a41beb5bc249555fa223 at `.superpowers/sdd/a5-no-flatten-tiny-package-source-independent-review-original-20260906.md`. Existing review and original seal remain unchanged. Actual preservation5af7d9/0.

The only source change replaces `for r in seal.values(): check(r)` with validation of the exact five-field seal schema and original scope string, then checks exactly archive, beforeEndpoint, dispatch and sourceArchive as three-field pin dictionaries. It never passes scope to check(). Original seal SHAf756db106c11373bf26af779ef5e14e4f9a5b3c62deca2d8e345d82d45582cd7 is unchanged.

Frozen revised audit.py:40,176 bytes, SHA256 `d4e43bb8b8582067de0eead881fa37e4ad1f501becaa0f4571a9209646a4cb3f`. Static36a875/0 verified the exact one-site replacement, AST validity, original seal schema, and unchanged builder/README/digest/proposal. All four initial package files remain the only package files. No build, audit, helper or native execution occurred.

Exact preservation and static-check arguments/actual responses:

```json
{
  "preservation": {
    "args": {
      "cmd": "python3 -B - <<'PY'\nimport hashlib\nfrom pathlib import Path\nR=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');S=R/'.superpowers/sdd'\nrows=[(R/'evidence/s02-candidate-a-completion/a5/no-flatten-tiny-retained-data/audit.py',S/'a5-no-flatten-tiny-package-audit-original-20260906.py','8f3b7211157e79ddfb3ba9680d3bd3921b58173a6ee44a1f80f36aba851d5780'),(S/'a5-no-flatten-tiny-package-source-independent-review-20260906.md',S/'a5-no-flatten-tiny-package-source-independent-review-original-20260906.md','7782552e2564fccf802761c351567a33f68fbf4e2162a41beb5bc249555fa223')]\nfor src,dst,h in rows:\n b=src.read_bytes();assert hashlib.sha256(b).hexdigest()==h\n with dst.open('xb') as f:f.write(b)\n print(str(dst),h)\nPY",
      "workdir": "/home/charl/Moriarty/.worktrees/s01-audit-start",
      "max_output_tokens": 600
    },
    "result": {
      "chunk_id": "5af7d9",
      "wall_time_seconds": 0.000004879,
      "exit_code": 0,
      "original_token_count": 97,
      "output": "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-tiny-package-audit-original-20260906.py 8f3b7211157e79ddfb3ba9680d3bd3921b58173a6ee44a1f80f36aba851d5780\n/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-tiny-package-source-independent-review-original-20260906.md 7782552e2564fccf802761c351567a33f68fbf4e2162a41beb5bc249555fa223\n"
    }
  },
  "validation": {
    "args": {
      "cmd": "PYTHONOPTIMIZE=0 python3 -B - <<'PY'\nimport ast,hashlib,json\nfrom pathlib import Path\nR=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');S=R/'.superpowers/sdd';D=R/'evidence/s02-candidate-a-completion/a5/no-flatten-tiny-retained-data'\nsha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()\noriginal=S/'a5-no-flatten-tiny-package-audit-original-20260906.py';source=D/'audit.py'\nassert sha(original)=='8f3b7211157e79ddfb3ba9680d3bd3921b58173a6ee44a1f80f36aba851d5780'\nassert sha(S/'a5-no-flatten-tiny-package-source-independent-review-original-20260906.md')==sha(S/'a5-no-flatten-tiny-package-source-independent-review-20260906.md')=='7782552e2564fccf802761c351567a33f68fbf4e2162a41beb5bc249555fa223'\nold=\"    for r in seal.values():check(r)\\n\"\nnew=\"    need(set(seal)=={'archive','beforeEndpoint','dispatch','sourceArchive','scope'} and\\n         seal['scope']=='two acyclic source archives complete before native launch','exact original dispatch seal schema/scope')\\n    for key in ['archive','beforeEndpoint','dispatch','sourceArchive']:\\n        need(type(seal[key]) is dict and set(seal[key])=={'path','bytes','sha256'},'exact dispatch seal pin schema')\\n        check(seal[key])\\n\"\nassert original.read_text().count(old)==1 and source.read_text()==original.read_text().replace(old,new,1)\nast.parse(source.read_text())\nsealpath=S/'a5-no-flatten-diagnostic-20260906/tiny-dispatch-seal.json';seal=json.loads(sealpath.read_bytes())\nassert set(seal)=={'archive','beforeEndpoint','dispatch','sourceArchive','scope'} and seal['scope']=='two acyclic source archives complete before native launch'\nassert all(type(seal[k]) is dict and set(seal[k])=={'path','bytes','sha256'} for k in ['archive','beforeEndpoint','dispatch','sourceArchive'])\nfor name,h in {'build_archive.py':'73948c7c2aabbe34389fdf0ca049bfdf4afb79cafa199189bfcefa7e150f05e2','README.md':'fb094152578245386bca72d1dd79e421c8e26bec356277178398b517ee4072da','adopted-proposal-sha256.txt':'44bf97dfff21eb7fa100318b88b538f286664efafd632c76a9115fd313aa2e31'}.items():assert sha(D/name)==h\nassert sha(S/'a5-no-flatten-tiny-package-proposal-20260906.md')=='bb65bddefe3bb2ac3673ebd7186a8a1b700e9a21bcbfc22127cd1c9c7c35443a'\nassert {p.name for p in D.iterdir()}=={'build_archive.py','audit.py','README.md','adopted-proposal-sha256.txt'}\nprint(json.dumps({'ok':True,'finding':'TP-S1','onlyChange':'replace heterogeneous seal.values loop with exact schema/scope and four named pin checks','audit':{'path':str(source),'bytes':source.stat().st_size,'sha256':sha(source)},'originalAuditSha256':sha(original),'originalReviewSha256':sha(S/'a5-no-flatten-tiny-package-source-independent-review-original-20260906.md'),'originalSealSha256':sha(sealpath),'allOtherSourceAndProposalBytesUnchanged':True,'astValid':True,'auditOrBuildOrNativeOrHelperExecuted':False},sort_keys=True))\nPY",
      "workdir": "/tmp",
      "max_output_tokens": 1500
    },
    "result": {
      "chunk_id": "36a875",
      "wall_time_seconds": 0.000008996,
      "exit_code": 0,
      "original_token_count": 192,
      "output": "{\"allOtherSourceAndProposalBytesUnchanged\": true, \"astValid\": true, \"audit\": {\"bytes\": 40176, \"path\": \"/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-candidate-a-completion/a5/no-flatten-tiny-retained-data/audit.py\", \"sha256\": \"d4e43bb8b8582067de0eead881fa37e4ad1f501becaa0f4571a9209646a4cb3f\"}, \"auditOrBuildOrNativeOrHelperExecuted\": false, \"finding\": \"TP-S1\", \"ok\": true, \"onlyChange\": \"replace heterogeneous seal.values loop with exact schema/scope and four named pin checks\", \"originalAuditSha256\": \"8f3b7211157e79ddfb3ba9680d3bd3921b58173a6ee44a1f80f36aba851d5780\", \"originalReviewSha256\": \"7782552e2564fccf802761c351567a33f68fbf4e2162a41beb5bc249555fa223\", \"originalSealSha256\": \"f756db106c11373bf26af779ef5e14e4f9a5b3c62deca2d8e345d82d45582cd7\"}\n"
    }
  }
}
```

