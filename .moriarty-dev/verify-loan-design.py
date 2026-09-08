# Recheck the retained design against its independent arithmetic/reference.
# Adaptation of the archived verifier: use this working tree and published subset status.
from pathlib import Path
import json,hashlib
W=Path.cwd();B=json.loads((W/'evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/binding.json').read_text());base=W/'evidence/moriarty-completion-program-2026-09-07'
def check(ok,label):
 if not ok:raise AssertionError(label)
for p,h in B['inputs'].items():check(hashlib.sha256((W/p).read_bytes()).hexdigest()==h,'input hash '+p)
for p in B['ownedFiles']:check((W/p).is_file(),'missing owned artifact '+p)
g=json.loads((base/'report-reconciliation/semantic-challenges.json').read_text());t=json.loads((base/'SP01/loan-swap-subset-01/traces.json').read_text());legacy=json.loads((base/'MC01/profile-04/evaluator/legacy-reference.json').read_text())
check(g['status']=='pending-review' and g['subsets']['RP01-MC02']['status']=='complete','published subset status; whole program remains pending');check(g['subsets']['RP01-MC03']['status']=='specified-only','native subset scope')
ids=['loan-accrue','loan-settle','swap-exact-input','swap-close'];check(set(t['positiveRows'])==set(ids),'four named positive rows');check(set(g['subsets']['RP01-MC02']['negativeTraceIds'])==set(t['negativeTraces']),'negative trace identities')
num=5000000000*8*31;den=100*365;q,rem=divmod(num,den);a=t['independentArithmetic']['loan'];check([int(a[x]) for x in ['numerator','denominator','floorInterest','remainder','totalDue']]==[num,den,q,rem,500000000+q],'independent loan arithmetic')
num=10000*997*2000000;den=1000000*1000+10000*997;q,rem=divmod(num,den);a=t['independentArithmetic']['swap'];check([int(a[x]) for x in ['numerator','denominator','floorOutput','remainder']]==[num,den,q,rem],'independent swap arithmetic')
names={'principalDue':'principal_due','interestDue':'interest_due','principalPaid':'principal_paid','interestPaid':'interest_paid','borrowerCash':'borrower_cash','lenderCash':'lender_cash','reserveA':'reserve_a','reserveB':'reserve_b','traderA':'trader_a','traderB':'trader_b','providerA':'provider_a','providerB':'provider_b'}
scalars={'demo:USD6':'USD_TEST_ASSET','demo:A':'ASSET_A','demo:B':'ASSET_B','pool:demo':'pool','USD':'USD_micro','loan:demo:principal':'lam01:period1:PR','loan:demo:interest':'lam01:period1:IP'}
for idx,rid in enumerate(ids):
 row=t['positiveRows'][rid];name='loan' if idx<2 else 'swap';step=idx%2;ref=legacy['cases'][name]['steps'][step]['result']
 rename=lambda x:('episode_closed' if name=='loan' else 'epoch_closed') if x=='closed' else names.get(x,x)
 values={x['name']:x['value']['value'] for x in row['afterValues']};check(values=={rename(k):v for k,v in ref['after']['values'].items()},rid+' complete state values');check(row['revision']==ref['after']['revision'] and row['remaining']==ref['after']['remaining'],rid+' lifecycle');check([x['field'] for x in row['orderedWrites']]==[rename(k) for k in ref['writes']],rid+' ordered writes')
 for ef in row['orderedEffects']:
  common={'amount','kind','ordinal'}
  if ef['kind'] in ['Transfer','Fee']:want=common|{'asset','from','to','settlement'}
  elif ef['kind']=='DueCreated':want=common|{'dueId','debtor','creditor','denomination'}
  else:want=common|{'dueId','debtor','creditor','denomination','asset','settlement'}
  check(set(ef)==want,rid+' '+ef['kind']+' must match exact runtime EffectRecord keys; absent fields cannot be null')
 expected=[]
 for ordinal,old in enumerate(ref['effects']):
  fields={k:scalars.get(v,v) for k,v in old['fields'].items()};unit='USD_micro' if name=='loan' else 'AssetA_quantum' if fields['asset']=='ASSET_A' else 'AssetB_quantum';amount={'tag':'Amount','unit':unit,'value':fields['amount']};ef={**fields,'amount':amount,'kind':old['kind'],'ordinal':str(ordinal)}
  if old['kind']!='DueCreated':ef['settlement']={'asset':fields['asset'],'binding':'USD_micro_asset' if name=='loan' else 'asset_a_binding' if unit=='AssetA_quantum' else 'asset_b_binding','ledgerAmount':amount['value'],'nominalAmount':amount,'quantum':{'tag':'Amount','unit':unit,'value':'1'},'unit':unit}
  expected.append(ef)
 check(row['orderedEffects']==expected,rid+' all complete ordered effect operands')
 check(row['episodeStatus']==('Open' if step==0 else 'Closed'),rid+' episode status');check(row['agreementStatus']==('Outstanding' if name=='loan' else 'NoOutstanding'),rid+' agreement status')
 if name=='loan':check(row['remainingNotional']['amount']['value']=='4500000000' and [x['status'] for x in row['retainedObligations']]==(['Outstanding']*2 if step==0 else ['Settled']*2),rid+' retained duty/tombstones')
 else:check(not row['retainedObligations'] and row['remainingNotional']=={'tag':'NotApplicable'},rid+' no dues')
print(json.dumps({'status':'pass','inputHashes':len(B['inputs']),'positiveRows':len(ids),'negativeTraceIdentities':len(t['negativeTraces']),'independentArithmetic':'exact Python integers','stateEffectReference':'retained independent legacy capture with documented naming and unit conversion','scope':'Static trace/reference/schema checks, no evaluator/proof/network execution'},indent=2))
