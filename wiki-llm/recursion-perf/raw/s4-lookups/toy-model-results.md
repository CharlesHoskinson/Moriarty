# Local lookup algebra and row-budget model results
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/recursion-perf/clusters/s4-lookups/wave-1/model-results.json
> Collected: 2026-10-26
> Published: 2026-10-26

SOURCE — Output of `python3 clusters/s4-lookups/wave-1/check_models.py`; no prover timing measured.

```json
{
  "classification": "SOURCE: local toy algebra checks, not proving benchmarks",
  "permutation_cases": 567,
  "cases_with_compression_collisions": 162,
  "nonzero_denominator_product_checks": 11660,
  "exceptional_challenge_cases_skipped": 16123,
  "invalid_tuple_inputs_rejected": 3,
  "block_scan_checks_including_zero": 390,
  "rows_for_selected_bounds_four_lanes": {
    "15": {
      "56": 1,
      "64": 2,
      "128": 3,
      "255": 5
    },
    "16": {
      "56": 1,
      "64": 1,
      "128": 2,
      "255": 5
    },
    "17": {
      "56": 1,
      "64": 1,
      "128": 2,
      "255": 4
    },
    "18": {
      "56": 1,
      "64": 1,
      "128": 2,
      "255": 4
    },
    "19": {
      "56": 1,
      "64": 1,
      "128": 2,
      "255": 4
    }
  },
  "bounds_1_to_255_same_rows_as_cap19": {
    "15": 143,
    "16": 170,
    "17": 206,
    "18": 231,
    "19": 255
  },
  "limits": [
    "No cryptographic proof generation or BLS arithmetic tested",
    "No zero-denominator proof acceptance claimed",
    "Row DP counts decomposition rows only, not synthesis extents",
    "No CPU/GPU timing or ledger acceptance measured"
  ]
}
```
