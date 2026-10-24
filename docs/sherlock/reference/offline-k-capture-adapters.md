# Offline K capture adapters

These Python APIs read supplied K standard logs, KORE states, `kprint` configurations and receipts. The F03 assembler builds the fixed fuel93/stack64 capture sequence; the decoder also supports the bounded N01 capture layout. They do not launch K or another producer. Use the [how-to](../how-to/assemble-offline-f03-observations.md) to assemble an existing F03 capture set.

## Interfaces

The [assembler](../../../models/sherlock/qa/assemble_k_f03_observations.py) exposes:

```text
load_stage_artifacts_from_actual_dir(
    actual_k_dir, *, receipts_by_stage_id,
) -> dict

assemble_f03_capture_payload(
    stage_artifacts, *, run_input_bytes, rule_span_inventory_bytes,
    source_path=F03_COMPILED_MAIN_K_PATH,
    require_pinned_inventory_sha256=True,
    k_commands_bytes=None, strict_stage_validation=False,
) -> dict

decode_f03_observations(
    stage_artifacts, *, run_input_bytes, rule_span_inventory_bytes,
    source_k_bytes, source_path=F03_COMPILED_MAIN_K_PATH,
    require_pinned_inventory_sha256=True,
    k_commands_bytes=None, strict_stage_validation=False,
) -> dict
```

The directory loader pairs existing files with the supplied 195 child receipts. It requires `kast`, `exec-000` through `exec-096`, `print-000` through `print-096`, and a separate `initializer` directory: 488 artifact files in total. It does not synthesize receipts from file hashes. Its path checks reject symlinks, check containment and bound file size before reading; these checks remain subject to concurrent filesystem changes.

The assembler requires the pinned F03 input, compiled-source path and span inventory. It returns 95 post-startup `captures`, separate `startup_probe` and `terminal_probe`, a `pre_startup` record, retained input and span metadata. `decode_f03_observations` additionally checks the supplied source bytes and returns three keys:

| Key | Contents |
| --- | --- |
| `bundle` | The versioned observation envelope for comparison. |
| `decoded_run` | Snapshots, ordered records, raw evidence, chain links, gaps and `comparison_ready`. |
| `assembled_provenance` | Source identities, selected limits, strictness, pre-startup retention and trust qualifications. |

The [capture decoder](../../../models/sherlock/qa/k_capture_observations.py) exposes:

```text
decode_capture(
    case_id, entry_k, captures, *, source_path, rule_spans=None,
    terminal_probe=None, startup_probe=None, run_input=None,
    max_captures=MAX_CAPTURES,
)
```

`captures[0]` supplies the initial post-startup configuration. Later captures supply the next configuration, incoming log, state KORE and engine/print receipts. Startup and terminal replay probes are separate. Missing evidence or inconsistent receipts can produce gaps and `comparison_ready=False`; malformed structural inputs can raise `ValueError`.

## Bounds and validation

| Input | Bound |
| --- | --- |
| Capture count | Default 64; explicit F03 ceiling 95. `max_captures` must be an integer from 1 through 95. |
| Configuration | 8,192 bytes. |
| State KORE | 1 MiB per artifact. |
| Log | 4 MiB and 65,536 lines per artifact. |
| Nesting / parsed term | 32 levels / 4,096 bytes. |
| Receipt argv | 64 arguments, each at most 4,096 characters. |
| Decoder span inventory | At most 4,096 rows; the pinned F03 inventory has exactly 113. |
| F03 `kast` stdout | 65,536 bytes, one admitted ground UIRun KORE term. |
| F03 inventory / command JSON | 1 MiB each. |

The F03 wrapper selects 95 explicitly; it does not raise the decoder's default. F03 includes 94 edges: 93 charged steps and the free publication edge, with startup and terminal probes outside that edge sequence.

`strict_stage_validation=True` raises `ValueError` for selected receipt and artifact inconsistencies during assembly. The default `False` preserves selected post-startup inconsistencies for the decoder's canonical gaps and non-ready result. It does not disable path, input, inventory, pre-startup or structural checks. Passing `k_commands_bytes` checks the pinned command document; omitting it retains admitted command-shape checks and records that the command document was not pinned.

## Evidence and limits of the result

The [eleven guard methods](../../results/sherlock-f03-general-adapter-guards-20261018/actual/eleven-01/RESULT.json) passed once. They cover the six assembler categories, multiline single-KItem handling and end-to-end capture-chain tampering. The [four installed-decoder regressions](../../results/sherlock-f03-general-adapter-guards-20261018/actual/installed-four-01/RESULT.json) subsequently passed on the installed bytes with all five input pins unchanged. Earlier seven helper guards and three passive operations remain separate observations and were not replayed. See the [Root adoption](../../results/sherlock-f03-general-adapter-guards-20261018/ROOT-GENERAL-ADAPTER-ADOPTION.json) and [complete archive](../../results/sherlock-f03-general-adapter-guards-20261018/README.md).

`comparison_ready=True` checks internal consistency of supplied bytes, hashes, spans and paths. Receipts remain declarations; this flag does not authenticate execution. Source hashes describe files on disk, not an attestation of loaded code. Oracle exclusions are specific to the named formats, basenames and complete artifact hashes. They do not prove arbitrary data independent.

The new eleven-method test is archival because it names retained local R fixtures; it is not installed into portable QA discovery. The bounded utility adoption adds no general K-format support, full corpus agreement, refinement theorem, finance/wait/B semantics, native/circuit/proof result or target settlement. Historical original021 acceptance and the current 15 original closures retain their scope; all 25 refinements remain open.
