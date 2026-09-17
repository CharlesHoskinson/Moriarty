# September 17 Preview loan delivery

The active target is the existing source-bound fixed LAM loan: deploy, initialize, accrue, settle. The loan deployment is now finalized at block **908628**, transaction `0074727c89072a920c3fe28bc0be689b0da73a7027fbf0ae734b6a62f743bb293a`. Contract address: `8034dffa6ce124cf34135799969d34831da5ce44fd287a3bead2fa3ce293a341`. Initialize, accrue and settle remain pending.

The fixed example initializes 20,000,000,000 test-asset units to the borrower, accrues principal due 500,000,000 and interest 33,972,602, then pays 533,972,602 units to the lender. Residual principal is 4,500,000,000. It does not demonstrate the newer four-step language lifecycle or mandatory proof-carrying transactions.

The current wallet is restored and synced; `wallet-readiness02.json` records 5,000 tNIGHT and 311,707,234,999,999,999 DUST specks. The lender is a distinct recoverable account derived from the same preserved seed. Private role secrets and contract-store password remain outside the repository.

The current financial caller uses the digest-bound September 17 wallet receipt and opt-in Preview HTTP submission. Preview tests passed 95/95. The build path now selects current WSL tool paths and retains version, source, artifact, resource and receipt checks. Build tests passed 46 Node and 16 Python cases. `build-readiness-02` is the current frozen proposal; `01` is superseded and has never dispatched.

Both Astra and Grok approved the initial caller/build source scopes. The real loan assets built in 23.407 seconds; independent artifact inspection and public preflight passed. The first HTTP deployment attempt failed with 403. A controlled read-only request of comparable size also returned 403, while a small request succeeded. One separately reviewed same-bytes attempt over the officially supported WebSocket endpoint submitted the unchanged deployment successfully. The native payload and identifiers did not change.

[Canonical finality](deployment-finality01.json) records indexer SUCCESS and a canonical finalized block. [State readback](deployment-state01.json) matches the signed deployment's entire initial state byte-for-byte. The WebSocket invocation exited zero with separately retained terminal and containment observations. The original failed HTTP run remains preserved; it did not initialize the contract or persist contract private state. A closed, verified continuation path is being implemented to restore the unchanged constructor state and execute the remaining three calls without deploying again.

The original fee reservation and the conservative second transport reservation remain charged. Full financial comparison, lender payment and independent complete-result review are still pending. No aggregate I2, SP05 or PCD acceptance is claimed.
