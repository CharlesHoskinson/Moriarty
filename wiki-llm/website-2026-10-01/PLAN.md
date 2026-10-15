# Tutorial website implementation plan

Historical October 1 implementation scope, reconciled October 14 against PR50
`fc6353bf19b87a76410319c378efe529d52d23d9`. Completion below means the
[recorded local integration](POLISH-DECISION.md#integrated-result) and
[exact R2 source/result publication decision](publication-reviews/R2-ROOT-PUBLICATION-DECISION.md),
not a fresh run or live deployment check. Current page sources include
[`documentation.html`](../../site/documentation.html),
[`tutorial.html`](../../site/tutorial.html), [`how-to.html`](../../site/how-to.html),
[`reference.html`](../../site/reference.html) and [`explanation.html`](../../site/explanation.html).

User authorizes six exact workers in parallel and GPT-generated illustrations/diagrams. Reuse existing React19/Vite8 static GitHub Pages application as a third tutorial.html entry; no second framework/backend or wallet operations. Follow the existing tutorial site plan and six developer feedback reports. Permissionless financial language with an optional kernel, explicit intent/effect boundaries and honest local/specified/open labels are controlling.

## Tasks and acceptance

- [x] S1 Sonnet5.5: shell, navigation/search/copy, purpose and architecture.
- [x] S2 Sonnet5.5: executable getting-started, transfer/repayment/testing, downloadable examples.
- [x] S3 Sonnet5.5: surface reference and reusable patterns.
- [x] G1 Grok4.7 xhigh: AMM/lending/stablecoins.
- [x] G2 Grok4.7 xhigh: options/oracles/governance.
- [x] G3 Grok4.7 xhigh: bridges/staking/trust matrix.
- [x] Root supplies actual GPT-generated hero and pipeline diagram, with text equivalents.
- [x] Integrate disjoint worker writes, independently check examples/links/statuses, run existing site typecheck/test/build and actual browser tutorial checks at /Moriarty/tutorial.html, mobile and keyboard.
- [x] Fresh full-source/result Astra and Grok publication audits on the R2 freeze; [root decision](publication-reviews/R2-ROOT-PUBLICATION-DECISION.md). These reviews retain their exact historical source/result scope.
- [ ] Fresh live GitHub Pages/developer-release qualification for the current candidate; this reconciliation does not establish deployment, clean independent authorship or U7 acceptance.

No module needs another module's private state; all export self-contained named React components. Use existing warm-paper/rust/ink tokens; responsive readable documentation, complete code with copy/download and side navigation. Cases labeled SpecifiedOnly must not imply financial execution. Native financial repair remains a separate delivery item and website work uses no native resource allocation.
