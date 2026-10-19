"""Root-only one-off selection of the frozen sixteen ordinary F03 children.

Run only after Root's non-author exact-three-string confirmation. This file
implements no process containment: every child uses the original F03.launch.
No source/model/expected values are calculated here. No retry or prefix replay.
"""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource

N = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-main-literal-complete-rule-correspondence-20261019/revision02-input-bindings-f03-gate')
PLAN = N / 'PROPOSED-EXACT16-OPERATIONS.json'
STATIC = N / 'STATIC-INPUT-PINS.json'
SOURCE = N / 'SOURCE-FREEZE.json'
APPROVED_METADATA = {
    str(PLAN): (87482, 'f10ab4bc68e1d74de952e9aee2752fcc66c623727cd14bdad5b7c76e2499b52b'),
    str(STATIC): (13277, '0830acb787aba84aa0ddc338262cda0b9ca47e3ee036dcb9de1a2ea0050b80f3'),
    str(SOURCE): (5033, '2dc1dde30ecd0eb576768622fce751fe8d23df6b182236dfa9f5685e803aa55d'),
}
FSIZE = 536870912


def pin(path):
    path = Path(path)
    raw = path.read_bytes()
    return {'path': str(path), 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()}


def checked(row):
    want = {k: row[k] for k in ('path', 'bytes', 'sha256')}
    if pin(row['path']) != want:
        raise ValueError('pin changed: ' + row['path'])
    if 'resolved_path' in row and str(Path(row['path']).resolve()) != row['resolved_path']:
        raise ValueError('resolved tool changed: ' + row['path'])
    return want


def read(path):
    return json.loads(Path(path).read_bytes())


def save(path, obj):
    path = Path(path)
    raw = (json.dumps(obj, sort_keys=True, indent=2, allow_nan=False) + '\n').encode()
    with path.open('xb') as stream:
        stream.write(raw)
        stream.flush()
        os.fsync(stream.fileno())
    return pin(path)


def sync(path):
    with Path(path).open('rb') as stream:
        os.fsync(stream.fileno())


def sync_dir(path):
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def main():
    for path, (size, digest) in APPROVED_METADATA.items():
        checked({'path': path, 'bytes': size, 'sha256': digest})
    plan, static = read(PLAN), read(STATIC)
    phases = plan['phases']
    if len(phases) != 16 or phases[0]['id'] != '00-synthetic-unit':
        raise ValueError('exact16/unit-first membership differs')
    hard = resource.getrlimit(resource.RLIMIT_FSIZE)[1]
    if hard != resource.RLIM_INFINITY and hard < FSIZE:
        raise ValueError('Root hard FSIZE below F03 preexec requirement; no child selected')
    # Existing reviewed import-time declarations only. Its __main__/old campaign
    # is unselected. No launch/containment function is copied or changed here.
    driver = plan['callback']['fullsource']
    checked(driver)
    spec = importlib.util.spec_from_file_location('root_existing_f03_only', driver['path'])
    f03 = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(f03)
    if f03.CHILD_SECONDS != 30 or f03.SWEEP_SECONDS != 3 or f03.FILE_SIZE_LIMIT != FSIZE:
        raise ValueError('existing F03 controls differ')
    tools = read(plan['tool4_JAR156']['path'])
    closure = read(plan['Kregular281']['path'])
    rows = [*static['static'], *static['compiled_regular_files'],
            *tools['entries'], *tools['classpath_jar_membership'], *closure['files']]
    rows += [{'path': p, 'bytes': z[0], 'sha256': z[1]} for p, z in APPROVED_METADATA.items()]
    # These two protected files are opaque-hashed only by Root. They are never
    # JSON-parsed here; only child13 may parse the new expected body.
    rows += [plan['opaque_existing_actual'], plan['independent_expected_body']]
    rows = list({row['path']: row for row in rows}.values())
    produced = {}

    def all_pins():
        checked_rows = [checked(row) for row in rows]
        checked_rows += [checked(row) for row in produced.values()]
        compiled = static['compiled_regular_files']
        directory = Path(compiled[0]['path']).parent
        if sorted(str(p) for p in directory.rglob('*') if p.is_file()) != sorted(r['path'] for r in compiled):
            raise ValueError('compiled exact15 membership changed')
        jars = Path(tools['classpath_pattern']).parent
        if sorted(str(p) for p in jars.glob('*.jar')) != sorted(r['path'] for r in tools['classpath_jar_membership']):
            raise ValueError('classpath exact156 jar membership changed')
        return checked_rows

    all_pins()
    actual = N / 'actual'
    if actual.exists():
        raise ValueError('actual already exists; no retry/reset')
    actual.mkdir(mode=0o700)
    receipt_root = Path(plan['root_receipts'])
    receipt_root.mkdir(mode=0o700)
    for phase in phases:
        cwd = Path(phase['cwd'])
        cwd.mkdir(mode=0o700)
        (cwd / 'home').mkdir(mode=0o700)
        (cwd / 'tmp').mkdir(mode=0o700)
        (receipt_root / phase['id']).mkdir(mode=0o700)
    selection = save(actual / 'ROOT-SELECTION.json', {
        'plan': pin(PLAN), 'static': pin(STATIC), 'source': pin(SOURCE),
        'caller': pin(__file__), 'phase_order': [p['id'] for p in phases],
        'Root_hard_fsize': hard, 'launcher': driver,
        'expected_body_reads': 0, 'Root_opaque_hashes_excluded': True,
        'source_test_model_changed': False,
    })
    produced[selection['path']] = selection
    results = []
    selected = []
    failure = None
    try:
        for phase in phases:
            label, cwd = phase['id'], Path(phase['cwd'])
            receipts = receipt_root / label
            if sorted(p.name for p in cwd.iterdir()) != ['home', 'tmp']:
                raise ValueError('phase cwd not fresh: ' + label)
            if list((cwd / 'home').iterdir()) or list((cwd / 'tmp').iterdir()):
                raise ValueError('phase scratch not fresh: ' + label)
            if label == '13-independent-comparison':
                # Whole actual data/products/receipts are durable before the
                # first selected JSON parse of the NEW independent expectation.
                members = list(produced.values())
                members += [pin(PLAN), pin(STATIC), pin(SOURCE), plan['opaque_existing_actual']]
                members = list({row['path']: row for row in members}.values())
                for row in members:
                    checked(row)
                    sync(row['path'])
                main_path = next(p for p in phases if p['id'] == '11-decode-main')['products'][0]
                relation_path = next(p for p in phases if p['id'] == '12-relate-complete')['products'][0]
                safe = read(N.parent / 'oracle-safe-request/SAFE-READSET.json')['original_safe_input']
                barrier = {'expected_body_reads': 0, 'actual_main': pin(main_path),
                           'actual_relation': pin(relation_path), 'safe_input': safe,
                           'products_streams_receipts': members}
                barrier_pin = save(actual / 'ROOT-ALL-ACTUAL-FROZEN-BEFORE-EXPECTED.json', barrier)
                produced[barrier_pin['path']] = barrier_pin
                for directory in [actual, receipt_root, *[Path(p['cwd']) for p in phases[:-1]],
                                  *[receipt_root / p['id'] for p in phases[:-1]]]:
                    sync_dir(directory)
            before = all_pins()
            pre = save(receipts / 'ROOT-PRECHECK.json', {'all_pins': before, 'phase': label})
            command = save(receipts / 'COMMAND.json', {'id': label, 'argv': phase['argv'],
                          'environment': phase['environment'], 'cwd': phase['cwd'],
                          'limits': phase['limits'], 'stdin': 'DEVNULL'})
            inputs = list(phase['input_paths'])
            if label == '13-independent-comparison':
                inputs += [row['path'] for row in read(actual / 'ROOT-ALL-ACTUAL-FROZEN-BEFORE-EXPECTED.json')['products_streams_receipts']]
            inputs = list(dict.fromkeys(inputs))
            readset = save(receipts / 'ROOT-READSET.json', {
                'source_resolved_inputs': [pin(path) for path in inputs],
                'basis': 'exact reviewed consumer/plan branches, not syscall access instrumentation',
                'expected_body_parse_selected': label == '13-independent-comparison',
                'opaque_Root_hashes_excluded_from_expected_body_reads': True,
            })
            record = {'id': label}
            selected.append(label)
            f03.launch(record, phase['argv'], dict(phase['environment']), phase['cwd'],
                       phase['environment']['SHERLOCK_K_ATTEMPT_TOKEN'], FSIZE)
            launch = save(receipts / 'LAUNCH-RESULT.json', record)
            results.append({'id': label, 'accepted': False, 'launch': launch,
                            'wall_seconds': record.get('elapsed_seconds')})
            inventory = f03.tree(phase['cwd'])
            inv = save(receipts / 'STAGE-INVENTORY.json', {'tree': inventory})
            allowed = {'home', 'tmp', 'stdout', 'stderr', *[Path(p).name for p in phase['products']]}
            unexpected = [p.name for p in cwd.iterdir() if p.name not in allowed]
            streams = [pin(phase['stdout']), pin(phase['stderr'])]
            products = [pin(path) for path in phase['products'] if Path(path).is_file()]
            sp = save(receipts / 'STREAM-PINS.json', streams)
            pp = save(receipts / 'PRODUCT-PINS.json', products)
            after = all_pins()
            post = save(receipts / 'ROOT-POSTCHECK.json', {'all_pins': after, 'unexpected': unexpected})
            for row in [pre, command, readset, launch, inv, sp, pp, post, *streams, *products]:
                produced[row['path']] = row
            for path in cwd.rglob('*'):
                if path.is_file():
                    produced[str(path)] = pin(path)
            ok = (record.get('outcome') == 'exited' and record.get('returncode') == 0
                  and record.get('timed_out') is False and record.get('containment_breach') is False
                  and record.get('remaining') == [] and not unexpected
                  and len(products) == len(phase['products']))
            results[-1]['accepted'] = ok
            sync_dir(receipts)
            if not ok:
                raise ValueError('fail-stop child: ' + label)
    except Exception as error:
        failure = {'type': type(error).__name__, 'message': str(error)}
    result = {'status': 'PASS_EXACT16_SCOPED' if failure is None and len(results) == 16 else 'FAIL_STOP',
              'phases': results, 'failure': failure,
              'selected_dispatches': selected,
              'unselected': [p['id'] for p in phases if p['id'] not in selected],
              'new_expected_body_parse_selected': any(x['id'] == '13-independent-comparison' for x in results),
              'old_subject_prefix_replayed': False, 'model_or_resource_changes': False,
              'context_binds_counterpart_source': False, 'refinement_native_or_Bclock_claim': False}
    save(actual / 'ROOT-RESULT.json', result)
    sync_dir(actual)
    print(json.dumps(result, sort_keys=True))
    if result['status'] != 'PASS_EXACT16_SCOPED':
        raise SystemExit(1)


if __name__ == '__main__':
    main()
