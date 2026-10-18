"""Normal oracle data equations only. No Rust/subject imports or execution."""
import hashlib
import json
import pathlib
import re
import time

ROOT = pathlib.Path(__file__).resolve().parent

def load(name):
    def pairs(items):
        result = {}
        for key, value in items:
            assert key not in result
            result[key] = value
        return result
    return json.loads((ROOT / name).read_text(), object_pairs_hook=pairs,
        parse_float=lambda _: (_ for _ in ()).throw(AssertionError('unexpected float')))

inputs = load('INPUTS.json')
expected = load('EXPECTED-NORMAL.json')
schema = load('EXPECTED-DATA-SCHEMA.json')
results = []

def check(name, function):
    start = time.monotonic()
    function()
    results.append({'name': name, 'status': 'PASS', 'elapsed_seconds': time.monotonic() - start})

def frozen_inputs():
    f = load('INPUT-SCHEMA-FREEZE.json')
    for entry in f['declaration_sources'] + [f['inputs'], f['expected_schema'], f['normative_readset']]:
        body = pathlib.Path(entry['path']).read_bytes()
        assert len(body) == entry['bytes'] and hashlib.sha256(body).hexdigest() == entry['sha256']
    original = load('input/normal-inputs-v1.json')
    assert inputs['cases'][:3] == original['cases']
    assert expected['layout_version'] == inputs['layout_version'] == original['layout_version']
    assert expected['api_declaration_sha256'] == f['api_declaration_sha256']


def original_mathematical_results():
    closed = {'D26': 2 ** 32 + 5, 'D27': 2 ** 64 + 2, 'D28': 3 * 2 ** 48 + 3,
        'N01-full-normal-square': 2 ** 128 - 2 ** 65 + 1,
        'N02-normal-cross-word-borrow': 2 ** 64 - 1}
    assert len(expected['cases']) == 10
    for case in expected['cases']:
        assert case['checked']['variant'] == 'Ok'
        assert int(case['checked']['value']) == closed[case['id']]
        assert 0 <= int(case['checked']['value']) < 2 ** 128
    assert len({(c['id'], c['word_bits']) for c in expected['cases']}) == 10


def exact_le_input_and_result_words():
    for case in expected['cases']:
        bits = case['word_bits']
        width, count = bits // 8, 128 // bits
        base = 2 ** bits
        for field in ['a', 'bv']:
            words = case['input_limbs_le'][field]
            assert len(words) == count
            raw = b''.join(int(x).to_bytes(width, 'little') for x in words)
            assert len(raw) == 16 and int.from_bytes(raw, 'little') == int(case['inputs'][field])
        assert len(case['full']['low']) == count
        assert all(0 <= int(x) < base for x in case['full']['low'])
        result_raw = b''.join(int(x).to_bytes(width, 'little') for x in case['full']['low'])
        assert int.from_bytes(result_raw, 'little') == int(case['checked']['value'])


def complete_component_equations():
    total = 0
    for case in expected['cases']:
        bits, op = case['word_bits'], case['operation']
        base, count = 2 ** bits, 128 // bits
        components = case['word_components']
        assert len(components) == (count ** 2 if op == 'mul' else count)
        total += len(components)
        if op == 'mul':
            assert [(c['a_index'], c['bv_index']) for c in components] == [(i, j) for i in range(count) for j in range(count)]
        else:
            assert [c['limb_index'] for c in components] == list(range(count))
        for c in components:
            a, b, low = int(c['a']), int(c['bv']), int(c['low'])
            assert 0 <= a < base and 0 <= b < base and 0 <= low < base
            if op == 'add':
                assert type(c['carry_in']) is bool and type(c['carry_out']) is bool
                assert a + b + int(c['carry_in']) == low + base * int(c['carry_out'])
            elif op == 'sub':
                assert type(c['borrow_in']) is bool and type(c['borrow_out']) is bool
                assert a - b - int(c['borrow_in']) == low - base * int(c['borrow_out'])
            else:
                assert 0 <= int(c['high']) < base
                assert a * b == low + base * int(c['high'])
    assert total == 58


def complete_full_256_products():
    count = 0
    for case in expected['cases']:
        if case['operation'] != 'mul':
            continue
        count += 1
        bits, full = case['word_bits'], case['full']
        width, words = bits // 8, 128 // bits
        assert len(full['low']) == len(full['high']) == words
        assert len(full['little_endian_256']) == 2 * words
        assert full['little_endian_256'] == full['low'] + full['high']
        raw = b''.join(int(x).to_bytes(width, 'little') for x in full['little_endian_256'])
        assert len(raw) == 32
        value = int.from_bytes(raw, 'little')
        assert value == int(full['mathematical_product'])
        assert value == int(case['inputs']['a']) * int(case['inputs']['bv'])
        assert value == int(case['checked']['value'])
    assert count == 4


def distinct_normal_word_carry():
    assert len(expected['mul_accumulate_cases']) == 2
    for case in expected['mul_accumulate_cases']:
        bits = case['word_bits']
        base = 2 ** bits
        data, output = case['inputs'], case['output']
        assert type(data['carry_in']) is str and type(output['carry_out']) is str
        assert int(data['carry_in']) > 1 and int(output['carry_out']) > 1
        assert all(0 <= int(x) < base for x in data.values())
        assert all(0 <= int(x) < base for x in output.values())
        assert int(data['a']) * int(data['bv']) + int(data['addend']) + int(data['carry_in']) == int(output['low']) + base * int(output['carry_out'])
        assert int(output['low']) == 6 and int(output['carry_out']) == base - 2


def complete_schema_and_field_coverage():
    assert list(expected) == schema['root_fields']
    for case in expected['cases']:
        assert list(case) == schema['case_fields']
        assert list(case['inputs']) == schema['inputs_fields']
        assert list(case['input_limbs_le']) == schema['input_limbs_le_fields']
        assert list(case['full']) == schema['full_' + case['operation'] + '_fields']
        assert list(case['checked']) == schema['checked_fields']
        for c in case['word_components']:
            assert list(c) == schema[case['operation'] + '_component_fields']
    for case in expected['mul_accumulate_cases']:
        assert list(case) == schema['mul_accumulate_case_fields']
        assert list(case['inputs']) == schema['mul_accumulate_input_fields']
        assert list(case['output']) == schema['mul_accumulate_output_fields']
    found = {}
    def walk(value, path=''):
        found[path] = value
        if isinstance(value, dict):
            for key, item in value.items():
                walk(item, path + '/' + key)
        elif isinstance(value, list):
            for i, item in enumerate(value):
                walk(item, path + '/' + str(i))
    walk(expected)
    coverage = load('FIELD-COVERAGE.json')
    rows = coverage['nodes']
    assert len(rows) == len(found) == coverage['node_count'] == 738
    assert len({r['json_pointer'] for r in rows}) == len(rows)
    assert {r['json_pointer'] for r in rows} == set(found)
    for row in rows:
        value = found[row['json_pointer']]
        assert row['origin']
        if 'expected_value' in row:
            assert type(row['expected_value']) is type(value) and row['expected_value'] == value


def lossless_decimal_and_literal_consumer_scope():
    def walk(value, key=''):
        if isinstance(value, dict):
            for k, item in value.items():
                walk(item, k)
        elif isinstance(value, list):
            for item in value:
                walk(item, key)
        elif isinstance(value, float):
            raise AssertionError('float')
        elif isinstance(value, str) and (key in ['a', 'bv', 'low', 'high', 'addend', 'carry_in', 'carry_out', 'value', 'mathematical_product', 'little_endian_256']):
            assert re.fullmatch(r'0|[1-9][0-9]*', value)
        elif isinstance(value, int) and not isinstance(value, bool):
            assert key in ['word_bits', 'limb_index', 'a_index', 'bv_index']
    walk(expected)
    text = (ROOT / 'word_arithmetic_independent_normal_tests.rs').read_text()
    declarations = (ROOT / 'input/API-DECLARATIONS-v1.rs.txt').read_text()
    declared = set(re.findall(r'pub fn ([a-z0-9_]+)\(', declarations))
    called = set(re.findall(r'\b((?:add_carry|sub_borrow|mul_wide|mul_accumulate|checked_u128_add|checked_u128_sub|checked_u128_mul)_(?:32|64))\(', text))
    assert called == declared
    assert text.count('#[test]') == 12
    assert 'include!' not in text and 'include_str!' not in text and 'serde' not in text
    assert text.count('let actual = checked_u128_') == 10


start = time.monotonic()
for name, fn in [('frozen_inputs_and_schema', frozen_inputs),
                 ('original_and_supplementary_normal_closed_forms', original_mathematical_results),
                 ('exact_little_endian_words_and_types', exact_le_input_and_result_words),
                 ('all_58_component_equations', complete_component_equations),
                 ('complete_untruncated_256_products', complete_full_256_products),
                 ('normal_whole_word_carry_distinct_from_bit', distinct_normal_word_carry),
                 ('complete_schema_and_738_node_coverage', complete_schema_and_field_coverage),
                 ('lossless_decimal_and_literal_consumer_declarations', lossless_decimal_and_literal_consumer_scope)]:
    check(name, fn)
print(json.dumps({'status': 'PASS', 'check_count': len(results), 'checks': results,
    'elapsed_seconds': time.monotonic() - start,
    'scope': 'normal oracle data only; no subject imports/Rust/provider/compiler/SDK/native execution'}, indent=2))
