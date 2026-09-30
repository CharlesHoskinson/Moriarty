# Website feedback — Moriarty beta tutorial site (proposal)

Based on my S2 first run (loan repayment). Host as a Git repo + GitHub Pages; suggested stack: mdBook or Docusaurus/Starlight with a CI job that **executes every shown command** against the pinned tarball and diffs the captured output, so pages cannot drift.

## Landing promise
"Write exact financial intent, see exactly what money would move, and learn precisely what is not yet proven." One sentence of limits under the hero: results are **PreparedUnqualified** — no signing, authentication, proof or ledger.

## Page order
1. Landing + limits badge (Local S0 / SpecifiedOnly / Open)
2. Prerequisites & install (Node ≥24; `npm install ./tarball`, `./node_modules/.bin/mori`; note `npx mori`; `init` refuses existing dirs)
3. First five minutes: `init`, `check --json`, `simulate`, `test .` — show the real outputs
4. First transfer (the starter, line by line)
5. First repayment (**missing today**): `obligation`, `repay(...)`, scenario `obligation{}`; partial vs full with worked atoms
6. Asset/identity/unit model: `USD` local name vs `id: "USDCanonical"`; scale, atoms, `Qty<USD>`; what `512.5 USD` does
7. Source intent vs untrusted fixture: what each file may and cannot do; scenario cannot override intent
8. First rejection: overpay 512.51 → `S0_EFFECT_RANGE`; expiry round 11 → `S0_INTENT_SCOPE`; show null effects and exit code 1
9. Bounded project tests: manifest schema, exact effects/post, `code`, `CoreRejected`; separate good/invalid dirs; negative control
10. Editor & AI setup (LSP/MCP minimal handshake; adapters are inert)
11. Support matrix (Local S0: transfer, repay; SpecifiedOnly: AMM, lending lifecycle, …; Open)
12. Proof/authentication/ledger limits: four premises, four unverified bindings — name them
13. API/reference: CLI, status & code catalogue, scenario JSON schema, test manifest schema, JS API, MCP tools
14. Next steps / feedback

## Content from my first run to include
- Accrual-first worked example: 500.00 + 12.50 owed; pay 100.00 → accrued 0, principal 412.50; pay 5.00 → accrued 7.50; pay 512.50 → Settled. Include the effect vector (Debit, Credit, SetObligation, UseAllowance, UseReplay, AdvanceHead) and that the replay key is `["Midnight","Alice","rp1"]`.
- A table of repay constraints: signer = payer, fee_cap = 0, net_floor = 0, gross_cap ≥ amount, amount ≤ outstanding, outstanding = principal + accrued, status Outstanding, creditor ≠ signer, window `valid` inclusive (0 and 10 accepted, 11 rejected).
- Error catalogue: `BETA_SIGNER`, `BETA_DECIMAL_SCALAR`, `BETA_PRECISION`, `BETA_SCENARIO_ACCOUNTING`, `BETA_JSON_DUPLICATE`, `S0_INTENT_SCOPE` (expiry/caps/fees), `S0_EFFECT_RANGE` (overpay/insufficient), `BETA_PROFILE_UNSUPPORTED`.
- Exit codes: 0 success, 1 for any rejection (including correct ones).
- How to read `simulate` JSON: jump to `result.candidate.effects`/`candidatePost`, or `result.rejection`.

## What to omit from getting started
Horizon families (AMM, derivatives, bridge…), wiki full-language grammar, VS Code packaging, Neovim, AI adapters, MCP, JS API, `fmt` hash implications beyond one sentence, the UTF-8/UTF-16 origin-map details. Link them after the first rejection.

## Tutorial acceptance checklist
- [ ] A new user with Node 24 runs every command copy-paste, no repo access
- [ ] Every code block is CI-executed; outputs diffed
- [ ] User writes a repay program without reading `dist/`
- [ ] Shows both a Core rejection and an authoring rejection with codes
- [ ] Explains accrual-first arithmetic with atoms
- [ ] States source-vs-fixture separation and "PreparedUnqualified" meaning on the first page it appears
- [ ] Lists the four premises and four bindings
- [ ] Demonstrates SpecifiedOnly → `BETA_PROFILE_UNSUPPORTED`
- [ ] Shows `mori test` on separate good/invalid directories plus a failing-control
- [ ] Support matrix matches `check --json` coverage fields

## Minority preferences to keep visible
A `--template repay` for `init`; test-failure diffs; dedicated expiry code; closed complete fixtures kept as the default even if overrides are added.
