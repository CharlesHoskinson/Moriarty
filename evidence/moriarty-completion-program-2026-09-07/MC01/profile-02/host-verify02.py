import json,pathlib,hashlib,collections,subprocess,time
from fractions import Fraction
s=pathlib.Path('/home/charl/.local/state/moriarty/mc01-supervised-20260907');w=pathlib.Path('/home/charl/Moriarty-wt-moriarty-mc01-20260907-plan-profile');spec=w/'experiments/moriarty-language/spec';start=time.monotonic()
assert (s/'worker-05-receipt.json').exists(),'worker not terminal'
b=json.loads((s/'budget.json').read_text())
if any(x['id']=='verify-profile-02' for x in b['reserved']):
 x=next(x for x in b['reserved'] if x['id']=='verify-profile-02');b['reserved'].remove(x);b['charges'].append({**x,'basis':'Full bound for host source verification, candidate freeze and evidence'});(s/'budget.json').write_text(json.dumps(b,indent=2)+'\n')
files=sorted(p for p in spec.rglob('*') if p.is_file());jsons=[p for p in files if p.suffix=='.json']
for p in jsons:json.loads(p.read_text())
old=json.loads((s/'profile-candidate-01.json').read_text());assert all((w/f['path']).is_file() for f in old['files'])
o=json.loads((s/'source-row-oracle.json').read_text());cross=json.loads((spec/'target-crosswalk.json').read_text());rows=cross['rows'];assert len(rows)==104 and len({x['row_id'] for x in rows})==104 and all(x['mc07_mandatory'] is True for x in rows)
for family,key in [('ACTUS','actus'),('DEFI','defi')]:
 actual=[x['source'] for x in rows if x['source_family']==family]
 assert collections.Counter(json.dumps(x,sort_keys=True) for x in actual)==collections.Counter(json.dumps(x,sort_keys=True) for x in o[key])
for f in o['source_files']:assert hashlib.sha256(pathlib.Path(f['path']).read_bytes()).hexdigest()==f['sha256']
interest=Fraction(5000000000*8*31,100*365);floor=interest.numerator//interest.denominator;settlement=500000000+floor;notional=5000000000-500000000
swap=Fraction(10000*997*2000000,1000000*1000+10000*997);output=swap.numerator//swap.denominator
assert (floor,settlement,notional,output)==(33972602,533972602,4500000000,19743)
changed=[f['path'] for f in old['files'] if hashlib.sha256((w/f['path']).read_bytes()).hexdigest()!=f['sha256']]
d={'scope':'JSON syntax, original artifacts, exact ACTUS/DeFi source rows, source preservation, independent rational sample arithmetic; not parser/typechecker/compiler/proof execution','json_files':len(jsons),'spec_files':len(files),'rows':{'ACTUS':32,'DeFi':72},'loan':{'interest':str(interest),'floor':floor,'remainder':str(interest-floor),'settlement':settlement,'remaining_notional':notional},'swap':{'output':output,'reserve_a':1010000,'reserve_b':2000000-output},'changed_original_files':changed,'worker_receipt':json.loads((s/'worker-05-receipt.json').read_text()),'elapsed_seconds':time.monotonic()-start,'checks_pass':True,'profile_approved':False}
(s/'host-verification-02.json').write_text(json.dumps(d,indent=2)+'\n');print(json.dumps(d,indent=2))
