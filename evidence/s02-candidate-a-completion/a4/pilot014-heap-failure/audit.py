"""Standalone data audit. Never imports archived code or reads original paths."""
import sys
if sys.flags.optimize:
    raise RuntimeError('Archive audit requires optimization disabled')
import ast
import hashlib
import json
import math
from pathlib import Path, PurePosixPath
import posixpath
import re
import shlex
import tarfile

HERE = Path(__file__).resolve().parent
ROOT = '/home/charl/Moriarty/.worktrees/s01-audit-start'
S = '.superpowers/sdd/'
INNER = S+'a4-producer-receipts/case-014-export/'
OUTER = S+'a4-largest-native-pilot-014/'
TRANSPORT = S+'a4-pilot014-transport-20260906/'
PROPOSAL = S+'a4-pilot014-failure-package-proposal-20260906.md'
PROPOSAL_SHA = 'f099390b42f717d24e7660e1c14b9bfc4a73834bdaf25b4e3f3d7e20da29d543'
ADMISSION = S+'a4-pilot014-failure-root-admission-20260906.json'
REPORT = S+'a4-pilot014-failure-independent-intake-20260906.md'
DISPATCH = S+'a4-pilot014-root-dispatch-20260906.json'
RH = 'evidence/s02-candidate-a-completion/a4/native-resources/runner.py'
RECORDER = 'scripts/record_s02_candidate_a_integrated.py'
HELPER = 'scripts/run_s02_candidate_a_factoring_pilot.py'
STORE = S+'a5-factoring-receipts/tool-store/'
HEAD = '08e426c7163880b9312f1f1f029a4dde9d0e7593'
NODE = '/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node'
CLI = '/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js'
PYTHON = '/home/charl/Moriarty/.venv/bin/python'

def need(ok, message):
    if not ok:
        raise ValueError(message)

def sha(b):
    return hashlib.sha256(b).hexdigest()

def unique(pairs):
    result = {}
    for k,v in pairs:
        need(k not in result, 'duplicate JSON key'); result[k] = v
    return result

def reject(value):
    raise ValueError('nonfinite JSON constant: '+value)

def number(value):
    result = float(value); need(math.isfinite(result), 'nonfinite exponent'); return result

def decode(b, limit=16*1024*1024):
    need(len(b) <= limit, 'JSON byte limit')
    return json.loads(b, object_pairs_hook=unique, parse_constant=reject, parse_float=number)

def relative(name):
    if name.startswith(ROOT+'/'):
        name = name[len(ROOT)+1:]
    p = PurePosixPath(name)
    need(name and not p.is_absolute() and '..' not in p.parts and str(p) == name, 'unsafe archive lookup')
    return name

def absolute(name):
    return ROOT+'/'+name

def literal_assignment(source, name):
    tree = ast.parse(source)
    rows = [n for n in tree.body if isinstance(n, ast.Assign) and
            any(isinstance(t, ast.Name) and t.id == name for t in n.targets)]
    need(len(rows) == 1, 'unique source assignment '+name)
    return ast.literal_eval(rows[0].value)

def main():
    index = decode((HERE/'index.json').read_bytes())
    need(index['schema'] == 'moriarty.a4-pilot014-failure-preservation/v1', 'schema')
    support = index['packageSupport']
    need(len(support) == 3 and {r['path'] for r in support} == {'README.md','build_archive.py','audit.py'}, 'support set')
    for row in support:
        b = (HERE/row['path']).read_bytes()
        need(len(b) == row['bytes'] and sha(b) == row['sha256'], 'support pin')
    archive = HERE/'original-evidence.tar.gz'
    need(archive.stat().st_size <= 64*1024*1024, 'compressed archive limit')
    b = archive.read_bytes()
    need(index['archive'] == {'path': archive.name,'bytes': len(b),'sha256': sha(b)}, 'archive pin')
    data, total = {}, 0
    with tarfile.open(archive, 'r:gz') as tar:
        for member in tar:
            name = relative(member.name)
            need(name == member.name and name not in data and member.isfile(), 'archive member type/name')
            need(0 <= member.size <= 134217728 and len(data) < 325, 'member bound')
            total += member.size; need(total <= 128*1024*1024, 'archive total bound')
            chunks, size, digest = [], 0, hashlib.sha256()
            with tar.extractfile(member) as stream:
                while chunk := stream.read(1024*1024):
                    size += len(chunk); need(size <= member.size, 'member overflow')
                    digest.update(chunk); chunks.append(chunk)
            content = b''.join(chunks)
            need(size == member.size and digest.hexdigest() == sha(content), 'member length/hash'); data[name] = content
    rows = index['members']; pins = {r['path']: r for r in rows}
    need(len(rows) == len(pins) == len(data) == 325 and set(pins) == set(data), 'exact indexed members')

    def get(name):
        return data[relative(name)]

    def read(name):
        return decode(get(name))

    def pin(name, digest, size=None):
        value = get(name)
        need(sha(value) == digest and (size is None or len(value) == size), 'original pin: '+name)

    for name,row in pins.items(): pin(name,row['sha256'],row['bytes'])
    pin(PROPOSAL, PROPOSAL_SHA)
    need(index['proposalSha256'] == PROPOSAL_SHA, 'proposal index')
    pin(ADMISSION,'f0676a056c57464478954ecc9ae622acfee14102bfcec8079b6900584dc3f758')
    pin(REPORT,'4259ddd09d8073edb158a2140f0d78cfcf7ea1e4c607d1303ccc8aeb38d3924b')
    original_index_name = S+'a4-pilot014-failure-independent-index-20260906.json'
    tool_name = S+'a4-pilot014-failure-independent-tool-receipt-20260906.json'
    pin(original_index_name,'e0f7cfda8a2ef9bd9cf913485d24d1928f9b211f795b4a6c40af4e99da8b4599',70090)
    pin(tool_name,'b300809e38b782488ec89cab390963d9acf2efe514224cb6b9499ec5d68f90ff')
    original_index = read(original_index_name); a = read(ADMISSION)
    need(original_index['count'] == len(original_index['originals']) == a['originalPinsChecked'] == 303, '303 base originals')
    need(original_index['selfExcluded'] is True and original_index_name not in original_index['originals'], 'original self exclusion')
    for n,row in original_index['originals'].items():pin(n,row['sha256'],row['bytes'])
    need(sum(r['bytes'] for r in original_index['originals'].values()) == 10005925, 'original byte total')
    extras = re.findall(r'^\| `((?:S/|evidence/|scripts/)[^`]+)` \| `([0-9a-f]{64})` \|$',get(PROPOSAL).decode(),re.M)
    need(len(extras) == 21, '21 proposal additions')
    expected_members = set(original_index['originals']) | {PROPOSAL}
    for n,h in extras:
        n = S+n[2:] if n.startswith('S/') else n
        need(n not in expected_members, 'additional overlap'); expected_members.add(n); pin(n,h)
    need(set(data) == expected_members, 'closed325 member union')
    for n,h in a['independentEvidence'].items():pin(n,h)
    need(a['originalIndexSha256'] == sha(get(original_index_name)), 'root index binding')
    tools = read(tool_name)
    need(tools['audit']['result']['chunk_id'] == '62b47c' and tools['audit']['result']['exit_code'] == 0 and
         tools['audit']['result']['original_token_count'] == 15621, 'actual truncated independent audit')
    need('truncat' in tools['audit']['result']['output'].lower(), 'preserved truncation marker')
    need(tools['index']['result']['chunk_id'] == '5b70df' and tools['index']['result']['exit_code'] == 0, 'actual independent index tool')
    need(decode(tools['index']['result']['output'].encode()) == {'ok':True,'index':absolute(original_index_name),
        'sha256':sha(get(original_index_name)),'bytes':70090,'originals':303,'case044DestinationsAbsent':True,
        'noITF':True,'nativeRerun':False}, 'actual complete index output')
    d = read(DISPATCH); rec = read(INNER+'receipt.json'); term = read(OUTER+'terminal.json'); launch = read(OUTER+'launch.json')
    pin(DISPATCH,'2d0394ee7fd47075dc365d9213f5218cc3ba6e873ec97f1611f9a770fa559f8e')
    need(len(d['bindings']) == 9 and a['rootDispatchSha256'] == sha(get(DISPATCH)), 'root dispatch binding')
    for n,h in d['bindings'].items():pin(n,h)
    parser_admission = read(S+'a4-final-source-parser-admission-20260906.json')
    need(d['actualDispatchHead'] == a['actualSourceHead'] == rec['sourceCommit'] == rec['sourceCommitAfter'] == HEAD, 'original source commits')
    source = d['sources']
    need(len(source) == 121 and parser_admission['sources'] == rec['sources_before'] == rec['sources_after'] == source, '121 sources')
    need(rec['not_yet_created_before'] == rec['not_yet_created_after'] == [], 'no missing sources')
    for phase in ['before','after']:
        need(read(INNER+phase+'/closure.json') == {'sources':source,'not_yet_created':[]}, 'closure map')
        prefix = INNER+phase+'/source/'
        need({n[len(prefix):] for n in data if n.startswith(prefix)} == set(source), 'source copy set')
        for name,h in source.items(): pin(prefix+name,h)
    src = lambda n: get(INNER+'before/source/'+n).decode()
    python_names = literal_assignment(src(RECORDER),'PYTHON')
    seen = set(python_names)
    pending = [f'specs/quint/s02/candidate_a_integrated_case_{i:03d}.qnt' for i in range(78)] + [
        'specs/quint/s02/'+n+'.qnt' for n in ['candidate_a_integrated_wrappers_typecheck','candidate_a_integrated_export_test',
                                            'candidate_a_authority_installment_test','candidate_a_authority_swap_test']]
    while pending:
        name = pending.pop()
        if name in seen: continue
        seen.add(name)
        for imp in re.findall(r'^\s*import\s+[^\n]*?\s+from\s+"([^"]+)"',src(name),re.M):
            need(imp.startswith('./'), 'local imports')
            child = posixpath.normpath(str(PurePosixPath(name).parent/(imp+'.qnt')))
            need(child.startswith('specs/quint/s02/'), 'closure escape'); pending.append(child)
    need(seen == set(source) and len(python_names) == 17 and len(seen-set(python_names)) == 104, 'archived source closure')
    need(source['specs/quint/s02/candidate_a_integrated_case_032.qnt'] == 'e9f4d1e63b4ca0d19725e26317a878fbb9f6dc949d0609bdb265b7b5805b267c' and
         source['specs/quint/s02/candidate_a_integrated_lowering.qnt'] == 'a6dac06db7bb10cab7b33dc8ac63e2b18f88c12e4fd52066d0e2614e15b7c322', 'two final-source changes')
    row = d['row']
    need(row == {'case_id':'installment/recover-r1-timeout100/ordinary/SignAfterResolve','lifecycle':'installment',
        'profile':'SignAfterResolve','scenario':'recover-r1-timeout100','control':'ordinary','event_count':28,
        'global_index':14,'entry':'specs/quint/s02/candidate_a_integrated_case_014.qnt',
        'input_path':'raw/case-014.itf.json','case_path':'cases/case-014.json'}, 'exact native row014')
    requested = ['quint','run',row['entry'],'--backend=rust','--seed=42','--max-samples=1','--n-traces=1','--max-steps=27',
                 '--invariants','noDiagnosticA4','sourceInvariantA4','--witnesses','completeA4','--out-itf',INNER+'case-014.itf.json']
    child = [PYTHON,'-B','-X','pycache_prefix='+INNER+'python-cache',RECORDER,'--stage','case-014-export','--',*requested]
    parent = [PYTHON,'-B','-X','pycache_prefix='+OUTER.rstrip('/')+'-parent-cache',RH,'--receipt-dir',OUTER.rstrip('/'),
              '--inner-receipt',INNER+'receipt.json','--wall-seconds','900','--',*child]
    need(rec['command'] == requested and rec['executed_command'] == [NODE,CLI,*requested[1:]], 'exact requested/actual native command')
    need(launch['child_argv'] == child and launch['argv'] == ['/usr/bin/time','-v','-o',absolute(OUTER+'resources.txt'),*child], 'measured argv')
    need(launch['original_parent_argv'] == parent and launch['wrapper_argv'] == parent[4:], 'parent argv')
    need(launch['inner_receipt'] == absolute(INNER+'receipt.json') and launch['cwd'] == rec['cwd'] == d['cwd'] == ROOT, 'cwd/receipt')
    need(rec['python_cache_prefix'] == absolute(INNER+'python-cache') and rec['python_bytecode_writes'] is False, 'cache/bytecode')
    need(d['extraNodeHeapOverride'] is False and not any('max-old-space-size' in x for x in rec['executed_command']), 'no explicit heap argv')
    tree = ast.parse(src(RECORDER))
    envfn = next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name == 'controlled_environment')
    need(any(isinstance(n,ast.Constant) and n.value == 'NODE_OPTIONS' for n in ast.walk(envfn)), 'archived NODE_OPTIONS clearing source')
    need((d['wallSeconds'],launch['wall_seconds'],term['wall_seconds']) == (900,900,900) and
         launch['grace_seconds'] == launch['cleanup_seconds'] == 5, 'wall/cleanup budgets')
    need(launch['fixed_environment'] == {'LC_ALL':'C','NODE_DISABLE_COMPILE_CACHE':'1','PYTHONDONTWRITEBYTECODE':'1','PYTHONNOUSERSITE':'1'}, 'fixed environment')
    need(rec['source_stable'] is True and rec['runtime_stable'] is True and a['sourceStable'] is True and a['runtimeStable'] is True, 'stable endpoints')
    need(rec['exit_code'] == -6 and rec['recorder_exit_code'] == term['actual_exit'] == term['return_code'] == 134, 'failed exit distinctions')
    need(a['actualExits'] == {'Node':-6,'recorder':134,'RH002':134,'outerTool':134}, 'root failure exits')
    need(term['eligible'] is False and term['timed_out'] is False and term['resource_complete'] is True and
         term['source_runtime_stable'] is True and all(term[k] is None for k in ['launch_error','monitor_error','resource_error']), 'admitted failure predicates')
    need(term['cleanup'] == a['cleanup'] == {'complete':True,'errors':[],'forced':False,'signals':[]}, 'unforced complete cleanup')
    need(term['inner_receipt_present'] is True and term['inner_receipt_complete'] is False and
         term['inner_receipt_independent_validation_required'] is True, 'original inner flags')
    pin(INNER+'receipt.json',term['inner_receipt_sha256'])
    for n,h in term['sidecar_sha256'].items():pin(n,h)
    pin(INNER+'stderr.bin','6ab5a839fdf570641f6df939a1cd9c036152004336565da901c00a1eeb7a896e',1510)
    need(get(INNER+'stderr.bin') == original_index['stderrOriginalUtf8'].encode() and
         b'Allocation failed - JavaScript heap out of memory' in get(INNER+'stderr.bin'), 'original OOM stderr')
    need(get(INNER+'stdout.bin') == get(OUTER+'stderr.bin') == b'' and rec['artifacts'] == {}, 'empty stdout/no ITF artifacts')
    need(not any(n.startswith(INNER) and n.endswith('.itf.json') for n in data), 'original stage contains no ITF')
    need(original_index['noITF'] is True and original_index['case044DestinationsAbsent'] is True and a['noITF'] is True, 'recorded noITF/no044')
    for c in ['stdout','stderr']:pin(INNER+c+'.bin',rec[c+'_sha256'])
    need(read(OUTER+'stdout.bin') == {'stage':absolute(INNER.rstrip('/')),'exit_code':-6,'source_stable':True,
         'runtime_stable':True,'recorder_exit_code':134,'artifacts':{}}, 'authentic recorder result')
    transport = read(TRANSPORT+'terminal.json'); command = read(TRANSPORT+'command.json')
    need(command['command'] == command['args']['cmd'] == transport['command'] == d['command'] and
         shlex.split(d['command']) == parent and d['command'] in get(S+'a4-largest-pilot-readiness.md').decode(), 'exact approved command')
    need(command['cwd'] == command['args']['workdir'] == transport['cwd'] == ROOT and transport['wrapperExitCode'] == 134, 'actual outer134')
    responses = [read(TRANSPORT+f'response-{i:03d}.json') for i in range(37)]
    need(responses == transport['toolResponses'] and responses[0]['chunk_id'] == 'e3a437' and
         responses[-1]['chunk_id'] == '8cda68' and responses[-1]['exit_code'] == 134, '37 original responses')
    need(all(x.get('session_id') == 69003 and 'exit_code' not in x for x in responses[:-1]), 'no intermediate terminal')
    fields = literal_assignment(get(RH).decode(),'FIELDS'); values = {}; diagnostics = []
    for line in get(OUTER+'resources.txt').decode().splitlines():
        line = line.strip(); matches = [k for k in fields if line.startswith(k+': ')]
        if not matches:diagnostics.append(line);continue
        need(len(matches) == 1 and matches[0] not in values, 'unique resource field')
        k = matches[0]; values[k] = line[len(k)+2:]
    need(len(values) == 23 and set(values) == set(fields) and values == term['resource']['fields'], '23 resource fields')
    need(diagnostics == term['resource']['diagnostics'] == ['Command exited with non-zero status 134'], 'original GNU-time diagnostic')
    need(values['Command being timed'] == '"'+' '.join(child)+'"' and values['Exit status'] == '134' and term['resource']['exit_status'] == 134, 'resource command/exit')
    need(values['Elapsed (wall clock) time (h:mm:ss or m:ss)'] == '5:56.80' and values['Maximum resident set size (kbytes)'] == '4371180' and
         values['User time (seconds)'] == '394.76' and values['System time (seconds)'] == '16.91' and
         term['resource']['maximum_rss_kib'] == 4371180, 'original resource observations')
    need(rec['elapsed_ns'] == rec['ended_ns']-rec['started_ns'] == 352649572797, 'Node subprocess elapsed')
    prod = read(S+'a4-producer-dispatch.json'); shared = read(S+'a5-factoring-receipts/dispatch.json'); manifest = read(STORE+'manifest.json')
    need(rec['beforeDispatchCommit'] == prod['beforeDispatchCommit'] == '386bf0ae10f767e051b414a7231be105cc4b0f71', 'producer base')
    shared_expected = {'beforeDispatchCommit':'900bb2051225b4a3d99bf422c3b2e5e386e3e7bc','dispatchSha256':sha(get(S+'a5-factoring-receipts/dispatch.json')),
                       'runtimeArchiveSha256':shared['runtimeArchiveSha256'],'runtimeManifestSha256':shared['runtimeManifestSha256']}
    need(rec['shared_runtime_before'] == rec['shared_runtime_after'] == parser_admission['sharedRuntimeReferences'] == shared_expected and
         shared['beforeDispatchCommit'] == shared_expected['beforeDispatchCommit'], 'closed shared references')
    need(rec['runtime_manifest_sha256'] == prod['runtimeManifestSha256'] == shared['runtimeManifestSha256'] == sha(get(STORE+'manifest.json')) and
         rec['runtime_archive_sha256'] == prod['runtimeArchiveSha256'] == shared['runtimeArchiveSha256'] == manifest['archiveSha256'], 'runtime pins')
    expected = {absolute(S+'a4-producer-dispatch.json'):sha(get(S+'a4-producer-dispatch.json')),
                absolute(S+'a5-factoring-receipts/dispatch.json'):prod['sharedDispatchSha256'],absolute(HELPER):prod['runtimeHelperSha256'],
                absolute(STORE+'manifest.json'):prod['runtimeManifestSha256'],absolute(STORE+'runtime.tar.gz'):prod['runtimeArchiveSha256'],
                manifest['reusedPythonManifest']:manifest['reusedPythonManifestSha256'],manifest['reusedPythonArchive']:manifest['reusedPythonArchiveSha256']}
    for row in manifest['files']+manifest['pythonFiles']:
        need(row['path'] not in expected or expected[row['path']] == row['sha256'], 'runtime conflict'); expected[row['path']] = row['sha256']
    for row in manifest['versions']:
        for stream in ['stdout','stderr']:
            name = STORE+row['name']+'.'+stream; pin(name,row[stream+'Sha256'])
            need(absolute(name) not in expected or expected[absolute(name)] == row[stream+'Sha256'], 'version conflict')
            expected[absolute(name)] = row[stream+'Sha256']
    python_manifest = read(manifest['reusedPythonManifest'])
    need(python_manifest['sourceBytes'] == manifest['pythonFiles'] and python_manifest['archiveSha256'] == manifest['reusedPythonArchiveSha256'], 'Python closure')
    pin(manifest['reusedPythonManifest'],manifest['reusedPythonManifestSha256'])
    need(len(expected) == 8102 and len(manifest['treeMembers']) == 4 and len(manifest['dynamicLibraryReceipts']) == 124, 'runtime inventory sizes')
    need(rec['runtime_before'] == rec['runtime_after'] == d['runtimeExpected'] == {'files':expected,'treeMembers':manifest['treeMembers']}, 'full recorded runtime endpoints')
    for name,h in expected.items():
        if name.startswith(ROOT+'/') and name[len(ROOT)+1:] in data: pin(name,h)
    need(launch['source_runtime_before'] == term['source_runtime_before'] == term['source_runtime_after'], 'outer endpoint pins')
    for name,h in term['source_runtime_before'].items():
        if name == absolute(RH): pin(name,h)
        else: need(expected[name] == h, 'outer executable runtime binding')
    need(get(INNER+'runtime-helper.py') == get(HELPER) and get(OUTER+'runner.py') == get(RH), 'original helper copies')
    for name,tool in rec['tools'].items(): need(expected[tool['path']] == tool['sha256'], 'actual executable binding')
    for name in ['node','quint','python']: need(rec['tools'][name]['version_exit'] == 0, 'successful version probe')
    need(rec['tools']['node']['version_stdout'] == 'v24.18.1\n' and rec['tools']['quint']['version_stdout'] == '0.32.0\n' and
         rec['tools']['python']['version_stdout'] == 'Python 3.13.14\n', 'version values')
    need(rec['tools']['rust']['version_exit'] == 1 and rec['tools']['rust']['version_stdout'] == '' and
         rec['tools']['rust']['version_stderr'].encode() == get(STORE+'rust.stderr'), 'original unsupported Rust probe')
    external = {r['path']:r for r in index['externalRuntimeArchives']}
    need(len(external) == 2 and external == {
        STORE+'runtime.tar.gz':{'path':STORE+'runtime.tar.gz','bytes':315704660,'sha256':'f0687656d30c0d59ece8dfd027508a9228020506d29b97aea3abe7568186c31c'},
        S+'a4-checker-task1-receipts/python-environment.tar.gz':{'path':S+'a4-checker-task1-receipts/python-environment.tar.gz','bytes':66716078,
        'sha256':'7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac'}}, 'external archive declarations')
    for name,row in external.items(): need(name not in data and expected[absolute(name)] == row['sha256'], 'external exclusion/binding')
    need(parser_admission['runtimePins'] == d['runtimeExpected'], 'admitted parser/native runtime identity')
    dependency = index['externalParserPackage']
    parser_index_name = 'evidence/s02-candidate-a-completion/a4/final-source-parser/index.json'
    pin(parser_index_name,'19348e1089004e998e096b3c59dff2cbc7888a98a5b31fd7ae89db5dc6a00932',78831)
    parser_index = read(parser_index_name)
    need(dependency == {'archive':{'path':'evidence/s02-candidate-a-completion/a4/final-source-parser/original-evidence.tar.gz',
        'bytes':5434195,'sha256':'7233f85c03bfacf80d83d50a657a10f05a23d4863c157ecc81a4c37b13ee8119'},
        'index':{'path':parser_index_name,'bytes':78831,'sha256':sha(get(parser_index_name))}}, 'parser dependency declarations')
    need(parser_index['archive'] == {'path':'original-evidence.tar.gz','bytes':5434195,
        'sha256':dependency['archive']['sha256']} and dependency['archive']['path'] not in data, 'external parser archive binding')
    ir = parser_admission['originalIR']; ir_relative = relative(ir['path'])
    members = {r['path']:r for r in parser_index['members']}
    need(len(members) == len(parser_index['members']) == 290 and ir_relative not in data and
         members[ir_relative] == {'path':ir_relative,'bytes':38527047,'sha256':'cbf57ad5a7f194ba62bf897553d3750ddba899190683ab8293cc633282886e1e'} and
         ir == {'path':absolute(ir_relative),'bytes':38527047,'sha256':members[ir_relative]['sha256']}, 'external raw IR binding; no duplicate IR')
    parser_admission_name = S+'a4-final-source-parser-admission-20260906.json'
    need(members[parser_admission_name]['sha256'] == sha(get(parser_admission_name)), 'same parser admission bytes')
    need(a['failure'] == 'JavaScript heap out of memory' and a['internalPhase'] == index['internalPhase'] == 'unknown', 'failure scope')
    for record in [a,index]:
        need(record['canonicalStageConsumed'] is True and all(record[k] is False for k in
             ['pilotAccepted','case044Authorized','full78Authorized','structuralIntakePerformed']), 'failed gate remains closed')
    need(index['explicitNodeHeapFlag'] is False and index['selfAndLaterReceiptHashesExcluded'] is True and
         index['archivedCodeExecuted'] is False and index['externalArchivesOrRuntimeRehashed'] is False and
         index['originalDispatchHead'] == HEAD and index['packagingHeadObserved'] == '4006244885c7cf34562fe54cbb76856142b0d244' and
         index['packagingHeadObservationToolChunk'] == 'f8db22', 'preservation scope/current head distinction')
    print(json.dumps({'ok':True,'scope':'offline failed014 preservation; recorded external dependencies only',
        'members':325,'sourceCopies':242,'sources':121,'runtimePins':8102,'runtimeTrees':4,'transportResponses':37,
        'actualExits':a['actualExits'],'failure':'JavaScript heap out of memory','noITF':True,'eligible':False,
        'timedOut':False,'internalPhase':'unknown','explicitNodeHeapFlag':False,'pilotAccepted':False,
        'case044Authorized':False,'structuralIntakePerformed':False,'externalArchivesOrRuntimeRehashed':False,
        'archivedCodeExecuted':False}))

if __name__ == '__main__':
    main()
