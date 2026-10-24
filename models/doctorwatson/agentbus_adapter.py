"""Optional, explicitly called AgentBus observations for DoctorWatson.

No AgentBus dependency, credential discovery, file access or network operation
occurs at import. The caller supplies canonical context and an already verified
transport envelope. A notice can only suggest inspecting canonical state: plan,
history, reviews and Git remain authoritative. Broker acknowledgement is not
receiver persistence, completion or execution permission.
"""
from __future__ import annotations

from datetime import datetime
import hashlib
import json
import math
from pathlib import PurePosixPath
import re

SCHEMA = 'doctorwatson.agentbus.advisory/1'
EVENTS = frozenset(('handoff', 'source-ready', 'review-ready', 'blocked', 'claim-lost'))
CONTEXT = ('run_id', 'task_id', 'attempt_id', 'task_fingerprint', 'integration_base', 'sender_did')
FIELDS = frozenset(('schema', 'event_id', 'event', 'refs', *CONTEXT))
MAX_BODY = 16 * 1024
MAX_FILES = 128
IDENTIFIER = re.compile(r'[A-Za-z0-9][A-Za-z0-9._:-]{0,127}\Z')
HEX = re.compile(r'[0-9a-f]{64}\Z')
DID = re.compile(r'did:[a-z0-9]+:[A-Za-z0-9._:%-]+\Z')


class AdapterError(ValueError):
    """Invalid adapter input; this never represents execution failure."""


def _text(value, maximum=256):
    if (type(value) is not str or not 1 <= len(value) <= maximum or
            any(ord(c) < 32 or ord(c) == 127 or 0xD800 <= ord(c) <= 0xDFFF for c in value)):
        raise AdapterError('invalid bounded text')
    return value


def _name(value):
    if type(value) is not str or not IDENTIFIER.fullmatch(value):
        raise AdapterError('invalid identifier')
    return value


def _sha(value):
    if type(value) is not str or not HEX.fullmatch(value):
        raise AdapterError('invalid SHA-256')
    return value


def _canonical(value):
    try:
        return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False,
                          allow_nan=False).encode('utf-8')
    except (ValueError, TypeError, UnicodeError, RecursionError) as exc:
        raise AdapterError('invalid JSON value') from exc


def _path(value, *, absolute):
    _text(value, 4096)
    path = PurePosixPath(value)
    if (path.is_absolute() != absolute or str(path) != value or
            '..' in path.parts or value in ('.', '/') or '#' in value or '\\' in value):
        raise AdapterError('path must be normalized and exact')
    return value


def _context(value):
    if type(value) is not dict or set(value) != set(CONTEXT):
        raise AdapterError('expected exact canonical context fields')
    for key in ('run_id', 'task_id', 'attempt_id'):
        _name(value[key])
    _sha(value['task_fingerprint'])
    _text(value['integration_base'])
    if not DID.fullmatch(_text(value['sender_did'])):
        raise AdapterError('invalid participant DID')


def _refs(refs):
    if type(refs) not in (tuple, list) or len(refs) > 16:
        raise AdapterError('at most 16 public file references are supported')
    out = []
    for ref in refs:
        if type(ref) is not dict or set(ref) != {'path', 'bytes', 'sha256'}:
            raise AdapterError('invalid public file reference')
        _path(ref['path'], absolute=True)
        if type(ref['bytes']) is not int or not 0 <= ref['bytes'] <= 1024**3:
            raise AdapterError('invalid public file size')
        _sha(ref['sha256'])
        out.append(dict(ref))
    if len({ref['path'] for ref in out}) != len(out):
        raise AdapterError('duplicate public reference path')
    return out


def make_notice(*, run_id, event, task_id, attempt_id, task_fingerprint,
                integration_base, sender_did, refs=()):
    """Build a stable application event; references are assertions, never opened."""
    value = dict(run_id=run_id, task_id=task_id, attempt_id=attempt_id,
                 task_fingerprint=task_fingerprint, integration_base=integration_base,
                 sender_did=sender_did)
    _context(value)
    if type(event) is not str or event not in EVENTS:
        raise AdapterError('unknown advisory event')
    value.update(schema=SCHEMA, event=event, refs=_refs(refs))
    value['event_id'] = hashlib.sha256(_canonical(value)).hexdigest()
    if len(_canonical(value)) > MAX_BODY:
        raise AdapterError('advisory body too large')
    return value


def _notice(value):
    if type(value) is not dict or set(value) != FIELDS or value['schema'] != SCHEMA:
        raise AdapterError('invalid advisory schema')
    expected = make_notice(event=value['event'], refs=value['refs'],
                           **{key: value[key] for key in CONTEXT})
    _sha(value['event_id'])
    if value != expected:
        raise AdapterError('advisory event identity mismatch')
    return expected


def _pairs(pairs):
    value = {}
    for key, item in pairs:
        if key in value:
            raise AdapterError('duplicate JSON member')
        value[key] = item
    return value


def _constant(_):
    raise AdapterError('nonfinite JSON number')


def classify_notice(envelope, *, expected, seen_ids=()):
    """Classify an already transport-verified envelope; perform no state change.

    This function does not authenticate signatures or models. The caller must
    obtain envelopes from the verified transport and pin the expected context.
    """
    _context(expected)
    try:
        if type(envelope) is not dict or type(envelope.get('body')) is not str:
            raise AdapterError('missing advisory envelope body')
        raw = envelope['body'].encode('utf-8')
        if len(raw) > MAX_BODY:
            raise AdapterError('advisory body too large')
        value = _notice(json.loads(raw, object_pairs_hook=_pairs, parse_constant=_constant))
        if envelope.get('run') != value['run_id'] or envelope.get('from') != value['sender_did']:
            raise AdapterError('transport attribution mismatch')
    except (AdapterError, UnicodeError, json.JSONDecodeError, RecursionError, ValueError):
        return {'status': 'invalid'}
    result = {'event_id': value['event_id']}
    if any(value[key] != expected[key] for key in CONTEXT):
        return result | {'status': 'stale'}
    if value['event_id'] in seen_ids:
        return result | {'status': 'duplicate'}
    return result | {'status': 'advisory', 'action': 'inspect-canonical-state'}


def _failure_code(exc):
    # Duck typing preserves the optional dependency boundary. Never expose the
    # transport exception text, which can contain private paths or payloads.
    code = getattr(exc, 'code', None)
    if type(code) is int and code in (2, 3, 4, 5, 6):
        return code
    if isinstance(exc, OSError):
        return 5
    raise exc


def publish_notice(transport, to, notice):
    """Send once; a published receipt describes broker ACK only, never delivery."""
    value = _notice(notice)
    _text(to, 128)
    result = {'event_id': value['event_id']}
    if transport is None:
        return result | {'status': 'disabled'}
    if transport.run != value['run_id'] or transport.did != value['sender_did']:
        raise AdapterError('transport context differs from advisory context')
    try:
        message_id = transport.send(to, 'DoctorWatson ' + value['event'],
                                    body=_canonical(value).decode('utf-8'))
    except Exception as exc:
        return result | {'status': 'unavailable', 'error_code': _failure_code(exc)}
    return result | {'status': 'published', 'message_id': message_id}


def claim_owned_files(transport, *, owned_files, files, lease_min=30, wait=0):
    """Acquire exact owned files cooperatively; never release uncertain claims.

    Keep each returned grant's ``resource`` and original ``hold`` token for
    explicit Agent.renew(resource, hold=hold)/release(resource, hold=hold).
    This is not a write fence or the external proving lock.
    """
    for value in (owned_files, files):
        if type(value) not in (tuple, list) or len(value) > MAX_FILES:
            raise AdapterError('expected bounded file list')
        for path in value:
            _path(path, absolute=False)
        if len(set(value)) != len(value):
            raise AdapterError('duplicate owned/requested file')
    if not set(files) <= set(owned_files):
        raise AdapterError('requested file outside explicit ownership')
    for number, positive in ((lease_min, True), (wait, False)):
        if (type(number) not in (int, float) or not math.isfinite(number) or
                number < 0 or positive and number == 0):
            raise AdapterError('invalid finite claim timing')
    result = {'grants': []}
    if transport is None:
        return result | {'status': 'disabled'}
    for path in files:
        try:
            grant = transport.claim('file:' + path, lease_min=lease_min, wait=wait)
        except Exception as exc:
            return result | {'status': 'incomplete', 'error_code': _failure_code(exc),
                             'uncertain_file': path}
        # A renewed/already-held grant may have a new command request ID.
        # Never use that request ID as an acquisition token fallback.
        if (type(grant) is not dict or grant.get('kind') != 'grant' or
                type(grant.get('resource')) is not str or not grant['resource'].startswith('file:') or
                type(grant.get('hold')) is not str or not IDENTIFIER.fullmatch(grant['hold'])):
            return result | {'status': 'incomplete', 'reason': 'grant-token-unknown',
                             'uncertain_file': path}
        result['grants'].append({'file': path, 'resource': grant['resource'], 'hold': grant['hold']})
    return result | {'status': 'advisory'}


def _time(value):
    if type(value) in (int, float) and math.isfinite(value):
        return float(value)
    if type(value) is str:
        try:
            stamp = datetime.fromisoformat(value.replace('Z', '+00:00'))
            if stamp.tzinfo is not None:
                return stamp.timestamp()
        except (ValueError, OverflowError):
            pass
    raise AdapterError('invalid lease timestamp')


def cleanup_retention(snapshot, *, run_id, resource_worktrees, now, max_age=30):
    """Project positive retention only; bus data NEVER completes lease inventory.

    Input is a caller-captured observation of actual Agent.claims() records, not
    a complete-inventory API. Independent canonical providers must establish
    all relevant runs/providers before cleanup can set leases_known=True.
    """
    _name(run_id)
    if type(resource_worktrees) is not dict:
        raise AdapterError('expected explicit resource/worktree mapping')
    for resource, worktree in resource_worktrees.items():
        _text(resource, 4096)
        _path(worktree, absolute=True)
    clock = _time(now)
    if type(max_age) not in (int, float) or not math.isfinite(max_age) or max_age < 0:
        raise AdapterError('invalid observation age limit')
    result = {'leases_known': False, 'lease_worktrees': [], 'reason': 'bus-inventory-incomplete'}
    if type(snapshot) is not dict or snapshot.get('run_id') != run_id:
        return result
    try:
        age = clock - _time(snapshot.get('captured_at'))
        if not 0 <= age <= max_age:
            result['reason'] = 'stale-bus-observation'
    except AdapterError:
        result['reason'] = 'unknown-bus-observation-time'
    claims = snapshot.get('claims')
    if type(claims) is not list or len(claims) > 4096:
        return result
    protected = set()
    for claim in claims:
        if type(claim) is not dict:
            continue
        try:
            if not claim.get('holder_did') or _time(claim.get('lease_end')) <= clock:
                continue
            worktree = resource_worktrees.get(claim.get('resource'))
            if worktree is not None:
                protected.add(worktree)
        except (AdapterError, TypeError):
            continue
    result['lease_worktrees'] = sorted(protected)
    return result


__all__ = ['AdapterError', 'make_notice', 'classify_notice', 'publish_notice',
           'claim_owned_files', 'cleanup_retention']
