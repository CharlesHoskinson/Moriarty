from pathlib import Path
import json,hashlib,subprocess,datetime,shutil,runpy,traceback
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');A=R/'financial-source-check-cases-terminal-review-20261018';P=A/'packet';D=R/'concurrency-increase-main238/financial-review-fidelity';K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def write(name,x):(D/name).write_text(json.dumps(x,indent=2,ensure_ascii=False)+'\n')
def delivered(p):
 for q in [P/'head'/p,P/p,P/'context'/p]:
  if q.is_file():return q
 raise FileNotFoundError(p)
x=json.loads((A/'job/TERMINAL-REVIEW.json').read_text());shutil.copyfile(A/'job/TERMINAL-REVIEW.json',D/'WHOLE-OPINION.json')
issues=[];passed=[]
def check(condition,label,detail=None):
 (passed if condition else issues).append({'check':label,'detail':detail})
task=json.loads((A/'TASK.json').read_text());manifest=json.loads((P/'MANIFEST.json').read_text());src=json.loads((P/'SOURCE-PINS.json').read_text());receipts=json.loads((P/'RECEIPT-PINS.json').read_text());schema=json.loads((P/'REVIEW-RESPONSE.schema.json').read_text())
for name,path in [('manifest',P/'MANIFEST.json'),('prompt',A/'PROMPT.md'),('schema',P/'REVIEW-RESPONSE.schema.json')]:check(sha(path)==task[name+'Sha256'],name+' task hash')
for f in manifest['files']:check(sha(P/f['path'])==f['sha256'] and (P/f['path']).stat().st_size==f['bytes'],'packet member '+f['path'])
for name in ['format','reviewToken','sourceManifestSha256','receiptManifestSha256','head','tree','base']:check(x[name]==schema['properties'][name]['const'],'review constant '+name)
check(set(x)==set(schema['required']),'exact top-level schema keys')
check(x['sourceManifestSha256']==sha(P/'SOURCE-PINS.json'),'review source manifest hash');check(x['receiptManifestSha256']==sha(P/'RECEIPT-PINS.json'),'review receipt manifest hash')
check(set(x['coverage'])=={f['path']for f in src['changed']},'all15 coverage keys')
for f in src['changed']:check(x['coverage'][f['path']]['sha256']==f['sha256'] and x['coverage'][f['path']]['readScope']=='full','coverage '+f['path'])
head=src['head'];base=src['base'];tree=src['tree']
def git(*args):return subprocess.check_output(['git',*args],cwd=K)
check(git('rev-parse',head+'^{tree}').decode().strip()==tree,'exact Git head tree')
check(set(git('diff','--name-only',base,head).decode().splitlines())=={f['path']for f in src['changed']},'exact changed-path set')
for label,commit in [('changed',head),('baseBodies',base)]:
 for f in src[label]:
  q=P/f['fullBody'];raw=git('show',commit+':'+f['path']);check(raw==q.read_bytes() and hashlib.sha256(raw).hexdigest()==f['sha256'],'exact Git body '+f['fullBody'])
for f in src['context']:
 if f.get('deliveryScope')=='full':check(git('show',head+':'+f['path'])==delivered(f['path']).read_bytes(),'exact context '+f['path'])
 else:
  q=P/'context'/f['originalPath'];raw=git('show',head+':'+f['originalPath']);check(raw==q.read_bytes() and hashlib.sha256(raw).hexdigest()==f['originalSha256'],'whole backing context '+f['originalPath'])
for f in receipts['raw']:check(sha(P/f['path'])==f['sha256'] and (P/f['path']).stat().st_size==f['bytes'],'receipt '+f['path'])
check(sha(P/'actual-driver.py')==receipts['driverSha256'],'actual driver hash')
blocks={f['path']:f for f in json.loads((P/'INLINE-BLOCKS.json').read_text())['blocks']};quoteChecks=[]
for axis in ['sourceChecks','resultChecks']:
 check(set(x[axis])==set(schema['properties'][axis]['required']),'required '+axis)
for axis,items in [('sourceChecks',x['sourceChecks']),('resultChecks',x['resultChecks']),('findings',{v['id']:v for v in x['findings']})]:
 for key,val in items.items():
  for i,q in enumerate(val['quotes']):
   raw=delivered(q['path']).read_bytes();text=raw.decode();lines=text.splitlines();flags=[]
   if q['path']not in blocks:flags.append('not a delivered block')
   if sha(delivered(q['path']))!=q['sha256'] or blocks.get(q['path'],{}).get('sha256')!=q['sha256']:flags.append('hash mismatch')
   if q['exactQuote'] not in text:flags.append('literal not in complete delivered file')
   if 'lineStart'in q:
    start,end=q['lineStart'],q['lineEnd']
    if not 1<=start<=end<=len(lines):flags.append(f'line range {start}-{end} outside delivered file of {len(lines)} lines')
    if q['exactQuote']not in '\n'.join(lines[start-1:end]):flags.append('literal not in attributed line range')
   else:
    value=json.loads(text)
    for part in q['jsonPointer'].split('/')[1:]:
     part=part.replace('~1','/').replace('~0','~');value=value[int(part)]if isinstance(value,list)else value[part]
    candidate=value if isinstance(value,str)else json.dumps(value,ensure_ascii=False,indent=2)
    if q['exactQuote'] not in candidate and q['exactQuote']not in json.dumps(value,ensure_ascii=False):flags.append('JSON pointer mismatch')
   locations=[i+1 for i in range(len(lines))if q['exactQuote']in '\n'.join(lines[i:])and not (i+1<len(lines)and q['exactQuote']in '\n'.join(lines[i+1:]))]
   quoteChecks.append({'axis':axis,'key':key,'quoteIndex':i,'path':q['path'],'claimedRange':[q.get('lineStart'),q.get('lineEnd')],'literalPresent':q['exactQuote']in text,'actualQuoteStartLine':locations,'flags':flags})
   if flags:issues.append({'check':'quote '+axis+'/'+key+'/'+str(i),'detail':quoteChecks[-1]})
try:
 runpy.run_path(str(P/'verify_review.py'),run_name='offline_verifier')['verify'](x);verifier={'pass':True}
except Exception as e:verifier={'pass':False,'exception':type(e).__name__,'message':str(e),'traceback':traceback.format_exc()}
result=json.loads((P/'actual-receipts/financial-source-check-tests/RESULT.json').read_text());final=json.loads((P/'actual-receipts/FINAL-RESULT.json').read_text());actual=json.loads((P/'actual-receipts/SOURCE-PINS.json').read_text());selected=json.loads((P/'SELECTED-ACTUAL-SOURCE-PINS.json').read_text())
for n in ['stdout','stderr']:check(sha(P/f'actual-receipts/financial-source-check-tests/{n.upper()}.log')==result[n+'Sha256'],n+' raw hash')
check(result['exitCode']==0 and result['ownedSessionRemaining']==[] and final['existsAfter']==False,'recorded terminal and cleanup')
headcases=json.loads((P/'head/crates/fixtures/check-source/cases.json').read_text());basecases=json.loads((P/'base/crates/fixtures/check-source/cases.json').read_text())
check(headcases[:8]==basecases and len(headcases)==14,'original eight manifest objects retained')
baseh=(P/'base/crates/mori/tests/check_source_dispatch.rs').read_bytes();head_h=(P/'head/crates/mori/tests/check_source_dispatch.rs').read_bytes();check(head_h.startswith(baseh),'original harness byte prefix retained')
caseFacts=[]
for i,c in enumerate(headcases[8:],8):
 source=P/'head/crates/fixtures/check-source'/c['source'];output=P/'head/crates/fixtures/check-source'/c['expectedStdout'];j=json.loads(output.read_text());check(sha(source)==j['source_hash'],c['id']+' source hash');check(c['argv']==['--color','never','check',c['fileLabel'],'--json']and c['humanArgv']==['--color','never','--quiet','check',c['fileLabel']]and c['expectedExit']==0,c['id']+' argv/exit')
 spans=[]
 if 'beta'in c['id']:
  for decl in j['declarations']+j['actions']:
   n=decl['name_span'];sp=decl['span'];check(source.read_bytes()[n['start']:n['end']].decode()==decl['name'],c['id']+' literal name span '+decl['name']);spans.append({'name':decl['name'],'span':sp,'nameSpan':n,'sourceBytes':source.read_bytes()[sp['start']:sp['end']].decode()})
 caseFacts.append({'id':c['id'],'manifestIndex':i,'sourceBytes':source.stat().st_size,'expectedBytes':output.stat().st_size,'sourceSha256':sha(source),'expectedSha256':sha(output),'expected':j,'spanLiteralFacts':spans})
write('MECHANICAL-RESULT.json',{'checkedAtUtc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'wholeOpinionSha256':sha(D/'WHOLE-OPINION.json'),'wholeOpinionBytes':(D/'WHOLE-OPINION.json').stat().st_size,'exactBase':base,'exactHead':head,'exactTree':tree,'passedChecks':passed,'mismatches':issues,'quoteChecks':quoteChecks,'packetVerifier':verifier,'actualSourcePinsCount':len(actual['files']),'selectedSourcePins':selected,'actualTerminalResult':result,'actualFinal':final,'caseFacts':caseFacts,'scope':'mechanical/source-claim fidelity only; no corrected vote, adoption, security judgment, provider/backend attestation or parent closure'})
print(json.dumps({'passed':len(passed),'mismatches':issues,'quotes':len(quoteChecks),'verifier':verifier},indent=2))
