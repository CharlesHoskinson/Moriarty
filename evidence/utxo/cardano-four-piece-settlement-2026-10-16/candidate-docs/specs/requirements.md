# Requirements: four-piece VS settlement (C2 → VI → VS3; VS1, VS2, VS4; S2′)

Status: PROPOSED source requirements (Opus security). Unreviewed, unrun. EARS form with OpenSpec-style scenarios. "Shall" binds the candidate in `candidate/aiken` placed per `SOURCE-CHANGE-MAP.json`. Each requirement names its consumer and its frozen tests (`EXPECTATIONS.json`).

## Requirement FW-ID: acyclic identity chain

- **FW-ID-1 (ubiquitous).** S2′ shall be parameterized by `(c2_hash, vi_hash, vs1_hash, vs2_hash, vs3_hash, vs4_hash)`, VS3 by `(c2_hash, vi_hash)`, VI by `(c2_hash)`; C2, VS1, VS2 and VS4 shall take no parameter. Consumers: `validators/*.ak`.
- **FW-ID-2 (ubiquitous).** The S2′ hash shall remain both the thread-token policy and the thread lock credential.
- **FW-ID-3 (unwanted).** If the C2, VI, four piece hashes and the S2′ policy are not seven distinct 28-byte values, or the piece list does not have exactly four entries, then S2′ genesis and every S2′ Settle spend shall refuse.

#### Scenario: aliased piece parameters
- GIVEN piece parameters with vs2 = vs1, a piece equal to VI, to C2 or to the policy, or only three pieces
- THEN `identities_distinct` is False (`q_alias_vs1_vs2`, `q_alias_vs3_vi`, `q_alias_vs4_c2`, `q_alias_vs2_policy`, `q_alias_three_pieces`), S2′ Settle refuses (`q_alias_s2p_refuses`) and genesis refuses (`q_alias_genesis_refuses`); the honest identities pass (`q_distinct_fixture_accepts`).

## Requirement FW-DEL: every piece is required at settlement

- **FW-DEL-1 (event).** When S2′ is spent with a constructor-0 (Settle) redeemer, it shall accept only if, for each k in 1..4, `Withdraw(Script(vs_k_hash))` is in the withdrawals, exactly one redeemer has that purpose, it decodes as `VsPieceV1`, its `piece` equals k, its `thread_policy` equals the spent input's script hash, and its `threads` contains the spent outref.
- **FW-DEL-2 (event).** When S2′ is spent with Revoke or Close, it shall run the unchanged `thread.revoke` or `thread.close`.
- **FW-DEL-3 (unwanted).** If a piece's slot carries a redeemer of another type, S2′ shall refuse (halt).
- **FW-DEL-4 (unwanted).** If any shard is invoked through a purpose other than withdraw, it shall fail (`else`).

#### Scenario: a missing, half-present or duplicated piece
- GIVEN the P01 transaction with VI and three of the four pieces, or a piece redeemer without its withdrawal, or a withdrawal without its redeemer, or two VS3 redeemers, or no shard at all
- THEN S2′ refuses (`q_s2p_missing_vs1..vs4`, `q_s2p_vs2_redeemer_without_withdrawal`, `q_s2p_vs4_withdrawal_without_redeemer`, `q_s2p_duplicate_vs3_redeemer`, `q_s2p_original_tx_refuses`).

#### Scenario: swapped or mis-typed pieces
- GIVEN tags 1 and 2 exchanged between the VS1 and VS2 slots, or a VI-typed redeemer in the VS1 slot
- THEN S2′ refuses (`q_s2p_swapped_piece_tags`, `q_s2p_piece_redeemer_wrong_type`), and each piece refuses a redeemer carrying another tag (`q_vs1_refuses_tag_2`, `q_vs2_refuses_tag_3`, `q_vs3_refuses_tag_4`, `q_vs4_refuses_tag_1`).

## Requirement FW-COV: exact coverage per piece

- **FW-COV-1 (event).** When piece k runs, it shall refuse unless its redeemer carries tag k, a 28-byte `thread_policy`, and `threads` equal to the non-empty covered set of that policy in transaction input order (the covered set is the parent's `shard.covered`, unchanged).
- **FW-COV-2 (event).** When VS3 runs, it shall also refuse unless VI's withdrawal is present with exactly one redeemer, which names the same `thread_policy` and the same outref list.
- **FW-COV-3 (ubiquitous).** Coverage, claim selection and custody checks shall read spent inputs only; a reference input shall never count as a covered thread or a consumed claim.

#### Scenario: partial, vacuous, permuted, duplicated or replayed coverage
- GIVEN P01 or P07
- WHEN a piece lists one of two threads, no thread, the reversed order, a duplicate, an extra absent outref or the P02 outref
- THEN that piece refuses (`q_vs2_one_of_two`, `q_vs4_one_of_two`, `q_vs1_empty_threads`, `q_vs4_empty_threads`, `q_vs3_permuted_order`, `q_vs2_duplicate_entry`, `q_vs1_extra_absent_outref`, `q_vs4_foreign_outref`), and S2′ refuses the thread a piece omits (`q_s2p_uncovered_thread`).

#### Scenario: cross-group pieces and producers
- GIVEN a piece or VI naming the foreign policy
- THEN S2′ refuses (`q_s2p_foreign_policy_vs4`) and the piece refuses (`q_vs1_foreign_policy`, `q_vs2_foreign_policy`, `q_vs3_vi_policy_mismatch`, `q_vs3_vi_coverage_mismatch`).

#### Scenario: reference inputs
- GIVEN the P01 thread moved to the reference inputs
- THEN it is not covered and VS1 and VS4 refuse (`q_thread_reference_input_not_covered`).
- GIVEN the P01 claim moved to the reference inputs
- THEN VS3 and VS4 refuse (`q_claim_reference_input_vs3_refuses`, `q_claim_reference_input_vs4_refuses`).

## Requirement FW-OBL: preserved settle obligations

- **FW-OBL-1 (ubiquitous).** Across VI and VS1–VS4, each of the 19 W1 settle components shall be evaluated exactly once per covered thread, except `global_outputs`, evaluated once per transaction in VS1; every signature shall be verified by VI. Allocation: DESIGN.md section 4.
- **FW-OBL-2 (ubiquitous).** No W1 component, stage function, signed-message byte, statement byte, ABI v2 type, Groth16 code or VI code shall change (`thread.ak`, `intent.ak`, `core.ak`, `types.ak`, `groth16.ak`, `claim.ak`, `shard.ak`, `vi.ak` byte-identical to C′).
- **FW-OBL-3 (event).** When VS3 builds a thread's statement, it shall use the shape digest it recomputed and the intents digest VI published for the same position.
- **FW-OBL-4 (event).** When VS4 builds a thread's next datum, it shall use the `statement_digest` of the one claim envelope that names the thread's outref; VS3 shall accept only if that digest equals the recomputed statement digest.
- **FW-OBL-5 (state).** While any script of a settle transaction refuses, no thread, claim, balance, duty, reserve or seq shall change (phase-2 all-or-nothing; PB-12 wording open).
- **FW-OBL-6 (event).** When an R1 BOOLEAN settle case is presented, exactly the pieces holding a False component shall refuse, without halting (`q_h*_split`; VS4 of H12 not predicted).

#### Scenario: financial outcome unchanged
- GIVEN P01..P05
- THEN the claim envelope digest equals `fx.stmt_digest_p0X` and VS4's next datum serialises to the R1 author's `fx.datum_post_p0X` (`fin_p01..p05_vs4_next_is_r1_datum_post`).

#### Scenario: limits, accounting and residual duties
- GIVEN H03–H06, H09, H43 (limit breaches), H11 (recomputation), H69 (bounds)
- THEN VS2 alone refuses among the pieces.
- GIVEN H26–H31 (frame, residual duties) and H20–H25, H44 (payouts)
- THEN VS4 alone refuses.

#### Scenario: double satisfaction
- GIVEN H45 (one output serving two threads) or H70
- THEN VS1 refuses once for the transaction (`q_h45_split`, `q_h70_split`).

#### Scenario: forged intermediate values
- GIVEN swapped or zeroed VI digests, or a claim envelope whose statement digest is forged
- THEN VS3 refuses (`q_vs3_swapped_vi_digests`, `q_vs3_forged_vi_digest`, `q_forged_claim_digest_vs3`) and VS4 refuses the forged envelope (`q_forged_claim_digest_vs4`).

## Requirement FW-GEN: genesis and shard credentials

- **FW-GEN-1 (event).** When S2′ mints with Genesis, it shall accept only if the unchanged `thread.genesis` accepts, FW-ID-3 holds, and no catalogue account's native credential and no reserve-owner payment credential equals `Script(h)` for h in {vi, vs1, vs2, vs3, vs4}.
- **FW-GEN-2 (ubiquitous).** SR-06 (also excluding `Script(c2_hash)` and the S2′ policy) is not implemented by this candidate and remains required before any deployment.

#### Scenario: shard credential as co-authority
- GIVEN the P08 catalogue with every account native set to `Script(vi_t)`, `Script(vs1_t)`, `Script(vs2_t)` or `Script(vs4_t)`, or the reserve owner set to `Script(vs3_t)`
- THEN `credentials_free` is False (`q_guard_refuses_*`), and True on the unmodified catalogue (`q_guard_accepts_fixture_catalogue`).

## Requirement FW-SIZE: deployment fit (measurement, not a unit test)

- **FW-SIZE-1 (ubiquitous).** Each of VS1, VS2, VS3, VS4 and S2′, applied, should be at most 14,336 B so that each can be deployed as a reference script in its own transaction. Status: PREDICTED only (EXPECTATIONS PS4-2, PS4-4). A miss is a failed prediction, preserved.
- **FW-SIZE-2 (ubiquitous).** The parent's PS-S4 failure (VS 20,503 B) shall remain recorded as observed; this candidate is a new candidate, not a retry.

## Not required here (open)

FX1, X1/PX04, OEX7, K2, M-HASH, M-REG/D-6 deployment confirmation, settle- and deploy-transaction size, ExUnits (PB-02), PB-01..PB-12, SR-06/08/09/12/15/16, the debtor-consent item, D-7, and every U0–U7, M1–M17, MC, SP and G exit.
