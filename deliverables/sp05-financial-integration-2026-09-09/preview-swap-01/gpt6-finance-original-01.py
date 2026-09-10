import pathlib,json
from fractions import Fraction
r=pathlib.Path('/home/charl/Moriarty/.worktrees/sp05-preview-owner');d=r/'deliverables/sp05-financial-integration-2026-09-09/preview-swap-01';raw=json.loads((d/'actual-run/integration-result.json').read_text());e=json.loads((r/'experiments/moriarty-midnight-financial/ledger/financial-expectations.json').read_text())['cases']['swap'];plan=json.loads((d/'actual-run/plan.json').read_text());colors=raw['assetBindings'];domains={a:v['domainHex'] for a,v in e['bindingParameters']['assets'].items()};roles={'trader':plan['roles']['firstAddress'],'provider':plan['roles']['secondAddress'],'pool':raw['driver']['contractAddress']};checks=[];previous={a:0 for a in colors}
canonical=lambda claims:sorted(json.dumps(c,sort_keys=True) for c in claims)
for rec,summary in zip(raw['driver']['stages'],raw['comparisons']):
 expected=e['stages'][rec['circuitId']];assert summary['stage']==rec['circuitId'];effects={'unshieldedMints':{},'unshieldedInputs':{},'unshieldedOutputs':{},'claimedUnshieldedSpends':[]}
 for a in rec['transaction']['actions']:
  for t in a.get('transcripts',[]):
   for f in effects:
    if isinstance(effects[f],dict):
     for k,v in t['effects'][f].items():effects[f][k]=str(int(effects[f].get(k,'0'))+int(v))
    else:effects[f].extend(t['effects'][f])
 for f in ['unshieldedMints','unshieldedInputs','unshieldedOutputs']:
  assert effects[f]==summary['nativeEffects'][f];observed={k:int(v) for k,v in effects[f].items() if int(v)};want={(domains[asset] if f=='unshieldedMints' else colors[asset]):int(v) for asset,v in expected['nativeEffects'][f].items() if int(v)};assert observed==want
 want=[{'type':colors[x['asset']],'recipientKind':x['recipientKind'],'recipient':roles[x['recipientRole']],'amount':x['amount']} for x in expected['nativeEffects']['claimedUnshieldedSpends']];assert canonical(effects['claimedUnshieldedSpends'])==canonical(want)==canonical(summary['nativeEffects']['claimedUnshieldedSpends'])
 for role,address in roles.items():
  if role=='pool':continue
  for asset,color in colors.items():
   delta=sum(int(x['value']) for x in rec['transaction']['outputs'] if x['owner']==address and x['type']==color)-sum(int(x['value']) for x in rec['transaction']['inputs'] if x['owner']==address and x['type']==color);assert str(delta)==expected['participantNetDeltas'][role][asset]
 for asset,color in colors.items():
  balance=int(rec['contractBalances'].get(color,'0'));assert balance==int(expected['contractReserves'][asset]);delta=balance-previous[asset]
  uin=sum(int(x['value']) for x in rec['transaction']['inputs'] if x['type']==color);uout=sum(int(x['value']) for x in rec['transaction']['outputs'] if x['type']==color)
  assert uout-uin+delta==int(effects['unshieldedMints'].get(domains[asset],'0'));assert delta==int(effects['unshieldedInputs'].get(color,'0'))-int(effects['unshieldedOutputs'].get(color,'0'));previous[asset]=balance
 checks.append({'stage':summary['stage'],'nativeEffectsMatchIndependentExpectations':True,'participantDeltasMatch':True,'perAssetConservation':True})
rate=Fraction(10000*997*2000000,1000000*1000+10000*997);output=rate.numerator//rate.denominator;assert output==19743;assert 2000000-output==1980257;assert 1000000+10000==1010000;assert previous=={'ASSET_A':0,'ASSET_B':0};assert raw['comparisons'][-1]['publicState']['remaining']=='0';out={'status':'PASS','checks':checks,'swapFormula':'floor(10000*997*2000000/(1000000*1000+10000*997))','tradeInputA':'10000','tradeOutputB':str(output),'roundingRemainder':str(rate-output),'nativeWalletInputA':'100000','changeA':'90000','closeToProvider':{'A':'1010000','B':'1980257'},'finalReserves':previous,'assetScope':'Actual native minted Preview test assets; no external backing inferred'};(d/'gpt6-financial-semantics-01.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({'status':'PASS','stages':4,'finalReserves':previous,'tradeOutputB':str(output)}))
