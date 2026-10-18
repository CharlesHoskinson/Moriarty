# Independent K10 compiled-wrapper operator source review

## Verdict: AGREE — source-ready candidate, no authority granted

The top finding is that the concrete five-file K10 compiled-wrapper operator correctly implements its finite sourced contract and contains no material source defect, while its candidate AUTH still cannot run. This review is a fresh non-author Muse Wonder 1.0 HIGH review under the indexed explicit user Gemini/Muse fallback after the original Opus source author timed out; it is not Opus approval, not the db759cf source author, and not the 00e128 actual-input auditor.

## Decisive evidence

The full candidate runner (`operator.py`, 26151 bytes, sha256 `7d591c1e...ada1c4`) evaluates only the already-compiled applied `c2.c2.mint` script (2770 CBOR bytes, policy `6a4b67e7...59c1`) through exactly seven sequential pinned Aiken `uplc eval --cbor /inputs/script.hex '<single 2649-byte context term>'` calls. All eleven pins (binary 38,299,808 bytes `4257cc...`, data manifest `8adde...`, and the nine generated inputs) match the indexed values and `ROOT-PUBLIC-PIN-CROSSCHECK.json`. The strict parsers were verified against the actual pinned Aiken source: T1 requires exit 0, empty stderr, exact `result/cpu/mem` JSON, the literal `(con unit ())` confirmed from `pretty.rs` Constant::Unit and `eval.rs`, and typed positive non-bool costs; T2-T7 require exit exactly 1, empty stdout, and the full newline-exact `Error/Costs` shape carrying only `the validator crashed / exited prematurely` from `machine/error.rs`, rejecting OutOfExError, syntax/Miette, signals, timeouts and trailing bytes. Guard names remain source-expected labels from `claim.issue` order, not runtime instrumentation.

The formerly PENDING fresh actual-input audit is now closed as ordinary data only by the fresh actual review `00e1287c...` and root K10 adoption `6a86c215...` over the nine files; the candidate's PENDING prose is historical source-time text. Root filled only `operatorSha256` in the candidate AUTH, leaving `CANDIDATE_NOT_AUTHORIZED` and the PENDING attestation unchanged, so the operator fail-closes today. Budgets and isolation are as specified (60s per call plus 10s kill-after, parent wait 80s, two 10s cleanup waits, CPU 30:35, 8 GiB AS, core 0, 16 MiB caps, plain logs, launch intent before `Popen`, `nativeExecutableStarted=not_measured`), with 600s read as the root outer hard bound (7x60+180), not a false 700s sum, and partial logs preserved if that bound hits.

## First blocked boundary and limits

The first blocked boundary is root materialization into the new uncreated public roots plus one exact new AUTH attesting the accepted actual-input audit, followed by the seven evaluations exactly once and a fresh actual-result audit; only then may FX1, signatures, claim consume/atomic settle, qualified ledger cost or real Preprod be proposed. The fabricated `2222../555..` context, zero fee and unbound 2030 expiry remain LOCAL FABRICATED / UNBOUND, local VM costs are not qualified ledger ExUnits, and Class-0 forgeability/VK-matrix provenance remain unproved. All 20 UTxO families, ACTUS, DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates and real Preprod remain open, with no percentage of full compatibility claimed.

## Independence and scope

Read-only review of the exact public mirror only: no shell/code execution, network, setup, RNG, proof, key, wallet, signing, ledger or nested agents were used. I did not author the candidate, operator, source review or results, and I invent no provider identity, authority or approval. Unread and unresolved items are listed explicitly in the coverage fields of this record.
