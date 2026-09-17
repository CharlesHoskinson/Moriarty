// Read-only setup check: never creates a wallet or submits a transaction.
import {readFileSync, existsSync, lstatSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {join} from 'node:path';

const root=fileURLToPath(new URL('../../', import.meta.url));
const scaffold=join(root,'experiments/moriarty-midnight-network/hello-world');
const evidence=process.env.MIDNIGHT_PREVIEW_EVIDENCE_DIRECTORY??join(root,'evidence/midnight-preview-2026-09-07');
const receipt=JSON.parse(readFileSync(join(evidence,'wallet-public.json')));
const pins=JSON.parse(readFileSync(join(root,'experiments/moriarty-midnight-financial/ledger/launch-runtime-pins.json')));
const checks=[];
const check=(name,ready,detail)=>checks.push({name,ready,detail});
let files=0, failures=[];
for(const [name,pin] of Object.entries(pins)) for(const [entry,digest] of Object.entries(pin.files)) {
  const path=join(scaffold,'node_modules',name,entry);
  try {if(createHash('sha256').update(readFileSync(path)).digest('hex')!==digest)failures.push(name+'/'+entry);files++;}
  catch {failures.push(name+'/'+entry+': missing');}
}
check('installed-financial-runtime-pins',failures.length===0,{files,failures});
check('compiled-hello-world',existsSync(join(scaffold,'contracts/managed/hello-world/contract/index.js')),'Compiled assets in current checkout');
const seedPath=process.env.MIDNIGHT_WALLET_SEED_FILE??receipt.seedFile;
let seedReady=false;
try {const s=lstatSync(seedPath);seedReady=s.isFile()&&!s.isSymbolicLink()&&s.uid===process.getuid()&&(s.mode&0o077)===0;}catch {}
check('existing-wallet-seed',seedReady,{path:seedPath,expectedAddress:receipt.address,identityVerified:false});
const stateDir=process.env.MIDNIGHT_PREVIEW_STATE_DIR??receipt.stateDirectory??'/home/charl/.local/share/moriarty/test-wallets/preview-runtime-20260907';
check('existing-wallet-snapshots',['shielded','unshielded','dust'].every(k=>existsSync(join(stateDir,'.midnight-wallet-state/preview',k+'.json'))),{path:stateDir,syncVerified:false});
try {
  const {inspectLocalLaunchRuntime}=await import('../moriarty-midnight-financial/ledger/launch-local.mjs');
  check('configured-financial-runtime',true,await inspectLocalLaunchRuntime());
} catch(error) { check('configured-financial-runtime',false,{error:error.message}); }
await Promise.all([
  ['preview-chain',receipt.configuration.node,{jsonrpc:'2.0',id:1,method:'system_chain',params:[]},body=>body.result==='Midnight Preview'],
  ['preview-indexer',receipt.configuration.indexer,{query:'{block {height hash timestamp}}'},body=>Number.isInteger(body.data?.block?.height)],
  ['local-proof-server',receipt.configuration.proofServer+'/health',null,body=>body.status==='ok'],
].map(async([name,url,body,valid])=>{
  try {const r=await fetch(url,{method:body?'POST':'GET',headers:body?{'content-type':'application/json'}:undefined,body:body?JSON.stringify(body):undefined,signal:AbortSignal.timeout(15000)});const data=await r.json();check(name,r.ok&&valid(data),{httpStatus:r.status,response:data});}
  catch(error){check(name,false,{error:error.message});}
}));
const ready=checks.every(c=>c.ready);
console.log(JSON.stringify({at:new Date().toISOString(),installationReady:ready,transactionReadiness:'not verified by installation checks',checks,walletStarted:false,transactionsSubmitted:0,limitation:'Seed existence is not identity, sync, funding, DUST or transaction-readiness verification.'},null,2));
process.exitCode=ready?0:1;
