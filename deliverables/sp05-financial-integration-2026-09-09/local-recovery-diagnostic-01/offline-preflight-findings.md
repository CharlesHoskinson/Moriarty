# Offline public wrapper coverage

Experiment observation: on repository HEAD `f55296c449f9fc271ad25097fd70435547941001`, the actual `validateLocalLaunchPlan` and `preflightLocalRecovery` entry points completed with `PUBLIC_STATE_VERIFIED` in 898.952485 ms. The subprocess exited 0 under `timeout --signal=TERM --kill-after=1s 10s`; stderr was empty, all 16 HTTP requests reached the ephemeral loopback fixture, and its server closed before exit.

Inputs were the reviewed `public-plan.json` (SHA256 `b50b6fc67f42013a978c1478b0e3fee3bcb53ecfd64f8f3c611d2d786199ecb2`), actual pinned runtime/SDK, unchanged proven build receipt/assets, retained deployment bytes/result and the native deployment's initial state. Only the plan's network endpoints and fixture deadline were changed in memory. The test loaded the actual generated state decoder and exercised genesis, exact finalized-tip readiness, SDK observation, full native state equality and stable finalized-head checks.

Filesystem traps on synchronous read/open/lstat/write/mkdir operations rejected accesses to all plan seed, wallet snapshot, roles, source private store, recovery store/inspection/password and output paths. The access log is empty. The test contains no wallet constructor or blockchain service command. Responses for chain tip/time, GraphQL metadata/fees and RPC finality were synthetic. This verifies wrapper compatibility for those fixture responses; it neither identifies the actual failed preflight cause nor establishes actual-chain acceptance.

Files: `offline-preflight-fixture.mjs`, `offline-preflight-result.json`, `offline-preflight-stderr.log`. Prior published fixtures and the seven diagnostic files under review were not modified.
