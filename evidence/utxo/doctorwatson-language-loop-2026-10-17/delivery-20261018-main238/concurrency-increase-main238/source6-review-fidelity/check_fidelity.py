"""Only offline public JSON/hash/text checks; never imports candidate code."""
from pathlib import Path
import hashlib, json, re

R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
P = R/'source6-max-rebate-read-once-review-preparation-20261018'
K = P/'packet'
O = Path(__file__).resolve().parent
digest = lambda b: hashlib.sha256(b).hexdigest()
raw=(P/'job/TERMINAL-REVIEW.json').read_bytes()
assert digest(raw)=='f8ff6eb2e18c9865e82f562cca8888fbdc14b06518f9b0fb1d9e647107fa23ce'
x=json.loads(raw)
(O/'TERMINAL-REVIEW.original.json').write_bytes(raw)
primary=(K/'PRIMARY.md').read_bytes(); text=primary.decode(); lines=text.splitlines()
m=json.loads((K/'PRIMARY-REPRESENTATION-MAP.json').read_text()); entries={e['path']:e for e in m['entries']}
assert len(primary)==207559 and digest(primary)==x['reviewedSubject']['primarySha256'] and len(lines)==2897
representations=[]
for path,e in entries.items():
    b=(K/e['physicalPacketPath']).read_bytes()
    assert len(b)==e['sourceBytes'] and digest(b)==e['sourceSha256']
    if e['kind']=='exact-raw-byte-alias':
        target=entries[e['aliasOf']]
        assert b==(K/target['physicalPacketPath']).read_bytes()
        assert e['aliasTargetRepresentation']['bodyByteRange']==target['bodyByteRange']
    else:
        a,z=e['bodyByteRange']; body=primary[a:z]
        assert (json.loads(b)==json.loads(body)) if e['kind']=='whole-json-value' else body==b
    representations.append({'path':path,'kind':e['kind'],'sha256':digest(b),'bytes':len(b),'completeRepresentationMatches':True})
candidates=[]
for path,c in x['candidateReadScopes'].items():
    e=entries['changed/'+path]
    assert c['sourceSha256']==e['sourceSha256']
    assert c['representation']['sectionLineRange']==e['sectionLineRange']
    assert c['representation']['bodyByteRange']==e['bodyByteRange']
    assert c['representation']['kind']==e['kind'] and c['representation']['sourceBytes']==e['sourceBytes']
    assert c['physicalFileReadClaimed'] is False
    ranges=c['actualPrimaryRangesRead']
    assert any(v['unit']=='primary-byte-half-open' and [v['start'],v['end']]==e['bodyByteRange'] for v in ranges)
    candidates.append({'path':path,'sourceSha256':c['sourceSha256'],'representationMatches':True,'readAttestation':'Reviewer-declared; no independent cognition/tool-read attestation.'})
assert len(candidates)==25 and len(x['predicates'])==8
quotes=[]
def walk(obj,loc=''):
    if isinstance(obj,dict):
        if isinstance(obj.get('quote'),str) and isinstance(obj.get('reference'),str):
            q=obj['quote']; ref=obj['reference']; match=re.fullmatch(r'PRIMARY.md:(\d+)(?:-(\d+))?(?:#(.*))?',ref)
            assert match,ref
            start=int(match[1]); end=int(match[2] or start)
            claimed='\n'.join(lines[start-1:end]); positions=[text[:v.start()].count('\n')+1 for v in re.finditer(re.escape(q),text)]
            norm=lambda s:'\n'.join(v.strip() for v in s.splitlines())
            quotes.append({'location':loc,'reference':ref,'representedPath':obj.get('representedPath'),'quoteInClaimedLines':q in claimed,'quoteInClaimedLinesIgnoringIndent':norm(q) in norm(claimed),'exactPrimaryStartLines':positions,'pointerQualification':'JSON pointer may name an anchor within a larger quoted compact-line fragment; not necessarily the complete selected value.' if match[3] else None})
        for k,v in obj.items():walk(v,loc+'/'+str(k))
    elif isinstance(obj,list):
        for i,v in enumerate(obj):walk(v,loc+'/'+str(i))
walk(x)
receipts={}
for group in ['rust-build','node-consumers']:
    base=K/'original/actual-results'/group; result=json.loads((base/'RESULT.json').read_text())
    assert result['exitCode']==0
    assert digest((base/'STDOUT.log').read_bytes())==result['stdoutSha256']
    assert digest((base/'STDERR.log').read_bytes())==result['stderrSha256']
    receipts[group]=result
node=(K/'original/actual-results/node-consumers/STDOUT.log').read_text()
assert 'ℹ tests 1\n' in node and 'ℹ pass 1\n' in node and 'ℹ fail 0\n' in node and 'ℹ skipped 0\n' in node
observations=K/'original/actual-results/observations/pay_rebate'
for process,stdout,stderr in [('process.json','stdout.source6','stderr.txt'),('check.process.json','check.stdout.json','check.stderr.txt')]:
    v=json.loads((observations/process).read_text())
    assert v['status']==0 and v['signal'] is None and v['error'] is None
    assert digest((observations/stdout).read_bytes())==v['stdoutSha256']
    assert digest((observations/stderr).read_bytes())==v['stderrSha256'] and (observations/stderr).read_bytes()==b''
    receipts[process]={'stdoutBytes':len((observations/stdout).read_bytes()),'status':0,'stderrBytes':0}
actual_bytes=(observations/'actual-consumer.json').read_bytes();actual=json.loads(actual_bytes)
assert len(actual_bytes)==9626
assert digest((K/'original/ORIGINAL-PACKET-MANIFEST.json').read_bytes())==x['reviewedSubject']['originalPacketManifestSha256']
assert set(x['predicates'])==set(json.loads((K/'REVIEW-PREDICATES.json').read_text()))
intent=actual['lowered']['intent']; selected=['sourceProfile','programId','sourceHash','policyDigest','keyRef','domain','asset','signer','nonce','preHead','notBefore','notAfter','grossCap','feeCap','netFloor','kind','recipient','feeRecipient','amount','fee']
fixture=K/'original/changed/experiments/moriarty-language/regressions/fixtures/source6-max-rebate/oracle'
expected=json.loads((fixture/'EXPECTED-SEMANTICS.json').read_text())['cases'][0]
ast=json.loads((fixture/'pay_rebate.source6.ast.expected.json').read_text())
assert actual['lowered']['ast']==ast==actual['outcome']['ast']
assert actual['lowered']['submittedEffects']==expected['core5SemanticOrderedEffects']==actual['outcome']['candidate']['effects']
assert ast['submitted']['effects']==expected['source6OrderedEffects'] and len(ast['submitted']['effects'])==6
post=expected['conditionalClosingFinancialProjection'];frame=lambda s:{k:s[k] for k in ['domain','asset','balances','allowances']}
assert frame(actual['lowered']['state'])==expected['openingFinancialFrame']
assert frame(actual['outcome']['candidate']['candidatePost'])==frame(post)
prefix=json.loads((fixture/'pay_rebate.effect-prefix-financial-states.expected.json').read_text())
assert len(prefix['prefixes'])==7
prior={}
for name in ['TERMINAL-EXPORT-RECEIPT.json','CLEANUP.json']:
    b=(P/'preserved-first-attempt'/name).read_bytes()
    (O/('PREVIOUS-'+name)).write_bytes(b)
    prior[name]={'sha256':digest(b),'publicValue':json.loads(b)}
dump={'reviewSha256':digest(raw),'primarySha256':digest(primary),'primaryBytes':len(primary),'primaryLines':len(lines),'representations':representations,'candidateHashesAndRepresentations':candidates,'quoteChecks':quotes,'quoteCount':len(quotes),'predicateCount':8,'actualReceipts':receipts,'actualJsonBytes':len(actual_bytes),'actualSerializedIntentKeys':list(intent),'selectedAssertedIntentKeys':selected,'intentFieldsOutsideSelectedAssertions':sorted(set(intent)-set(selected)),'oraclePrefixFrames':7,'actualIntermediatePrefixRuntimeTracesEstablished':False,'previousPublicFailurePreserved':prior,'actualBackendIdentityAttested':False,'scope':'Offline source/result fidelity only; original whole opinion unchanged. No provider/product/native execution or security judgment/adoption/parent closure.'}
(O/'MECHANICAL-CHECKS.json').write_text(json.dumps(dump,indent=2)+'\n')
corrections=[q for q in quotes if not q['quoteInClaimedLines']]
(O/'QUOTE-QUALIFICATIONS.json').write_text(json.dumps(corrections,indent=2)+'\n')
print(json.dumps({'candidateCount':len(candidates),'representations':len(representations),'quotes':len(quotes),'quoteIssues':corrections,'intentKeys':len(intent),'selectedIntentKeys':len(selected)},indent=2))
