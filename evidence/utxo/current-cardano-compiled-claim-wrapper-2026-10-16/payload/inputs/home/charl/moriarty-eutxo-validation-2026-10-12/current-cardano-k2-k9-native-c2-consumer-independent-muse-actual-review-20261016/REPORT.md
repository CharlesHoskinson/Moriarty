# Fresh independent actual review — K9 native C2 consumer

## Verdict

**nativeC2ConsumerPrerequisiteClosed: true — for exactly `pvc2/claim.issue` seven-case LOCAL predicate, and nothing broader.** I did not author the K9 operator, builder, tests, source audits or K8 results, and I executed nothing in this review.

## What was observed

The unchanged production gate is [claim.ak](/home/charl/Moriarty/.worktrees/utxo/experiments/cardano-four-piece-settlement/lib/pvc2/claim.ak:49) `issue`: singleton mint/output expects, then `qty==1`, holder, quantity, single-token, no-refscript, `name==envelope_name(raw)`, `well_formed`, `vk_digest==blake2b_256(vk)`, and last `groth16.verify_digest(vk, proof, statement_digest)` (488-byte VK, 192-byte proof, statement digest c9ef8ea0...). The builder recomputes `H(serialiseData)` token/mint/output from the FULL tampered envelope for N1/N2, and N3 reuses the positive envelope with the actual A/C-swapped proof, so pairing is the first possible refusal for N1–N3. All 7 tests are normal tests with explicit `!claim.issue(...)`, no `fail` annotation, no duplicated acceptance, diag stub or host bool.

The actual run (RESULT.json, capture outer exit 0, wall 6.933s, CHILDREN CPU 0.249917s not transitive) returned both Aiken calls 0 in bwrap offline under 1400+10s scope, 1 attempt, 0 retries, 2 spawn commands, with summary 7/7/0, unit 7 property 0, all `fail_immediately`. Costs are Aiken VM only: positive mem 200075 cpu 2238951754; N1/N2 mem 207952 cpu 2241631930; N3 mem 200676 cpu 2239123803; early guards mem 90334/130616/178092 cpu 44949797/58721558/94235004. They are not a Preprod node cost model, tx budget fit, ledger ExUnits or on-chain acceptance.

## Scope and limitations

The local `a2a2..` policy is NOT the compiled C2 hash, expiry 1893456000000 is builder-chosen and unbound, and 2222../555../a2.. context is caller-fabricated fixture data, not real Preprod ledger data; fees, inputs, minADA and asset conservation are not fully ledger-checked by this placeholder transaction. C2 alone proves only claim/proof/digest under the VK; it does not authenticate outref, claimant, predicate, expiry, signature, current UTxO or unspent non-membership — S2 recompute, authenticated catalogue and signed intent remain future work. Class 0 forgeability, the unproved VK matrix and unmeasured transitive CPU are retained. No C2 APPLIED B9 compiled wrapper, genuine hash, FX1, VI actual signature, ledger ExUnits, wallet funding or real Preprod result exists yet. All 20 ACTUS 27718 types / 32 tax / DS01-07, DeFi 72 / DA01-24, general and recursion, private handoff, bounded multi-parent and universal obligations remain open, with no percentage claimed.

## First blocked boundary

If accepted, the next boundary is the compiled APPLIED B9 wrapper with genuine hash, FX1 and actual target cost/transaction qualification. The routine K8 ordinary metadata correction is separate existing-data work, not a native prefix repeat.

## Independence

Fresh independent muse-wonder-1.0 HIGH review in a new context under the indexed explicit user Gemini/Muse fallback after the original Opus author timed out; this is same-family freshness, not cross-provider independence, and no provider identity, authority or approval beyond this scoped verdict is invented. Source/auth are unchanged, all 18 host pins and the staged 9 AK + toml/lock + stdlib 38 + packages + tool hash/mode postchecks are source-observed (binary SHA 4257 is manifest-declared only, body not mirrored, no binary-from-source proof), and the typed actual claim function — not a validator wrapper — was compiled.
