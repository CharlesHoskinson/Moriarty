# Interface contracts (implementer-authored, schema only)

A contract pins the observable interface of an entrypoint BEFORE the implementer posts STATUS and
before the evaluator authors a holdout against it. It carries no case content. The implementer
writes `qa/holdout/contracts/<ID>.json`; `scripts/doctorwatson contract-check <ID>` must exit 0
before a STATUS or CONTRACT CHANGE post. Any later change to the entrypoint output means editing
the contract file in the same commit, so the evaluator sees it in git (see scripts/evaluator-watch).

```json
{
  "format": "doctorwatson-contract/1",
  "id": "U013",
  "entrypoint": {"argv": ["target/debug/mori", "simulate", "src.mor", "--chunk", "1"], "cwd": "."},
  "expect": {
    "exit": 0,
    "json": true,
    "max_stdout_bytes": 1048576,
    "timeout_s": 600,
    "required_paths": ["technical_chunks.cumulative", "chunks[].charges.events"],
    "forbidden_paths": ["chunks[].accounting"]
  }
}
```

Paths are dotted; `name[]` fans out over a list and every element must carry the rest of the
path (an empty list fails). Exit codes: 0 all expectations hold, 1 a check failed, 2 bad or missing
contract. Pick sizes with the real output cap in mind: the U013 chunk report broke the 1 MiB
stdout cap only after the first contract post, which is the failure this check prevents.
