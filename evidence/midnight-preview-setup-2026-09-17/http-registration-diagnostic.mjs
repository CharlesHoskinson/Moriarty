import fs from 'node:fs/promises';
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
import crypto from 'node:crypto';
const root='/home/charl/Moriarty';
const req=createRequire(root+'/experiments/moriarty-midnight-network/hello-world/package.json');
const imp=async n=>import(pathToFileURL(req.resolve(n)).href);
globalThis.WebSocket=req('ws').WebSocket;
const ledger=req('@midnight-ntwrk/midnight-js-protocol/ledger');
const {makeDefaultSubmissionServiceEffect}=await import(pathToFileURL(root+'/experiments/moriarty-midnight-network/hello-world/node_modules/@midnight-ntwrk/wallet-sdk-capabilities/dist/submission/index.js').href);
const {Effect,Exit}=await imp('effect');
const file='/home/charl/.local/share/moriarty/test-wallets/preview-runtime-20260917/dust-registration-2026-09-17T16-03-48-451Z.tx';
const raw=new Uint8Array(await fs.readFile(file));
const tx=ledger.Transaction.deserialize('signature','proof','binding',raw);
const hash=tx.transactionHash();
if(hash!=='680156b4b58058f4160d0b8da187e6f17880d04c1e61c90281a690026607c089')throw Error('Retained transaction mismatch');
const id='0074fcd5910e16809368733e60c97aa6e49724747c8231369bcd30f2d754487b69';
if(!Buffer.from(tx.serialize()).equals(Buffer.from(raw))||!tx.identifiers().includes(id))throw Error('Serialized bytes or identifier mismatch');
const endpoint='https://indexer.preview.midnight.network/api/v4/graphql';
const prior=await fetch(endpoint,{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify({query:'query($id: HexEncoded!){transactions(offset:{identifier:$id}){hash block{height hash}}}',variables:{id}}),signal:AbortSignal.timeout(15000)}).then(r=>r.json());
console.log(JSON.stringify({event:'indexer-preflight',response:prior}));
if(prior.errors)throw Error('Indexer preflight failed');
if(prior.data.transactions.length){console.log(JSON.stringify({event:'already-indexed-no-rebroadcast'}));process.exit(0);}
const chain=await fetch('https://rpc.preview.midnight.network',{method:'POST',headers:{'content-type':'application/json'},body:JSON.stringify({jsonrpc:'2.0',id:1,method:'system_chain',params:[]}),signal:AbortSignal.timeout(15000)}).then(r=>r.json());
if(chain.result!=='Midnight Preview')throw Error('Wrong chain');
console.log(JSON.stringify({event:'retained-transaction-verified',hash,identifiers:tx.identifiers(),bytes:raw.length,sha256:crypto.createHash('sha256').update(raw).digest('hex')}));
if(!process.argv.includes('--submit'))process.exit(0);
const marker=root+'/evidence/midnight-preview-setup-2026-09-17/http-registration-attempt.json';
await fs.writeFile(marker,JSON.stringify({started:new Date().toISOString(),hash,scope:'same registration bytes over HTTP; no new recipe'}),{flag:'wx',mode:0o600});
const {ApiPromise,HttpProvider}=req('@polkadot/api');
const api=await ApiPromise.create({provider:new HttpProvider('https://rpc.preview.midnight.network'),noInitWarn:true});
try {
 const extrinsic=api.tx.midnight.sendMnTransaction('0x'+Buffer.from(raw).toString('hex'));
 const submitted=await extrinsic.send();
 console.log(JSON.stringify({event:'http-submitted',extrinsicHash:submitted.toHex(),identifiers:tx.identifiers(),transactionHash:hash}));
} catch(error) {
 console.log(JSON.stringify({event:'http-rejected',name:error.name,message:String(error.message).replace(/(?:0x)?[a-fA-F0-9]{128,}/g,'[redacted-long-hex]').slice(0,2000),code:error.code}));process.exitCode=1;
} finally {await api.disconnect();}
