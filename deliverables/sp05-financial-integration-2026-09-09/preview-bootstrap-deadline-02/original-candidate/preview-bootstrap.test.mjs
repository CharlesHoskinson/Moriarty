import test from 'node:test';
import assert from 'node:assert/strict';
const api=await import('./preview-bootstrap.mjs').catch(()=>({}));
test('Preview bootstrap exposes an explicit existing-wallet launch and public preflight',()=>{
 assert.equal(typeof api.launchPreviewFinancialCase,'function');assert.equal(typeof api.preflightPreviewLaunch,'function');
});
import {mkdtempSync,mkdirSync,readFileSync,existsSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';import {join} from 'node:path';import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
const originalHelpers=await import('./launch-local.mjs');
const pins=JSON.parse(readFileSync(new URL('./launch-runtime-pins.json',import.meta.url)));
async function fixture(t,{missing,invalid,wrongRestore=false,reuse=false,genesisWrong=false}={}){
 const dir=mkdtempSync(join(tmpdir(),'moriarty-preview-bootstrap-')),events=[];t.after(()=>rmSync(dir,{recursive:true,force:true}));mkdirSync(join(dir,'wallet'),{mode:0o700});
 const p={schema:'moriarty.preview-financial-launch/1',kind:'loan',build:{receiptPath:'/public-fixture',receiptSha256:'ab'.repeat(32),sourceManifestHash:'cd'.repeat(32)},networkConfig:{networkId:'preview',node:'https://rpc.preview.midnight.network',indexer:'https://indexer.preview.midnight.network/api/v4/graphql',indexerWS:'wss://indexer.preview.midnight.network/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'},wallet:{stateDirectory:join(dir,'wallet'),seedFile:join(dir,'seed'),expectedAddress:'CONTROLLED_PREVIEW_ADDRESS'},roles:{secretsFile:join(dir,'roles'),firstAddress:'01'.repeat(32),secondAddress:'02'.repeat(32)},privateState:{directory:join(dir,'contract'),passwordFile:join(dir,'password')},networkTag:'03'.repeat(32),expectedProtocolVersion:1000000,limits:{allocationId:'fixture',submissions:4,deadlineMs:Date.now()+10000,dustFee:'1000',grossByLogicalAsset:{USD_TEST_ASSET:'20000000000'}},outputDirectory:join(dir,'run')};
 if(reuse)mkdirSync(p.outputDirectory,{mode:0o700});
 const childDir=join(p.wallet.stateDirectory,'.midnight-wallet-state/preview');mkdirSync(childDir,{recursive:true,mode:0o700});for(const kind of ['shielded','unshielded','dust'])writeFileSync(join(childDir,kind+'.json'),JSON.stringify({version:1,state:'persisted-'+kind}),{mode:0o600});
 // Dependency modules are mocked only by the Node source-test runner. Production
 // launch accepts no adapter objects and uses the fixed real imports.
 t.mock.module(new URL('./preview-launch.mjs',import.meta.url).href,{namedExports:{validatePreviewLaunchPlan:x=>structuredClone(x),retainPreviewIntegrationResult:(_d,r)=>{events.push('result:'+r.status);},retainPreviewStage:(_d,s)=>({status:'RECORDED',stage:s.stage,txId:s.txId}),publicPreviewLaunchEvent:e=>e}});
 t.mock.module(new URL('./proven-assets.mjs',import.meta.url).href,{namedExports:{inspectFinancialBuild:async()=>{events.push('build');}}});
 t.mock.module(new URL('./financial-rpc.mjs',import.meta.url).href,{namedExports:{createPreviewRpc:()=>async(m,params)=>{assert.equal(m,'chain_getBlockHash');assert.deepEqual(params,[0]);events.push('genesis');return '0x'+(genesisWrong?'ff'.repeat(32):p.networkTag);}}});
 t.mock.module(new URL('./launch-local.mjs',import.meta.url).href,{namedExports:{...originalHelpers,inspectLocalLaunchRuntime:async()=>{events.push('pins');},readLocalStoragePassword:async()=>{events.push('private-password');return 'CONTROLLED_PASSWORD';},readPrivateLaunchFile:file=>{
  events.push('private:'+file);assert.ok(existsSync(p.outputDirectory));assert.ok(events.includes('genesis'));
  if(missing&&file.endsWith('/'+missing+'.json'))throw Error('MISSING_PERSISTED_CHILD');
  if(invalid&&file.endsWith('/'+invalid+'.json'))return Buffer.from('{"version":1,"state":""}');
  if(file===p.wallet.seedFile)return Buffer.from('11'.repeat(32));
  if(file===p.roles.secretsFile)return Buffer.from(JSON.stringify({firstSecret:'22'.repeat(32),secondSecret:'33'.repeat(32)}));
  const kind=file.split('/').at(-1).replace('.json','');assert.ok(file.includes('/.midnight-wallet-state/preview/'));return readFileSync(file);
 }}});
 let stopped=0;
 const progress={isConnected:true,isStrictlyComplete:()=>true};
 const wallet={unshielded:{state:{}},stop:async()=>{stopped++;events.push('stop');},start:async()=>events.push('start'),waitForSyncedState:async()=>({isSynced:true,shielded:{progress},unshielded:{progress,pendingCoins:[]},dust:{progress,availableCoins:[{}],state:{pendingDust:[]}}}),submitTransaction:async()=>{throw Error('NO_SUBMIT');}};
 for(const kind of ['shielded','unshielded','dust'])wallet[kind]={...wallet[kind],serializeState:async()=> 'updated-'+kind};
 const sdk={Roles:{Zswap:'z',NightExternal:'n',Dust:'d'},HDWallet:{fromSeed:()=>({type:'seedOk',hdWallet:{selectAccount:()=>({selectRoles:()=>({deriveKeysAt:()=>({type:'keysDerived',keys:{z:new Uint8Array(32),n:new Uint8Array(32),d:new Uint8Array(32)}})})}),clear(){events.push('hd-clear');}}})},createKeystore:(_k,n)=>{assert.equal(n,'preview');return {getBech32Address:()=>({asString:()=>p.wallet.expectedAddress,toString:()=>p.wallet.expectedAddress}),getAddress:()=>p.roles.firstAddress,getSecretKey:()=>new Uint8Array(32),getPublicKey:()=> 'public-key'};},NoOpTransactionHistoryStorage:class{},WalletFacade:{init:async o=>{events.push('facade-init');assert.equal(o.configuration.networkId,'preview');for(const name of ['shielded','unshielded','dust'])await o[name]({});return wallet;}}};
 for(const [key,kind] of [['ShieldedWallet','shielded'],['UnshieldedWallet','unshielded'],['DustWallet','dust']])sdk[key]=()=>({restore:async state=>{events.push('restore:'+kind);assert.equal(state,'persisted-'+kind);return {stop:async()=>{stopped++;}};}});
 const modules={'@midnight-ntwrk/wallet-sdk':sdk,'@midnight-ntwrk/midnight-js-protocol':{ZswapSecretKeys:{fromSeed:()=>({})},DustSecretKey:{fromSeed:()=>({})},signingKeyFromBip340:()=> 'controlled-key',signatureVerifyingKey:()=> 'public-key'},'@midnight-ntwrk/midnight-js-network-id':{setNetworkId:n=>{assert.equal(n,'preview');events.push('network');}},rxjs:{firstValueFrom:async()=>({state:{networkId:wrongRestore?'undeployed':'preview',publicKey:{publicKey:'public-key',addressHex:p.roles.firstAddress,address:p.wallet.expectedAddress}}})},ws:{WebSocket:class{}}};
 for(const [name,exports] of Object.entries(modules))t.mock.module(pathToFileURL(join(PINNED_NM,name,pins[name].entry)).href,{namedExports:exports});
 t.mock.module(new URL('./integrate-preview.mjs',import.meta.url).href,{namedExports:{integratePreviewFinancialCase:async o=>{events.push('integration');assert.equal(o.networkConfig.networkId,'preview');assert.equal(o.limits.dustFee,1000n);assert.equal(o.roles.firstSecret.length,32);assert.equal(typeof o.walletContext.wallet.submitTransaction,'function');await o.walletContext.wallet.stop();return {status:'PASS',networkAcceptance:false,proofAcceptance:false,financialAcceptance:false};}}});
 const subject=await import('./preview-bootstrap.mjs?fixture='+Math.random());return {p,events,subject,stops:()=>stopped};
}
test('existing Preview children restore before guarded integration and durable result',async t=>{
 const f=await fixture(t);const r=await f.subject.launchPreviewFinancialCase(f.p);assert.equal(r.status,'PASS');assert.equal(f.stops(),1);assert.equal(f.events.at(-1),'result:PASS');
 assert.ok(f.events.indexOf('genesis')<f.events.findIndex(x=>x.startsWith('private:')));assert.ok(f.events.findIndex(x=>x.endsWith('/dust.json'))<f.events.indexOf('facade-init'));assert.ok(f.events.indexOf('restore:dust')<f.events.indexOf('integration'));
});
for(const kind of ['shielded','unshielded','dust'])test('missing persisted '+kind+' stops before any SDK child restore',async t=>{
 const f=await fixture(t,{missing:kind});await assert.rejects(f.subject.launchPreviewFinancialCase(f.p));assert.ok(!f.events.includes('facade-init'));assert.ok(!f.events.includes('integration'));
});
test('reused output and wrong genesis stop before private material is read',async t=>{
 const f=await fixture(t,{reuse:true});await assert.rejects(f.subject.launchPreviewFinancialCase(f.p));assert.ok(!f.events.some(x=>x.startsWith('private')));
});
test('wrong genesis stops before output or private preparation',async t=>{
 const f=await fixture(t,{genesisWrong:true});await assert.rejects(f.subject.launchPreviewFinancialCase(f.p));assert.equal(existsSync(f.p.outputDirectory),false);assert.ok(!f.events.some(x=>x.startsWith('private')));
});
test('wrong restored network stops wallet before start or integration',async t=>{
 const f=await fixture(t,{wrongRestore:true});await assert.rejects(f.subject.launchPreviewFinancialCase(f.p));assert.equal(f.stops(),1);assert.ok(!f.events.includes('start'));assert.ok(!f.events.includes('integration'));
});

for(const kind of ['shielded','unshielded','dust'])test('invalid persisted '+kind+' does not fall back to child creation',async t=>{
 const f=await fixture(t,{invalid:kind});await assert.rejects(f.subject.launchPreviewFinancialCase(f.p));assert.ok(!f.events.includes('facade-init'));assert.ok(!f.events.includes('integration'));
});
import {writeFileSync} from 'node:fs';
test('wallet persistence preserves previous snapshots and replaces only a complete serialized set',async t=>{
 assert.equal(typeof api.persistPreviewWalletState,'function');const dir=mkdtempSync(join(tmpdir(),'moriarty-preview-persist-'));t.after(()=>rmSync(dir,{recursive:true,force:true}));
 const stateDirectory=dir,parent=join(dir,'.midnight-wallet-state','preview');mkdirSync(parent,{recursive:true,mode:0o700});const original={},wallet={};
 for(const kind of ['shielded','unshielded','dust']){original[kind]=Buffer.from(JSON.stringify({version:1,state:'original-'+kind}));writeFileSync(join(parent,kind+'.json'),original[kind],{mode:0o600});wallet[kind]={serializeState:async()=> 'updated-'+kind};}
 const r=await api.persistPreviewWalletState({wallet,stateDirectory,allocationId:'fixture-persist',original,deadlineMs:Date.now()+1000});assert.deepEqual(r,{status:'PERSISTED',children:3});
 for(const kind of ['shielded','unshielded','dust']){assert.deepEqual(readFileSync(join(parent,'.moriarty-backup-fixture-persist',kind+'.json')),original[kind]);assert.equal(JSON.parse(readFileSync(join(parent,kind+'.json'))).state,'updated-'+kind);}
});
test('failed child serialization leaves all original snapshots untouched',async t=>{
 assert.equal(typeof api.persistPreviewWalletState,'function');const dir=mkdtempSync(join(tmpdir(),'moriarty-preview-persist-fail-'));t.after(()=>rmSync(dir,{recursive:true,force:true}));
 const parent=join(dir,'.midnight-wallet-state','preview');mkdirSync(parent,{recursive:true,mode:0o700});const original={},wallet={};
 for(const kind of ['shielded','unshielded','dust']){original[kind]=Buffer.from(JSON.stringify({version:1,state:'original-'+kind}));writeFileSync(join(parent,kind+'.json'),original[kind],{mode:0o600});wallet[kind]={serializeState:async()=>{if(kind==='dust')throw Error('CONTROLLED_SERIALIZE_FAIL');return 'updated-'+kind;}};}
 await assert.rejects(api.persistPreviewWalletState({wallet,stateDirectory:dir,allocationId:'fixture-fail',original,deadlineMs:Date.now()+1000}));
 for(const kind of ['shielded','unshielded','dust'])assert.deepEqual(readFileSync(join(parent,kind+'.json')),original[kind]);
});
import fs from 'node:fs';import {syncBuiltinESMExports} from 'node:module';
test('interrupted multi-child replacement retains backups and blocks further bootstrap restoration',async t=>{
 const dir=mkdtempSync(join(tmpdir(),'moriarty-preview-interrupted-'));t.after(()=>rmSync(dir,{recursive:true,force:true}));
 const parent=join(dir,'.midnight-wallet-state','preview');mkdirSync(parent,{recursive:true,mode:0o700});const original={},wallet={};
 for(const kind of ['shielded','unshielded','dust']){original[kind]=Buffer.from(JSON.stringify({version:1,state:'original-'+kind}));writeFileSync(join(parent,kind+'.json'),original[kind],{mode:0o600});wallet[kind]={serializeState:async()=> 'updated-'+kind};}
 const rename=fs.renameSync;let renames=0;t.mock.method(fs,'renameSync',(...args)=>{if(++renames===2)throw Error('CONTROLLED_RENAME_FAILURE');return rename(...args);});syncBuiltinESMExports();t.after(()=>{t.mock.restoreAll();syncBuiltinESMExports();});
 const options={wallet,stateDirectory:dir,allocationId:'fixture-partial',original,deadlineMs:Date.now()+1000};await assert.rejects(api.persistPreviewWalletState(options),{message:'CONTROLLED_RENAME_FAILURE'});
 assert.equal(existsSync(join(parent,'.moriarty-persistence-pending.json')),true);
 for(const kind of ['shielded','unshielded','dust'])assert.deepEqual(readFileSync(join(parent,'.moriarty-backup-fixture-partial',kind+'.json')),original[kind]);
 assert.equal(JSON.parse(readFileSync(join(parent,'shielded.json'))).state,'updated-shielded');assert.deepEqual(readFileSync(join(parent,'dust.json')),original.dust);
 await assert.rejects(api.persistPreviewWalletState({...options,allocationId:'fixture-next'}),{message:'PREVIEW_PERSIST_PENDING'});
});
test('a pending persistence marker stops launch before any private material or SDK restore',async t=>{
 const f=await fixture(t);writeFileSync(join(f.p.wallet.stateDirectory,'.midnight-wallet-state/preview/.moriarty-persistence-pending.json'),'{}',{mode:0o600});
 await assert.rejects(f.subject.launchPreviewFinancialCase(f.p));assert.ok(!f.events.some(x=>x.startsWith('private')));assert.ok(!f.events.includes('facade-init'));
});
