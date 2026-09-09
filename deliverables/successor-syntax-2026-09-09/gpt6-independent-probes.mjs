import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
const repo=process.argv[2];
const {parseSuccessorSource:p,SuccessorSyntaxError:E,SYNTAX_BOUNDS:B}=await import(pathToFileURL(repo+'/experiments/moriarty-language/src/successor/frontend.ts'));
const {formatSuccessorSource:f}=await import(pathToFileURL(repo+'/experiments/moriarty-language/src/successor/format.ts'));
const wrap=x=>'profile "moriarty-successor-syntax/0";agreement X{'+x+'}';
const act=x=>wrap('action a(){'+x+'}');
const strip=x=>Array.isArray(x)?x.map(strip):x&&typeof x==='object'?Object.fromEntries(Object.entries(x).filter(([k])=>k!=='span').map(([k,v])=>[k,strip(v)])):x;
let cases=0;
const accept=s=>{cases++; const a=p(s),out=f(s);assert.deepEqual(strip(p(out)),strip(a));assert.equal(f(out),out);return a;};
const reject=(s,code)=>{cases++;assert.throws(()=>p(s),e=>e instanceof E&&e.code===code);};
accept(wrap('unit '+'A'.repeat(64)+';'));reject(wrap('unit '+'A'.repeat(65)+';'),'IDENTIFIER_BOUND');
for(const c of ['a','é','😀']) {const n=1024/Buffer.byteLength(c);accept(wrap('const x:T="'+c.repeat(n)+'";'));reject(wrap('const x:T="'+c.repeat(n+1)+'";'),'STRING_BOUND');}
accept(wrap('const x:T='+'1'.repeat(78)+';'));reject(wrap('const x:T='+'1'.repeat(79)+';'),'INTEGER_BOUND');
accept(wrap('unit A;'.repeat(256)));reject(wrap('unit A;'.repeat(257)),'DECLARATION_BOUND');
for(const statement of ['requires true;','ensures true;']) {accept(act(statement.repeat(256)));reject(act(statement.repeat(257)),'STATEMENT_BOUND');}
accept(act('requires true;'.repeat(128)+'ensures true;'.repeat(128)));reject(act('requires true;'.repeat(128)+'ensures true;'.repeat(129)),'STATEMENT_BOUND');
for(const n of [64,65]) {
const rows=[act('let x=f('+Array(n).fill('a').join(',')+');'),wrap('action x('+Array(n).fill('a:T').join(',')+'){}'),act('emit T{'+Array(n).fill('a:1').join(',')+'};')];
for(const row of rows)n===64?accept(row):reject(row,'ARITY_BOUND');
}
accept(act('let x=a'+'.b'.repeat(63)+';'));reject(act('let x=a'+'.b'.repeat(64)+';'),'NESTING_BOUND');
const base=wrap('/* */');accept(base.replace('/* */','/*'+'x'.repeat(65536-Buffer.byteLength(base)+1)+'*/'));reject(' '.repeat(65537),'SOURCE_BOUND');
for(const raw of ['"\\u0041"','"A"','"\\uD83D\\uDE00"','"\\/"','"\\b\\f\\n\\r\\t"']) {const a=accept(wrap('const x:T='+raw+';'));assert.equal(a.agreement.declarations[0].value.raw,raw);assert.equal(a.agreement.declarations[0].value.decoded,JSON.parse(raw));}
for(const text of ['"\\uD800"','"\\uDC00"','"\\uD800\\u0041"'])reject(wrap('const x:T='+text+';'),'INVALID_SURROGATE');
for(const n of [4083,4084]) {const s=wrap('const a:T<'+Array(n).fill('A').join(',')+'>=1;const b:T<A>=1;');n===4083?accept(s):reject(s,'TOKEN_BOUND');}
let state=739175; const rand=n=>{state=(1664525*state+1013904223)>>>0;return state%n};
function expr(d){if(!d)return ['x','true','123','"\\u0041é😀"','f()'][rand(5)];switch(rand(8)){case 0:return 'not '+expr(d-1);case 1:return '('+expr(d-1)+').field';case 2:return 'f('+expr(d-1)+','+expr(d-1)+')';default:return '('+expr(d-1)+') '+['+','-','*','and','or','==','!=','<','<=','>','>='][rand(11)]+' ('+expr(d-1)+')';}}
for(let i=0;i<3000;i++)accept(wrap('const v:T='+expr(4)+';'));
let rejectedMutations=0,acceptedMutations=0;
const seed=wrap('action a(x:T){requires x>=1;let y=f(x,2);next.x=y;emit T{a:y};ensures true;}');
const chars=['\0','\uD800','😀','é','"','\\','/','*','{','}','(',')',';','=','<','>','a',' ','\r','\n'];
for(let i=0;i<2000;i++) {const at=rand(seed.length);const s=seed.slice(0,at)+chars[rand(chars.length)]+seed.slice(at+rand(2));try{p(s);accept(s);acceptedMutations++;}catch(e){assert.ok(e instanceof E);assert.ok(Number.isInteger(e.start)&&e.start>=0&&e.end>=e.start);rejectedMutations++;}}
console.log(JSON.stringify({cases,randomRoundtrips:3000,mutationCases:2000,acceptedMutations,rejectedMutations}));
