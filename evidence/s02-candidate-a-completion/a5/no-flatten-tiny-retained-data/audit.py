"""Finite offline audit of one tiny run and two recorded interpretations."""
import sys
if sys.flags.optimize:
    raise RuntimeError('Archive audit requires optimization disabled')
import ast
import hashlib
import io
import json
import math
from pathlib import Path, PurePosixPath
import re
import shlex
import tarfile

HERE = Path(__file__).resolve().parent
ROOT = '/home/charl/Moriarty/.worktrees/s01-audit-start'
S = ROOT+'/.superpowers/sdd/'
Q = '/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/'
META = S+'a5-no-flatten-diagnostic-20260906/'
INNER = S+'a4-producer-receipts/a5-noflat-tiny-20260906/'
OLD = S+'a5-factoring-receipts/'
STORE = OLD+'tool-store/'
PROPOSAL = S+'a5-no-flatten-tiny-package-proposal-20260906.md'
BASE = S+'a5-no-flatten-tiny-independent-index-20260906.json'
ADMISSION = META+'root-admission-tiny-retained-type-application.json'
PLAN = ROOT+'/docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-diagnostic.md'
FENCE = chr(96)*3
SUPPORT = ['README.md','build_archive.py','audit.py','adopted-proposal-sha256.txt']
HEAD = '2a33b45f647c4ada426ff840bc9033cbbe913bfa'
NODE = '/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node'
CLI = Q+'dist/src/cli.js'
PYTHON = '/home/charl/Moriarty/.venv/bin/python'
RH = ROOT+'/evidence/s02-candidate-a-completion/a4/native-resources/runner.py'
HELPER = ROOT+'/scripts/run_s02_candidate_a_factoring_pilot.py'
RECORDER = ROOT+'/scripts/record_s02_candidate_a_integrated.py'
EXTERNAL = [
    {'path':STORE+'runtime.tar.gz','sha256':'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c'},
    {'path':S+'a4-checker-task1-receipts/python-environment.tar.gz','sha256':'7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac'}]

def need(ok, message):
    if not ok:
        raise ValueError(message)

def sha(b):
    return hashlib.sha256(b).hexdigest()

def unique(pairs):
    result = {}
    for k,v in pairs:
        need(k not in result,'duplicate JSON key');result[k] = v
    return result

def bad(value):
    raise ValueError('nonfinite JSON constant: '+value)

def finite(value):
    result = float(value);need(math.isfinite(result),'nonfinite exponent')
    return result

def decode(raw, limit=128*1024*1024):
    need(len(raw)<=limit,'JSON bound')
    return json.loads(raw.decode('utf-8'),object_pairs_hook=unique,parse_constant=bad,parse_float=finite)

def safe_name(name):
    p = PurePosixPath(name)
    need(name and not p.is_absolute() and str(p)==name and '..' not in p.parts and
         all(part not in ('','.') for part in name.split('/')),'noncanonical archive path')
    return name

def original_name(path):
    need(path.startswith('/') and not path.startswith('//'),'original absolute path')
    safe_name(path[1:])
    return path[1:]

def archive(raw, count):
    need(len(raw)<=64*1024*1024,'compressed archive bound')
    result,total = {},0
    with tarfile.open(fileobj=io.BytesIO(raw),mode='r:gz') as tar:
        for member in tar:
            name = safe_name(member.name)
            need(member.isfile() and name not in result and len(result)<count,'regular unique bounded member')
            need(0<=member.size<=16*1024*1024,'member length bound')
            total += member.size;need(total<=128*1024*1024,'expanded archive bound')
            chunks,size = [],0
            with tar.extractfile(member) as stream:
                while chunk := stream.read(1048576):
                    size += len(chunk);need(size<=member.size,'member overflow');chunks.append(chunk)
            need(size==member.size,'member exact length');result[name] = b''.join(chunks)
    need(len(result)==count,'exact archive count')
    return result

def strip(value):
    if isinstance(value,list):
        return [strip(x) for x in value]
    if isinstance(value,dict):
        return {k:strip(v) for k,v in value.items() if k not in ('id','depth')}
    return value

def record(fields):
    return {'kind':'rec','fields':{'kind':'row','fields':[
        {'fieldName':n,'fieldType':t} for n,t in fields],'other':{'kind':'empty'}}}

def main():
    index = decode((HERE/'index.json').read_bytes())
    need(index['schema']=='moriarty.a5-no-flatten-tiny-retained-data/v1','package schema')
    need(len(index['packageSupport'])==4 and {r['path'] for r in index['packageSupport']}==set(SUPPORT),'four support names')
    for row in index['packageSupport']:
        b = (HERE/row['path']).read_bytes()
        need(len(b)==row['bytes'] and sha(b)==row['sha256'],'support bytes')
    adopted = (HERE/'adopted-proposal-sha256.txt').read_text()
    need(re.fullmatch('[0-9a-f]{64}\n',adopted) and adopted.strip()==index['proposalSha256'],'proposal adoption digest')
    archive_path = HERE/'original-evidence.tar.gz'
    need(archive_path.stat().st_size<=64*1024*1024,'compressed archive bound')
    raw = archive_path.read_bytes()
    need(index['archive']=={'path':archive_path.name,'bytes':len(raw),'sha256':sha(raw)},'outer archive pin')
    data = {'/'+n:b for n,b in archive(raw,784).items()}
    def get(path):
        original_name(path)
        return data[path]
    def read(path):
        return decode(get(path))
    def row(path):
        b = get(path)
        return {'path':path,'bytes':len(b),'sha256':sha(b)}
    def check(pin):
        actual = row(pin['path'])
        need(all(actual[k]==v for k,v in pin.items()),'archived original pin: '+pin['path'])
    def pinned(path,h,size=None):
        pin = {'path':path,'sha256':h}
        if size is not None:pin['bytes']=size
        check(pin)
    def add(wanted,r):
        need(set(r)=={'path','bytes','sha256'} and type(r['bytes']) is int and r['bytes']>=0,'exact pin schema')
        original_name(r['path'])
        need(r['path'] not in wanted or wanted[r['path']]==r,'conflicting union pin')
        wanted[r['path']] = r
    need(len(index['members'])==784,'indexed count')
    indexed = {}
    for r in index['members']:
        need(set(r)=={'originalPath','archivePath','bytes','sha256'} and r['archivePath']==original_name(r['originalPath']),'index member schema/path')
        need(r['originalPath'] not in indexed,'duplicate index member')
        indexed[r['originalPath']] = {'path':r['originalPath'],'bytes':r['bytes'],'sha256':r['sha256']}
    need(set(indexed)==set(data),'closed outer membership')
    for r in indexed.values():check(r)
    pinned(PROPOSAL,adopted.strip())
    pinned(BASE,'a0310464e1382cfe0786012af951bb37c86e0a17d5825d21d72cfb4eff89d69e')
    pinned(ADMISSION,'09eeff84e1f750b4ffae73dc44173aae058a5343f3bf64801be4480ab848e0ac',86218)
    base = read(BASE)['originals'];admission = read(ADMISSION)
    need(len(base)==746 and len(admission['files'])==284,'original admitted counts')
    wanted = {}
    for name,r in base.items():
        need(name==r['path'],'base path');add(wanted,r)
    for r in admission['files']:add(wanted,r)
    need(len(wanted)==758,'758 intermediate union')
    extras = decode(get(PROPOSAL).split(('<!-- tiny-package-extras -->\n'+FENCE+'json\n').encode(),1)[1].split(('\n'+FENCE).encode(),1)[0])
    need(len(extras)==25,'25 extras')
    for name,size,h in extras:
        need(name[:2] in ['S/','Q/'],'two exact prefixes')
        path = (S if name[:2]=='S/' else Q)+name[2:]
        need(path not in wanted,'extra overlap');add(wanted,{'path':path,'bytes':size,'sha256':h})
    need(len(wanted)==783 and sum(r['bytes'] for r in wanted.values())==35860565,'existing union/bytes')
    need(PROPOSAL not in wanted,'separate proposal');add(wanted,row(PROPOSAL))
    need(wanted==indexed,'exact784 recomputed closure')
    need(index['limits']=={'existingOriginals':783,'archiveMembers':784,'existingOriginalBytes':35860565},'fixed limits')
    need(index['externalRuntimeArchives']==EXTERNAL and all(r['path'] not in data for r in EXTERNAL),'external archives excluded')
    need(META+'root-admission-tiny.json' not in data and not any(
        '/a5-no-flatten-full-supplement' in n or META+'full-' in n or META+'root-dispatch-full.json' in n or
        META+'transport/full/' in n or '/a4-producer-receipts/a5-noflat-full-20260906/' in n for n in data),'no new full originals')
    pinned(PLAN,'79325d985b5f58a62a5f490b0ac51f60c873afcb36775be3d95e372d144b2260')
    for name,h in [
        ('a5-no-flatten-plan-original-20260906.md','c8df95776d75b2702d1d41d78f66ed3cb7ceea95e4918802c7192b108885e59d'),
        ('a5-no-flatten-plan-editing-intermediate-20260906.md','7c75e475956a4909d0eb359bf91a7cee4ef43b884028c0ee66b9f255ddca18e1'),
        ('a5-no-flatten-plan-first-review-as-read-20260906.md','9597833b9a0c6191673a659adf76ec89c5fea2000e434e55c02be822d6cf07d4'),
        ('a5-no-flatten-plan-independent-review-20260906.md','9597833b9a0c6191673a659adf76ec89c5fea2000e434e55c02be822d6cf07d4')]:pinned(S+name,h)
    failed = read(META+'tiny-intake.json');corrected = read(META+'tiny-retained-type-application-intake.json')
    pinned(META+'tiny-intake.json','2c39b46364d588dbe78f1505a657f5df003051b8a59ad67c411193becd44a988',81784)
    pinned(META+'tiny-retained-type-application-intake.json','f2d73ee627dd4025b7038149f2a5b99f78abb29a6e55fba44f1f4d8d121ed4aa',82939)
    need(failed['mode']=='tiny' and failed['validationErrors']==[{'message':'retained Concrete=Box[int]','type':'RuntimeError'}],'original predicate failure')
    for k in ['preservation','sourceEndpointsStable','runtimeEndpointsStable']:need(failed[k] is True,'failed preservation endpoints')
    need(failed['capabilityOrFullSuccess'] is False and failed['compilerAcceptance'] is False and failed['H1']=='unresolved','failed scope')
    failedtool = read(META+'transport/intake-tiny.json')
    need(failedtool['result']['chunk_id']=='2a1216' and failedtool['result']['exit_code']==2,'actual original intake exit2')
    need(decode(failedtool['result']['output'].encode())['intake']==row(META+'tiny-intake.json'),'failed actual output binding')
    failure = read(S+'a5-no-flatten-tiny-failure-root-admission-20260906.json')
    pinned(S+'a5-no-flatten-tiny-failure-root-admission-20260906.json','7cdbc33d9556e64e605a7a85b25851d11afba2abaaaed745080602e8bfa72a49')
    need(failure['actualCompilerExit']==failure['actualOuterExit']==0 and failure['actualPlannedIntakeExit']==2,'failed admission exits')
    need(failure['originalPinsChecked']==746 and failure['consumedTinySlot'] is True and failure['originalCapabilityGateFailed'] is True,'failed admission scope')
    need(all(failure[k] is False for k in ['capabilityAdmitted','fullDispatchAuthorized','nativeRetryAuthorized']),'closed failed gates')
    need(len(failure['independentEvidence'])==3,'independent evidence count')
    for r in failure['independentEvidence']:check(r)
    for k in ['originalIntake','outerTerminal','transportTerminal']:check(failure[k])

    before = read(META+'tiny-before/endpoint.json');after = read(META+'tiny-after/endpoint.json')
    dispatch = read(META+'root-dispatch-tiny.json');seal = read(META+'tiny-dispatch-seal.json')
    terminal = read(META+'tiny-outer/terminal.json');launch = read(META+'tiny-outer/launch.json')
    receipt = read(INNER+'receipt.json');transport = read(META+'transport/tiny/terminal.json')
    for ep,when in [(before,'before'),(after,'after')]:
        need(ep['mode']=='tiny' and ep['endpoint']==when and ep['ok'] is True and ep['errors']==[],'original endpoint status')
    need(before['sourcePins']==after['sourcePins'] and before['runtime']==after['runtime'],'equal root endpoints')
    need(before['actualHead']==after['actualHead']==dispatch['actualDispatchHead']==receipt['sourceCommit']==receipt['sourceCommitAfter']==HEAD,'original actual HEAD')
    for ep in [before,after]:
        check(ep['sourceArchive']);rows = ep['sourceArchiveMembers']
        need(len(rows)==230 and len({r['archivePath'] for r in rows})==230,'230 source members')
        need({r['path']:r['sha256'] for r in rows}==ep['sourcePins'],'complete source member map')
        nested = archive(get(ep['sourceArchive']['path']),230)
        need(set(nested)=={r['archivePath'] for r in rows},'nested source membership')
        for r in rows:
            need(r['archivePath']==original_name(r['path']) and nested[r['archivePath']]==get(r['path']),'source archive equals preserved original')
            check({k:r[k] for k in ['path','bytes','sha256']})
    need(set(seal)=={'archive','beforeEndpoint','dispatch','sourceArchive','scope'} and
         seal['scope']=='two acyclic source archives complete before native launch','exact original dispatch seal schema/scope')
    for key in ['archive','beforeEndpoint','dispatch','sourceArchive']:
        need(type(seal[key]) is dict and set(seal[key])=={'path','bytes','sha256'},'exact dispatch seal pin schema')
        check(seal[key])
    need(seal['dispatch']==row(META+'root-dispatch-tiny.json') and seal['beforeEndpoint']==dispatch['beforeEndpoint']==row(META+'tiny-before/endpoint.json'),'acyclic dispatch links')
    need(seal['sourceArchive']==dispatch['sourceArchive']==before['sourceArchive'],'dispatch source prerequisite')
    nested = archive(get(seal['archive']['path']),1)
    need(nested=={original_name(seal['dispatch']['path']):get(seal['dispatch']['path'])},'exact one-member dispatch archive')
    for k in ['beforeEndpoint','afterEndpoint','dispatchSeal','outerTerminal','transportTerminal']:
        check(failed[k]);need(failed[k]==corrected[k],'same original endpoints')
    planadopt = read(S+'a5-no-flatten-plan-root-adoption-20260906.json')
    need(planadopt['plan']['path']==PLAN,'adopted original plan')
    for r in [planadopt['plan'],planadopt['review']]:check(r)
    need(get(planadopt['review']['path']).startswith(b'PASS\n'),'actual final plan review')
    groups = decode(get(PLAN).split(('<!-- noflat-pins -->\n'+FENCE+'json\n').encode(),1)[1].split(('\n'+FENCE).encode(),1)[0])
    fixed = {}
    for name,pins in groups.items():
        need(len(pins)=={'integrated121':121,'a5viewAndOriginal54':54,'alias4':4,'support':38}[name],'fixed group sizes')
        for path,h in pins.items():
            need(path not in fixed or fixed[path]==h,'fixed conflict');fixed[path] = h;pinned(path,h)
    need(len(fixed)==215 and all(before['sourcePins'].get(p)==h for p,h in fixed.items()),'215 fixed plus dynamic')
    slot = read(S+'a5-no-flatten-slot-tiny-20260906.json')
    need(slot['mode']=='tiny' and slot['authorized'] is True and slot['schedulerExclusive'] is True and slot['ownedPgids'],'recorded root slot')
    for r in slot['terminalBindings']+[slot['psObservation']]:
        check(r);need(before['sourcePins'].get(r['path'])==r['sha256'],'predecessor in source archive')
    dynamic = [PLAN,S+'a5-no-flatten-plan-root-adoption-20260906.json',planadopt['review']['path'],
        S+'a5-no-flatten-slot-tiny-20260906.json',*[r['path'] for r in slot['terminalBindings']],slot['psObservation']['path']]
    full_source = dict(fixed)
    for path in dynamic:
        h = sha(get(path));need(path not in full_source or full_source[path]==h,'dynamic conflict');full_source[path] = h
    need(len(full_source)==230 and full_source==before['sourcePins'],'215 fixed plus exact dynamic source union')
    sources = {p[len(ROOT)+1:]:h for p,h in groups['integrated121'].items()}
    need(before['integrated121']==after['integrated121']==receipt['sources_before']==receipt['sources_after']==sources,'all121 identities')
    need(before['missingIntegrated']==after['missingIntegrated']==receipt['not_yet_created_before']==receipt['not_yet_created_after']==[],'no source omissions')
    for prefix in [META+'tiny-before/integrated/',META+'tiny-after/integrated/',INNER+'before/',INNER+'after/']:
        need(read(prefix+'closure.json')=={'sources':sources,'not_yet_created':[]},'snapshot closure')
        need({p[len(prefix+'source/'):] for p in data if p.startswith(prefix+'source/')}==set(sources),'exact121 copied members')
        for name,h in sources.items():pinned(prefix+'source/'+name,h)
    view = read(OLD+'pilot-compilation-view/view-manifest.json')['files']
    frozen = read(OLD+'task2-frozen-source.json')['files']
    need(len(view)==24 and len(frozen)==30,'24+30')
    names = {ROOT+'/'+r['view']:r['viewSha256'] for r in view}
    names.update({ROOT+'/'+r['path']:r['sha256'] for r in frozen})
    need(names==groups['a5viewAndOriginal54'],'54 exact map')
    changed = 0
    for r in view:
        original = ROOT+'/'+r['original'];copy = get(ROOT+'/'+r['view'])
        need(original in fixed,'all view originals fixed')
        if r['addedImport']:
            add = r['addedImport'].encode();changed += 1
            need(copy.count(add)==1 and copy.replace(add,b'',1)==get(original),'exact reversible import')
        else:need(copy==get(original),'unchanged view bytes')
    need(changed==2,'two adapter additions')
    rt = before['runtime'];manifest = read(STORE+'manifest.json')
    producer = read(S+'a4-producer-dispatch.json');shared = read(OLD+'dispatch.json')
    pinned(STORE+'manifest.json','fa3e9838c66480557ed6d928a2d159ebc074d05f67f15ef08ab20388d9c49e0b')
    pinned(OLD+'dispatch.json','393a8aa904bcd9473670ac48934b3dca4ed611ca9b9accbfa94f15bb4798ac2f')
    pinned(HELPER,'816c3ad79dc3a67ca9a03729af56d74188c161a7546b9c43e821c222a4bce7f2')
    need(rt['manifest']==manifest and rt['producer']==producer and rt['shared']==shared,'recorded runtime original objects')
    need(producer['beforeDispatchCommit']==receipt['beforeDispatchCommit']=='386bf0ae10f767e051b414a7231be105cc4b0f71' and shared['beforeDispatchCommit']=='900bb2051225b4a3d99bf422c3b2e5e386e3e7bc','distinct historical bases')
    need(producer['runtimeManifestSha256']==shared['runtimeManifestSha256']==sha(get(STORE+'manifest.json')) and producer['runtimeArchiveSha256']==shared['runtimeArchiveSha256']==manifest['archiveSha256']==EXTERNAL[0]['sha256'],'manifest/archive references')
    expected = {S+'a4-producer-dispatch.json':sha(get(S+'a4-producer-dispatch.json')),
        OLD+'dispatch.json':producer['sharedDispatchSha256'],HELPER:producer['runtimeHelperSha256'],
        STORE+'manifest.json':producer['runtimeManifestSha256'],STORE+'runtime.tar.gz':producer['runtimeArchiveSha256'],
        manifest['reusedPythonManifest']:manifest['reusedPythonManifestSha256'],manifest['reusedPythonArchive']:manifest['reusedPythonArchiveSha256']}
    need(len(manifest['files'])==4758 and len(manifest['pythonFiles'])==3329 and len(manifest['treeMembers'])==4,'full recorded runtime inventories')
    for r in manifest['files']+manifest['pythonFiles']:
        need(r['path'] not in expected or expected[r['path']]==r['sha256'],'runtime conflict');expected[r['path']] = r['sha256']
    need(len(manifest['versions'])==6 and {v['name'] for v in manifest['versions']}=={'node','quint','java','python','rust','libc'},'six versions')
    for v in manifest['versions']:
        for stream in ['stdout','stderr']:
            path = STORE+v['name']+'.'+stream;h = v[stream+'Sha256'];pinned(path,h)
            need(path not in expected or expected[path]==h,'version pin conflict');expected[path] = h
    python = read(manifest['reusedPythonManifest'])
    pinned(manifest['reusedPythonManifest'],'cd004057c4067bf7cc536d2cec5038add88d0852c1a7de5030e402ff2204f9c4')
    need(python['sourceBytes']==manifest['pythonFiles'] and python['archiveSha256']==manifest['reusedPythonArchiveSha256']==EXTERNAL[1]['sha256'] and manifest['reusedPythonArchive']==EXTERNAL[1]['path'],'reused Python binding')
    need(len(expected)==8102 and expected==rt['expected'],'8102 reconstructed runtime pins')
    need(rt['observed']==receipt['runtime_before']==receipt['runtime_after']=={'files':expected,'treeMembers':manifest['treeMembers']},'all recorded runtime endpoints')
    verified = {'beforeDispatchCommit':shared['beforeDispatchCommit'],'dispatchSha256':sha(get(OLD+'dispatch.json')),'runtimeArchiveSha256':shared['runtimeArchiveSha256'],'runtimeManifestSha256':shared['runtimeManifestSha256']}
    need(rt['verified']==receipt['shared_runtime_before']==receipt['shared_runtime_after']==verified,'shared runtime verification records')
    need(len(manifest['dynamicLibraryReceipts'])==124 and 'javaRelease' in manifest and 'quintPackage' in manifest,'recorded runtime support retained')
    for path,h in expected.items():
        if path in data:pinned(path,h)
    installed = [p for p in data if p.startswith(Q)]
    need(len(installed)==12,'twelve selected installed source files')
    for p in installed:need(expected[p]==sha(get(p)),'selected source against runtime')
    for name,tool in receipt['tools'].items():
        v = next(v for v in manifest['versions'] if v['name']==name)
        need(tool['version_exit']==v['exitCode'] and expected[tool['path']]==tool['sha256'],'actual probe/executable binding')
        for stream in ['stdout','stderr']:need(sha(tool['version_'+stream].encode())==v[stream+'Sha256'],'actual version streams')
    need(set(receipt['tools'])=={'node','quint','python','rust'} and receipt['tools']['rust']['version_exit']==1,'four probes/known unsupported rust')
    need(launch['source_runtime_before']==terminal['source_runtime_before']==terminal['source_runtime_after'],'RH002 endpoint dictionaries')
    for path,h in terminal['source_runtime_before'].items():
        if path==RH:pinned(path,h)
        else:need(expected[path]==h,'outer runtime binding')
    need(get(INNER+'runtime-helper.py')==get(HELPER) and get(META+'tiny-outer/runner.py')==get(RH),'original helper copies')

    compiler = [NODE,'--max-old-space-size=4096',CLI,'compile',OLD+'alias-visibility-control/src/driver_direct.qnt',
        '--main=alias_visibility_direct','--target=json','--invariant=safety','--verbosity=0','--flatten=false']
    inside = [PYTHON,'-B','-X','pycache_prefix='+INNER+'python-cache',RECORDER,'--stage','a5-noflat-tiny-20260906','--',*compiler]
    parent = [PYTHON,'-B','-X','pycache_prefix='+META+'tiny-parent-cache',RH,'--receipt-dir',META+'tiny-outer',
        '--inner-receipt',INNER+'receipt.json','--wall-seconds','120','--cwd',ROOT,'--',*inside]
    need(dispatch['compilerArgv']==receipt['command']==receipt['executed_command']==compiler,'exact original compiler argv')
    need(dispatch['innerArgv']==launch['child_argv']==inside and dispatch['rh002Argv']==launch['original_parent_argv']==parent and launch['wrapper_argv']==parent[4:],'exact recorder/RH002 argv')
    need(launch['argv']==['/usr/bin/time','-v','-o',META+'tiny-outer/resources.txt',*inside] and launch['cwd']==receipt['cwd']==dispatch['cwd']==ROOT and launch['inner_receipt']==INNER+'receipt.json','time argv/cwd')
    need(receipt['python_cache_prefix']==INNER+'python-cache' and receipt['python_bytecode_writes'] is False,'cache and bytecode')
    need(dispatch['wallSeconds']==terminal['wall_seconds']==launch['wall_seconds']==120 and dispatch['nodeOldSpaceMiB']==4096 and launch['grace_seconds']==launch['cleanup_seconds']==5,'original bounds')
    removed = ['PYTHONPATH','PYTHONHOME','PYTHONSTARTUP','PYTHONINSPECT','PYTHONOPTIMIZE','PYTEST_ADDOPTS','PYTEST_PLUGINS','NODE_PATH','NODE_OPTIONS','NODE_COMPILE_CACHE','LD_PRELOAD','LD_LIBRARY_PATH','JAVA_TOOL_OPTIONS','_JAVA_OPTIONS','JDK_JAVA_OPTIONS']
    fixed = {'PYTHONOPTIMIZE':'0','PYTHONNOUSERSITE':'1','PYTHONDONTWRITEBYTECODE':'1','LC_ALL':'C',
        'PATH':'/usr/lib/jvm/java-25-openjdk-amd64/bin:'+str(PurePosixPath(NODE).parent)+':/home/charl/.npm-global/bin:/usr/bin:/bin',
        'JVM_ARGS':'-Xmx4096m','JVM_GC_ARGS':'-XX:+UseG1GC -XX:G1PeriodicGCInterval=600000 -XX:+G1PeriodicGCInvokesConcurrent',
        'APALACHE_JAR':'/home/charl/.quint/apalache-dist-0.56.1/apalache/lib/apalache.jar'}
    need(dispatch['parentRemoved']==removed and dispatch['parentFixed']==fixed,'original parent environment')
    need(shlex.split(dispatch['shellCommand'])==['env',*[x for k in removed for x in ['-u',k]],*[k+'='+v for k,v in fixed.items()],*parent],'exact original shell tokens')
    need(launch['fixed_environment']=={'PYTHONNOUSERSITE':'1','PYTHONDONTWRITEBYTECODE':'1','NODE_DISABLE_COMPILE_CACHE':'1','LC_ALL':'C'},'RH002 environment')
    need([receipt['exit_code'],receipt['recorder_exit_code'],terminal['actual_exit'],terminal['return_code'],transport['actualOuterExit']]==[0]*5 and all(type(x) is int for x in [receipt['exit_code'],receipt['recorder_exit_code'],terminal['actual_exit'],terminal['return_code'],transport['actualOuterExit']]),'five authentic zero exits')
    need(receipt['source_stable'] is True and receipt['runtime_stable'] is True,'inner stable endpoints')
    need(terminal['cleanup']=={'complete':True,'errors':[],'forced':False,'signals':[]} and terminal['timed_out'] is False,'recorded unforced cleanup')
    need(all(terminal[k] is True for k in ['eligible','resource_complete','source_runtime_stable','inner_receipt_present','inner_receipt_independent_validation_required']) and terminal['inner_receipt_complete'] is False,'original RH002 predicates')
    need(all(terminal[k] is None for k in ['launch_error','monitor_error','resource_error']),'no original launch/resource error')
    pinned(INNER+'receipt.json','c03d9b9eeb1d66f593599b244e9d22b7f8057255f53ea77b1d1e3a4f18c8d5da')
    pinned(INNER+'receipt.json',terminal['inner_receipt_sha256'])
    for path,h in terminal['sidecar_sha256'].items():pinned(path,h)
    for stream in ['stdout','stderr']:pinned(INNER+stream+'.bin',receipt[stream+'_sha256'])
    need(get(INNER+'stderr.bin')==get(META+'tiny-outer/stderr.bin')==b'' and receipt['artifacts']=={},'empty error streams/no ITF')
    need(read(META+'tiny-outer/stdout.bin')=={'stage':INNER.rstrip('/'),'exit_code':0,'source_stable':True,'runtime_stable':True,'recorder_exit_code':0,'artifacts':{}},'original recorder summary')
    nodes = [n for n in ast.parse(get(RH).decode()).body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='FIELDS' for t in n.targets)]
    need(len(nodes)==1,'one archived resource field declaration');field_names = ast.literal_eval(nodes[0].value)
    values = {}
    for line in get(META+'tiny-outer/resources.txt').decode().splitlines():
        line = line.strip();matches = [k for k in field_names if line.startswith(k+': ')]
        need(len(matches)==1 and matches[0] not in values,'finite unique GNU-time line')
        k = matches[0];values[k] = line[len(k)+2:]
    need(len(values)==23 and set(values)==set(field_names) and values==terminal['resource']['fields'] and terminal['resource']['diagnostics']==[],'original23 resource fields')
    need(values['Exit status']=='0' and terminal['resource']['exit_status']==0 and values['Elapsed (wall clock) time (h:mm:ss or m:ss)']=='0:04.40' and values['Maximum resident set size (kbytes)']=='380056' and terminal['resource']['maximum_rss_kib']==380056 and values['Command being timed']=='"'+' '.join(inside)+'"','exact setup-inclusive resources')
    replies = [read(META+'transport/tiny/response-%03d.json'%i) for i in range(2)]
    need(transport['responses']==replies and replies[0]['chunk_id']=='b637fc' and replies[0]['session_id']==46237 and 'exit_code' not in replies[0] and replies[1]['chunk_id']=='92b92b' and replies[1]['exit_code']==0,'two original native responses')
    pre = read(META+'transport/tiny/prelaunch.json');pretool = read(META+'transport/prelaunch-tiny.json')
    need(pre['actualToolResponse']==pretool['result'] and pretool['result']['chunk_id']=='c22e80' and pretool['result']['exit_code']==0,'actual prelaunch')
    need(pre['parsedOutput']==decode(pretool['result']['output'].encode()),'actual prelaunch parsed output')
    need(transport['args']==read(META+'transport/tiny/command.json')['args']==pre['parsedOutput']['args'] and transport['args']['cmd']==dispatch['shellCommand'] and transport['args']['workdir']==ROOT,'transport command equality')

    diagnosis = S+'a5-no-flatten-tiny-type-application-diagnosis-20260906.md'
    pinned(diagnosis,'6d8a52ea54b5a70d1e2d6f0bd7bc72aa298c3139fc96af34febe31ed7a5703d0')
    pinned(S+'a5-no-flatten-tiny-type-application-diagnosis-original-20260906.md','b780b803f746dd7d9e3deae604f0c21779687df5bfc847933d1c8a98077b170b')
    pinned(S+'a5-no-flatten-retained-data-proposal-independent-review-20260906.md','6b20f3e4fcebc8fa4993cdb1c26b568b0b5a87a8e319ef2d805bc062764be4f8')
    adoption = read(S+'a5-no-flatten-retained-data-proposal-root-adoption-20260906.json')
    pinned(S+'a5-no-flatten-retained-data-proposal-root-adoption-20260906.json','16ba5d306515b3fc836765bf4d36a7fc5629d4f9d87e6099cb0bd918f1f7492c')
    need(adoption['decision']=='adopt exact reviewed retained-data correction and authorize one data-only intake' and adoption['nativeRerunAuthorized'] is False and adoption['fullDispatchAuthorized'] is False and adoption['originalFailedGateRetained'] is True,'correction-only adoption')
    for k in ['proposal','independentReview','failedPreservationAdmission']:check(adoption[k])
    correctiontool = read(S+'a5-no-flatten-retained-data-intake-tool-20260906.json')
    commands = re.findall('^'+FENCE+'sh\n(.*?)^'+FENCE+'$',get(diagnosis).decode(),re.M|re.S)
    need(len(commands)==1 and correctiontool['args']['cmd']==commands[0] and correctiontool['args']['workdir']==ROOT,'exact actual adopted recipe')
    actual = correctiontool['result']
    need(actual['chunk_id']=='f108b4' and actual['exit_code']==0 and 'session_id' not in actual,'actual correction terminal')
    need(decode(actual['output'].encode())=={'preservation':True,'capabilityOrFullSuccess':True,'intake':row(META+'tiny-retained-type-application-intake.json'),'mode':'tiny','H1':'unresolved','compilerAcceptance':False},'actual corrected output')
    reviewpath = S+'a5-no-flatten-retained-data-independent-review-20260906.md'
    pinned(reviewpath,'82ec3e4aa6aa0ce84c18eb7d2c8df25255076e46f97b67ea6fde96b4bbeef5c4')
    review = get(reviewpath).decode();need(review.startswith('PASS\n'),'actual independent corrected PASS')
    blocks = re.findall('^'+FENCE+'json\n(.*?)^'+FENCE+'$',review,re.M|re.S);need(len(blocks)==1,'one original independent tool receipt')
    reviewtool = decode(blocks[0].encode());need(reviewtool['result']['chunk_id']=='ac3d90' and reviewtool['result']['exit_code']==0 and isinstance(reviewtool['args'],dict),'actual independent data tool')
    reviewout = decode(reviewtool['result']['output'].encode())
    need(reviewout['originalPinsRechecked']==746 and reviewout['actualRecipeChunk']=='f108b4' and reviewout['actualRecipeExit']==0,'independent correction observation')
    for r in reviewout['bindings']:check(r)
    changedkeys = {k for k in set(corrected)|set(failed) if corrected.get(k)!=failed.get(k)}
    need(changedkeys=={'capabilityOrFullSuccess','validationErrors','compilerJSON','declarations','generatedSelections','originalImports','serializedMain','retainedDataCorrection'},'exact eight interpretation differences')
    need(corrected['capabilityOrFullSuccess'] is True and corrected['validationErrors']==[] and corrected['compilerAcceptance'] is False and corrected['H1']=='unresolved','corrected capability only')
    need(all(corrected[k] is True for k in ['preservation','sourceEndpointsStable','runtimeEndpointsStable']),'corrected preserved endpoints')
    need(corrected['actualCompilerExit']==corrected['actualTimeWrappedExit']==corrected['actualOuterExit']==0,'corrected actual exits')
    need(len(corrected['originalArtifacts'])==254 and corrected['originalArtifacts']==failed['originalArtifacts'] and corrected['originalDirectories']==failed['originalDirectories'],'same254 original artifact list')
    artifact_names = set()
    for r in corrected['originalArtifacts']:check(r);need(r['path'] not in artifact_names,'unique artifact');artifact_names.add(r['path'])
    need(artifact_names=={p for p in data if p.startswith(INNER) or p.startswith(META+'tiny-outer/')},'closed original artifact membership')
    directories = set()
    for p in artifact_names:
        basepath = INNER.rstrip('/') if p.startswith(INNER) else META+'tiny-outer'
        parentpath = str(PurePosixPath(p).parent)
        while parentpath!=basepath:
            directories.add(parentpath);parentpath = str(PurePosixPath(parentpath).parent)
    # The recorder creates an empty python-cache directory, which has no file descendant.
    directories.add(INNER+'python-cache')
    need(set(corrected['originalDirectories'])==directories and len(corrected['originalDirectories'])==len(directories),'recorded directory closure')
    meta = corrected['retainedDataCorrection']
    need(meta['nativeRerun'] is False and meta['fullDispatchAuthorized'] is False and meta['originalPlanSha256']==sha(get(PLAN)),'correction scope')
    for k in ['originalFailedIntake','failedPreservationAdmission']:check(meta[k])
    newtype = record([('key',{'kind':'const','name':'Key'}),('value',{'kind':'int'})])
    oldtype = record([('key',record([('id',{'kind':'int'})])),('value',{'kind':'int'})])
    wrapper = commands[0].split("<<'PY'\n",1)[1].rsplit('\nPY',1)[0]
    tree = ast.parse(wrapper)
    def literal(name):
        nodes = [n.value for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id==name for t in n.targets)]
        need(len(nodes)==1,'unique recipe literal');return ast.literal_eval(nodes[0])
    original = literal('original');old_comparison = literal('old_comparison');old_output = literal('old_output')
    replacement = "        require(strip_ids(names['Concrete'])=="+repr({'kind':'typedef','name':'Concrete','type':newtype})+",'typecheck-resolved Concrete retains Key constant')"
    tighter = old_comparison+"\n        require(strip_ids(flat_names['Concrete'])=="+repr({'kind':'typedef','name':'Concrete','type':oldtype})+",'exact old flattened Concrete expands Key')"
    tighter += "\n        require(strip_ids(flat_names['box'])=="+repr({'kind':'var','name':'box','typeAnnotation':oldtype})+",'exact old flattened variable annotation')"
    ordered = {'originalFailedIntake':meta['originalFailedIntake'],'originalPlanSha256':meta['originalPlanSha256'],'failedPreservationAdmission':meta['failedPreservationAdmission'],'scope':meta['scope'],'nativeRerun':False,'fullDispatchAuthorized':False}
    new_output = "result['retainedDataCorrection']="+repr(ordered)+"\npath=M/'tiny-retained-type-application-intake.json'"
    section = get(PLAN).decode().split('## Task NF005: Strict data-only artifact intake after actual terminal and cleanup',1)[1].split('## Task NF006:',1)[0]
    blocks = re.findall('^'+FENCE+'python\n(.*?)^'+FENCE+'$',section,re.M|re.S);need(len(blocks)==1,'one original NF005 body');body = blocks[0]
    for old,new in [(original,replacement),(old_comparison,tighter),(old_output,new_output)]:
        need(body.count(old)==1,'exact unique substitution');body = body.replace(old,new,1)
    ast.parse(body)
    need(len(body.splitlines())==213 and sha(body.encode())==adoption['expectedConstructedNF005BodySha256']==reviewout['constructedBodySha256']=='8de559a96e8757ef38a98ac4fef8c90f735262f77aa1399b8ec28708f01984c1','constructed body source binding only')
    pinned(INNER+'stdout.bin','4638a8d77df74903912b7e023375116b5fdcbf62ea45cfd42246a8b9b1955ff2',13904)
    oldjson = OLD+'alias-visibility-control/direct-compile/input.qnt.json'
    pinned(oldjson,'fd8e96c2ced2081dd5635ae8a42916e831895ae40fb5adb49d18b9189d6173c5',16540)
    doc = decode(get(INNER+'stdout.bin'));old = decode(get(oldjson))
    need(set(doc)=={'stage','warnings','modules','table','types','effects','errors','main'} and doc['stage']=='compiling' and doc['main']=='alias_visibility_direct' and doc['errors']==doc['warnings']==[],'exact compiler serializer schema')
    need(all(type(doc[k]) is dict and doc[k] for k in ['table','types','effects']) and len(doc['modules'])==1,'nonempty maps/one module')
    module = doc['modules'][0];ds = module['declarations'];ods = old['modules'][0]['declarations']
    need(set(module)=={'id','name','declarations'} and module['name']=='alias_visibility_direct' and type(module['id']) is int and module['id']>=0,'original module')
    need(len(ds)==10 and len({x['id'] for x in ds})==10 and all(type(x['id']) is int and x['id']>=0 for x in ds),'ten declarations')
    need([strip(x) for x in ds[:2]]==[{'kind':'import','defName':'Box','protoName':'generic','fromSource':'./generic'},{'kind':'import','defName':'Key','protoName':'keys','fromSource':'./keys'}],'exact two imports')
    need([(x.get('kind'),x.get('name'),x.get('qualifier')) for x in ds[2:]]==[('typedef','Concrete',None),('var','box',None),('def','init','action'),('def','step','action'),('def','safety','val'),('def','q::init','action'),('def','q::step','action'),('def','q::inv','val')],'exact remaining declarations')
    names = {x['name']:strip(x) for x in ds[2:]};oldnames = {x['name']:strip(x) for x in ods}
    need(len(ods)==8 and all(x['kind']!='import' for x in ods),'old flattened declarations')
    need(names['Concrete']=={'kind':'typedef','name':'Concrete','type':newtype} and names['box']=={'kind':'var','name':'box','typeAnnotation':{'kind':'const','name':'Concrete'}},'new analyzed type/annotation')
    need(oldnames['Concrete']=={'kind':'typedef','name':'Concrete','type':oldtype} and oldnames['box']=={'kind':'var','name':'box','typeAnnotation':oldtype},'old expanded type/annotation')
    for n,q,e in [('q::init','action',{'kind':'name','name':'init'}),('q::step','action',{'kind':'name','name':'step'}),('q::inv','val',{'kind':'app','opcode':'and','args':[{'kind':'name','name':'safety'}]})]:
        need(names[n]=={'kind':'def','name':n,'qualifier':q,'expr':e},'generated selections')
    need(all(names[n]==oldnames[n] for n in ['init','step','safety','q::init','q::step','q::inv']),'exact normalized body equality')
    check(corrected['compilerJSON'])
    need(corrected['originalImports']==2 and corrected['declarations']==10 and corrected['serializedMain']=='alias_visibility_direct' and corrected['generatedSelections']==['q::init','q::step','q::inv'],'corrected result metadata')
    need(admission['verdict']=='PASS retained tiny capability only' and admission['nativeOriginalsBound']==254 and admission['originalPinsChecked']==746 and admission['originalCapabilityGateFailed'] is True,'separate corrected root admission')
    need(all(admission[k] is False for k in ['fullDispatchAuthorized','nativeRetryAuthorized','compilerAcceptance']) and admission['H1']=='unresolved','root tiny scope')
    need(len({r['path'] for r in admission['files']})==284,'284 unique admission originals')
    for r in admission['files']:check(r)
    for k in ['correctedIntake','originalFailedIntake','independentReview']:check(admission[k])
    need(admission['correctedIntake']==row(META+'tiny-retained-type-application-intake.json') and admission['originalFailedIntake']==row(META+'tiny-intake.json') and admission['independentReview']==row(reviewpath),'root interpretation bindings')
    need(index['rootToolAvailability']=='583fc6 and b31f8d remain transcript-only references; admission files preserved; no reconstruction','root response availability limit')
    print(json.dumps({'verdict':'PASS packaged tiny preservation and corrected-data recorded capability only',
        'archiveMembers':784,'existingOriginalBytes':35860565,'sourceArchiveMembers':[230,230],'sourceCopies':484,
        'recordedRuntimePins':8102,'selectedInstalledSources':12,'nativeTransportResponses':2,
        'originalPlannedIntakeExit':2,'correctedDataIntakeExit':0,'originalNativeExit':0,
        'nativeInvocations':0,'archivedCodeExecuted':False,'runtimeArchivesRead':False,'installedRuntimeTreesRead':False,
        'originalFailedGateRetained':True,'compilerAcceptance':False,'fullDispatchAuthorized':False,'H1':'unresolved',
        'rootTranscriptOnlyReferences':['583fc6','b31f8d'],
        'resourceScope':'recorded setup-inclusive recorder/descendant invocation; old-space is not total RSS',
        'runtimeScope':'recorded original bindings and archived source snippets only; prior admission supplies external byte identity',
        'originalNF005CompletionScope':'source-bound exact recipe and actual zero plus independent review; not re-executed',
        'processScope':'historically recorded cleanup/absence only; no current process inspection'},sort_keys=True))

if __name__=='__main__':
    main()
