# Task 5 report: fail-closed S01 evidence validator

## Scope and result

Implemented the S01 validator, focused mutation suite, validation report, and
self-hashed evidence manifest. The implementation changes no Core, swap,
semantic-scope, schema, or previously reviewed normative artifact bytes.

The published result is limited to an S01 specification freeze and a local
exact-transfer experiment at S3. It grants no signing authority and establishes
no mechanized theorem, proof-system result, backend correspondence, ledger
correspondence, wallet correspondence, or ACTUS compatibility.

## RED evidence

Command:

```text
/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s01_intent_evidence.py -q
```

Raw relevant result before the validator existed:

```text
76 failed in 0.73s
FileNotFoundError: scripts/validate_s01_intent_evidence.py
```

The two final trust-boundary tests also completed explicit RED cycles:

```text
2 failed, 75 deselected in 0.19s
```

Those failures showed that an extra scope-index field was ignored and that a
hostile earlier `PYTHONPATH` entry could win when the addressed root was already
present later in `sys.path`.

The pre-import source test completed a separate explicit RED cycle:

```text
1 failed, 77 deselected in 0.16s
AssertionError: assert not sentinel.exists()
```

This demonstrated that a changed local Core source could execute before its
digest was rejected. The implementation now hashes the package import boundary,
Core, swap, and local checker before importing any of them.

The five package-path read-boundary tests completed another RED cycle:

```text
5 failed, 78 deselected in 0.57s
```

Each substituted artifact, schema, scope, report, or manifest path was opened
before path closure. Package entry points now close these paths before parsing
their JSON.

The independent review's five additional semantic mutations initially failed:
term and actor whitespace reached the generic override check, weakened complete
projection language reached the generic override check, and missing vector
`resolution` or `scope` leaked `KeyError`. The corrected targeted run was:

```text
22 passed, 66 deselected in 1.07s
```

## GREEN evidence

Focused command:

```text
/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s01_intent_evidence.py -q
```

Raw result:

```text
........................................................................ [ 81%]
................                                                         [100%]
88 passed in 6.89s
```

Read-only package validation command:

```text
/home/charl/Moriarty/.venv/bin/python scripts/validate_s01_intent_evidence.py
```

Raw relevant result:

```json
{
  "gate_results": {
    "S01-01": true,
    "S01-02": true,
    "S01-03": true,
    "S01-04": true,
    "S01-05": true,
    "S01-06": true,
    "S01-07": true,
    "S01-08": true,
    "S01-09": true,
    "S01-10": true
  },
  "package": "S01",
  "schema_version": 1,
  "status": "recomputed-package-gate-passed"
}
```

Full-suite command:

```text
/home/charl/Moriarty/.venv/bin/python -m pytest -q
```

Raw result:

```text
........................................................................ [ 26%]
........................................................................ [ 53%]
........................................................................ [ 80%]
.....................................................                    [100%]
269 passed in 8.22s
```

Syntax and diff checks also returned exit status zero:

```text
/home/charl/Moriarty/.venv/bin/python -m py_compile scripts/validate_s01_intent_evidence.py tests/test_s01_intent_evidence.py
git diff --check
```

## Public API decisions

- `ValidationError` is the stable programmatic failure type for supplied data,
  schema, semantic, pin, and receipt failures.
- `load_artifacts()` loads only the nine normative S01 JSON artifacts. Generated
  receipts do not participate in semantic gate computation.
- `_recompute_semantic_gate()` is the private bootstrap helper. It recomputes
  the ten semantic gates, verifies addressed source imports, and checks immutable
  and reviewed-byte pins without reading generated receipts.
- `recompute_gate()` is the planned closed public API. After private semantic
  recomputation, it independently validates report identity, manifest identity,
  canonical self-hash, exact path closure, roles, and file digests.
- `validate_published_evidence()` delegates to the closed `recompute_gate()`.
  Default CLI execution uses this public closed path. Neither public API can
  report package success when a receipt or validator digest is missing or stale.
- `--write-evidence` is the only publishing mode. It runs every semantic gate,
  constructs both receipts in memory, schema-validates them, checks hashes and
  closure, and only then replaces the two bounded output paths.
- CLI failures are one JSON object on stdout with nonzero status and no Python
  traceback. Unknown arguments fail the same way.

This separation satisfies both governing requirements: the planned public
`recompute_gate()` API remains receipt-closed, while the audited correction's
separate semantic computation is private and is used only to bootstrap explicit
publication.

## Validation coverage

The focused suite mutates all ten gates. It also covers exact premise
expressions, hard-predicate authorization flags, missing and duplicate
preference identities, canonical Core settlement recomputation, coordinated
baseline/policy/expected edits, wrong mutant and restoration, complete
projection and both signing profiles, empty overrides, unknown fields,
non-finite JSON, malformed JSON, duplicate JSON keys, and unavailable effect
evidence.

Receipt cases cover missing, extra, duplicate, escaping, symlink-substituted,
and stale paths; stale report and self-hash; coordinated immutable-source and
manifest edits; coordinated prose-definition and manifest edits; coordinated
weakened-schema and manifest edits; stale validator/checker digests; addressed
worktree import identity; rejection of changed local source before import; and
path closure before reading package artifact, schema, scope, report, or manifest
JSON. All filesystem mutations occur in temporary package copies.

## Trust limitations

The validator and its embedded reviewed digest table are the trust root. The
manifest self-hash detects accidental or uncoordinated replacement; it is not a
signature or proof. Coordinated replacement of the validator and its trusted
constants is outside this mechanism. The Python interpreter, standard library,
JSON Schema implementation, filesystem, and operating system remain trusted.
Byte preservation proves identity with the reviewed boundary, not semantic
correctness by itself.

The validator executes the pinned Python Core, swap, and local checker only
after their source digests and the package import-boundary digest match. It also
checks the loaded module source paths. This is local source-identity evidence,
not a claim that an adversarial interpreter or operating system is safe.

## Changed paths

- `scripts/validate_s01_intent_evidence.py`
- `tests/test_s01_intent_evidence.py`
- `evidence/s01-intent-theorem-freeze/validation-report.json`
- `evidence/s01-intent-theorem-freeze/evidence-manifest.json`
- `.superpowers/sdd/task-5-report.md`

No constants-only helper was needed. No additional source-file separation is
required for this task.

## Review disposition and concerns

Independent review identified the pre-import source-execution issue and three
additional Important findings: missing optional vector identity fields leaked
`KeyError`, projection checks did not enforce all supplement bindings, and term
or actor definitions could contain only whitespace. Each issue was reproduced
with a failing test and fixed before final verification. The final mutation
suite and full repository suite found no unresolved functional concern. The
reviewer rechecked the stable, resealed files and approved them with no remaining
Critical, Important, or Minor finding.

## Post-review API and schema-reference correction

The controller's final spec-and-quality review found that the public
`recompute_gate()` returned semantic success without validating generated
receipts, and that JSON Schema reference-resolution failures escaped the stable
`ValidationError` contract.

The initial focused reproduction was:

```text
7 failed, 88 deselected in 0.90s
```

Five failures showed `recompute_gate()` returning success with a missing or
stale report, missing or stale manifest, or stale validator digest. Two failures
showed `_WrappedReferencingError` instead of `ValidationError` for a missing
definition or unresolved local reference.

Two self-containment mutations then completed a separate RED cycle:

```text
2 failed, 95 deselected in 0.49s
```

External `$ref` and `$dynamicRef` values reached the resolver instead of being
rejected before resolution.

The corrected targeted result was:

```text
9 passed, 88 deselected in 1.26s
```

After the final source changes, the actual publication command was run once:

```text
/home/charl/Moriarty/.venv/bin/python scripts/validate_s01_intent_evidence.py --write-evidence
```

The resealed default read-only validator returned all ten true gates and status
`recomputed-package-gate-passed`. Final test results were:

```text
97 passed in 8.24s
278 passed in 9.61s
```

The first result is the focused Task 5 suite. The second is the full repository
suite. Syntax compilation and `git diff --check` also returned exit status zero.
