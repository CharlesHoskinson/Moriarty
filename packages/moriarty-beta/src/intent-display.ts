/** Pure review of signed owner intents. No I/O, no process, no keys.
 *  Every string that reaches a terminal from the signed-intent commands is printable ASCII plus newline.
 *  Rows are built from the statement the native verifier returned after the beta source comparison. */
import {createHash} from 'node:crypto';
import {LocalError} from './json.ts';
import type {OwnerIntentPreparation,SignedLocalPreparation} from './auth.ts';

type Rec=Record<string,unknown>;
export type ReviewContext={sourceSha256:string;scenarioSha256:string;scenario?:unknown};
export type StatementLeaf={path:string;value:unknown};
const DEC=/^(0|[1-9][0-9]*)$/;
const shape=(message:string):never=>{throw new LocalError('BETA_DISPLAY_SHAPE',message);};
const sha256=(bytes:Buffer):string=>createHash('sha256').update(bytes).digest('hex');
const isRec=(v:unknown):v is Rec=>!!v&&typeof v==='object'&&!Array.isArray(v);
function rec(v:unknown,what:string):Rec{return isRec(v)?v:shape(`Expected ${what}`);}
function str(v:unknown,what:string):string{return typeof v==='string'?v:shape(`Expected text for ${what}`);}

/** Escape-by-default: printable ASCII passes; every other scalar becomes \u{HEX}. */
function escapeBody(value:string,quoted:boolean):string{
 let out='';
 for(const ch of value){
  const c=ch.codePointAt(0)!;
  if(ch==='\\'||(quoted&&ch==='"'))out+='\\'+ch;
  else if(c>=0x20&&c<=0x7e)out+=ch;
  else out+=`\\u{${c.toString(16).toUpperCase()}}`;
 }
 return out;
}
export const escapeText=(value:string):string=>`"${escapeBody(value,true)}"`;
/** Unquoted form for messages. A literal backslash is doubled so output cannot imitate an escape. */
export const escapeAscii=(value:string):string=>escapeBody(value,false);
/** JSON text whose parsed value is unchanged and whose bytes are printable ASCII plus newline. */
export function asciiJson(value:unknown,space=2):string{
 return JSON.stringify(value,null,space).replace(/[^\x20-\x7e\n]/g,c=>'\\u'+c.charCodeAt(0).toString(16).padStart(4,'0'));
}
/** Derived decimal for display only. BigInt-free string arithmetic; canonical input or an error. */
export function formatAtoms(atoms:string,scale:number|string):string{
 const places=typeof scale==='string'?(DEC.test(scale)&&scale.length<=2?Number(scale):NaN):scale;
 if(typeof atoms!=='string'||!DEC.test(atoms)||!Number.isInteger(places)||places<0||places>18)throw new LocalError('BETA_INTENT_AMOUNT_FORMAT','Amounts are canonical unsigned decimal atoms with scale 0..18');
 const digits=atoms.padStart(places+1,'0');
 return places===0?digits:`${digits.slice(0,-places)}.${digits.slice(-places)}`;
}

type Kind='text'|'id'|'hex'|'round'|'amount'|'scale'|'symbol'|'empty'|'source'|'owner';
type Label={kind:Kind;note?:string};
/** Closed table keyed by statement path, in display order. A leaf without an entry cannot be shown, so it fails. */
export const STATEMENT_LABELS:Record<string,Label>={
 'profile':{kind:'text',note:'signed profile identifier'},
 'core':{kind:'text'},
 'sourceProfile':{kind:'text'},
 'authoringProfile':{kind:'text'},
 'agreementId':{kind:'id',note:'symbolic agreement ID from the source'},
 'actionName':{kind:'id',note:'action selected with --action'},
 'selectedActionId':{kind:'id'},
 'sourceSha256':{kind:'source'},
 'ownerProgramSha256':{kind:'owner'},
 'domain/id':{kind:'text',note:'domain identity claimed by the source'},
 'domain/chain':{kind:'text'},
 'domain/network':{kind:'text'},
 'asset/id':{kind:'text',note:'asset identity; amounts below are atoms of this asset'},
 'asset/representation':{kind:'text'},
 'asset/scale':{kind:'scale',note:'decimal places used only to display atoms'},
 'asset/symbol':{kind:'symbol'},
 'signature/scheme':{kind:'text',note:'must match the key; fixed by the signer and signed'},
 'signature/publicKeyHex':{kind:'hex'},
 'signature/keyRef':{kind:'text'},
 'signature/framing':{kind:'text'},
 'intent/version':{kind:'text'},
 'intent/advertisedSourceHash':{kind:'text',note:'author claim, not checked against any hash'},
 'intent/advertisedPolicyDigest':{kind:'text',note:'author claim, not checked against any digest'},
 'intent/signer':{kind:'id',note:'symbolic account ID, not an address or a key'},
 'intent/nonce':{kind:'text',note:'opaque replay label; freshness is not checked here'},
 'intent/preHead':{kind:'text',note:'opaque head label; the current head is not checked here'},
 'intent/notBefore':{kind:'round',note:'domain round number, inclusive; not wall-clock time and not money'},
 'intent/notAfter':{kind:'round',note:'domain round number, inclusive; not wall-clock time and not money'},
 'intent/grossCap':{kind:'amount',note:'includes the fee'},
 'intent/feeCap':{kind:'amount'},
 'intent/netFloor':{kind:'amount'},
 'intent/failure':{kind:'text'},
 'intent/observations':{kind:'empty'},
 'intent/disclosures':{kind:'empty'},
 'intent/retainedEffects':{kind:'empty'},
 'intent/retainedDuties':{kind:'empty'},
 'intent/delegation':{kind:'text'},
 'intent/recovery':{kind:'text'},
 'intent/operation/kind':{kind:'text'},
 'intent/operation/from':{kind:'id',note:'symbolic account ID'},
 'intent/operation/recipient':{kind:'id',note:'symbolic account ID'},
 'intent/operation/feeRecipient':{kind:'id',note:'symbolic account ID'},
 'intent/operation/payer':{kind:'id',note:'symbolic account ID'},
 'intent/operation/obligationId':{kind:'id',note:'symbolic obligation ID'},
 'intent/operation/amount':{kind:'amount'},
 'intent/operation/fee':{kind:'amount'},
 'intent/operation/conversion':{kind:'text'},
};
const OPERATION_FIELDS:Record<string,string[]>={Transfer:['from','recipient','feeRecipient','amount','fee'],Repay:['payer','obligationId','amount','conversion']};

/** Every leaf of a closed statement; empty arrays are leaves, so absent and empty stay distinguishable. */
export function signedLeaves(value:unknown,prefix=''):StatementLeaf[]{
 if(Array.isArray(value))return value.length?value.flatMap((item,i)=>signedLeaves(item,`${prefix}${prefix?'/':''}${i}`)):[{path:prefix,value}];
 if(isRec(value))return Object.keys(value).flatMap(key=>signedLeaves(value[key],`${prefix}${prefix?'/':''}${key}`));
 return [{path:prefix,value}];
}
function requiredPaths(statement:Rec):string[]{
 const kind=(statement.intent as Rec|undefined)?.operation&&((statement.intent as Rec).operation as Rec).kind;
 if(typeof kind!=='string'||!Object.hasOwn(OPERATION_FIELDS,kind))return shape('Unknown operation kind');
 return Object.keys(STATEMENT_LABELS).filter(p=>!p.startsWith('intent/operation/')||p==='intent/operation/kind'||OPERATION_FIELDS[kind].includes(p.slice('intent/operation/'.length)));
}
function leafMap(statement:Rec):Map<string,unknown>{
 const required=requiredPaths(statement),leaves=signedLeaves(statement),map=new Map(leaves.map(l=>[l.path,l.value]));
 if(map.size!==leaves.length)shape('Duplicate statement paths');
 for(const path of map.keys())if(!required.includes(path))shape(`Unlabeled statement leaf ${escapeAscii(path)}`);
 for(const path of required)if(!map.has(path))shape(`Missing statement leaf ${path}`);
 return map;
}

const amountText=(atoms:unknown,asset:string,scale:string):string=>{
 const a=str(atoms,'amount');
 return `${a} atoms | asset ${escapeText(asset)} | scale ${scale} | ${formatAtoms(a,scale)} (display only)`;
};
const hexOr=(v:string)=>/^[0-9a-f]*$/.test(v)?v:escapeText(v);
function statementRows(statement:Rec,context:ReviewContext):{rows:string[];nonAscii:boolean}{
 const leaves=leafMap(statement),asset=rec(statement.asset,'asset'),assetId=str(asset.id,'asset.id'),scale=str(asset.scale,'asset.scale');
 formatAtoms('0',scale);
 const signer=escapeText(str((statement.intent as Rec).signer,'signer')),keyRef=escapeText(str((statement.signature as Rec).keyRef,'keyRef'));
 let nonAscii=false;
 const rows=[...leaves].sort((a,b)=>Object.keys(STATEMENT_LABELS).indexOf(a[0])-Object.keys(STATEMENT_LABELS).indexOf(b[0])).map(([path,value])=>{
  const label=STATEMENT_LABELS[path];let text:string;
  if(label.kind==='empty')text=Array.isArray(value)&&value.length===0?'none':shape(`Expected empty list at ${path}`);
  else if(label.kind==='symbol')text=value===null?'none (no symbol committed)  label only; not identity and not a conversion permission':`${escapeText(str(value,path))}  label only; not identity and not a conversion permission`;
  else{
   const v=str(value,path);if(/[^\x20-\x7e]/.test(v))nonAscii=true;
   if(label.kind==='amount')text=amountText(v,assetId,scale);
   else if(label.kind==='round')text=DEC.test(v)?`${v}  ${label.note}`:escapeText(v);
   else if(label.kind==='scale')text=`${escapeText(v)}  ${label.note}`;
   else if(label.kind==='hex'){const bytes=v.length/2;text=`${hexOr(v)}  (${bytes} bytes) supplied with --public-key; checked only as a valid key for this scheme. Nothing here links it to account ${signer} or keyRef ${keyRef}`;}
   else if(label.kind==='source')text=`${hexOr(v)}  statement value; computed from the exact source file bytes: ${context.sourceSha256} (${v===context.sourceSha256?'match':'MISMATCH'})`;
   else if(label.kind==='owner')text=`${hexOr(v)}  digest of the owner-term projection; not a compiled-program hash`;
   else text=`${escapeText(v)}${path==='signature/keyRef'?'  symbolic label copied from the source key:; not a key, and nothing here resolves it':path==='signature/framing'?'  signed metadata; no other framing is tried':label.note?`  ${label.note}`:''}`;
  }
  return `  [${path}] ${text}`;
 });
 return {rows,nonAscii};
}
const NOT_ESTABLISHED=[
 'owner authority: no link from the key to the account or to the keyRef is checked',
 'key-to-account binding',
 'current state: balances, allowance and head come from an unsigned scenario',
 'replay freshness beyond the local stipulation',
 'proof',
 'ledger acceptance',
 'settlement',
];
function notEstablished(state:Rec,signer:string):string[]{
 return ['Not established by this review',...NOT_ESTABLISHED.map(x=>`  ${x.replace('the account',`account ${signer}`)}`),
  `  authority_valid ${asciiJson(state.authority_valid??null,0)}, snapshot_membership_valid ${asciiJson(state.snapshot_membership_valid??null,0)}, transition_valid ${asciiJson(state.transition_valid??null,0)}, ledger_accepted ${asciiJson(state.ledger_accepted,0)}`];
}
function scenarioLines(context:ReviewContext,asset:Rec):string[]{
 const out=['Unsigned scenario context (not part of the signed statement; every value below can change without touching a signature)',
  `  scenario sha256 ${context.scenarioSha256}  computed from the scenario file bytes (unsigned)`];
 const s=context.scenario;if(!isRec(s))return out;
 const assetId=str(asset.id,'asset.id'),scale=str(asset.scale,'asset.scale');
 const shown=(v:unknown)=>typeof v==='string'?escapeText(v):escapeText(JSON.stringify(v)??'undefined');
 const amount=(v:unknown)=>{try{return amountText(v as string,assetId,scale);}catch{return shown(v);}};
 const facts=['head','predecessor','round','post_head','replay','work_remaining','work_spent'].filter(k=>Object.hasOwn(s,k)).map(k=>`${k} ${shown(s[k])}`);
 if(facts.length)out.push(`  ${facts.join(', ')}`);
 if(Array.isArray(s.balances))for(const b of s.balances)if(isRec(b))out.push(`  balance ${shown(b.account)} ${amount(b.amount)}`);
 if(isRec(s.allowance))out.push(`  allowance owner ${shown(s.allowance.owner)} remaining ${amount(s.allowance.remaining)}; spent ${amount(s.allowance.spent)}`);
 if(isRec(s.obligation))out.push(`  obligation ${shown(s.obligation.id)} principal ${amount(s.obligation.principal)}; accrued ${amount(s.obligation.accrued)}; outstanding ${amount(s.obligation.outstanding)}`);
 return out;
}
function frameLines(r:Rec,statement:Rec):{lines:string[];message:number}{
 const frameHex=str(r.frame_hex,'frame_hex'),messageHex=str(r.signing_message_hex,'signing_message_hex'),claimed=str(r.frame_sha256,'frame_sha256');
 if(!/^(?:[0-9a-f]{2})+$/.test(frameHex)||!/^(?:[0-9a-f]{2})+$/.test(messageHex))shape('Frame bytes are not hex');
 const frame=Buffer.from(frameHex,'hex'),computed=sha256(frame),framing=str((statement.signature as Rec).framing,'framing');
 const lines=[`  frame ${frame.length} bytes`,`  frame sha256 claimed by native Rust: ${escapeAscii(claimed)}`,`  frame sha256 computed here: ${computed} (${claimed===computed?'match':'MISMATCH'})`];
 if(framing==='midnight-sign-data')lines.push(`  signing message ${messageHex.length/2} bytes: the ASCII prefix midnight_signed_message:${frame.length}: followed by the frame`);
 else lines.push(`  signing message ${messageHex.length/2} bytes: raw framing, the signing message is the frame itself`);
 lines.push('  Framing is signed metadata; no other framing is tried. Live wallet compatibility is not verified.');
 return {lines,message:messageHex.length/2};
}
const warningLine='  Warning: contains non-ASCII or control text; compare the escaped form (\\u{HEX} marks one scalar).';

export function renderOwnerIntentReview(result:OwnerIntentPreparation,context:ReviewContext):string{
 const r=rec(result,'owner intent result'),statement=rec(r.statement,'statement');
 if(r.status!=='OwnerIntentPrepared')shape('Not an owner intent result');
 const {rows,nonAscii}=statementRows(statement,context),frame=frameLines(r,statement),asset=rec(statement.asset,'asset');
 const signer=escapeText(str((statement.intent as Rec).signer,'signer'));
 const out=[`OwnerIntentPrepared (UNSIGNED)  ${escapeAscii(str(statement.profile,'profile'))}`,
  'Nothing was signed. This tool holds no keys and has no signing command.',
  'Signed owner terms (every leaf of the statement native Rust returned; it equals the terms derived from this source):',...rows];
 if(nonAscii)out.push(warningLine);
 out.push('Signing frame (native Rust bytes, re-hashed here)',...frame.lines,
  'Sign the DECODED bytes of signing_message_hex: not the hex text, not the sha256, and with no other framing.',
  ...scenarioLines(context,asset),...notEstablished(r,signer),
  `Signing message hex (${frame.message} bytes; decode before signing):`,`  ${escapeAscii(str(r.signing_message_hex,'signing_message_hex'))}`);
 return out.join('\n')+'\n';
}

function effectLine(effect:unknown,index:number,statementAsset:Rec):string{
 const e=rec(effect,'effect'),assetId=typeof e.asset==='string'?e.asset:str(statementAsset.id,'asset.id'),scale=str(statementAsset.scale,'asset.scale');
 const parts=Object.entries(e).filter(([k])=>k!=='kind'&&k!=='asset').map(([k,v])=>['amount','principal','accrued','outstanding'].includes(k)?`${k} ${amountText(v,assetId,scale)}`:`${k} ${escapeText(typeof v==='string'?v:JSON.stringify(v))}`);
 return `  [${index}] ${escapeAscii(str(e.kind,'effect kind'))} ${parts.join(' ')}`;
}
function postLines(post:Rec,asset:Rec):string[]{
 const assetId=str(asset.id,'asset.id'),scale=str(asset.scale,'asset.scale'),q=(v:unknown)=>escapeText(String(v)),out:string[]=[];
 out.push(`  post head ${q(post.head)} round ${q(post.round)} workRemaining ${q(post.workRemaining)} workSpent ${q(post.workSpent)}`);
 for(const b of (post.balances as Rec[]??[]))out.push(`  post balance ${q(b.account)} ${amountText(b.amount,assetId,scale)}`);
 for(const a of (post.allowances as Rec[]??[]))out.push(`  post allowance owner ${q(a.owner)} remaining ${amountText(a.remaining,assetId,scale)}; spent ${amountText(a.spent,assetId,scale)}`);
 for(const o of (post.obligations as Rec[]??[]))out.push(`  post obligation ${q(o.id)} principal ${amountText(o.principal,assetId,scale)}; accrued ${amountText(o.accrued,assetId,scale)}; outstanding ${amountText(o.outstanding,assetId,scale)}; status ${q(o.status)}`);
 for(const k of (post.consumedReplay as unknown[]??[]))out.push(`  post consumedReplay ${q(k)}`);
 return out;
}
function localLines(local:unknown,asset:Rec):string[]{
 if(local===null||local===undefined)return ['Local result NotRun','  Local preparation did not run because the signature was not valid for this frame.'];
 const l=rec(local,'local result'),result=rec(l.result,'local core result'),out=[`Local result ${escapeAscii(str(l.status,'local status'))} (local stipulation against the unsigned scenario)`];
 if(l.status==='PreparedUnqualified'){
  const candidate=rec(result.candidate,'candidate');
  out.push('  Effects (candidate, unqualified):');
  (candidate.effects as unknown[]).forEach((e,i)=>out.push('  '+effectLine(e,i,asset)));
  out.push(...postLines(rec(candidate.candidatePost,'candidatePost'),asset));
 }else if(isRec(result.rejection)){
  const j=result.rejection;
  out.push(`  Core rejected the candidate: judgment ${escapeText(String(j.judgment))} code ${escapeAscii(String(j.code))}.`,'  No effects were published and no post-state was published.');
 }else out.push('  '+escapeAscii(asciiJson(result,0).slice(0,400)));
 return out;
}

export function renderVerificationReview(result:SignedLocalPreparation,context:ReviewContext):string{
 const r=rec(result,'verification result'),receipt=rec(r.signature,'signature receipt'),statement=rec(receipt.statement,'statement'),asset=rec(statement.asset,'asset');
 const sig=rec(statement.signature,'signature'),valid=receipt.signature_valid,signer=escapeText(str((statement.intent as Rec).signer,'signer'));
 if(typeof valid!=='boolean')shape('Missing signature_valid');
 const {rows,nonAscii}=statementRows(statement,context),frame=frameLines(receipt,statement);
 const out=[`${escapeAscii(str(r.status,'status'))}  (signature checked by native Rust; owner authority NOT checked)`,
  `Signature check (native Rust, scheme ${escapeAscii(str(sig.scheme,'scheme'))}, framing ${escapeAscii(str(sig.framing,'framing'))})`,
  `  signature_valid ${valid}`,
  valid?`  Meaning: the key used signed these exact frame bytes. This does not show that the key controls account ${signer}, that the terms are current or unused, or that anything settled.`
   :'  Meaning: the signature does not verify for this frame under the stated key, scheme and framing. The other framing was not tried.',
  ...frame.lines,
  'Source match',
  `  source matched ${r.sourceMatched===true?'yes':'NO'} (sourceMatched ${asciiJson(r.sourceMatched,0)}): the artifact statement equals the terms derived from this source and action`,
  `  source sha256 claimed by artifact: ${escapeAscii(str(statement.sourceSha256,'sourceSha256'))}; computed from this file: ${context.sourceSha256}`,
  'Signed owner terms (every leaf of the matched statement):',...rows];
 if(nonAscii)out.push(warningLine);
 out.push(...localLines(r.local,asset));
 if(r.status==='SignedCoreRejected')out.push('  A valid signature does not make a candidate acceptable.');
 out.push('Checks as returned (local labels, not authority)');
 for(const key of ['keyAuthority','domainMapping','expiry','replay','financial','state','nativeProof','ledger'])if(Object.hasOwn(r,key))out.push(`  ${key} ${escapeAscii(String(r[key]))}`);
 out.push(`  requiredPremises ${(r.requiredPremises as unknown[]).map(x=>escapeAscii(String(x))).join(', ')}`,`  unverifiedBindings ${(r.unverifiedBindings as unknown[]).map(x=>escapeAscii(String(x))).join(', ')}`);
 out.push(...scenarioLines(context,asset),...notEstablished({...r,authority_valid:receipt.authority_valid,snapshot_membership_valid:receipt.snapshot_membership_valid,transition_valid:receipt.transition_valid},signer));
 return out.join('\n')+'\n';
}

const HINTS:Record<string,string>={
 BETA_SIGNATURE_SOURCE_MISMATCH:'The artifact statement differs from the terms this source, action and signing metadata produce. This says nothing yet about the signature. Restore the exact signed source bytes (formatting counts) or sign again.',
 BETA_CRYPTO_BINARY_UNAVAILABLE:'Build the verifier once (cargo build --release --locked --manifest-path experiments/midnight-crypto/Cargo.toml), copy the binary, and pass its absolute path with --crypto-binary. There is no PATH lookup, no build at run time and no fallback.',
 BETA_CRYPTO_BINARY_PATH:'--crypto-binary must be an absolute path to the native verifier. There is no PATH lookup and no fallback.',
 BETA_CRYPTO_TIMEOUT:'The native verifier did not finish in time. Nothing was judged.',
 BETA_CRYPTO_OUTPUT_BOUND:'The native verifier wrote more than the allowed output. Nothing was judged.',
 BETA_CRYPTO_INPUT_BOUND:'The request to the native verifier exceeds its input bound. Nothing was judged.',
 BETA_CRYPTO_PROCESS:'The native verifier process failed while reading input. Nothing was judged.',
 BETA_CRYPTO_RESPONSE:'The native verifier answered outside the closed protocol (unknown field, hash mismatch, authority claim or exit/status mismatch). The answer was discarded.',
 BETA_SIGNATURE_SCHEMA:'The signature artifact or signing metadata is malformed. Expected {"statement":{...},"signatureHex":"<128 lowercase hex>"} and a key that matches the scheme.',
 BETA_AUTH_SOURCE:'The source or scenario was rejected before any owner terms were derived.',
};
/** `judgment`: the tool reached a negative finding (exit 1). `unable`: it could not judge at all (exit 2). */
export function renderErrorReview(error:{code:string;message:string},outcome:'judgment'|'unable',extra:readonly string[]=[]):string{
 const hint=Object.hasOwn(HINTS,error.code)?HINTS[error.code]:error.code.startsWith('BETA_JSON_')?'The JSON text was rejected by the bounded duplicate-safe parser.':null;
 const out=[`FormationRejected  ${escapeAscii(error.code)}`,
  outcome==='unable'?'  Outcome: no judgment was made. Nothing was prepared or checked; signature validity is unknown, not false.':'  Outcome: rejected. No effects were published.',
  `  Detail: ${escapeAscii(error.message)}`];
 if(hint)out.push(`  Hint: ${hint}`);
 out.push(...extra.map(escapeAscii));
 return out.join('\n')+'\n';
}
/** First leaf, in table order, where an artifact statement differs from the source-derived statement. */
export function renderStatementDifference(claimed:unknown,computed:unknown):string[]{
 if(!isRec(claimed)||!isRec(computed))return [];
 const a=new Map(signedLeaves(claimed).map(l=>[l.path,l.value])),b=new Map(signedLeaves(computed).map(l=>[l.path,l.value]));
 const order=Object.keys(STATEMENT_LABELS),paths=[...new Set([...a.keys(),...b.keys()])].sort((x,y)=>{const i=order.indexOf(x),j=order.indexOf(y);return (i<0?1e6:i)-(j<0?1e6:j)||(x<y?-1:1);});
 const show=(m:Map<string,unknown>,p:string)=>m.has(p)?escapeAscii(asciiJson(m.get(p),0)):'(absent)';
 const path=paths.find(p=>asciiJson(a.get(p),0)!==asciiJson(b.get(p),0)||a.has(p)!==b.has(p));
 if(path===undefined)return [];
 return [`  First difference /statement/${escapeAscii(path)}`,`  claimed by artifact: ${show(a,path)}; computed from this source and action: ${show(b,path)}`];
}
