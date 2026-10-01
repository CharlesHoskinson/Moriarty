# Signed intent developer walkthrough result

Scope: public package walkthrough, three ready public signature examples and
actual offline consumer trial. Repository observation and experiment observation,
2026-10-01. This is scoped developer evidence, not financial ledger acceptance.

## Delivered files

- `packages/moriarty-beta/SIGNED-INTENT.md`: npm archive installation, separately
  built trusted native binary, exact signing bytes, scheme/framing conventions,
  JSON/review modes, result/exit meanings, numerical and authority limits.
- Package README and getting-started links distinguish existing unsigned local
  simulation from the new native signature consumer.
- `examples/signed-intent/transfer-schnorr-raw`: fee transfer/BIP340/raw.
- `examples/signed-intent/transfer-ecdsa-wallet`: fee transfer/ECDSA/prefix.
- `examples/signed-intent/repay-ecdsa-raw`: partial repayment/ECDSA/raw.

Each directory has `program.mori`, `scenario.json`, `signature.json`,
`expected.json` and `mori.tests.json`. Sources are exact byte strings from the
independent `experiments/midnight-crypto/fixtures/intent-vectors.json`.
Scenarios are actual starterScenario/repaymentScenario. Expected complete ordered
effects and Core/5 post-state are derived from the independent vector economics
and explicit fixture counters/head/replay identities, not from consumer outputs.
Transfer: 1000 value +10 fee, debit1010, owner8990. Partial repayment: debit3000,
principal98000, accrued0, outstanding98000, payer197000.

The existing Rust test-only `intent-fixture` helper generated native signatures
with OsRng keys retained only in process memory. Files contain public statements,
keys and signatures only; no private key was printed or persisted. These keys
have no account authority. The production binary remains readonly.

## Actual checks

Three actual production Rust verifications and three actual beta CLI verifications
returned `SignedPreparedUnqualified`, `signature_valid:true` and
`ledger_accepted:false`. Complete effects/post matched the independent expectations.
Each `mori test` manifest returned TestsPassed.

An actual `npm pack --ignore-scripts` followed by local archive installation in a
fresh external temporary project was exercised. A copy of the native binary was
selected by absolute path; PATH was empty for CLI invocations. All three installed
public examples verified and their effects/post matched `expected.json`. The
project was removed after recording public results. No registry publication,
wallet interaction, Preview transaction or new website publication occurred.

External immutable run records for this task:

- `/home/charl/research/moriarty-signed-intent-2026-10-01/G2-public-fixtures-receipt.json`
- `/home/charl/research/moriarty-signed-intent-2026-10-01/G2-public-pack-receipt.json`
- `/home/charl/research/moriarty-signed-intent-2026-10-01/G2-public-fixtures.mjs`

The pack receipt records `guide_in_tarball:true`. Root's pack test also checks
SIGNED-INTENT.md inclusion. The worker's final message incorrectly reported a
missing allowlist entry; the retained actual receipt supersedes that assertion.

## Attribution and remaining gates

The third Sonnet implementation seat reached its weekly usage cap before its
public walkthrough/examples were completed. Root and the GPT-6.1 Sol G2 worker
finished this bounded scope. Do not attribute this completion to Sonnet or claim
that the unavailable seat supplied a completed implementation/audit.

The walkthrough documents the implemented ASCII-escaped review and JSON behavior;
`--review` and `--json` are mutually exclusive. The native message APIs apply
SHA256 internally to the exact raw or wallet-prefixed message bytes; signing a
frame digest/hex string or adding a prefix twice is not this protocol. Live wallet
compatibility remains unperformed. Platform portability beyond the tested host
is not established by copying this host's binary.

All key/address authority, snapshot/head/history authority, native complete
financial proof and atomic financial ledger acceptance premises remain open.
Source network `preview` is a user claim bound in the signed statement,
not a network observation. Local fixture balances/obligation creditor are not
owner-signed authenticated state. No real Preview financial settlement is shown.
The root's separately measured native numerical proof remains distinct from
these signature/devtools experiments and does not change their ledger status.
