# Luna pass 2: compiler/native/crypto/ledger graph

Base: `cb8280ef54082853dd8bb6c6a52050328d0fc6fa`. This is an advisory map of tracked public source and requirements. It does not establish product acceptance.

## Boundary map

`mori-ledger` exposes several distinct stages. Static compilation accepts source, parsed program, bindings, an explicit authorization mode, and work limits, then exports a bounded source-static bundle. Its declared qualification is unproven/uninstalled. It cannot be promoted through its public types directly into installed, relation, or proving capabilities. The ordinary relation compiler checks source identity, validates bindings, creates ZKIR and a manifest, and records open predicates. `relation::check` checks a supplied proof preimage against that relation; the module explicitly says ordinary checking is not a proof.

The local preparation path consumes supplied deployment/frame data, predecessor state, parameters, and a call request. It runs the supplied operation and checks transcript, relation projection, state and gas readback. The custody helper constructs a prospective local offer from an admitted predecessor and execution. Those operations give useful deterministic local evidence, while the readback explicitly leaves fee payment, settlement and deployed qualification unperformed.

Native APIs are present. The prover calls the SDK intent prover and then verifies the returned intent. The verifier checks a V3 proof with the `Real` ledger verifier against a supplied operation and the independently prepared statement. Its returned scope and open-predicate list still exclude independently trusted SRS/setup provenance, exact SDK keygen correspondence, the full source-constraint theorem, authenticated installation/predecessor, account authority, replay, fees, application/settlement, and recursive private composition. Presence of these APIs is therefore a partial capability, not U1 or U2 acceptance.

The signed path binds a fixed local JubJub/FENS public key policy, owner/beneficiary labels, request payload, and host-verified signature. The signature module states that its equation does not establish account-label authority. Signed-v2 adds a domain-scoped nonce map over supplied state, but its module says it does not authenticate history. This supports local intent and transition checks; it does not by itself establish chain authorization or unique public consumption.

## Evidence classes kept separate

- **Source-static outputs:** key-free, bounded, unproven/uninstalled artifacts and their manifests. Hashes and recompilation checks establish artifact identity within the source path; they do not certify meaning, soundness, key provenance or ledger acceptance.
- **Local cryptographic evidence:** `ROADMAP.md` records a bounded local signed financial circuit proof and hostile rejections. It labels this one laboratory case and leaves account/key ownership, authenticated state/history, nonce, Preview settlement, recursion and privacy open. This historical result was not rerun in this pass.
- **Public network acceptance:** the same roadmap records a previous Preview attempt rejected for excess-NIGHT test genesis with unchanged state. No financial Preview acceptance is established here. Local proof or a signed transaction is not a settlement result.
- **Canonical pins:** workspace dependencies pin the Midnight ledger/ZKIR crate family to revision `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`; `mori-ledger` enables the proof-verifying feature and depends on ZKIRv3. These are source dependency pins, not a complete pin of the deployed compiler, circuit/key, verifier, runtime and ledger instance required by the product contract.
- **Unresolved provenance and correspondence:** source code includes a source-closure digest, but comments define it as declared source identity rather than a complete Rust build or SRS certificate. The product contract still requires adversarial-witness soundness, source/compiler correspondence, per-primitive cost and target pins.

The roadmap specifies U0–U2 and U4 as accepted artifacts, not merely APIs: one semantic and target matrix; a certified minimum basis; two distinct newly authored source programs through actual pinned ZKIRv3/native verification and complete effects; then recursive financial history and separate-party private composition. U4 depends on U2, U3 lifecycle invariants and accepted SP07/SP08 semantics. M1–M10 remain part of the retained M1–M17 line, but the inspected current milestone table does not give a complete item-by-item M1–M10 status. I left those exits unclaimed.

## Concrete audit pressure

Focus audits on (1) capability promotion and manifest/key/proof cross-binding; (2) grammar coverage and adversarial witnesses that satisfy constraints while breaking source semantics, framing, complete custody/balance or liability accounting; (3) the exact boundary between supplied state and authenticated current ledger state; (4) nonce races, reorg/replay and phase failure; (5) fee recipients, guaranteed/fallible phase effects, gross debits, net outcomes and residual duties under the same signed statement; (6) exact workspace versus deployed target identities; and (7) the evidence needed for independent SRS/keygen provenance and U4 recursive/private handoff.

The JSON contains the bounded node/edge graph, per-capability evidence and gaps, coverage limitations, audit questions, and SHA-256 digests for inspected files. No native SDK operation, build, test, proof, VM run, chain query or campaign was performed.
