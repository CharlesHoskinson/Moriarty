import type { ActionDecl, BoundProgram, Bounds, CoreAction, CoreExpression, CoreInstruction, EffectKind, EffectType, FieldPolicy, FieldPolicyDecl, LocalType, LocalValue, NamedStoredValue, NamedType, Resolution, SemanticManifest, SourceAST, SourceExpression, SourceLiteral, SourceRef, SourceType, Span, StoredType, StoredValue, TypedAnnotation, TypedProgram, UnitVector } from './types.ts';
import { canonicalEncode, checkEncoding, fail, hashDomain, hashRawDomain, sourceText, utf8 } from './codec.ts';
import { PROFILE, checkUInt } from './parser.ts';
const numeric=(t:LocalType)=>t.tag==='UInt128'||t.tag==='Amount'||t.tag==='Quantity';
const vector=(t:LocalType):UnitVector=>t.tag==='Amount'?[{exponent:'1',unit:t.unit}]:t.tag==='Quantity'?t.unitVector:[];
const same=(a:LocalType,b:LocalType):boolean=>canonicalEncode(a)===canonicalEncode(b);
const standardEffects:Record<EffectKind,string[]>={Transfer:['asset','from','to','amount'],Fee:['asset','from','to','amount'],DueCreated:['due_id','debtor','creditor','denomination','amount'],DueSettled:['due_id','debtor','creditor','denomination','amount','asset']};
function limit(count:number,max:number,label:string,span?:Span):void{if(!Number.isSafeInteger(max)||max<0)fail('INVALID_BOUNDS',label);if(count>max)fail('SHAPE_BOUND',`${label}: ${count} > ${max}`,span);}
function unique<T>(map:Map<string,T>,key:string,value:T,span:Span):void{if(map.has(key))fail('DUPLICATE_DECLARATION',key,span);map.set(key,value);}
function resolve<T>(map:Map<string,T>,key:string,span:Span):T{const value=map.get(key);if(value===undefined)fail('UNRESOLVED_NAME',key,span);return value;}
function stored(value:LocalValue,where:Span):StoredValue {if(value.tag==='Bool'||value.tag==='Quantity')fail('NON_STORED_VALUE',value.tag,where);return value;}
type Target={action:string;statementIndex:number;expression:CoreExpression;span:Span;attach:(policy:string)=>void};
type LocalEntry={type:LocalType;expression:CoreExpression;statementIndex:number};
export function checkAndLower(source:SourceAST,boundsBytes:Uint8Array):{typed:TypedProgram;bound:BoundProgram;metrics:Record<string,ReturnType<typeof checkEncoding>>} {
  let bounds:Bounds;try{bounds=JSON.parse(sourceText(boundsBytes)) as Bounds;}catch{fail('INVALID_BOUNDS','bounds JSON cannot be decoded');}
  if(bounds.schemaVersion!=='moriarty-bounds/1'||bounds.semanticProfile!==PROFILE)fail('INVALID_BOUNDS','wrong bounds registry/profile');
  const shape=bounds.programShape;
  limit(Number(source.sourceUtf8Bytes),bounds.sourceEncoding.sourceUtf8Bytes,'sourceUtf8Bytes',source.span);
  if(checkUInt(source.lifetime.token,source.lifetime.span)==='0')fail('LIFETIME_ZERO','lifetime must be positive',source.lifetime.span);
  checkUInt(source.horizon.token,source.horizon.span);
  const astMetrics=checkEncoding(source,bounds.astEncoding,'SourceAST');
  const units=new Map<string,true>(), constants=new Map<string,StoredType>(), states=new Map<string,StoredType>(), observations=new Map<string,StoredType>();
  const policies=new Map<string,FieldPolicyDecl>(),effectSchemas=new Map<EffectKind,{label:string;type:EffectType}[]>(),actions=new Map<string,CoreAction>(),actionSources=new Map<string,ActionDecl>();
  const actionLocals=new Map<string,Map<string,LocalEntry>>(),targets=new Map<string,Target>(),settlementNames=new Map<string,true>();
  const bindings:SemanticManifest['settlementBindings']=[],initialState:NamedStoredValue[]=[],constantValues:NamedStoredValue[]=[],annotations:TypedAnnotation[]=[];
  const reserves:Extract<SourceAST['declarations'][number],{tag:'ReserveDecl'}>[]=[];
  let episode:SemanticManifest['statusRules']['episode']|undefined,agreement:SemanticManifest['statusRules']['agreement']|undefined;
  const ref=(span:Span):SourceRef=>({generatedTag:'Source',sourceHash:source.sourceHash,spans:[span]});
  function type(t:SourceType,effect=false):EffectType {if(t.tag==='Amount'){resolve(units,t.unit,t.span);return {tag:'Amount',unit:t.unit};}if(t.tag==='BareAmount'){if(!effect)fail('NON_STORED_TYPE','bare Amount only allowed in effect declarations',t.span);return {tag:'AmountFromOperand'};}return {tag:t.tag};}
  function storedType(t:SourceType):StoredType{const result=type(t);if(result.tag==='AmountFromOperand')fail('NON_STORED_TYPE','bare Amount',t.span);return result;}
  function literal(l:SourceLiteral):{type:LocalType;value:LocalValue} {
    if(l.tag==='UIntLiteral')return {type:{tag:'UInt128'},value:{tag:'UInt128',value:checkUInt(l.token,l.span)}};
    if(l.tag==='AmountLiteral'){resolve(units,l.unit,l.span);return {type:{tag:'Amount',unit:l.unit},value:{tag:'Amount',unit:l.unit,value:checkUInt(l.token,l.span)}};}
    if(l.tag==='BoolLiteral')return {type:{tag:'Bool'},value:{tag:'Bool',value:l.value}};
    return {type:{tag:'Text'},value:{tag:'Text',value:l.decoded}};
  }
  function combine(left:LocalType,right:LocalType,subtract:boolean,span:Span):LocalType {
    if(!numeric(left)||!numeric(right))fail('TYPE_MISMATCH','numeric operands required',span);
    const entries=new Map<string,number>();for(const c of vector(left))entries.set(c.unit,Number(c.exponent));for(const c of vector(right))entries.set(c.unit,(entries.get(c.unit)??0)+(subtract?-1:1)*Number(c.exponent));
    const v:UnitVector=[...entries].filter(([,e])=>e!==0).sort(([a],[b])=>a<b?-1:a>b?1:0).map(([unit,exponent])=>({exponent:String(exponent),unit}));
    limit(v.length,shape.expressionUnitComponents,'expressionUnitComponents',span);for(const c of v)limit(Math.abs(Number(c.exponent)),shape.absoluteUnitExponent,'absoluteUnitExponent',span);
    if(v.length===0)return {tag:'UInt128'};if(v.length===1&&v[0].exponent==='1')return {tag:'Amount',unit:v[0].unit};return {tag:'Quantity',unitVector:v};
  }
  function action(sourceAction:ActionDecl):CoreAction {
    const actionIndex=actions.size;const args=new Map<string,StoredType>(),locals=new Map<string,LocalEntry>(),writes=new Set<string>();
    for(const parameter of sourceAction.parameters)unique(args,parameter.name,storedType(parameter.type),parameter.span);
    if(args.get('actor')?.tag!=='Text')fail('ACTOR_PARAMETER','each action requires actor: Text',sourceAction.span);
    limit(args.size,shape.argumentFieldsPerEntrypoint,'argumentFieldsPerEntrypoint',sourceAction.span);
    let expressionNodes=0,expressionDepth=0,effects=0;
    const instructions:CoreInstruction[]=[];
    sourceAction.statements.forEach((statement,statementIndex)=>{
      let preorder=0;
      const statementId=`a_${actionIndex}_s_${statementIndex}`;
      function expression(e:SourceExpression,depth=1):CoreExpression {
        expressionNodes++;expressionDepth=Math.max(expressionDepth,depth);
        limit(expressionNodes,shape.expressionNodesPerEntrypoint,'expressionNodesPerEntrypoint',e.span);limit(depth,shape.expressionDepthRootOne,'expressionDepthRootOne',e.span);
        const nodeId=`${statementId}_e_${preorder++}`,sourceRef=ref(e.span),slot=annotations.length;
        const annotation:TypedAnnotation={nodeId,resolution:{tag:'Operator'},sourceRef,type:{tag:'UInt128'},unitVector:[]};annotations.push(annotation);
        let result:CoreExpression;let resolution:Resolution={tag:'Operator'};
        const metadata=(t:LocalType)=>({nodeId,sourceRef,type:t,unitVector:vector(t)});
        if(e.tag==='Literal'){const v=literal(e.literal);resolution={tag:'Literal'};result={...metadata(v.type),tag:'Literal',value:v.value};}
        else if(e.tag==='Remaining'){resolution={tag:'Remaining'};result={...metadata({tag:'UInt128'}),tag:'Remaining'};}
        else if(e.tag==='LocalRef'){const entry=resolve(locals,e.name,e.span);resolution={action:sourceAction.name,local:e.name,tag:'Local'};result={...metadata(entry.type),action:sourceAction.name,local:e.name,tag:'LocalRef'};}
        else if(e.tag==='StateRef'||e.tag==='ArgRef'||e.tag==='ObservationRef'||e.tag==='ConstRef') {
          const tables={StateRef:states,ArgRef:args,ObservationRef:observations,ConstRef:constants};const names={StateRef:'State',ArgRef:'Argument',ObservationRef:'Observation',ConstRef:'Constant'} as const;
          const t=resolve(tables[e.tag],e.name,e.span);resolution={declaration:e.name,tag:names[e.tag]};result={...metadata(t),declaration:e.name,tag:e.tag};
        } else if(e.tag==='Not') {const operand=expression(e.operand,depth+1);if(operand.type.tag!=='Bool')fail('TYPE_MISMATCH','not requires Bool',e.span);result={...metadata({tag:'Bool'}),operand,tag:'Not'};}
        else if(e.tag==='FloorDiv') {const numerator=expression(e.numerator,depth+1),denominator=expression(e.denominator,depth+1);result={...metadata(combine(numerator.type,denominator.type,true,e.span)),denominator,numerator,tag:'FloorDiv'};}
        else {
          const left=expression(e.left,depth+1),right=expression(e.right,depth+1);let t:LocalType;
          if(e.tag==='Mul')t=combine(left.type,right.type,false,e.span);
          else if(e.tag==='And'||e.tag==='Or'){if(left.type.tag!=='Bool'||right.type.tag!=='Bool')fail('TYPE_MISMATCH',`${e.tag} requires Bool operands`,e.span);t={tag:'Bool'};}
          else {if(!same(left.type,right.type))fail('TYPE_MISMATCH',`${e.tag} requires identical operand types and units`,e.span);if(e.tag!=='Eq'&&!numeric(left.type))fail('TYPE_MISMATCH',`${e.tag} requires numeric operands`,e.span);t=e.tag==='Add'||e.tag==='Sub'?left.type:{tag:'Bool'};}
          result={...metadata(t),left,right,tag:e.tag};
        }
        annotations[slot]={nodeId,resolution,sourceRef,type:result.type,unitVector:result.unitVector};return result;
      }
      const base={sourceRef:ref(statement.span),statementId};
      if(statement.tag==='Guard'){const condition=expression(statement.condition);if(condition.type.tag!=='Bool')fail('TYPE_MISMATCH','guard requires Bool',statement.span);instructions.push({...base,condition,message:statement.message.decoded,tag:'Guard'});}
      else if(statement.tag==='Let'){if(locals.has(statement.name))fail('DUPLICATE_LOCAL',statement.name,statement.span);const value=expression(statement.expression);locals.set(statement.name,{type:value.type,expression:value,statementIndex});instructions.push({...base,expression:value,name:statement.name,tag:'Let',type:value.type,unitVector:value.unitVector});}
      else if(statement.tag==='Set'){
        const targetType=resolve(states,statement.field,statement.span);if(writes.has(statement.field))fail('DUPLICATE_WRITE',statement.field,statement.span);writes.add(statement.field);
        const value=expression(statement.expression);if(!same(targetType,value.type))fail('TYPE_MISMATCH',`set ${statement.field}`,statement.span);
        const instruction:Extract<CoreInstruction,{tag:'Set'}>={...base,expression:value,field:statement.field,policy:{tag:'NonFinancial'},tag:'Set'};instructions.push(instruction);
        targets.set(`W:${sourceAction.name}:${statement.field}`,{action:sourceAction.name,statementIndex,expression:value,span:statement.span,attach:policy=>{instruction.policy={name:policy,tag:'Financial'};}});
      } else {
        const schema=resolve(effectSchemas,statement.kind,statement.span);
        if(statement.fields.length!==schema.length||statement.fields.some((f,i)=>f.label!==schema[i].label))fail('EFFECT_SCHEMA','emit fields must match exact declared order',statement.span);
        const fields=statement.fields.map((f,i)=>{const value=expression(f.expression);const expected=schema[i].type;if(expected.tag==='AmountFromOperand'?value.type.tag!=='Amount':!same(expected,value.type))fail('TYPE_MISMATCH',`effect field ${f.label}`,f.span);return {expression:value,label:f.label};});
        const ordinal=String(effects++),instruction:Extract<CoreInstruction,{tag:'Emit'}>={...base,fields,kind:statement.kind,ordinal,policies:[],tag:'Emit'};instructions.push(instruction);
        for(const f of fields)targets.set(`E:${sourceAction.name}:${ordinal}:${f.label}`,{action:sourceAction.name,statementIndex,expression:f.expression,span:statement.span,attach:policy=>{instruction.policies.push({field:f.label,policy});}});
      }
    });
    limit(instructions.length,shape.instructionsPerEntrypoint,'instructionsPerEntrypoint',sourceAction.span);limit(locals.size,shape.localsPerEntrypoint,'localsPerEntrypoint',sourceAction.span);limit(effects,shape.effectsPerEntrypoint,'effectsPerEntrypoint',sourceAction.span);
    actionLocals.set(sourceAction.name,locals);
    return {actorParameter:'actor',arguments:[...args].map(([name,type])=>({name,type})),instructions,name:sourceAction.name,resourceCounts:{effects:String(effects),expressionDepth:String(expressionDepth),expressionNodes:String(expressionNodes),instructions:String(instructions.length),locals:String(locals.size)},sourceRef:ref(sourceAction.span)};
  }
  for(const declaration of source.declarations) {
    switch(declaration.tag) {
      case 'UnitDecl':unique(units,declaration.name,true,declaration.span);break;
      case 'ConstDecl':case 'StateDecl':{
        const t=storedType(declaration.type),v=literal(declaration.value);if(!same(t,v.type))fail('TYPE_MISMATCH',`${declaration.name} initializer`,declaration.span);
        const values=declaration.tag==='ConstDecl'?constantValues:initialState;unique(declaration.tag==='ConstDecl'?constants:states,declaration.name,t,declaration.span);values.push({name:declaration.name,value:stored(v.value,declaration.span)});break;
      }
      case 'ObservationDecl':unique(observations,declaration.name,storedType(declaration.type),declaration.span);break;
      case 'SettlementDecl':{
        unique(settlementNames,declaration.name,true,declaration.span);const q=literal(declaration.quantum).value;
        if(q.tag!=='Amount'||q.value==='0'||declaration.asset.decoded==='')fail('SETTLEMENT_BINDING','positive Amount quantum and nonempty Text asset required',declaration.span);
        if(bindings.some(b=>b.unit===q.unit||b.asset===declaration.asset.decoded))fail('SETTLEMENT_BINDING_AMBIGUOUS','unit and asset must each be unique',declaration.span);
        bindings.push({asset:declaration.asset.decoded,name:declaration.name,quantum:q,unit:q.unit});break;
      }
      case 'EpisodeStatusDecl':{
        if(episode)fail('STATUS_DUPLICATE','episode rule',declaration.span);const t=resolve(states,declaration.field,declaration.span),v=literal(declaration.literal);if(!same(t,v.type))fail('TYPE_MISMATCH','episode status literal',declaration.span);episode={field:declaration.field,literal:stored(v.value,declaration.span),tag:'ClosedWhenEqual'};break;
      }
      case 'NotionalStatusDecl':if(agreement)fail('STATUS_DUPLICATE','agreement rule',declaration.span);if(resolve(states,declaration.field,declaration.span).tag!=='Amount')fail('TYPE_MISMATCH','remaining notional must be Amount',declaration.span);agreement={field:declaration.field,tag:'RemainingNotional'};break;
      case 'NoNotionalStatusDecl':if(agreement)fail('STATUS_DUPLICATE','agreement rule',declaration.span);agreement={tag:'NoRemainingNotional'};break;
      case 'ReserveDecl':reserves.push(declaration);break;
      case 'FieldPolicyDecl':resolve(units,declaration.unit,declaration.span);unique(policies,declaration.name,declaration,declaration.span);break;
      case 'EffectDecl':{
        const expected=standardEffects[declaration.kind];if(declaration.fields.length!==expected.length||declaration.fields.some((f,i)=>f.label!==expected[i]||(f.label==='amount'?f.type.tag!=='BareAmount':f.type.tag!=='Text')))fail('EFFECT_SCHEMA','fixed effect fields/types/order required',declaration.span);
        unique(effectSchemas,declaration.kind,declaration.fields.map(f=>({label:f.label,type:type(f.type,true)})),declaration.span);break;
      }
      case 'ActionDecl':if(actions.has(declaration.name))fail('DUPLICATE_DECLARATION',declaration.name,declaration.span);actionSources.set(declaration.name,declaration);actions.set(declaration.name,action(declaration));break;
    }
  }
  if(actions.size===0)fail('ACTION_REQUIRED','at least one action');if(!episode||!agreement)fail('STATUS_REQUIRED','one episode and one agreement status rule required');if(observations.get('now')?.tag!=='UInt128')fail('OBSERVATION_NOW_REQUIRED','now: UInt128 observation is mandatory');
  const usedTargets=new Set<string>(),manifestPolicies:FieldPolicy[]=[];
  for(const p of policies.values()) {
    let roundingNode:FieldPolicy['roundingNode']={tag:'None'};let roundingLocal:LocalEntry|undefined;
    if(p.rounding.tag==='Floor') {const locals=resolve(actionLocals,p.rounding.action,p.rounding.span);roundingLocal=resolve(locals,p.rounding.local,p.rounding.span);if(roundingLocal.expression.tag!=='FloorDiv')fail('POLICY_ROUNDING','rounding local root must be FloorDiv',p.rounding.span);roundingNode={action:p.rounding.action,coreNodeId:roundingLocal.expression.nodeId,local:p.rounding.local,tag:'Floor'};}
    const policyTargets:FieldPolicy['targets']=[];
    for(const target of p.targets) {
      const key=target.tag==='Write'?`W:${target.action}:${target.field}`:`E:${target.action}:${target.ordinal}:${target.field}`;
      const actual=resolve(targets,key,target.span);const actionSource=resolve(actionSources,target.action,target.span);
      if(BigInt(p.span.startByte)>BigInt(actionSource.span.startByte))fail('POLICY_DECLARATION_ORDER','policy must precede target action',p.span);
      if(usedTargets.has(key))fail('POLICY_DUPLICATE_TARGET',key,target.span);usedTargets.add(key);
      if(actual.expression.type.tag!=='Amount'||actual.expression.type.unit!==p.unit)fail('POLICY_UNIT','policy target must be Amount of exact policy unit',target.span);
      if(roundingNode.tag==='Floor'&&(roundingNode.action!==target.action||roundingLocal!.statementIndex>=actual.statementIndex))fail('POLICY_ROUNDING','rounding local must precede target in same action',target.span);
      actual.attach(p.name);
      policyTargets.push(target.tag==='Write'?{action:target.action,field:target.field,tag:'Write'}:{action:target.action,field:target.field,ordinal:target.ordinal,tag:'Effect'});
    }
    manifestPolicies.push({comparisonPolicy:p.comparison.decoded,derivation:p.derivation.decoded,name:p.name,proofStatement:p.proof.decoded,remainderDisposition:p.remainder.decoded,roundingNode,targets:policyTargets,unit:p.unit});
  }
  for(const [key,target] of targets)if(target.expression.type.tag==='Amount'&&!usedTargets.has(key))fail('POLICY_MISSING_TARGET',key,target.span);
  for(const a of actions.values())for(const i of a.instructions)if(i.tag==='Emit')i.policies.sort((a,b)=>standardEffects[i.kind].indexOf(a.field)-standardEffects[i.kind].indexOf(b.field));
  const reserveActions=new Set<string>();
  for(const r of reserves){resolve(actions,r.action,r.span);resolve(actions,r.closure,r.span);if(r.action===r.closure||reserveActions.has(r.action))fail('RESERVE_RULE','reserve action must be distinct and unique',r.span);reserveActions.add(r.action);const a=resolve(actionSources,r.action,r.span);if(!a.statements.some(s=>s.tag==='Guard'&&s.condition.tag==='Gt'&&s.condition.left.tag==='Remaining'&&s.condition.right.tag==='Literal'&&s.condition.right.literal.tag==='UIntLiteral'&&s.condition.right.literal.token==='1'))fail('RESERVE_GUARD','reserved action requires exact remaining > uint(1) guard',r.span);}
  for(const r of reserves)if(reserveActions.has(r.closure))fail('RESERVE_RULE','closure cannot itself be a reserved action',r.span);
  const counts:[number,number,string][]=[[units.size,shape.declaredUnits,'declaredUnits'],[actions.size,shape.entrypoints,'entrypoints'],[states.size,shape.stateFields,'stateFields'],[constants.size,shape.constFields,'constFields'],[observations.size,shape.observationFields,'observationFields'],[effectSchemas.size,shape.effectKinds,'effectKinds'],[policies.size,shape.fieldPolicies,'fieldPolicies'],[[...policies.values()].reduce((n,p)=>n+p.targets.length,0),shape.policyTargets,'policyTargets'],[bindings.length,shape.settlementBindings,'settlementBindings'],[reserves.length,shape.reserveRules,'reserveRules']];
  for(const [count,max,label] of counts)limit(count,max,label);for(const fields of effectSchemas.values())limit(fields.length,shape.effectFields,'effectFields');
  const requiredClaims:SemanticManifest['requiredClaims']=[{claimId:'bounded_profile_safety_v1',kind:'ContractProperty'},{claimId:'atomic_intent_refinement_v1',kind:'IntentRefinement'},{claimId:'bounded_atomic_transition_v1',kind:'TransitionValidity'},{claimId:'bounded_history_compliance_v1',kind:'PredecessorHistory'}];
  for(const p of manifestPolicies)if(!requiredClaims.some(c=>c.kind==='ContractProperty'&&c.claimId===p.proofStatement))requiredClaims.push({claimId:p.proofStatement,kind:'ContractProperty'});
  function byteCompare(a:string,b:string):number{const aa=utf8(a),bb=utf8(b);for(let i=0;i<Math.min(aa.length,bb.length);i++)if(aa[i]!==bb[i])return aa[i]-bb[i];return aa.length-bb.length;}
  requiredClaims.sort((a,b)=>byteCompare(a.kind,b.kind)||byteCompare(a.claimId,b.claimId));
  const namedTypes=(table:Map<string,StoredType>):NamedType[]=>[...table].map(([name,type])=>({name,type}));
  const manifest:SemanticManifest={bounds:{boundsHash:hashRawDomain('MORIARTY-BOUNDS-bounded-atomic/1',boundsBytes),registryId:'moriarty-bounds/1'},constants:constantValues,core:{actions:[...actions.values()],coreVersion:'moriarty-core/1',schemaVersion:'moriarty-core-program/1'},effectSchemas:[...effectSchemas].map(([kind,fields])=>({fields,kind})),horizon:source.horizon.token,initialState,lifetime:source.lifetime.token,name:source.name,observationSchema:namedTypes(observations),policies:manifestPolicies,profile:PROFILE,schemaVersion:'moriarty-semantic-manifest/1',settlementBindings:bindings,sourceHash:source.sourceHash,stateSchema:namedTypes(states),statusRules:{agreement,episode},requiredClaims,units:[...units.keys()],reserveRules:reserves.map(r=>({action:r.action,closure:r.closure}))};
  const typed:TypedProgram={annotations,profile:PROFILE,schemaVersion:'moriarty-typed-program/1',source};
  const bound:BoundProgram={manifest,programHash:hashDomain('MORIARTY-PROGRAM-bounded-atomic/1',manifest),schemaVersion:'moriarty-program/1'};
  const metrics={sourceAST:astMetrics,typedProgram:checkEncoding(typed,bounds.typedProgramEncoding,'TypedProgram'),boundProgram:checkEncoding(bound,bounds.programManifestEncoding,'BoundProgram')};
  return {typed,bound,metrics};
}
