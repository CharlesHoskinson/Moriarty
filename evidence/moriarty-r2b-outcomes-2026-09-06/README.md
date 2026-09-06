# R2b outcome-intent evidence

Status: S3 local experiment. Developers can sign bounded outcomes, then select
either registered pool route, or settle existing loan dues through the same
independent authority checker. Open `/intents` in the
[workspace](../../experiments/moriarty-developer-mock/README.md).

The signature binds principal, domain, structured assets, aggregate gross debit
budgets, recipients, fee caps, net goals, validity, nonce, agreement programs and
four mandatory claims. It does not bind a concrete route. The runtime recomputes
one registered action, checks complete shared balances and effects, then commits
financial state and nonce together only for local simulation. Neither supplied
after-state nor a signature alone can authorize an arbitrary counterparty debit.

| Check | Result and scope |
| --- | --- |
| TypeScript build | Exit 0; [output](build.stdout.txt). Build precedes compiled-module tests. |
| Node suite | 67 passed, zero failed: prior 47 plus 9 outcome-authority and 11 runtime tests; [output](node-tests.stdout.txt). |
| Outcome browser | Exit 0; [output](browser.stdout.txt). Same signature across pools, stricter gross cap with fresh nonce, expiry, replay, loan dues, missing proofs, export and mobile layout. Deterministic digest/verify gates check proposal, reset and simulation UI locking. |
| Existing exact-plan browser | Exit 0; [output](r2-browser.stdout.txt). Existing loan/swap authoring and navigation remain usable. |
| Visual inspection | [Desktop](desktop-intents.png), [mobile](mobile-intents.png), [receipt](visual-check.json). Signed pool B preview with unavailable claims; no horizontal overflow or page errors. |
| Held-out provenance | [Four pinned source blobs](heldouts-source-check.json) and three local document hashes match the [held-out manifest](../moriarty-r2b-heldouts-2026-09-06/manifest.json). Source-only, no financial execution. |

The [verification manifest](verification.json) binds the source, configuration,
tests and receipts. Public example exports contain local generated public keys
and signatures; no private key is exported. Test outputs from earlier failing
and passing runs are preserved without rewriting. In particular,
`runtime-first.txt` records Node's unsupported TypeScript parameter-property
strip mode; runtime tests now use freshly compiled modules. The first financial
run rejected an identifier-shaped demo nonce; the final explicit nonce profile
is canonical UInt128. These setup failures are not passing financial evidence.

Independent reviewer `r2b_review` inspected the authority/runtime/UI and ran the
then-current 16 focused Node tests. Two UI findings were reproduced and fixed:
missing signing status, and a pending pool B proposal leaving pool A executable
and later pairing its receipt with a different displayed plan. The final browser
test gates proposal hashing to check that the old plan disappears and execution
is disabled. A bounded read-only follow-up confirmed both findings resolved.
This is source/test review, not an independent security audit or formal proof.

Parent review also corrected non-string proposal hash coercion; the failing
[runtime regression](runtime-review-red.txt) and [passing result](runtime-review-green.txt)
are retained. Added controls exercise caller mutation during asynchronous checks,
clock changes before commit, returned snapshot/receipt isolation and wrong trust.
Earlier authority review added readable signing fields, unique agreement IDs
and required goal balance cells; its original failure and passing receipts remain.
UI follow-up invalidates older operation results, locks reset/simulation, clears
stale receipts and handles malformed exports visibly.

The profile has one registered financial action and at most four explicit fees;
UInt128 values, 16 effects, 32 balance cells and 128 consumed nonces with no eviction.
Two pools share trader balances. A nonce is keyed by domain/principal/nonce, not
intent hash. Validity is inclusive at notBefore and exclusive at expiresAt.
Unused one-shot authority is extinguished on commit; there is no partial or
pending success. The logical clock and balances are synthetic. Changing examples
or reloading resets the world and nonce history. This is not durable replay
protection, wallet authority, custody or distributed ledger consumption.

Four real claims remain unavailable: ContractInvariant, IntentRefinement,
TransitionValidity and HistoryCompliance. No native proof, contract theorem,
compiler correspondence or ledger acceptance was produced. ACTUS NAM19, Maple
refinance and Huma pending redemption are frozen needs-extension requirements,
not implemented features. The full ACTUS/DeFi matrices remain open.
The next deliverable is the [bounded R3 native experiment](../../experiments/moriarty-native-ivc-r3/README.md),
which is still specified-only; no native proving campaign ran in R2b.
