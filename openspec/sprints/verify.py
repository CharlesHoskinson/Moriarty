#!/usr/bin/env python3
"""Read-only validation of sprint navigation and retained OpenSpec coverage."""
import json
import csv
import hashlib
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SPRINTS = ROOT / 'openspec/sprints'

def require(condition, message):
    if not condition:
        raise SystemExit(message)

schedule = json.loads((SPRINTS / 'sprints.json').read_text())
coverage = json.loads((SPRINTS / 'coverage.json').read_text())
program = json.loads((ROOT / schedule['programRegister']).read_text())
sprints = {s['id']: s for s in schedule['sprints']}
require(len(sprints) == len(schedule['sprints']), 'Duplicate sprint identity')
require(not schedule['dispatchEnabled'] and not schedule['resourceAllocationGranted'], 'Plan grants runtime authority')
packages = {p['id']: p for p in program['packages']}
stages = {s['id'] for s in program['reportReconciliation']['stageAdmission']['stages']}
visited, active = set(), set()

def visit(id):
    require(id in sprints, f'Unknown sprint {id}')
    require(id not in active, f'Cyclic dependency at {id}')
    if id in visited:
        return
    active.add(id)
    for dep in sprints[id]['completionRequires']:
        visit(dep)
    active.remove(id)
    visited.add(id)

for id, sprint in sprints.items():
    visit(id)
    require((ROOT / sprint['path']).is_file(), f'Missing plan {id}')
    require(set(sprint['owners']) <= set(packages), f'Unknown package owner {id}')
    require(set(sprint['stages']) <= stages, f'Unknown stage {id}')
    text = (ROOT / sprint['path']).read_text()
    require('- [ ]' in text and '## Exit gate' in text, f'Incomplete contract {id}')
    require('- [x]' not in text.lower(), f'Unverified sprint task marked complete {id}')

# Stage admission must also remain acyclic; sprint-level edges alone are insufficient.
stage_records = {s['id']: s for s in program['reportReconciliation']['stageAdmission']['stages']}
require(len(stage_records) == len(program['reportReconciliation']['stageAdmission']['stages']), 'Duplicate stage identity')
stage_visited, stage_active = set(), set()
def visit_stage(id):
    require(id in stage_records, f'Unknown stage prerequisite {id}')
    require(id not in stage_active, f'Cyclic stage dependency at {id}')
    if id in stage_visited:
        return
    stage_active.add(id)
    for dep in stage_records[id]['requires']:
        visit_stage(dep)
    stage_active.remove(id)
    stage_visited.add(id)
for id in stage_records:
    visit_stage(id)
for sprint in sprints.values():
    require({g['stage'] for g in sprint['entryGates']} == set(sprint['stages']), 'Missing task entry gate')
    for gate in sprint['entryGates']:
        source = stage_records[gate['stage']]
        require(gate['requires'] == source['requires'], 'Task admission differs from RP prerequisite graph')
        require(gate['campaignOwner'] == source['owners'], 'Task campaign ownership differs from RP record')
        require(gate['tasks'] and all(t.startswith(sprint['id'] + '.') for t in gate['tasks']), 'Task gate belongs to another sprint')
    declared_tasks = set(re.findall(r'^## (SP[0-9]{2}\.[0-9]+):', (ROOT / sprint['path']).read_text(), re.MULTILINE))
    gated_tasks = {t for gate in sprint['entryGates'] for t in gate['tasks']}
    require(declared_tasks == gated_tasks, f'Missing or unknown task admission in {sprint["id"]}: {declared_tasks ^ gated_tasks}')
subset_gates = {g['stage']: g['tasks'] for g in sprints['SP01']['entryGates']}
require(set(subset_gates['rp01-mc02']).isdisjoint(subset_gates['rp01-full']), 'MC02 subset collapsed into full RP01 task')
require(set(subset_gates['rp01-mc03']).isdisjoint(subset_gates['rp01-full']), 'MC03 subset collapsed into full RP01 task')
require('native-path-freeze' in stage_records['f0a']['requires'] and 'native-path-freeze' in stage_records['f2']['requires'], 'Native writer path freeze missing')
for id in ['SP02', 'SP03', 'SP07', 'SP08']:
    require(sprints[id]['stages'], f'Missing successor implementation admission for {id}')

actual = set()
for spec in sorted((ROOT / 'openspec/changes').glob('mc*/specs/*/spec.md')):
    for line in spec.read_text().splitlines():
        if line.startswith('### Requirement: '):
            actual.add((str(spec.relative_to(ROOT)), line.removeprefix('### Requirement: ')))
expected = {(row['spec'], row['requirement']) for row in coverage['requirements']}
require(len(expected) == len(coverage['requirements']), 'Duplicate requirement crosswalk')
require(actual == expected, f'Requirement crosswalk differs: {actual ^ expected}')
for section in ['requirements', 'outcomes', 'targetFamilies']:
    for row in coverage[section]:
        require(row['sprints'] and set(row['sprints']) <= set(sprints), f'Invalid coverage owner: {row}')
for row in coverage['requirements']:
    require(row['primaryClosingSprint'] in row['sprints'], 'Missing primary closing sprint')
    require(set(row['contributingSprints']) == set(row['sprints']) - {row['primaryClosingSprint']}, 'Contributor crosswalk differs')
    require(all(row['package'] in sprints[id]['owners'] for id in row['sprints']), 'Requirement ownership mismatch')
for p in program['packages']:
    require(p['sprints'] == [s['id'] for s in schedule['sprints'] if p['id'] in s['owners']], 'Program/sprint register mismatch')
for path in coverage['sourceInventories']:
    require((ROOT / path).is_file(), f'Missing source inventory {path}')
require({s for sprint in sprints.values() for s in sprint['stages']} == stages, 'An RP stage has no sprint owner')
families = {row['id']: row for row in coverage['targetFamilies']}
required_families = {'ACTUS-fixtures': 277, 'ACTUS-executable-types': 18, 'ACTUS-taxonomy-dispositions': 32, 'original-DeFi-rows': 72, 'DeFi-actions': 24, 'heldouts': 3, 'intent-cases': 8, 'DeFi-regressions': 8, 'composition': 5, 'additional-report-products': 12}
require(set(families) == set(required_families), 'Missing or extra target family requires explicit plan reconciliation')
for id, count in required_families.items():
    row = families[id]
    require(row.get('count', len(row.get('ids', []))) == count, f'Target denominator changed: {id}')
    if 'ids' in row:
        require(len(set(row['ids'])) == count, f'Duplicate target identity: {id}')
for path, digest in coverage['sourceDigests'].items():
    require(hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, f'Stale planning source: {path}')
with (ROOT / 'deliverables/defi-language-design-2026-09-07/action-targets.csv').open() as handle:
    actions = {row['target_id'] for row in csv.DictReader(handle)}
require(actions == set(families['DeFi-actions']['ids']), 'Action matrix mismatch')
with (ROOT / 'evidence/moriarty-design-sprint-2026-09-06/actus-32-requirements.csv').open() as handle:
    actus = list(csv.DictReader(handle))
require(len(actus) == 32 and sum(int(row['fixture_count']) for row in actus) == 277, 'ACTUS inventory mismatch')
require(sum(int(row['fixture_count']) > 0 for row in actus) == 18, 'ACTUS executable type mismatch')
with (ROOT / 'evidence/moriarty-design-sprint-2026-09-06/defi-72-requirements.csv').open() as handle:
    require(len(list(csv.DictReader(handle))) == 72, 'Original DeFi inventory mismatch')
print(json.dumps({'status': 'pass', 'scope': 'planning structure only; no implementation or dispatch acceptance', 'sprints': len(sprints), 'requirements': len(actual), 'stages': len(stages)}, indent=2))
