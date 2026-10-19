"""Offline supplied-publication/ITF observations; never execute stage or models.

Generic data glue over the existing reviewed normalizers and H5 equality helper.
It adds no domain/admission/shape guards. H2/H3/H4/H6 remain owner prerequisites.
The expected-publication file is independently authored before actual capture;
Root opens it only after freezing complete actuals. No model seed is generated.
"""
import argparse
import hashlib
import json
from pathlib import Path

from core2_await_boundary_wire import (
    boundary_observations_equal, core2_observation, quint_record_observation,
    supplied_itf_record,
)
from quint_observations import canonical_bytes


def publication_observation(publication):
    return core2_observation({
        'outcome': publication['financial_observation']['outcome'],
        'awaitPrefix': publication['await_prefix'],
    })


def capture_observations(capture):
    resume = capture['resumePublication']
    return {
        'start': publication_observation(capture['startPublication']),
        'resume': None if resume is None else publication_observation(resume),
    }


def compare_supplied(capture, itf, state_ordinal, record_index, expected_publications):
    """Report comparisons; expected publications are separate caller data.

    Entire raw publications and the narrower common boundary are distinct checks.
    This routine neither establishes expectation independence nor same-call origin.
    A missing resume is retained and then ordinary normalization cannot compare it;
    it is never filled from an expected/other result. The selected input cohort
    requires a real returned Waiting continuation, as a separately reviewed premise.
    """
    actual = {
        'start': publication_observation(capture['startPublication']),
        'resume': publication_observation(capture['resumePublication']),
    }
    expected = {
        'start': publication_observation(expected_publications['startPublication']),
        'resume': publication_observation(expected_publications['resumePublication']),
    }
    selected_record = supplied_itf_record(itf, state_ordinal, record_index)
    quint = quint_record_observation(selected_record)
    return {
        'format': 'sherlock-core2-owning-common-comparison/1',
        'caseId': capture['caseId'],
        'coreObservations': actual,
        'independentObservations': expected,
        'quintObservation': quint,
        'rawStateOrdinal': state_ordinal,
        'recordIndex': record_index,
        'rawDecodedQuintRecord': selected_record,
        # Whole-publication equality uses the already existing byte serializer.
        # These are not advertised as the narrower result/await projection.
        'startPublicationEqual': canonical_bytes(capture['startPublication']) == canonical_bytes(expected_publications['startPublication']),
        'resumePublicationEqual': canonical_bytes(capture['resumePublication']) == canonical_bytes(expected_publications['resumePublication']),
        # The actual boundary comparisons consume exactly the reviewed H5 helper.
        'startBoundaryEqual': boundary_observations_equal(actual['start'], expected['start']),
        'resumeBoundaryEqual': boundary_observations_equal(actual['resume'], expected['resume']),
        'quintBoundaryEqual': boundary_observations_equal(quint, expected['resume']),
        'coreQuintBoundaryEqual': boundary_observations_equal(actual['resume'], quint),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('core_capture', type=Path)
    parser.add_argument('--quint-itf', type=Path)
    parser.add_argument('--state-ordinal', type=int)
    parser.add_argument('--record-index', type=int)
    parser.add_argument('--independent-publications', type=Path)
    args = parser.parse_args()
    raw_core = args.core_capture.read_bytes()
    capture = json.loads(raw_core)
    report = {
        'format': 'sherlock-core2-owning-boundary-observations/1',
        'coreInputSha256': hashlib.sha256(raw_core).hexdigest(),
        'rawCoreCapture': capture,
        'observations': capture_observations(capture),
    }
    if args.independent_publications is not None:
        raw_itf = args.quint_itf.read_bytes()
        raw_expected = args.independent_publications.read_bytes()
        itf = json.loads(raw_itf)
        expected = json.loads(raw_expected)
        report.update({
            'quintInputSha256': hashlib.sha256(raw_itf).hexdigest(),
            'rawQuintInput': itf,
            'independentInputSha256': hashlib.sha256(raw_expected).hexdigest(),
            'rawIndependentPublications': expected,
            'comparison': compare_supplied(capture, itf, args.state_ordinal,
                                           args.record_index, expected),
        })
    print(canonical_bytes(report).decode())


if __name__ == '__main__':
    main()
