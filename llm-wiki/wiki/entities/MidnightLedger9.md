---
title: "Midnight ledger 9"
type: entity
tags: [midnight, ledger9, on-chain]
sources: [c6-outcome-ledger9-review, cost-lane-fable, elastic-chip-gpu-fable, n1-native-circuits, prover-graph, n1b-native-proving-time]
last_updated: 2026-11-11
---

# Midnight ledger 9

The target ledger, read at tag `ledger-9.1.0.1` (`a2b30128`). Review verdict on whether the chip leaf design runs on it: **yes with changes**, 3 of 3 ([[c6-outcome-ledger9-review]]).

- A call proof is verified with `midnight_zk_stdlib::verify`: PLONK, KZG, BLS12-381, Blake2b transcript.
- No maximum-k check; a comment says no verifiable circuit exceeds 2^14 rows, but it is not enforced, and a later release could enforce it.
- The data catalogue lists ceremony SRS files through `bls_midnight_2p25`, so the SRS does not exclude k = 21.
- The ledger and node tags resolve `midnight-proofs` 0.8.2; the workspace proves with 0.8.3. Qualifying 0.8.3 against 0.8.2 is open work.
- The chip leaf is verified only inside the node; nothing on chain sees the leaf or the node.
- No blocker in ledger 9. Open work: new contract, template and keys; no validated `acceptRun` on a 9.1.0.1 node; target `LedgerParameters`; transaction construction items.

All from [[c6-outcome-ledger9-review]].

## The 2^14 verifier parameters (cost and elastic chip rounds, 2026-11-11)

- Ledger 9 (`ledger-9`, `f15fb9d`) embeds verifier parameters of a 2^14 setup (`VERIFIER_MAX_DEGREE = 14`, documented as limiting the number of public inputs); a comment says no verifiable circuit exceeds 2^14 rows. Native `zerocash` `spend` is k = 15 (measured, [[n1-native-circuits]]).
- [[cost-lane-fable]] section 7: the ledger wrapper compares no k with the parameters and the data provider lists parameters to at least `bls_midnight_2p18`; whether verification inside `midnight_zk_stdlib` succeeds for a k > 14 key was not traced. Open; experiment C7 on ledger 9 with a k > 14 key. Ledger limits that hold: verifier keys over 50,000 bytes refused, 256 operations per contract, 65,536 public inputs per call.
- [[elastic-chip-gpu-fable]]: confirmed from sources, not re-run. `ParamsVerifierKZG` holds only G2 elements, so the embedded file does not bound circuit size for non-committed instances; a prior run verified a k = 20 decider with the stock and installed ledger verifier. Experiment X-D0 repeats it on a GPU proof.
- Prover pin: the ledger 9 lock names `midnight-proofs` 0.8.2; Moriarty uses 0.8.3; `midnight-zk` HEAD changes the proof format ([[elastic-chip-gpu-fable]] F-D3).
- N1b proved all ten native Compact entries on the ledger 9 proof-server pin (`midnight-zkir` 3.2.0 on `midnight-proofs` 0.8.2); the ceremony 2p21 and 2p22 files match the hashes pinned in ledger 9; the ledger 9 `zkir-v3` crate 3.0.0 refuses zkir 3.1 from `compactc` 0.36.0 ([[n1b-native-proving-time]]).
- The ledger 9 proof server, `midnight-zk` and `midnight-zkir` are graphed in [[prover-graph]]. See also [[RunDecider]], [[AcceptRunContract]].
