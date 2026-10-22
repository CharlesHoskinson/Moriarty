# Collect the additional records

Use the unchanged canonical f11817c26392b2b0b87a89bad158fb5e61846af7 benchmark consumer. From this directory, with SUITE set to the installed models/sherlock/benchmarks directory:

```sh
python3 -B "$SUITE/metrics.py" collect --template templates/independent-parent-verify.json --bindings bindings/independent-parent-verify.json --base-dir . --output samples/independent-parent-verify.json
python3 -B "$SUITE/metrics.py" collect --template templates/keyed-build9.json --bindings bindings/keyed-build9.json --base-dir . --output samples/keyed-build9.json
python3 -B "$SUITE/metrics.py" report --samples samples/parent-proof.json samples/anchor-cost.json samples/independent-parent-verify.json samples/keyed-build9.json --output descriptive-report.json --markdown descriptive-report.md
```

README.md preserves the earlier two-record command example and its dated scope. These commands add the two later observations and render all four records. No native subject runs. Qualified repetitions remain0, and report acceptance remains not-assessed. Current separate-verifier review and Root-adoption status are explicit in MILESTONE-UPDATE.json; no missing adoption is inferred from native success.
