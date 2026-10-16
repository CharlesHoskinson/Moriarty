#!/usr/bin/env python3
"""Copy the explicit public bounded-source attempt bytes; never execute a capture."""
import argparse
import gzip
import hashlib
import io
import json
from pathlib import Path
import tarfile


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def encoded(value):
    return (json.dumps(value, indent=2, sort_keys=True) + '\n').encode()


def archive_bytes(members):
    buffer = io.BytesIO()
    with gzip.GzipFile(fileobj=buffer, mode='wb', filename='', mtime=0) as compressed:
        with tarfile.open(fileobj=compressed, mode='w', format=tarfile.USTAR_FORMAT) as archive:
            for name, raw in sorted(members.items()):
                entry = tarfile.TarInfo(name)
                entry.size, entry.mode, entry.mtime = len(raw), 0o644, 0
                entry.uid = entry.gid = 0
                entry.uname = entry.gname = ''
                archive.addfile(entry, io.BytesIO(raw))
    return buffer.getvalue()


def package(checkout, runtime, destination):
    attempt = runtime / 'bounded-source-01'
    receipt = json.loads((attempt / 'result.json').read_bytes())
    manifest_raw = (attempt / 'manifest.json').read_bytes()
    manifest = json.loads(manifest_raw)
    assert receipt['terminal_state'] == 'success'
    assert all(step['exit'] == 0 and not step['timed_out'] for step in receipt['steps'])
    assert receipt['counts'] == dict(programs=2, shared_pending_states=68, source_occurrences=17,
                                    prefix_runs=70, records=1259)
    assert all(receipt[name] for name in ('dependencies_unchanged', 'tools_unchanged',
                                        'source_unchanged', 'snapshot_unchanged'))
    members, bindings = {}, []
    for entry in manifest:
        relative = entry['path']
        assert not Path(relative).is_absolute() and '..' not in Path(relative).parts
        raw = (attempt / 'source' / relative).read_bytes()
        assert len(raw) == entry['bytes'] and sha(raw) == entry['sha256'], relative
        members['bounded-source-01/source/' + relative] = raw
        current = checkout / relative
        current_sha = sha(current.read_bytes()) if current.is_file() else None
        bindings.append(dict(path=relative, bytes=len(raw), snapshot_sha256=sha(raw),
                             current_checkout_sha256=current_sha, current_matches_snapshot=current_sha == sha(raw)))
    selected = ['result.json', 'manifest.json', 'artifacts.json', 'bounded_harness.rs',
                'dependencies-before.json', 'dependencies-after.json',
                'toolchain-before.json', 'toolchain-after.json', '01-core.d', '02-tests.d', '05-producer.d']
    selected += [step['name'] + suffix for step in receipt['steps'] for suffix in ('.stdout', '.stderr')]
    selected = sorted(set(selected))
    artifacts = {row['path']: row for row in json.loads((attempt / 'artifacts.json').read_bytes())}
    for relative in selected:
        raw = (attempt / relative).read_bytes()
        if relative in artifacts:
            assert sha(raw) == artifacts[relative]['sha256'], relative
        members['bounded-source-01/' + relative] = raw
    for step in receipt['steps']:
        for stream in ('stdout', 'stderr'):
            info = step[stream]
            raw = (attempt / Path(info['path']).name).read_bytes()
            assert len(raw) == info['bytes'] and sha(raw) == info['sha256']
    observations = json.loads((attempt / '06-run-source-producer.stdout').read_bytes())
    comparison = json.loads((attempt / '07-compare-independent-oracles.stdout').read_bytes())
    assert comparison['status'] == 'pass' and comparison['first_difference'] is None
    assert len(observations['programs']) == 2 and len(observations['runs']) == 70
    assert sum(len(run['records']) for run in observations['runs']) == 1259
    assert sum(record['charge'] for run in observations['runs'] for record in run['records']) == 1189
    assert len(receipt['test_names']) == 14
    listing = (attempt / '03-list-exact-tests.stdout').read_text()
    names = [line.removesuffix(': test') for line in listing.splitlines() if line.endswith(': test')]
    assert sorted(names) == sorted(receipt['test_names'])
    tests = (attempt / '04-run-exact-tests.stdout').read_text()
    assert '14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;' in tests
    for folder in ('bounded-source-preflight-opus', 'bounded-source-preflight-muse'):
        # Reports and launch identity only: provider stdout and prompts are excluded.
        for name in ('REPORT.md', 'LAUNCH.json', 'CANDIDATE.json'):
            members[folder + '/' + name] = (runtime / folder / name).read_bytes()
    destination.mkdir(parents=True, exist_ok=True)
    for name, raw in sorted(members.items()):
        # Keep the complete producer output and source snapshot in the archive;
        # small original receipts/logs remain directly reviewable.
        if '/source/' not in name and not name.endswith('/06-run-source-producer.stdout'):
            path = destination / 'records' / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(raw)
    archive = archive_bytes(members)
    assert archive == archive_bytes(members)
    with tarfile.open(fileobj=io.BytesIO(archive), mode='r:gz') as opened:
        assert opened.getnames() == sorted(members)
        for entry in opened:
            assert opened.extractfile(entry).read() == members[entry.name]
    (destination / 'public-evidence.tar.gz').write_bytes(archive)
    (destination / 'ARCHIVE-MEMBERS.json').write_bytes(encoded(dict(
        archive=dict(path='public-evidence.tar.gz', bytes=len(archive), sha256=sha(archive)),
        members=[dict(path=name, bytes=len(raw), sha256=sha(raw)) for name, raw in sorted(members.items())])))
    (destination / 'SOURCE-BINDINGS.json').write_bytes(encoded(dict(
        source='bounded-source-01/source', manifest_sha256=sha(manifest_raw), files=bindings,
        note='Exact immutable attempt snapshot; broader than rustc dep-info. Changed checkout bytes never replace captured bytes.')))
    print(json.dumps(dict(archive_bytes=len(archive), archive_sha256=sha(archive),
                         member_count=len(members), source_count=len(bindings),
                         counts=receipt['counts'], verification='all selected original digests and archive members match')))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('checkout', type=Path)
    parser.add_argument('runtime', type=Path)
    args = parser.parse_args()
    package(args.checkout.resolve(), args.runtime.resolve(), Path(__file__).resolve().parent)


if __name__ == '__main__':
    main()
