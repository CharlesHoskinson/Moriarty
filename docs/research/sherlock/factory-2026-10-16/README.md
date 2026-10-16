# Factory research and review

2026-10-16. Source baseline: `9bcbca52bbfbc58443f6bdb287d73afb7d71f1ab`.
Nine planning reviews (three requested Gemini, three Opus 5.5, three Astra high)
returned before ten additional Luna agents researched public best practices.
The [original proposals and hash manifest](../../../../raw/reviews/sherlock-factory-20261016/manifest.json)
preserve disagreement. They are recommendations, not implementation or acceptance.
Provider prompts and raw transcripts are not published.

The resulting [working factory](../../../DOCTORWATSON.md) uses local CI,
existing independent observations and direct Git/PR delivery. The user's latest
instruction rejects hosted CI costs and controls older proposals mentioning
GitHub Actions. The detailed [V1 draft](ROADMAP-DRAFT.md) and
[feature decomposition](SPRINTS.json) retain downstream scope; they are advisory
inputs, not a second canonical roadmap or executable dispatch manifest.

## What we adopted

| Research | Primary source and finding | Application and limit |
|---|---|---|
| Independent agent tasks | [OpenAI multi-agent documentation](https://developers.openai.com/api/docs/guides/agents-api/multi-agent) describes separate contexts and clearly scoped independent tasks. | Bounded Luna inventories and fixtures; shared decisions stay with experts. This does not measure Luna's correctness or speed. |
| CI delivery | [GitHub required checks](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks) documents skipped/pending checks and merge queue triggers. | Useful background only: user selects local CI. No new automatic Actions job or branch-protection claim. |
| Compiler checking | [Alive2 README](https://github.com/AliveToolkit/alive2/blob/master/README.md) documents source/target validation with explicit unsupported transformations. | Compare declared common domains and report gaps. Tests are not general refinement proofs. |
| Regression replay | [Hypothesis API](https://hypothesis.readthedocs.io/en/latest/reference/api.html) distinguishes explicit examples, seeds and version-specific failure blobs. | Preserve complete concrete failures as permanent fixtures. No Hypothesis dependency added. |
| Circuit test limits | [Halo2 issue 820](https://github.com/zcash/halo2/issues/820) discusses non-unique decomposition and its assumptions. | Input to Opus's security work; an issue or honest witness pass cannot certify Sherlock constraints. |
| Executable semantics | [K user manual](https://github.com/runtimeverification/k/blob/master/docs/user_manual.md) distinguishes execution and reachability claims. | Require actual rule/state observations; source emission is not execution. Manual is under construction. |
| Finite model traces | [Quint cheat sheet](https://quint-lang.org/quint-cheatsheet.pdf) illustrates chained steps and assertions. | Explicit finite scenarios and projection contracts. This source does not establish CLI seed/export/replay behavior. |
| Recursive reference | [Lean recursive definitions](https://lean-lang.org/doc/reference/latest/Definitions/Recursive-Definitions/) explains structural recursion and termination checking. | Extend existing reference work; required developer QA stays separate from the public runtime. |
| Build isolation | [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) describes feature unification and additive features. | A disabled feature alone is not an isolation guarantee; inspect the actual ordinary command closure. The first local seam uses Python stdlib. |
| Target evidence | [Midnight deployment guide](https://docs.midnight.network/guides/deploy-and-operate) describes providers, transaction metadata and indexed state observation. | Require actual Sherlock compiler/proof/target readback. Compact examples establish neither custom-ZKIR support nor the requested stagenet configuration. |

Each `research-*/SOURCE.json` records the agent's actual fetch scope and limits.
The hermetic-build researcher fetched no usable source (two 404 responses).
The property-testing researcher initially had search snippets but no readable
body. Root subsequently read the actual Hypothesis API, including explicit
examples and seed/failure-reproduction sections, and the Cargo features page
(feature-unification section). Those two sources support only the narrow claims
above. The unavailable Quint CLI page remains unavailable. No inaccessible
page is counted as inspected primary evidence.

## Corrections retained during synthesis

The [Opus addendum](../../../../raw/reviews/sherlock-factory-20261016/opus-1-security-addendum.md)
governs conflicting original security proposals: required K/Quint/Lean-Clean QA,
12-bit multiplication carries rather than Boolean carries, honest overflow
coverage in the 18 authored tests, exact fuel precedence, C4 sequencing and
Sherlock's own required Midnight stagenet run. A108 charging is governed by the
existing E1/E-FUEL conditions; the addendum's limited-input uncertainty does not
reopen that decision. M5's k ≤ 19 applies to fraud circuits; all other actual
native bounds also remain unchanged.

Gemini delivery suggestions mentioning nonexistent crates, stale unmerged work,
a fixed six-lane ceiling, broad backend Cargo execution, or a 221-test completion
denominator are not adopted. Proposed feature separation is not implemented
isolation. Removing Foreman does not waive existing trace postgates or restart
terminal attempts. Original reports stay unchanged so these differences remain
reviewable.

The initial local check supplies corpus/static-source evidence. K execution,
complete cross-layer adapters, Lean/Clean component correspondence, native
proofs, stagenet qualification and every full U/M/MC/SP/G exit remain open.

The user subsequently named the factory DoctorWatson. Original reports and the
advisory V1 draft retain their original filenames and command proposals; current
commands and scope are in the linked DoctorWatson plan.
