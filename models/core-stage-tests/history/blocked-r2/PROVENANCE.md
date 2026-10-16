# Staged-test fixture provenance

This directory retains two distinct ownership records. The original expected
files, preimages and `expectation-freeze.json` are **implementation-author
expectations**, frozen before `stage_test.rs` was written. They were assembled
without evaluator/parser/compiler output, but their author also implemented the
consumer. Their historical format and issuer names do not establish independent
authorship. Those original bytes and their freeze are unchanged.

The new [independent-oracle-r2](independent-oracle-r2/INDEPENDENT-FREEZE.json)
directory preserves separately authored normal expectation data from
`/root/mori_test_stages_oracle`, GPT-6.1 Sol high. It contains the exact r2 freeze
and all 21 members bound by that freeze, including six complete report
candidates, four preimages, derivations and premise records. The freeze SHA-256
is `428d9eab4cf6bab885f9eb866d4a54ad14cf592ce8adf0b6818c64ab4e431918`.
Its [derivation](independent-oracle-r2/DERIVATION.md) records ownership,
construction, source-derived premises and limits. No captured file was edited,
and the included assembly script was not run during this integration.

Root subsequently obtained distinct source-only reviews
`mori-test-stages-source-opus-r1` and `mori-test-stages-source-muse-r2`, both
**APPROVE_SOURCE_ONLY**, and distinct full-data reviews
`mori-test-stages-independent-r2-opus-r1` and
`mori-test-stages-independent-r2-muse-r1`, both
**APPROVE_FROZEN_NORMAL_EXPECTATIONS_ONLY**. Each is limited to its reviewed
candidate and conditions. These receipts do not approve this changed integration
candidate, publication, execution, a build/test/comparison, comparison admission,
a runtime result, a byte-format result, native acceptance or any parent exit.
Final exact-source and publication reviews remain Root-owned and pending.

The copied r2 derivation/freeze still say full-freeze review is pending. Those
strings describe the original immutable capture, before the later data reviews
recorded above. They remain historical records; they are not rewritten to
retroactively claim approval. The projection premise remains a source-observed,
unadopted draft. The preserved r1 derivation/accounting likewise retain their
original partial-content and GAP-TRACE-1 labels.

Root's later `mori-test-stages-LITERAL-EXPECTATION-CORRESPONDENCE.json` records
literal-data checks of all eight inputs and four canonical preimages and exact
byte/JSON equality between all six separate r2 report candidates and the six
original author expected files. That is agreement between two sets of static
expectation data. No Moriarty product was executed and no actual program output
was compared. The manifest continues to name the original unchanged expected
files; the separate frozen copy supplies provenance, not a different runtime
input path or a generated expectation mechanism.

All product execution, public CLI/build/tests and actual-output comparisons
remain **UNRUN**. Comparison admission remains **NOT_GRANTED**, native authority
remains **0**, and this integration grants no execution authority. No CLI pass,
printed-byte-format acceptance, evaluator correctness, proof, financial/native
acceptance or U0–U7/M1–M17/MC/SP/G/partial-and-conditional exit follows.

## Independent content and disclosed representation premises

The independent author derived expression values, guard truth, balances,
liabilities, ordered financial effects, captures and work/accounting content
from the source/scenario and disclosed normative material. Twelve former r1
`Step.value` placeholders were filled in r2 using **P-TRACE-PROJ/draft** only
for attachment/presence: which already derived value attaches to `Step.value`
and whether an Await Step exists. This is source-derived convention conformance,
not independent verification of that convention or evaluator correctness.
The [replacement map](independent-oracle-r2/old-to-new-gap-map.json) preserves
each old pointer, gap, replacement value and applicable reviewed rule category.

Other representation premises remain separately disclosed: original parser
allocation appends each occurrence without interning, binary children precede
their parent, state initializers are direct Values; original IR/tag/Option
shapes determine serialization; canonical preimages use recursively sorted
keys, ordered arrays, string UInts and compact UTF-8 without a trailing newline;
SHA-256 binds those literal preimages. Original stage/plan/public rendering
source supplies capture order, ordinals, frames, labels, report envelopes,
semantics and committed-prefix representation. Static `cost.rs` schedules
supply the reported logical work representation. None is a parser/compiler,
hash implementation, boundary-normalizer or meter correctness proof.

Preimages and premise-hash inventories are review aids. `mori test` consumes
neither preimages nor the copied oracle's assembly script. A future complete
report predicate is JSON **value** equality under these premises; object member
order and whitespace/printed-byte equality are outside the data approval.
The existing source test's printed-byte assertion remains proposed and UNRUN.

## Retained C1–C6 conditions

These summarize the conditions preserved in the immutable r2 derivation; its
complete wording remains controlling.

- **C1:** Projection approval covers value attachment/Await presence only. The
  twelve replacements are `Step.value` fields; all other fields retain r1's
  separate content and representation premises.
- **C2:** Only Core/2 start/resume through `run_segment` is contemplated.
  Metered Core/1 and backend replay/`--trace` remain outside scope.
- **C3:** Draft refusal columns are unused and non-exhaustive; no expression
  failure is inferred away and no refusal/hostile outcome is accepted here.
- **C4:** Author self-assessment has zero review authority. Only separate
  non-author receipts supply their expressly limited judgments.
- **C5:** Numeric/Boolean/guard/effect/state/work content was independently
  derived, not copied from evaluator output or new evaluator-loop logic.
- **C6:** The disclosed traps remain limits: Liability/Discharge Step values are
  increments/decrements rather than closing/residual amounts; self/zero
  Transfers retain amount values; zero Liability retains `"0"` and a present-zero
  row; accepted-resume Await publishes Bool true with snapshot pre, not its event
  tuple; Await effects are empty and are not extended into Execution.effects.
  Liability/Discharge/self/zero cases are absent here and remain untested.

## Retained N1–N6 conditions

These summarize the later Opus full-data review; Muse incorporated both C and N
conditions. This record adds no security judgment.

- **N1:** Unsupplied source premises were checked only for consistency with
  normative text/types, not independently verified by that reviewer:
  `intent.rs` canonical encoding, `simulate.rs` envelopes, admission/runtime
  flow and the Await comment. In particular, start advanced=true, resumed
  frame_pre, the `/2` semantics string, waiting committed and Await presence
  retain those premises. Draft section4 was outside C1 approval.
- **N2:** Preimage/digest equality, 21-file binding and exactly-twelve value-only
  r1-to-r2 changes rest on Root's mechanical record, not reviewer recomputation.
- **N3:** Work uses source-derived static schedules, not an observed meter.
- **N4:** Future comparison means JSON value equality; byte order/format is
  outside scope and GAP-04 remains open.
- **N5:** C6 traps are not exercised by these cases and remain untested limits.
- **N6:** Author self-assessment has zero review weight; C1–C6 remain binding.

## All fourteen draft gaps remain open

1. GAP-01: line numbers.
2. GAP-02: stage report internals.
3. GAP-03: the `/2` semantics string.
4. GAP-04: key order and format.
5. GAP-05: parser ExprId allocation.
6. GAP-06: stage admission premises.
7. GAP-07: stage-specific error codes.
8. GAP-08: the `/2` trace bound.
9. GAP-09: no normative source for Step.value.
10. GAP-10: no normative source for Await Step presence.
11. GAP-11: no `/2` decode or replay.
12. GAP-12: MOR clause mapping.
13. GAP-13: boundary canonical-order provenance.
14. GAP-14: cap refusal during a false resume.

Complete candidate content closes none of these draft-wide obligations.

## Retained normal accounting expectations

The two-delivery case starts with alice20/bob0, transfers7, sets phase1,
observes an approval false without advancing, transfers3 after approval true,
and sets phase2/emits2 after receipt true. Observer11, treasury0 and duty5
survive every segment. Total balances31; gross debit10, fees0, bob net+10,
alice net-10, liability accrual0/discharge0/closing5, reserve dust0. Logical
costs5/5/6 commit16; the false observation attempts2 without committing, so
this listed path attempts18 overall.

The quoted-fee case captures input amount4 and let payable5 across one await.
It charges payer5 to treasury, sets total5 and emits5. Marker9, observer11 and
duty3 survive. Total balances31; gross debit5, fees5, treasury net+5, payer
net-5, liability accrual0/discharge0/closing3, reserve dust0. Logical costs5/8
commit13. The start stores an explicit null cap; the two-delivery start stores16.
Neither scalar total nor phase is a custody balance. Retained balances are not
claimed as rounding dust. These remain normal expected data, not observations
or a financial-validity/security assessment.
