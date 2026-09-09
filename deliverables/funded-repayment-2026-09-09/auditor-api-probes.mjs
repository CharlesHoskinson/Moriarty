import assert from 'node:assert/strict';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';

const target = process.argv[2];
const {prepareRepayment: prepare, REPAYMENT_VERSION: version, REPAYMENT_BOUNDS: bounds} = await import(pathToFileURL(target).href);
const results = [];
const cp = structuredClone;
const max = '340282366920938463463374607431768211455';
function base() {
  return {schemaVersion:version,state:{balances:[{party:'Payer',asset:'USD_TOKEN',amount:'100'},{party:'Lender',asset:'USD_TOKEN',amount:'0'}],allowances:[{party:'Payer',asset:'USD_TOKEN',remaining:'100',spent:'0'}],obligations:[{id:'Loan',debtor:'Payer',creditor:'Lender',denomination:'USD_micro',settlementAsset:'USD_TOKEN',principal:'100',accrued:'0',outstanding:'100',allocationRule:'AccrualFirst',conversion:{mantissa:'1',scale:'0',rounding:'none'},status:'Outstanding'}],usedTransferIds:[],usedAllocationIds:[],work:{remaining:'100',spent:'0',closureReserve:'16'}},actions:[{kind:'Transfer',id:'Payment',from:'Payer',to:'Lender',asset:'USD_TOKEN',amount:'30'},{kind:'Repay',allocationId:'Allocation',transferId:'Payment',obligationId:'Loan',payer:'Payer',nominalAmount:'30'}]};
}
function test(name, fn) {try {fn();results.push({name,ok:true});} catch(e) {results.push({name,ok:false,error:e.message});}}
function run(input) {const before=JSON.stringify(input);const result=prepare(before);assert.equal(JSON.stringify(input),before);return result;}
function reject(input,index) {const r=typeof input==='string'?prepare(input):run(input);assert.equal(r.status,'Rejected',JSON.stringify(r));assert.deepEqual(Object.keys(r).sort(),['actionIndex','code','status']);assert.equal(typeof r.code,'string');assert.ok(r.code.length>0);if(index!==undefined)assert.equal(r.actionIndex,index);return r;}
function good(input) {const r=run(input);assert.equal(r.status,'Prepared',JSON.stringify(r));assert.deepEqual(Object.keys(r).sort(),['effects','post','schemaVersion','status']);assert.equal(r.effects.length,input.actions.length);assert.equal(r.schemaVersion,version);assert.doesNotThrow(()=>JSON.stringify(r));return r;}
test('bounded constants immutable',()=>{assert.equal(version,'moriarty-funded-repayment/0');assert.deepEqual(bounds,{sourceUtf8Bytes:65536,collectionCapacity:128,identifierCharacters:64,maxScale:18,uint128Max:max});assert.ok(Object.isFrozen(bounds));});
test('baseline exact cash principal allowance work',()=>{const r=good(base());assert.deepEqual(r.post.balances.map(x=>x.amount),['70','30']);assert.equal(r.post.obligations[0].principal,'70');assert.deepEqual(r.post.allowances[0],{party:'Payer',asset:'USD_TOKEN',remaining:'70',spent:'30'});assert.deepEqual(r.post.work,{remaining:'98',spent:'2',closureReserve:'16'});});
test('prototype member names are legitimate identifiers',()=>{const x=base();x.actions[0].id='constructor';x.actions[1].transferId='constructor';x.actions[1].allocationId='toString';x.state.obligations[0].id='hasOwnProperty';x.actions[1].obligationId='hasOwnProperty';good(x);});
test('namespaces separate',()=>{const x=base();x.actions[1].allocationId='Payment';x.state.obligations[0].id='Payment';x.actions[1].obligationId='Payment';x.state.usedAllocationIds=['Other'];x.state.usedTransferIds=['Allocation'];good(x);});
test('all untouched records and order survive',()=>{const x=base();x.state.balances.unshift({party:'Zed',asset:'OTHER',amount:'57'});x.state.allowances.unshift({party:'Zed',asset:'OTHER',remaining:'9',spent:'4'});const o={...cp(x.state.obligations[0]),id:'Unrelated',status:'Settled',principal:'0',accrued:'0',outstanding:'0'};x.state.obligations.unshift(o);x.state.usedAllocationIds=['PreviousAllocation'];x.state.usedTransferIds=['PreviousTransfer'];const r=good(x);assert.deepEqual(r.post.balances[0],x.state.balances[0]);assert.deepEqual(r.post.allowances[0],x.state.allowances[0]);assert.deepEqual(r.post.obligations[0],o);assert.deepEqual(r.post.usedTransferIds,['PreviousTransfer','Payment']);assert.deepEqual(r.post.usedAllocationIds,['PreviousAllocation','Allocation']);});
test('third-party payer uses own account and allowance',()=>{const x=base();x.state.obligations[0].debtor='OtherDebtor';const r=good(x);assert.equal(r.post.obligations[0].debtor,'OtherDebtor');});
test('same payer different assets remain independent',()=>{const x=base();x.state.balances.push({party:'Payer',asset:'Other',amount:'88'});x.state.allowances.push({party:'Payer',asset:'Other',remaining:'44',spent:'2'});const r=good(x);assert.deepEqual(r.post.balances[2],x.state.balances[2]);assert.deepEqual(r.post.allowances[1],x.state.allowances[1]);});
test('fresh receiver append preserves order',()=>{const x=base();x.state.balances.pop();const r=good(x);assert.deepEqual(r.post.balances,[{party:'Payer',asset:'USD_TOKEN',amount:'70'},{party:'Lender',asset:'USD_TOKEN',amount:'30'}]);});
test('refund retains gross spending and pays own allowance',()=>{const x=base();x.state.allowances.push({party:'Lender',asset:'USD_TOKEN',remaining:'30',spent:'0'});x.actions.push({kind:'Transfer',id:'Refund',from:'Lender',to:'Payer',asset:'USD_TOKEN',amount:'30'});const r=good(x);assert.deepEqual(r.post.balances.map(x=>x.amount),['100','0']);assert.deepEqual(r.post.allowances.map(x=>[x.remaining,x.spent]),[['70','30'],['0','30']]);assert.equal(r.post.obligations[0].outstanding,'70');});
test('refund cannot allow repeated gross spending above cap',()=>{const x=base();x.state.allowances[0].remaining='30';x.state.allowances.push({party:'Lender',asset:'USD_TOKEN',remaining:'30',spent:'0'});x.actions.push({kind:'Transfer',id:'Refund',from:'Lender',to:'Payer',asset:'USD_TOKEN',amount:'30'},{kind:'Transfer',id:'Again',from:'Payer',to:'Lender',asset:'USD_TOKEN',amount:'1'});reject(x,3);});
test('settled obligation preserved and histories reject next-call replay',()=>{const x=base();x.actions[0].amount='100';x.actions[1].nominalAmount='100';const r=good(x);assert.equal(r.post.obligations.length,1);assert.equal(r.post.obligations[0].status,'Settled');const next={...x,state:r.post};reject(next,0);});
test('past transfer with surplus cannot fund a later call',()=>{const x=base();x.actions[0].amount='50';const r=good(x);const next={...x,state:r.post,actions:[{...x.actions[1],allocationId:'NewAllocation',nominalAmount:'1'}]};reject(next,0);});
test('new payment and allocation can continue a residual debt',()=>{const x=base();const r=good(x);x.state=r.post;x.actions[0].id='Payment2';x.actions[1].transferId='Payment2';x.actions[1].allocationId='Allocation2';const r2=good(x);assert.equal(r2.post.obligations[0].outstanding,'40');assert.deepEqual(r2.post.work,{remaining:'96',spent:'4',closureReserve:'16'});});
test('prior allocation cannot repay different obligation',()=>{const x=base();const r=good(x);x.state=r.post;x.state.obligations.push({...cp(x.state.obligations[0]),id:'NewLoan'});x.actions[0].id='Payment2';x.actions[1].transferId='Payment2';x.actions[1].obligationId='NewLoan';reject(x,1);});
test('two distinct obligations share exact available funding',()=>{const x=base();x.state.obligations.push({...cp(x.state.obligations[0]),id:'Loan2'});x.actions[1].nominalAmount='10';x.actions.push({...x.actions[1],allocationId:'Allocation2',obligationId:'Loan2',nominalAmount:'20'});const r=good(x);assert.deepEqual(r.post.obligations.map(x=>x.outstanding),['90','80']);x.actions[1].nominalAmount='20';reject(x,2);});
for(const [name,change,index] of [
  ['missing transfer',x=>x.actions.shift(),0],
  ['future transfer',x=>x.actions.reverse(),0],
  ['wrong payer',x=>x.actions[1].payer='Other',1],
  ['wrong creditor',x=>x.state.obligations[0].creditor='Other',1],
  ['wrong asset',x=>x.state.obligations[0].settlementAsset='Other',1],
  ['too little transfer',x=>x.actions[0].amount='1',1],
  ['insufficient cash',x=>x.state.balances[0].amount='29',0],
  ['insufficient allowance',x=>x.state.allowances[0].remaining='29',0],
  ['missing exact allowance',x=>x.state.allowances[0].asset='Other',0],
  ['insufficient work cannot consume reserve',x=>x.state.work.remaining='1',null],
  ['balance credit overflow',x=>x.state.balances[1].amount=max,0],
  ['zero transfer',x=>x.actions[0].amount='0',0],
  ['self transfer',x=>x.actions[0].to='Payer',0],
  ['zero repay',x=>x.actions[1].nominalAmount='0',1],
  ['invalid late shape precedes any execution',x=>{x.actions[0].amount='1000';x.actions[1].unknown=true;},null],
]) test(name,()=>{const x=base();change(x);reject(x,index);});
for(const key of ['constructor','prototype','__proto__','label','authorized','profile']) test('closed root rejects '+key,()=>{const x=base();Object.defineProperty(x,key,{value:true,enumerable:true});reject(x,null);});
for(const [name,at] of [['state',x=>x.state],['balance',x=>x.state.balances[0]],['allowance',x=>x.state.allowances[0]],['obligation',x=>x.state.obligations[0]],['conversion',x=>x.state.obligations[0].conversion],['work',x=>x.state.work],['transfer',x=>x.actions[0]],['repay',x=>x.actions[1]]])test('closed nested '+name,()=>{const x=base();at(x).unexpected='ignored';reject(x,null);});
for(const [name,at] of [['root',x=>x],['state',x=>x.state],['balance',x=>x.state.balances[0]],['allowance',x=>x.state.allowances[0]],['obligation',x=>x.state.obligations[0]],['conversion',x=>x.state.obligations[0].conversion],['work',x=>x.state.work],['transfer',x=>x.actions[0]],['repay',x=>x.actions[1]]])for(const key of Object.keys(at(base())))test('required '+name+'.'+key,()=>{const x=base();delete at(x)[key];reject(x,null);});
for(const value of ['Payer\n','Payer\r','Payer\u2028','Payer\u2029','A'.repeat(65),'','_Bad','9Bad','é','\ud800'])test('bad identifier '+JSON.stringify(value),()=>{const x=base();x.actions[0].id=value;reject(x,null);});
for(const value of ['01','00','+1','-1','1.0','1e1','1\n',' 1','',null,true,1,{},[],(BigInt(max)+1n).toString()])test('bad amount '+JSON.stringify(value),()=>{const x=base();x.actions[0].amount=value;reject(x,null);});
for(const collection of ['balances','allowances','obligations','usedTransferIds','usedAllocationIds'])test('duplicate '+collection,()=>{const x=base();if(collection.startsWith('used'))x.state[collection]=['History','History'];else x.state[collection].push(cp(x.state[collection][0]));reject(x,null);});
for(const collection of ['usedTransferIds','usedAllocationIds']) {
  test(collection+' 127 permits final slot',()=>{const x=base();x.state[collection]=Array.from({length:127},(_,i)=>'Historical'+i);good(x);});
  test(collection+' 128 rejects append',()=>{const x=base();x.state[collection]=Array.from({length:128},(_,i)=>'Historical'+i);reject(x,collection==='usedTransferIds'?0:1);});
  test(collection+' 129 rejects admission',()=>{const x=base();x.state[collection]=Array.from({length:129},(_,i)=>'Historical'+i);reject(x,null);});
}
test('128 balances allow existing recipient',()=>{const x=base();for(let i=2;i<128;i++)x.state.balances.push({party:'Party'+i,asset:'USD_TOKEN',amount:'0'});good(x);});
test('128 balances reject new recipient at transfer',()=>{const x=base();x.state.balances.pop();for(let i=1;i<128;i++)x.state.balances.push({party:'Party'+i,asset:'USD_TOKEN',amount:'0'});reject(x,0);});
test('128 actions boundary succeeds with exact work',()=>{const x=base();x.state.balances[0].amount='128';x.state.allowances[0].remaining='128';x.state.work.remaining='128';x.actions=Array.from({length:128},(_,i)=>({...x.actions[0],id:'T'+i,amount:'1'}));const r=good(x);assert.equal(r.post.work.remaining,'0');assert.equal(r.post.work.spent,'128');});
test('129 actions rejected before work',()=>{const x=base();x.actions=Array.from({length:129},(_,i)=>({...x.actions[0],id:'T'+i,amount:'1'}));reject(x,null);});
test('unknown action rejects admission',()=>{const x=base();x.actions[1].kind='Mint';reject(x,null);});
test('work aggregate overflow rejects admission',()=>{const x=base();x.state.work.remaining=max;reject(x,null);});
test('allowance aggregate overflow rejects admission',()=>{const x=base();x.state.allowances[0].remaining=max;x.state.allowances[0].spent='1';reject(x,null);});
test('obligation aggregate overflow rejects admission',()=>{const x=base();x.state.obligations[0].principal=max;x.state.obligations[0].accrued='1';x.state.obligations[0].outstanding='0';reject(x,null);});
test('conversion multiplication rejects before quotient',()=>{const x=base();x.state.obligations[0].conversion={mantissa:max,scale:'18',rounding:'floor'};reject(x,1);});
test('scale eighteen exact boundary',()=>{const x=base();x.state.obligations[0].conversion={mantissa:'1000000000000000000',scale:'18',rounding:'none'};good(x);});
test('scale nineteen rejects',()=>{const x=base();x.state.obligations[0].conversion.scale='19';reject(x,null);});
test('source primitive admission never calls coercion hooks',()=>{let calls=0;for(const x of [null,undefined,7,3n,Symbol('source'),{},[],new String(JSON.stringify(base())),{toString(){calls++;throw Error('called');},[Symbol.toPrimitive](){calls++;throw Error('called');}}]) {const r=prepare(x);assert.equal(r.status,'Rejected');assert.equal(r.actionIndex,null);}assert.equal(calls,0);});
test('hostile proxy never observed',()=>{let calls=0;const r=prepare(new Proxy({},{get(){calls++;throw Error('get');},getPrototypeOf(){calls++;throw Error('prototype');}}));assert.equal(r.status,'Rejected');assert.equal(calls,0);});
test('nonminimal source forms reject',()=>{const s=JSON.stringify(base());for(const v of [s+'\n',' '+s,s.replace('Payer','\\u0050ayer'),s.replace('"schemaVersion":','"schemaVersion":"bad","schemaVersion":')])reject(v,null);});
test('key reorder remains accepted input, not signing encoding',()=>{const x=base();good({actions:x.actions,state:x.state,schemaVersion:x.schemaVersion});});
test('malformed and deeply nested strings return only rejection',()=>{for(const s of ['','{','null','[]','"\\ud800"','"\ud800"','['.repeat(12000)+'0'+']'.repeat(12000),' '.repeat(65537),'"'+'é'.repeat(33000)+'"'])reject(s,null);});
test('effect complete and ordered',()=>{const x=base();const r=good(x);assert.deepEqual(r.effects[0],x.actions[0]);assert.deepEqual(r.effects[1],{kind:'Repayment',allocationId:'Allocation',transferId:'Payment',obligationId:'Loan',payer:'Payer',creditor:'Lender',denomination:'USD_micro',settlementAsset:'USD_TOKEN',nominalAmount:'30',settlementAmount:'30',principalDischarged:'30',accruedDischarged:'0',remainingOutstanding:'70'});});
console.log(JSON.stringify({source:target,sha256:createHash('sha256').update(readFileSync(target)).digest('hex'),total:results.length,passed:results.filter(r=>r.ok).length,failed:results.filter(r=>!r.ok)},null,2));
process.exitCode=results.some(r=>!r.ok)?1:0;
