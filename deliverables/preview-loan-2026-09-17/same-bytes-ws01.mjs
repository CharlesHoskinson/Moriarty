// One admitted supported-transport attempt. Never construct or rebalance a transaction.
import fs from 'node:fs';
import {createHash} from 'node:crypto';
import {createRequire} from 'node:module';
const root='/home/charl/Moriarty';
const dir=root+'/deliverables/preview-loan-2026-09-17';
const req=createRequire(root+'/experiments/moriarty-midnight-network/hello-world/package.json');
const ledger=req('@midnight-ntwrk/midnight-js-protocol/ledger');
const {ApiPromise,WsProvider}=req('@polkadot/api');
const expectedHash='ecd8468a01cf85896b2bc53f08c6621029659d31f127b6dc50c7f9eb71b9d400';
const expectedIds=['0001ec58451698a6d3816ef48038be1c5aae635f8e148eb2eed75a2e2f632f1f4c','0074727c89072a920c3fe28bc0be689b0da73a7027fbf0ae734b6a62f743bb293a'];
const plan=JSON.parse(fs.readFileSync(dir+'/launch-plan01.json'));
const deadline=Math.min(Date.now()+55000,plan.limits.deadlineMs-1000);
const check=(v,m)=>{if(!v)throw Error(m)};
const before=()=>check(Date.now()<deadline,'DIAGNOSTIC_DEADLINE');
const emit=x=>console.log(JSON.stringify({at:new Date().toISOString(),...x}));
process.umask(0o077);
const timer=setTimeout(()=>process.exit(124),Math.max(1,deadline-Date.now()));
let api;
try {
 before();
 const raw=fs.readFileSync('/home/charl/.local/state/moriarty/preview-loan-20260917-01/run/public-transactions/'+expectedHash+'.bin');
 check(createHash('sha256').update(raw).digest('hex')===expectedHash,'RETAINED_HASH');
 const tx=ledger.Transaction.deserialize('signature','proof','binding',raw);
 check(tx.transactionHash()===expectedHash&&Buffer.from(tx.serialize()).equals(raw)&&JSON.stringify(tx.identifiers())===JSON.stringify(expectedIds),'RETAINED_IDENTITY');
 const ttl=Math.min(...[...tx.intents.values()].map(i=>i.ttl.getTime()));check(ttl>Date.now()+60000,'RETAINED_EXPIRY');
 for(const id of expectedIds){
  const r=await fetch('https://indexer.preview.midnight.network/api/v4/graphql',{method:'POST',redirect:'error',headers:{'content-type':'application/json'},body:JSON.stringify({query:'query($id:HexEncoded!){transactions(offset:{identifier:$id}){hash block{height hash}}}',variables:{id}}),signal:AbortSignal.timeout(10000)});
  check(r.ok,'INDEXER_HTTP');const body=await r.json();check(!body.errors&&Array.isArray(body.data?.transactions),'INDEXER_RESPONSE');
  if(body.data.transactions.length){emit({event:'already-indexed-no-send',id,transactions:body.data.transactions});process.exitCode=0;throw Error('ALREADY_INDEXED');}
 }
 before();
 const provider=new WsProvider('wss://rpc.preview.midnight.network/',false);api=new ApiPromise({provider,noInitWarn:true});
 const connection=provider.connect();
 await Promise.race([Promise.all([connection,api.isReadyOrError]),new Promise((_,reject)=>{const t=setTimeout(()=>reject(Error('WS_CONNECT_TIMEOUT')),15000);t.unref();})]);
 check((await api.rpc.system.chain()).toString()==='Midnight Preview','CHAIN');
 check((await api.rpc.chain.getBlockHash(0)).toHex()==='0x'+plan.networkTag,'GENESIS');before();
 fs.writeFileSync(dir+'/same-bytes-ws-attempt01.json',JSON.stringify({at:new Date().toISOString(),transactionHash:expectedHash,identifiers:expectedIds,bytes:raw.length,deadlineMs:deadline,originalDeadlineMs:plan.limits.deadlineMs,transportAttempts:1,retainedFeeReservation:'300000000000001',additionalFeeReservation:'300000000000001',totalReservedFee:'600000000000002',newTransactions:0})+'\n',{flag:'wx',mode:0o600});
 before();
 const extrinsicHash=await api.tx.midnight.sendMnTransaction('0x'+raw.toString('hex')).send();
 emit({event:'submitted',transport:'websocket',extrinsicHash:extrinsicHash.toHex(),transactionHash:expectedHash,identifiers:expectedIds,finality:'unknown'});
} catch(error){
 if(error.message!=='ALREADY_INDEXED'){emit({event:'failure',name:error.name,message:String(error.message).replace(/(?:0x)?[a-fA-F0-9]{64,}/g,'[hex redacted]').slice(0,1000),finality:'unknown'});process.exitCode=1;}
} finally {if(api)await api.disconnect();clearTimeout(timer);}
process.exit(process.exitCode??0);
