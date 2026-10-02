# Independent R4/R5 publication audit — Sol

**Verdict: APPROVE_SCOPED. Blocking findings: none.**

Candidate freeze SHA256: **0c597a0a286271257a54ea1f047a0b9b82fd365fec6a8a9d4f2294fae209ec79**.

Base and inspected HEAD: `1d0d598cd63af591be0b2a9f6a5cdeb6768b93bb`.
Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.
Requested independent auditor: `gpt-6.1-sol`, high. This report is the independent delegated Sol publication review. No host conversation transcript was inspected for model attestation; the parent must retain the actual dispatch/terminal identity receipt. I did not author the candidate, consult counterpart current publication reports, or delegate.

Approval covers faithful publication of the already accepted successful R4 finalized financial proof and complete R5 independent verification for the exact trusted public fixture and fixed production Beta/Core handoff. It neither establishes general product completion nor authorizes native allocation or public submission. The other two required current publication reviews must independently complete on this same digest before publication approval is complete.

## Startup and method

Loaded installed `moriarty-dev:develop`, read the current checkout AGENTS instructions, applicable publication authority, product contract and relevant orchestration stop rules, and refreshed guarded read-only `status --json`. Status reports SP01.6 loan-swap-subset blocked on historical admission/accounting/operational-history gaps and `pendingTransactions: []`. This review stayed within publication scope. Durable `AUDIT-ROUTING-20261001.json` records the later user-selected Sol high, Opus 5.5 high and Grok 4.7 xhigh route; historical repository routing does not override it.

Used read-only Git status/diff/HEAD, standard-library Python hashing/JSON/AST/log checks, source reads and the prescribed archive checker. No native binary, prover, verifier, build, keygen, helper import, network experiment or service action ran. No candidate file was changed. The only authored file is this report outside the freeze. Native source was inspected, not typechecked. Retained tagged poststate was hash-checked, not independently decoded; financial content is supported by the inspected native acceptance path, recorded successful execution and unchanged reviewed outputs.

## Exact file identity and complete archive coverage

Independently hashed every one of the **197** entries in `PUBLICATION-FREEZE.json`: zero missing files and zero mismatches. Recomputed the freeze file's own SHA256 and confirmed the requested digest, then repeated all 197 checks at the end. The checkout HEAD equals the declared base.

Read `archive-manifest.json` and compared every mapped repository body directly with its immutable original: **186/186 byte-identical**, zero missing originals or unequal bodies. All archive files are included in the publication freeze; there are no extra unlisted files under the archive. Each mapped original path is unique. The manifest explicitly distinguishes relocated copies from historical absolute configurations.

The prescribed command completed exit 0:

`python3 wiki-llm/native-financial-kernel-2026-10-01/evidence/native-financial-r4-r5/check-archive.py`

Its actual output was:

`{"files_checked": 186, "actual_frozen_files": 165, "proof_outputs": 22, "verification_outputs": 30, "three_actual_reviews": true, "recorded_result_invariants": "pass", "native_execution": false}`

Inspected the checker itself. It only reads archived files and validates mappings, hashes, sizes, frozen coverage, review/report receipts, recorded result assertions, output sets, fee arithmetic, native source identities and PI-log indices. It imports no archived execution code and invokes no prover/verifier. Its prescribed ordinary-Python command is an integrity and recorded-evidence check, not a fresh native verification claim.

The original actual-result freeze remains `32305c38817649e1d6bca2d51a93360e8dc065584a5bf7c8fe8bf2c764fd3c26`, containing all **165** original actual bodies. The archive also includes all **88** old R4 actual entries and **67** R4 proof-gate entries. Full output membership matches receipts: R4 proof **22 files**, R5 verification **30 files**, and failed R4 partial verification **13 files**. Final three actual reports and their terminal receipts/root acceptance are separately copied rather than falsely described as members of the earlier 165 freeze.

Independently streamed the external R5 input freeze **1130/1130 paths**, 3,299,423,613 bytes; R4 input freeze **1039/1039**, 3,298,681,719 bytes; and R5 source freeze **8/8**. Zero missing files or hash mismatches. This includes the exact 698,019,744-byte native ELF (`ae7e8dab21c4cf09143fa7d58741c0e18ec5f525018de1b8b4099f6fe5a18239`), PK/SRS/runtime/dependency identities. Those local checks do not turn external bodies into Git contents. Both manifest and maintained prose explicitly disclaim a self-contained rebuild/replay packet.

## Source and raw result review

Reviewed the full native consumer modules `src/{main,funding,verify,provider,artifacts,keygen}.rs`, their source manifests, official `deserializable.rs`, bounded wrapper paths, R4/R5 helper expectations, proof/verify configurations and receipt/output sets. Read the external pinned production `native-beta-handoff/handoff.ts` and its README to trace the restricted Beta/Core producer into the native construction. Reviewed current source/resource/actual-result binding records, CPU clarification/root disposition, root acceptance and all three full final actual-result reports. Relevant archived body identities were checked in addition to reviewing the publication diff.

The native producer supplies the finalized nonzero binding through the ledger path and enforces one actual proof, resolver/registered key identity, actual preimage/context equality and skip agreement. Independent verification reads exact pinned proof/statement/transaction/genesis/history/claim, canonical-decodes them, reconstructs the trusted fixture, checks the registered v3 VK and finalized call public inputs, and invokes both explicit-SRS and embedded native verifier parameters. `accept()` calls default Real `well_formed` and actual ledger `apply`; requires full Success; checks escrow 8990, generated expected storage, consumed funding input, A1 recipient outputs 1000/10 and one same-owner NIGHT 1e12 output; checks positive consumed DUST and replay failure with unchanged state. This is the actual fixed consumer's acceptance path, not a receipt-only or host-effect surrogate.

Independently checked both pristine raw logs exhaustively: **1035** mutation refusals in exact index order 0–1034, each `Invalid proof`, then the exact missing/extra PI, actual address/entrypoint/communication/gas/effects, proof corruption/truncation/suffix, four strict tagged suffix refusals, absent native ownership-signature refusal and final native good marker. First-good and good-again logs are byte-identical: **58,725 bytes / 1,051 lines** each. Reconstructed the entire combined phase log exactly from first-good, producer, six faults, final-good and literal helper completion marker: **118,240 bytes / 2,119 lines**. There is no omitted traceback or intervening crash.

For all six history/claim faults, checked native exit 1, absent output path, configuration hash, single selected config-field substitution, artifact hash, raw expected error and log hash. Suffix bodies equal original plus zero; truncations equal the half-length prefix; canonical native alternates differ and produce the exact independent-reconstruction refusal. Raw truncation logs establish `UnexpectedEof`; typed suffix logs establish the official EOF error. Alternate producer and final pristine run have exit 0. The alternate remains a local keyless fixture that also differs in operation registration, not an isolated timestamp-only test or authenticated-history proof.

Application result, poststate and replay artifacts match byte-for-byte across original R4 and both R5 pristine passes; receipts also match byte-for-byte. Receipt poststate/prestate/statement digests match the actual retained artifacts. Application text begins Success; replay records IntentAlreadyExists. Independently verified the positive fee arithmetic:

- consumed DUST `1117490000000001`;
- allowance `100000000000000000000`;
- available `5000000000000000000000`;
- remainder `4999998882509999999999`;
- `0 < consumed <= allowance <= available` and consumed plus remainder equals available.

Receipt runtime 293/47 fields are literals carried from R4 construction's actual transcript gates. The publication explicitly says R5 did not recount them. The receipt does not contain separately named escrow/output amount fields; those values are the native acceptance checks required before it is written, corroborated by retained successful outputs and the reviewed path. This limits independent interpretation of the tagged poststate but does not block this faithful publication.

## Acceptance, dissent and resource scope

Root acceptance remains SHA256 `ad507ee85cd394ad2c5a9d567cae1905ff14a155b32b296d6161786ac0482be6`. The three full final actual-result reports each approve the same actual165 digest. Terminal receipts record actual Sol/high, canonical Opus 5.5, and returned `grok-4.7-build`, with successful completion/end_turn metadata. They are actual-result audits, distinct from earlier source/resource votes and from these new publication audits. The author is not counted as an independent auditor. Raw host transcripts are referenced only by path/digest and are outside the published archive.

Retained R4 proof receipt is exit 0/no stop/error, 287.614 seconds. R5 is exit 0/no stop/error, 67.036 seconds, sampled CPU 58.18 seconds and RSS 716,906,496 bytes. The sampled 120/240/2GiB verification envelope is preserved. Sampling can miss peaks, per-process address space is distinct from group RSS, Rayon is not a hard global thread cap, and the outer service stop has grace. No guaranteed arbitrary-load completion follows.

R4 partial verification remains exit 1/postconditions false, lacks final suite completion and retains the correctly refused history suffix plus obsolete helper-literal failure. R5 preserves that failure and reuses the unchanged proof/ELF. Historical Opus R4 CPU admission refusal and the 2–1 resource disposition remain explicit. Earlier disk/signal/unsorted-offer/NightBalance failures are not relabeled success. Cleanup remains limited to generated .lake/build in two already-trashed repositories; observed 14,917,431,296-byte free-space increase carries concurrent activity/hardlink qualifications. Sol's symlink-cache correction and Grok's historical digest display error are explicitly dispositioned while original reports stay immutable.

## Wiki, provenance, navigation and privacy

Reviewed `R4-R5-REVIEWED-SUCCESS.md`, dossier README, collection metadata, source inventory, portable source ledger and all five canonical wiki paths: session, source ledger, index, log and hot page. New intake is SRC-0119/S3 experiment evidence; it promotes no canonical financial accepted claim. CSV and portable legacy record match field-for-field, including collection hash. Collection manifest/root acceptance/actual165 identities match copied artifacts.

Independently checked `wiki-bundle.json`, `wiki-inspect.json` and `wiki-apply.json`. Canonical sorted compact JSON bundle SHA256 is `3a6569b9341cf6a1275c752ef02255275c9cfb9d52497cb822a92298af1f0bf6`, matching inspection/apply. Inspection valid, apply complete, approval digest consistent, exact same five paths and output hashes. Every resulting file matches its bundle draft hash and publication freeze. The bundle's canonical digest is distinct from its pretty-printed file-byte hash; this is expected, not a failed transaction.

The dossier leads with R4/R5 as latest and labels preceding sections historical. Wiki additions are explicitly dated 2026-10-02; previous failure/preparation observations remain dated history. The broad open gates are preserved throughout: generic compiler/source correspondence, source-owner/native-payer mapping, authenticated funding/deployment, generic contract/intent/transition/history/PCD and Preview settlement. No public transaction or new allocation is claimed.

Inspected publication artifacts and JSON field structure for private thought/credential exposure without opening environments or host transcripts. Provider stdout contains public results and usage counters; thinking-token counts are metadata, not private thought contents. No private-thought block, provider/bearer token, private-key PEM, user wallet import or unredacted environment was found. Deterministic public development signing keys and public-fixture preimages are explicitly scoped; they are not user wallet secrets. Paths to private host records do not publish their contents. Historical Rust/wrappers are provenance assets and are not wired into public product behavior.

## Nonblocking editorial observations

1. `evidence/source-inventory.csv:117` and the corresponding portable legacy record retain confidence text “high for retained identities/Failure” from the earlier failed experiment. The new title, scope, reproduction, S3 status and evidence correctly describe Success. This is a minor wording defect, not a changed verdict or missing proof.
2. `wiki/meta/ledgers/source-ledger.json:4329` has an unrelated legacy range changed from an en dash to a literal escaped Unicode spelling. JSON remains valid and no identity/claim status changed, but future editorial cleanup should avoid unrelated text churn.
3. `R4-R5-REVIEWED-SUCCESS.md:5` summarizes escrow/output predicates as recorded by the native receipt. More precise wording would identify them as checks on the native accepted path accompanying that receipt; the receipt's direct fields chiefly record hashes/fees/status/events. The full source/evidence supports those predicates, so this does not obstruct publication.

No candidate change is required for this scoped verdict. Supplemental publication reviews/check receipts can be appended with their exact hashes as instructed; this report does not require recursive review of itself. Any substantive change to frozen candidate bytes needs current audits of the new candidate.
