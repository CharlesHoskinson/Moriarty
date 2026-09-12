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
for spec in sorted((ROOT / 'openspec/changes').glob('*/specs/*/spec.md')):
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
    if row['package'] == 'SPRINT-PROGRAM':
        require(row['spec'] == 'openspec/changes/bounded-language-completion-sprints/specs/moriarty-sprint-program/spec.md', 'Invalid sprint-program owner')
    else:
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
# Retain source-qualified original task text and status, not just aggregate counts.
task_paths = {}
for sprint in sprints.values():
    for task in re.findall(r'^## (SP[0-9]{2}\.[0-9]+):', (ROOT / sprint['path']).read_text(), re.MULTILINE):
        task_paths[task] = sprint['path']
original_tasks = {}
for path in sorted((ROOT / 'openspec/changes').glob('mc*/tasks.md')):
    package = path.parent.name[:4].upper()
    for checked, task, text in re.findall(r'^- \[([ xX])\] ([A-Za-z0-9]+\.[0-9]+) (.+)$', path.read_text(), re.MULTILINE):
        key = (package, task)
        require(key not in original_tasks, f'Duplicate original task {key}')
        original_tasks[key] = (str(path.relative_to(ROOT)), text, checked.lower() == 'x')
task_map = json.loads((SPRINTS / 'package-task-map.json').read_text())
mapped = {(r['package'], r['taskId']): r for r in task_map['rows']}
require(len(mapped) == len(task_map['rows']), 'Duplicate original task mapping')
require(set(mapped) == set(original_tasks), 'Original task crosswalk differs')
require(task_map['count'] == len(mapped), 'Original task count differs')
for key, row in mapped.items():
    require((row['sourcePath'], row['originalText'], row['originalChecked']) == original_tasks[key], f'Original task text differs: {key}')
    closing = row['primaryClosingTask']
    require(closing in task_paths and set(row['contributingTasks']) <= set(task_paths), f'Unknown closing task: {key}')
    require(row['package'] in sprints[closing.split('.')[0]]['owners'], f'Original task ownership differs: {key}')
    require(row['closingTaskPath'] == task_paths[closing], f'Original task path differs: {key}')

reports = json.loads((SPRINTS / 'report-lessons.json').read_text())
for path, digest in reports['sourceDigests'].items():
    require(hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, f'Stale report source: {path}')
for section in ['lessons', 'modeledRegressions', 'comparativeCases', 'sourceGaps']:
    rows = reports[section]
    require(len({r['id'] for r in rows}) == len(rows), f'Duplicate report identity: {section}')
    for row in rows:
        require(row['tasks'] and set(row['tasks']) <= set(task_paths), f'Unknown report task owner: {row["id"]}')
        require(row['status'] in {'specified-only', 'source-gap'}, f'Report claims unverified completion: {row["id"]}')
        for path in row.get('sources', [row['source']] if 'source' in row else []):
            require(path in reports['sourceDigests'], f'Unpinned report reference: {path}')
        for field in (['positive', 'negative'] if section in {'lessons', 'modeledRegressions'} else ['acceptance']):
            require(row.get(field, '').strip(), f'Missing report acceptance: {row["id"]}')
require({r['id'] for r in reports['lessons']} == {f'LR{i:02}' for i in range(1, 19)}, 'Report lesson identities differ')
for sprint in sprints.values():
    wanted = {r['id'] for r in reports['lessons'] if any(t.startswith(sprint['id'] + '.') for t in r['tasks'])}
    require(set(sprint['reportLessons']) == wanted, f'Sprint report coverage differs: {sprint["id"]}')
expected_tests = {f'TX{i:02}' for i in range(1, 13)} | {f'VX{i:02}' for i in range(1, 7)}
require({r['id'] for r in reports['modeledRegressions']} == expected_tests, 'Report regression identities differ')
for row in reports['modeledRegressions']:
    lines = [line for line in (ROOT / row['source']).read_text().splitlines() if re.match(r'\| ' + row['id'] + r'\b', line)]
    require(len(lines) == 1, f'Missing report test source: {row["id"]}')
    cells = [c.strip() for c in lines[0].strip('|').split('|')]
    require([row['title'], row['positive'], row['negative']] == cells[:3], f'Report test expectation differs: {row["id"]}')
atlas_path = 'deliverables/modern-defi-taxonomy-2026-09-08/cases.json'
atlas_cases = {r['id']: r for r in json.loads((ROOT / atlas_path).read_text())['cases']}
require({r['id'] for r in reports['comparativeCases']} == set(atlas_cases), 'Modern case identities differ')
for row in reports['comparativeCases']:
    source = atlas_cases[row['id']]
    require(row['component'] == source['component'] and row['scope'] == source['version_chain_scope'], f'Modern case scope differs: {row["id"]}')
    require(row['implementationClaim'] is False, 'Comparative case claims implementation')
require({r['id'] for r in reports['sourceGaps']} == {'FIN-CAP.2', 'FIN-MGT.3', 'FIN-RSK.3'}, 'Uncovered taxonomy leaves differ')

legacy = json.loads((SPRINTS / 'legacy-release-gates.json').read_text())
for path, digest in legacy['sourceSha256'].items():
    require(hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, f'Stale legacy source: {path}')
legacy_record = json.loads((ROOT / 'evidence/execution/moriarty-v1.3-program.json').read_text())
original_gates = {r['id']: r['predicate'] for r in legacy_record['release_gates']}
gate_rows = {r['id']: r for r in legacy['gates']}
require(len(gate_rows) == len(legacy['gates']) == 24 and set(gate_rows) == set(original_gates), 'Legacy gate identities differ')
for id, row in gate_rows.items():
    require(row['originalPredicate'] == original_gates[id], f'Legacy gate text differs: {id}')
    xml = (ROOT / row['sourcePath']).read_text()
    require(f'<gate id="{id}">{row["originalPredicate"]}</gate>' in xml, f'Legacy gate XML differs: {id}')
    require(row['primaryClosingTask'] in task_paths and set(row['evidenceOwners']) <= set(task_paths), f'Legacy gate owner differs: {id}')

# PCD integration: the ledger-anchored core must not wait on certificates, and release keeps them.
pcd = json.loads((SPRINTS / 'pcd-integration.json').read_text())
for path, digest in pcd['sourceDigests'].items():
    require(hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == digest, f'Stale PCD integration source: {path}')
def stage_ancestors(id):
    found, pending = set(), list(stage_records[id]['requires'])
    while pending:
        dep = pending.pop()
        if dep not in found:
            found.add(dep)
            pending.extend(stage_records[dep]['requires'])
    return found
require(not {'f0a', 'f1-fixtures', 'f1', 'f2'} & stage_ancestors('mandatory'), 'PCD core depends on certificate stage')
require('f2' in stage_ancestors('release'), 'PCD release lost certificate stage')
pcd_requirements = {name for spec, name in actual if spec.startswith('openspec/changes/pcd-ledger-anchored-acceptance/specs/')}
for row in pcd['stageMapping'] + pcd['experiments']:
    require(row['stage'] in stage_records and row['tasks'] and set(row['tasks']) <= set(task_paths), f'Unknown PCD crosswalk reference: {row["id"]}')
for row in pcd['supersessions']:
    require((row['spec'], row['requirement']) in actual and set(row['governing']) <= pcd_requirements, f'Unknown PCD crosswalk reference: {row["requirement"]}')
for row in pcd['lockedTasks']:
    require((row['package'], row['taskId']) in mapped and set(row['governing']) <= pcd_requirements, f'Unknown PCD crosswalk reference: {row["package"]} {row["taskId"]}')

print(json.dumps({'status': 'pass', 'scope': 'planning structure only; no implementation or dispatch acceptance', 'sprints': len(sprints), 'requirements': len(actual), 'stages': len(stages), 'originalTasks': len(mapped), 'sprintTasks': len(task_paths), 'reportLessons': len(reports['lessons']), 'modeledRegressions': len(expected_tests), 'comparativeCases': len(atlas_cases), 'legacyGates': len(gate_rows)}, indent=2))
