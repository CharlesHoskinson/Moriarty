# Independent full-source and actual-result audit — candidate 2

Verdict: **requestchanges**

Candidate: `2dfcc57029af302d6bee3f0f3e3936828eac5b2c`.
Base: `2905cb6da0bf0ccfdca28b0f22478e0eda2420e3`.
Manifest: `wiki-llm/signed-intent-2026-10-01/candidate-manifest.json`, SHA256 `e7e8be80024e035c87f8d8abb99b43528b6ae4f4e2406e285d3775bf1856749a`.
All 242 manifest file digests matched the checkout. HEAD matched the candidate and tracked working tree was clean before and after checks.

Requested reviewer route: `gpt-6-astra`, medium. This fresh delegated context received that requested route in its assignment. The host did not expose an actual returned provider model identifier; no returned identity is asserted or invented. This review did not read other reviewers' audit files or reuse their source verdicts. Result documentation links to historical reviews, but those linked reviews were not consulted.

Startup: loaded AGENTS.md and the installed Moriarty development skill; inspected guarded status. No pending public transaction notification. Existing operational/admission gaps remain; this review did not dispatch a campaign.

## Finding

1. **P2 — Native JSON response failures are misclassified as a negative caller judgment.** `packages/moriarty-beta/src/auth.ts:78` preserves any `LocalError` from `parseJsonWithinLimits` when parsing native stdout. Consequently duplicate keys, malformed JSON and parser work/shape limits can escape as `BETA_JSON_*`. `packages/moriarty-beta/src/cli.ts:138` assigns exit 2 only to `BETA_CRYPTO_*`, and line 144 selects the human `judgment` outcome for these escaped parser errors. This violates the explicit native-response failure contract in SIGNED-INTENT.md and PROTOCOL.md: a native transport/response failure must mean inability to judge, with signature validity unknown, and CLI exit 2.

   Actual reproduction on the frozen candidate: configured an absolute temporary executable which consumes stdin and emits exactly `{"status":"a","status":"b"}\n`; invoked the actual source CLI `verify-intent` with the packaged transfer-schnorr-raw source/scenario/signature and `--review`. Observed exit **1**, empty stderr, and:

   ```text
   FormationRejected  BETA_JSON_DUPLICATE
     Outcome: rejected. No effects were published.
     Detail: Duplicate JSON key status
     Hint: The JSON text was rejected by the bounded duplicate-safe parser.
   ```

   Expected: exit **2** and the no-judgment/unknown-signature explanation. A failing or incompatible configured verifier is sufficient; this is not an assertion that an untrusted source controls deployment configuration. This does not cause acceptance, state mutation or a false signature boolean. It does erase the documented distinction needed by callers and the human review flow. The current auth transport test expressly expects `BETA_JSON_DUPLICATE`, explaining why all tests pass.

   Required repair: normalize parser failures at the native stdout boundary into `BETA_CRYPTO_RESPONSE` (or an equivalently classified native-response error), while preserving existing `BETA_JSON_*` handling for malformed caller artifact/scenario input. Add actual CLI JSON and review regressions that require exit 2 for malformed native output and exit 1 for malformed caller input. Preserve the original failure evidence.

## Source review observations

The review covered the selected complete execution chain, rather than the latest diff alone: Rust closed intent codec and command dispatcher, underlying verification adapter, beta authoring/bridge/source-to-Core consumer, complete Core/5 financial preparation, auth API and subprocess protocol, local store and authority resolution, CLI/display/JSON paths, public declarations and build/package configuration, relevant adversarial and financial tests, public fixture generation, protocol/local settlement documentation, numerical circuit and proof test. The 242-file hash check establishes identity, not a claim of line-by-line review of every historical/transitive frontend and evidence file.

- Rust validates the closed versioned schema, canonical decimals and monetary limits, positive amounts, identity distinctions, selected operation, bounded validity window, empty unsupported policies, canonical key encoding and valid native curve keys. ECDSA compressed prefix 02/03 is enforced before the upstream decoder. Projection and full frame have separate tags and big-endian byte lengths; owner projection excludes signature metadata while the actual signed full frame includes it. Wallet framing counts bytes and has no fallback. The ordinary native signing APIs use SHA256 internally; public docs and the independent Python verifier state this distinction correctly.
- Beta binds the entire returned statement to the actual selected source/action and exact UTF-8 source digest. It independently checks frame shape, canonical payload, projection digest and signing-message construction, then calls configured native Rust for the signature verdict. No callback or caller flag supplies signature validity. Deployment configuration intentionally trusts the explicit absolute binary path; this is not an untrusted source field and redundant host cryptography is not required.
- Subprocess input/output/time limits, no shell/PATH fallback, strict UTF-8 and one-line response admission are present. Timeout settles the Promise directly even when a descendant retains pipes. The finding above concerns semantic classification after parser rejection, not bypass of bounded admission.
- Local settlement derives scenarios from immutable store state. Digest includes domain, cells, predecessor, receipts and registry. Native verification occurs outside serialization; authority and digest are checked again in the synchronous serialized commit. Revocation participates in that serialization. Exact retries need native verification and matching frame digest and return acknowledgement without new effects. Conflicting statements cannot use that path. Successor head binds the complete financial effects and frame; Core is rerun before the single root replacement. Fault steps precede root publication.
- Financial review independently checked the manual expectations: transfer 1000 plus fee 10 debits/uses allowance 1010, credits 1000 and 10, leaves payer/allowance 8990; zero fee leaves 9000. Repay 500 consumes 500 accrued interest, leaves principal 100000/accrued 500/outstanding 100500; repay 3000 leaves principal/outstanding 98000; repay 101000 clears the debt and leaves payer 99000. Each uses one work unit and one replay entry. Core checks gross cap including fee, fee cap, recipient net amount floor, balance/counter ranges, complete ordered effects and accrual-first updates. Chained repayment expectations are independently consistent (500 then 3000 leaves 97500).
- The repaired supplementary review lines all pass through escapeAscii. ESC, bidi, newline and tab artifact keys are rejected and cannot inject terminal control or extra review lines; JSON preserves the actual pointer after decoding. Signed CLI JSON and human output retain unqualified status. Public emitted declarations include real scheme/framing unions and concrete statement/receipt types; relative Core type dependencies are emitted under dist/types and packaged.
- Three packed public examples include complete effect/post expectations derived from independent fixture arithmetic, not consumer output. The pack test uses a copied actual native verifier outside the checkout with empty PATH, verifies both output forms, and exercises changed source, expired scenario, forged signature and missing binary.

## Actual checks and retained evidence

Fresh executions in this review:

- `MORIARTY_REQUIRE_NATIVE=1 MORIARTY_CRYPTO_BINARY=/home/charl/research/moriarty-crypto-2026-09-30/target/debug/moriarty-midnight-crypto npm test`: **189 passed, 0 failed, 0 skipped**, including package build/install and real native signature consumers. Log: `/home/charl/research/moriarty-signed-intent-2026-10-01/final-astra-audit-2-tests.txt`.
- `npm run typecheck`: exit 0.
- `python3 experiments/midnight-crypto/reference-intent.py check`: match, **20 golden vectors and 433 corpus entries**.
- Actual malformed-native-output CLI reproduction described above.
- Native binary SHA256: `42fa598d8b54334650f60bc13428bff3d79bb240a5bb0c203fc84684b5a30339`.

Inspected retained result evidence, without rerunning proving:

- `final-beta-tests-2.txt` records 189/0/0 and matches the fresh result. The older 187 result remains historical.
- `final-rust-tests.txt` records 32 passing tests with the numerical native proof test deliberately ignored in that ordinary suite; it is not evidence that the ignored proof ran there.
- `native-run-receipt.json` pins the six current runnable numerical inputs; their manifest digests match. `native-run.log` records the separate one-shot native numerical proof: 10,924 bytes, k10, test exit 0. The source test performs prepare, strict transcript EOF and final guard pairing; checks all 28 public-input mutations, corrupted first byte, half truncation and appended byte. This supports the stated scoped successful experiment. It is receipt/source review, not a fresh proof rerun by this auditor.
- Circuit review found actual range decomposition/equalities for every public numerical value and auxiliary slack, bounded signed money fields, positive amount, conserved balances/allowance/work and owner caps/window. No host success flag substitutes for these constraints. All 28 instance values are constrained to witness cells. The circuit does not bind source/identity/signature/nonce/head or authenticated state.

## Limits

This verdict is not product acceptance. Authentic owner/account binding and current state, persistence or crash recovery, ledger atomic acceptance, native compiler correspondence, PCD, complete financial/ACTUS coverage, ledger-version proof compatibility, wallet compatibility and Preview settlement remain open. The local registry is stipulated and its revoker is not authenticated. Source network labels do not establish a Preview connection. The numerical proof is a standalone local precursor and cannot discharge these gates. No SRS/proof setup, wallet, network, Preview action, source edit, commit or delegated review was performed. External compiler/kernel prototypes are outside this candidate and this review.

Fix finding 1, freeze the resulting candidate, and obtain fresh independent source/result audits for those exact bytes.
