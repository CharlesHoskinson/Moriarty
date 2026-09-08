# Candidate03 financial comparator review

**Source: APPROVED within the fixed-fixture utility scope. Overall task: BLOCKED on evidence/report completion.**

Candidate `96c7db7d742ff795fe96ca6fd6de95c7f9aa9351da884fdcbe63f55af2b54510`,214299bytes. All9owned files and6protected inputs match before/after. Comparator hash `4a99290b40d89a28b363fa1c58ee43c6922ed73e86d5079e43ec27fdcb2c53fe`; tests hash `0eb366c7ccab6712bcc9b2afe85b077578a2d973ad2caed526fdcaa4d72e979c`.

The current comparator independently validates both records against closed financial identity sets and source-derived arithmetic. No new source correctness finding was identified. This is not receipt authentication, ledger settlement, finality, Preview, SP05, proof or language acceptance.

## Source evidence

- 52existing tests pass independently.
-  All4prior attacks return `ok:true` under hash-verified candidate02 and `ok:false` under candidate03, including equal bad inputs.
- 12,690additional/equal/either-side negative comparisons reject without exceptions, input mutation, nondeterminism or network claims;42malformed root comparisons also pass.
- 2corrected indexed-object permutation controls pass. An initial reviewer probe also reversed ordered role-name lists and correctly received BINDING_MISMATCH; that failed assumption is preserved in the JSON report.
- 123independent financial replay checks cover all6fixture stages, balance conservation, actor effects, continuity, supply and work. Loan interest33972602, cash533972602 and residual4500000000 are retained. Swap fee30, output19743 and provider close reserves1010000A/1980257B are exact.

## Four prior attacks

- `rogue-settledDue-999`: candidate02 accepts; candidate03 rejects. `DUE_MISMATCH` at `expected.stages[settle].liabilities.dues[rogue-due]`; `BINDING_MISMATCH` at `expected.stages[settle].liabilities.dues[rogue-due].creditor`.
- `residualDebt-999`: candidate02 accepts; candidate03 rejects. `RESIDUAL_DUTY_MISMATCH` at `expected.stages[settle].residualDuties[rogue-duty]`.
- `attacker-balances-999-zeroeffects`: candidate02 accepts; candidate03 rejects. `BINDING_MISMATCH` at `expected.stages[close].actorEffects[attacker|ASSET_A|AssetA_quantum]`; `BINDING_MISMATCH` at `expected.stages[close].actorEffects[attacker|ASSET_A|AssetA_quantum].actor`.
- `extra-zero-closeTransfer`: candidate02 accepts; candidate03 rejects. `EXTRA_TRANSFER` at `expected.stages[close].transfers[zero-extra]`.

## Closed financial domain

- **Independent validation of both inputs:** Each side validates structurally and independently replays the admitted source case before equality can return ok. Fixed semantic constants are derived with BigInt arithmetic, not imported from the caller expected record or the fixture JSON. Paths: `src/differential.mjs:1810`, `src/differential.mjs:1221`, `src/differential.mjs:1414`.
- **Closed schemas and identity domains:** Required/unknown fields, indexed duplicate identities, role-map exact names/values, exact assets, actor×asset balance/effect coverage and row references compose to an exact admitted domain. Setup-mint is excluded as a balance/effect actor and allowed only for specified setup transfers. Paths: `src/differential.mjs:214`, `src/differential.mjs:291`, `src/differential.mjs:740`, `src/differential.mjs:754`, `src/differential.mjs:822`, `src/differential.mjs:870`, `src/differential.mjs:883`.
- **Transfer/due/duty closure:** Required exact transfer key/id/ordinal/unit/amount tuples plus duplicate/extra checks close setup/accrue/settle/swap/close transfer collections. Required exact dues and residual duties plus extra-ID rejection close obligations. Amount0 does not create an exception. Swap dues are explicitly empty. Paths: `src/differential.mjs:1043`, `src/differential.mjs:1086`, `src/differential.mjs:1098`, `src/differential.mjs:1112`.
- **Closed finite inventory:** Loan has3stages setup/accrue/settle;2balance/effect actors×1asset per side;transfer counts1/0/1;due counts0/2/2;one remaining-notional duty each stage. Swap has3stages setup/swap/close;3actors×2assets per side;transfer counts3/2/2;no dues;one reserve-for-close duty each stage. Role maps also contain the sole setup-mint exception.
- **Financial fields and lifecycle:** Both sides independently constrain all case financial scalars and source pins; full balances/effects replay against transfers, stage continuity and fixed supplies. Refund cannot erase gross debit; economic swap fee30 is already included in gross10000 and is not subtracted twice. Loan residual4500000000 persists after dues settle and work reaches0. Paths: `src/differential.mjs:66`, `src/differential.mjs:122`, `src/differential.mjs:776`, `src/differential.mjs:798`, `src/differential.mjs:1126`, `src/differential.mjs:1196`, `src/differential.mjs:1387`.
- **Ordering and scope:** Indexed object arrays compare by identity, with transfer ordinals retained. Stage role-name lists are compared in order; they are not indexed object arrays. Ordinary JSON scalar/container coverage passed. No general JavaScript proxy/getter/cycle or unbounded-resource claim is made.
- **Network claims:** Both valid cases and all tested invalid inputs retain networkAcceptance=false and incompleteNetworkEvidence. Symbolic roles/assets and no production receipt decoder are accurate scope limits, not standalone code blockers.

## Evidence blockers

### E3-1 — Historical45-test reports do not bind the current comparator and tests

`experiments/moriarty-midnight-financial/test-evidence.json#/ownedFileHashes`; `experiments/moriarty-midnight-financial/test-evidence.json#/green`; `FOREMAN_REPORT.json`; `FOREMAN_REPORT.md`.

- All three files are byte-identical to candidate02 and to the interrupted identity-correction snapshot. Their retained GREEN is45tests,6893stdout bytes/hash7ecadc5351e5137c803cebc80420a1a5095b4bb9047249fe9cf73e738a848881.
- The historical comparator hash isaa6351df2a2644fcf463a2897261c36fb386d9ebcb718efae0871716da19c0a4; current hash is4a99290b40d89a28b363fa1c58ee43c6922ed73e86d5079e43ec27fdcb2c53fe. Historical test hash ise851aeb58126d22d114b3e27206ad381bc7c4a9cf3fdb5782c5b53585a72619d; current hash is0eb366c7ccab6712bcc9b2afe85b077578a2d973ad2caed526fdcaa4d72e979c.
- The historical hashes are legitimate historical identifiers, not faulty hashes to replace. They cannot serve as evidence of current source validation.

Append accurately attributed current verification and regenerate current reports/manifests while preserving historical evidence unchanged.

### E3-2 — README completion claim is stronger than the retained author receipts

`experiments/moriarty-midnight-financial/README.md:155`; `experiments/moriarty-midnight-financial/README.md:157`; `identity-correction-worker-receipt.json`; `identity-completion-worker-receipt.json`.

- Both identity-correction and reporting-completion dispatches exited124 and have resultParseError. Actual serving model is unavailable; only the requested grok-4.6 high configuration is known.
- README:157 describes a completion GREEN/exit0 as this author run. A52pass standalone capture does exist, but the dispatch did not complete and current reports never bound its source/capture/exit/provenance. The review does not infer a successful author dispatch.
- Root verify5 is a separate completed exit0 run with a source-bound freeze and52passing tests. The independent reviewer also ran52tests successfully. These support code correctness without retroactively completing either author dispatch.

Use the known root/reviewer evidence with exact attribution, preserve partial capture and both124 receipts, mark actual serving identity unknown, and remove any implication of completed author reporting.

## Minimum closure

1. Keep the comparator, tests and protected fixture/source bytes unchanged unless a new defect is found. This review found no source correction necessary.
2. Create a current evidence section or file record bound to candidate03 comparator4a99290b… and tests0eb366c7…. Reference the completed root verify5 command/exit0 and exact52-test stdout8311bytes/SHA2567d02415e391447cc9ec27e1439c81d2f9f76ed28ba378a7dc66c51a578728e6a, with empty stderr. Label this root verification, not an author run. A newly captured report-closure test may be added with its own actual argv/cwd/exit/source hashes.
3. Preserve candidate01 mismatched historical captures, candidate02 RED/GREEN45test records, both timeout124 receipts, and the partial completion capture as separate history. Do not change old hashes or imply old tests exercised candidate03.
4. Record requested author configuration grok-4.6 high separately from actual serving model, which is unavailable for both timed-out passes. State that no successful complete author dispatch was observed. Do not infer serving identity or success from filenames, requested flags, README prose,52pass text, or this GPT-6 review.
5. Correct README:157 to distinguish the partial completion capture from a completed, receipt-bound author run. Its standalone8306-byte capture may be retained as observed text with SHA256163d6ab2347523f0d65b0adb61469126054e62f7288be1b3bcb4205a88d324e4; lack of a completed author receipt remains explicit.
6. Update both FOREMAN reports and current owned-file/evidence manifests with current source/test/README hashes,52test scope, this independent source verdict, historical failure disposition and unresolved ledger limits. Avoid circular report self-hashes: freeze the complete final owned-file set externally after report writes.
7. Freeze that artifact-only closure as a new exact candidate and verify its source/test/fixture pins against this reviewed candidate. A narrow evidence/report audit can then decide utility-task closure. It cannot restore missing historical serving metadata or grant ledger/Preview/SP05 acceptance.

No code rewrite is requested. Preserve the functional source/test/fixture hashes and limit the next phase to accurate evidence/report closure unless a new defect is found.

## Commands and limits

- Exit0: Read binding, identity correction/recovery prompts, prior review2 and candidate03 freeze. 
- Exit0: Cold source/schema/math/README/package/test inspection with rg/sed; Python initial9owned+6input digest check. 
- Exit0: /usr/local/bin/node --test tests/differential.test.mjs. 52pass0fail
- Exit1: Read-only Node four old/new attacks and12690negative comparisons, then initial permutation control. All negative comparisons pass;2ordered-role-list permutation assumptions fail. Session50005 completed on poll; full command retained.
- Exit0: Safe structured timeout/verify5 receipt inspection, report/partial-source comparisons and captured test-stream hashing. 
- Exit0: Corrected indexed-object permutation controls and42malformed root comparisons. 44checks pass
- Exit0: Python independent123financial replay/conservation checks; final9owned+6input hashes; allcandidate02hash verification. 
- Exit0: Inspect README:157, verify5 command provenance and historical evidence source/test hashes. 
- Exit0: Write only state/gpt6-review3.json and state/gpt6-review3.md. 

- Bounded source/content review; no universal formal proof of the comparator.
- No source or test fixes, dependency installs, build, proof, network, wallet, ledger, git mutation, or private/raw worker reasoning reads.
- Only review files were written. All relevant financial records in probes are synthetic clones held in memory.
- Root and reviewer evidence cannot supply missing historical serving-model identity or turn either124dispatch into a completed author run.

