"""Upstream drift: the pinned ZKIR v3 crates against the current upstream heads.

Two surfaces are compared, each by the serde `op` names of its `Instruction`
enum (`ir.rs`) with the names and types of their fields, the variants of its
`IrType` enum (`ir_types.rs`), the `midnight-proofs`, `midnight-zk-stdlib` and
`midnight-circuits` versions its `Cargo.lock` resolves for the crate, and the
bodies of the semantics-bearing sources (`git diff --stat <pin> <head> --` on
the crate's `ir_vm.rs`, its `ir_instructions/` directory and, where the crate
has one, `transient-crypto/src/proofs.rs`): a non-empty diff-stat is drift,
since a body change can alter `preprocess` or `Relation::circuit` without
touching a serde name:

  midnight-ledger  zkir-v3   pinned at 92e8bdd3 (repos/_build/ledger-92e8bdd3)
  midnight-zkir    zkir      pinned at 2ffe2d1  (repos/_build/midnight-zkir-2ffe2d1)

Each pinned crate is read from its extracted source tree and compared with the
head of the upstream default branch and, when the pin lives on another branch,
with the head of that branch (`git -C <clone> fetch origin <branch>`, then
`git show origin/<branch>:<path>`). When the fetch fails the local ref is used
and the output says so.

The enums are parsed with regular expressions after comments are stripped:
`#[serde(rename = "...")]` on a variant wins, else the enum's
`#[serde(rename_all = "...")]` is applied to the variant name. Struct variants
also contribute their field names and field types (`name: Type`, whitespace
normalised): a changed type is a "shape changed" entry, and the field-name set
is how renamed-looking pairs are detected (same field set, or a close name).

Exit status: 0 when no counted comparison drifts, 1 on drift, 2 on a tool
failure. A comparison counts when the pinned commit is an ancestor of the
compared head (forward drift); a comparison against a branch that does not
contain the pin is printed as informational unless it is the only one.

Usage: uv run --group zkir-k python experiments/zkir-k/tools/upstream_drift.py [--no-fetch]
"""
from __future__ import annotations

import argparse
import datetime as dt
import difflib
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent.parent

SURFACES = [
    {
        'name': 'midnight-ledger zkir-v3',
        'clone': REPO / 'repos/midnightntwrk/midnight-ledger',
        'pinned_dir': REPO / 'repos/_build/ledger-92e8bdd3',
        'pinned_commit': '92e8bdd3a97b61b229e38916e1b180de6f448dd5',
        'pin_branch': 'ledger-9',
        'ir': 'zkir-v3/src/ir.rs',
        'types': 'zkir-v3/src/ir_types.rs',
        'lock': 'Cargo.lock',
        'lock_package': 'midnight-zkir-v3',
        'bodies': ['zkir-v3/src/ir_vm.rs', 'zkir-v3/src/ir_instructions/', 'transient-crypto/src/proofs.rs'],
    },
    {
        'name': 'midnight-zkir zkir',
        'clone': REPO / 'repos/midnightntwrk/midnight-zkir',
        'pinned_dir': REPO / 'repos/_build/midnight-zkir-2ffe2d1',
        'pinned_commit': '2ffe2d17bbb736aec36fb300aeaca679a10d2278',
        'pin_branch': None,
        'ir': 'zkir/src/ir.rs',
        'types': 'zkir/src/ir_types.rs',
        'lock': 'Cargo.lock',
        'lock_package': 'midnight-zkir',
        'bodies': ['zkir/src/ir_vm.rs', 'zkir/src/ir_instructions/'],
    },
]

DEPS = ('midnight-proofs', 'midnight-zk-stdlib', 'midnight-circuits')


class ToolFailure(Exception):
    pass


# --- git ---------------------------------------------------------------------------------

def git(clone: Path, *args: str, check: bool = True, timeout: int = 180) -> str:
    res = subprocess.run(['git', '-C', str(clone), *args], capture_output=True, text=True, timeout=timeout)
    if check and res.returncode != 0:
        raise ToolFailure(f"git {' '.join(args)} in {clone}: {res.stderr.strip()}")
    return res.stdout


def default_branch(clone: Path, fetch: bool) -> tuple[str, str]:
    """The upstream default branch and how it was found."""
    if fetch:
        try:
            out = git(clone, 'ls-remote', '--symref', 'origin', 'HEAD', timeout=60)
            m = re.search(r'^ref: refs/heads/(\S+)\tHEAD$', out, re.M)
            if m:
                return m.group(1), 'ls-remote'
        except (ToolFailure, subprocess.TimeoutExpired):
            pass
    try:
        out = git(clone, 'symbolic-ref', 'refs/remotes/origin/HEAD').strip()
        return out.rsplit('/', 1)[-1], 'local origin/HEAD'
    except ToolFailure:
        pass
    out = git(clone, 'rev-parse', '--abbrev-ref', 'HEAD').strip()
    return out, 'local checkout'


def resolve_head(clone: Path, branch: str, fetch: bool) -> tuple[str, str, str]:
    """(ref, commit, note) for origin/<branch>, fetched when possible."""
    ref = f'origin/{branch}'
    if fetch:
        try:
            git(clone, 'fetch', '--quiet', 'origin', f'+refs/heads/{branch}:refs/remotes/origin/{branch}')
            return ref, git(clone, 'rev-parse', ref).strip(), 'fetched'
        except (ToolFailure, subprocess.TimeoutExpired) as e:
            note = f'fetch failed ({str(e).splitlines()[-1][:80]}); '
    else:
        note = 'fetch skipped; '
    try:
        return ref, git(clone, 'rev-parse', '--verify', '--quiet', ref).strip(), note + f'using local {ref}'
    except ToolFailure:
        return 'HEAD', git(clone, 'rev-parse', 'HEAD').strip(), note + 'using local HEAD'


def show(clone: Path, commit: str, path: str) -> str:
    try:
        return git(clone, 'show', f'{commit}:{path}')
    except ToolFailure as e:
        raise ToolFailure(f'{path} is absent at {commit[:8]}: {e}') from e


def body_diff_stat(clone: Path, pin: str, head: str, paths: list[str]) -> tuple[str, list[str]]:
    """`git diff --stat pin head -- paths`: the summary line and the per-file
    lines; empty when the bodies are identical."""
    out = git(clone, 'diff', '--stat=200', pin, head, '--', *paths)
    lines = [line.rstrip() for line in out.splitlines() if line.strip()]
    if not lines:
        return '', []
    return lines[-1].strip(), lines[:-1]


def is_ancestor(clone: Path, a: str, b: str) -> bool:
    res = subprocess.run(['git', '-C', str(clone), 'merge-base', '--is-ancestor', a, b], capture_output=True)
    return res.returncode == 0


# --- Rust parsing ------------------------------------------------------------------------

def strip_comments(src: str) -> str:
    src = re.sub(r'/\*.*?\*/', '', src, flags=re.S)
    return re.sub(r'//[^\n]*', '', src)


def enum_block(src: str, name: str) -> tuple[list[str], str]:
    """(attributes preceding the enum, body between its braces)."""
    src = strip_comments(src)
    m = re.search(r'((?:^[ \t]*#\[[^\n]*\]\s*\n)*)^[ \t]*pub(?:\([^)]*\))?\s+enum\s+' + re.escape(name) + r'\b[^{]*\{', src, re.M)
    if not m:
        raise ToolFailure(f'enum {name} not found')
    attrs = re.findall(r'#\[(.*?)\]\s*$', m.group(1), re.M)
    i = m.end()
    depth = 1
    while depth and i < len(src):
        c = src[i]
        depth += (c == '{') - (c == '}')
        i += 1
    if depth:
        raise ToolFailure(f'unbalanced braces in enum {name}')
    return attrs, src[m.end():i - 1]


def rename_all(attrs: list[str]) -> str | None:
    for a in attrs:
        m = re.search(r'serde\s*\(.*?rename_all\s*=\s*"([^"]+)"', a)
        if m:
            return m.group(1)
    return None


def apply_case(name: str, rule: str | None) -> str:
    if rule is None:
        return name
    words = re.findall(r'[A-Z][a-z0-9]*|[a-z0-9]+', name)
    if rule == 'snake_case':
        return '_'.join(w.lower() for w in words)
    if rule == 'SCREAMING_SNAKE_CASE':
        return '_'.join(w.upper() for w in words)
    if rule == 'kebab-case':
        return '-'.join(w.lower() for w in words)
    if rule == 'lowercase':
        return name.lower()
    if rule == 'UPPERCASE':
        return name.upper()
    if rule == 'camelCase':
        return words[0].lower() + ''.join(w.title() for w in words[1:])
    if rule == 'PascalCase':
        return ''.join(w.title() for w in words)
    return name


def variants(body: str, rule: str | None) -> dict[str, dict]:
    """serde name -> {variant, fields, payload}; fields are the struct-variant field names."""
    out: dict[str, dict] = {}
    i, n = 0, len(body)
    pending_attrs: list[str] = []
    while i < n:
        c = body[i]
        if c.isspace() or c == ',':
            i += 1
            continue
        if c == '#':
            j = body.index('[', i)
            depth = 0
            while j < n:
                depth += (body[j] == '[') - (body[j] == ']')
                j += 1
                if depth == 0:
                    break
            pending_attrs.append(body[i + 1:j].strip('[]'))
            i = j
            continue
        m = re.match(r'[A-Za-z_][A-Za-z0-9_]*', body[i:])
        if not m:
            raise ToolFailure(f'unexpected character {c!r} in enum body at offset {i}')
        ident = m.group(0)
        i += len(ident)
        while i < n and body[i].isspace():
            i += 1
        payload = ''
        fields: tuple[str, ...] = ()
        if i < n and body[i] in '{(':
            open_, close = body[i], {'{': '}', '(': ')'}[body[i]]
            j, depth = i, 0
            while j < n:
                depth += (body[j] == open_) - (body[j] == close)
                j += 1
                if depth == 0:
                    break
            inner = body[i + 1:j - 1]
            if open_ == '{':
                inner_no_attrs = re.sub(r'#\[[^\]]*\]', '', inner)
                typed = struct_fields(inner_no_attrs)
                fields = tuple(sorted(name for name, _ in typed))
                payload = '{' + ', '.join(f'{name}: {ty}' for name, ty in sorted(typed)) + '}'
            else:
                payload = '(' + ' '.join(inner.split()) + ')'
            i = j
        if i < n and body[i] == '=':  # discriminant
            while i < n and body[i] != ',':
                i += 1
        rename = None
        for a in pending_attrs:
            r = re.search(r'serde\s*\(.*?\brename\s*=\s*"([^"]+)"', a)
            if r:
                rename = r.group(1)
        serde = rename if rename is not None else apply_case(ident, rule)
        out[serde] = {'variant': ident, 'fields': fields, 'payload': payload, 'explicit': rename is not None}
        pending_attrs = []
    return out


def struct_fields(inner: str) -> list[tuple[str, str]]:
    """(name, type) of every field of a struct variant body, the type with its
    whitespace normalised; fields are split on the commas outside brackets."""
    out: list[tuple[str, str]] = []
    depth = 0
    part = ''
    parts = []
    for c in inner:
        if c in '<([{':
            depth += 1
        elif c in '>)]}':
            depth -= 1
        if c == ',' and depth == 0:
            parts.append(part)
            part = ''
        else:
            part += c
    parts.append(part)
    for item in parts:
        m = re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(.*)$', item.strip(), re.S)
        if m:
            out.append((m.group(1), ' '.join(m.group(2).split())))
    return out


def instructions(src: str) -> dict[str, dict]:
    attrs, body = enum_block(src, 'Instruction')
    return variants(body, rename_all(attrs))


def ir_types(src: str) -> dict[str, dict]:
    attrs, body = enum_block(src, 'IrType')
    return variants(body, rename_all(attrs))


# --- Cargo.lock --------------------------------------------------------------------------

def lock_packages(text: str) -> list[dict]:
    pkgs = []
    for block in re.split(r'^\[\[package\]\]\s*$', text, flags=re.M)[1:]:
        name = re.search(r'^name = "([^"]+)"', block, re.M)
        version = re.search(r'^version = "([^"]+)"', block, re.M)
        deps = re.search(r'^dependencies = \[(.*?)^\]', block, re.M | re.S)
        pkgs.append({
            'name': name.group(1) if name else '?',
            'version': version.group(1) if version else '?',
            'deps': re.findall(r'"([^"]+)"', deps.group(1)) if deps else [],
        })
    return pkgs


def resolved_versions(text: str, package: str) -> dict[str, str]:
    """dep name -> version as resolved for `package` (all versions when unresolvable)."""
    pkgs = lock_packages(text)
    by_name: dict[str, list[str]] = {}
    for p in pkgs:
        by_name.setdefault(p['name'], []).append(p['version'])
    owner = [p for p in pkgs if p['name'] == package]
    out = {}
    for dep in DEPS:
        versions = sorted(set(by_name.get(dep, [])))
        if not versions:
            out[dep] = 'absent'
            continue
        if owner:
            entries = [d for d in owner[0]['deps'] if d == dep or d.startswith(dep + ' ')]
            if entries:
                e = entries[0]
                out[dep] = e.split(' ', 1)[1] if ' ' in e else (versions[0] if len(versions) == 1 else '/'.join(versions))
                continue
            out[dep] = 'not a dependency of ' + package + ' (lock has ' + ', '.join(versions) + ')'
            continue
        out[dep] = '/'.join(versions) + f' (package {package} not in lock)'
    return out


# --- comparison --------------------------------------------------------------------------

def renamed_pairs(removed: dict[str, dict], added: dict[str, dict]) -> list[tuple[str, str, str]]:
    pairs = []
    for r, rv in removed.items():
        for a, av in added.items():
            if rv['fields'] and rv['fields'] == av['fields']:
                pairs.append((r, a, 'same fields ' + rv['payload']))
                continue
            ratio = difflib.SequenceMatcher(None, r, a).ratio()
            if ratio >= 0.6 or r.startswith(a) or a.startswith(r):
                pairs.append((r, a, f'name similarity {ratio:.2f}'))
    return pairs


def fmt(items: dict[str, dict]) -> str:
    return ', '.join(f'{k}{v["payload"] if v["payload"] and not v["payload"].startswith("{") else ""}' for k, v in items.items()) or '(none)'


def fmt_shape(items: dict[str, dict], keys: set[str]) -> str:
    return ', '.join(f'{k}{items[k]["payload"]}' for k in sorted(keys)) or '(none)'


def compare(label: str, pinned: dict, head: dict) -> tuple[bool, list[str]]:
    lines = []
    drift = False
    for kind, key in (('instructions', 'instr'), ('types', 'types')):
        a, b = pinned[key], head[key]
        added = {k: v for k, v in b.items() if k not in a}
        removed = {k: v for k, v in a.items() if k not in b}
        changed = {k for k in a if k in b and a[k]['payload'] != b[k]['payload']}
        drift = drift or bool(added or removed or changed)
        lines.append(f'  {kind}: pinned {len(a)}, {label} {len(b)}')
        lines.append(f'    added:   {fmt(added)}')
        lines.append(f'    removed: {fmt(removed)}')
        if kind == 'instructions':
            pairs = renamed_pairs(removed, added)
            lines.append('    renamed-looking: ' + ('; '.join(f'{r} -> {s} ({why})' for r, s, why in pairs) or '(none)'))
        if changed:
            lines.append('    shape changed (field names or types): ' + ', '.join(f'{k}: {a[k]["payload"]} -> {b[k]["payload"]}' for k in sorted(changed)))
    dep_drift = False
    for dep in DEPS:
        p, h = pinned['deps'].get(dep, '?'), head['deps'].get(dep, '?')
        mark = '' if p == h else '   <- differs'
        dep_drift = dep_drift or (p != h)
        lines.append(f'  {dep}: pinned {p}, {label} {h}{mark}')
    return drift or dep_drift, lines


def load_side(ir_src: str, types_src: str, lock_src: str, package: str) -> dict:
    return {'instr': instructions(ir_src), 'types': ir_types(types_src), 'deps': resolved_versions(lock_src, package)}


def run_surface(s: dict, fetch: bool) -> tuple[list[str], str, bool]:
    out = [f"== {s['name']}"]
    clone, pin = s['clone'], s['pinned_commit']
    pinned_dir = s['pinned_dir']
    for p in (s['ir'], s['types'], s['lock']):
        if not (pinned_dir / p).exists():
            raise ToolFailure(f'pinned file missing: {pinned_dir / p}')
    pinned = load_side((pinned_dir / s['ir']).read_text(), (pinned_dir / s['types']).read_text(),
                       (pinned_dir / s['lock']).read_text(), s['lock_package'])
    out.append(f"  pinned commit: {pin} ({pinned_dir.relative_to(REPO)})")
    out.append(f"  pinned instructions ({len(pinned['instr'])}): {fmt(pinned['instr'])}")
    out.append(f"  pinned types ({len(pinned['types'])}): {fmt(pinned['types'])}")

    branch, how = default_branch(clone, fetch)
    out.append(f'  upstream default branch: {branch} (from {how})')
    targets = [(branch, 'default branch')]
    if s['pin_branch'] and s['pin_branch'] != branch:
        targets.append((s['pin_branch'], 'branch of the pin'))

    comparisons = []
    for b, role in targets:
        ref, commit, note = resolve_head(clone, b, fetch)
        anc = is_ancestor(clone, pin, commit)
        try:
            head = load_side(show(clone, commit, s['ir']), show(clone, commit, s['types']),
                             show(clone, commit, s['lock']), s['lock_package'])
        except ToolFailure as e:
            comparisons.append((b, commit, anc, None, [f'  compared commit: {commit} ({ref}, {role}; {note}): {e}']))
            continue
        drift, lines = compare(f'{b} head', pinned, head)
        try:
            stat, files = body_diff_stat(clone, pin, commit, s['bodies'])
        except ToolFailure as e:
            comparisons.append((b, commit, anc, None, [f'  compared commit: {commit} ({ref}, {role}; {note}): body diff failed: {e}']))
            continue
        lines.append(f"  bodies ({', '.join(s['bodies'])}): git diff --stat {pin[:8]}..{commit[:8]}: " + (stat if stat else 'identical'))
        for f in files[:40]:
            lines.append(f'    {f.strip()}')
        if len(files) > 40:
            lines.append(f'    ... {len(files) - 40} more files')
        drift = drift or bool(stat)
        header = f'  compared commit: {commit} ({ref}, {role}; {note}); pin is {"an ancestor" if anc else "NOT an ancestor"}'
        comparisons.append((b, commit, anc, drift, [header, *lines]))

    counted = [c for c in comparisons if c[2]] or comparisons
    surface_drift = any(c[3] for c in counted if c[3] is not None)
    failed = [c for c in comparisons if c[3] is None]
    for b, commit, anc, drift, lines in comparisons:
        out.append('')
        out.extend(lines)
        if drift is not None:
            status = 'DRIFT' if drift else 'no drift'
            out.append(f'  result vs {b} ({commit[:8]}): {status}' + ('' if (b, commit, anc, drift, lines) in counted else ' (informational: pin not an ancestor)'))
    parts = []
    for b, commit, anc, drift, _ in comparisons:
        if drift is None:
            parts.append(f'vs {b} {commit[:8]}: tool failure')
        else:
            parts.append(f'vs {b} {commit[:8]}: {"DRIFT" if drift else "no drift"}' + ('' if anc else ' (pin not on branch)'))
    summary = f"SUMMARY {s['name']}: pinned {pin[:8]}; " + '; '.join(parts)
    if failed:
        raise ToolFailure(summary)
    return out, summary, surface_drift


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument('--no-fetch', action='store_true', help='do not touch the network; use local refs')
    args = ap.parse_args()
    print(f'upstream_drift.py  {dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%d %H:%M UTC")}  repo {REPO}')
    print(f'network fetch: {"disabled" if args.no_fetch else "enabled"}')
    summaries, any_drift = [], False
    for s in SURFACES:
        print()
        try:
            lines, summary, drift = run_surface(s, fetch=not args.no_fetch)
        except ToolFailure as e:
            print(f"== {s['name']}\n  TOOL FAILURE: {e}")
            print(f"\nexit 2: tool failure on {s['name']}")
            return 2
        print('\n'.join(lines))
        summaries.append(summary)
        any_drift = any_drift or drift
    print()
    for line in summaries:
        print(line)
    print(f'\nexit {1 if any_drift else 0}: {"drift on a counted comparison" if any_drift else "no drift on the counted comparisons"}')
    return 1 if any_drift else 0


if __name__ == '__main__':
    try:
        sys.exit(main())
    except ToolFailure as e:
        print(f'TOOL FAILURE: {e}')
        sys.exit(2)
