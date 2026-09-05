# Design: S02 bounded architecture comparison contract

## Boundary

The package defines what future S02 execution must demonstrate. It does not
perform the experiment. Bounded-depth model checking, sampled simulation,
mechanized proof, and production correspondence are distinct evidence classes;
none may be reported as another.

The four representations are A, agreement Core with an intent envelope; B,
intent Core with agreement libraries; C, two calculi with a refinement bridge;
and D, a Compact library with a local verifier. They share the canonical swap
and two-installment workloads and the `SignAfterResolve` and `SignBeforeResolve`
profiles. Four labels over one interpreter are not four representations.

## Dependencies

Execution depends on completed S01, the reviewed S02 design, and the closed
requirements registry. The prompt, frozen S01 outputs, Core, swap, acquired
receipts, and semantic scope remain unchanged.

The common observation and authorization supplement is also a pinned design
dependency: `docs/superpowers/specs/2026-09-04-moriarty-s02-observation-authorization-design.md`
at SHA-256
`1adc668e970d35a492e272f4f22ac468dccfd690b2c2e1a0ce3ac62030d3ae25`.
It fixes the shared transfer, ledger, observation, authority, pre-sign, and
recovery interfaces used by the bounded experiment. It does not select an
architecture or establish a proof.

## Immutable inputs

The future validator must compare the following files to reviewed pins before it
accepts evidence:

- `deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml`
  at SHA-256
  `86b80dd1cbd14d1e5759988be9f619355495fc10c4e2fb6b6d9162367670ddcd`;
- `docs/superpowers/specs/2026-09-04-moriarty-s02-model-comparison-design.md`
  at SHA-256
  `75e242cabdde75fc77b6512bf8949715ad8f289311f32663d04da3c600856ab9`;
- the completed S01 manifest
  `evidence/s01-intent-theorem-freeze/evidence-manifest.json` at SHA-256
  `b1a5ac79236fd4fc95a689e3e78e32ad0dfab9a7db3c84a54919e63749adf8ef`
  and `evidence/s01-intent-theorem-freeze/intent-safety-judgment.json` at
  SHA-256
  `c4ea5bfe4b87b0ace36a73d1903fa9abeddeb09297a4e490b16bd01ff921e0db`,
  with proof status `candidate-unmechanized`;
- semantic scope `0.0.0-e00.2` at SHA-256
  `9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`;
- frozen `moriarty/core.py` at SHA-256
  `564a926779beb54bd16ff58481f5c950bae6fb50fcd5b0d7c9eb56427f83273b`
  and `moriarty/swap.py` at SHA-256
  `82e9e1da76af65706171049f0238953aca5333edec4aeed6caa43dbd57c79797`;
- the candidate-D Compact specialization
  `experiments/moriarty-core-swap/swap.compact` at SHA-256
  `1a564672475b020eafdb96aeb8326f01bcdc2144efec872d44c7020f6e535281`,
  interpreted through `experiments/moriarty-core-swap/artifact-manifest.json`
  at SHA-256
  `b65846b3e2e7eef4b85a10e0de71977d8e7b37e39f835f2cc31041cf35de4569`;
- the closed requirements registry
  `evidence/s02-model-comparison/requirements.json` at SHA-256
  `fc76ecccfde5a20360cb9c725af8eea7910a94fe4460278ca30bd4ffd53eb3b7`.

## Exact outputs

Execution must name and preserve, without predeclaring nonexistent digests:

1. four distinct concrete `.qnt` model representations and their instantiated
   swap and installment workloads;
2. separate `_test.qnt` scenario modules and explicit test-discovery results;
3. the requirement/operation/property coverage map;
4. the generator-independent E00 correspondence report, including both deadline
   cases and abstraction-map mutation detection;
5. raw negative-control counterexample or redundancy-defense traces;
6. raw typecheck, run, test, and verify receipts;
7. the source and binary toolchain inventory;
8. the evidence-backed selection, tie/discriminating-experiment, or stop decision;
9. the independently recomputed validation report; and
10. the complete evidence manifest.

## Acceptance predicates

All candidates must receive the same required workload and safety floor. Every
candidate and signing profile must have completed, decisive checks. Passing the
common safety and nonvacuity floor determines eligibility; it is not a
requirement that every candidate be eligible. A failed safety property requires
a reachable preserved counterexample and an explicit candidate-rejection
disposition. A demonstrated candidate failure does not by itself block a valid
comparison, selection of a safe alternative, or a completed stop decision.

Every candidate must also receive completed witness checks. Eligible candidates
must preserve paths to all required positive and per-major-action witnesses.
Reject-all behavior or a demonstrably unreachable required operation makes that
candidate ineligible. A zero sampled count is inconclusive, not proof of
unreachability. Any missing, stale, simulation-only, or otherwise inconclusive
required evidence blocks the package.

If no candidate meets the floor, the decision may be a completed, evidence-backed
`stop-language-path-with-evidence`. If eligible candidates tie, the decision must
be `require-discriminating-experiment` until that experiment is completed. Cost,
proof, or production claims unsupported by their own evidence are prohibited.

### Closed workloads, safety floor, and witnesses

The exact workloads are `canonical-swap` and
`two-installment-obligation`. The common state-dependent floor is:

- `complete-signed-effects`
- `asset-conservation`
- `nonnegative-balances`
- `authorized-refunds`
- `nonce-replay-exclusion`
- `cancel-fill-exclusion`
- `residual-authority-conservation`
- `authority-bindings`
- `rejection-preservation`
- `display-is-not-authority`
- `settlement-evidence-level`
- `nonterminal-enabled`

The required witness set is:

- `settlement`
- `voluntary-refund`
- `deadline-refund`
- `deadline-input-rollback`
- `extra-effect-rejection`
- `first-installment-residual`
- `second-installment-completion`
- `cancel-wins`
- `fill-wins`
- `after-resolve-execution`
- `before-resolve-execution`

## Negative controls

The controls use the exact closed-registry identifiers and apply as follows:

| Control | Representation applicability |
| --- | --- |
| `missing-deposit-dependency` | A, B, C, D |
| `executed-artifact-substitution` | D |
| `reversed-timeout-priority` | A, B, C, D |
| `installment-replay` | A, B, C, D |
| `residual-expansion` | A, B, C, D |
| `elaboration-corruption` | B |
| `bridge-corruption` | C |
| `agreement-only-advancement` | C |
| `intent-only-consumption` | C |
| `stale-bridge` | C |
| `extraction-corruption` | D |
| `post-sign-plan-substitution` | A, B, C, D |
| `omitted-state-verification` | A, B, C, D |
| `abstraction-map-corruption` | the independent E00 comparison for A, B, C, D |

Each applicable critical control must produce a reachable counterexample under
unchanged remaining premises. A redundant or equivalent control must preserve
evidence of the surviving independent defense. An unexplained critical survivor
fails the package. Trivial rejection of an unreachable attack is not control
evidence.

Every action has an explicit guard and assigns all state variables. The model
must record genuine terminal states separately from unexpected nonterminal
deadlocks. It must not add a blanket stutter action to hide a deadlock.

The witness mapping retains all ten registry scenarios. Under S02-05 and
S02-09, every candidate and both signing profiles must cover the additional
recovery paths `cancel-wins/recovery-before-any-fill` and
`fill-wins/recovery-after-first-fill`, including final escrow, payment, refund,
and authority states. The `SignAfterResolve` path must record a successful
complete-plan pre-sign check before signing; the `SignBeforeResolve` path must
not label a concrete plan pre-sign verified. Execution checks remain separate
from both pre-sign paths.

## Evidence manifest

For every run, the future manifest must record source and executable binary
SHA-256 pins; complete command argv; cwd; initializer; step operator; named
invariants; finite domain bounds; seed when applicable; tool versions; raw
stdout and stderr paths; exit status; exploration depth and completeness class;
witness or counterexample paths; and output digests. It must close the controls,
correspondence evidence, candidate dispositions, and final decision.

Construction uses Quint `typecheck` followed by `run`, with scenario tests
discovered through `test`. Final safety checking uses
`verify --backend apalache`. A stored success flag, summarized log, or simulation
receipt is insufficient. The validator must independently recompute the declared
gate from the pinned sources and raw receipts.

The registry names this backend `quint-apalache`. While the package remains
specified-only, the registry evidence list remains empty and its selected
candidate remains null.

## Excluded claims

The bounded package does not establish `unbounded-proof`,
`cryptographic-authenticity`, `compact-correspondence`, `ledger-execution`,
`actus-completeness`, `human-preference`, `production-cost`, or
`semantic-scope-change`.

## Failure outcomes

- `block-s02-on-incomplete-evidence` applies to a missing, inconclusive, stale,
  simulation-only, mismatched, or unrecomputable required result.
- `reject-architecture-on-counterexample` applies to a candidate with a decisive
  safety failure or demonstrably absent required behavior.
- `require-discriminating-experiment` applies when decisive evidence leaves an
  unresolved tie.
- `stop-language-path-with-evidence` applies only after completed evidence shows
  that no candidate meets the floor and records the repair-or-stop reasoning.

## Rollback

Rollback preserves acquired receipts, frozen Core and swap inputs, the S01
package, and semantic scope. An invalidated experiment and its reason are
preserved and superseded; they are not silently erased or rewritten.

## Semantic-scope transition

S02 is evidence-only at `0.0.0-e00.2`. Candidate lifecycle extensions are model
experiments, not accepted semantic motions. S03 owns any accepted semantic
motion. This package passes no unrelated XML release gate.
