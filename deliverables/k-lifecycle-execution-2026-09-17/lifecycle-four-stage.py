import json,sys,time,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]; H=ROOT/'experiments/moriarty-language/formal/k';sys.path.insert(0,str(H))
import run, lifecycle_codec as lc, expression_codec as ex
run.BUILD=H/'.build-lifecycle-v1';run.DEFINITION=run.BUILD/'lifecycle-v1-kompiled';run.DEADLINE=time.monotonic()+120
binding=json.loads((run.BUILD/'binding.json').read_text());assert binding['sources']=={n:run.sha(H/n) for n in binding['sources']};assert binding['artifacts']==run.artifacts()
lock=json.loads((H/'expression-toolchain.lock.json').read_text());assert all(run.sha(Path(n))==v for n,v in lock['files'].items())
fixtures=Path(__file__).parent/'four-stage-fixtures.json';cases=json.loads(fixtures.read_text());assert len(cases)==4
with (run.BUILD/'four-stage-attempt.json').open('x') as f:json.dump({'fixtureSha256':run.sha(fixtures),'started':time.time(),'maxInvocations':4},f)
previous=None;observations=[]
for i,c in enumerate(cases):
 packet=c['packet'].copy()
 if previous is not None:
  request=json.loads(packet['request']);request['Pre']=previous['post'];request['workInitial']=previous['workRemaining'];packet['request']=ex._canonical(request);packet['financialPreState']=ex._canonical(previous['financialPost'])
 admitted=lc.admit_packet(packet);term=lc.encode_packet(admitted)
 label=f'lifecycle-{i+1:02d}';program=run.BUILD/(label+'.input');program.write_text(ex.serialize_term({'format':'KAST','version':4,'term':term})+'\n')
 binding['krunInvocations']+=1;run.write(run.BUILD/'binding.json',binding)
 raw=run.command(['/usr/bin/krun',str(program),'--definition',str(run.DEFINITION),'--output','json','--parser',str(H/'lifecycle-parser.py'),'--no-expand-macros'],20,label,64*1024*1024)
 result=lc.decode_result(raw,admitted);match=ex._canonical(result)==ex._canonical(c['expected']);run.write(run.BUILD/(label+'.result.json'),result);observations.append({'id':c['id'],'matchesExpected':match,'result':result});run.write(run.BUILD/'four-stage-observations.json',observations);print(json.dumps({'id':c['id'],'matchesExpected':match}),flush=True)
 if not match:raise RuntimeError('EXPECTED_RESULT_MISMATCH')
 previous=result
