# Developer experience result: six completed beta trials

## Result and artifact boundary

All six requested developer seats completed without model substitutions: three Sonnet5.5 trials and three Grok4.7 trials configured at xhigh. Their original programs and bounded case manifests are archived. Parent reproduction records65 original cases matching their stated expectations;12 produce unqualified local money candidates. Independently recomputed money agrees with every supplied successful effect/post expectation. The trials also found documentation, diagnostic and static authoring defects, recorded below.

These are artificial-user trials, not a human usability study. Completion does not establish population usability, a measured five-minute onboarding time or financial settlement.

All seats consumed trial artifact **v3**, package label `0.1.0-beta.1`, SHA-256:

`4841e3697aef99c349b2162a795dfae786d2053912461cf18598d5206b2b4224`.

The package label alone does not identify bytes. The subsequent [v4 repair report](audits/code-repair-v4.md) and [v5 repair report](audits/code-repair-v5.md) describe changed production code/content. This report does not promote archived v3 results into v5 execution evidence. Parent final v5 reruns and fresh v6 package reruns are complete; the final section links the current receipts. This synthesis is not a code authorization or acceptance verdict.

Startup restored AGENTS.md and moriarty-dev:develop, and refreshed guarded status in the assigned worktree. SP01.6 dependent dispatch remains blocked by stale campaign inputs, missing accounting/live state and unresolved operational history; pending transactions are empty. This authorized result synthesis performed no campaign dispatch, signing or network transaction. The independent synthesis initially edited only this file; the parent subsequently reconciled final package measurements and links.

## Requested and returned model evidence

Every [model receipt](developer-trials/S1/model-receipt.json) records requested configuration and returned modelUsage. All process exit codes are0. Sonnet receipts report success with is_error false; Grok receipts report stopReason end_turn. Reports, feedback, original program/manifests and parent reproduction exist for every seat.

| Seat | Requested configuration | Returned modelUsage name | Assigned original work | Receipt |
| --- | --- | --- | --- | --- |
| S1 | claude-sonnet-5-5; default effort | claude-sonnet-5-5, canonicalModel same | Merchant invoice, exact scale3 fee | [S1 receipt](developer-trials/S1/model-receipt.json) |
| S2 | claude-sonnet-5-5; default effort | claude-sonnet-5-5, canonicalModel same | AccrualFirst partial/interest-only/full repay | [S2 receipt](developer-trials/S2/model-receipt.json) |
| S3 | claude-sonnet-5-5; default effort | claude-sonnet-5-5, canonicalModel same | AMM/oracle authoring plus local fee transfer | [S3 receipt](developer-trials/S3/model-receipt.json) |
| G1 | grok-4.7; xhigh | grok-4.7-build | Two independent payroll transfers | [G1 receipt](developer-trials/G1/model-receipt.json) |
| G2 | grok-4.7; xhigh | grok-4.7-build | Draw, installment, payoff and lending authoring | [G2 receipt](developer-trials/G2/model-receipt.json) |
| G3 | grok-4.7; xhigh | grok-4.7-build | Bridge/staking authoring and nominal identity probes | [G3 receipt](developer-trials/G3/model-receipt.json) |

The returned usage names support the recorded model routing. Effective effort has no independent provider attestation; xhigh is the explicit requested configuration. No seat was replaced by another model. No missing model receipt or model-routing contradiction was found. No private reasoning transcript is reproduced here.

Each seat's artifact-manifest.json binds the same v3 tarball hash. Independently checked all677 listed archived file entries against their SHA-256 values: none missing or mismatched. Original trial inputs, outputs, reports and feedback remain preserved; subsequent repair reports do not rewrite their observations.

## Programs, manifests and reproduction counts

| Seat | Original source and report | Original case manifests | Cases | Successful local money cases | Parent reproduction |
| --- | --- | --- | ---: | ---: | --- |
| S1 | [merchant source](developer-trials/S1/merchant/merchant.mori), [report](developer-trials/S1/report.md) | [merchant](developer-trials/S1/merchant/mori.tests.json) | 4 | 1 | [S1](developer-trials/S1/parent-reproduction-v3.json) |
| S2 | [repay source](developer-trials/S2/loan/loan.mori), [report](developer-trials/S2/report.md) | [loan](developer-trials/S2/loan/mori.tests.json), [invalid variants](developer-trials/S2/loan/invalid/mori.tests.json) | 13 | 5 | [S2](developer-trials/S2/parent-reproduction-v3.json) |
| S3 | [pool/oracle source](developer-trials/S3/src/pool_oracle.mori), [fee transfer](developer-trials/S3/src/swap_fee.mori), [report](developer-trials/S3/report.md) | [good](developer-trials/S3/cases/good/mori.tests.json), [bad](developer-trials/S3/cases/bad/mori.tests.json) | 9 | 1 | [S3](developer-trials/S3/parent-reproduction-v3.json) |
| G1 | [payroll source](developer-trials/G1/payroll/payroll.mori), [report](developer-trials/G1/report.md) | [payroll](developer-trials/G1/payroll/mori.tests.json) | 7 | 2 | [G1](developer-trials/G1/parent-reproduction-v3.json) |
| G2 | [servicing source](developer-trials/G2/loan-desk/servicing.mori), [report](developer-trials/G2/report.md) | [loan desk](developer-trials/G2/loan-desk/mori.tests.json) | 14 | 3 | [G2](developer-trials/G2/parent-reproduction-v3.json) |
| G3 | [bridge/staking source](developer-trials/G3/bridge-stake/vault-bridge.mori), [report](developer-trials/G3/report.md) | [bridge/staking](developer-trials/G3/bridge-stake/mori.tests.json) | 18 | 0 | [G3](developer-trials/G3/parent-reproduction-v3.json) |
| Total | Six assigned trials | Eight original manifests | **65** | **12** | All original cases match expectations |

The65 original rows are12 PreparedUnqualified,17 CoreRejected,12 AuthoringRejected,21 Unsupported and3 FormationRejected. TestsPassed means the supplied expectations match these statuses; it does not mean65 payments executed. G3's18 original rows are14 Unsupported plus4 AuthoringRejected: thirteen bridge/staking actions, an unsupported malformed-fixture probe, an unknown composed action, and three nominal/identity rejections. It has no original financial execution.

Twelve successful original cases contain **11 complete ordered effect assertions and10 complete post assertions**. S2's last-valid-round10 case omits post; its first-valid-round0 case asserts status only. Their arithmetic is independently derivable, but those two test rows establish less observation coverage than the ten full-vector/full-post rows.

Additional controls are separate:

- Six copied transfer starters each pass their one complete expectation. These are six copies of one shipped example, not six original solutions.
- S1's [negative-control manifest](developer-trials/S1/merchant/negative-control/mori.tests.json) is a4-case copy. Exactly one deliberately wrong rejection code fails;3 rows pass, command exit1/TestsFailed. It proves mismatch sensitivity without adding original cases.
- Other deliberate failed expectations and discovery probes remain in seat outputs/logs, outside the65-case total. They are not new original passing tests.
- G3's two successful local money probes are separate from its bridge/staking assignment manifest: a25000-atom zero-fee transfer and a125-atom repayment. Its one-atom unfunded foreign transfer rejects. None executes bridge/staking.

Thus parent manifest reproductions cover65 original rows plus6 starter rows, with a separate4-row intentional control. Repeated runs and copied files are not additional coverage units. The original parent reproduction described here is local argv-only evidence against archived v3; final v6 reproduction appears below. Neither is native execution.

## Independent money result

The [independent arithmetic report](audits/developer-arithmetic-result.md), SHA-256
`181c8d495964ed7045c887b2c97d55c71459225835151b75404b01c3ca0fb02c`, recomputes integer effects/post from explicit source amounts and scenario inputs without importing the implementation. It checks all supplied successful original expectations, six copied starters, G3's successful local probes and the current shipped repayment tutorial fixture. No supplied successful money amount, ordered effect or asserted post discrepancy was found.

Representative independent results:

| Source/fixture | Result |
| --- | --- |
| S1 scale3 invoice125.375+fee1.250 | Gross126625 atoms; balances373375/125375/1250; allowance73375 remaining/126625 spent |
| S2 principal500/accrued12.50 | Pay100 leaves412.50 principal/accrued0; pay5 leaves500 principal/accrued7.50; independent full512.50 settles |
| S3 transfer250 plus3×0.25 fee | Gross25075 atoms; balances74925/30000/75. No AMM output or invariant is computed |
| G1 Alice/Bob wages and exact fees | Gross246300/188490 atoms; balances4753700/260075/1475 and3011510/187550/9740 |
| G2 installment218.75 on principal1000/accrued18.75 | Pays18.75 interest and200 principal; residual800/accrued0. Separate payoff806.25 supplies new accrued6.25 |
| G3 separate local transfer/repay probes | Transfer leaves975000/25000/50000; repay125 clears accrued125 and leaves principal/outstanding5000, balances9875/125 |

Effect order, exact economic IDs, allowance remaining/spent, work remaining/spent, tuple replay consumption and stipulated head successor agree where asserted. Successful preparation uses one work unit; observation round stays unchanged. Zero fee credits are omitted. Repayment applies accrued interest before principal.

These fixtures are independent preparations. S2 partial/full/interest-only restart the same opening loan; G1 Alice/Bob use different fixtures/heads; G2 payoff is not the installment candidate fed into a ledger. Source schedule constants unused in operative terms do not bind fixture liabilities. Head/predecessor strings, consumed replay and spent counters are candidate projections from untrusted state, not authenticated chained history or accrued interest evidence.

## Original discoveries and recorded repair dispositions

Original developer reports retain their labels and uncertainty. The following column reports the author’s v4/v5 dispositions; it does not independently certify changed code or imply the developers reran final v5.

| Original finding / source | Recorded disposition and remaining boundary |
| --- | --- |
| S1/S2/S3/G1/G2: missing repay lesson/schema; units, economic IDs, replay and hash claims unclear | v4 supplies repay init/template/example, complete fixture/test reference and getting-started guide; independent arithmetic checks the tutorial literals. Final v6 consumer reruns pass; see final reconciliation below |
| S1/S2/S3/G1: failed tests hide the differing field | v4 reports bounded first mismatch pointers and expected/actual summaries, retaining exact comparison and exit1 |
| G1 Medium: misleading call indentation; S1/S3/G2 similar friction | v4 delimiter-aware formatting keeps named calls compact; claimed-hash changes still explicit |
| G1 Medium: scenario identity/pointer diagnostics | v4 separates domain/asset expected/actual IDs and uses escaped child pointers; source spans distinguished from fixture locations |
| G1/G2 Low: const hover calls money identity metadata | v4 quantity/checked-value hover; unverified identity warning retained for identity declarations |
| G2 Low: slash diagnostic wrongly calls an ASCII character non-ASCII | v4 explicitly explains unsupported division; no division/rounding feature added |
| S1/G1/G2 init and help friction | v4 BETA_INIT_EXISTS and per-command help; missing files/other filesystem errors remain command failures |
| G2 Low: refused expansion/simulation lacks qualification; S3 empty catalog premises confusing | v4 failure qualification and documentation. v5 distinguishes pure authoring/contextual shapes; empty SpecifiedOnly S0 catalogs do not close any gate |
| G3 severity-specific discoveries | Exact disposition table below; original static authoring defects were found even though all original manifest expectations passed |

S1 explicitly found no wrong money result/crash; S2's low-medium preflight suggestion and G1's broken-cap check are structural-versus-Core boundaries, not evidence of financial acceptance. Shared S0_INTENT_SCOPE/S0_EFFECT_RANGE codes, verbose machine output, explicit fixtures and one-action preparation remain documented limitations/preferences. Source/6 bare nonce versus Core tuple replay is a boundary mapping, preserved and explained. Node NO_COLOR/FORCE_COLOR stderr warnings belong to the trial environment.

### G3: preserve original severities and narrow their meaning

| G3 original label | Original discriminator | v5 recorded disposition / limit |
| --- | --- | --- |
| **High1** | Home owner escrows foreign nominal amount; bridge skips all domain arguments | Restrict foreign exceptions to named source/destination; require local custody/account/amount alignment. This repairs static nominal authoring, not cryptographic authority or bridge execution |
| **High2** | Optional horizon domain/asset/signer headers disagree | Align provided headers, operation-local domains and rounds domain. Financial relations/authentication remain open |
| **Medium3** | Bridge cap uses wrong nominal asset | Bind cap unit to explicit asset or inferred local bridge amount; AMM output floor retains output asset. No financial cap inequality is executed for SpecifiedOnly |
| **Medium4** | retained_duties accepts number4; signed_floor accepts quantity | Intent retained-duty hints restricted to text/string arrays; stage hints to text only. signed_floor explicitly permits text or local Qty, so the observed quantity is a supported hint, not inherently erroneous. Hint linkage, freshness and enforceable duties remain open |
| **Medium5** | Duplicate domain/share_class/id accepted | Duplicate share identity rejects within kind/domain; this does not establish share accounting or custody |
| **Medium6** | Unsupported accepts even an invalid scenario expectation | Support refusal remains before the absent profile evaluator; NotAppliedUnsupported/diagnostic explains scenario not applied. It deliberately does not pretend to validate or execute a bridge fixture |
| **Low7** | Empty/partial policy accepted | Retain partial SpecifiedOnly hints and document that policy/duty-preservation relations are not validated. This is a deliberate partial authoring boundary, not a financial policy certificate |
| **Low8** | Existing init and missing manifest report raw filesystem errors | Existing-target init gets BETA_INIT_EXISTS; other filesystem failures remain stderr/nonzero command errors |

The original High/Medium/Low rankings above are preserved as reported. Their scope is source authoring and presentation; none demonstrates a bypass of an executed bridge/staking relation, because those profiles publish no financial effects. v5 records targeted static repairs and clarification choices, with fresh final v6 reproduction recorded below. Unchecked reads/writes/stage-link strings, evidence freshness and earliest-round financial relations are explicitly open, not silently certified.

The v4/v5 reports also include separate code-review discoveries: invalid UTF-8/BOM handling, bounded inspection refusal/depth, additive check metadata, consistent coverage labels and VSIX packaging exclusions. These are distinct from developer trial discoveries. They do not add trial cases or constitute approval by this result report.

## Editor and AI evidence

All six reports describe real local MCP stdio use. G1/G2/G3 also used local LSP clients; G2/G3 ran the shipped headless Neovim smoke with its own demo buffer. Those scopes cover protocol operations, not all editor features or their complete financial programs inside an editor. S1/S2/S3 did not use LSP/editors. No trial establishes VS Code extension installation/activation or provider adapter adoption. Reading inert AI instructions, MCP preview and an AI-authored program do not authenticate source claims or a scenario.

## Six website comments and the concrete outcome

| Feedback | Main requested lesson/layout |
| --- | --- |
| [S1 website comments](developer-trials/S1/website-feedback.md) | Atoms/name-to-ID table, rejection/test schemas, readable first result, repeated qualification boundary |
| [S2 website comments](developer-trials/S2/website-feedback.md) | Worked AccrualFirst partial/full/interest-only lesson; repay starter and mismatch differences; prefers machine output first |
| [S3 website comments](developer-trials/S3/website-feedback.md) | Show local result beside unsupported refusal; keep full explicit intent appendix; distinguish empty catalog premises |
| [G1 website comments](developer-trials/G1/website-feedback.md) | Independent payroll fixtures, exact agreed fees, source claim versus digest, readable output before origin maps |
| [G2 website comments](developer-trials/G2/website-feedback.md) | Worked note service, supplied accrual and unverified predecessor, closed fixtures, optional later editor/AI page |
| [G3 website comments](developer-trials/G3/website-feedback.md) | Status-specific support matrix, nominal domains/assets, no composed bridge/stake action, scenario-not-applied warning |

The [tutorial site plan](TUTORIAL-SITE-PLAN.md) selects versioned repository Markdown, a narrow content column, persistent navigation/search/contents, copyable commands and accessible text support labels. It orders install→starter→units→transfer/repay→fixtures/tests/rejections→optional editor/AI→support/limits/reference. [GETTING-STARTED.md](../../packages/moriarty-beta/GETTING-STARTED.md) supplies the concrete transfer and repayment content, full schema/identity/unit/replay/hash explanations and open gates; independent arithmetic checks its money claims.

Framework suggestions differed: mdBook, Docusaurus, MkDocs, Starlight and VitePress were preferences, not a comparison study. The selected outcome is the guide and content/layout plan. No website deployment, framework installation or example-running website CI is claimed. Future site acceptance requires commands against the final pinned release and actual deployment evidence. A measured human onboarding study remains unperformed.

Retained dissent/preferences: S2 prefers machine JSON first; most seats prefer readable check. S3 values all explicit signing fields; complete source remains visible without hidden financial defaults. Richer summaries, distinct explanations for shared Core codes and broader inspect projections remain future preferences. Tutorial source/outputs must be regenerated when artifact bytes change rather than copied indefinitely from v3.

## Final consumer reproduction and boundaries

- All required seats completed; receipts and677 manifest entries are present and consistent. No model substitution or missing archived receipt was found.
- Parent argv-only reruns against the freshly installed v6 package pass all65 original cases and six starter cases. The four-row S1 negative control intentionally remains TestsFailed/exit1. Each seat preserves its [parent-reproduction-v6.json](developer-trials/S1/parent-reproduction-v6.json) alongside original v3 and v5 evidence; use the corresponding seat directory for the other five receipts. G3 authoring probes reflect the repaired static rejections. No original expected money output was rewritten.
- The two weaker S2 rows remain original coverage limitations. New expectations must come from independent inputs/arithmetic, not copied simulator output.
- Every successful money result here remains PreparedUnqualified/local-stipulation-only. Four external premises remain canonical intent signature, snapshot-to-head, head extension and atomic ledger compare-and-consume; four unverified bindings remain agreement ID, selected program, asset scale and authenticated predecessor.
- No trial or synthesis closes authentication, native proof, financial correspondence, ledger acceptance, horizon execution or full eight-family financial coverage. Catalog check success and Unsupported test success cannot replace those obligations.

The demonstrable outcome is six completed model trials, preserved original evidence, independently consistent local money expectations, concrete feedback-driven repair dispositions and a supplied tutorial guide/plan. Final v6 consumer reproduction is complete; financial/native qualification remains open.


### Final artifact and additional client observation

The final v6 tarball SHA256 is `f6de131ef477c3139ca43a9c5f416daadbc11ef506dbbb4309f85b1a242bbd74`. Its installed CLI matches current dist and the extracted VSIX server byte for byte; [artifact receipt](evidence/artifact-hashes-v6.json). The [current beta suite](evidence/beta-tests-final-v6.txt) passes79/79 tests. These are parent observations, separate from the developers’ original v3 results and model verdicts.

A separate bounded [native Claude Code MCP observation](evidence/claude-mcp-activation-v5.json) connects the real host to the byte-identical executable, performs actual check and inspect tool calls with correlated input hashes, and exits successfully using exact Sonnet5.5. This is a client activation experiment, not a seventh developer trial. It does not establish adoption of inert project instruction adapters, other AI clients or VS Code GUI activation.
