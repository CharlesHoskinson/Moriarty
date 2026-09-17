"""Prepare full configurations and validate KAST/KORE offline; never runs krun."""
import json,sys,subprocess,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
H=ROOT/'experiments/moriarty-language/formal/k'
sys.path.insert(0,str(H))
import expression_codec as ex
D=H/'.build-lifecycle-v1/lifecycle-v1-kompiled'
OUT=Path(__file__).with_name('unicode-counter-wrapper04');OUT.mkdir(exist_ok=True)
cases=json.loads((H/'fixtures/unicode-counter-cases.json').read_text())
records=[]
for i,c in enumerate(cases,1):
 token=ex._string(c['text'])
 fn=lambda name,module,sort,arg:ex._apply(f'{name}(_)_MORIARTY-LIFECYCLE-{module}_Int_{sort}',arg)
 values=[fn('lxUtf8','EXPRESSION-WIRE','String',token),fn('lcUtf16','EXPRESSION-V1','String',token),fn('lxJsonBytes','EXPRESSION-WIRE','EJSON',ex._apply('ejString',token)),fn('lcQuotedUnits','EXPRESSION-V1','String',token)]
 term=ex._apply('ejNil')
 for value in reversed(values):term=ex._apply('ejCons',ex._apply('ejNumber',value),term)
 term=ex._apply('ejArray',term)
 cells=[ex._apply('<k>',{'node':'KSequence','arity':1,'items':[term]})]
 for name in ['sigma','request','pre','args','obs']:cells.append(ex._apply(f'<{name}>',ex._apply('ejMissing')))
 for name in ['localTypes','locals','written','writes']:cells.append(ex._apply(f'<{name}>',ex._apply('ejObject',ex._apply('ejNil'))))
 cells.append(ex._apply('<descriptors>',ex._apply('ejNil')))
 for name in ['sourceBytes','initialWork','work']:cells.append(ex._apply(f'<{name}>',ex._token('Int','0')))
 cells.extend([ex._apply('<reducing>',ex._token('Bool','false')),ex._apply('<out>',ex._apply('lcPending')),ex._apply('<financial>',ex._apply('lcMissingContext'))])
 for name in ['financialPre','financialPost']:cells.append(ex._apply(f'<{name}>',ex._apply('ejMissing')))
 cells.append(ex._apply('<stepTransfers>',ex._apply('ejArray',ex._apply('ejNil'))))
 for name in ['effects','actions','suffix']:cells.append(ex._apply(f'<{name}>',ex._apply('ejNil')))
 for name in ['suffixIndex','suffixWork','generatedCounter']:cells.append(ex._apply(f'<{name}>',ex._token('Int','0')))
 assert len(cells)==26
 p=OUT/f'counter-{i}.kast.json';p.write_text(ex.serialize_term({'format':'KAST','version':4,'term':ex._apply('<generatedTop>',*cells)})+'\n')
 cmd=['/usr/bin/kast','--input','json','--output','kore','--sort','GeneratedTopCell','--definition',str(D),str(p)]
 result=subprocess.run(cmd,capture_output=True,text=True,timeout=20)
 (OUT/f'counter-{i}.kore').write_text(result.stdout);(OUT/f'counter-{i}.stderr').write_text(result.stderr)
 assert result.returncode==0,result.stderr
 assert result.stdout.startswith("Lbl'-LT-'generatedTop'-GT-'{}("),result.stdout[:100]
 assert 'SortERequest' not in result.stdout and '(null)' not in result.stdout
 assert "kseq{}(inj{SortEJSON{}, SortKItem{}}" in result.stdout
 records.append({'id':i,'command':cmd,'returncode':result.returncode,'koreSha256':hashlib.sha256(result.stdout.encode()).hexdigest()})
(OUT/'preflight.json').write_text(json.dumps({'nativeInvocations':0,'records':records},indent=2)+'\n')
print('Six full GeneratedTopCell configurations passed offline parsing; zero native invocations.')
