# Aeon experiment: bounded multichain signing request

**Status:** reproduced advisory type-checking result, not a Moriarty implementation or a cryptographic proof. **Date:** 2026-09-29.

## Question and model

Can an Aeon refinement check that a proposed signer call keeps an authorized domain, derivation path, canonical payload digest, recipient, amount bound, key epoch and nonce? Can the type level distinguish a signed payload from a broadcast transaction and a finalized effect?

The [probe](probe.py) encodes identifiers and a digest as small `Int` tags. `authorize` accepts only domain `1`, path `7`, digest `314159`, recipient `42`, value in `1..100`, epoch `5` and nonce `9`. It returns an encoded `signed` phase. Separate refined phase tags `1`, `2` and `3` represent signed, broadcast and finalized. These are deliberately **abstract tags**, not serialized chain transactions, signatures, receipts or proofs.

## Reproduction

Source: [`alcides/aeon`](https://github.com/alcides/aeon), `master`, pinned commit [`ef66bd95e6b7d2d5309453ee63640bc7fa1d988f`](https://github.com/alcides/aeon/tree/ef66bd95e6b7d2d5309453ee63640bc7fa1d988f), local checkout `/home/charl/.graphify/repos/alcides/aeon`. Python 3.13.15 from that checkout's `.venv`. The probe uses `AeonDriver` with `strict_decidable=True`, zero synthesis budget and no entry point. Its trust inspection uses `compute_trust_report`. The earlier [Moriarty Aeon study](../../aeon-study-2026-09-19/RESEARCH.md) documents this pinned compiler and its trust frontier.

Run from the Moriarty worktree root:

```sh
/home/charl/.graphify/repos/alcides/aeon/.venv/bin/python deliverables/aeon-kernel-experiments-2026-09-29/signing/probe.py > deliverables/aeon-kernel-experiments-2026-09-29/signing/run.log 2>&1
```

The command exited `0`. Full machine-readable outcomes are in [results.json](results.json), the raw stdout is in [run.log](run.log), and each generated Aeon input is a separate `.ae` file in this directory.

## Observations

| Group | Actual result | Interpretation |
| --- | --- | --- |
| Exact request; bounded dynamic value | Both accepted | The encoded refinements can constrain a fixed request and a caller-provided bounded amount. |
| Wrong domain, path, digest, recipient, epoch or nonce | All six rejected with `LiquidTypeCheckingFailedRelation` | Aeon checked each substituted field against its declared predicate. |
| Amount `101`, amount `0`, unconstrained dynamic amount | All three rejected with `LiquidTypeCheckingFailedRelation` | The source obligation covers both constant violations and a caller whose input lacks a bound. |
| Signed used as broadcast; signed used as finalized; broadcast used as finalized | All three rejected with `LiquidTypeCheckingFailedRelation` | Distinct encoded phase types prevent these substitutions in the checked source. |
| Literal finalized token | Accepted | A declared token with the expected tag passes; it does **not** show that real finality was observed. |
| `native "1"` declared as finalized token | Accepted; trust report identifies `observed` as `native` | A native promise can satisfy the type checker without external evidence. |

The successful and rejected cases support a narrow proposal: an off-chain authoring checker can diagnose mismatched signing parameters and misuse of lifecycle phases **if** the checked program carries these constraints. The finality-native case is an explicit counterexample to treating an Aeon refinement as a ledger fact.

## Consequence for the kernel boundary

Keep the user-signed Moriarty envelope and a pinned adapter codec as the authority for each `SignRequest`. The kernel may ask an MPC service to sign only after matching the exact domain, path, payload commitment, recipient, value, key epoch and nonce against that envelope. Store separate receipts for signature, submission, inclusion and finalization. A signature must not cause the language transition that requires finality. An imported finality claim needs a named verifier or attestor premise; a native declaration or host Boolean cannot supply it silently.

This experiment did not test canonical serialization, hash collision resistance, MPC threshold behavior, signer equivocation, cross-chain nonce semantics, replay protection, actual broadcast, foreign-chain finality, or the Moriarty compiler/ledger path. No transaction was submitted. Integer tags make the checked predicate visible but cannot establish that an adapter's bytes match those tags. The next executable discriminator is an exact-byte adapter fixture checked against a signed policy, followed by independently observed chain effects and a negative wrong-byte request.
