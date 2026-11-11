---
title: "Midnight ledger 9"
type: entity
tags: [midnight, ledger9, on-chain]
sources: [c6-outcome-ledger9-review]
last_updated: 2026-11-10
---

# Midnight ledger 9

The target ledger, read at tag `ledger-9.1.0.1` (`a2b30128`). Review verdict on whether the chip leaf design runs on it: **yes with changes**, 3 of 3 ([[c6-outcome-ledger9-review]]).

- A call proof is verified with `midnight_zk_stdlib::verify`: PLONK, KZG, BLS12-381, Blake2b transcript.
- No maximum-k check; a comment says no verifiable circuit exceeds 2^14 rows, but it is not enforced, and a later release could enforce it.
- The data catalogue lists ceremony SRS files through `bls_midnight_2p25`, so the SRS does not exclude k = 21.
- The ledger and node tags resolve `midnight-proofs` 0.8.2; the workspace proves with 0.8.3. Qualifying 0.8.3 against 0.8.2 is open work.
- The chip leaf is verified only inside the node; nothing on chain sees the leaf or the node.
- No blocker in ledger 9. Open work: new contract, template and keys; no validated `acceptRun` on a 9.1.0.1 node; target `LedgerParameters`; transaction construction items.

All from [[c6-outcome-ledger9-review]]. See also [[RunDecider]], [[AcceptRunContract]].
