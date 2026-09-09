# Independent comparison of report 8 with Moriarty

Reviewer: separate GPT-6 Astra agent, read-only research review on 2026-09-09. Scope: the complete supplied report (623 numbered lines), Moriarty's September 8 taxonomy README/category reference/data method/semantic boundaries/schema/categories, case names and current roadmap/refinement. No external source verification, campaign, implementation, or product-acceptance review. This is independent analysis for the parent researcher, not a candidate-bound acceptance receipt.

Sources below use repository-relative paths. `R` means `.raw/captured/02e57f7b7616b730a7be59cb7cdc63e3753b007c2d0806393d63a35a9a6c1045.md`; `D/` means `deliverables/modern-defi-taxonomy-2026-09-08/`.

**Verdict (inference): retain our existing research model; use report 8 as a clearer introduction and a source of additional case leads.** Its main conceptual architecture already exists in our work. Our research package has more precise boundaries, inspectable records and stronger evidence/version discipline. Our financial-language architecture addresses a different problem; its intended proof guarantees are not yet delivered broadly enough to claim demonstrated superiority.

| Report root | Existing root | Disposition |
|---|---|---|
| FF.MNY | FIN-MON | Close conceptual alias; retain our exclusions for incidental transfers. |
| FF.EXE | FIN-EXC | Close alias. |
| FF.CRD | FIN-CRE | Close alias; preserve our explicit inventory/debt-issuance/transaction-liquidity subfunctions. |
| FF.CAP | FIN-CAP plus FIN-MGT.2 | **Not an exact alias:** report includes treasury deployment; ours separates capital raising from treasury management. |
| FF.DER | FIN-DER | Close alias; scope payoff construction separately from traded or held claim. |
| FF.AMG | FIN-MGT | Close alias. |
| FF.SEC | FIN-SEC | Close alias. |
| FF.RSK | FIN-RSK | Close alias. |

Evidence: R:28–52,81–90; D/README.md:9–22; D/CATEGORIES.md:53–99,245–267,373–381. Preserve report IDs as source-local aliases, not replacements for stable FIN IDs. Flash liquidity is not a new disagreement about a top-level sector: report says use a capability/lifecycle at R:122; our FIN-CRE.3 is a subordinate transaction-scoped borrowing behavior, not a ninth root.

**Where the report is stronger (inference):**

- It communicates the model efficiently: one short chain, eight roots, facet table, concrete flow/claim/control legends and five composition walkthroughs (R:8–12,28–71,308–425). These are useful onboarding patterns, not a new ontology.
- It offers additional or version-distinct case leads absent from our 24-case list: Compound III Comet, 1inch Fusion EVM/Solana, Morpho Vault V2, Morpho Midnight term credit, Liquity V2/BOLD, USDe/sUSDe as standalone positions, Lido V3 stVault, Ondo OUSG and separate SY/PT/YT positions (R:279–304). Our MetaMorpho case is explicitly V1.1, not V2; our Pendle case concerns a historical PT-sUSDe collateral position. These differences are useful source-intake candidates, not verified new deployments or additional language support.
- Its metric paragraph explicitly supplies function-specific measurements and explains recursive TVL double counting (R:73). This is useful presentation guidance. No numerical dataset, exposure reconstruction or quantitative comparison is supplied, so numerical superiority cannot be inferred.

**Where our research model is stronger (repository observation plus inference):**

- Sharper capital boundary: report FF.CAP includes treasury deployment and labels fund components AMG/CAP without an explicit separate-offering test (R:35,86,299–301). Our category rules place passive treasury allocation in MGT, distinguish ordinary share receipt issuance from primary fundraising, and require a distinct primary offering before CAP applies (D/CATEGORIES.md:59,267,283,373–381; D/README.md:49,85). Retain ours. A fund subscription can be CAP if the separately scoped transaction is an actual fundraising offering; it is not automatically CAP merely because shares are minted.
- Sharper zero-label and multi-label rules: incidental transfers, reward emissions, routine share issuance and solver reimbursement do not automatically create payment, security, capital-formation or credit labels (D/README.md:49–57). Report agrees generally but lacks this compact decision procedure.
- Portable, populated evidence: our package lists 31 category records, 47 normative/historical standard profiles, 14 dependency stubs, 253 mappings, 24 cases and a typed graph; pins normative repos by full commits and distinguishes dates, documentation, implementation, deployment, adoption and security (D/DATA-AND-METHOD.md:7–23,31–39). Schema requires source commit/hash and evidence axes and fixes capability mappings to `implementation_claim=false` (D/schema.json:65–79,112–141). This is research-data discipline, not a formal financial proof.
- More executable semantic detail: our vault analysis includes all four preview directions and rounding/bounds, nominal asset/share/request units, pull claims, affected async-preview reverts, ERC-7540's requirement of ERC-7575, cancellation inconsistencies and NAV correction/freshness (D/SEMANTIC-BOUNDARIES.md:20–40). Report explains the families well but with fewer obligations (R:169–185,197).
- More honest coverage characterization: our source method admits no RPC snapshots, protocol execution, conformance tests, reserve attestations or market measurements; classifier check was not blinded and no agreement percentage is claimed; gaps are named (D/DATA-AND-METHOD.md:39–45,75–90). Report calls its 26 examples 'stress-tested' but the supplied Markdown provides classifications, no independent classifier procedure/results (R:273–304). More examples alone do not establish broader validated coverage. Its benchmark does not clearly exercise standalone capital formation or restaking despite their roots.
- Our return account explicitly includes asset-value changes and transaction costs (D/SEMANTIC-BOUNDARIES.md:64–75). Report's simple return identity (R:113) is useful intuition but should not become an implementation equation without defining mark-to-market, timing, reinvestment and leverage.

**Evidence gaps in report 8 (file observation):**

- Web/file citations are opaque session tokens (`turn...`) throughout, not usable bibliography links in this standalone file. This does not mean the underlying claims are false; it means the supplied artifact does not permit independent source lookup from those tokens.
- R:571–577 links three purported companion files at a different session's `sandbox:/mnt/data/` paths. Their contents are not included in the inspected Markdown. Do not claim to have inspected those artifacts or that they do not exist elsewhere.
- The example record has one global confidence and verification date but no source URL/digest/version, deployment address or claim-specific evidence scope (R:544–568). Our actual schema is richer, although deliberately permissive and requiring separate reference checks (D/schema.json:1–4).
- Several formal standard statuses are explicitly unreproduced rather than guessed (R:161–167,579). This is an honest limitation, not a completed verification.
- R:288's “Morpho Midnight” is presented as a named term-credit product. The report does **not** establish a Midnight blockchain deployment or connection to Moriarty. Do not turn the shared word into an integration claim.

**Taxonomy versus formal model:** both DeFi taxonomies classify functions, claims, interfaces, lifecycle and dependencies. Neither supplies an executable transition relation, machine-checked theorem, deployed bytecode correspondence or safety result merely by graphing those relationships. Moriarty's intended language additionally specifies bounded execution, signed intent, contract invariants, transition validity, history compliance and ledger correspondence (ROADMAP.md:3–5,73–77,101–117,184). Those obligations make it a richer target for financial execution, not a demonstrated competitor victory over a taxonomy. The roadmap itself says the taxonomy does not determine Core constructors (openspec/ROADMAP-REFINEMENT-2026-09-09.md:7,48).

**Actual language progress (repository observation, not newly reproduced):**

- Existing experimental syntax/parser/type checker/canonical encoder/local evaluator and restricted Compact lowering; local loan/swap examples (ROADMAP.md:38).
- Narrow atomic boundaries/reviews accepted; complete successor and MC01 open (ROADMAP.md:38; refinement:28).
- Preview hello-world deployment/call recorded, but financial settlement and mandatory PCD acceptance unperformed; failed k17 recursion experiment and no retained successful recursive proof from its replacement (ROADMAP.md:40).
- Successor check/format work is syntax only, with no partial-payment typing/execution; funded repayment projection is local and disconnected from `.mori`, K, authenticated state and Midnight (ROADMAP.md:42–44).
- Startup CLI observed `SP01.6 loan-swap-subset`; implementation/repair blocked by unresolved or unverified operational history; next read-only `sp01-loan-report` allowed; no financial transaction evidence recorded. No campaign was dispatched for this review.

**Useful action within existing tasks (recommendation):**

1. SP01.2/.3: add report FF→FIN aliases and preserve the CAP-versus-treasury/ordinary-share boundary; retain existing identifiers and financial denominator. The obligation metadata/refinement work is already assigned (sp01 sprint:114).
2. SP08.1: source-pin only the distinct cases needed to challenge existing rules, particularly Morpho V2 versus our V1.1 and a genuine standalone primary offering. Existing uncovered crowdfunding, managed-liquidity and default-backstop cases still have priority; report examples do not automatically close them (refinement:54–58; sp08 sprint:98).
3. SP08.2/.3 and SP11.1: use selected report cases as positive/negative comparison fixtures; bind behavior, valuation, exit, authority and standard revision before claiming implementation equivalence. Central async/preview/authority and TX/VX work is already planned (sp08 sprint:99–100; refinement:50–59).
4. SP03.1 remains the first demonstrable capability: source→typed Core→K/evaluator partial payment preserving residual duty. Report 8 provides no reason to replace this with ontology work or create another sprint (refinement:61; sp03 sprint:77–79).

Do not call the eight-root/faceted model, async vault lifecycle, typed interface edges, current ERC-7683 resolver semantics, behavioral compatibility or external-trust separation newly discovered: all appear explicitly in the September 8 package and September 9 task crosswalk (D/README.md:3,49–73; D/SEMANTIC-BOUNDARIES.md:36–60,99; refinement:48–59). The defensible delta is presentation plus case/source leads and a boundary disagreement to preserve, not wholesale replacement.
