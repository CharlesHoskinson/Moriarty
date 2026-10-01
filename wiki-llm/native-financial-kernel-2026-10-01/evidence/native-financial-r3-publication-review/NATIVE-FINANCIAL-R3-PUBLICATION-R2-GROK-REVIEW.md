# Independent Grok 4.6 high publication audit — native financial R3 r2

2026-10-01. Full-candidate publication audit of the exact 63-file draft. This report is source/result faithfulness, design alignment, and scoped Git publication / routine canonical intake only.

## Requested and returned identity

| Field | Value |
| --- | --- |
| Requested model | `grok-4.6` |
| Requested effort | high |
| Returned identity | Grok 4.6 |
| Host system line | Grok 4.6 released by xAI |
| This document | report body at `/home/charl/research/moriarty-signed-intent-2026-10-01/NATIVE-FINANCIAL-R3-PUBLICATION-R2-GROK-REVIEW.md` |
| Provider receipt | root attaches the actual session receipt after this turn; this body does not invent a `modelUsage` key |

No current publication Astra counterpart was read. No subagents. No Git, network, wallet, Cargo, native, wrapper import, ELF copy, vault write, or source edit. The only file written is this report.

## Startup

Checkout `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. Loaded `AGENTS.md` and `plugins/moriarty-dev/skills/develop/SKILL.md`. Guarded CLI:

```text
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
```

Capability `SP01.6 loan-swap-subset`. `pendingTransactions` empty. `blockedAction` implementation/repair of loan-swap-subset. Missing evidence: stale SP01 binding/candidate inputs, missing `.moriarty-dev/runtime/current-accounting.json`, unavailable `sp01-loan-swap-grok-01` live resource, unresolved operational history. This audit does not dispatch that campaign.

Git inspect was skipped under the READONLY no-Git constraint. Freeze `base_head` `4eae809cb042e6ebb96e062bc5b4717bd3b82ce1` is recorded as operator metadata.

## Verdicts

**Source faithfulness of the exact 63-file r2 publication: APPROVE.** Every freeze path hashes. The original 45 failed-result identities plus the closed ELF object match. Collection, CSV, and portable-ledger legacy mapping for SRC-0118 share content SHA-256 `b2065cb2ea580f9d99a27428ab1c0bcf947dd0e07ac46ddc988785a839a66241`. Latest ROADMAP / kernel README / R3-04 / session / index / log / hot paragraphs describe a consumed NightBalance Failure with producing-process default Real WF. Historical snapshots stay in place. No accepted financial Success, payout, fee, replay-refusal, or independent verifier is asserted as completed.

**Result faithfulness of the published failure record: APPROVE as a consumed native apply Failure.** Application-result is exact `Failure(InvariantViolation(NightBalance(24001000000000000)))`. Genesis and poststate are byte-identical 5227-byte objects `63f008f253b133570827334446c3a173f309e68fc699e1ff63954d48498a146f`. Proof 6336 `21321b44…`, statement 2275 `50712a16…`, signed sealed transaction 10499 `8195468…`. Child exit 1, `stop_reason` null, `native_postconditions_passed` false, wall 203.823581967 s, sampled group CPU 269.11 s, sampled peak RSS 2910072832 bytes, inside the admitted 600/1200/4 GiB envelope. Both full actual-result reviews remain in the archive: Astra `c9c80346…`, Grok `511b26dd…`, external Grok receipt `516580a1…` (`grok-4.6-build`, `end_turn`, process exit 0).

**Design alignment and honest scope: APPROVE.** U0–U7 exits, including U2 and U7, are unchanged. Site beta authoring paragraphs are unchanged. The diagnostic is pending source work. The funding route is source-only. Distinct distribution/claim nonces supersede the funding note’s same-nonce step. Broader authentication, compiler, property/intent/transition/history, PCD, and Preview gates remain Open. This docs change implies no test rerun and no new native, campaign, resource, or chain authority.

**Scoped reviewed Git publication of the 63-file draft: APPROVE.** Wiki-llm archive, ROADMAP latest observation, SRC-0118 collection, and CSV append are documentation of a consumed public-development fixture. Absolute historical paths stay metadata.

**Routine canonical intake of the coupled wiki transaction: APPROVE the draft bytes as schema-aligned with current vault `expected_hashes`.** Writes use `content_file` and match freeze SHAs. Inspect approval `c20412f58834e784f30ee12cdba78247b6fc9590e316490b4094e769941e880d` was not found on disk in this audit (Open process evidence below). Original external-draft/schema refusal remains preserved. No vault write is authorized by this vote.

Financial acceptance, independent verification, diagnostic execution, funding demonstration, retry, Compact, key regeneration, rebound proof, wallet, and chain action remain REFUSED.

## Freeze identity

Prompt token `SHA3119fa7e…` is SHA-256 `3119fa7ef514eecca73343871006278f9efd46d556f3541bf69075b80aafa9db` of `NATIVE-FINANCIAL-R3-PUBLICATION-R2-FULL-DRAFT-FREEZE.json` (5898-byte sibling failed-result freeze is a different object). Independently rehashed:

| Object | SHA-256 | Result |
| --- | --- | --- |
| r2 full-draft freeze | `3119fa7ef514eecca73343871006278f9efd46d556f3541bf69075b80aafa9db` | match; 63/63 paths; extra 0; missing 0 |
| wiki r2 bundle `NATIVE-FINANCIAL-R3-WIKI-R2-BUNDLE.json` | `8460e2f837ab67dcdb22eed5e661342422f92e8a998fdf775cae3020eb760b6e` | match; 2302 bytes |
| original failed-result freeze | `6ed8f63baad2d9c0933e41bf081894b158cbadc95392ebe4f82fbd2d047f91e5` | match; 45/45 |
| draft copies of those 45 | same 45 SHAs | match |
| live ELF `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer` | `7430db3b105a0da85bc64f92885f3cc3e9968371df79f49d28094939d9a96fb6` | 696902872 bytes; ELF is external and absent from the 63-file public set |

r1 and r2 drafts are identical except `wiki/meta/ledgers/source-ledger.json` (`9302f692…` → `53ecb297…`): UTC `generated_at` `2026-10-01T15:13:53Z` and portable source id `src-14f3f49cfbcdaa99561d` in place of locator-hash `src-e59d6e047cab3a4164a1`.

## Same-process default WF and real proof versus independent verification

Executed caller `src/main.rs` `2a25e005931c4efb940aae25260ebf23f0d70846a7cf9a43172d9a4641a00d80` / 30741 bytes. `prove()` constructs, proves, requires one provider record and PI equality, signs, seals, rechecks PIs, then `accept()`. `accept()` order:

1. `tx.well_formed(state, WellFormedStrictness::default(), tblock)?`
2. `state.apply(&verified, &TransactionContext { ref_state: state.clone(), … })`
3. write `application-result.txt` and `poststate.tagged`
4. return error unless `TransactionResult::Success`
5. only then write `receipt.json` / replay-refusal and print Success

Official `WellFormedStrictness::default()` at cargo `9a8777c` `ledger/src/verify.rs:461` enables balancing, native proofs, contract proofs, signatures, limits, and `ProofVerificationMode::Real`. `well_formed` errors are `MalformedTransaction` and abort before those two writes. The retained Failure file therefore means default Real WF returned `Ok` in this child, and `apply` returned NightBalance.

`provider.rs` `1bf45f54…` dual-verifies the same VK against SRS-derived `ParamsVerifier` and `PARAMS_VERIFIER` before recording the proof. `proof.raw` exists after that producer path. Those are same-process implications from source order plus retained artifacts.

Independent `verify::run` (`src/verify.rs` `8e9c59d0…`) remains unexecuted. Wrapper verify requires parent `native_postconditions_passed` true, a successful proof-result freeze, and a distinct `phase: verify` authorization. Those objects are absent. Wrapper native postconditions (`run-native-financial-r3-proof-bounded.py` `eb0f65aa…` lines 265–280) require exit 0, `receipt.json` with `result` `Success`, distinct pre/post hashes, positive consumed fee, and replay-refusal. Receipt `native_postconditions_passed` is false. Identity of `proof.raw` / `transaction.tagged` is binary output identity. It is producing-process evidence.

Envelope-preflight.json still records `well_formed_checked` false / `ledger_applied` false / `ledger_accepted` false. That file is construction-stage eager offer checking. Default transaction WF is the later `accept()` call.

## Guaranteed-section rollback versus blanket phase atomicity

Pinned `semantics.rs:1343` `apply` on guaranteed error returns `(self.clone(), TransactionResult::Failure(e))`. `apply_section` checks `check_night_balance_invariant` at line 1272. Byte-identical genesis/poststate corroborates this guaranteed-section Failure. Product contract and historical session text already record that a failed fallible phase can retain guaranteed-phase effects. This result does not establish rollback for every fallible-section failure.

## NightBalance cause

Cargo checkout `/home/charl/.cargo/git/checkouts/midnight-ledger-b2f9c59d942dfdca/9a8777c` (commit `9a8777c4d035fc7f38ae286bcf5f8656668efd9f`):

| API | SHA-256 | Fact used |
| --- | --- | --- |
| `ledger/src/structure.rs` | `eebb60349d7c4b1e9954d4e07a1ed389b81612d55d946e2aa43e0525b8bbde9b` | `MAX_SUPPLY` line 3361 = `24_000_000_000 * 1_000_000` = `24000000000000000`; `LedgerState::new` line 3367 puts that amount in reserve |
| `ledger/src/semantics.rs` | `023f16f66436830246f89c58cf1e051f61b7955842aadd8d968656a727f3effd` | invariant line 529 sums UTXO NIGHT + locked + reserve + block rewards + treasury NIGHT + unclaimed + bridge + contract NIGHT; mismatch returns `NightBalance(total)` |
| `ledger/src/verify.rs` | `35b93fc008ae052e9b7948d1f87959b82e459321a2d40fbc09f099d8dcad25db` | default Real WF; no NightBalance check |
| `serialize/src/serializable.rs` | `87cec39617be153146efa49dfb163f7d28bde52358b83ae9995bbcd4875f5238` | `tagged_serialize(value, writer)` |
| `zkir/src/ir.rs` `e82d81d` | `61118e731ae492f61bbcae73633c7ba4ab627cbe7cd36b5ac455fc721028052c` | actual prove/check |

Caller `genesis()` (`main.rs:65–76`) builds `LedgerState::new`, inserts A1 escrow 10000 as Unshielded `[0xa1;32]`, then inserts one NIGHT UTXO of fixture `night_value` `1000000000000` without debiting reserve. Config `96684666…` confirms that value. A1 is excluded from NIGHT annotation. Constructed total `24000000000000000 + 1000000000000 = 24001000000000000`, equal to the observed Failure. Returning the same NIGHT principal in the guaranteed offer leaves the excess.

Diagnosis `NATIVE-FINANCIAL-R3-NIGHT-SUPPLY-DIAGNOSIS.md` `7181e4dbc…` matches this arithmetic. No new native invariant check on retained genesis was executed in this publication.

## Diagnostic pending; funding note superseded

The 63-file set contains the diagnosis and the source-only funding note. It does not contain `night-supply-diagnostic.rs`. R3-04 states the diagnostic is uncompiled/unexecuted. Removal of the excess UTXO in an isolated clone is a causal control. It does not fund a replacement genesis.

Funding note `NATIVE-NIGHT-FUNDING-ROUTE-SOURCE-NOTE.md` `9658a5ff…` retains original step 3 wording that reuses the distribution nonce on the claim. R3-04 explicitly supersedes that wording: distribution records its intent hash (`semantics.rs:1823–1829` `OutputInstructionUnshielded` + nonce → `mk_intent_hash`), so distribution and claim nonces must differ. The route itself is unimplemented in this candidate. Official `DistributeReserve` (line 981) and `DistributeNight` (line 583) are inspected public transitions. They are not demonstrated here.

`artifacts.rs` `4e7be84e…` calls `tagged_serialize(value, &mut bytes)` in the official order. Historical Astra failed-result review records that a proposed diagnostic reversed those arguments. That proposed file is outside this publication.

## Five Rust modules, wrapper, 17 outputs, parent receipts

| File | SHA-256 | Bytes |
| --- | --- | --- |
| `src/main.rs` | `2a25e005931c4efb940aae25260ebf23f0d70846a7cf9a43172d9a4641a00d80` | 30741 |
| `src/provider.rs` | `1bf45f54594e1741214635251dd98ed3cac13651c57d1df30c491f808aaf2c6f` | 4324 |
| `src/verify.rs` | `8e9c59d0d80efbeb28a5d2f84d899be1dc5d732833178d4025656e69c8fe3c8b` | 6433 |
| `src/artifacts.rs` | `4e7be84ede3fb11dc90a58fd166fb749bc53ec07247f86266cacb6bf6ee622d7` | 1771 |
| `src/keygen.rs` | `9f97af1657ad1e23b099e184ea8fb57d91a96976149d7163541f8218ee915f5b` | 4526 |

`SOURCE-HASHES.json` `0179484a…` matches those five plus Cargo/patch/README. `INPUT-SOURCE-HASHES.json` `9d34d657…` is a historical inspected-API map whose `native-ledger-consumer/src/main.rs` is older `2d9ddbc8…`. Live older consumer at B still hashes to `2d9ddbc8…`. Executed R3 caller is `2a25e005…`. Cargo.toml pins ledger `9a8777c4…` with `proof-verifying` and ZKIR `e82d81d2…`. `Cargo.lock` has no `mock-verify` feature string in this audit’s use of the frozen lock.

Archived candidate README still says no proof has been executed for that source packet. That is the original source-freeze wording. Latest kernel README, R3-04, session, index, log, and hot qualify the later Failure.

Seventeen proof-directory artifacts match receipt `1f49d47f…` and freeze. `receipt.json`, `replay-refusal.txt`, and `independent-ledger-receipt.json` are absent. Log `950188d1…` ends with `Error: "not full native Success; inspect retained guaranteed/fallible fee effects"`. Construction diagnostics: 293 ops / 47 reads, preimage equal, eight CallContext fields equal. Envelope: guaranteed 1/1, fallible 0/2. Preliminary fee `1128519108356650` remains an unproven estimate. `skips.json` equals `native-check-skips.json`.

Parent prove allocation: root decision `dd40ccaa…`, authorization `c7f5d8d7…` with `proof_result_freeze` null and `phase` prove, live admission `a8758ea9…`, attempt `861e5006…` state `reserved before child launch; never delete to retry`. Source/resource and preflight votes are historical admission of that consumed prove.

Public fixture Schnorr bytes `[0x42;32]` and funding intent `[0x51;32]` are deterministic development constants. Secret-pattern scan of the 63 text files found no wallet/private-key/mnemonic material. Large PK/VK/SRS/ELF bodies stay external and hash-bound.

## SRC-0118 collection, CSV, ledger

Collection `raw/sources/native-financial-r3-failure-2026-10-01/collection.json` SHA-256 `b2065cb2ea580f9d99a27428ab1c0bcf947dd0e07ac46ddc988785a839a66241`. CSV appends one row SRC-0118 with the same SHA; other 114 rows identical to current inventory. Lifecycle `S3`. Notes: consumed Failure; no new proof/native/network authority.

r2 ledger `53ecb297fc258e14218a7e8701e2bdd409ee7b67465d74b8290e3cd7fd90ab0e`: schema `claude-obsidian.source-ledger.v1`, `generated_at` `2026-10-01T15:13:53Z`, 6226 sources (current 6225). Portable record `src-14f3f49cfbcdaa99561d` has `content_sha256` `b2065cb2…`, independence_key `moriarty-native-financial-r3-single-experiment-20261001`, `legacy_hash_matches_observed.SRC-0118` true, `review_status` unreviewed. `legacy_source_ids["SRC-0118"]` = that portable id.

r1 portable id `src-e59d6e047cab3a4164a1` equals `sha256(locator)[:20]`. r2 id is not independently reconstructed from independence_key, locator, or content SHA alone. Existing SRC-0114–0116 portable ids also differ from locator/content prefixes, so r2 follows that house style.

Inherited pre-existing gap: `legacy_source_ids` lacks `SRC-0117` in current vault (116 map entries) and in both r1/r2 (117 map entries after adding only SRC-0118). SRC-0117 portable record `src-93d3a20e70edeb8634d5` exists in `sources`. This publication does not repair that older map hole.

## Canonical notes and ROADMAP

Current ROADMAP `896dc583…` versus draft `88999dae…`: only the 2026-10-01 delivery observation changes, from V8-03 unsorted-WF refusal to sorted R3 producing-process WF then NightBalance application refusal. Immediate repair becomes supply-consistent funding and early genesis invariant check. U2/U7 table text is identical. Site beta remains a delivered precursor. Authoring-beta paragraphs in index/hot stay.

Current kernel README `4b0cbae9…` has no R3-04 and no r3-failure archive. Draft README `4e49c1ed…` adds latest application-refusal status and keeps historical V5/V6/V7/V8 sections. New `R3-04-ACTUAL-APPLICATION-REFUSAL.md` `018e46e9…`. Session draft adds SRC-0118 and a latest refusal section after the retained V5/V7 snapshots. Index adds SRC-0118 in frontmatter and rewrites the native-kernel paragraph. Log prepends an SRC-0118 Failure line; log frontmatter still omits SRC-0118, matching the pre-existing SRC-0117 pattern. Hot adds a native-iteration paragraph; beta-language paragraph unchanged.

Wiki bundle expected_hashes match current vault bytes:

| Path | Current = expected |
| --- | --- |
| `wiki/sessions/native-financial-kernel-2026-10-01.md` | `6cceba599722b85fb08b5e9016eb55cde0902ca8f090d6e77b05faa4840d5ba6` |
| `wiki/meta/ledgers/source-ledger.json` | `1a14585c8d27834c5e03b31871904846bbd88ada91eaf28e2c350a667c92d19b` |
| `wiki/index.md` | `a76d312315749f3935d8472a99ca21868afb164f314cf37fd556fb0fa3667c36` |
| `wiki/log.md` | `78b1d45e907add3dc183e8a88b845126374c202213b0e964410eabdac9136ed6` |
| `wiki/hot.md` | `09813745de0b6d5e6ac06dcf4838c4c041a75ddfe69204c4e0356aa23042d66d` |

Writes match freeze SHAs. Schema `claude-obsidian.transaction.v1`, operation `ingest-native-financial-r3-failure-20261001-r2`. Absolute `content_file` paths are apply-time locators.

Original refused inspect: `vault-ingest-native-financial-inspect.json` empty `e3b0c442…`; `vault-ingest-native-financial-bundle-rejected.json` `3ee10ba9…`. Older inspect-v2 approval `811c6227…` is a different kernel ingest.

## Inspect approval Open

Prompt states portable-core inspect succeeded with approval `c20412f58834e784f30ee12cdba78247b6fc9590e316490b4094e769941e880d`. Search of B, checkout wiki, `.vault-meta`, `/tmp`, and related inspect JSON found that digest only in `NATIVE-FINANCIAL-R3-PUBLICATION-R2-GROK-PROMPT.md`. Independent on-disk inspect receipt for this r2 bundle is missing. Bundle bytes and current `expected_hashes` are inspectable; the approval object itself stays Open.

## Alignment with the product contract

`docs/MORIARTY-PRODUCT-CONTRACT.md` keeps Moriarty as a permissionless ZKIRv3/Midnight language. Maintainer review is internal. Objective financial/proof gates remain. Signed-intent full financial goal is unchanged. A trusted-genesis fixture Failure does not reduce U2 generality or U7 release. Preview published-source format incompatibility remains conditional on live implementation correspondence. No new network transaction or chain rejection is claimed. Network in the fixture is `undeployed`; the child ran under `unshare -Urn`.

## All blockers and Opens at once

Publication-content defects that would refuse the 63-file faithfulness vote: none found.

Process evidence that stays Open:

1. Inspect approval `c20412f5…` absent from disk in this audit. Path searched: B inspect/ingest JSON, checkout wiki, prompt-only hit.
2. Inherited `legacy_source_ids` hole for SRC-0117 (current vault and r2). SRC-0118 mapping itself agrees.
3. r2 portable id recipe for `src-14f3f49c…` not independently derived from independence_key/locator/content in this reading; house style matches SRC-0114–0116.
4. Git HEAD cleanliness unverified here because Git commands were forbidden.

Product and execution gates that this publication correctly leaves Open (missing actual evidence; no promotion):

5. Native full Success, escrow 8990, accepted A1 1000/10, conserved NIGHT principal, consumed DUST, replay-refusal after Success. Apply returned NightBalance; poststate equals genesis.
6. Independent `verify::run`, mutation/EOF/absent-signature suite, `independent-ledger-receipt.json`, proof-result freeze.
7. Completed native invariant diagnostic on retained genesis `63f008…`. Diagnosis is source reading. Diagnostic source is pending and outside the 63 files.
8. Implemented/demonstrated reserve→reward→signed-claim funding with distinct nonces, actual produced UTXO, timestamps, replay controls.
9. Authenticated source-owner, native payer, public genesis, A1 mint, deploy, funding, time, history.
10. General Moriarty compiler lowering; property, intent refinement, transition validity, history compliance; PCD / recursive composition.
11. Preview settlement and published-format compatibility against a live attested node. Conditional published-source mismatch remains; no chain rejection observed.
12. Ceremony audit of SRS `4a9ef6c7…`.
13. Sorted-but-wrong semantic/VK/funding/dust/time/stale-state negative matrix.
14. I2/MC02/MC05, SP05, mandatory proof-carrying product acceptance.
15. Guarded SP01 operational history, accounting, live resource.
16. Consumed prove reservation `861e5006…`. Deleting it to retry is forbidden. No automatic retry, old-reservation delete, fifth Compact, key regeneration, invariant bypass, private-field hack, or rebound of proof `21321b44…`.

Items 5–8 block any Success, verify, diagnostic-complete, or funded-repair claim. Items 9–16 stay Open after a later supply-consistent fixture, which would be a new source packet.

## Boundaries of these verdicts

Source faithfulness covers identity of the 63 freeze paths, original 45 plus ELF, five executed Rust modules, wrapper, 17 outputs, parent prove receipts, pinned cargo `9a8777c` / `e82d81d` APIs, SRC-0118 collection/CSV/ledger content mapping, and the latest-paragraph qualifications. Result faithfulness covers the published Failure as an accurate consumed prove: real proof and signed transaction bytes, producing-process dual VK verify and default Real WF implied by `accept()` order plus retained application outputs, and native apply NightBalance with unchanged genesis.

This review grants no Success, independent verification, diagnostic completion, funding demonstration, Preview, product completion, Git commit, vault apply, or future allocation.
)
