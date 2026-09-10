import test from 'node:test';
import assert from 'node:assert/strict';
import {createRequire} from 'node:module';
import {pathToFileURL} from 'node:url';
import {join} from 'node:path';
import {PINNED_NM} from './providers.mjs';
const api=await import('./swap-wallet.mjs').catch(()=>({}));
const require=createRequire(join(PINNED_NM,'../package.json')),{Schema}=require('effect');
const {WalletSyncUpdateSchema}=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/wallet-sdk-unshielded-wallet/dist/v1/SyncSchema.js')).href);
const {MidnightBech32m,UnshieldedAddress}=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/wallet-sdk-address-format/dist/index.js')).href);
const owner='9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb';
const address=(hex,network='undeployed')=>MidnightBech32m.encode(network,new UnshieldedAddress(Buffer.from(hex,'hex'))).toString();
// Synthetic transaction/address/color metadata: no live swap has been observed.
// Exact fixed swap mint quantity, actual installed GraphQL-to-wallet decoder.
function fixture(){
 const roles={firstAddress:owner,secondAddress:'22'.repeat(32)},assetBindings={ASSET_A:'33'.repeat(32),ASSET_B:'44'.repeat(32)},origin='55'.repeat(32),txId='00'+'66'.repeat(32),hash='77'.repeat(32),contract='88'.repeat(32);
 const output={segment:21861,section:'guaranteed',owner,type:assetBindings.ASSET_A,value:'100000',intentHash:origin,offerIndex:0};
 const receipt={schema:'moriarty.finalized-financial-stage/1',circuitId:'initialize',contractAddress:contract,txId,blockHash:'99'.repeat(32),blockHeight:100,finalizedHead:'0x'+'aa'.repeat(32),finalizedHeight:101,protocolVersion:1000000,acceptance:'uncertified-I2-observation',transaction:{transactionHash:hash,rawSha256:hash,identifiers:[txId],proofVerified:false,ledgerAccepted:false,inputs:[],outputs:[output],actions:[{kind:'call',address:contract,entryPoint:'initialize',segment:21861}]}};
 const wire={type:'UnshieldedTransaction',transaction:{id:1,hash,type:'RegularTransaction',protocolVersion:1000000,block:{timestamp:1789027704001},transactionResult:{status:'SUCCESS',segments:null}},createdUtxos:[{owner:address(owner),tokenType:assetBindings.ASSET_A,value:'100000',intentHash:origin,outputIndex:0,ctime:1,registeredForDustGeneration:false}],spentUtxos:[]};
 const coin=Schema.decodeUnknownSync(WalletSyncUpdateSchema)(wire).createdUtxos[0];
 const progress=()=>({isConnected:true,isStrictlyComplete:()=>true});const synced={shielded:{progress:progress()},unshielded:{progress:progress(),availableCoins:[coin],pendingCoins:[]},dust:{progress:progress(),state:{pendingDust:[]}}};
 return {receipt,roles,assetBindings,synced};
}
const run=f=>{assert.equal(typeof api.assertSwapInitializedWallet,'function');return api.assertSwapInitializedWallet(f);};
test('fixed swap mint matches an actual SDK decoded available coin, without authority claims',()=>{
 const f=fixture(),r=run(f);assert.equal(r.status,'SWAP_INITIALIZED_WALLET_VERIFIED');assert.equal(r.dispatchAuthorized,false);assert.equal(r.mintedOutput.owner,owner);assert.equal(r.mintedOutput.value,'100000');assert.equal(r.mintedOutput.outputNo,0);assert.ok(Object.isFrozen(r)&&Object.isFrozen(r.mintedOutput));
 assert.notEqual(f.synced.unshielded.availableCoins[0].utxo.owner,owner,'real wallet boundary is Bech32m');
});
for(const [name,change] of [
 ['missing receipt',f=>f.receipt=undefined],['wrong stage',f=>f.receipt.circuitId='swap'],['wrong tx hash',f=>f.receipt.transaction.rawSha256='00'.repeat(32)],['missing id',f=>f.receipt.transaction.identifiers=[]],['unfinalized metadata',f=>f.receipt.finalizedHeight=99],['wrong protocol',f=>f.receipt.protocolVersion=9],
 ['missing mint',f=>f.receipt.transaction.outputs=[]],['extra mint',f=>f.receipt.transaction.outputs.push({...f.receipt.transaction.outputs[0]})],['native input',f=>f.receipt.transaction.inputs=[{}]],['wrong action',f=>f.receipt.transaction.actions[0].entryPoint='swap'],['wrong contract',f=>f.receipt.transaction.actions[0].address='00'.repeat(32)],['wrong physical segment',f=>f.receipt.transaction.outputs[0].segment++],['wrong native section',f=>f.receipt.transaction.outputs[0].section='fallible'],['wrong native index',f=>f.receipt.transaction.outputs[0].offerIndex=1],['wrong native owner',f=>f.receipt.transaction.outputs[0].owner=f.roles.secondAddress],['wrong native type',f=>f.receipt.transaction.outputs[0].type=f.assetBindings.ASSET_B],['wrong native value',f=>f.receipt.transaction.outputs[0].value='10000'],['malformed native origin',f=>f.receipt.transaction.outputs[0].intentHash='bad'],
 ['same roles',f=>f.roles.secondAddress=owner],['same assets',f=>f.assetBindings.ASSET_B=f.assetBindings.ASSET_A],['disconnected shielded',f=>f.synced.shielded.progress.isConnected=false],['incomplete unshielded',f=>f.synced.unshielded.progress.isStrictlyComplete=()=>false],['missing dust progress',f=>delete f.synced.dust.progress],['pending dust',f=>f.synced.dust.state.pendingDust=[{}]],['pending coin',f=>f.synced.unshielded.pendingCoins=[f.synced.unshielded.availableCoins[0]]],['missing available coin',f=>f.synced.unshielded.availableCoins=[]],['duplicate available coin',f=>f.synced.unshielded.availableCoins.push(f.synced.unshielded.availableCoins[0])],
 ['rawhex wallet owner',f=>f.synced.unshielded.availableCoins[0].utxo.owner=owner],['wrong wallet owner',f=>f.synced.unshielded.availableCoins[0].utxo.owner=address(f.roles.secondAddress)],['wrong wallet network',f=>f.synced.unshielded.availableCoins[0].utxo.owner=address(owner,'preview')],['wrong wallet type',f=>f.synced.unshielded.availableCoins[0].utxo.type=f.assetBindings.ASSET_B],['wrong wallet amount',f=>f.synced.unshielded.availableCoins[0].utxo.value=99999n],['string wallet amount',f=>f.synced.unshielded.availableCoins[0].utxo.value='100000'],['wrong wallet origin',f=>f.synced.unshielded.availableCoins[0].utxo.intentHash='ab'.repeat(32)],['wrong wallet index',f=>f.synced.unshielded.availableCoins[0].utxo.outputNo=1],['flattened coin',f=>f.synced.unshielded.availableCoins=[f.synced.unshielded.availableCoins[0].utxo]]
])test(`swap mint gate rejects ${name}`,()=>{const f=fixture();change(f);assert.throws(()=>run(f),/SWAP_WALLET/);});
test('unrelated correctly shaped coins do not hide exact mint duplicates or fund the predicate',()=>{const f=fixture(),other={utxo:{...f.synced.unshielded.availableCoins[0].utxo,intentHash:'bb'.repeat(32)}};f.synced.unshielded.availableCoins.unshift(other);assert.equal(run(f).status,'SWAP_INITIALIZED_WALLET_VERIFIED');f.synced.unshielded.availableCoins.pop();assert.throws(()=>run(f),/SWAP_WALLET/);});
