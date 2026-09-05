# Candidate B native design review and local adoption

Date: 2026-09-05 UTC. Classification: repository observation, experiment
observation and delegated design decision; no implemented-model claim.

Root authored the design. Native nonauthor `/root/b_native_design_review`,
dispatched as GPT-6 Astra formal-methods reviewer, reviewed it read-only. This
record summarizes its delivered review messages; it is not a provider transcript
or the required exact-provider Council. The reviewer ran no Quint checks.

## Original review and corrections

Original SHA-256:
`d21b52d9a5de76fca0aa08d9d62ed04909aa172faae9ee0dc9602691d4b6c820`.
Exact bytes: `proposed-design-original.md` in this directory. Verdict: hold.

1. The proposed swap minimumTime1 silently excluded the existing semantic Time0
   request. The existing A harness starts from emptyAState minimumTime0; its
   later Time1 restrictions schedule corpus records, not evaluator semantics.
   Root reproduced the frozen Python difference in `time0-probe.json`: the
   same deposit10 at now0 accepts from minimumTime0 and rejects time_before_state
   from minimumTime1. The first probe had a formatting AttributeError, retained
   separately; only the corrected terminal receipt establishes the observation.
2. Generic admitted graphs can strand escrow: a single deposit10 obligation
   exhausts its frontier while retaining funds. Excluded prerequisites can also
   leave unresolved obligations without a frontier. These were static design
   consequences, not executed B counterexamples. The revision defines disjoint
   graph status outcomes and requires both negative experiments. It does not
   invent Core Close behavior or claim universal generic non-locking.
3. No-timeout deadline error priority was ambiguous. The revision requires
   BNoMatchingInput before considering supplied input when no timeout exists.

The revision also requires actual nonlibrary execution evidence, complete
successor validity with visible diagnostic failure, and named mapping failures.
The reviewer found no static canonical-prefix, payment-count, dependency or
exclusion contradiction; this is not a checked preservation proof.

## Re-review

Revision SHA-256:
`3b35bb6697b47059fc187f94c813714c1f6e03fa2928d8ae071049dffbb1b76b`.
The reviewer confirmed the three blockers resolved, with two clarification
repairs: BClosed also rejects supplied input at an available timeout; mapping
failure must cover complete successor state, not merely continuation.

Root applied those two changes. Final reviewed design SHA-256:
`2cc795c4ad3a83c3b7b5ecf6ff39ef0af486a0ae1a864fa2fe3e0b24fe2f70ac`.
The reviewer recomputed that hash and read the changed paragraphs. Its final
disposition was pass for staged B implementation, with no outstanding blocker
from this bounded review.

## Local adoption

Root adopts the exact final bytes of
`docs/superpowers/specs/2026-09-05-moriarty-s02-candidate-b-native-graph-design.md`
as the staged B implementation contract under the user's existing delegated
design authority. This separate disposition preserves the reviewed bytes.

This does not select B or complete S02. Native implementation, runtime traces,
independent full-inventory correspondence, authority integration, requested
model checking and exact-provider Council are still required. Generic non-locking
remains a disclosed restriction to evaluate, not an assumed property. Foreman
development remains closed.
