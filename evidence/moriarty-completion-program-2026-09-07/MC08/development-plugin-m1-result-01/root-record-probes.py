"""Independent admission mutations using the author's declared supported fixture.

The fixture grants no production authority. Its purpose is to check whether the
reader rejects contradictions even under its own accepted input contract.
Run against the completed, frozen candidate only.
"""
import hashlib
import json
import sys
from pathlib import Path

worktree = Path('/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok')
sys.path.insert(0, str(worktree / 'plugins/moriarty-dev/tests'))
from test_records import GenuineRegisters, verified_history
from moriarty_dev.records import load_snapshot, read_actions
from moriarty_dev.policy import assess

results = []
def run(name, mutate):
    fixture = GenuineRegisters()
    try:
        fixture.setUp()
        fixture._install_admitted_sp05()
        mutate(fixture)
        snapshot = load_snapshot(str(fixture.root), 'sp05-ledger-implement', history_reader=verified_history())
        decision = assess(snapshot, read_actions(str(fixture.root))[0])
        results.append({'name': name, 'passed': decision['allow'] is False,
                        'snapshot': snapshot, 'decision': decision})
    finally:
        fixture.doCleanups()

def resource(fixture, value):
    (fixture.root / fixture.resource).write_text(json.dumps(value))

run('empty resource object cannot authorize implementation', lambda f: resource(f, {}))
run('negative allocation cannot authorize implementation', lambda f: resource(f, {'allocationSeconds': -1}))
run('changed candidate bytes invalidate admission', lambda f: f.candidate_path.write_text('different implementation\n'))
run('missing candidate bytes invalidate admission', lambda f: f.candidate_path.unlink())

print(json.dumps({'passed': sum(x['passed'] for x in results), 'total': len(results),
                  'results': results}, indent=2))
raise SystemExit(0 if all(x['passed'] for x in results) else 1)
