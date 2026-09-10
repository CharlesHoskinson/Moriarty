/** Actual SDK public-provider method, controlled cross-fetch transport only.
 * No HTTP server, WebSocket connection, wallet or private-store access.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import https from 'node:https';
import http from 'node:http';
import {Writable,PassThrough} from 'node:stream';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
import {extractNativeContractBalances} from './contract-balances.mjs';

const providerPath=join(PINNED_NM,'@midnight-ntwrk/midnight-js-indexer-public-data-provider/dist/index.mjs');
const compact=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs')).href);
const transactions=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
const address='8824d69c9058f322b4f6da7e7cd8d49f3235db5fbe3d6080d25f239243812261';
const blockHash='f268da29bb3e6e81f8a97bae0eb53938f3a146fce677deb5c7ae18b8c5335386';
const stateBytes=readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/swap-initialize-diagnosis-01/indexed-initialize-state.bin',import.meta.url));

test('actual SDK queryContractState producer passes native balance extraction with its compact-runtime class',async t=>{
 assert.equal(stateBytes.length,8150);
 assert.equal(createHash('sha256').update(stateBytes).digest('hex'),'5a1dcdb726f25a724f89d0185af0fb8bb863b37e094fce8fc7dc1e75caf2b92c');
 // Intercept below the installed cross-fetch ponyfill, before any socket or
 // DNS creation. The SDK, GraphQL query, Apollo decoding and WASM stay real.
 const requests=[];
 t.mock.method(http,'request',()=>{throw Error('HTTP_FORBIDDEN');});
 t.mock.method(https,'request',(options)=>{
  assert.equal(options.hostname,'indexer-fixture.invalid');
  assert.equal(options.path,'/graphql');assert.equal(options.method,'POST');
  const chunks=[];
  const request=new Writable({write(chunk,_encoding,done){chunks.push(Buffer.from(chunk));done();},final(done){
   const body=JSON.parse(Buffer.concat(chunks).toString('utf8'));requests.push(body);
   assert.equal(body.operationName,'CONTRACT_STATE_QUERY');
   assert.deepEqual(body.variables,{address,offset:{blockOffset:{hash:blockHash}}});
   assert.match(body.query,/contractAction\(address: \$address, offset: \$offset\)/);
   const response=new PassThrough();response.statusCode=200;response.statusMessage='OK';response.headers={'content-type':'application/json'};
   queueMicrotask(()=>{request.emit('response',response);response.end(JSON.stringify({data:{contractAction:{__typename:'ContractCall',state:stateBytes.toString('hex')}}}));});done();
  }});
  request.abort=()=>request.destroy();return request;
 });
 class NoWebSocket {static CONNECTING=0;static OPEN=1;static CLOSING=2;static CLOSED=3;constructor(){throw Error('WEBSOCKET_FORBIDDEN');}}
 try{
  const {indexerPublicDataProvider}=await import(pathToFileURL(providerPath).href);
  const provider=indexerPublicDataProvider('https://indexer-fixture.invalid/graphql','wss://indexer-fixture.invalid/graphql',NoWebSocket);
  const state=await provider.queryContractState(address,{type:'blockHash',blockHash});
  assert.equal(requests.length,1);
  assert.ok(state instanceof compact.ContractState);
  assert.equal(state instanceof transactions.ContractState,false);
  assert.deepEqual(Buffer.from(state.serialize()),stateBytes);
  assert.deepEqual(extractNativeContractBalances({state}),{
   '5475695f8ccb85c05055a4ffef06bbbff208c58b07cf5c61052d25143729d166':'1000000',
   'e7d969726b0884ada7bb477283143911dc4df1b18da25a776a433bfaac8646d4':'2000000',
  });
  assert.deepEqual(Buffer.from(state.serialize()),stateBytes);
 }finally{t.mock.restoreAll();}
});
