#!/usr/bin/env python3
"""Compare supplied fixed Core-source diagnostics to frozen independent literals.

File comparison only: no parser, evaluator, compiler, SDK, model or network is
invoked. The shared normal adapter expands the existing literal prefix overlay;
this consumer implements no transition or alternative source semantics.
"""
import argparse
import importlib.util
import json
from pathlib import Path
import sys

REPO = Path(__file__).resolve().parents[3]
PRODUCER = 'sherlock-core-source-reference'
ORACLES = 'models/sherlock/oracles/'
FROZEN = {
    'source-normal-programs.json': 'a3542b1101251f71e6b41ef6e7a8d7d7694a3ec3c9d594ed79c8c9ea0d3eaa00',
    'source-normal-prefixes.json': '184f9136da321975d6d447bf46bb0d1ac6459c12a67d0ae112ebf92fefa5f652',
    'SOURCE-NORMAL-PROVENANCE.md': 'e5f6f212b06b32e12530ff5b12961778aca4a40686088564f907a91db8a23371',
}
FIXTURES = {
    'crates/mori-core/src/sherlock/source_adapter_fixtures/weighted-total.mori':
        '073474d19e6294ab77a053dcc6e0a66defdfdb32f047a795cb16652ce4b5bb65',
    'crates/mori-core/src/sherlock/source_adapter_fixtures/threshold-check.mori':
        'caba5067afdddf62835af5fa8864168fdb5ca4b1493281b3085796262cdbf796',
}
PROGRAM_FIELDS = ('case_id', 'source_path', 'source_text', 'source_language',
                  'source_semantics', 'code_k', 'entry_k', 'preparation')


def qa_adapter():
    """Load the actual sibling checker even under Python -I, without sys.path edits."""
    path = Path(__file__).resolve().with_name('check.py')
    raw = path.read_bytes()
    spec = importlib.util.spec_from_file_location('sherlock_source_shared_qa', path)
    if spec is None or spec.loader is None:
        raise ValueError('shared literal checker unavailable')
    module = importlib.util.module_from_spec(spec)
    exec(compile(raw, str(path), 'exec', dont_inherit=True), module.__dict__)
    return module


def frozen_documents(repo, qa):
    """Read exact pre-execution oracle bytes and both actual source specimens."""
    repo = Path(repo)
    documents, files = {}, []
    for name, expected_sha in FROZEN.items():
        path = repo / ORACLES / name
        if name.endswith('.json'):
            reference, value = qa.document(path, strict=True)
            documents[name] = value
        else:
            reference = dict(path=str(path), sha256=qa.digest(path.read_bytes()))
        qa.require(reference['sha256'] == expected_sha, f'frozen source oracle changed: {name}')
        files.append(dict(path=ORACLES + name, sha256=reference['sha256']))
    programs = documents['source-normal-programs.json']
    prefixes = documents['source-normal-prefixes.json']
    for document, member in ((programs, 'programs'), (prefixes, 'traces')):
        qa.require(document['profile'] == qa.PROFILE, 'source oracle reference profile mismatch')
        qa.require(document['source'] == qa.SOURCE, 'source oracle K identity mismatch')
        qa.require([row['case_id'] for row in document[member]] == ['S01', 'S02'],
                   'source oracle membership/order mismatch')
    for program, (path, expected_sha) in zip(programs['programs'], FIXTURES.items(), strict=True):
        raw = (repo / path).read_bytes()
        actual_sha = qa.digest(raw)
        qa.require(actual_sha == expected_sha, f'frozen source fixture changed: {path}')
        qa.require(program['source_path'] == path and program['source_sha256'] == expected_sha
                   and program['source_text'].encode('utf-8') == raw,
                   f'frozen source fixture/oracle identity mismatch: {path}')
        qa.require(program['source_language'] == 'moriarty/1'
                   and program['source_semantics'] == 'moriarty-core/1',
                   'source oracle Core profile mismatch')
        files.append(dict(path=path, sha256=actual_sha))
    source = qa.SOURCE
    qa.require(qa.digest((repo / source['path']).read_bytes()) == source['sha256'],
               'frozen K source changed')
    files.append(dict(source))
    return programs, prefixes, files


def project_expected(programs, prefixes, qa):
    """Use only the shared, documented literal overlay; retain full source reports."""
    host = qa.host_adapter()
    expected = host.expected_observations([(programs, prefixes)], producer=PRODUCER)
    expected['programs'] = [{key: program[key] for key in PROGRAM_FIELDS}
                            for program in programs['programs']]
    qa.require(len(expected['runs']) == 70, 'frozen source prefix count mismatch')
    qa.require(sum(len(run['records']) for run in expected['runs']) == 1259,
               'frozen source record count mismatch')
    return host, expected


def expected_observations(repo):
    """Expose the literal projection for ordinary comparator tests and callers."""
    qa = qa_adapter()
    programs, prefixes, _ = frozen_documents(repo, qa)
    _, expected = project_expected(programs, prefixes, qa)
    return expected


def run(repo, *, observations):
    qa = qa_adapter()
    programs, prefixes, files = frozen_documents(repo, qa)
    host, expected = project_expected(programs, prefixes, qa)
    reference, observed = qa.document(observations, strict=True)
    difference = host.compare_observations(expected, observed, qa.first_difference)
    declared = observed if type(observed) is dict else {}
    actual_programs = declared.get('programs')
    program_difference = qa.first_difference(expected['programs'], actual_programs, '/programs')
    return dict(
        status='mismatch' if difference else 'pass',
        scope='complete-supplied-fixed-core-source-diagnostic-artifact-comparison-only',
        acceptance='none', profile=qa.PROFILE, frozen_files=files,
        counts=dict(programs=2, shared_pending_states=sum(len(trace['states']) for trace in prefixes['traces']),
                    source_occurrences=sum(len(row['preparation']['occurrences']) for row in programs['programs']),
                    prefix_runs=len(expected['runs']), records=sum(len(run['records']) for run in expected['runs'])),
        source_preparation_comparison=dict(
            status='mismatch' if program_difference else 'match',
            scope='exact-source-profiles-full-program-identities-and-complete-preparation-reports',
            observed_programs=actual_programs, first_difference=program_difference),
        supplied_source_observation=dict(
            status='mismatch' if difference else 'match', file=reference,
            declared_producer=declared.get('producer'), declared_profile=declared.get('profile'),
            declared_source_sha256=declared.get('source_sha256'),
            producer_authenticity='not-established', execution_receipt='not-supplied',
            constructor_startup='unsupported', expected_programs=2,
            expected_prefix_runs=70, expected_records=1259),
        first_difference=difference,
        execution_layers=[dict(layer=layer, status='not-run',
                               reason='Supplied artifact comparison launches no execution adapter')
                          for layer in ('Rust-host', 'K', 'Quint', 'Lean', 'Clean',
                                        'handwritten-circuit', 'native-ledger')],
        gaps=['Source-static scalar projection compares emitted values; Core effects are not executed.',
              'Opaque source names are host diagnostics, outside the K numeric-name comparison grammar.',
              'No general source grammar, draft frontend, bounded B, proof or target qualification.',
              'A matching supplied artifact authenticates neither its producer nor an execution.'])


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=REPO)
    parser.add_argument('--observations', type=Path, required=True)
    args = parser.parse_args(argv)
    try:
        report = run(args.repo, observations=args.observations)
    except (OSError, ValueError, TypeError, KeyError) as error:
        print(json.dumps(dict(status='input-error', error=str(error))), file=sys.stderr)
        return 2
    print(json.dumps(report, indent=2))
    return 0 if report['status'] == 'pass' else 1


if __name__ == '__main__':
    raise SystemExit(main())
