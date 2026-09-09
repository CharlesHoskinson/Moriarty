# Independent GPT-6 README small-step audit

Verdict: **PASS** for the reviewed documentation addition. No blocking findings.

Reviewer: fresh delegated GPT-6 Astra reviewer (`gpt-6-astra`), independent of the author. Review date: 2026-09-09. Checkout: `/home/charl/Moriarty/.worktrees/readme-small-step`, branch `docs/readme-small-step`, base/HEAD `9aa03189c44d956e3c4b21079fffa9b92ce78e87`. Candidate is the uncommitted README-only addition, lines 184–242. Approval applies only to the exact README hash below.

## Scope and commands

Read `AGENTS.md` and `plugins/moriarty-dev/skills/develop/SKILL.md`; applied the repository development review workflow. Ran `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`. Status reports unresolved operational history for loan-swap implementation and no pending transactions; this audit does not dispatch that blocked work.

Inspected `git status --short`, `git diff -- README.md`, `git rev-parse HEAD`, the complete executable K definition, codec admission/encoding/decoding, the K README, fixture records, published acceptance/execution/comparison records, source and K observations, and the linked sprint table. Ran `git diff --check` successfully. Used standard-library-only `python3 - <<'PY'` assertions to independently evaluate the displayed equations against all six positive expectations, compare all sixteen complete recorded K/source/expected objects, count guard indices, validate section links/fences, and calculate SHA-256 hashes. No implementation or codec import was used for the arithmetic checks.

No K compilation/execution, proof attempt, source evaluator execution, network call, installation, or product file modification occurred. This is a static source/evidence audit with independent arithmetic, not new empirical K coverage. GitHub Markdown was reviewed as source (balanced text fences, a conventional three-column table, defined Unicode notation, valid local link targets); browser rendering remains the root agent's separate check.

## Findings

1. The five displayed control rules accurately summarize reachable admitted configurations. START resolves row indices; EXPAND installs the ordered checks and finish continuation; CHECK-PASS removes one successful guard; CHECK-FAIL empties the entire continuation and writes the rejection; FINISH empties computation and emits the prepared fields. Helper reductions are explicitly distinguished from displayed control transitions. The text does not equate two work units with two K rewrites.
2. All 21 ordered guards and diagnostic indices match. State has four checks at index -1 (distinct balance pairs, allowance UInt128 sum, debt sum/status invariant, work sum including reserve); work has two at -1 (remaining >= 2, spent + 2 bounded); Transfer has eight at 0 (positive amount, distinct parties, sender exists, sender sufficient, allowance identity, allowance sufficient, spent overflow, receiver overflow); Repay has seven at 1 (positive amount, obligation identity, outstanding status, bounded debt payment, same-step transfer ID, payer/creditor/asset match, sufficient unallocated transfer). The compressed README table preserves this order and links to the exact predicates. The codec maps internal -1 to a null action index; the table is explicitly about the displayed K control layer.
3. Both allocation equations agree exactly with `principalPart`, the finish rule, and the independent fixtures. Debt decreases by nominal N; balances and allowance change by transferred T. Guards ensure successful subtractions remain valid. Work changes by (-2,+2) and reserve is copied unchanged.
4. Sender/receiver indices preserve input row order. Missing sender rejection and missing receiver amount/append behavior are correctly described at the combined K/codec boundary. K computes the appended balance amount; the codec constructs its metadata and list entry. The paragraph explicitly states the codec reconstructs metadata/effects/used-ID appends and remains trusted and unproved.
5. The executed principal-partial example is exact: balances (70,30), principal/accrued/outstanding (70,0,70), Outstanding, allowance (70,30), work (98,2,16). Rejection output contains code/index with no post-state or effects.
6. The lowered repayment profile is clearly separated from both the preceding successor syntax grammar and funded-source preparation profile. Finite comparison, unsupported full semantics, unproved correspondence, and lack of authorization/proof/ledger settlement are stated accurately. Broader action semantics also have SP07/SP08 dependencies in the linked sprint table; the compact SP03/SP09 pointer does not claim those broader actions are implemented.
7. Published records support sixteen complete comparisons, six positives and ten financial rejections. Independent equality checks on retained objects passed for every case. The retained observation/fixture/source hashes match those named by the complete-comparison record. This audit does not expand the old evidence's coverage or replace the required separate Opus review.

## Independent equation checks

Each row also checked both balances, allowance remaining/spent, work remaining/spent/reserve, settlement amount, discharge effect fields, remaining-outstanding effect, Transfer effect identity, and used-ID appends against fixture expectations.

| Fixture | T | N | Principal/accrued discharge | Result principal/accrued/outstanding | Result balances | Result |
| --- | --- | --- | --- | --- | --- | --- |
| principal-partial | 30 | 30 | 30 / 0 | 70 / 0 / 70 | 70 / 30 | PASS |
| interest-first | 7 | 7 | 0 / 7 | 100 / 3 / 103 | 93 / 7 | PASS |
| principal-first | 7 | 7 | 7 / 0 | 93 / 10 / 103 | 93 / 7 | PASS |
| full-payment | 100 | 100 | 90 / 10 | 0 / 0 / 0, Settled | 0 / 100 | PASS |
| cross-interest-boundary | 15 | 15 | 5 / 10 | 95 / 0 / 95 | 85 / 15 | PASS |
| overfunded-transfer | 40 | 30 | 30 / 0 | 70 / 0 / 70 | 60 / 40 | PASS |

All nonzero residual debts above remain Outstanding. Every case's work result is (98,2,16).

## Exact SHA-256 hashes

- `README.md`: `007e5c6caec0f5da4dcb0a98adf7001d4f87384b604d63cb1611cde96c49c2d9`
- `experiments/moriarty-language/formal/k/moriarty.k`: `0e695ac53e7daa04de226e4f95a20d1f46bebaee02e89db7977a9b1be314c0cd`
- `experiments/moriarty-language/formal/k/codec.py`: `923120b9628bca2b0e5c1cdd2488b0811f432d663d76a9e93293f5f6c6180f1d`
- `experiments/moriarty-language/formal/k/README.md`: `0a98f2d0723ea6e03049cd3d185dd7258664a4dd761d5385b64a371e525dc47a`
- `experiments/moriarty-language/formal/k/fixtures/cases.json`: `d46679961af39a10adec03cb86b4481a39fc9393d15ceb48b37b10b8ce28f832`
- `deliverables/bounded-k-2026-09-09/acceptance.json`: `1f089ed3e2011118dbab7507309f7258b80bce1e768b06c0b048113734d2aa02`
- `deliverables/bounded-k-2026-09-09/execution-result.json`: `2bd070c1f0fd44b2b89f9497f4bc12e5fab7645db9d9c7bd47d505cb430bd41c`
- `deliverables/bounded-k-2026-09-09/complete-result-comparison.json`: `5c509b4413975beddf583154f125e8ff12e229fe940a5909e81bbfbd40bf5b05`
- `deliverables/bounded-k-2026-09-09/source-observations.json`: `1aa2618e18c7022980095d8a12eb8d84e7e16753323bac5f37857d5b33e2d918`
- `deliverables/bounded-k-2026-09-09/attempt-03/observations.json`: `777b9bb7f71a070440bfec21e192b60ac706bf0ae0acf2c4ff411abdca77c0f8`
