# Candidate A Task 7 checker takeover report

Final status is in the appended completeness correction: 84 tests, complete-mode
53-record comparison, and nine durable mutation inputs/CLI receipts. Earlier
73-test and temporary-artifact sections are preserved development history.

## Scope and authorship

This author took over the previously independently authored checker foundation
and its three existing tests. This author also wrote the Candidate A evaluator
and agreement harnesses. The final implementation therefore has a shared-author
history with the model; it must not be described as independent non-author
source review, cross-vendor corroboration, a Council result, or exhaustive
correspondence. The checker remains a separate code implementation: it imports
neither exporter, Quint evaluator/projection, nor production intent effect helper.
Its semantic oracle is the pinned Python Core, with independently literal
Python fixture tables, not model-produced expected results.

The takeover owns only checker, checker tests, this report and new stage snapshots.
No Core, Quint, exporter, prior report, or archived evidence was changed by this
author. No commit was made. Root-created corpus archives are read-only inputs.

## Implemented boundary

The checker independently decodes all sixteen node IDs, constructor fields,
ordered cases and exact edge IDs, preserves absent choices versus stored zero,
and admits only finite raw time enums, input values, stored choice values, and
the stated account potential bound. All edges must decrease rank; cycles produce
DecodeError before recursive construction. Exact symbolic fixture tables compare
every node and edge, including unused Close nodes and the root ID. Canonical swap
also must equal the frozen canonical_swap constructor at runtime. Actual Python
evaluation starts from the recorded current continuation in the independent
fixture table. Retained reference object identity determines the exact successor
ID, while rejected results must retain the original ID.

Accepted/error, complete state, minimum time, ordered payments, full optional
warning payloads and reductions are compared separately for raw and projected
results. Projected string choice names and integer time have their own strict
conversion. Effects are independently derived from reference pre-input payments,
accepted deposit, and post-input payments in that order; the pre-input payments
must explicitly be a prefix of the successful reference result. Rejections have
no effects. Diagnostics never count as Core rejections or successful comparisons.

Successful check_document requires input_root. Every case binds to its relative
input path, exact raw-file digest, state and record indices, exact copied request,
bare raw result, projection and effects. Every repeated-prefix occurrence must
be listed once; missing, duplicate, conflicting or shrinking prefixes fail.
The generator is read from the sole ITF state variable, not trusted from case ID.
ITF metadata must declare format ITF and the exact generator source. Diagnostic
fixture/case IDs are checked against the fixed independent catalog. JSON duplicate
keys, malformed enums and invalid path traversal fail closed. All declared source
pins, including generators/exporter, are checked against files. Imported Python
oracle function-origin bytes are also pinned, so a valid archived source_root
cannot validate a different loaded implementation.

These checks establish finite record equality and declared provenance linkage.
They do not authenticate who ran Quint, independently prove the declared command
generated a raw file, or establish all-program equivalence. A malicious party
able to change both a raw artifact and its declared digest still faces semantic
comparison, but file hashes are not execution attestation. Unrelated raw ledger
semantics are not reimplemented by this checker.

## Test-first evidence classification

Three genuine failing-first stages are preserved:

1. Initial takeover: 38 failures and 9 passes against the copied inherited checker.
   Failures include admission gaps, cycle RecursionError, and rejection of valid
   exporter-schema records. The first attempted run additionally had one snapshot
   location test error; its 39-failure output is preserved as a development mistake,
   not an additional semantic RED. The corrected test explicitly supplied source_root.
   The corrected source was preserved separately before logic.
2. Imported-oracle pinning: three controls passed through relocated same-behavior
   oracle wrappers before the fix; all three tests failed as intended.
3. Root-review alignment: missing/wrong ITF metadata, invalid reference payment
   prefix and runtime canonical-fixture binding produced five failing controls
   before the corresponding code changes.

The other controls added after existing fixes are regression/mutation verification,
not retroactively claimed original RED. In particular, named semantic controls
first prove an accepted baseline, retain the original workload/source pins, change
one observed field/mapping, and update only the isolated test raw copy and digest.
They require a semantic comparison difference and nonzero CLI exit, not merely a
provenance rejection. The asset/continuation controls mutate projection mappings.
The critical continuation control changes an actual unexpired When to Close.
Separate stronger exact-ID checks also reject alternative Close IDs.

Each copied checker snapshot imports the immutable Python sources pinned below.
The tests load archived source-bound actual ITFs. Snapshot paths are selected by
S02_CHECKER_PATH while tests run from their real repository location; snapshots
are not falsely claimed standalone independent Python packages.

## Final source and corpus hashes

```text
1cbcf64b9e5deab8976e238020e849a2e94c1956a2adfc70aaae122987218be0  scripts/check_s02_candidate_a_correspondence.py
8ed0b7c61724f48e52b1faf60fe18c24f542fa6aba48bcd87fd191d3e85bb88a  tests/test_s02_candidate_a_correspondence.py
1b1a04cae642591c58a54f207b3b8dc6d6cef4cf2f3aee82b865a708ab8311a9  evidence/s02-model-comparison/candidate-a-correspondence/export/cases.json
```

Snapshots, source-binding manifest, and frozen Python sources:

```text
9f4dbfc0eaf589ab35604bc2f5870527bf250b550f10c1cc7b2fafcef5266cd6  .superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py
ba50088a1b97246a2e686d940d77c507f2700ac5012211d0cf08b6a89b751639  .superpowers/sdd/candidate-a-checker-takeover-stages/red/test_corrected_location.py
9c1f014af4fda3e975216450e09f7e1e5418f6ca677ae523acfe7bc3cd379f5f  .superpowers/sdd/candidate-a-checker-takeover-stages/red/test_s02_candidate_a_correspondence.py
c4c1a218b12413117bbc80311ce547dfb94559d66775cb9f5c3033b67c4d7bbb  .superpowers/sdd/candidate-a-checker-takeover-stages/oracle-red/check_s02_candidate_a_correspondence.py
2e56db8639e5394997adc1fffc96a206e519f7eeb4cd0ae74c432bfc3de0c493  .superpowers/sdd/candidate-a-checker-takeover-stages/oracle-red/test_s02_candidate_a_correspondence.py
36df3745b8f38c5d4fd40dad544e0545c8ffc59f91bb0c4e03f7c89ad0281ebd  .superpowers/sdd/candidate-a-checker-takeover-stages/review-red/check_s02_candidate_a_correspondence.py
8ed0b7c61724f48e52b1faf60fe18c24f542fa6aba48bcd87fd191d3e85bb88a  .superpowers/sdd/candidate-a-checker-takeover-stages/review-red/test_s02_candidate_a_correspondence.py
1010537818383d2d668a4f891eb8de2f7cc8f3d43650b7f60c1f29b51a2a9fac  evidence/s02-model-comparison/candidate-a-correspondence/export/inputs/source-bindings.json
564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b  moriarty/core.py
82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797  moriarty/swap.py
```

Repository HEAD observed at handoff (other agents may commit concurrently):

```text
02a4e94b604760e54b04f94c0dc089ede0de938b
```

Canonical regression corpus is
evidence/s02-model-comparison/candidate-a-correspondence/export.
Tests default to this archive and fail if it is absent. S02_CORPUS_DIR is only an
explicit development override. The actual 53-case export comprises 24 literal
diagnostic requests and 29 stateful transaction records across 13 target traces.
Source declarations and all fourteen input hashes are in inputs/source-bindings.json.

## Initial attempt with disclosed snapshot-location error

```sh
PYTHONDONTWRITEBYTECODE=1 S02_CHECKER_PATH=.superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py .venv/bin/python -m pytest tests/test_s02_candidate_a_correspondence.py -q --tb=short
```

Terminal exit: 1.

```text
..FFFF..FFF.FFFFF...FFFFFFFFFFFFFFFFFFFFFFFFFFF                          [100%]
=================================== FAILURES ===================================
________________________ test_empty_corpus_is_rejected _________________________
tests/test_s02_candidate_a_correspondence.py:39: in test_empty_corpus_is_rejected
    report = MODULE.check_document({"schema_version": 1, "source_pins": MODULE.PINNED, "cases": []})
             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py:578: in check_document
    if hashlib.sha256((source_root / name).read_bytes()).hexdigest() != expected:
                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
../../../.local/share/uv/python/cpython-3.13.14-linux-x86_64-gnu/lib/python3.13/pathlib/_abc.py:625: in read_bytes
    with self.open(mode='rb') as f:
         ^^^^^^^^^^^^^^^^^^^^
../../../.local/share/uv/python/cpython-3.13.14-linux-x86_64-gnu/lib/python3.13/pathlib/_local.py:537: in open
    return io.open(self, mode, buffering, encoding, errors, newline)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   FileNotFoundError: [Errno 2] No such file or directory: '/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/candidate-a-checker-takeover-stages/moriarty/core.py'
____________________ test_raw_time_requires_finite_enum[0] _____________________
tests/test_s02_candidate_a_correspondence.py:100: in test_raw_time_requires_finite_enum
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
___________________ test_raw_time_requires_finite_enum[raw1] ___________________
tests/test_s02_candidate_a_correspondence.py:100: in test_raw_time_requires_finite_enum
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
___________________ test_raw_time_requires_finite_enum[raw2] ___________________
tests/test_s02_candidate_a_correspondence.py:100: in test_raw_time_requires_finite_enum
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_____________________ test_deposit_input_finite_domain[-2] _____________________
tests/test_s02_candidate_a_correspondence.py:108: in test_deposit_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_____________________ test_deposit_input_finite_domain[3] ______________________
tests/test_s02_candidate_a_correspondence.py:108: in test_deposit_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_____________________ test_deposit_input_finite_domain[22] _____________________
tests/test_s02_candidate_a_correspondence.py:108: in test_deposit_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_____________________ test_choice_input_finite_domain[-2] ______________________
tests/test_s02_candidate_a_correspondence.py:114: in test_choice_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
______________________ test_choice_input_finite_domain[3] ______________________
tests/test_s02_candidate_a_correspondence.py:114: in test_choice_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_____________________ test_choice_input_finite_domain[100] _____________________
tests/test_s02_candidate_a_correspondence.py:114: in test_choice_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
__________________________ test_state_potential_bound __________________________
tests/test_s02_candidate_a_correspondence.py:122: in test_state_potential_bound
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_______________________ test_state_stored_choice_domain ________________________
tests/test_s02_candidate_a_correspondence.py:129: in test_state_stored_choice_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
___________________ test_cycle_is_explicit_domain_diagnostic ___________________
tests/test_s02_candidate_a_correspondence.py:158: in test_cycle_is_explicit_domain_diagnostic
    MODULE._decode_program(raw, "program")
.superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py:200: in _decode_program
    return build(root), {node: build(node) for node in NODES}, root
           ^^^^^^^^^^^
.superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py:196: in build
    cases.append(Case(_action(case_data["caseAction"], path + ".caseAction"), build(_node(case_data["continuation"], path + ".continuation"))))
                                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py:181: in build
    result = Pay(_account(data["account"], path + ".account"), _party(data["payee"], path + ".payee"), _constant(data["amount"], path + ".amount"), build(_node(data["continuation"], path + ".continuation")))
                                                                                                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py:181: in build
    result = Pay(_account(data["account"], path + ".account"), _party(data["payee"], path + ".payee"), _constant(data["amount"], path + ".amount"), build(_node(data["continuation"], path + ".continuation")))
                                                                                                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   RecursionError: maximum recursion depth exceeded
!!! Recursion detected (same locals & position)
__ test_actual_installment_records_and_repeated_prefix_provenance[two-fills] ___
tests/test_s02_candidate_a_correspondence.py:164: in test_actual_installment_records_and_repeated_prefix_provenance
    assert result.ok, result.differences
E   AssertionError: ('cases[0].trace_id: unsupported trace variable',)
E   assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
__ test_actual_installment_records_and_repeated_prefix_provenance[refund-ten] __
tests/test_s02_candidate_a_correspondence.py:164: in test_actual_installment_records_and_repeated_prefix_provenance
    assert result.ok, result.differences
E   AssertionError: ('cases[0].trace_id: unsupported trace variable',)
E   assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
_ test_actual_installment_records_and_repeated_prefix_provenance[refund-five] __
tests/test_s02_candidate_a_correspondence.py:164: in test_actual_installment_records_and_repeated_prefix_provenance
    assert result.ok, result.differences
E   AssertionError: ('cases[0].trace_id: unsupported trace variable',)
E   assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
_ test_actual_installment_records_and_repeated_prefix_provenance[residual-deadline-cleanup] _
tests/test_s02_candidate_a_correspondence.py:164: in test_actual_installment_records_and_repeated_prefix_provenance
    assert result.ok, result.differences
E   AssertionError: ('cases[0].trace_id: unsupported trace variable',)
E   assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
___________________ test_full_check_requires_raw_input_root ____________________
tests/test_s02_candidate_a_correspondence.py:170: in test_full_check_requires_raw_input_root
    assert not result.ok and "input_root" in str(result.differences)
E   assert (not False and 'input_root' in "('cases[0].trace_id: unsupported trace variable',)")
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +  and   "('cases[0].trace_id: unsupported trace variable',)" = str(('cases[0].trace_id: unsupported trace variable',))
E    +    where ('cases[0].trace_id: unsupported trace variable',) = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).differences
_______________ test_raw_provenance_field_substitution[request] ________________
tests/test_s02_candidate_a_correspondence.py:176: in test_raw_provenance_field_substitution
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_field_subs0'))
________________ test_raw_provenance_field_substitution[result] ________________
tests/test_s02_candidate_a_correspondence.py:176: in test_raw_provenance_field_substitution
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_field_subs1'))
______________ test_raw_provenance_field_substitution[projection] ______________
tests/test_s02_candidate_a_correspondence.py:176: in test_raw_provenance_field_substitution
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_field_subs2'))
_______________ test_raw_provenance_field_substitution[effects] ________________
tests/test_s02_candidate_a_correspondence.py:176: in test_raw_provenance_field_substitution
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_field_subs3'))
______________________ test_raw_provenance_controls[hash] ______________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_controls_h0'))
_____________________ test_raw_provenance_controls[state] ______________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_controls_s0'))
______________________ test_raw_provenance_controls[step] ______________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_controls_s1'))
______________________ test_raw_provenance_controls[path] ______________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_controls_p0'))
____________________ test_raw_provenance_controls[absolute] ____________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_controls_a0'))
___________________ test_raw_provenance_controls[traversal] ____________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_controls_t0'))
___________________ test_raw_provenance_controls[duplicate] ____________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_controls_d0'))
______________________ test_raw_provenance_controls[omit] ______________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_controls_o0'))
__________________ test_raw_provenance_controls[wrong_trace] ___________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_raw_provenance_controls_w0'))
____________________ test_exact_symbolic_node_table[unused] ____________________
tests/test_s02_candidate_a_correspondence.py:206: in test_exact_symbolic_node_table
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_exact_symbolic_node_table0'))
_____________________ test_exact_symbolic_node_table[edge] _____________________
tests/test_s02_candidate_a_correspondence.py:206: in test_exact_symbolic_node_table
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_exact_symbolic_node_table1'))
_____________________ test_exact_symbolic_node_table[root] _____________________
tests/test_s02_candidate_a_correspondence.py:206: in test_exact_symbolic_node_table
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_exact_symbolic_node_table2'))
______________ test_exact_symbolic_node_table[projection_unused] _______________
tests/test_s02_candidate_a_correspondence.py:206: in test_exact_symbolic_node_table
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_exact_symbolic_node_table3'))
_______________ test_exact_symbolic_node_table[projection_edge] ________________
tests/test_s02_candidate_a_correspondence.py:206: in test_exact_symbolic_node_table
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_exact_symbolic_node_table4'))
____________________ test_exact_close_node_identity[result] ____________________
tests/test_s02_candidate_a_correspondence.py:223: in test_exact_close_node_identity
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_exact_close_node_identity0'))
__________________ test_exact_close_node_identity[projection] __________________
tests/test_s02_candidate_a_correspondence.py:223: in test_exact_close_node_identity
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_exact_close_node_identity1'))
_____________________ test_optional_generator_pin_checked ______________________
tests/test_s02_candidate_a_correspondence.py:231: in test_optional_generator_pin_checked
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-95/test_optional_generator_pin_ch0'))
=========================== short test summary info ============================
FAILED tests/test_s02_candidate_a_correspondence.py::test_empty_corpus_is_rejected
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_time_requires_finite_enum[0]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_time_requires_finite_enum[raw1]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_time_requires_finite_enum[raw2]
FAILED tests/test_s02_candidate_a_correspondence.py::test_deposit_input_finite_domain[-2]
FAILED tests/test_s02_candidate_a_correspondence.py::test_deposit_input_finite_domain[3]
FAILED tests/test_s02_candidate_a_correspondence.py::test_deposit_input_finite_domain[22]
FAILED tests/test_s02_candidate_a_correspondence.py::test_choice_input_finite_domain[-2]
FAILED tests/test_s02_candidate_a_correspondence.py::test_choice_input_finite_domain[3]
FAILED tests/test_s02_candidate_a_correspondence.py::test_choice_input_finite_domain[100]
FAILED tests/test_s02_candidate_a_correspondence.py::test_state_potential_bound
FAILED tests/test_s02_candidate_a_correspondence.py::test_state_stored_choice_domain
FAILED tests/test_s02_candidate_a_correspondence.py::test_cycle_is_explicit_domain_diagnostic
FAILED tests/test_s02_candidate_a_correspondence.py::test_actual_installment_records_and_repeated_prefix_provenance[two-fills]
FAILED tests/test_s02_candidate_a_correspondence.py::test_actual_installment_records_and_repeated_prefix_provenance[refund-ten]
FAILED tests/test_s02_candidate_a_correspondence.py::test_actual_installment_records_and_repeated_prefix_provenance[refund-five]
FAILED tests/test_s02_candidate_a_correspondence.py::test_actual_installment_records_and_repeated_prefix_provenance[residual-deadline-cleanup]
FAILED tests/test_s02_candidate_a_correspondence.py::test_full_check_requires_raw_input_root
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_field_substitution[request]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_field_substitution[result]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_field_substitution[projection]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_field_substitution[effects]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[hash]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[state]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[step]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[path]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[absolute]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[traversal]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[duplicate]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[omit]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[wrong_trace]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_symbolic_node_table[unused]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_symbolic_node_table[edge]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_symbolic_node_table[root]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_symbolic_node_table[projection_unused]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_symbolic_node_table[projection_edge]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_close_node_identity[result]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_close_node_identity[projection]
FAILED tests/test_s02_candidate_a_correspondence.py::test_optional_generator_pin_checked
39 failed, 8 passed in 0.29s
```

## Corrected takeover behavioral RED

```sh
PYTHONDONTWRITEBYTECODE=1 S02_CHECKER_PATH=.superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py .venv/bin/python -m pytest tests/test_s02_candidate_a_correspondence.py -q --tb=short
```

Terminal exit: 1.

```text
...FFF..FFF.FFFFF...FFFFFFFFFFFFFFFFFFFFFFFFFFF                          [100%]
=================================== FAILURES ===================================
____________________ test_raw_time_requires_finite_enum[0] _____________________
tests/test_s02_candidate_a_correspondence.py:100: in test_raw_time_requires_finite_enum
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
___________________ test_raw_time_requires_finite_enum[raw1] ___________________
tests/test_s02_candidate_a_correspondence.py:100: in test_raw_time_requires_finite_enum
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
___________________ test_raw_time_requires_finite_enum[raw2] ___________________
tests/test_s02_candidate_a_correspondence.py:100: in test_raw_time_requires_finite_enum
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_____________________ test_deposit_input_finite_domain[-2] _____________________
tests/test_s02_candidate_a_correspondence.py:108: in test_deposit_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_____________________ test_deposit_input_finite_domain[3] ______________________
tests/test_s02_candidate_a_correspondence.py:108: in test_deposit_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_____________________ test_deposit_input_finite_domain[22] _____________________
tests/test_s02_candidate_a_correspondence.py:108: in test_deposit_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_____________________ test_choice_input_finite_domain[-2] ______________________
tests/test_s02_candidate_a_correspondence.py:114: in test_choice_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
______________________ test_choice_input_finite_domain[3] ______________________
tests/test_s02_candidate_a_correspondence.py:114: in test_choice_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_____________________ test_choice_input_finite_domain[100] _____________________
tests/test_s02_candidate_a_correspondence.py:114: in test_choice_input_finite_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
__________________________ test_state_potential_bound __________________________
tests/test_s02_candidate_a_correspondence.py:122: in test_state_potential_bound
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
_______________________ test_state_stored_choice_domain ________________________
tests/test_s02_candidate_a_correspondence.py:129: in test_state_stored_choice_domain
    with pytest.raises(MODULE.DecodeError):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
___________________ test_cycle_is_explicit_domain_diagnostic ___________________
tests/test_s02_candidate_a_correspondence.py:158: in test_cycle_is_explicit_domain_diagnostic
    MODULE._decode_program(raw, "program")
.superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py:200: in _decode_program
    return build(root), {node: build(node) for node in NODES}, root
           ^^^^^^^^^^^
.superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py:196: in build
    cases.append(Case(_action(case_data["caseAction"], path + ".caseAction"), build(_node(case_data["continuation"], path + ".continuation"))))
                                                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py:181: in build
    result = Pay(_account(data["account"], path + ".account"), _party(data["payee"], path + ".payee"), _constant(data["amount"], path + ".amount"), build(_node(data["continuation"], path + ".continuation")))
                                                                                                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
.superpowers/sdd/candidate-a-checker-takeover-stages/red/check_s02_candidate_a_correspondence.py:181: in build
    result = Pay(_account(data["account"], path + ".account"), _party(data["payee"], path + ".payee"), _constant(data["amount"], path + ".amount"), build(_node(data["continuation"], path + ".continuation")))
                                                                                                                                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   RecursionError: maximum recursion depth exceeded
!!! Recursion detected (same locals & position)
__ test_actual_installment_records_and_repeated_prefix_provenance[two-fills] ___
tests/test_s02_candidate_a_correspondence.py:164: in test_actual_installment_records_and_repeated_prefix_provenance
    assert result.ok, result.differences
E   AssertionError: ('cases[0].trace_id: unsupported trace variable',)
E   assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
__ test_actual_installment_records_and_repeated_prefix_provenance[refund-ten] __
tests/test_s02_candidate_a_correspondence.py:164: in test_actual_installment_records_and_repeated_prefix_provenance
    assert result.ok, result.differences
E   AssertionError: ('cases[0].trace_id: unsupported trace variable',)
E   assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
_ test_actual_installment_records_and_repeated_prefix_provenance[refund-five] __
tests/test_s02_candidate_a_correspondence.py:164: in test_actual_installment_records_and_repeated_prefix_provenance
    assert result.ok, result.differences
E   AssertionError: ('cases[0].trace_id: unsupported trace variable',)
E   assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
_ test_actual_installment_records_and_repeated_prefix_provenance[residual-deadline-cleanup] _
tests/test_s02_candidate_a_correspondence.py:164: in test_actual_installment_records_and_repeated_prefix_provenance
    assert result.ok, result.differences
E   AssertionError: ('cases[0].trace_id: unsupported trace variable',)
E   assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
___________________ test_full_check_requires_raw_input_root ____________________
tests/test_s02_candidate_a_correspondence.py:170: in test_full_check_requires_raw_input_root
    assert not result.ok and "input_root" in str(result.differences)
E   assert (not False and 'input_root' in "('cases[0].trace_id: unsupported trace variable',)")
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +  and   "('cases[0].trace_id: unsupported trace variable',)" = str(('cases[0].trace_id: unsupported trace variable',))
E    +    where ('cases[0].trace_id: unsupported trace variable',) = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).differences
_______________ test_raw_provenance_field_substitution[request] ________________
tests/test_s02_candidate_a_correspondence.py:176: in test_raw_provenance_field_substitution
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_field_subs0'))
________________ test_raw_provenance_field_substitution[result] ________________
tests/test_s02_candidate_a_correspondence.py:176: in test_raw_provenance_field_substitution
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_field_subs1'))
______________ test_raw_provenance_field_substitution[projection] ______________
tests/test_s02_candidate_a_correspondence.py:176: in test_raw_provenance_field_substitution
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_field_subs2'))
_______________ test_raw_provenance_field_substitution[effects] ________________
tests/test_s02_candidate_a_correspondence.py:176: in test_raw_provenance_field_substitution
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_field_subs3'))
______________________ test_raw_provenance_controls[hash] ______________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_controls_h0'))
_____________________ test_raw_provenance_controls[state] ______________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_controls_s0'))
______________________ test_raw_provenance_controls[step] ______________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_controls_s1'))
______________________ test_raw_provenance_controls[path] ______________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_controls_p0'))
____________________ test_raw_provenance_controls[absolute] ____________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_controls_a0'))
___________________ test_raw_provenance_controls[traversal] ____________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_controls_t0'))
___________________ test_raw_provenance_controls[duplicate] ____________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_controls_d0'))
______________________ test_raw_provenance_controls[omit] ______________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_controls_o0'))
__________________ test_raw_provenance_controls[wrong_trace] ___________________
tests/test_s02_candidate_a_correspondence.py:189: in test_raw_provenance_controls
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_raw_provenance_controls_w0'))
____________________ test_exact_symbolic_node_table[unused] ____________________
tests/test_s02_candidate_a_correspondence.py:206: in test_exact_symbolic_node_table
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_exact_symbolic_node_table0'))
_____________________ test_exact_symbolic_node_table[edge] _____________________
tests/test_s02_candidate_a_correspondence.py:206: in test_exact_symbolic_node_table
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_exact_symbolic_node_table1'))
_____________________ test_exact_symbolic_node_table[root] _____________________
tests/test_s02_candidate_a_correspondence.py:206: in test_exact_symbolic_node_table
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_exact_symbolic_node_table2'))
______________ test_exact_symbolic_node_table[projection_unused] _______________
tests/test_s02_candidate_a_correspondence.py:206: in test_exact_symbolic_node_table
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_exact_symbolic_node_table3'))
_______________ test_exact_symbolic_node_table[projection_edge] ________________
tests/test_s02_candidate_a_correspondence.py:206: in test_exact_symbolic_node_table
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_exact_symbolic_node_table4'))
____________________ test_exact_close_node_identity[result] ____________________
tests/test_s02_candidate_a_correspondence.py:223: in test_exact_close_node_identity
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_exact_close_node_identity0'))
__________________ test_exact_close_node_identity[projection] __________________
tests/test_s02_candidate_a_correspondence.py:223: in test_exact_close_node_identity
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::0', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 0, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_exact_close_node_identity1'))
_____________________ test_optional_generator_pin_checked ______________________
tests/test_s02_candidate_a_correspondence.py:231: in test_optional_generator_pin_checked
    assert checked(document, tmp_path).ok
E   AssertionError: assert False
E    +  where False = CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)).ok
E    +    where CheckReport(ok=False, differences=('cases[0].trace_id: unsupported trace variable',)) = checked({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...d': 'trace.itf.json::1', 'fixture_id': 'installment-two-when-v1', 'trace_id': 'trace.itf.json', 'step_index': 1, ...}]}, PosixPath('/tmp/pytest-of-charl/pytest-97/test_optional_generator_pin_ch0'))
=========================== short test summary info ============================
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_time_requires_finite_enum[0]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_time_requires_finite_enum[raw1]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_time_requires_finite_enum[raw2]
FAILED tests/test_s02_candidate_a_correspondence.py::test_deposit_input_finite_domain[-2]
FAILED tests/test_s02_candidate_a_correspondence.py::test_deposit_input_finite_domain[3]
FAILED tests/test_s02_candidate_a_correspondence.py::test_deposit_input_finite_domain[22]
FAILED tests/test_s02_candidate_a_correspondence.py::test_choice_input_finite_domain[-2]
FAILED tests/test_s02_candidate_a_correspondence.py::test_choice_input_finite_domain[3]
FAILED tests/test_s02_candidate_a_correspondence.py::test_choice_input_finite_domain[100]
FAILED tests/test_s02_candidate_a_correspondence.py::test_state_potential_bound
FAILED tests/test_s02_candidate_a_correspondence.py::test_state_stored_choice_domain
FAILED tests/test_s02_candidate_a_correspondence.py::test_cycle_is_explicit_domain_diagnostic
FAILED tests/test_s02_candidate_a_correspondence.py::test_actual_installment_records_and_repeated_prefix_provenance[two-fills]
FAILED tests/test_s02_candidate_a_correspondence.py::test_actual_installment_records_and_repeated_prefix_provenance[refund-ten]
FAILED tests/test_s02_candidate_a_correspondence.py::test_actual_installment_records_and_repeated_prefix_provenance[refund-five]
FAILED tests/test_s02_candidate_a_correspondence.py::test_actual_installment_records_and_repeated_prefix_provenance[residual-deadline-cleanup]
FAILED tests/test_s02_candidate_a_correspondence.py::test_full_check_requires_raw_input_root
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_field_substitution[request]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_field_substitution[result]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_field_substitution[projection]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_field_substitution[effects]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[hash]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[state]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[step]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[path]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[absolute]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[traversal]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[duplicate]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[omit]
FAILED tests/test_s02_candidate_a_correspondence.py::test_raw_provenance_controls[wrong_trace]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_symbolic_node_table[unused]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_symbolic_node_table[edge]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_symbolic_node_table[root]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_symbolic_node_table[projection_unused]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_symbolic_node_table[projection_edge]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_close_node_identity[result]
FAILED tests/test_s02_candidate_a_correspondence.py::test_exact_close_node_identity[projection]
FAILED tests/test_s02_candidate_a_correspondence.py::test_optional_generator_pin_checked
38 failed, 9 passed in 0.27s
```

## First implementation GREEN

```sh
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -m pytest tests/test_s02_candidate_a_correspondence.py -q --tb=short
```

Terminal exit: 0.

```text
...............................................                          [100%]
47 passed in 0.22s
```

## Named controls added after existing fixes

```sh
PYTHONDONTWRITEBYTECODE=1 S02_CORPUS_DIR=.superpowers/sdd/candidate-a-export-stages/final .venv/bin/python -m pytest tests/test_s02_candidate_a_correspondence.py -q --tb=short
```

Terminal exit: 0.

```text
................................................................         [100%]
64 passed in 0.93s
```

## Imported reference origin RED

```sh
PYTHONDONTWRITEBYTECODE=1 S02_CHECKER_PATH=.superpowers/sdd/candidate-a-checker-takeover-stages/oracle-red/check_s02_candidate_a_correspondence.py .venv/bin/python -m pytest tests/test_s02_candidate_a_correspondence.py -q --tb=short -k imported_oracle_origin
```

Terminal exit: 1.

```text
FFF                                                                      [100%]
=================================== FAILURES ===================================
__________ test_imported_oracle_origin_is_pinned[compute_transaction] __________
tests/test_s02_candidate_a_correspondence.py:397: in test_imported_oracle_origin_is_pinned
    assert not result.ok and "imported reference" in str(result.differences)
E   assert (not True)
E    +  where True = CheckReport(ok=True, differences=()).ok
_________ test_imported_oracle_origin_is_pinned[reduce_to_quiescence] __________
tests/test_s02_candidate_a_correspondence.py:397: in test_imported_oracle_origin_is_pinned
    assert not result.ok and "imported reference" in str(result.differences)
E   assert (not True)
E    +  where True = CheckReport(ok=True, differences=()).ok
____________ test_imported_oracle_origin_is_pinned[canonical_swap] _____________
tests/test_s02_candidate_a_correspondence.py:397: in test_imported_oracle_origin_is_pinned
    assert not result.ok and "imported reference" in str(result.differences)
E   assert (not True)
E    +  where True = CheckReport(ok=True, differences=()).ok
=========================== short test summary info ============================
FAILED tests/test_s02_candidate_a_correspondence.py::test_imported_oracle_origin_is_pinned[compute_transaction]
FAILED tests/test_s02_candidate_a_correspondence.py::test_imported_oracle_origin_is_pinned[reduce_to_quiescence]
FAILED tests/test_s02_candidate_a_correspondence.py::test_imported_oracle_origin_is_pinned[canonical_swap]
3 failed, 65 deselected in 0.08s
```

## Imported reference origin GREEN

```sh
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -m pytest tests/test_s02_candidate_a_correspondence.py -q --tb=short
```

Terminal exit: 0.

```text
....................................................................     [100%]
68 passed in 0.95s
```

## Review alignment RED

```sh
PYTHONDONTWRITEBYTECODE=1 S02_CHECKER_PATH=.superpowers/sdd/candidate-a-checker-takeover-stages/review-red/check_s02_candidate_a_correspondence.py .venv/bin/python -m pytest tests/test_s02_candidate_a_correspondence.py -q --tb=short -k 'itf_metadata or reference_pre_payments or runtime_literal_swap'
```

Terminal exit: 1.

```text
FFFFF                                                                    [100%]
=================================== FAILURES ===================================
_____________ test_itf_metadata_binds_declared_generator[missing] ______________
tests/test_s02_candidate_a_correspondence.py:419: in test_itf_metadata_binds_declared_generator
    assert not result.ok and "meta" in str(result.differences)
E   assert (not True)
E    +  where True = CheckReport(ok=True, differences=()).ok
______________ test_itf_metadata_binds_declared_generator[format] ______________
tests/test_s02_candidate_a_correspondence.py:419: in test_itf_metadata_binds_declared_generator
    assert not result.ok and "meta" in str(result.differences)
E   assert (not True)
E    +  where True = CheckReport(ok=True, differences=()).ok
______________ test_itf_metadata_binds_declared_generator[source] ______________
tests/test_s02_candidate_a_correspondence.py:419: in test_itf_metadata_binds_declared_generator
    assert not result.ok and "meta" in str(result.differences)
E   assert (not True)
E    +  where True = CheckReport(ok=True, differences=()).ok
__________ test_reference_pre_payments_must_be_actual_payment_prefix ___________
tests/test_s02_candidate_a_correspondence.py:433: in test_reference_pre_payments_must_be_actual_payment_prefix
    with pytest.raises(MODULE.DecodeError, match="prefix"):
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
E   Failed: DID NOT RAISE DecodeError
______________________ test_runtime_literal_swap_binding _______________________
tests/test_s02_candidate_a_correspondence.py:443: in test_runtime_literal_swap_binding
    assert result and "canonical" in str(result)
E   assert ([])
=========================== short test summary info ============================
FAILED tests/test_s02_candidate_a_correspondence.py::test_itf_metadata_binds_declared_generator[missing]
FAILED tests/test_s02_candidate_a_correspondence.py::test_itf_metadata_binds_declared_generator[format]
FAILED tests/test_s02_candidate_a_correspondence.py::test_itf_metadata_binds_declared_generator[source]
FAILED tests/test_s02_candidate_a_correspondence.py::test_reference_pre_payments_must_be_actual_payment_prefix
FAILED tests/test_s02_candidate_a_correspondence.py::test_runtime_literal_swap_binding
5 failed, 68 deselected in 0.11s
```

## Final default-path 73-test GREEN

```sh
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -m pytest tests/test_s02_candidate_a_correspondence.py -q --tb=short
```

Terminal exit: 0.

```text
........................................................................ [ 98%]
.                                                                        [100%]
73 passed in 1.00s
```

## Final actual 53-case comparison

```sh
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=. .venv/bin/python scripts/check_s02_candidate_a_correspondence.py --cases evidence/s02-model-comparison/candidate-a-correspondence/export/cases.json --input-root evidence/s02-model-comparison/candidate-a-correspondence/export/inputs --source-root evidence/s02-model-comparison/candidate-a-correspondence/export/source
```

Terminal exit: 0.

```text
(no output)
```

## Exact named mutation terminal receipts

The following diagnostic runner calls the named tests unchanged and records the
real CLI child-process command, exit and output. Its temporary mutation files
were removed by TemporaryDirectory afterwards; the recorded hashes describe those
observed bytes, not archived files. The deterministic mutations and pinned original
corpus remain executable in the test source. The canonical raw corpus is not changed.
All nine controls exited one with the expected differing field, stderr empty.

## Named semantic mutation CLI observations

```sh
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python - <<'PY'
import importlib.util,json,tempfile,hashlib
from pathlib import Path
spec=importlib.util.spec_from_file_location("mutation_receipt_tests",Path("tests/test_s02_candidate_a_correspondence.py"))
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
original=m.subprocess.run
receipts=[]
def observed(*args,**kwargs):
    result=original(*args,**kwargs)
    command=args[0]
    case_path=Path(command[command.index("--cases")+1])
    doc=json.loads(case_path.read_text())
    receipts.append({"command":command,"exit_code":result.returncode,"stdout":result.stdout,"stderr":result.stderr,
                     "case_id":doc["cases"][0]["case_id"],"case_json_sha256":hashlib.sha256(case_path.read_bytes()).hexdigest(),
                     "raw_sha256":doc["cases"][0]["provenance"][0]["input_sha256"]})
    return result
m.subprocess.run=observed
names=["test_absent_choice_is_not_zero","test_continuation_mapping_mutation_rejected","test_payment_order_mutation_rejected",
       "test_asset_mapping_mutation_rejected","test_reduction_count_mutation_rejected","test_deposit_effect_omission_rejected",
       "test_deposit_insertion_order_mutation_rejected","test_partial_warning_payload_mutation_rejected","test_deadline_priority_mutation_rejected"]
for name in names:
    with tempfile.TemporaryDirectory(prefix="s02-checker-control-") as directory:
        getattr(m,name)(Path(directory))
        receipts[-1]["control"]=name
print(json.dumps(receipts,indent=2))
PY
```

Terminal exit: 0.

```text
[
  {
    "command": [
      "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
      "--cases",
      "/tmp/s02-checker-control-0troz3i6/mutated-cases.json",
      "--input-root",
      "/tmp/s02-checker-control-0troz3i6",
      "--source-root",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
    ],
    "exit_code": 1,
    "stdout": "cases[0].result.state: differs from frozen core\n",
    "stderr": "",
    "case_id": "diagnostic-corpus.itf.json::absent-choice",
    "case_json_sha256": "aaff6c0b361d4a071fdf842b346a3fd8f04f884c838d754038cbc60b93a771ec",
    "raw_sha256": "e9af90817665c8d9295bcd0f6226086ac80f7b6b256dff458179108b7859be21",
    "control": "test_absent_choice_is_not_zero"
  },
  {
    "command": [
      "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
      "--cases",
      "/tmp/s02-checker-control-6yg9r47j/mutated-cases.json",
      "--input-root",
      "/tmp/s02-checker-control-6yg9r47j",
      "--source-root",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
    ],
    "exit_code": 1,
    "stdout": "cases[0].projection.result.state.continuation: differs from frozen core\ncases[0].projection.state.continuation.node: differs from exact evaluator node\n",
    "stderr": "",
    "case_id": "installment-two-fills.itf.json::0",
    "case_json_sha256": "c832028351123499b9a20cb1cabcf8c53e269f82b4f63a799ecb12d5487b9971",
    "raw_sha256": "0ff6ce72494b44f431eac41be35ccf67ec463b1afca7fd0e8fd5f98013db8ca2",
    "control": "test_continuation_mapping_mutation_rejected"
  },
  {
    "command": [
      "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
      "--cases",
      "/tmp/s02-checker-control-v66sa_is/mutated-cases.json",
      "--input-root",
      "/tmp/s02-checker-control-v66sa_is",
      "--source-root",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
    ],
    "exit_code": 1,
    "stdout": "cases[0].result.payments: differs from frozen core\n",
    "stderr": "",
    "case_id": "swap-settlement.itf.json::2",
    "case_json_sha256": "dea587c432d167c72c1da1c33dc243432dbe7c58e5a324a1c15c02c74592be40",
    "raw_sha256": "36600798bec8d65b09e00f077ff4d70a049ce65fbe10f328ba5bb4afb79c91b4",
    "control": "test_payment_order_mutation_rejected"
  },
  {
    "command": [
      "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
      "--cases",
      "/tmp/s02-checker-control-ooza4rlo/mutated-cases.json",
      "--input-root",
      "/tmp/s02-checker-control-ooza4rlo",
      "--source-root",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
    ],
    "exit_code": 1,
    "stdout": "cases[0].projection.payments[0]: payment asset differs from source account\n",
    "stderr": "",
    "case_id": "swap-settlement.itf.json::2",
    "case_json_sha256": "9e543dd5f7f0785ee5ce28f4a2e2c05fe8461a8bb5e87c73d2a54b5182d99a3c",
    "raw_sha256": "39b8fcb01b23c35fe806bf417b13bd5ad16f0d46f893999b61b91b70a21d9655",
    "control": "test_asset_mapping_mutation_rejected"
  },
  {
    "command": [
      "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
      "--cases",
      "/tmp/s02-checker-control-96kvgcyw/mutated-cases.json",
      "--input-root",
      "/tmp/s02-checker-control-96kvgcyw",
      "--source-root",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
    ],
    "exit_code": 1,
    "stdout": "cases[0].result.reductions: differs from frozen core\n",
    "stderr": "",
    "case_id": "diagnostic-corpus.itf.json::pre-deposit-post",
    "case_json_sha256": "99b7ed28f15d0b054c18e0ce908531df4c10f1f829dc20f532c88b81161e7a49",
    "raw_sha256": "cb21dfa9815c45642e6c68411c639c5e34ad20b7ebf844df315820f6fc37b297",
    "control": "test_reduction_count_mutation_rejected"
  },
  {
    "command": [
      "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
      "--cases",
      "/tmp/s02-checker-control-jof4nko0/mutated-cases.json",
      "--input-root",
      "/tmp/s02-checker-control-jof4nko0",
      "--source-root",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
    ],
    "exit_code": 1,
    "stdout": "cases[0].effects: differs from independently derived ordered effects\n",
    "stderr": "",
    "case_id": "diagnostic-corpus.itf.json::deposit-without-payment",
    "case_json_sha256": "fa5e5e369a2d9e2063bc35749ca3bbc2d9b5f6914b67dfd2c069cb63bd6e1224",
    "raw_sha256": "7141ba52563a66bdddd37feff0bc49b1149048577fe1f2b3a6e6f4c4360cf966",
    "control": "test_deposit_effect_omission_rejected"
  },
  {
    "command": [
      "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
      "--cases",
      "/tmp/s02-checker-control-vn_owgrc/mutated-cases.json",
      "--input-root",
      "/tmp/s02-checker-control-vn_owgrc",
      "--source-root",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
    ],
    "exit_code": 1,
    "stdout": "cases[0].effects: differs from independently derived ordered effects\n",
    "stderr": "",
    "case_id": "diagnostic-corpus.itf.json::pre-deposit-post",
    "case_json_sha256": "739d75362d3a6a63b705f01c0e68735a754657f54e2577f1639bef442afaf509",
    "raw_sha256": "c646e2c35a98072ce3e52a32e95e27db128e369a4a18431c659576adaeffac8f",
    "control": "test_deposit_insertion_order_mutation_rejected"
  },
  {
    "command": [
      "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
      "--cases",
      "/tmp/s02-checker-control-99k7pn8c/mutated-cases.json",
      "--input-root",
      "/tmp/s02-checker-control-99k7pn8c",
      "--source-root",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
    ],
    "exit_code": 1,
    "stdout": "cases[0].projection.result.warnings: differs from frozen core\n",
    "stderr": "",
    "case_id": "diagnostic-corpus.itf.json::warning-partial",
    "case_json_sha256": "41b8d45348d08d85806b691f4fe02fcd74b3a301c610d7b3ca6c92ae4fb91901",
    "raw_sha256": "36fef803a6a7f5261a29fdee6d05f565ef1963c47fb050c5a0993ad5f1886a97",
    "control": "test_partial_warning_payload_mutation_rejected"
  },
  {
    "command": [
      "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
      "--cases",
      "/tmp/s02-checker-control-xudhpihc/mutated-cases.json",
      "--input-root",
      "/tmp/s02-checker-control-xudhpihc",
      "--source-root",
      "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
    ],
    "exit_code": 1,
    "stdout": "cases[0].result.accepted: differs from frozen core\n",
    "stderr": "",
    "case_id": "installment-deadline-ten-cleanup.itf.json::0",
    "case_json_sha256": "924fcd499371d681a42d38d649bc959a1b6c78b055f8ecf2f6052916f2671368",
    "raw_sha256": "bcd8b50a753e36b6037efd5f0868bc8ae1c94d0f9e1ab0e6b18633b45d57f864",
    "control": "test_deadline_priority_mutation_rejected"
  }
]
```

## Handoff limits

All 73 focused tests and the 53-case actual comparison passed with terminal exit0.
The named mutation child commands returned exit1 as required. This author did not
run the full Python suite, rerun Quint, claim a new simulation result, or perform
an independent source review of their own model. Root source/evidence review and
integration disposition remain separate. The checker source is frozen at the
hash above pending that review.

# Final completeness correction and durable-mutation addendum

This addendum supersedes the earlier 73-test handoff and temporary-only mutation
limitation above. Earlier source hashes and command receipts remain historical,
not rewritten as if they had used the final complete-inventory mode.

Root and the non-author reviewer identified that selected-record success was not
the plan's missing-export gate. A fourth failing-first stage preserved ten failures
and one full-corpus positive: the old CLI accepted dropped cases, dropped input
plus its cases, missing/unlisted input pins/files, conflicting input/source digests,
invalid bindings schema/extra fields, absent bindings, and a one-record subset.

The selected-record Python API remains deliberate for isolated controls.
The CLI now defaults to complete inventory; only explicit --allow-subset selects
the weaker inventory scope. Both scopes retain full source, finite semantic and
raw-provenance validation. --report output labels its scope.

Complete mode strictly decodes input_root/source-bindings.json, requires its source
pins to equal those of the case document, verifies every declared ITF hash and the
exact on-disk ITF inventory, enumerates all final raw record positions, and requires
each position exactly once in the export. All repeated raw occurrences are still
checked. This closes missing exports relative to the declared manifest; the manifest
is not a cryptographically authenticated statement of which workloads ought to exist.
The retained final corpus tests still require exactly 53 records.

Final complete-mode source and fourth RED snapshot hashes:

```text
1cbcf64b9e5deab8976e238020e849a2e94c1956a2adfc70aaae122987218be0  .superpowers/sdd/candidate-a-checker-takeover-stages/inventory-red/check_s02_candidate_a_correspondence.py
eb6bdf21259b2f567b405469f57fe291ee71f98b850ffcac317f929196f20fd2  .superpowers/sdd/candidate-a-checker-takeover-stages/inventory-red/test_s02_candidate_a_correspondence.py
c6923d0e08ec0206dfddad70eaff5a3a9a2f4a160328802e0e69be965be90aec  scripts/check_s02_candidate_a_correspondence.py
eb6bdf21259b2f567b405469f57fe291ee71f98b850ffcac317f929196f20fd2  tests/test_s02_candidate_a_correspondence.py
```

## Complete-inventory failing-first RED

```sh
PYTHONDONTWRITEBYTECODE=1 S02_CHECKER_PATH=.superpowers/sdd/candidate-a-checker-takeover-stages/inventory-red/check_s02_candidate_a_correspondence.py .venv/bin/python -m pytest tests/test_s02_candidate_a_correspondence.py -q --tb=short -k 'complete_cli or subset_is_explicit'
```

Terminal exit: 1.

```text
FFFFFFFFFF.                                                              [100%]
=================================== FAILURES ===================================
___ test_complete_cli_rejects_missing_or_conflicting_inventory[dropped_case] ___
tests/test_s02_candidate_a_correspondence.py:494: in test_complete_cli_rejects_missing_or_conflicting_inventory
    assert result.returncode == 1, (result.returncode, result.stdout, result.stderr)
E   AssertionError: (0, '', '')
E   assert 0 == 1
E    +  where 0 = CompletedProcess(args=['/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python', '.superpowers/sdd/candidate...dit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'], returncode=0, stdout='', stderr='').returncode
__ test_complete_cli_rejects_missing_or_conflicting_inventory[dropped_input] ___
tests/test_s02_candidate_a_correspondence.py:494: in test_complete_cli_rejects_missing_or_conflicting_inventory
    assert result.returncode == 1, (result.returncode, result.stdout, result.stderr)
E   AssertionError: (0, '', '')
E   assert 0 == 1
E    +  where 0 = CompletedProcess(args=['/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python', '.superpowers/sdd/candidate...dit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'], returncode=0, stdout='', stderr='').returncode
_ test_complete_cli_rejects_missing_or_conflicting_inventory[dropped_input_pin] _
tests/test_s02_candidate_a_correspondence.py:494: in test_complete_cli_rejects_missing_or_conflicting_inventory
    assert result.returncode == 1, (result.returncode, result.stdout, result.stderr)
E   AssertionError: (0, '', '')
E   assert 0 == 1
E    +  where 0 = CompletedProcess(args=['/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python', '.superpowers/sdd/candidate...dit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'], returncode=0, stdout='', stderr='').returncode
__ test_complete_cli_rejects_missing_or_conflicting_inventory[unlisted_input] __
tests/test_s02_candidate_a_correspondence.py:494: in test_complete_cli_rejects_missing_or_conflicting_inventory
    assert result.returncode == 1, (result.returncode, result.stdout, result.stderr)
E   AssertionError: (0, '', '')
E   assert 0 == 1
E    +  where 0 = CompletedProcess(args=['/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python', '.superpowers/sdd/candidate...dit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'], returncode=0, stdout='', stderr='').returncode
____ test_complete_cli_rejects_missing_or_conflicting_inventory[input_hash] ____
tests/test_s02_candidate_a_correspondence.py:494: in test_complete_cli_rejects_missing_or_conflicting_inventory
    assert result.returncode == 1, (result.returncode, result.stdout, result.stderr)
E   AssertionError: (0, '', '')
E   assert 0 == 1
E    +  where 0 = CompletedProcess(args=['/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python', '.superpowers/sdd/candidate...dit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'], returncode=0, stdout='', stderr='').returncode
___ test_complete_cli_rejects_missing_or_conflicting_inventory[source_pins] ____
tests/test_s02_candidate_a_correspondence.py:494: in test_complete_cli_rejects_missing_or_conflicting_inventory
    assert result.returncode == 1, (result.returncode, result.stdout, result.stderr)
E   AssertionError: (0, '', '')
E   assert 0 == 1
E    +  where 0 = CompletedProcess(args=['/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python', '.superpowers/sdd/candidate...dit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'], returncode=0, stdout='', stderr='').returncode
______ test_complete_cli_rejects_missing_or_conflicting_inventory[schema] ______
tests/test_s02_candidate_a_correspondence.py:494: in test_complete_cli_rejects_missing_or_conflicting_inventory
    assert result.returncode == 1, (result.returncode, result.stdout, result.stderr)
E   AssertionError: (0, '', '')
E   assert 0 == 1
E    +  where 0 = CompletedProcess(args=['/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python', '.superpowers/sdd/candidate...dit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'], returncode=0, stdout='', stderr='').returncode
____ test_complete_cli_rejects_missing_or_conflicting_inventory[extra_key] _____
tests/test_s02_candidate_a_correspondence.py:494: in test_complete_cli_rejects_missing_or_conflicting_inventory
    assert result.returncode == 1, (result.returncode, result.stdout, result.stderr)
E   AssertionError: (0, '', '')
E   assert 0 == 1
E    +  where 0 = CompletedProcess(args=['/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python', '.superpowers/sdd/candidate...dit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'], returncode=0, stdout='', stderr='').returncode
_ test_complete_cli_rejects_missing_or_conflicting_inventory[missing_bindings] _
tests/test_s02_candidate_a_correspondence.py:494: in test_complete_cli_rejects_missing_or_conflicting_inventory
    assert result.returncode == 1, (result.returncode, result.stdout, result.stderr)
E   AssertionError: (0, '', '')
E   assert 0 == 1
E    +  where 0 = CompletedProcess(args=['/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python', '.superpowers/sdd/candidate...dit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'], returncode=0, stdout='', stderr='').returncode
____________________ test_subset_is_explicit_cli_mode_only _____________________
tests/test_s02_candidate_a_correspondence.py:501: in test_subset_is_explicit_cli_mode_only
    assert inventory_cli(document, source, tmp_path).returncode == 1
E   AssertionError: assert 0 == 1
E    +  where 0 = CompletedProcess(args=['/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python', '.superpowers/sdd/candidate...dit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'], returncode=0, stdout='', stderr='').returncode
E    +    where CompletedProcess(args=['/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python', '.superpowers/sdd/candidate...dit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'], returncode=0, stdout='', stderr='') = inventory_cli({'schema_version': 1, 'source_pins': {'moriarty/core.py': '564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f8...ime-before-state', 'fixture_id': 'canonical-swap-v1', 'trace_id': 'diagnostic-corpus.itf.json', 'step_index': 0, ...}]}, PosixPath('/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source'), PosixPath('/tmp/pytest-of-charl/pytest-108/test_subset_is_explicit_cli_mo0'))
=========================== short test summary info ============================
FAILED tests/test_s02_candidate_a_correspondence.py::test_complete_cli_rejects_missing_or_conflicting_inventory[dropped_case]
FAILED tests/test_s02_candidate_a_correspondence.py::test_complete_cli_rejects_missing_or_conflicting_inventory[dropped_input]
FAILED tests/test_s02_candidate_a_correspondence.py::test_complete_cli_rejects_missing_or_conflicting_inventory[dropped_input_pin]
FAILED tests/test_s02_candidate_a_correspondence.py::test_complete_cli_rejects_missing_or_conflicting_inventory[unlisted_input]
FAILED tests/test_s02_candidate_a_correspondence.py::test_complete_cli_rejects_missing_or_conflicting_inventory[input_hash]
FAILED tests/test_s02_candidate_a_correspondence.py::test_complete_cli_rejects_missing_or_conflicting_inventory[source_pins]
FAILED tests/test_s02_candidate_a_correspondence.py::test_complete_cli_rejects_missing_or_conflicting_inventory[schema]
FAILED tests/test_s02_candidate_a_correspondence.py::test_complete_cli_rejects_missing_or_conflicting_inventory[extra_key]
FAILED tests/test_s02_candidate_a_correspondence.py::test_complete_cli_rejects_missing_or_conflicting_inventory[missing_bindings]
FAILED tests/test_s02_candidate_a_correspondence.py::test_subset_is_explicit_cli_mode_only
10 failed, 1 passed, 73 deselected in 3.02s
```

## Final 84-test GREEN

```sh
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -m pytest tests/test_s02_candidate_a_correspondence.py -q --tb=short
```

Terminal exit: 0.

```text
........................................................................ [ 85%]
............                                                             [100%]
84 passed in 3.59s
```

## Final CLI complete 53-record comparison

```sh
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=. .venv/bin/python scripts/check_s02_candidate_a_correspondence.py --cases evidence/s02-model-comparison/candidate-a-correspondence/export/cases.json --input-root evidence/s02-model-comparison/candidate-a-correspondence/export/inputs --source-root evidence/s02-model-comparison/candidate-a-correspondence/export/source
```

Terminal exit: 0.

```text
(no output)
```

## Durable named mutation inputs

The nine semantic mutation controls were regenerated under
.superpowers/sdd/candidate-a-checker-takeover-stages/mutations.
Each exact test-name directory retains mutated-cases.json, the modified raw ITF,
and receipt.json with actual command, exit, stdout/stderr and hashes. No cleanup
removes these new artifacts. Generation refuses an existing directory to avoid
overwriting an earlier receipt. Mutated case/raw hashes agree with the earlier
temporary observations; these are controlled corruption experiments, not authentic
new Quint exports. Original workload and source pins remain unchanged. All child
commands explicitly use --allow-subset, and each fails the intended semantic
comparison while retaining valid selected-record provenance.

## Durable mutation generation

```sh
PYTHONDONTWRITEBYTECODE=1 S02_MUTATION_ARTIFACT_DIR=.superpowers/sdd/candidate-a-checker-takeover-stages/mutations .venv/bin/python -m pytest tests/test_s02_candidate_a_correspondence.py -q --tb=short -k 'test_absent_choice_is_not_zero or test_continuation_mapping_mutation_rejected or test_payment_order_mutation_rejected or test_asset_mapping_mutation_rejected or test_reduction_count_mutation_rejected or test_deposit_effect_omission_rejected or test_deposit_insertion_order_mutation_rejected or test_partial_warning_payload_mutation_rejected or test_deadline_priority_mutation_rejected'
```

Terminal exit: 0.

```text
.........                                                                [100%]
9 passed, 75 deselected in 0.66s
```

Durable artifact hashes:

```text
6f6787d862a0ea826423e1d541f84458d257388d66044c8a5077bf0db8884d34  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_partial_warning_payload_mutation_rejected/receipt.json
36fef803a6a7f5261a29fdee6d05f565ef1963c47fb050c5a0993ad5f1886a97  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_partial_warning_payload_mutation_rejected/diagnostic-corpus.itf.json
41b8d45348d08d85806b691f4fe02fcd74b3a301c610d7b3ca6c92ae4fb91901  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_partial_warning_payload_mutation_rejected/mutated-cases.json
c33fc7dd53f378ede58c41e61e5cb326424efae68fc8130be496fe227f23535e  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_reduction_count_mutation_rejected/receipt.json
cb21dfa9815c45642e6c68411c639c5e34ad20b7ebf844df315820f6fc37b297  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_reduction_count_mutation_rejected/diagnostic-corpus.itf.json
99b7ed28f15d0b054c18e0ce908531df4c10f1f829dc20f532c88b81161e7a49  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_reduction_count_mutation_rejected/mutated-cases.json
27a35fc0e305a5f89936ac3ef38234fdc99ecdd78559ebb33870c599e320b560  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deadline_priority_mutation_rejected/receipt.json
924fcd499371d681a42d38d649bc959a1b6c78b055f8ecf2f6052916f2671368  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deadline_priority_mutation_rejected/mutated-cases.json
bcd8b50a753e36b6037efd5f0868bc8ae1c94d0f9e1ab0e6b18633b45d57f864  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deadline_priority_mutation_rejected/installment-deadline-ten-cleanup.itf.json
8ce62eb179d55d1bd5320526ae25dc847505093144427996e59b57d49afab743  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deposit_effect_omission_rejected/receipt.json
7141ba52563a66bdddd37feff0bc49b1149048577fe1f2b3a6e6f4c4360cf966  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deposit_effect_omission_rejected/diagnostic-corpus.itf.json
fa5e5e369a2d9e2063bc35749ca3bbc2d9b5f6914b67dfd2c069cb63bd6e1224  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deposit_effect_omission_rejected/mutated-cases.json
0e4f4846d021307df98a7f12a6ee26cddf398d1f2e0c0c2b7fd5dedab561392a  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deposit_insertion_order_mutation_rejected/receipt.json
c646e2c35a98072ce3e52a32e95e27db128e369a4a18431c659576adaeffac8f  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deposit_insertion_order_mutation_rejected/diagnostic-corpus.itf.json
739d75362d3a6a63b705f01c0e68735a754657f54e2577f1639bef442afaf509  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deposit_insertion_order_mutation_rejected/mutated-cases.json
73c47a65f78f78a49e2c896041b1fd4229a3c21388b5221fa679495fb51f09c0  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_asset_mapping_mutation_rejected/receipt.json
9e543dd5f7f0785ee5ce28f4a2e2c05fe8461a8bb5e87c73d2a54b5182d99a3c  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_asset_mapping_mutation_rejected/mutated-cases.json
39b8fcb01b23c35fe806bf417b13bd5ad16f0d46f893999b61b91b70a21d9655  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_asset_mapping_mutation_rejected/swap-settlement.itf.json
b4ccad2ac2969c3daea9a8377502d348d730a002f1aed69506873935ce6185f7  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_continuation_mapping_mutation_rejected/receipt.json
c832028351123499b9a20cb1cabcf8c53e269f82b4f63a799ecb12d5487b9971  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_continuation_mapping_mutation_rejected/mutated-cases.json
0ff6ce72494b44f431eac41be35ccf67ec463b1afca7fd0e8fd5f98013db8ca2  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_continuation_mapping_mutation_rejected/installment-two-fills.itf.json
363423e2079edddd631e4a28ff8c8e61c9c0f67e3d227d29c57f9da4dbf0db8a  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_payment_order_mutation_rejected/receipt.json
dea587c432d167c72c1da1c33dc243432dbe7c58e5a324a1c15c02c74592be40  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_payment_order_mutation_rejected/mutated-cases.json
36600798bec8d65b09e00f077ff4d70a049ce65fbe10f328ba5bb4afb79c91b4  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_payment_order_mutation_rejected/swap-settlement.itf.json
2b6099b050965daab5654c746fc0a23fa822a66477f0a79990c2f4696ec3626c  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_absent_choice_is_not_zero/receipt.json
e9af90817665c8d9295bcd0f6226086ac80f7b6b256dff458179108b7859be21  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_absent_choice_is_not_zero/diagnostic-corpus.itf.json
aaff6c0b361d4a071fdf842b346a3fd8f04f884c838d754038cbc60b93a771ec  .superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_absent_choice_is_not_zero/mutated-cases.json
```

Exact durable CLI receipts:

```json
{
  "command": [
    "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
    "--allow-subset",
    "--cases",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_partial_warning_payload_mutation_rejected/mutated-cases.json",
    "--input-root",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_partial_warning_payload_mutation_rejected",
    "--source-root",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
  ],
  "exit_code": 1,
  "stdout": "cases[0].projection.result.warnings: differs from frozen core\n",
  "stderr": "",
  "mode": "explicit selected-record mutation control",
  "case_sha256": "41b8d45348d08d85806b691f4fe02fcd74b3a301c610d7b3ca6c92ae4fb91901",
  "raw_sha256": "36fef803a6a7f5261a29fdee6d05f565ef1963c47fb050c5a0993ad5f1886a97"
}
{
  "command": [
    "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
    "--allow-subset",
    "--cases",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_reduction_count_mutation_rejected/mutated-cases.json",
    "--input-root",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_reduction_count_mutation_rejected",
    "--source-root",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
  ],
  "exit_code": 1,
  "stdout": "cases[0].result.reductions: differs from frozen core\n",
  "stderr": "",
  "mode": "explicit selected-record mutation control",
  "case_sha256": "99b7ed28f15d0b054c18e0ce908531df4c10f1f829dc20f532c88b81161e7a49",
  "raw_sha256": "cb21dfa9815c45642e6c68411c639c5e34ad20b7ebf844df315820f6fc37b297"
}
{
  "command": [
    "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
    "--allow-subset",
    "--cases",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deadline_priority_mutation_rejected/mutated-cases.json",
    "--input-root",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deadline_priority_mutation_rejected",
    "--source-root",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
  ],
  "exit_code": 1,
  "stdout": "cases[0].result.accepted: differs from frozen core\n",
  "stderr": "",
  "mode": "explicit selected-record mutation control",
  "case_sha256": "924fcd499371d681a42d38d649bc959a1b6c78b055f8ecf2f6052916f2671368",
  "raw_sha256": "bcd8b50a753e36b6037efd5f0868bc8ae1c94d0f9e1ab0e6b18633b45d57f864"
}
{
  "command": [
    "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
    "--allow-subset",
    "--cases",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deposit_effect_omission_rejected/mutated-cases.json",
    "--input-root",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deposit_effect_omission_rejected",
    "--source-root",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
  ],
  "exit_code": 1,
  "stdout": "cases[0].effects: differs from independently derived ordered effects\n",
  "stderr": "",
  "mode": "explicit selected-record mutation control",
  "case_sha256": "fa5e5e369a2d9e2063bc35749ca3bbc2d9b5f6914b67dfd2c069cb63bd6e1224",
  "raw_sha256": "7141ba52563a66bdddd37feff0bc49b1149048577fe1f2b3a6e6f4c4360cf966"
}
{
  "command": [
    "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
    "--allow-subset",
    "--cases",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deposit_insertion_order_mutation_rejected/mutated-cases.json",
    "--input-root",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_deposit_insertion_order_mutation_rejected",
    "--source-root",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
  ],
  "exit_code": 1,
  "stdout": "cases[0].effects: differs from independently derived ordered effects\n",
  "stderr": "",
  "mode": "explicit selected-record mutation control",
  "case_sha256": "739d75362d3a6a63b705f01c0e68735a754657f54e2577f1639bef442afaf509",
  "raw_sha256": "c646e2c35a98072ce3e52a32e95e27db128e369a4a18431c659576adaeffac8f"
}
{
  "command": [
    "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
    "--allow-subset",
    "--cases",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_asset_mapping_mutation_rejected/mutated-cases.json",
    "--input-root",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_asset_mapping_mutation_rejected",
    "--source-root",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
  ],
  "exit_code": 1,
  "stdout": "cases[0].projection.payments[0]: payment asset differs from source account\n",
  "stderr": "",
  "mode": "explicit selected-record mutation control",
  "case_sha256": "9e543dd5f7f0785ee5ce28f4a2e2c05fe8461a8bb5e87c73d2a54b5182d99a3c",
  "raw_sha256": "39b8fcb01b23c35fe806bf417b13bd5ad16f0d46f893999b61b91b70a21d9655"
}
{
  "command": [
    "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
    "--allow-subset",
    "--cases",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_continuation_mapping_mutation_rejected/mutated-cases.json",
    "--input-root",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_continuation_mapping_mutation_rejected",
    "--source-root",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
  ],
  "exit_code": 1,
  "stdout": "cases[0].projection.result.state.continuation: differs from frozen core\ncases[0].projection.state.continuation.node: differs from exact evaluator node\n",
  "stderr": "",
  "mode": "explicit selected-record mutation control",
  "case_sha256": "c832028351123499b9a20cb1cabcf8c53e269f82b4f63a799ecb12d5487b9971",
  "raw_sha256": "0ff6ce72494b44f431eac41be35ccf67ec463b1afca7fd0e8fd5f98013db8ca2"
}
{
  "command": [
    "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
    "--allow-subset",
    "--cases",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_payment_order_mutation_rejected/mutated-cases.json",
    "--input-root",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_payment_order_mutation_rejected",
    "--source-root",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
  ],
  "exit_code": 1,
  "stdout": "cases[0].result.payments: differs from frozen core\n",
  "stderr": "",
  "mode": "explicit selected-record mutation control",
  "case_sha256": "dea587c432d167c72c1da1c33dc243432dbe7c58e5a324a1c15c02c74592be40",
  "raw_sha256": "36600798bec8d65b09e00f077ff4d70a049ce65fbe10f328ba5bb4afb79c91b4"
}
{
  "command": [
    "/home/charl/Moriarty/.worktrees/s01-audit-start/.venv/bin/python",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/check_s02_candidate_a_correspondence.py",
    "--allow-subset",
    "--cases",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_absent_choice_is_not_zero/mutated-cases.json",
    "--input-root",
    ".superpowers/sdd/candidate-a-checker-takeover-stages/mutations/test_absent_choice_is_not_zero",
    "--source-root",
    "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-model-comparison/candidate-a-correspondence/export/source"
  ],
  "exit_code": 1,
  "stdout": "cases[0].result.state: differs from frozen core\n",
  "stderr": "",
  "mode": "explicit selected-record mutation control",
  "case_sha256": "aaff6c0b361d4a071fdf842b346a3fd8f04f884c838d754038cbc60b93a771ec",
  "raw_sha256": "e9af90817665c8d9295bcd0f6226086ac80f7b6b256dff458179108b7859be21"
}
```

Final author disposition: 84 focused tests and complete 53-record CLI comparison
pass; all nine durable semantic mutation child commands fail as intended.
Source remains frozen at c6923d0e... / eb6bdf21... pending separate root review.
Authorship overlap, no independent non-author/Council claim, no execution
attestation, and the other limits above remain unchanged.
