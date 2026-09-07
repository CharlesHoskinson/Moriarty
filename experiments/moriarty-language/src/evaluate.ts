import {compile} from './frontend.ts';
import {types as nodeTypes} from 'node:util';
import {registeredBounds} from './registered-bounds.ts';
import {canonicalEncode, canonicalDecode, checkEncoding, measureEncoding, hashDomain, FrontendError, scalarText} from './codec.ts';
import type {BoundProgram, CoreExpression, EncodingLimits, LocalValue, NamedStoredValue, NamedType, SemanticManifest, Span, StoredValue} from './types.ts';
import type * as R from './runtime-types.ts';
export type * from './runtime-types.ts';
const PROFILE='moriarty-bounded-atomic/1' as const;
const MAX=(1n<<128n)-1n;
const ZERO='0'.repeat(64);
const UNKNOWN={startByte:'0',endByte:'0'};
const clone=<T>(v:T):T=>structuredClone(v);
const same=(a:unknown,b:unknown)=>canonicalEncode(a)===canonicalEncode(b);
export const hash=(kind:string,value:unknown)=>hashDomain(`MORIARTY-${kind}-bounded-atomic/1`,value);
class RuntimeError extends Error { code:string;stage:number;span:Span;constructor(code:string,stage:number,span=UNKNOWN,message=code){super(message);this.code=code;this.stage=stage;this.span=span;} }
function need(ok:unknown,code='INPUT_SCHEMA',stage=9,span=UNKNOWN,message=code):asserts ok {if(!ok)throw new RuntimeError(code,stage,span,message);}
function uint(v:unknown):bigint {need(typeof v==='string'&&/^(0|[1-9][0-9]*)$/.test(v)&&v.length<=39);const n=BigInt(v);need(n<=MAX);return n;}
export function checkedUInt128(n:bigint,code='ARITHMETIC_OVERFLOW',stage=11,span=UNKNOWN):bigint{need(n>=0n&&n<=MAX,code,stage,span);return n;}
function rejected(e:unknown,p=ZERO):R.Rejected {let error=e instanceof RuntimeError?e:new RuntimeError('INPUT_SCHEMA',9);if(e instanceof FrontendError)error=new RuntimeError(e.code==='SOURCE_MAP'?'SOURCE_MAP':'PROGRAM_ENCODING',8,e.primarySpan);return {diagnostics:[{code:error.code,message:error.message,primarySpan:error.span,relatedSpans:[],stage:String(error.stage)}],outcome:'Rejected',profile:PROFILE,programHash:p,schemaVersion:'moriarty-result/1'};}
function enc(v:unknown,limits:EncodingLimits,code:string,stage:number){try{checkEncoding(v,limits,code);}catch{throw new RuntimeError(code,stage);}}
/** Admit the original JS graph before any clone, property read, callback or
 * canonical encoder. Descriptor values are copied exactly once; accessors and
 * proxies never run. Limits bound traversal, allocation and string encoding.
 * Shared references expand as JSON does; only ancestor cycles are forbidden.
 */
function inertSnapshot<T>(input:T,limits:EncodingLimits,schemaCode='INPUT_SCHEMA',boundsCode='INPUT_BOUNDS',stage=9):T {
 const active=new Set<object>();let nodes=0,bytes=0;
 const bad=(ok:unknown,code=schemaCode)=>need(ok,code,stage);
 const add=(n:number)=>{bytes+=n;bad(bytes<=limits.utf8Bytes,boundsCode);};
 function string(s:string):void{bad(s.length<=limits.textJavascriptCodeUnits,boundsCode);bad(scalarText(s));bad(new TextEncoder().encode(s).length<=limits.textUtf8Bytes,boundsCode);add(new TextEncoder().encode(JSON.stringify(s)).length);}
 function visit(v:unknown,depth:number):unknown {
  bad(++nodes<=limits.decodedNodes&&depth<=limits.decodedDepthRootZero,boundsCode);
  if(typeof v==='string'){string(v);return v;}
  if(typeof v==='boolean'){add(v?4:5);return v;}
  bad(v!==null&&typeof v==='object');const obj=v as object;
  bad(!nodeTypes.isProxy(obj));bad(!active.has(obj));active.add(obj);
  const isArray=Array.isArray(obj),proto=Object.getPrototypeOf(obj);
  bad(isArray?proto===Array.prototype:proto===Object.prototype||proto===null);
  const keys=Reflect.ownKeys(obj);bad(keys.every(k=>typeof k==='string'));
  let output:unknown;
  if(isArray){
   const lengthDescriptor=Object.getOwnPropertyDescriptor(obj,'length');bad(lengthDescriptor&&Object.hasOwn(lengthDescriptor,'value'));const length=lengthDescriptor!.value;
   bad(Number.isSafeInteger(length)&&length>=0);bad(length<=limits.arrayLength,boundsCode);bad(keys.length===length+1);
   const values:unknown[]=[];add(2+Math.max(0,length-1));
   for(let j=0;j<length;j++){const d=Object.getOwnPropertyDescriptor(obj,String(j));bad(d&&d.enumerable&&Object.hasOwn(d,'value'));values.push(visit(d!.value,depth+1));}
   output=Object.freeze(values);
  }else{
   bad(keys.length<=limits.keysPerRecord,boundsCode);const value:Record<string,unknown>={};add(2+Math.max(0,keys.length-1));
   for(const key of keys as string[]){bad(/^[\x20-\x7e]+$/.test(key));const d=Object.getOwnPropertyDescriptor(obj,key);bad(d&&d.enumerable&&Object.hasOwn(d,'value'));string(key);add(1);Object.defineProperty(value,key,{value:visit(d!.value,depth+1),enumerable:true});}
   output=Object.freeze(value);
  }
  active.delete(obj);return output;
 }
 return visit(input,0) as T;
}

// Closed schemas apply before any semantic access. JSON numbers/null and unknown
// members cannot become trusted merely through a TypeScript type assertion.
type Validator=(v:any)=>void;
const textValue:Validator=v=>need(typeof v==='string'&&scalarText(v)&&new TextEncoder().encode(v).length<=256);
const digest:Validator=v=>need(typeof v==='string'&&/^[0-9a-f]{64}$/.test(v));
const boolean:Validator=v=>need(typeof v==='boolean');
const literal=(s:string):Validator=>v=>need(v===s);
const oneOf=(...s:string[]):Validator=>v=>need(s.includes(v));
const array=(f:Validator):Validator=>v=>{need(Array.isArray(v));v.forEach(f);};
function record(fields:Record<string,Validator>):Validator{return v=>{need(v&&typeof v==='object'&&!Array.isArray(v));need(same(Object.keys(v).sort(),Object.keys(fields).sort()));for(const k of Object.keys(fields))fields[k](v[k]);};}
const keywords=new Set('agreement profile lifetime horizon unit const state observation settlement asset quantum policy targets write derivation rounding remainder comparison proof none floor status episode closed_when remaining_notional no_remaining_notional effect action guard let set emit UInt128 Text Amount Transfer Fee DueCreated DueSettled uint text amount true false remaining floor_div arg obs not and or reserve for'.split(' '));
const identifier:Validator=v=>{textValue(v);need(!keywords.has(v));need(/^[A-Za-z][A-Za-z0-9_]{0,63}$/.test(v)&&!['constructor','prototype','__proto__'].includes(v));};
const value:Validator=v=>{need(v&&typeof v==='object');if(v.tag==='Text')record({tag:literal('Text'),value:textValue})(v);else if(v.tag==='UInt128')record({tag:literal('UInt128'),value:uint})(v);else record({tag:literal('Amount'),value:uint,unit:identifier})(v);};
const amountValue:Validator=v=>{value(v);need(v.tag==='Amount');};
const named=array(record({name:identifier,value}));
const boundsRef=record({boundsHash:digest,registryId:literal('moriarty-bounds/1')});
const programRefSchema=record({bounds:boundsRef,coreVersion:literal('moriarty-core/1'),profile:literal(PROFILE),programHash:digest,schemaVersion:literal('moriarty-program-ref/1'),sourceHash:digest});
const domain=record({deployment:textValue,network:textValue});
const claims=array(record({claimId:textValue,kind:oneOf('ContractProperty','IntentRefinement','TransitionValidity','PredecessorHistory')}));
const actionSchema=record({arguments:named,name:identifier,schemaVersion:literal('moriarty-action/1')});
const obligation=record({amount:amountValue,creditor:textValue,debtor:textValue,denomination:textValue,dueId:textValue,status:oneOf('Outstanding','Settled')});
const remainingNotional:Validator=v=>v?.tag==='Amount'?record({amount:amountValue,tag:literal('Amount')})(v):record({tag:literal('NotApplicable')})(v);
const stateSchema=record({body:record({agreementStatus:oneOf('Outstanding','NoOutstanding'),episodeStatus:oneOf('Open','Closed'),genesisHash:digest,instanceId:textValue,obligations:array(obligation),profile:literal(PROFILE),programHash:digest,remaining:uint,remainingNotional,revision:uint,schemaVersion:literal('moriarty-state-body/1'),values:named}),schemaVersion:literal('moriarty-state/1'),stateHash:digest});
const genesisSchema=record({body:record({bounds:boundsRef,domain,horizon:uint,initialState:named,instanceId:textValue,lifetime:uint,observationBindings:array(record({authenticationPolicy:textValue,name:identifier,provider:textValue})),principalBindings:array(record({actor:textValue,principal:textValue})),profile:literal(PROFILE),program:programRefSchema,requiredClaimRoot:digest,schemaVersion:literal('moriarty-genesis-body/1')}),genesisHash:digest,schemaVersion:literal('moriarty-genesis/1')});
const settlementSchema=record({asset:textValue,binding:identifier,ledgerAmount:uint,nominalAmount:amountValue,quantum:amountValue,unit:identifier});
const effectSchema:Validator=v=>{need(v&&typeof v==='object');const common={amount:amountValue,kind:literal(v.kind),ordinal:uint};if(v.kind==='Transfer'||v.kind==='Fee')record({...common,asset:textValue,from:textValue,to:textValue,settlement:settlementSchema})(v);else {need(v.kind==='DueCreated'||v.kind==='DueSettled');record({...common,creditor:textValue,debtor:textValue,denomination:textValue,dueId:textValue,...(v.kind==='DueSettled'?{asset:textValue,settlement:settlementSchema}:{})})(v);}};
const authoritySchema:Validator=v=>{
 const common={beforeStateHash:digest,domain,genesisHash:digest,instanceId:textValue,nonce:textValue,predecessors:array(digest),principal:textValue,program:programRefSchema,requiredClaimRoot:digest,requiredClaims:claims,validity:record({notBefore:uint,notAfterExclusive:uint})};
 need(v?.tag==='ExactPlan'||v?.tag==='IntentRefinement');
 const exact=v.tag==='ExactPlan';
 record({domain:literal(exact?'MORIARTY-SIGN-bounded-atomic/1':'MORIARTY-OUTCOME-bounded-atomic/1'),schemaVersion:literal('moriarty-authority/1'),signature:record({algorithm:textValue,bytes:(b:unknown)=>need(typeof b==='string'&&/^(?:[0-9a-f]{2})*$/.test(b)),keyId:textValue}),tag:literal(v.tag),statement:record({...common,...(exact?{action:actionSchema,exactEffects:array(record({effect:effectSchema})),exactWrites:array(record({field:identifier,value})),mode:literal('ExactPlan'),schemaVersion:literal('moriarty-exact-plan/1')}:{allowedActions:array(identifier),grossDebitCaps:array(record({actor:textValue,asset:textValue,maximumLedgerAmount:uint})),minimumNetCredits:array(record({actor:textValue,asset:textValue,minimumLedgerAmount:uint})),mode:literal('IntentRefinement'),permittedCalls:array(record({callee:textValue,selector:textValue})),permittedRecipients:array(textValue),schemaVersion:literal('moriarty-outcome-intent/1')})})})(v);
};
const localTypeSchema:Validator=v=>{need(v&&typeof v==='object');if(v.tag==='Quantity')record({tag:literal('Quantity'),unitVector:unitVectorSchema})(v);else if(v.tag==='Amount')record({tag:literal('Amount'),unit:identifier})(v);else record({tag:oneOf('UInt128','Text','Bool')})(v);};
const unitVectorSchema=array(record({unit:identifier,exponent:(v:unknown)=>need(typeof v==='string'&&/^(0|-?[1-9][0-9]*)$/.test(v)&&BigInt(v)>=-16n&&BigInt(v)<=16n)}));
const sourceRefSchema=record({generatedTag:textValue,sourceHash:digest,spans:array(record({startByte:uint,endByte:uint}))});
const localValueSchema:Validator=v=>{if(v?.tag==='Bool')record({tag:literal('Bool'),value:boolean})(v);else if(v?.tag==='Quantity')record({tag:literal('Quantity'),unitVector:unitVectorSchema,value:uint})(v);else value(v);};
const coreExpressionSchema:Validator=v=>{
 need(v&&typeof v==='object');const meta={nodeId:textValue,sourceRef:sourceRefSchema,type:localTypeSchema,unitVector:unitVectorSchema};
 if(v.tag==='Literal')record({...meta,tag:literal('Literal'),value:localValueSchema})(v);
 else if(v.tag==='Remaining')record({...meta,tag:literal('Remaining')})(v);
 else if(['StateRef','ArgRef','ObservationRef','ConstRef'].includes(v.tag))record({...meta,tag:literal(v.tag),declaration:identifier})(v);
 else if(v.tag==='LocalRef')record({...meta,tag:literal('LocalRef'),action:identifier,local:identifier})(v);
 else if(v.tag==='Not')record({...meta,tag:literal('Not'),operand:coreExpressionSchema})(v);
 else if(v.tag==='FloorDiv')record({...meta,tag:literal('FloorDiv'),numerator:coreExpressionSchema,denominator:coreExpressionSchema})(v);
 else record({...meta,tag:oneOf('Add','Sub','Mul','Eq','Lt','Lte','Gt','Gte','And','Or'),left:coreExpressionSchema,right:coreExpressionSchema})(v);
};
const policyUseSchema:Validator=v=>v?.tag==='Financial'?record({tag:literal('Financial'),name:identifier})(v):record({tag:literal('NonFinancial')})(v);
const coreInstructionSchema:Validator=v=>{
 need(v&&typeof v==='object');const meta={sourceRef:sourceRefSchema,statementId:textValue};
 if(v.tag==='Guard')record({...meta,tag:literal('Guard'),condition:coreExpressionSchema,message:textValue})(v);
 else if(v.tag==='Let')record({...meta,tag:literal('Let'),expression:coreExpressionSchema,name:identifier,type:localTypeSchema,unitVector:unitVectorSchema})(v);
 else if(v.tag==='Set')record({...meta,tag:literal('Set'),expression:coreExpressionSchema,field:identifier,policy:policyUseSchema})(v);
 else record({...meta,tag:literal('Emit'),kind:oneOf('Transfer','Fee','DueCreated','DueSettled'),ordinal:uint,fields:array(record({label:textValue,expression:coreExpressionSchema})),policies:array(record({field:textValue,policy:identifier}))})(v);
};
const coreSchema=record({actions:array(record({actorParameter:literal('actor'),arguments:array(record({name:identifier,type:(v:unknown)=>{localTypeSchema(v);need((v as LocalValue).tag!=='Bool'&&(v as LocalValue).tag!=='Quantity');}})),instructions:array(coreInstructionSchema),name:identifier,resourceCounts:record({effects:uint,expressionDepth:uint,expressionNodes:uint,instructions:uint,locals:uint}),sourceRef:sourceRefSchema})),coreVersion:literal('moriarty-core/1'),schemaVersion:literal('moriarty-core-program/1')});
function admitBoundProgram(compiled:BoundProgram,bound:BoundProgram,limits:EncodingLimits):BoundProgram {
 const copied=inertSnapshot(bound,limits,'PROGRAM_ENCODING','PROGRAM_ENCODING',8);
 try{need(copied&&same(Object.keys(copied).sort(),Object.keys(compiled).sort()));need(copied.manifest&&same(Object.keys(copied.manifest).sort(),Object.keys(compiled.manifest).sort()));coreSchema(copied.manifest.core);}catch{throw new RuntimeError('PROGRAM_ENCODING',8);}
 // Malformed/unrelated manifest differences remain PROGRAM_ENCODING; a valid
 // Core tree, node, type or source-reference mismatch is specifically SOURCE_MAP.
 need(same({...copied,programHash:compiled.programHash,manifest:{...copied.manifest,core:compiled.manifest.core}},compiled),'PROGRAM_ENCODING',8);
 need(same(copied.manifest.core,compiled.manifest.core),'SOURCE_MAP',8);
 need(copied.programHash===compiled.programHash,'PROGRAM_ENCODING',8);return copied;
}
function admitEvaluation(input:R.EvaluationInput,limits:any):R.EvaluationInput {
 const copied=inertSnapshot(input,limits.evaluationEncoding);inputSchema(copied);enc(copied.authority.statement,limits.signingEnvelope,'INPUT_BOUNDS',9);return copied;
}
const inputSchema=record({action:actionSchema,authority:authoritySchema,checks:record({authenticatedPrincipal:textValue,genesisValid:boolean,nonceFresh:boolean,observationsAuthentic:boolean,predecessorSetValid:boolean,signatureValid:boolean,stateCurrentAndUnconsumed:boolean}),genesis:genesisSchema,observations:record({observations:array(record({evidenceDigest:digest,name:identifier,provider:textValue,value})),schemaVersion:literal('moriarty-observations/1')}),program:programRefSchema,schemaVersion:literal('moriarty-evaluation/1'),state:stateSchema});
export function decodeEvaluation(bytes:string|Uint8Array):R.EvaluationInput{
 need((typeof bytes==='string'?new TextEncoder().encode(bytes).length:bytes.length)<=65536,'INPUT_BOUNDS',9);
 try{return canonicalDecode(bytes,inputSchema) as R.EvaluationInput;}catch(e){if(e instanceof RuntimeError)throw e;throw new RuntimeError('INPUT_SCHEMA',9);}
}
export function programRef(bound:BoundProgram):R.ProgramRef {return {bounds:clone(bound.manifest.bounds),coreVersion:'moriarty-core/1',profile:PROFILE,programHash:bound.programHash,schemaVersion:'moriarty-program-ref/1',sourceHash:bound.manifest.sourceHash};}
export function sealState(body:R.StateBody):R.StateEnvelope{return {body:clone(body),schemaVersion:'moriarty-state/1',stateHash:hash('STATE',body)};}
function statuses(m:SemanticManifest,values:NamedStoredValue[],obligations:R.ObligationRecord[]):R.Status{
 const lookup=(name:string)=>values.find(x=>x.name===name)!.value;
 const r=m.statusRules.agreement;
 const remainingNotional:R.Status['remainingNotional']=r.tag==='NoRemainingNotional'?{tag:'NotApplicable'}:{tag:'Amount',amount:clone(lookup(r.field) as R.AmountValue)};
 return {episodeStatus:same(lookup(m.statusRules.episode.field),m.statusRules.episode.literal)?'Closed':'Open',agreementStatus:obligations.some(o=>o.status==='Outstanding')||(remainingNotional.tag==='Amount'&&uint(remainingNotional.amount.value)>0n)?'Outstanding':'NoOutstanding',remainingNotional};
}
function namedMatch(actual:NamedStoredValue[],schema:NamedType[]){need(actual.length===schema.length);for(let j=0;j<schema.length;j++){const a=actual[j],s=schema[j];need(a.name===s.name&&a.value.tag===s.type.tag);if(s.type.tag==='Amount')need(a.value.tag==='Amount'&&a.value.unit===s.type.unit);}}
const byteOrder=(a:string,b:string)=>{const x=new TextEncoder().encode(a),y=new TextEncoder().encode(b);for(let i=0;i<Math.min(x.length,y.length);i++)if(x[i]!==y[i])return x[i]-y[i];return x.length-y.length;};
function unique<T>(xs:T[],key:(x:T)=>string,sorted=false){const keys=xs.map(key);need(new Set(keys).size===keys.length);if(sorted)need(same(keys,[...keys].sort(byteOrder)));}
function validate(bound:BoundProgram,i:R.EvaluationInput,limits:any,admissionOnly=false){
 try{canonicalEncode(i);}catch{throw new RuntimeError('INPUT_SCHEMA',9);}
 inputSchema(i);enc(i,limits.evaluationEncoding,'INPUT_BOUNDS',9);
 enc(i.authority.statement,limits.signingEnvelope,'INPUT_BOUNDS',9);
 const m=bound.manifest,g=i.genesis.body,s=i.state.body,a=i.authority.statement,p=programRef(bound);
 need(same(i.program,p),'PROGRAM_BINDING');
 need(hash('GENESIS',g)===i.genesis.genesisHash&&hash('STATE',s)===i.state.stateHash,'DIGEST_MISMATCH');
 for(const [ok,code] of [[same(g.program,p),'GENESIS_PROGRAM_BINDING'],[same(g.initialState,m.initialState),'GENESIS_INITIAL_STATE_BINDING'],[g.lifetime===m.lifetime,'GENESIS_LIFETIME_BINDING'],[g.horizon===m.horizon,'GENESIS_HORIZON_BINDING'],[same(g.bounds,p.bounds),'GENESIS_BOUNDS_BINDING'],[s.programHash===p.programHash,'STATE_PROGRAM_BINDING'],[s.genesisHash===i.genesis.genesisHash,'STATE_GENESIS_BINDING'],[s.instanceId===g.instanceId,'INSTANCE_BINDING'],[same(a.program,p),'AUTHORITY_PROGRAM_BINDING'],[same(a.domain,g.domain)&&a.genesisHash===i.genesis.genesisHash&&a.instanceId===g.instanceId&&a.beforeStateHash===i.state.stateHash,'AUTHORITY_CONTEXT_BINDING'],[same(a.requiredClaims,m.requiredClaims)&&a.requiredClaimRoot===hash('CLAIMS',m.requiredClaims)&&g.requiredClaimRoot===hash('CLAIMS',m.requiredClaims),'CLAIM_ROOT_BINDING'],[same(a.predecessors,[i.state.stateHash]),'PREDECESSOR_BINDING']] as [boolean,string][])need(ok,code);
 const action=m.core.actions.find(x=>x.name===i.action.name);need(action);namedMatch(i.action.arguments,action.arguments);namedMatch(s.values,m.stateSchema);namedMatch(i.observations.observations,m.observationSchema);
 unique(g.principalBindings,x=>x.principal,true);unique(g.principalBindings,x=>x.actor);unique(g.observationBindings,x=>x.name);
 need(same(g.observationBindings.map(x=>x.name),m.observationSchema.map(x=>x.name)));
 unique(s.obligations,o=>o.dueId);need(s.obligations.length<=limits.programShape.obligationRecordsIncludingSettled);
 for(const o of s.obligations)need(uint(o.amount.value)>0n&&o.denomination===o.amount.unit&&m.units.includes(o.amount.unit));
 if(a.mode==='ExactPlan')need(same(a.action,i.action),'EXACT_ACTION_BINDING');
 else {unique(a.allowedActions,x=>x);need(a.allowedActions.includes(action.name)&&a.allowedActions.every(n=>m.core.actions.some(x=>x.name===n)),'INTENT_ACTION_FORBIDDEN');need(same(a.allowedActions,m.core.actions.map(x=>x.name).filter(n=>a.allowedActions.includes(n))));unique(a.permittedRecipients,x=>x,true);for(const xs of [a.grossDebitCaps,a.minimumNetCredits]){unique<{actor:string;asset:string}>(xs,x=>canonicalEncode([x.actor,x.asset]));const sorted=[...xs].sort((x,y)=>byteOrder(x.actor,y.actor)||byteOrder(x.asset,y.asset));need(same(xs,sorted));}}
 const derived=statuses(m,s.values,s.obligations);
 if(s.revision==='0')need(same(s.values,m.initialState)&&s.remaining===g.lifetime&&s.obligations.length===0&&same(derived,{episodeStatus:s.episodeStatus,agreementStatus:s.agreementStatus,remainingNotional:s.remainingNotional}),'GENESIS_STATE_BINDING');
 if(admissionOnly)return {action,actor:''};
 const c=i.checks;
 for(const [key,code] of [['genesisValid','GENESIS_UNAUTHENTICATED'],['signatureValid','SIGNATURE_INVALID'],['nonceFresh','NONCE_STALE'],['stateCurrentAndUnconsumed','STATE_NOT_CURRENT'],['observationsAuthentic','OBSERVATION_UNAUTHENTICATED'],['predecessorSetValid','PREDECESSOR_UNAUTHENTICATED']] as const)need(c[key]===true,code,10);
 for(let j=0;j<i.observations.observations.length;j++)need(i.observations.observations[j].provider===g.observationBindings[j].provider,'OBSERVATION_UNAUTHENTICATED',10);
 const principal=g.principalBindings.find(x=>x.principal===a.principal);
 need(c.authenticatedPrincipal===a.principal&&principal&&same(i.action.arguments.find(x=>x.name==='actor')?.value,{tag:'Text',value:principal.actor}),'PRINCIPAL_BINDING',10);
 need(checkedUInt128(uint(s.revision)+uint(s.remaining),'LIFETIME_INVARIANT',10)===uint(g.lifetime),'LIFETIME_INVARIANT',10);need(uint(s.remaining)>0n,'LIFETIME_EXHAUSTED',10);
 const now=uint(i.observations.observations.find(x=>x.name==='now')!.value.value);need(now<uint(g.horizon),'HORIZON_EXPIRED',10);need(uint(a.validity.notBefore)<=now&&now<uint(a.validity.notAfterExclusive)&&uint(a.validity.notAfterExclusive)<=uint(g.horizon),'VALIDITY_INTERVAL',10);
 need(same(derived,{episodeStatus:s.episodeStatus,agreementStatus:s.agreementStatus,remainingNotional:s.remainingNotional}),'STATUS_MISMATCH',10);
 return {action,actor:principal.actor};
}
type RuntimeValue={value:LocalValue;floors:Set<string>};
function execute(bound:BoundProgram,i:R.EvaluationInput,limits:any,skipExact:boolean):R.Simulation {
 const m=bound.manifest,{action,actor}=validate(bound,i,limits),s=i.state.body;
 const state=new Map(s.values.map(x=>[x.name,{value:clone(x.value),floors:new Set<string>()} as RuntimeValue]));
 const constants=new Map(m.constants.map(x=>[x.name,x.value]));const args=new Map(i.action.arguments.map(x=>[x.name,x.value]));const obs=new Map(i.observations.observations.map(x=>[x.name,x.value]));const locals=new Map<string,RuntimeValue>();
 let nodes=0,depthMax=0,components=0,instructions=0;
 const writes:R.WriteRecord[]=[];const raw:{kind:string;ordinal:string;fields:Record<string,StoredValue>;span:Span}[]=[];
 const pack=(value:LocalValue,floors=new Set<string>()):RuntimeValue=>({value:clone(value),floors:new Set(floors)});
 function evalExpr(e:CoreExpression,depth=1):RuntimeValue {
  nodes++;depthMax=Math.max(depthMax,depth);components=Math.max(components,e.unitVector.length);
  const span=e.sourceRef.spans[0];let out:RuntimeValue;
  if(e.tag==='Literal')out=pack(e.value);
  else if(e.tag==='Remaining')out=pack({tag:'UInt128',value:s.remaining});
  else if(e.tag==='StateRef'){const v=state.get(e.declaration)!;out=pack(v.value,v.floors);}
  else if(e.tag==='ConstRef')out=pack(constants.get(e.declaration)!);
  else if(e.tag==='ArgRef')out=pack(args.get(e.declaration)!);
  else if(e.tag==='ObservationRef')out=pack(obs.get(e.declaration)!);
  else if(e.tag==='LocalRef'){const v=locals.get(e.local)!;out=pack(v.value,v.floors);}
  else if(e.tag==='Not')out=pack({tag:'Bool',value:!evalExpr(e.operand,depth+1).value.value});
  else {
   const left=evalExpr(e.tag==='FloorDiv'?e.numerator:e.left,depth+1);
   if((e.tag==='And'&&left.value.value===false)||(e.tag==='Or'&&left.value.value===true))out=pack({tag:'Bool',value:left.value.value as boolean});
   else {
    const right=evalExpr(e.tag==='FloorDiv'?e.denominator:e.right,depth+1);
    if(e.tag==='And'||e.tag==='Or')out=pack({tag:'Bool',value:right.value.value as boolean});
    else if(e.tag==='Eq')out=pack({tag:'Bool',value:same(left.value,right.value)});
    else if(['Lt','Lte','Gt','Gte'].includes(e.tag)){const a=uint(left.value.value),b=uint(right.value.value);out=pack({tag:'Bool',value:e.tag==='Lt'?a<b:e.tag==='Lte'?a<=b:e.tag==='Gt'?a>b:a>=b});}
    else {
     const a=uint(left.value.value),b=uint(right.value.value);let n:bigint;
     if(e.tag==='Add')n=checkedUInt128(a+b,'ARITHMETIC_OVERFLOW',11,span);
     else if(e.tag==='Sub')n=checkedUInt128(a-b,'ARITHMETIC_UNDERFLOW',11,span);
     else if(e.tag==='Mul')n=checkedUInt128(a*b,'ARITHMETIC_OVERFLOW',11,span);
     else {need(b!==0n,'DIVISION_BY_ZERO',11,span);n=a/b;}
     const floors=new Set([...left.floors,...right.floors]);if(e.tag==='FloorDiv')floors.add(e.nodeId);
     const type=e.type;need(type.tag!=='Bool'&&type.tag!=='Text','PROGRAM_ENCODING',8,span);
     out=pack(type.tag==='Amount'?{tag:'Amount',unit:type.unit,value:String(n)}:type.tag==='Quantity'?{tag:'Quantity',unitVector:clone(type.unitVector),value:String(n)}:{tag:'UInt128',value:String(n)},floors);
    }
   }
  }
  if(out.value.tag!=='Text'&&out.value.tag!=='Bool')uint(out.value.value);
  return out;
 }
 function policy(name:string,v:RuntimeValue,span:Span){const p=m.policies.find(x=>x.name===name)!;need(p&&v.value.tag==='Amount'&&v.value.unit===p.unit,'POLICY_PROVENANCE',11,span);need(p.roundingNode.tag==='None'?v.floors.size===0:v.floors.size===1&&v.floors.has(p.roundingNode.coreNodeId),'POLICY_PROVENANCE',11,span);}
 for(const ins of action.instructions){instructions++;const span=ins.sourceRef.spans[0];
  if(ins.tag==='Guard'){need(evalExpr(ins.condition).value.value===true,'GUARD_FAILED',11,span,ins.message);}
  else if(ins.tag==='Let')locals.set(ins.name,evalExpr(ins.expression));
  else if(ins.tag==='Set'){const v=evalExpr(ins.expression);if(ins.policy.tag==='Financial')policy(ins.policy.name,v,span);state.set(ins.field,v);writes.push({field:ins.field,policy:clone(ins.policy),value:clone(v.value as StoredValue)});}
  else {const fields:Record<string,StoredValue>={};for(const field of ins.fields){const v=evalExpr(field.expression);const p=ins.policies.find(x=>x.field===field.label);if(p)policy(p.policy,v,span);fields[field.label]=clone(v.value as StoredValue);}raw.push({kind:ins.kind,ordinal:ins.ordinal,fields,span});}
 }
 const effects:R.EffectRecord[]=[];const obligations=clone(s.obligations);const delta:R.CompleteBody['obligationDelta']={created:[],settled:[]};
 function settle(asset:string,amount:R.AmountValue,span:Span):R.SettlementResolution {
  const b=m.settlementBindings.find(x=>x.asset===asset&&x.unit===amount.unit);need(b,'SETTLEMENT_BINDING_MISSING',12,span);const q=uint(b.quantum.value),n=uint(amount.value);need(q>0n,'SETTLEMENT_OVERFLOW',12,span);need(n%q===0n,'SETTLEMENT_NON_DIVISIBLE',12,span);const ledger=checkedUInt128(n/q,'SETTLEMENT_OVERFLOW',12,span);need(checkedUInt128(ledger*q,'SETTLEMENT_OVERFLOW',12,span)===n,'SETTLEMENT_OVERFLOW',12,span);return {asset,binding:b.name,ledgerAmount:String(ledger),nominalAmount:clone(amount),quantum:clone(b.quantum),unit:b.unit};
 }
 for(const r of raw){const f=r.fields,amount=clone(f.amount as R.AmountValue),str=(key:string)=>f[key].value as string;let effect:R.EffectRecord;
  if(r.kind==='Transfer'||r.kind==='Fee')effect={amount,asset:str('asset'),from:str('from'),kind:r.kind,ordinal:r.ordinal,settlement:settle(str('asset'),amount,r.span),to:str('to')};
  else {
   const due={amount,creditor:str('creditor'),debtor:str('debtor'),denomination:str('denomination'),dueId:str('due_id')};
   if(r.kind==='DueCreated')effect={...due,kind:'DueCreated',ordinal:r.ordinal};else effect={...due,asset:str('asset'),kind:'DueSettled',ordinal:r.ordinal,settlement:settle(str('asset'),amount,r.span)};
   need(uint(amount.value)>0n,'OBLIGATION_ZERO',12,r.span);need(due.denomination===amount.unit,'OBLIGATION_MISMATCH',12,r.span);
   const old=obligations.find(o=>o.dueId===due.dueId);
   if(r.kind==='DueCreated'){need(!old,'OBLIGATION_DUPLICATE',12,r.span);need(obligations.length<limits.programShape.obligationRecordsIncludingSettled,'OBLIGATION_CAPACITY',12,r.span);const next:R.ObligationRecord={...clone(due),status:'Outstanding'};obligations.push(next);delta.created.push(clone(next));}
   else {need(old,'OBLIGATION_UNKNOWN',12,r.span);need(old.status==='Outstanding','OBLIGATION_ALREADY_SETTLED',12,r.span);need(old.creditor===due.creditor&&old.debtor===due.debtor&&old.denomination===due.denomination&&old.amount.unit===amount.unit,'OBLIGATION_MISMATCH',12,r.span);need(uint(amount.value)>=uint(old.amount.value),'OBLIGATION_PARTIAL_UNSUPPORTED',12,r.span);need(uint(amount.value)<=uint(old.amount.value),'OBLIGATION_EXCESS',12,r.span);old.status='Settled';delta.settled.push(clone(old));}
  }
  effects.push(effect);
 }
 const groupKey=(asset:string,from:string,to:string,unit:string)=>canonicalEncode([asset,from,to,unit]);
 const dueGroups=new Map<string,[bigint,bigint]>(),transferGroups=new Map<string,[bigint,bigint]>();
 const addGroup=(map:Map<string,[bigint,bigint]>,key:string,nominal:string,ledger:string)=>{const old=map.get(key)??[0n,0n];map.set(key,[checkedUInt128(old[0]+uint(nominal),'OBLIGATION_SUM_OVERFLOW',12),checkedUInt128(old[1]+uint(ledger),'OBLIGATION_SUM_OVERFLOW',12)]);};
 for(const e of effects)if(e.kind==='DueSettled')addGroup(dueGroups,groupKey(e.asset,e.debtor,e.creditor,e.amount.unit),e.amount.value,e.settlement.ledgerAmount);
 for(const e of effects)if(e.kind==='Transfer'){const key=groupKey(e.asset,e.from,e.to,e.amount.unit);if(dueGroups.has(key))addGroup(transferGroups,key,e.amount.value,e.settlement.ledgerAmount);}
 for(const [key,v] of dueGroups){const t=transferGroups.get(key)??[0n,0n];need(v[0]===t[0]&&v[1]===t[1],'OBLIGATION_CONSERVATION',12);}
 const a=i.authority.statement;
 if(a.mode==='ExactPlan'){if(!skipExact)need(same(a.exactWrites,writes.map(({field,value})=>({field,value})))&&same(a.exactEffects,effects.map(effect=>({effect}))),'EXACT_PLAN_MISMATCH',12);}
 else {
  need(a.permittedCalls.length===0,'INTENT_CALL_UNSUPPORTED',12);
  need([...a.grossDebitCaps,...a.minimumNetCredits].every(x=>x.actor===actor),'INTENT_ACTOR_SCOPE',12);
  const debits=new Map<string,bigint>(),credits=new Map<string,bigint>();
  const add=(map:Map<string,bigint>,asset:string,n:string)=>map.set(asset,checkedUInt128((map.get(asset)??0n)+uint(n),'INTENT_ARITHMETIC_OVERFLOW',12));
  for(const e of effects)if(e.kind==='Transfer'||e.kind==='Fee'){
   need(a.permittedRecipients.includes(e.to),'INTENT_RECIPIENT_FORBIDDEN',12);
   if(e.from===actor){need(a.grossDebitCaps.some(c=>c.asset===e.asset),'INTENT_DEBIT_UNCAPPED',12);add(debits,e.asset,e.settlement.ledgerAmount);}
   if(e.to===actor)add(credits,e.asset,e.settlement.ledgerAmount);
  }
  for(const cap of a.grossDebitCaps)need((debits.get(cap.asset)??0n)<=uint(cap.maximumLedgerAmount),'INTENT_DEBIT_CAP',12);
  for(const goal of a.minimumNetCredits)need((credits.get(goal.asset)??0n)>=checkedUInt128((debits.get(goal.asset)??0n)+uint(goal.minimumLedgerAmount),'INTENT_ARITHMETIC_OVERFLOW',12),'INTENT_NET_GOAL',12);
 }
 const values=s.values.map(x=>({name:x.name,value:clone(state.get(x.name)!.value as StoredValue)}));
 const after=sealState({...clone(s),...statuses(m,values,obligations),values,obligations,remaining:String(uint(s.remaining)-1n),revision:String(checkedUInt128(uint(s.revision)+1n,'REVISION_OVERFLOW',12))});
 const countBody:Omit<R.CompleteBody,'resourceCounts'>={actionHash:hash('ACTION',i.action),after,authorityConsumption:{mode:a.mode,nonce:a.nonce,principal:a.principal,statementDigest:hashDomain(i.authority.domain,a)},beforeStateHash:i.state.stateHash,effects,obligationDelta:delta,observationsHash:hash('OBSERVATIONS',i.observations),outcome:'Complete',predecessors:clone(a.predecessors),profile:PROFILE,programHash:bound.programHash,schemaVersion:'moriarty-complete-body/1',writes};
 const counts=measureEncoding(countBody),shape=limits.programShape;
 need(instructions<=shape.instructionsPerEntrypoint&&nodes<=shape.expressionNodesPerEntrypoint&&depthMax<=shape.expressionDepthRootOne&&effects.length<=shape.effectsPerEntrypoint&&components<=shape.expressionUnitComponents,'RESOURCE_BOUNDS',12);
 const body:R.CompleteBody={...countBody,resourceCounts:{canonicalDepth:String(counts.decodedDepthRootZero),canonicalNodes:String(counts.decodedNodes),canonicalUtf8Bytes:String(counts.utf8Bytes),effects:String(effects.length),executedInstructions:String(instructions),expressionNodes:String(nodes),maximumExpressionDepth:String(depthMax),unitComponents:String(components)}};
 const candidate:R.Complete={body,schemaVersion:'moriarty-result/1',traceHash:hash('TRACE',body)};
 enc(after,limits.evaluationEncoding,'RESULT_BOUNDS',12);enc(candidate,limits.resultEncoding,'RESULT_BOUNDS',12);
 const context:R.ProofContext={actionHash:body.actionHash,authorityDigest:hash('AUTHORITY',i.authority),beforeStateHash:i.state.stateHash,domain:clone(i.genesis.body.domain),genesisHash:i.genesis.genesisHash,predecessors:clone(a.predecessors),program:clone(i.program),requiredClaimRoot:a.requiredClaimRoot,schemaVersion:'moriarty-proof-context/1',traceHash:candidate.traceHash};
 enc(context,limits.proofAcceptanceEncoding,'PROOF_SCHEMA',13);
 return {kind:'Simulation',candidate,context};
}
/** Candidate derivation always recompiles the original bytes; arbitrary caller
 * Core records, policies or source maps cannot substitute for source lowering. */
export function derive(bound:BoundProgram,input:R.EvaluationInput,binding:R.SourceBinding,options:{unsignedExactPlan?:boolean}={}):R.Simulation|R.Rejected {
 let verifiedHash=ZERO;
 try {
  need(binding&&(typeof binding.source==='string'||binding.source instanceof Uint8Array),'PROGRAM_ENCODING',8);
  const compiled=compile(binding.source,binding.bounds).bound,limits=registeredBounds().bounds;
  const admitted=admitBoundProgram(compiled,bound,limits.programManifestEncoding);verifiedHash=compiled.programHash;
  return execute(admitted,admitEvaluation(input,limits),limits,options.unsignedExactPlan===true);
 }catch(e){return rejected(e,verifiedHash);}
}
export function createSimulator(source:string|Uint8Array,bounds:string|Uint8Array){
 const admitted=compile(source,bounds).bound;
 const binding={source:typeof source==='string'?source:new Uint8Array(source),bounds:typeof bounds==='string'?bounds:new Uint8Array(bounds)};
 const bound=admitted,program=programRef(bound),m=bound.manifest;
 return {bound:clone(bound),program:clone(program),
  makeGenesis(options:{domain:R.ExecutionDomain;instanceId:string;principalBindings:R.PrincipalBinding[];observationBindings:R.ObservationBinding[]}):R.Genesis{
   const body:R.GenesisBody={bounds:clone(m.bounds),domain:clone(options.domain),horizon:m.horizon,initialState:clone(m.initialState),instanceId:options.instanceId,lifetime:m.lifetime,observationBindings:clone(options.observationBindings),principalBindings:clone(options.principalBindings),profile:PROFILE,program:clone(program),requiredClaimRoot:hash('CLAIMS',m.requiredClaims),schemaVersion:'moriarty-genesis-body/1'};
   const genesis:R.Genesis={body,genesisHash:hash('GENESIS',body),schemaVersion:'moriarty-genesis/1'};genesisSchema(genesis);return genesis;
  },
  initialState(genesis:R.Genesis):R.StateEnvelope{genesisSchema(genesis);need(same(genesis.body.program,program),'GENESIS_PROGRAM_BINDING');return sealState({...statuses(m,m.initialState,[]),genesisHash:genesis.genesisHash,instanceId:genesis.body.instanceId,obligations:[],profile:PROFILE,programHash:bound.programHash,remaining:m.lifetime,revision:'0',schemaVersion:'moriarty-state-body/1',values:clone(m.initialState)});},
  simulate(input:R.EvaluationInput,options:{unsignedExactPlan?:boolean}={}){return derive(bound,input,binding,options);}
 };
}
/** Only deployment-owned implementations belong in backend. Client booleans are
 * discarded and reconstructed for the exact tuple. No such backend is shipped.
 * This function therefore fails closed in this repository's actual deployment. */
export async function evaluate(bound:BoundProgram,input:R.EvaluationInput,backend?:R.TrustedAcceptanceBackend):Promise<R.Rejected|R.Complete>{
 if(!backend||typeof backend.authenticate!=='function'||typeof backend.verifyAndCommit!=='function')return rejected(new RuntimeError('PROOF_INVALID',13));
 let admitted:BoundProgram,copied:R.EvaluationInput,limits:any;
 try {
  need(backend.source&&(typeof backend.source==='string'||backend.source instanceof Uint8Array),'PROGRAM_ENCODING',8);
  const compiled=compile(backend.source,backend.bounds).bound;limits=registeredBounds().bounds;
  admitted=admitBoundProgram(compiled,bound,limits.programManifestEncoding);
  copied=admitEvaluation(input,limits);validate(admitted,copied,limits,true);
 }catch(e){return rejected(e);}
 try {
  const {checks:ignored,...tuple}=copied;
  // Only inert admitted data reaches authentication. Client precheck booleans
  // remain shape-checked but never constitute authentication authority.
  const freshChecks=await backend.authenticate(admitted,Object.freeze(tuple));
  const authenticated=admitEvaluation({...copied,checks:inertSnapshot(freshChecks,limits.evaluationEncoding)},limits);
  const result=execute(admitted,authenticated,limits,false);
  const committed=await backend.verifyAndCommit(admitted,authenticated,Object.freeze({kind:'Simulation' as const,candidate:inertSnapshot(result.candidate,limits.resultEncoding),context:inertSnapshot(result.context,limits.proofAcceptanceEncoding)}));
  need(committed?.tag==='Committed'&&committed.traceHash===result.candidate.traceHash&&committed.proofContextHash===hash('PROOF-CONTEXT',result.context)&&committed.afterStateHash===result.candidate.body.after.stateHash,'COMMIT_CONFLICT',13);
  return result.candidate;
 }catch(e){return rejected(e instanceof RuntimeError?e:new RuntimeError('PROOF_INVALID',13),admitted.programHash);}
}
