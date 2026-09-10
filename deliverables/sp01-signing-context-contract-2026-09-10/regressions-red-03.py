import importlib.util,json
from pathlib import Path
O=Path(__file__).parent;s=importlib.util.spec_from_file_location('f',O/'context-checker-02.test.py');f=importlib.util.module_from_spec(s);s.loader.exec_module(f)
rows=[]
d=f.fixture();d['steps'][0]['plan']['arguments'][1]['value']='99';f.rehash(d);rows.append({'case':'disconnectedDisplayedQuantity','actual':f.c.validate(d,f.context()),'expected':'ARGUMENT_BINDING'})
d=f.fixture();d['authority']['nonce']='\ud800'
try:r=f.c.validate(d,f.context())
except Exception as e:r={'uncaught':type(e).__name__}
rows.append({'case':'malformedUnicode','actual':r,'expected':'closed rejection'})
g=f.fixture()['genesis'];g['funding']=[{'id':'one','owner':'alice','asset':'A','amount':str(f.c.MAX)},{'id':'two','owner':'alice','asset':'A','amount':'1'}];r=f.c.initial(g);rows.append({'case':'summedGenesisOverflow','actual':r['balances']['alice.A'],'expected':'NUMERIC_DOMAIN'})
ctx=f.context();ctx['network']='preview';d=f.fixture();d['authority']['network']='preview';f.rehash(d);rows.append({'case':'profileNetworkNotGeneric','actual':f.c.validate(d,ctx),'expected':'support exact profile-owned Preview label'})
print(json.dumps(rows,indent=2));raise SystemExit(1)
