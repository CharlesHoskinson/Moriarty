# Fresh independent actual-result security review — Cardano Preprod fresh wallet

## Verdict

**walletCreatedPrerequisiteClosed = true; walletFundingAddressReady = true (narrow test-ADA funding report only).** The top finding is that the actual public export is fully valid: the NEW dedicated enterprise payment wallet was created once by the reviewed official CLI and its public address may now be reported to the human for Preprod test ADA funding.

**Exact funding address:** `addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc`

**Public result digest:** public RESULT.json 2433 B, SHA-256 `eb85770af187c927f91cff4a5b5341ae4fcd376f34bce02e69d4b2cd20611dae`, status `WALLET_CREATED_PUBLIC_EXPORT_COMPLETE`, walletCreated true.

This closes only the wallet-created prerequisite. It is not funded status, live ledger ownership, a signature proof, mainnet advice, or Preprod settlement confirmation. authorityProposalReady = false; no new native authority is proposed or granted, and actual ledger Preprod acceptance is none.

## Findings

Actual run: root RESULT exit 0, wall 0.9268561 s, waited children CPU 0.74048 s (not transitive), 1 attempt, 0 retries, input pins unchanged, exactly four CLI calls (address-key-gen, address-key-hash, address-build --testnet-magic 1, address-info) all exit 0 with no timeouts. AUTH (28 keys, status AUTHORIZED_ONE_RUN, operator 7944d08f...) is spent: AUTH-SPENT spent true, attemptsConsumed 1, noNewAuthority true; the frozen AUTH text is historical and confers no future authority. Root adoption uses the R2 candidate copy (9 files, candidate 442c018f...) with only the AUTH status flip; generic nativeRoundtrip/ordinarySelftest boilerplate was expressly not adopted — scope here is the scoped wallet actual only.

Public consistency: payment.addr (64 B, 84897c9f...), payment.vkey (190 B, 6a257005..., PaymentVerificationKeyShelley_ed25519, cborHex 5820 + 32-byte key), payment.vkey.hash (57 B, 92fffc33..., hash 96a9d23c533e33a10611cdd9e88e1b8fba539b4b98346c578bb78ddb), address-info.json (231 B, 71378a4a..., base16 0x60 header + the same 28-byte hash, 29 bytes, type payment/era shelley/encoding bech32, address identical to payment.addr), RESULT.json and attempt-ledger.json all match root pins and the mirror index. Haskell Address/Run.hs generateAndWriteKeyFiles generates and writes the one pair, so CLI keypair correspondence is established by source; no separate signature is required before funding this test address, and live ownership/spendability remains a separate later predicate.

Secret boundary: signingKeyRetainedPrivate/privateRootMode0700/signingKeyMode0600 true and signingKeyExported/signingKeyHashedByOperator false are operator-reported booleans backed by operator.py lstat/chmod-only handling (never opens payment.skey); this review did not and must not inspect the secret. Root stdout is 113 B sanitized JSON only, stderr 0 B, raw native outputs remain private, and no skey hash, seed, mnemonic, or credential appears publicly. The retained key is a CLI text envelope, not an HD mnemonic; the owner must protect the private file offline and share only the public address.

## Scope, limitations, first blocked boundary

Read-only mirror review; no execution of any kind. Target Preprod rests on explicit configured --testnet-magic 1 (testnet encoding alone cannot distinguish Preprod from Preview) with no ledger observed. Qualifications retained: official-binary CSPRNG dependency, no reproducible-build proof, D-state/pipe-bound and non-transitive-CPU residuals, staged binary not in this mirror. Root PROCESS 'running' is the immutable at-spawn record; TERMINAL exit 0 is the authoritative captured completion, not a current liveness claim. Root RESULT actualResultIndependentlyReviewed false was captured before this review and is historical, not a failure.

First blocked next step: human funding of the exact address above, then public ledger observation, then any sign/transaction under separate review plus compiled C2 wrapper genuine hash/FX1/qualified ledger cost. All 20 UTxO families, ACTUS/DeFi/general obligations, recursion, private handoff, bounded multi-parent PCD, universal certificates, and real Preprod transactions remain open. No compatibility percentage is stated.

## Independence

Fresh non-author Muse Wonder HIGH review in a new context under the indexed explicit user Gemini/Muse fallback confirmation after the original Opus source author timed out. This reviewer did not author the candidate, operator, source review, or execution result and did not execute anything. Same-family Muse fallback is disclosed; no provider identity, authority, approval, or cross-provider independence is claimed. Effort requested HIGH; provider identity is not independently measured.
