"""Offline whole B06 Machine projection over already-frozen supplied ITF.

No model/source evaluation or expected-value synthesis. Internal actionName stays
internal; canonical public action aliasing belongs only to the common-wire path.
"""
import argparse
import hashlib
import json
from pathlib import Path

from core2_await_boundary_wire import boundary_observations_equal, quint_typed_wire
from quint_observations import _variable, canonical_bytes


def invocation_wire(invocation):
    """Convert only the declared Invocation.guardCosts integer-map row names."""
    wire = quint_typed_wire(invocation)
    wire['guardCosts'] = [{'site': row['key'], 'cost': row['value']}
                          for row in wire['guardCosts']]
    return wire


def record_wire(record):
    wire = quint_typed_wire(record)
    wire['invocation'] = invocation_wire(record['invocation'])
    return wire


MACHINE_NULLARY = frozenset({'Ready', 'Published', 'NoResult', 'NoPrefix', 'NotAdvanced'})


def machine_nullary_wire(value):
    """Bare strings only for declared nullaries with the decoded unit payload.

    The installed serializer emits unit as #tup:[], decoded as an empty tuple.
    Unexpected payloads remain present; they are never erased into a bare tag.
    """
    if type(value) is list:
        return [machine_nullary_wire(item) for item in value]
    if type(value) is not dict:
        return value
    if len(value) == 1:
        tag, payload = next(iter(value.items()))
        if tag in MACHINE_NULLARY and type(payload) is tuple and len(payload) == 0:
            return tag
    return {key: machine_nullary_wire(item) for key, item in value.items()}


def machine_wire(machine):
    """Preserve the complete Machine; only its typed Invocation paths differ."""
    wire = quint_typed_wire(machine)
    wire['current'] = invocation_wire(machine['current'])
    wire['records'] = [record_wire(record) for record in machine['records']]
    return machine_nullary_wire(wire)


def machine_states(document):
    return [machine_wire(_variable(state, 'machine')) for state in document['states']]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('itf', type=Path)
    parser.add_argument('independent_expected', type=Path)
    args = parser.parse_args()
    raw_itf = args.itf.read_bytes()
    raw_expected = args.independent_expected.read_bytes()
    document = json.loads(raw_itf)
    expected = json.loads(raw_expected)
    actual_states = machine_states(document)
    report = {
        'format': 'supplied-b06-whole-machine-comparison/1',
        'itfInputSha256': hashlib.sha256(raw_itf).hexdigest(),
        'independentInputSha256': hashlib.sha256(raw_expected).hexdigest(),
        'rawItf': document,
        'rawIndependentExpected': expected,
        'machineStates': actual_states,
        'machineStatesEqual': boundary_observations_equal(actual_states, expected['states']),
    }
    print(canonical_bytes(report).decode())


if __name__ == '__main__':
    main()
