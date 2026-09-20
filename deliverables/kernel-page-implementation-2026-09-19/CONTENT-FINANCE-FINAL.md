# Final independent financial-content review

Verdict: PASS within the financial-content scope, for exact candidate-03. F1 and F2 are resolved; no remaining blocking factual financial-copy issue found.

Checkout: `/home/charl/Moriarty-pages-20260919`. Review compares the complete article and rendered scenario/component strings against the approved page design, product contract, consolidated design, language alignment, roadmap and financial MPLRs identified in `CONTENT-FINANCE.md`. The earlier full-source review is supplemented by inspection of every candidate-02 to candidate-03 changed line; unchanged file hashes and unchanged portions retain their reviewed identity. No other reviewers' reports were consulted. Candidate files were not edited.

## Findings closed

- F1: `site/kernel.html` now defines custody as the last confirmed accounted balance and explicitly marks the pending amount's location unknown. The static timeout/unknown transcripts and unknown-outcome prose repeat that distinction. `KernelExplorer.tsx` labels the row “Last confirmed escrow balance”; its always-visible note says it is not a live reading, with an in-flight explanation that the destination may already have executed. Timeout and unknown event descriptions carry the same qualification. The observed-account arithmetic remains unchanged. This resolves the unsupported current-custody implication.
- F2: `kernel-scenarios.mjs` now gives the delivery duty's discharge condition as “Authenticated confirmed receipt of every required fill, with each fill accepted against all signed predicates.” This matches the separate observation and acceptance states and the duty retained after external success with missing predicates.

The complete financial narrative still distinguishes consent from proposal, funding from gross spending, incurred fees from pending fee exposure, reservations from custody, partial accepted progress from completion, external effects from acceptance, unknown from authenticated nonexecution, recovery from compensation, and consumed terminal results from new authority. OWS/x402 remain planned and preserve payment/availability/delivery and logical-request distinctions. No new deployed-kernel or unconditional financial-recovery guarantee was introduced. New links identify the already-scoped historical loan/swap and K results; this review does not independently reproduce those results.

## Exact change-scope verification

All 17 candidate-03 file hashes match the checkout. All 17 frozen-02 files match candidate-02. Six files changed: `site/kernel.html`, `site/src/data/kernel-scenarios.mjs`, `site/src/data/kernel-scenarios.test.mjs`, `site/src/kernel/KernelExplorer.tsx`, `site/src/kernel/evidence.ts`, and `site/test/kernel.spec.py`.

The changes are text, source links, presentation of the custody explanation, and corresponding copy assertions in tests. In particular, the scenario module is byte-identical to frozen-02 after substituting exactly three display strings: delivery discharge, timeout meaning, and unknown meaning. There are no changes to reducer guards, arithmetic, policy, state transitions, consumption, availability rules or account effects. This directly verifies that reducer logic did not change since candidate-02. The UI gained a conditional text note for the existing in-flight phase; it does not change event dispatch or state behavior.

## Limits

This is scoped financial-content approval, not general code, cryptography, accessibility, deployment, or native-proof approval. Tests were inspected for scope but not executed by this reviewer. The prior report's source and empirical limitations remain applicable. No candidate edits or network acquisition occurred.

## Candidate-03 SHA-256 identities

- `.github/workflows/site.yml`: `29113ceb3fbf13103f4af5ec3889357c5bad162efab136727e6a4acb9580e8e7`
- `README.md`: `4feebfd848a2253cb36589084744d036314d2eeb7f1a5182ac5c1418b3668022`
- `site/CONTENT-SPEC.md`: `d34b2a4a6e4b259c6853fdb62551b386261d6b686e688f23e9f704d3bac29722`
- `site/kernel.html`: `c33ab0367bb9a12938af374e4677ec5c5c63a4f223b27b5ab01843fba18e99d1`
- `site/package.json`: `8e0147105e4f26857997215d32ab74ace160e20cf8b7e46b56667de5e60824d5`
- `site/src/App.tsx`: `aca077ac237c1e8393b97ade3bb36ee46bc08d8aeb46349364c48ee27b59c371`
- `site/src/data/kernel-scenarios.d.mts`: `906ef089b8ccebdb387524d87175a9c9d7a4948ba8847f7cc447edce326e5b88`
- `site/src/data/kernel-scenarios.mjs`: `c2a989b14d09c2d7e2e08f36337fd3d9653e2d3da2c02da6988fbe7ad4fcebf8`
- `site/src/data/kernel-scenarios.test.mjs`: `60f4cff4c09c91b55c740879d456306ee964ed512eef522cda3086682625176a`
- `site/src/kernel/EvidenceInspector.tsx`: `3368fe0d6e5325578af01e3ed3f5dac1a2128bda6d120afe315ba574600665db`
- `site/src/kernel/KernelExplorer.tsx`: `ede763dcf43cc9cd17da764b3795b3e6531ae42fdc89d69a35a720f9f1619e56`
- `site/src/kernel/evidence.ts`: `f08e662bfe397da37b63148e560585e8f9b212191e51a77440c5502ca3750549`
- `site/src/kernel/main.tsx`: `3283b83e810b917913b382f202a19e34f28936275987b54b08b8612525b6ff55`
- `site/src/kernel/styles.css`: `c50757b3a69218839aed3fd0f434d03d465b74fb789b57a31ae1ad3545601dda`
- `site/test/README.md`: `6dcbee33cfe2fbc61c0750030ba5abc183da7194788868e3edd110cb104d3b27`
- `site/test/kernel.spec.py`: `078edfd3425b08a8589c252f23b1ce48758d01939fbaa302e1400646d0784ee9`
- `site/vite.config.ts`: `1882275187f40f31470fc65f999baa406e9926f984b59643f5f0459989aee295`
