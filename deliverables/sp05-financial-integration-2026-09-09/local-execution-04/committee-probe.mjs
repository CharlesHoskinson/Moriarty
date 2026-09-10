/** Draft one-shot public readiness probe. Does not import wallets or submit.
 * argv: HTTP loopback node URL, HTTP loopback indexer GraphQL URL.
 * Runtime/storage observations use api.at(one finalized hash).
 * Source semantics and caveats: /tmp/moriarty-committee-probe-sources.md
 */
import {createRequire} from 'node:module';
const require=createRequire('/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/package.json');
const {ApiPromise,HttpProvider}=require('@polkadot/api');
const check=(ok,code)=>{if(!ok)throw Error(code);};
const endpoint=x=>{const u=new URL(x);check(u.protocol==='http:'&&['127.0.0.1','[::1]'].includes(u.hostname)&&!u.username&&!u.password&&!u.hash,'LOCAL_ENDPOINT');return u.href;};
const scalar=(x,name)=>{check(typeof x?.toBigInt==='function',name+'_CODEC');const n=x.toBigInt();check(n>=0n&&n<=0xffffffffffffffffn,name+'_RANGE');return n;};
const member=(x,name)=>{check(typeof x?.get==='function'&&x.has(name),'MISSING_'+name);return x.get(name);};
const safe=(x,name)=>{check(x<=BigInt(Number.MAX_SAFE_INTEGER),name+'_SAFE');return Number(x);};
const timestamp=x=>{check(typeof x==='number'||typeof x==='string','INDEXER_TIMESTAMP_TYPE');const n=new Date(x).getTime();check(Number.isSafeInteger(n)&&n>0,'INDEXER_TIMESTAMP');return n;};
let api;
// Whole-process hard stop also covers hung SDK init/disconnect; no reconnect loop.
const deadline=setTimeout(()=>{console.log(JSON.stringify({status:'NOT_READY',error:'PROBE_DEADLINE'}));process.exit(2);},15000);
try {
 check(process.argv.length===4,'ARGUMENTS');
 const node=endpoint(process.argv[2]),indexer=endpoint(process.argv[3]);
 api=await ApiPromise.create({provider:new HttpProvider(node),noInitWarn:true});
 const hash=(await api.rpc.chain.getFinalizedHead()).toHex();
 check(/^0x[0-9a-f]{64}$/.test(hash),'FINALIZED_HASH');
 const at=await api.at(hash);
 check(typeof at.call.sessionValidatorManagementApi?.getCurrentCommittee==='function'&&typeof at.call.sessionValidatorManagementApi?.getNextCommittee==='function'&&typeof at.call.getSidechainStatus?.getSidechainStatus==='function'&&typeof at.call.slotApi?.slotConfig==='function','RUNTIME_APIS');
 check(typeof at.query.sidechain?.epochNumber==='function'&&typeof at.query.timestamp?.now==='function'&&typeof at.query.system?.events==='function'&&typeof at.events.session?.NewSession?.is==='function','STORAGE_EVENTS');
 const [header,committee,next,stored,status,config,time,events]=await Promise.all([
  api.rpc.chain.getHeader(hash),at.call.sessionValidatorManagementApi.getCurrentCommittee(),
  at.call.sessionValidatorManagementApi.getNextCommittee(),at.query.sidechain.epochNumber(),
  at.call.getSidechainStatus.getSidechainStatus(),at.call.slotApi.slotConfig(),
  at.query.timestamp.now(),at.query.system.events()
 ]);
 check(committee.length===2&&committee[1].length>0,'CURRENT_COMMITTEE');
 check(next.isSome===true,'NEXT_COMMITTEE');const nextPair=next.unwrap();
 check(nextPair.length===2&&nextPair[1].length>0,'NEXT_COMMITTEE_SHAPE');
 const committeeEpoch=scalar(committee[0],'COMMITTEE'),nextCommitteeEpoch=scalar(nextPair[0],'NEXT_COMMITTEE');
 const storedEpoch=scalar(stored,'STORED_EPOCH'),epoch=scalar(member(status,'epoch'),'EPOCH');
 const slot=scalar(member(status,'slot'),'SLOT'),slotsPerEpoch=scalar(member(status,'slotsPerEpoch'),'SLOTS_PER_EPOCH');
 const configuredSlots=scalar(member(config,'slotsPerEpoch'),'CONFIG_SLOTS');
 const slotDurationMs=scalar(member(config,'slotDuration'),'SLOT_DURATION');
 const timestampMs=safe(scalar(time,'TIMESTAMP'),'TIMESTAMP'),height=safe(scalar(header.number,'HEIGHT'),'HEIGHT');
 check(epoch>0n&&storedEpoch>0n&&slotsPerEpoch>0n&&slotDurationMs>0n&&timestampMs>0,'NONZERO_STATE');
 check(slotsPerEpoch===configuredSlots&&slot/slotsPerEpoch===epoch&&storedEpoch===epoch,'EPOCH_COHERENCE');
 check(BigInt(timestampMs)/slotDurationMs===slot,'SLOT_TIMESTAMP_COHERENCE');
 const rotations=[...events].filter(({event})=>at.events.session.NewSession.is(event));
 check(rotations.every(x=>x.phase.isInitialization===true),'SESSION_EVENT_PHASE');
 const sessionNewSession=rotations.length>0;
 const response=await fetch(indexer,{method:'POST',redirect:'error',headers:{'content-type':'application/json'},signal:AbortSignal.timeout(5000),body:JSON.stringify({query:'query($offset: BlockOffset) { latest: block { height hash timestamp } pinned: block(offset: $offset) { height hash timestamp } }',variables:{offset:{hash:hash.slice(2)}}})});
 check(response.ok&&response.body,'INDEXER_HTTP');
 const chunks=[];let size=0;for await(const chunk of response.body){size+=chunk.length;check(size<=65536,'INDEXER_BODY_LIMIT');chunks.push(chunk);}
 const body=JSON.parse(Buffer.concat(chunks).toString('utf8'));check(!body.errors&&body.data?.latest&&body.data?.pinned,'INDEXER_DATA');
 const latest=body.data.latest,pinned=body.data.pinned;
 check(pinned.hash===hash.slice(2)&&pinned.height===height&&timestamp(pinned.timestamp)===timestampMs,'INDEXER_PINNED_MISMATCH');
 check(Number.isSafeInteger(latest.height)&&latest.height>=0&&typeof latest.hash==='string'&&/^[0-9a-f]{64}$/.test(latest.hash),'INDEXER_LATEST');
 check(Math.abs(latest.height-height)<=2,'INDEXER_LAG');
 check((await api.rpc.chain.getBlockHash(latest.height)).toHex()==='0x'+latest.hash,'INDEXER_CANONICAL');
 const now=Date.now(),latestTimestampMs=timestamp(latest.timestamp);
 const wallEpoch=BigInt(now)/slotDurationMs/slotsPerEpoch;
 const checks={fresh:timestampMs<=now&&now-timestampMs<=60000&&latestTimestampMs<=now&&now-latestTimestampMs<=60000,
  committeeCaughtUp:committeeEpoch===epoch&&epoch===wallEpoch,
  nextCommitteeReady:nextCommitteeEpoch===committeeEpoch+1n,
  nonRotation:!sessionNewSession};
 const ready=Object.values(checks).every(Boolean);
 console.log(JSON.stringify({status:ready?'READY':'NOT_READY',observedAt:new Date(now).toISOString(),hash,height,timestampMs,
  committeeEpoch:committeeEpoch.toString(),nextCommitteeEpoch:nextCommitteeEpoch.toString(),storedEpoch:storedEpoch.toString(),
  sidechainEpoch:epoch.toString(),sidechainSlot:slot.toString(),slotsPerEpoch:slotsPerEpoch.toString(),slotDurationMs:slotDurationMs.toString(),wallEpoch:wallEpoch.toString(),sessionNewSession,
  sessionIndices:rotations.map(x=>scalar(x.event.data[0],'SESSION_INDEX').toString()),
  indexer:{height:latest.height,hash:latest.hash,timestampMs:latestTimestampMs,pinnedHashMatched:true},checks,
  scope:'Single public observation; no financial admission, old transaction reconciliation, or future block guarantee'}));
 process.exitCode=ready?0:2;
} catch(error) {
 console.log(JSON.stringify({status:'NOT_READY',error:typeof error?.message==='string'&&/^[A-Z_]+$/.test(error.message)?error.message:'PROBE_ERROR'}));
 process.exitCode=2;
} finally {
 if(api)await api.disconnect();
 clearTimeout(deadline);
}
