# K10 compiled-C2 input adapter (source-only)

## What this is

A DATA-only Python (stdlib) adapter that prepares the exact inputs for executing the **already compiled** APPLIED B9 `c2.c2.mint` minting validator — the real Aiken wrapper whose `Issue` branch calls `claim.issue` — under the pinned Aiken `uplc eval --cbor` interface, with the genuine public Groth16 VK/proof payload and coherent tampered contexts. It replaces the K9 locally chosen `a2a2..` test policy with the compiled artifact's policy everywhere, so the full wrapper (singleton mint/output expects, holder check, token-name guard, VK-digest guard, and the pairing checked last) is what would run.

Nothing here executes a validator. Execution needs a fresh full source/effect audit, ordinary data generation, a fresh actual-input review, and a separate new exact bounded native AUTH (see `EXECUTION-PROPOSAL.json`).

## Files

| File | Role |
|---|---|
| `tools/build_k10_inputs.py` | The adapter: pinned-input verification, envelope CBOR/token-name derivation, PlutusV3 ScriptContext term generation, deterministic manifest. |
| `EXPECTATIONS.json` | Source-frozen expectations for the seven tests, the context shape, and the compiled-artifact identity classification. |
| `EXECUTION-PROPOSAL.json` | The exact future native command shape, budgets discipline and isolation; grants no authority. |
| `DESIGN-POSITIONS.json` | The source author's explicit positions (FOR) on every consequential clause. |

## Generation (ordinary data step, later)

```
python3 tools/build_k10_inputs.py <b9-applied.json> <k8_fixture.ak> \
  BASE/current-cardano-k2-k10-public-inputs-generated-20261016
```

Outputs: `script.hex` (verbatim `c2.c2.mint` compiledCode), `contexts/K10-T*.uplc.txt` (one `(con data (...))` PlutusV3 ScriptContext term each), and `manifest.json`. Inputs are SHA-256-pinned and re-verified after generation; a non-empty output directory is refused.

## Identity honesty

The policy `6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1` is **tool-declared** (the artifact's `hash` field, echoed in `hash-c2.txt`). It is not independently recomputed: Aiken's `ast.rs` delegates to Pallas `PlutusScript::<3>(cbor).compute_hash()`, and that Pallas source (0.35.1) is not in the public mirror. Acquiring it is the named next source step; until then no independent-hash claim is made, while the adapter still binds the compiled wrapper to its declared identity so the wrapper's own checks exercise it.

## Limits

The context is locally fabricated (K9 `2222../555..` fixtures, builder-chosen unbound 2030 expiry, zero fee, empty inputs) — semantically well-formed for the wrapper, but not a ledger transaction and no basis for fee, conservation, admission or Preprod claims. The single-party Class-0 setup could forge proofs; all broader obligations (20 UTxO families, ACTUS/DeFi, recursion, private handoff, multi-parent PCD, universal certificates, real Preprod) remain open.
