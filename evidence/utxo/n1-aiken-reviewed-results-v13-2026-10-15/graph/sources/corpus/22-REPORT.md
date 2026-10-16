# Independent actual-result review: P3S1-R1 Aiken H/A1/A2 run

**Reviewer:** `claude-opus-5-5` (runtime-stated; high effort requested, not self-observable). This is a fresh context, and I authored none of the code, source plan, expectations, oracles, reviews or root metadata. Every earlier author, reviewer and voter in this lineage is the same model family, so my independence is at the context level only. Per the security routing, the security, ledger and cryptographic judgments below are mine. They are not the root's.

**Reviewed run:** `current-cardano-p3-r1-aiken-local-execution-20261015`, pinned by root `RESULT-MANIFEST.json` (163 task files + 84 root records; binary body excluded, pinned `4257cc8b…7677`, mode 0555).

## Bottom line

- **Predicate: YES, qualified.** The immutable run establishes the exact scoped outcomes and nothing more: local CEK library agreement for the 16-file package, plus compiled identities. All 146 test names, modules, statuses and expected-failure flags equal the frozen index exactly. All 25 steps exited 0, with 0 retries. Effect, resource, source, counter and expectation integrity hold, within the stated observation limits.
- **Recommendation: adopt A1 and A2 as scoped evidence only.** A1 is GREEN library agreement through `test_ctx` under aiken v1.1.24+bacbeb3 / stdlib v2.2.0 at trace level silent. A2 provides compiled identity and byte-size records. Adopt neither as ledger, cost, deployment, signature, proof or settlement evidence.
- **Hard Preprod blocker (my judgment): S2 cannot be deployed as compiled.** The applied S2 is 30,764 bytes. Preprod's observed `maxTxSize` is 16,384. The ledger rule `txsize ≤ maxTxSize` (pinned implementation and formal source) applies to every transaction. That includes the transaction that would create a reference-script output, so neither an inline witness nor a reference script can carry S2. C2 (2,911 B) fits.
- **Second Preprod blocker (my judgment, lower evidence grade): settle cost.** These are host CEK figures, not ExUnits. Even so, every one of the 80 settle-path tests exceeds the observed per-transaction memory limit of 17.5 M; the lowest is 26.26 M. I subtracted the harness decoding cost estimated from the run's own tests. A single-thread settle still comes to roughly 2.8–3.7× the per-transaction memory limit and 1.4–1.9× the step limit. The two-thread atomic case (P07) probably exceeds the per-block memory limit. Only a compiled-wrapper evaluation (X1) under the pinned Preprod cost model can settle this. A cost-model difference alone is unlikely to close a gap this large.
- **Next capability:** source work to cut S2's size and cost and split it into pieces. See `NEXT-CAPABILITY.md`. It preserves every financial, authority, frame, residual-duty, recursion and multi-parent duty. No funding, key, proof or ledger action is approved here.

## 1. Test results: independent comparison (all 146)

I read the raw `logs/A1.log` in full (1,376 lines). I also read the root extraction `A1-TEST-RESULTS.json` (1,370 lines), `receipts/A1-tests.tsv` (147 lines) and the frozen `data/EXPECTED-A1-TESTS.json`, all in full. All comparisons were done by reading. I used no parser or diff tool.

| Check | Result |
|---|---|
| Raw log vs root extraction | The JSON block (log lines 6–1375) aligns line for line with `A1-TEST-RESULTS.json` lines 1–1370, at a constant offset of 5. I checked every title, status, `on_failure`, `mem` and `cpu`. Spot anchors at the start, middle and end of each module matched (for example p01 74,140,029; p07_1 208,714,819 / 56,964,090,172; statement_refuses_short_claimant 853 / 2,247,543). |
| TSV vs log | All 146 rows plus the summary/seed row agree with the log. |
| Names, order and modules | `pvc2/settle_r1_test` has 110 tests, `pvc2/unit_r1_test` 9 and `pvc2/unit_test` 27. Every name equals the frozen index, in the same order. There are no duplicates, none missing and none unexpected. |
| Status | 146/146 `pass`. Summary: 146 total, 146 passed, 0 failed; unit 146, property 0. |
| Expected-failure flags | All 37 `fail:true` entries carry `succeed_eventually`, and all 109 others carry `fail_immediately`. Per module: settle 27 (6 placeholder-signature refusals, 10 ERROR-PATH, 11 `_refuses`), unit_r1 1, unit 9. |
| Seed | 1945995588. It is irrelevant because there are no property tests. |
| Log tail | `Summary 146 checks, 0 errors, 0 warnings`. There was no fetch line; `Resolving dependencies` resolved offline (see §3). |
| First discriminators | p02 facts, h56, h37, h60, h59, h63 and every positive all pass. |

**What the 37 `fail` tests prove.** Each one passes if its predicate either errors or returns False. The 10 ERROR-PATH tests and the 6 placeholder-signature tests are therefore refusal evidence only. With trace level silent, the run records no failure location. The cost profile weakly corroborates the documented locations. For example, `p01_r1_full_refuses_placeholder_signatures` costs 74,127,913, almost the same as `p01_r1_facts_match_fixtures` at 74,140,029. That is consistent with a failure after the facts are complete, at the signature check. h33, h40, h41 and h64 cost 26–29 M, which is close to decode-only cost and consistent with an early `expect`. This is not isolation evidence (AR-08).

**Identical costs.** h28, h30 and h31 have the same cost (76,608,381 / 21,122,584,936). So do h46_parts and h58_parts, and two `cells_ordered_refuses_*` unit tests. Equal-length mutations on the same path would explain this. The test sources name distinct fixture constants. I did not read the fixtures, so I cannot confirm that the fixture bytes differ (AR-11).

## 2. All 25 steps: results, logs, freezes and identities

I read every `LAUNCH`, `OBSERVATION` and `TOOL-RESULT` record (75), every `.exit` file (25, all `0`), every log (23; P1 and F2 have none by design), every journal (25) and the receipts listed in `READ-COVERAGE.json`.

| Item | Observation |
|---|---|
| Order | P1 → P0 → P2 → P3 → P4 → H1–H5 → Hp → D1 (21:31:45.98) → A1pl → A1f → A1 (A1-TF) → A1p → A1x → D2 (21:37:08.69) → A2b (A2b-TF) → A2bp → A2t → A2a → A2h → A2r → A2p → F1 → F2. Journal start/end times are strictly sequential and never overlap. Each OBSERVATION time follows its journal end. |
| Literal launches | All 25 have the same `env -i … bash --noprofile --norc <step>` argv, `cwd /` and `stdin /dev/null`. Every `fileSha256` equals the `PRE-LAUNCH.json` pin. Exactly one TF/TL variant ran for A1 and for A2b. |
| Counters | 25 reserved, 25 completed, 0 retries (`ACCOUNTING.json`). |
| D1 | TF. The quoted `--trace-filter` and `--trace-level` blocks in `D1.json` equal frozen `help/check.txt` (H2 log lines 65–94); both show their values under `[possible values: …]`. `check` defaults to `verbose` and `build` to `silent`, so the explicit `silent` was necessary. |
| D2 | All four criteria hold against the H3, H4 and H5 help, which I read in full: build has both options with their values; apply has `--in --out --module --validator`, a positional `[CBOR]` and the "Plutus Data (CBOR, hex-encoded)" description; hash has `--in --module --validator`. The final GREEN label was deferred until after F1 (RE-11 applied). |
| Tool identity | H1 `aiken v1.1.24+bacbeb3`. Hp, A1p, A2bp and A2p each re-checked the binary sha and mode 555. `bin-stat.txt` and `staging.json` record copy mode 0600, then 0555, with the DATA-ONLY original 0644 (F1). |
| Network | P4 shows `/proc/net/dev` in a fresh netns with only `lo`, at 0 bytes and 0 packets. Every aiken call ran under `unshare --user --map-root-user --net` with `env -i` and task-local HOME, XDG and TMPDIR. |
| Hygiene | All five empty directories stayed empty at Hp, A1p, A2bp and A2p (an explicit `find … -quit` gate). The `tmp/` listings after H, A1 and A2 are 0 bytes. `build/packages` held exactly 2 entries. `aiken.lock` holds one package, stdlib v2.2.0, with an empty `[etags]`; its sha `ec5d6a58…da83` equals the source author's prediction. |
| Run tree | 56 files before the check (A1f, as predicted); 58 after (adds `aiken.lock` and an empty `build/aiken-compile.lock`, as predicted); 59 after the build (adds `plutus.json`). The other 56 rows are unchanged; I compared the full `run-before.json` and `run-after-A1.json`. |
| Sources | P0, A1pl, A1p, A2bp and F1 re-checked the R1 manifest, the 16 package files, the Sol helper, the reused tools, the stdlib pin and its 38 files, and the DATA-ONLY binary and manifest. All show `OK`. F1 confirmed no `build`, `aiken.lock` or `plutus.json` in the R1 tree. |
| Freezes | `help-freeze` holds 5 files; `run-before`, `run-after-A1`, `run-after-A2b` and `run-after-A2`; `a2-freeze` holds 2; `task-final` holds 77 (2 a2 + 5 help + 47 logs + 23 receipts). Every hash I compared in `task-final` equals the corresponding root `RESULT-MANIFEST` row. That is cross-record agreement only; I recomputed no hash. Journals are outside F2 by design and are root-pinned only (AR-14). |
| Tool-result timing | `TOOL-RESULT.wall_time_seconds` is not the step duration (A1: 0.82 s against 176.07 s in the journal). The A1 tool output was truncated. The root correctly read the log file (AR-12). |

### Host resources (A1)

The A1 step spanned 21:31:56.906 to 21:34:52.989 in the journal (176.08 s), and `/usr/bin/time` recorded 2:56.07 wall. Bash `times` for the children gives 5m30.447s user and 5m6.026s sys, matching `time` (330.44 / 306.01). CPU was 361%. Max RSS was 424,668 KiB, about 1.3% of the 32 GiB AS cap. The wall time was about 9.8% of the 1,800 s step cap. There were 24.76 M voluntary context switches, which accounts for the high system time. Filesystem outputs were 32 blocks. Linux does not fill the socket counters, so they show nothing. The precedent's 12.3 s check covered a different package at verbose trace level and is not comparable. A2b took 4.78 s.

## 3. A2 identities and what the byte count means

| Record | Value (log, receipt and root records agree) |
|---|---|
| Unapplied blueprint | `8444db4f…6d45`, 236,796 B; byte-equal at `run/plutus.json` and `a2/plutus.unapplied.json` (0444), re-verified at A2a and A2p |
| Applied blueprint | `aa6d4627…cc77`, 236,529 B |
| C2 | hash `23bdcc2d…4f19`, the same for all three handlers and in both stages; 2,911 B; compiledCode sha `6b410078…0a38` unchanged by apply |
| S2 unapplied | hash `5323f552…18c9`, 30,728 B |
| S2 applied | parameter `581c‖C2` (a CBOR byte string of length 28, i.e. Plutus Data `B`); hash `338293e4…1b7e` from both the blueprint field and `blueprint hash`; 30,764 B (+36 B from application); compiledCode sha `fb73af71…35d6` |

**What the byte count measures.** A2t and A2r compute `len(bytes.fromhex(compiledCode))`. I looked at the opaque C2 hex in the applied blueprint: it starts `590b5c`, a CBOR byte-string header for 2,908 bytes, giving 3 + 2,908 = 2,911. So each figure is the single-CBOR-wrapped flat script, which is the script's byte form. A transaction that carries the script (in the witness set, or as `script_ref` in an output) adds further CBOR framing on top. The exact framing is in the Babbage/Conway TxOut CDDL, which is not in the packet. The script therefore occupies **at least** 30,764 bytes of any transaction that carries it.

**Can it fit Preprod today? No, for S2.** This is my ledger judgment.

- *Rule (pinned source, commit `56ddee12`, source-only):* Conway `UTXO` calls `Babbage.babbageUtxoValidation` (`Conway/Rules/Utxo.hs` L253). That runs `Shelley.validateMaxTxSizeUTxO pp tx`, `{- txsize tx ≤ maxTxSize pp -}` (`Babbage/Rules/Utxo.hs` L420–421). The formal Conway `UTXO-inductive` has the premise `txsize ≤ maxTxSize pp` (`formal/…/Utxo.lagda.md` L554). Formal UTXOW requires every needed script either in the witnesses or via a reference input (L252).
- *Limit (provider-observed grade):* `maxTxSize` 16,384, from an anonymous Koios HTTP response (epoch 319, block 5,296,620, slot 136,417,272, retrieved 21:41:35Z). It carries no consensus, finality or authentication proof. The root states that the raw bytes equal the earlier capture.
- *Consequence:* an inline S2 witness makes the transaction ≥ 30,764 B. A reference-script output must be created by a transaction whose body contains that output, so that transaction is ≥ 30,764 B too. Both exceed 16,384 by ≥ 14,380 B (1.88×). The reference-script mechanism moves cost to the spending side; it does not lift the size bound on the creating transaction. Short of a governance change to `maxTxSize`, which is outside our control and should not be assumed, **S2 needs at least a ~50% byte reduction or a split into several scripts, each well under 16 KiB.**
- *Terminology:* A2r prints "reference-script bound for comparison (bytes): 16384". 16,384 is `maxTxSize`, not a reference-script limit. The ledger's reference-script limits are a separate per-transaction total (`refScriptsSize ≤ maxRefScriptSizePerTx`, formal L555; implementation `validateRefScriptSize`, `Conway/Rules/Ledger.hs` L467–481) and a per-block limit. Their values are defined in Conway `PParams`, which is not in the packet. The root was right not to adopt that wording.
- *Not limits on this:* `maxValueSize` 5,000 limits only the serialized `Value` (`Alonzo/Rules/Utxo.hs` L420–444), not `script_ref`.
- *Deployed identity:* I do not assume the deployed Preprod node matches commit `56ddee12`. The `txsize ≤ maxTxSize` rule appears in both the pinned implementation and the formal spec. That it applies on the deployed node is my inference, not a verified fact.

**Independence of the S2 hash.** The blueprint `hash` field and `aiken blueprint hash` both come from the same tool. Their agreement shows tool self-consistency; it does not show an independent recomputation (AR-04). Before any signer anchors an intent to `338293e4…`, a non-aiken implementation should recompute it as blake2b-224 over the language-tagged script bytes, with the rule taken from pinned Alonzo `Scripts` source.

## 4. Cost: what the CEK numbers do and do not show

These are host CEK `execution_units` from aiken's own evaluator and default cost model, at trace level silent. **They are not ledger ExUnits.** The tests also include harness work that a deployed validator would not do:

1. `test_ctx.context_of` runs `aiken/cbor.deserialise`, a UPLC-level CBOR parser, over the whole fixture ScriptContext. On chain, the ledger passes the context as Data.
2. `accepted(ctx, n)` evaluates `facts` and then `settle_checks`, about two full passes. The p01 accept test (148.3 M) is 2.0× p01 facts (74.1 M).
3. Signature verification is not reached on positives (placeholders). The compiled-wrapper decode (X1) is absent.

**Decode-cost estimate from the run's own tests.** `sequential_chain_invoice_r1` is two `settle_parts` decodes plus two `serialise_data` calls: 50,961,208 mem and 13,471,438,461 cpu, or about 25.5 M / 6.74 B per decode. `sequential_chain_reserve_r1` is one decode: 28,272,972 / 7,492,758,878.

| Path (host CEK) | mem | cpu | vs per-tx 17.5 M / 10 B |
|---|---|---|---|
| p07_1 accept (max) | 208,714,819 | 56,964,090,172 | 11.9× / 5.7×; 2.7× the per-block mem limit of 77.5 M |
| p01 facts (one settle pass + decode) | 74,140,029 | 20,631,803,584 | 4.2× / 2.1× |
| p01 facts minus ~25.5 M decode (approx.) | ≈48.7 M | ≈13.9 B | ≈2.8× / ≈1.4× |
| p04 facts minus ~28.3 M decode (approx.) | ≈65.2 M | ≈18.6 B | ≈3.7× / ≈1.9× |
| p07_0 full, one thread, one pass | 94,250,082 | 25,746,942,873 | two threads at about 2 × (94 − ~28–30) M ≈ 120–135 M, above the per-block 77.5 M |
| genesis single pass (h46_parts) | 9,756,452 | 2,641,286,080 | 0.56× / 0.26× (includes decode) |
| close single pass (h51_parts) | 13,059,752 | 3,417,083,625 | 0.75× / 0.34× |
| revoke single pass (h49_parts) | 17,500,956 | 4,586,796,938 | 1.00005× mem (includes decode) |
| release single pass (h53_parts) | 18,208,659 | 4,736,903,852 | 1.04× mem (includes decode) |
| refund single pass (h55_parts) | 3,628,211 | 984,888,776 | 0.21× / 0.10× |

**My cost conclusions:**

- Every settle-path test exceeds per-transaction memory, including harness decode. There are 80 of them: all 110 settle-module tests except the 30 genesis, revoke, close, release and refund tests. The lowest is h41 at 26.26 M.
- Subtracting the estimated decode cost still leaves a single-thread settle at ≈2.8–3.7× per-transaction memory and ≈1.4–1.9× steps. The settle transaction also carries the C2 release (P12), whose single pass is ≈18 M including decode. I see **no evidence that settle fits, and a strong host signal that it does not.** A cost-model difference between aiken's default and Preprod's PlutusV3 model is unlikely to close a gap of more than 2.5×. That is my judgment, not a measurement.
- Multi-thread atomic settle, as written, runs S2's full logic once per thread input. The two-thread case probably exceeds the per-block memory limit. Cost therefore bears on the multi-parent design, not only on tuning.
- Genesis, close and refund look feasible. Revoke and release are at the margin, before the decode is removed.
- The C2 Groth16 `issue` cost is **unmeasured**: there is no real VK or proof, only `groth16_refuses_short_vk`.
- Ledger fit requires all of these: the compiled scripts (X1) evaluated against ledger-shaped ScriptContext Data; the Preprod PlutusV3 cost model taken from a pinned parameter source; a pinned parameter-name mapping for the 297-entry PlutusV3 array (not in the packet); and per-redeemer and per-transaction totals checked against `maxTxExUnits`. The formal `scriptIntegrityHash` binds a transaction to the cost-model language view in force (`Utxow.lagda.md` L261–266). A measurement under a different model is therefore not ledger evidence.
- Packet coverage note: the per-transaction ExUnits bound appears in the implementation (`Alonzo.validateExUnitsTooBigUTxO`, called from `babbageUtxoValidation` L423–424). The formal UTXO and UTXOW rules I read do not state it. I make no claim that it is absent elsewhere in the formal spec (AR-16).

**Phase-2 consequence.** If a script is submitted marked valid but exceeds its budget at run time, it fails phase 2. The ledger then consumes the collateral inputs (`Babbage/Rules/Utxo.hs` L489–502; formal `Scripts-No` L503); observed `collateralPercentage` is 150 and `maxCollateralInputs` 3. Budget overruns must be found by off-chain evaluation before submission. The product contract requires every failure outcome to keep its residual duties and account for its fees and collateral.

## 5. Security and effect assessment (Opus)

- **Effects stayed in scope, within the observable limits.** The run created and wrote only `T`. All source packets, tools, pins and the DATA-ONLY binary were unchanged. There is no fetch indication, the lock has no etag, and the netns showed only `lo`. Nothing touched keys, wallets, credentials, SDKs, proofs, the ledger or funds.
- **Accepted residuals still apply.** There was no mount namespace, pathname UNIX sockets were reachable, there was no FSIZE cap and no syscall trace, and P1 and F2 ran unwrapped. The post-checks cannot show the absence of writes outside the checked paths. I found no contrary evidence. These residuals were accepted only for this run; they do not waive the R1 Rust mount-view requirement.
- **Signature path.** No positive was observed. h63 shows that aiken's Rust Schnorr builtin returned False on a zero signature. The Haskell ledger builtin's behaviour on that input is unverified. Either an error or False refuses, so this affects only the isolation claim (AR-09).
- **Gate-name strings in production bytecode.** The C2 compiled hex contains `own_token`, `single_token`, `burned`, `claimant_signs` and `spent_or_expired`. The named-component diagnostics are compiled into the deployable script. This costs size and execution, and S2 is likely affected the same way (I did not read the S2 hex) (AR-07).
- **Encoding lineage.** The fixtures and predictions share one author lineage. Library agreement does not show Rust/Aiken canonical-encoding agreement or Core correspondence. OEX7 (`serialise_data` against ledger bytes) is unobserved.

## 6. Scoped adoption (see `ADOPTION-RECOMMENDATION.json`)

**Adopt, as scoped maintainer-local evidence:**

1. GREEN(A1): exact local CEK library agreement for the 16-file P3S1-R1 package (`a54de569…a957`) under aiken v1.1.24+bacbeb3 and stdlib v2.2.0 at trace silent/all. 146/146 pass, the frozen comparison is exact, and this holds through `test_ctx` only.
2. The A2 identity records, as identities only (§3).
3. Run integrity: 25/25 exits 0, 0 retries, pins unchanged, hygiene, and host resources.

**Do not adopt:** ledger ExUnits or budget fit; deployability; reference-script feasibility; minUTxO, fees, collateral or funding; compiled-wrapper behaviour (X1); OEX7; signature positives (FX1); Groth16 setup, key or proof; Rust R1 ABI2 or BUILD2; RR-03; any settlement or Preprod result. Funding approval is none.

## 7. Concrete remaining Preprod blockers

| ID | Blocker | Kind |
|---|---|---|
| PB-01 | S2 is 30,764 B against `maxTxSize` 16,384; it is undeployable, inline or by reference | Design and source (hard) |
| PB-02 | Settle cost: host signal of 2.8–3.7× per-transaction memory after decode removal; P07 likely above the block limit; C2 issue unmeasured | Design, then X1 measurement |
| PB-03 | X1 compiled ScriptContext wrapper evaluation unrun | Measurement |
| PB-04 | A pinned, authenticated parameter and cost-model source, and the deployed node/ledger identity, are both missing; the PlutusV3 parameter-name mapping is missing | Evidence |
| PB-05 | OEX7: `serialise_data` against ledger datum bytes unobserved | Measurement |
| PB-06 | FX1: no valid-signer positives with test keys | Source and measurement |
| PB-07 | No genuine current-language Groth16 setup, VK or proof (K2); no legacy Source5 relabel | Proof |
| PB-08 | Rust R1 ABI2 native/BUILD2 unrun; canonical-encoding agreement Rust↔Aiken unshown; mount view required | Native |
| PB-09 | Independent recomputation of the C2 and S2 hashes | Evidence |
| PB-10 | Deployment economics: minUTxO for reference outputs, ref-script fees, collateral, stake-credential deposit if withdraw-zero is used; no funding authority | Authority and evidence |
| PB-11 | RR-03 general constraint source/runtime adoption | Product |
| PB-12 | Explicit phase-2 failure, collateral and residual-duty policy for every settle failure outcome | Product and security |

## 8. Preserved as open

Actual Rust source → a new Groth16 proof (no legacy Source5 relabel); versioned catalogue, proof bundle and claim-issuance composition; physical settlement end to end. Unsigned or structural catalogue acceptance is a separate axis, distinct from grants. Also open: the original bridge 42/52 gap; all 20 pattern families; ACTUS (277/18/32/DS01–07); DeFi (72/DA01–24/held-outs/intents/regressions) and general financial obligations; recursion and PCD; private handoff; multi-parent composition; Preprod; and every U0–U7, M1–M17, MC01–MC08, SP01–SP12 and G01–G24 exit. This review is a maintainer record, not admission of any public program. The public language stays permissionless.

## 9. Method and limits

I used Read, Write and Edit only. I ran no Bash, Grep, Glob, listing, hashing, interpreter, compiler, Aiken, SDK, crypto, network, Git, key, proof, wallet, ledger action or subagent. I cite hashes as root-provided or as recorded in receipts, and I recomputed none. I did not open any provider-result, process.stderr, credential, PROMPT, PROCESS, FINAL or wrapper file, or any live draft. All counts and ratios are hand arithmetic over the read text. Read coverage, full versus structural versus hash-only, is in `READ-COVERAGE.json`.
