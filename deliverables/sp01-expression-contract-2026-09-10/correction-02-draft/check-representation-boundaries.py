"""Fixed public JSON boundary arithmetic; not a runtime value or Core codec."""
import json


def canonical_bytes(tree):
    # These fixed samples contain ASCII keys/text and no escape ambiguity.
    return json.dumps(tree, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode('utf-8')


def measured(tree):
    return len(canonical_bytes(tree))


def value(typ, payload):
    return {'type': typ, 'value': payload}


U64 = ['UInt64']
T = ['Collection', ['Collection', U64, '16'], '128']
maximum = str(2**64-1)
mat = [[maximum]*16 for _ in range(128)]
arg = {'r': mat}
assert measured(arg) == 47367
assert measured(value(T, mat)) <= 65536
# The former node-bound example now has an exact normative JSON tree as well.
zeros = [['0']*16 for _ in range(128)]
args_zero = {'r': zeros}
post = {'a': zeros, 'b': zeros}
assert measured(args_zero) < 65536 and measured(value(T, zeros)) < 65536
assert measured(post) < 65536
assert 1+128*17 == 2177 and 1+2*2177 == 4355 > 4096
# Independent aggregate-byte boundary. Two values each fit their individual W
# limit; only the exact enclosing argument-record boundary distinguishes them.
CText = ['Collection', ['Text'], '128']
def boundary(tail):
    return {'a': ['x'*1024]*32, 'b': ['x'*1024]*31+['x'*tail]}
fits, exceeds = boundary(819), boundary(820)
assert measured(fits) == 65536 and measured(exceeds) == 65537
for obj in [fits, exceeds]:
    assert all(measured(value(CText, arr)) < 65536 for arr in obj.values())
    assert all(len(s.encode()) <= 1024 for arr in obj.values() for s in arr)
    assert 1+sum(1+len(arr) for arr in obj.values()) == 67
# Metadata/wrapper examples for every finite value family; fixed expected trees.
samples = {
 'UInt64': value(['UInt64'], '7'),
 'UInt128': value(['UInt128'], '7'),
 'SInt128': value(['SInt128'], '-5'),
 'Bool': value(['Bool'], True),
 'Text': value(['Text'], 'x'),
 'Amount': value(['Amount', 'A'], '30'),
 'Shares': value(['Shares', 'V', 'H'], '4'),
 'Rate': value(['Rate', '2'], '-5'),
 'Price': value(['Price', 'B', 'A', '4'], '19743'),
 'Quantity': value(['Quantity', [['USD', '1']], '2'], '-123'),
 'Record': value(['Record', 'R'], {'x': '9'}),
 'Enum': value(['Enum', 'E'], 'Open'),
 'None': value(['Option', ['UInt128']], []),
 'Some': value(['Option', ['UInt128']], ['4']),
 'Collection': value(['Collection', ['UInt128'], '2'], ['4', '9']),
 'Operation': value(['Operation', 'O'], {'operation': 'O', 'fields': {'amount': '3'}}),
}
assert all(measured(x) < 65536 for x in samples.values())
P = {'kind': 'synthetic', 'start': '0', 'end': '0'}
node = {'constructor': 'LitUInt', 'operands': {'width': '128', 'value': '7'}, 'span': P}
action = {'span': P, 'statements': [{'constructor':'Require','operands':{'condition':{'constructor':'LitBool','operands':{'value':True},'span':P}},'span':P}]}
assert measured(node) == 114 and measured(action) == 269
expected_sizes = dict(zip(samples, [31,32,33,30,29,36,39,34,46,54,41,36,42,45,57,76]))
assert {k: measured(v) for k,v in samples.items()} == expected_sizes
assert measured(value(['Text'], 'é\n')) == 32
result = {
 'scope': 'fixed mathematical JSON embeddings and independent byte/node arithmetic only; no production codec, evaluator or proof',
 'maximumUInt64Matrix': {'argsBytes': measured(arg), 'standaloneValueBytes': measured(value(T,mat)), 'valueNodes':2178, 'expectedAdmission':'fits these byte/node bounds, assuming well-typed schema/source'},
 'finiteRecordNodeFailure': {'argsBytes':measured(args_zero),'standaloneValueBytes':measured(value(T,zeros)),'assembledPostBytes':measured(post),'assembledPostNodes':4355,'expected':'VALUE_BOUND on constructed Record, three expression entries'},
 'byteBoundary': {'fittingArgsBytes':measured(fits),'excessArgsBytes':measured(exceeds),'fittingIndividualBytes':[measured(value(CText,arr)) for arr in fits.values()],'excessIndividualBytes':[measured(value(CText,arr)) for arr in exceeds.values()],'nodes':67,'expectedExcess':'INPUT_BOUND, workUsed0'},
 'finiteTypeSamples': {k:{'tree':v,'bytes':measured(v)} for k,v in samples.items()},
 'syntheticCoreSamples': {'node':node,'nodeBytes':measured(node),'action':action,'actionBytes':measured(action)},
}
print(json.dumps(result,indent=2))
