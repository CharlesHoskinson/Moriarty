/** Public APIs take text. This module has no CLI or network side effects. */
import { analyze, check, format, type Value } from './frontend.ts';
import { LocalError } from './json.ts';
export { check, format };
export { expand, simulate } from './bridge.ts';
export function inspect(source:string):object{
 const a=analyze(source);
 let work=0,bytes=0;
 const charge=(value:unknown):void=>{bytes+=Buffer.byteLength(JSON.stringify(value));if(bytes>524288||++work>8192)throw new LocalError('BETA_INSPECTION_BOUND','Signed scope summary exceeds bounded response work or bytes');};
 // Entities are shallow claims, never recursively expanded declaration DAGs.
 const claims=(v:Extract<Value,{tag:'entity'}>):Record<string,unknown>=>{
  const result:Record<string,unknown>={};
  for(const [key,x] of Object.entries(v.fields)){
   if(x.tag==='string'||x.tag==='scalar'||x.tag==='bool')result[key]=x.value;
   else if(x.tag==='entity')result[key]={declaration:x.name,kind:x.kind,id:x.fields.id?.tag==='string'?x.fields.id.value:null};
  }
  charge(result);return result;
 };
 const term=(v:Value,depth=0):object=>{
  if(depth>64)throw new LocalError('BETA_INSPECTION_BOUND','Signed scope projection exceeds depth64');
  charge(v.tag);
  if(v.tag==='entity')return {tag:v.tag,kind:v.kind,declaration:v.name,claims:claims(v),authenticated:false};
  if(v.tag==='qty'){charge(v.atoms);return {tag:v.tag,asset:v.asset,atoms:v.atoms};}
  if(v.tag==='tag')return {tag:v.tag,name:v.name};
  if(v.tag==='string'||v.tag==='scalar'||v.tag==='bool'){charge(v.value);return {tag:v.tag,value:v.value};}
  if(v.tag==='array')return {tag:v.tag,items:v.items.map(x=>term(x,depth+1))};
  const values:Record<string,object>={};
  for(const [key,x] of Object.entries(v.tag==='call'?v.args:v.fields)){charge(key);values[key]=term(x,depth+1);}
  return v.tag==='call'?{tag:v.tag,name:v.name,args:values}:{tag:v.tag,fields:values};
 };
 try{
 const identities=a.declarations.filter(d=>['domain','account','asset','obligation'].includes(d.kind)).map(d=>{
  const v=d.value;if(v.tag!=='entity')return {kind:d.kind,name:d.name};
  return {kind:d.kind,name:d.name,claims:claims(v),authenticated:false,span:d.span};
 });
 const intents=a.declarations.filter(d=>d.kind==='intent').map(d=>{
  const terms:Record<string,object>={};
  if(d.value.tag==='entity')for(const [key,v] of Object.entries(d.value.fields)){charge(key);terms[key]=term(v);}
  return {name:d.name,terms};
 });
 const scalarAndQuantityMax=((1n<<128n)-1n).toString(),s0SignedFieldMax=((1n<<127n)-1n).toString();
 const s0Premises=['canonical-intent-signature','snapshot-to-head','head-extension','atomic-ledger-compare-and-consume'];
 const s0Bindings=['agreement-id','selected-program','asset-scale','authenticated-predecessor'];
 const localS0Actions=a.actions.filter(action=>action.support==='LocalS0').map(action=>action.name);
 const hasLocalS0=localS0Actions.length>0;
 const actions=a.actions.map(action=>{
  const local=action.support==='LocalS0';
  return {...action,coverage:{financialRelations:local?'DelegatedToCoreDuringLocalPreparation':'Open',localPreparation:local?'AvailableUnqualified':'Unsupported'},
   bounds:{scalarAndQuantityMax,...(local?{s0SignedFieldMax}:{})},requiredPremises:local?s0Premises:[],unverifiedBindings:local?s0Bindings:[]};
 });
 const result={status:a.status,sourceHash:a.sourceHash,diagnostics:a.diagnostics,agreement:a.agreement,identities,actions,intents,
  coverage:{syntax:'checked when diagnostics empty',names:'checked',quantities:'nominal exact arithmetic',financialRelations:hasLocalS0?'ConditionalOnLocalS0Preparation':'Open',localS0Actions,authentication:'open',ledgerCommit:'open'},
  bounds:{scalarAndQuantityMax,...(hasLocalS0?{s0SignedFieldMax,s0Scope:'LocalS0ActionsOnly'}:{})},
  requiredPremises:hasLocalS0?s0Premises:[],unverifiedBindings:hasLocalS0?s0Bindings:[]};
 if(Buffer.byteLength(JSON.stringify(result))>524288)throw new LocalError('BETA_INSPECTION_BOUND','Serialized inspection exceeds 524288 bytes');
 return result;
 }catch(error){
  if(!(error instanceof LocalError))throw error;
  return {status:'InspectionRejected',authoringStatus:a.status,sourceHash:a.sourceHash,diagnostics:[{code:error.code,message:error.message,span:{start:0,end:0}}],agreement:a.agreement,identities:[],actions:[],intents:[]};
 }
}

export {prepareOwnerIntent,verifyAndPrepare,type Signing,type CryptoConfig,type SignedIntentStatement,type NativeIntentReceipt,type NativeIntentVerificationReceipt,type OwnerIntentPreparation,type SignedLocalPreparation} from './auth.ts';
export {LocalSettlementStore,deriveSuccessorHead,validateSuccessor,type Snapshot,type SettleResult,type Binding,type Domain} from './atomic.ts';
export {IntentSourceMismatchError} from './auth.ts';
