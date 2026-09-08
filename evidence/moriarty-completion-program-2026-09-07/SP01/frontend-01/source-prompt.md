You are the user-selected Grok4.6 high source implementation worker. Make these precise source edits now with Edit/Write. This is Moriarty, not bridge/Goldbach. Parent completed source review, reproduced failures, and verified/froze regression tests before this dispatch. No new planning or permission cycle. User selected Grok implementation and independent GPT6 checking.

OBJECTIVE: Correct two reproduced frontend diagnostic-priority defects without widening the accepted language.
FILES: Own only /home/charl/Moriarty/.worktrees/sp01-frontend-grok/experiments/moriarty-language/src/validate.ts and FOREMAN_REPORT.md/.json in that tree. Do not modify frozen tests, checker/parser, schemas, bounds, or anything else. No shell commands, tests, Git writes, agents/providers, installs, web, native proofs, wallets.

CURRENT COMPLETE VALIDATE SOURCE:
```ts
/** Separate complete-stage validation prevents later-stage errors masking earlier ones. */
import type { ActionDecl, Bounds, LocalType, SourceAST, SourceExpression, SourceLiteral, SourceType, Span, UnitVector } from './types.ts';
import { canonicalEncode, checkEncoding, FrontendError } from './codec.ts';
import { diagnostic, firstDiagnostic, type Diagnostic, type DiagnosticCode } from './diagnostics.ts';
import { UINT_MAX, PROFILE, type ParsedSource } from './parser.ts';
import { throwDiagnostic } from './diagnostics.ts';
const fixed={Transfer:['asset','from','to','amount'],Fee:['asset','from','to','amount'],DueCreated:['due_id','debtor','creditor','denomination','amount'],DueSettled:['due_id','debtor','creditor','denomination','amount','asset']};
const children=(e:SourceExpression):SourceExpression[]=>'left'in e?[e.left,e.right]:e.tag==='FloorDiv'?[e.numerator,e.denominator]:e.tag==='Not'?[e.operand]:[];
const expressions=(a:ActionDecl):SourceExpression[]=>a.statements.flatMap(s=>s.tag==='Guard'?[s.condition]:s.tag==='Emit'?s.fields.map(f=>f.expression):[s.expression]);
const walk=(root:SourceExpression,visit:(e:SourceExpression)=>void)=>{const todo=[root];while(todo.length){const e=todo.pop()!;visit(e);todo.push(...children(e).reverse());}};
const localType=(t:SourceType):LocalType|undefined=>t.tag==='BareAmount'?undefined:t.tag==='Amount'?{tag:'Amount',unit:t.unit}:{tag:t.tag};
const vec=(t:LocalType):UnitVector=>t.tag==='Amount'?[{exponent:'1',unit:t.unit}]:t.tag==='Quantity'?t.unitVector:[];
const numeric=(t:LocalType)=>t.tag==='UInt128'||t.tag==='Amount'||t.tag==='Quantity';
const same=(a:LocalType,b:LocalType)=>canonicalEncode(a)===canonicalEncode(b);
/** These are exactly the fields broadened by the internal syntax-tree type.
 * All other record shapes are constructed directly by the complete parser.
 */
export function sourceShapeDiagnostics(ast:ParsedSource):Diagnostic[] {
  const errors:Diagnostic[]=[];
  if(ast.profile!==PROFILE)errors.push(diagnostic('DECLARATION_SCHEMA',ast.span));
  for(const declaration of ast.declarations)if(declaration.tag==='SettlementDecl'&&(declaration.asset.tag!=='TextLiteral'||declaration.quantum.tag!=='AmountLiteral'))errors.push(diagnostic('DECLARATION_SCHEMA',declaration.span));
  return errors;
}
export function assertSourceShape(ast:ParsedSource):asserts ast is SourceAST {
  const error=firstDiagnostic(sourceShapeDiagnostics(ast));if(error)throwDiagnostic(error);
}
export function validateSource(ast:ParsedSource,bounds:Bounds,through:4|5|6=6):Diagnostic|undefined {
  const errors:Diagnostic[]=sourceShapeDiagnostics(ast);const add=(code:DiagnosticCode,span?:Span)=>errors.push(diagnostic(code,span));
  const finish=()=>firstDiagnostic(errors);
  // Stage 4: closed declaration shape, every duplicate namespace, and aggregate AST.
  if(!ast.declarations.some(d=>d.tag==='ActionDecl'))add('DECLARATION_SCHEMA',ast.span);
  const names=new Map<string,Set<string>>();
  const duplicate=(namespace:string,name:string,span:Span)=>{const set=names.get(namespace)??new Set<string>();if(set.has(name))add('DUPLICATE_NAME',span);set.add(name);names.set(namespace,set);};
  for(const d of ast.declarations){
    if('name'in d&&d.tag!=='SettlementDecl')duplicate(d.tag,d.name,d.span);
    if(d.tag==='EffectDecl'){duplicate('EffectDecl',d.kind,d.span);const labels=new Set<string>();for(const f of d.fields){if(labels.has(f.label))add('DUPLICATE_NAME',f.span);labels.add(f.label);}const want=fixed[d.kind];if(d.fields.length!==want.length||d.fields.some((f,i)=>f.label!==want[i]||(f.label==='amount'?f.type.tag!=='BareAmount':f.type.tag!=='Text')))add('DECLARATION_SCHEMA',d.span);}
    if(d.tag==='ActionDecl'){for(const p of d.parameters)duplicate(`args:${d.name}`,p.name,p.span);for(const s of d.statements){if(s.tag==='Let')duplicate(`locals:${d.name}`,s.name,s.span);if(s.tag==='Set')duplicate(`writes:${d.name}`,s.field,s.span);if(s.tag==='Emit'){const labels=new Set<string>();for(const f of s.fields){if(labels.has(f.label))add('DUPLICATE_NAME',f.span);labels.add(f.label);}const want=fixed[s.kind];if(s.fields.length!==want.length||s.fields.some((f,i)=>f.label!==want[i]))add('DECLARATION_SCHEMA',s.span);}}}
  }
  try{checkEncoding(ast,bounds.astEncoding,'SourceAST');}catch(e){if(!(e instanceof FrontendError))throw e;add('AST_BOUNDS');}
  if(errors.length||through===4)return finish();
  assertSourceShape(ast);
  // Stage 5: declaration-before-use over the complete source, before any type error.
  const units=new Set<string>(),constants=new Map<string,LocalType>(),states=new Map<string,LocalType>(),observations=new Map<string,LocalType>(),effects=new Set<string>();
  const unit=(name:string,span:Span)=>{if(!units.has(name))add('NAME_RESOLUTION',span);};
  const typeUse=(t:SourceType)=>{if(t.tag==='Amount')unit(t.unit,t.span);};
  const literalUse=(l:SourceLiteral)=>{if(l.tag==='AmountLiteral')unit(l.unit,l.span);};
  for(const d of ast.declarations){
    if(d.tag==='UnitDecl')units.add(d.name);
    else if(d.tag==='ConstDecl'||d.tag==='StateDecl'){typeUse(d.type);literalUse(d.value);(d.tag==='ConstDecl'?constants:states).set(d.name,localType(d.type)!);}
    else if(d.tag==='ObservationDecl'){typeUse(d.type);observations.set(d.name,localType(d.type)!);}
    else if(d.tag==='SettlementDecl')literalUse(d.quantum);
    else if(d.tag==='FieldPolicyDecl')unit(d.unit,d.span);
    else if(d.tag==='EffectDecl'){d.fields.forEach(f=>typeUse(f.type));effects.add(d.kind);}
    else if(d.tag==='EpisodeStatusDecl')literalUse(d.literal);
    else if(d.tag==='ActionDecl'){
      const args=new Set<string>(),locals=new Set<string>();for(const p of d.parameters){typeUse(p.type);args.add(p.name);}
      for(const s of d.statements){const roots=s.tag==='Emit'?s.fields.map(f=>f.expression):s.tag==='Guard'?[s.condition]:[s.expression];for(const root of roots)walk(root,e=>{if(e.tag==='Literal')literalUse(e.literal);else if('name'in e){const table=e.tag==='StateRef'?states:e.tag==='ConstRef'?constants:e.tag==='ObservationRef'?observations:e.tag==='ArgRef'?args:locals;if(!table.has(e.name))add('NAME_RESOLUTION',e.span);}});if(s.tag==='Let')locals.add(s.name);if(s.tag==='Set'&&!states.has(s.field))add('NAME_RESOLUTION',s.span);if(s.tag==='Emit'&&!effects.has(s.kind))add('NAME_RESOLUTION',s.span);}
    }
  }
  if(errors.length||through===5)return finish();
  // Stage 6: infer independent expressions, skipping dependent checks after an invalid operand.
  const uint=(token:string,span:Span)=>{if(token.length>39||BigInt(token)>UINT_MAX){add('UINT_RANGE',span);return false;}return true;};
  uint(ast.horizon.token,ast.horizon.span);if(!uint(ast.lifetime.token,ast.lifetime.span)||ast.lifetime.token==='0')add('UINT_RANGE',ast.lifetime.span);
  const literal=(l:SourceLiteral):LocalType|undefined=>{if(l.tag==='UIntLiteral'||l.tag==='AmountLiteral'){if(!uint(l.token,l.span))return undefined;return l.tag==='UIntLiteral'?{tag:'UInt128'}:{tag:'Amount',unit:l.unit};}return {tag:l.tag==='TextLiteral'?'Text':'Bool'};};
  type Target={type:LocalType|undefined;span:Span;action:string;statement:number};
  const targets=new Map<string,Target>(),localTables=new Map<string,Map<string,{type:LocalType|undefined;expression:SourceExpression;statement:number}>>(),actionMap=new Map<string,ActionDecl>();
  let episode=0,agreement=0;const settlementNames=new Set<string>(),settlementUnits=new Set<string>(),assets=new Set<string>();
  for(const d of ast.declarations){
    if(d.tag==='ConstDecl'||d.tag==='StateDecl'){const t=literal(d.value),want=localType(d.type);if(t&&(!want||!same(t,want)))add('TYPE_MISMATCH',d.span);}
    if(d.tag==='SettlementDecl'){if(d.asset.tag!=='TextLiteral'||d.quantum.tag!=='AmountLiteral')add('SETTLEMENT_DECLARATION',d.span);else{literal(d.quantum);if(!d.asset.decoded||d.quantum.token==='0')add('SETTLEMENT_DECLARATION',d.span);if(settlementNames.has(d.name)||settlementUnits.has(d.quantum.unit)||assets.has(d.asset.decoded))add('SETTLEMENT_BINDING_AMBIGUOUS',d.span);settlementNames.add(d.name);settlementUnits.add(d.quantum.unit);assets.add(d.asset.decoded);}}
    if(d.tag==='EpisodeStatusDecl'){if(episode++)add('STATUS_RULE',d.span);const want=states.get(d.field),actual=literal(d.literal);if(!want||actual&&!same(want,actual))add('STATUS_RULE',d.span);}
    if(d.tag==='NotionalStatusDecl'||d.tag==='NoNotionalStatusDecl'){if(agreement++)add('STATUS_RULE',d.span);if(d.tag==='NotionalStatusDecl'&&states.get(d.field)?.tag!=='Amount')add('STATUS_RULE',d.span);}
    if(d.tag!=='ActionDecl')continue;
    actionMap.set(d.name,d);const args=new Map(d.parameters.map(p=>[p.name,localType(p.type)!])),locals=new Map<string,{type:LocalType|undefined;expression:SourceExpression;statement:number}>();localTables.set(d.name,locals);
    if(args.get('actor')?.tag!=='Text')add('ACTOR_PARAMETER',d.span);
    const infer=(e:SourceExpression):LocalType|undefined=>{
      if(e.tag==='Literal')return literal(e.literal);if(e.tag==='Remaining')return {tag:'UInt128'};if('name'in e)return e.tag==='LocalRef'?locals.get(e.name)?.type:(e.tag==='StateRef'?states:e.tag==='ConstRef'?constants:e.tag==='ObservationRef'?observations:args).get(e.name);
      const parts=children(e).map(infer);if(parts.some(t=>!t))return undefined;const left=parts[0]!,right=parts[1]!;
      const bad=(code:DiagnosticCode='TYPE_MISMATCH')=>{add(code,e.span);return undefined;};
      if(e.tag==='Not')return left.tag==='Bool'?{tag:'Bool'}:bad();
      if(e.tag==='And'||e.tag==='Or')return left.tag==='Bool'&&right.tag==='Bool'?{tag:'Bool'}:bad();
      if(e.tag==='FloorDiv'||e.tag==='Mul'){
        if(!numeric(left)||!numeric(right))return bad();const m=new Map<string,number>();for(const c of vec(left))m.set(c.unit,Number(c.exponent));for(const c of vec(right))m.set(c.unit,(m.get(c.unit)??0)+(e.tag==='FloorDiv'?-1:1)*Number(c.exponent));const v=[...m].filter(([,v])=>v!==0).sort(([a],[b])=>a<b?-1:a>b?1:0).map(([unit,n])=>({unit,exponent:String(n)}));if(v.length>bounds.programShape.expressionUnitComponents||v.some(c=>Math.abs(Number(c.exponent))>bounds.programShape.absoluteUnitExponent))return bad('UNIT_VECTOR');return v.length===0?{tag:'UInt128'}:v.length===1&&v[0].exponent==='1'?{tag:'Amount',unit:v[0].unit}:{tag:'Quantity',unitVector:v};
      }
      if(!same(left,right)||e.tag!=='Eq'&&!numeric(left))return bad();return e.tag==='Add'||e.tag==='Sub'?left:{tag:'Bool'};
    };
    let ordinal=0;
    d.statements.forEach((s,index)=>{if(s.tag==='Guard'){const t=infer(s.condition);if(t&&t.tag!=='Bool')add('TYPE_MISMATCH',s.span);}else if(s.tag==='Let')locals.set(s.name,{type:infer(s.expression),expression:s.expression,statement:index});else if(s.tag==='Set'){const t=infer(s.expression),want=states.get(s.field)!;if(t&&!same(t,want))add('TYPE_MISMATCH',s.span);targets.set(`W:${d.name}:${s.field}`,{type:t,span:s.span,action:d.name,statement:index});}else{for(const f of s.fields){const t=infer(f.expression);if(t&&(f.label==='amount'?t.tag!=='Amount':t.tag!=='Text'))add('TYPE_MISMATCH',f.span);targets.set(`E:${d.name}:${ordinal}:${f.label}`,{type:t,span:s.span,action:d.name,statement:index});}ordinal++;}});
  }
  if(!episode||!agreement)add('STATUS_RULE');if(observations.get('now')?.tag!=='UInt128')add('OBSERVATION_SCHEMA');
  // Policy metadata resolves only after all actions and their locals/targets exist.
  // Policy declaration order does not constrain the action it covers.
  const covered=new Set<string>();
  for(const d of ast.declarations)if(d.tag==='FieldPolicyDecl'){
    let floor:{type:LocalType|undefined;expression:SourceExpression;statement:number}|undefined;
    if(d.rounding.tag==='Floor'){floor=localTables.get(d.rounding.action)?.get(d.rounding.local);if(!floor||floor.expression.tag!=='FloorDiv')add('POLICY_ROUNDING',d.rounding.span);}
    for(const t of d.targets){const key=t.tag==='Write'?`W:${t.action}:${t.field}`:`E:${t.action}:${t.ordinal}:${t.field}`,actual=targets.get(key);if(!actual||covered.has(key)||actual.type&&(actual.type.tag!=='Amount'||actual.type.unit!==d.unit))add('POLICY_TARGET',t.span);covered.add(key);if(actual&&floor&&d.rounding.tag==='Floor'&&(d.rounding.action!==t.action||floor.statement>=actual.statement))add('POLICY_ROUNDING',t.span);}
  }
  for(const [key,t] of targets)if(t.type?.tag==='Amount'&&!covered.has(key))add('POLICY_TARGET',t.span);
  const reserves=ast.declarations.filter(d=>d.tag==='ReserveDecl'),reserved=new Set<string>();
  for(const r of reserves){const a=actionMap.get(r.action);if(!a||!actionMap.has(r.closure)||r.action===r.closure||reserved.has(r.action)||!a.statements.some(s=>s.tag==='Guard'&&s.condition.tag==='Gt'&&s.condition.left.tag==='Remaining'&&s.condition.right.tag==='Literal'&&s.condition.right.literal.tag==='UIntLiteral'&&s.condition.right.literal.token==='1'))add('RESERVE_RULE',r.span);reserved.add(r.action);}
  for(const r of reserves)if(reserved.has(r.closure))add('RESERVE_RULE',r.span);
  return finish();
}

```

REVIEWED EXACT CHANGES:
1. After stage5 tables are declared and before stage5 traversal, add:
```ts
  const declaredStates=new Set(ast.declarations.filter(d=>d.tag==='StateDecl').map(d=>d.name));
```
Replace `else if(d.tag==='EpisodeStatusDecl')literalUse(d.literal);` with:
```ts
    else if(d.tag==='EpisodeStatusDecl'||d.tag==='NotionalStatusDecl'){
      if(!states.has(d.field)&&declaredStates.has(d.field))add('NAME_RESOLUTION',d.span);
      if(d.tag==='EpisodeStatusDecl')literalUse(d.literal);
    }
```
This preserves stage6 STATUS_RULE for globally absent fields while moving premature existing-field references into stage5. Keep all other stage5 collection and firstDiagnostic sorting.

2. Next to `const covered=new Set<string>();`, add `const deferredCoverage=new Set<string>();`. Expand the policy-target loop for readability, preserving every existing independent check. Before constructing each target key, add:
```ts
      if(t.tag==='Effect'&&!uint(t.ordinal,t.span)){
        for(const [candidateKey,candidate] of targets){
          const parts=candidateKey.split(':');
          if(parts[0]==='E'&&candidate.action===t.action&&parts[3]===t.field&&candidate.type?.tag==='Amount'&&candidate.type.unit===d.unit)deferredCoverage.add(candidateKey);
        }
        continue;
      }
```
Change final uncovered-target condition from `t.type?.tag==='Amount'&&!covered.has(key)` to `t.type?.tag==='Amount'&&!covered.has(key)&&!deferredCoverage.has(key)`.

INTERFACES: `validateSource(ast,bounds,through)` and DiagnosticCode registry unchanged. Existing uint helper applies UInt128 range checks. Invalid ordinals prevent exact target resolution, so only dependent missing-coverage errors for same action/effect-label/unit are deferred. Retain independent uncovered writes, different actions/units, valid target errors, rounding and expression diagnostics, and deterministic source-span selection. No global invalid-ordinal shortcut.

VERIFICATION: Parent executes frozen focused tests, full frontend tests, typecheck, complete suite including current Compact --skip-zk, then independent GPT6 review. Do not run them yourself. Update reports with exact edits and limitations. Finish within1190 seconds.

PARENT RED/FREEZE RECEIPT:
```json
{
  "testFile": "experiments/moriarty-language/tests/frontend.test.mjs",
  "sha256": "7c9a287454352fb0c35dc9f76c9760c6187a9bedf48ca9d56713dbec4c93da54",
  "sourceSha256": "350e137853c043b7fdd9f4501e86b087f0668ce7f0e8b826ded43b8dc5f10d4c",
  "red": {
    "total": 7,
    "pass": 4,
    "fail": 3,
    "substantiveFailures": [
      "episode status: TYPE_MISMATCH instead of NAME_RESOLUTION",
      "notional status: TYPE_MISMATCH instead of NAME_RESOLUTION",
      "ordinal overflow: POLICY_TARGET instead of UINT_RANGE"
    ],
    "fixtureErrors": false
  },
  "recordedAt": "2026-09-08T01:20:25.345388+00:00"
}
```
The tests use single-target policies before AND after actions; do not bypass either dependency case.
