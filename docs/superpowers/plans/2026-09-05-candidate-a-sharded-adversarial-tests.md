# Candidate A Task6 Case-Sharded Tests Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Root owns independent review, edit-window release, evidence admission, and integration.

**Goal:** Adapt every adopted Task6 control to bounded, file-backed case shards without replacing native package acceptance with synthetic tests.

**Architecture:** Append tests and helper functions to the existing pinned checker test module. Each synthetic triple runs in its own subprocess, writes incremental files, establishes complete raw linkage, and then checks semantic rejection and two passing controls. Full-package controls use the actual admitted native files and small temporary manifest/admission copies; they do not materialize native traces in memory.

**Tech Stack:** Existing Python 3.13 environment, pytest, admitted strict streaming utility, independently admitted checker. No new dependency, producer import, or semantic oracle.

## Global constraints

- PLAN ONLY. This ignored document authorizes no implementation before root review.
- Controlling source: adopted independent-replay Task6, immutable replay-interface addendum, and case-sharded transport section6. The corrected inventory is exactly78 cases/1557 events, with canonical SHA2568a1a6136afc9fda3c29e050df8b80144750cc21365e9194924401c155dc6402b.
- Only tracked implementation ownership is tests/test_s02_candidate_a_integrated.py. A temporary single-line behavioral RED in scripts/check_s02_candidate_a_integrated.py requires root's explicit source-edit window and must restore the exact admitted bytes.
- No producer, utility, frozen Core/common/A, authority, case, or inventory source edits. No helper module is introduced.
- All package and authority semantics remain independent from producer code. Shared utility is disclosed lexical transport only.
- No pytest-xdist, parallel triples, skip, xfail, synthetic replacement of native files, or subset final acceptance.
- Root controls commits. Implementation worker stops after each gate; no automatic commit commands are authorized.
- Prior sharded report's narrative dispatch base discrepancy remains disclosed: original recorder baseCommit values were9d26d9b, not the narrative fdb81c7. Future receipts record actual dispatch/source HEAD without rewriting prior receipts.

## EARS requirements and OpenSpec scenarios

### T6S001 — Complete inventory

The Task6 suite SHALL retain all27 named semantic triples, one latest-field substitution triple, five locator triples, JSON/carrier/path controls, and seven actual-package controls.

Scenario: WHEN the final suite runs THEN all36 new pytest instances execute, alongside37 admitted checker and42 utility instances:115 total. No control is dropped because of sharding.

### T6S002 — Effective rebound semantics

WHEN a semantic mutation is generated, the test SHALL detect at least one changed event and changed serialized artifact before rebinding. The bad files SHALL then pass full raw hash, metadata, local adjacency, event mirroring, and count linkage before Invalid from semantic comparison counts as rejection.

Scenario: WHEN a mutation rule matches no fixture value THEN setup fails with effective mutation; it is not reported as a killed mutant.
Scenario: WHEN bad bytes are malformed, stale-hashed, resource-limited, or cause a child failure THEN the triple fails; none of those failures are caught as semantic rejection.

### T6S003 — Positive controls

After each rejected mutant, the test SHALL accept a corrected counterpart and an unrelated honest case through the same bounded provenance and semantic APIs.

Scenario: WHEN a checker rejects every input THEN either corrected or unrelated control fails and the triple does not pass.

### T6S004 — Bounded transport and process lifecycle

Synthetic helpers SHALL read/write one case at a time using incremental serialization, never load native arrays or build all78 expected histories. Each synthetic triple SHALL run in its own sequential child process. The child may retain one expected case history plus bounded event/window/codec copies; process exit releases its allocator memory.

Scenario: WHEN one triple completes THEN its child is reaped before the next starts; the parent retains only paths, small reports, argv, and terminal records.

### T6S005 — Actual package prerequisite

Final acceptance SHALL require separately admitted native raw shards, staged case files, full admission, manifest, exact source pins including final test bytes, original producer receipts, and inventory under evidence/s02-candidate-a-completion/a4.

Scenario: WHEN cases.json or admission.json is absent THEN the actual-package test fails with actual A4 package required; no skip occurs.
Scenario: WHEN source pins predate the appended test source THEN actual package admission fails; tests must not silently replace the root admission.

### T6S006 — Package controls

Each of seven package mutants SHALL start from the admitted actual manifest/admission and reference unchanged admitted raw/case/receipt files. After each rejection, the checker SHALL replay all78 unmodified shards again.

Scenario: WHEN missing-case, duplicate-case, extra-case, missing-transitive-pin, changed-frozen-pin, missing-input, or changed-receipt is applied THEN the mutant is rejected and the unchanged complete package subsequently passes.

### T6S007 — Original evidence

The recorder SHALL retain original source/runtime bytes, exact parent and child argv, stdout/stderr, exits, mutation identity, and generated control artifacts. The RED source must import successfully and fail behaviorally.

Scenario: WHEN raw-field equality is disabled THEN latest-field substitution fails with DID NOT RAISE; after the exact guard is restored the triple and final suite pass.
Scenario: WHEN a child exits nonzero or is interrupted THEN its original streams remain and no mutation success report is invented.

## Task A: File-backed synthetic triples

**Modify:** tests/test_s02_candidate_a_integrated.py, append the complete block below. Preserve the entire admitted prefix.
**Temporary RED only:** scripts/check_s02_candidate_a_integrated.py, exact raw-field equality line below.
**Interfaces:** check_shard(case_path,raw_path,descriptor,global_index,input_sha); bind_window(event,previous,current,index,input_name,input_sha); array_items(stream,target,metadata); write_array_document(stream,target,items,after=metadata.items()); file_digest(path). No check_document or whole-raw read_bytes calls.
**Source closure:** Existing test path is already in checker and producer PYTHON_SOURCES. Child uses runpy on that same file; no new source path or producer import is introduced. Receipt closure includes this file and every imported frozen checker/Core/utility/runtime byte. A fresh recorder source is separately pinned, not added to semantic PYTHON_SOURCES.

- [ ] Obtain root's edit-window release; no producer recorded command may overlap source changes.
- [ ] Append the complete code below. The code includes Task B's package test, but that test is not claimed passed until native prerequisites exist.
- [ ] Record import/syntax success with exact final test source and temporary RED checker.
- [ ] Replace only this current line:
  `require(exported_raw == raw_field, 'raw provenance field '+name)`
  with:
  `require(True, 'raw provenance field '+name)`
- [ ] Run only test_raw_latest_field_substitution_triple. Require terminal1 with the child's DID NOT RAISE visible in retained stderr, not an import/parser/setup failure. The parent assertion may be the pytest headline because the worker is a subprocess.
- [ ] Restore the exact equality line, verify checker SHA matches the admitted source, and run all synthetic controls and previous regressions.
- [ ] Freeze source and await root's independent source/receipt admission before producer source-pinned native exports.

The full append follows. The existing27 mutation rule bodies are preserved verbatim inside mutation_value; only their application changes from a whole-case recursive copy to one event at a time. Six event-level mutations retain their exact operation while streaming. Event-order buffers only the two swapped events. The fabricated cancellation mutation retains the first earlier nonempty computation list, not an invented evaluation.

```python
# Task6 additions: synthetic files are never native-package evidence.
from copy import deepcopy
from pathlib import Path
import gc
import json
import os
import subprocess
import sys
from scripts.a4_carrier import loads
from scripts.check_s02_candidate_a_integrated import (
    canonical, digest, array_items, validate_raw_state, validate_raw_metadata,
    raw_same, check_package, load_manifest, safe_path, shape)

SEMANTIC_MUTANTS = (
    'optional-zero','nonce','signer','token','revision','parent-paid',
    'rejection-reason','rejection-stage','raw-reductions','claimed-reductions',
    'effects-order','request-chooser','plan-omission','unused-program',
    'signing-snapshot','concealed-rejection','omitted-computation',
    'fabricated-cancellation-core','denial-as-transition','observed-guard',
    'event-order','payment-order','deposit-effect','rollback-time',
    'retained-loser','proof-context','cancellation-identity')

def task6_descriptor(loop, scenario, profile='SignBeforeResolve'):
    return next(d for d in descriptors() if
                (d['lifecycle'], d['scenario'], d['control'], d['profile']) ==
                (loop, scenario, 'ordinary', profile))

def rewrite(value, rule):
    if type(value) is dict: value = {k: rewrite(v, rule) for k, v in value.items()}
    elif type(value) is list: value = [rewrite(v, rule) for v in value]
    return rule(value)

def mutation_value(value, name):
    def rule(x):
        if type(x) is not dict: return x
        if name=='optional-zero' and x=={'tag':'NoInt','value':{'#tup':[]}}: return {'tag':'IntValue','value':0}
        if name=='nonce' and set(x)=={'policy','signer','token'} and x['policy']['body']['key']['nonce']==1:
            p=x['policy']; b=p['body']; return x|{'policy':p|{'body':b|{'key':b['key']|{'nonce':0}}}}
        if name=='signer' and set(x)=={'policy','signer','token'}: return x|{'signer':encode(V('Mallory'))}
        if name=='token' and set(x)=={'policy','signer','token'}: return x|{'token':1}
        if name=='revision' and 'revision' in x and type(x['revision']) is int: return x|{'revision':x['revision']+1}
        if name=='parent-paid' and {'paid','remainingAllowance','usedSlots'}<=set(x): return x|{'paid':x['paid']+1,'remainingAllowance':x['remainingAllowance']-1}
        if name=='rejection-reason' and {'reason','stage','observedContext'}<=set(x): return x|{'reason':encode(V('UnauthorizedEffect'))}
        if name=='rejection-stage' and {'reason','stage','observedContext'}<=set(x): return x|{'stage':encode(V('VerificationBoundary'))}
        if name=='raw-reductions' and x.get('tag')=='TransactionComputedA': return x|{'value':x['value']|{'reductions':x['value']['reductions']+1}}
        if name=='payment-order' and 'payments' in x and type(x['payments']) is list and len(x['payments'])==2: return x|{'payments':list(reversed(x['payments']))}
        if name=='deposit-effect' and set(x)=={'source','destination','asset','quantity'} and x['source'].get('tag')=='Wallet' and x['destination'].get('tag')=='Escrow': return x|{'quantity':x['quantity']+1}
        if name=='rollback-time' and x.get('tag')=='TransactionComputedA' and x['value']['accepted'] is False:
            raw=x['value']; return x|{'value':raw|{'state':raw['state']|{'minimumTime':encode(V('Time100'))}}}
        if name=='retained-loser' and x.get('tag')=='RejectedOperation': return encode(V('NoAttempt'))
        if name=='proof-context' and set(x)=={'attempt','disposition'}:
            a=x['attempt']; c=a['context']; return x|{'attempt':a|{'context':c|{'environment':c['environment']|{'anchor':1}}}}
        if name=='cancellation-identity' and 'artifactAndCall' in x and x['artifactAndCall'].get('tag')=='CancellationCallA' and 'proposedSuccessor' in x:
            b=x['proposedSuccessor']; return x|{'proposedSuccessor':b|{'state':b['state']|{'minimumTime':encode(V('Time100'))}}}
        if name=='claimed-reductions' and x.get('tag')=='CoreProjected': return x|{'value':x['value']|{'reductions':x['value']['reductions']+1}}
        if name=='effects-order' and 'effects' in x and type(x['effects']) is list and len(x['effects'])==2: return x|{'effects':list(reversed(x['effects']))}
        if name=='request-chooser' and x.get('tag')=='ChoiceInputA': return x|{'value':x['value']|{'chooser':encode(V('Mallory'))}}
        if name=='plan-omission' and 'operations' in x and type(x['operations']) is list and len(x['operations'])==4: return x|{'operations':x['operations'][:-1]}
        if name=='unused-program' and set(x)=={'root','nodes'}:
            nodes=deepcopy(x['nodes'])
            for pair in nodes['#map']:
                if pair[0]==encode(V('N15')): pair[1]=encode(V('PayA',r(account=r(owner=V('Alice'),asset=V('TokenA')),payee=V('Bob'),amount=V('ConstantA',5),continuation=V('N0'))))
            return x|{'nodes':nodes}
        if name=='signing-snapshot' and set(x)=={'physicalTime','anchor','implementationVersion','enforcementMechanism'}: return x|{'anchor':1}
        return x
    return rewrite(value, rule)

def task6_items(path, target):
    metadata = {}
    with Path(path).open('rb') as stream:
        yield from array_items(stream, target, metadata)

def task6_write(path, target, items, metadata):
    with Path(path).open('xb') as stream:
        return write_array_document(stream, target, items, after=metadata.items())

def task6_honest(path, d):
    def events():
        # Only this one independent history is retained; no full inventory history.
        for record in history(d):
            e = {public: encode(record['latest'][internal])
                 for public, internal in RENAME.items()}
            yield e | {'before': encode(record['before']),
                       'after': encode(record['after']), 'provenance': {}}
    task6_write(path, 'events', events(), d)
    gc.collect()

def task6_mutate(source, output, d, name):
    changed = 0
    done = False
    saved = None
    sample = None
    def events():
        nonlocal changed, done, saved, sample
        for index, e in enumerate(task6_items(source, 'events')):
            before = canonical(e)
            if sample is None and e['computations']:
                sample = deepcopy(e['computations'])
            if name == 'event-order':
                if index == 1:
                    saved = e
                    continue
                if index == 2:
                    changed += 2
                    yield e
                    yield saved
                    saved = None
                    continue
            elif name == 'concealed-rejection' and not done:
                if e['kind'] == 'transition' and e['arguments']['command']['tag'].startswith('Reject'):
                    changed += 1
                    done = True
                    continue
            elif name == 'omitted-computation' and not done and e['computations']:
                e['computations'] = []
                done = True
            elif name == 'fabricated-cancellation-core' and not done:
                cmd = e['arguments']['command']
                if cmd['tag'] == 'ProposeA4' and cmd['value']['operation']['tag'] == 'OpCancelParent':
                    require(sample is not None, 'cancellation mutation needs earlier actual computation')
                    e['computations'] = deepcopy(sample)
                    done = True
            elif name in ('denial-as-transition', 'observed-guard') and not done:
                if e['kind'] == 'denied-probe':
                    if name == 'denial-as-transition': e['kind'] = 'transition'
                    else: e['observed_guard'] = True
                    done = True
            elif name not in ('concealed-rejection', 'omitted-computation',
                              'fabricated-cancellation-core',
                              'denial-as-transition', 'observed-guard', 'event-order'):
                e = mutation_value(e, name)
            if canonical(e) != before: changed += 1
            yield e
        require(saved is None, 'event-order fixture has second event')
    task6_write(output, 'events', events(), d)
    require(changed > 0, 'effective mutation '+name)
    require(file_digest(source) != file_digest(output), 'changed artifact '+name)
    return changed

def task6_rebind(source, folder, d):
    folder.mkdir()
    ordinal = descriptors().index(d)
    normalized = folder/'normalized.json'
    raw = folder/'raw.itf.json'
    case = folder/'case.json'
    def normalized_events():
        previous = None
        for index, e in enumerate(task6_items(source, 'events')):
            e['sequence'] = index
            e['before'] = deepcopy(e['after'] if previous is None else previous)
            previous = e['after']
            e['provenance'] = {}
            yield e
    count = task6_write(normalized, 'events', normalized_events(), d)
    meta = {'format': 'ITF',
            'format-description': 'https://apalache-mc.org/docs/adr/015adr-trace.html',
            'source': ENTRIES[d['case_id']], 'status': 'ok',
            'description': 'synthetic rebound control; not generator evidence',
            'timestamp': 0}
    def states():
        for index, e in enumerate(task6_items(normalized, 'events')):
            latest = {internal: e[public] for public, internal in RENAME.items()}
            latest['sequence'] = {'#bigint': str(index)}
            yield {'#meta': {'index': index}, 'authorityState': e['after'],
                   'latestEvent': latest, 'caseIndex': {'#bigint': str(ordinal)},
                   'cursor': {'#bigint': str(index)}}
    assert task6_write(raw, 'states', states(), {'#meta': meta, 'vars': list(VARS)}) == count
    sha = file_digest(raw)
    def rebound_events():
        for index, e in enumerate(task6_items(normalized, 'events')):
            e['provenance'] = {'input_path': f'raw/case-{ordinal:03d}.itf.json',
                               'input_sha256': sha, 'before_index': max(0, index-1),
                               'after_index': index}
            yield e
    assert task6_write(case, 'events', rebound_events(), d) == count
    return case, raw, sha, ordinal, count

def task6_bound(artifact, d):
    case, raw, sha, ordinal, expected_count = artifact
    assert file_digest(raw) == sha
    case_meta, raw_meta = {}, {}
    absent = object()
    count = 0
    with case.open('rb') as cs, raw.open('rb') as rs:
        events = array_items(cs, 'events', case_meta)
        states = array_items(rs, 'states', raw_meta)
        previous = None
        while True:
            e, current = next(events, absent), next(states, absent)
            if e is absent or current is absent:
                assert e is absent and current is absent
                break
            validate_raw_state(current, count, ordinal)
            bind_window(e, current if previous is None else previous, current,
                        count, f'raw/case-{ordinal:03d}.itf.json', sha)
            previous = current
            count += 1
    assert count == expected_count
    assert raw_same(case_meta, d)
    validate_raw_metadata(raw_meta, ENTRIES[d['case_id']])
    return count

def task6_accept(artifact, d):
    task6_bound(artifact, d)
    case, raw, sha, ordinal, count = artifact
    assert check_shard(case, raw, d, ordinal, sha) == count

def task6_other(folder, loop='swap'):
    d = (task6_descriptor('swap', 'funded2-refund', 'SignAfterResolve')
         if loop == 'swap' else task6_descriptor('installment', 'two-fills'))
    source = folder/'other-source.json'
    task6_honest(source, d)
    artifact = task6_rebind(source, folder/'other', d)
    task6_accept(artifact, d)
    gc.collect()

def task6_first_pair(artifact, index):
    case, raw, sha, ordinal, count = artifact
    # Read only the requested bounded window, closing both generators explicitly.
    es = task6_items(case, 'events')
    ss = task6_items(raw, 'states')
    previous = None
    try:
        for i in range(index+1):
            e, current = next(es), next(ss)
            if i == index:
                return e, current if previous is None else previous, current
            previous = current
    finally:
        es.close()
        ss.close()

def task6_worker(mode, name, folder):
    folder = Path(folder)
    if mode == 'semantic':
        d = (task6_descriptor('swap', 'funded2-settle')
             if name in ('effects-order', 'payment-order', 'deposit-effect')
             else task6_descriptor('installment',
                  'recover-r1-refuse100' if name == 'rollback-time' else 'recover-r1-choice2'))
        source = folder/'honest-source.json'
        task6_honest(source, d)
        mutated = folder/'mutated-source.json'
        changed = task6_mutate(source, mutated, d, name)
        bad = task6_rebind(mutated, folder/'bad', d)
        task6_bound(bad, d)  # MUST succeed before semantic rejection is counted.
        with pytest.raises(Invalid):
            case, raw, sha, ordinal, _ = bad
            check_shard(case, raw, d, ordinal, sha)
        corrected = task6_rebind(source, folder/'corrected', d)
        task6_accept(corrected, d)
        task6_other(folder)
        return {'mode': mode, 'name': name, 'changed_events': changed,
                'raw_linkage': True, 'mutant_rejected': True,
                'corrected': True, 'unrelated': True}
    d = task6_descriptor('swap', 'funded2-settle')
    source = folder/'honest-source.json'
    task6_honest(source, d)
    honest = task6_rebind(source, folder/'honest', d)
    index = 0 if mode == 'substitution' else 1
    event, previous, current = task6_first_pair(honest, index)
    if mode == 'substitution':
        assert event['profile'] == 'SignBeforeResolve'
        event['profile'] = 'SignAfterResolve'
        message = 'raw provenance field profile'
    else:
        field, value = (('before_index',99), ('after_index',99),
                        ('after_index',True), ('input_path','../swap.itf.json'),
                        ('input_sha256','0'*64))[int(name)]
        event['provenance'][field] = value
        message = None
    bad_case = folder/'substituted-case.json'
    def substituted_events():
        for i, original_event in enumerate(task6_items(honest[0], 'events')):
            yield event if i == index else original_event
    task6_write(bad_case, 'events', substituted_events(), d)
    bad_artifact = (bad_case, *honest[1:])
    event, previous, current = task6_first_pair(bad_artifact, index)
    with pytest.raises(Invalid, match=message):
        bind_window(event, previous, current, index,
                    f'raw/case-{honest[3]:03d}.itf.json', honest[2])
    del event, previous, current
    task6_accept(honest, d)
    task6_other(folder, 'installment')
    return {'mode': mode, 'name': name, 'mutant_rejected': True,
            'corrected': True, 'unrelated': True}

def task6_process(tmp_path, mode, name):
    # No moving producer imports. Same test source is the pinned worker.
    root = Path(__file__).resolve().parents[1]
    script = "import runpy,sys,json; m=runpy.run_path(sys.argv[1]); print(json.dumps(m['task6_worker'](sys.argv[2],sys.argv[3],sys.argv[4]),sort_keys=True))"
    argv = [sys.executable, '-B', '-X', 'pycache_prefix='+str(tmp_path/'fresh-cache'),
            '-c', script, str(Path(__file__).resolve()), mode, str(name), str(tmp_path)]
    env = os.environ.copy()
    env.pop('PYTHONPATH', None)
    env['PYTHONDONTWRITEBYTECODE'] = '1'
    env['PYTHONPYCACHEPREFIX'] = str(tmp_path/'fresh-cache')
    (tmp_path/'argv.json').write_text(json.dumps(argv))
    with (tmp_path/'stdout.txt').open('xb') as out, (tmp_path/'stderr.txt').open('xb') as err:
        completed = subprocess.run(argv, cwd=root, env=env, stdout=out, stderr=err)
    (tmp_path/'terminal.json').write_text(json.dumps({'exit': completed.returncode}))
    assert completed.returncode == 0, (tmp_path/'stderr.txt').read_text()
    report = json.loads((tmp_path/'stdout.txt').read_text())
    assert report['mode'] == mode and report['name'] == str(name)
    assert report['mutant_rejected'] and report['corrected'] and report['unrelated']
    return report

@pytest.mark.parametrize('name', SEMANTIC_MUTANTS)
def test_semantic_rebound_triples(tmp_path, name):
    report = task6_process(tmp_path, 'semantic', name)
    assert report['changed_events'] > 0 and report['raw_linkage']

def test_raw_latest_field_substitution_triple(tmp_path):
    task6_process(tmp_path, 'substitution', 'profile')

@pytest.mark.parametrize('locator', range(5))
def test_provenance_locator_triples(tmp_path, locator):
    task6_process(tmp_path, 'locator', str(locator))

def test_duplicate_json_map_set_and_unit_controls():
    with pytest.raises(Invalid): loads('{"schema_version":3,"schema_version":3}')
    with pytest.raises(Invalid): decode({'#set':[1,1]})
    with pytest.raises(Invalid): decode({'#map':[[1,0],[1,0]]})
    with pytest.raises(Invalid): shape({'tag':'NoInt','value':[]},V('NoInt'))
    shape({'tag':'NoInt','value':{'#tup':[]}},V('NoInt'))
    shape({'tag':'IntValue','value':{'#bigint':'0'}},V('IntValue',0))

def test_path_traversal_and_symlink_triples(tmp_path):
    target = tmp_path/'honest.json'
    target.write_text('{}')
    alias = tmp_path/'alias.json'
    alias.symlink_to(target)
    with pytest.raises(Invalid): safe_path(tmp_path,'../honest.json')
    with pytest.raises(Invalid): safe_path(tmp_path,'alias.json')
    assert safe_path(tmp_path,'honest.json') == target
    other = tmp_path/'other.json'
    other.write_text('[]')
    assert safe_path(tmp_path,'other.json') == other

def test_actual_complete_package_and_inventory_triples(tmp_path):
    root = Path(__file__).resolve().parents[1]
    folder = root/'evidence/s02-candidate-a-completion/a4'
    manifest, admission, inv = folder/'cases.json', folder/'admission.json', folder/'inventory.json'
    require(manifest.is_file() and admission.is_file(), 'actual A4 package required; no skip')
    doc, admitted = load_manifest(manifest), load_manifest(admission)
    def honest():
        result = check_package(manifest, admission, root, folder, inv)
        assert result['ok'] is True and result['events'] == 1557
        assert result['cases'] == result['shards'] == result['validated_inputs'] == result['validated_case_files'] == 78
        gc.collect()
    honest()
    for name in ('missing-case','duplicate-case','extra-case','missing-transitive-pin',
                 'changed-frozen-pin','missing-input','changed-receipt'):
        bad, auth = deepcopy(doc), deepcopy(admitted)
        if name == 'missing-case': bad['shards'].pop()
        elif name == 'duplicate-case': bad['shards'][-1] = deepcopy(bad['shards'][0])
        elif name == 'extra-case': bad['shards'].append(deepcopy(bad['shards'][0]))
        elif name == 'missing-transitive-pin':
            path = 'specs/quint/s02/consumption.qnt'
            del bad['source_pins'][path]
            del auth['source_pins'][path]
        elif name == 'changed-frozen-pin':
            path = 'specs/quint/s02/consumption.qnt'
            bad['source_pins'][path] = auth['source_pins'][path] = '0'*64
        elif name == 'missing-input':
            path = 'raw/case-032.itf.json'
            del bad['input_pins'][path]
            del auth['input_pins'][path]
        else:
            path = sorted(bad['receipt_pins'])[0]
            bad['receipt_pins'][path] = '0'*64
        bad_path, auth_path = tmp_path/(name+'-manifest.json'), tmp_path/(name+'-admission.json')
        bad_path.write_text(canonical(bad))
        auth_path.write_text(canonical(auth))
        with pytest.raises(Invalid): check_package(bad_path, auth_path, root, folder, inv)
        honest()  # All 78 semantics revisited; no semantic-result cache.
    task6_other(tmp_path, 'installment')

```

## Task B: Native-package gate and seven package controls

**Ownership:** Same appended test file; no additional implementation source.
**Prerequisite:** Task A source is frozen, behavioral RED restored, and root has admitted native producer receipts and sealed schema3 files against those exact final source pins. Root must separately confirm actual generator provenance; passing the supplied admission file alone is not generator authentication.

- [ ] Root checks all78 native raw shards, case files and original command/source/tool/terminal receipts, then supplies full admission.json and cases.json under the fixed evidence directory. Inventory remains unchanged.
- [ ] Run the actual-package test with no synthetic replacement. The first honest pass must return78 shards/1557 events.
- [ ] Execute all seven small manifest mutations. Each starts from the same unmodified admitted manifest and admission. The missing input is raw/case-032.itf.json, the first swap shard; the transitive/frozen pin is specs/quint/s02/consumption.qnt. Changed receipt preserves the root admission and changes only a submitted receipt hash, testing admission equality.
- [ ] After each rejection, invoke check_package on the complete unchanged package again. This gives eight full honest traversals (one initial plus seven corrected controls), each visiting all78 case semantics. No semantic-result cache is used.
- [ ] Run the unrelated installment two-fills bounded synthetic positive control, clearly labeled as such.
- [ ] Run the complete115-instance checker+utility suite with no deselection or skips. Root then runs the separately owned shared schema1/Python regression inventory and non-author review.

No overlay is needed for these seven package controls: they mutate only tiny manifest/admission copies in tmp_path and reference the original read-only input_root. This is the lower-copy equivalent of the transport section6 overlay prescription. If a future control changes a raw/case/receipt file, it requires a separately reviewed overlay with explicit copies or hard links, exclusive copy-on-write replacement, no symlinks, and no writes through links to original evidence. No such native-file semantic mutation is introduced or claimed by this plan.

The package test holds only the small manifests and the checker's one-case history/window. It invokes gc.collect after each complete pass but does not claim that Python returns every allocator page to the OS. Synthetic triple workers terminate individually; native checking already has case-bounded semantic memory. Raw files are hashed in chunks and no complete native JSON document is loaded.

## Exact command sequence and evidence

All commands run from /home/charl/Moriarty/.worktrees/s01-audit-start. Root's recorder must wrap each command, snapshot the exact source/runtime closure first, assert before/after stability, capture original streams and terminal exit, and retain a fresh per-stage pytest basetemp directory. These are execution commands, not commands run while drafting this plan.

The recorder's own parent process must start with -B and -X pycache_prefix pointing to a nonexistent stage-specific directory, and record sys.orig_argv. Reuse the admitted3329-file Python environment archive by hash; do not duplicate it. Every child triple records its exact argv, stdout, stderr, and terminal in its unique tmp_path. Root preserves that complete basetemp tree with the stage evidence before any cleanup. No automatic artifact deletion is part of this plan.

~~~bash
env PYTHONPATH= PYTEST_ADDOPTS= /home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-task6-receipts/import-cache -c "import scripts.check_s02_candidate_a_integrated; import runpy; runpy.run_path('tests/test_s02_candidate_a_integrated.py')"
~~~

Expected terminal0 for the importable temporary RED source. Test collection is not behavioral RED.

~~~bash
env PYTHONPATH= PYTEST_ADDOPTS= /home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-task6-receipts/red-cache -m pytest -q tests/test_s02_candidate_a_integrated.py::test_raw_latest_field_substitution_triple --basetemp=.superpowers/sdd/a4-task6-receipts/red-artifacts
~~~

Expected terminal1 caused by absent raw-profile rejection; the child stderr must contain DID NOT RAISE. An import error, resource failure, no-op mutation, or malformed file does not satisfy this expectation. Restore the exact admitted checker equality immediately after preserving the original receipt.

~~~bash
env PYTHONPATH= PYTEST_ADDOPTS= /home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-task6-receipts/synthetic-cache -m pytest -q tests/test_s02_candidate_a_integrated.py tests/test_a4_json_stream.py -k 'not test_actual_complete_package_and_inventory_triples' --basetemp=.superpowers/sdd/a4-task6-receipts/synthetic-artifacts
~~~

Expected114 passed, one deselected. This deliberately limited pre-export unit gate is not Task6 completion. Freeze and root review occur here before native producer exports against final test-source pins.

After separately admitted actual files exist:

~~~bash
env PYTHONPATH= PYTEST_ADDOPTS= /home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-task6-receipts/final-cache -m pytest -q tests/test_s02_candidate_a_integrated.py tests/test_a4_json_stream.py --basetemp=.superpowers/sdd/a4-task6-receipts/final-artifacts
~~~

Expected115 passed, zero skips/deselections. The actual-package test's lack of files is a blocking failure, not grounds to revise the command.

Root also retains the direct actual CLI report:

~~~bash
env PYTHONPATH= /home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-task6-receipts/cli-cache scripts/check_s02_candidate_a_integrated.py --cases evidence/s02-candidate-a-completion/a4/cases.json --admission evidence/s02-candidate-a-completion/a4/admission.json --inventory evidence/s02-candidate-a-completion/a4/inventory.json --source-root . --input-root evidence/s02-candidate-a-completion/a4 --report .superpowers/sdd/a4-task6-receipts/actual-cli-report.json
~~~

Expected terminal0, ok true, shards78, cases78, events1557. No report is fabricated on interruption.

Before execution, root creates a fresh ignored receipt root and verifies adequate disk space for the retained original synthetic artifacts and native package. Retaining all mutant artifacts can require many gigabytes even though memory is case-bounded. If disk or the lexical single-value cap is exceeded, stop and preserve the failure; do not remove a mutant or reinterpret ResourceLimit as successful rejection. A reviewed archive/cleanup policy may later release temporary artifacts only after original-byte admission.

## Review checkpoints and limitations

1. Root reviews this complete append and its schema/API compatibility before dispatch. This plan has not been executed or syntax-checked; no runtime pass is claimed.
2. Root opens a source-mutation window; implementation captures importable behavioral RED, restores exact checker source, and proves the114-instance synthetic gate.
3. Root admits the final test source and original unit receipts, then schedules native exports and full admission using those source pins.
4. Root reviews the complete native package and executes the115-instance final suite plus direct CLI and shared regressions.
5. Independent non-author review checks source, mutant effectiveness and original receipt provenance. No author self-review substitutes for it.

The future implementation report must distinguish all27 semantic mutant kills from raw-profile and locator failures, the seven package-structure/admission failures, lexical/carrier failures, and native honest acceptance. Current finite-record and symbolic evidence/signature limitations remain. No model checking, cryptographic verification, Council acceptance, or full A4 completion follows from this ignored plan.
