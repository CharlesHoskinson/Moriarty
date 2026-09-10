# Existing-wallet local financial launch

`launch-local.mjs` composes the reviewed integration with the installed wallet SDK.
Importing it performs no wallet operation. Actual launch requires separate source,
build-result and execution admission plus outer process containment.

The command is `node ledger/launch-local.mjs --run --plan /absolute/private/plan.json --sha256 <exact-plan-sha256>`, from this package directory.
The plan must be a private, owner-only regular file. An invalid invocation prints
only a fixed diagnostic. It never accepts a seed or role secret on the command line.

The closed `moriarty.local-financial-launch/1` plan contains:

- `kind`: `loan` or `swap`.
- `build`: exact `receiptPath`, `receiptSha256` and `sourceManifestHash` accepted by the proven-asset loader.
- `networkConfig`: `networkId: "undeployed"`, `node`, `indexer`, `indexerWS`, `proofServer`. Only literal loopback IPv4/IPv6 addresses and HTTP/WS are admitted.
- `wallet`: existing `seedFile`, `stateDirectory`, and expected public Bech32 `expectedAddress`.
- `roles`: distinct public hex `firstAddress`, `secondAddress`, and private `secretsFile`. That existing JSON file contains exactly `firstSecret` and `secondSecret`, each 32 bytes in hex.
- `privateState`: an existing private `directory` for contract state and a private `passwordFile` containing at least 16 characters.
- `networkTag`: 32 bytes in hex; `expectedProtocolVersion`: a nonnegative safe integer.
- `limits`: `allocationId`, an absolute `deadlineMs` no more than one hour ahead, `submissions: 4`, canonical decimal `dustFee`, and `grossByLogicalAsset` (`USD_TEST_ASSET` for loan; `ASSET_A` and `ASSET_B` for swap).
- `outputDirectory`: a fresh, nonexistent absolute directory beneath an existing private directory. It must be separate from wallet and contract state.

All private inputs must already exist with owner-only file permissions and no
symlink ancestors. The wallet state directory must contain all three files at
`.midnight-wallet-state/undeployed/{shielded,unshielded,dust}.json`, using the existing
version-1 envelope and SDK serialized string. Missing or incompatible snapshots
stop the run. There is no seed generation, fresh-sync fallback, DUST registration,
automatic funding, retry, or overwrite of the retained wallet snapshots.

The implementation reuses the existing account-0/index-0 keys, checks the derived
public address and restored unshielded identity before starting synchronization,
and converts the existing Night signing key with the actual ledger
`signingKeyFromBip340` API. It checks the resulting verifying key. It does not
generate a deployment signing identity.

Each public provider event is projected to a closed record and written exclusively
with file and directory fsync. Each checked stage receives its own durable file
before the integration can continue. At the actual wallet submit boundary, after
the existing provider reserves the debit, `public-transactions/` receives the
canonical finalized native transaction bytes plus a metadata file with byte count,
SHA-256, native transaction hash and identifiers. Both writes must succeed before
the wallet submit method is called. A later submission failure keeps those files;
an existing capture blocks duplicate submission rather than overwriting evidence.
These are public chain payload bytes, never private recipes or witness objects.

`integration-result.json` retains the complete reviewed public integration result
on success or failure, including driver receipts, cleanup disposition, all stage
comparisons and `financialComparison.finish()` when present. This writer receives
only the real reviewed integration producer in the CLI; it is not an arbitrary
SDK-object sanitizer. Error objects themselves are never retained.

Private SDK results and exception messages
are not copied to those records. The original wallet snapshots remain unchanged;
they are not claimed to contain the post-run pending transaction state. Durable
submission identifiers and reservations must be reconciled before any later run.

The CLI has a terminal timer six seconds after the operation deadline and exits
after recording its result. This does not replace outer containment or establish
that an external transaction was cancelled. Production integration currently
reports incomplete containment even if all four financial comparisons pass.
The launcher preserves that limitation and never claims financial, network or
proof acceptance.

Source-only verification: `node --test ledger/launch-local.test.mjs`. This checks
plan rejection, private-file safety, event projection, strict state envelopes,
CLI argument redaction, direct runtime hashes and actual SDK export availability.
It does not restore or start a wallet, generate keys/proofs, or contact Docker.
Runtime pins cover the named package metadata and selected entry bytes; they are
not a full transitive supply-chain attestation.

The pinned keystore's `getSecretKey()` implementation returns
`Buffer.from(secretKey)` where the HD-derived key is a `Uint8Array`. That operation
copies the bytes; clearing the temporary buffer after `signingKeyFromBip340`
does not zero the live keystore. The native payload tests replay the historical
public fixture through an inert submitter and inject storage failure. They make
no wallet or network call and generate no identity or proof.

At source implementation time, the historical dedicated local wallet was located
at `/home/charl/.local/share/moriarty/test-wallets/hello-world-dedicated-v2`, with its
public identity in `evidence/moriarty-midnight-network-2026-09-07/local-receipts.json`.
Only names and permissions of the private artifacts were inspected. The three
child files were mode `0644` beneath a `0700` ancestor. The parent subsequently
tightened all three to `0600` without reading their contents or changing size/mtime;
see `deliverables/sp05-financial-integration-2026-09-09/local-readiness-01/snapshot-permissions.json`.
Existing role-secret and password files can be prepared under the later bounded
execution admission. The reviewed public participant selection, a finite admitted plan, current
local service readiness, successful strict restoration and available DUST remain
required inputs/checks. No private artifact was read or changed during this task.
