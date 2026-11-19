// Each invocation represents a separate owner client. Its only persistent input
// is the accepted ledger snapshot supplied by the local ledger service.
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createRequire} from 'node:module';
import {readContinuation} from '../src/continuation.mjs';
import {signOrder} from '../src/authorization.mjs';

const {head,modules,directory,deployment,request,enrollment} = JSON.parse(readFileSync(0,'utf8'));
const require = createRequire(join(modules,'../package.json'));
const L = await import(pathToFileURL(require.resolve('@midnight-ntwrk/midnight-js-protocol/v9')));
const runtime = await import(pathToFileURL(require.resolve('@midnight-ntwrk/compact-runtime')));
const compiled = await import(pathToFileURL(join(directory,'contract/index.js')));
const state = L.LedgerState.deserialize(Buffer.from(head,'hex'));
const order = readContinuation({network:deployment.network,index:a=>state.index(a)},runtime,compiled,deployment);
const message = Buffer.from(JSON.stringify(['O9-wallet-enrollment/1',deployment,
  runtime.decodeUserAddress(order.terms.account.bytes),String(order.ownerKey.x),String(order.ownerKey.y)]));
if (L.addressFromKey(enrollment.key) !== runtime.decodeUserAddress(order.terms.account.bytes)
    || !L.verifySignature(enrollment.key,message,enrollment.signature)) throw Error('Invalid staged wallet enrollment');
const reply = {pid:process.pid,stage:Number(order.current.f2),terminal:Number(order.terminal),remaining:Number(order.remaining),
  revision:Number(order.revision),eventEnabled:order.eventEnabled};
if (request !== null && (order.current.f2 === 1n || (order.current.f2 === 0n && request === 0))) {
  const signature = signOrder(runtime,compiled.pureCircuits,{
    terms:order.terms,contract:{bytes:runtime.encodeContractAddress(deployment.address)},
    owner:order.ownerKey,action:BigInt(request),revision:order.revision,
  },123456789n); // Published fixture scalar for the isolated network.
  reply.signature = {announcement:{x:String(signature.announcement.x),y:String(signature.announcement.y)},response:String(signature.response)};
}
console.log(JSON.stringify(reply));
