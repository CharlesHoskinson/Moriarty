"""Independent harmless probes of existing runner/store. Never invokes K or live store."""
import hashlib, json, os, pathlib, sqlite3, sys, tempfile, time, uuid
REPO=pathlib.Path('/home/charl/Moriarty')
sys.path.insert(0,str(REPO/'plugins/moriarty-dev/scripts'))
from moriarty_dev import runner,store
E=pathlib.Path(__file__).resolve().parent/'snapshot-envelope-controls-11'
E.mkdir(exist_ok=False)
SOURCE=REPO/'deliverables/language-to-ledger-2026-09-12/k-admission/root-runner-consumer-probes-03.py'
source_paths=[pathlib.Path(__file__).resolve().parent/'candidate-source-11.py',pathlib.Path(__file__).resolve().parent/'snapshot-envelope-child-11.py',pathlib.Path('/usr/bin/bwrap'),REPO/'plugins/moriarty-dev/scripts/moriarty_dev/runner.py',REPO/'plugins/moriarty-dev/scripts/moriarty_dev/store.py',runner.FOREMAN_LAUNCHER,pathlib.Path('/usr/bin/python3.14'),pathlib.Path('/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node'),SOURCE]
source_before={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in source_paths}

def recover(db,identity):
    with sqlite3.connect(db) as conn:
        row=conn.execute('SELECT repository,action_id,candidate,status,receipt_json FROM reservations WHERE id=?',(identity['reservationId'],)).fetchone()
    if row is None: raise ValueError('missing reservation')
    if row[0]!=str(REPO) or row[1]!=identity['actionId'] or row[2]!=identity['candidateHash'] or row[3] not in ('finished','failed'): raise ValueError('row identity/status')
    packet=json.loads(row[4]); receipt=packet['runnerReceipt']
    if any(receipt.get(k)!=v for k,v in identity.items()): raise ValueError('receipt identity')
    if receipt.get('completionAmbiguous') is not False or receipt.get('outputLimitExceeded') is not False or receipt['exitCode']==124 or packet['exitCode']!=receipt['exitCode']: raise ValueError('incomplete result')
    output={}
    for name in ['stdout','stderr']:
        raw=packet[name].encode('utf-8')
        if len(raw)!=receipt['outputBytes'][name] or hashlib.sha256(raw).hexdigest()!=receipt['outputSha256'][name]: raise ValueError('raw byte recovery')
        output[name]=raw
    return output

def live_token(token):
    hits=[]
    for p in pathlib.Path('/proc').glob('[0-9]*/cmdline'):
        try:
            if token.encode() in p.read_bytes(): hits.append(int(p.parent.name))
        except (FileNotFoundError,PermissionError,ProcessLookupError): pass
    return hits

rows=[]
with tempfile.TemporaryDirectory(prefix='moriarty-runner-consumer-') as tmp:
    t=pathlib.Path(tmp); db=t/'test.sqlite3'
    codes={
      'exact':"import os; os.write(1,'stdout é\\n'.encode()); os.write(2,b'error\\n')",
      'exit113':"raise SystemExit(113)",
      'invalidUtf8':"import os; os.write(1,b'\\xff')",
      'timeout':"import time; time.sleep(10)",
      'overflow':"import os; os.write(1,b'x'*4096)",
    }
    # Keep child source literals exact without shell interpolation.
    codes['exact']='import os; os.write(1,'+repr('stdout é\n'.encode())+'); os.write(2,'+repr(b'error\n')+')'
    codes['invalidUtf8']="import os; os.write(1,bytes([255]))"
    token='moriarty-probe-'+uuid.uuid4().hex
    ready=t/'ready'; escaped=t/'escaped'
    descendant="import pathlib,time; pathlib.Path("+repr(str(ready.parent))+").mkdir(parents=True,exist_ok=True); pathlib.Path("+repr(str(ready))+").write_text('ready'); time.sleep(3); pathlib.Path("+repr(str(escaped))+").write_text('survived')"
    codes['detached']="import subprocess,sys,time,pathlib; subprocess.Popen([sys.executable,'-c',"+repr(descendant)+","+repr(token)+"],start_new_session=True,stdin=subprocess.DEVNULL,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL); p=pathlib.Path("+repr(str(ready))+"); deadline=time.monotonic()+2\nwhile not p.exists() and time.monotonic()<deadline: time.sleep(.01)\nassert p.exists()\nprint('DETACHED_READY')"
    for name,code in codes.items():
        action={'id':'harmless-'+name,'candidate':'a'*64,'kind':'reproduce','requirement':'OFFLINE-TEST','capability':'harmless-runner-probe'}
        plan={'action':action,'argv':['/usr/bin/python3.14','-I','-B',str(pathlib.Path(__file__).resolve().parent/'snapshot-envelope-child-11.py'),code], 'launcher':['/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node',str(runner.FOREMAN_LAUNCHER)],'timeoutSeconds':2 if name=='timeout' else 8,'graceSeconds':1,'outputLimitBytes':1024,'chargeId':'test-only-'+name,'assertion':{'id':'test113','exitCode':113,'stdoutSha256':hashlib.sha256(b'').hexdigest()}}
        digest=hashlib.sha256(json.dumps(plan,sort_keys=True,separators=(',',':')).encode()).hexdigest()
        rid=store.reserve(db,str(REPO),action,{},charge_id=plan['chargeId'])
        case_dir=E/name;case_dir.mkdir()
        (case_dir/'plan.json').write_text(json.dumps(plan,indent=2)+'\n')
        (case_dir/'child.py').write_text(code+'\n')
        packet=runner.execute(REPO,plan,digest,rid)
        (case_dir/'actual-packet.json').write_text(json.dumps(packet,indent=2)+'\n')
        store.finish(db,rid,packet)
        with sqlite3.connect(db) as conn:
            stored=conn.execute('SELECT receipt_json FROM reservations WHERE id=?',(rid,)).fetchone()[0]
        (case_dir/'stored-receipt.json').write_text(stored)
        assert json.loads(stored)==packet
        # Save fixture database after every case so failures retain actual persisted evidence.
        with sqlite3.connect(db) as conn, sqlite3.connect(E/'isolated-test-store.sqlite3') as dest:
            conn.backup(dest)
        identity={'actionId':action['id'],'candidateHash':action['candidate'],'runnerDigest':digest,'chargeId':plan['chargeId'],'reservationId':rid}
        accepted=True; failure=None
        try: raw=recover(db,identity)
        except ValueError as ex: accepted=False;failure=str(ex)
        (case_dir/'identity.json').write_text(json.dumps(identity,indent=2)+'\n')
        if accepted:
            for stream,data in raw.items(): (case_dir/(stream+'.recovered.bin')).write_bytes(data)
        (case_dir/'recovery.json').write_text(json.dumps({'accepted':accepted,'rejection':failure},indent=2)+'\n')
        expected=name in ('exact','exit113','detached')
        assert accepted==expected,(name,accepted,failure,packet)
        if name!='timeout': assert 'SNAPSHOT506_VERIFIED_HOME_UID_CWD_STACK_READONLY_FDS' in packet['stderr'],packet
        if name=='exit113': assert packet['exitCode']==113 and packet['runnerReceipt']['behavioralAssertions']
        if name=='timeout': assert packet['exitCode']==124
        if name=='exact': assert raw['stdout']=='stdout é\n'.encode() and raw['stderr'].endswith(b'error\n')
        if name=='detached':
            assert 'DETACHED_READY' in packet['stdout'],packet
            (case_dir/'descendant-observation.json').write_text(json.dumps({'readyBeforeCheck':'DETACHED_READY' in packet['stdout'],'token':token,'escapedBeforeWait':escaped.exists()})+'\n')
            time.sleep(3.2)
            hits=live_token(token)
            (case_dir/'descendant-observation.json').write_text(json.dumps({'readyObserved':'DETACHED_READY' in packet['stdout'],'token':token,'waitSeconds':3.2,'escapedAfterWait':escaped.exists(),'matchingHostProcessesAfterWait':hits})+'\n')
            assert not escaped.exists() and not hits,(escaped.exists(),hits)
        rows.append({'name':name,'acceptedRawEvidence':accepted,'rejection':failure,'runnerReceipt':packet['runnerReceipt'],'detachedSurvived':False if name=='detached' else None})
        for mutation in ['candidateHash','runnerDigest','chargeId','reservationId','actionId']:
            bad=dict(identity);bad[mutation]='mismatch'
            try: recover(db,bad)
            except ValueError: pass
            else: raise AssertionError('identity mismatch accepted '+mutation)
    out={'scope':'Independent actual existing runner and temporary SQLite consumer; harmless Python only; no K/strace/compile/live accounting/store changes','sourceHashes':source_before,'cases':rows,'identityMismatchRejections':len(rows)*5,'actualStorePath':str(store.get_db_path(REPO)),'verdict':'PASS'}
    out['sourceHashesAfter']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in source_paths}
    out['sourceUnchanged']=out['sourceHashes']==out['sourceHashesAfter']
    out['fixtureStorePreserved']=str(E/'isolated-test-store.sqlite3')
    out['apiScope']='Actual runner.execute plus store.reserve/finish on temporary DB; candidate11 actual506-file bwrap argv around harmless Python. Existing recovery predicates. No production run_shim/preflight or admission claim.'
    out['testScriptSHA256']=hashlib.sha256(pathlib.Path(__file__).read_bytes()).hexdigest()
    (E/'result.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps({'verdict':out['verdict'],'cases':len(rows),'identityMismatchRejections':len(rows)*5}))
