/** Extract unshielded financial balances from the same exact-block native state
 * the caller uses for its public-state decoder. The caller supplies its pinned
 * ledger runtime; no provider/runtime loading, queries or ledger acceptance here.
 * Shielded/DUST variants are rejected because they cannot share this raw-ID map.
 */
export const CONTRACT_BALANCE_FAILURE_CODES=Object.freeze(['STATE','MAP','COUNT','TOKEN','TOKEN_FIELDS','AMOUNT','DUPLICATE'].map(x=>'CONTRACT_BALANCE_'+x));
const check=(ok,label)=>{if(!ok)throw Error('CONTRACT_BALANCE_'+label);};
const mapSize=Object.getOwnPropertyDescriptor(Map.prototype,'size').get;
export function extractNativeContractBalances({state,ledger}){
 check(typeof ledger?.ContractState==='function'&&state instanceof ledger.ContractState,'STATE');
 const balances=state.balance;check(balances instanceof Map,'MAP');
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
