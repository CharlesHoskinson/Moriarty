"""One bounded migration, with legacy restoration before the host tool returns."""
from pathlib import Path
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile

ROOT = Path('/home/charl/Moriarty')
OUT = ROOT / 'deliverables/hook-input-repair-2026-09-12'
SOURCE = ROOT / 'plugins/moriarty-dev'
MARKET = Path('/home/charl/plugins/moriarty-dev')
sys.path.insert(0, str(SOURCE / 'scripts'))
from moriarty_dev.deployment import files, retained_roots, upgrade


def verdict(path, grok=False):
    receipt = json.loads(path.read_text())
    if receipt['exit_code'] != 0:
        raise ValueError('audit did not complete: ' + str(path))
    if grok:
        event = json.loads(receipt['stdout'])
        if event.get('stopReason') != 'end_turn':
            raise ValueError('Grok audit not terminal')
        text = event['text']
    else:
        events = [json.loads(line) for line in receipt['stdout'].splitlines() if line.strip()]
        if not any(event.get('type') == 'turn.completed' for event in events):
            raise ValueError('Astra audit not terminal')
        text = '\n'.join(event['item']['text'] for event in events
                         if event.get('type') == 'item.completed' and event.get('item', {}).get('type') == 'agent_message')
    decoder = json.JSONDecoder()
    for offset, char in enumerate(text):
        if char != '{':
            continue
        try:
            value, _ = decoder.raw_decode(text[offset:])
        except ValueError:
            continue
        if isinstance(value, dict) and 'verdict' in value:
            return value
    raise ValueError('audit verdict missing')


manifest = json.loads((OUT / 'upgrade-review-manifest-02.json').read_text())
reviews = [verdict(OUT / 'grok-upgrade-audit-02.json', True), verdict(OUT / 'astra-upgrade-audit-03.json')]
for review in reviews:
    if review['verdict'] != 'APPROVED' or review['candidateDigest'] != manifest['candidateDigest']:
        raise ValueError('exact-candidate approval missing')
    if any(row.get('severity') in ('blocking', 'high', 'medium') for row in review.get('findings', [])):
        raise ValueError('unresolved substantive finding')
expected = {row['path']: row['sha256'] for row in manifest['completePackage']}
if files(SOURCE) != expected:
    raise ValueError('source changed after audit freeze')
codex = shutil.which('codex')
catalog = json.loads(subprocess.run([codex, 'plugin', 'list', '--marketplace', 'personal', '--json'],
                    capture_output=True, text=True, check=True, timeout=10).stdout)
matches = [row for key in ('installed', 'available') for row in catalog.get(key, [])
           if row.get('pluginId') == 'moriarty-dev@personal']
if not matches or any(row.get('source', {}).get('source') != 'local' or
                     Path(row['source']['path']).resolve() != MARKET for row in matches):
    raise ValueError('unexpected marketplace mapping')

# Preserve and validate the previously installed source; never overwrite a
# concurrent user's marketplace edit merely because this candidate was reviewed.
prior = json.loads((OUT / 'review-manifest.json').read_text())['files']
tracked = subprocess.run(['git', 'ls-tree', '-r', '--name-only', 'HEAD', '--', 'plugins/moriarty-dev'],
                         cwd=ROOT, capture_output=True, text=True, check=True).stdout.splitlines()
previous = {}
for name in tracked:
    relative = name.removeprefix('plugins/moriarty-dev/')
    previous[relative] = prior.get(name) or hashlib.sha256(subprocess.run(
        ['git', 'show', 'HEAD:' + name], cwd=ROOT, capture_output=True, check=True).stdout).hexdigest()
if files(MARKET) != previous:
    raise ValueError('marketplace source changed since prior reviewed repair; preserve and reconcile it')
private = Path('/home/charl/moriarty-private-records/hook-upgrade-2026-09-12')
backup = Path(tempfile.mkdtemp(prefix='marketplace-before-durable-', dir=private))
shutil.copytree(MARKET, backup / 'source', ignore=shutil.ignore_patterns('__pycache__'))
if files(backup / 'source') != previous:
    raise ValueError('marketplace backup mismatch')
shutil.copytree(SOURCE, MARKET, dirs_exist_ok=True, ignore=shutil.ignore_patterns('__pycache__'))
if files(MARKET) != expected:
    raise ValueError('staged marketplace candidate mismatch')

runtime_home = Path.home() / '.local/share/moriarty-dev'
codex_home = Path(os.environ.get('CODEX_HOME') or Path.home() / '.codex')
extra = [Path(os.environ[key]) for key in ('PLUGIN_ROOT', 'CLAUDE_PLUGIN_ROOT') if os.environ.get(key)]
roots = retained_roots(codex_home, extra)
# This initial migration still has legacy cache-bound sessions. Bound the
# installer child below the host tool's 30-second wait, then restore in upgrade's
# finally block. Future registrations independently survive cache deletion.
report = upgrade(MARKET, codex_home / 'plugins/cache/personal/moriarty-dev', runtime_home / 'backups', runtime_home,
                 ['timeout', '15s', codex, 'plugin', 'add', 'moriarty-dev@personal'], roots)
(OUT / 'durable-install-result.json').write_text(json.dumps(report, indent=2) + '\n')
(OUT / 'durable-source-approval.json').write_text(json.dumps({'candidateDigest':manifest['candidateDigest'],
    'reviews':reviews, 'installedCandidateVerified':report['installedVerified'], 'scope':'Source and installed bytes; host coverage requires separate smoke.'}, indent=2) + '\n')
print(json.dumps({key:report[key] for key in ('runtimeDigest','backup','installerReturnCode','restorationComplete','installedVerified')}))
raise SystemExit(report['installerReturnCode'] or (0 if report['installedVerified'] else 1))
