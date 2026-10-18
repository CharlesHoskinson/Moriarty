"""Independent integer equations over frozen input data; no implementation imports."""
import hashlib
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parent
INPUTS = json.loads((ROOT / 'INPUTS.json').read_text())
API_SHA = '61b764e2aff77c751975372e2a2d94b5c10c42261c412fda02c0a95746ebaa61'
assert hashlib.sha256((ROOT / 'input/API-DECLARATIONS-v1.rs.txt').read_bytes()).hexdigest() == API_SHA

def limbs(value, bits, count):
    return [str((value // (1 << (bits * i))) % (1 << bits)) for i in range(count)]

cases = []
for item in INPUTS['cases']:
    a, bv = int(item['a']), int(item['bv'])
    operation = item['operation']
    result = a + bv if operation == 'add' else a - bv if operation == 'sub' else a * bv
    # This corpus intentionally contains only ordinary in-range checked results.
    assert 0 <= a < 2 ** 128 and 0 <= bv < 2 ** 128 and 0 <= result < 2 ** 128
    for bits in INPUTS['word_bits']:
        base, count = 1 << bits, 128 // bits
        al, bl = limbs(a, bits, count), limbs(bv, bits, count)
        components = []
        if operation in ['add', 'sub']:
            for i in range(count):
                # Derive carries from whole-prefix equations, not a producer ripple loop.
                before, after = base ** i, base ** (i + 1)
                if operation == 'add':
                    components.append({'limb_index': i, 'a': al[i], 'bv': bl[i],
                        'carry_in': bool(((a % before) + (bv % before)) // before),
                        'low': limbs(result, bits, count)[i],
                        'carry_out': bool(((a % after) + (bv % after)) // after)})
                else:
                    components.append({'limb_index': i, 'a': al[i], 'bv': bl[i],
                        'borrow_in': (a % before) < (bv % before),
                        'low': limbs(result, bits, count)[i],
                        'borrow_out': (a % after) < (bv % after)})
            full = {'low': limbs(result, bits, count)}
            full['carry_out' if operation == 'add' else 'borrow_out'] = False
        else:
            # All NxN word pairs, even zero words, are retained as separate component inputs.
            for i in range(count):
                for j in range(count):
                    high, low = divmod(int(al[i]) * int(bl[j]), base)
                    components.append({'a_index': i, 'bv_index': j, 'a': al[i], 'bv': bl[j],
                                       'low': str(low), 'high': str(high)})
            full = {'low': limbs(result, bits, count),
                    'high': limbs(result // (1 << 128), bits, count),
                    'little_endian_256': limbs(result, bits, 2 * count),
                    'mathematical_product': str(result)}
        cases.append({'id': item['id'], 'word_bits': bits, 'operation': operation,
            'inputs': {'a': item['a'], 'bv': item['bv']}, 'input_limbs_le': {'a': al, 'bv': bl},
            'word_components': components, 'full': full,
            'checked': {'variant': 'Ok', 'value': str(result)}})

macs = []
for item in INPUTS['mul_accumulate_cases']:
    bits = item['word_bits']
    values = {key: int(item[key]) for key in ['a', 'bv', 'addend', 'carry_in']}
    base = 1 << bits
    assert all(0 <= n < base for n in values.values())
    total = values['a'] * values['bv'] + values['addend'] + values['carry_in']
    high, low = divmod(total, base)
    assert 0 <= high < base
    macs.append({'id': item['id'], 'word_bits': bits,
        'inputs': {key: item[key] for key in ['a', 'bv', 'addend', 'carry_in']},
        'output': {'low': str(low), 'carry_out': str(high)}})

expected = {'format': 'sherlock-typed-words-independent-normal/1',
    'api_declaration_sha256': API_SHA, 'layout_version': INPUTS['layout_version'],
    'cases': cases, 'mul_accumulate_cases': macs}
(ROOT / 'EXPECTED-NORMAL.json').write_text(json.dumps(expected, indent=2) + '\n')

# Source-only consumer: direct calls/assertions with fully populated literal expected values.
# This generated source contains no evaluator, materializer, oracle lookup or subject run.
imports = ['CarryBit', 'BorrowBit']
for bits in [32, 64]:
    imports.extend([name + str(bits) for name in ['MulCarry', 'AddCarry', 'SubBorrow', 'MulWide', 'MulAccumulate']])
    imports.extend([f'{name}_{bits}' for name in ['add_carry', 'sub_borrow', 'mul_wide', 'mul_accumulate',
        'checked_u128_add', 'checked_u128_sub', 'checked_u128_mul']])
lines = ['//! Independent normal literals, source-only/uncompiled. Derived from frozen UIREQ030 inputs.',
    f'//! Frozen public declaration SHA256: {API_SHA}',
    '//! No expected values are computed from actual producer output. Root owns registration/run.',
    'use super::{\n    ' + ',\n    '.join(imports) + '\n};', '']

def boolean(value):
    return 'true' if value else 'false'

def array(values, bits):
    return '[' + ', '.join(v + f'u{bits}' for v in values) + ']'

for item in cases:
    bits, operation = item['word_bits'], item['operation']
    name = item['id'].lower().replace('-', '_') + f'_word{bits}'
    lines.extend(['#[test]', f'fn independent_{name}() {{'])
    for i, c in enumerate(item['word_components']):
        if operation == 'add':
            call = f'add_carry_{bits}({c["a"]}u{bits}, {c["bv"]}u{bits}, CarryBit({boolean(c["carry_in"])}))'
            value = f'AddCarry{bits} {{ low: {c["low"]}u{bits}, carry_out: CarryBit({boolean(c["carry_out"])}) }}'
        elif operation == 'sub':
            call = f'sub_borrow_{bits}({c["a"]}u{bits}, {c["bv"]}u{bits}, BorrowBit({boolean(c["borrow_in"])}))'
            value = f'SubBorrow{bits} {{ low: {c["low"]}u{bits}, borrow_out: BorrowBit({boolean(c["borrow_out"])}) }}'
        else:
            call = f'mul_wide_{bits}({c["a"]}u{bits}, {c["bv"]}u{bits})'
            value = f'MulWide{bits} {{ low: {c["low"]}u{bits}, high: {c["high"]}u{bits} }}'
        lines.append(f'    assert_eq!({call}, {value}, "component {i}");')
    lines.append(f'    let actual = checked_u128_{operation}_{bits}({item["inputs"]["a"]}u128, {item["inputs"]["bv"]}u128);')
    lines.append(f'    assert_eq!(actual.full.low, {array(item["full"]["low"], bits)});')
    if operation == 'add':
        lines.append(f'    assert_eq!(actual.full.carry_out, CarryBit({boolean(item["full"]["carry_out"])}));')
    elif operation == 'sub':
        lines.append(f'    assert_eq!(actual.full.borrow_out, BorrowBit({boolean(item["full"]["borrow_out"])}));')
    else:
        lines.append(f'    assert_eq!(actual.full.high, {array(item["full"]["high"], bits)});')
        count = 128 // bits
        actual_full = ', '.join(f'actual.full.{side}[{i}]' for side in ['low', 'high'] for i in range(count))
        lines.append(f'    let actual_full_256 = [{actual_full}];')
        lines.append(f'    assert_eq!(actual_full_256, {array(item["full"]["little_endian_256"], bits)});')
    lines.extend([f'    assert_eq!(actual.checked, Ok({item["checked"]["value"]}u128));', '}', ''])
for item in macs:
    bits, inputs, output = item['word_bits'], item['inputs'], item['output']
    lines.extend(['#[test]', f'fn independent_mac_word{bits}() {{',
        f'    let actual = mul_accumulate_{bits}({inputs["a"]}u{bits}, {inputs["bv"]}u{bits}, {inputs["addend"]}u{bits}, MulCarry{bits}({inputs["carry_in"]}u{bits}));',
        f'    assert_eq!(actual, MulAccumulate{bits} {{ low: {output["low"]}u{bits}, carry_out: MulCarry{bits}({output["carry_out"]}u{bits}) }});', '}', ''])
(ROOT / 'word_arithmetic_independent_normal_tests.rs').write_text('\n'.join(lines))
print(json.dumps({'status': 'DATA_DERIVED_SOURCE_CONSUMER_WRITTEN_UNCOMPILED',
    'checked_normal_case_layouts': len(cases), 'mul_accumulate_normals': len(macs),
    'component_calls_expected': sum(len(c['word_components']) for c in cases),
    'no_subject_import_or_execution': True}))
