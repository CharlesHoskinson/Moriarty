/** Public state is produced by the SDK Compact runtime; transactions use a
 * different ledger runtime. Bind the state class to the actual provider import.
 * These direct package/entry pins are not a transitive supply-chain attestation.
 * Import reads only public installed code, never providers, wallets or secrets.
 */
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
const root=PINNED_NM+'/@midnight-ntwrk/';
const pins={
 'midnight-js-protocol/package.json':'bdfe30f046627f364fd057fdf4dd756655fb4a4309722b64bb3e46484bf9effd',
 'midnight-js-protocol/dist/compact-runtime.mjs':'24e261044bcf1b12452393d31a0d1965335397e03a32b288822ffd06fb1c012b',
 'compact-runtime/package.json':'ac4f818510afca0d17758b4c38af613f7b1a489aec96633548d0d361683f124c',
 'compact-runtime/dist/index.js':'c55a8ab3e7533b3fa6e27b66ab742c783d912d5a00d6ecc23d9f6142ddb0ecde'
};
for(const [path,digest] of Object.entries(pins))if(createHash('sha256').update(readFileSync(root+path)).digest('hex')!==digest)throw Error('CONTRACT_BALANCE_RUNTIME_PIN');
const {ContractState:ProviderContractState}=await import(pathToFileURL(root+'midnight-js-protocol/dist/compact-runtime.mjs').href);
export const CONTRACT_BALANCE_FAILURE_CODES=Object.freeze(['RUNTIME_PIN','STATE','MAP','COUNT','TOKEN','TOKEN_FIELDS','AMOUNT','DUPLICATE'].map(x=>'CONTRACT_BALANCE_'+x));
const check=(ok,label)=>{if(!ok)throw Error('CONTRACT_BALANCE_'+label);};
const mapSize=Object.getOwnPropertyDescriptor(Map.prototype,'size').get;
export function extractNativeContractBalances({state}){
 check(state instanceof ProviderContractState,'STATE');
 return projectUnshieldedContractBalanceMap(state.balance);
}
/** Pure map projection; establishes no state identity, provenance or dispatch. */
export function projectUnshieldedContractBalanceMap(balances){
 check(balances instanceof Map,'MAP');
 const size=mapSize.call(balances);check(Number.isSafeInteger(size)&&size>=0&&size<=4096,'COUNT');
 const result={};let count=0;
 // Use the actual Map iterator, not caller-overridden iterable methods.
 for(const [token,amount] of Map.prototype.entries.call(balances)){
  check(++count<=4096,'COUNT');
  check(token&&Object.getPrototypeOf(token)===Object.prototype,'TOKEN');
  const fields=Object.getOwnPropertyDescriptors(token);
  check(Reflect.ownKeys(fields).length===2&&Object.hasOwn(fields,'tag')&&Object.hasOwn(fields,'raw')&&Object.hasOwn(fields.tag,'value')&&Object.hasOwn(fields.raw,'value'),'TOKEN_FIELDS');
  const tag=fields.tag.value,raw=fields.raw.value;
  check(tag==='unshielded'&&typeof raw==='string'&&/^[a-f0-9]{64}$/.test(raw),'TOKEN');
  check(typeof amount==='bigint'&&amount>=0n&&amount<(1n<<128n),'AMOUNT');
  check(!Object.hasOwn(result,raw),'DUPLICATE');result[raw]=amount.toString();
 }
 check(count===size,'COUNT');return Object.freeze(result);
}
