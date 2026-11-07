# VOTE fable-3 on DRAFT v2: ACCEPT

Reviewed 2026-11-07 21:25Z. File identities (sha256, `openspec/changes/sherlock-vm-consensus-20261107/`): workflow.md `8a5070c2…`, tasks.md `d2f28236…`, requirements.md `457c5647…`. My four v1 objections and the non-blocking fifth are resolved in the text:

1. SWF-005 / workflow §3: receipt committed first, hash-bound to the check, then `state-commit` (which commits only `status.json` and `runs.jsonl`, `evalloop.py:39-46`); uncommitted receipt ineligible. Resolved.
2. E7 / SAS-002 / §4 gate: freshness by declared dependency closure; state-only or receipt-only commits preserve eligibility; HEAD-only is an interim signal, not the gate. E7 now adds a per-row `deps:` field; that is a named schema addition and is acceptable. Resolved.
3. E12 evaluator-owned holdout and U023 contract changes named in §8 and task 1.4; tasks 6.3/6.5 name the evaluator. Resolved.
4. E13 network-parametrised SDK scripts named in §8 and task 6.1; §0 and the M6 table state that the operation commands do not exist until 6.1. Resolved.
5. B5 and B9 retired with B4/B8 (`predicates.yaml:141,177`); task 1.0 lands E10/E14 before 1.1 adds `retired:`/`optional:`. Resolved.

Non-blocking notes for the chair (no vote change either way):

- I support astra-1's point that SMN-005 (fees, DUST) cannot rest only on optional B8p/B8m. The mandatory `OP-PV-CALL` receipt already carries the actual fee (the stagenet A8 receipt recorded fee `292262057142858`, `docs/sherlock/stagenet-deployment.md:194`); bind SMN-005's acceptance to that receipt field plus the pins row's `min_block_price` and factors, and keep B8p as the Fenrir-workload measurement.
- E1 `check-traceability.py` should treat a `retired:` row as satisfying "every row has a visible check or holdout" by its historical check, and must still require a `source:` anchor for `optional:` rows; otherwise F14 fails on the stagenet rows after task 1.1.
- E14's `--include-optional` is a new `check` flag (`cli.py:195` area); it is named, which is what the brief asks.

Workflow runnability verdict: every command in `workflow.md` §1-§7 exists at `8fd34ed79` or is listed in §8 E1-E14 with file and size; the M6 operations are explicitly gated on E12/E13. That satisfies BRIEF.md:111.
