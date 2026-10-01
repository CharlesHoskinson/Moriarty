# Grok 4.6 high — R2 alignment and actual failed v8-03 publication review

2026-10-01. Independent read-only review of the exact R2 publication freeze. This report is the assigned Grok 4.6 high audit body. No counterpart report, future disposition, sorted successor, resource grant, retry, import, candidate execution, network, wallet, proof, or additional auditor body was used.

## Reviewer identity

- Requested: exact model `grok4.6` at high effort.
- Returned / actual: Grok 4.6 (xAI), this session. No substitution.
- Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`
- Startup: repository `AGENTS.md`, installed `moriarty-dev:develop` skill, guarded `status --json` only.
- Guarded status: `capability` SP01.6 loan-swap-subset; `pendingTransactions` empty; `blockedAction` implementation/repair of loan-swap-subset on unresolved operational history. That maintainer status does not vote on this publication.

## Distinct verdicts

**Verdict A — narrow actual-failed-result publication: approve.**

The frozen v8-03 archive is a complete, hash-faithful record of one consumed native prove attempt that retained a 6,336-byte `proof.raw` and a signed/sealed `transaction.tagged`, then stopped at strict native well-formedness `OutputsNotSorted` before apply. Claims in the mapped refusal note, archive scope strings, and parent receipt match that executed path. Full ledger Success, independent `verify`, apply, poststate, and financial acceptance are absent and are not claimed.

**Verdict B — root alignment/status publication on the exact R2 map: approve.**

The R2 kernel README places the current V7/V8 actual summary first, labels the older evidence/design/plan sections historical and pre-execution, and points at the refusal and alignment notes. Root `README.md`, `ROADMAP.md`, and `wiki-llm/ALIGNMENT-REVIEW-2026-10-01.md` keep Beta as a delivered precursor, keep U0–U7 with original exits, treat the fixed trusted-genesis proof as insufficient for U2 generality, and keep mandatory properties/intent/transition/history and residual duties. Stale present-tense ambiguity in the kernel note is disposed for publication by those headings plus the explicit sentence that missing-keys/proofs language describes the earlier stage.

**Publication blockers requiring a change before these two votes: none.**

Resource approval, sorted-successor source, retry, Compact compile, independent verification, apply, Preview, and a second financial compiler remain outside this review and are not granted.

## Method and limits

Read-only file, SHA256, and AST/source inspection. Guarded `status --json` was the only executed product CLI.

This review did **not**: run the candidate ELF; call `prove`/`verify`/`apply`/`well_formed`; decode tagged artifacts with the native crates; run Cargo/Node; touch network, wallet, or SRS; spawn agents; read counterpart Astra/Grok/combined reports; read future disposition; inspect `native-ledger-sorted-candidate`; re-run PR15 blob hashes, SRC-0117 lint, or CLI tests cited inside the alignment note.

Cryptographic re-verification of `proof.raw` is therefore unperformed. Provider two-verifier success is inferred from executed source order plus retained artifacts and parent identity flags, as specified below.

Website `site/` and `wiki-llm/website-2026-10-01/` working-tree files are outside the freeze map. The freeze scope already excludes them. Parallel website work is permitted by the alignment note; it is unreviewed here.

## Freeze and hash closure

R2 freeze file:

- Path: `/home/charl/research/moriarty-signed-intent-2026-10-01/ALIGNMENT-V8-FAILED-PUBLICATION-R2-FREEZE.json`
- SHA256: `91c7b58ee7a5a1abf16810049308e022ae7e981c12215710141f93403f0f5abb` (matches the requested pin)
- Declared base: `df8b155143e90450168af04fbfd20f7620043620`
- Predecessor freeze SHA256: `f0a42b4f0f773aa39298a86be513324454ab535a28d3b58baa9f3e69842d8648`
- External failed-result freeze SHA256: `ef283f6d93f0123f48acc8295621bc6198312f8ec875f7fa43fd4da14be69079`

Worktree HEAD, `origin/main`, and the requested base are the same commit: `df8b155143e90450168af04fbfd20f7620043620` (PR15 merge). Branch name: `feat/native-keygen-result-20261001`. Mapped publication files are uncommitted relative to that commit.

All 25 R2-mapped paths hash-match the freeze. All 19 external failed-result freeze paths hash-match disk. Every basename shared between the repo archive and the external originals is byte-identical.

Predecessor comparison: **one** mapped file changed.

| Path | Predecessor SHA256 | R2 SHA256 |
|---|---|---|
| `wiki-llm/native-financial-kernel-2026-10-01/README.md` | `304737ce82e0c3465d733e7ad06d0c6216670932a38b5f74eb40841999c1a00e` | `4b0cbae9aee2b3db8735e730b07fb8daa50760b441cb6d8e5d0712cd4aef4018` |

The other 24 mapped files are unchanged, including `V8-03-ACTUAL-REFUSAL.md`, both failed-result freeze copies, `archive-manifest.json`, the four prove records, and all fourteen proof artifacts.

V8-03 input freeze `4254e9547d3211c4fd695ebf3627a91add2a565925221e325ece2e35ec723a0b` contains **667** `sha256` entries. All 667 exist on disk and match. Wrapper `run-native-financial-proof-v8-03-bounded.py` is `f5af2a11614365a702eb805c42c5252b59c9fe7b77e2398ed129e63c7e572687`, matching the attempt record. Source manifest `native-ledger-keygen-candidate/SOURCE-HASHES.json` is `73a68f8577c271a236dd737d213742f2a7dcc8d3e5c5e1cab0688ef839adf8df`. Current ELF `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer` is `9e1b153a17cd55969dd800fd47bc728923963093267cc8d23765f0a6ea484559`, 696,706,768 bytes, matching the input freeze `binary` object and the prove receipt `produced_binary_sha256`.

All nine files listed in `SOURCE-HASHES.json` still match that manifest.

## Actual failed-result path (Verdict A evidence)

Parent receipt (`native_postconditions_passed: false`, `exit_code: 1`, `stop_reason: null`, `supervisor_error: null`, `child_launched: true`):

- Command: `unshare -Urn -- env RAYON_NUM_THREADS=2 <ELF> prove <config> <config sha> <proof dir>`
- Elapsed 226.68 s; peak sampled group RSS 2,888,863,744; sampled group CPU 286.94 s; limits 600 s / 1200 CPU s / 4 GiB. Historical measurements of this attempt.
- `source_identity_preserved: true`, `binary_identity_preserved: true` after the child. The wrapper re-runs its full identity `verify()` (667-input freeze, source manifest, v7 freeze, inherited consumer/runtime/v5 closures, ELF pin) and then re-hashes the ELF before recording those flags.
- Qualification on the receipt: conditional trusted genesis, fixed kernel, exact production Beta/Core handoff.

Child stdout/stderr is one line:

```
Error: OutputsNotSorted([UtxoOutput { value: 1000, owner: UserAddress(0202…02), type_: UnshieldedTokenType(a1a1…a1) }, UtxoOutput { value: 10, owner: UserAddress(0303…03), type_: UnshieldedTokenType(a1a1…a1) }])
```

That vector is recipient02=1000 before fee03=10, same asset. It is the fallible A1 offer. Partition diagnostics record `guaranteed=None` and a fallible transcript with `unshielded_outputs` A1=1010 split as claimed spends 1000 (0202) and 10 (0303).

Retained after that error:

| Artifact | Bytes | SHA256 |
|---|---:|---|
| `proof.raw` | 6336 | `6d5b5d2c464a5a96e3a224e14f6c454b58798d4776c5a3a8bc8d03799ee2b6d2` |
| `transaction.tagged` | 10499 | `7c4de0ca2def7e8b9abae4831f7d026a97e5e975905bd6f9b1d92895e49ef416` |
| `statement.tagged` | 2275 | `50712a1652cb42de2f562734d61a2e952fd3c214790cfee81f5b77561dfe3ac6` |
| `constructed.preimage` / `frozen.preimage` | 4316 each, equal | `96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402` |

`skips.json` equals `native-check-skips.json` (293 `null` entries). `native-check-result.txt` is `Ok` of that vector. `construction-diagnostics.json` reports `preimage_equal: true`, all listed preimage fields equal, `actual_context_matches: true`. `preparation.json` is the registered-VK *preflight* object (`proof_invocations: 0`); it is written during `construct` before proving.

Absent from the archive, as required for this failure:

- `receipt.json`
- `application-result.txt`
- `poststate.tagged`
- `independent-ledger-receipt.json`
- any `fullSuccess` / replay-acceptance artifact

The archive directory contains exactly the 19 freeze-listed files plus `archive-manifest.json` (the manifest hashes the 19; R2 hashes the manifest itself).

This is a failed native postcondition with retained partial proof/transaction bytes. It is not ledger financial acceptance.

## Native consumers and APIs actually called

Executed binary: `beta-native-ledger-consumer` mode `prove`. Source tree used as cwd: immutable `native-ledger-keygen-candidate` (`SOURCE-HASHES` `73a68f…`). `provider.rs` / `verify.rs` / `artifacts.rs` match the older `native-ledger-consumer` copies; `main.rs` differs by the added `keygen` CLI (`393efec2…` vs consumer `2d9ddbc8…`).

Actual call sequence in `prove` / `construct` / `Provider::prove` / `accept`:

1. Config pin check; load IR (`IrSource::load`), PK/VK/SRS by pinned SHA; `k==17`.
2. `genesis` from fixture `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY`, Schnorr development key `[0x42;32]`, undeployed network.
3. Official runtime replay (`QueryContext::query`, 293 ops / 47 reads) against exported poststate.
4. `ledger::construct` `add_calls` / `PrePartitionContractCall`.
5. Envelope: `offer()` copies input/output vectors with empty signatures; `financial_outputs()` returns `[UtxoOutput{1000, recipient02, A1}, UtxoOutput{10, fee03, A1}]` with no sort.
6. `Zkir::check` during construct; skips retained.
7. `Transaction::prove` → `Provider::prove`:
   - refuses missing or zero `overwrite_binding_input` (finalized ledger binding required);
   - `ir.prove`;
   - `vk.verify` with SRS-derived `ParamsVerifier` (EOF-checked);
   - `vk.verify` with `transient_crypto::proofs::PARAMS_VERIFIER`;
   - records `proof`/`pis`/`skips`/`binding` only after both verifies.
8. Caller checks public inputs against the finalized call, including `pis.first() == Some(&rec.binding)`.
9. Exclusive write of `proof.raw`, `statement.tagged`, `skips.json`.
10. Intent `sign` then `Transaction::seal`.
11. Public-input identity after sign/seal.
12. Exclusive write of `transaction.tagged`.
13. `accept` → `tx.well_formed(state, WellFormedStrictness::default(), tblock)` → `UnshieldedOffer::well_formed` → `OutputsNotSorted`.
14. Process exits 1. Wrapper therefore never sets `native_postconditions_passed`. Independent wrapper phase `verify` requires parent `exit_code==0` and `native_postconditions_passed: true`; it did not run.

`verify.rs` (independent mutation/WF/accept-again consumer) was not invoked. Provider-internal same-VK verification is a different API from that independent consumer and from ledger `well_formed` proof checks.

`WellFormedStrictness::default()` is native Real with proof-verifying enabled. This attempt never reached those ledger proof checks: offer-output order fails first in `ledger/src/verify.rs` (pinned in the 667 map, SHA256 `35b93fc008ae052e9b7948d1f87959b82e459321a2d40fbc09f099d8dcad25db`):

```
if !outs.is_sorted() {
    return Err(MalformedTransaction::OutputsNotSorted(outs));
}
```

`UtxoOutput` in pinned `structure.rs` (`eebb60349d7c4b1e9954d4e07a1ed389b81612d55d946e2aa43e0525b8bbde9b`) derives `Ord` with `value` first, then `owner`, then `type_`. 1000 before 10 is unsorted.

Official WASM `UnshieldedOffer::new` in `ledger-wasm/src/unshielded.rs` at the same ledger rev `9a8777c4d035fc7f38ae286bcf5f8656668efd9f` sorts `inputs` and `outputs` before the envelope is stored. That file is **not** one of the 667 frozen hashes; it was read at the pinned checkout as supporting official API. The Rust caller `offer()` performs no equivalent sort.

Cargo pins: `midnight-ledger` rev `9a8777c4d035fc7f38ae286bcf5f8656668efd9f`; `midnight-zkir` rev `e82d81d25aabcc5f5092e2bd559083487f577914`; legacy `midnight-transient-crypto` patch rev `a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1`. `Cargo.lock` SHA256 `b0cddb391929e964c82b1587675d9bd5079413e8461b2f2ac2b5e72b8de9b11e`. Official `zkir/src/ir.rs` `61118e731ae492f61bbcae73633c7ba4ab627cbe7cd36b5ac455fc721028052c`. Pay IR `c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae`.

## Provider two-verifier / binding claim — matched strength

Stronger claim that is supported: the executed *source path* records a proof only after finalized nonzero binding, `ir.prove`, and both `vk.verify` calls, then checks public inputs, then signs/seals, then writes `transaction.tagged`, then hits WF. Retained `proof.raw` and `transaction.tagged` plus `identity_ok`/`binary_ok` are consistent with that path completing through step 12.

Claim that is **not** supported: independent cryptographic verification of those 6,336 bytes; ledger-level proof verification inside `well_formed`; `verify::run`; or full Success.

This review therefore treats provider two-verifier passage as **source-ordered, identity-bound, artifact-consistent**, and **un-replayed**.

## Parent / source / binary identity

| Item | Value |
|---|---|
| Source manifest | `73a68f8577c271a236dd737d213742f2a7dcc8d3e5c5e1cab0688ef839adf8df` |
| Wrapper | `f5af2a11614365a702eb805c42c5252b59c9fe7b77e2398ed129e63c7e572687` |
| Config | `96684666c6d855c4992caef4fa65de553a385b5ab1842bd6ef4736173f03ea4b` |
| ELF | `9e1b153a17cd55969dd800fd47bc728923963093267cc8d23765f0a6ea484559` |
| V8 input freeze | `4254e9547d3211c4fd695ebf3627a91add2a565925221e325ece2e35ec723a0b` (667 files) |
| V7 result freeze | `44c63a77861faca25737320db910fa404ae1081653918fceab1593e03b3e3389` |
| Prove-config IR/PK/VK/SRS/handoff/preimage | match the input freeze `prove_config` object and the on-disk prove-config |
| Fixture trust | `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY` |
| SRS | `4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74` (25,166,212 bytes in wrapper asserts) |

Requested identities in the review instruction (`source73a68`, input667/wrapper, ELF `9e1b153a`) match wrapper pins, attempt JSON, receipt, and current disk. Provider/parent attestation of those identities is honest relative to the files now on disk.

`exclusive()` uses `create_new(true)` plus `sync_all`. The attempt file is `O_EXCL`. The consumed reservation remains.

## Root alignment / R2 README (Verdict B evidence)

R2 kernel README change, in order:

1. New **Latest status after V8-03** — V7 keygen complete; V8-03 finalized proof and signed transaction; WF `OutputsNotSorted` before apply; independent verification unrun; full acceptance/custody/correspondence/Preview open; links to refusal and alignment notes.
2. Explicit statement that following sections are the preparation-stage record from before V7 keygen and V8-03 execution, and that missing-keys/proofs sentences describe that earlier stage.
3. Renames: Historical evidence / design / plan **before V7 key generation**.
4. V7/V8 source-preparation heading dated **before execution**, retaining “No V8 authorization or execution exists at this checkpoint.”
5. Reviewed milestone heading records launch; earlier sections are preparation-time status.
6. Closing **Latest actual native refusal** matches the 6,336-byte proof, unsorted offer, consumed attempt, and need for fresh actual-result plus distinct source/resource repair.

Disposition of stale present-tense ambiguity: **adequate for this publication.** Interior historical sentences still speak in present/future (“keygen extension remains uncompiled”, “execution observations will be filed”, “No V8 authorization or execution exists at this checkpoint”). Those sentences now sit under dated historical/pre-execution headings plus the scoping sentence in (2). They are not the current status. Current status lives in the first and last sections, `V8-03-ACTUAL-REFUSAL.md`, `ROADMAP.md` “Current delivery observation — 2026-10-01”, and the alignment review.

Root `README.md` vs `df8b155`: signed-intent consumer does actual Midnight signature verification; authenticated account/chain state, complete native financial acceptance, and ledger settlement remain open; local preparation remains PreparedUnqualified. That matches executed evidence.

Root `ROADMAP.md` adds the 2026-10-01 observation: precursors delivered; latest attempt produced a finalized proof and signed transaction then WF refused unsorted outputs; independent verification and complete financial acceptance open; U0–U7 retained; immediate repair is canonical offer construction before binding/proving/signing; a fixed trusted-genesis proof cannot close U2 or U7. U-table exits are unchanged.

`wiki-llm/ALIGNMENT-REVIEW-2026-10-01.md` (unchanged from predecessor, SHA256 `93b37fd581b4acc613b8bfcdbb2ef3a6b85a082eb021870e171e08a80f7a5b1b`):

- Product contract, consolidated design, single U0–U7 roadmap retained.
- Authoring beta is a delivered precursor; it does not close U2 or U7.
- Fixed manually funded kernel / stipulated identity map is specialization; U2 still needs newly authored plus contrasting programs, generality over the admitted grammar, and tamper controls.
- Native PartialSuccess / guaranteed-phase fees and residual duties preserved; refunds cannot erase gross debit limits.
- Parallel tutorial website with designer reviews is permitted in separate worktrees; not another financial compiler.
- Mandatory properties, intent, transition, history remain controlling acceptance obligations.
- Revisit rule: success is the original supported-language-to-settlement objective.

Those statements agree with `docs/MORIARTY-PRODUCT-CONTRACT.md` and `docs/MORIARTY-CONSOLIDATED-DESIGN.md` as read in this review. Alignment-review sentences about PR15 107-file hashes, scoped CLI tests, and SRC-0117 lint are **root-authored prior observations**; this audit did not re-run them.

Stage mapping held:

| Claim | Mapping |
|---|---|
| Authoring beta | Precursor / tooling, open financial acceptance |
| V7 keygen | Precursor keys for this IR; not U1 full certified basis |
| V8-03 retained proof + signed tx + WF refusal | Executable progress on U1/U2 slice; U2 unclosed |
| U7 independent developer release | Open |
| Mandatory duties / history | Preserved as obligations; unproven on this fixture |
| Recursion / PCD / U4 | Planning horizon unchanged |
| Website | Parallel, excluded from this freeze |

## Open boundaries (unchanged by this failure)

Trusted development genesis; signature owner versus payer (one fixture Schnorr key); authenticated assets/accounts/addresses; funding/deployment; constructor time/history; fees actually consumed (only an unproven margin-2 estimate exists); authenticated context/chain head; generic source-to-native compiler correspondence; mandatory property/intent/transition/history enforcement on native; recursive/private PCD; Preview settlement.

Preliminary fee estimate `1128519108356650` with `vk_present: true` is explicitly unproven. Construction CallContext field equality is a local replay match against the trusted fixture, not authenticated chain provenance.

## Blocker changes and unperformed limits

**Required changes to publish these two R2 scopes: none.**

Together, remaining work that this vote does **not** authorize or complete:

1. Cheap structural preflight that rejects the original unsorted vector and admits a canonically sorted envelope, with Source/Core effect order, recipients, values, assets, and phase meaning unchanged.
2. Fresh source and resource reviews of any changed caller/binary; no unchanged retry of this proof; no silent resource reset; no fifth Compact compile.
3. One new prove/verify/apply allocation only after those reviews; independent `verify::run`; strict WF; full native Success; positive fees/remainder; state/UTXO conservation; replay refusal.
4. Authenticated genesis/deployment/funding/accounts/assets/time/history qualification.
5. General supported-program U2 evidence (new plus contrasting programs, grammar-wide argument).
6. Preview interface/build correspondence before any public financial action.
7. U3–U7 original exits.
8. Website implementation (permitted in parallel; unreviewed here).

Excluded here: source-only sorted successor inspection/approval; resource amendment; candidate execution.

## Honest claim matching

Allowed by this evidence: complete failed-result archive; partial real finalized (ledger-binding) native proof bytes; signed/sealed transaction bytes; provider-path two-verifier *source* obligation before those bytes were recorded; post-child source/binary identity flags; WF `OutputsNotSorted` on the A1 fallible offer; consumed one-shot attempt.

Forbidden by this evidence: full acceptance; independent verify success; apply/poststate; Preview; generic compiler correspondence; U2/U7 closure; treating the retained proof as evidence for a later sorted transaction.

V8-03-ACTUAL-REFUSAL.md, the freeze scope strings, ROADMAP current observation, alignment review, and R2 kernel latest-status section stay within the allowed set.
