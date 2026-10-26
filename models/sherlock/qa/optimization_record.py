"""UIREQ-024 optimization proposal record and its preservation-predicate check.

A proposal that replaces a reference operation or representation must state a
preservation predicate over every declared observation and cost. The record
(format ``sherlock-optimization-preservation/1``) lists, per declared
observation plane, the predicate, the evidence ids that support it and the
unresolved gap ids, with evidence and gaps in two separate tables.

The check never trusts the record's words about runs: each compiled-and-run
evidence item is cross-checked against the factory run log
(``qa/sherlock/runs.jsonl``), the run trace on disk (pinned by sha256), git
history (the run commit is an ancestor of HEAD and the candidate, predicate and
test files are unchanged since) and the test source (a ``#[test] fn``). Nothing
here runs cargo, proves, verifies or grants acceptance.
"""
import hashlib
import json
from pathlib import Path
import re
import shlex
import subprocess

FORMAT = 'sherlock-optimization-preservation/1'
REPORT_FORMAT = 'sherlock-optimization-preservation-check/1'
CLAUSES = ('values', 'rejection/exhaustion', 'evaluation/effect order', 'boundaries',
           'duties', 'resource schedule')
EVIDENCE_KINDS = ('compiled-and-run', 'vacuous-by-profile')
COST_STATUS = ('measured', 'unknown')
DEFAULT_RECORD = 'models/sherlock/optimizations/intern-equal-tree-nodes.json'


class RecordError(Exception):
    """The record fails the preservation-predicate check (exit 1)."""


class Malformed(Exception):
    """The record is not a well-formed record of this format (exit 2)."""


def _need(cond, msg, exc=RecordError):
    if not cond:
        raise exc(msg)


class Repo:
    def __init__(self, root):
        self.root = Path(root)

    def git(self, *args):
        return subprocess.run(['git', '-C', str(self.root), *args], capture_output=True, text=True)

    def read(self, rel):
        p = self.root / rel
        return p.read_text() if p.is_file() else None

    def runs(self):
        out = []
        for line in (self.read('qa/sherlock/runs.jsonl') or '').splitlines():
            try:
                out.append(json.loads(line))
            except ValueError:
                pass
        return out


def _safe_rel(rel):
    return (isinstance(rel, str) and rel and not rel.startswith('/')
            and '..' not in rel.split('/'))


def _is_test_fn(text, fn):
    return re.search(r'#\[test\][^\n]*\n(?:\s*#\[[^\n]*\n)*\s*fn\s+' + re.escape(fn) + r'\s*\(',
                     text) is not None


def verify_run_evidence(ev, repo, pinned_files):
    """Cross-check one compiled-and-run evidence item; return its verified summary."""
    bad = f"evidence {ev.get('id')!r}"
    sha = ev.get('commit')
    _need(isinstance(sha, str) and re.fullmatch(r'[0-9a-f]{40}', sha), f'{bad}: commit {sha!r}')
    _need(repo.git('merge-base', '--is-ancestor', sha, 'HEAD').returncode == 0,
          f'{bad}: commit {sha[:12]} is not an ancestor of HEAD')
    cmd = ev.get('command') or ''
    argv = shlex.split(cmd)
    _need(argv[:2] == ['cargo', 'test'] and '--no-run' not in argv,
          f'{bad}: command {cmd!r} is not a cargo test run')
    recs = [r for r in repo.runs() if r.get('run') == ev.get('run')]
    _need(len(recs) == 1, f"{bad}: run {ev.get('run')!r} is not in qa/sherlock/runs.jsonl")
    rec = recs[0]
    _need(rec.get('exit') == 0, f"{bad}: run exited {rec.get('exit')}")
    _need(rec.get('head') == sha, f'{bad}: run head differs from the evidence commit')
    _need(rec.get('cmd') == cmd, f'{bad}: run log command differs from the evidence command')
    _need(rec.get('trace') == ev.get('trace'), f'{bad}: trace path differs from the run log')
    trace = repo.read(ev.get('trace') or '')
    _need(trace is not None, f"{bad}: trace {ev.get('trace')} is not on disk (unverifiable)")
    _need(hashlib.sha256(trace.encode()).hexdigest() == ev.get('trace_sha256'),
          f'{bad}: trace sha256 differs')
    results = re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed', trace, re.M)
    _need(results and all(s == 'ok' and f == '0' for s, _, f in results),
          f'{bad}: the run had failures or no cargo test result')
    tf = ev.get('file')
    text = repo.read(tf) if _safe_rel(tf) else None
    _need(text is not None, f'{bad}: test file {tf!r} does not exist')
    tests = ev.get('tests') or []
    _need(tests, f'{bad}: names no test')
    for name in tests:
        _need(re.search(r'^test ' + re.escape(name) + r' \.\.\. ok$', trace, re.M),
              f'{bad}: test {name} did not pass in the trace')
        _need(_is_test_fn(text, name.split('::')[-1]), f'{bad}: {tf} has no #[test] fn {name}')
    files = sorted(set(pinned_files) | {tf})
    _need(repo.git('diff', '--quiet', sha, '--', *files).returncode == 0,
          f'{bad}: candidate, predicate or test files changed since {sha[:12]}')
    return dict(id=ev['id'], run=rec['run'], commit=sha, tests=len(tests), status='verified')


def check(record, repo):
    """Return the check report; raise RecordError / Malformed."""
    _need(isinstance(record, dict) and record.get('format') == FORMAT,
          f'format is not {FORMAT}', Malformed)
    for key in ('candidate', 'reference', 'preservation_predicate', 'clauses', 'evidence',
                'unresolved_gaps', 'costs'):
        _need(key in record, f'missing {key}', Malformed)
    cand = record['candidate']
    _need(cand.get('kind') in ('frame-storage', 'node-sharing'),
          'candidate kind must be frame-storage or node-sharing (UIREQ-024 scenario)')
    _need(cand.get('replaces') and cand.get('by'), 'candidate must state what it replaces and by what')
    pinned = []
    for rel in [cand.get('module'), record['preservation_predicate'].get('file')]:
        _need(_safe_rel(rel) and repo.read(rel) is not None, f'named file {rel!r} does not exist')
        pinned.append(rel)
    pred = record['preservation_predicate']
    _need(f"pub fn {pred.get('entry')}" in (repo.read(pred['file']) or ''),
          f"predicate entry {pred.get('entry')!r} not found")
    evidence = {e.get('id'): e for e in record['evidence']}
    gaps = {g.get('id'): g for g in record['unresolved_gaps']}
    _need(len(evidence) == len(record['evidence']) and None not in evidence, 'duplicate/missing evidence id')
    _need(len(gaps) == len(record['unresolved_gaps']) and None not in gaps, 'duplicate/missing gap id')
    _need(not set(evidence) & set(gaps), 'evidence and gaps must be listed separately (shared ids)')
    seen = [c.get('observation') for c in record['clauses']]
    missing = [c for c in CLAUSES if c not in seen]
    _need(not missing, f'record does not cover {missing}')
    _need(len(seen) == len(set(seen)) and set(seen) == set(CLAUSES),
          f'clauses must be exactly {list(CLAUSES)}')
    clauses = []
    for c in record['clauses']:
        name = c['observation']
        _need(isinstance(c.get('predicate'), str) and c['predicate'].strip(), f'{name}: no predicate')
        ev_ids, gap_ids = c.get('evidence') or [], c.get('gaps') or []
        _need(ev_ids or gap_ids, f'{name}: neither evidence nor gap')
        for i in ev_ids:
            _need(i in evidence, f'{name}: evidence {i!r} is not in the evidence table')
        for i in gap_ids:
            _need(i in gaps, f'{name}: gap {i!r} is not in the gap table')
        clauses.append(dict(observation=name, predicate=c['predicate'], evidence=ev_ids, gaps=gap_ids))
    verified = []
    for e in record['evidence']:
        kind = e.get('kind')
        _need(kind in EVIDENCE_KINDS, f"evidence {e['id']}: kind {kind!r}")
        if kind == 'compiled-and-run':
            verified.append(verify_run_evidence(e, repo, pinned))
        else:
            _need(e.get('profile') == record['reference'].get('profile') and e.get('reason'),
                  f"evidence {e['id']}: vacuous evidence must name the reference profile and reason")
            verified.append(dict(id=e['id'], status='vacuous-by-profile'))
    for g in record['unresolved_gaps']:
        _need(g.get('statement') and g.get('clauses'), f"gap {g['id']}: needs statement and clauses")
        for cl in g['clauses']:
            _need(cl in CLAUSES or cl == 'costs', f"gap {g['id']}: unknown clause {cl!r}")
    costs = record['costs']
    for m in costs:
        _need(m.get('status') in COST_STATUS, f"cost {m.get('metric')}: status {m.get('status')!r}")
        if m['status'] == 'measured':
            _need(m.get('evidence') in evidence and 'reference' in m and 'candidate' in m,
                  f"cost {m.get('metric')}: measured cost needs values and evidence")
        else:
            _need(m.get('gap') in gaps, f"cost {m.get('metric')}: unknown cost needs a gap")
    run_ids = {e['id'] for e in record['evidence'] if e.get('kind') == 'compiled-and-run'}
    for c in clauses:
        c['status'] = ('evidenced' if set(c['evidence']) & run_ids else
                       'vacuous' if c['evidence'] else 'gap-only')
        if c['gaps']:
            c['status'] += '+open-gaps'
    return dict(format=REPORT_FORMAT, status='predicate-stated-and-evidence-verified',
                candidate=cand['id'], reference=record['reference'],
                preservation_predicate=pred, clauses=clauses, evidence=verified,
                unresolved_gaps=[dict(id=g['id'], clauses=g['clauses']) for g in record['unresolved_gaps']],
                costs=[dict(metric=m['metric'], status=m['status']) for m in costs],
                acceptance='none', native_evidence='none', adoption='not-adopted')


def main(record_path, repo_root):
    """Return (exit, report)."""
    repo = Repo(repo_root)
    path = Path(record_path) if record_path else Path(repo_root) / DEFAULT_RECORD
    try:
        record = json.loads(path.read_text())
        return 0, check(record, repo)
    except RecordError as e:
        return 1, dict(format=REPORT_FORMAT, status='record-incomplete', reason=str(e))
    except (Malformed, ValueError, KeyError, TypeError, AttributeError, OSError) as e:
        return 2, dict(format=REPORT_FORMAT, status='input-error', error_type=type(e).__name__,
                       reason=str(e) if isinstance(e, Malformed) else 'malformed record')
