from pathlib import Path
import json,hashlib,os,re,runpy,datetime
D=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/batch22-parametric-literal-reference-current-source-20261019')
H=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-execution-preparation/revision03-tail-four/run_tail_four.py')
assert hashlib.sha256(H.read_bytes()).hexdigest()=='713119d9400f82d559967e4296313cec8cde7125dc91a74152500577ced00626'
api=runpy.run_path(str(H),run_name='capture_only')
plan=json.loads((D/'VALIDATION-PROPOSAL.json').read_bytes())
flat=json.loads((D/'ROOT-FLAT-PINS.proposed.json').read_bytes())
assert len(flat['files'])==4675 and len(flat['symlinks'])==3
review=D/'actual-fresh-opus-source-operation-review-01'
assert json.loads((review/'RESULT.json').read_bytes())['providerIsError'] is False
assert hashlib.sha256((review/'FINAL-RESPONSE.txt').read_bytes()).hexdigest()=='15ab26d65aec5a6193a938d0765c8df323a4f9c5f83e06c91edee3b7b5984de8'
def check():
    result=api['static_check'](flat['files'])
    result['symlinks']=[{'path':x['path'],'match':Path(x['path']).is_symlink() and os.readlink(x['path'])==x['target'] and str(Path(x['path']).resolve(strict=True))==x['resolved'] and Path(x['path']).is_dir()==x['isDirectory']} for x in flat['symlinks']]
    result['exactSourceDirectoryMatch']=sorted(x.name for x in (D/'source').iterdir())==sorted(plan['sourceExactEntries'])
    result['rawChecks']=len(result['files'])+len(result['symlinks'])
    result['all_match']=result['all_match'] and all(x['match'] for x in result['symlinks']) and result['exactSourceDirectoryMatch']
    return result
observed=[]
for i,op in enumerate(plan['operations']):
    phase=D/('actual-'+op['id']);phase.mkdir(mode=0o700,exist_ok=False)
    before=check();api['save'](phase/'ROOT-PRECHECK.json',before);assert before['all_match']
    actualop=dict(op,keptBytesPerStream=4194304)
    result=api['capture'](actualop,phase)
    api['save'](phase/'RESULT.json',result)
    after=check();api['save'](phase/'ROOT-POSTCHECK.json',after)
    assert after['all_match']
    assert result['actual_exit']==0 and result['controller_failure'] is None
    for stream in result['streams']:
        assert stream['full_stream_retained'] is True and stream['bytes_seen']==stream['bytes_retained']==stream['bytes']
    out=(phase/'stdout.bin').read_bytes();err=(phase/'stderr.bin').read_bytes()
    assert err==b''
    assert not re.search(rb'QNT\d+|\[DEBUG\]|error:',out,re.I)
    if i<3:assert out==b''
    else:
        text=out.decode()
        names=re.findall(r'^\s*ok (\w+) passed 1 test\(s\)\s*$',text,re.M)
        assert names==op['expected']['exactPassingNames'] and len(names)==8
        assert len(re.findall(r'^\s*ok ',text,re.M))==8
        assert re.search(r'^\s*8 passing \(\d+ms\)\s*$',text,re.M)
        assert not re.search(r'failed|ignored|pending|QNT|DEBUG|error:|truncat',text,re.I)
        other=[]
        for line in text.splitlines():
            s=line.strip()
            if not s:continue
            if re.fullmatch(r'ok (\w+) passed 1 test\(s\)',s):continue
            if re.fullmatch(r'8 passing \(\d+ms\)',s):continue
            other.append(s)
        assert other==['literalParametricImagesTest'],other
        api['save'](phase/'ROOT-EXACT-TEST-PARSER.json',{'names':names,'moduleHeader':other,'count':8,'stderrBytes':0,'parserProvenance':'B21 prepare_result_publication.py regex and refusal conditions, parameter count8; additional strict whitelist of all other lines; original B21 execution cell unavailable.'})
    observed.append({'id':op['id'],'exit':result['actual_exit'],'seconds':result['elapsed_seconds'],'fullStreams':result['streams']})
    print(json.dumps(observed[-1]),flush=True)
api['save'](D/'ROOT-ACTUAL-FOUR-OPERATIONS-OUTCOME.json',{'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'PASS_FOUR_OPERATIONS_EIGHT_NEW_TESTS','operations':observed,'sourceAndExpectationsUnchanged':True,'sourceDirectoryExactThreeMembers':True,'pinChecksPerPreAndPost':4678,'sourceReview':str(review/'FINAL-RESPONSE.txt'),'scope':'Parametric finite model; same-author source-exposed expectations; D1-D6 strengthening/negativecoverage gaps retained; no actual Rust/crosslayer/native/recursive/target/EARS evidence.'})
