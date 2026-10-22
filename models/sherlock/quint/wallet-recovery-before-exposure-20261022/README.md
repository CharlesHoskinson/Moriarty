# Recovery before signed-byte exposure — 2026-10-22

This finite synthetic Quint model distinguishes possible disclosure of signed bytes, conservative external ownership, a possible own-submit flag, and abstract own-submit attempts. Read-only remote validation can give an RPC operator signed bytes that might be relayed. A read-only method name is not evidence that bytes remain private or economic authority is revoked.

The source-order comparison is:

- Earlier Source28: full financial guard → remote validation/exposure possibility → prepared records and durable recovery → identity/ownership arm → own submit.
- Source-only Source29 correction: full financial guard → coherent prepared records → durable recovery → bound identity and conservative ownership arm → remote validation → at most one own submit after a valid verdict.

The exact passed model and tests retain both routes. A three-state, two-transition counterexample violates `exposureSafety` on the earlier route: exposure becomes possible while durable recovery, prepared identity and ownership are false, with zero modeled own attempts. That is a source-order witness, not an observed delivery, relay or live transaction.

Recorded with Quint 0.32.0, TypeScript backend:

| Check | Recorded result |
| --- | --- |
| Typecheck | Exit 0, 1.977 seconds |
| Directed tests | 10 named tests passed, one sample each, 1.791 seconds |
| Sampled `correctedSafety` | 10,000 traces, seed 1, maximum 10 steps, no violation, 5.899 seconds |
| Earlier-order counterexample | Expected invariant violation, exit 1, 1.917 seconds |

All six sampled witnesses were nonzero: earlier unrecovered exposure 2,736; corrected protected exposure 1,722; corrected modeled own attempt 569; refusal after protected exposure 1,123; crash after protected exposure 30; and fsync failure before exposure 328. The unchanged failstop wrapper stopped on the final expected exit 1 after retaining its complete raw output. It was not a passing validator receipt or an unexpected tool failure.

Run locally from this directory:

```sh
quint typecheck recoveryExposureTest.qnt
quint test recoveryExposureTest.qnt --main recoveryExposureTest --match '.*Test$' --backend typescript --max-samples 1 --seed 1
quint run recoveryExposure.qnt --main recoveryExposure --backend typescript --max-samples 10000 --seed 1 --max-steps 10 --invariant correctedSafety
```

To reproduce the deliberately unsafe-order counterexample (expected exit 1):

```sh
quint run recoveryExposure.qnt --main recoveryExposure --backend typescript --init initSource28 --max-samples 1 --seed 1 --max-steps 2 --invariant exposureSafety
```

Refusal, crash or local bookkeeping rollback does not erase exposure history or revoke still-live signed authority. Failed writes/fsync prevent corrected exposure; post-exposure refusal retains recovery and ownership uncertainty. A possible own-submit flag does not establish an actual call. `modeledExplicitAttempts` counts only synthetic actions, never observed SDK/network calls.

Coherent records/identity, actual filesystem durability, full financial and cryptographic validity, authentic/current ledger state and TTL enforcement remain external premises. Logical ownership is not a claim volatile JavaScript state survives a crash. There is no automatic recovery/resubmit action. Source29 is a source-only correction in this evidence, not live runtime or security acceptance. These are sampled source-order results, not exhaustive proof or compiled-code correspondence. They establish no registration, finality, deployment or EARS acceptance; the unique earlier Source18 cause remains unknown. This package contains no raw captures, wallet/SDK source, transaction bytes, keys or private fixtures, and selected no financial operation.
