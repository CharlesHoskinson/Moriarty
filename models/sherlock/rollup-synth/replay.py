"""Prepare a pinned, independent Fenrir Rust oracle inside the current worktree."""
from pathlib import Path
import argparse
import sys

from common import Error, canonical, digest, load_json, regular, relative


def prepare_oracle(pin, fenrir, output):
    doc = load_json(pin)
    rows = doc.get('files')
    if doc.get('schema') != 'sherlock-b2-oracle-sources/1' or not isinstance(rows, list) or not rows:
        raise Error('invalid oracle source cut')
    if digest(canonical(rows)) != doc.get('content_digest'):
        raise Error('oracle cut digest mismatch')
    fenrir, output = Path(fenrir).resolve(), Path(output)
    payload = {}
    for row in rows:
        relative(row['source'])
        relative(row['path'])
        if row['path'] in payload:
            raise Error('duplicate oracle destination: ' + row['path'])
        raw = regular(fenrir / row['source'])
        if len(raw) != row['bytes'] or digest(raw) != row['sha256']:
            raise Error('stale oracle source: ' + row['source'])
        for old, new in row.get('path_rewrites', []):
            if raw.count(old.encode()) != 1:
                raise Error('oracle dependency path rewrite is not unique')
            raw = raw.replace(old.encode(), new.encode())
        payload[row['path']] = raw
    # Reuse only an exact snapshot. Cargo output belongs in a separate target dir.
    if output.exists():
        if output.is_symlink() or not output.is_dir():
            raise Error('invalid prepared oracle directory')
        actual = {}
        for path in output.rglob('*'):
            if path.is_symlink():
                raise Error('prepared oracle symlink')
            if path.is_file():
                actual[path.relative_to(output).as_posix()] = regular(path)
            elif not path.is_dir():
                raise Error('prepared oracle non-regular member')
        if actual != payload:
            raise Error('prepared oracle differs from pinned sources')
    else:
        output.mkdir(parents=True)
        for name, raw in payload.items():
            path = output / name
            path.parent.mkdir(parents=True, exist_ok=True)
            with path.open('xb') as stream:
                stream.write(raw)
    return output / 'generator/Cargo.toml'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--pins', required=True)
    parser.add_argument('--fenrir-root', required=True)
    parser.add_argument('--out', required=True)
    args = parser.parse_args()
    try:
        print(prepare_oracle(args.pins, args.fenrir_root, args.out))
    except (Error, OSError, KeyError, TypeError, ValueError) as exc:
        print(str(exc), file=sys.stderr)
        return 2
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
