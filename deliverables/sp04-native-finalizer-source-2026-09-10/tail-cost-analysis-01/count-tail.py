#!/usr/bin/env python3
"""Public integer/source counts only; no circuit, compiler or proof execution."""
import contextlib
import hashlib
import io
import json
from pathlib import Path
import runpy
import tarfile

here = Path(__file__).resolve().parent
prior = here.parent
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
assert digest(prior / 'proposal.json') == '6e979f88697d354660599d572ffcf5fb60cc278a45c761e58a77c341f7a18589'
assert digest(prior / 'schedule-supplement-01/manifest.json') == 'e03b044388140a3c222ec8b90a27b066bf6f8a173506ddede2b917ded263a354'
for manifest in [prior / 'proposal.json', prior / 'schedule-supplement-01/manifest.json']:
    for name, expected in json.loads(manifest.read_text())['files'].items():
        assert digest(manifest.parent / name) == expected
evidence = json.loads((here / 'source-evidence.json').read_text())
archive = Path(evidence['archive']['path'])
assert digest(archive) == evidence['archive']['archiveSha256']
with tarfile.open(archive) as t:
    for entry in evidence['members']:
        raw = t.extractfile(entry['member']).read()
        assert hashlib.sha256(raw).hexdigest() == entry['sha256']
        lines = raw.decode().splitlines()
        for excerpt in entry['excerpts']:
            assert '\n'.join(lines[excerpt['start']-1:excerpt['end']]) == excerpt['text']

# Reuse the already reviewed PURE PYTHON auxiliary-bound port. Its exact bytes
# and all input results were checked above. Suppress its normal JSON print.
with contextlib.redirect_stdout(io.StringIO()):
    cost = runpy.run_path(str(prior / 'schedule-supplement-01/count-gadget-cost.py'))
L, B, rows = cost['L'], cost['B'], cost['rows']
assert L == 7 and rows(56) == 2

# Conditional on well-formed input limbs, an already constrained identity bit,
# and the same native5/foreign14, four-parallel-eight-bit lookup layout.
select_per_fp = L  # seven native one-row select calls
assign_per_fp = L * rows(56)  # top limb overcounted at56 bits
equal_fixed_per_fp = L * 2  # fixed request + copy region, no cache credit
field_tail = 24*select_per_fp + 12*assign_per_fp + 24*equal_fixed_per_fp
assert field_tail == 672

# Fixed-value SLOT counts, not verified constants. Freeze and validate actual
# inner-SRS line values and canonical Montgomery conversion before use.
line_fp_slots = 2 * 68 * 3 * 2
frobenius_fp_slots = 4 * (5*2 + 4)
identity_fp_slots = 2  # reusable field zero and one
fixed_slot_rows = (line_fp_slots + frobenius_fp_slots + identity_fp_slots) * L
assert fixed_slot_rows == 6118

# Source on-curve gate: a=0,b=4. Inputs already assigned/well-formed.
# It first computes x*x via one generic multiply, then a two-row identity
# and the existing-cell u/v checks. Identity condition derivation is separate.
def on_curve_expr(bs, bs2):
    linear = sum(bs)*(B-1)
    quadratic = sum(bs2)*(B-1)**2
    return -quadratic-2*linear-4, quadratic+2*linear-4

widths = cost['auxiliary'](
    on_curve_expr(cost['bp'], cost['dbp']),
    [on_curve_expr([x % m for x in cost['bp']],
                   [x % m for x in cost['dbp']]) for m in cost['moduli']])
assert widths == [116, 120, 117]
on_curve_rows = 45 + 2 + sum(rows(w)+1 for w in widths)
assert on_curve_rows == 62
# assign_without_subgroup_check: two Fp coordinates, boolean bit, NOT(bit)
# (one fixed-zero request plus native linear row), conditional on-curve gate.
on_curve_assignment_rows = 2*assign_per_fp + 1 + 2 + on_curve_rows
assert on_curve_assignment_rows == 93

h = 0x396c8c005555e1568c00aaab0000aaab
z = -0xd201000000010000
assert h*cost['r'] == cost['p']-z and h == (z-1)**2//3
# Literal helper schedule count, NOT a validated subgroup-check row bound.
cofactor_counts = {'bits': h.bit_length(), 'popcount': h.bit_count(),
                   'doublings': h.bit_length()-1, 'incompleteAdds': h.bit_count()-1}
assert cofactor_counts == {'bits':126, 'popcount':48, 'doublings':125, 'incompleteAdds':47}

print(json.dumps({
    'status':'PASS_PUBLIC_TAIL_COUNTS_ONLY_REVIEW_REQUIRED',
    'fieldTail': {'selections':24, 'inverseWitnessFpAssignments':12,
                  'identityFpEqualities':24, 'conditionalRegionSubtotal':field_tail,
                  'inverseProductAlreadyInPriorArithmetic':True},
    'fixedConstantSlots': {'preparedLineFpSlots':line_fp_slots,
                          'frobeniusFpSlots':frobenius_fp_slots,
                          'identityFpSlots':identity_fp_slots,
                          'conditionalAssignmentRows':fixed_slot_rows,
                          'actualValuesFrozenOrValidated':False},
    'conditionalFieldAndConstantTailSubtotal':field_tail+fixed_slot_rows,
    'conditionalOnCurveOnly': {'auxiliaryWidths':widths,
                              'alreadyAssignedCoordinatesRows':on_curve_rows,
                              'assignmentWithBitAndNotRowsPerPoint':on_curve_assignment_rows,
                              'canonicalIdentityOrBytesEstablished':False,
                              'subgroupEstablished':False},
    'cofactorRootHelper': {'cofactorHex':hex(h), **cofactor_counts,
                          'incompleteAdditionPreconditionsEstablished':False,
                          'subgroupRows':None},
    'completeVerifierRowUpperBound':None,
    'operations': {'nativeExecutions':0,'compiles':0,'proofs':0,'network':0,'privateReads':0},
    'scope':'Conditional region counts only; no full-domain fit, backend/SRS selection, F0 go or resource admission.'
}, indent=2))
