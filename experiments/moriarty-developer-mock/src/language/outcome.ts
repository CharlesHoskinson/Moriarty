/** Solver-independent, bounded outcome authority. Counterparty authorization is an outer-runtime duty. */
import type { Action } from './core.ts';
import { canonical, decodeCanonical, hashCanonical } from './claims.ts';

export type AssetId = {domain:string; issuer:string; reference:string; kind:'token'|'claim'};
export type OutcomeDomain = {network:string; deployment:string};
export type IntentIR = {version:'moriarty-intent/1'; domain:OutcomeDomain; principal:string; nonce:string;
  validity:{notBefore:string; expiresAt:string}; lifecycle:'atomic';
  authority:{asset:AssetId; maxDebit:string; recipients:string[]}[];
  goals:{asset:AssetId; account:string; minCredit:string}[];
  fees:{asset:AssetId; maxFee:string}[];
  agreements:{id:string; programHash:string}[];
  requiredClaims:['ContractInvariant','IntentRefinement','TransitionValidity','HistoryCompliance']; extensions:[]};
export type SignedIntent = {kind:'SignedOutcomeIntent'; intent:IntentIR; intentHash:string; publicKey:string; signature:string};
export type TrustContext = {domain:OutcomeDomain; principal:string; publicKey:string};
export type AssetEffect = {kind:'Transfer'|'Fee'; asset:AssetId; from:string; to:string; amount:string};
export type Balance = {asset:AssetId; account:string; amount:string};
export type PlanIR = {version:'moriarty-plan/1'; intentHash:string;
  steps:{agreementId:string; programHash:string; predecessorHash:string; action:Action}[]; fees:AssetEffect[]};
export type Checked = {outcome:'checked'};
export type Rejected = {outcome:'rejected'; code:string; message:string};
export type TraceChecked = {outcome:'checked'; grossDebits:{asset:AssetId;amount:string}[]; netCredits:{asset:AssetId;account:string;amount:string}[]};

const encoder=new TextEncoder();
const MAX=(1n<<128n)-1n;
const ID=/^[A-Za-z0-9][A-Za-z0-9_.:/-]{0,63}$/;
const REQUIRED=['ContractInvariant','IntentRefinement','TransitionValidity','HistoryCompliance'] as const;
function fail(code:string,message:string):never {const error=new Error(message) as Error&{code:string};error.code=code;throw error;}
function rejected(error:unknown):Rejected {return {outcome:'rejected',code:typeof error==='object'&&error!==null&&'code'in error&&typeof error.code==='string'?error.code:'MalformedIntent',message:error instanceof Error?error.message:'Malformed outcome intent.'};}
function record(value:unknown,label:string):Record<string,unknown>{if(value===null||typeof value!=='object'||Array.isArray(value))fail('MalformedIntent',`${label} must be an object.`);return value as Record<string,unknown>;}
function exact(value:unknown,keys:readonly string[],label:string):Record<string,unknown>{const x=record(value,label);const actual=Object.keys(x).sort();if(actual.length!==keys.length||actual.some((k,i)=>k!==[...keys].sort()[i]))fail('MalformedIntent',`${label} has unknown or missing fields.`);return x;}
function id(value:unknown,label:string):string {if(typeof value!=='string'||!ID.test(value))fail('MalformedIntent',`${label} must be a bounded ASCII identifier.`);return value;}
function uint(value:unknown,label:string):string {if(typeof value!=='string'||!/^(0|[1-9][0-9]*)$/.test(value)||value.length>39||BigInt(value)>MAX)fail('MalformedIntent',`${label} must be a canonical UInt128 string.`);return value;}
function hex(value:unknown,bytes:number,label:string):string {if(typeof value!=='string'||value.length!==bytes*2||!new RegExp(`^[0-9a-f]{${bytes*2}}$`).test(value))fail('MalformedIntent',`${label} must be lowercase ${bytes}-byte hex.`);return value;}
function array(value:unknown,max:number,label:string):unknown[]{if(!Array.isArray(value)||value.length>max)fail('MalformedIntent',`${label} must be an array of at most ${max} items.`);return value;}
function asset(value:unknown,label:string):AssetId {const x=exact(value,['domain','issuer','reference','kind'],label);const kind=x.kind;if(kind!=='token'&&kind!=='claim')fail('MalformedIntent',`${label}.kind is unsupported.`);return {domain:id(x.domain,`${label}.domain`),issuer:id(x.issuer,`${label}.issuer`),reference:id(x.reference,`${label}.reference`),kind};}
function domain(value:unknown,label:string):OutcomeDomain {const x=exact(value,['network','deployment'],label);return {network:id(x.network,`${label}.network`),deployment:id(x.deployment,`${label}.deployment`)};}
function distinct(values:string[],label:string):void {if(new Set(values).size!==values.length)fail('MalformedIntent',`${label} contains duplicates.`);}
function add(a:bigint,b:bigint,label:string):bigint {const n=a+b;if(n>MAX)fail('AccountingOverflow',`${label} exceeds UInt128.`);return n;}

export function assetKey(value:AssetId):string {const a=asset(decodeCanonical(canonical(value)),'asset');return canonical(a);}

export function readIntent(value:unknown):IntentIR {
  const root=exact(decodeCanonical(canonical(value)),['version','domain','principal','nonce','validity','lifecycle','authority','goals','fees','agreements','requiredClaims','extensions'],'intent');
  if(root.version!=='moriarty-intent/1'||root.lifecycle!=='atomic')fail('UnsupportedProfile','Unsupported intent version or lifecycle.');
  const validity=exact(root.validity,['notBefore','expiresAt'],'intent.validity');
  const authority=array(root.authority,8,'intent.authority').map((v,n)=>{const x=exact(v,['asset','maxDebit','recipients'],`authority[${n}]`);const recipients=array(x.recipients,8,`authority[${n}].recipients`).map((r,i)=>id(r,`recipient[${i}]`));distinct(recipients,`authority[${n}].recipients`);return {asset:asset(x.asset,`authority[${n}].asset`),maxDebit:uint(x.maxDebit,`authority[${n}].maxDebit`),recipients};});
  distinct(authority.map(x=>assetKey(x.asset)),'intent.authority assets');
  const goals=array(root.goals,8,'intent.goals').map((v,n)=>{const x=exact(v,['asset','account','minCredit'],`goals[${n}]`);return {asset:asset(x.asset,`goals[${n}].asset`),account:id(x.account,`goals[${n}].account`),minCredit:uint(x.minCredit,`goals[${n}].minCredit`)};});
  distinct(goals.map(x=>assetKey(x.asset)+'|'+x.account),'intent goals');
  const fees=array(root.fees,8,'intent.fees').map((v,n)=>{const x=exact(v,['asset','maxFee'],`fees[${n}]`);return {asset:asset(x.asset,`fees[${n}].asset`),maxFee:uint(x.maxFee,`fees[${n}].maxFee`)};});distinct(fees.map(x=>assetKey(x.asset)),'intent fee assets');
  const agreements=array(root.agreements,8,'intent.agreements').map((v,n)=>{const x=exact(v,['id','programHash'],`agreements[${n}]`);return {id:id(x.id,`agreements[${n}].id`),programHash:hex(x.programHash,32,`agreements[${n}].programHash`)};});distinct(agreements.map(x=>x.id),'intent agreement identifiers');
  const required=array(root.requiredClaims,4,'intent.requiredClaims');if(canonical(required)!==canonical(REQUIRED))fail('MissingRequiredClaim','Mandatory claims and order are fixed.');
  if(array(root.extensions,0,'intent.extensions').length!==0)fail('UnsupportedExtension','Semantic extensions are unsupported.');
  const notBefore=uint(validity.notBefore,'notBefore'),expiresAt=uint(validity.expiresAt,'expiresAt');if(BigInt(notBefore)>=BigInt(expiresAt))fail('MalformedIntent','Intent validity interval must be nonempty.');
  return {version:'moriarty-intent/1',domain:domain(root.domain,'intent.domain'),principal:id(root.principal,'intent.principal'),nonce:uint(root.nonce,'intent.nonce'),validity:{notBefore,expiresAt},lifecycle:'atomic',authority,goals,fees,agreements,requiredClaims:[...REQUIRED],extensions:[]};
}

export async function hashIntent(value:unknown):Promise<string>{return hashCanonical({domain:'MORIARTY-OUTCOME-INTENT-v1',intent:readIntent(value)});}
function bytesToHex(value:ArrayBuffer):string{return Array.from(new Uint8Array(value),b=>b.toString(16).padStart(2,'0')).join('');}
function hexBytes(value:string):Uint8Array<ArrayBuffer>{return Uint8Array.from(value.match(/../g)!,x=>parseInt(x,16));}
export async function signIntent(value:IntentIR,keys:CryptoKeyPair):Promise<SignedIntent> {
  const intent=readIntent(value);
  const intentHash=await hashIntent(intent);
  const publicKey=bytesToHex(await crypto.subtle.exportKey('raw',keys.publicKey));
  const signedBytes=encoder.encode(canonical({domain:'MORIARTY-OUTCOME-SIGN-v1',intentHash}));
  const signature=bytesToHex(await crypto.subtle.sign('Ed25519',keys.privateKey,signedBytes));
  return {kind:'SignedOutcomeIntent',intent,intentHash,publicKey,signature};
}

export async function verifyIntent(value:unknown,trustValue:TrustContext,nowValue:string):Promise<Checked|Rejected> {
  try {
    const envelope=exact(decodeCanonical(canonical(value)),['kind','intent','intentHash','publicKey','signature'],'signed intent');
    if(envelope.kind!=='SignedOutcomeIntent')fail('MalformedIntent','Wrong signed intent kind.');
    const intent=readIntent(envelope.intent);
    const trust=exact(decodeCanonical(canonical(trustValue)),['domain','principal','publicKey'],'trust');
    const trustedDomain=domain(trust.domain,'trust.domain');
    const now=uint(nowValue,'now');
    if(canonical(intent.domain)!==canonical(trustedDomain))fail('WrongDomain','Intent domain is not trusted.');
    if(intent.principal!==id(trust.principal,'trust.principal'))fail('WrongPrincipal','Intent principal is not trusted.');
    const publicKey=hex(envelope.publicKey,32,'publicKey');
    if(publicKey!==hex(trust.publicKey,32,'trust.publicKey'))fail('WrongSigner','Envelope key is not trusted.');
    if(BigInt(now)<BigInt(intent.validity.notBefore))fail('NotYetValid','Intent is not yet valid.');
    if(BigInt(now)>=BigInt(intent.validity.expiresAt))fail('Expired','Intent has expired.');
    const intentHash=hex(envelope.intentHash,32,'intentHash');
    if(intentHash!==await hashIntent(intent))fail('IntentHashMismatch','Intent hash does not bind the intent.');
    const signature=hex(envelope.signature,64,'signature');
    const key=await crypto.subtle.importKey('raw',hexBytes(publicKey),'Ed25519',false,['verify']);
    const signedBytes=encoder.encode(canonical({domain:'MORIARTY-OUTCOME-SIGN-v1',intentHash}));
    if(!await crypto.subtle.verify('Ed25519',key,hexBytes(signature),signedBytes))fail('InvalidSignature','Ed25519 verification failed.');
    return {outcome:'checked'};
  } catch(error) {
    return rejected(error);
  }
}

function showAsset(value:AssetId):string {
  return `domain=${value.domain}, issuer=${value.issuer}, reference=${value.reference}, kind=${value.kind}`;
}

export function renderIntentSummary(value:unknown):string {
  const intent=readIntent(value);
  const lines=[
    `Outcome intent ${intent.version}`,
    `Domain: ${intent.domain.network} / ${intent.domain.deployment}`,
    `Principal: ${intent.principal}`,
    `Nonce: ${intent.nonce} (one-shot)`,
    `Valid: ${intent.validity.notBefore} inclusive, ${intent.validity.expiresAt} exclusive`,
    'Lifecycle: atomic; unused authority expires on commit',
    '',
    'Authority (aggregate gross debits, including fees):',
  ];
  for(const entry of intent.authority)lines.push(`Asset: ${showAsset(entry.asset)}`,`Aggregate gross debit cap: ${entry.maxDebit}`,`Recipients: ${entry.recipients.join(', ')||'(none)'}`);
  lines.push('','Net goals:');
  for(const goal of intent.goals)lines.push(`Asset: ${showAsset(goal.asset)}`,`Net credit goal: account=${goal.account}, minimum=${goal.minCredit}`);
  lines.push('','Fee caps:');
  for(const fee of intent.fees)lines.push(`Asset: ${showAsset(fee.asset)}`,`Fee cap: ${fee.maxFee} (included in gross debit budget)`);
  lines.push('','Allowed agreements:');
  for(const agreement of intent.agreements)lines.push(`Agreement: id=${agreement.id}`,`Program hash: ${agreement.programHash}`);
  lines.push('',`Mandatory claims: ${intent.requiredClaims.join(', ')}`,'Unsupported semantic extensions: none');
  return lines.join('\n');
}

function balances(value:unknown,label:string):{rows:Balance[];map:Map<string,bigint>} {
  const rows=array(decodeCanonical(canonical(value)),32,label).map((item,n)=>{
    const x=exact(item,['asset','account','amount'],`${label}[${n}]`);
    return {asset:asset(x.asset,`${label}[${n}].asset`),account:id(x.account,`${label}[${n}].account`),amount:uint(x.amount,`${label}[${n}].amount`)};
  });
  const keys=rows.map(row=>assetKey(row.asset)+'|'+row.account);
  distinct(keys,label);
  return {rows,map:new Map(rows.map((row,n)=>[keys[n]!,BigInt(row.amount)]))};
}

function readEffects(value:unknown):AssetEffect[] {
  return array(decodeCanonical(canonical(value)),16,'effects').map((item,n)=>{
    const x=exact(item,['kind','asset','from','to','amount'],`effects[${n}]`);
    if(x.kind!=='Transfer'&&x.kind!=='Fee')fail('MalformedTrace',`effects[${n}].kind is unsupported.`);
    return {kind:x.kind,asset:asset(x.asset,`effects[${n}].asset`),from:id(x.from,`effects[${n}].from`),to:id(x.to,`effects[${n}].to`),amount:uint(x.amount,`effects[${n}].amount`)};
  });
}

export function checkOutcomeTrace(intentValue:unknown,beforeValue:unknown,effectsValue:unknown,afterValue:unknown):TraceChecked|Rejected {
  try {
    const intent=readIntent(intentValue);
    const before=balances(beforeValue,'before');
    const after=balances(afterValue,'after');
    if(before.map.size!==after.map.size||[...before.map.keys()].some(key=>!after.map.has(key)))fail('IncompleteAccounting','Before and after must contain the same complete balance cells.');
    for(const goal of intent.goals) {
      const key=assetKey(goal.asset)+'|'+goal.account;
      if(!before.map.has(key)||!after.map.has(key))fail('IncompleteAccounting','Every goal must have before and after balance cells.');
    }

    const current=new Map(before.map);
    const gross=new Map<string,bigint>();
    const feeTotals=new Map<string,bigint>();
    for(const effect of readEffects(effectsValue)) {
      const assetIdentity=assetKey(effect.asset);
      const from=assetIdentity+'|'+effect.from;
      const to=assetIdentity+'|'+effect.to;
      const amount=BigInt(effect.amount);
      if(effect.kind==='Fee'&&effect.from!==intent.principal)fail('UnauthorizedFee','Fees may debit only the signed principal.');
      if(!current.has(from)||!current.has(to))fail('IncompleteAccounting','Every effect endpoint must have a balance cell.');
      const source=current.get(from)!;
      if(source<amount)fail('InsufficientPrefixBalance','An effect prefix overdraws its source.');
      current.set(from,source-amount);
      current.set(to,add(current.get(to)!,amount,'balance'));
      if(effect.from!==intent.principal)continue;
      gross.set(assetIdentity,add(gross.get(assetIdentity)??0n,amount,'gross debit'));
      const authority=intent.authority.find(entry=>assetKey(entry.asset)===assetIdentity);
      if(!authority)fail('UnauthorizedDebit','Principal debit lacks authority.');
      if(!authority.recipients.includes(effect.to))fail('UnauthorizedRecipient','Principal debit recipient is not permitted.');
      if(effect.kind==='Fee')feeTotals.set(assetIdentity,add(feeTotals.get(assetIdentity)??0n,amount,'fees'));
    }
    for(const [key,amount] of current)if(amount!==after.map.get(key))fail('AccountingMismatch','After balances do not reproduce the complete effect trace.');
    for(const authority of intent.authority)if((gross.get(assetKey(authority.asset))??0n)>BigInt(authority.maxDebit))fail('GrossDebitExceeded','Aggregate gross debit exceeds authority.');
    for(const [key,amount] of feeTotals) {
      const cap=intent.fees.find(entry=>assetKey(entry.asset)===key);
      if(!cap||amount>BigInt(cap.maxFee))fail('FeeExceeded','Aggregate fee exceeds its cap.');
    }
    const netCredits=intent.goals.map(goal=>{
      const key=assetKey(goal.asset)+'|'+goal.account;
      const delta=after.map.get(key)!-before.map.get(key)!;
      if(delta<BigInt(goal.minCredit))fail('GoalNotMet','Net final credit is below the promised minimum.');
      return {asset:goal.asset,account:goal.account,amount:delta.toString()};
    });
    const grossDebits=intent.authority
      .filter(entry=>(gross.get(assetKey(entry.asset))??0n)>0n)
      .map(entry=>({asset:entry.asset,amount:(gross.get(assetKey(entry.asset))??0n).toString()}));
    return {outcome:'checked',grossDebits,netCredits};
  } catch(error) {
    return rejected(error);
  }
}
