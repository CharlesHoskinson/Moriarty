# Make your first local Sherlock observations

We will run DoctorWatson's local checks, inspect their report, and exercise the fixed-source observation checker. Finish with a complete literal comparison that you can repeat.

Use a trusted checkout with an already installed Python 3.10 or newer, Bash, and standard shell tools. This lesson installs nothing. Start in the repository root, where `scripts/doctorwatson.sh` exists.

## Run the local check

```bash
python3 --version
./scripts/doctorwatson.sh
```

The first line should identify Python 3.10 or newer. DoctorWatson prints a JSON report, followed by verbose Python test results. On the documented October 17 integrated baseline, the ordinary run finished with 241 tests and `OK`; later source changes can change that count.

Find these fields in the JSON:

```json
{
  "status": "pass",
  "scope": "frozen-normal-corpus-and-source-literal-QA",
  "acceptance": "none",
  "profile": "sherlock-k-a/1"
}
```

Notice the counts: eight programs, 208 state rows, 200 charged edges, eight publications and 216 fuel-prefix rows. The two Rust literal transcriptions should match. `source_reference_facts` inventories explicit input byte reads; it establishes neither the complete dependency closure nor reviewer coverage. `execution_layers` remains `not-run`: this command reads the corpus and committed source literals.

If the shell cannot execute the script, check that you are in the checkout root. If Python reports a frozen-source, oracle or serializer digest error, keep that output and use [find the first difference](../how-to/find-first-difference.md). Do not change a digest to make this exercise pass.

## Inspect the two source specimens

Read the two committed files:

- `crates/mori-core/src/sherlock/source_adapter_fixtures/weighted-total.mori`
- `crates/mori-core/src/sherlock/source_adapter_fixtures/threshold-check.mori`

Now run only the existing source-observation Python regressions:

```bash
PYTHONDONTWRITEBYTECODE=1 python3 -B -m unittest discover \
  -s models/sherlock/qa -p 'test_source_observations.py' -v
```

Expect ten tests and `OK` on this baseline. They exercise both source identities, complete preparation reports, 70 fuel runs and 1,259 records. The matching inputs here are copied independent literals used as test fixtures. The tests also check that changed source mappings, frames, terminal outputs and missing publications are detected. This command does not parse or execute the Moriarty specimens.

Notice the test named `test_matching_copied_literals_remain_supplied_artifact_comparison_only`. A matching file and an execution receipt are separate results. [Compare an actual supplied source artifact](../how-to/compare-complete-observations.md) when one is available.

## Complete a repeatable comparison

Create exercise copies outside the checkout, then compare them:

```bash
work=$(mktemp -d)
cp models/sherlock/oracles/normal-programs.json "$work/normal-programs.json"
cp models/sherlock/oracles/normal-prefixes.json "$work/normal-prefixes.json"
python3 -I -B models/sherlock/qa/check.py \
  --family normal \
  --observed-programs "$work/normal-programs.json" \
  --observed-prefixes "$work/normal-prefixes.json"
```

Expect `status: pass`, `supplied_observation_status: match` and `first_difference: null`. These copies demonstrate the comparison path; they are not interpreter output. Your terminal now contains a local QA result and a complete supplied-document comparison.

Continue with [compare complete observations](../how-to/compare-complete-observations.md) for your own captured artifact. See [DoctorWatson reference](../reference/doctorwatson.md) for command options and [profiles and results](../reference/profiles-results.md) for the report vocabulary.

For the evidence distinction behind these results, read [evidence and acceptance](../explanation/evidence-and-acceptance.md).


## Supported-profile documentation update: no exercise change

The [draft supported-profile report](../reference/draft-source-compiler.md#supported-profile-report)
adds returned compiler metadata; it changes no transition or circuit semantics
used in this exercise. The tutorial commands and their historical observations
are unchanged. R256's two new report tests passed; the
[separate file-publication example](../how-to/compile-draft-program.md#inspect-the-supported-profile-field)
remains unrun. The separate [public runtime result](../../results/sherlock-public-runtime-20261018/README.md)
now records two passing host tests whose inspected assertions check UInt 7/8 and
call/counter relations. Those are separate observations, not output from this
unchanged Python exercise. Raw stdout contains test status, not a complete trace
or exact B clock. No transition/circuit semantics or circuit/website iteration
changed in this tutorial; no command here was rerun for this documentation update.


The [unrun simulate command](../how-to/compile-draft-program.md#pending-command-shape-unrun)
adds no observed exercise step here. R299's one-test/24-row argument check and
R301's normal Core compile have separate scoped observations. R310 also passed
one two-program real-run/report/serialization test; that is not enclosing CLI,
file I/O or a command printout. Those command/file observations remain UNRUN.
Existing tutorial commands/results stay unchanged; there is no transition/
circuit semantic change or circuit/website iteration.
