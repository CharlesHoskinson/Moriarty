# Claim references

## OWS01

Normative core, optional access/isolation profiles and non-normative reference documentation are distinct. Conformance must name supported profiles; hosted custody and service payment flows are outside core scope.

- [docs/00-specification.md:7–85](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/00-specification.md#L7-L85) — SHA256 `c3a55e1ad3755ae582a3ac96d6e25789c65e7584ef6c90258321adfc68f1f73e`

## OWS02

Owner credentials bypass policies; API tokens require scoped wallet lookup and AND evaluation of attached policies before decryption. This is code-path authority, not cryptographic attenuation of the underlying wallet key.

- [docs/03-policy-engine.md:5–32](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/03-policy-engine.md#L5-L32) — SHA256 `8147877bbc8d177c94ef803f71a8ad6b4c703bacfe07169c5b4bbc141de99ced`
- [ows/crates/ows-lib/src/key_ops.rs:289–351](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/key_ops.rs#L289-L351) — SHA256 `7556d54c66b7633449dc44103034964beaa6a09e44fc0b3bbd6a9d67327cd18c`
- [ows/crates/ows-lib/src/ops.rs:576–607](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/ops.rs#L576-L607) — SHA256 `f2747765e28e011d6ca71df35102c46d4c893560325e381ebcf03662827c5105`

## OWS03

API key creation encrypts each complete wallet secret under HKDF(token); token plus encrypted file can recover secret outside OWS, as explicitly acknowledged in the threat table.

- [docs/03-policy-engine.md:30–107](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/03-policy-engine.md#L30-L107) — SHA256 `8147877bbc8d177c94ef803f71a8ad6b4c703bacfe07169c5b4bbc141de99ced`
- [ows/crates/ows-lib/src/key_ops.rs:15–70](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/key_ops.rs#L15-L70) — SHA256 `7556d54c66b7633449dc44103034964beaa6a09e44fc0b3bbd6a9d67327cd18c`
- [ows/crates/ows-lib/src/key_ops.rs:402–422](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/key_ops.rs#L402-L422) — SHA256 `7556d54c66b7633449dc44103034964beaa6a09e44fc0b3bbd6a9d67327cd18c`

## OWS04

File deletion revokes the ordinary lookup path but cannot erase an adversary-retained ciphertext/token or recovered secret. Owner passphrase changes do not invalidate independently encrypted API copies.

- [docs/03-policy-engine.md:94–107](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/03-policy-engine.md#L94-L107) — SHA256 `8147877bbc8d177c94ef803f71a8ad6b4c703bacfe07169c5b4bbc141de99ced`
- [docs/quickstart.md:190–198](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/quickstart.md#L190-L198) — SHA256 `ccc90c64f330bcb3d998ef9597e1f546aab29bdae8c2ec016a89d537e121e24b`

## OWS05

Current isolation is same-process hardening; the future enclave described is a per-request child process, not a specified attested hardware TEE. In-process memory compromise is explicitly not fully mitigated.

- [docs/05-key-isolation.md:57–113](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/05-key-isolation.md#L57-L113) — SHA256 `bbba700f549e9d5ee0b9f921c88b3830a65e0d2e6bd343337de4d259c585c548`

## OWS06

Quickstart labels the signer an isolated process although detailed isolation describes current same-process operation and future subprocess. Treat the diagram as inconsistent with the detailed current-state description.

- [docs/quickstart.md:200–219](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/quickstart.md#L200-L219) — SHA256 `ccc90c64f330bcb3d998ef9597e1f546aab29bdae8c2ec016a89d537e121e24b`
- [docs/05-key-isolation.md:81–113](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/05-key-isolation.md#L81-L113) — SHA256 `bbba700f549e9d5ee0b9f921c88b3830a65e0d2e6bd343337de4d259c585c548`

## OWS07

Built-in rules cover allowed chains, expiry and typed-data verifying contracts. Spending, recipients, aggregate limits and simulation require custom executable policy.

- [docs/03-policy-engine.md:109–183](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/03-policy-engine.md#L109-L183) — SHA256 `8147877bbc8d177c94ef803f71a8ad6b4c703bacfe07169c5b4bbc141de99ced`
- [ows/crates/ows-lib/src/policy_engine.rs:39–48](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/policy_engine.rs#L39-L48) — SHA256 `f1adbcea4bf7ba4d828426c79fd833e7c92a272ca44a3a842ec6ba0502fc1258`

## OWS08

The typed-data contract allowlist does not restrict non-typed-data requests; source returns allow when typed_data is absent. Empty policy lists also allow. Neither is a universal economic policy.

- [docs/03-policy-engine.md:129–144](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/03-policy-engine.md#L129-L144) — SHA256 `8147877bbc8d177c94ef803f71a8ad6b4c703bacfe07169c5b4bbc141de99ced`
- [ows/crates/ows-lib/src/policy_engine.rs:6–15](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/policy_engine.rs#L6-L15) — SHA256 `f1adbcea4bf7ba4d828426c79fd833e7c92a272ca44a3a842ec6ba0502fc1258`
- [ows/crates/ows-lib/src/policy_engine.rs:79–110](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/policy_engine.rs#L79-L110) — SHA256 `f1adbcea4bf7ba4d828426c79fd833e7c92a272ca44a3a842ec6ba0502fc1258`

## OWS09

Raw hash signing applies no prefix or domain transformation. The authorization helper signs an EIP-7702 tuple with selected chain ID; raw signHash can sign a caller-precomputed wildcard tuple. Chain selection alone does not establish semantic domain binding of an opaque digest.

- [docs/02-signing-interface.md:119–146](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/02-signing-interface.md#L119-L146) — SHA256 `46c2b53219f5b37d28edf54f4a153fd4a689b5936feac22e4c683ed192c213e6`
- [ows/crates/ows-lib/src/ops.rs:615–666](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/ops.rs#L615-L666) — SHA256 `f2747765e28e011d6ca71df35102c46d4c893560325e381ebcf03662827c5105`
- [ows/crates/ows-lib/src/key_ops.rs:163–200](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/key_ops.rs#L163-L200) — SHA256 `7556d54c66b7633449dc44103034964beaa6a09e44fc0b3bbd6a9d67327cd18c`

## OWS10

The current policy context uses explicit request_type; only Cardano fills transaction.effects. Other chains including EVM carry raw bytes with empty effects; typed data has a separate optional payload.

- [docs/03-policy-engine.md:242–309](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/03-policy-engine.md#L242-L309) — SHA256 `8147877bbc8d177c94ef803f71a8ad6b4c703bacfe07169c5b4bbc141de99ced`
- [ows/crates/ows-signer/src/traits.rs:109–125](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-signer/src/traits.rs#L109-L125) — SHA256 `9c096ec8c580a6fe167f717deb5a8fb52e71226fc15518e525ca6976683181ee`
- [ows/crates/ows-lib/src/key_ops.rs:127–184](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/key_ops.rs#L127-L184) — SHA256 `7556d54c66b7633449dc44103034964beaa6a09e44fc0b3bbd6a9d67327cd18c`

## OWS11

The documented optional transaction-field migration can change a defensive custom policy from deny to allow; malformed-output fail-closed behavior does not prevent a valid allow computed from an incomplete predicate.

- [docs/03-policy-engine.md:271–291](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/03-policy-engine.md#L271-L291) — SHA256 `8147877bbc8d177c94ef803f71a8ad6b4c703bacfe07169c5b4bbc141de99ced`
- [CHANGELOG.md:28–44](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/CHANGELOG.md#L28-L44) — SHA256 `b5cee004d389a7c6a4a9c3fae5816e8d3d04424df26e72bd8056eced536f99e1`

## OWS12

The implementation guide calls spending.daily_total cumulative value signed today, but the inspected production context builder uses noop_spending_context with literal zero. Do not rely on this field for cumulative budgets.

- [docs/policy-engine-implementation.md:102–119](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/policy-engine-implementation.md#L102-L119) — SHA256 `6a05e97f1daf6c22f7b53eb66e2b7e89580cda4016949a286e5a4cfca52f4df9`
- [ows/crates/ows-lib/src/key_ops.rs:327–343](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/key_ops.rs#L327-L343) — SHA256 `7556d54c66b7633449dc44103034964beaa6a09e44fc0b3bbd6a9d67327cd18c`
- [ows/crates/ows-lib/src/key_ops.rs:367–372](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/key_ops.rs#L367-L372) — SHA256 `7556d54c66b7633449dc44103034964beaa6a09e44fc0b3bbd6a9d67327cd18c`

## OWS13

Executable policies are arbitrary local processes; unsuccessful/malformed results deny and the documented timeout is five seconds. This is not a total certified language or a proof that an allow realizes a signed formal intention.

- [docs/03-policy-engine.md:160–183](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/03-policy-engine.md#L160-L183) — SHA256 `8147877bbc8d177c94ef803f71a8ad6b4c703bacfe07169c5b4bbc141de99ced`
- [docs/03-policy-engine.md:327–340](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/03-policy-engine.md#L327-L340) — SHA256 `8147877bbc8d177c94ef803f71a8ad6b4c703bacfe07169c5b4bbc141de99ced`
- [ows/crates/ows-lib/src/policy_engine.rs:117–229](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/policy_engine.rs#L117-L229) — SHA256 `f1adbcea4bf7ba4d828426c79fd833e7c92a272ca44a3a842ec6ba0502fc1258`

## OWS14

Cardano policy construction fetches referenced transaction CBOR and verifies the requested hash, requiring every referenced input and collateral input. This binds values to referenced bytes but is not by itself proof of chain inclusion, finality or unspentness.

- [docs/cardano.md:580–630](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/cardano.md#L580-L630) — SHA256 `1d9547c5adeb543a5a0fa3e4b9ec0078582307c8781c6b974725e4a3d653827d`
- [docs/cardano.md:1019–1037](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/cardano.md#L1019-L1037) — SHA256 `1d9547c5adeb543a5a0fa3e4b9ec0078582307c8781c6b974725e4a3d653827d`

## OWS15

Cardano successful effects and failure-contingent collateral are separate fields; policies must cover collateral explicitly. This is a concrete reference for preserving phase-dependent loss obligations.

- [docs/cardano.md:671–707](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/cardano.md#L671-L707) — SHA256 `1d9547c5adeb543a5a0fa3e4b9ec0078582307c8781c6b974725e4a3d653827d`
- [docs/03-policy-engine.md:387–411](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/03-policy-engine.md#L387-L411) — SHA256 `8147877bbc8d177c94ef803f71a8ad6b4c703bacfe07169c5b4bbc141de99ced`

## OWS16

The non-normative minimal spending-cap example omits collateral while the normative policy example includes it. Its cap is therefore not a bound over both success and failure outflows.

- [docs/policy-engine-implementation.md:64–100](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/policy-engine-implementation.md#L64-L100) — SHA256 `6a05e97f1daf6c22f7b53eb66e2b7e89580cda4016949a286e5a4cfca52f4df9`
- [docs/cardano.md:671–707](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/cardano.md#L671-L707) — SHA256 `1d9547c5adeb543a5a0fa3e4b9ec0078582307c8781c6b974725e4a3d653827d`

## OWS17

Cardano dependent unconfirmed transactions are documented as unsupported through the policy path when Koios lacks parent CBOR; this creates an evidence-availability barrier distinct from malformed transactions.

- [docs/cardano.md:825–847](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/cardano.md#L825-L847) — SHA256 `1d9547c5adeb543a5a0fa3e4b9ec0078582307c8781c6b974725e4a3d653827d`

## OWS18

signAndSend returns a broadcast transaction identifier, not an explicit finality proof, and current implementations do not provide per-wallet nonce management or same-wallet serialization.

- [docs/02-signing-interface.md:35–56](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/02-signing-interface.md#L35-L56) — SHA256 `46c2b53219f5b37d28edf54f4a153fd4a689b5936feac22e4c683ed192c213e6`
- [docs/02-signing-interface.md:165–167](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/02-signing-interface.md#L165-L167) — SHA256 `46c2b53219f5b37d28edf54f4a153fd4a689b5936feac22e4c683ed192c213e6`

## OWS19

OWS NEAR message signing is raw Ed25519; structured NEP-413 is a stated follow-up. A generic NEAR signing capability therefore does not establish native NEP-413 Intents integration.

- [docs/02-signing-interface.md:76–83](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/02-signing-interface.md#L76-L83) — SHA256 `46c2b53219f5b37d28edf54f4a153fd4a689b5936feac22e4c683ed192c213e6`

## OWS20

The chain registry and supported-chain specification contain no Midnight profile. Generic multi-key traits mention Midnight roles as examples; those comments are not an implemented adapter, ZKIRv3 integration or proof-system binding.

- [docs/07-supported-chains.md:29–46](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/07-supported-chains.md#L29-L46) — SHA256 `5376ebd29bfbfcfee5cd8beb161c2020c57cff2bb0ecef111bbadf2a152244c1`
- [ows/crates/ows-core/src/chain.rs:6–39](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-core/src/chain.rs#L6-L39) — SHA256 `c21538188f48be06cf388a6f39b677e522188ad4f7a5cade33d957e8586569bc`
- [ows/crates/ows-signer/src/traits.rs:132–160](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-signer/src/traits.rs#L132-L160) — SHA256 `9c096ec8c580a6fe167f717deb5a8fb52e71226fc15518e525ca6976683181ee`

## OWS21

No reviewed OWS core profile defines MPC threshold custody, TEE attestation or ZK/PCD compliance. Current local decrypted-key signing and optional subprocess guidance cannot establish the federated kernel theorem. This is a bounded absence conclusion at the pin, not an impossibility claim.

- [docs/00-specification.md:7–85](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/00-specification.md#L7-L85) — SHA256 `c3a55e1ad3755ae582a3ac96d6e25789c65e7584ef6c90258321adfc68f1f73e`
- [docs/05-key-isolation.md:81–113](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/05-key-isolation.md#L81-L113) — SHA256 `bbba700f549e9d5ee0b9f921c88b3830a65e0d2e6bd343337de4d259c585c548`
- [ows/crates/ows-lib/src/key_ops.rs:327–351](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/ows/crates/ows-lib/src/key_ops.rs#L327-L351) — SHA256 `7556d54c66b7633449dc44103034964beaa6a09e44fc0b3bbd6a9d67327cd18c`

## OWS22

x402 usage-based settlement examples place authorized-versus-settled accounting, per-session caps and external-ledger reconciliation in the application layer.

- [examples/x402-upto-settlement.md:1–17](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/examples/x402-upto-settlement.md#L1-L17) — SHA256 `727378a54b9bb5f121d8778b889dcf0bb2aa3e305cca3f1a73952f1aaa741e59`
- [examples/x402-upto-settlement.md:47–65](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/examples/x402-upto-settlement.md#L47-L65) — SHA256 `727378a54b9bb5f121d8778b889dcf0bb2aa3e305cca3f1a73952f1aaa741e59`

## OWS23

Audit append-only requirements are from the implementation perspective and omit secrets; they do not define recursive proof-carrying provenance or tamper resistance against the filesystem owner.

- [docs/08-conformance-and-security.md:99–111](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/08-conformance-and-security.md#L99-L111) — SHA256 `9ac683e3a1d51da06ae977a6fd80d329f809d0da773ec2959a9079e2112b3d9c`
- [docs/01-storage-format.md:194–212](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/01-storage-format.md#L194-L212) — SHA256 `97aed4fe36a77d597638e05e1e6d2cfc8c5771589800dba990523cd52d7d9392`

## OWS24

The storage introduction says correct file reading/writing makes a conforming OWS implementation, while scope/conformance documents prohibit unqualified complete claims for partial targets. Read the former as storage-profile conformance.

- [docs/01-storage-format.md:1–5](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/01-storage-format.md#L1-L5) — SHA256 `97aed4fe36a77d597638e05e1e6d2cfc8c5771589800dba990523cd52d7d9392`
- [docs/00-specification.md:36–48](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/00-specification.md#L36-L48) — SHA256 `c3a55e1ad3755ae582a3ac96d6e25789c65e7584ef6c90258321adfc68f1f73e`
- [docs/08-conformance-and-security.md:5–19](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/docs/08-conformance-and-security.md#L5-L19) — SHA256 `9ac683e3a1d51da06ae977a6fd80d329f809d0da773ec2959a9079e2112b3d9c`

## OWS25

Repository default CODEOWNERS names one GitHub owner and SECURITY supplies a reporting address. These are maintenance metadata, not wallet authorization or federated validator membership.

- [CODEOWNERS:1–2](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/CODEOWNERS#L1-L2) — SHA256 `781ce4618281b9658afd190f77248feb907b4af4bcea61244cfb866ce0432f39`
- [SECURITY.md:11–24](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/SECURITY.md#L11-L24) — SHA256 `8315a1b4524eb5790e1f0019a67e35e05c013a1030cc043daf273bc51b3a3f09`

## OWS26

The website build copies canonical docs into deployed md and emits llms-full plus skill and installer. Nine of eleven checked-in website-docs/md copies differ from canonical docs, but all 15 acquired deployed Markdown files match canonical docs at this pin.

- [website-docs/vercel.json:1–4](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/website-docs/vercel.json#L1-L4) — SHA256 `a9098527a7541f31ea354ec643861c5ca10e58f369066220b45131231b6ca371`
- [website-docs/js/docs.js:1–33](https://github.com/open-wallet-standard/core/blob/b3fd7f9b3c2dd200605a2db06726c6a0d4709205/website-docs/js/docs.js#L1-L33) — SHA256 `44db936f2978ac331658536dd32e8293a4458b0d37d74a66752bdbbd1f3e3c35`
