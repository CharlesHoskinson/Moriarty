# Dedicated Cardano Preprod wallet

**For Cardano Preprod test ADA only**, the reviewed funding address is:

```text
addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc
```

Copy it from [payment.addr](public/payment.addr). The address file is 64 bytes, SHA-256 `84897c9f27a559ea8f59893895d7136f361125f9b53a2b43c13c2e909aade299`. **Funding status is unknown.** No ledger query, ownership signature, spend, transaction or settlement was performed. This is a dedicated test wallet creation record, not a completed Preprod prototype.

The [fresh actual-result review](reviews/actual/REPORT.md), SHA-256 `a4deebf6d92123508c8ddb2c428e6127c3fb6d2b18ff0789632fa895b6a92859`, accepts reporting this address to the human for test-ADA funding. The [root actual adoption](root/WALLET-ACTUAL-ADOPTION.json), SHA-256 `771883ed5581fae6de9e4de72c3286fc1ee88e4e0795ef01a4d976c2bad2be93`, closes only `walletCreatedPrerequisiteClosed` and `walletFundingAddressReady`. This README transcribes that qualified result; it supplies no new security judgment.

**What actually ran.** The [public RESULT](public/RESULT.json) is 2,433 bytes, SHA-256 `eb85770af187c927f91cff4a5b5341ae4fcd376f34bce02e69d4b2cd20611dae`, with status `WALLET_CREATED_PUBLIC_EXPORT_COMPLETE`. One offline attempt made four CLI calls, all exit 0 without timeout: address key generation, payment-key hash derivation, address construction with `--testnet-magic 1`, and address inspection. [attempt-ledger.json](public/attempt-ledger.json) records four calls and zero retries. No existing wallet or keys were used, and no signing, faucet, network or transaction action occurred.

Preprod is the **configured target** (`networkMagic: 1`), not an observed ledger. The [address-info export](public/address-info.json) reports a Shelley payment address encoded as bech32; its 29-byte enterprise representation has header `0x60` followed by the [public payment-key hash](public/payment.vkey.hash). The existing audit accepts that consistency with the [public verification key](public/payment.vkey) and the reviewed CLI generation/derivation path. Testnet address encoding alone cannot distinguish Preprod from Preview. Live ownership and spendability remain separate later observations.

**Private-key retention is operator-reported.** The public result reports a retained CLI signing-key text envelope with file mode `0600` in a private directory of mode `0700`. Root and the auditor never read, hashed or inspected that private key; the audit relied on the operator's presence/mode booleans and source path. The key is not an HD mnemonic backup. The owner should protect the private file locally and share only the public address. Private paths mentioned inside archived source or manifests are provenance, not access permission; there is no private-key link or recovery material in this README.

**Source and review history stays intact.** The [R1 source review](reviews/source-r1/REPORT.md) was conditional and not ready for execution: original P-25 incorrectly claimed a 120-second whole-execution bound. The [R2 source review](reviews/source-r2/REPORT.md) reviewed all nine files and accepted an additive correction, with the original five files unchanged. The [source manifest](source/ROOT-SOURCE-MANIFEST.json) pins the nine-file candidate; the operator remains 30,735 bytes, SHA-256 `7944d08ff689172a4985c5de657d4b6add0bb47dc93ecc44774f4c45e8e091b6`.

The [README addendum](source/README-ADDENDUM.md), [scope addendum](source/SCOPE-ADDENDUM.json), [outer proposal](source/OUTER-EXECUTION-PROPOSAL.json), and [new source positions](source/DESIGN-POSITIONS-ADDENDUM.json) preserve the correction explicitly. P-29 supplies the documented outer 180-second bound from operator spawn, including staging, TERM followed by 10 seconds kill grace, and inherited core-dump limit zero. The inner 120-second clock starts after extraction; it is a native-call window, not a whole-execution bound. Each call has a 30-second wall limit, CPU soft/hard limits of 15/16 seconds and a 4 GiB address-space limit. Descendant-session, uninterruptible-process, pipe and non-transitive-CPU limitations remain as qualified by the reviews.

Old P-02/P-11 null or unresolved metadata wording was also superseded additively: P-30 records the measured literal operator/member hashes and static-linkage metadata. P-31 records source-grounded CLI options/defaults/dispatch, and P-32 retains the secret/public boundary. The [root source adoption](root/SOURCE-ADOPTION-AND-ONE-RUN-AUTHORITY.json) records 29 literal author-FOR/fresh-reviewer-AGREE pairs: 25 original clauses plus four new clauses P-29–P-32. The three historical DISAGREEs on P-02, P-11 and P-25 remain unchanged; no replacement votes are invented. Source readiness, proposed authority, actual creation and ledger acceptance are separate predicates.

**Timing and provenance are bounded evidence.** The [root terminal result](root/RESULT.json) observes exit 0, wall 0.9268561039934866 seconds and waited-children CPU 0.74048 seconds. That CPU value is not transitive native CPU. [PREPARED.json](root/PREPARED.json) records the actual outer launch and input pins. Public [binary/ELF metadata](provenance/cardano-cli/DATA-ONLY-MANIFEST.json), [ten-file source-selection metadata](provenance/cli-source/SOURCE-MANIFEST.json), and [five-file supplement metadata](provenance/cli-source-supplement/SOURCE-MANIFEST.json) preserve the recorded CLI 11.3.1.0 acquisition and source pins. Neither tool/archive bodies nor the selected Haskell source bodies are bundled. These pins and the review's dependency on the official binary do not establish a reproducible build or a strong CSPRNG theorem. No binary or private file was re-read to package this capsule.

The source author used Muse under the user's explicit fallback after the original Opus author timed out. The later source and actual reviewers were fresh non-author Muse contexts. Their independence is same-family freshness, not cross-provider independence. HIGH effort was requested; actual provider effort was not independently observed. Preserved review records retain their own identities and limitations.

**The one-run authority is spent.** [AUTH.json](root/AUTH.json) retains historical status `AUTHORIZED_ONE_RUN`; [AUTH-SPENT.json](root/AUTH-SPENT.json) records `spent: true`, one consumed attempt, zero retries and no new authority. The [attempt-start record](root/ATTEMPT-STARTED.json) and [TERMINAL.json](root/TERMINAL.json) identify captured start and completion. Root's at-spawn PROCESS record is not included; copied review process records are historical receipts, not current liveness observations. Earlier “no wallet”, “unrun” or pre-review flags in source/captures remain truthful authoring-time snapshots, superseded only within the later actual adoption's scope. Generic proof-roundtrip recommendation flags were not adopted as wallet evidence.

This fixed public capsule is not a wallet-generation smoke-test or transaction reproduction kit. Its source scripts have hardcoded experiment paths and fresh-leaf guards, with no rerun, old-key, faucet, signing or transaction authority supplied here. Internal AUTH and review controls govern this recorded experiment; they are not public Moriarty compile/prove/deploy admission requirements.

Human test-ADA funding is the next action, followed by a public ledger observation. Any later signature or transaction remains separately reviewed and scoped, with the compiled applied C2 wrapper, genuine script identity, FX1, settlement and qualified transaction-cost work still required. Funding alone does not complete the prototype. All 20 UTxO families, ACTUS/DeFi/general obligations, real recursion, private handoff, bounded multi-parent PCD, universal certificates and actual Preprod transactions remain open. No compatibility percentage is claimed.

[INVENTORY.json](INVENTORY.json) lists all **53 copied public payloads, 422,073 bytes**, with old/new paths, byte lengths and SHA-256:

| Group | Files | Contents |
|---|---:|---|
| Public exports | 6 | Address, public verification key/hash, address info, result, attempt ledger |
| Reviewed source | 10 | Nine R2 source files plus their manifest |
| Root records | 10 | Qualified adoption, source adoption, spent authority, preparation/start/terminal records and sanitized operator captures |
| Review bundles | 24 | Eight allowed public outputs each for R1 source, R2 source and actual review |
| Acquisition metadata | 3 | Binary/ELF, source selection and source-supplement pins only |

Copied payloads are immutable and retain their original links/provenance strings. Use the inventory to locate their public copies. [STAGING-MANIFEST.json](STAGING-MANIFEST.json) distinguishes complete copied-byte verification from partial semantic reading. This staged package still requires fresh independent packaging review before publication.
