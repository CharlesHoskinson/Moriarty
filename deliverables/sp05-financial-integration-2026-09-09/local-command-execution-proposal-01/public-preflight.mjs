/** Operational public-only child. Syntax-check only; do not import/run for tests.
 * The executor runs this under its fixed diagnostic cgroup and setup deadline.
 */
import {readFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
const check=(ok,code)=>{if(!ok)throw Error(code);};
try {
 const raw=readFileSync(0,'utf8');check(Buffer.byteLength(raw)<=32768,'PUBLIC_INPUT_BOUND');
 const input=JSON.parse(raw);check(Object.keys(input).sort().join(',')==='plan,setupDeadlineMs','PUBLIC_INPUT');
 const {plan,setupDeadlineMs}=input;check(Number.isSafeInteger(setupDeadlineMs)&&Date.now()<setupDeadlineMs,'PUBLIC_SETUP_DEADLINE');
 const stop=()=>{check(Date.now()<setupDeadlineMs,'PUBLIC_SETUP_DEADLINE');};
 const probe=fileURLToPath(new URL('../local-execution-04/committee-probe.mjs',import.meta.url));
 let committee;
 for(let count=0;count<60;count++){
  stop();const r=spawnSync(process.execPath,[probe,plan.networkConfig.node,plan.networkConfig.indexer],{encoding:'utf8',timeout:Math.min(16000,setupDeadlineMs-Date.now()),maxBuffer:65536});stop();
  check(!r.error,'PUBLIC_COMMITTEE_CHILD');
  try{committee=JSON.parse(r.stdout.trim().split('\n').at(-1));}catch{throw Error('PUBLIC_COMMITTEE_OUTPUT');}
  if(r.status===0&&committee.status==='READY'&&committee.indexer.hash===committee.hash.slice(2)&&committee.indexer.height===committee.height)break;
  check(count<59,'PUBLIC_COMMITTEE_NOT_READY');await new Promise(resolve=>setTimeout(resolve,Math.min(1000,Math.max(1,setupDeadlineMs-Date.now()))));
 }
 stop();const {validateLocalLaunchPlan}=await import('../../../experiments/moriarty-midnight-financial/ledger/launch-local.mjs');
 const {inspectFinancialBuild}=await import('../../../experiments/moriarty-midnight-financial/ledger/proven-assets.mjs');
 const {inspectLocalLaunchRuntime}=await import('../../../experiments/moriarty-midnight-financial/ledger/launch-local.mjs');
 const p=validateLocalLaunchPlan(plan);stop();const b=await inspectFinancialBuild({case:p.kind,...p.build});stop();await inspectLocalLaunchRuntime();stop();
 check(b.status==='assets-inspected'&&b.case===p.kind,'PUBLIC_BUILD');
 console.log(JSON.stringify({status:'FRESH_LOCAL_PUBLIC_VERIFIED',kind:p.kind,receiptSha256:b.receiptSha256,sourceManifestHash:b.sourceManifestHash,networkTag:p.networkTag,committee:{hash:committee.hash,height:committee.height}}));
 process.exit(0);
} catch(error){
 console.log(JSON.stringify({status:'PUBLIC_PREFLIGHT_FAILED',code:typeof error?.message==='string'&&/^[A-Z][A-Z0-9_]{0,127}$/.test(error.message)?error.message:'PUBLIC_PREFLIGHT_UNKNOWN'}));
 process.exit(1);
}
