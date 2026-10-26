# Sherlock Lean specification

This package checks the public mathematical specifications under `Clean/Sherlock`.
It uses Lean's `Init` library and has no external dependencies. It does not contain
the private Clean implementation or establish Rust, constraint or native refinement.
It is maintainer tooling; public compile/prove/deploy commands do not require it.

The toolchain is pinned to Lean 4.33.1, matching the existing adapter's recorded
toolchain. Historical observations are not fresh verification of changed sources.
Build products go to the repository's ignored `target/sherlock-lean` directory.

From the repository root, run the full build through the shared heavy runner:

```sh
scripts/doctorwatson heavy --id sherlock-lean-build -- \
  memcap 48G lake -d clean/fork build
```

The heavy runner takes `~/.cache/doctorwatson/heavy.lock` and runs preflight.
If another job holds the lock, retry later. If a build fails, inspect its trace
and supply the one-line hypothesis using `--hypothesis` on the next heavy call.
Do not bypass a failed memory cap or the shared lock.

Check an individual module after its imports have been built:

```sh
memcap 48G lake -d clean/fork env lean \
  Clean/Sherlock/DecodedCounterAdapterAxioms.lean
```

Each new module's metadata describes its implementation sources, scope, verification
and remaining obligations. `Axioms.lean` queries expose theorem dependencies;
custom axioms or `sorry` require an explicit entry in the corresponding metadata.
