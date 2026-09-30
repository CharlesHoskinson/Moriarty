/** Authoring proposal → strict Source/6 → existing Core/5. Never ledger admission. */
import { createHash } from 'node:crypto';
import { analyze, type Span, type Value, type Diagnostic, type Reference } from './frontend.ts';
import { LocalError, parseBoundedJson, scalarText } from './json.ts';
import { parseAndLowerSource6, Source6Error } from '../../../experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts';
import { prepareSource6S0Unqualified, type Source6S0Outcome } from '../../../experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts';
const U=(1n<<128n)-1n, S=(1n<<127n)-1n;
const digest=(text:string):string=>createHash('sha256').update(text).digest('hex');
export type Origin = {id:string;kind:'source'|'scenario'|'derived'|'generated';span?:Span;pointer?:string;rule?:string;dependencies?:string[]};
export type FieldMap = {field:string;generatedSpan:Span;origins:string[]};
export interface BetaFailure {qualification:'local-stipulation-only';status:'AuthoringRejected'|'FormationRejected'|'Unsupported';diagnostics:Diagnostic[];publishedPost:null;publishedEffects:null;sourceHash:string;scenarioHash:string|null;scenarioValidation?:'NotAppliedUnsupported'}
export interface BetaExpansion {status:'Expanded';sourceHash:string;scenarioHash:string;source6:string;fieldMap:FieldMap[];origins:Origin[];proposalNote:string|null;qualification:'local-stipulation-only'}
export type Expanded = BetaFailure|BetaExpansion;
export type Simulated = BetaFailure|{status:Source6S0Outcome['status'];sourceHash:string;scenarioHash:string;result:Source6S0Outcome;qualification:'local-stipulation-only'};
function problem(code:string,message:string,pointer=''):never {throw new LocalError(code,message,pointer);}
function record(value:unknown,required:string[],optional:string[]=[],pointer=''):Record<string,unknown>{
 if(!value||typeof value!=='object'||Array.isArray(value))problem('BETA_SCENARIO_SCHEMA','Expected record',pointer);
 const r=value as Record<string,unknown>;
 for(const key of Object.keys(r))if(!required.includes(key)&&!optional.includes(key))problem('BETA_SCENARIO_SCHEMA',`Unknown field ${key}`,`${pointer}/${key.replaceAll('~','~0').replaceAll('/','~1')}`);
 for(const key of required)if(!Object.hasOwn(r,key))problem('BETA_SCENARIO_SCHEMA',`Missing field ${key}`,`${pointer}/${key}`);
 return r;
}
function str(v:unknown,pointer:string,identifier=false):string{
 // Formation checks transport representability. Core alone checks opaque claims.
 if(typeof v!=='string'||!v||Buffer.byteLength(v)>1024||!scalarText(v))problem('BETA_SCENARIO_STRING','Expected nonempty scalar text',pointer);
 if(identifier&&!/^[A-Za-z][A-Za-z0-9_]{0,63}$/.test(v as string))problem('BETA_SCENARIO_ID','ID is not representable in Source/6',pointer);
 return v as string;
}
function uint(v:unknown,pointer:string,max=U):string{
 if(typeof v!=='string'||v.length>39||!/^(0|[1-9][0-9]*)$/.test(v)||BigInt(v)>max)problem('BETA_SCENARIO_INTEGER','Expected canonical bounded decimal string',pointer);
 return v as string;
}
function fields(v:Value,tag='record'):Record<string,Value>{
 if(v.tag==='record'||v.tag==='entity')return v.fields;
 return problem('BETA_SOURCE_SHAPE',`Expected ${tag}`);
}
function field(r:Record<string,Value>,name:string):Value {if(!r[name])problem('BETA_SOURCE_SHAPE',`Missing ${name}`);return r[name];}
function text(v:Value):string {if(v.tag!=='string')problem('BETA_SOURCE_SHAPE','Expected string');return (v as Extract<Value,{tag:'string'}>).value;}
function scalar(v:Value):string {if(v.tag!=='scalar')problem('BETA_SOURCE_SHAPE','Expected scalar');return (v as Extract<Value,{tag:'scalar'}>).value;}
function qty(v:Value):string {if(v.tag!=='qty')problem('BETA_SOURCE_SHAPE','Expected quantity');return (v as Extract<Value,{tag:'qty'}>).atoms;}
function entity(v:Value,kind:string):Record<string,Value>{if(v.tag!=='entity'||v.kind!==kind)problem('BETA_SOURCE_SHAPE',`Expected ${kind}`);return fields(v);}
function economicId(v:Value,kind:string):string{return text(field(entity(v,kind),'id'));}
interface Scenario {
 domain:string;asset:string;head:string;predecessor:string;round:string;balances:{account:string;amount:string}[];
 allowance:{owner:string;remaining:string;spent:string};replay:'unused'|'consumed';work_remaining:string;work_spent:string;post_head:string;
 obligation?:{id:string;debtor:string;creditor:string;asset:string;principal:string;accrued:string;outstanding:string;status:'Outstanding'};
 candidate_effects?:Record<string,unknown>[];
}
function scenario(textInput:string,repay:boolean,domain:string,asset:string,signer:string,expectedAccounts:string[],obligationId?:string):Scenario{
 const r=record(parseBoundedJson(textInput),['profile','kind','domain','asset','head','predecessor','round','balances','allowance','replay','work_remaining','work_spent','post_head'],repay?['obligation','candidate_effects']:['candidate_effects']);
 if(r.profile!=='moriarty-local-scenario/1'||r.kind!=='local-stipulation')problem('BETA_SCENARIO_PROFILE','Expected local scenario profile');
 const gotDomain=str(r.domain,'/domain',true),gotAsset=str(r.asset,'/asset',true);
 if(gotDomain!==domain)problem('BETA_SCENARIO_IDENTITY',`Expected domain ID ${domain}; got ${gotDomain}`,'/domain');
 if(gotAsset!==asset)problem('BETA_SCENARIO_IDENTITY',`Expected asset ID ${asset}; got ${gotAsset}`,'/asset');
 let obligation:Scenario['obligation'];
 if(repay){
  const d=record(r.obligation,['id','debtor','creditor','asset','principal','accrued','outstanding','status'],[],'/obligation');
  if(d.status!=='Outstanding')problem('BETA_SCENARIO_SCHEMA','S0 requires an Outstanding obligation','/obligation/status');
  obligation={id:str(d.id,'/obligation/id',true),debtor:str(d.debtor,'/obligation/debtor',true),creditor:str(d.creditor,'/obligation/creditor',true),asset:str(d.asset,'/obligation/asset',true),principal:uint(d.principal,'/obligation/principal',S),accrued:uint(d.accrued,'/obligation/accrued',S),outstanding:uint(d.outstanding,'/obligation/outstanding',S),status:'Outstanding'};
  if(obligation.id!==obligationId||obligation.debtor!==signer||obligation.asset!==asset||obligation.creditor===signer)problem('BETA_SCENARIO_IDENTITY','Obligation does not match source request','/obligation');
  if(BigInt(obligation.principal)+BigInt(obligation.accrued)!==BigInt(obligation.outstanding))problem('BETA_SCENARIO_ACCOUNTING','Outstanding must equal principal plus accrued','/obligation');
  expectedAccounts=[signer,obligation.creditor];
 }
 if(!Array.isArray(r.balances)||r.balances.length!==expectedAccounts.length)problem('BETA_SCENARIO_CELLS','Exact ordered balance cells required','/balances');
 const balances=(r.balances as unknown[]).map((v,index)=>{
  const b=record(v,['account','amount'],[],`/balances/${index}`);
  const account=str(b.account,`/balances/${index}/account`,true);
  if(account!==expectedAccounts[index])problem('BETA_SCENARIO_CELLS','Balance account/order mismatch',`/balances/${index}`);
  return {account,amount:uint(b.amount,`/balances/${index}/amount`)};
 });
 if(new Set(balances.map(x=>x.account)).size!==balances.length)problem('BETA_SCENARIO_CELLS','Duplicate balance identities','/balances');
 const a=record(r.allowance,['owner','remaining','spent'],[],'/allowance');
 const allowance={owner:str(a.owner,'/allowance/owner',true),remaining:uint(a.remaining,'/allowance/remaining'),spent:uint(a.spent,'/allowance/spent')};
 if(allowance.owner!==signer)problem('BETA_SCENARIO_CELLS','Allowance owner mismatch','/allowance');
 const work_remaining=uint(r.work_remaining,'/work_remaining'),work_spent=uint(r.work_spent,'/work_spent');
 if(BigInt(work_remaining)+BigInt(work_spent)>U||BigInt(allowance.remaining)+BigInt(allowance.spent)>U)problem('BETA_SCENARIO_ACCOUNTING','Counter total exceeds UInt128');
 if(r.replay!=='unused'&&r.replay!=='consumed')problem('BETA_SCENARIO_SCHEMA','Expected closed replay tag','/replay');
 const result:Scenario={domain,asset,head:str(r.head,'/head'),predecessor:str(r.predecessor,'/predecessor'),round:uint(r.round,'/round'),balances,allowance,replay:r.replay,work_remaining,work_spent,post_head:str(r.post_head,'/post_head'),...(obligation?{obligation}:{})};
 if(Object.hasOwn(r,'candidate_effects')){
  if(!Array.isArray(r.candidate_effects)||r.candidate_effects.length>16)problem('BETA_SCENARIO_EFFECTS','Candidate effect array exceeds closed S0 bounds','/candidate_effects');
  result.candidate_effects=r.candidate_effects as Record<string,unknown>[];
 }
 return result;
}
export function expand(source:string,actionName:string,scenarioText:string):Expanded{
 const sourceHash=typeof source==='string'?digest(source):'',scenarioHash=typeof scenarioText==='string'?digest(scenarioText):null;
 const failure=(status:BetaFailure['status'],diagnostics:Diagnostic[]):BetaFailure=>({qualification:'local-stipulation-only',status,diagnostics,publishedPost:null,publishedEffects:null,sourceHash,scenarioHash,...(status==='Unsupported'?{scenarioValidation:'NotAppliedUnsupported' as const}:{})});
 const analysis=analyze(source);
 if(analysis.diagnostics.length)return failure('AuthoringRejected',analysis.diagnostics);
 const action=analysis.actions.find(x=>x.name===actionName);
 if(!action)return failure('AuthoringRejected',[{code:'BETA_ACTION_UNKNOWN',message:`Unknown action ${actionName}`,span:{start:0,end:0}}]);
 if(action.support!=='LocalS0')return failure('Unsupported',[{code:'BETA_PROFILE_UNSUPPORTED',message:'Recognized authoring profile has no local execution; scenario was not schema-checked or applied',span:action.span}]);
 let generatedMap:FieldMap[]=[],generatedOrigins:Origin[]=[];
 try{
  const intentDecl=analysis.declarations.find(x=>x.name===action.intent)!;
  const intent=fields(intentDecl.value);
  const domain=economicId(field(intent,'domain'),'domain'),asset=economicId(field(intent,'asset'),'asset');
  const assetFields=entity(field(intent,'asset'),'asset');
  const signer=economicId(field(intent,'signer'),'account');
  const operation=field(intent,'operation');
  if(operation.tag!=='call')problem('BETA_SOURCE_SHAPE','Expected closed request');
  const call=operation as Extract<Value,{tag:'call'}>,args=call.args;
  const isRepay=call.name==='repay';
  const debtor=isRepay?economicId(field(args,'payer'),'account'):economicId(field(args,'from'),'account');
  const recipient=isRepay?'':economicId(field(args,'to'),'account'),feeRecipient=isRepay?'':economicId(field(args,'fee_to'),'account');
  const loan=isRepay?economicId(field(args,'obligation'),'obligation'):undefined;
  const s=scenario(scenarioText,isRepay,domain,asset,signer,isRepay?[]:[debtor,recipient,feeRecipient],loan);
  const amount=qty(field(args,isRepay?'amount':'value')),fee=isRepay?'0':qty(field(args,'fee'));
  const gross=(BigInt(amount)+BigInt(fee)).toString();
  const nonce=text(field(intent,'nonce')),preHead=text(field(intent,'pre_head'));
  const validity=field(intent,'valid');
  if(validity.tag!=='call')problem('BETA_SOURCE_SHAPE','Expected rounds');
  const validArgs=(validity as Extract<Value,{tag:'call'}>).args;
  const statement=isRepay?`repay obligation ${loan} payer ${debtor} amount ${amount} conversion identity`:`transfer from ${debtor} to ${recipient} fee_to ${feeRecipient} value ${amount} fee ${fee}`;
  const origins:Origin[]=[],fieldMap:FieldMap[]=[],chunks:string[]=[];
  generatedMap=fieldMap;generatedOrigins=origins;
  let byte=0;
  const origin=(entry:Omit<Origin,'id'>):string=>{
   if(origins.length>=2048)problem('BETA_ORIGIN_BOUND','Origins exceed 2048');
   const id=`o${origins.length}`;origins.push({id,...entry});return id;
  };
  const spanKey=(span:Span):string=>`${span.start}:${span.end}`;
  const declarations=new Map(analysis.declarations.map(d=>[spanKey(d.span),d]));
  const sourceMemo=new Map<string,string>(),rangeMemo=new Map<string,string>(),useMemo=new Map<Reference,string>(),definitionMemo=new Map<string,string>();
  const sourceOrigin=(span:Span):string=>{
   const key=spanKey(span),cached=sourceMemo.get(key);if(cached)return cached;
   const id=origin({kind:'source',span});sourceMemo.set(key,id);return id;
  };
  const definitionOrigin=(span:Span):string=>{
   const key=spanKey(span),cached=definitionMemo.get(key);if(cached)return cached;
   const declaration=declarations.get(key)!;
   const id=origin({kind:'derived',rule:'declaration-definition',dependencies:[]});definitionMemo.set(key,id);
   origins[Number(id.slice(1))].dependencies=[rangeOrigin(span),rangeOrigin(declaration.value.span)];return id;
  };
  const useOrigin=(ref:Reference):string=>{
   const cached=useMemo.get(ref);if(cached)return cached;
   const id=origin({kind:'source',span:ref.useSpan,dependencies:[definitionOrigin(ref.declarationSpan)]});useMemo.set(ref,id);return id;
  };
  const rangeOrigin=(span:Span):string=>{
   const key=spanKey(span),cached=rangeMemo.get(key);if(cached)return cached;
   const refs=analysis.references.filter(r=>r.useSpan.start>=span.start&&r.useSpan.end<=span.end);
   const base=sourceOrigin(span);
   if(!refs.length){rangeMemo.set(key,base);return base;}
   const id=origin({kind:'derived',rule:'resolved-reference-use-and-definition',dependencies:[]});rangeMemo.set(key,id);
   origins[Number(id.slice(1))].dependencies=[base,...refs.map(useOrigin)];return id;
  };
  const src=(v:Value):string=>rangeOrigin(v.span);
  const fieldOrigin=(container:Value,key:string):string=>{
   const use=analysis.fieldUses.find(f=>spanKey(f.containerSpan)===spanKey(container.span)&&f.key===key);
   return use?rangeOrigin(use.span):src(field(fields(container),key));
  };
  const intentOrigin=(key:string):string=>fieldOrigin(intentDecl.value,key);
  const sc=(pointer:string):string=>origin({kind:'scenario',pointer});
  const derived=(rule:string,dependencies:string[]):string=>origin({kind:'derived',rule,dependencies});
  const gen=origin({kind:'generated',rule:'beta1-to-source6-s0'});
  const line=(fieldName:string,lineText:string,inputOrigins:string[]):void=>{
   const textLine=lineText+'\n',next=byte+Buffer.byteLength(textLine);
   if(next>65536)problem('BETA_EXPANSION_BOUND','Generated Source/6 exceeds 65536 bytes');
   chunks.push(textLine);fieldMap.push({field:fieldName,generatedSpan:{start:byte,end:next},origins:inputOrigins});byte=next;
  };
  const q=JSON.stringify;
  line('profile','profile "moriarty-financial-agreement-source/6";',[gen]);
  line('agreement',`agreement ${analysis.agreement} {`,[origin({kind:'source',span:analysis.agreementSpan!})]);
  line('domain',`domain ${domain};`,[intentOrigin('domain')]);
  line('settlement',`settlement ${asset} scale ${scalar(field(assetFields,'scale'))};`,[intentOrigin('asset'),fieldOrigin(field(intent,'asset'),'scale')]);
  line('selected',`selected ${isRepay?'RepayAccrualFirst':'TransferLiteralFee'} source_hash ${q(text(field(intent,'source_hash')))} digest ${q(text(field(intent,'policy_digest')))};`,[gen,intentOrigin('operation'),intentOrigin('source_hash'),intentOrigin('policy_digest')]);
  line('intent','intent {',[src(intentDecl.value)]);
  line('intent.signer',`signer ${signer} key ${q(text(field(intent,'key')))};`,[intentOrigin('signer'),intentOrigin('key')]);
  line('intent.nonce',`nonce ${q(nonce)};`,[intentOrigin('nonce')]);
  line('intent.pre_head',`pre_head ${q(preHead)};`,[intentOrigin('pre_head')]);
  line('intent.valid',`valid ${scalar(field(validArgs,'from'))}..${scalar(field(validArgs,'to'))};`,[intentOrigin('valid')]);
  for(const key of ['gross_cap','fee_cap','net_floor'])line(`intent.${key}`,`${key} ${qty(field(intent,key))};`,[intentOrigin(key)]);
  line('intent.failure','failure success_only;',[intentOrigin('failure')]);
  line('intent.signed_action',`signed_action ${statement};`,[intentOrigin('operation')]);
  for(const key of ['observations','disclosures','retained_effects','retained_duties'])line(`intent.${key}`,`${key} empty;`,[intentOrigin(key)]);
  for(const key of ['delegation','recovery'])line(`intent.${key}`,`${key} none;`,[intentOrigin(key)]);
  line('intent.end','}',[gen]);line('authenticated','authenticated {',[gen]);
  for(const key of ['head','predecessor'])line(`scenario.${key}`,`${key} ${q(s[key as 'head'|'predecessor'])};`,[sc(`/${key}`)]);
  line('scenario.round',`round ${s.round};`,[sc('/round')]);
  s.balances.forEach((b,i)=>line(`scenario.balances.${i}`,`balance ${b.account} ${b.amount};`,[sc(`/balances/${i}`)]));
  line('scenario.allowance',`allowance ${s.allowance.owner} remaining ${s.allowance.remaining} spent ${s.allowance.spent};`,[sc('/allowance')]);
  if(s.obligation){
   const d=s.obligation;line('scenario.obligation',`obligation ${d.id} {`,[sc('/obligation/id')]);
   for(const key of ['debtor','creditor','asset','principal','accrued','outstanding'] as const)line(`scenario.obligation.${key}`,`${key} ${d[key]};`,[sc(`/obligation/${key}`)]);
   line('scenario.obligation.status','status outstanding;',[sc('/obligation/status')]);line('scenario.obligation.end','}',[gen]);
  }
  line('scenario.replay',`replay ${s.replay};`,[sc('/replay')]);
  line('scenario.work_remaining',`work_remaining ${s.work_remaining};`,[sc('/work_remaining')]);
  line('scenario.work_spent',`work_spent ${s.work_spent};`,[sc('/work_spent')]);
  line('authenticated.end','}',[gen]);line('submit',`submit ${statement};`,[intentOrigin('operation')]);line('effects','effects {',[gen]);
  const opOrigin=intentOrigin('operation'),proposalOrigins=[opOrigin,...['domain','signer','nonce','pre_head'].map(k=>intentOrigin(k)),sc('/post_head'),sc('/balances'),...(isRepay?[sc('/obligation')]:[])];
  let proposalNote:string|null=null;
  let effects:Record<string,unknown>[];
  if(isRepay){
   const d=s.obligation!,n=BigInt(amount),a=BigInt(d.accrued),p=BigInt(d.principal);
   const over=n>BigInt(d.outstanding),da=n<a?n:a,dp=n-da;
   const principal=over?d.principal:(p-dp).toString(),accrued=over?d.accrued:(a-da).toString();
   const outstanding=over?d.outstanding:(BigInt(principal)+BigInt(accrued)).toString();
   if(over&&!s.candidate_effects)proposalNote='Invalid overpayment proposal preserves unchanged obligation; only Core determines the first failure.';
   effects=[{kind:'Debit',account:debtor,asset,amount},{kind:'Credit',account:d.creditor,asset,amount},{kind:'SetObligation',id:d.id,principal,accrued,outstanding,status:outstanding==='0'?'Settled':'Outstanding'}];
  }else effects=[{kind:'Debit',account:debtor,asset,amount:gross},{kind:'Credit',account:recipient,asset,amount},...(BigInt(fee)>0n?[{kind:'Credit',account:feeRecipient,asset,amount:fee}]:[])];
  effects.push({kind:'UseAllowance',owner:signer,amount:gross},{kind:'UseReplay',key:nonce},{kind:'AdvanceHead',predecessor:preHead,successor:s.post_head});
  if(s.candidate_effects)effects=s.candidate_effects;
  effects.forEach((value,i)=>{
   const ptr=`/candidate_effects/${i}`,kind=String(value?.kind),keys:Record<string,string[]>={Debit:['kind','account','asset','amount'],Credit:['kind','account','asset','amount'],SetObligation:['kind','id','principal','accrued','outstanding','status'],UseAllowance:['kind','owner','amount'],UseReplay:['kind','key'],AdvanceHead:['kind','predecessor','successor']};
   if(!Object.hasOwn(keys,kind))problem('BETA_SCENARIO_EFFECTS','Unknown effect kind',ptr);
   const e=record(value,keys[kind],[],ptr);let emission='';
   if(kind==='Debit'||kind==='Credit'){
    if(str(e.asset,`${ptr}/asset`,true)!==asset)problem('BETA_SCENARIO_IDENTITY','Candidate asset cannot be represented by this Source/6 envelope',ptr);
    emission=`${kind.toLowerCase()} ${str(e.account,`${ptr}/account`,true)} ${uint(e.amount,`${ptr}/amount`)};`;
   }else if(kind==='SetObligation'){
    if(e.status!=='Outstanding'&&e.status!=='Settled')problem('BETA_SCENARIO_EFFECTS','Unknown obligation status',ptr);
    emission=`set_obligation ${str(e.id,`${ptr}/id`,true)} principal ${uint(e.principal,ptr,S)} accrued ${uint(e.accrued,ptr,S)} outstanding ${uint(e.outstanding,ptr,S)} status ${String(e.status).toLowerCase()};`;
   }else if(kind==='UseAllowance')emission=`use_allowance ${str(e.owner,ptr,true)} ${uint(e.amount,ptr)};`;
   else if(kind==='UseReplay')emission=`use_replay ${q(str(e.key,ptr))};`;
   else emission=`advance_head ${q(str(e.predecessor,ptr))} ${q(str(e.successor,ptr))};`;
   line(`effects.${i}`,emission,s.candidate_effects?[sc(ptr)]:[derived(isRepay?'accrual-first-proposal':'literal-transfer-proposal',proposalOrigins)]);
  });
  line('effects.end','}',[gen]);line('post_head',`post_head ${q(s.post_head)};`,[sc('/post_head')]);line('agreement.end','}',[gen]);
  const source6=chunks.join('');parseAndLowerSource6(source6);
  const result:BetaExpansion={status:'Expanded',sourceHash,scenarioHash:scenarioHash!,source6,fieldMap,origins,proposalNote,qualification:'local-stipulation-only'};
  if(Buffer.byteLength(JSON.stringify(result))>524288)problem('BETA_EXPANSION_BOUND','Serialized expansion exceeds 524288 bytes');
  return result;
 }catch(error){
  if(error instanceof LocalError)return failure('FormationRejected',[{code:error.code,message:`${error.message} (${error.pointer}); source span identifies the action, not the scenario location`,span:action.span}]);
  if(error instanceof Source6Error){
   const mapped=generatedMap.find(m=>m.generatedSpan.start<=error.offset&&error.offset<m.generatedSpan.end);
   const related=generatedOrigins.filter(o=>mapped?.origins.includes(o.id));
   const sourceOrigin=related.find(o=>o.kind==='source'&&o.span);
   const pointers=related.filter(o=>o.kind==='scenario').map(o=>o.pointer).join(',');
   return failure('FormationRejected',[{code:error.code,message:`Generated Source/6 formation: ${error.message}; field ${mapped?.field??'unknown'}; byte ${error.offset}${pointers?`; scenario ${pointers}`:''}`,span:sourceOrigin?.span??action.span}]);
  }
  throw error;
 }
}
export function simulate(source:string,action:string,scenarioText:string):Simulated{
 const expansion=expand(source,action,scenarioText);if(expansion.status!=='Expanded')return expansion;
 const result=prepareSource6S0Unqualified(expansion.source6);
 return {status:result.status,sourceHash:expansion.sourceHash,scenarioHash:expansion.scenarioHash,result,qualification:'local-stipulation-only'};
}
