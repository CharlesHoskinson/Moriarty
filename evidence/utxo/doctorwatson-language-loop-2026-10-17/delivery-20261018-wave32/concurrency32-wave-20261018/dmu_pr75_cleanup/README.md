# PR75 provenance cleanup ready for integration

Base: `64c6dedff1d99b33c23832330d7911a5fad9ed59`. [cleanup.patch](cleanup.patch) changes only
`docs/CORE-STAGED-TESTS.md` and `models/core-stage-tests/PROVENANCE.md`.
Full changed bodies are under `files/`; [MANIFEST.json](MANIFEST.json) records
Git blob/SHA-256 pins, exact scope, consumer trace, requirements and expectations.

This completes the prior unapplied cleanup: stale pre-merge prose becomes the
recorded merge and historical review scope; the historical-directory argument
is exact; original native0/admission/UNRUN fields are clearly capture-time
history. Current execution instructions and override semantics remain intact.
All later report-library/Core2/M01/M03 clauses and historical receipts remain
unchanged. No product-code or semantic change is proposed.

Root can reproduce the literal validation without product execution:

```sh
python3 /home/charl/moriarty-c2-opus55-restored-20261017/concurrency32-wave-20261018/dmu_pr75_cleanup/validate.py --repo /home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018
```

The validator reconstructs every hunk against immutable Git blobs, compares
complete output bodies/hashes, checks local links and confirms the entire later
guide suffix is unchanged. No product tests were added or run for this prose.
No fresh provider audit or acceptance is claimed; root alone integrates.
