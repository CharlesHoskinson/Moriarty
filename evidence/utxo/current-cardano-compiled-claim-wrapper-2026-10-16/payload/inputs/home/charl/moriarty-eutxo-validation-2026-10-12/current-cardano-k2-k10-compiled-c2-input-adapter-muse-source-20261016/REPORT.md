# K10 compiled-C2 input adapter — source author report

## Verdict

The five requested source files are authored completely and are ready for a fresh full source/effect audit. Nothing has been executed, generated, or authorized. The compiled wrapper can be bound to its actual (tool-declared) policy without rerunning the passed K8 pairing or K9 typed-consumer slices, and the one genuine primary-source gap — independent PlutusV3 script-hash recomputation — is stated explicitly with the exact crate source to acquire, rather than filled with an invented formula.

## What the adapter does

`tools/build_k10_inputs.py` (stdlib Python, DATA-only) verifies the pinned `b9-applied.json` and `k8_fixture.ak` by SHA-256 before and after, extracts the shared `c2.c2.mint` compiledCode (2770 CBOR bytes) into `script.hex`, and emits seven `(con data (...))` PlutusV3 ScriptContext terms plus a deterministic manifest into a fresh `current-cardano-k2-k10-public-inputs-generated-20261016` directory. The genuine 488-byte VK / 192-byte proof and the N1/N2/N3 public constants are taken from the K8 fixture constants carried in the mirror; their digests are recomputed from the public bytes during generation. The token name is blake2b-256 of the EnvV2 datum CBOR via a minimal encoder grounded in stdlib `cbor.ak` and `convert_constr_to_tag`, checked against a field-by-field frozen positive preimage; byte strings over 64 bytes fail closed (unreachable here: every envelope byte field is at most 32 bytes, and the 488-byte VK / 721-byte statement never enter the hashed datum).

## Key findings

- The wrapper is real: `validators/c2.ak` mint `Issue` delegates to `claim.issue`, and all three c2 titles share one compiledCode and hash with no parameters. Rejections from the compiled wrapper will be non-zero `uplc eval` exits (eval.rs error path), not K9-style Bool false.
- Policy identity is **tool-declared only**. `ast.rs` delegates hashing to Pallas `PlutusScript::<3>(cbor).compute_hash()`; that implementation (pallas-primitives / pallas-traverse 0.35.1, per Cargo.lock) is not in the mirror, so no independent-hash claim is made. The adapter still uses `6a4b67e7…59c1` everywhere so the wrapper's own holder/policy checks bind to the compiled identity, and never carries over the K9 locally chosen `a2a2..` policy.
- The PlutusV3 context shape follows `to_plutus_data.rs` exactly (16 TxInfo fields, bare-Coin fee, zero-Ada-omitted value maps, InlineDatum Constr 2, `interval.everything`, bare-bytes tx id, Minting Constr 0), and the eval interface is the confirmed `uplc eval --cbor <hex-file> <single (con data ...) term>` with single-JSON success output.
- Seven expectations are frozen: genuine accepts; coherent N1/N2 rebuilds and the N3 A/C swap are refused by the pairing; quantity-2, token-name bit-flip, and VK-digest bit-flip are refused at their named first guards.

## Scope, limits, and first blocked boundary

The context is locally fabricated in K9 placeholder style (2222../555.. fixtures, unbound 2030 expiry, zero fee): semantically well-formed for the wrapper, but not a ledger transaction, and no fee, conservation, admission, wallet, signing, or Preprod claim follows. Class-0 single-party setup forgeability, the VK matrix, all 20 UTxO families, ACTUS/DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, and universal certificates remain open.

First blocked boundary: **execution**. A fresh full source/effect audit, ordinary data generation, a fresh actual-input review, and a separate new exact bounded native AUTH must precede any native run; in parallel, any independent-hash claim waits on acquiring the exact Pallas 0.35.1 ComputeHash source. This source is an author FOR position only — it is not independent approval, not an AUTH, and not evidence of target acceptance.
