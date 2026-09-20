import json
from aeon.facade.driver import AeonConfig,AeonDriver
from aeon.facade.trust import compute_trust_report
from aeon.synthesis.uis.api import SilentSynthesisUI
cases={
 'valid_bounded_payment':'def pay : {p:Int | p >= 90 && p + 3 <= 100} := 97;',
 'fee_over_budget':'def pay : {p:Int | p >= 90 && p + 3 <= 100} := 100;',
 'misses_minimum':'def pay : {p:Int | p >= 90 && p + 3 <= 100} := 89;',
 'false_native_promise':'def pay : {p:Int | p >= 90 && p + 3 <= 100} := native "100";',
 'zero_division':'def pay : Int := 10 / 0;',
}
result=[]
for name,source in cases.items():
 d=AeonDriver(AeonConfig(synthesizer='smt',synthesis_ui=SilentSynthesisUI(),synthesis_budget=0,no_main=True,strict_decidable=True))
 errors=list(d.parse(aeon_code=source)); trust=[]
 if not errors: trust=[{'name':x.display,'kind':x.kind} for x in compute_trust_report(d.core).items]
 result.append({'case':name,'source':source,'accepted':not errors,'errors':[type(e).__name__ for e in errors],'trust':trust})
print(json.dumps(result,indent=2))
assert [r['accepted'] for r in result]==[True,False,False,True,False]
assert any(t['kind']=='native' for t in result[3]['trust'])
