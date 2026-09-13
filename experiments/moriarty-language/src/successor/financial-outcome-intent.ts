/** Per-asset outcome constraints. Amounts are already exact ledger quanta.
 * This is an admission/checking boundary, not a signature verifier. */
export interface GrossCap { actor: string; asset: string; maximumLedgerAmount: string }
export interface NetGoal { actor: string; asset: string; minimumLedgerAmount: string }
export interface OutcomeIntent { grossCaps: GrossCap[]; minimumNetCredits: NetGoal[] }
export interface LifecycleAuthority {
  networkTag: string;
  programDigest: string;
  debtor: { party: string; capability: string };
  lender: { party: string; capability: string };
  outcomeIntent: OutcomeIntent;
}
const MAX = (1n << 128n) - 1n;
const identifier = (x: unknown): x is string => typeof x === 'string' && /^[A-Za-z][A-Za-z0-9_]{0,63}$/.test(x) && !/[\r\n]/.test(x);
const uint = (x: unknown): x is string => typeof x === 'string' && x.length <= 39 && /^(0|[1-9][0-9]*)$/.test(x) && !/[\r\n]/.test(x) && BigInt(x) <= MAX;
const record = (x: unknown): x is Record<string, any> => x !== null && typeof x === 'object' && !Array.isArray(x);
const keys = (x: unknown, expected: string[]) => record(x) && Object.keys(x).length === expected.length && expected.every(k => Object.hasOwn(x,k));
const pair = (actor: string, asset: string) => actor+'\0'+asset;
export function validOutcomeIntent(value: unknown): value is OutcomeIntent {
  if (!keys(value,['grossCaps','minimumNetCredits'])) return false;
  const v = value as OutcomeIntent;
  return ([['grossCaps','maximumLedgerAmount'],['minimumNetCredits','minimumLedgerAmount']] as const).every(([list,amount]) => {
    const rows = v[list]; if (!Array.isArray(rows) || rows.length > 128) return false;
    let previous = '';
    return rows.every(row => {
      if (!keys(row,['actor','asset',amount]) || !identifier(row.actor) || !identifier(row.asset) || !uint((row as any)[amount])) return false;
      const k=pair(row.actor,row.asset);if(k<=previous)return false;previous=k;return true;
    });
  });
}
export function validLifecycleAuthority(value: unknown): value is LifecycleAuthority {
  if (!keys(value,['networkTag','programDigest','debtor','lender','outcomeIntent'])) return false;
  const v=value as LifecycleAuthority;
  const digest=(s:unknown)=>typeof s==='string'&&/^[0-9a-f]{64}$/.test(s)&&s.length===64;
  return digest(v.networkTag) && digest(v.programDigest)
    && [v.debtor,v.lender].every(r=>keys(r,['party','capability'])&&identifier(r.party)&&digest(r.capability))
    && v.debtor.party!==v.lender.party && v.debtor.capability!==v.lender.capability && validOutcomeIntent(v.outcomeIntent)
    && [...v.outcomeIntent.grossCaps,...v.outcomeIntent.minimumNetCredits].every(r=>r.actor===v.debtor.party||r.actor===v.lender.party);
}
export function checkOutcomeIntent(effects: readonly any[], intent: OutcomeIntent): {ok:true} | {ok:false;code:string} {
  const reject=(code:string)=>({ok:false as const,code});
  if (!validOutcomeIntent(intent) || !Array.isArray(effects) || effects.length>128) return reject('INTENT_SCHEMA');
  const caps=new Map(intent.grossCaps.map(c=>[pair(c.actor,c.asset),BigInt(c.maximumLedgerAmount)]));
  const credits=new Map<string,bigint>(),debits=new Map<string,bigint>();
  for(const e of effects){
    if(!record(e)||typeof e.kind!=='string')return reject('INTENT_EFFECT_SCHEMA');
    if(['Originate','Accrue','Repay','Origination','Accrual','Repayment','DueCreated','DueSettled'].includes(e.kind))continue;
    if(e.kind!=='Transfer'&&e.kind!=='Fee')return reject('INTENT_EFFECT_SCHEMA');
    if(!identifier(e.from)||!identifier(e.to)||!identifier(e.asset)||!uint(e.amount))return reject('INTENT_SCHEMA');
    const from=pair(e.from,e.asset),to=pair(e.to,e.asset),amount=BigInt(e.amount);
    if(!caps.has(from))return reject('INTENT_DEBIT_UNCAPPED');
    const debit=(debits.get(from)??0n)+amount,credit=(credits.get(to)??0n)+amount;
    if(debit>MAX||credit>MAX)return reject('INTENT_ARITHMETIC_OVERFLOW');
    debits.set(from,debit);credits.set(to,credit);
  }
  for(const [key,debit] of debits)if(debit>caps.get(key)!)return reject('INTENT_DEBIT_CAP');
  for(const goal of intent.minimumNetCredits){
    const key=pair(goal.actor,goal.asset),required=(debits.get(key)??0n)+BigInt(goal.minimumLedgerAmount);
    if(required>MAX)return reject('INTENT_ARITHMETIC_OVERFLOW');
    if((credits.get(key)??0n)<required)return reject('INTENT_NET_GOAL');
  }
  return {ok:true};
}
/** Explicit exact conversion for SDK units. No DUST/Cash conversion exists. */
export function exactLedgerAmount(amount: string, numerator: string, denominator: string): string {
  if(!uint(amount)||!uint(numerator)||!uint(denominator)||numerator==='0'||denominator==='0')throw Error('QUANTUM_SCHEMA');
  const product=BigInt(amount)*BigInt(numerator);
  if(product>MAX)throw Error('INTENT_ARITHMETIC_OVERFLOW');
  if(product%BigInt(denominator)!==0n)throw Error('QUANTUM_INEXACT');
  return (product/BigInt(denominator)).toString();
}
