import test from 'node:test';
import assert from 'node:assert/strict';
const frontend = await import('../src/frontend.ts').catch(() => ({}));
const analyze = s => { assert.equal(typeof frontend.analyze, 'function', 'frontend analyze must exist'); return frontend.analyze(s); };
const wrap = body => `profile "moriarty-beta/1"; agreement Invoice { ${body} }`;
const base = `domain Preview = { id: "Midnight", chain: "midnight", network: "preview" };
account Buyer = { domain: Preview, id: "Owner" }; account Seller = { domain: Preview, id: "Recipient" };
account Treasury = { domain: Preview, id: "Fee" };
asset USD = { domain: Preview, id: "A", scale: 2, representation: "canonical", symbol: "USD" };`;
const intent = `intent Payment = { domain: Preview, asset: USD, signer: Buyer, key: "key1", nonce: "n1", pre_head: "h0",
valid: rounds(domain: Preview, from: 100, to: 200), gross_cap: 10.10 USD, fee_cap: 0.10 USD, net_floor: 10.00 USD,
operation: transfer(from: Buyer, to: Seller, fee_to: Treasury, value: 10.00 USD, fee: 0.10 USD),
source_hash: "src1", policy_digest: "policy1", failure: SuccessOnly, observations: [], disclosures: [], retained_effects: [], retained_duties: [], delegation: None, recovery: None }; action pay uses Payment;`;
const good = tail => wrap(base + tail);
const rejected = s => { const a=analyze(s); assert.equal(a.status,'AuthoringRejected', JSON.stringify(a)); assert.ok(a.diagnostics[0].code); return a; };
const accepted = s => { const a=analyze(s); assert.deepEqual(a.diagnostics,[]); assert.equal(a.status,'AuthoringChecked'); return a; };
test('exact decimals, separated comments and quantity algebra',()=> {
 const a=accepted(good('const x: Qty<USD> = 1_000.01 /* units */ USD; const y = min(a: x * 2, b: 3000 USD) - 0.01 USD;'));
 assert.equal(a.declarations.at(-1).value.atoms,'200001');
 for(const amount of ['10.001 USD','10.00USD','01 USD','1__0 USD','1_00.0_ USD','10.00']) rejected(good(`const x = ${amount};`));
});
test('nominal types and annotation matching',()=> {
 const extra='asset EUR = { domain: Preview, id: "E", scale: 2, representation: "canonical" };';
 rejected(good(extra+'const x = 1 USD + 1 EUR;')); rejected(good('const x: Scalar = 1 USD;'));
 rejected(good('const x: Qty<USD> = 1;')); rejected(good('const x = 1 USD * 1 USD;'));
});
test('strict prior references, duplicate declarations, keys and args',()=> {
 for(const s of ['const x = y; const y = 1;','const x = 1; const x = 2;','const x = { a: 1, a: 2 };','const x = min(a: 1, a: 2, b: 3);','const x = Unknown;']) rejected(good(s));
});
test('economic identities, source6 alphabet, schema closure',()=> {
 rejected(good('account Alias = { domain: Preview, id: "Owner" };'));
 rejected(good('asset Alias = { domain: Preview, id: "A", scale: 3, representation: "wrapped" };'));
 for(const id of ['Owner-1','Owner.1','transfer']) rejected(wrap(base.replace('"Owner"',JSON.stringify(id))));
 accepted(wrap(base.replace('"Owner"','"Owner_1"')));
 rejected(wrap(base.replace('scale: 2','scale: 19'))); rejected(wrap(base.replace('scale: 2','scale: 2, typo: 1')));
 rejected(wrap(base.replace('domain: Preview, id: "Owner"','domain: USD, id: "Owner"')));
});
test('local S0 sealed schemas, nominal domains and UInt127 assignments',()=> {
 const a=accepted(good(intent)); assert.equal(a.actions[0].support,'LocalS0');
 rejected(good(intent.replace('delegation: None,',''))); rejected(good(intent.replace('failure: SuccessOnly','failure: "SuccessOnly"')));
 rejected(good(intent.replace('fee: 0.10 USD','fee: 0.10 USD, typo: 1')));
 rejected(good(intent.replace('fee_to: Treasury','fee_to: Seller')));
 rejected(good(intent.replace('signer: Buyer','signer: Seller')));
 rejected(good(intent.replace('gross_cap: 10.10 USD',`gross_cap: atoms(asset: USD, value: ${2n**127n})`)));
 accepted(good('const wide = atoms(asset: USD, value: '+(2n**128n-1n)+');'));
 rejected(good('const wide = '+(2n**128n-1n)+' + 1;')); rejected(good('const x = 1 - 2;'));
 accepted(good(intent.replace('value: 10.00 USD','value: 0.00 USD')));
});
test('horizon closed operation schema and truthful check coverage',()=> {
 const a=accepted(good('pool Spot = { domain: Preview, id: "Spot", assets: [USD] }; intent H = { operation: amm.redeem(pool: Spot, owner: Buyer, share_atoms: 1) }; action redeem uses H;'));
 assert.equal(a.actions[0].support,'SpecifiedOnly');
 rejected(good('intent H = { operation: amm.typo() };')); rejected(good('intent H = { operation: amm.redeem(pool: Buyer, owner: Buyer, share_atoms: 1) };'));
 const summary=frontend.check(good(intent)); assert.ok(summary.operationSchemas); assert.doesNotThrow(()=>JSON.stringify(summary)); assert.equal(summary.declarations[0].value,undefined);
});
test('byte spans, UTF16 boundaries, CRLF and malformed unicode',()=> {
 const s='// 😀\r\n'+good('const greeting = "é😀";'); const a=accepted(s); const d=a.declarations.at(-1);
 assert.equal(Buffer.from(s).subarray(d.nameSpan.start,d.nameSpan.end).toString(),'greeting');
 assert.deepEqual(frontend.byteToPosition('é😀\r\nx',6),{line:0,character:3});
 assert.deepEqual(frontend.byteToPosition('é😀\r\nx',8),{line:1,character:0});
 rejected(good('const x = "\\ud800";')); rejected(good('const x = "\ud800";'));
});
test('bounded source, nesting, declaration memo DAG and immutable values',()=> {
 rejected(' '.repeat(65537)); rejected(good('const x = '+ '('.repeat(65)+'1'+')'.repeat(65)+';'));
 rejected(good('const x = "'+ 'x'.repeat(1025)+'";'));
 let body='const a = { value: 1 };'; for(let i=0;i<150;i++) body+=`const a${i} = [${i?`a${i-1}`:'a'}, ${i?`a${i-1}`:'a'}];`;
 const a=accepted(wrap(body)); assert.equal(a.declarations.at(-1).value.items[0],a.declarations.at(-1).value.items[1]);
 assert.ok(Object.isFrozen(a.declarations.at(-1).value));
 rejected(wrap(Array.from({length:257},(_,i)=>`const a${i}=1;`).join('')));
});
test('formatter preserves spelling/comments/meaning, is idempotent, rejects incomplete edits',()=> {
 const s='// header 😀\n'+good('const x=10.00/*money*/USD;// boundary\n'+intent);
 const f=frontend.format(s); assert.deepEqual(f.diagnostics,[]); assert.ok(f.text.includes('// boundary\n')); assert.ok(f.text.includes('10.00')); assert.ok(f.text.includes('/*money*/'));
 assert.equal(frontend.format(f.text).text,f.text); assert.equal(accepted(f.text).declarations.find(d=>d.name==='x').value.atoms,'1000');
 for(const x of [s.slice(0,-1),good('const x = ;'),s+'garbage']) { assert.equal(frontend.format(x).text,null); rejected(x); }
});
test('horizon resource roles require declared assets and retain closed call schemas',()=> {
 rejected(good('instrument Empty = { domain: Preview, id: "Empty" }; intent H = { operation: stablecoin.mint(instrument: Empty, owner: Buyer, supply: 1 USD, backing: 1 USD) };'));
 rejected(good('intent H = { operation: governance.queue(policy: Buyer, next_epoch: 1) };'));
 rejected(good('pool P = { domain: Preview, id: "P", assets: [USD] }; intent H = { domain: 1, operation: amm.redeem(pool: P, owner: Buyer, share_atoms: 1) };'));
 const one=frontend.check(good(intent)); try { one.operationSchemas.transfer.evil='scalar'; } catch {}
 accepted(good(intent));
 const two=frontend.check(good(intent)); assert.equal(two.operationSchemas.transfer.evil,undefined);
});
test('expression spans belong to use site; annotations resolve before self binding',()=> {
 const s=good('const a = 1; const b = a + 2;'); const a=accepted(s); const v=a.declarations.at(-1).value;
 assert.equal(Buffer.from(s).subarray(v.span.start,v.span.end).toString(),'a + 2');
 rejected(good('const USD: Qty<USD> = 1;'));
});
test('quantity precision never rounds even trailing zero; operation width leaves sums to Core',()=> {
 rejected(good('const x = 1.000 USD;')); accepted(good('const x = atoms(asset: USD, value: 0);'));
 const max=(2n**127n-1n).toString(); const a=accepted(good(intent.replace('value: 10.00 USD',`value: atoms(asset: USD, value: ${max})`).replace('fee: 0.10 USD',`fee: atoms(asset: USD, value: ${max})`)));
 assert.equal(a.actions[0].support,'LocalS0');
 rejected(good('const x = -1;')); rejected(good('const x = max(a: 1 USD, b: 1);'));
});
test('invalid arbitrary host calls, field keys and malformed profiles stay rejected',()=> {
 for(const s of ['const x = eval(a: "1");','const x = { __proto__: 1 };','const x = constructor();','const x = 0_1;','const x = 00.10 USD;']) rejected(good(s));
 rejected(good(intent).replace('moriarty-beta/1','moriarty-beta/2')); rejected(good(intent).replace('agreement Invoice','agreement transfer'));
});
test('published horizon catalog accepts every fixed schema and rejects added args',()=> {
 const resources=`pool P = { domain: Preview, id: "P", assets: [USD] };
obligation Loan = { domain: Preview, id: "Loan", asset: USD };
instrument I = { domain: Preview, id: "I", asset: USD, backing: USD, underlying: USD, settlement: USD };
observation O = { domain: Preview, id: "O" }; policy Policy = { epoch: 1 };
share_class Shares = { domain: Preview, id: "Shares", backing: USD };`;
 const roleValue={pool:'P',account:'Buyer',asset:'USD',obligation:'Loan',instrument:'I',observation:'O',policy:'Policy',share_class:'Shares',domain:'Preview',scalar:'1',qty:'1 USD','qty[]':'[1 USD]',string:'"claim1"'};
 const catalog=frontend.check(good(intent)).operationSchemas;
 assert.equal(Object.keys(catalog).filter(n=>n.includes('.')).length,24);
 for(const [name,schema] of Object.entries(catalog).filter(([n])=>n.includes('.'))) {
  const args=Object.entries(schema).map(([key,role])=>`${key}: ${roleValue[role]}`).join(', ');
  const src=good(resources+`intent H = { operation: ${name}(${args}) }; action horizon uses H;`);
  assert.equal(accepted(src).actions[0].support,'SpecifiedOnly',name);
  rejected(src.replace(`${name}(${args})`,`${name}(${args}, typo: 1)`));
 }
});
test('domain nominal identity, exact width and token/field bounds',()=> {
 const foreign='domain Other = { id: "Elsewhere", chain: "else", network: "test" }; account Visitor = { domain: Other, id: "Owner" };';
 accepted(good(foreign)); rejected(good(foreign+intent.replace('to: Seller','to: Visitor')));
 rejected(good('domain Other = { id: "Midnight", chain: "else", network: "test" };'));
 accepted(good(`const x = ${2n**128n-1n};`)); rejected(good(`const x = ${2n**128n};`));
 accepted(good(intent.replace('gross_cap: 10.10 USD',`gross_cap: atoms(asset: USD, value: ${2n**127n-1n})`)));
 rejected(wrap('const x = {'+Array.from({length:65},(_,i)=>`f${i}: 1`).join(',')+'};'));
 rejected(wrap('const x = ['+'1,'.repeat(4100)+'];'));
});
test('bounded reference metadata preserves use and declaration without cloning DAG',()=> {
 const s=good('const a = [1]; const b = [a, a]; const q: Qty<USD> = 1 USD;');
 const a=accepted(s), declaration=a.declarations.find(x=>x.name==='a');
 const uses=a.references.filter(r=>r.name==='a'); assert.equal(uses.length,2);
 for(const r of uses) { assert.equal(Buffer.from(s).subarray(r.useSpan.start,r.useSpan.end).toString(),'a'); assert.deepEqual(r.declarationSpan,declaration.span); }
 const b=a.declarations.find(x=>x.name==='b').value; assert.equal(b.items[0],declaration.value); assert.equal(b.items[1],declaration.value);
 assert.ok(a.references.some(r=>r.name==='USD' && Buffer.from(s).subarray(r.useSpan.start,r.useSpan.end).toString()==='USD'));
});
test('annotation nominal references retain their own editor use spans',()=> {
 const s=good('const x: Qty<USD> = 1 USD;'); const a=accepted(s);
 const uses=a.references.filter(r=>r.name==='USD'); assert.equal(uses.length,2);
 const spellings=uses.map(r=>Buffer.from(s).subarray(r.useSpan.start,r.useSpan.end).toString()); assert.deepEqual(spellings,['USD','USD']);
});
test('specified declaration fields preserve nominal and textual authoring roles',()=> {
 rejected(good('policy P = { source_hash: 1 USD };'));
 rejected(good('instrument I = { domain: Preview, id: "I", strike: 1 };'));
 rejected(good('grant G = { signers: [USD] };'));
 rejected(good('stage S = { domain: Preview, reads: [Buyer] };'));
 rejected(good('pool P = { domain: Preview, id: "P", assets: [USD] }; intent H = { operation: amm.redeem(pool: P, owner: Buyer, share_atoms: 1), policy: Buyer };'));
 accepted(good('policy P = { epoch: 1, source_hash: "hash", duty_preservation: "required" };'));
});
test('agreement origin is its exact declaration identifier',()=> {
 const s=good(intent), a=accepted(s);
 assert.ok(a.agreementSpan); assert.equal(Buffer.from(s).subarray(a.agreementSpan.start,a.agreementSpan.end).toString(),'Invoice');
 assert.equal(analyze('profile "broken";').agreementSpan,null);
});

test('line comments terminate on CR, LF and CRLF without consuming subsequent declarations',()=>{
 for(const newline of ['\r','\n','\r\n']){
  const s=`profile "moriarty-beta/1";${newline}agreement Commented {${newline}// 😀 comment${newline}const amount = 1; // suffix${newline}const total = amount + 1;${newline}}`;
  const a=frontend.analyze(s);assert.equal(a.status,'AuthoringChecked',JSON.stringify(a.diagnostics));assert.deepEqual(a.declarations.map(d=>d.name),['amount','total']);assert.equal(a.declarations[1].value.value,'2');const ref=a.references.find(r=>r.name==='amount');assert.equal(Buffer.from(s).subarray(ref.useSpan.start,ref.useSpan.end).toString(),'amount');assert.deepEqual(frontend.byteToPosition(s,ref.useSpan.start),{line:4,character:14});
  const f=frontend.format(s);assert.equal(f.diagnostics.length,0);assert.ok(f.text.includes('// 😀 comment'));assert.equal(frontend.analyze(f.text).declarations[1].value.value,'2');assert.equal(frontend.format(f.text).text,f.text);
 }
});
