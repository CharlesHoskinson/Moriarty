"""Bounded local cycle finalization; default inspect, explicit --apply.

The existing graph worker supplies an immutable refresh receipt. This helper checks
its exact source coverage and graph output bytes, appends one wiki-llm entry, then
retires only explicitly selected inactive merged local refs/worktrees. It does not
fetch PRs, authenticate supplied inventory, refresh graphs, delete remote refs,
prune objects or run caller-provided commands. Active-owner inventory must be
supplied fresh; freshness does not establish completeness or semantic correctness.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time

MAX_AGE = 900
MAX_JSON_BYTES = 4 * 1024 * 1024
SHA = re.compile(r'[0-9a-f]{64}\Z')
OID = re.compile(r'(?:[0-9a-f]{40}|[0-9a-f]{64})\Z')
NAME = re.compile(r'[A-Za-z0-9][A-Za-z0-9._:-]{0,127}\Z')


class CleanupError(ValueError):
    pass


def _canonical(value):
    return (json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False) + '\n').encode()


def _sha(raw):
    return hashlib.sha256(raw).hexdigest()


def _read_json(path, raw=None):
    raw = Path(path).read_bytes() if raw is None else raw
    if len(raw) > MAX_JSON_BYTES:
        raise CleanupError('JSON size limit exceeded')
    def pairs(items):
        result = {}
        for key, value in items:
            if key in result:
                raise CleanupError('duplicate JSON member: ' + key)
            result[key] = value
        return result
    value = json.loads(raw, object_pairs_hook=pairs)
    if not isinstance(value, dict):
        raise CleanupError('JSON object required')
    return value


def _path(repo, relative):
    if not isinstance(relative, str) or not relative or any(ord(c) < 32 for c in relative):
        raise CleanupError('invalid relative path')
    path = Path(relative)
    if path.is_absolute() or path.as_posix() != relative or any(p in ('.', '..') for p in path.parts):
        raise CleanupError('path must be normalized repository relative: ' + relative)
    target = repo / path
    for parent in [target, *target.parents]:
        if parent == repo:
            break
        if parent.is_symlink():
            raise CleanupError('symlink path refused: ' + relative)
    if not target.is_relative_to(repo) or '.git' in path.parts:
        raise CleanupError('protected repository path: ' + relative)
    return target


def _git(repo, *args, data=None):
    proc = subprocess.run(['git', '-C', str(repo), *args], input=data,
                          text=True, capture_output=True)
    if proc.returncode:
        raise CleanupError(json.dumps({'command': ['git', *args], 'exit_code': proc.returncode,
                                      'stdout': proc.stdout, 'stderr': proc.stderr}, sort_keys=True))
    return proc.stdout.strip()


def _tip(repo, ref):
    return _git(repo, 'rev-parse', '--verify', ref + '^{commit}')


def _branch(repo, value):
    if not isinstance(value, str) or value.startswith('-'):
        raise CleanupError('invalid branch')
    name = value.removeprefix('refs/heads/')
    if value.startswith('refs/') and not value.startswith('refs/heads/'):
        raise CleanupError('only local branch refs supported')
    _git(repo, 'check-ref-format', 'refs/heads/' + name)
    return name


def _root_ref(repo, value):
    if not isinstance(value, str) or not value.startswith(('refs/heads/', 'refs/remotes/')):
        raise CleanupError('retained root must be a full local or remote-tracking ref')
    _git(repo, 'check-ref-format', value)
    return value


def _root_branch(repo, value):
    value = _root_ref(repo, value)
    if value.startswith('refs/heads/'):
        return value.removeprefix('refs/heads/')
    remote = value.removeprefix('refs/remotes/')
    if '/' not in remote:
        raise CleanupError('remote-tracking root must identify remote and branch')
    return remote.split('/', 1)[1]


def _worktrees(repo):
    records = []
    for block in _git(repo, 'worktree', 'list', '--porcelain').split('\n\n'):
        row = {}
        for line in block.splitlines():
            key, _, value = line.partition(' ')
            row[key] = value
        if 'worktree' in row:
            records.append(row)
    return records


def _save_report(path, report):
    """Atomic replacement: a failed final save never persists 'complete'."""
    data = _canonical(report)
    fd, temporary = tempfile.mkstemp(prefix='.doctorwatson-report-', dir=path.parent)
    try:
        with os.fdopen(fd, 'wb') as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, path)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def _validate(repo, request, report_path):
    if request.get('schema') != 'doctorwatson.cleanup-request/1':
        raise CleanupError('unsupported cleanup request schema')
    if not NAME.fullmatch(request.get('cycle_id', '')):
        raise CleanupError('invalid cycle_id')
    if Path(_git(repo, 'rev-parse', '--show-toplevel')).resolve() != repo:
        raise CleanupError('repo must be the worktree root')
    if report_path.is_symlink() or report_path.exists() and not report_path.is_file():
        raise CleanupError('report must be a regular file')
    if not report_path.parent.is_dir():
        raise CleanupError('report parent must exist')
    if not report_path.is_relative_to(repo):
        raise CleanupError('report must be inside repository')
    _path(repo, report_path.relative_to(repo).as_posix())
    wiki = _path(repo, request['wiki_path'])
    if not wiki.is_relative_to(repo / 'wiki-llm'):
        raise CleanupError('wiki path must be under wiki-llm')
    graph = _path(repo, request['graph_receipt']['path'])
    sources = request['affected_sources']
    if not isinstance(sources, list) or not sources or len(sources) > 4096:
        raise CleanupError('affected_sources must be a nonempty bounded list')
    protected = {wiki, graph, report_path}
    seen = set()
    for source in sources:
        path = _path(repo, source['path'])
        if source['path'] in seen or not SHA.fullmatch(source['sha256']):
            raise CleanupError('invalid or duplicate source pin')
        seen.add(source['path'])
        if path in protected:
            raise CleanupError('source overlaps output path')
        protected.add(path)
    if len({wiki, graph, report_path}) != 3:
        raise CleanupError('output paths overlap')
    # Inspect every graph output before creating/replacing the report: an output
    # receipt must never overwrite the graph it is supposed to verify.
    receipt = _read_json(graph)
    for artifact in receipt.get('artifacts', []):
        path = _path(repo, artifact['path'])
        if path in protected:
            raise CleanupError('graph artifact overlaps protected cycle path')
        protected.add(path)
    # A hardlinked wiki could otherwise append to source or graph data.
    if wiki.exists():
        wiki_stat = wiki.stat()
        for path in protected - {wiki, report_path}:
            if path.exists():
                st = path.stat()
                if (st.st_dev, st.st_ino) == (wiki_stat.st_dev, wiki_stat.st_ino):
                    raise CleanupError('wiki aliases protected source or graph bytes')
    for key in ('candidates', 'active_branches', 'active_worktrees'):
        if not isinstance(request[key], list) or len(request[key]) > 4096:
            raise CleanupError(key + ' must be a bounded list')
    roots = request['retained_roots']
    if not isinstance(roots, list) or not 1 <= len(roots) <= 4096:
        raise CleanupError('retained roots required')
    for root in roots:
        _root_ref(repo, root['ref'])
        if not OID.fullmatch(root['tip']):
            raise CleanupError('root must bind full retained ref and commit')
    names = set()
    for candidate in request['candidates']:
        branch = _branch(repo, candidate['branch'])
        if branch in names or not OID.fullmatch(candidate['tip']):
            raise CleanupError('invalid or duplicate candidate')
        names.add(branch)
        wt = candidate.get('worktree')
        if wt is not None:
            path = Path(wt)
            if not path.is_absolute() or str(path.resolve()) != wt or not OID.fullmatch(candidate.get('head', '')):
                raise CleanupError('worktree must have canonical absolute path and exact HEAD')
            if path == repo or any(p.is_relative_to(path) or path.is_relative_to(p) for p in protected):
                raise CleanupError('worktree overlaps protected cycle paths')
    for key in ('outcome',):
        if not isinstance(request[key], str) or not request[key] or len(request[key]) > 16384:
            raise CleanupError('invalid cycle outcome')
    if not isinstance(request['lessons'], list) or any(not isinstance(x, str) or len(x) > 16384 for x in request['lessons']):
        raise CleanupError('invalid lessons')
    # Render user prose as JSON, preventing marker injection into the append log.
    return wiki, graph


def _graph(repo, request, receipt_path):
    raw = receipt_path.read_bytes()
    if not SHA.fullmatch(request['graph_receipt']['sha256']) or _sha(raw) != request['graph_receipt']['sha256']:
        raise CleanupError('graph refresh receipt digest mismatch')
    receipt = _read_json(receipt_path, raw=raw)
    if (receipt.get('schema') != 'doctorwatson.graph-refresh/1' or
            receipt.get('cycle_id') != request['cycle_id'] or
            receipt.get('affected_sources') != request['affected_sources'] or
            sorted(receipt.get('coverage', [])) != sorted(s['path'] for s in request['affected_sources'])):
        raise CleanupError('graph receipt cycle, exact source pins or coverage mismatch')
    for source in request['affected_sources']:
        if _sha(_path(repo, source['path']).read_bytes()) != source['sha256']:
            raise CleanupError('affected source digest changed: ' + source['path'])
    artifacts = receipt.get('artifacts')
    if not isinstance(artifacts, list) or not artifacts:
        raise CleanupError('graph receipt requires actual graph output artifacts')
    paths = set()
    excluded = {request['wiki_path'], request['graph_receipt']['path'], *(s['path'] for s in request['affected_sources'])}
    for artifact in artifacts:
        path = _path(repo, artifact['path'])
        if artifact['path'] in excluded or artifact['path'] in paths or not SHA.fullmatch(artifact['sha256']):
            raise CleanupError('invalid graph artifact binding')
        paths.add(artifact['path'])
        if _sha(path.read_bytes()) != artifact['sha256']:
            raise CleanupError('graph output digest changed: ' + artifact['path'])
    return {'receipt_sha256': _sha(raw), 'affected_sources': request['affected_sources'], 'artifacts': artifacts}


def _wiki(path, request, graph, apply):
    # Existing prefix is retained byte-for-byte; exclusive per-file lock coordinates
    # helper invocations and the O_NOFOLLOW open refuses replaced leaf symlinks.
    import fcntl
    payload = {'cycle_id': request['cycle_id'], 'outcome': request['outcome'],
               'lessons': request['lessons'], 'graph_receipt_sha256': graph['receipt_sha256']}
    marker = '<!-- doctorwatson-cycle:' + request['cycle_id'] + ':'
    entry = (marker + _sha(_canonical(payload)) + ' -->\n' +
             _canonical(payload).decode().replace('<', '\\u003c') + '<!-- /doctorwatson-cycle -->\n').encode()
    flags = os.O_RDWR if apply else os.O_RDONLY
    fd = os.open(path, flags | os.O_NOFOLLOW)
    try:
        with os.fdopen(fd, 'r+b' if apply else 'rb') as stream:
            fcntl.flock(stream, fcntl.LOCK_EX if apply else fcntl.LOCK_SH)
            before = stream.read()
            if marker.encode() in before:
                if before.count(marker.encode()) != 1 or entry not in before:
                    raise CleanupError('cycle already logged with different or partial content')
                return {'status': 'already-appended', 'entry_sha256': _sha(entry)}
            if apply:
                stream.seek(0, os.SEEK_END)
                stream.write(b'\n' + entry)
                stream.flush()
                os.fsync(stream.fileno())
            return {'status': 'appended' if apply else 'planned', 'entry_sha256': _sha(entry)}
    except BaseException:
        # fdopen owns the descriptor after construction.
        raise


def _inventory(repo, request, now):
    inventory = request['inventory']
    captured = inventory.get('captured_at')
    if type(captured) not in (int, float) or not math.isfinite(captured) or not 0 <= now - captured <= MAX_AGE:
        raise CleanupError('inventory stale or future-dated; maximum age 900 seconds')
    common = Path(_git(repo, 'rev-parse', '--path-format=absolute', '--git-common-dir')).resolve()
    if inventory.get('repo_common_dir') != str(common) or inventory.get('head') != _tip(repo, 'HEAD'):
        raise CleanupError('inventory repository identity or current HEAD changed')
    if inventory.get('open_prs_complete') is not True or inventory.get('leases_known') is not True:
        raise CleanupError('open PR or active lease inventory unknown')
    if not isinstance(inventory.get('open_prs'), list) or not isinstance(inventory.get('lease_worktrees'), list):
        raise CleanupError('missing explicit PR/lease inventory')
    for root in request['retained_roots']:
        if _tip(repo, root['ref']) != root['tip']:
            raise CleanupError('retained root moved: ' + root['ref'])
    protected = {'main', 'master', 'sherlock'}
    current = _git(repo, 'symbolic-ref', '--quiet', '--short', 'HEAD')
    protected.add(current)
    protected.update(_root_branch(repo, r['ref']) for r in request['retained_roots'])
    protected.update(_branch(repo, b) for b in request['active_branches'])
    for pr in inventory['open_prs']:
        protected.update((_branch(repo, pr['head']), _branch(repo, pr['base'])))
    # Remote symbolic default, when available, is an additional local guard.
    remote_default = subprocess.run(['git', '-C', str(repo), 'symbolic-ref', '--quiet', 'refs/remotes/origin/HEAD'], text=True, capture_output=True)
    if remote_default.returncode == 0:
        protected.add(remote_default.stdout.strip().removeprefix('refs/remotes/origin/'))
    active = set(request['active_worktrees']) | set(inventory['lease_worktrees'])
    return protected, active


def _disposition(repo, request, candidate, now, prior_deleted=False):
    protected, active = _inventory(repo, request, now)
    branch = _branch(repo, candidate['branch'])
    row = {'branch': branch, 'tip': candidate['tip'], 'worktree': candidate.get('worktree'),
           'owner': candidate.get('owner') or 'unknown', 'status': 'retained', 'reason': 'unknown',
           'next_action': 'refresh inventory and inspect retained work', 'revisit_event': 'owner or source state changes',
           'worktree_removed': False, 'branch_deleted': False, 'branch_config': 'retained; no config pruning performed'}
    def stop(reason, blocked=False):
        row.update(reason=reason, status='blocked' if blocked else 'retained')
        return row
    # A missing/expired transport lease is not proof of inactivity. The owner
    # must explicitly attest the selected candidate inactive, independently of
    # MQTT presence, broker state, DID identity or lease expiration.
    if candidate.get('inactive') is not True or not isinstance(candidate.get('owner'), str) or not candidate['owner'].strip() or candidate['owner'] == 'unknown':
        return stop('candidate inactivity unknown; explicit owner attestation required', True)
    if branch in protected:
        return stop('current/default/main/integration, explicit active or open-PR head/base protected')
    try:
        tip = _tip(repo, 'refs/heads/' + branch)
    except CleanupError:
        absent = subprocess.run(['git', '-C', str(repo), 'show-ref', '--verify', '--quiet',
                                 'refs/heads/' + branch]).returncode == 1
        wt = candidate.get('worktree')
        still_registered = any(w.get('branch') == 'refs/heads/' + branch or wt and w['worktree'] == wt
                               for w in _worktrees(repo))
        if prior_deleted and absent and not still_registered and (not wt or not Path(wt).exists()):
            row.update(reason='already retired in exact prior apply receipt; branch/worktree still absent',
                       previously_deleted=True, next_action='none', revisit_event='branch or worktree recreated')
            return row
        return stop('branch missing or unknown; refresh disposition', True)
    if tip != candidate['tip']:
        return stop('candidate branch tip moved', True)
    merged = False
    for root in request['retained_roots']:
        p = subprocess.run(['git', '-C', str(repo), 'merge-base', '--is-ancestor', tip, root['tip']], capture_output=True, text=True)
        if p.returncode not in (0, 1):
            raise CleanupError(p.stderr)
        merged |= p.returncode == 0
    if not merged:
        return stop('candidate is not reachable from any retained root; unique or unmerged work')
    checked = [w for w in _worktrees(repo) if w.get('branch') == 'refs/heads/' + branch]
    if len(checked) > 1:
        return stop('multiple worktrees or unknown branch use', True)
    wt = candidate.get('worktree')
    if checked and (not wt or checked[0]['worktree'] != wt):
        return stop('checked-out worktree not explicitly selected', True)
    if wt:
        if wt in active:
            return stop('explicit active worktree or active lease')
        if not checked or checked[0].get('HEAD') != candidate['head'] or candidate['head'] != tip:
            return stop('worktree registration or exact HEAD changed', True)
        if 'locked' in checked[0] or 'prunable' in checked[0]:
            return stop('locked or unavailable worktree')
        if _tip(Path(wt), 'HEAD') != candidate['head']:
            return stop('worktree HEAD changed', True)
        if _git(Path(wt), 'status', '--porcelain=v1', '--untracked-files=all', '--ignored=matching'):
            return stop('worktree is not pristine (tracked, untracked or ignored files)')
        # Git status can hide nested .git metadata in tracked directories.
        if _embedded_repository(Path(wt)):
            return stop('embedded repository metadata retained for explicit owner disposition')
        if _git(Path(wt), 'submodule', 'status', '--recursive'):
            return stop('submodule worktree retained for explicit owner disposition')
    row.update(status='eligible', reason='exact inactive tip reachable from retained history; selected worktree pristine',
               next_action='apply bounded local retirement', revisit_event='before apply revalidate roots, tips, PRs and leases')
    return row


def _embedded_repository(worktree):
    """Inspect names/metadata only; never read nested repository payloads."""
    def inaccessible(error):
        raise CleanupError('worktree metadata traversal failed: ' + str(error))
    root = Path(worktree)
    for directory, dirs, files in os.walk(root, followlinks=False, onerror=inaccessible):
        path = Path(directory)
        if path == root:
            # A linked worktree has one root .git file. A directory or symlink
            # here is unexpected user repository metadata and must be retained.
            sentinel = root / '.git'
            if '.git' in dirs or sentinel.is_symlink():
                return True
        elif '.git' in dirs or '.git' in files:
            return True
    return False


def _delete_ref(repo, request, candidate):
    # Compare-and-delete and verify retained roots in one Git ref transaction.
    # Unlike branch -d this binds the expected tip and does not depend on the
    # branch's configured upstream; ancestry has already been checked separately.
    if any(w.get('branch') == 'refs/heads/' + _branch(repo, candidate['branch']) for w in _worktrees(repo)):
        raise CleanupError('branch remains checked out; deletion refused')
    lines = ['start'] + ['verify ' + r['ref'] + ' ' + r['tip'] for r in request['retained_roots']]
    lines += ['delete refs/heads/' + _branch(repo, candidate['branch']) + ' ' + candidate['tip'], 'prepare', 'commit', '']
    _git(repo, 'update-ref', '--stdin', data='\n'.join(lines))


def _finalize_locked(repo, request, report_path, *, apply=False, now=None):
    repo = Path(repo).resolve()
    report_path = Path(report_path).absolute()
    report = {'schema': 'doctorwatson.cleanup-report/1', 'cycle_id': request.get('cycle_id', 'unknown'),
              'mode': 'apply' if apply else 'inspect', 'status': 'incomplete', 'phases': [], 'dispositions': [],
              'retirement_receipts': [],
              'inventory_trust': 'caller supplied; API completeness and owner truth not independently verified',
              'remote_refs': 'retained; unsupported', 'object_gc': 'not performed'}
    clock = (lambda: time.time()) if now is None else (lambda: now)
    try:
        wiki, receipt_path = _validate(repo, request, report_path)
    except (CleanupError, OSError, ValueError, KeyError, TypeError) as exc:
        # Unsafe output paths are not written. The caller receives an incomplete
        # report and the CLI returns failure; no phase has mutated anything.
        report['error'] = str(exc)
        return report
    report['request_sha256'] = _sha(_canonical(request))
    # Live inventory changes do not change the cleanup operation's identity.
    # They are still checked afresh before every disposition/destructive action.
    operation = {key: value for key, value in request.items() if key != 'inventory'}
    operation['repository_common_dir'] = str(Path(_git(repo, 'rev-parse', '--path-format=absolute', '--git-common-dir')).resolve())
    report['operation_sha256'] = _sha(_canonical(operation))
    prior_deleted = set()
    if report_path.exists():
        try:
            prior = _read_json(report_path)
            if prior.get('schema') == report['schema']:
                receipts = prior.get('retirement_receipts', [])
                if not isinstance(receipts, list) or len(receipts) > 4096:
                    raise CleanupError('invalid prior retirement receipts')
                # Retain exact historical apply receipts even in inspect or a
                # later incomplete report. Only a current matching operation can
                # consume them; no planned disposition creates a receipt.
                report['retirement_receipts'] = receipts
                prior_deleted = {(r['branch'], r['tip'], r.get('worktree')) for r in receipts
                                 if r.get('schema') == 'doctorwatson.local-retirement/1'
                                 and r.get('operation_sha256') == report['operation_sha256']
                                 and r.get('branch_deleted') is True}
        except (OSError, ValueError, KeyError, TypeError):
            pass  # Unreadable prior reports provide no retirement evidence.
    _save_report(report_path, report)  # Receipt must be writable before mutation.
    phase = 'graph'
    try:
        graph = _graph(repo, request, receipt_path)
        report['phases'].append({'name': 'graph', 'status': 'verified', **graph})
        _save_report(report_path, report)
        phase = 'wiki'
        wiki_result = _wiki(wiki, request, graph, apply)
        report['phases'].append({'name': 'wiki', **wiki_result})
        _save_report(report_path, report)
        phase = 'retirement'
        _inventory(repo, request, clock())
        _graph(repo, request, receipt_path)
        for candidate in request['candidates']:
            row = _disposition(repo, request, candidate, clock(),
                               (_branch(repo, candidate['branch']), candidate['tip'], candidate.get('worktree')) in prior_deleted)
            report['dispositions'].append(row)
            _save_report(report_path, report)
            if apply and row['status'] == 'eligible':
                try:
                    # Every destructive action rechecks the exact inventory and
                    # fresh local disposition, rather than trusting inspect.
                    _graph(repo, request, receipt_path)
                    fresh = _disposition(repo, request, candidate, clock())
                    if fresh['status'] != 'eligible':
                        row.update(fresh)
                        _save_report(report_path, report)
                        continue
                    if candidate.get('worktree'):
                        _git(repo, 'worktree', 'remove', candidate['worktree'])
                        row['worktree_removed'] = True
                        _save_report(report_path, report)
                    _inventory(repo, request, clock())
                    _graph(repo, request, receipt_path)
                    _delete_ref(repo, request, candidate)
                    row.update(status='deleted', branch_deleted=True, reason='redundant local ref retired with expected-tip/root transaction',
                               next_action='none', revisit_event='new branch or recovery request')
                    report['retirement_receipts'].append({
                        'schema': 'doctorwatson.local-retirement/1',
                        'operation_sha256': report['operation_sha256'],
                        'cycle_id': request['cycle_id'], 'branch': row['branch'],
                        'tip': candidate['tip'], 'worktree': candidate.get('worktree'),
                        'retained_roots': request['retained_roots'],
                        'worktree_removed': row['worktree_removed'], 'branch_deleted': True})
                except CleanupError as exc:
                    row.update(status='blocked', reason=str(exc), next_action='inspect exact failure and refresh disposition',
                               revisit_event='failure cause resolved; exact state re-inspected')
                _save_report(report_path, report)
        _graph(repo, request, receipt_path)
        _wiki(wiki, request, graph, False)
        blocked = any(row['status'] == 'blocked' for row in report['dispositions'])
        report['phases'].append({'name': 'retirement', 'status': 'blocked' if blocked else 'complete' if apply else 'planned'})
        report['status'] = 'incomplete' if blocked else 'complete' if apply else 'planned'
        report['phases'].append({'name': 'final', 'status': report['status']})
        _save_report(report_path, report)
        return report
    except (CleanupError, ValueError, KeyError, TypeError) as exc:
        report['status'] = 'incomplete'
        report['phases'].append({'name': phase, 'status': 'blocked', 'error': str(exc)})
        report['error'] = str(exc)
        _save_report(report_path, report)
        return report
    except OSError as exc:
        # A report failure must propagate, never return a successful result. File
        # errors in graph/wiki still get an incomplete receipt when writable.
        report['status'] = 'incomplete'
        report['error'] = str(exc)
        report['phases'].append({'name': phase, 'status': 'blocked', 'error': str(exc)})
        _save_report(report_path, report)
        return report


def finalize(repo, request, report_path, *, apply=False, now=None):
    """Apply requires a common-dir advisory lock shared by cleanup writers.

    Arbitrary Git/worktree editors do not honor this lock. The caller must stop
    other writers for an apply window; exact ref CAS does not lock user files or
    atomically prevent a concurrent checkout. Inspect needs no mutation lock.
    """
    if not apply:
        return _finalize_locked(repo, request, report_path, apply=False, now=now)
    import fcntl
    import stat
    fd = None
    try:
        common = Path(_git(Path(repo), 'rev-parse', '--path-format=absolute', '--git-common-dir')).resolve()
        fd = os.open(common / 'doctorwatson-cleanup.lock', os.O_CREAT | os.O_RDWR | os.O_NOFOLLOW, 0o600)
        if not stat.S_ISREG(os.fstat(fd).st_mode):
            raise CleanupError('cleanup lock is not a regular file')
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except (OSError, CleanupError) as exc:
        if fd is not None:
            os.close(fd)
        return {'schema': 'doctorwatson.cleanup-report/1', 'cycle_id': request.get('cycle_id', 'unknown'),
                'mode': 'apply', 'status': 'incomplete', 'phases': [], 'dispositions': [],
                'error': 'cleanup lock unavailable: ' + str(exc)}
    try:
        return _finalize_locked(repo, request, report_path, apply=True, now=now)
    finally:
        os.close(fd)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', required=True)
    parser.add_argument('--request', required=True)
    parser.add_argument('--report', required=True)
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args(argv)
    try:
        result = finalize(args.repo, _read_json(args.request), args.report, apply=args.apply)
    except (OSError, CleanupError, ValueError, TypeError, KeyError) as exc:
        print(json.dumps({'schema': 'doctorwatson.cleanup-report/1', 'status': 'incomplete', 'error': str(exc)}))
        return 1
    print(json.dumps(result, sort_keys=True))
    return 1 if result['status'] == 'incomplete' else 0


if __name__ == '__main__':
    sys.exit(main())
