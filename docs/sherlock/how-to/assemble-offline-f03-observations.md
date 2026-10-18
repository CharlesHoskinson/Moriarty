# Assemble observations from an existing F03 capture set

Use this guide when the fixed F03 fuel93/stack64 run has already been captured and you need its observation envelope. This is an offline API example, not a producer command or a recorded new execution. The assembler is a callable Python module; it has no dedicated command-line interface.

## Select the retained inputs

Use an existing `actual/k` directory and its original `receipt.json`, the matching `F03-fuel093-stack064.ui`, `COMPILED-MAIN-RULE-SPANS-01.json`, captured `main.k` and `K-COMMANDS.json`. The source accepts the pinned F03 identities, not a replacement program with similar counts. Keep the capture directory unchanged while it is read. Do not create receipts from current file hashes or substitute expected outputs for missing artifacts.

The [actual F03 archive](../../results/sherlock-k-quint-f03-actual-20261018/README.md) identifies the historical captures and comparison. Startup `exec-001` and terminal `exec-096` are separate probes; `exec-000` belongs to pre-startup normalization. The 95 post-startup states yield 94 edges, including the free publication edge.

## Call the assembler

Use these calls from an application whose Python import path contains the trusted checkout. Replace the example paths with retained files. The example reads supplied files and returns dictionaries; it has not been executed as a new documentation procedure.

```python
import json
from pathlib import Path
from models.sherlock.qa import assemble_k_f03_observations as f03

actual_k = Path("/absolute/retained/actual/k")
receipt = json.loads((actual_k / "receipt.json").read_text(encoding="utf-8"))
child_receipts = {
    row["stage"]: row["child_receipt"]
    for row in receipt["stages"]
    if row["stage"] != "initializer"
}
stages = f03.load_stage_artifacts_from_actual_dir(
    actual_k, receipts_by_stage_id=child_receipts,
)
result = f03.decode_f03_observations(
    stages,
    run_input_bytes=Path("/absolute/retained/F03-fuel093-stack064.ui").read_bytes(),
    rule_span_inventory_bytes=Path(
        "/absolute/retained/COMPILED-MAIN-RULE-SPANS-01.json"
    ).read_bytes(),
    source_k_bytes=Path("/absolute/retained/main.k").read_bytes(),
    k_commands_bytes=Path("/absolute/retained/K-COMMANDS.json").read_bytes(),
    strict_stage_validation=True,
)
```

Keep the complete `result`, its source/input pins and original receipts. Inspect `result["decoded_run"]["gaps"]` and both `result["decoded_run"]["comparison_ready"]` and `result["bundle"]["comparison_ready"]`. A false flag is a diagnostic result, not a matching observation envelope. A `ValueError` identifies a refused input; keep the error and fix the input/source mismatch rather than deleting fields or changing limits to force a pass.

`result["bundle"]` is the K envelope. `result["assembled_provenance"]` records source identities and limits. If your application persists the bundle as JSON, retain the complete other two result fields alongside it; this guide does not prescribe or perform file writes.

## Compare supplied observation files

Once your application has retained K and Quint envelopes from the same declared F03 slice, the existing file-only comparator accepts:

```bash
python3 -I -B models/sherlock/qa/check.py \
  --repo /path/to/trusted/checkout \
  --paired-k-observations /absolute/k-observations.json \
  --paired-quint-observations /absolute/quint-observations.json \
  --selected-case F03 --selected-initial-fuel 93
```

Run from the trusted checkout and replace the paths with existing artifacts. This is the documented comparator interface, not a replay of the accepted producer runs or a claim that this new path example has passed. Follow [complete observation comparison](compare-complete-observations.md) to retain stdout, stderr, terminal status and exact scope. A matching supplied-file comparison still does not authenticate either producer.

## Read the observed checks

The [eleven complementary guards](../../results/sherlock-f03-general-adapter-guards-20261018/actual/eleven-01/RESULT.json) passed before canonical installation. The [four installed-decoder checks](../../results/sherlock-f03-general-adapter-guards-20261018/actual/installed-four-01/RESULT.json) then passed: source identity, embedded N01 comparison, unit-tail retention and hostile unit-tail refusals. Both are offline Python observations, with no K, Quint, SDK, Cargo or native child run. Do not repeat the earlier seven helper guards, three passive comparisons or model runs merely to read these receipts.

Use the [reference](../reference/offline-k-capture-adapters.md) for the default 64-capture limit, explicit 95-capture F03 selection and unchanged artifact bounds. Internal consistency does not establish execution authenticity, general K-format coverage, refinement, finance/wait/B, proof or native/target acceptance.
