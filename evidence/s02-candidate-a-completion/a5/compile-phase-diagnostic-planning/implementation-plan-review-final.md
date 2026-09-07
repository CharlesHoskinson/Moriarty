# A5 compiler phase implementation plan: final CP-R1 review

Spec verdict: PASS.
Quality verdict: PASS.
CP-R1 is resolved in the corrected plan. These verdicts combine the original
complete source review with this narrow correction review. They admit the plan
for root consideration, not materialization, control execution or full dispatch.

Reviewed corrected plan:
`docs/superpowers/plans/2026-09-06-candidate-a-compiler-phase-diagnostic.md`

SHA256: `b1df377b077cb7189f945ac493039e1f8d941f5076f61c1be6aaf8559de7bff9`.

Adopted design SHA256:
`19ff70c32004d824ee1b5e34e722425a354412d3548f321a943b6e32fc9ca67a`.

## Correction inspected

The complete diff against the preserved original contains only the movement
of raw/content predicates into the existing child loop and the corresponding
CP003 sequencing sentence. In the corrected plan's `record.py` block,
lines 685–702 now check each direct baseline after its ordinary terminal gate,
then check each observed pair before the loop can start another child.

- Success-direct validates JSON shape, main, selected `q::init`, `q::step`,
  `q::inv` declarations and empty stderr before success-observed can launch.
- Error-direct validates empty stdout and genuine name-resolution stderr
  before error-observed can launch.
- Each observed child compares both original stdout and stderr byte-for-byte
  with its already validated direct baseline. In particular, a success-pair
  mismatch prevents both error children from launching.
- Predicate failures raise into the unchanged exception/finalization path.
  The child lifecycle was appended before spawn, so that child's cleanup and
  original artifacts remain covered even when its result is not added to the
  successful-results map. No retry or continuation path was added.
- Four-child order, exact inputs/argv, 120-second and 4096-MiB settings,
  trace/argv/resource gates, source/runtime checks, terminal preservation and
  final all-four `checks.json` predicate remain unchanged. That final predicate
  is reached only after all four iterations and their immediate checks finish.
  The full diagnostic branch and its 900-second budget are unchanged.

The three JavaScript blocks are byte-identical to those reviewed originally.
Their observation, launcher and mock-control behavior therefore receives no
new implementation claim from this correction. Root review/adoption and all
actual execution gates remain required. H1 remains unresolved.

## Preserved review chain and verification scope

Rechecked the following small-file hashes and retained the original files:

| Artifact | SHA256 |
| --- | --- |
| `.superpowers/sdd/a5-phase-plan-original-20260906.md` | `6a861022ed8641e2674f36f5725636c79550fffe3550928012694331cc0d76e4` |
| `.superpowers/sdd/a5-compiler-phase-implementation-plan-review-20260906.md` | `9091fb1864de49500d048e5e7926a26b466b20c49c4e9e8ba67820cf6d4c31cc` |
| `.superpowers/sdd/a5-phase-plan-correction-20260906.md` | `4952cefce0986df6d40be34105d1a1495345e9d0e224a633075ffbd46a9f82e4` |

Read the correction report and complete textual diff. Compared all three
JavaScript block contents as data and checked the four-block inventory. No
proposed code was imported, materialized or executed; no native/mock/test tool
ran and no large artifact was hashed. Only this new review file was written.
