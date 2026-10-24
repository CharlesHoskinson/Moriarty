"""Read-only DoctorWatson delivery selector, proposed public adapter interface.

This module opens pinned files, decodes bounded public JSON and reports conditional
project readiness. It does not launch tools, prove, query a ledger, authenticate a
producer, or issue execution permission. For sherlock-anchor-settle-readback the
fixed settle_observation decoder is integrated below: a supplied observation alone
cannot pass without a decoding raw harness source-result. Other native producers
and the selected existing operation boundary remain separate owner responsibilities.
"""
from __future__ import annotations

import argparse
from dataclasses import dataclass
import hashlib
import json
import math
import os
from pathlib import Path
import re
import stat
import sys
from typing import Any

from capabilities import adapter_capability

MAX_JSON_BYTES = 4 * 1024 * 1024
MAX_ARTIFACT_BYTES = 1024 * 1024 * 1024
MAX_TASKS = 128
HEX = re.compile(r'[0-9a-f]{64}\Z')
NAME = re.compile(r'[A-Za-z0-9][A-Za-z0-9._:-]{0,127}\Z')
COMMON_ROLES = ('observation', 'stdout', 'stderr', 'resource-receipt')
BUILD9 = ('contract/index.js', 'zkir/acceptValidity.zkir', 'zkir/acceptValidity.bzkir',
          'keys/acceptValidity.prover', 'keys/acceptValidity.verifier',
          'zkir/settleValidity.zkir', 'zkir/settleValidity.bzkir',
          'keys/settleValidity.prover', 'keys/settleValidity.verifier')
ROLES = {
    'source-integration': ('observation',),
    'moriarty-sdk-build': COMMON_ROLES + ('backend-elf',),
    'sdk-selected-tests': COMMON_ROLES,
    'compact-anchor-build9': COMMON_ROLES + BUILD9,
    'handkey-verify-pis': COMMON_ROLES + ('key', 'proof', 'statement'),
    'recl9-terminal-verify': COMMON_ROLES + ('relation', 'key', 'proof', 'statement'),
    'sherlock-anchor-settle-readback': COMMON_ROLES + ('source-result', 'post-state'),
    'sherlock-rollup-benchmark': COMMON_ROLES + ('source-result',),
}
HANDKEY_PINS = ('key_sha256', 'proof_sha256', 'statement_sha256')
VERIFY_PINS = ('relation_sha256', 'key_sha256', 'proof_sha256', 'statement_sha256')
# Predispatch inputs only: observed state bytes are not part of task identity.
SETTLE_INPUTS = ('network', 'anchor', 'deployment_sha256', 'binding_sha256', 'batch', 'expected_root')
BENCH_INPUTS = ('profile', 'transactions', 'batches', 'seed')


class SchemaError(ValueError):
    """Malformed or unsupported input, distinct from a failed operation."""


class CleanupDebt(SchemaError):
    pass


def _fields(value: Any, keys: tuple[str, ...], at: str) -> dict:
    if type(value) is not dict or set(value) != set(keys):
        raise SchemaError(f'{at}: expected exactly {keys}')
    return value


def _text(value: Any, at: str) -> str:
    if type(value) is not str or not value or len(value) > 4096:
        raise SchemaError(f'{at}: expected nonempty bounded string')
    return value


def _name(value: Any, at: str) -> str:
    if type(value) is not str or not NAME.fullmatch(value):
        raise SchemaError(f'{at}: expected identifier')
    return value


def _enum(value: Any, choices, at: str) -> str:
    if type(value) is not str or value not in choices:
        raise SchemaError(f'{at}: unknown value {value!r}')
    return value


def _int(value: Any, at: str, minimum=0, maximum=2**63-1) -> int:
    if type(value) is not int or not minimum <= value <= maximum:
        raise SchemaError(f'{at}: expected integer in [{minimum},{maximum}]')
    return value


def _bool(value: Any, at: str) -> bool:
    if type(value) is not bool:
        raise SchemaError(f'{at}: expected boolean')
    return value


def _sha(value: Any, at: str) -> str:
    if type(value) is not str or not HEX.fullmatch(value):
        raise SchemaError(f'{at}: expected lowercase SHA-256')
    return value


def _array(value: Any, at: str, maximum=MAX_TASKS, minimum=0) -> list:
    if type(value) is not list or not minimum <= len(value) <= maximum:
        raise SchemaError(f'{at}: expected bounded array')
    return value


def _unique(values, at: str):
    if len(values) != len(set(values)):
        raise SchemaError(f'{at}: duplicate member')


def _pairs(pairs):
    out = {}
    for key, value in pairs:
        if key in out:
            raise SchemaError(f'duplicate JSON member: {key}')
        out[key] = value
    return out


def _float(text):
    value = float(text)
    if not math.isfinite(value):
        raise SchemaError('nonfinite JSON number')
    return value


def _constant(text):
    raise SchemaError(f'non-JSON numeric constant: {text}')


def _json(raw: bytes):
    if len(raw) > MAX_JSON_BYTES:
        raise SchemaError('JSON size limit exceeded')
    try:
        return json.loads(raw.decode('utf-8'), object_pairs_hook=_pairs,
                          parse_constant=_constant, parse_float=_float)
    except (UnicodeError, json.JSONDecodeError, RecursionError, ValueError) as exc:
        raise SchemaError(f'invalid JSON: {exc}') from exc


def _canonical(value) -> str:
    return json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False)


def _digest(value) -> str:
    return hashlib.sha256(_canonical(value).encode()).hexdigest()


@dataclass(frozen=True)
class FileRef:
    path: str
    bytes: int
    sha256: str
    role: str = ''

    def record(self):
        value = {'path': self.path, 'bytes': self.bytes, 'sha256': self.sha256}
        if self.role:
            value['role'] = self.role
        return value


def _ref(value, at, role=False):
    _fields(value, ('path', 'bytes', 'sha256', 'role') if role else ('path','bytes','sha256'), at)
    path = _text(value['path'], at + '.path')
    if any(ord(c)<32 for c in path) or not Path(path).is_absolute() or str(Path(os.path.normpath(path))) != path:
        raise SchemaError(f'{at}: file path must be normalized absolute')
    return FileRef(path, _int(value['bytes'], at+'.bytes', maximum=MAX_ARTIFACT_BYTES),
                   _sha(value['sha256'], at+'.sha256'), _text(value['role'], at+'.role') if role else '')


def _read(ref: FileRef, *, public_json=False):
    """Read once per check, hash opaque assets in chunks, retain no proof payload."""
    if public_json and ref.bytes > MAX_JSON_BYTES:
        raise SchemaError('public JSON size limit exceeded')
    fd = os.open(ref.path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        before = os.fstat(fd)
        if not stat.S_ISREG(before.st_mode) or before.st_size != ref.bytes:
            raise SchemaError(f'{ref.path}: not a regular file of pinned size')
        digest = hashlib.sha256()
        chunks = [] if public_json else None
        total = 0
        while True:
            chunk = os.read(fd, 1024 * 1024)
            if not chunk:
                break
            total += len(chunk)
            if total > ref.bytes:
                raise SchemaError(f'{ref.path}: file grew while reading')
            digest.update(chunk)
            if chunks is not None:
                chunks.append(chunk)
        after = os.fstat(fd)
        stamp = lambda s: (s.st_dev,s.st_ino,s.st_size,s.st_mtime_ns,s.st_ctime_ns)
        if stamp(before) != stamp(after) or total != ref.bytes or digest.hexdigest() != ref.sha256:
            raise SchemaError(f'{ref.path}: pinned bytes changed')
        return _json(b''.join(chunks)) if chunks is not None else None
    finally:
        os.close(fd)


class _AssessmentReads:
    """Reuse exact FileRefs only during one assessment, never across dispatches.

    Parsing mode is part of identity; roles do not change the checked bytes.
    Opaque artifact reads retain no payload. Failures are local too.
    """
    def __init__(self):
        self.cache = {}

    def read(self, ref, *, public_json=False):
        key = (ref.path, ref.bytes, ref.sha256, public_json)
        if key not in self.cache:
            try:
                self.cache[key] = (True, _read(ref, public_json=public_json))
            except (OSError, SchemaError) as exc:
                self.cache[key] = (False, exc)
        succeeded, value = self.cache[key]
        if not succeeded:
            raise value
        return value


def _document_bytes(path):
    # Input documents are data only; their paths never become executable authority.
    with open(path, 'rb') as stream:
        return stream.read(MAX_JSON_BYTES + 1)


def _document(path):
    return _json(_document_bytes(path))


@dataclass(frozen=True)
class Task:
    id: str
    owner: str
    adapter: str
    attempt: str
    disposition: str
    launch_state: str
    identity_json: str
    inputs_json: str
    dependencies: tuple[tuple[str, str], ...]
    eligible_reviewers: tuple[str, ...]
    input_files: tuple[FileRef, ...]

    @property
    def inputs(self):
        return json.loads(self.inputs_json)


@dataclass(frozen=True)
class Plan:
    integrator: str
    tasks: tuple[Task, ...]
    order: tuple[str, ...]

    @property
    def by_id(self):
        return {task.id: task for task in self.tasks}


def required_roles(adapter):
    if adapter not in ROLES:
        raise SchemaError('unknown adapter')
    return ROLES[adapter]


def _inputs(adapter, value):
    keys = {'source-integration': (), 'moriarty-sdk-build': (), 'sdk-selected-tests': ('selectors',),
            'compact-anchor-build9': (), 'recl9-terminal-verify': VERIFY_PINS,
            'handkey-verify-pis': HANDKEY_PINS,
            'sherlock-anchor-settle-readback': SETTLE_INPUTS,
            'sherlock-rollup-benchmark': BENCH_INPUTS}[adapter]
    _fields(value, keys, 'task.inputs')
    if adapter == 'sdk-selected-tests':
        names = [_text(n, 'selector') for n in _array(value['selectors'],'selectors',minimum=1)]
        _unique(names, 'selectors')
    elif adapter in ('recl9-terminal-verify','handkey-verify-pis'):
        for key in (VERIFY_PINS if adapter=='recl9-terminal-verify' else HANDKEY_PINS):
            _sha(value[key], key)
    elif adapter == 'sherlock-anchor-settle-readback':
        for key in ('network','anchor'):
            _text(value[key], key)
        for key in ('deployment_sha256','binding_sha256'):
            _sha(value[key],key)
        _int(value['batch'],'batch')
        root=value['expected_root']
        if (type(root) is not str or re.fullmatch(r'(0|[1-9][0-9]{0,77})',root) is None
            or int(root)>2**256-1):
            raise SchemaError('expected_root: expected canonical public decimal uint256')
    elif adapter == 'sherlock-rollup-benchmark':
        _text(value['profile'],'profile'); _text(value['seed'],'seed')
        batches = _int(value['batches'],'batches',minimum=1,maximum=6)
        tx = _int(value['transactions'],'transactions',minimum=16,maximum=96)
        if batches not in (1,2,4,6) or tx != 16*batches:
            raise SchemaError('benchmark requires explicit 16/32/64/96 transaction prefix')
    return _canonical(value)


def validate_dag(tasks: tuple[Task, ...]) -> tuple[str, ...]:
    by_id = {task.id: task for task in tasks}
    _unique([t.id for t in tasks], 'task ids')
    visiting, done, order = set(), set(), []
    def visit(name):
        if name not in by_id:
            raise SchemaError(f'unknown dependency {name}')
        if name in visiting:
            raise SchemaError(f'dependency cycle at {name}')
        if name in done:
            return
        visiting.add(name)
        for dep, stage in by_id[name].dependencies:
            visit(dep)
            if stage=='source-integrated' and by_id[dep].adapter!='source-integration':
                raise SchemaError(f'{name}: source-integrated edge requires source-integration producer {dep}')
        visiting.remove(name); done.add(name); order.append(name)
    for task in tasks:
        visit(task.id)
    return tuple(order)


def parse_plan_bytes(raw: bytes) -> Plan:
    """Strictly parse a plan from bytes already read once by the caller.

    Byte entrypoint for callers that pin/hash the exact raw bytes they
    parse (reconciliation.load_readiness): the returned Plan is derived
    from ``raw`` itself, never from a second pathname read.
    """
    value = _fields(_json(raw), ('schema','integrator','tasks'),'plan')
    _enum(value['schema'], ('doctorwatson.delivery-plan/1',),'plan.schema')
    integrator = _name(value['integrator'],'integrator')
    tasks = []
    for raw in _array(value['tasks'],'tasks',minimum=1):
        _fields(raw, ('id','owner','adapter','attempt','disposition','launch_state','identity',
                      'inputs','dependencies','eligible_reviewers'),'task')
        adapter = _enum(raw['adapter'],ROLES,'adapter')
        identity = _fields(raw['identity'],('integration_base','source','tools','config','oracle_sha256'),'identity')
        _text(identity['integration_base'],'integration_base'); _sha(identity['oracle_sha256'],'oracle_sha256')
        refs = []
        for category in ('source','tools','config'):
            group = [_ref(v,category) for v in _array(identity[category],category,minimum=1)]
            _unique([ref.path for ref in group],category)
            refs.extend(group)
        deps = []
        for dep in _array(raw['dependencies'],'dependencies'):
            _fields(dep,('task','stage'),'dependency')
            deps.append((_name(dep['task'],'dependency.task'),
                         _enum(dep['stage'],('source-integrated','observed','accepted'),'dependency.stage')))
        _unique([d[0] for d in deps],'dependencies')
        reviewers = tuple(_name(v,'reviewer') for v in _array(raw['eligible_reviewers'],'reviewers',minimum=1))
        _unique(reviewers,'reviewers')
        tasks.append(Task(_name(raw['id'],'task.id'),_name(raw['owner'],'owner'),adapter,
                          _name(raw['attempt'],'attempt'),
                          _enum(raw['disposition'],('active','blocked','cancelled'),'disposition'),
                          _enum(raw['launch_state'],('not-started','possible-launch'),'launch_state'),
                          _canonical(identity),_inputs(adapter,raw['inputs']),tuple(deps),reviewers,tuple(refs)))
    tasks = tuple(tasks)
    return Plan(integrator,tasks,validate_dag(tasks))


def load_plan(path) -> Plan:
    return parse_plan_bytes(_document_bytes(path))


def fingerprints(plan: Plan) -> dict[str, str]:
    """Compute the existing fingerprint format once in dependency order."""
    by_id = plan.by_id
    cache = {}
    for name in plan.order:
        task = by_id[name]
        value = {'task':task.id,'adapter':task.adapter,'identity':json.loads(task.identity_json),
                 'inputs':task.inputs,'owner':task.owner,'eligible_reviewers':task.eligible_reviewers,
                 'integrator':plan.integrator,'dependencies':[
                     {'task':dep,'stage':stage,'fingerprint':cache[dep],'attempt':by_id[dep].attempt}
                     for dep,stage in task.dependencies]}
        cache[name] = _digest(value)
    return cache


def fingerprint(plan: Plan, name: str) -> str:
    return fingerprints(plan)[name]


@dataclass(frozen=True)
class Result:
    task: str
    attempt: str
    fingerprint: str
    adapter: str
    terminal: str
    exit_code: int | None
    signal: int | None
    cleanup: str
    submission: str
    artifacts: tuple[FileRef, ...]
    review: FileRef | None
    adoption: FileRef | None


def load_result(raw) -> Result:
    _fields(raw,('task','attempt','fingerprint','adapter','terminal','exit_code','signal','cleanup',
                 'submission','artifacts','review','adoption'),'result')
    adapter = _enum(raw['adapter'],ROLES,'result.adapter')
    artifacts = tuple(_ref(v,'artifact',True) for v in _array(raw['artifacts'],'artifacts'))
    _unique([a.role for a in artifacts],'artifact roles')
    if any(a.role not in required_roles(adapter) for a in artifacts):
        raise SchemaError('unexpected artifact role')
    for key,maximum in [('exit_code',255),('signal',128)]:
        if raw[key] is not None:
            _int(raw[key],key, minimum=1 if key=='signal' else 0,maximum=maximum)
    review = _ref(raw['review'],'review',True) if raw['review'] is not None else None
    adoption = _ref(raw['adoption'],'adoption',True) if raw['adoption'] is not None else None
    if (review and review.role!='review') or (adoption and adoption.role!='adoption'):
        raise SchemaError('review/adoption reference role mismatch')
    return Result(_name(raw['task'],'result.task'),_name(raw['attempt'],'result.attempt'),
                  _sha(raw['fingerprint'],'result.fingerprint'),adapter,
                  _enum(raw['terminal'],('succeeded','failed','refused','timed-out','cancelled','unknown-submission'),'terminal'),
                  raw['exit_code'],raw['signal'],_enum(raw['cleanup'],('complete','owed','unknown'),'cleanup'),
                  _enum(raw['submission'],('not-applicable','not-submitted','included','rejected','unknown'),'submission'),
                  artifacts,review,adoption)


def parse_history_bytes(raw: bytes) -> tuple[Result, ...]:
    """Strictly parse history from bytes already read once by the caller.

    Byte entrypoint matching parse_plan_bytes: the returned Results are
    derived from ``raw`` itself, never from a second pathname read.
    """
    value = _fields(_json(raw),('schema','results'),'history')
    _enum(value['schema'],('doctorwatson.delivery-history/1',),'history.schema')
    results = tuple(load_result(v) for v in _array(value['results'],'results',maximum=4096))
    _unique([(r.task,r.attempt) for r in results],'result attempts')
    return results


def load_history(path) -> tuple[Result, ...]:
    return parse_history_bytes(_document_bytes(path))


def artifact_manifest_digest(refs):
    records = [r.record() if isinstance(r,FileRef) else r for r in refs]
    return _digest(sorted(records,key=lambda r:r['role']))


def _observation(task: Task, result: Result, value):
    payload = {
        'source-integration': ('integrated',),
        'moriarty-sdk-build': ('built','test_bodies_executed'),
        'sdk-selected-tests': ('tests',),
        'compact-anchor-build9': ('built','skip_zk'),
        'recl9-terminal-verify': ('verified',)+VERIFY_PINS,
        'handkey-verify-pis': ('verifies',)+HANDKEY_PINS,
        'sherlock-anchor-settle-readback': ('accepted','settled','phase','post_state_sha256')+SETTLE_INPUTS,
        'sherlock-rollup-benchmark': ('settled',)+BENCH_INPUTS+('k','validator_cpu_ms','validator_cpu_status'),
    }[task.adapter]
    _fields(value,('schema','task','attempt','fingerprint')+payload,'observation')
    expected = {'schema':'doctorwatson.'+task.adapter+'-observation/1','task':task.id,
                'attempt':result.attempt,'fingerprint':result.fingerprint}
    if any(type(value[k]) is not str or value[k]!=v for k,v in expected.items()):
        raise SchemaError('observation source/task/attempt identity mismatch')
    inputs = task.inputs
    if task.adapter == 'source-integration':
        return _bool(value['integrated'],'integrated') is True
    if task.adapter == 'moriarty-sdk-build':
        built=_bool(value['built'],'built')
        executed=_bool(value['test_bodies_executed'],'test_bodies_executed')
        return built is True and executed is False
    if task.adapter == 'sdk-selected-tests':
        names=[]; passed=True
        for item in _array(value['tests'],'tests'):
            _fields(item,('name','status'),'test')
            names.append(_text(item['name'],'test.name'))
            passed = (_enum(item['status'],('passed','failed','listed','not-run'),'test.status')=='passed') and passed
        _unique(names,'executed test names')
        return passed and set(names)==set(inputs['selectors'])
    if task.adapter == 'compact-anchor-build9':
        built=_bool(value['built'],'built'); skip=_bool(value['skip_zk'],'skip_zk')
        return built is True and skip is False
    if task.adapter == 'handkey-verify-pis':
        for key in HANDKEY_PINS:
            _sha(value[key],key)
        return _bool(value['verifies'],'verifies') is True and all(value[k]==inputs[k] for k in HANDKEY_PINS)
    if task.adapter == 'recl9-terminal-verify':
        for key in VERIFY_PINS:
            _sha(value[key],key)
        return _bool(value['verified'],'verified') is True and all(value[k]==inputs[k] for k in VERIFY_PINS)
    if task.adapter == 'sherlock-anchor-settle-readback':
        _inputs(task.adapter,{k:value[k] for k in SETTLE_INPUTS})
        _sha(value['post_state_sha256'],'observed post_state_sha256')
        accepted=_bool(value['accepted'],'accepted'); settled=_bool(value['settled'],'settled')
        return (accepted is True and settled is True
                and _text(value['phase'],'phase')=='SETTLED_FINALIZED_READBACK_OK'
                and all(type(value[k]) is type(inputs[k]) and value[k]==inputs[k] for k in SETTLE_INPUTS))
    _inputs(task.adapter,{k:value[k] for k in BENCH_INPUTS})
    _int(value['k'],'k',minimum=1,maximum=63)
    status = _enum(value['validator_cpu_status'],('measured','unavailable'),'validator_cpu_status')
    if status=='unavailable':
        if value['validator_cpu_ms'] is not None:
            raise SchemaError('unavailable validator CPU must remain null')
    else:
        _int(value['validator_cpu_ms'],'validator_cpu_ms')
    return _bool(value['settled'],'settled') is True and all(value[k]==inputs[k] for k in BENCH_INPUTS)


def _cleanup(task: Task, result: Result, value):
    _fields(value,('schema','task','attempt','fingerprint','cleanup','restoration'),'cleanup observation')
    expected={'schema':'doctorwatson.cleanup-observation/1','task':task.id,
              'attempt':result.attempt,'fingerprint':result.fingerprint}
    if any(type(value[k]) is not str or value[k]!=v for k,v in expected.items()):
        raise SchemaError('cleanup observation identity mismatch')
    for key in ('cleanup','restoration'):
        if _enum(value[key],('complete','owed','unknown'),key)!='complete':
            raise CleanupDebt(f'actual cleanup observation retains {key} debt')


def validate_artifacts(task: Task, result: Result, *, _reader=None):
    read = _reader or _read
    if {a.role for a in result.artifacts} != set(required_roles(task.adapter)):
        raise SchemaError('required artifact membership incomplete')
    observation = None
    native_verifier = None
    settle_source_result = None
    for ref in result.artifacts:
        if ref.bytes == 0 and ref.role not in ('stdout','stderr'):
            raise SchemaError(f'{ref.role}: empty required artifact')
        value = read(ref,public_json=(ref.role in ('observation','resource-receipt') or (task.adapter=='handkey-verify-pis' and ref.role=='stdout') or (task.adapter=='sherlock-anchor-settle-readback' and ref.role=='source-result')))
        if task.adapter=='sherlock-anchor-settle-readback' and ref.role=='source-result':
            settle_source_result = value
        if ref.role=='resource-receipt':
            _cleanup(task,result,value)
        if task.adapter=='handkey-verify-pis' and ref.role=='stdout':
            native_verifier = _fields(value,('verifies','verify_s'),'handkey stdout')
            _bool(value['verifies'],'handkey.verifies')
            duration=value['verify_s']
            if type(duration) not in (int,float) or not 0 <= duration <= 2**63-1 or not math.isfinite(duration):
                raise SchemaError('handkey.verify_s must be nonnegative finite number')
        if ref.role=='observation':
            observation = value
    success=_observation(task,result,observation)
    # Bind the opaque proof/key/statement/relation bytes, without deserializing them.
    if task.adapter in ('recl9-terminal-verify','handkey-verify-pis'):
        for ref in result.artifacts:
            if ref.role+'_sha256' in VERIFY_PINS and ref.sha256!=task.inputs[ref.role+'_sha256']:
                raise SchemaError(f'{ref.role}: opaque asset pin mismatch')
    if task.adapter=='sherlock-anchor-settle-readback':
        post = next(a for a in result.artifacts if a.role=='post-state')
        if post.sha256!=observation['post_state_sha256']:
            raise SchemaError('observed post-state digest differs from checked output artifact')
        # Fixed decoder: supplied observation booleans are not
        # sufficient; the raw harness result must decode and agree.
        import settle_observation
        try:
            decoded = settle_observation.decode_settle_value(
                settle_source_result, task.inputs,
                post_state_sha256=post.sha256)
        except settle_observation.SettleDecodeError as exc:
            raise SchemaError(f'source-result settle decode failed: {exc}') from exc
        for key in ('accepted','settled','phase','post_state_sha256')+SETTLE_INPUTS:
            if observation[key]!=decoded[key]:
                raise SchemaError('normalized settle observation != decoded raw source-result')
    if task.adapter=='handkey-verify-pis':
        success=success and native_verifier['verifies'] is True
    return observation, success


def _acceptance(plan, task, result, *, _reader=None):
    """Applicable local review/adoption are external premises, not authentication."""
    read = _reader or _read
    if result.review is None or result.adoption is None:
        return False,['missing applicable review or Root adoption']
    try:
        review = _fields(read(result.review,public_json=True),
                         ('schema','task','attempt','fingerprint','artifact_manifest_sha256','reviewer','approved'),'review')
        adoption = _fields(read(result.adoption,public_json=True),
                           ('schema','task','attempt','fingerprint','artifact_manifest_sha256','integrator','review_sha256','adopted'),'adoption')
        _enum(review['schema'],('doctorwatson.delivery-review/1',),'review.schema')
        _enum(adoption['schema'],('doctorwatson.delivery-adoption/1',),'adoption.schema')
        common={'task':task.id,'attempt':result.attempt,'fingerprint':result.fingerprint,
                'artifact_manifest_sha256':artifact_manifest_digest(result.artifacts)}
        for value in (review,adoption):
            if any(type(value[k]) is not str or value[k]!=v for k,v in common.items()):
                raise SchemaError('review/adoption binding mismatch')
        reviewer = _name(review['reviewer'],'reviewer')
        if reviewer==task.owner or reviewer not in task.eligible_reviewers:
            raise SchemaError('reviewer is author or not eligible')
        if _bool(review['approved'],'approved') is not True:
            raise SchemaError('review is not approved')
        if (_name(adoption['integrator'],'integrator')!=plan.integrator or
            _sha(adoption['review_sha256'],'review_sha256')!=result.review.sha256 or
            _bool(adoption['adopted'],'adopted') is not True):
            raise SchemaError('Root adoption does not bind applicable approved review')
        return True,[]
    except (OSError,SchemaError) as exc:
        return False,[str(exc)]


def attempt_obligations(task: Task, result: Result, currentness: str, *, _reader=None):
    """Per-original (unknown, cleanup_debt) for ONE history attempt.

    This is the single consumer derivation used by assess itself and by
    the reconciliation fold, so both judge an original's obligations
    identically. Debt has two sources: the Result record's own cleanup
    string, and -- for any non-current attempt of a non-source-integration
    adapter -- the actual resource-receipt artifact: missing membership,
    unreadable/unpinned bytes, wrong task/attempt/fingerprint identity,
    or a receipt whose cleanup/restoration is not complete (all checked
    against the pinned artifact bytes via _read + _cleanup) all count as
    debt even when the record claims cleanup complete.
    """
    read = _reader or _read
    unknown = result.submission=='unknown' or result.terminal=='unknown-submission'
    debt = result.cleanup!='complete'
    if currentness!='current' and task.adapter!='source-integration':
        receipt=next((a for a in result.artifacts if a.role=='resource-receipt'),None)
        try:
            if receipt is None:
                raise CleanupDebt('historical cleanup receipt missing')
            _cleanup(task,result,read(receipt,public_json=True))
        except (OSError,SchemaError):
            debt=True
    return unknown,debt


def assess(plan: Plan, results: tuple[Result, ...]):
    return _assess(plan, results, _AssessmentReads())


def _assess(plan: Plan, results: tuple[Result, ...], reads):
    by_id = plan.by_id
    task_fingerprints = fingerprints(plan)
    rows={}
    ready=[]
    for name in plan.order:
        task=by_id[name]; fp=task_fingerprints[name]
        blockers=[]; files_current=True
        for ref in task.input_files:
            try:
                reads.read(ref)
            except (OSError,SchemaError) as exc:
                files_current=False; blockers.append(f'current input unavailable or changed: {exc}')
        unmet=[]
        for dep,stage in task.dependencies:
            upstream=rows[dep]
            available={'source-integrated':upstream['source_integrated_recorded'],
                       'observed':upstream['observation_recorded'],
                       'accepted':upstream['acceptance_recorded']}[stage]
            if not available:
                unmet.append(f'{dep} requires {stage}')
        blockers.extend(unmet)
        history=[]; current=None
        historical_unknown=False; historical_debt=False
        for result in results:
            if result.task!=name:
                continue
            if result.attempt!=task.attempt:
                freshness='stale-attempt'
            elif result.fingerprint!=fp or result.adapter!=task.adapter:
                freshness='stale-fingerprint'
            elif not files_current:
                freshness='stale-input-files'
            elif unmet:
                freshness='stale-dependencies'
            else:
                freshness='current'; current=result
            obligation_unknown,obligation_debt=attempt_obligations(task,result,freshness,_reader=reads.read)
            if obligation_unknown:
                historical_unknown=True
            if obligation_debt:
                historical_debt=True
            history.append({'attempt':result.attempt,'fingerprint':result.fingerprint,
                            'terminal':result.terminal,'currentness':freshness,
                            'artifacts':[a.record() for a in result.artifacts],
                            'review':result.review.record() if result.review else None,
                            'adoption':result.adoption.record() if result.adoption else None})
        row={'id':name,'adapter':task.adapter,'attempt':task.attempt,'fingerprint':fp,
             'scope':('supplied handkey public-vector verification; ledger acceptance not established' if task.adapter=='handkey-verify-pis' else 'supplied typed public adapter observation; native producer authenticity not established'),
             'state':'ready','source_integrated_recorded':False,'observation_recorded':False,
             'acceptance_recorded':False,'producer_authenticity':'not-established',
             'observation':None,'history':history,'blockers':blockers,'acceptance_blockers':[],
             'execution_authority':False,'selection_candidate':False}
        if task.disposition!='active':
            row['state']=task.disposition
        elif historical_unknown:
            row['state']='unknown-reconcile'; blockers.append('historical submission outcome unresolved; reconcile original attempt')
        elif historical_debt:
            row['state']='cleanup-debt'; blockers.append('historical cleanup/restoration debt remains unresolved')
        elif current is None:
            if task.launch_state=='possible-launch':
                row['state']='unknown-reconcile'; blockers.append('possible launch has no current terminal receipt; reconcile original operation')
            elif history:
                row['state']='stale'; blockers.append('history retained; no current qualifying receipt')
            elif not files_current:
                row['state']='blocked-input'
            elif unmet:
                row['state']='blocked-dependency'
            else:
                row['selection_candidate']=True
            # A new explicit plan attempt can select work; the old receipt cannot complete it.
            if (history and files_current and not unmet and task.launch_state=='not-started'
                and all(h['attempt']!=task.attempt for h in history)):
                if any(h['fingerprint']==fp for h in history):
                    blockers.append('unchanged consumed operation requires changed inputs')
                else:
                    row['selection_candidate']=True
        elif current.submission=='unknown' or current.terminal=='unknown-submission':
            row['state']='unknown-reconcile'; blockers.append('submission outcome unknown; query original transaction before retry')
        elif current.cleanup!='complete':
            row['state']='cleanup-debt'; blockers.append('cleanup/restoration incomplete or unknown')
        elif current.terminal!='succeeded':
            row['state']=current.terminal; blockers.append('terminal result retained; no automatic retry')
        elif current.exit_code!=0 or current.signal is not None:
            row['state']='terminal-inconsistent'; blockers.append('success requires exit zero without signal')
        elif task.adapter in ('sherlock-anchor-settle-readback','sherlock-rollup-benchmark') and current.submission!='included':
            row['state']='predicate-failed'; blockers.append('target inclusion not recorded')
        else:
            try:
                observation, success=validate_artifacts(task,current,_reader=reads.read)
                row['observation']=observation
                if not success:
                    row['state']='predicate-failed'; blockers.append('typed public observation did not meet adapter predicate')
                else:
                    source=task.adapter=='source-integration'
                    row['state']='source-integrated' if source else 'observed'
                    row['source_integrated_recorded']=source
                    row['observation_recorded']=not source
                    accepted,reasons=_acceptance(plan,task,current,_reader=reads.read)
                    row['acceptance_recorded']=accepted; row['acceptance_blockers']=reasons
            except CleanupDebt as exc:
                row['state']='cleanup-debt'; blockers.append(str(exc))
            except (OSError,SchemaError) as exc:
                row['state']='artifact-invalid'; blockers.append(str(exc))
        rows[name]=row
    for task in plan.tasks:
        if rows[task.id]['selection_candidate']:
            ready.append(task.id)
    return apply_capabilities({'schema':'doctorwatson.delivery-report/1','records_valid':True,
            'execution_authority':False,'producer_authenticity':'not-established',
            'acceptance_basis':'exact inspected local review and Root adoption records are external project premises',
            'adapter_basis':'typed public adapter outputs; settle source-result uses the integrated fixed decoder, other native producers and the execution bridge remain separate',
            'ready_for_root_selection':ready,'tasks':[rows[t.id] for t in plan.tasks],
            'orphaned_history':[{'task':r.task,'attempt':r.attempt,'fingerprint':r.fingerprint,'terminal':r.terminal,
                                 'artifacts':[a.record() for a in r.artifacts]}
                                for r in results if r.task not in by_id]})


def apply_capabilities(report):
    """Refresh dispatch facts after the final qualified readiness calculation.

    The legacy candidate field remains a dependency/input/history-qualified
    candidate. Availability alone never promotes a consumed or blocked task.
    """
    dispatch = []
    for row in report['tasks']:
        cap = adapter_capability(row['adapter'])
        dependency_ready = row['selection_candidate']
        blockers = [] if dependency_ready else (
            list(row['blockers']) or ['task is not an unconsumed dependency-ready candidate'])
        if cap['blocked_reason'] is not None:
            blockers.append(cap['blocked_reason'])
        row.update(dependency_ready=dependency_ready,
                   collector_available=cap['collector_available'],
                   executor_available=cap['executor_available'],
                   ready_for_dispatch=(dependency_ready and cap['collector_available']
                                       and cap['executor_available']),
                   dispatch_blockers=blockers)
        if row['ready_for_dispatch']:
            dispatch.append(row['id'])
    report['ready_for_dispatch'] = dispatch
    report['dispatch_preflight'] = 'not-observed'
    report['dispatch_readiness_scope'] = 'dependencies-and-adapter-capabilities'
    return report


def inspect_delivery(plan_path, history_path):
    return assess(load_plan(plan_path),load_history(history_path))


def main(argv=None):
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan',required=True)
    parser.add_argument('--results',required=True)
    parser.add_argument('--next',action='store_true',help='emit only conditional ready task IDs and evidence limits')
    args=parser.parse_args(argv)
    try:
        report=inspect_delivery(args.plan,args.results)
    except (OSError,SchemaError) as exc:
        print(json.dumps({'schema':'doctorwatson.delivery-error/1','records_valid':False,
                          'execution_authority':False,'error':str(exc)}),file=sys.stderr)
        return 2
    if args.next:
        report={key:report[key] for key in ('schema','records_valid','execution_authority',
                  'producer_authenticity','ready_for_root_selection','ready_for_dispatch',
                  'dispatch_preflight','dispatch_readiness_scope')}
    print(json.dumps(report,sort_keys=True,indent=2,allow_nan=False))
    # Exit zero means inspectable records, never successful tasks or execution approval.
    return 0


if __name__=='__main__':
    raise SystemExit(main())
