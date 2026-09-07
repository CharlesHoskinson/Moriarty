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
INNER = S+'a4-producer-receipts/task6-final-source-aggregate-parse-20260906/'
OUTER = S+'a4-final-source-aggregate-parse-20260906/'
TRANSPORT = S+'a4-final-source-parser-transport-20260906/'
PLAN = 'docs/superpowers/plans/2026-09-06-candidate-a-final-source-parser-capture.md'
PROPOSAL = S+'a4-final-source-parser-package-proposal-20260906.md'
PROPOSAL_SHA = 'b1a3bc6facb99e14dc94ab08bf4f4c20d988b7b6682587e2e40f5d26d67fd763'
ADMISSION = S+'a4-final-source-parser-admission-20260906.json'
REPORT = S+'a4-final-source-parser-independent-intake-20260906.md'
DISPATCH = S+'a4-final-source-parser-dispatch-20260906.json'
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
    need(index['schema'] == 'moriarty.a4-final-source-parser-preservation/v1', 'schema')
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
            need(0 <= member.size <= 134217728 and len(data) < 290, 'member bound')
            total += member.size; need(total <= 128*1024*1024, 'archive total bound')
            chunks, size, digest = [], 0, hashlib.sha256()
            with tar.extractfile(member) as stream:
                while chunk := stream.read(1024*1024):
                    size += len(chunk); need(size <= member.size, 'member overflow')
                    digest.update(chunk); chunks.append(chunk)
            content = b''.join(chunks)
            need(size == member.size and digest.hexdigest() == sha(content), 'member length/hash'); data[name] = content
    rows = index['members']; pins = {r['path']: r for r in rows}
    need(len(rows) == len(pins) == len(data) == 290 and set(pins) == set(data), 'exact indexed members')

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
    pin(ADMISSION,'29a427818e9894ce6674e9f4f26dfe4527736fa6289bde6b5b3a5e6664caec71')
    pin(REPORT,'c1d219a8dc772807f6b729f328eecee0998d9c4a526ad1e87add46805fbf06f4')
    a = read(ADMISSION)
    blocks = [decode(b.encode()) for b in re.findall(r'```json\n(.*?)\n```',get(REPORT).decode(),re.S)]
    need(len(blocks) == 4 and blocks[1]['chunk_id'] == '40b973' and blocks[1]['session_id'] == 21304 and
         blocks[2]['chunk_id'] == '02536f' and blocks[2]['exit_code'] == 0, 'independent actual audit responses')
    need(blocks[3]['result']['chunk_id'] == 'f5dd8e' and blocks[3]['result']['exit_code'] == 0, 'supplement terminal')
    stdout = blocks[2]['output'].encode()
    need(sha(stdout) == '7c9bb66987de1c5bd30f0f53d23fa189bc6dbb923b90ce1a07d22897b6f058e0', 'actual audit stdout')
    need(sha(json.dumps(blocks[2],sort_keys=True,separators=(',',':')).encode()) ==
         'f50f841739f2ab8f5d15c8a6f88f1431fa03ddc2aa0a2e106ae6c8f6cfecc569', 'recorded response canonical digest')
    observed = decode(stdout)
    need(observed['originals'] == a['originals'] and len(a['originals']) == 263, 'original index agreement')
    for name,row in a['originals'].items(): pin(name,row['sha256'],row['bytes'])
    extras = re.findall(r'^\| `((?:S/|evidence/|scripts/)[^`]+)` \| `([0-9a-f]{64})` \|$',get(PROPOSAL).decode(),re.M)
    need(len(extras) == 26, 'proposal additions')
    expected_members = set(a['originals']) | {PROPOSAL}
    for name,h in extras:
        name = S+name[2:] if name.startswith('S/') else name
        need(name not in expected_members, 'duplicate extra'); expected_members.add(name); pin(name,h)
    need(set(data) == expected_members, 'closed proposal union')
    d = read(DISPATCH); rec = read(INNER+'receipt.json'); term = read(OUTER+'terminal.json'); launch = read(OUTER+'launch.json')
    pin(PLAN,'9a871c550813c9db9fb0c85fc4759b3f41780c69f59a42b49888aca6672841e5')
    pin(DISPATCH,'8ec40238528bc59a1962ce6a7ddd4e0345c2362d0eedff55278c5c2bc45f8ee9')
    need(get(S+'a4-final-source-parser-plan-root-review-20260906.md').startswith(b'PASS\n'), 'root plan adoption')
    for name,h in d['bindings'].items(): pin(name,h)
    need(d['actualDispatchHead'] == a['actualDispatchHead'] == rec['sourceCommit'] == rec['sourceCommitAfter'] == HEAD, 'source commits')
    source = d['sources']
    need(len(source) == d['sourceCount'] == 121 and a['sources'] == rec['sources_before'] == rec['sources_after'] == source, '121 sources')
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
    native = [NODE,'--max-old-space-size=4096',CLI,'parse','specs/quint/s02/candidate_a_integrated_wrappers_typecheck.qnt','--out',absolute(INNER+'aggregate-ir.json')]
    child = [PYTHON,'-B','-X','pycache_prefix='+absolute(INNER+'python-cache'),absolute(RECORDER),'--stage','task6-final-source-aggregate-parse-20260906','--',*native]
    parent = [PYTHON,'-B','-X','pycache_prefix='+absolute(OUTER.rstrip('/')+'-parent-cache'),absolute(RH),
              '--receipt-dir',absolute(OUTER.rstrip('/')),'--inner-receipt',absolute(INNER+'receipt.json'),
              '--wall-seconds','900','--cwd',ROOT,'--',*child]
    need(rec['command'] == rec['executed_command'] == native and rec['cwd'] == launch['cwd'] == ROOT, 'native argv/cwd')
    need(launch['child_argv'] == child and launch['argv'] == ['/usr/bin/time','-v','-o',absolute(OUTER+'resources.txt'),*child], 'measured argv')
    need(launch['original_parent_argv'] == parent and launch['wrapper_argv'] == parent[4:], 'original Python argv')
    need(rec['python_cache_prefix'] == absolute(INNER+'python-cache') and rec['python_bytecode_writes'] is False, 'cache/bytecode')
    need((d['outerWallSeconds'],d['nodeHeapMiB'],d['parserAcceptanceBytes']) == (900,4096,134217728) and
         launch['wall_seconds'] == term['wall_seconds'] == 900 and launch['grace_seconds'] == launch['cleanup_seconds'] == 5, 'budgets')
    need(launch['fixed_environment'] == {'LC_ALL':'C','NODE_DISABLE_COMPILE_CACHE':'1','PYTHONDONTWRITEBYTECODE':'1','PYTHONNOUSERSITE':'1'}, 'fixed environment')
    for key in ['source_stable','runtime_stable']: need(rec[key] is True, key)
    need(rec['exit_code'] == rec['recorder_exit_code'] == term['actual_exit'] == term['return_code'] == 0, 'four receipt exits')
    need(term['eligible'] is True and term['resource_complete'] is True and term['source_runtime_stable'] is True and
         term['timed_out'] is False and all(term[k] is None for k in ['launch_error','monitor_error','resource_error']), 'outer eligibility')
    need(term['cleanup'] == {'complete':True,'errors':[],'forced':False,'signals':[]}, 'unforced cleanup')
    need(term['inner_receipt_present'] is True and term['inner_receipt_complete'] is False and term['inner_receipt_independent_validation_required'] is True, 'original inner flags')
    pin(INNER+'receipt.json',term['inner_receipt_sha256'])
    for name,h in term['sidecar_sha256'].items(): pin(name,h)
    need(rec['artifacts'] == {} and get(INNER+'stdout.bin') == get(INNER+'stderr.bin') == get(OUTER+'stderr.bin') == b'', 'original empty streams/artifacts')
    for channel in ['stdout','stderr']: pin(INNER+channel+'.bin',rec[channel+'_sha256'])
    need(read(OUTER+'stdout.bin') == {'stage':absolute(INNER.rstrip('/')),'exit_code':0,'source_stable':True,'runtime_stable':True,'recorder_exit_code':0,'artifacts':{}}, 'recorder result')
    transport = read(TRANSPORT+'terminal.json'); command = read(TRANSPORT+'command.json')
    expected_command = ['PYTHONOPTIMIZE=0',*parent]
    need(shlex.split(transport['command']) == expected_command and command['command'] == command['args']['cmd'] == transport['command'], 'exact transport command')
    need(command['cwd'] == command['args']['workdir'] == transport['cwd'] == ROOT and transport['wrapperExitCode'] == 0, 'actual outer terminal')
    responses = [read(TRANSPORT+f'response-{i:03d}.json') for i in range(3)]
    need(responses == transport['toolResponses'] and [v['chunk_id'] for v in responses] == ['f81ea6','2535e8','88c051'] and
         responses[0]['session_id'] == responses[1]['session_id'] == 54358 and responses[2]['exit_code'] == 0, 'three actual responses')
    # GNU-time names come from archived constants, never an imported parser.
    fields = literal_assignment(get(RH).decode(),'FIELDS')
    values = {}
    for line in get(OUTER+'resources.txt').decode().splitlines():
        matches = [k for k in fields if line.strip().startswith(k+': ')]
        need(len(matches) == 1 and matches[0] not in values, 'unique GNU-time field')
        k = matches[0]; values[k] = line.strip()[len(k)+2:]
    need(len(values) == 23 and set(values) == set(fields) and values == term['resource']['fields'], 'all GNU-time fields')
    need(values['Command being timed'] == '"'+' '.join(child)+'"' and values['Exit status'] == '0', 'resource command/exit')
    need(values['Elapsed (wall clock) time (h:mm:ss or m:ss)'] == '0:16.09' and values['Maximum resident set size (kbytes)'] == '968996', 'resource observations')
    need(term['resource']['diagnostics'] == [] and term['resource']['exit_status'] == 0 and term['resource']['maximum_rss_kib'] == 968996, 'parsed resource')
    need(rec['elapsed_ns'] == rec['ended_ns']-rec['started_ns'] == 12130368479, 'inner elapsed')
    prod = read(S+'a4-producer-dispatch.json'); shared = read(S+'a5-factoring-receipts/dispatch.json'); manifest = read(STORE+'manifest.json')
    need(rec['beforeDispatchCommit'] == prod['beforeDispatchCommit'] == '386bf0ae10f767e051b414a7231be105cc4b0f71', 'producer base')
    shared_expected = {'beforeDispatchCommit':'900bb2051225b4a3d99bf422c3b2e5e386e3e7bc','dispatchSha256':sha(get(S+'a5-factoring-receipts/dispatch.json')),
                       'runtimeArchiveSha256':shared['runtimeArchiveSha256'],'runtimeManifestSha256':shared['runtimeManifestSha256']}
    need(rec['shared_runtime_before'] == rec['shared_runtime_after'] == a['sharedRuntimeReferences'] == shared_expected and
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
    need(rec['runtime_before'] == rec['runtime_after'] == a['runtimePins'] == {'files':expected,'treeMembers':manifest['treeMembers']}, 'full recorded runtime endpoints')
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
    ir = get(INNER+'aggregate-ir.json')
    need(len(ir) == 38527047 and sha(ir) == 'cbf57ad5a7f194ba62bf897553d3750ddba899190683ab8293cc633282886e1e', 'original IR bytes')
    parsed = decode(ir,134217728)
    need(set(parsed) == {'stage','warnings','modules','table','errors'} and parsed['stage'] == 'parsing' and
         parsed['warnings'] == parsed['errors'] == [] and len(parsed['modules']) == 99, 'IR schema')
    types = {}
    for module in parsed['modules']:
        for decl in module['declarations']:
            if decl['kind'] == 'typedef':
                need(decl['name'] not in types, 'unique typedef'); types[decl['name']] = module['name']
    required = {n:'candidate_a_integrated_observer' for n in ['A4Arguments','A4Command','A4Computation','A4Event']}
    required.update(AAuthorityExecution='candidate_a_authority_adapter',AProgram='candidate_a_types',AState='candidate_a_types')
    need(len(types) == 122 and {n:types[n] for n in required} == required, '122 typedefs/seven names')
    need(a['originalIR'] == {'path':absolute(INNER+'aggregate-ir.json'),'bytes':len(ir),'sha256':sha(ir)} and
         a['parserPredicates'] == {'strictJSON':True,'exactSchema':True,'uniqueTypedefs':122,'requiredTypes':required,'acceptanceBytes':134217728}, 'admitted IR predicates')
    dc = read(S+'a4-final-source-parser-data-check-tool-20260906.json')
    need(dc['result']['chunk_id'] == '7660c0' and dc['result']['exit_code'] == 0, 'actual root data check')
    plan_blocks = re.findall(r'```sh\n(.*?)```',get(PLAN).decode(),re.S)
    need(dc['args']['cmd'] == plan_blocks[-1], 'exact planned data check')
    need(transport['command'] in get(PLAN).decode(), 'native command occurs in adopted plan')
    need('root81c702 refused interim review hash' in a['rootIntakeHistory'] and 'c1d219a8' in a['rootIntakeHistory'], 'root refusal history')
    for record in [a,index]:
        need(all(record[k] is False for k in ['nativePilotAccepted','full78ExporterAccepted','finalA4Accepted']) and record['futureFreshParserCallsRequired'] is True, 'scope limits')
    need(index['selfAndLaterReceiptHashesExcluded'] is True and index['archivedCodeExecuted'] is False and
         index['currentRuntimeOrExternalArchivesRehashed'] is False and index['originalDispatchHead'] == HEAD and
         index['packagingHeadObserved'] == '4006244885c7cf34562fe54cbb76856142b0d244' and
         index['packagingHeadObservationToolChunk'] == '3ec86b', 'preservation scope')
    print(json.dumps({'ok':True,'scope':'offline preserved parser evidence; external runtime bindings only',
                      'members':len(data),'sourceCopies':242,'sources':121,'runtimePins':8102,'runtimeTrees':4,
                      'irBytes':len(ir),'irSha256':sha(ir),'modules':99,'uniqueTypedefs':122,'requiredTypes':required,
                      'actualOriginalOuterExit':0,'archivedCodeExecuted':False,'currentRuntimeOrExternalArchivesRehashed':False,
                      'nativePilotAccepted':False,'full78ExporterAccepted':False,'finalA4Accepted':False}))

if __name__ == '__main__':
    main()
