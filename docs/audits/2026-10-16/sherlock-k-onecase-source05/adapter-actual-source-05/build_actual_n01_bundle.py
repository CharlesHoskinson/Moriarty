"""Assemble the one actual N01 fuel-1 observation bundle from attempt-engine-N01-01.

Pure standard library plus the fixed source05 leaf loader. Reads only fixed
absolute paths under R: the frozen attempt receipt and its stage artifacts, the
compiled source-attribute inventory, the compiled-input main.k and the retained
N01-fuel-001.ui input, plus the three fixed adapter files. It opens no oracle,
expectation, fixture or withheld comparison file. It launches nothing and uses
no network. Every artifact is checked by size and SHA-256 against the receipt
and its postflight tree before use.

Pairing is fixed:
  startup     engine 04 / kprint 09
  semantic    engine 05 / kprint 10
  publication engine 06 / kprint 11
  terminal    engine 07 / kprint 12
Stages 01, 02, 03 and 08 are retained raw only. The pre-startup state exists only
as the Kore that the 04 log shows On configuration; the adapter keeps it untyped.

The only writes are two new files, created exclusively in
R/adapter-actual-source-05; nothing is overwritten. Consistency of the supplied
bytes, receipts and paths is checked. Authenticity is not established. Reliance
stays blocked pending fresh review.
"""
import argparse
import base64
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import stat
import sys

sys.dont_write_bytecode = True

R = Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01')
ATTEMPT = R / 'attempt-engine-N01-01'
STAGES_DIR = ATTEMPT / 'stages'
OUT_DIR = R / 'adapter-actual-source-05'
OUTPUT = OUT_DIR / 'ACTUAL-N01-FUEL1.json'
STATUS = OUT_DIR / 'ACTUAL-N01-FUEL1-OPERATOR-STATUS.json'
QA = OUT_DIR / 'models' / 'sherlock' / 'qa'
MODULE_NAME = 'k_capture_observations.py'
ADAPTER = {MODULE_NAME: (41183, '5df990ed9c8b54ca1e090e80883f483f2e745138a740cdbcbbfc80fd00ce77d1'),
           'k_observations.py': (7686, '540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07'),
           'k_terms.py': (5666, 'c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca')}
RECEIPT = (ATTEMPT / 'receipt.json', 295790,
           'd6123acfdfd16128d533701dae3316bf166c08e5ae393fccbb958657f38d02cc')
INVENTORY = (R / 'COMPILED-MAIN-RULE-SPANS-01.json', 85954,
             '9e77ddfe74acaa1e2deccaa1fcde70794494dd3583362b3f9a6f58473d9e941c')
SOURCE = (R / 'compile-input' / 'models' / 'sherlock' / 'k' / 'main.k', 16249,
          'b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41')
RUN_INPUT = (R / 'inputs' / 'N01-fuel-001.ui', 43,
             'e5fe7b26e9565fdafdc97da0fb25fb81c6171afd69e99eaa6dfb38b5409a2e0f')
DRIVER_SHA256 = 'af9f49b07af4b8d16607fd7cbe01df79ff30302a782084ab502a17dda10e56d2'
IDENTITY_SHA256 = 'f0fee1e80b0a90648d6655ffe1dfd53482779767897358f14394c095717e8d80'
DEFINITION_SHA256 = 'd795e32885e5651b49ea59eef7b57595a431a6908ef8e9c36901a5ec5e43b6e4'
INVENTORY_SCHEMA = 'sherlock-k-compiled-source-attribute-inventory/1'
INVENTORY_ROWS = 113
CASE_ID = 'N01'
ENTRY_K = 'uiLiteral(uiU(7))'
ENVELOPE = 'sherlock-k-actual-n01-fuel1-envelope/1'
STATUS_SCHEMA = 'sherlock-k-actual-n01-fuel1-operator-status/1'
STAGES = ('01-kast', '02-initializer', '03-exec-depth0', '04-exec-depth1-1', '05-exec-depth1-2',
          '06-exec-depth1-3', '07-exec-depth1-4', '08-kprint-03', '09-kprint-04', '10-kprint-05',
          '11-kprint-06', '12-kprint-07')
INITIALIZER = STAGES[1]
ENGINE = STAGES[2:7]
PRINT = STAGES[7:]
STARTUP, SEMANTIC, PUBLICATION, TERMINAL = tuple(zip(ENGINE[1:], PRINT[1:]))
ENGINE_ARTIFACTS = frozenset({'state.kore', 'stderr', 'stdout'})
STREAM_ARTIFACTS = frozenset({'stderr', 'stdout'})
CHILD_EMPTY = ('problems', 'remaining', 'swept', 'scratch_residue', 'group_members_after_exit')
POSTFLIGHT_EMPTY = ('blockers', 'captured_artifacts_changed', 'changed', 'pinned_problems',
                    'protected_changed')
OBSERVATION_KEYS = frozenset({'producer', 'profile', 'source_sha256', 'boundary', 'programs', 'runs'})
MAX_SELF_BYTES = 1024 * 1024
RELIANCE = ('blocked pending fresh full non-author Opus and Muse audits of the source05 delivery '
            'and of this assembler, and owner confirmation')
_DIGEST = re.compile(r'[0-9a-f]{64}')
_CHECKS = []


class _Blocked(Exception):
    def __init__(self, message, gaps):
        super().__init__(message)
        self.gaps = list(gaps)


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _raw(data):
    return dict(base64=base64.b64encode(data).decode('ascii'), sha256=_sha(data), size=len(data))


def _ref(spec):
    return dict(path=str(spec[0]), size=spec[1], sha256=spec[2])


def _require(condition, label):
    if not condition:
        raise ValueError('check failed: ' + label)
    _CHECKS.append(label)


def _bytes_at(path, limit):
    """Read one regular, non-symlinked file whose path is its own real path, within a bound."""
    text = str(path)
    if os.path.realpath(text) != text:
        raise ValueError('path is not its own resolved absolute path: ' + text)
    descriptor = os.open(text, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        if not stat.S_ISREG(os.fstat(descriptor).st_mode):
            raise ValueError('not a regular file: ' + text)
        chunks, total = [], 0
        while True:
            chunk = os.read(descriptor, limit + 1 - total)
            if not chunk:
                break
            chunks.append(chunk)
            total += len(chunk)
            if total > limit:
                raise ValueError('file exceeds its fixed size bound: ' + text)
    finally:
        os.close(descriptor)
    return b''.join(chunks)


def _read(path, size, digest, label):
    if type(size) is not int or size < 0:
        raise ValueError(label + ' has no recorded integer size')
    data = _bytes_at(path, size)
    _require(len(data) == size and _sha(data) == digest, f'{label}: {size} bytes, sha256 {digest}')
    return data


def _json(data, label):
    def pairs(items):
        if len({key for key, _ in items}) != len(items):
            raise ValueError(label + ' repeats a JSON object key')
        return dict(items)

    def constant(name):
        raise ValueError(label + ' carries non-finite JSON constant ' + name)

    return json.loads(data.decode('utf-8'), object_pairs_hook=pairs, parse_constant=constant)


def _ascii(data, label):
    if not data.isascii():
        raise ValueError(label + ' is not ASCII')
    return data.decode('ascii')


def _option(argv, flag):
    if argv.count(flag) != 1 or argv.index(flag) + 1 >= len(argv):
        raise ValueError('command does not carry exactly one ' + flag)
    return argv[argv.index(flag) + 1]


def _attributes_agree(row):
    attributes = row['raw_attributes']
    location = 'Location({},{},{},{})'.format(row['start_line'], row['start_column'],
                                             row['end_line'], row['end_column'])
    return (type(attributes) is str and location in attributes
            and "UNIQUE'Unds'ID{}(\"" + row['unique_id'] + '")' in attributes
            and 'Source(' + row['source_path'] + ')' in attributes)


def _stages(receipt):
    """Admit the receipt only as all-completed, twelve fixed stages, eleven clean exit-0 children."""
    preflight, postflight, stages = receipt['preflight'], receipt['postflight'], receipt['stages']
    _require(receipt['schema'] == 'sherlock-k-engine-attempt-receipt/1'
             and receipt['attempt'] == preflight['attempt'] == ATTEMPT.name,
             'receipt schema and attempt-engine-N01-01 identity')
    _require(receipt['outcome'] == 'all-stages-completed', 'receipt outcome all-stages-completed')
    _require(receipt['driver_sha256'] == preflight['driver_sha256'] == DRIVER_SHA256,
             'driver sha256 ' + DRIVER_SHA256 + ' in receipt and preflight')
    _require(receipt['identity_sha256'] == preflight['identity_sha256'] == IDENTITY_SHA256,
             'identity sha256 ' + IDENTITY_SHA256 + ' in receipt and preflight')
    _require(preflight['blockers'] == [], 'preflight blockers empty')
    _require(all(postflight[key] == [] for key in POSTFLIGHT_EMPTY),
             'postflight ' + ', '.join(POSTFLIGHT_EMPTY) + ' all empty')
    _require(type(stages) is list and [stage.get('stage') for stage in stages] == list(STAGES),
             'all twelve stages recorded in fixed order')
    by_name = {stage['stage']: stage for stage in stages}
    for name in STAGES:
        stage = by_name[name]
        if name == INITIALIZER:
            _require(stage['outcome'] == 'written' and stage['problems'] == []
                     and stage['child'].startswith('none:'),
                     name + ' driver-only binding written without child or problem')
        else:
            _require(stage['outcome'] == 'exited' and type(stage['returncode']) is int
                     and stage['returncode'] == 0 and stage['timed_out'] is False
                     and stage['containment_breach'] is False
                     and all(stage[key] == [] for key in CHILD_EMPTY),
                     name + ' child exited 0; no timeout, breach, problem, residue or survivor')
    return by_name


def _artifacts(by_name, tree):
    """Read every recorded stage artifact, bound to receipt record and postflight tree entry."""
    blobs, retained = {}, {}
    for name in STAGES:
        if name == INITIALIZER:
            continue
        recorded = by_name[name]['artifacts']
        keys = ENGINE_ARTIFACTS if name in ENGINE else STREAM_ARTIFACTS
        _require(set(recorded) == keys, name + ' records exactly ' + ', '.join(sorted(keys)))
        for key in sorted(keys):
            record, path = recorded[key], str(STAGES_DIR / name / key)
            _require(record['path'] == path and record['kind'] == 'file' and tree.get(path) == record,
                     f'{name}/{key} receipt record equals the postflight tree entry at its path')
            blobs[name, key] = _read(path, record['size'], record['sha256'], f'{name}/{key}')
            retained[f'{name}/{key}'] = dict(record, raw=_raw(blobs[name, key]))
    stage, path = by_name[INITIALIZER], str(STAGES_DIR / INITIALIZER / 'initializer.kore')
    record, entry = stage['initializer'], tree.get(path, {})
    _require(record['path'] == path and entry.get('sha256') == record['sha256']
             and entry.get('size') == record['bytes'],
             'initializer record equals the postflight tree entry at its path')
    blobs[INITIALIZER, 'initializer.kore'] = _read(path, record['bytes'], record['sha256'],
                                                   'initializer.kore')
    retained[INITIALIZER + '/initializer.kore'] = dict(entry, raw=_raw(blobs[INITIALIZER,
                                                                             'initializer.kore']))
    parsed, kast = stage['parsed'], by_name['01-kast']['artifacts']['stdout']
    _require(parsed['path'] == kast['path'] and parsed['sha256'] == kast['sha256']
             and parsed['bytes'] == kast['size'], 'initializer binds the parsed 01-kast stdout')
    return blobs, retained


def _commands(by_name, tools, definition_path):
    """Check the recorded command chain: input -> kast -> initializer -> 03..07 -> kprint 08..12."""
    kompiled = os.path.dirname(definition_path)
    kast = by_name['01-kast']['command']
    _require(kast[-1] == str(RUN_INPUT[0]) and _option(kast, '--definition') == kompiled,
             '01-kast parsed the retained N01-fuel-001.ui with the compiled definition')
    previous = str(STAGES_DIR / INITIALIZER / 'initializer.kore')
    for name in ENGINE:
        argv, output = by_name[name]['command'], str(STAGES_DIR / name / 'state.kore')
        depth = '0' if name == ENGINE[0] else '1'
        _require(argv[0] == tools['kore-exec'] and argv[1] == definition_path
                 and _option(argv, '--pattern') == previous and _option(argv, '--output') == output
                 and _option(argv, '--depth') == depth,
                 f'{name}: pinned kore-exec, compiled definition, pattern {previous}, depth {depth}')
        previous = output
    for engine, printer in zip(ENGINE, PRINT):
        argv = by_name[printer]['command']
        _require(len(argv) == 5 and argv[0] == tools['kprint'] and argv[1] == kompiled
                 and argv[2] == str(STAGES_DIR / engine / 'state.kore'),
                 f'{printer}: pinned kprint over the {engine} output')


def _engine_receipt(stage):
    artifacts = stage['artifacts']
    return dict(argv=list(stage['command']), exit_status=stage['returncode'],
                stdout_sha256=artifacts['stdout']['sha256'],
                stderr_sha256=artifacts['stderr']['sha256'],
                output_sha256=artifacts['state.kore']['sha256'])


def _print_receipt(stage):
    artifacts = stage['artifacts']
    return dict(argv=list(stage['command']), exit_status=stage['returncode'],
                stdout_sha256=artifacts['stdout']['sha256'],
                stderr_sha256=artifacts['stderr']['sha256'])


def _probe(by_name, blobs, engine, printer, log_key='log'):
    return {'configuration': _ascii(blobs[printer, 'stdout'], printer + ' stdout'),
            log_key: blobs[engine, 'stderr'], 'state_kore': blobs[engine, 'state.kore'],
            'engine_receipt': _engine_receipt(by_name[engine]),
            'print_receipt': _print_receipt(by_name[printer])}


def _load_adapter():
    """Hash the fixed source05 leaf and siblings, load the leaf, compare its import-time hashes."""
    for name, (size, digest) in ADAPTER.items():
        _read(QA / name, size, digest, 'adapter ' + name + ' before load')
    spec = importlib.util.spec_from_file_location('sherlock_k_capture_observations_05',
                                                  QA / MODULE_NAME)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    _require(module.ADAPTER_SOURCES == {name: digest for name, (_, digest) in ADAPTER.items()},
             'adapter import-time source hashes equal the fixed source05 hashes')
    return module


def _assemble(self_path, self_sha):
    receipt_bytes = _read(*RECEIPT, 'attempt receipt')
    receipt = _json(receipt_bytes, 'attempt receipt')
    preflight = receipt['preflight']
    by_name = _stages(receipt)
    blobs, retained = _artifacts(by_name, {entry['path']: entry
                                           for entry in receipt['postflight']['attempt_tree']})
    run_input = _read(*RUN_INPUT, 'retained run input')
    _require(any(entry.get('path') == str(RUN_INPUT[0]) and entry.get('sha256') == RUN_INPUT[2]
                 and entry.get('size') == RUN_INPUT[1] for entry in preflight['protected']['inputs']),
             'receipt protects the retained 43-byte N01-fuel-001.ui')
    _read(*SOURCE, 'compiled-input main.k')
    declared = preflight['sources']['models/sherlock/k/main.k']
    _require(declared == dict(bytes=SOURCE[1], sha256=SOURCE[2])
             and any(entry.get('path') == str(SOURCE[0]) and entry.get('sha256') == SOURCE[2]
                     for entry in preflight['protected']['compile_input']),
             'receipt declares and protects the compiled-input main.k source')
    inventory = _json(_read(*INVENTORY, 'compiled rule span inventory'), 'rule span inventory')
    rows, definition_path = inventory['rules'], inventory['definition_path']
    _require(inventory['schema'] == INVENTORY_SCHEMA and inventory['main_sha256'] == SOURCE[2]
             and inventory['definition_sha256'] == DEFINITION_SHA256
             and any(entry.get('path') == definition_path and entry.get('sha256') == DEFINITION_SHA256
                     for entry in preflight['protected']['compile_attempt']),
             'inventory binds main.k and the receipt-protected compiled definition.kore')
    _require(type(rows) is list and len(rows) == INVENTORY_ROWS
             and all(type(row) is dict and row['source_path'] == str(SOURCE[0])
                     and _attributes_agree(row) for row in rows),
             'inventory carries 113 compiled-input main.k rows with agreeing raw attributes')
    _commands(by_name, preflight['tools'], definition_path)
    adapter = _load_adapter()
    input_text = _ascii(run_input, 'run input')
    retained_input = adapter.retained_run_input(input_text)
    entry_k = retained_input['entry_k']
    _require(entry_k == ENTRY_K, 'entry_k ' + ENTRY_K + ' projected from the retained input only')
    captures = [dict(configuration=_ascii(blobs[STARTUP[1], 'stdout'], STARTUP[1] + ' stdout')),
                _probe(by_name, blobs, *SEMANTIC, log_key='incoming_log'),
                _probe(by_name, blobs, *PUBLICATION, log_key='incoming_log')]
    decoded = adapter.decode_capture(
        CASE_ID, entry_k, captures, source_path=str(SOURCE[0]), rule_spans=rows,
        terminal_probe=_probe(by_name, blobs, *TERMINAL),
        startup_probe=_probe(by_name, blobs, *STARTUP), run_input=input_text)
    bundle = adapter.observation_bundle([decoded], source_sha256=declared['sha256'])
    gaps = list(bundle['projection']['gaps'])
    if gaps or bundle['comparison_ready'] is not True:
        raise _Blocked('adapter reported gaps; bundle not written', gaps)
    observations = bundle['observations']
    _require(bundle['format'] == adapter.BUNDLE_FORMAT and set(observations) == OBSERVATION_KEYS
             and observations['producer'] == adapter.PRODUCER
             and observations['source_sha256'] == SOURCE[2]
             and observations['programs'] == [dict(case_id=CASE_ID, code_k=retained_input['code_k'],
                                                   entry_k=entry_k)],
             'bundle format, six observation keys, producer, source and program identity unchanged')
    links = decoded['edge_links']
    _require(len(links) == 2 and all(
        link['engine_output_kore_sha256'] == by_name[engine]['artifacts']['state.kore']['sha256']
        and link['print_stdout_configuration_sha256'] == by_name[printer]['artifacts']['stdout']['sha256']
        and link['engine_output_kore_sha256'] != link['print_stdout_configuration_sha256']
        for link, (engine, printer) in zip(links, (SEMANTIC, PUBLICATION))),
        'edge links carry receipt Kore and pretty hashes separately; no Kore hash as pretty hash')
    for name, (size, digest) in ADAPTER.items():
        _read(QA / name, size, digest, 'adapter ' + name + ' after the run')
    return dict(
        schema=ENVELOPE,
        bundle=bundle,
        scope=dict(case_id=CASE_ID, attempt=ATTEMPT.name, run_input=str(RUN_INPUT[0]),
                   initial_fuel=retained_input['initial_fuel'],
                   claim='exactly one actual N01 fuel-1 capture',
                   case_id_origin='program prefix of the receipt-recorded kast input file name; '
                                  'no oracle case-id convention consulted',
                   not_claimed=['K match or common-state comparison (none has run)',
                                '216-input or any corpus coverage', 'producer authenticity',
                                'rewrite axioms distinguished from equations',
                                'Moriarty SDK, native, proof, circuit or publication authority']),
        pairing=dict(startup=dict(engine=STARTUP[0], printer=STARTUP[1],
                                  before='pre-startup state only as the On-configuration Kore '
                                         'logged by the 04 engine; never typed'),
                     post_startup_initial=dict(printer=STARTUP[1]),
                     semantic_edge=dict(engine=SEMANTIC[0], printer=SEMANTIC[1]),
                     publication_edge=dict(engine=PUBLICATION[0], printer=PUBLICATION[1]),
                     terminal_replay=dict(engine=TERMINAL[0], printer=TERMINAL[1]),
                     retained_only=['01-kast', INITIALIZER, ENGINE[0], PRINT[0]]),
        sources=dict(assembler=dict(path=self_path, sha256=self_sha),
                     adapter={name: dict(path=str(QA / name), size=size, sha256=digest)
                              for name, (size, digest) in ADAPTER.items()},
                     adapter_import_sources=dict(adapter.ADAPTER_SOURCES),
                     receipt=_ref(RECEIPT), rule_span_inventory=dict(_ref(INVENTORY), rows=len(rows)),
                     compiled_source=dict(_ref(SOURCE), declared_by='receipt preflight sources '
                                                                   'and compile_input protection'),
                     run_input=_ref(RUN_INPUT)),
        retained=dict(receipt=_raw(receipt_bytes), stage_receipts=receipt['stages'],
                      artifacts=retained, run_input_text=input_text),
        consistency=dict(status='consistent',
                         meaning='supplied bytes, receipt records, postflight tree, command path '
                                 'chain and adapter gaps agree; this is not authentication',
                         checks=list(_CHECKS), comparison_ready=bundle['comparison_ready'],
                         adapter_gaps=gaps),
        authenticity=dict(status='not-established',
                          reasons=['receipt, artifacts, argv, exit status and pairing are supplied '
                                   'bytes checked for consistency only',
                                   'the attempt receipt is unsigned; driver and identity hashes '
                                   'are compared, not attested',
                                   'adapter hashes are checked before load, at import and after the '
                                   'run, but are not bound to the executed code',
                                   'the assembler self hash is compared only with the '
                                   'operator-supplied approved hash']),
        reliance=dict(status='blocked', requires=[RELIANCE],
                      comparison='independent comparison against the withheld Root expected '
                                 'subset has not run'),
        not_opened=['candidate oracles and any expected-outcome data',
                    'the withheld independent Root expected subset',
                    'ACTUAL-FIXTURE-BYTES.json and source05 tests',
                    'compiled definition.kore (its hash is taken from the receipt protection record)',
                    'the receipt expectations field (retained verbatim in the raw receipt only)'])


def _encode(value):
    return json.dumps(value, sort_keys=True, indent=1, ensure_ascii=True,
                      allow_nan=False).encode('ascii') + b'\n'


def _create(path, data):
    descriptor = os.open(str(path), os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW
                         | os.O_CLOEXEC, 0o600)
    try:
        view = memoryview(data)
        while view:
            view = view[os.write(descriptor, view):]
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def _status(outcome, self_sha, **fields):
    return dict(schema=STATUS_SCHEMA, outcome=outcome, attempt=ATTEMPT.name, case_id=CASE_ID,
                scope='one N01 fuel-1 capture only', output=str(OUTPUT),
                assembler_sha256=self_sha, adapter_sha256=ADAPTER[MODULE_NAME][1],
                checks_passed=list(_CHECKS), authenticity='not-established', reliance=RELIANCE,
                **fields)


def _refuse(message):
    print('refused before any write: ' + message, file=sys.stderr)
    return 2


def _blocked(self_sha, message, gaps):
    try:
        _create(STATUS, _encode(_status('blocked', self_sha, error=message, gaps=gaps)))
    except OSError as failure:
        print('operator status not written: ' + str(failure), file=sys.stderr)
    print('blocked: ' + message, file=sys.stderr)
    return 2


def main(argv=None):
    parser = argparse.ArgumentParser(description='Assemble the one actual N01 fuel-1 bundle.')
    parser.add_argument('--approved-self-sha256', required=True)
    parser.add_argument('--approved-module-sha256', required=True)
    args = parser.parse_args(argv)
    if not sys.flags.isolated:
        return _refuse('run under python3 -I -B')
    if any(_DIGEST.fullmatch(value) is None
           for value in (args.approved_self_sha256, args.approved_module_sha256)):
        return _refuse('approved hashes must be lowercase hex SHA-256')
    self_path = os.path.abspath(__file__)
    try:
        self_sha = _sha(_bytes_at(self_path, MAX_SELF_BYTES))
    except (OSError, ValueError) as error:
        return _refuse('assembler bytes unreadable: ' + str(error))
    if self_sha != args.approved_self_sha256:
        return _refuse('assembler bytes differ from the approved self hash; found ' + self_sha)
    if args.approved_module_sha256 != ADAPTER[MODULE_NAME][1]:
        return _refuse('approved module hash is not the fixed source05 module hash')
    try:
        directory = (stat.S_ISDIR(os.lstat(OUT_DIR).st_mode)
                     and os.path.realpath(OUT_DIR) == str(OUT_DIR))
    except OSError:
        directory = False
    if not directory or os.path.lexists(OUTPUT) or os.path.lexists(STATUS):
        return _refuse('output directory absent, or output or status already exists')
    try:
        payload = _encode(_assemble(self_path, self_sha))
    except _Blocked as error:
        return _blocked(self_sha, str(error), error.gaps)
    except (ValueError, KeyError, TypeError, AttributeError, IndexError, OSError,
            RecursionError) as error:
        return _blocked(self_sha, f'{type(error).__name__}: {error}', [])
    try:
        _create(OUTPUT, payload)
    except OSError as error:
        return _blocked(self_sha, f'output creation failed; a partial file may exist: {error}', [])
    try:
        _create(STATUS, _encode(_status('written', self_sha, output_sha256=_sha(payload),
                                        output_size=len(payload))))
    except OSError as error:
        print('bundle written; operator status not written: ' + str(error), file=sys.stderr)
        return 3
    print(f'written {OUTPUT} sha256 {_sha(payload)} size {len(payload)}; '
          'consistency checked, authenticity not established, reliance blocked')
    return 0


if __name__ == '__main__':
    sys.exit(main())
