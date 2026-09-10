"""Reviewer-owned finite arithmetic/structure controls; not a Core evaluator or codec.

All data are public synthetic examples. No candidate file is mutated.
"""
import copy
import importlib.util
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[3]
SPEC = ROOT / 'experiments/moriarty-language/spec/successor'
PACKET = ROOT / 'deliverables/sp01-expression-contract-2026-09-10'
checks = []

def check(name, condition, detail):
    assert condition, name
    checks.append({'id': name, 'result': 'PASS', 'scope': detail})

def size(tree):
    return len(json.dumps(tree, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode('utf-8'))

def char_size(text):
    # Independent arithmetic of the J string spelling, not an encoder.
    return 2 + sum(2 if c in '"\\\b\f\n\r\t' else 6 if ord(c) < 32 else len(c.encode('utf-8')) for c in text)

cases = json.loads((SPEC / 'expression-cases.json').read_text())
signatures = json.loads((SPEC / 'expression-signatures.json').read_text())
names = {row['constructor'] for row in signatures['constructors']}
prior = json.loads((PACKET / 'original-candidate-01/experiments/moriarty-language/spec/successor/expression-cases.json').read_text())
old_ids = {row['id'] for row in prior['cases'] + prior['combinedCases']}
all_rows = cases['cases'] + cases['combinedCases']
new_ids = {row['id'] for row in all_rows}
dispositions = json.loads((PACKET / 'case-admission-review-02.json').read_text())['rows']
check('all-old-dispositions', len(old_ids) == 95 and old_ids <= new_ids and old_ids <= {r['id'] for r in dispositions}, 'Identity coverage only; semantic derivations independently inspected in review.')
check('counts', len(names) == 40 and len(cases['cases']) == 80 and len(cases['combinedCases']) == 20 and len(new_ids) == 100, 'Exact inventory/cardinality, not executable semantics.')
for row in cases['cases']:
    if row['kind'] == 'positive':
        count = sum(token in names for token in re.findall(r'\b([A-Z][A-Za-z]+)\(', row['term']))
        acc = row['derivationAccounting']
        check('work-' + row['id'], count == int(acc['derivedWorkUsed']) and int(acc['workInitial']) - count == int(row['expected']['workRemaining']), 'All entered constructor occurrences in this successful fragment; metadata is not counted.')

# Independent canonical string-size arithmetic, including controls and astral UTF-8.
for i, text in enumerate(['', 'é', 'e\u0301', '\U0001f642', '\u2028', '"\\', ''.join(map(chr, range(32))), 'é\n']):
    check(f'J-string-{i}', size(text) == char_size(text), 'Fixed string arithmetic checks UTF-8, JSON short/control escapes, and no normalization.')
check('no-normalization', size('é') != size('e\u0301'), 'Canonically equivalent Unicode strings remain distinct exact scalar sequences.')

matrix = [[str(2**64 - 1)] * 16 for _ in range(128)]
typ = ['Collection', ['Collection', ['UInt64'], '16'], '128']
check('matrix-byte-recount', size({'r': matrix}) == 47367 and size({'type': typ, 'value': matrix}) == 47430, 'Direct tree count; no author sizing helper imported.')
check('matrix-node-recount', 1 + 128 * (1 + 16) == 2177 and 1 + 2 * 2177 == 4355, 'Compound occurrence arithmetic. Sharing does not reduce the count.')
for n in [818, 819, 820]:
    obj = {'a': ['x' * 1024] * 32, 'b': ['x' * 1024] * 31 + ['x' * n]}
    # 2 outer delimiters + 7 field punctuation + 4 array delimiters +
    # 62 array commas + 128 string quotes + 63*1024+n text bytes.
    check(f'aggregate-{n}', size(obj) == 2 + 7 + 4 + 62 + 128 + 63 * 1024 + n == 65536 + n - 819, 'Independent closed-form byte arithmetic; boundary is inclusive.')

op = {'operation': 'O', 'fields': {'amount': '3'}}
check('descriptor-sequence-count', size([op] * 128) == 2 + 128 * size(op) + 127 and 1 + 128 * 3 == 385, '128 descriptors fit these bytes/nodes; 129 fails descriptor cardinality despite fitting these bytes/nodes.')

def graph_cycle(graph):
    active, done = set(), set()
    def visit(node):
        if node in active:
            return True
        if node in done:
            return False
        active.add(node)
        if any(visit(child) for child in graph.get(node, ())):
            return True
        active.remove(node)
        done.add(node)
        return False
    return any(visit(node) for node in graph)

check('operation-cycle-discriminator', graph_cycle({'R': ['O'], 'O': ['R']}) and not graph_cycle({'R': [], 'O': ['R']}), 'Graph arithmetic after manually applying the written Option/Collection transparent dependency traversal; not a schema validator.')
for n, d in [(-5, 2), (-6, 2), (-(2**127), 1), (2**127 - 1, 2), (0, 3)]:
    q, r = divmod(n, d)
    check(f'euclidean-{n}-{d}', n == q*d+r and 0 <= r < d and q*d <= n < (q+1)*d and (q+(r != 0))*d >= n, 'Finite signed Euclidean quotient and ceiling boundaries.')

# Adequacy negative controls: the candidate's validator deliberately verifies
# records/references, not semantic expected values or combined-case correctness.
module_spec = importlib.util.spec_from_file_location('reviewed_document_validator', SPEC / 'check-expression-contract.py')
module = importlib.util.module_from_spec(module_spec)
module_spec.loader.exec_module(module)
bad = copy.deepcopy(cases)
bad['cases'][0]['expected']['value'] = 'UInt64(999)'
check('validator-not-value-oracle', module.validate(signatures, bad) == [], 'Wrong specified semantic value survives structure checks, as disclosed; not candidate evaluator acceptance.')
bad = copy.deepcopy(cases)
bad['combinedCases'] = []
check('validator-not-combined-oracle', module.validate(signatures, bad) == [], 'Removing all combined derivations survives structure checks, as disclosed.')
bad_signature = copy.deepcopy(signatures)
bad_signature['constructors'].pop()
check('validator-inventory-negative', 'constructor-set' in module.validate(bad_signature, cases), 'The validator does detect missing inventory; this control distinguishes its actual scope.')

print(json.dumps({'scope': 'Reviewer-owned finite arithmetic/structure controls only; no production codec, typechecker, Core execution or proof.', 'passed': len(checks), 'checks': checks}, indent=2))
