"""One reviewed local update, preserving the active sessions' cached packages."""
from pathlib import Path
import hashlib
import json
import os
import shutil
import subprocess
import tempfile

OUT = Path(__file__).resolve().parent
CANDIDATE = Path('/home/charl/Moriarty-plugin-compatibility-20260911')
REPO = Path('/home/charl/Moriarty')
PREFIX = Path('plugins/moriarty-dev')
manifest = json.loads((OUT / 'candidate-02.json').read_text())
baseline = json.loads((OUT / 'baseline-manifest.json').read_text())
preserved = json.loads((OUT / 'preserved-installations.json').read_text())
source = Path(preserved['marketplaceSource']['path'])


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(root, hashes):
    for name, expected in hashes.items():
        path = root / name
        if not path.is_file() or digest(path) != expected:
            raise RuntimeError('Changed or missing file: ' + str(path))


# The caller runs this only after both independent final reviews approve.
verify(CANDIDATE, manifest)
verify(REPO, baseline)
verify(source, preserved['marketplaceSource']['hashes'])
for root, record in preserved['roots'].items():
    verify(Path(root), record['hashes'])
for root, prior in ((REPO / PREFIX, {str(Path(name).relative_to(PREFIX)): value for name, value in baseline.items()}),
                    (source, preserved['marketplaceSource']['hashes'])):
    for name, expected in manifest.items():
        relative = Path(name).relative_to(PREFIX)
        path = root / relative
        if str(relative) not in prior and path.exists() and digest(path) != expected:
            raise RuntimeError('Unrelated new file conflicts with candidate: ' + str(path))

# New modules are installed before the existing entrypoints import them.
ordered = sorted(manifest, key=lambda name: (name in baseline, name))
for root in (REPO / PREFIX, source):
    for name in ordered:
        relative = Path(name).relative_to(PREFIX)
        target = root / relative
        if target.is_file() and digest(target) == manifest[name]:
            continue
        target.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.NamedTemporaryFile(dir=target.parent, prefix='.reviewed-', delete=False) as file:
            temporary = Path(file.name)
            file.write((CANDIDATE / name).read_bytes())
        shutil.copymode(CANDIDATE / name, temporary)
        os.replace(temporary, target)

try:
    result = subprocess.run(['codex', 'plugin', 'add', 'moriarty-dev@personal', '--json'], capture_output=True, text=True)
    (OUT / 'install-result.json').write_text(json.dumps({'exitCode': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr}, indent=2) + '\n')
finally:
    for root, record in preserved['roots'].items():
        path = Path(root)
        if not path.exists():
            shutil.copytree(record['backup'], path, symlinks=True)
        verify(path, record['hashes'])
if result.returncode:
    raise SystemExit(result.returncode)
version = json.loads((source / '.codex-plugin/plugin.json').read_text())['version']
installed = Path('/home/charl/.codex/plugins/cache/personal/moriarty-dev') / version
verify(installed, {str(Path(name).relative_to(PREFIX)): expected for name, expected in manifest.items()})
verify(REPO, manifest)
print('Reviewed candidate installed:', installed)
print('All previously retained cache roots remain byte-identical.')
