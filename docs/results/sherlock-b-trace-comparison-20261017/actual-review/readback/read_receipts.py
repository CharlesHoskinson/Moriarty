"""R61 DATA reader."""
from pathlib import Path
import hashlib, json, os, re
R=Path(__file__).resolve().parent.parent
A=R / '61-b-pairwise-actual/attempt-01'
inputs={}
def raw(p):
    b=p.read_bytes()
    inputs[str(p.relative_to(R))]={'bytes': len(b), 'sha256': hashlib.sha256(b).hexdigest()}
    return b
def load(p): return json.loads(raw(p))
def txt(p): return raw(p).decode('utf-8')
def key(p): return os.path.normpath(str(p))
def rows(xs): return {key(x['path']): x for x in xs}
def outs(result):
    return {n: {'bytes_match': len(raw(Path(result[n]['path'])))==result[n]['bytes'],
                'sha_match': hashlib.sha256(raw(Path(result[n]['path']))).hexdigest()==result[n]['sha256']}
            for n in ('stdout', 'stderr')}
pre=load(A/'PREEXEC-INVENTORY.json'); post=load(A/'POSTEXEC-INVENTORY.json')
adopt=load(A/'OWNER-CLASSIFICATION-ADOPTION.json'); summary=load(A/'RESULT.json')
sources=rows(pre['sources']); deps=rows(pre['normal_dependency_inventory'])
prior=load(R/'46-b-observer-prospective-tests/attempt-01/PREEXEC-INVENTORY.json')
prior_tools=[prior['rustc']] + prior['rustc_driver'] + prior['target_library_inventory'] + prior['linker_tools']
frozen=load(R/'49-b-complete-pairwise-source/CANDIDATE-FREEZE-02.json')
source_checks=[]
for f in frozen['files']:
    p=R/'49-b-complete-pairwise-source/candidate'/f['path']; b=raw(p)
    row=next(x for x in pre['sources'] if x['path'].endswith('/'+f['path']))
    source_checks.append({'path': f['path'], 'frozen_match':
        f['bytes']==row['bytes']==len(b) and f['sha256']==row['sha256']==hashlib.sha256(b).hexdigest()})
reg=load(R/'55-b-pairwise-registration/REGISTRATION.json')
mod=next(x for x in pre['sources'] if x['path'].endswith('/sherlock/helper_machine/mod.rs'))
old=load(R/'55-b-pairwise-review/INPUT-MEMBERSHIP.json')
refs=[x for x in old['files'] if '/.worktrees/' in x['path']]
old_missing=[x['path'] for x in refs if key(x['path']) not in sources]
old_mismatch=[x['path'] for x in refs if key(x['path']) in sources and
    (x['bytes'], x['sha256'])!=(sources[key(x['path'])]['bytes'], sources[key(x['path'])]['sha256'])]
phases=[]; selectors=[]; previous_end=None
for name in ['build', 'test-compile', 'list'] + [f'test-{i}' for i in range(1,10)]:
    p=load(A/(name+'-PREEXEC.json')); q=load(A/(name+'-RESULT.json')); argv=q['argv']
    cap='120s' if name=='build' else '30s' if name=='test-compile' else '10s'
    selector=argv[argv.index('--exact')+1] if '--exact' in argv else None
    if selector is not None: selectors.append(selector)
    phases.append({'phase': name, 'argv_equal': p['argv']==argv, 'exit': q['exit'], 'seconds': q['seconds'],
        'serial': previous_end is None or previous_end <= p['utc'],
        'env_cwd_match': p['cwd']==pre['cwd'] and p['environment']==pre['environment'],
        'rehash_record': p['sources_rehashed_unchanged'], 'timeout_match': argv[:3]==['/usr/bin/timeout','--signal=KILL',cap],
        'serial_flag': '--test-threads=1' in argv if selector else None, 'logs': outs(q)})
    previous_end=q['utc']
listing=txt(A/'list.stdout'); listed=[s[:-6] for s in listing.splitlines() if s.endswith(': test')]
logs=[txt(A/f'test-{i}.stdout') for i in range(1,10)]
agg=load(A/'TEST-RESULT.json')
def depinfo(name):
    paths=sorted({key(p) for p in re.findall(r'/[^\s:]+\.rs', txt(A/name))})
    return {'unique_rs_paths': len(paths), 'unpinned': [p for p in paths if p not in sources]}
build=load(A/'build-PREEXEC.json')['argv']; compile_argv=load(A/'test-compile-PREEXEC.json')['argv']
externs=[build[i+1].split('=',1)[1] for i,x in enumerate(build) if x=='--extern']
publication=load(R/'64-b-pairwise-publication-proposal/docs/results/sherlock-b-trace-comparison-20261017/ACTUAL-PUBLICATION-SELECTION.json')
pub_bad=[]
for f in publication['files']:
    b=raw(Path(f['source']))
    if len(b)!=f['bytes'] or hashlib.sha256(b).hexdigest()!=f['sha256']: pub_bad.append(f['published'])
result={'scope':'Literal DATA.',
 'C1': {'source_commit': pre['source_commit'], 'base':pre['reviewed_source_base'],
  'source_count': len(pre['sources']), 'post_changes': post['source_changes'],
  'post_count': post['source_files_unchanged'], 'new_source': source_checks,
  'module_match': mod['sha256']==reg['registration']['after_sha256'],
  'R55_refs':len(refs),'R55_noncompiled_refs':old_missing,'R55_mismatches':old_mismatch,
  'core.d': depinfo('core.d'), 'tests.d': depinfo('tests.d'), 'depinfo_record': load(A/'DEPINFO-SCOPE.json')},
 'C2': {'pre_count': len(deps), 'post_count': len(post['normal_dependency_inventory']),
  'records_equal': pre['normal_dependency_inventory']==post['normal_dependency_inventory'],
  'stale_libmori_core': [p for p in deps if Path(p).name.startswith('libmori_core')],
  'externs': len(externs), 'extern_unpinned': [p for p in externs if key(p) not in deps]},
 'C3': {'records': len(pre['toolchain_linker_sysroot']), 'R46_equal': pre['toolchain_linker_sysroot']==prior_tools,
  'sysroot': sum('/lib/rustlib/x86_64-unknown-linux-gnu/lib/' in x['path'] for x in pre['toolchain_linker_sysroot'])},
 'C4_C5': {'phases': phases, 'approved':adopt['selectors'], 'list_equal':sorted(listed)==sorted(adopt['selectors']), 'run_order_equal':selectors==adopt['selectors'],
  'same9': len(listed)==len(selectors)==len(adopt['selectors'])==9 and set(listed)==set(selectors)==set(adopt['selectors']),
  'raw_pass_lines': [bool(re.search(r'1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out',s)) for s in logs],
  'aggregate':[agg['completed'],agg['tests_passed']],
  'build_debug0': build[-2:]==['-C','debuginfo=0'], 'test_debug0': compile_argv[-2:]==['-C','debuginfo=0'],
  'envelope':[pre['environment'],pre['cwd']],
  'artifacts': [load(A/'BUILD-ARTIFACT.json'),load(A/'TEST-ARTIFACT.json')],
  'artifact_flags':[post['binary_unchanged'],post['rlib_unchanged']]},
 'summary_record':summary, 'publication': {'files': len(publication['files']), 'pin_mismatches': pub_bad},
 'input_files': inputs}
print(json.dumps(result, separators=(',',':')))
