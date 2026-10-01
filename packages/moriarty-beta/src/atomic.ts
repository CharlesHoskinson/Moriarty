/**
 * Local single-writer settlement consumer. In-memory, qualification always `local-stipulation`.
 * It never reports ledger acceptance or external key authority; see wiki-llm/signed-intent-2026-10-01/LOCAL-SETTLEMENT.md.
 */
import {createHash} from 'node:crypto';
import {isAbsolute} from 'node:path';
import {isDeepStrictEqual} from 'node:util';
import {expand,simulate,type Simulated} from './bridge.ts';
import {prepareOwnerIntent,verifyAndPrepare,type CryptoConfig,type Signing} from './auth.ts';
import {LocalError,parseBoundedJson,scalarText} from './json.ts';
import {parseAndLowerSource6} from '../../../experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts';
import type {S0Effect,S0PreparedUnqualified,S0State} from '../../../experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts';
export type Domain={id:string;chain:string;network:string};
export type Scheme='schnorr_bip340'|'ecdsa_secp256k1_sha256';
/** Explicit local stipulation. Nothing here is an authenticated or ledger-finalized registry. */
export type Binding={bindingId:string;domain:Domain;account:string;keyRef:string;scheme:Scheme;publicKeyHex:string;validFromRound:string;validUntilRound:string;revokedFromRound:string|null};
export type Receipt={frameSha256:string;head:string;effectsSha256:string;bindingId:string};
export type Snapshot={provenance:'local-stipulation';qualification:'local-stipulation';ledger_accepted:false;authority_valid:null;ledgerDomain:Domain;state:S0State;predecessor:string;receipts:Record<string,Receipt>;registry:{revision:string;bindings:Binding[]};stateDigest:string};
export type Layer='input'|'source'|'verification'|'signature'|'authority'|'core'|'atomic';
export type FaultHook=(step:string)=>void;
type R=Record<string,unknown>;
type Root={ledgerDomain:Domain;state:S0State;predecessor:string;receipts:Record<string,Receipt>;registry:{revision:string;bindings:Binding[]}};
type Statement={domain:Domain;signature:{keyRef:string;scheme:string;publicKeyHex:string};intent:{signer:string}};
type Claims={qualification:'local-stipulation';ledger_accepted:false;authority_valid:null;requiredPremises:string[];unverifiedBindings:string[]};
const U=(1n<<128n)-1n,S=(1n<<127n)-1n;
const QUAL='local-stipulation' as const;
const ID=/^[A-Za-z][A-Za-z0-9_]{0,63}$/,HEX32=/^[a-f0-9]{64}$/;
const PREMISES=['canonical-intent-signature','snapshot-to-head','head-extension','atomic-ledger-compare-and-consume'];
const BINDINGS=['agreement-id','selected-program','asset-scale','authenticated-predecessor'];
const LOCAL_CHECKS=['native-signature-verified','local-registry-binding-matched','expected-state-digest-matched-at-read-and-commit','core5-reprepared-on-committed-state','successor-head-equals-derivation','single-writer-copy-on-write-commit'];
const KEY_WIDTH:Record<string,number>={schnorr_bip340:64,ecdsa_secp256k1_sha256:66};
const MAX_BINDINGS=64,MAX_REPLAY=4096,MAX_INFLIGHT=32;
/** Rejection carrying its layer; `signatureValid` is null while the native verdict is unknown. */
class Refusal extends LocalError{
 layer:Layer;signatureValid:boolean|null;
 constructor(layer:Layer,code:string,message:string,signatureValid:boolean|null=null){super(code,message);this.name='Refusal';this.layer=layer;this.signatureValid=signatureValid;}
}
const config=(message:string):never=>{throw new LocalError('ATOMIC_CONFIG_SCHEMA',message);};
const claims=():Claims=>({qualification:QUAL,ledger_accepted:false,authority_valid:null,requiredPremises:[...PREMISES],unverifiedBindings:[...BINDINGS]});
const clone=<T>(v:T):T=>JSON.parse(JSON.stringify(v));
function deepFreeze<T>(v:T):T{if(v&&typeof v==='object'&&!Object.isFrozen(v)){Object.freeze(v);for(const x of Object.values(v))deepFreeze(x);}return v;}
function canonical(v:unknown):string{if(Array.isArray(v))return '['+v.map(canonical).join(',')+']';if(v&&typeof v==='object')return '{'+Object.keys(v).sort().map(k=>JSON.stringify(k)+':'+canonical((v as R)[k])).join(',')+'}';return JSON.stringify(v);}
function tagged(tag:string,payload:string):string{const b=Buffer.from(payload),n=Buffer.alloc(4);n.writeUInt32BE(b.length);return createHash('sha256').update(Buffer.concat([Buffer.from(tag),n,b])).digest('hex');}
const digests=new WeakMap<object,string>();
/** Covers ledger domain, all cells, predecessor, receipts, registry revision and every binding. */
function digestOf(root:Root):string{let d=digests.get(root);if(!d){d=tagged('moriarty-local-settlement-state/1\0',canonical(root));digests.set(root,d);}return d;}
/** Hashes the signed frame and every financial effect except AdvanceHead, avoiding a circular head. */
export function deriveSuccessorHead(domain:Domain,preHead:string,frameSha256:string,effects:unknown[]):string{
 if(typeof frameSha256!=='string'||!HEX32.test(frameSha256)||typeof preHead!=='string'||!Array.isArray(effects))throw new Refusal('input','ATOMIC_INPUT_SCHEMA','Invalid successor derivation input');
 return tagged('moriarty-local-successor/1\0',canonical({domain:{id:domain.id,chain:domain.chain,network:domain.network},effects,frameSha256,preHead}));
}
/** Accept only the exact derived head: one trailing AdvanceHead from preHead, equal to the post-state head. */
export function validateSuccessor(domain:Domain,preHead:string,frameSha256:string,candidate:{effects:unknown;candidatePost:unknown}):void{
 const bad=():never=>{throw new Refusal('atomic','ATOMIC_HEAD_NOT_DERIVED','Successor head is not the deterministic derivation of the frame and effects');};
 const effects=candidate?.effects;if(!Array.isArray(effects)||effects.length<1)return bad();
 const last=effects[effects.length-1] as R,rest=effects.slice(0,-1) as R[];
 if(!last||last.kind!=='AdvanceHead'||rest.some(e=>!e||e.kind==='AdvanceHead')||last.predecessor!==preHead)return bad();
 const post=candidate.candidatePost as R|undefined;
 if(typeof last.successor!=='string'||last.successor!==deriveSuccessorHead(domain,preHead,frameSha256,rest)||post?.head!==last.successor)return bad();
}
function plain(v:unknown,required:string[],optional:string[]=[]):R{
 const bad=():never=>{throw new Refusal('input','ATOMIC_INPUT_SCHEMA','Closed request schema violated');};
 if(!v||typeof v!=='object')return bad();
 const proto=Object.getPrototypeOf(v);if(proto!==Object.prototype&&proto!==null)return bad();
 const out:R={};
 for(const key of Reflect.ownKeys(v)){
  if(typeof key!=='string'||(!required.includes(key)&&!optional.includes(key)))return bad();
  const d=Object.getOwnPropertyDescriptor(v,key)!;if(!('value' in d)||!d.enumerable)return bad();out[key]=d.value;
 }
 for(const key of required)if(!Object.hasOwn(out,key))return bad();
 return out;
}
function bounded(v:unknown,max:number):string{if(typeof v!=='string'||Buffer.byteLength(v)>max||!scalarText(v))throw new Refusal('input','ATOMIC_INPUT_SCHEMA','Invalid bounded text');return v;}
function idInput(v:unknown):string{if(typeof v!=='string'||!ID.test(v))throw new Refusal('input','ATOMIC_INPUT_SCHEMA','Invalid identifier');return v;}
// Configuration parsing: closed schemas over bounded duplicate-safe JSON text.
function rec(v:unknown,required:string[],optional:string[]=[]):R{
 if(!v||typeof v!=='object'||Array.isArray(v))return config('Expected object');
 const r=v as R;
 for(const key of Object.keys(r))if(!required.includes(key)&&!optional.includes(key))config(`Unknown field ${key}`);
 for(const key of required)if(!Object.hasOwn(r,key))config(`Missing field ${key}`);
 return r;
}
function text(v:unknown):string{if(typeof v!=='string'||!v||Buffer.byteLength(v)>1024||!scalarText(v)||/[\u0000-\u001f\u007f]/.test(v))return config('Expected nonempty bounded text');return v;}
function id(v:unknown):string{if(typeof v!=='string'||!ID.test(v))return config('Expected identifier');return v;}
function uint(v:unknown,max=U):string{if(typeof v!=='string'||v.length>39||!/^(0|[1-9][0-9]*)$/.test(v)||BigInt(v)>max)return config('Expected canonical bounded decimal');return v;}
function domainOf(v:unknown):Domain{const r=rec(v,['id','chain','network']);return {id:id(r.id),chain:text(r.chain),network:text(r.network)};}
function parseScenario(source:string):{state:S0State;predecessor:string}{
 const r=rec(parseBoundedJson(source),['profile','kind','domain','asset','head','predecessor','round','balances','allowance','replay','work_remaining','work_spent','post_head'],['obligation']);
 if(r.profile!=='moriarty-local-scenario/1'||r.kind!=='local-stipulation'||r.replay!=='unused')config('Initial scenario must be an unused local-stipulation scenario');
 text(r.post_head); // accepted for scenario-file compatibility; successor heads are always derived
 if(!Array.isArray(r.balances)||r.balances.length<2||r.balances.length>3)config('Expected 2..3 ordered balance cells');
 const balances=(r.balances as unknown[]).map(v=>{const b=rec(v,['account','amount']);return {account:id(b.account),amount:uint(b.amount)};});
 if(new Set(balances.map(b=>b.account)).size!==balances.length)config('Duplicate balance accounts');
 const a=rec(r.allowance,['owner','remaining','spent']),allowance={owner:id(a.owner),remaining:uint(a.remaining),spent:uint(a.spent)};
 const workRemaining=uint(r.work_remaining),workSpent=uint(r.work_spent);
 if(BigInt(workRemaining)+BigInt(workSpent)>U||BigInt(allowance.remaining)+BigInt(allowance.spent)>U)config('Counter total exceeds UInt128');
 let obligations:S0State['obligations']=[];
 if(Object.hasOwn(r,'obligation')){
  const d=rec(r.obligation,['id','debtor','creditor','asset','principal','accrued','outstanding','status']);
  if(d.status!=='Outstanding')config('Initial obligation must be Outstanding');
  const o={id:id(d.id),debtor:id(d.debtor),creditor:id(d.creditor),asset:id(d.asset),principal:uint(d.principal,S),accrued:uint(d.accrued,S),outstanding:uint(d.outstanding,S),status:'Outstanding' as const};
  if(BigInt(o.principal)+BigInt(o.accrued)!==BigInt(o.outstanding))config('Outstanding must equal principal plus accrued');
  obligations=[o];
 }
 return {predecessor:text(r.predecessor),state:{core:'moriarty-core/5',domain:id(r.domain),asset:id(r.asset),head:text(r.head),round:uint(r.round),workRemaining,workSpent,balances,allowances:[allowance],obligations,consumedReplay:[]}};
}
function parseRegistry(source:string,state:S0State):Root['registry']&{ledgerDomain:Domain}{
 const r=rec(parseBoundedJson(source),['profile','kind','ledgerDomain','revision','bindings']);
 if(r.profile!=='moriarty-local-authority-bindings/1'||r.kind!=='local-stipulation')config('Expected a local-stipulation bindings document');
 const ledgerDomain=domainOf(r.ledgerDomain);if(ledgerDomain.id!==state.domain)config('Ledger domain id must equal the scenario domain');
 if(!Array.isArray(r.bindings)||r.bindings.length>MAX_BINDINGS)config(`Expected at most ${MAX_BINDINGS} bindings`);
 const bindings=(r.bindings as unknown[]).map((v):Binding=>{
  const b=rec(v,['bindingId','domain','account','keyRef','scheme','publicKeyHex','validFromRound','validUntilRound','revokedFromRound']);
  if(b.scheme!=='schnorr_bip340'&&b.scheme!=='ecdsa_secp256k1_sha256')config('Unknown scheme');
  const scheme=b.scheme as Scheme;
  if(typeof b.publicKeyHex!=='string'||!new RegExp(`^[a-f0-9]{${KEY_WIDTH[scheme]}}$`).test(b.publicKeyHex))config('Public key width/case does not match the scheme');
  const validFromRound=uint(b.validFromRound),validUntilRound=uint(b.validUntilRound);
  if(BigInt(validFromRound)>BigInt(validUntilRound))config('validFromRound exceeds validUntilRound');
  return {bindingId:id(b.bindingId),domain:domainOf(b.domain),account:id(b.account),keyRef:text(b.keyRef),scheme,publicKeyHex:b.publicKeyHex as string,validFromRound,validUntilRound,revokedFromRound:b.revokedFromRound===null?null:uint(b.revokedFromRound)};
 });
 const names=bindings.map(b=>b.bindingId),slots=bindings.map(b=>canonical([b.domain,b.account,b.keyRef]));
 if(new Set(names).size!==names.length||new Set(slots).size!==slots.length)config('Duplicate binding id or (domain, account, keyRef)');
 return {ledgerDomain,revision:uint(r.revision),bindings};
}
function cryptoCopy(c:unknown):CryptoConfig{
 if(!c||typeof c!=='object'||typeof (c as R).binaryPath!=='string'||!isAbsolute((c as R).binaryPath as string))throw new LocalError('BETA_CRYPTO_BINARY_PATH','Configure an absolute native verifier path');
 const r=c as R;for(const key of Object.keys(r))if(key!=='binaryPath'&&key!=='timeoutMs')config(`Unknown crypto field ${key}`);
 if(r.timeoutMs!==undefined&&(!Number.isInteger(r.timeoutMs)||(r.timeoutMs as number)<1||(r.timeoutMs as number)>30000))throw new LocalError('BETA_CRYPTO_TIMEOUT','Timeout must be 1..30000 ms');
 return Object.freeze({binaryPath:r.binaryPath as string,...(r.timeoutMs===undefined?{}:{timeoutMs:r.timeoutMs as number})});
}
/**
 * The scenario is always derived from store state, never from a caller snapshot. A Settled obligation is rendered
 * with the scenario's only status label and zero amounts; Core/5 then refuses that stage (S0 requires Outstanding).
 */
function scenarioText(root:Root,replay:'unused'|'consumed',postHead:string):string{
 const s=root.state,a=s.allowances[0],o=s.obligations[0];
 return JSON.stringify({profile:'moriarty-local-scenario/1',kind:'local-stipulation',domain:s.domain,asset:s.asset,head:s.head,predecessor:root.predecessor,round:s.round,
  balances:s.balances.map(b=>({account:b.account,amount:b.amount})),allowance:{owner:a.owner,remaining:a.remaining,spent:a.spent},replay,work_remaining:s.workRemaining,work_spent:s.workSpent,post_head:postHead,
  ...(o?{obligation:{id:o.id,debtor:o.debtor,creditor:o.creditor,asset:o.asset,principal:o.principal,accrued:o.accrued,outstanding:o.outstanding,status:'Outstanding'}}:{})});
}
const pendingHead=(root:Root):string=>tagged('moriarty-local-pending/1\0',root.state.head);
function snapshotOf(root:Root):Snapshot{return Object.freeze({provenance:QUAL,qualification:QUAL,ledger_accepted:false as const,authority_valid:null,ledgerDomain:root.ledgerDomain,state:root.state,predecessor:root.predecessor,receipts:root.receipts,registry:root.registry,stateDigest:digestOf(root)});}
function statementOf(v:unknown):Statement{
 const bad=():never=>{throw new Refusal('verification','ATOMIC_STATEMENT_SHAPE','Verified statement has an unexpected shape');};
 const s=v as R,d=s?.domain as R,g=s?.signature as R,i=s?.intent as R;
 if(!s||!d||!g||!i||typeof d.id!=='string'||typeof d.chain!=='string'||typeof d.network!=='string'||typeof g.keyRef!=='string'||typeof g.scheme!=='string'||typeof g.publicKeyHex!=='string'||typeof i.signer!=='string')return bad();
 return {domain:{id:d.id,chain:d.chain,network:d.network},signature:{keyRef:g.keyRef,scheme:g.scheme,publicKeyHex:g.publicKeyHex},intent:{signer:i.signer}};
}
/** Resolve the explicit local registry at the root's round. Revocation and window are evaluated on the root read here. */
function resolve(root:Root,st:Statement):Binding{
 const deny=(code:string,message:string):never=>{throw new Refusal('authority',code,message,true);};
 const same=(a:Domain,b:Domain)=>a.id===b.id&&a.chain===b.chain&&a.network===b.network;
 if(!same(st.domain,root.ledgerDomain))return deny('AUTH_DOMAIN_MISMATCH','Statement domain differs from the store ledger domain');
 const slot=root.registry.bindings.filter(b=>b.account===st.intent.signer&&b.keyRef===st.signature.keyRef);
 if(!slot.length)return deny('AUTH_NO_BINDING','No local binding for this account and keyRef');
 const here=slot.filter(b=>same(b.domain,st.domain));
 if(!here.length)return deny('AUTH_DOMAIN_MISMATCH','Local binding exists only for a different chain, network or domain');
 const b=here.find(x=>x.scheme===st.signature.scheme&&x.publicKeyHex===st.signature.publicKeyHex);
 if(!b)return deny('AUTH_KEY_MISMATCH','Signed scheme or public key differs from the local binding');
 const round=BigInt(root.state.round);
 if(b.revokedFromRound!==null&&round>=BigInt(b.revokedFromRound))return deny('AUTH_REVOKED','Local binding is revoked at the state round');
 if(round<BigInt(b.validFromRound))return deny('AUTH_NOT_YET_VALID','Local binding is not yet valid');
 if(round>BigInt(b.validUntilRound))return deny('AUTH_EXPIRED','Local binding has expired');
 return b;
}
function candidateOf(sim:Simulated):S0PreparedUnqualified|null{return 'result' in sim&&sim.result.status==='PreparedUnqualified'?sim.result.candidate:null;}
function refusalOf(local:unknown):Refusal{
 const l=local as R|null,res=l?.result as R|undefined;
 if(l?.status==='CoreRejected')return new Refusal('core',String((res?.rejection as R)?.code),'Core/5 rejected the prepared candidate',true);
 if(l?.status==='SourceRejected')return new Refusal('source',String(res?.code),'Generated Source/6 was rejected',true);
 return new Refusal('source',String(((l?.diagnostics as R[]|undefined)?.[0])?.code??'ATOMIC_CORE_UNAVAILABLE'),'Scenario formation was rejected',true);
}
function failure(e:unknown){
 if(!(e instanceof LocalError))throw e;
 const r=e instanceof Refusal?e:new Refusal('verification',e.code,e.message);
 return {status:'Rejected' as const,layer:r.layer,code:r.code,message:r.message,signature_valid:r.signatureValid,stateChanged:false as const,...claims()};
}
function repeated(root:Root,replayKey:string,r:Receipt){return {status:'AlreadyCommitted' as const,replayKey,frameSha256:r.frameSha256,head:r.head,bindingId:r.bindingId,stateDigest:digestOf(root),signature_valid:true as const,stateChanged:false as const,...claims()};}
function committed(c:{binding:Binding;replayKey:string;frameSha256:string;head:string;preStateDigest:string;effects:S0Effect[];next:Root}){return {status:'LocalCommitted' as const,signature_valid:true as const,localAuthorityMatched:true as const,bindingId:c.binding.bindingId,replayKey:c.replayKey,frameSha256:c.frameSha256,head:c.head,
 preStateDigest:c.preStateDigest,postStateDigest:digestOf(c.next),effects:deepFreeze(clone(c.effects)),snapshot:snapshotOf(c.next),localChecks:[...LOCAL_CHECKS],stateChanged:true as const,...claims()};}
export type SettleResult=ReturnType<typeof failure>|ReturnType<typeof committed>|ReturnType<typeof repeated>;
export class LocalSettlementStore{
 #root:Root;#crypto:CryptoConfig;#hook:FaultHook|undefined;#queue:Promise<void>=Promise.resolve();#inflight=0;
 /** `initialScenarioText` is a moriarty-local-scenario/1 stipulation; `bindingsText` is moriarty-local-authority-bindings/1. */
 constructor(initialScenarioText:string,bindingsText:string,crypto:CryptoConfig,options?:{faultHook?:FaultHook}){
  const {state,predecessor}=parseScenario(initialScenarioText),{ledgerDomain,revision,bindings}=parseRegistry(bindingsText,state);
  this.#crypto=cryptoCopy(crypto);
  if(options!==undefined){const o=rec(options,[],['faultHook']);if(o.faultHook!==undefined&&typeof o.faultHook!=='function')config('faultHook must be a function');this.#hook=o.faultHook as FaultHook|undefined;}
  this.#root=deepFreeze({ledgerDomain,state,predecessor,receipts:{},registry:{revision,bindings}});
 }
 /** Immutable local view. Provenance is always local-stipulation. */
 snapshot():Snapshot{return snapshotOf(this.#root);}
 /** Owner bytes are state independent; the scenario is only constructed from store state to form the source. */
 async prepareIntent(input:unknown){
  const r=plain(input,['source','action','signing']),s=plain(r.signing,['scheme','publicKeyHex','framing']);
  const source=bounded(r.source,65536),action=idInput(r.action),root=this.#root;
  const prepared=await prepareOwnerIntent(source,action,scenarioText(root,'unused',pendingHead(root)),{scheme:s.scheme as Signing['scheme'],publicKeyHex:s.publicKeyHex as string,framing:s.framing as Signing['framing']},this.#crypto);
  return {...prepared,provenance:QUAL,stateDigest:digestOf(root)};
 }
 /** Local operator action; no authenticated revoker. Takes effect from `fromRound` (default: current state round). */
 async revoke(input:unknown){
  try{
   const r=plain(input,['bindingId'],['fromRound']),bindingId=idInput(r.bindingId);
   if(r.fromRound!==undefined&&(typeof r.fromRound!=='string'||!/^(0|[1-9][0-9]*)$/.test(r.fromRound)||r.fromRound.length>39||BigInt(r.fromRound)>U))throw new Refusal('input','ATOMIC_INPUT_SCHEMA','Invalid fromRound');
   return await this.#serial(()=>{
    const cur=this.#root,index=cur.registry.bindings.findIndex(b=>b.bindingId===bindingId);
    if(index<0)throw new Refusal('atomic','ATOMIC_UNKNOWN_BINDING','No such local binding');
    const old=cur.registry.bindings[index],from=(r.fromRound as string|undefined)??cur.state.round;
    const effective=old.revokedFromRound!==null&&BigInt(old.revokedFromRound)<=BigInt(from)?old.revokedFromRound:from;
    const revision=BigInt(cur.registry.revision)+1n;if(revision>U)throw new Refusal('atomic','ATOMIC_CAPACITY','Registry revision exhausted');
    const bindings=cur.registry.bindings.map((b,i)=>i===index?{...b,revokedFromRound:effective}:b);
    const next=deepFreeze({ledgerDomain:cur.ledgerDomain,state:cur.state,predecessor:cur.predecessor,receipts:cur.receipts,registry:{revision:revision.toString(),bindings}} as Root);
    this.#root=next;
    return {status:'LocalBindingRevoked' as const,bindingId,revokedFromRound:effective,registryRevision:next.registry.revision,stateDigest:digestOf(next),stateChanged:true as const,...claims()};
   });
  }catch(e){return failure(e);}
 }
 /** Verify (native Rust) then Core/5 on store state, then commit everything at once or nothing. */
 async settle(input:unknown):Promise<SettleResult>{
  try{
   const r=plain(input,['source','action','signatureText','expectedStateDigest']);
   const req={source:bounded(r.source,65536),action:idInput(r.action),signatureText:bounded(r.signatureText,65536),expectedStateDigest:r.expectedStateDigest as string};
   if(typeof req.expectedStateDigest!=='string'||!HEX32.test(req.expectedStateDigest))throw new Refusal('input','ATOMIC_INPUT_SCHEMA','expectedStateDigest must be 64 lowercase hex characters');
   if(this.#inflight>=MAX_INFLIGHT)throw new Refusal('atomic','ATOMIC_BUSY','Too many concurrent settlements');
   this.#inflight++;
   try{return await this.#settle(req);}finally{this.#inflight--;}
  }catch(e){return failure(e);}
 }
 async #settle(req:{source:string;action:string;signatureText:string;expectedStateDigest:string}){
  const read=this.#root,readDigest=digestOf(read),stale=req.expectedStateDigest!==readDigest;
  const stale_=()=>new Refusal('atomic','ATOMIC_STALE_STATE','State changed since the expected digest',true);
  const first=expand(req.source,req.action,scenarioText(read,'unused',pendingHead(read)));
  if(first.status!=='Expanded')throw new Refusal('source',first.diagnostics[0]?.code??'ATOMIC_SOURCE',first.diagnostics[0]?.message??'Source rejected');
  const {intent}=parseAndLowerSource6(first.source6),replayKey=JSON.stringify([intent.domain,intent.signer,intent.nonce]);
  if(stale&&!read.receipts[replayKey])throw new Refusal('atomic','ATOMIC_STALE_STATE','State changed since the expected digest');
  const replay=read.state.consumedReplay.includes(replayKey)?'consumed':'unused';
  const verified=await verifyAndPrepare(req.source,req.action,scenarioText(read,replay,pendingHead(read)),req.signatureText,this.#crypto);
  if(verified.status==='SignatureRejected')throw new Refusal('signature','SIGNATURE_INVALID','Native verifier reports the signature is invalid',false);
  const frameSha256=verified.signature.frame_sha256,statement=statementOf(verified.signature.statement);
  if(typeof frameSha256!=='string'||!HEX32.test(frameSha256))throw new Refusal('verification','ATOMIC_STATEMENT_SHAPE','Verified frame digest has an unexpected shape');
  const seen=this.#root.receipts[replayKey];
  if(seen&&seen.frameSha256===frameSha256)return repeated(this.#root,replayKey,seen);
  if(stale)throw stale_();
  resolve(read,statement);
  const placeholder=verified.status==='SignedPreparedUnqualified'&&verified.local?candidateOf(verified.local):null;
  if(!placeholder)throw refusalOf(verified.local);
  if(!isDeepStrictEqual(placeholder.requiredPremises,PREMISES))throw new Refusal('core','ATOMIC_CORE_PREMISES','Core/5 premise list changed; refusing to commit',true);
  return this.#serial(()=>this.#commit({source:req.source,action:req.action,read,readDigest,replayKey,frameSha256,statement,prepared:placeholder.effects}));
 }
 /** Re-read, re-verify authority and digest, re-run Core/5 on the committed state, then swap one root. */
 #commit(c:{source:string;action:string;read:Root;readDigest:string;replayKey:string;frameSha256:string;statement:Statement;prepared:S0Effect[]}){
  const cur=this.#root,seen=cur.receipts[c.replayKey];
  if(seen&&seen.frameSha256===c.frameSha256)return repeated(cur,c.replayKey,seen);
  const binding=resolve(cur,c.statement);
  if(digestOf(cur)!==c.readDigest)throw new Refusal('atomic','ATOMIC_STALE_STATE','State or registry changed between prepare and commit',true);
  if(Object.keys(cur.receipts).length>=MAX_REPLAY)throw new Refusal('atomic','ATOMIC_CAPACITY','Replay receipt capacity reached',true);
  const mismatch=(message:string)=>new Refusal('atomic','ATOMIC_CANDIDATE_MISMATCH',message,true);
  const financial=c.prepared.slice(0,-1),head=deriveSuccessorHead(cur.ledgerDomain,cur.state.head,c.frameSha256,financial);
  const sim=simulate(c.source,c.action,scenarioText(cur,'unused',head)),cand=candidateOf(sim);
  if(!cand)throw mismatch('Core/5 no longer prepares the candidate on the committed state');
  validateSuccessor(cur.ledgerDomain,cur.state.head,c.frameSha256,cand);
  if(!isDeepStrictEqual(cand.effects.slice(0,-1),financial))throw mismatch('Re-prepared effects differ from the verified candidate');
  const post=cand.candidatePost,st=cur.state;
  if(post.core!==st.core||post.domain!==st.domain||post.asset!==st.asset||post.round!==st.round||!isDeepStrictEqual(post.consumedReplay,[c.replayKey])||post.head!==head)throw mismatch('Candidate post-state does not extend the stored state');
  const step=(name:string)=>{this.#hook?.(name);};
  try{
   // Copy-on-write: every write builds a new value; only the final assignment publishes anything.
   step('balances');const balances=clone(post.balances);
   step('allowances');const allowances=clone(post.allowances);
   step('obligations');const obligations=clone(post.obligations);
   step('replay');const consumedReplay=[...st.consumedReplay,c.replayKey];
   step('work');const {workRemaining,workSpent}=post;
   step('head');
   step('receipt');const receipts={...cur.receipts,[c.replayKey]:{frameSha256:c.frameSha256,head,effectsSha256:tagged('moriarty-local-effects/1\0',canonical(cand.effects)),bindingId:binding.bindingId}};
   const next=deepFreeze({ledgerDomain:cur.ledgerDomain,state:{...st,balances,allowances,obligations,consumedReplay,workRemaining,workSpent,head},predecessor:st.head,receipts,registry:cur.registry} as Root);
   step('swap');this.#root=next;
   return committed({binding,replayKey:c.replayKey,frameSha256:c.frameSha256,head,preStateDigest:c.readDigest,effects:cand.effects,next});
  }catch(e){
   if(e instanceof Refusal)throw e;
   throw new Refusal('atomic','ATOMIC_COMMIT_FAULT','Commit aborted; state unchanged',true);
  }
 }
 /** Mutex over registry and commit sections; slow native work stays outside it and is re-validated inside. */
 #serial<T>(fn:()=>T):Promise<T>{const run=this.#queue.then(fn);this.#queue=run.then(()=>undefined,()=>undefined);return run;}
}
