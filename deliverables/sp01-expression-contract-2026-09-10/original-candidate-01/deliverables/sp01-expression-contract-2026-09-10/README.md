# SP01.3 expression contract candidate

**Proposed complete expression layer, pending independent reviews.** The new
[semantic contract](../../experiments/moriarty-language/spec/successor/semantic-contract.md)
and [static judgments](../../experiments/moriarty-language/spec/successor/static-semantics.md)
cover all 40 expression constructors listed by the blocked archived candidate04.
They specify finite value domains, type indices/overloads, operand order, staged
writes, local work, rejection and the operation-descriptor boundary. The
[machine-readable table](../../experiments/moriarty-language/spec/successor/expression-signatures.json)
contains every constructor, compared with the original eight signature rows.
The proposed contract does not define the separate 38 financial operations.

Author: GPT-6 Astra, `/root/spec_audit_early`. Base commit:
`66b75d328c8ff0f4c101f3c89882ed5d8f73d581`; branch
`feat/sp01-expression-contract`. The [task plan](PLAN.md) records scope. Source
and protected-input digests are in `source-candidate-01.json`. Archived candidate04,
its blocked findings, funded-source runtime, syntax/formatter and K are unchanged.
No commit, publication, native/proof execution, wallet access or network call
was made by this task.

New choices EX-D1–EX-D8 remain proposals until fresh GPT-6 and Grok reviews agree.
They include strict Booleans, explicit numeric overloads, post-read representation,
local work charging, access aliases, descriptor-only Emit, finite limits, and
statement-only Unit/ordinary field classification. The existing pre/next/post,
residual-duty and no-host-acceptance requirements are kept separate from those
choices. Root source inspection prompted two corrections before freeze: an
explicit retained Price<AssetB,AssetA,4> example and exclusion of Unit from data
types. These comments are not independent approvals.

The [derivation cases](../../experiments/moriarty-language/spec/successor/expression-cases.json)
contain 80 constructor cases and 15 combined cases, including x10→11, false
ensures12, duplicate write, next/post misuse, mismatched asset, two runtime
failures, strict Boolean failure, work exhaustion, quantity rounding, blocked
financial writes and an exact aggregate-size rejection. They are author-derived
mathematical expectations, not results of a newly implemented evaluator or K.
Fresh reviewers must check their semantic validity; the structural checker does
not interpret the terms or verify their derivations.

Actual checks:

- `python3 experiments/moriarty-language/spec/successor/test-expression-contract.py`:
  10 tests pass. The retained initial RED has ten failures because the validator
  did not exist. Controls remove/duplicate constructors and corrupt type,
  operand, rule, source and case references. Logs: `structure-red.txt`,
  `structure-green-01.txt`, `structure-green-02.txt`.
- `python3 experiments/moriarty-language/spec/successor/check-expression-contract.py`:
  40 records/40 case pairs, no structural errors; `structure-checks.json`.
- `python3 deliverables/sp01-expression-contract-2026-09-10/check-arithmetic.py`:
  10 selected arithmetic/count checks; `arithmetic-checks.json`. This is neither
  an expression interpreter nor a proof of typing or financial conservation.

Full RP01 remains open. Required financial equations (requests, refunds, rewards,
event payouts), genesis funding, signing/display identity, contextual histories,
cancellation backing and the full challenge map are not repaired here. The
combined order of financial descriptor execution, ordinary writes and financial
postconditions needs an exact relation before full Core implementation. Privacy,
authenticated observations and lifecycle authority are also unresolved interfaces.
`ExpressionPrepared` deliberately grants no financial or ledger acceptance.

This candidate addresses the expression-signature part of C04-F1 only. It does
not close SP01.3, admit SP02/SP03, freeze a base profile or establish any of the
six required metatheorems. The actual compiled-language-to-Midnight Preview and
mandatory-proof gates remain required across the roadmap.
