import {readFileSync,writeFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
import {secp256k1} from '@noble/curves/secp256k1.js';
const root='/home/charl/Moriarty/.worktrees/moriarty-beta-20260930',base=root+'/packages/moriarty-beta/examples/signed-intent/transfer-ecdsa-wallet',binary='/home/charl/research/moriarty-crypto-2026-09-30/target/debug/moriarty-midnight-crypto';
const source=readFileSync(base+'/program.mori','utf8'),artifact=JSON.parse(readFileSync(base+'/signature.json')),scenario=JSON.parse(readFileSync(base+'/scenario.json'));
const native=(cmd,input)=>JSON.parse(execFileSync(binary,[cmd],{input:JSON.stringify(input),encoding:'utf8'}));
const draft=structuredClone(artifact.statement);delete draft.ownerProgramSha256;
const built=native('intent-build',draft),framed=native('intent-frame',built.statement),checked=native('intent-verify',artifact);
assert.deepEqual(built.statement,artifact.statement);assert.equal(built.frame_hex,framed.frame_hex);assert.equal(checked.signature_valid,true);assert.equal(built.statement.sourceSha256,createHash('sha256').update(source).digest('hex'));
const beta=JSON.parse(execFileSync(process.execPath,[root+'/packages/moriarty-beta/dist/cli.js','verify-intent',base+'/program.mori','--action','pay','--scenario',base+'/scenario.json','--signature',base+'/signature.json','--crypto-binary',binary],{encoding:'utf8'}));
assert.equal(beta.status,'SignedPreparedUnqualified');assert.equal(beta.sourceMatched,true);
const message=Buffer.from(framed.signing_message_hex,'hex'),pk=secp256k1.Point.fromHex(artifact.statement.signature.publicKeyHex).toAffine(),op=artifact.statement.intent.operation;
assert.equal(op.kind,'Transfer');assert.equal(op.amount,'1000');assert.equal(op.fee,'10');assert.equal(artifact.statement.intent.preHead,'h0');assert.equal(artifact.statement.intent.nonce,'n1');
const le=(v,n=32)=>{const b=[];for(let i=0;i<n;i++){b.push(Number(v&255n));v>>=8n;}return b;},bytes=b=>'Bytes['+[...b].join(',')+']',half=0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141n/2n,mask=(1n<<128n)-1n;
const beforeHead=createHash('sha256').update('local-fixture:h0').digest(),digest=createHash('sha256').update(message).digest();
const checker=`circuit configuredKey(pk: Secp256k1Point): [] {
 assert(pk != default<Secp256k1Point>, "IDENTITY_KEY");
 assert((secp256k1PointX(pk) as Bytes<32>) == ${bytes(le(pk.x))}, "SOURCE_SEC1_X");
 assert((secp256k1PointY(pk) as Bytes<32>) == ${bytes(le(pk.y))}, "SOURCE_SEC1_Y_PARITY");
}
circuit canonicalSignature(sig: Secp256k1EcdsaSignature): [] {
 const sb: Bytes<32> = sig.s as Bytes<32>;
 const hi: Uint<128> = slice<16>(sb, 16) as Uint<128>;
 const lo: Uint<128> = slice<16>(sb, 0) as Uint<128>;
 assert(hi < ${half>>128n} || (hi == ${half>>128n} && lo <= ${half&mask}), "HIGH_S");
 assert(sig.r != default<Secp256k1Scalar> && sig.s != default<Secp256k1Scalar>, "ZERO_SIGNATURE");
}
export circuit signatureIsCanonical(sig: Secp256k1EcdsaSignature): [] { canonicalSignature(sig); }
export circuit verifyOwner(message: Bytes<${message.length}>, sig: Secp256k1EcdsaSignature, pk: Secp256k1Point): Bytes<32> {
 configuredKey(pk); canonicalSignature(sig);
 assert(message == ${bytes(message)}, "FRAME_SOURCE_MISMATCH");
 const digest: Bytes<32> = persistentHash<Bytes<${message.length}>>(message);
 assert(digest == ${bytes(digest)}, "FRAME_SHA256_MISMATCH");
 assert(secp256k1EcdsaVerify(digest, sig, pk), "BAD_SIGNATURE");
 return digest;
}`;
const compact=`pragma language_version >= 0.27 && <= 0.27;
import CompactStandardLibrary;
export { Secp256k1EcdsaSignature };
${checker}
export ledger ownerKey: Secp256k1Point;
export ledger sourceDigest: Bytes<32>;
export ledger ownerProgramDigest: Bytes<32>;
export ledger usedNonce: Boolean;
export ledger head: Bytes<32>;
export ledger revision: Uint<128>;
export ledger trustedRound: Uint<128>;
export ledger ownerBalance: Uint<128>;
export ledger recipientBalance: Uint<128>;
export ledger feeBalance: Uint<128>;
export ledger workRemaining: Uint<128>;
export ledger workSpent: Uint<128>;
export ledger allowanceRemaining: Uint<128>;
export ledger allowanceSpent: Uint<128>;
export ledger assetColor: Bytes<32>;
export ledger recipientAddress: UserAddress;
export ledger feeAddress: UserAddress;
export ledger lastDebit: Uint<128>;
export ledger lastRecipientCredit: Uint<128>;
export ledger lastFeeCredit: Uint<128>;
export ledger lastAllowanceUse: Uint<128>;
constructor(pk: Secp256k1Point, color: Bytes<32>, recipient: UserAddress, fee: UserAddress, funding: Uint<128>, allowance: Uint<128>, work: Uint<128>, round: Uint<128>) {
 configuredKey(pk);
 assert(recipient.bytes != fee.bytes, "ALIASED_DESTINATIONS");
 assert(color == Bytes[161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161,161], "FIXTURE_ASSET_MAPPING");
 assert(recipient.bytes == Bytes[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2] && fee.bytes == Bytes[3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3], "FIXTURE_DESTINATION_MAPPING");
 ownerKey=disclose(pk);assetColor=disclose(color);recipientAddress=disclose(recipient);feeAddress=disclose(fee);
 sourceDigest=${bytes(Buffer.from(artifact.statement.sourceSha256,'hex'))};
 ownerProgramDigest=${bytes(Buffer.from(artifact.statement.ownerProgramSha256,'hex'))};
 usedNonce=false;head=${bytes(beforeHead)};revision=0;trustedRound=disclose(round);
 ownerBalance=disclose(funding);recipientBalance=0;feeBalance=0;
 workRemaining=disclose(work);workSpent=0;allowanceRemaining=disclose(allowance);allowanceSpent=0;
 lastDebit=0;lastRecipientCredit=0;lastFeeCredit=0;lastAllowanceUse=0;
}
export circuit pay(message: Bytes<${message.length}>, sig: Secp256k1EcdsaSignature): Bytes<32> {
 const signedDigest: Bytes<32> = disclose(verifyOwner(disclose(message), sig, ownerKey));
 assert(!usedNonce, "NONCE_CONSUMED");
 assert(head == ${bytes(beforeHead)} && revision == 0, "STALE_HEAD");
 assert(trustedRound >= ${artifact.statement.intent.notBefore} && trustedRound <= ${artifact.statement.intent.notAfter}, "TRUSTED_ROUND_WINDOW");
 assert(ownerBalance >= 1010, "OWNER_UNDERFLOW");assert(allowanceRemaining >= 1010, "ALLOWANCE_UNDERFLOW");assert(workRemaining >= 1, "WORK_EXHAUSTED");
 const workTotal: Uint<128> = (workRemaining + workSpent) as Uint<128>;
 const allowanceTotal: Uint<128> = (allowanceRemaining + allowanceSpent) as Uint<128>;
 assert(workTotal >= workRemaining && allowanceTotal >= allowanceRemaining, "TOTAL_RANGE");
 const entry: Uint<128> = unshieldedBalance(assetColor);
 assert(entry == ownerBalance, "ESCROW_BALANCE_MISMATCH");
 assert(unshieldedBalanceGte(assetColor, 1010), "ESCROW_UNDERFUNDED");
 const recipientAfter: Uint<128> = (recipientBalance + 1000) as Uint<128>;
 const feeAfter: Uint<128> = (feeBalance + 10) as Uint<128>;
 const spentAfter: Uint<128> = (allowanceSpent + 1010) as Uint<128>;
 const workSpentAfter: Uint<128> = (workSpent + 1) as Uint<128>;
 sendUnshielded(assetColor,1000,right<ContractAddress,UserAddress>(recipientAddress));
 sendUnshielded(assetColor,10,right<ContractAddress,UserAddress>(feeAddress));
 ownerBalance=(ownerBalance-1010) as Uint<128>;recipientBalance=recipientAfter;feeBalance=feeAfter;
 allowanceRemaining=(allowanceRemaining-1010) as Uint<128>;allowanceSpent=spentAfter;
 workRemaining=(workRemaining-1) as Uint<128>;workSpent=workSpentAfter;
 lastDebit=1010;lastRecipientCredit=1000;lastFeeCredit=10;lastAllowanceUse=1010;
 usedNonce=true;revision=1;
 head=persistentHash<Vector<16,Bytes<32>>>([head,signedDigest,sourceDigest,ownerProgramDigest,assetColor,recipientAddress.bytes,feeAddress.bytes,ownerBalance as Bytes<32>,recipientBalance as Bytes<32>,feeBalance as Bytes<32>,allowanceRemaining as Bytes<32>,allowanceSpent as Bytes<32>,workRemaining as Bytes<32>,workSpent as Bytes<32>,revision as Bytes<32>,trustedRound as Bytes<32>]);
 return head;
}
`;
writeFileSync(new URL('./fixed-transfer.compact',import.meta.url),compact);
const projection={source,artifact,scenario,built,framed,beta,publicKey:{x:pk.x.toString(),y:pk.y.toString(),identity:false},expected:beta.local.result.candidate,beforeHead:beforeHead.toString('hex'),messageSha256:digest.toString('hex'),messageLength:message.length,source_sha256:artifact.statement.sourceSha256,binary_sha256:createHash('sha256').update(readFileSync(binary)).digest('hex'),trust:'constructor fixture round/address/color/funding mapping; local escrow, not wallet balance'};
writeFileSync(new URL('./projection.json',import.meta.url),JSON.stringify(projection,null,2));
console.log(JSON.stringify({native_signature_valid:checked.signature_valid,messageLength:message.length,frame_sha256:framed.frame_sha256,message_sha256:digest.toString('hex'),sourceMatched:beta.sourceMatched,status:beta.status}));
