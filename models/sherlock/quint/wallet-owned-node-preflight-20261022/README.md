# Owned node-validation requests — 2026-10-22

This finite synthetic Quint model covers the source-level lifecycle of four owned RPC requests: one best-head query, two parallel runtime/metadata queries pinned to that head, and one admission query. It models response/error termination, settlement only after observed closure, timeout precedence, sibling cancellation and finally cleanup. Closure means a WebSocket close event or `readyState === CLOSED`; `CLOSING` does not settle a request.

The files are exact passed Stage08 R2 model/test bytes. Recorded with Quint 0.32.0, TypeScript backend:

| Check | Recorded result |
| --- | --- |
| Typecheck | Exit 0, 3.771 seconds |
| Directed tests | 10 named tests passed, one sample each, 1.983 seconds |
| Sampled invariant | 10,000 traces, seed 1, maximum 16 steps, no violation, 7.592 seconds |

The sampled run reached refusal 10,000 times, response-awaiting-close 1,561, timeout closure 4,365 and partial-context cleanup 529. It reached successful-return and cache-limited-success **zero** times. Both positive paths were reached by directed tests; they are not claimed as sampled coverage or exhaustive verification. The first attempt had eight passing tests and two QNT502 test-DSL errors from continuing after a blocked-action `fail()` assertion. R2 repaired only those test chains; the model was unchanged. That earlier failure was retained separately.

Run locally from this directory:

```sh
quint typecheck ownedNodeValidationTest.qnt
quint test ownedNodeValidationTest.qnt --main ownedNodeValidationTest --match '.*Test$' --backend typescript --max-samples 1 --seed 1
quint run ownedNodeValidation.qnt --main ownedNodeValidation --backend typescript --max-samples 10000 --seed 1 --max-steps 16 --invariant ownershipAndDeadline
```

This is a source-grounded abstraction of the frozen Source28 request lifecycle, after configuration, strict flags, serialization and lazy-session setup. It does not execute SDK, wallet, RPC, native code or transaction bytes. One pinned best head is not continuous freshness. The deadline is a timer-callback event, not a hard wall-clock guarantee. Noncooperative sockets, process termination, codec correctness and normal buffer operations remain external premises. Node hash-only soft-cache acceptance (60 seconds) is not a fresh strict certificate or coin reservation.

`dispatches == 0` counts only this service's explicit submissions. It does **not** mean signed bytes are unexposed: the earlier Source28 caller sent them to remote validation before durable recovery. That ordering defect is separate from these transport results; the companion recovery/exposure model addresses it. No validation result establishes registration, finality, deployment or EARS acceptance. The unique cause of the earlier Source18 failure remains unknown. This package contains model/test source and this recorded summary, not raw captures, wallet/SDK source, keys or private fixtures. No model check selected any financial operation.
