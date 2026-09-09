// Independent README audit checks. Mutations use a disposable tree only.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync, mkdirSync, mkdtempSync, rmSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { tmpdir } from 'node:os';
import { spawnSync } from 'node:child_process';
import { parseSuccessorSource, SYNTAX_BOUNDS } from '../../experiments/moriarty-language/src/successor/frontend.ts';
import { prepareSuccessor } from '../../experiments/moriarty-language/src/successor/evaluate.ts';
const read=p=>readFileSync(new URL('../../'+p,import.meta.url),'utf8');
const readme=read('README.md');
const grammar=read('experiments/moriarty-language/spec/successor/grammar.ebnf');
const test=read('experiments/moriarty-language/tests/readme-grammar.test.mjs');
let count=0;
const check=(label,fn)=>{fn();count++;console.log('PASS '+label);};
const block=/^```ebnf\n([\s\S]*?)^```$/gm;
check('complete uniquely fenced canonical grammar',()=>{const matches=[...readme.matchAll(block)];assert.equal(matches.length,1);assert.equal(matches[0][1],grammar);});
check('all grammar references defined exactly once',()=>{const stripped=grammar.replace(/\(\*[\s\S]*?\*\)/g,'').replace(/"[^"]*"/g,'').replace(/\?[^?]*\?/g,'');const defined=[...stripped.matchAll(/^([a-z_]+)\s*=/gm)].map(m=>m[1]);assert.equal(new Set(defined).size,defined.length);const refs=[...stripped.matchAll(/\b[a-z_]+\b/g)].map(m=>m[0]);assert.ok(refs.every(r=>defined.includes(r)));assert.equal(defined[0],'program');console.log('GRAMMAR_PRODUCTIONS '+defined.length);});
check('every profile bound equals parser constant',()=>assert.deepEqual(JSON.parse(read('experiments/moriarty-language/spec/successor/syntax-profile.json')).bounds,SYNTAX_BOUNDS));
check('all links in added grammar section resolve locally',()=>{const section=readme.slice(readme.indexOf('### Successor source grammar'),readme.indexOf('## What a developer writes'));for(const m of section.matchAll(/\]\(([^)]+)\)/g))assert.ok(read(m[1]).length);});
const wrap=s=>'profile "moriarty-successor-syntax/0"; agreement Review {'+s+'}';
const action=s=>wrap('action review(){'+s+'}');
check('all declaration and statement productions compose',()=>{const ast=parseSuccessorSource(wrap('unit USD;party Alice;asset Coin:Asset<USD>;const x:UInt=1;state y:Map<Debt<USD>,Pair<A,B>>=f();action review(p:T,q:U){requires p;let v=f(1,"text",true,false).field;next.y=v;emit E<T>{a:1,b:q};ensures true;ensures false;}action empty(){}'));assert.deepEqual(ast.agreement.declarations.map(d=>d.tag),['UnitDecl','PartyDecl','AssetDecl','ConstDecl','StateDecl','ActionDecl','ActionDecl']);const a=ast.agreement.declarations[5];assert.deepEqual(a.statements.map(s=>s.tag),['Requires','Let','Next','Emit']);assert.equal(a.postconditions.length,2);});
check('empty agreement and empty effect fields admitted',()=>{parseSuccessorSource(wrap(''));parseSuccessorSource(action('emit Tick {};'));});
check('precedence and left associativity agree with productions',()=>{const ast=parseSuccessorSource(action('let v=not a==b and c or d;let x=8-3-2;let y=1+2*3;'));const [v,x,y]=ast.agreement.declarations[0].statements.map(s=>s.expression);assert.equal(v.operator,'or');assert.equal(v.left.operator,'and');assert.equal(v.left.left.operator,'not');assert.equal(v.left.left.operand.operator,'==');assert.equal(x.left.operator,'-');assert.equal(y.right.operator,'*');});
for(const op of ['==','!=','<','<=','>','>='])check('comparison '+op,()=>assert.equal(parseSuccessorSource(action('let v=a'+op+'b;')).agreement.declarations[0].statements[0].expression.operator,op));
check('parenthesized nested comparison admitted',()=>parseSuccessorSource(action('let x=(a<b)==(c>d);')));
check('JSON escaped profile decoded to selected profile',()=>parseSuccessorSource(wrap('').replace('syntax/0','syntax/\\u0030')));
check('non-nesting comments and ASCII separators',()=>parseSuccessorSource('/* /* */'+wrap('unit\tUSD;\r\nparty A;//last\n')));
for(const [label,s,code] of [
 ['comparison chain',action('let x=a<b<c;'),'CHAINED_COMPARISON'],
 ['empty generic',wrap('asset A:T<>;'),'EMPTY_TYPE_ARGS'],
 ['parameter trailing comma',wrap('action a(p:T,){}'),'UNEXPECTED_TOKEN'],
 ['call trailing comma',action('let x=f(1,);'),'UNEXPECTED_TOKEN'],
 ['effect trailing comma',action('emit E{f:1,};'),'UNEXPECTED_TOKEN'],
 ['statement after postcondition',action('ensures true;let x=1;'),'STATEMENT_AFTER_ENSURES'],
 ['missing profile','agreement A{}','UNEXPECTED_TOKEN'],
 ['second agreement',wrap('')+'agreement B{}','TRAILING_INPUT'],
 ['old profile',wrap('').replace('moriarty-successor-syntax/0','moriarty-bounded-atomic/1'),'PROFILE_MISMATCH'],
 ['division',action('let x=1/2;'),'UNEXPECTED_CHAR'],
 ['leading zero',action('let x=01;'),'INTEGER_TOKEN'],
 ['non-ASCII whitespace','\u00a0'+wrap(''),'UNEXPECTED_CHAR'],
 ['BOM retained','\ufeff'+wrap(''),'UNEXPECTED_CHAR'],
 ['unary minus',action('let x=-1;'),'UNEXPECTED_TOKEN'],
 ['keyword identifier',wrap('unit emit;'),'UNEXPECTED_TOKEN'],
 ['non-ASCII identifier',wrap('unit USé;'),'NON_ASCII_IDENTIFIER'],
])check(label,()=>assert.throws(()=>parseSuccessorSource(s),e=>e.code===code));
check('linked examples parse but have distinct execution admission',()=>{const funded=read('experiments/moriarty-language/spec/successor/examples/funded-partial-payment.mori');const syntax=read('experiments/moriarty-language/spec/successor/examples/partial-payment.mori');const input=read('experiments/moriarty-language/spec/successor/examples/funded-partial-payment.invocation.json');parseSuccessorSource(funded);parseSuccessorSource(syntax);assert.equal(prepareSuccessor(funded,input).status,'Prepared');assert.equal(prepareSuccessor(syntax,input).code,'UNSUPPORTED_DECLARATION');});
check('loan and swap source headers use atomic profile',()=>{for(const name of ['loan','swap'])assert.match(read('experiments/moriarty-language/spec/examples/'+name+'.mori'),/^agreement \w+ profile "moriarty-bounded-atomic\/1"/);});
const temp=mkdtempSync(join(tmpdir(),'moriarty-readme-audit-'));
try {
 const testPath=join(temp,'experiments/moriarty-language/tests/readme-grammar.test.mjs');
 const grammarPath=join(temp,'experiments/moriarty-language/spec/successor/grammar.ebnf');
 mkdirSync(dirname(testPath),{recursive:true});mkdirSync(dirname(grammarPath),{recursive:true});writeFileSync(testPath,test);
 const fullBlock=[...readme.matchAll(block)][0][0];
 for(const [label,mutated,canonical,expected] of [
  ['baseline',readme,grammar,0],
  ['changed terminal',readme.replace('unit_decl = "unit"','unit_decl = "money"'),grammar,1],
  ['missing production',readme.replace('expression = disjunction ;\n',''),grammar,1],
  ['duplicate block',readme+'\n'+fullBlock+'\n',grammar,1],
  ['missing block',readme.replace(fullBlock,''),grammar,1],
  ['canonical drift',readme,grammar+'\n',1],
  ['comment drift',readme.replace('Specified-only syntax profile.','Changed syntax profile.'),grammar,1],
 ])check('real drift test '+label,()=>{writeFileSync(join(temp,'README.md'),mutated);writeFileSync(grammarPath,canonical);const r=spawnSync(process.execPath,['--test',testPath],{encoding:'utf8',timeout:5000});assert.equal(r.error,undefined);assert.equal(r.status,expected,r.stdout+r.stderr);});
} finally {rmSync(temp,{recursive:true,force:true});}
console.log(`${count} independent README grammar checks passed`);
