# How to run and check the stream realization

Use this guide to build the fixed normal `visit_steps` producer, run its selected tests and compare complete observations with independent expectations. Use a trusted checkout, installed Python 3.10 or newer, and the already installed Rust compiler pinned by `rust-toolchain.toml` (1.99.0 in the recorded attempt). Nothing is installed automatically.

## Select a fresh attempt

From the checkout root, use normalized absolute paths. The compiler must be the toolchain's actual executable with a sibling sysroot, not a rustup proxy or symlink. The output parent must exist; the attempt directory must not exist and must lie outside the checkout. Its path may contain only letters, digits, `.`, `_`, `+`, `/` and `-`.

```bash
python3 -I -B models/sherlock/qa/run_host.py \
  --rustc /absolute/toolchains/1.99.0/bin/rustc \
  --output-dir /absolute/new-stream-attempt \
  --realization stream
```

Replace the example paths with your installed compiler and fresh attempt path. Stream selection belongs to this direct wrapper. `scripts/doctorwatson.sh --host` selects the default host realization; the shell entrypoint has no realization argument. Its default ordinary check remains separate.

## Inspect each stage

Expect exit 0, `terminal_state: success`, `realization: stream` and `test_disposition: ok` in `result.json`. Retain `receipt.json`, `comparison.json`, `stream-observations.json` and `logs/` together with the source snapshot and build identities.

In `receipt.json`, require `ok` for preflight, compiler version, producer compilation, test compilation, eight selected tests, producer execution, complete comparison and postflight. Each child has a 30-second wall limit. No failed stage permits dependent execution. Source/compiler/Python pins are checked after the attempt; linker, sysroot, Python library and kernel are not fully hashed.

The producer cycles observation-call widths 1, 2, 4 and 8 over N01–N08. Compare all 216 fuel runs, complete initial states, 5,355 rule/charge/before/after records, terminal states and no-next declarations. `comparison.json` should have `status: pass`, `supplied_host_observation.status: match` and `first_difference: null`. Its supplied-file authenticity fields remain unestablished; the separate wrapper receipt records the actual local execution.

The eight tests cover N01 terminal precedence, N02 complete records at budgets 1/2/3/8/16, inert zero budgets, positive budgets after terminal, observer breaks at N02 cuts 1–7 with resumption, a budget cut before pending free publication, zero-fuel exhaustion, and recursive N05 caller environments at charged cuts 19/20/22/25/26/27. These scopes do not claim every cut of every program.

## Retain failures and scope

If the compiler is missing, mismatched or a proxy, select the correct existing compiler. If the output directory exists, choose a new path and preserve the earlier result. Invalid arguments or an unusable fresh output path return exit 2; recorded stage failure returns exit 1. Read the first failed stage and its stderr; keep dependent `not-run` stages. For comparison failure, use [find the first difference](find-first-difference.md).

The October 17 [stream-actual-01 archive](../../results/sherlock-stream-20261017/README.md) records Rust 1.99.0, eight passing tests, all 216 complete comparisons and 26 unchanged source pins. Its result SHA-256 is `a4f90bbc8d6c6fd72534a5ce7b09ceacfdb4fbeb175a7c86708e27b9197c175c`. The separate integrated ordinary run passed 241 Python tests.

This is a post-startup A-profile host observation. The producer retains all records for comparison; it demonstrates no streaming memory or allocation bound. Constructor startup, K/Quint/Lean/Clean, handwritten constraints, native proof/ledger and full V1 acceptance remain separate.

A separate [composed host archive](../../results/sherlock-composed333-20261017/README.md) records a passing named and ordered comparison of the same 333 noncontext carriers. Its initial lexical dep-info check remains FAILED; the continuation normalized the exact ten-member set and reused the host artifact without recompilation. Both context limbs, checked335, SDK/circuit/native and target evidence remain unobserved.

The [reconciled design](../design.md) incorporates ten Gemini research reports and three Astra MAX reconciliation reviews. Ordinary-loop readiness remains conditional; current Quint validation is ongoing. These records do not establish a model pass or an armed completion loop.

See [host interfaces](../reference/host-machine.md), [DoctorWatson reference](../reference/doctorwatson.md), [comparison instructions](compare-complete-observations.md) and [chunks and financial boundaries](../explanation/chunks-and-financial-boundaries.md).
