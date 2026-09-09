import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {isDeepStrictEqual} from 'node:util';
import {PINNED_NM} from './providers.mjs';

const sha=bytes=>createHash('sha256').update(bytes).digest('hex');
const requireThat=(ok,message)=>{if(!ok)throw Error(message);};
export async function loadFinancialContractsSdk() {
  const root=join(PINNED_NM,'@midnight-ntwrk/midnight-js-contracts');
  requireThat(JSON.parse(readFileSync(join(root,'package.json'),'utf8')).version==='4.1.1','CONTRACTS_SDK_VERSION');
  const entry=join(root,'dist/index.mjs');
  requireThat(sha(readFileSync(entry))==='9c8079430513e7b459d52fc6d22ab5dede22f86073a4b50dcbdf35de99bb4072','CONTRACTS_SDK_BYTES');
  return import(pathToFileURL(entry).href);
}

/**
 * Run the real constructor once and fix its native deployment address BEFORE
 * allocating asset-specific spending limits. This operation does not prove,
 * balance, submit or create a wallet. The caller supplies an existing wallet's
 * public-key accessor and an admitted compiled contract's verifier-key provider.
 * Private constructor data stays in the single-use closure, never in public.
 * The adapter submits the exact prepared bytes through pinned SDK submitTx and
 * follows its submitDeployTx private-state lifecycle after observed success.
 * An injected SDK is for offline source tests, not a network acceptance claim.
 */
export async function prepareFinancialDeployment({deploymentOptions,publicWalletProvider,zkConfigProvider,signingKey,ledger,sdk}) {
  const options=deploymentOptions;
  requireThat(options && Object.keys(options).sort().join(',')==='args,compiledContract,initialPrivateState,privateStateId','DEPLOYMENT_OPTIONS_SHAPE');
  requireThat(['sp05-loan','sp05-swap'].includes(options.privateStateId) && isDeepStrictEqual(options.initialPrivateState,{}),'DEPLOYMENT_PRIVATE_STATE');
  requireThat(Array.isArray(options.args)&&options.args.length===6&&options.compiledContract,'DEPLOYMENT_CONSTRUCTOR_REQUIRED');
  requireThat(signingKey!==undefined&&ledger?.Transaction&&ledger?.ContractDeploy,'DEPLOYMENT_NATIVE_BINDINGS_REQUIRED');
  const compiledContract=options.compiledContract;
  const fixed={args:structuredClone(options.args),initialPrivateState:{},privateStateId:options.privateStateId};
  sdk??=await loadFinancialContractsSdk();
  const create=sdk.createUnprovenDeployTx,submit=sdk.submitTx,call=sdk.submitCallTx;
  requireThat([create,submit,call].every(fn=>typeof fn==='function'),'DEPLOYMENT_SDK_INTERFACE');
  const prepared=await create({walletProvider:publicWalletProvider,zkConfigProvider},{compiledContract,...structuredClone(fixed),signingKey});
  requireThat(prepared?.private?.unprovenTx instanceof ledger.Transaction,'PREPARED_NATIVE_TRANSACTION_REQUIRED');
  const raw=Buffer.from(prepared.private.unprovenTx.serialize());
  const native=ledger.Transaction.deserialize('signature','pre-proof','pre-binding',raw);
  requireThat(Buffer.from(native.serialize()).equals(raw),'PREPARED_NONCANONICAL_TRANSACTION');
  const actions=[...native.intents.values()].flatMap(intent=>intent.actions);
  const contractAddress=prepared.public?.contractAddress;
  requireThat(typeof contractAddress==='string'&&/^[a-f0-9]{64}$/.test(contractAddress)&&actions.length===1&&actions[0] instanceof ledger.ContractDeploy&&actions[0].address===contractAddress,'PREPARED_DEPLOYMENT_IDENTITY');
  const initialPrivateState=structuredClone(prepared.private.initialPrivateState),privateSigningKey=prepared.private.signingKey;
  requireThat(isDeepStrictEqual(initialPrivateState,{})&&privateSigningKey===signingKey,'PREPARED_PRIVATE_STATE_CHANGED');
  let consumed=false;
  const driverSdk=Object.freeze({
    async deployContract(providers,requested) {
      requireThat(!consumed,'PREPARED_DEPLOYMENT_ALREADY_CONSUMED');
      requireThat(requested && Object.keys(requested).sort().join(',')==='args,compiledContract,initialPrivateState,privateStateId'&&requested.compiledContract===compiledContract&&isDeepStrictEqual({args:requested.args,initialPrivateState:requested.initialPrivateState,privateStateId:requested.privateStateId},fixed),'PREPARED_DEPLOYMENT_OPTIONS_CHANGED');
      for(const name of ['setContractAddress','set','setSigningKey'])requireThat(typeof providers?.privateStateProvider?.[name]==='function','DEPLOYMENT_PRIVATE_PROVIDER');
      consumed=true; // Submission ambiguity is terminal for this exact handle.
      const unprovenTx=ledger.Transaction.deserialize('signature','pre-proof','pre-binding',raw);
      const finalized=await submit(providers,{unprovenTx});
      requireThat(finalized?.status==='SucceedEntirely','DEPLOYMENT_NOT_SUCCEEDED');
      requireThat(typeof finalized.txId==='string'&&finalized.txId.length>0,'DEPLOYMENT_MISSING_TRANSACTION_ID');
      providers.privateStateProvider.setContractAddress(contractAddress);
      await providers.privateStateProvider.set(fixed.privateStateId,initialPrivateState);
      await providers.privateStateProvider.setSigningKey(contractAddress,privateSigningKey);
      return {deployTxData:{public:{contractAddress,txId:finalized.txId}}};
    },
    submitCallTx:call,
  });
  return Object.freeze({public:Object.freeze({contractAddress,unprovenTransactionSha256:sha(raw),scope:'prepared-only'}),driverSdk});
}
