# How to compare complete observations

Use this guide when you already have a diagnostic artifact and need to check it against the independent normal expectations. Use an installed Python 3.10 or newer from a trusted checkout. These commands install nothing and compare supplied files only.

## Select the matching input shape

For existing literal program/prefix documents, supply both files and the matching family:

```bash
python3 -I -B models/sherlock/qa/check.py \
  --family normal \
  --observed-programs /absolute/normal-programs.json \
  --observed-prefixes /absolute/normal-prefixes.json
```

Use `--family extra-normal` for N07/N08. `normal` selects N01–N06; the checker still validates both frozen families and both committed Rust literal transcriptions before comparing your selected family.

For a complete host artifact, use:

```bash
python3 -I -B models/sherlock/qa/check.py \
  --host-observations /absolute/host-observations.json
```

Stream output uses the same `--host-observations` mode and declared producer as host output. See [run the stream realization](check-stream-realization.md) for actual producer execution and receipt checks.

For another realization's versioned envelope, choose exactly one applicable command:

```bash
python3 -I -B models/sherlock/qa/check.py --image-observations /absolute/image.json
python3 -I -B models/sherlock/qa/check.py --k-observations /absolute/k.json
python3 -I -B models/sherlock/qa/check.py --quint-observations /absolute/quint.json
python3 -I -B models/sherlock/qa/check.py --lean-observations /absolute/lean.json
```

The envelope modes compare all eight normal identities and 216 post-startup runs. Keep their `projection` and required raw diagnostics. A K envelope needs the declared raw runs, evidence and `comparison_ready` fields; having that flag does not authenticate execution. Consult [host and observation interfaces](../reference/host-machine.md) for the artifact boundaries.

For the two fixed Core-source specimens, use their separate checker:

```bash
python3 -I -B models/sherlock/qa/check_source_observations.py \
  --observations /absolute/source-observations.json
```

This compares both complete preparation reports and all 70 fuel runs/1,259 records. It expects S01/S02, not N01–N08. Their source profiles are `moriarty/1` and `moriarty-core/1`; their common observation profile is `sherlock-k-a/1`. Their opaque source names remain outside the K numeric-name correspondence claim.

## Retain the result

Replace the example paths with existing artifact paths. Save stdout, stderr and the terminal exit status alongside the original artifact, its SHA-256, source pins and producer receipt. Keep failures as well as passes.

| Exit | Action |
| --- | --- |
| 0 | Retain the matching comparison and its scope. |
| 1 | Inspect `first_difference` using [the diagnostic guide](find-first-difference.md). |
| 2 | Correct an input/path/format error or investigate frozen-source/transcription drift before comparing again. |

The checkers reject missing files, malformed or duplicate JSON members and nonfinite numbers. The normal checker rejects combined layer modes and combined literal/envelope modes. Do not strip fields, reorder arrays or normalize K text to obtain a match: the comparison retains exact text, complete states and ordered records.

A pass retains `acceptance: none`. Supplied producer and profile labels are declarations; `producer_authenticity: not-established` and `execution_receipt: not-supplied` remain appropriate for file comparison. Attach a separate actual producer receipt when reporting execution. See [profiles and results](../reference/profiles-results.md) and [DoctorWatson commands](../reference/doctorwatson.md).

For projection and clock differences, read [observation boundaries](../explanation/observation-boundaries.md).
