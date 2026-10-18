# K10 compiled-C2 native wrapper operator (source-only)

## What this is

A minimal stdlib-Python operator for the next real capability step: evaluating the **already compiled** APPLIED B9 `c2.c2.mint` wrapper, with its genuine published Groth16 proof/VK and raw PlutusV3 ScriptContext, under the independently recomputed compiled policy `6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1`. It deliberately does not carry the K9 full project compiler, stdlib staging, test telemetry, 147/472 parent or SDK machinery: the script is already compiled, so the only native command is the pinned Aiken `uplc eval --cbor`.

This source was authored under the user's explicit Gemini/Muse fallback after the original Opus source author timed out. Authorship is not independent approval, and nothing here has been executed by its author.

## The seven calls

Exactly seven sequential calls, one attempt each, zero retries, launch intent counted before `Popen`:

```
/tool/aiken uplc eval --cbor /inputs/script.hex '<single context term>'
```

Each term is the full content of one pinned 2649-byte generated context file, bounded-read, hash-checked, strict-UTF-8 decoded, with exactly one trailing newline stripped if present, passed as one argv element. No shell, no string evaluation, no `--flat`, `--debug` or other guessed flags. Source basis: Aiken 1.1.24 `eval.rs` (`--cbor` reads hex text, each positional argument is parsed as a term and applied, success prints one pretty JSON object, failure exits 1 with Error/Costs and optional Logs on stderr).

T1 accepts only on exit 0, empty stderr, and strict JSON with exactly `result`/`cpu`/`mem`, result exactly `(con unit ())`, and non-bool positive integer costs. T2–T7 accept only on exit exactly 1, empty stdout, and full-match stderr carrying the exact `machine/error.rs` EvaluationFailure text `the validator crashed / exited prematurely` plus strict Costs, with at most one bounded Logs section. Out-of-budget, syntax, signal, timeout or any trailing unexplained content fails closed and is preserved, never retried.

Guard labels (pairing for T2–T4, `qty == 1`, token name, VK digest) are source expectations from `claim.issue` conjunct order, the coherent positive control and each minimal changed case. They are finite-predicate support only: the runtime does not instrument named guards, and a typed-decode failure could also surface as EvaluationFailure.

## Custody, budgets and isolation

All eleven pins (Aiken binary 38,299,808 B `4257cc…7677`, its data-only manifest, and the nine generated inputs) plus the staged read-only tool (mode 555) and all five operator files are hashed before and after; a wrong size/hash fails before any command. Release identity is parsed from the pinned data manifest (v1.1.24, target `bacbeb35…acb4a`); no `--version` or other executable probe is run, and argv never self-claims a version.

Budgets chosen by the source author: 60 s wall per call plus 10 s kill-after, parent wait 80 s, CPU 30:35, 8 GiB address space, core 0, 16 MiB file-size and per-log caps; 7×60 s planned aggregate plus 180 s staging/final reserve equals a 600 s outer bound plus 10 s kill-after. No uninterruptible-death or transitive-CPU claim is made. Isolation is `bwrap --unshare-all --die-with-parent --new-session --clearenv` with the tool and inputs read-only, only a fresh capture directory writable, and no network, keys, wallet, stdlib/build trees or host mounts.

## Authority order and limits

`AUTH-CANDIDATE.json` is `CANDIDATE_NOT_AUTHORIZED`, carries a placeholder operator SHA for root to fill after materialization, and records the fresh actual-input audit as PENDING. The operator reads no future audit file and invents no audit hash: a real run requires root to first accept that pending audit, obtain a fresh full non-author source/effect/resource audit with two substantive votes, and issue an exact new AUTH attesting that acceptance. Only after the seven calls and a fresh actual-result audit may FX1, signatures, claim consume/atomic settle, qualified ledger cost or real Preprod be proposed.

The contexts retain fabricated `2222…/555…` values, zero fee and the unbound 2030 expiry: LOCAL FABRICATED / UNBOUND, never a live Preprod claim. Local VM costs under `ExBudget::default()` are not qualified ledger ExUnits, fees or tx-fit evidence. The Class-0 setup remains forgeable, and all 20 UTxO families, ACTUS, DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates and real Preprod remain open.
