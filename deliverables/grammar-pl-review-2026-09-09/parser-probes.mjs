import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import { parseSuccessorSource as parse } from '/home/charl/Moriarty/experiments/moriarty-language/src/successor/frontend.ts';
import { formatSuccessorSource as format } from '/home/charl/Moriarty/experiments/moriarty-language/src/successor/format.ts';
const root='/home/charl/Moriarty/experiments/moriarty-language';
const wrap = body => `profile "moriarty-successor-syntax/0"; agreement P { ${body} }`;
const expr = e => wrap(`const x:T=${e};`);
const strip = v => Array.isArray(v)?v.map(strip):v && typeof v==='object'?Object.fromEntries(Object.entries(v).filter(([k])=>k!=='span').map(([k,v])=>[k,strip(v)])):v;
let checks=0;
function accepted(label, source) { const a=parse(source), f=format(source); assert.deepEqual(strip(parse(f)),strip(a), label); assert.equal(format(f),f,label); checks++;console.log('PASS accept/roundtrip '+label); return a; }
function rejected(label, source, code) {assert.throws(()=>parse(source),e=>Boolean(e.code)&&(!code||e.code===code),label);checks++;console.log('PASS reject '+label);}
for(const [label,body] of [
 ['empty',''],['all declarations','unit U; party p; asset a:A<U>; const c:T=0; state s:T=true; action f(x:T,y:T<U,V>) { requires not x==y; let z=f(x,y).field; next.s=z; emit E<U>{a:x,b:y}; ensures x; ensures y; }'],
 ['compact nested generic','const a:Map<Debt<USD>>=debt(1,USD);'], ['split generic comments','const a:Map<Debt<USD>/*x*/>=debt(1,USD);'], ['empty lists','action f(){emit E{};let x=f();}'],
 ['pre/post identifiers','unit pre; unit post; action f(){next.pre=post;}'], ['syntactically valid duplicate unknown types','unit U;unit U;asset a:Unknown<Unknown>;'],
 ['unsupported words as identifiers','unit function;party loop;asset settlement:obligation;'], ['maximal type arity is not call arity','asset a:T<'+Array(65).fill('T').join(',')+'>;']
]) accepted(label,wrap(body));
for(const [label,e] of [ ['precedence','not a+b*c==d and e or f'],['left subtraction','a-b-c'],['right subtraction','a-(b-c)'],['comparison left','(a<b)<c'],['comparison right','a<(b<c)'],['double not','not not a'],['project all primaries','(not a).x+(a<b).y+f().z+1.x+true.y+"s".z'],['raw escapes','"\\u0041\\/\\n\\uD83D\\uDE00"'],['unicode string','"é😀"'],['max integer','9'.repeat(78)], ['token adjacency','1and true'],['identifier adjacent string comparison','x=="x"'],['negative by subtraction','0-1'] ]) accepted(label,expr(e));
for(const [label,body,code] of [ ['trailing parameter','action f(x:T,){}'],['trailing field','action f(){emit E{x:1,};}'],['trailing argument','const x:T=f(1,);'],['trailing type','asset x:T<U,>;'],['empty type','asset x:T<>;','EMPTY_TYPE_ARGS'],['keyword name','unit next;'],['keyword projection','const x:T=pre.asset;'],['after ensures','action f(){ensures true; let a=1;}','STATEMENT_AFTER_ENSURES'],['call on projection','const x:T=f.x();'],['call returned function','const x:T=f()();'],['parenthesized callee','const x:T=(f)();'],['call generics','const x:T=f<U>();'],['unsupported declaration','function f(){}','UNKNOWN_DECLARATION'],['missing separator','unit a unit b;'] ]) rejected(label,wrap(body),code);
for(const e of ['a<b<c','a==b!=c','-1','+1','1.5','1e2','1E+2','1_000','00','01','a/b','a&&b','!a','a**b','a=>b','not','a+not b']) rejected(e,expr(e));
for(const [label,text,code] of [['NBSP','\u00a0','UNEXPECTED_CHAR'],['BOM','\ufeff','UNEXPECTED_CHAR'],['vertical tab','\u000b','UNEXPECTED_CHAR'],['unclosed block','/*x','UNTERMINATED_COMMENT'],['slash','/','UNEXPECTED_CHAR'],['nonASCII name',wrap('unit é;'),'NON_ASCII_IDENTIFIER'],['combining suffix',wrap('unit e\u0301;'),'NON_ASCII_IDENTIFIER'],['leading underscore',wrap('unit _x;'),'UNEXPECTED_CHAR']]) rejected(label,text.startsWith('profile')?text:text+wrap(''),code);
for(const [label,prefix,suffix] of [['line EOF','','//é😀'],['CR line comment','//ignored\ragreement Bad{}\n',''],['first block closer','/* /* */',''],['unicode comments','/*é😀*/','//é😀'],['all whitespace',' \t\r\n','']]) accepted(label,prefix+wrap('')+suffix);
for(const e of ['"\\q"','"\\u000X"','"a\nb"','"\\uD800"','"\\uDC00"','"\\uD800\\u0041"','"unterminated']) rejected('bad string '+JSON.stringify(e),expr(e));
for(const f of readdirSync(root+'/spec/successor/examples').filter(f=>f.endsWith('.mori'))) accepted('example '+f,readFileSync(root+'/spec/successor/examples/'+f,'utf8'));
// Independently expected parse for every adjacent binary-operator pair.
const ops=['or','and','==','!=','<','<=','>','>=','+','-','*']; const precedence={or:1,and:2,'==':4,'!=':4,'<':4,'<=':4,'>':4,'>=':4,'+':5,'-':5,'*':6};
for(const first of ops) for(const second of ops){const source=expr(`a ${first} b ${second} c`);if(precedence[first]===4&&precedence[second]===4){assert.throws(()=>parse(source),e=>e.code==='CHAINED_COMPARISON');}else{const node=parse(source).agreement.declarations[0].value;const expectedRoot=precedence[first]<precedence[second]?first:second;assert.equal(node.operator,expectedRoot);if(expectedRoot===first&&precedence[first]<precedence[second])assert.equal(node.right.operator,second);else assert.equal(node.left.operator,first);assert.deepEqual(strip(parse(format(source))),strip(parse(source)));}checks++;}
console.log('PASS 121 adjacent operator-pair precedence/rejection probes');
// Deterministic grammar-generated expressions: every binary level, projection, call, not.
let seed=9157;function rnd(n){seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed%n;}function gen(d){if(d===0)return ['a','1','true','"x"'][rnd(4)];switch(rnd(5)){case 0:return `(not ${gen(d-1)})`;case 1:return `(${gen(d-1)}).x`;case 2:return `f(${gen(d-1)},${gen(d-1)})`;default:return `(${gen(d-1)} ${ops[rnd(ops.length)]} ${gen(d-1)})`;}}
for(let i=0;i<1000;i++){const source=expr(gen(4));const a=parse(source),f=format(source);assert.deepEqual(strip(parse(f)),strip(a));assert.equal(format(f),f);checks++;}
console.log('PASS 1000 generated expression format roundtrips; seed=9157 depth=4');
console.log(JSON.stringify({checks,failed:0,node:process.version}));
