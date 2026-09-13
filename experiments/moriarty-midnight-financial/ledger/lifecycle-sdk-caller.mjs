/** Offline preparation for the SDK call boundary. No submission or deployment.
 * acceptedRead must come from the caller's accepted-state reader. This module
 * checks correspondence of supplied values, not authenticity of that read.
 * Full financial-state readback and compiled lifecycle correspondence stay open.
 */
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
import {lowerLifecycleSourceBinding} from './lifecycle-source-binding.mjs';
import {createFinancialAgreementSourceV6} from '../../moriarty-language/src/successor/financial-agreement-source-v6.ts';
import {admitFinancialLifecycleStateJSON} from '../../moriarty-language/src/successor/financial-lifecycle-v2.ts';
import {exactLedgerAmount} from '../../moriarty-language/src/successor/financial-outcome-intent.ts';
const requireThat=(x,c)=>{if(!x)throw Error(c);};
const hex=x=>typeof x==='string'&&/^[0-9a-f]{64}$/.test(x)&&x.length===64;
const uint=(x,bits)=>typeof x==='string'&&x.length<=39&&/^(0|[1-9][0-9]*)$/.test(x)&&!/[\r\n]/.test(x)&&BigInt(x)<(1n<<BigInt(bits));
const bytes=x=>{requireThat(hex(x),'LIFECYCLE_PUBLIC_BYTES');return Uint8Array.from(Buffer.from(x,'hex'));};
const sha=x=>createHash('sha256').update(x).digest('hex');
export async function loadLifecycleCapabilityRuntime() {
 const root=join(PINNED_NM,'@midnight-ntwrk/compact-runtime');
 requireThat(sha(readFileSync(join(root,'package.json')))==='ac4f818510afca0d17758b4c38af613f7b1a489aec96633548d0d361683f124c','LIFECYCLE_RUNTIME_PACKAGE');
 const path=join(root,'dist/index.js');
 requireThat(sha(readFileSync(path))==='c55a8ab3e7533b3fa6e27b66ab742c783d912d5a00d6ecc23d9f6142ddb0ecde','LIFECYCLE_RUNTIME_ENTRY');
 return import(pathToFileURL(path).href);
}
export function lifecycleCapability(runtime,domain,networkTag,programDigest,secret) {
 requireThat(secret instanceof Uint8Array&&secret.length===32,'LIFECYCLE_SECRET_BYTES');
 requireThat(['moriarty:sp05:loan:borrower','moriarty:sp05:loan:lender'].includes(domain),'LIFECYCLE_ROLE_DOMAIN');
 const padded=new Uint8Array(32);padded.set(new TextEncoder().encode(domain));
 const type=new runtime.CompactTypeVector(4,new runtime.CompactTypeBytes(32));
 return Buffer.from(runtime.persistentHash(type,[padded,bytes(networkTag),bytes(programDigest),secret])).toString('hex');
}
/** Build SDK call options in a private closure. The method invokes the supplied
 * SDK's createUnprovenCallTx only. It never invokes submitCallTx or submitTx.
 * Compiled contract provenance remains the enclosing launcher's responsibility.
 */
export async function prepareLifecycleOriginationCall({source,snapshotJSON,stateJSON,acceptedRead,expectedHead,roles,assetBinding,now,runtime}) {
 // Own all mutable caller inputs before the first asynchronous runtime load.
 acceptedRead=structuredClone(acceptedRead);expectedHead=structuredClone(expectedHead);
 roles=structuredClone(roles);assetBinding=structuredClone(assetBinding);
 const binding=lowerLifecycleSourceBinding(source);
 const admitted=admitFinancialLifecycleStateJSON(stateJSON);
 requireThat(admitted.ok,'LIFECYCLE_STATE_REJECTED');
 const state=admitted.value,a=state.authority;
 requireThat(a.debtor.party==='Borrower'&&a.lender.party==='Lender','AUTHORITY_ROLE');
 requireThat(acceptedRead&&expectedHead&&roles&&assetBinding,'LIFECYCLE_BINDING_REQUIRED');
 requireThat(hex(expectedHead.contractAddress)&&uint(expectedHead.revision,128),'LIFECYCLE_HEAD_SCHEMA');
 requireThat(acceptedRead.contractAddress===expectedHead.contractAddress&&acceptedRead.revision===expectedHead.revision,'REVISION_MISMATCH');
 requireThat(acceptedRead.originated===false,'ALREADY_ORIGINATED');
 for(const key of ['sourceDigest','coreDigest','programDigest'])requireThat(acceptedRead[key]===binding[key],'PROGRAM_MISMATCH');
 requireThat(a.programDigest===binding.programDigest&&a.networkTag===acceptedRead.networkTag,'NETWORK_PROGRAM_BINDING');
 requireThat(hex(roles.borrowerAddress)&&hex(roles.lenderAddress)&&roles.borrowerAddress!==roles.lenderAddress,'LIFECYCLE_ROLE_ADDRESSES');
 requireThat(roles.borrowerAddress===acceptedRead.borrowerAddress&&roles.lenderAddress===acceptedRead.lenderAddress,'AUTHORITY_ADDRESS');
 requireThat(a.debtor.capability===acceptedRead.borrowerCapability&&a.lender.capability===acceptedRead.lenderCapability,'AUTHORITY_BINDING');
 runtime??=await loadLifecycleCapabilityRuntime();
 requireThat(lifecycleCapability(runtime,'moriarty:sp05:loan:lender',a.networkTag,a.programDigest,roles.lenderSecret)===a.lender.capability,'LENDER_CAPABILITY');
 requireThat(lifecycleCapability(runtime,'moriarty:sp05:loan:borrower',a.networkTag,a.programDigest,roles.borrowerSecret)===a.debtor.capability,'BORROWER_CAPABILITY');
 requireThat(assetBinding.sourceAsset==='Cash'&&hex(assetBinding.ledgerColor)&&assetBinding.ledgerColor===acceptedRead.cashColor,'SETTLEMENT_ASSET');
 // Successor Amount<Cash> and kernel amounts already denote ledger quanta.
 requireThat(assetBinding.numerator==='1'&&assetBinding.denominator==='1','LIFECYCLE_QUANTUM_UNSUPPORTED');
 requireThat([...a.outcomeIntent.grossCaps,...a.outcomeIntent.minimumNetCredits].every(x=>x.asset==='Cash'),'LIFECYCLE_INTENT_ASSET_UNSUPPORTED');
 for(const [party,prefix] of [['Borrower','borrower'],['Lender','lender']]) {
  const cap=a.outcomeIntent.grossCaps.find(x=>x.actor===party&&x.asset==='Cash');
  requireThat(cap,'INTENT_DEBIT_UNCAPPED');
  requireThat(cap.maximumLedgerAmount===acceptedRead[prefix+'GrossCap'],'INTENT_BINDING');
  const goal=a.outcomeIntent.minimumNetCredits.find(x=>x.actor===party&&x.asset==='Cash');
  requireThat(acceptedRead[prefix+'HasNetGoal']===(goal!==undefined),'INTENT_BINDING');
  if(goal)requireThat(goal.minimumLedgerAmount===acceptedRead[prefix+'MinimumNetCredit'],'INTENT_BINDING');
 }
 requireThat(uint(now,64)&&BigInt(now)<2000000000n,'HORIZON_EXPIRED');
 const prepared=createFinancialAgreementSourceV6().evaluate(source,'originate',snapshotJSON,stateJSON);
 requireThat(prepared.status==='FundedExpressionPrepared',prepared.code??'LIFECYCLE_SOURCE_REJECTED');
 requireThat(prepared.effects.map(x=>x.kind).join(',')==='Transfer,Origination,Fee','LIFECYCLE_EFFECT_SHAPE');
 const [disbursement,,fee]=prepared.effects;
 requireThat(disbursement.amount===binding.disbursementAmount&&fee.from==='Borrower'&&fee.to==='Lender'&&fee.asset==='Cash','LIFECYCLE_EFFECT_BINDING');
 const feeAmount=exactLedgerAmount(fee.amount,assetBinding.numerator,assetBinding.denominator);
 const args=[new Uint8Array(roles.borrowerSecret),new Uint8Array(roles.lenderSecret),bytes(binding.programDigest),bytes(a.networkTag),BigInt(expectedHead.revision),5n,2n,BigInt(feeAmount),BigInt(now)];
 const contractAddress=expectedHead.contractAddress;
 return Object.freeze({
  public:Object.freeze({binding,contractAddress,expectedRevision:expectedHead.revision,circuitId:'originate',feeAmount,sourcePrepared:true,compiledVerified:false,networkAcceptance:false}),
  async createUnprovenCall({sdk,providers,compiledContract}) {
   requireThat(typeof sdk?.createUnprovenCallTx==='function'&&compiledContract,'LIFECYCLE_COMPILED_SDK_REQUIRED');
   return sdk.createUnprovenCallTx(providers,{compiledContract,contractAddress,privateStateId:'loan-lifecycle',circuitId:'originate',args:structuredClone(args)});
  },
 });
}
