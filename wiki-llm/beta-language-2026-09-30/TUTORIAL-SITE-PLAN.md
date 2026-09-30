# Tutorial GitHub website: content and layout decision

Recommendation derived from six independent developer trials, not a deployed
website or an empirical comparison of documentation frameworks. Original feedback
is preserved: [S1](developer-trials/S1/website-feedback.md),
[S2](developer-trials/S2/website-feedback.md),
[S3](developer-trials/S3/website-feedback.md),
[G1](developer-trials/G1/website-feedback.md),
[G2](developer-trials/G2/website-feedback.md),
[G3](developer-trials/G3/website-feedback.md).
The delivered [getting-started guide](../../packages/moriarty-beta/GETTING-STARTED.md)
implements the prerequisite content and exact examples now.

## Selected layout

A Git-backed, versioned documentation site: narrow readable content column,
persistent left navigation, in-page contents on the right, search, copy buttons,
repository/version links. Use accessible text support labels next to examples:
**LocalS0**, **SpecifiedOnly**, **Open**, **PreparedUnqualified**.
Color complements these words; it never carries the status alone.
Show a small real program and a readable CLI result first. Financial principles
and explicit open gates appear with the example, before a reader mistakes local
preparation for a settled payment. Dense JSON/origin maps sit behind a details
section or a reference link. No dashboard or simulation service is needed.

## Navigation and learning sequence

| Page | Reader learns | Required runnable evidence |
| --- | --- | --- |
| Overview | Financial intent, explicit bounds, current support | Small transfer + visible unqualified status |
| Install | Node24+, checkout or pinned `.tgz`, no npm-publication assumption | Help; init fresh directory; safe existing-directory refusal |
| First five minutes | Author/check/test/prepare | Transfer starter, readable check, complete test, simulate |
| Units and identity | Declaration name vs economic ID vs display symbol; scale/atoms | 10.00 USD →1000 atoms; scenario uses asset ID A |
| First transfer | Gross includes fee; nonce/head/window authority | Invoice, then original merchant/payroll example |
| First repayment | AccrualFirst; interest-only/partial/full | Independent amount derivation, complete obligation/effects/post |
| Fixtures and tests | Untrusted explicit state, closed JSON, exact comparisons | Full schema; duplicate-key rejection; mismatch pointer |
| First rejection | Check success can precede Core rejection | Wrong asset, precision, expired window, fee cap, replay/stale head |
| Editor and AI | Highlighting/LSP/MCP and optional project adapters | Exact setup commands, separate tested/unperformed activation |
| Support matrix | All eight DeFi areas, local/specified/open split | Unsupported expansion/simulation with null effects |
| Intent and kernel | Signed owner terms vs candidate/service evidence | Typed horizon links; no pasteable claim of beta grammar support |
| Limits and reference | Premises/bindings, APIs/diagnostics/exit codes | Four local external premises/four unverified bindings |

"Five minutes" is a content target, not a measured human completion time. The six
agents are a small artificial-user cohort; their successful programs do not
establish population-wide usability.

## Copy and examples

Teach `check` text before `--json`, and use JSON for automation. Show economic IDs
beside source aliases and symbols. Annotate display amounts and atom strings on
the same line. State inclusive round windows, unchanged observation round,
Source/6 bare nonce versus Core tuple replay key, and claimed `source_hash` versus
computed `sourceHash`. Explicit intent terms and complete closed fixtures remain
visible; do not hide defaults that change financial meaning.

For repayment, hand derive interest/principal changes before running a tool.
For payroll, two intents mean two independent preparations/scenarios; do not
imply atomic chained execution. Source constants do not bind fixture liabilities
unless used in operative signed terms. Scenario predecessor remains unverified.

Teach one deliberate failed money expectation: the test must fail and identify
its differing field. Keep it separate from a rejection test that expects Core to
reject. Never teach copying simulator outputs into expected results to force green.

All code blocks should come from versioned runnable example files. Store their
independent expectations, actual command outputs and the release digest together.
A future site CI can run the displayed CLI commands and compare scopes/statuses;
site CI is proposed, not installed by this delivery. Each displayed output must
state which release produced it; historical feedback describes the v3 trial
artifact and does not replace the final package's current verification.

## Framework and deployment boundary

The developers suggested Astro Starlight, mdBook, Docusaurus, MkDocs and plain
Markdown. These are preferences, not benchmark findings. Select **portable
Markdown content in the Moriarty repo first**. A future site build may use an
existing repository documentation tool or Starlight if its search/navigation and
GitHub Pages requirements are verified at that time. Avoid adding a second
framework merely to publish this feedback. Current scope supplies the content,
examples and layout plan; no website deployment or framework installation is
claimed.

## Preserved differences in preference

S2 preferred complete machine output first; S1/S3/G1/G2 prefer the readable summary.
Choose readable check for the first page with an explicit JSON link.
S3 prefers keeping every signed field prominent; some readers prefer collapsed
boilerplate. Choose complete source files with short explanatory excerpts and
expandable full intent; no hidden signing defaults.
Grok's code reviewer preferred a closed check summary; retain bounded additive
span metadata for tools and document keys. A simpler future JSON view can be
added as a separate API without deleting established authoring metadata.

## Getting-started exclusions

Defer source-origin graphs, all profile internals, VSIX build machinery and the
full typed lifecycle grammar to advanced pages. Provide direct links for readers
who need them. Omit fabricated signing/proving/sending/deployment steps, wallet
setup, authenticated provider shortcuts, percentage rounding, arbitrary async
loops and unimplemented DeFi execution. MCP preview remains a local preparation.
Do not modify global editor/provider configuration as part of a tutorial.

## Future site acceptance checklist

- Fresh Node24 installation runs the transfer and repayment lessons outside a
  Moriarty checkout using a pinned package artifact.
- Every shown source and fixture exists and has independent complete expectations.
- Wrong expected money fails; a Core rejection publishes no new effects/post.
- Identity/unit terms and four premises/four bindings are visible and accurate.
- SpecifiedOnly returns Unsupported; empty S0 premise lists are not qualification.
- Human text, machine JSON and editor UTF16 positions are described correctly.
- Setup distinguishes protocol smoke from actual VS Code/provider activation.
- Each page has release scope, accessible support labels and stable reference links.
- No deployment/publication claim precedes a real deployed site receipt.
