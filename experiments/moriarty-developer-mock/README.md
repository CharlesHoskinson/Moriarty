# Moriarty developer mock

A local developer workspace with imported references and a simulated proof flow
at `/`, a bounded executable JSON language slice at `/language`, and signed
outcomes with independent plan choice at `/intents`.

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

At `/language`, developers can edit an `Actus.LAM.FirstPeriod` or
`Exchange.ConstantProduct` JSON package, elaborate a generic Core program,
evaluate it, inspect state/effects and apply a separate editable intent policy.
The loan creates dues before settling them with matching cash transfers; the
pool computes a swap and closes a finite epoch. Both use the same evaluator.
All integer inputs and intermediates are checked UInt128 with explicit floor
rounding. Programs, expressions, fields, effects and lifecycle work have finite
limits; unknown syntax and hidden effects reject.

Signing uses genuine local Ed25519 keys and SHA-256 canonical claim commitments.
This signs one exact plan, including its predecessor and policy. Edge authority
caps count gross outgoing movements; minimum credits count net delivery after
fees. This is local key possession and policy checking, not a wallet mandate,
contract theorem or recursive proof. Four mandatory claims remain unavailable.
**Advance local simulation** is separate from ledger acceptance. Edits clear old
results and invalidate pending signatures. Private keys stay in memory and are
not exported.

The original `/` workspace retains:

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

The executable authoring form is a restricted package DSL, not final textual
syntax or full ACTUS conformance. The loan covers one period and uses micro-USD:
default interest is 33.972602 USD versus the reference 33.972602739726… USD.
The remaining 4,500 USD notional is not discharged when this episode closes.
The AMM fee is retained inside the full input reserve movement, not an extra
debit. Synthetic account balances do not establish custody.

Source previews in the original workspace remain illustrative, and imported
expected results remain separate from calculated results. Its workflow evidence
is `SimulatedEvidence`; real verification in every mode returns `unavailable`.
Midnight-native recursion is the first planned backend and is not connected.
The bounded outcome profile below implements local expiry/nonces and aggregate
authority. Residual capabilities, durable consumption and compiler/ledger
correspondence remain open under the [intents amendment](../../docs/research/2026-09-06-intents-report-integration.md).

Configuration persists locally; prepared evidence never survives reload.
The simulated consumption ledger lives only in the tab and resets on reload or
an explicit **Reset demo ledger**. Editing inputs preserves that tab's consumed
predecessors. This is not durable submission/crash recovery.

## Outcome intents

At `/intents`:

1. Edit the outcome: maximum gross spending, allowed recipients, minimum net
   delivery, fees, asset identities, validity, nonce and permitted agreements.
2. Read the signing summary, then sign with a generated in-memory Ed25519 key.
3. Propose pool A or B. The signed intent stays the same while the concrete plan
   and its hash change. The loan example instead proposes full existing-due settlement.
4. Preview or advance local simulation. The runtime recomputes the action from
   current registered state; independent authority and accounting checks must pass.
   Successful simulation consumes the nonce. Replay, stale state and over-spend reject.
5. Inspect the receipt or export public objects. Real acceptance reports the four
   required proofs as unavailable and cannot commit a simulation.

The profile has exactly one agreement action plus at most four fees, UInt128
values, at most 16 effects and 32 balance cells, and a 128-entry nonce table that
never evicts consumed entries. Two pools share trader balances. Nonces bind
network/deployment/principal, so re-signing an edited intent cannot reuse a nonce.
Changing examples or reloading creates a new demo world and resets nonce history.
Logical time is a demo input. Pending redemption, refinancing, capitalization,
arbitrary calls, multi-step routing, private composition and durable ledger
consumption are outside this profile. A simulated-complete receipt is not PCD.

See the [R2b evidence](../../evidence/moriarty-r2b-outcomes-2026-09-06/README.md)
and [held-out cases](../../evidence/moriarty-r2b-heldouts-2026-09-06/README.md).

## Validation

`npm test` covers numerical rejection, stale evidence, distinct proof/ledger
failures and deduplication. `npm run build` checks TypeScript. With the server
running and Python Playwright/Chromium installed, run:

```sh
python tests/browser_check.py
MORIARTY_R2_URL=http://127.0.0.1:4173 python tests/language_browser_check.py
MORIARTY_OUTCOME_URL=http://127.0.0.1:4173 python tests/outcome_browser_check.py
```

The browser smoke checks desktop/mobile, imported fields, export, evidence
invalidation, consumption preservation, missing proof, stale predecessor and no
external requests. Results and screenshots are saved in
[evidence](../../evidence/moriarty-developer-mock-2026-09-06/).
The [revised plan](../../docs/research/2026-09-06-pcd-report-integration.md)
records the remaining semantics, certificate and real-proof gates.
The [R2 evidence](../../evidence/moriarty-r2-language-2026-09-06/README.md)
records the source-scoped executable-slice checks and independent review.
