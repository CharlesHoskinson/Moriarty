"""Offline complete sequence comparisons of already frozen caller-supplied data.

No stage/model execution, seed generation or expected-value construction. This
ordinary adapter reports byte-based equality through the reviewed H5 helper.
Typed supplied-input/ITF premises and H2/H3/H4/H6 limitations remain unchanged.
"""
import argparse
import hashlib
import json
from pathlib import Path

from b06_whole_machine_observations import machine_states
from core2_await_boundary_wire import (
    boundary_observations_equal, quint_record_observation,
)
from core2_owning_capture_observations import publication_observation
from quint_observations import _variable, canonical_bytes


def sequence_comparison(capture, itf, expected_publications, expected_machine):
    """Retain every publication and every Machine state, without zip/truncation.

    The last supplied raw ITF state is the declared end of the finite run. Its
    entire record vector is compared; earlier Ready/Published states also remain
    in the full Machine comparison. No selected record replaces the vector.
    Nonempty valid ITF and exact finite observation bound are reviewed caller
    premises, not inferred here. No equality is used to choose a state or result.
    """
    publications = capture['publications']
    independent_publications = expected_publications['publications']
    core = [publication_observation(row) for row in publications]
    independent_core = [publication_observation(row) for row in independent_publications]
    states = machine_states(itf)
    raw_final_records = _variable(itf['states'][-1], 'machine')['records']
    quint = [quint_record_observation(row) for row in raw_final_records]
    return {
        'format': 'sherlock-core2-owning-sequence-comparison/1',
        'caseId': capture['caseId'],
        'rawFinalStateOrdinal': len(itf['states']) - 1,
        'rawFinalRecords': raw_final_records,
        'coreObservations': core,
        'independentPublicationObservations': independent_core,
        'quintObservations': quint,
        'machineStates': states,
        'publicationsEqual': boundary_observations_equal(publications, independent_publications),
        'coreBoundariesEqual': boundary_observations_equal(core, independent_core),
        'quintBoundariesEqual': boundary_observations_equal(quint, expected_machine['commonWires']),
        'coreQuintBoundariesEqual': boundary_observations_equal(core, quint),
        'machineStatesEqual': boundary_observations_equal(states, expected_machine['states']),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('core_capture', type=Path)
    parser.add_argument('quint_itf', type=Path)
    parser.add_argument('independent_publications', type=Path)
    parser.add_argument('independent_machine', type=Path)
    args = parser.parse_args()
    supplied = {
        'coreCapture': args.core_capture,
        'quintItf': args.quint_itf,
        'independentPublications': args.independent_publications,
        'independentMachine': args.independent_machine,
    }
    raw = {name: path.read_bytes() for name, path in supplied.items()}
    documents = {name: json.loads(value) for name, value in raw.items()}
    report = {
        'format': 'supplied-core2-b06-sequence-complete-comparison/1',
        'inputPaths': {name: str(path) for name, path in supplied.items()},
        'inputSha256': {name: hashlib.sha256(value).hexdigest() for name, value in raw.items()},
        'rawDocuments': documents,
        'comparison': sequence_comparison(documents['coreCapture'], documents['quintItf'],
                                          documents['independentPublications'],
                                          documents['independentMachine']),
    }
    print(canonical_bytes(report).decode())


if __name__ == '__main__':
    main()
