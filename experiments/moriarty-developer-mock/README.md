# Moriarty developer mock

A local browser workspace for inspecting ACTUS reference events, exact arithmetic
illustrations and a simulated proof-carrying transaction workflow.

From this directory, with Node 24+ and TypeScript installed:

```sh
npm ci
npm run build
npm test
npm start
```

Open http://127.0.0.1:4173. The server binds only to loopback. Set
`MORIARTY_MOCK_PORT` to use another port. No runtime npm package is required.
The UI has no external requests, wallet, chain, prover, analytics or CDN.

## What works

- Imported LAM, business-day and option reference events with all present fields,
  exact decimal strings, source commits and hashes.
- Editable first-period loan interest as an exact rational, displayed separately
  from the unchanged imported schedule and its unresolved numeric profile.
- Exact integer swap output with floor rounding and minimum-output checks.
- Scripted mandate, partial-fill and two-parent examples with missing behavior
  stated in the interface. Partial fill is one revision, not an order executor.
- Five explicit demo stages; proof failure versus ledger conflict; repeated
  predecessor rejection; reset controls and JSON export retaining mock markers.
- Four proposed mandatory claim descriptions, each with unavailable real evidence.
- Desktop/mobile layout, keyboard controls and browser-local configuration restore.

This is not a DSL parser, general interpreter, contract checker, PCD implementation
or ACTUS-conformance engine. Source previews are illustrative syntax. Imported
expected results are not generated results. Due is not paid. All workflow
evidence is `SimulatedEvidence`; real verification returns `unavailable`.
Midnight-native Halo2/recursion is the first planned backend, not connected here.

Configuration persists locally; prepared evidence never survives reload.
The simulated consumption ledger lives only in the tab and resets on reload or
an explicit **Reset demo ledger**. Editing inputs preserves that tab's consumed
predecessors. This is not durable submission/crash recovery.

## Validation

`npm test` covers numerical rejection, stale evidence, distinct proof/ledger
failures and deduplication. `npm run build` checks TypeScript. With the server
running and Python Playwright/Chromium installed, run:

```sh
python tests/browser_check.py
```

The browser smoke checks desktop/mobile, imported fields, export, evidence
invalidation, consumption preservation, missing proof, stale predecessor and no
external requests. Results and screenshots are saved in
[evidence](../../evidence/moriarty-developer-mock-2026-09-06/).
The [revised plan](../../docs/research/2026-09-06-pcd-report-integration.md)
records the remaining semantics, certificate and real-proof gates.
