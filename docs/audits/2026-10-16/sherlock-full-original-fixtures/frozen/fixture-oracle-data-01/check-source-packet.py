"""Source-static assertions over fixed literal oracle artifacts, no execution."""
from pathlib import Path
import hashlib
import json

R = Path(__file__).parent
W = Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
digest = lambda b: hashlib.sha256(b).hexdigest()
pins = json.loads((R/'input-pins.json').read_text())
for pin in pins:
    raw = (W/pin['path']).read_bytes()
    assert len(raw) == pin['bytes']
    assert digest(raw) == pin['sha256'], pin['path']

old_prefixes = [json.loads((W/'models/sherlock/oracles'/name).read_text())
               for name in ['normal-prefixes.json','extra-normal-prefixes.json']]
old_count = sum(len(t['ordinary_fuel_prefixes'])
                for pack in old_prefixes for t in pack['traces'])
assert old_count == 216

program_pack = json.loads((R/'fixture-normal-programs.json').read_text())
prefix_pack = json.loads((R/'fixture-normal-prefixes.json').read_text())
assert prefix_pack['representation'] == old_prefixes[0]['representation']
assert prefix_pack['profile'] == program_pack['profile'] == 'sherlock-k-a/1'
assert prefix_pack['baseline_commit'] == program_pack['baseline_commit'] == 'c9f7cf6a712c7e6069cdc7a3ecdd86fb79817429'
programs = program_pack['programs']
traces = prefix_pack['traces']
assert [p['case_id'] for p in programs] == ['F03','F04','F05']
assert [t['case_id'] for t in traces] == ['F03','F04','F05']
main_lines = (W/'models/sherlock/k/main.k').read_text().splitlines()
keys = ['case_id','code_k','entry_k','initial_arguments','stack_limit','exact_completion_fuel']
state_keys = {'charged_steps','remaining_fuel','depth','control_k','environment_k','frames_top_first_k','via_rule_start_line'}
expected_counts = {'F03':93,'F04':60,'F05':13}
expected_depths = {'F03':11,'F04':5,'F05':2}

def balanced(value):
    """Check parenthesis balance in literal strings; no parse or execution."""
    depth = 0
    for char in value:
        if char == '(':
            depth += 1
        elif char == ')':
            depth -= 1
            assert depth >= 0, value
    assert depth == 0, value

for p,t in zip(programs,traces):
    cid = p['case_id']
    count = expected_counts[cid]
    canonical_input = json.dumps({key:p[key] for key in keys},sort_keys=True,separators=(',',':')).encode()
    assert p['input_sha256'] == t['input_sha256'] == digest(canonical_input)
    assert p['exact_completion_fuel'] == count
    assert p['expected_maximum_frame_depth'] == expected_depths[cid]
    assert p['stack_limit'] == 64
    fixture = (W/p['original_fixture']['path']).read_text()
    literal_run = f"uiRun({p['code_k']}, {p['entry_k']}, 1000, 64)"
    assert ''.join(fixture.split()) == ''.join(literal_run.split())
    assert p['original_fixture']['sha256'] == digest(fixture.encode())
    balanced(p['code_k'])
    balanced(p['entry_k'])
    assert t['fixed_state_fields']['code_ref'] == dict(file='fixture-normal-programs.json',case_id=cid,input_sha256=p['input_sha256'])
    assert t['fixed_state_fields']['initial_fuel'] == count
    assert t['fixed_state_fields']['stack_limit'] == 64
    assert t['fixed_state_fields']['output_k'] == 'uiPending()'
    assert len(t['states']) == count+1
    assert max(s['depth'] for s in t['states']) == expected_depths[cid]
    for i,s in enumerate(t['states']):
        assert set(s) == state_keys
        assert s['charged_steps'] == i
        assert s['remaining_fuel'] == count-i
        assert s['depth'] == len(s['frames_top_first_k'])
        assert s['depth'] <= 64
        assert s['via_rule_start_line'] is None if i == 0 else s['via_rule_start_line'] is not None
        if i:
            assert main_lines[s['via_rule_start_line']-1].strip().startswith('rule <k>')
        for term in [s['control_k'],s['environment_k']]+s['frames_top_first_k']:
            balanced(term)
    assert t['states'][0]['control_k'] == f"uiEval({p['entry_k']})"
    assert t['states'][-1]['control_k'] == f"uiRet({p['expected_value_k']})"
    assert t['states'][-1]['environment_k'] == '.Map'
    assert t['states'][-1]['frames_top_first_k'] == []
    assert t['startup'] == dict(rule_start_line=74,charge=0,post_state_index=0)
    done = f"uiDone({p['expected_value_k']})"
    assert t['publication'] == dict(pre_state_index=count,rule_start_line=80,charge=0,
        post_overrides=dict(control_k='.K',output_k=done))
    assert len(t['ordinary_fuel_prefixes']) == count+2
    for i,row in enumerate(t['ordinary_fuel_prefixes']):
        assert row == ([i,i,0,'uiExhausted()',84] if i<count else [i,count,i-count,done,80])
    o = t['original_fixture_run']
    assert o['initial_configuration']['control_k'] == literal_run
    assert o['initial_configuration']['remaining_fuel'] == 0
    assert o['initial_configuration']['initial_fuel'] == 0
    assert o['initial_configuration']['stack_limit'] == 0
    assert o['fixed_state_overrides'] == dict(initial_fuel=1000,stack_limit=64)
    assert o['state_fuel_overlays'] == [[i,1000-i] for i in range(count+1)]
    assert o['publication'] == t['publication']
    terminal = o['terminal']
    assert terminal['remaining_fuel'] == 1000-count
    assert terminal['initial_fuel'] == 1000
    assert terminal['charged_steps'] == count
    assert terminal['output_k'] == done
    assert terminal['code_ref'] == t['fixed_state_fields']['code_ref']
    assert terminal['environment_k'] == '.Map'
    assert terminal['frames_top_first_k'] == []
    assert terminal['depth'] == 0
    assert terminal['stack_limit'] == 64
    assert terminal['control_k'] == '.K'

receipt = dict(
    status='source-static assertions passed; hand-derived specified-only/unexecuted/unreviewed',
    command='python3 '+str(R/'check-source-packet.py'),
    assertion_scope='JSON/data structure, existing literal representation, original-fixture whitespace-only source equality, source pins, input digests, exact row counts, balanced text, complete state fields, rule start-line locators, full fuel overlays and terminal snapshots. No semantic execution or producer comparison.',
    preserved_existing_ordinary_prefix_count=old_count,
    new_cases=len(programs),
    new_state_count=sum(len(t['states']) for t in traces),
    new_ordinary_prefix_count=sum(len(t['ordinary_fuel_prefixes']) for t in traces),
    original_fixture_state_overlay_count=sum(len(t['original_fixture_run']['state_fuel_overlays']) for t in traces),
    specified_completion_work=expected_counts,
    specified_maximum_frame_depth=expected_depths,
    subject_execution_performed=False,
    inputs_unchanged=True)
(R/'source-static-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt,indent=2))
