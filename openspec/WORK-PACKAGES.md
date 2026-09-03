# Moriarty evidence-gated OpenSpec work packages

These twelve changes translate the approved sprint program into executable
instructions. Each package has a separate evidence gate. A later package cannot
convert an earlier failure into success by widening the Core.

| ID | Sprint | Package | Depends on | Terminal gate |
|---|---|---|---|---|
| WP01 | S00 | Compact DSL feasibility | None | Zero divergence and no unbounded witness path |
| WP02 | S01 | Evidence and taxonomy | None | Reproducible corpus metrics and genuine-rater protocol |
| WP03 | S02 | Marlowe delta | WP02 for the final roster | Every inheritance claim has exact evidence |
| WP04 | S03 | Core semantic scope | WP03 | One disposition for every Core motion |
| WP05 | S04 | Normative assurance | WP03 and WP04 inputs | Select one maintainable assurance strategy |
| WP06 | S05 | High-risk composition | WP01, WP04, WP05 | Conservation and authorization checks pass |
| WP07 | S06 | Seven demonstrations | WP01, WP03, WP04, WP05 | Seven bounded applications compile |
| WP08 | S07 | 72-row coverage | WP02, WP03, WP04, WP07 | Every row has a reviewed disposition |
| WP09 | S08 | Complete development SDK | WP01, WP04, WP07 | Every component has a specification and verifier duty |
| WP10 | S09 | Real proof and cost | WP01, WP05-WP07, WP09 | Real measurements meet approved budgets |
| WP11 | S10 | Independent evaluation | WP02, WP07, WP09, WP10 | No open critical or high finding |
| WP12 | S11 | Council and decision | WP01-WP11 | Record language, library-only, or stop |

## Global execution rules

1. Pin every repository and toolchain input.
2. Preserve raw outputs and SHA-256 digests.
3. Mark unexecuted experiments `specified-only`.
4. Update the smallest applicable wiki page.
5. Record contradictions without silently selecting a source.
6. Run focused tests before the repository test suite.
7. Stop on a failed package gate.
8. Record the fallback named by that package.
