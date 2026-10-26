#!/usr/bin/env python3
"""Reject profiling dependencies and features in the production workspace."""
import argparse
from pathlib import Path
import re
import tomllib

ROOT = Path(__file__).resolve().parents[2]
FEATURE = re.compile(r'(?:^|[/?:])recursion-phase-profile(?:$|[/?:])')


def strings(value):
    if isinstance(value, dict):
        for key, item in value.items():
            yield key
            yield from strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from strings(item)
    elif isinstance(value, str):
        yield value


def check_isolation(root=ROOT):
    root = Path(root).resolve()
    errors = []
    paths = [root / 'Cargo.toml', root / 'Cargo.lock', *sorted(root.glob('crates/*/Cargo.toml'))]
    vendor = root / 'tools/profile/vendor'
    for path in paths:
        name = str(path.relative_to(root))
        try:
            data = tomllib.loads(path.read_text())
        except (OSError, ValueError):
            errors.append(f'{name}: cannot read valid TOML')
            continue
        for value in strings(data):
            normalized = value.replace('\\', '/')
            if FEATURE.search(value):
                errors.append(f'{name}: enables recursion-phase-profile')
            if 'midnight-proofs-0.8.3' in normalized or 'tools/profile/vendor' in normalized:
                errors.append(f'{name}: references profiling vendor')
            if '/' in normalized and not normalized.startswith(('registry+', 'git+', 'https:')):
                resolved = (path.parent / normalized).resolve()
                if resolved == vendor or vendor in resolved.parents:
                    errors.append(f'{name}: resolves to profiling vendor')
        if path.name == 'Cargo.lock':
            for package in data.get('package', []):
                if package.get('name') == 'midnight-proofs' and not package.get('source', '').startswith('registry+'):
                    errors.append(f'{name}: midnight-proofs must have a registry source')
    return sorted(set(errors))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=ROOT)
    args = parser.parse_args(argv)
    errors = check_isolation(args.root)
    print('\n'.join(errors) if errors else 'profile isolation: OK')
    return int(bool(errors))


if __name__ == '__main__':
    raise SystemExit(main())
