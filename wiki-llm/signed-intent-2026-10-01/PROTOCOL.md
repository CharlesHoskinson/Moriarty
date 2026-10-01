# Signed owner intent /1 implementation contract

Status: specified until implementation and independent actual-result reviews.
Decision: strict closed JSON, encoded once by native Rust. G1 and G2 recommend
this bounded route. S1 recommends fixed binary instead; preserve that alternative
in plans/S1.md. JSON reuses the existing duplicate-safe bounded parser and makes
reviewable owner terms available without a second language-specific byte encoder.
No extensibility is implied: unknown fields, JSON numbers and duplicates reject.
This profile is separate from W-D2 and the earlier experimental signature frame.

## Closed statement

Every field below is required. `intent-build` input omits exactly
`ownerProgramSha256`; all other fields are identical to the full statement.

```text
{
 profile: "moriarty-signed-intent/1",
 core: "moriarty-core/5",
 sourceProfile: "moriarty-financial-agreement-source/6",
 authoringProfile: "moriarty-beta/1",
 agreementId: ID, actionName: ID,
 selectedActionId: "TransferLiteralFee" | "RepayAccrualFirst",
 sourceSha256: HEX32, ownerProgramSha256: HEX32,
 domain: {id: ID, chain: TEXT, network: TEXT},
 asset: {id: ID, representation: TEXT, scale: DEC, symbol: TEXT | null},
 signature: {scheme: "schnorr_bip340" | "ecdsa_secp256k1_sha256",
             publicKeyHex: HEXKEY, keyRef: TEXT,
             framing: "raw" | "midnight-sign-data"},
 intent: {
  version: "moriarty-intent/3",
  advertisedSourceHash: TEXT, advertisedPolicyDigest: TEXT,
  signer: ID, nonce: TEXT, preHead: TEXT,
  notBefore: DEC, notAfter: DEC, grossCap: DEC, feeCap: DEC, netFloor: DEC,
  failure: "success_only", observations: [], disclosures: [],
  retainedEffects: [], retainedDuties: [], delegation: "none", recovery: "none",
  operation: {kind: "Transfer", from: ID, recipient: ID, feeRecipient: ID,
              amount: DEC, fee: DEC}
          | {kind: "Repay", payer: ID, obligationId: ID,
              amount: DEC, conversion: "identity"}
 }
}
```

IDs: ASCII `[A-Za-z][A-Za-z0-9_]{0,63}`. TEXT: nonempty Unicode scalar UTF-8,
<=1024 bytes, no U+0000..001F/U+007F, no normalization. DEC: canonical unsigned
decimal string `0|[1-9][0-9]*`; rounds <=2^128-1, money <=2^127-1, scale<=18.
HEX32: exactly64 lowercase hex characters. Schnorr key exactly64, compressed
ECDSA key exactly66 lowercase hex characters with canonical SEC1 prefix `02` or `03`; decode a valid native curve key.
Signature exactly128 lowercase hex characters. No `0x` prefix.
The pinned Midnight Schnorr API applies SHA256 to the framed message before
BIP340 signing/verification through its k256 trait; ECDSA likewise hashes the
framed message with SHA256. Raw framing refers to the bytes before this internal
hash. A direct raw BIP340 primitive over un-hashed F does not implement this API.

Transfer: signer=from; from/recipient/feeRecipient pairwise distinct even when
fee is zero; amount>0; amount+fee fits money; grossCap>=amount+fee, feeCap>=fee,
netFloor<=amount. Repay: signer=payer; amount>0; grossCap>=amount; feeCap=netFloor=0;
selectedActionId and operation kind match. notBefore<=notAfter. Creditor is read
from the obligation cell, not manufactured as an owner source term.

## Exact hashes and bytes

Canonical JSON: recursively sort object keys by ordinal Unicode ordering,
compact UTF-8 JSON, no whitespace, no ASCII escaping for ordinary non-ASCII
characters; JSON control escaping follows serde_json, arrays preserve order.
Keys here are closed ASCII, avoiding cross-language collation questions.

Projection P is the closed subset `{authoringProfile,core,sourceProfile,
agreementId,actionName,selectedActionId,domain,asset,intent}`. Hash:
`SHA256(UTF8("moriarty-owner-program/1\\0") || u32BE(len(P)) || P)`.
Here `\\0` denotes one NUL byte, not backslash characters.
`ownerProgramSha256` must equal that computed digest. This is an owner-term
projection, not a compiled ZKIR artifact or compiler correspondence proof.

`sourceSha256` is SHA256 of actual exact UTF-8 beta source bytes, including
comments and whitespace. advertisedSourceHash/advertisedPolicyDigest remain
opaque user claims. They are never interpreted as provenance or real hashes.

C is the full validated canonical statement. Signing frame F:
`UTF8("moriarty-signed-intent/1\\0") || u32BE(len(C)) || C`.
frame_sha256 is SHA256(F). Raw signing message is F. midnight-sign-data message
is `UTF8("midnight_signed_message:" + decimal(len(F)) + ":") || F`.
Framing itself is signed metadata. No alternate-framing verification fallback.
Wallet input should encode F as hex/base64; signing frame_sha256 or hex text
instead of decoded F does not implement this protocol. Live wallet compatibility
remains unperformed. F<=16384 bytes, input<=65536, depth<=32, nodes<=4096.

## Native Rust commands and output

`intent-build`: draft statement stdin. `intent-frame`: full statement stdin.
`intent-verify`: closed `{statement: fullStatement, signatureHex: HEXSIG}` stdin.
All commands parse duplicate-safe closed schema and validate typed constraints.
Build computes projection digest; frame/verify check supplied digest. Native
public-key deserialization precedes emitting signable bytes.

Success build/frame output exact key set:
```text
{status: "IntentBuilt" | "IntentFrame", statement: fullStatement,
 frame_hex: HEX, frame_sha256: HEX32, signing_message_hex: HEX,
 scheme: scheme, framing: framing,
 authority_valid: null, snapshot_membership_valid: null,
 transition_valid: null, ledger_accepted: false,
 qualification: "signature-protocol-only"}
```
Verification output same key set with status `IntentSignatureChecked` and
additional required `signature_valid: true|false`. Build/frame exit0;
verify exits0 for true,1 for false. Malformed input/key/signature is exit2,
exact rejection shape `{status:"IntentRejected",code:TEXT,error:TEXT,
signature_valid:null,ledger_accepted:false}`. Stable coarse codes classify
input/schema/range/hash/key/signature; never merge malformed with native false.
No signing or secret-key CLI command. Existing frame/verify APIs remain unchanged.

## Beta consumer contract

New async APIs `prepareOwnerIntent(source,action,scenarioText,signing,crypto)` and
`verifyAndPrepare(source,action,scenarioText,signatureText,crypto)` take source and
JSON text. Signing = `{scheme,publicKeyHex,framing}`. Crypto configuration =
`{binaryPath:absolute path,timeoutMs?:1..30000}`; default10000ms.
The beta consumer derives owner intent from its actual selected Source6 AST,
agreement/action and actually referenced beta domain/asset entities, including
chain/network/representation/scale/symbol. It never picks first declarations.
keyRef is always the current selected source intent's key reference.
The entire emitted/received statement must equal the source-derived one,
including actual source SHA256. Scenario changes may alter candidate preparation,
never owner-signing terms. Signature artifact is exactly
`{statement:fullStatement,signatureHex:HEXSIG}`.

Trusted executable path is explicit deployment configuration, never source or
scenario data. Spawn fixed command, no shell, no PATH lookup or runtime build.
Bound stdin65536/stdout262144/stderr8192 and timeout; fatal UTF-8, no BOM, one JSON
object and newline; reject unknown/missing/duplicate fields, mismatching hashes,
exit/status mismatch, malformed hex, foreign qualification and authority claims.
Hex response strings may exceed the default JSON parser's1024 string bound;
use a bounded internal response parser only, preserve ordinary-input defaults.

`mori intent FILE --action NAME --scenario FILE --scheme SCHEME --public-key HEX
 --framing raw|midnight-sign-data --crypto-binary /ABS/BINARY`
`mori verify-intent FILE --action NAME --scenario FILE --signature FILE
 --crypto-binary /ABS/BINARY`

Prepare output: inspectable statement, frame/signing bytes, digest and clear
qualification; no claim of signature possession. Verify reports sourceMatched,
signature checked by Rust, key authority unverified, local Core result, all
remaining premises/bindings and ledger_accepted:false. Valid signature plus Core
rejection remains signature-valid with rejected financial candidate and exit1.
Transport failure remains unknown signature validity, never false or success.
Existing simulate/inspect/expand/check semantics and required premises unchanged.
No result named Accepted. Human displays escape control/bidi/line separator
characters and print integer atoms alongside asset scale and identity.
