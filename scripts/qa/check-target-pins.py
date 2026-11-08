#!/usr/bin/env python3
"""F15 static target-pins check (SWF-007, SMN-001, SMN-004; workflow.md E2).

Reads qa/sherlock/target-pins.json and checks, read-only:
  1. Structure: rows `undeployed`, `preview`, `mainnet` and `stagenet` (retired); each
     row has a status, a chain identity (network name, genesis hash or null with a note,
     specVersion, ledger revision or null with a note) and every pinned component with
     a source that names a `sources` key or an existing file. No seed, mnemonic,
     secret or private key field.
  2. The workspace pins match the row named by `workspaceTarget` (or `--network`):
     - Cargo.toml: every midnight-ledger git dependency is at the row's ledger commit;
       midnight-circuits, midnight-zk-stdlib and midnight-proofs versions;
     - Cargo.lock: midnight-ledger-v9 version and the ZKIR v3 crate version;
     - SDK pins committed in tools/sherlock-sdk/*.mjs (package version asserts and
       proof-server /version asserts);
     - the SDK root's package.json and package-lock.json: the committed manifest in
       tools/sherlock-sdk/, or the root named by `--sdk-root` (or MORIARTY_SDK_ROOT).

Exit 0 when the structure is valid and the workspace matches the target row, 1 otherwise.
It also prints which rows, if any, the workspace matches in full.

Standard library only.
"""
import argparse
import json
import os
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PINS = 'qa/sherlock/target-pins.json'
SDK_DIR = 'tools/sherlock-sdk'
NETWORKS = ('undeployed', 'preview', 'mainnet', 'stagenet')
STATUSES = ('active', 'blocked', 'retired')
COMPONENTS = ('ledger', 'node', 'circuits', 'zkStdlib', 'proofs', 'zkir', 'compact',
              'midnightJs', 'walletSdk', 'ledgerJs', 'onchainRuntime', 'proofServer', 'indexer')
LEDGER_REPO = 'github.com/midnightntwrk/midnight-ledger'
FORBIDDEN = re.compile(r'seed|mnemonic|secret|private', re.I)
NPM_PIN = re.compile(r"""['"](@midnight-?ntwrk/[\w.-]+)['"]\s*:\s*['"]([^'"]+)['"]""")
SEMVER = re.compile(r"""['"](\d+\.\d+\.\d+(?:-[\w.]+)?)['"]""")


# ---------------------------------------------------------------- structure

def _walk_keys(value, path=''):
    if isinstance(value, dict):
        for key, item in value.items():
            yield f'{path}.{key}' if path else key, key
            yield from _walk_keys(item, f'{path}.{key}' if path else key)
    elif isinstance(value, list):
        for i, item in enumerate(value):
            yield from _walk_keys(item, f'{path}[{i}]')


def _source_ok(root, sources, text):
    tokens = [t.strip() for t in str(text or '').split(',') if t.strip()]
    if not tokens:
        return False
    for token in tokens:
        if token in sources:
            continue
        if not (root / token.split('#', 1)[0]).exists():
            return False
    return True


def check_structure(pins, root):
    errors = []
    if pins.get('schema') != 'moriarty.target-pins/1':
        errors.append('schema is not moriarty.target-pins/1')
    sources = pins.get('sources') or {}
    networks = pins.get('networks') or {}
    for path, key in _walk_keys(pins):
        if FORBIDDEN.search(key):
            errors.append(f'{path}: field name looks like a secret; target-pins.json holds no secrets')
    for name in NETWORKS:
        row = networks.get(name)
        if not isinstance(row, dict):
            errors.append(f'{name}: row missing')
            continue
        if row.get('status') not in STATUSES:
            errors.append(f'{name}: status must be one of {", ".join(STATUSES)}')
        chain = row.get('chain') or {}
        if chain.get('networkId') != name:
            errors.append(f'{name}: chain.networkId must be {name!r}')
        for field in ('genesisHash', 'ledgerRev'):
            if field not in chain:
                errors.append(f'{name}: chain.{field} missing (SMN-004)')
            elif chain[field] is None and not chain.get(field.replace('Hash', '') + 'Note') \
                    and not chain.get(field + 'Note'):
                errors.append(f'{name}: chain.{field} is null without a note')
            elif chain[field] is not None and not re.fullmatch(r'(0x)?[0-9a-f]{40,64}', str(chain[field])):
                errors.append(f'{name}: chain.{field} is not a hex hash')
        if not isinstance(chain.get('specVersion'), int):
            errors.append(f'{name}: chain.specVersion must be an integer')
        if not _source_ok(root, sources, chain.get('source')):
            errors.append(f'{name}: chain.source missing or does not resolve')
        if not _source_ok(root, sources, row.get('statusSource')):
            errors.append(f'{name}: statusSource missing or does not resolve')
        for comp in COMPONENTS:
            item = row.get(comp)
            if not isinstance(item, dict):
                errors.append(f'{name}: {comp} missing')
            elif not _source_ok(root, sources, item.get('source')):
                errors.append(f'{name}: {comp}.source missing or does not resolve')
    if (networks.get('stagenet') or {}).get('status') != 'retired':
        errors.append('stagenet: status must be retired')
    extra = sorted(set(networks) - set(NETWORKS))
    if extra:
        errors.append(f'unknown rows: {", ".join(extra)}')
    target = pins.get('workspaceTarget')
    if target not in networks:
        errors.append(f'workspaceTarget {target!r} names no row')
    elif networks[target].get('status') == 'retired':
        errors.append(f'workspaceTarget {target!r} is a retired row')
    return errors


# ---------------------------------------------------------------- workspace observation

def _strip_req(version):
    return str(version).lstrip('=^~ ').strip()


def _dep_version(spec):
    return spec if isinstance(spec, str) else (spec or {}).get('version')


def observe(root, sdk_root=None):
    """Return (observations, notes). Each observation is (component, field, value, where)."""
    obs, notes = [], []
    manifests = [root / 'Cargo.toml', *sorted(root.glob('crates/*/Cargo.toml'))]
    for path in manifests:
        rel = str(path.relative_to(root))
        try:
            data = tomllib.loads(path.read_text())
        except (OSError, ValueError) as exc:
            obs.append(('cargo', 'parse', f'unreadable ({exc})', rel))
            continue
        tables = [('workspace.dependencies', (data.get('workspace') or {}).get('dependencies') or {})]
        for section in ('dependencies', 'dev-dependencies', 'build-dependencies'):
            tables.append((section, data.get(section) or {}))
        patches = (data.get('patch') or {}).get('crates-io') or {}
        tables.append(('patch.crates-io', {k: v for k, v in patches.items()
                                          if not (isinstance(v, dict) and v.get('package'))}))
        for section, deps in tables:
            for name, spec in deps.items():
                if isinstance(spec, dict) and LEDGER_REPO in str(spec.get('git', '')):
                    obs.append(('ledger', 'commit', spec.get('rev') or spec.get('tag') or spec.get('branch'),
                                f'{rel} [{section}] {name}'))
        if path.name == 'Cargo.toml' and path.parent == root:
            wdeps = (data.get('workspace') or {}).get('dependencies') or {}
            for crate, comp in (('midnight-circuits', 'circuits'), ('midnight-zk-stdlib', 'zkStdlib'),
                                ('midnight-proofs', 'proofs')):
                version = _dep_version(wdeps.get(crate))
                if version:
                    obs.append((comp, 'version', _strip_req(version), f'{rel} {crate}'))
    lock = root / 'Cargo.lock'
    if lock.is_file():
        packages = tomllib.loads(lock.read_text()).get('package', [])
        for pkg in packages:
            name, version = pkg.get('name'), pkg.get('version', '')
            if name == 'midnight-ledger-v9':
                obs.append(('ledger', 'crateVersion', version, 'Cargo.lock midnight-ledger-v9'))
            if name == 'midnight-zkir-v3' or (name == 'midnight-zkir' and version.startswith('3.')):
                obs.append(('zkir', 'crate', f'{name} {version}', f'Cargo.lock {name}'))
    else:
        notes.append('Cargo.lock missing')
    sdk = root / SDK_DIR
    for path in sorted(sdk.glob('*.mjs')):
        if path.name.endswith('.test.mjs'):
            continue
        rel = str(path.relative_to(root))
        for number, line in enumerate(path.read_text().splitlines(), 1):
            for pkg, version in NPM_PIN.findall(line):
                obs.append(('npm', pkg, version, f'{rel}:{number}'))
            # Proof-server /version asserts: fetch(.../version) ... .trim(), 'X' or .trim() === 'X'.
            if '/version' in line or '.trim()' in line:
                for version in SEMVER.findall(line):
                    obs.append(('proofServer', 'version', version, f'{rel}:{number}'))
    if not sdk_root and (sdk / 'package.json').is_file():
        sdk_root = sdk
    if sdk_root:
        sdk_root = Path(sdk_root)
        pkg_json = sdk_root / 'package.json'
        lock_json = sdk_root / 'package-lock.json'
        if not pkg_json.is_file():
            notes.append(f'SDK root {sdk_root} has no package.json')
        else:
            data = json.loads(pkg_json.read_text())
            for section in ('dependencies', 'devDependencies'):
                for pkg, version in (data.get(section) or {}).items():
                    if re.match(r'@midnight-?ntwrk/', pkg):
                        obs.append(('npm', pkg, _strip_req(version), f'{pkg_json} {section}'))
        if lock_json.is_file():
            data = json.loads(lock_json.read_text())
            for key, entry in (data.get('packages') or {}).items():
                m = re.fullmatch(r'node_modules/(@midnight-?ntwrk/[\w.-]+)', key)
                if m and entry.get('version'):
                    obs.append(('npm', m.group(1), entry['version'], f'{lock_json}'))
        else:
            notes.append(f'SDK root {sdk_root} has no package-lock.json')
    else:
        notes.append(f'no SDK root: {SDK_DIR}/package.json is missing and neither --sdk-root nor '
                     'MORIARTY_SDK_ROOT is set; only the version asserts in '
                     f'{SDK_DIR}/*.mjs are compared')
    return obs, notes


# ---------------------------------------------------------------- comparison

def _caret_ok(version, req):
    req = str(req).strip()
    if req.startswith('='):
        return version == req[1:].strip()
    base = req.lstrip('^').strip()
    try:
        v = [int(x) for x in version.split('-')[0].split('.')]
        b = [int(x) for x in base.split('.')]
    except ValueError:
        return False
    if '-' in version:
        return False
    if b[0] > 0:
        return v[0] == b[0] and v >= b
    return v[:2] == b[:2] and v >= b


def npm_component(pkg):
    name = pkg.split('/', 1)[1]
    if name.startswith('midnight-js'):
        return 'midnightJs', 'version'
    return {'wallet-sdk': ('walletSdk', 'version'),
            'compact-runtime': ('compact', 'runtime'),
            'ledger-v9': ('ledgerJs', 'version'),
            'onchain-runtime-v4': ('onchainRuntime', 'version')}.get(name, (None, None))


def compare(row, observations):
    """Return mismatch strings of the workspace observations against one row."""
    out = []
    for comp, field, value, where in observations:
        if comp == 'cargo':
            out.append(f'{where}: {value}')
        elif comp == 'ledger':
            want = (row.get('ledger') or {}).get(field)
            if value != want:
                out.append(f'ledger.{field}: workspace {value} != pins {want} ({where})')
        elif comp in ('circuits', 'zkStdlib'):
            want = (row.get(comp) or {}).get('version')
            if value != want:
                out.append(f'{comp}.version: workspace {value} != pins {want} ({where})')
        elif comp == 'proofs':
            req = (row.get('proofs') or {}).get('versionReq')
            if not req or not _caret_ok(value, req):
                out.append(f'proofs.versionReq: workspace {value} does not satisfy pins {req} ({where})')
        elif comp == 'zkir':
            item = row.get('zkir') or {}
            want = f"{item.get('crate')} {item.get('version')}"
            if value != want:
                out.append(f'zkir: workspace {value} != pins {want} ({where})')
        elif comp == 'proofServer':
            accepted = (row.get('proofServer') or {}).get('accepted') or []
            if value not in accepted:
                out.append(f'proofServer: workspace asserts {value}, pins accept {accepted} ({where})')
        elif comp == 'npm':
            target, key = npm_component(field)
            if target is None:
                continue
            want = (row.get(target) or {}).get(key)
            if value != want:
                out.append(f'{target}.{key}: workspace {field} {value} != pins {want} ({where})')
    return out


def run(root=ROOT, network=None, sdk_root=None, out=sys.stdout):
    root = Path(root).resolve()
    path = root / PINS
    try:
        pins = json.loads(path.read_text())
    except (OSError, ValueError) as exc:
        print(f'F15: cannot read {PINS}: {exc}', file=out)
        return 1
    errors = check_structure(pins, root)
    target = network or pins.get('workspaceTarget')
    networks = pins.get('networks') or {}
    if network and network not in networks:
        errors.append(f'--network {network!r} names no row')
    for error in errors:
        print(f'F15: {PINS}: {error}', file=out)
    if errors:
        print(f'F15 FAIL: {len(errors)} structural error(s)', file=out)
        return 1
    observations, notes = observe(root, sdk_root)
    for note in notes:
        print(f'F15 NOTE: {note}', file=out)
    matches = [name for name in NETWORKS if not compare(networks[name], observations)]
    print(f"F15 workspace matches: {', '.join(matches) if matches else 'no row'}", file=out)
    mismatches = compare(networks[target], observations)
    for item in mismatches:
        print(f'F15: {target}: {item}', file=out)
    if mismatches:
        print(f'F15 FAIL: workspace pins differ from the {target!r} row in {len(mismatches)} place(s)', file=out)
        return 1
    print(f'F15 PASS: workspace pins match the {target!r} row ({len(observations)} pins compared)', file=out)
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    parser.add_argument('--root', default=str(ROOT))
    parser.add_argument('--network', choices=NETWORKS, help='compare with this row instead of workspaceTarget')
    parser.add_argument('--sdk-root', default=os.environ.get('MORIARTY_SDK_ROOT'),
                        help='SDK root whose package.json and package-lock.json are compared '
                             f'(default: the committed manifest in {SDK_DIR}/)')
    args = parser.parse_args(argv)
    return run(args.root, args.network, args.sdk_root)


if __name__ == '__main__':
    sys.exit(main())
