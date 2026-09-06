"""Portable standard-library audit of preserved bytes; never executes originals."""
import sys
if sys.flags.optimize != 0:
    raise RuntimeError('Archive audit requires Python optimization disabled')
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import re
import tarfile

HERE = Path(__file__).resolve().parent
ROOT = '/home/charl/Moriarty/.worktrees/s01-audit-start'
OUT = ROOT+'/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906'
OLD = ROOT+'/.superpowers/sdd/a5-factoring-receipts'
PLANNING = ROOT+'/evidence/s02-candidate-a-completion/a5/compile-phase-diagnostic-planning'
PLAN = ROOT+'/docs/superpowers/plans/2026-09-06-candidate-a-compiler-phase-diagnostic.md'
PROPOSAL = ROOT+'/.superpowers/sdd/a5-phase-package-proposal-20260906.md'
PROPOSAL_SHA = '30521716a0f5a440edc878846811487918de582388cb16c37ffc3425ba598193'
INTAKE_SHA = 'f52aabfe60056dfce067edefb3cae9e6fd4863761d1fda2275927599ef4bcac6'
HEAD = '07c3a5462154d45e733d01d8a8bae856cb0f63c4'
NODE = '/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node'
CLI = '/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js'
PYTHON = '/home/charl/Moriarty/.venv/bin/python'
MODES = ['prepare','mocks','adopt-mocks','aliases','adopt-aliases','authorize-full','full','intake']
MOCKS = ['identity','open','write','partial','records','bytes','classify']
ALIASES = ['success-direct','success-observed','error-direct','error-observed']
FIXED = {'APALACHE_JAR': '/home/charl/.quint/apalache-dist-0.56.1/apalache/lib/apalache.jar',
         'JVM_ARGS': '-Xmx4096m', 'JVM_GC_ARGS': '-XX:+UseG1GC -XX:G1PeriodicGCInterval=600000 -XX:+G1PeriodicGCInvokesConcurrent',
         'NODE_DISABLE_COMPILE_CACHE': '1', 'NODE_OPTIONS': '--max-old-space-size=4096',
         'PATH': '/usr/lib/jvm/java-25-openjdk-amd64/bin:'+str(PurePosixPath(NODE).parent)+':/usr/bin:/bin',
         'PYTHONDONTWRITEBYTECODE': '1', 'PYTHONNOUSERSITE': '1'}
REMOVED = ['NODE_PATH','NODE_COMPILE_CACHE','PYTHONPATH','JAVA_TOOL_OPTIONS','_JAVA_OPTIONS','JDK_JAVA_OPTIONS','LD_PRELOAD','LD_LIBRARY_PATH','PYTHONOPTIMIZE']

def need(ok, message):
    if not ok:
        raise RuntimeError(message)

def sha(data):
    return hashlib.sha256(data).hexdigest()

def unique(pairs):
    result = {}
    for key, value in pairs:
        need(key not in result, 'duplicate JSON key: '+key)
        result[key] = value
    return result

def decode(data):
    return json.loads(data, object_pairs_hook=unique)

def norm(name):
    return name if name.startswith('/') else ROOT+'/'+name

def unpack(data, allow_dirs=False):
    files, dirs, seen = {}, set(), set()
    with tarfile.open(fileobj=io.BytesIO(data), mode='r:gz') as tar:
        for item in tar:
            name = item.name
            need(name and not name.startswith('/') and '..' not in PurePosixPath(name).parts and str(PurePosixPath(name)) == name,
                 'unsafe/noncanonical archive path: '+name)
            need(name not in seen, 'duplicate tar member: '+name); seen.add(name)
            if item.isdir():
                need(allow_dirs and item.size == 0, 'unexpected directory'); dirs.add(name)
            else:
                need(item.isfile() and item.size <= 8_000_000, 'nonregular/oversized tar member')
                data = tar.extractfile(item).read()
                need(len(data) == item.size, 'truncated tar member'); files[name] = data
    return files, dirs

def indexed(rows):
    result = {norm(r['path']): r for r in rows}
    need(len(result) == len(rows), 'duplicate pin inventory')
    return result

def integer(value, wanted, message):
    need(type(value) is int and value == wanted, message)

def main():
    index = decode((HERE/'index.json').read_bytes())
    need(index['schema'] == 'moriarty.a5-phase-diagnostic-preservation/v1', 'index schema')
    support = indexed(index['packageSupport'])
    need(set(support) == {ROOT+'/'+n for n in ['README.md','build_archive.py','audit.py']}, 'support membership')
    for row in support.values():
        b = (HERE/row['path']).read_bytes()
        need(len(b) == row['bytes'] and sha(b) == row['sha256'], 'package support changed')
    archive = (HERE/'original-small-evidence.tar.gz').read_bytes()
    need(index['archive'] == {'path': 'original-small-evidence.tar.gz','bytes': len(archive),'sha256': sha(archive)}, 'archive pin')
    raw, directories = unpack(archive, True)
    file_pins = indexed(index['members'])
    need(len(file_pins) == len(raw) == 282 and {ROOT+'/'+n for n in raw} == set(file_pins), 'exact archive files')
    need(len(directories) == 25 and sorted(directories) == index['directories'], 'exact archive directories')
    data = {ROOT+'/'+n: b for n,b in raw.items()}

    def get(name):
        return data[norm(name)]

    def read(name):
        return decode(get(name))

    def check(row):
        b = get(row['path'])
        need(sha(b) == row['sha256'] and ('bytes' not in row or len(b) == row['bytes']), 'pin mismatch: '+row['path'])

    for row in file_pins.values():
        need(row['kind'] == 'file', 'member kind'); check(row)
    need(sha(get(PROPOSAL)) == index['proposalSha256'] == PROPOSAL_SHA, 'reviewed proposal')
    need(sha(get(OUT+'/intake.json')) == index['intakeSha256'] == INTAKE_SHA, 'original intake')
    proposal = get(PROPOSAL).decode()
    table = proposal.split('## External original members\n')[1].split('## External large runtime bindings\n')[0]
    external = re.findall(r'^\| `([^`]+)` \| (\d+) \| `([0-9a-f]{64})` \|$', table, re.M)
    need(len(external) == 22 and len({r[0] for r in external}) == 22, 'external proposal membership')
    for name,size,digest in external:
        check({'path': name,'bytes': int(size),'sha256': digest})
    intake = read(OUT+'/intake.json')
    originals = indexed(intake['files'])
    need(len(originals) == 255 and all(n.startswith(OUT+'/') for n in originals), '255 intake originals')
    for row in originals.values(): check(row)
    post = {OUT+'/intake.json', *[OUT+'/transport/intake/'+n for n in ['command.json','response-000.json','terminal.json']]}
    expected = set(originals) | post | {norm(n) for n,_,_ in external} | {PROPOSAL}
    need(set(file_pins) == expected, 'proposal closed union')
    need({ROOT+'/'+d for d in directories} == set(intake['directories']) | {OUT+'/transport/intake'}, 'postindex directory closure')
    need(OUT+'/mocks/open/trace.jsonl' in {ROOT+'/'+d for d in directories}, 'intentional mock trace directory')
    need(intake['selfExcluded'] is True and intake['incrementalIntakeTransportExcluded'] is True, 'intake original exclusions')

    # Check every original nested source archive, without extracting or executing it.
    stages = [('mocks',MOCKS,103),('aliases',ALIASES,105),('full',['full-observed'],109)]
    nested_count = 0
    for stage,cases,count in stages:
        start = read(OUT+'/'+stage+'/start.json')
        stage_sources = indexed(start['sources'])
        need(len(stage_sources) == count and start['head'] == HEAD and start['mode'] == stage, 'stage source set/head')
        need(start['parentOrigArgv'] == [PYTHON,'-B',OUT+'/record.py',stage], 'stage parent argv')
        for case in cases:
            folder = OUT+'/'+stage+'/'+case
            child = read(folder+'/start.json')
            check(child['sourceArchive'])
            nested, empty_dirs = unpack(get(folder+'/source.tar.gz'))
            rows = child['sourceBytes']
            wanted = {r['archivePath']: r for r in rows}
            need(len(wanted) == len(rows) and set(wanted) == set(nested) and not empty_dirs, 'nested exact membership')
            source_set = set(stage_sources)
            if case.endswith('-observed'): source_set.add(folder+'-invocation.json')
            need({r['path'] for r in rows} == source_set and len(rows) == count + int(case.endswith('-observed')), 'nested stage/invocation closure')
            for name,b in nested.items():
                row = wanted[name]
                need(name == row['path'].lstrip('/') and len(b) == row['bytes'] and sha(b) == row['sha256'], 'nested member pin')
                path = row['path']
                if path in data: need(data[path] == b, 'inconsistent repeated original: '+path)
                else: data[path] = b
                if path in stage_sources:
                    need(row['bytes'] == stage_sources[path]['bytes'] and row['sha256'] == stage_sources[path]['sha256'], 'nested stage source binding')
            nested_count += len(rows)
        for row in stage_sources.values(): check(row)

    freeze = read(OUT+'/freeze.json')
    need(sha(get(OUT+'/freeze.json')) == '59a2ce768c0e26683ebec421d7ef091b9c3a8f5894b844ad6f8c57e599d9e405', 'preparation freeze')
    need(freeze['preparationHead'] == HEAD and len(indexed(freeze['sources'])) == 102, 'preparation source count/head')
    for row in freeze['sources']: check(row)
    frozen = indexed(freeze['sources'])
    for stage,_,count in stages:
        expected_sources = set(frozen) | {OUT+'/freeze.json'}
        if stage in ['aliases','full']: expected_sources |= {OUT+'/adopt-mocks.json',OUT+'/review-mocks.md'}
        if stage == 'full': expected_sources |= {OUT+'/adopt-aliases.json',OUT+'/review-aliases.md',OUT+'/root-dispatch.json',OUT+'/review-dispatch.md'}
        need(set(indexed(read(OUT+'/'+stage+'/start.json')['sources'])) == expected_sources, 'exact expanded stage source set')
    plan = get(PLAN)
    need(sha(plan) == 'b1df377b077cb7189f945ac493039e1f8d941f5076f61c1be6aaf8559de7bff9', 'adopted plan')
    blocks = re.findall(r'^<!-- file: ([^ ]+) -->\n```(?:javascript|python)\n(.*?)^```$', plan.decode(), re.S|re.M)
    need(len(blocks) == 4 and {n for n,_ in blocks} == {'observer.cjs','launch.cjs','controls.cjs','record.py'}, 'source block set')
    for name,source in blocks: need(get(OUT+'/'+name) == source.encode(), 'exact materialization')
    check({'path':ROOT+'/evidence/s02-candidate-a-completion/a4/native-resources/runner.py','sha256':'d8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d'})
    check({'path':ROOT+'/.superpowers/sdd/a4-task6-resumption-20260906/recorder.py','sha256':'63d8f00d024dbafb08fef0d48da96c91ca594bfe33476946ab0960bfd7dcf666'})
    design = read(PLANNING+'/design-adoption.json')
    check({'path':design['design'],'sha256':design['designSha256']})
    for row in design['sourcesAndReviews']: check({**row,'path':row['retained']})
    adoption = read(PLANNING+'/implementation-plan-adoption.json')
    check({'path':adoption['plan'],'sha256':adoption['planSha256']})
    for row in adoption['originals']: check(row)
    need(design['H1'] == 'unresolved' and design['historicalCompilerPhase'] == 'unknown', 'historical design limits')

    view = read(OLD+'/pilot-compilation-view/view-manifest.json')
    old_sources = read(OLD+'/task2-frozen-source.json')['files']
    need(len(view['files']) == 24 and len(old_sources) == view['task2FrozenCount'] == 30, 'view source inventory')
    for row in old_sources: check(row)
    changed = 0
    for row in view['files']:
        original, copied = get(row['original']), get(row['view'])
        need(len(original) == row['originalBytes'] and sha(original) == row['originalSha256'], 'view original')
        need(len(copied) == row['viewBytes'] and sha(copied) == row['viewSha256'], 'view copy')
        if row['addedImport']:
            addition = row['addedImport'].encode(); changed += 1
            need(copied.count(addition) == 1 and copied.replace(addition,b'',1) == original, 'view exact import reversal')
        else: need(original == copied, 'unchanged view')
    need(changed == 2, 'two import additions')

    runtime = read(OLD+'/tool-store/manifest.json')
    python = read(runtime['reusedPythonManifest'])
    need(len(runtime['files']) == 4758 and len(runtime['pythonFiles']) == len(python['sourceBytes']) == 3329, 'runtime source counts')
    need(runtime['pythonFiles'] == python['sourceBytes'], 'Python manifest source map')
    runtime_map = {}
    for row in runtime['files']+runtime['pythonFiles']:
        need(row['path'] not in runtime_map or runtime_map[row['path']] == row['sha256'], 'inconsistent shared runtime pin')
        runtime_map[row['path']] = row['sha256']
    need(len(runtime_map) == 8083, 'unique runtime mapping')
    recorded = indexed(freeze['runtime']['files'])
    need({n:r['sha256'] for n,r in recorded.items()} == runtime_map, 'all runtime endpoint pins')
    need(freeze['runtime']['runtimeFiles'] == 4758 and freeze['runtime']['pythonFiles'] == 3329, 'snapshot counters')
    need(len(runtime['treeMembers']) == 4, 'runtime tree manifest inventory')
    for tree,names in runtime['treeMembers'].items():
        need(names == sorted(set(names)) and set(names) <= set(runtime_map), 'runtime tree manifest bindings')
    refs = freeze['runtime']['references']
    need(len(refs) == 4, 'runtime references')
    for row in refs:
        if not row['path'].endswith('.tar.gz'): check(row)
    large = [r for r in refs if r['path'].endswith('.tar.gz')]
    need(index['externalRuntimeArchives'] == large and sum(r['bytes'] for r in large) == 382420738, 'external archive index')
    need(large[0]['sha256'] == runtime['archiveSha256'] == 'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c', 'shared archive binding')
    need(large[1]['sha256'] == python['archiveSha256'] == runtime['reusedPythonArchiveSha256'] == '7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac', 'Python archive binding')
    need(runtime['reusedPythonManifestSha256'] == sha(get(runtime['reusedPythonManifest'])), 'Python manifest binding')
    for stage,_,_ in stages:
        need(read(OUT+'/'+stage+'/runtime-before.json') == read(OUT+'/'+stage+'/runtime-after.json') == freeze['runtime'], 'runtime endpoint equality')

    responses_total = 0
    for mode in MODES:
        base = OUT+'/transport/'+mode
        command, terminal = read(base+'/command.json'), read(base+'/terminal.json')
        expected_command = 'PYTHONOPTIMIZE=0 '+PYTHON+' -B '+OUT+'/record.py '+mode
        need(command['command'] == terminal['command'] == command['args']['cmd'] == expected_command and command['cwd'] == terminal['cwd'] == command['args']['workdir'] == ROOT, 'transport exact command')
        response_names = sorted(n for n in file_pins if n.startswith(base+'/response-'))
        need(response_names == [base+'/response-%03d.json'%i for i in range(len(response_names))] and response_names, 'consecutive transport files')
        responses = [read(n) for n in response_names]
        need(responses == terminal['toolResponses'], 'exact original transport responses')
        integer(terminal['wrapperExitCode'],0,'outer wrapper exit'); integer(responses[-1]['exit_code'],0,'actual terminal response')
        for row in responses[:-1]:
            need('exit_code' not in row and row['session_id'] == responses[0]['session_id'], 'nonterminal same-session response')
        if mode == 'full':
            need(len(responses) == 91 and responses[0]['session_id'] == 86357 and responses[-1]['chunk_id'] == '77d8d8', 'full transport identity')
        if mode == 'intake':
            need(len(responses) == 1 and responses[0]['chunk_id'] == 'ce3615', 'actual intake response')
            reply = decode(responses[0]['output']); check(reply['intake']); need(reply['ok'] is True, 'intake output')
        responses_total += len(responses)

    def effective(kind):
        if kind == 'full': entry,main,inv = OLD+'/pilot-compilation-view/view/specs/quint/s02/factored_verification/candidate_a_funding_pilot.qnt','candidate_a_funding_pilot','pilotSafety'
        else:
            suffix = 'direct' if kind == 'success' else 'original'
            entry,main,inv = OLD+'/alias-visibility-control/src/driver_'+suffix+'.qnt','alias_visibility_'+suffix,'safety'
        return [NODE,CLI,'compile',entry,'--main='+main,'--target=json','--invariant='+inv,'--verbosity=0']

    def trace(folder, expected):
        raw_trace = get(folder+'/trace.jsonl')
        need(len(raw_trace) <= 16384 and raw_trace.endswith(b'\n'), 'trace bounds/newline')
        rows = [decode(line) for line in raw_trace.splitlines()]
        need(len(rows) == len(expected) <= 32, 'trace exact row count')
        previous = -1
        for i,(row,wanted) in enumerate(zip(rows,expected),1):
            need(set(row) == {'seq','phase','event','outcome','ns'}, 'trace schema')
            integer(row['seq'],i,'trace sequence')
            need(type(row['ns']) is str and re.fullmatch(r'\d+',row['ns']) and int(row['ns']) >= previous, 'trace monotonic decimal clock')
            previous = int(row['ns'])
            need((row['phase'],row['event'],row['outcome']) == wanted, 'exact trace event')
        validation = read(folder+'/trace-validation.json')
        need(validation['valid'] is True and validation['argvMatched'] is True and validation['rows'] == rows, 'trace receipt agreement')
        need(validation['locationMeaning'] == 'includes observer I/O and scheduling margins', 'trace interpretation')
        return validation

    child_count = 0
    for stage,cases,_ in stages:
        base = OUT+'/'+stage; terminal = read(base+'/terminal.json')
        integer(terminal['recorderExit'],0,'stage recorder exit')
        need(terminal['finalized'] is True and terminal['error'] is None, 'stage preservation terminal')
        owned = indexed(terminal['ownedFiles'])
        need(set(owned) == {n for n in file_pins if n.startswith(base+'/') and n != base+'/terminal.json'}, 'exact finalized owned files')
        for row in owned.values(): check(row)
        lives = []
        for case in cases:
            folder = base+'/'+case
            start, result, ended, process = [read(folder+'/'+n+'.json') for n in ['start','result','process-terminal','process']]
            expected_code = (-15 if stage == 'full' else (0 if case == 'identity' else 74) if stage == 'mocks' else (0 if case.startswith('success') else 1))
            integer(ended['childExit'],expected_code,'authentic child exit'); integer(result['childExit'],expected_code,'result child exit')
            lifecycle = {k:result[k] for k in ['childExit','cleanup','finalizationBlocked','monitorError','ownedPgid','receivedSignals','timedOut']}
            need(all(ended[k] == v for k,v in lifecycle.items()), 'process/result lifecycle binding')
            need(process['ownedPgid'] == process['pid'] == ended['ownedPgid'], 'owned process identity')
            need(ended['monitorError'] is None and ended['receivedSignals'] == [] and ended['finalizationBlocked'] is False, 'lifecycle completeness')
            forced = stage == 'full'
            cleanup = {'complete':True,'errors':[],'forced':forced,'signals':[{'sent':True,'signal':'SIGTERM'},{'sent':False,'signal':'SIGKILL'}] if forced else []}
            need(ended['cleanup'] == cleanup and ended['timedOut'] is forced, 'exact authentic cleanup')
            need(start['actualDispatchHead'] == HEAD and start['cwd'] == ROOT and start['parentOrigArgv'] == [PYTHON,'-B',OUT+'/record.py',stage], 'child source head/parent')
            env = dict(FIXED)
            if stage == 'mocks': env['NODE_OPTIONS'] = '--max-old-space-size=128 --unhandled-rejections=strict'
            need(start['fixedEnvironment'] == env and start['removedEnvironmentKeys'] == REMOVED, 'exact child environment')
            need(start['wallSeconds'] == {'mocks':15,'aliases':120,'full':900}[stage], 'wall bound')
            if stage == 'mocks': argv = [NODE,OUT+'/controls.cjs',case,folder]
            else:
                kind = 'full' if stage == 'full' else case.split('-')[0]
                argv = effective(kind)
                if case.endswith('-observed'):
                    invocation_path = folder+'-invocation.json'
                    spec, launched = read(invocation_path), read(folder+'/argv.json')
                    need(spec == {'argvReceipt':folder+'/argv.json','effectiveArgv':argv,'trace':folder+'/trace.jsonl'}, 'immutable invocation')
                    launched_argv = [NODE,OUT+'/launch.cjs',invocation_path]
                    need(launched == {'actualLaunchArgv':launched_argv,'effectiveArgv':argv,'execPath':NODE,'execArgv':[],'cwd':ROOT}, 'exact actual launcher receipt')
                    argv = launched_argv
            need(start['argv'] == argv and start['measuredArgv'] == ['/usr/bin/time','-v','-o',folder+'/resources.txt',*argv], 'exact measured argv')
            check(result['stdout']); check(result['stderr'])
            if stage == 'full':
                need(result['resource'] is None and result['resourceError'] == 'ResourceError: incomplete resource field inventory', 'missing resources retained')
                need(all(get(folder+'/'+n) == b'' for n in ['stdout.bin','stderr.bin','resources.txt']), 'empty full streams')
                need(ended['elapsedSeconds'] >= 900, 'timeout duration')
            else:
                fields = {}
                for line in get(folder+'/resources.txt').decode().splitlines():
                    if line.startswith('\t'):
                        k,v = line[1:].rsplit(': ',1); need(k not in fields,'duplicate resource field'); fields[k]=v
                need(len(fields) == 23 and fields == result['resource']['fields'] and result['resourceError'] is None, 'original resource fields')
                need(fields['Command being timed'] == '"'+' '.join(argv)+'"' and int(fields['Exit status']) == expected_code and int(fields['Maximum resident set size (kbytes)']) > 0, 'resource argv/exit/RSS')
            lives.append(lifecycle); child_count += 1
        need(terminal['lifecycles'] == lives, 'exact stage lifecycle inventory')

    identity = read(OUT+'/mocks/identity/stdout.bin')
    controls = identity['controls']
    need(identity['ok'] is True and get(OUT+'/mocks/identity/stderr.bin') == b'', 'identity result')
    need([r['mode'] for r in controls] == ['sync-right','sync-left','resolve','reject','throw','right-chain','left-chain'], 'identity modes')
    need(all(r['calls'] == 1 for r in controls[:5]) and [r['rows'][1]['outcome'] for r in controls[:5]] == ['Right','Left','Right','rejected','thrown'], 'identity calls/outcomes')
    need(controls[5]['calls'] == ['load','parse','typecheck','compile','outputCompilationTarget','outputResult'] and controls[6]['calls'] == ['load','parse','typecheck','compile','outputResult'], 'identity full/left chains')
    for case in MOCKS[1:]:
        folder = OUT+'/mocks/'+case
        need(get(folder+'/stdout.bin') == b'' and get(folder+'/stderr.bin') == b'phase-observer: diagnostic failure\n', 'mock failure streams')
        need((folder+'/original-called.txt' in data) == (case == 'classify'), 'mock original call presence')
        if case == 'classify': need(get(folder+'/original-called.txt') == b'1\n', 'classify original called once')
        if case in ['write','bytes']: need(get(folder+'/trace.jsonl') == b'', 'zero fault trace')
        if case == 'partial': need(get(folder+'/trace.jsonl') and not get(folder+'/trace.jsonl').endswith(b'\n'), 'partial write retained')
        if case in ['records','classify']: need(len(get(folder+'/trace.jsonl').splitlines()) == (32 if case == 'records' else 1), 'mock trace boundary')
    mocks = read(OUT+'/mocks/checks.json')
    need(mocks['ok'] is True and [r['case'] for r in mocks['cases']] == MOCKS, 'mock checks inventory')
    for row in mocks['cases']: need(row['ok'] is True,'mock predicate'); check(row['result'])
    alias_checks = read(OUT+'/aliases/checks.json')
    need(alias_checks['runs'] == ALIASES and all(alias_checks[k] is True for k in ['ok','rawPairsEqual','successfulJSONByteIdentical']), 'alias predicates')
    for kind in ['success','error']:
        for stream in ['stdout.bin','stderr.bin']:
            need(get(OUT+'/aliases/'+kind+'-direct/'+stream) == get(OUT+'/aliases/'+kind+'-observed/'+stream), 'alias exact stream pair')
    need(len(get(OUT+'/aliases/success-direct/stdout.bin')) == 16540 and get(OUT+'/aliases/success-direct/stderr.bin') == b'', 'alias success streams')
    need(len(get(OUT+'/aliases/error-direct/stderr.bin')) == 288 and get(OUT+'/aliases/error-direct/stdout.bin') == b'', 'alias error streams')
    prefix = [(p,event,outcome) for p in ['load','parse','typecheck'] for event,outcome in [('enter','stage'),('resolve','Right')]]
    success = prefix+[('compile','enter','stage'),('compile','resolve','Right'),('outputCompilationTarget','enter','stage'),('outputCompilationTarget','resolve','Right'),('outputResult','enter','Right')]
    error = prefix+[('compile','enter','stage'),('compile','resolve','Left'),('outputResult','enter','Left')]
    trace(OUT+'/aliases/success-observed',success); trace(OUT+'/aliases/error-observed',error)
    fulltrace = trace(OUT+'/full/full-observed',prefix+[('compile','enter','stage')])
    need(len(get(OUT+'/full/full-observed/trace.jsonl')) == 595 and fulltrace['lastEntered'] == fulltrace['incompleteObservedInterval'] == 'compile', 'new incomplete compile interval')
    need(fulltrace['lastCompleted'] == {'event':'resolve','outcome':'Right','phase':'typecheck'}, 'last completed phase')
    checks = read(OUT+'/full/checks.json')
    need(checks == intake['fullObservation'] and checks['trace'] == fulltrace and checks['ok'] is True and checks['childExit'] == -15 and checks['timedOut'] is True and checks['observationFailureExit74'] is False, 'full findings binding')
    need(checks['H1'] == 'unresolved' and checks['compilerAcceptance'] is False and intake['compilerAcceptance'] is False and intake['historicalCompilerPhase'] == 'unknown', 'acceptance limits')

    for mode in ['mocks','aliases']:
        adoption = read(OUT+'/adopt-'+mode+'.json')
        need(adoption['head'] == HEAD and adoption['rootDecision'] == 'admit wrapper controls only', 'control adoption scope')
        for row in adoption['files']: check(row)
        expected_files = {n for n in file_pins if n.startswith(OUT+'/transport/'+mode+'/')} | {OUT+'/'+mode+'/terminal.json',OUT+'/review-'+mode+'.md',OUT+'/freeze.json'}
        need(set(indexed(adoption['files'])) == expected_files, 'exact adoption bindings')
    for name in ['implementation','mocks','aliases','dispatch','final']:
        need(get(OUT+'/review-'+name+'.md').startswith(b'PASS\n'), 'separate root review')
    dispatch = read(OUT+'/root-dispatch.json')
    need(dispatch['actualDispatchHead'] == HEAD and dispatch['argv'] == effective('full') and dispatch['invocations'] == 1 and dispatch['wallSeconds'] == 900 and dispatch['jvmHeapMiB'] == dispatch['nodeHeapMiB'] == 4096, 'one bounded dispatch')
    for row in dispatch['files']: check(row)
    a4 = ROOT+'/.superpowers/sdd/a4-task6-resumption-20260906/fresh97'
    need(set(indexed(dispatch['files'])) == {a4+'/terminal.json',a4+'/outer-tool-receipt.json',OUT+'/review-dispatch.md',OUT+'/freeze.json',OUT+'/adopt-mocks.json',OUT+'/adopt-aliases.json'}, 'dispatch predecessor closure')
    a4terminal, a4outer = read(a4+'/terminal.json'), read(a4+'/outer-tool-receipt.json')
    integer(a4terminal['exitCode'],0,'A4 predecessor recorder exit'); integer(a4outer['responses'][-1]['exit_code'],0,'A4 actual outer exit')
    need(a4terminal['error'] is None and a4terminal['artifactFinalizationBlocked'] is False and a4terminal['originalParentExit'] is None, 'A4 original parent limitation')
    for row in a4terminal['commandLifecycles']:
        need(row['eligible'] is True and row['cleanup']['complete'] is True and row['cleanup']['forced'] is False, 'A4 prerequisite cleanup')
    need(a4outer['monitoringLimitations'], 'A4 transport gap retained')
    need(a4terminal['completedAt'] < dispatch['createdAt'] < read(OUT+'/full/start.json')['createdAt'], 'A4/dispatch/stage temporal order')
    disposition = read(ROOT+'/.superpowers/sdd/a5-phase-alias-transport-root-disposition-20260906.json')
    check(disposition['originalFailureReceipt']); check(disposition['supplement'])
    need(disposition['actualRootRun']['actualOuterExit'] == 0 and disposition['actualRootRun']['sessionId'] == 19278 and disposition['actualRootRun']['terminalChunk'] == 'fdc322', 'alias takeover authentic identity')
    need(any('not retained' in s for s in disposition['evidenceLimits']), 'missing nested-save response disclosure')
    full_review = get(ROOT+'/.superpowers/sdd/a5-phase-full-independent-intake-20260906.md')
    need(sha(full_review) == 'e4b745a1c7d09b0a8cc11a7cd3d3666da1bd8a1da3309bfaa4057d49669fb4f4' and b"SyntaxError: Unexpected token '?'" in full_review, 'full review/failure disclosure')
    need(index['largeRuntimeRehashed'] is False and index['nativeExecuted'] is False and index['compilerAcceptance'] is False, 'package limits')
    report = {'ok':True,'regularArchiveMembers':282,'originalDirectories':25,'intakeOriginalPins':255,'postindexOriginals':4,
              'nestedSourceArchives':12,'nestedSourceMembers':nested_count,'preparationSourcePins':102,'runtimeUniqueRecordedPins':8083,
              'mockChildren':7,'aliasChildren':4,'fullChildren':1,'childRecords':child_count,'returnedTransportResponses':responses_total,'fullReturnedResponses':91,
              'fullActualChildExit':-15,'fullActualRecorderExit':0,'fullActualOuterExit':0,'intakeActualOuterExit':0,
              'newIncompleteObservedInterval':'compile','historicalCompilerPhase':'unknown','compilerAcceptance':False,'H1':'unresolved',
              'archivedCodeExecuted':False,'nativeExecuted':False,'externalRuntimeRehashed':False,'optimization':sys.flags.optimize,
              'aliasPrelaunchFailurePreserved':True,'A4PollingGapPreserved':True,'continuousHistoricalImmutabilityProved':False}
    print(json.dumps(report,sort_keys=True))

if __name__ == '__main__':
    main()
