Consensus row C0 (measure the phase profile first) is backed by all nine wave-1 clusters. Wave 2 turns it into an implementable design. No new survey is needed.

1. Pin the version: Sherlock's A3 parent and A4 decider run midnight-proofs 0.8.3; recl9 ran 0.8.2. Design the profile for 0.8.3 first. Name the exact functions or phases to time (key load and setup_pk, witness synthesis, each advice/fixed commitment MSM batch, lookup permutation and grand products, quotient with extended-domain NTTs, multiopen, transcript), from the published 0.8.3 source in `raw/`.
2. Choose the mechanism: feature-gated `tracing` spans or explicit timers in a patched crate. Bound the overhead (target under 1% of wall). Record per phase: wall seconds, CPU core-seconds, peak RSS delta and bytes moved. Write the JSON schema of the profile record.
3. Define the runs: A2 children, A3 parent, A4 decider, plus one B3 batch. Acceptance: phases sum to within 5% of the measured total. Results go to `qa/sherlock/runs.jsonl` style receipts.
4. Explain how the profile decides the other rows: give, for each of C1-C10, the one profile number that confirms or kills its gain bound.
5. Deliverable in addition to FINDINGS.md and EXPERIMENTS.md: `clusters/s9-orchestration/wave-2/OPENSPEC-DRAFT.md`, a draft OpenSpec change "recursion-phase-profile" (proposal, tasks, one spec delta with SHALL requirements and scenarios).
