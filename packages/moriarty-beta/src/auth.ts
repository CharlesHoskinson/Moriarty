import {spawn} from 'node:child_process';
import {isAbsolute} from 'node:path';
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {expand,simulate,type Simulated} from './bridge.ts';
import {analyze,type Value} from './frontend.ts';
import {LocalError,parseBoundedJson,parseJsonWithinLimits} from './json.ts';
import {parseAndLowerSource6} from '../../../experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts';
export type Signing={scheme:'schnorr_bip340'|'ecdsa_secp256k1_sha256';publicKeyHex:string;framing:'raw'|'midnight-sign-data'};
export type CryptoConfig={binaryPath:string;timeoutMs?:number};
/** Public signed terms are structural observations, never authority evidence. */
export interface SignedIntentStatement {
 profile:'moriarty-signed-intent/1';core:'moriarty-core/5';
 sourceProfile:'moriarty-financial-agreement-source/6';authoringProfile:'moriarty-beta/1';
 agreementId:string;actionName:string;selectedActionId:'TransferLiteralFee'|'RepayAccrualFirst';
 sourceSha256:string;ownerProgramSha256:string;
 domain:{id:string;chain:string;network:string};
 asset:{id:string;representation:string;scale:string;symbol:string|null};
 signature:Signing & {keyRef:string};
 intent:{version:'moriarty-intent/3';advertisedSourceHash:string;advertisedPolicyDigest:string;
  signer:string;nonce:string;preHead:string;notBefore:string;notAfter:string;
  grossCap:string;feeCap:string;netFloor:string;failure:'success_only';
  observations:[];disclosures:[];retainedEffects:[];retainedDuties:[];delegation:'none';recovery:'none';
  operation:{kind:'Transfer';from:string;recipient:string;feeRecipient:string;amount:string;fee:string}
   |{kind:'Repay';payer:string;obligationId:string;amount:string;conversion:'identity'};};
}
export interface NativeIntentReceipt {
 status:'IntentBuilt'|'IntentFrame'|'IntentSignatureChecked';statement:SignedIntentStatement;
 frame_hex:string;frame_sha256:string;signing_message_hex:string;
 scheme:Signing['scheme'];framing:Signing['framing'];authority_valid:null;
 snapshot_membership_valid:null;transition_valid:null;ledger_accepted:false;
 qualification:'signature-protocol-only';
}
export interface NativeIntentVerificationReceipt extends NativeIntentReceipt {status:'IntentSignatureChecked';signature_valid:boolean}
export interface OwnerIntentPreparation extends Omit<NativeIntentReceipt,'status'> {
 status:'OwnerIntentPrepared';signaturePossession:'NotChecked';keyAuthority:'Unverified';
}
export interface SignedLocalPreparation {
 status:'SignatureRejected'|'SignedPreparedUnqualified'|'SignedCoreRejected';sourceMatched:true;
 signature:NativeIntentVerificationReceipt;keyAuthority:'Unverified';domainMapping:'SourceClaimsBound';
 expiry:'LocalRoundWithinWindow'|'LocalRoundOutsideWindow'|'NotChecked';
 replay:'LocallyConsumed'|'LocallyUnused'|'NotChecked';financial:'LocallyPrepared'|'CoreRejected'|'NotChecked';
 state:'LocalStipulationOnly';nativeProof:'NotChecked';ledger:'NotSubmitted';ledger_accepted:false;
 local:Simulated|null;requiredPremises:string[];unverifiedBindings:string[];
}
export class IntentSourceMismatchError extends LocalError {
 readonly claimed:unknown; readonly computed:unknown;
 constructor(pointer:string,claimed:unknown,computed:unknown){super('BETA_SIGNATURE_SOURCE_MISMATCH','Signed statement differs from current source',pointer);this.claimed=claimed;this.computed=computed;}
}
function firstDifference(claimed:unknown,computed:unknown,pointer='/statement'):{pointer:string;claimed:unknown;computed:unknown}|null{
 if(isDeepStrictEqual(claimed,computed))return null;
 if(claimed&&computed&&typeof claimed==='object'&&typeof computed==='object'&&!Array.isArray(claimed)&&!Array.isArray(computed)){
  const a=claimed as R,b=computed as R;
  for(const key of new Set([...Object.keys(b),...Object.keys(a)])){
   const found=firstDifference(a[key],b[key],pointer+'/'+key.replaceAll('~','~0').replaceAll('/','~1'));if(found)return found;
  }
 }
 return {pointer,claimed,computed};
}
type R=Record<string,unknown>;
const fail=(code:string,message:string):never=>{throw new LocalError(code,message);};
const hash=(b:string|Buffer)=>createHash('sha256').update(b).digest('hex');
// Validate returned canonical bytes only; Rust remains the signing encoder.
function canonical(v:unknown):string {if(Array.isArray(v))return '['+v.map(canonical).join(',')+']';if(v&&typeof v==='object')return '{'+Object.keys(v).sort().map(k=>JSON.stringify(k)+':'+canonical((v as R)[k])).join(',')+'}';return JSON.stringify(v);}
function taggedDigest(tag:string,payload:string):string{const b=Buffer.from(payload),size=Buffer.alloc(4);size.writeUInt32BE(b.length);return hash(Buffer.concat([Buffer.from(tag),size,b]));}
function closed(v:unknown,keys:string[],code='BETA_CRYPTO_RESPONSE'):R{if(!v||typeof v!=='object'||Array.isArray(v))return fail(code,'Expected closed object');const r=v as R;if(Object.keys(r).length!==keys.length||keys.some(k=>!Object.hasOwn(r,k)))fail(code,'Unexpected object fields');return r;}
function native(config:CryptoConfig,command:string,value:unknown):Promise<{value:unknown;exit:number}>{
 if(!config||typeof config.binaryPath!=='string'||!isAbsolute(config.binaryPath))fail('BETA_CRYPTO_BINARY_PATH','Configure an absolute native verifier path');
 const timeout=config.timeoutMs??10000;if(!Number.isInteger(timeout)||timeout<1||timeout>30000)fail('BETA_CRYPTO_TIMEOUT','Timeout must be 1..30000 ms');
 const input=Buffer.from(JSON.stringify(value));if(input.length>65536)fail('BETA_CRYPTO_INPUT_BOUND','Native request exceeds bound');
 return new Promise((resolve,reject)=>{const child=spawn(config.binaryPath,[command],{shell:false,stdio:['pipe','pipe','pipe']});let settled=false;const chunks:Buffer[]=[];let out=0,err=0;
 // A descendant may keep inherited pipes open after the configured process dies.
 // Settle on the bound itself; waiting for 'close' would remove the time bound.
 const stop=(code:string,message:string)=>{if(settled)return;settled=true;clearTimeout(timer);child.kill('SIGKILL');child.stdin.destroy();child.stdout.destroy();child.stderr.destroy();reject(new LocalError(code,message));};
 const timer=setTimeout(()=>stop('BETA_CRYPTO_TIMEOUT','Native verifier timed out'),timeout);
 child.on('error',()=>stop('BETA_CRYPTO_BINARY_UNAVAILABLE','Native verifier unavailable; configure an installed binary'));child.stdin.on('error',()=>stop('BETA_CRYPTO_PROCESS','Native stdin failed'));
 child.stdout.on('data',(b:Buffer)=>{out+=b.length;if(out>262144)stop('BETA_CRYPTO_OUTPUT_BOUND','Native stdout exceeds bound');else chunks.push(b);});child.stderr.on('data',(b:Buffer)=>{err+=b.length;if(err>8192)stop('BETA_CRYPTO_OUTPUT_BOUND','Native stderr exceeds bound');});
 child.on('close',exit=>{if(settled)return;settled=true;clearTimeout(timer);try{const b=Buffer.concat(chunks);if(b[0]===239&&b[1]===187&&b[2]===191)fail('BETA_CRYPTO_RESPONSE','Response BOM');const t=new TextDecoder('utf-8',{fatal:true}).decode(b);if(!t.endsWith('\n')||t.slice(0,-1).includes('\n'))fail('BETA_CRYPTO_RESPONSE','Expected one JSON line');resolve({value:parseJsonWithinLimits(t,262144,131072),exit:exit??-1});}catch(e){reject(e instanceof LocalError&&e.code.startsWith('BETA_CRYPTO_')?e:new LocalError('BETA_CRYPTO_RESPONSE',e instanceof LocalError?`Invalid native JSON (${e.code})`:'Invalid native response'));}});child.stdin.end(input);});
}
function fields(v:Value):Record<string,Value>{if(v.tag!=='entity'&&v.tag!=='record')return fail('BETA_SIGNATURE_SCHEMA','Expected source entity');return v.fields;}
function text(v:Value):string{if(v.tag!=='string'&&v.tag!=='scalar')return fail('BETA_SIGNATURE_SCHEMA','Expected source text');return v.value;}
function signingValue(v:unknown):Signing{const r=closed(v,['scheme','publicKeyHex','framing'],'BETA_SIGNATURE_SCHEMA');if(!['schnorr_bip340','ecdsa_secp256k1_sha256'].includes(String(r.scheme))||!['raw','midnight-sign-data'].includes(String(r.framing))||typeof r.publicKeyHex!=='string'||!new RegExp(`^[a-f0-9]{${r.scheme==='schnorr_bip340'?64:66}}$`).test(r.publicKeyHex))fail('BETA_SIGNATURE_SCHEMA','Invalid signing metadata');return r as Signing;}
function draft(source:string,action:string,scenario:string,signing:Signing):R{
 const e=expand(source,action,scenario);if(e.status!=='Expanded')return fail('BETA_AUTH_SOURCE',JSON.stringify(e));
 const ast=parseAndLowerSource6(e.source6).ast,a=analyze(source),selected=a.actions.find(x=>x.name===action)!;
 const intent=fields(a.declarations.find(x=>x.name===selected.intent)!.value),domain=fields(intent.domain),asset=fields(intent.asset),i=ast.intent,op=i.signedAction;
 return {profile:'moriarty-signed-intent/1',core:'moriarty-core/5',sourceProfile:ast.profile,authoringProfile:'moriarty-beta/1',agreementId:ast.programId,actionName:action,selectedActionId:ast.selected.actionId,sourceSha256:hash(source),domain:{id:text(domain.id),chain:text(domain.chain),network:text(domain.network)},asset:{id:text(asset.id),representation:text(asset.representation),scale:text(asset.scale),symbol:asset.symbol?text(asset.symbol):null},signature:{...signing,keyRef:i.keyRef},intent:{version:'moriarty-intent/3',advertisedSourceHash:ast.selected.sourceHash,advertisedPolicyDigest:ast.selected.policyDigest,signer:i.signer,nonce:i.nonce,preHead:i.preHead,notBefore:i.notBefore,notAfter:i.notAfter,grossCap:i.grossCap,feeCap:i.feeCap,netFloor:i.netFloor,failure:'success_only',observations:[],disclosures:[],retainedEffects:[],retainedDuties:[],delegation:'none',recovery:'none',operation:op.kind==='Transfer'?{kind:'Transfer',from:op.from,recipient:op.to,feeRecipient:op.feeTo,amount:op.value,fee:op.fee}:{kind:'Repay',payer:op.payer,obligationId:op.obligation,amount:op.amount,conversion:'identity'}}};
}
const keys=['status','statement','frame_hex','frame_sha256','signing_message_hex','scheme','framing','authority_valid','snapshot_membership_valid','transition_valid','ledger_accepted','qualification'];
function checked(result:{value:unknown;exit:number},expected:R,status:string,verify:true):NativeIntentVerificationReceipt;
function checked(result:{value:unknown;exit:number},expected:R,status:string,verify?:false):NativeIntentReceipt;
function checked(result:{value:unknown;exit:number},expected:R,status:string,verify=false):NativeIntentReceipt|NativeIntentVerificationReceipt{
 if(result.value&&typeof result.value==='object'&&(result.value as R).status==='IntentRejected'){const r=closed(result.value,['status','code','error','signature_valid','ledger_accepted']);if(result.exit!==2||r.signature_valid!==null||r.ledger_accepted!==false||typeof r.code!=='string'||typeof r.error!=='string')fail('BETA_CRYPTO_RESPONSE','Invalid rejection');fail('BETA_SIGNATURE_SCHEMA',`${r.code}: ${r.error}`);}
 const r=closed(result.value,verify?[...keys,'signature_valid']:keys);
 if(r.status!==status||r.qualification!=='signature-protocol-only'||r.authority_valid!==null||r.snapshot_membership_valid!==null||r.transition_valid!==null||r.ledger_accepted!==false)fail('BETA_CRYPTO_RESPONSE','Invalid native claims');
 if(verify?typeof r.signature_valid!=='boolean'||result.exit!==(r.signature_valid?0:1):result.exit!==0)fail('BETA_CRYPTO_RESPONSE','Status/exit mismatch');
 if(!r.statement||typeof r.statement!=='object')fail('BETA_CRYPTO_RESPONSE','Missing statement');const statement=r.statement as R,{ownerProgramSha256,...without}=statement;
 if(typeof ownerProgramSha256!=='string'||!/^[a-f0-9]{64}$/.test(ownerProgramSha256)||!isDeepStrictEqual(without,expected))fail('BETA_CRYPTO_RESPONSE','Statement differs from source');
 const projection:R={};for(const key of ['authoringProfile','core','sourceProfile','agreementId','actionName','selectedActionId','domain','asset','intent'])projection[key]=expected[key];
 if(taggedDigest('moriarty-owner-program/1\0',canonical(projection))!==ownerProgramSha256)fail('BETA_CRYPTO_RESPONSE','Owner projection hash mismatch');
 const sig=expected.signature as Signing;if(r.scheme!==sig.scheme||r.framing!==sig.framing)fail('BETA_CRYPTO_RESPONSE','Signing metadata mismatch');
 if(typeof r.frame_hex!=='string'||!/^(?:[a-f0-9]{2})+$/.test(r.frame_hex)||r.frame_hex.length>32768||typeof r.signing_message_hex!=='string'||!/^(?:[a-f0-9]{2})+$/.test(r.signing_message_hex)||hash(Buffer.from(r.frame_hex,'hex'))!==r.frame_sha256)fail('BETA_CRYPTO_RESPONSE','Invalid bytes/hash');
 const frame=Buffer.from(r.frame_hex as string,'hex'),tag=Buffer.from('moriarty-signed-intent/1\0');if(!frame.subarray(0,tag.length).equals(tag)||frame.length<tag.length+4||frame.readUInt32BE(tag.length)!==frame.length-tag.length-4)fail('BETA_CRYPTO_RESPONSE','Invalid frame');
 if(frame.subarray(tag.length+4).toString('utf8')!==canonical(statement))fail('BETA_CRYPTO_RESPONSE','Noncanonical frame payload');
 if(!isDeepStrictEqual(parseJsonWithinLimits(new TextDecoder('utf-8',{fatal:true}).decode(frame.subarray(tag.length+4)),16384,1024),statement))fail('BETA_CRYPTO_RESPONSE','Frame statement mismatch');
 const message=sig.framing==='raw'?frame:Buffer.concat([Buffer.from(`midnight_signed_message:${frame.length}:`),frame]);if(message.toString('hex')!==r.signing_message_hex)fail('BETA_CRYPTO_RESPONSE','Message mismatch');return r as unknown as NativeIntentReceipt|NativeIntentVerificationReceipt;
}
export async function prepareOwnerIntent(source:string,action:string,scenarioText:string,signing:Signing,crypto:CryptoConfig):Promise<OwnerIntentPreparation>{const expected=draft(source,action,scenarioText,signingValue(signing));const receipt=checked(await native(crypto,'intent-build',expected),expected,'IntentBuilt');return {...receipt,status:'OwnerIntentPrepared',signaturePossession:'NotChecked',keyAuthority:'Unverified',ledger_accepted:false};}
export async function verifyAndPrepare(source:string,action:string,scenarioText:string,signatureText:string,crypto:CryptoConfig):Promise<SignedLocalPreparation>{
 const artifact=closed(parseBoundedJson(signatureText),['statement','signatureHex'],'BETA_SIGNATURE_SCHEMA');if(typeof artifact.signatureHex!=='string'||!/^[a-f0-9]{128}$/.test(artifact.signatureHex))fail('BETA_SIGNATURE_SCHEMA','Invalid signature hex');
 const supplied=artifact.statement as R;if(!supplied||typeof supplied!=='object')fail('BETA_SIGNATURE_SCHEMA','Missing statement');const sig=closed(supplied.signature,['scheme','publicKeyHex','keyRef','framing'],'BETA_SIGNATURE_SCHEMA');const signing=signingValue({scheme:sig.scheme,publicKeyHex:sig.publicKeyHex,framing:sig.framing}),expected=draft(source,action,scenarioText,signing);
 const built=checked(await native(crypto,'intent-build',expected),expected,'IntentBuilt');const difference=firstDifference(supplied,built.statement);if(difference)throw new IntentSourceMismatchError(difference.pointer,difference.claimed,difference.computed);
 const receipt=checked(await native(crypto,'intent-verify',artifact),expected,'IntentSignatureChecked',true),local=receipt.signature_valid?simulate(source,action,scenarioText):null;
 const e=expand(source,action,scenarioText);if(e.status!=='Expanded')return fail('BETA_AUTH_SOURCE','Source expansion changed');const lowered=parseAndLowerSource6(e.source6);
 const round=BigInt(lowered.state.round),within=round>=BigInt(lowered.intent.notBefore)&&round<=BigInt(lowered.intent.notAfter),replayKey=JSON.stringify([lowered.intent.domain,lowered.intent.signer,lowered.intent.nonce]);
 return {status:!receipt.signature_valid?'SignatureRejected':local?.status==='PreparedUnqualified'?'SignedPreparedUnqualified':'SignedCoreRejected',sourceMatched:true,signature:receipt,keyAuthority:'Unverified',domainMapping:'SourceClaimsBound',expiry:receipt.signature_valid?(within?'LocalRoundWithinWindow':'LocalRoundOutsideWindow'):'NotChecked',replay:receipt.signature_valid?(lowered.state.consumedReplay.includes(replayKey)?'LocallyConsumed':'LocallyUnused'):'NotChecked',financial:local?.status==='PreparedUnqualified'?'LocallyPrepared':local?'CoreRejected':'NotChecked',state:'LocalStipulationOnly',nativeProof:'NotChecked',ledger:'NotSubmitted',ledger_accepted:false,local,requiredPremises:['canonical-intent-signature','snapshot-to-head','head-extension','atomic-ledger-compare-and-consume'],unverifiedBindings:['agreement-id','selected-program','asset-scale','authenticated-predecessor']};
}
