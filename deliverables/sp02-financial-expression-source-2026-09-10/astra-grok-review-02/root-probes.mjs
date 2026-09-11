import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
const root=process.argv[2];
const {createFinancialExpressionSourceV1}=await import(`${root}/experiments/moriarty-language/src/successor/financial-expression-source-v1.ts`);
const {formatFinancialExpressionSource}=await import(`${root}/experiments/moriarty-language/src/successor/financial-expression-source-frontend.ts`);
const {canonical}=await import(`${root}/experiments/moriarty-language/src/successor/expression-wire-v1.ts`);
const base=`${root}/experiments/moriarty-language/spec/successor/examples/financial-vault-quote`;
const source=readFileSync(base+'.mori','utf8');
const schema=readFileSync(base+'.schema.json','utf8');
const language=createFinancialExpressionSourceV1(schema);
const initial=JSON.parse(readFileSync(base+'.snapshots.json','utf8'));
let cases=0;
const MAX=(1n<<128n)-1n;
for(const [deposit,supply,valuation] of [[0n,0n,1n],[1n,1n,3n],[4n,3n,10n],[MAX,MAX,MAX],[MAX,2n,2n],[MAX,MAX,1n],[123456789n,987654321n,97n]]){
 const snapshot=structuredClone(initial);
 Object.assign(snapshot.Args,{deposit:String(deposit),supply:String(supply),valuation:String(valuation)});
 const expected=deposit*supply/valuation;
 const result=language.evaluate(source,canonical(snapshot));
 if(expected>MAX){assert.equal(result.status,'Rejected');assert.equal('post' in result,false);assert.equal('descriptors' in result,false);}
 else {assert.equal(result.status,'ExpressionPrepared');assert.deepEqual(result.post,{last:String(expected)});assert.deepEqual(result.descriptors,[{operation:'Notice',fields:{allocation:String(expected),offered:String(deposit)}}]);}
 cases++;
}
const formatted=formatFinancialExpressionSource(source);
assert.equal(formatFinancialExpressionSource(formatted),formatted);
assert.deepEqual(language.check(formatted),language.check(source));
const original=language.evaluate(source,canonical(initial));
const afterFormat=language.evaluate(formatted,canonical(initial));
assert.deepEqual(afterFormat.post,original.post);assert.deepEqual(afterFormat.descriptors,original.descriptors);assert.equal(afterFormat.workUsed,original.workUsed);cases++;
const selected=structuredClone(initial);selected.Args.useSupplied=true;
const failure=language.evaluate(source,canonical(selected));
assert.equal(failure.status,'Rejected');assert.equal(failure.code,'OPTION_NONE');assert.equal('post' in failure,false);assert.equal('descriptors' in failure,false);cases++;
const altered=language.elaborate(source);altered.core.statements.length=0;
assert.deepEqual(language.evaluate(source,canonical(initial)),original);cases++;
console.log(JSON.stringify({scope:'Independent public API vault arithmetic, formatting, rollback, and returned-Core ownership',cases,passed:true},null,2));
