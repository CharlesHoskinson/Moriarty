// Each invocation is a separate owner client process. Its persistent inputs are the
// accepted ledger snapshot supplied by the local ledger service and its own journal file.
//   read:     verify the enrollment, report the accepted stage and sign an eligible grant.
//   journal:  sign and prepare a terminal action, persist the unsubmitted transaction, exit.
//   resubmit: a restarted process reloads its journal from disk and submits it unchanged.
import {createHash} from 'node:crypto';
import {readFileSync, writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createRequire} from 'node:module';
import {readContinuation, prepareTerminal, verifyStagedEnrollment} from '../src/continuation.mjs';
import {signOrder} from '../src/authorization.mjs';
import {NativeLedger} from './native-ledger.mjs';

const sha = bytes => createHash('sha256').update(bytes).digest('hex');
const input = JSON.parse(readFileSync(0,'utf8'));
if (input.mode === 'resubmit') {
  // The restarted client deliberately trusts its stale journal; only the ledger can refuse it.
  const journal = JSON.parse(readFileSync(input.journal,'utf8'));
  console.log(JSON.stringify({pid:process.pid,mode:'resubmit',action:journal.action,
    preparedAgainstSha256:journal.preparedAgainstSha256,journalSha256:sha(readFileSync(input.journal)),submit:journal.transaction}));
  process.exit(0);
}
const {head,modules,directory,deployment,request,enrollment} = input;
const require = createRequire(join(modules,'../package.json'));
const L = await import(pathToFileURL(require.resolve('@midnight-ntwrk/midnight-js-protocol/v9')));
const runtime = await import(pathToFileURL(require.resolve('@midnight-ntwrk/compact-runtime')));
const compiled = await import(pathToFileURL(join(directory,'contract/index.js')));
const bytes = Buffer.from(head,'hex');
const state = L.LedgerState.deserialize(bytes);
const order = readContinuation({network:deployment.network,index:a=>state.index(a)},runtime,compiled,deployment);
verifyStagedEnrollment(L,runtime,deployment,order,enrollment);
const reply = {pid:process.pid,mode:input.mode ?? 'read',stage:Number(order.current.f2),terminal:Number(order.terminal),
  remaining:Number(order.remaining),revision:Number(order.revision),eventEnabled:order.eventEnabled};
const eligible = request !== null && order.terminal === 0n &&
  ((order.current.f2 === 1n && [1,2,3].includes(request)) || (order.current.f2 === 0n && [0,4].includes(request)));
if (eligible) {
  const signature = signOrder(runtime,compiled.pureCircuits,{
    terms:order.terms,contract:{bytes:runtime.encodeContractAddress(deployment.address)},
    owner:order.ownerKey,action:BigInt(request),revision:order.revision,
  },123456789n); // Published fixture scalar for the isolated network.
  reply.signature = {announcement:{x:String(signature.announcement.x),y:String(signature.announcement.y)},response:String(signature.response)};
  if (input.mode === 'journal') {
    const ledger = Object.assign(Object.create(NativeLedger.prototype),{L,runtime,state,time:input.time,network:deployment.network});
    const instance = {address:deployment.address,contract:{generated:compiled,instance:new compiled.Contract({})}};
    const action = {2:'resume',3:'refund'}[request];
    const prepared = await prepareTerminal(ledger,instance,deployment,{action,capture:order.pending,
      delivery:BigInt(input.delivery ?? 0),now:BigInt(input.now ?? 0),signature});
    const transaction = Buffer.from(ledger.assemble([prepared]).serialize()).toString('hex');
    writeFileSync(input.journal,JSON.stringify({action,preparedAgainstSha256:sha(bytes),preparedAt:input.time,transaction})+'\n');
    reply.journalSha256 = sha(readFileSync(input.journal));
  }
}
console.log(JSON.stringify(reply));
