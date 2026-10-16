# Cardano Preprod fresh wallet operator — proposed source

## Status

This package authors source only. It does **not** create a wallet, generate a key, extract or execute the Cardano CLI, contact a network, sign anything, build a transaction, or prove that an address can receive or spend funds.

The user requested one new dedicated Cardano Preprod wallet for test ADA. The recorded explicit fallback permits Muse source authorship after the original Opus author timed out. Authorship is not independent approval. A fresh non-author source/effect/security review and a new exact one-run AUTH are required before any native or key operation.

The wallet is **not yet created**. Do not fund any address until a later independently reviewed execution produces the complete public result and a separate audit accepts it.

## Exact scope

The operator proposes to:

1. Validate one exact AUTH file. `AUTH-CANDIDATE.json` is deliberately rejected because its status is `CANDIDATE_NOT_AUTHORIZED`, its operator hash is a placeholder, its staged-binary hash is null, and its linkage is unresolved.
2. Create the fresh public root `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-preprod-fresh-wallet-public-20261016` and the fresh private root `/home/charl/.local/share/moriarty/cardano-preprod/moriarty-utxo-20261016`. Creation fails if either leaf already exists. No retry, repair, reuse, overwrite, or regeneration is attempted.
3. Create or validate private parent directories without reading any pre-existing wallet content. It does not recursively inspect the user's home, old wallets, old keys, seeds, credentials, or provider state.
4. Verify the already acquired public binary archive as data, enumerate its members without `extractall`, require exactly one regular non-link member named `cardano-cli-x86_64-linux` of 102,556,168 bytes, stream that member to a staged public executable, and compare its SHA-256 with the measured value pinned in the actual AUTH.
5. Parse the staged ELF header and program headers as data. The actual AUTH must declare either static linkage with no runtime binds or dynamic linkage with exact read-only runtime files. Dynamic execution must not silently fall back to a host CLI, installation, broad `/usr`, broad `/home`, or existing user state.
6. Record the single attempt in the public attempt ledger and increment `attemptsStarted` to 1 before the first native process spawn.
7. Run at most four bounded, sandboxed CLI calls: `address key-gen`, `address key-hash`, `address build --testnet-magic 1`, and `address info`.
8. Retain the CLI-generated signing key only at the private `payment.skey` path. The operator never opens, reads, hashes, copies, prints, exports, or backs up that file. It checks only presence and mode with `lstat` and normalizes the mode to `0600`.
9. Export public material only after every validation succeeds: `payment.vkey`, `payment.vkey.hash`, `payment.addr`, `address-info.json`, the staged public binary, the attempt ledger, and `RESULT.json`.

No faucet request, login, ledger query, signing, transaction construction, transaction submission, stake key, stake registration, mainnet operation, or use of an old key is authorized.

## Source basis and limits

The pinned public source commit is `fc466f22ca5c03c0e373e2d37053b7f1884c50c9`. The source archive SHA-256 is `20ba6e5711a1f8b8aa916cc87001baa04862c19358b9bf2aaebdb9425dc536e2`. The binary archive is Cardano CLI 11.3.1.0, 24,057,685 bytes, SHA-256 `330d20a904d5f7526a0ba22edec6ba8fe5670978e8a37bf83c27d4b63faeeebe`.

The mirrored `Address/Command.hs` defines `AddressKeyGen`, `AddressKeyHash`, `AddressBuild`, and `AddressInfo`. `Address/Option.hs` mounts those commands below `address` and parses key generation, key hash, build, and info. `Address/Run.hs` shows that Shelley key generation creates a payment pair and writes both files, that build uses `PaymentCredentialByKey (verificationKeyHash vkey)`, and that omitting the stake identifier gives `NoStakeAddress`. `Address/Info/Run.hs` shows the Shelley payment result fields `type`, `era`, `encoding`, `address`, and `base16`.

The mirror does **not** include the imported common-option definitions or the top-level executable dispatch. Therefore the exact leaf spellings for verification/signing key files, output files, payment verifier input, address input, network magic, address key type, and output format are not completely proved by the mirrored files alone. `operator.py` uses the exact proposed spellings stated in `EXECUTION-PROPOSAL.json`. The fresh reviewer must explicitly agree or disagree with those spellings. If a spelling or default is wrong, the native call fails, the attempt is spent, and no retry is permitted.

This is not a reproducible-build attestation. Public source/ELF availability and archive hashes establish only the stated public integrity pins.

## Address and validation boundary

The proposed address is an enterprise, key-controlled Shelley testnet address: payment credential only, no stake credential, no stake key, and no registration. Address info can show a testnet Shelley payment address, but testnet encoding alone cannot distinguish Preview from Preprod. “Cardano Preprod” is the declared configuration target of network magic 1, not a ledger observation.

`address key-gen` is the source-supported operation that generates the corresponding signing and verification keys together. `address key-hash` hashes the public verification key, `address build` constructs the address from that payment credential with no stake reference, and `address info` parses and classifies the resulting address.

The operator additionally applies a disclosed fail-closed public invariant: address-info `base16` must decode to 29 bytes, begin with header byte `0x60`, and end with the 28-byte CLI payment key hash. The fresh reviewer must specifically agree or disagree with this invariant. Even if every check passes, the result proves only that the retained private key file was generated in the same CLI key-generation operation as the exported verification key and that the public address was derived and parsed consistently. It does not prove live ownership or spendability before a future separately authorized signature test.

## Key retention warning

The retained secret is a raw CLI `PaymentSigningKey` text envelope. It is **not** an HD mnemonic and there is no mnemonic backup in this design. Whoever controls the private retained file controls the future ability to sign for the address, subject to a separate authorization and signature test.

Protect the private retained file offline. Never send it, upload it, paste it, commit it, or ask the user to reveal it. Only the public `addr_test...` address may be shared in chat or GitHub after complete validation and fresh audit. This wallet is for test ADA only.

Root must never read, stat, hash, walk, list, back up, or upload the private root or any private key/log beneath it.

## Failure behaviour

Any native error, timeout, hash change, unexpected file, invalid output, linkage mismatch, or validation failure leaves the attempt failed and spent. Partial private files may remain for the operator's private boundary, but no funding address is published from partial logs. Public output contains only fixed categories, exit codes, counters, and public hashes. Raw native stdout and stderr remain private.

CPU limits are per-process and are not falsely represented as transitive across every descendant. Timeout handling kills the outer process group on a best-effort basis; it cannot guarantee death of an uninterruptible process. Those limits are stated rather than hidden.

## Required next boundary

Before any execution, a fresh non-author reviewer must review all five materialized files and agree or disagree with the literal clauses in `DESIGN-POSITIONS.json`. Root then fills the exact operator SHA-256, the data-measured staged-binary SHA-256, and the data-inspected ELF linkage/runtime binds into a separate one-run AUTH. Source readiness alone grants no native authority.

Compiled C2 wrappers, genuine hash/signature qualification, settlement, qualified cost, all 20 UTxO families, ACTUS/DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates, and real Preprod transactions remain separate and open.
