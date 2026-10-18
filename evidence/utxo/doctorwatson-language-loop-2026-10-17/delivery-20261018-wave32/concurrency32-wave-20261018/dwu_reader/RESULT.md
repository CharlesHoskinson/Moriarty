# Original requirement source locations

Prepared against the explicitly authorized UTXO source
`2a9ae207a2b4c573a508d74b338c72bc5fac5e1f`. The compiler-wave base
`64c6dedff1d99b33c23832330d7911a5fad9ed59` does not contain this reader.
The original and candidate complete reader bodies, the new regression, exact
Git blob/SHA-256 pins and the patch are retained in this directory.

The public consumer is `scripts/doctorwatson-utxo.sh --requirement EUC-001`
through `check.py`'s plain-text selector output. Its former `Source:` value was
`openspec/changes/eutxo-compiler-correspondence/specs/eutxo-compiler/spec.md#EUC-001 Explicit factory and version routing`.
This combines a repository-relative path with a stable heading identity rather
than an openable file location. The pinned original requirement is at line 3.
When invoked from another directory, the relative path also resolves against
the wrong working directory.

`reader-source-location.patch` replaces that display with the resolved selected
repository's absolute source path and the positive stored `startLine`, when
available. It prints the original heading separately. It changes no JSON
fields, clauses, scenarios, evidence classification, status or gate logic.
DWU-01's original-source preservation and DWU-06's inspectable consumer remain
the reporting obligations; this patch establishes no product acceptance.

The focused reporting regression starts the public CLI in another directory,
with spaces in both the selected repository and original source path. It checks
that the displayed path opens the exact original requirement line and retains
the complete clause, both scenarios, unknown applicability, missing observation
and open gate. Before the display edit it failed once with the former pseudo
location (exit 1). The candidate passed the one new test (exit 0); the final
confined run took 0.042 seconds. `git apply --check` against the retained pinned
source copy passed (exit 0). Existing passing suites were not rerun.

Root can inspect/apply the patch against the pinned UTXO source, then run only:

```bash
python3 -B models/utxo/qa/test_requirement_source_locations.py -v
```

For the packet without integration:

```bash
TMPDIR="$PWD/scratch" python3 -B candidate/models/utxo/qa/test_requirement_source_locations.py -v
```

The patch has not been applied to a shared checkout. No compiler, SDK, model,
proof, key, wallet, network or target operation ran. Initial temporary fixtures
used `/tmp` and were cleaned; the final run confines them to packet `scratch/`.
I previously authored the reader: this is author corroboration of an ordinary
display defect, not a fresh independent oracle, provider audit or security
judgment. Applicable independent review remains pending.
