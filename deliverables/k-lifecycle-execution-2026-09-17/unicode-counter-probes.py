import sys,json,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];H=ROOT/'experiments/moriarty-language/formal/k';sys.path.insert(0,str(H))
import run,expression_codec as ex
run.BUILD=H/'.build-unicode-counters';run.DEFINITION=H/'.build-lifecycle-v1/lifecycle-v1-kompiled';run.DEADLINE=time.monotonic()+140
binding=json.loads((H/'.build-lifecycle-v1/binding.json').read_text());assert binding['sources']=={n:run.sha(H/n) for n in binding['sources']};assert binding['artifacts']==run.artifacts()
lock=json.loads((H/'expression-toolchain.lock.json').read_text());assert all(run.sha(Path(n))==v for n,v in lock['files'].items())
cases=json.loads((H/'fixtures/unicode-counter-cases.json').read_text());assert len(cases)==6;run.BUILD.mkdir(exist_ok=True)
with (run.BUILD/'attempt.json').open('x') as f:json.dump({'started':time.time(),'maxInvocations':6,'fixtureSha256':run.sha(H/'fixtures/unicode-counter-cases.json')},f)
observations=[]
for i,c in enumerate(cases):
 s=ex._quote_k_string(c['text']);values=[f'lxUtf8({s})',f'lcUtf16({s})',f'lxJsonBytes(ejString({s}))',f'lcQuotedUnits({s})'];term='ejNil()'
 for v in reversed(values):term=f'ejCons(ejNumber({v}),{term})'
 label=f'counter-{i+1}';program=run.BUILD/(label+'.input');program.write_text(f'ejArray({term})\n')
 run.write(run.BUILD/'invocations.json',{'attempted':i+1})
 parser=f'/usr/bin/kast --sort K --output kore --definition {run.DEFINITION}'
 raw=run.command(['/usr/bin/krun',str(program),'--term','--definition',str(run.DEFINITION),'--output','json','--parser',parser,'--no-expand-macros'],20,label,64*1024*1024)
 result=ex.decode_term(ex._parse(raw,True)['term']);expected=[c[k] for k in ['utf8','utf16','quotedBytes','quotedUnits']];observations.append({'id':label,'result':result,'expected':expected,'matchesExpected':result==expected});run.write(run.BUILD/'observations.json',observations);print(json.dumps(observations[-1]),flush=True)
 if result!=expected:raise RuntimeError('COUNTER_MISMATCH')
