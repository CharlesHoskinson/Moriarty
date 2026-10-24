"""Proposed Root composition of existing ordinary capture functions; UNRUN.
Usage after fresh review/Root selection: python3 -B THIS.py ROOT-SELECTION.json
No product imports; no helper main, retries, Cargo or old test execution.
"""
from pathlib import Path
import hashlib,json,os,re,runpy,stat,sys

B=Path(__file__).resolve().parent
load=lambda p:json.loads(Path(p).read_text())
def rawpin(p):
    p=Path(p);b=p.read_bytes()
    return dict(path=str(p),bytes=len(b),sha256=hashlib.sha256(b).hexdigest())
PLAN={'path':str(B/'PROPOSED-FIVE-OPERATIONS.json'),'bytes':22386,'sha256':'f9292729a41e480d42e5a9d30d8bc1e5d22c7a8f82908fe8c28d67d70aada970'}
FLAT={'path':str(B/'STATIC-PREPOST-PINS.json'),'bytes':149442,'sha256':'a5fdf3d0dc5b0e8502cbb3a8c116d2901c4e8ffa36cb03be6719524895eec065'}
assert rawpin(PLAN['path'])==PLAN and rawpin(FLAT['path'])==FLAT
j=load(PLAN['path']);h=j['captureHelper'];assert rawpin(h['path'])==h
api=runpy.run_path(h['path'],run_name='root_fin_carrier_existing_ordinary_functions_only')
pin,check=api['pin'],api['check']
def durable(p,x):
    assert not p.exists(),str(p)
    api['save'](p,x)
    with p.open('rb')as f:os.fsync(f.fileno())
    fd=os.open(p.parent,os.O_RDONLY|os.O_DIRECTORY);os.fsync(fd);os.close(fd)
def inventory():
    ans=[]
    for dr in load(B/'CORE-EXACT-L-DIRECTORY-MEMBERSHIP.json')['directories']:
        d=Path(dr['directory']);assert sorted(p.name for p in d.iterdir())==sorted(x['name']for x in dr['entries'])
        for x in dr['entries']:
            p=d/x['name'];s=p.lstat()
            assert stat.S_ISLNK(s.st_mode)==x['symlink'] and stat.S_ISREG(s.st_mode)==x['regularFile']
            assert oct(stat.S_IMODE(s.st_mode))==x['mode'] and s.st_size==x['size']
            if x['symlink']:assert os.readlink(p)==x['linkTarget']
            if 'qualifiedLibrary'in x:check(x['qualifiedLibrary'])
        ans.append(dict(directory=str(d),members=len(dr['entries']),exact=True))
    return ans
def main(selection_path):
    selection=load(selection_path)
    assert selection['RootSelectedAllFive'] is True and selection['RootReadFullOrdinaryReview'] is True
    assert selection['ordinaryReviewEligibleAllFive'] is True
    assert selection['proposal']==PLAN and selection['flat']==FLAT and selection['composition']==pin(__file__)
    assert selection['helper']==h
    rv=selection['ordinaryReview'];check(rv)
    ops=j['operations'];assert len(ops)==5
    assert selection['selectedIds']==[o['id']for o in ops]
    controls=[pin(selection_path),pin(__file__),PLAN,FLAT,h,rv]
    flat=load(FLAT['path'])['files'];assert len(flat)==457
    closures=load(B/'EXPECTED-DEP-MEMBERSHIP.json')
    expected=load(B/'EXPECTED-SIXTEEN-TEST-NAMES.json');names=expected['exactNames'];prefix=expected['prefix']
    assert len(names)==len(set(names))==16 and names==sorted(names)
    source=load(B/'SOURCE-COMPOSITION.json')
    # Preserve exact current canonical source preimages without requiring any PW write.
    canonical=[r['currentPW'] for r in source['all187PWMembersEqualCurrentTypedInputSnapshot']]
    generated=[];records=[];fullnames=None
    def checks():
        s=api['static_check'](flat+controls+canonical+generated)
        d=inventory()
        assert s['all_match'],'fail-stop changed static/control/canonical/generated input'
        return dict(static=s,LDirectories=d,staticCount=len(flat),canonicalPreimages=len(canonical),generated=len(generated))
    checks()
    W=Path(ops[0]['cwd']);A=B/'ordinary-QA/actual-five-01'
    for p in [W,A]:
        assert not p.exists() and not p.is_symlink(),str(p)
        assert not any(x.is_symlink() for x in p.parents)
    for o in ops:
        assert o['wallSeconds']==30 and o['keptBytesPerStream']==131072
        for p in o['outputs']:assert not Path(p).exists()
    W.mkdir();(W/'build').mkdir()
    for n in ['normal','cfgtest','test']:(W/'tmp'/n).mkdir(parents=True)
    A.mkdir(parents=True);durable(A/'ROOT-SELECTION.json',selection)
    try:
        for ix,o in enumerate(ops):
            phase=A/o['id'];phase.mkdir()
            durable(phase/'PRECHECK.json',checks())
            raw=api['capture'](o,phase)
            # Retain exact raw return before parsing, postchecks or acceptance.
            durable(phase/'RESULT.json',raw)
            for n in ['COMMAND.json','stdout.bin','stderr.bin','RESULT.json']:
                with (phase/n).open('rb')as f:os.fsync(f.fileno())
            durable(phase/'POSTCHECK.json',checks())
            assert load(phase/'COMMAND.json')==o
            assert raw['actual_exit']==0 and raw['controller_failure'] is None
            assert len(raw['streams'])==2
            assert {Path(x['path']).name for x in raw['streams']}=={'stdout.bin','stderr.bin'}
            for row in raw['streams']:
                check(row)
                assert row['full_stream_retained'] and row['bytes']==row['bytes_seen']==row['bytes_retained']
            out=(phase/'stdout.bin').read_bytes();err=(phase/'stderr.bin').read_bytes()
            accepted={'id':o['id'],'rawResult':pin(phase/'RESULT.json'),'accepted':True}
            if ix<2:
                assert out==b''
                products=[]
                for path in o['outputs']:
                    p=Path(path);assert p.is_file() and not p.is_symlink() and p.stat().st_size>0
                    products.append(pin(p))
                # Product hashes exist before dependency parsing; failure retains them.
                durable(phase/'ACTUAL-GENERATED-PINS.json',{'files':products})
                key='normal' if ix==0 else 'cfgTest';cl=closures[key]
                allowed={str(Path(x['path']).resolve(strict=True)):x for x in cl['files']}
                assert len(allowed)==cl['count']==(116 if ix==0 else 186)
                deps=api['dep_inputs'](o['outputs'][1],allowed)
                actual={str(Path(x['selected_binding']).resolve(strict=True))for x in deps}
                assert actual==set(allowed)
                assert any(x['path'].endswith('/shared/fin_carrier.rs')for x in deps)
                durable(phase/'EXACT-DEP-MEMBERSHIP.json',dict(files=deps,canonicalCount=len(actual),expected=cl['count'],exact=True))
                for n,p in enumerate(o['outputs']):os.chmod(p,0o500 if ix==1 and n==0 else 0o400)
                generated.extend(products)
                accepted.update(exactCanonicalDependencies=len(actual),generated=products,stderrFullRetained=True)
            else:
                assert err==b''
                if ix in [2,3]:
                    # libtest terse list consists solely of sorted name: test lines.
                    text=out.decode('utf-8',errors='strict')
                    lines=text.splitlines();assert lines and all(lines)
                    matches=[re.fullmatch(r'([A-Za-z_][A-Za-z0-9_:]*): test',s)for s in lines]
                    assert all(matches),'unexpected benchmark, summary or list diagnostic'
                    seen=[m.group(1)for m in matches]
                    assert seen==sorted(set(seen))
                    assert text==''.join(n+': test\n'for n in seen)
                    if ix==2:
                        assert [n for n in seen if prefix in n]==names
                        fullnames=seen
                    else:assert seen==names
                    membership={'names':seen,'count':len(seen),'exactProjection':names,'sourceNames':pin(B/'EXPECTED-SIXTEEN-TEST-NAMES.json')}
                    durable(phase/'FROZEN-ACTUAL-MEMBERSHIP.json',membership)
                    generated.extend([pin(phase/'stdout.bin'),pin(phase/'FROZEN-ACTUAL-MEMBERSHIP.json')])
                    accepted['members']=len(seen)
                else:
                    assert fullnames is not None
                    filtered=len(fullnames)-16;assert filtered>=0
                    body='\nrunning 16 tests\n'+''.join('test '+n+' ... ok\n'for n in names)
                    pattern=re.escape(body)+r'\ntest result: ok\. 16 passed; 0 failed; 0 ignored; 0 measured; '+str(filtered)+r' filtered out; finished in \d+\.\d+s\n\n'
                    assert re.fullmatch(pattern,out.decode('utf-8',errors='strict')),'not exact sixteen-test whole pretty envelope'
                    accepted.update(exactNames=names,passed=16,failed=0,ignored=0,measured=0,filteredOut=filtered,fullListCount=len(fullnames))
            durable(phase/'ACCEPTANCE.json',accepted)
            records.append(accepted)
            durable(phase/'FINAL-PIN-CHECK.json',checks())
        durable(A/'ROOT-RESULT.json',dict(allFivePassed=True,phases=records,generated=generated,sourceMembers=188,scope='Normal Core integration and only16carrier source-exposed tests; no financial owner/public/admission/genesis/runtime/fullfinance/EARS/native acceptance',retry=False))
    except BaseException as e:
        durable(A/'FAILURE.json',dict(error=type(e).__name__+': '+str(e),phasesCompleted=records,retry=False,remainingUnrun=True))
        raise
if __name__=='__main__':main(sys.argv[1])
