"""Read-only audit of ordered parts, complete outer members and original source archives."""
import hashlib
import io
import json
import tarfile
from pathlib import Path

HERE = Path(__file__).resolve().parent


class Parts(io.RawIOBase):
    def __init__(self, entries):
        self.entries = iter(entries)
        self.current = None

    def readable(self):
        return True

    def readinto(self, buffer):
        while True:
            if self.current is None:
                entry = next(self.entries, None)
                if entry is None:
                    return 0
                self.current = (HERE / entry['path']).open('rb')
            count = self.current.readinto(buffer)
            if count:
                return count
            self.current.close()
            self.current = None

    def close(self):
        if self.current:
            self.current.close()
        super().close()


def main():
    manifest = json.loads((HERE / 'archive-manifest.json').read_text())
    combined = hashlib.sha256()
    total = 0
    for ordinal, entry in enumerate(manifest['parts']):
        assert entry['path'] == f'original-evidence.tar.gz.part-{ordinal:03d}'
        assert 0 < entry['bytes'] <= 50 * 1024 * 1024
        actual = hashlib.sha256()
        count = 0
        with (HERE / entry['path']).open('rb') as stream:
            while chunk := stream.read(1024 * 1024):
                combined.update(chunk)
                actual.update(chunk)
                count += len(chunk)
        assert count == entry['bytes'] and actual.hexdigest() == entry['sha256']
        total += count
    assert total == manifest['archiveBytes'] and combined.hexdigest() == manifest['archiveSha256']
    expected = {e['path']: e for e in manifest['members']}
    assert len(expected) == manifest['memberCount'] == len(manifest['members'])
    found = set()
    retained = {}
    # Verify original byte membership first. Never extract paths into the filesystem.
    with io.BufferedReader(Parts(manifest['parts'])) as source, tarfile.open(fileobj=source, mode='r|gz') as archive:
        for member in archive:
            assert member.isfile() and member.name in expected and member.name not in found
            found.add(member.name)
            entry = expected[member.name]
            assert member.size == entry['bytes']
            stream = archive.extractfile(member)
            digest = hashlib.sha256()
            keep = member.name.endswith('/input.json') or member.name.endswith('/task2-terminal-index.json')
            chunks = []
            while chunk := stream.read(1024 * 1024):
                digest.update(chunk)
                if keep:
                    chunks.append(chunk)
            assert digest.hexdigest() == entry['sha256']
            if keep:
                retained[member.name] = json.loads(b''.join(chunks))
    assert found == expected.keys()
    index = retained['.superpowers/sdd/a5-factoring-receipts/task2-terminal-index.json']
    for entry in [e for s in index['stages'] for e in s['files']] + index['attachments']:
        assert expected[entry['path']] == entry
    for entry in index['frozenSources']['files']:
        assert expected[entry['path']]['sha256'] == entry['sha256']
    nested_count = 0
    nested_archives = 0
    with io.BufferedReader(Parts(manifest['parts'])) as source, tarfile.open(fileobj=source, mode='r|gz') as archive:
        for member in archive:
            if not member.name.endswith('/source-and-runner.tar.gz'):
                continue
            receipt = retained[member.name.removesuffix('source-and-runner.tar.gz') + 'input.json']
            cwd = receipt['cwd'].rstrip('/') + '/'
            pins = {e['path'][len(cwd):]: e['sha256'] for e in receipt['pins'] if e['path'].startswith(cwd)}
            seen = set()
            with tarfile.open(fileobj=archive.extractfile(member), mode='r|gz') as nested:
                for original in nested:
                    assert original.isfile() and original.name in pins and original.name not in seen
                    seen.add(original.name)
                    assert hashlib.file_digest(nested.extractfile(original), 'sha256').hexdigest() == pins[original.name]
            assert seen == pins.keys()
            nested_count += len(seen)
            nested_archives += 1
    assert nested_archives == 52 and nested_count == 3650
    print(json.dumps(dict(ok=True, scope='Lossless original evidence; no native rerun',
                         memberCount=len(found), memberBytes=sum(e['bytes'] for e in expected.values()),
                         archiveBytes=total, archiveSha256=combined.hexdigest(),
                         partCount=len(manifest['parts']), originalSourceArchives=nested_archives,
                         originalSourceMembers=nested_count), indent=2))


if __name__ == '__main__':
    main()
