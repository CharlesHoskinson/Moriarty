"""N01–N08 selection of the existing independent corpus comparison functions.

Trusted fixed local artifacts only. No expected row is supplied by the Rust
producer, encoder, evaluator or observed artifact. No oracle file is changed.
"""
from pathlib import Path
import importlib.util
import json
import sys

repo = Path(sys.argv[1])
observed_path = Path(sys.argv[2])
check_path = repo / 'models/sherlock/qa/check.py'
spec = importlib.util.spec_from_file_location('frozen_normal_checker', check_path)
check = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check)
# The existing consumer checks all fixed source/corpus pins and transcriptions.
corpus = check.run(repo)
check.require(corpus['status'] == 'pass', 'existing frozen corpus/source check did not pass')
families = []
expectation_sources = []
for family, prefix in [('normal', 'normal'), ('extra-normal', 'extra-normal')]:
    program_file, programs = check.document(repo / f'models/sherlock/oracles/{prefix}-programs.json')
    prefix_file, prefixes = check.document(repo / f'models/sherlock/oracles/{prefix}-prefixes.json')
    check.validate_pair(programs, prefixes, family=family)
    families.append((programs, prefixes))
    expectation_sources += [program_file, prefix_file]
adapter = check.host_adapter()
expected = adapter.expected_observations(families, producer='sherlock-layout-reference')
observed_file, observed = check.document(observed_path, strict=True)
difference = adapter.compare_observations(expected, observed, check.first_difference)
print(json.dumps(dict(status='mismatch' if difference else 'pass',
    scope='complete-corpus-projection-match-only; not independent physical codec evidence',
    programs=len(expected['programs']), prefix_runs=len(expected['runs']),
    first_difference=difference, observed_file=observed_file,
    expectation_sources=expectation_sources, corpus_source_check=corpus,
    native_authority=0), indent=2, allow_nan=False))
raise SystemExit(1 if difference else 0)
