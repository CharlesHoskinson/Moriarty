# Independent GPT-6 high R3 helper audit

**APPROVED_SCOPED.** The exact eight-file helper repair may be integrated. No scoped blocking finding remains. This is not task 5.2, production, financial or release approval.

The review read all four full helper sources and all four full test files, plus both R3 author reports and the preserved R2 audit. All live and frozen SHA-256 values matched before and after the audit. The manifest file-map digest is `ce0474a915172b3e953986491b20ebfd7b37ee039a95565705fdcb3ee3667fb4`.

The two R2 residual findings pass independent reproductions with corrected success predicates:

- Abort or timeout during the final `buildResponse` await, followed by late fulfilment or rejection, produces no response write. Abort starts no denial; timeout records exactly one. The listener is detached.
- Pending replay denial persistence remains visible after successful handshake and executor return. Two simultaneous replay records report `pendingProtocolWork: 2` and `outstandingWork: true`; late fulfilment and late rejection each reduce the live count to zero without writing to replay sockets. A response write awaiting its callback also remains pending through return. Child-error rejection preserves the outstanding denial record. Returned diagnostics remain an honest snapshot.

The original deadline, cancellation, nonce exception, late allocation and latest-start cases also pass. A real socket allocated before the deadline but delivered late reports its initial cleanup failure, then closes and removes socket/directory through `lateAllocation.settled`; there is no spawn. H2 is byte-identical: null/unknown raw exits remain unknown despite otherwise complete success facts; omission of the required rawExit field rejects validation.

Independent verification: **123/123 tests passed**, 0 failed or cancelled, 1896.5 ms. The expanded independent probe exits 0 with 21 observations and no stderr. Three blocked result-read and cleanup replicates returned in 39.73, 39.61 and 39.51 ms against a 40 ms total plus a 10 ms scheduling allowance. `git diff --check` passed. The author allowance is now 25 ms, below its 40 ms outer bound.

The full guarded gate was independently inspected from the terminal event and extracted full log: queue 1658, event sequence 26, `2026-09-13T19:07:54Z`, gate rc 0 and fresh report. Reported pass counts: language 906; Midnight 63; ledger 647; compiled 3; compiled rejection 11; finalized state 27; Preview 89; executor 123. All reported failure counts are zero. This gate was not rerun by the reviewer.

Production Python consumer/accounting and real terminal/stop/timer evidence remain the next capability. Their absence does not block this scoped helper integration. Synchronous blocking I/O is not preemptible; pending work is reported, not cancelled; late allocation may remain unresolved forever. C wrapper integration and the historical five-minute hang are not approved or claimed fixed here.

Artifacts:

- `/tmp/moriarty-handoff-fable-r3-audit-gpt6-high.json`
- `/tmp/moriarty-handoff-fable-r3-probes-gpt6-high.mjs`
- `/tmp/moriarty-handoff-fable-r3-probes-gpt6-high.jsonl`
- `/tmp/moriarty-handoff-fable-r3-executor-tests-gpt6-high.log`
- `/tmp/moriarty-handoff-r3-evidence/full-gate.log`

## Frozen SHA-256 manifest

- `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs`: `7452ab77b1c2d619858cdb5826495573877f2ffe793fdd0a6f4c53b53fc272f9`
- `experiments/moriarty-midnight-financial/ledger/executor-caller.test.mjs`: `0ce3b8bb5bae0fa92b68a6eb19fdf5407eda4f8a3dc14a433eda624e2dcf944a`
- `experiments/moriarty-midnight-financial/ledger/fault-matrix.mjs`: `1d6b52248b8b1748e554395cfd22ee0075141de261e082edd5063a7fda4619dc`
- `experiments/moriarty-midnight-financial/ledger/fault-matrix.test.mjs`: `3a2fc296104a0928095e5e413e6329353be0fb5a99d6eff00688fb278ef857b6`
- `experiments/moriarty-midnight-financial/ledger/invocation-handoff.mjs`: `5b748c56a9bb53055077f2fe2b735d0cb6ca25b5c92124209c64618b4f50754f`
- `experiments/moriarty-midnight-financial/ledger/invocation-handoff.test.mjs`: `d781ed0e27bb2d5b74711b112633ca0017d0b8ef419ccbae94c49fe3ed08066f`
- `experiments/moriarty-midnight-financial/ledger/prover-lifetime.mjs`: `a046a77a17cd38ab4d0a97e3df601add97ebde8443e3303d1dfc1a9bb8967cdb`
- `experiments/moriarty-midnight-financial/ledger/prover-lifetime.test.mjs`: `693f88c809668f0daf513f8b5ed12826d4facfea72a66a9eda160ac373d5e6ed`
