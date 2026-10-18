from pathlib import Path
import hashlib,json,re,shutil,subprocess,datetime,difflib
import jsonschema
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017');P=R/'inspect-header-terminal-review-20261018/packet';D=R/'concurrency-increase-main238/inspect-review-fidelity';O=P.parent/'job/TERMINAL-REVIEW.json';W=Path('/home/charl/Moriarty/.worktrees/utxo')
sha=lambda b:hashlib.sha256(b).hexdigest()
def write(n,x):(D/n).write_text(json.dumps(x,indent=2,ensure_ascii=False)+'\n')
raw=O.read_bytes();shutil.copyfile(O,D/'WHOLE-OPINION.json');x=json.loads(raw);schema=json.loads((P/'REVIEW-RESPONSE.schema.json').read_text());mandatory=json.loads((P/'MANDATORY-INPUTS.json').read_text());cand=json.loads((P/'CANDIDATE.json').read_text());full=(P/'FULL-INPUT.md').read_bytes();passed=[];failed=[]
def check(ok,name,detail=None):(passed if ok else failed).append({'check':name,'detail':detail})
errors=[{'path':'/'+('/'.join(map(str,e.absolute_path))),'message':e.message}for e in jsonschema.Draft202012Validator(schema).iter_errors(x)];check(not errors,'whole schema shape',errors)
check(x['reviewToken']==cand['reviewToken'],'direct review token');man=json.loads((P/'PACKET-MANIFEST.json').read_text())
check(sha(full)==man['fullInputSha256']and len(full)==man['fullInputBytes'],'full mandatory input identity')
for mname in ['MANIFEST.json','PACKET-MANIFEST.json']:
 for f in json.loads((P/mname).read_text())['files']:
  p=(P/f['path']).resolve();assert p.is_relative_to(P.resolve());check(sha(p.read_bytes())==f['sha256'],'manifest '+mname+'/'+f['path'])
blocks=[]
for f in mandatory['files']:
 path=f['path'];data=(P/path).read_bytes();check(sha(data)==f['sha256']and len(data)==f['bytes'],'mandatory body '+path)
 marker=f"BEGIN FILE {path} | SHA256 {f['sha256']} | BYTES {f['bytes']}\n".encode();is_preamble=path=='REVIEW-REQUEST.md';check(full.startswith(data) if is_preamble else full.count(marker)==1,'mandatory inline marker or complete preamble '+path);a=0 if is_preamble else full.index(marker)+len(marker);check(full[a:a+f['bytes']]==data,'mandatory complete inline bytes '+path);blocks.append({'path':path,'sha256':sha(data),'bytes':len(data),'fullInputBodyLineStart':full[:a].count(b'\n')+1,'fullInputBodyLineEnd':full[:a].count(b'\n')+len(data.splitlines())})
check(set(x['mandatoryInputCoverage']['filesReadCompletely'])=={f['path']for f in mandatory['files']},'exact141 declared mandatory read set');check(len(mandatory['files'])==141,'141 mandatory files');check(not x['mandatoryInputCoverage']['unreadMandatoryFiles']and x['mandatoryInputCoverage']['entireInlineInputRead'],'complete read declaration only')
quotes=[]
for cov in x['sourceAssessment']['changedPathCoverage']:
 p=P/'source'/cov['path'];data=p.read_bytes();check(sha(data)==cov['sha256'],'changed source hash '+cov['path'])
 for i,q in enumerate(cov['quoteAnchors']):
  lines=data.decode().splitlines();start=q['originalLine'];n=len(q['exactQuote'].splitlines());actual='\n'.join(lines[start-1:start+n-1]);loc=[i+1 for i in range(len(lines))if '\n'.join(lines[i:i+n])==q['exactQuote']];quotes.append({'axis':'source','path':cov['path'],'quoteIndex':i,'sha256':sha(data),'claimedStart':start,'exactQuote':q['exactQuote'],'actualAttributedText':actual,'actualStartLines':loc,'literalAnywhere':q['exactQuote']in data.decode(),'coordinateExact':actual==q['exactQuote']})
for phase in x['actualResultAssessment']['phaseCoverage']:
 for i,q in enumerate(phase['quoteAnchors']):
  data=(P/q['packetPath']).read_bytes();lines=data.decode().splitlines();n=len(q['exactQuote'].splitlines());loc=[i+1 for i in range(len(lines))if '\n'.join(lines[i:i+n])==q['exactQuote']];quotes.append({'axis':'phase','phase':phase['phase'],'quoteIndex':i,'path':q['packetPath'],'sha256':sha(data),'exactQuote':q['exactQuote'],'actualStartLines':loc,'literalAnywhere':q['exactQuote']in data.decode(),'coordinateExact':None})
# Exact source and historical formatting deltas; no product parsing or execution.
old=(P/'actual/INSPECT-BEFORE.rs').read_bytes();new=(P/'source/crates/mori/src/commands/inspect.rs').read_bytes();a=b'mori_core::language(&source).is_ok_and(|language| language == "moriarty/2")';b=b'mori_core::language(&source).map_err(super::core_io::operation)? == "moriarty/2"'
check(old.replace(a,b,1)==new,'sole first source-only production probe replacement');check(old.splitlines(keepends=True)[46:]==new.splitlines(keepends=True)[46:],'entire native block and suffix byte-identical');check(new==(P/'actual/INSPECT-AFTER.rs').read_bytes(),'candidate inspect equals actual AFTER')
told=(P/'actual/AUTHOR-TEST-BEFORE-FORMAT.rs').read_bytes();tnew=(P/'source/crates/mori/tests/inspect_header_dispatch.rs').read_bytes();arr=b'        let args = [\n            "--color",\n            "never",\n            "inspect",\n            "header.mori",\n        ];';one=b'        let args = ["--color", "never", "inspect", "header.mori"];'
check(told.count(arr)==1 and told.replace(arr,one)==tnew,'sole6line to1line array formatting delta');check(tnew==(P/'actual/TEST-AFTER-FORMAT.rs').read_bytes(),'test actual/formatted source same');check(told.split(b'struct Project')[0]==tnew.split(b'struct Project')[0],'CASES source/error/stderr literals byte-identical');check(told.split(arr)[1]==tnew.split(one)[1],'all post-array test logic byte-identical')
final=json.loads((P/'actual/FINAL-RESULT.json').read_text());phases=[]
for ph in ['red','green','core1-positive','core2-positive']:
 p=P/'actual'/ph;s=json.loads((p/'START.json').read_text());r=json.loads((p/'RESULT.json').read_text());c=json.loads((p/'CHILD.json').read_text());check(r in final['results'],'phase in complete FINAL '+ph)
 for k in ['stdout','stderr']:check(sha((p/(k.upper()+'.log')).read_bytes())==r[k+'Sha256'],'complete phase stream hash '+ph+'/'+k)
 check(r['exitCode']==(101 if ph=='red'else 0)and r['ownedSessionRemaining']==[],'phase exit and owned session '+ph);check('--no-default-features'in s['argv']and '--exact'in s['argv']and '--test-threads=1'in s['argv'],'phase ordinary selection '+ph)
 phases.append({'phase':ph,'start':s,'result':r,'child':c,'stdout':(p/'STDOUT.log').read_text(),'stderr':(p/'STDERR.log').read_text()})
check(final['allFourCommandsCompleted']and not final['existsAfter'],'recorded four commands/owned root removed')
expect=json.loads((P/'actual/EXPECTATIONS.json').read_text());rawfacts=[]
for ph in ['red','green']:
 for case in expect['cases']:
  p=P/'actual'/ph/'public-cli'/case['name'];producer=json.loads((p/'producer-error.json').read_text());process=json.loads((p/'process.json').read_text());check((p/'header.mori').read_bytes()==case['source'].encode(),'literal header '+ph+'/'+case['name']);check(producer==case['error'],'complete producer DTO '+ph+'/'+case['name']);check(process['status']==case['exitCode']==1 and (p/'stdout.bin').read_bytes()==b'','exit and entire empty stdout '+ph+'/'+case['name']);check(((p/'stderr.txt').read_bytes()==case['stderr'].encode())==(ph=='green'),'full human stream red/green predicate '+ph+'/'+case['name'])
  orig=P/'original-author/expectations'/case['name'];check(json.loads((orig/'producer-error.json').read_text())==producer,'original frozen producer JSON object '+ph+'/'+case['name']);check((orig/'stderr.txt').read_bytes()==case['stderr'].encode(),'original frozen stderr '+ph+'/'+case['name']);rawfacts.append({'phase':ph,'case':case['name'],'producer':producer,'process':process,'artifacts':[{'name':f.name,'bytes':f.stat().st_size,'sha256':sha(f.read_bytes())}for f in sorted(p.iterdir())],'stderrLiteral':(p/'stderr.txt').read_text(),'producerFrozenRawBytesEqual':(orig/'producer-error.json').read_bytes()==(p/'producer-error.json').read_bytes(),'producerFrozenJsonObjectEqual':True})
# Check every digest repeated in each raw-capture prose against that case's exact public files.
for row in x['actualResultAssessment']['capturedProcessCoverage']:
 p=P/'actual'/row['phase']/'public-cli'/row['case'];digests={sha(f.read_bytes())for f in p.iterdir()if f.is_file()};prose=re.findall(r'(?<![0-9a-f])[0-9a-f]{64}(?![0-9a-f])',row['scopeAssessment']);check(all(d in digests for d in prose),'raw capture prose digests '+row['phase']+'/'+row['case'])
# Explicit public Git object reads only: bind903 recorded after hashes, parent before hashes, and exact2changed paths.
before=json.loads((P/'actual/SOURCE-PINS-BEFORE.json').read_text());after=json.loads((P/'actual/SOURCE-PINS-AFTER.json').read_text());post=json.loads((P/'actual/POST-COMMIT-BINDING.json').read_text());check(len(before['files'])==len(after['files'])==903,'903 before/after source pins');bm={r['path']:r['sha256']for r in before['files']};am={r['path']:r['sha256']for r in after['files']};check([k for k in am if bm[k]!=am[k]]==['crates/mori/src/commands/inspect.rs'],'one between-phase pin mutation')
def git(*args,input=None):return subprocess.check_output(['git',*args],cwd=W,input=input)
check(git('rev-parse',cand['head']+'^{tree}').decode().strip()==cand['tree'],'candidate Git tree');check(git('show','-s','--format=%P',cand['head']).decode().strip()==cand['parent'],'candidate Git parent');check(set(git('diff','--name-only',cand['parent'],cand['head']).decode().splitlines())=={r['path']for r in cand['changedPaths']},'exact2 candidate Git changed paths')
def batchpins(commit,rows):
 data=git('cat-file','--batch',input=''.join(commit+':'+r['path']+'\n'for r in rows).encode());at=0
 for r in rows:
  e=data.index(b'\n',at);header=data[at:e].decode().split();assert header[1]=='blob',r['path'];n=int(header[2]);b=data[e+1:e+1+n];check(sha(b)==r['sha256'],'pinned public Git blob '+commit[:8]+'/'+r['path']);at=e+n+2
 assert at==len(data)
batchpins(cand['head'],after['files']);batchpins(cand['parent'],[r for r in before['files']if r['path']!='crates/mori/tests/inspect_header_dispatch.rs']);check(bm['crates/mori/tests/inspect_header_dispatch.rs']==sha(tnew),'untracked added test pinned prephase')
# Exact source/log coordinate prose qualifications, rather than assuming endpoint+one convention.
coord={'fullInputBytes':len(full),'fullInputSplitLines':len(full.splitlines()),'fullInputNewlineCount':full.count(b'\n'),'inspectCompleteLines':len(new.splitlines()),'testCompleteLines':len(tnew.splitlines()),'blocks':blocks}
write('MECHANICAL-RESULT.json',{'completedAtUtc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'wholeOpinionSha256':sha(raw),'wholeOpinionBytes':len(raw),'schemaErrors':errors,'passedChecks':passed,'otherFailures':failed,'quoteChecks':quotes,'quoteFailures':[q for q in quotes if q['literalAnywhere']is False or q['coordinateExact']is False],'fourPhases':phases,'sixCaptureFacts':rawfacts,'coordinateFacts':coord,'beforeAfterPinCounts':[len(before['files']),len(after['files'])],'gitBinding':'903head blobs +902parent blobs verified; new test pinned frozen beforephase and committed after; no Git mutations','exactCandidate':cand,'postCommitBinding':post,'declarationsNotReadAttestations':True,'subjectExecutionCalls':0,'providerCalls':0,'nativeSdkSecurityCalls':0,'sharedWrites':0,'privateCaptureReads':False})
print(json.dumps({'opinionSha256':sha(raw),'passedChecks':len(passed),'otherFailures':failed,'quotes':len(quotes),'quoteFailures':[{'axis':q['axis'],'path':q['path'],'start':q.get('claimedStart'),'found':q['actualStartLines']}for q in quotes if q['literalAnywhere']is False or q['coordinateExact']is False],'fullInputLines':len(full.splitlines()),'inspectLines':len(new.splitlines())},indent=2))
