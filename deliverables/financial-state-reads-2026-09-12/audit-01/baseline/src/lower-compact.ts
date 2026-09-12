import {compile} from './frontend.ts';
import {canonicalEncode,sha256} from './codec.ts';
import type {BoundProgram,CoreExpression,CoreInstruction,LocalType,NamedStoredValue,SourceRef,StoredValue,UnitVector} from './types.ts';
const MAX=(1n<<128n)-1n,B=1n<<64n;
export class MappingError extends Error {code:string;constructor(code:string){super(code);this.name='MappingError';this.code=code;}}
function requireMapping(ok:unknown,code:string):asserts ok{if(!ok)throw new MappingError(code);}
const compareUtf8=(a:string,b:string)=>{const x=new TextEncoder().encode(a),y=new TextEncoder().encode(b);for(let i=0;i<Math.min(x.length,y.length);i++)if(x[i]!==y[i])return x[i]-y[i];return x.length-y.length;};
const clone=<T>(x:T):T=>structuredClone(x);
export type CompactMapping={schemaVersion:'moriarty-compact-mapping/1';source:string;bound:BoundProgram;metadata:{schemaVersion:'moriarty-compact-kernel-metadata/1';programHash:string;sourceHash:string;boundsHash:string;compactSourceHash:string;compiler:'0.31.1';language:'0.23.0';runtime:'0.16.0';textTable:{id:string;text:string}[];stateFields:{field:string;name:string;type:LocalType}[];observationFields:{field:string;name:string;type:LocalType}[];actions:{name:string;circuit:string;arguments:{field:string;name:string;type:LocalType}[];hints:{field:string;nodeId:string;kind:'Mul'|'FloorDiv'}[];effects:{field:string;kind:string;ordinal:string;fields:{field:string;label:string;type:LocalType;unitVector:UnitVector}[]}[]}[];sourceMap:{action:string;coreNodeId:string;generatedLine:string;sourceRef:SourceRef;type:LocalType;unitVector:UnitVector}[];restrictions:string[]}};
function children(e:CoreExpression):CoreExpression[]{if(e.tag==='FloorDiv')return [e.numerator,e.denominator];if('left'in e)return [e.left,e.right];if(e.tag==='Not')return [e.operand];return [];}
function compactType(type:LocalType):string{return type.tag==='Bool'?'Boolean':type.tag==='Text'?'Uint<32>':'Uint<128>';}
/** Restricted deterministic Core kernel. No package-name dispatch, host-computed
 * final state, financial ledger surrogate, proof generation or acceptance. */
export function lowerCompact(source:string|Uint8Array,bounds:string|Uint8Array):CompactMapping{
 const bound=compile(source,bounds).bound,m=bound.manifest;
 requireMapping(m.stateSchema.every(x=>x.type.tag!=='Text'),'MAPPING_PERSISTENT_TEXT_UNSUPPORTED');
 requireMapping(m.observationSchema.every(x=>x.type.tag!=='Text'),'MAPPING_OBSERVATION_TEXT_UNSUPPORTED');
 const strings=new Set<string>(m.constants.filter(x=>x.value.tag==='Text').map(x=>x.value.value));
 function visit(e:CoreExpression){requireMapping(e.tag!=='And'&&e.tag!=='Or','MAPPING_SHORT_CIRCUIT_UNSUPPORTED');if(e.tag==='Literal'&&e.value.tag==='Text')strings.add(e.value.value);children(e).forEach(visit);}
 for(const action of m.core.actions)for(const ins of action.instructions){if(ins.tag==='Guard')visit(ins.condition);else if(ins.tag==='Emit')ins.fields.forEach(x=>visit(x.expression));else visit(ins.expression);}
 const textTable=[...strings].sort(compareUtf8).map((text,i)=>({id:String(i),text}));
 const textId=(s:string)=>{const found=textTable.find(x=>x.text===s);requireMapping(found,'MAPPING_TEXT_UNKNOWN');return found.id;};
 const knownText=(e:CoreExpression):string|undefined=>e.tag==='Literal'&&e.value.tag==='Text'?e.value.value:e.tag==='ConstRef'&&m.constants.find(x=>x.name===e.declaration)?.value.tag==='Text'?m.constants.find(x=>x.name===e.declaration)!.value.value:undefined;
 for(const action of m.core.actions)for(const arg of action.arguments.filter(x=>x.type.tag==='Text')){
  const matching=action.instructions.filter(x=>x.tag==='Guard').map(x=>x.condition).filter(e=>e.tag==='Eq'&&((e.left.tag==='ArgRef'&&e.left.declaration===arg.name&&knownText(e.right)!==undefined)||(e.right.tag==='ArgRef'&&e.right.declaration===arg.name&&knownText(e.left)!==undefined)));
  requireMapping(matching.length>0,'MAPPING_TEXT_ARGUMENT_UNGUARDED');
 }
 const stateFields=m.stateSchema.map((x,i)=>({field:'f'+i,...clone(x)})),observationFields=m.observationSchema.map((x,i)=>({field:'o'+i,...clone(x)}));
 const metadata:CompactMapping['metadata']={schemaVersion:'moriarty-compact-kernel-metadata/1',programHash:bound.programHash,sourceHash:m.sourceHash,boundsHash:m.bounds.boundsHash,compactSourceHash:'',compiler:'0.31.1',language:'0.23.0',runtime:'0.16.0',textTable,stateFields,observationFields,actions:[],sourceMap:[],restrictions:['Core arithmetic/guards/writes/typed effect operands only; no obligation-ledger or authority/proof/settlement acceptance relation.','Persistent Text and observation Text are unsupported. Every Text argument requires an exact top-level equality guard against a Text literal or constant.','And/Or are rejected; initial mapping does not implement short-circuit circuits.','Text IDs are collision-free indices in the program-bound sorted UTF8 text table, not hashes or external identities.','Amount and Quantity arithmetic uses UInt128; exact nominal unit vectors and policy/source bindings remain in the companion metadata and bound program.','Hints are untrusted function inputs constrained by checkedMul/checkedDiv. Host hint preparation does not compute accepted output.']};
 const lines=['// Generated restricted Moriarty Core kernel; no ledger effects or accepted PCD.',`// sourceHash ${m.sourceHash}`,`// programHash ${bound.programHash}`,'include "arithmetic";','export struct DivisionHint { q: Uint<128>; r: Uint<128>; product: MulHint; }'];
 const emit=(line:string)=>lines.push(line);
 const struct=(name:string,fields:{field:string;type:string}[])=>emit(`export struct ${name} { ${fields.map(f=>`${f.field}: ${f.type};`).join(' ')} }`);
 struct('KernelState',stateFields.map(x=>({field:x.field,type:compactType(x.type)})));struct('KernelObservations',observationFields.map(x=>({field:x.field,type:compactType(x.type)})));
 for(let ai=0;ai<m.core.actions.length;ai++){
  const action=m.core.actions[ai],argFields=action.arguments.map((x,i)=>({field:'a'+i,...clone(x)}));
  const meta:CompactMapping['metadata']['actions'][number]={name:action.name,circuit:'transition'+ai,arguments:argFields,hints:[],effects:[]};metadata.actions.push(meta);
  function hints(e:CoreExpression){children(e).forEach(hints);if(e.tag==='Mul'||e.tag==='FloorDiv')meta.hints.push({field:'h'+meta.hints.length,nodeId:e.nodeId,kind:e.tag});}
  for(const ins of action.instructions){if(ins.tag==='Guard')hints(ins.condition);else if(ins.tag==='Emit')ins.fields.forEach(x=>hints(x.expression));else hints(ins.expression);}
  struct('Arguments'+ai,argFields.map(x=>({field:x.field,type:compactType(x.type)})));
  struct('Hints'+ai,meta.hints.length?meta.hints.map(h=>({field:h.field,type:h.kind==='Mul'?'MulHint':'DivisionHint'})):[{field:'unused',type:'Uint<1>'}]);
  for(const ins of action.instructions)if(ins.tag==='Emit'){
   const index=meta.effects.length;const fields=ins.fields.map((f,j)=>({field:'v'+j,label:f.label,type:clone(f.expression.type),unitVector:clone(f.expression.unitVector)}));
   meta.effects.push({field:'effect'+index,kind:ins.kind,ordinal:ins.ordinal,fields});struct(`Effect${ai}_${index}`,fields.map(f=>({field:f.field,type:compactType(f.type)})));
  }
  struct('Result'+ai,[{field:'after',type:'KernelState'},{field:'remaining',type:'Uint<128>'},{field:'revision',type:'Uint<128>'},...meta.effects.map((e,j)=>({field:e.field,type:`Effect${ai}_${j}`}))]);
  emit(`export pure circuit ${meta.circuit}(before: KernelState, args: Arguments${ai}, observations: KernelObservations, remaining: Uint<128>, revision: Uint<128>, hints: Hints${ai}): Result${ai} {`);
  emit(`  assert(checkedAdd(revision, remaining) == ${m.lifetime}, "LIFETIME_INVARIANT");`);emit('  assert(remaining > 0, "LIFETIME_EXHAUSTED");');emit(`  assert(observations.${observationFields.find(x=>x.name==='now')!.field} < ${m.horizon}, "HORIZON_EXPIRED");`);
  const state=new Map(stateFields.map(x=>[x.name,{name:'before.'+x.field,floors:new Set<string>()}])),locals=new Map<string,{name:string;floors:Set<string>}>();let exprIndex=0,effectIndex=0;
  const effects:string[]=[];
  function literal(v:StoredValue|Extract<CoreExpression,{tag:'Literal'}>['value']):string{return v.tag==='Text'?textId(v.value):v.tag==='Bool'?String(v.value):v.value;}
  function expression(e:CoreExpression):{name:string;floors:Set<string>}{
   let body:string;let floors=new Set<string>();
   if(e.tag==='Literal')body=literal(e.value);
   else if(e.tag==='Remaining')body='remaining';
   else if(e.tag==='ArgRef')body='args.'+argFields.find(x=>x.name===e.declaration)!.field;
   else if(e.tag==='ObservationRef')body='observations.'+observationFields.find(x=>x.name===e.declaration)!.field;
   else if(e.tag==='ConstRef')body=literal(m.constants.find(x=>x.name===e.declaration)!.value);
   else if(e.tag==='StateRef'){const v=state.get(e.declaration)!;body=v.name;floors=new Set(v.floors);}
   else if(e.tag==='LocalRef'){const v=locals.get(e.local)!;body=v.name;floors=new Set(v.floors);}
   else if(e.tag==='Not')body='!'+expression(e.operand).name;
   else {
    const a=expression(e.tag==='FloorDiv'?e.numerator:e.left),b=expression(e.tag==='FloorDiv'?e.denominator:e.right);floors=new Set([...a.floors,...b.floors]);
    if(e.tag==='Add'||e.tag==='Sub')body=`${e.tag==='Add'?'checkedAdd':'checkedSub'}(${a.name}, ${b.name})`;
    else if(e.tag==='Mul')body=`checkedMul(${a.name}, ${b.name}, hints.${meta.hints.find(h=>h.nodeId===e.nodeId)!.field})`;
    else if(e.tag==='FloorDiv'){const h='hints.'+meta.hints.find(h=>h.nodeId===e.nodeId)!.field;body=`checkedDiv(${a.name}, ${b.name}, ${h}.q, ${h}.r, ${h}.product)`;floors.add(e.nodeId);}
    else {const op={Eq:'==',Lt:'<',Lte:'<=',Gt:'>',Gte:'>='}[e.tag as 'Eq'|'Lt'|'Lte'|'Gt'|'Gte'];requireMapping(op,'MAPPING_EXPRESSION_UNSUPPORTED');body=`${a.name} ${op} ${b.name}`;floors.clear();}
   }
   const name='e'+exprIndex++;metadata.sourceMap.push({action:action.name,coreNodeId:e.nodeId,generatedLine:String(lines.length+1),sourceRef:clone(e.sourceRef),type:clone(e.type),unitVector:clone(e.unitVector)});emit(`  const ${name}: ${compactType(e.type)} = ${body};`);return {name,floors};
  }
  function policy(name:string,floors:Set<string>){const p=m.policies.find(x=>x.name===name)!;requireMapping(p.roundingNode.tag==='None'?floors.size===0:floors.size===1&&floors.has(p.roundingNode.coreNodeId),'MAPPING_POLICY_PROVENANCE');}
  for(const ins of action.instructions){
   if(ins.tag==='Guard')emit(`  assert(${expression(ins.condition).name}, ${JSON.stringify(ins.message)});`);
   else if(ins.tag==='Let')locals.set(ins.name,expression(ins.expression));
   else if(ins.tag==='Set'){const v=expression(ins.expression);if(ins.policy.tag==='Financial')policy(ins.policy.name,v.floors);state.set(ins.field,v);}
   else {const fields=ins.fields.map((f,j)=>{const v=expression(f.expression),p=ins.policies.find(x=>x.field===f.label);if(p)policy(p.policy,v.floors);return `v${j}: ${v.name}`;});effects.push(`effect${effectIndex}: Effect${ai}_${effectIndex} { ${fields.join(', ')} }`);effectIndex++;}
  }
  emit(`  return Result${ai} { after: KernelState { ${stateFields.map(f=>`${f.field}: ${state.get(f.name)!.name}`).join(', ')} }, remaining: checkedSub(remaining, 1), revision: checkedAdd(revision, 1)${effects.length?', '+effects.join(', '):''} };`);emit('}');
 }
 const generated=lines.join('\n')+'\n';requireMapping(new TextEncoder().encode(generated).length<=1048576,'MAPPING_OUTPUT_BOUNDS');metadata.compactSourceHash=sha256(generated);
 return {schemaVersion:'moriarty-compact-mapping/1',source:generated,bound,metadata};
}
/** Advisory witness preparation only. Generated checked circuits independently
 * constrain every limb and quotient/remainder against their dynamic operands. */
function mulHint(a:bigint,b:bigint){return {aLo:a%B,aHi:a/B,bLo:b%B,bHi:b/B,lo:(a%B)*(b%B)%B,carry:(a%B)*(b%B)/B};}
export function prepareCompactCall(mapping:CompactMapping,actionName:string,state:NamedStoredValue[],args:NamedStoredValue[],observations:NamedStoredValue[],remaining:string,revision:string){
 const meta=mapping.metadata.actions.find(a=>a.name===actionName),action=mapping.bound.manifest.core.actions.find(a=>a.name===actionName);requireMapping(meta&&action,'MAPPING_ACTION_UNKNOWN');
 const number=(v:StoredValue)=>{if(v.tag==='Text'){const item=mapping.metadata.textTable.find(x=>x.text===v.value);requireMapping(item,'MAPPING_TEXT_UNKNOWN');return BigInt(item.id);}requireMapping(/^(0|[1-9][0-9]*)$/.test(v.value),'MAPPING_INPUT_RANGE');return BigInt(v.value);};
 const encode=(fields:{field:string;name:string;type:LocalType}[],values:NamedStoredValue[])=>{requireMapping(fields.length===values.length,'MAPPING_INPUT_SCHEMA');const out:Record<string,bigint>={};fields.forEach((f,i)=>{const v=values[i];requireMapping(v.name===f.name&&v.value.tag===f.type.tag&&(f.type.tag!=='Amount'||v.value.tag==='Amount'&&v.value.unit===f.type.unit),'MAPPING_INPUT_SCHEMA');const n=number(v.value);requireMapping(n>=0n&&n<=MAX,'MAPPING_INPUT_RANGE');out[f.field]=n;});return out;};
 requireMapping(/^(0|[1-9][0-9]*)$/.test(remaining)&&/^(0|[1-9][0-9]*)$/.test(revision)&&BigInt(remaining)<=MAX&&BigInt(revision)<=MAX,'MAPPING_INPUT_RANGE');
 const encodedState=encode(mapping.metadata.stateFields,state),encodedArgs=encode(meta.arguments,args),encodedObs=encode(mapping.metadata.observationFields,observations);
 const st=new Map(state.map(x=>[x.name,number(x.value)])),ar=new Map(args.map(x=>[x.name,number(x.value)])),ob=new Map(observations.map(x=>[x.name,number(x.value)])),lo=new Map<string,bigint|boolean>();
 const hints:Record<string,unknown>=meta.hints.length?{}:{unused:0n};
 function expr(e:CoreExpression):bigint|boolean{
  if(e.tag==='Literal')return e.value.tag==='Bool'?e.value.value:number(e.value as StoredValue);
  if(e.tag==='Remaining')return BigInt(remaining);if(e.tag==='StateRef')return st.get(e.declaration)!;if(e.tag==='ArgRef')return ar.get(e.declaration)!;if(e.tag==='ObservationRef')return ob.get(e.declaration)!;if(e.tag==='ConstRef')return number(mapping.bound.manifest.constants.find(x=>x.name===e.declaration)!.value);if(e.tag==='LocalRef')return lo.get(e.local)!;if(e.tag==='Not')return !expr(e.operand);
  const a=expr(e.tag==='FloorDiv'?e.numerator:e.left),b=expr(e.tag==='FloorDiv'?e.denominator:e.right);
  if(e.tag==='Eq')return a===b;if(e.tag==='Lt')return a<b;if(e.tag==='Lte')return a<=b;if(e.tag==='Gt')return a>b;if(e.tag==='Gte')return a>=b;
  requireMapping(typeof a==='bigint'&&typeof b==='bigint','MAPPING_HINT_TYPE');
  if(e.tag==='Add')return a+b;if(e.tag==='Sub')return a-b;
  const h=meta!.hints.find(x=>x.nodeId===e.nodeId);requireMapping(h,'MAPPING_HINT_TYPE');
  if(e.tag==='Mul'){hints[h.field]=mulHint(a,b);return a*b;}
  requireMapping(b!==0n,'MAPPING_HINT_DIVISION_BY_ZERO');const q=a/b,r=a%b;hints[h.field]={q,r,product:mulHint(b,q)};return q;
 }
 for(const ins of action.instructions){if(ins.tag==='Guard')expr(ins.condition);else if(ins.tag==='Let')lo.set(ins.name,expr(ins.expression));else if(ins.tag==='Set')st.set(ins.field,expr(ins.expression) as bigint);else ins.fields.forEach(x=>expr(x.expression));}
 return {circuit:meta.circuit,arguments:[encodedState,encodedArgs,encodedObs,BigInt(remaining),BigInt(revision),hints] as [Record<string,bigint>,Record<string,bigint>,Record<string,bigint>,bigint,bigint,Record<string,unknown>]};
}
export function decodeCompactResult(mapping:CompactMapping,actionName:string,result:any){
 const action=mapping.metadata.actions.find(a=>a.name===actionName);requireMapping(action,'MAPPING_ACTION_UNKNOWN');
 const decode=(type:LocalType,v:bigint):StoredValue=>{requireMapping(typeof v==='bigint'&&v>=0n&&v<=MAX,'MAPPING_RESULT_RANGE');if(type.tag==='Text'){const t=mapping.metadata.textTable.find(x=>x.id===String(v));requireMapping(t,'MAPPING_TEXT_UNKNOWN');return {tag:'Text',value:t.text};}requireMapping(type.tag==='UInt128'||type.tag==='Amount','MAPPING_RESULT_TYPE');return type.tag==='Amount'?{tag:'Amount',unit:type.unit,value:String(v)}:{tag:'UInt128',value:String(v)};};
 const values=mapping.metadata.stateFields.map(f=>({name:f.name,value:decode(f.type,result.after[f.field])}));
 const effects=action.effects.map(e=>{const record:Record<string,unknown>={kind:e.kind,ordinal:e.ordinal};for(const f of e.fields){const v=decode(f.type,result[e.field][f.field]);record[f.label==='due_id'?'dueId':f.label]=v.tag==='Amount'?v:v.value;}return record;});
 return {values,effects,remaining:String(result.remaining),revision:String(result.revision)};
}
/** Test-only public snapshot wrapper. These ledger cells retain kernel outputs
 * solely to exercise compiler ZKIR generation; they are not asset settlement. */
export function compactSnapshotHarness(mapping:CompactMapping,includeName='kernel'):string{
 requireMapping(/^[A-Za-z][A-Za-z0-9_-]*$/.test(includeName),'MAPPING_INCLUDE_NAME');
 const lines=['// TEST HARNESS ONLY: stores kernel output, never moves financial assets.',`include "${includeName}";`];
 mapping.metadata.actions.forEach((a,i)=>{
  lines.push(`export ledger snapshot${i}: Result${i};`);
  lines.push(`export circuit record${i}(before: KernelState, args: Arguments${i}, observations: KernelObservations, remaining: Uint<128>, revision: Uint<128>, hints: Hints${i}): [] { snapshot${i} = disclose(${a.circuit}(before, args, observations, remaining, revision, hints)); }`);
 });
 return lines.join('\n')+'\n';
}
