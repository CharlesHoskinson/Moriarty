# Deadline completion correction

The first candidate's timer race could return a successful snapshot after its absolute deadline when synchronous work delayed timer delivery. The independent GPT6 BLOCKED review and its reproduction remain retained. The original reader and test are archived under `original-candidate/`; original manifests, README and logs are unchanged.

`reader-late-rpc-red.tap` reproduces three missing predicates through connected interfaces: late first RPC, late final canonical RPC, and expiry during native cleanup. It records18passes/3failures. The first case returned the wrong deadline classification; the final RPC and cleanup cases incorrectly succeeded. Tests advance Date.now inside the RPC/native-cleanup callbacks, including the reader's60second cap under a longer caller deadline.

The reader now checks that capped absolute deadline before and after each awaited RPC and after native cleanup, immediately before successful return. Late completion throws `OBSERVATION_TIMEOUT_UNKNOWN`; it cannot trigger another RPC or escape native cleanup. This local repair does not change shared receipt.mjs. Root separately corrected the transport's post-response deadline check, with `rpc-deadline-red.tap` and `rpc-deadline-green.tap` retaining its reproduction and94passing tests.

The combined package command passed27tests, zero failed/skipped, in `reader-late-rpc-green.tap`. `git diff --check` exited0. Reproduction still requires the same retained build environment variable and runs no service, proof, compiler, private input or network operation:

```sh
MORIARTY_SWAP_BUILD_RECEIPT=/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/swap-output/build/build-receipt.json npm --prefix experiments/moriarty-midnight-financial run test:finalized-state
```

`source-candidate-02.json` binds the corrected reader/test, corrected transport/test and package script. Scope remains an unauthenticated read-only observation at an explicit finalized anchor. Actual installed-node RPC support and before/after ledger nonmutation remain unobserved. New independent audits are required; old verdicts do not approve corrected bytes.
