"""Check recorded pre/post-upgrade host observations, not synthetic hook calls."""
from pathlib import Path
import json

out = Path(__file__).resolve().parent
before = json.loads((out / 'host-before-upgrade-result.json').read_text())
after = json.loads((out / 'host-after-upgrade-result.json').read_text())
assert before['turnStatus'] == after['turnStatus'] == 'completed'
assert before['markerExists'] is after['markerExists'] is False
expected_version = json.loads((Path('/home/charl/Moriarty/plugins/moriarty-dev/.codex-plugin/plugin.json')).read_text())['version']
counts = {}
for name, expected_blocks in [('host-before-upgrade', 2), ('host-after-upgrade', 1)]:
    events = json.loads((out / (name + '-events.json')).read_text())
    hooks = [e['params']['run'] for e in events if e.get('method') == 'hook/completed' and 'moriarty-dev' in e.get('params', {}).get('run', {}).get('sourcePath', '')]
    assert {'sessionStart', 'preToolUse', 'postToolUse', 'stop'} <= {r['eventName'] for r in hooks}
    assert all(r['status'] in ('completed', 'blocked') for r in hooks)
    assert sum(r['status'] == 'blocked' for r in hooks) == expected_blocks
    assert all(not any(entry.get('kind') == 'error' for entry in r['entries']) for r in hooks)
    if name == 'host-after-upgrade':
        assert all('/' + expected_version + '/' in r['sourcePath'] for r in hooks)
        assert any(r['eventName'] == 'stop' and r['status'] == 'completed' for r in hooks)
    counts[name] = {'completed': sum(r['status'] == 'completed' for r in hooks), 'blocked': expected_blocks}
assert len(before['commands']) == 1
assert len(after['commands']) == 2
assert all(item['exitCode'] == 0 for item in after['commands'])
assert any(item['aggregatedOutput'] == 'compatibility-denied-marker' for item in after['commands'])
print(json.dumps({'version': expected_version, 'hookCounts': counts, 'falseDenialFixed': True, 'registeredRawDispatchStillBlocked': True, 'markerAbsent': True}, indent=2))
