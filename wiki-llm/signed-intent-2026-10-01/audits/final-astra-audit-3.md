# Independent final source and actual-result audit

Verdict: **approve-scoped**.

Candidate: `fd9725e0804c4db0866a3cf733ab300cf2c9179b`.
Base: `2905cb6da0bf0ccfdca28b0f22478e0eda2420e3`.
Manifest: `wiki-llm/signed-intent-2026-10-01/candidate-manifest.json`, SHA256 `73622f068a9ea6ff2af36c8f07f7ae962a90da1f11b67d77f342d2872d6ee89f`.
All 252 listed file digests matched at startup and again after checks. HEAD matched; tracked worktree remained clean.

Requested reviewer route: `gpt-6-astra`, medium effort. The host delegation did not expose a provider-returned model identity or a provider receipt to this agent. This report records the requested route only and does not invent identity attestation.

This was a fresh review of the complete scoped implementation paths and actual results, not approval of only the last diff. I did not read other reviewers' verdict/audit files. I read the author repair contracts and raw red/green defect evidence. AGENTS, the develop skill and current guarded status were loaded first. Status had no pending transactions and retained unresolved financial admission/history/resource evidence.

## Findings and disposition

1. **Blocking/high/medium: none found within the scoped local precursor contract.** The canonical signing implementation, actual API/CLI consumer, atomic local store, display, public examples and bounded numerical proof evidence support scoped approval. This is not an approval of financial or Preview acceptance.
2. **Informational — deployment trust remains explicit.** Native signature verification trusts the installed, explicitly configured executable. The TypeScript response checks bind returned source terms, projection, frame, hash, framing, claims and exit status; they are not a second independent cryptographic verifier. A malicious deployment-configured executable is outside the claimed trust boundary.
3. **Informational — local authority and state remain stipulations.** The registry is not authenticated chain authority, revocation is a local operator action, and the store is one in-memory process without durable recovery. Idempotent retries report the prior commit after revocation without applying a new effect. These limitations are documented and do not contradict the scoped result.
4. **Informational — numerical proof is separate from signed financial acceptance.** The KZG result constrains the finite numerical transfer relation only. It has no signature, source hash, account identity, registry, replay/head, recursive PCD or ledger correspondence constraints. Its local unsafe ephemeral setup is not a production SRS or Preview proof.

## Source review

I traced `auth.ts` from source/action/scenario formation through fixed native commands, bounded process I/O, strict response validation, exact full statement comparison and actual Core/5 preparation. Rust `intent.rs`, `codec.rs`, `lib.rs` and `main.rs` enforce the closed protocol, canonical decimal/hex types, valid curve keys, SEC1 02/03 prefixes, byte framing, owner projection digest, money/round bounds, operation identities and error/status distinctions. Both schemes pass the framed message to the pinned native APIs; the documented SHA256 prehash semantics match the reference/test contracts. Raw and wallet framing are distinct signed metadata with no fallback.

I read the beta frontend, bridge, Source/6 parser/lowering and Core/5 preparation path, public exports, CLI, display, JSON and stdio transports, build/package configuration and relevant tests. Exact source bytes are hashed; selection follows the named action and its referenced domain/asset. Scenario state remains unsigned. Gross debit includes fee, recipient net floor uses the actual recipient credit, allowance/work consumption is complete, and repayment reduces accrual before principal with bounded conservation and overpayment rejection. Unsupported families do not acquire signed or financial execution acceptance.

I reviewed all of `atomic.ts` and its financial/race/fault tests. State is derived internally; authority and digest are rechecked at commit; Core effects are recomputed; the successor binds frame, domain, predecessor and financial effects. Balance, allowance, obligation, replay, work, head and receipt updates are constructed before one root publication. Exact retries require native verification and the same frame, while competing statements, revocation and stale state cannot publish a partial update. All outward results retain local qualification and false/null external acceptance claims.

I reviewed ASCII rendering of statement leaves, amount/scale displays, mismatch/error paths and the supplementary-line repair. The native response parser now normalizes parser-origin errors to `BETA_CRYPTO_RESPONSE`; valid negative crypto results, malformed signature inputs and transport failures remain distinct. The four new regressions exercise the actual child boundary and both CLI formats; malformed caller JSON remains exit 1. Public types expose signing fields without claiming authority, and the packed declaration test compiles a real strict client.

I reviewed `financial_transfer.rs` and its complete test file. Each of 28 public numerical fields is equated to a witness; independent UInt128 decompositions and signed-127 restrictions prevent field-wrap substitutions. Arithmetic and bounded slacks enforce transfer, gross/fee/net bounds, round window, work and allowance conservation. The actual proof test finishes pairing verification and strict transcript EOF, then rejects every changed public field and corrupt, truncated and trailing proof bytes. The fixture adapter's identity/order checks remain host checks, not circuit correspondence.

## Actual evidence

Recorded evidence inspected at this exact manifest:

- `final-beta-tests-3.txt`: 193 passed, zero failures, zero skips, including real native/helper paths and packed signed examples.
- `final-typecheck-3.txt`: successful TypeScript check.
- `final-rust-tests.txt`: 32 passed, one ignored new numerical native-proof test; the older small native proof smoke ran. This is not a new execution by this reviewer.
- `native-run-receipt.json` and `native-run.log`: separately executed bounded native numerical proof, exit 0, k=10, 10,924 proof bytes. Receipt source/lock/fixture pins match this manifest. The test source establishes the 28 altered-public-input checks and three proof-byte negative controls. I did not rerun setup/proving.
- Raw native-response red evidence shows the former exit-1/parser-code misclassification; the retained green result and my execution confirm correction without changing caller-error behavior.

Fresh checks executed by this reviewer with Node `v24.21.0`, the supplied actual Rust binary and `MORIARTY_REQUIRE_NATIVE=1`:

- Response classification, supplementary error escaping, descendant-pipe timeout and packed signed-distribution tests: **8 passed, zero failed/skipped**. The distribution test copied the actual binary, installed the archive outside the checkout, used empty PATH, verified all three public examples in JSON and review forms, compared complete expected effects/post-state, and exercised changed-source, expired, forged and unavailable-binary controls. Log: `final-astra-checks-3.txt` beside this report.
- Atomic consumer and packed public API/declaration tests: **69 passed, zero failed/skipped**. Includes real native ECDSA, full/partial repayment and zero/positive-fee transfer fixtures, races, revocation, all eight write fault stages, chained state, retries and strict installed TypeScript imports. Log: `final-astra-consumer-checks-3.txt`.
- `npm run typecheck`: **exit 0**, log `final-astra-typecheck-3.txt`.

No candidate source edits, commit, network/wallet/Preview actions, SRS generation or ignored native-proof rerun were performed. Build/pack checks regenerated only excluded derived output. Outside compiler/kernel prototypes were not audited or counted.

## Approval limits

Approve the versioned native signing codec and its bounded beta consumer, readable review and three portable public fixtures, stipulated local atomic consumer, and the demonstrated finite numerical proof precursor. Owner-to-account authority, authenticated state/snapshot, compiled-program and financial correspondence, native recursive PCD, ledger compare-and-consume and all financial Preview acceptance gates remain owed. This report does not discharge those gates or authorize a new proving/network campaign.
