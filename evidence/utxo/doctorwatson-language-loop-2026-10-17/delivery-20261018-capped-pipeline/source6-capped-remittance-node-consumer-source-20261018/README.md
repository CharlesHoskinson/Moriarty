# Independent capped-remittance real-consumer source extension

Status: COMPLETE SOURCE/TEST PATCH, UNAPPLIED and UNRUN. Base K is34257f3ef95e7a1bf29031417e7f6d33f708f06a, treef529a1924885019aa884bc6447f8d47fff47526f, on mainbasee4d7a2b5709caa18a94f5256d03a0302364b8235. No Node/Rust/compiler/test, SDK/native/proof/model, provider dispatch, Git or shared worktree writes occurred. All author writes are in this new R directory. Root integrates/audits and selects any later execution.

`CANDIDATE.patch` adds a dedicated opt-in `experiments/moriarty-language/regressions/rust-source6-capped-remittance.mjs` and a new `regressions/fixtures/source6-capped-remittance/` namespace. It does not alter the older `rust-source6-transfer.mjs`, old two-program `FINANCIAL-PRE-POST.json`/`FIXTURE-PINS.json`, old fixtures or their custody records. A small dedicated test avoids changing an existing frozen consumer/helper interface; it uses the same actual TypeScript APIs and public CLI process path.

## Exact independent input custody

The complete independently authored `source6-capped-remittance-independent-oracle-20261018` is copied without byte changes under the new fixture namespace's `oracle/` directory. Its23 frozen members plus `FREEZE.sha256` are present. The new namespace's separate `FIXTURE-PINS.json` pins those exact copies and the original identities; it does not rewrite the independent freeze or make old receipts describe this candidate.

- FREEZE.sha256 SHA256: `4a859749e91cc31cce74a1e9220e91235c9263a50e5dee1d84949e158885ba79`.
- Original MANIFEST.json SHA256: `c911856c73f9970bc87187ad132412b1f142c4721c2891cb5dcfe1cb5758b5d1`.
- EXPECTED-SEMANTICS.json SHA256: `da997f7f4f59300007cb0368b2d96b728049623315e6649e1bf9c6cccf6fd624`.
- Source input SHA256: `898ffbec44b4165f1afb298398fe36fccc7459e2c1ff92ca7be3ea61fb7c043c`.
- Scenario input SHA256: `6c8c1548cbbc52eca5584f16d4e6e873b245841e6697ad816d22afe73b361dfa`.

Both inputs equal the exact integrated K source/scenario bytes. The independent author did not read the new producer's expected Source/6 fixtures, new test body or author shape. Earlier source/emitter exposure remains disclosed by its unchanged provenance. This consumer author read the frozen oracle only after its terminal freeze notification. No product-derived expectations were created or substituted.

## One callable predicate

For each selected `pay_tight` and `pay_loose` action from the unchanged source and the same independently supplied opening scenario, actual public Rust expansion stdout is consumed untouched by **both** `parseAndLowerSource6` and `prepareSource6S0Unqualified`. Its full JSON-observed Source/6 AST, all ordered source/Core effect records, lowerer intent/input financial projections and returned conditional candidate financial projection must equal the independently frozen literal expectations. Missing compared projections are failures, not fabricated candidate objects. Public process exit/quiet/newline checks use the adopted CLI interface, separately from the oracle's unknown result-status labels.

The TypeScript calls receive only the actual UTF-8-decoded public stdout string. There is no trim, JSON unwrap, author fixture Source/6 substitution, regenerated AST, handwritten typed state or host simulation. `TextDecoder` is fatal and retains a possible BOM in the decoded input rather than stripping a prefix. The independently rendered Source/6 documents and ordered-line files are preserved as normative derivation material, **not** fed to the consumer and **not** treated as byte-for-byte stdout goldens. Actual whitespace bytes remain a captured observation.

## Full capture before product assertions

Each test first spawns the explicit Rust binary with the real `--quiet … expand … --emit-source6-transfer` arguments and a fresh per-action destination. It writes complete stdout and stderr bytes and actual process metadata, including errors/signals/exit and byte hashes. It then independently attempts both real TypeScript calls; decoding and either API's thrown errors are recorded. `actual-consumer.json` contains both complete returned objects or the actual caught error details before any product-result assertion.

The JSON record is the observation representation. An undefined optional return member is omitted by JSON serialization; no field value is recalculated and no expected AST is constructed from the product. The script checks the complete recorded actual AST against the independently frozen JSON AST. Thus the TypeScript object's temporary `obligation:undefined` property is not confused with an oracle claim about a JavaScript own-property API or a full Core Program wire format.

`run.json` also captures the actual Node version, selected binary/hash and oracle identities. Exact source/scenario and original oracle manifest/freeze bytes are retained alongside observations. Input hash preflight occurs before product invocation; assertions about product results occur only after all available process and consumer captures.

## Compared independent fields

The complete AST equality covers profile/agreement/domain/settlement/selection, all intent and signed/submitted-action fields, every supplied claim, validity/caps, explicit empty/none forms, all three opening balance rows, allowance, replay/input work/head/predecessor/round, complete ordered effects and proposed post head. Source/6 UseReplay is the bare nonce; the independent Core effect vector contains its documented encoded domain/signer/nonce tuple. Both complete five-record vectors are compared directly, with no amount calculator or field-name effect adapter.

The lowerer comparisons retain all supplied opening balances/allowances, head/round and input work12/2; the finite known intent projection includes source/profile/action IDs, literal claims, identities, validity/caps, amount/fee and endpoint fields. The source quantities are210/225 and fee0 from the independent oracle, not compiler calculations.

The independently conditional closing projection is checked for domain, exact asset, all3 balance rows, allowance, headc1, round4 and selected replay key. Tight: Owner790/Recipient215/Fee7 and allowance790 remaining/235 spent. Loose:775/230/7 and775 remaining/250 spent. Both retain the zero-fee recipient's preexisting7. The proposed head and candidate predecessor/effects are compared to the independent literals. The two cases are alternative independent expansions of the same opening inputs; no sequential or shared-nonce acceptance is inferred.

## Explicit unknowns and limits

No assertion predicts or certifies `outcome.status`, `candidate.status`, qualification, requiredPremises, unverifiedBindings, gates, measured cost/work, post-work or complete result wrappers. Their actual fields remain captured. Candidate financial fields must exist to perform the positive comparison; that shape requirement is not promoted into an independent status or ledger-acceptance oracle. Public exit/quiet/LF checks are ordinary adopted-interface checks rather than oracle-authored Core classifications.

The independent closing record is a **conditional financial projection**, not a complete Core post. Global/unlisted replay inventory, generic residual-duty DTOs, whole Core Program/ExprIds, Target, full Artifact/ABI/ZKIR, proofs/native/ledger/authentication and all parent acceptance remain open. Only the selected replay key's presence is compared; no complete-history claim follows. No obligations/global-frame inventory is invented to fill the independent oracle's explicit unknowns.

## Future integration/check surface

After root's exact-source audit and authorization, the dedicated opt-in regression uses the same environment interface as the old regression:

```sh
MORI_RUST_BIN=/absolute/reviewed/rust/mori \
MORI_SOURCE6_OBSERVATIONS=/fresh/absolute/observations \
node --test experiments/moriarty-language/regressions/rust-source6-capped-remittance.mjs
```

This is a proposed invocation, **not executed evidence**. The selected binary must carry the reviewed `source6-expansion` implementation. This patch adds no build, package script, scheduler, network step, SDK operation or automatic hosted check.

Static verification covers all independent custody hashes, source-base snapshots, exact K input equality, new-path-only applicability, unchanged old fixture identities, JSON parsing, capture-before-assert ordering and exact full-diff reconstruction. JavaScript/Node syntax and actual API behavior remain UNRUN; fresh source audit and actual-result review are separate. Preserve all original grammar/four-golden/current gates and U0–U7/M1–M17/MC/SP/G, full financial/composition and target/release requirements without percentage or inferred closure.
