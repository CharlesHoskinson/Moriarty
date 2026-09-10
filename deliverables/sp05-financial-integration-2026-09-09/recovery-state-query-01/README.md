# Recovery state-query repair

The read-only diagnostic failed before private recovery. The block filter selects a contract action in that block; a later empty block returns null. Captured exact source SQL and an unchanged fixture using the pinned real SDK reproduce the failure.

The repair reads latest state between fresh matching indexed/finalized tip observations, retains historical canonical checks and requires complete native-state equality. All 291 ledger tests pass; the same actual-SDK fixture goes RED to GREEN. These tests use synthetic transport and do not prove live recovery.

Candidate04 is approved independently by GPT-6 Astra and Grok4.6 high. Both separately approve the bounded recovery02 resource proposal and the failed contained diagnostic result. Grok completed in 393.283 seconds under a 900-second allowance without caller cancellation. Exact source/proposal/commands/runner hashes and substantive reviews are retained here and in ../local-recovery-02.

Latest-state consistency trusts coherent local indexer/RPC observations. It is not cryptographic inclusion or a guarantee against future state changes. Original deployment, wallet/role identity, private storage, native bytes and every prior charge remain preserved. Actual recovery and initialize/accrue/settle, swap, Preview and full SP05/PCD acceptance remain open.
