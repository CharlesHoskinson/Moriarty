# GPT-6 observed-KAST repair and third-allocation review

Verdict: **approve candidate 03 and one new bounded allocation under resource-proposal-03.json.** Separate exact Fable approval and root dispatch checks remain required.

Manifest SHA-256: `bec04a6b4de57255bd328cf677ca99475cc0b9877c16e943ae901944de4a4c23`. All 30 current candidate/supporting bindings match. I refreshed guarded status and retained its existing operational blocks. I made no implementation changes and ran no K command.

## Actual failure and repair

Attempt 02 compiled successfully in 4.334284568 seconds. Its first krun exited 0 in 1.919454367 seconds. The captured output uses KAST version 4, structured KLabel/KSort records, and KApply without the former variable field. The old v3-only decoder rejected that envelope. This was an output-format mismatch after actual execution, not a financial rejection.

The repaired decoder accepts exactly the observed v4 format. It checks closed label/sort records, empty parameters, names, integer arity/version, token sorts, UInt128 bounds, packet digest, receiver identity and full JSON consumption. Old v3 and malformed mixed formats reject. Financial reconstruction is unchanged. There is no fallback evaluator or substituted expected result.

The captured regression fixture is byte-identical to attempt-02/trace-01.stdout: SHA-256 `75a692c44f223d42faca6308a468bf2383d0837b4f87add7eb50698879e0ae4e`. I independently decoded that raw capture and compared every field with the first independent expected result. It matches: cash 70/30, principal/outstanding 70, allowance 70/30, work 98/2, preserved reserve/metadata, correct tombstones and ordered effects.

All six codec tests pass. Additional independent mutations of the actual capture reject: stale digest, wrong receiver row, duplicate JSON key, boolean arity, legacy string sort and an extra label field. K rules, runner, all sixteen financial inputs/expectations, toolchain and financial reference inputs remain unchanged. No broader semantic re-review or K invocation was needed for this compatibility repair.

The root artifact-stability receipt reports 174 compiled hashes unchanged after the first krun. This is evidence for that one call only. Keep fail-closed artifact comparison unchanged.

## Resource decision

Approve one new compile attempt of at most 180 seconds, up to sixteen krun invocations of at most 20 seconds each, and 512 seconds aggregate. The unit03 command retains one heavy tree, nonblocking flock, 4 GiB whole-cgroup memory, zero swap and immediate SIGKILL. This prospective allocation is justified by the exact observed format mismatch and successful offline replay of its repair.

Historical charges remain two compile attempts and one krun. Preserve both earlier evidence sets. Rename the original build directory intact to .build-attempt-02 only after both reviews. Root must verify unchanged candidate bytes and no overlapping heavy workload before the exact reviewed dispatch. Stop on first failure and retain counters and raw observations.

This review establishes offline compatibility with the first captured result. It does not establish the remaining fifteen K observations or execution of the repaired candidate. Both independent result audits remain required after the full bounded run. No full correspondence, proof, semantic freeze, SP03 completion, PCD or ledger acceptance claim follows.
