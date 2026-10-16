# DoctorWatson builds and tests Sherlock locally

Observed October 16, 2026. This delivery adds the real `--host` lane and uses it to build the new Sherlock `Machine::run_steps` capability. The capability advances the existing machine in bounded event groups, retains every complete step record, resumes after a cut and stops at terminal publication.

| Final observed check | Result |
|---|---|
| Ordinary Python QA | 79 tests passed |
| Compiled production Sherlock host tests | 7 tests passed |
| Actual producer versus independent frozen normal expectations | 8 programs, 216 fuel-prefix runs, 5,355 complete records matched |
| Complete host entrypoint | Exit 0, 3.611 seconds |
| DoctorWatson workflow model | 34 main scenarios and 1 directed test passed |
| Workflow random sampler | 10,000 traces, 60-step limit, seed 1; no violation observed in 21.383 seconds |
| Complete workflow entrypoint | Exit 0, 31.733 seconds across five separately bounded model commands |

These are local observations for the bound source files, not whole-language acceptance. The workflow model is separate from the Sherlock interpreter model. Eight random witnesses remain zero (`testPassed`, `reviewed`, `rebased`, `integrated`, `dependenciesChanged`, `layerRecorded`, `completed`, `sourceOnly`). Directed reachability has its own result; it does not overwrite random counts. Sampling establishes neither fairness nor exhaustive verification.

## Run the actual host lane

Use an already installed real compiler matching `rust-toolchain.toml` and a new absolute output directory outside the checkout:

```bash
DOCTORWATSON_RUSTC=/absolute/path/to/1.99.0/bin/rustc \
DOCTORWATSON_OUTPUT_DIR=/absolute/path/to/new-attempt \
./scripts/doctorwatson.sh --host
```

The command runs ordinary QA, snapshots the declared inputs, compiles the actual producer and seven tests, executes both, compares the complete diagnostic output and records postflight hashes. It does not install dependencies or run Cargo. Every host child has a 30-second wall limit. No memory or disk bound is claimed. See the [local CI contract](../../../DOCTORWATSON-CI.md) and the [observation format](../../../../models/sherlock/qa/HOST-OBSERVATIONS.md).

## Evidence

- [Aggregate result and consumed source hashes](RESULT.json).
- [Independent acceptance contract](acceptance/CONTRACT.md), frozen before implementation inspection.
- [Final actual host receipt](host-FINAL/receipt.json), [outer command/source receipt](host-FINAL-outer/result.json), [ordinary and host output](host-FINAL-outer/stderr), [compiled test output](host-FINAL/logs/run-tests.stdout) and [complete comparison](host-FINAL/comparison.json).
- [Actual complete producer output, gzip](host-FINAL/host-observations.json.gz), whose decompressed SHA-256 is `cfdc36e0b513aa8370c6d1371bc40ff1e11675651d5843ea4da94f175b529da3`. The consumed source snapshot is retained alongside it in `host-FINAL/source/` as individual gzip files. Executable hashes are in the receipt; binaries stay local.
- [Negative-control preparation](PREDECESSOR-SOURCE.json), [actual RED receipt](host-RED/receipt.json) and [compiler error](host-RED/logs/compile-producer.stderr). The predecessor without `run_steps` failed with exactly E0599 after ordinary QA passed. Every dependent stage stayed not-run; postflight passed.
- [First actual GREEN](host-GREEN/receipt.json) and [final workflow command/source receipt](design-FINAL/result.json), [model output](design-FINAL/stdout) and [ordinary test output](design-FINAL/stderr).
- [Initial workflow timeout](design-01/result.json) is preserved. [Performance change](performance-02/REPORT.md) reduced repeated pure computation without changing bounds, choices or invariants; the final full sampler passed within its original limit.
- [Four counter-history RED scenarios](stop-counters/01-stop-counters-RED.log) and [34 GREEN scenarios](stop-counters/03-stop-counters-GREEN.log).
- [Existing child-test source compatibility inspection](source-compatibility.json): no removed helper reference was found. The old umbrella was not compiled. [Entrypoint regression evidence](entrypoint-host-tests/RESULT-01.json) and 22 Opus-authored wrapper guard tests are included in the final 79-test runs.

Recorded absolute paths identify the original local run. The relative copies here preserve the same evidence bytes. `host-GREEN` and `host-FINAL` produced the same complete output digest. Raw provider prompts and private inventory sources are excluded.

## Review and remaining scope

Fresh [Muse](reviews/preflight-muse.md) and [Opus](reviews/preflight-opus.md) preflight reviews approved only the bounded execution. They are not merge approvals. Opus authored the build wrapper, its 22 guard tests and the narrow serializer pin update; author identities are retained under `reviews/`. A separate final candidate/result review is required before merge.

Opus preflight M1 is addressed by the explicit child-test source inspection above; M2 by ten entrypoint and 22 wrapper guard tests. Budget selection uses frozen corpus metadata as inputs; expected traces and outputs are not linked into the producer. If comparison exits nonzero, its raw JSON and digest remain available even though the wrapper receipt does not extract the first difference on that path.

This lane runs fixed trusted normal programs after constructor startup. Diagnostic K text is a projection, not a canonical codec or typed equivalence proof. Snapshot files are read-only; directories remain writable. The compiler sysroot, system linker, Python standard library and kernel are trusted and unhashed. Local hashes do not authenticate a hostile checkout. Timeout kills the child process group; normal-exit descendants are not separately swept.

K, Sherlock interpreter Quint, Lean/Clean, handwritten circuits, native ledger and Midnight stagenet were not run. Constructor startup is outside this observation domain. Native authority remains zero; C4, counters, frozen terminal attempts and all U0–U7/M1–M17/MC/SP/G exits stay unchanged. This delivery establishes the bounded local factory host lane and its first real Sherlock implementation cycle.
