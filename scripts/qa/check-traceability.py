#!/usr/bin/env python3
"""F14 traceability check for an openspec change (SWF-002, workflow.md E1).

Checks, all read-only:
  1. The requirement IDs of requirements.md and of specs/*/spec.md are the same set.
  2. Every requirement ID appears in the traceability.md requirement table, and that
     table names no unknown requirement ID. A predicate row it names that is not in
     predicates.yaml yet (a later milestone's row) is a warning, not an error.
  3. Every predicate row that is not retired has a `source:`, and every `source:`
     anchor resolves (file exists; `#Ln` is in range; `#slug` names a heading or a
     table-row ID). An anchor into this change resolves to an existing requirement ID.
  4. Every predicate row has a visible check: a non-empty `check:` whose script paths
     exist. `unimplemented.sh` names its own row. Retired rows keep their historical
     check, which is not verified.
  5. Every task in tasks.md names only requirement IDs and predicate rows that exist.

Exit 0 when every check holds, 1 otherwise (one `F14: ...` line per error,
one `F14 WARN: ...` line per warning).

`--render [PATH]` rewrites the traceability.md requirement table with a `Rows (status)`
column computed from predicates.yaml and status.json, and leaves the rest of the file
unchanged. PATH defaults to the change's traceability.md; `-` prints to stdout.

Standard library only: predicates.yaml is read with a parser for its flat row format.
"""
import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHANGE = 'openspec/changes/sherlock-vm-consensus-20261107'
PREDICATES = 'qa/sherlock/predicates.yaml'
STATUS = 'qa/sherlock/status.json'

REQ_ROW = re.compile(r'^\|\s*([A-Z]{2,5}-\d{3})\s*\|')
SPEC_REQ = re.compile(r'^###\s+Requirement:\s+([A-Z]{2,5}-\d{3})\b')
HEADING = re.compile(r'^#{1,6}\s+(.*?)\s*#*\s*$')
TASK = re.compile(r'^- \[[ xX]\] (\d+\.\d+)\s+(.*)$')
PRED_TOKEN = re.compile(r'(?<![\w-])([ABCUF]\d+[a-z]?)(?![\w-])')
LINE_ANCHOR = re.compile(r'^L(\d+)(?:-L?(\d+))?$')
COLUMN = 'Rows (status)'


# ---------------------------------------------------------------- predicates.yaml

def _scalar(text):
    text = text.strip()
    if not text:
        return ''
    if text[0] == '"':
        return json.loads(text)
    if text[0] == "'":
        return text[1:-1].replace("''", "'")
    if text[0] == '[':
        inner = text[1:-1].strip()
        return [_scalar(part) for part in inner.split(',')] if inner else []
    if text in ('true', 'false'):
        return text == 'true'
    if re.fullmatch(r'-?\d+', text):
        return int(text)
    return text.split(' #', 1)[0].strip()


def load_predicates(path):
    """Parse the flat `predicates:` list of predicates.yaml (scalars and inline lists)."""
    rows = []
    in_list = False
    for number, raw in enumerate(Path(path).read_text().splitlines(), 1):
        line = raw.rstrip()
        if not line.strip() or line.lstrip().startswith('#'):
            continue
        if not line.startswith(' '):
            in_list = line.startswith('predicates:')
            continue
        if not in_list:
            continue
        item = re.match(r'^\s*-\s+([A-Za-z_][\w-]*):\s?(.*)$', line)
        field = re.match(r'^\s+([A-Za-z_][\w-]*):\s?(.*)$', line)
        if item:
            rows.append({item.group(1): _scalar(item.group(2))})
        elif field and rows:
            rows[-1][field.group(1)] = _scalar(field.group(2))
        else:
            raise ValueError(f'{path}:{number}: unsupported line: {raw!r}')
    return rows


# ---------------------------------------------------------------- markdown helpers

def slug(text):
    """GitHub heading anchor."""
    text = text.strip().lower()
    text = re.sub(r'[^\w\- ]', '', text)
    return text.replace(' ', '-')


def headings(text):
    out = set()
    for line in text.splitlines():
        m = HEADING.match(line)
        if m:
            out.add(slug(m.group(1)))
    return out


def table_ids(text):
    return {m.group(1) for m in map(REQ_ROW.match, text.splitlines()) if m}


def requirement_ids(change):
    req = []
    for line in (change / 'requirements.md').read_text().splitlines():
        m = REQ_ROW.match(line)
        if m:
            req.append(m.group(1))
    spec = {}
    for path in sorted((change / 'specs').glob('*/spec.md')):
        for line in path.read_text().splitlines():
            m = SPEC_REQ.match(line)
            if m:
                spec.setdefault(m.group(1), []).append(path)
    return req, spec


def cells(line):
    return [c.strip() for c in line.strip().strip('|').split('|')]


def trace_table(text):
    """Return (start, end, header, rows) of the table whose first header cell is Requirement."""
    lines = text.splitlines()
    for i, line in enumerate(lines):
        if line.startswith('|') and cells(line)[0] == 'Requirement':
            end = i + 1
            while end < len(lines) and lines[end].startswith('|'):
                end += 1
            body = [cells(row) for row in lines[i + 2:end]]
            return i, end, cells(line), body
    return None


# ---------------------------------------------------------------- checks

def resolve_anchor(root, source, errors, rid, req_set, change_rel):
    path_part, _, anchor = source.partition('#')
    path = root / path_part
    if not path_part or not path.is_file():
        errors.append(f'{rid}: source file missing: {source}')
        return
    if not anchor:
        return
    text = path.read_text()
    m = LINE_ANCHOR.match(anchor)
    if m:
        count = len(text.splitlines())
        hi = int(m.group(2) or m.group(1))
        if not 1 <= int(m.group(1)) <= hi <= count:
            errors.append(f'{rid}: source line anchor out of range ({count} lines): {source}')
        return
    in_change = path_part.startswith(change_rel + '/')
    ids = {i.lower(): i for i in table_ids(text)}
    if anchor in ids:
        if in_change and ids[anchor] not in req_set:
            errors.append(f'{rid}: source anchor names no requirement of the change: {source}')
        return
    if anchor in headings(text):
        if in_change:
            m = re.match(r'^requirement-([a-z]{2,5}-\d{3})(?:-|$)', anchor)
            if not m or m.group(1).upper() not in req_set:
                errors.append(f'{rid}: source anchor names no requirement of the change: {source}')
        return
    errors.append(f'{rid}: source anchor does not resolve: {source}')


def check_paths(root, rid, check, errors):
    base = root
    for part in re.split(r'\s*(?:&&|\|\||;|\|)\s*', check):
        tokens = part.split()
        if not tokens:
            continue
        if tokens[0] == 'cd' and len(tokens) > 1:
            base = root / tokens[1]
            if not base.is_dir():
                errors.append(f'{rid}: check cds into a missing directory: {tokens[1]}')
            continue
        for tok in tokens:
            tok = tok.strip('"\'')
            if tok.startswith('-') or '$' in tok or '/' not in tok:
                continue
            if not re.search(r'\.(sh|py|mjs|js)$', tok) and not tok.startswith(('scripts/', 'qa/')):
                continue
            if not ((base / tok).exists() or (root / tok).exists()):
                errors.append(f'{rid}: check names a missing path: {tok}')


def check(root=ROOT, change_rel=CHANGE, predicates_rel=PREDICATES):
    root = Path(root).resolve()
    change = root / change_rel
    errors = []
    warnings = []

    req, spec = requirement_ids(change)
    req_set = set(req)
    for dup in sorted({i for i in req if req.count(i) > 1}):
        errors.append(f'requirements.md: duplicate requirement ID {dup}')
    for rid, paths in sorted(spec.items()):
        if len(paths) > 1:
            errors.append(f'specs: duplicate requirement ID {rid}')
        if rid not in req_set:
            errors.append(f'{rid}: in {paths[0].relative_to(root)} but not in requirements.md')
    for rid in req:
        if rid not in spec:
            errors.append(f'{rid}: in requirements.md but in no specs/*/spec.md')
    all_ids = req_set | set(spec)
    prefixes = {i.split('-')[0] for i in all_ids}
    id_token = re.compile(r'\b(' + '|'.join(sorted(prefixes)) + r')-(\d{3})(?:\.\.(\d{3}))?\b') if prefixes else None

    rows = load_predicates(root / predicates_rel)
    pred_ids = [r.get('id') for r in rows]
    pred_set = set(pred_ids)
    for dup in sorted({i for i in pred_ids if pred_ids.count(i) > 1}):
        errors.append(f'predicates.yaml: duplicate row id {dup}')

    # 2. traceability.md
    trace_path = change / 'traceability.md'
    table = trace_table(trace_path.read_text()) if trace_path.is_file() else None
    if table is None:
        errors.append('traceability.md: no table whose first column is "Requirement"')
    else:
        _, _, header, body = table
        pcol = header.index('Predicate') if 'Predicate' in header else None
        traced = set()
        for row in body:
            m = re.match(r'^([A-Z]{2,5}-\d{3})\b', row[0])
            if not m:
                errors.append(f'traceability.md: row without a requirement ID: {row[0]!r}')
                continue
            traced.add(m.group(1))
            if m.group(1) not in all_ids:
                errors.append(f'traceability.md: unknown requirement ID {m.group(1)}')
            if pcol is not None and pcol < len(row):
                for tok in PRED_TOKEN.findall(row[pcol]):
                    if tok not in pred_set:
                        warnings.append(f'traceability.md: {m.group(1)} names predicate row {tok}, '
                                        'which predicates.yaml does not have yet')
        for rid in sorted(all_ids - traced):
            errors.append(f'{rid}: missing from traceability.md')

    # 3, 4. predicate rows
    for row in rows:
        rid = row.get('id', '?')
        retired = bool(row.get('retired'))
        source = row.get('source')
        if not source:
            if not retired:
                errors.append(f'{rid}: no source: anchor')
        else:
            resolve_anchor(root, str(source), errors, rid, all_ids, change_rel)
        command = str(row.get('check') or '').strip()
        if not command:
            errors.append(f'{rid}: no check:')
            continue
        if retired:
            continue
        m = re.match(r'^scripts/qa/unimplemented\.sh(?:\s+(\S+))?\s*$', command)
        if m and m.group(1) != rid:
            errors.append(f'{rid}: unimplemented.sh names {m.group(1)!r}, not {rid}')
        check_paths(root, rid, command, errors)

    # 5. tasks.md
    tasks_path = change / 'tasks.md'
    for line in tasks_path.read_text().splitlines():
        m = TASK.match(line)
        if not m:
            continue
        number, body = m.groups()
        if id_token:
            for prefix, lo, hi in id_token.findall(body):
                for n in range(int(lo), int(hi or lo) + 1):
                    rid = f'{prefix}-{n:03d}'
                    if rid not in all_ids:
                        errors.append(f'tasks.md {number}: unknown requirement ID {rid}')
        if '->' in body:
            for tok in PRED_TOKEN.findall(body.rsplit('->', 1)[1]):
                if tok not in pred_set:
                    errors.append(f'tasks.md {number}: unknown predicate row {tok}')
    return errors, warnings


# ---------------------------------------------------------------- render

def row_state(row, status):
    if row.get('retired'):
        return 'retired'
    entry = status.get(row['id'])
    if not entry:
        state = 'not run'
    elif entry.get('passes'):
        state = 'pass'
    elif entry.get('exit') == 2:
        state = 'unimplemented'
    else:
        state = 'fail'
    return state + (', optional' if row.get('optional') else '')


def render(root=ROOT, change_rel=CHANGE, predicates_rel=PREDICATES, status_rel=STATUS):
    root = Path(root).resolve()
    change = root / change_rel
    text = (change / 'traceability.md').read_text()
    table = trace_table(text)
    if table is None:
        raise ValueError('traceability.md: no requirement table')
    start, end, header, body = table
    rows = {r['id']: r for r in load_predicates(root / predicates_rel)}
    try:
        status = json.loads((root / status_rel).read_text()).get('predicates', {})
    except (OSError, ValueError):
        status = {}
    by_source = {}
    for row in rows.values():
        anchor = str(row.get('source') or '').partition('#')[2]
        m = re.match(r'^(?:requirement-)?([a-z]{2,5}-\d{3})', anchor)
        if m and str(row.get('source')).startswith(change_rel + '/'):
            by_source.setdefault(m.group(1).upper(), []).append(row['id'])
    if COLUMN in header:
        drop = header.index(COLUMN)
        header = header[:drop] + header[drop + 1:]
        body = [r[:drop] + r[drop + 1:] for r in body]
    pcol = header.index('Predicate') if 'Predicate' in header else len(header) - 1
    out = ['| ' + ' | '.join(header[:pcol + 1] + [COLUMN] + header[pcol + 1:]) + ' |',
           '|' + '---|' * (len(header) + 1)]
    for row in body:
        rid = re.match(r'^([A-Z]{2,5}-\d{3})', row[0])
        names = [t for t in PRED_TOKEN.findall(row[pcol]) if t in rows] if pcol < len(row) else []
        if rid:
            names += [n for n in by_source.get(rid.group(1), []) if n not in names]
        states = '; '.join(f'{n} {row_state(rows[n], status)}' for n in names) or 'no row'
        out.append('| ' + ' | '.join(row[:pcol + 1] + [states] + row[pcol + 1:]) + ' |')
    lines = text.splitlines()
    return '\n'.join(lines[:start] + out + lines[end:]) + '\n'


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    parser.add_argument('--root', default=str(ROOT))
    parser.add_argument('--change', default=CHANGE)
    parser.add_argument('--render', nargs='?', const='', metavar='PATH')
    args = parser.parse_args(argv)
    root = Path(args.root)
    if args.render is not None:
        text = render(root, args.change)
        if args.render == '-':
            sys.stdout.write(text)
        else:
            target = Path(args.render) if args.render else root / args.change / 'traceability.md'
            target.write_text(text)
            print(f'F14: rendered {target}')
    errors, warnings = check(root, args.change)
    for warning in warnings:
        print(f'F14 WARN: {warning}')
    for error in errors:
        print(f'F14: {error}')
    if errors:
        print(f'F14 FAIL: {len(errors)} error(s)')
        return 1
    print('F14 PASS: requirements, traceability, sources, checks and tasks agree')
    return 0


if __name__ == '__main__':
    sys.exit(main())
