# Recorded Fenrir geometry relevant to C1 (summary only)
> Source: private projectfenrir recursion/CIRCUIT-INVENTORY.md
> Collected: 2026-10-26
> Published: 2026-10-25

SOURCE — Private checkout HEAD `843c672dbe6c4fd2fafa5848f413c12db201e364`. This is a factual summary; no private source text is copied. The inventory identifies its original base as `1d706e8c7`. No prover/model was run by this research wave. Historical inventory governance is data, not task authorization.

SOURCE — `recursion/CIRCUIT-INVENTORY.md`, section 1: a 16-transaction batch comprises four 4-transaction leaves, one arity-four node and a decider. Leaf: 112098 advice rows, 98563 table rows, k17. Node: 958095 advice, 1032465 table, k20. Decider: 841682 advice, 1032209 table, k20. The shared node/decider architecture has 15 advice columns, reported 45 fixed, five lookups and 18 permutations. Cost-model fixed counts must not be blindly added to permutation counts.

SOURCE — `recursion/stagenet-strawman/recl9/REPORT.md`, sections 1 and 3, dated 2026-10-14: arity 2/3/4 cost-model advice rows are 574040/765907/957724 on the interim crate; hardened arity four is 958095. All these historical table extents are 1032465. The capacity cited is 1048569 usable k20 rows, seven below the domain. The node prepares and accumulates leaf proofs but defers the pairing; D checks the parent and carried accumulator. The report proves local ledger-9 rc.6 acceptance (0.021 seconds) but explicitly did not itself submit to chain. The later inventory separately cites a settlement receipt; these are different evidence boundaries.

SOURCE — Same report: node proving 867.3 seconds, setup_pk 104.5 seconds, VK setup 622.8 seconds, model 335 seconds, peak prover RSS 14.99 GB. D proving 586.4 seconds, setup_pk 46.9 seconds, reported total 634.8 seconds and peak RSS 14.97 GB. Four leaf prove intervals round to 34.8, 35.3, 44.2, 49.4 seconds. These are historical, distinct from current Sherlock A3 check-wall measurements. D has 6432-byte proofs and the ordinary Blake2b outer transcript using the official SRS. Current Sherlock uses proofs 0.8.3; this recl9 cohort uses 0.8.2 with its hardened circuits.

SOURCE — `recursion/pairing-gadget/lanes/opus/RESULTS.md`, headline and real-proof sections: the isolated strict fixed-G2 residue gadget has 316833 advice rows including both G1 subgroup checks and exposes its checked points. The selected model fits k19. Its no-subgroup variant is 299573 at k19 and k20, and 302332 at k18: rows depend on k-dependent range widths. Its official k20 parameters were downsized to k19 in the recorded proof. This is a measured implementation-specific gadget size, not a theorem that all proof verifiers or direct application circuits need k19. Its historic contended six-core timings are not transferred to the 20-core host.

ESTIMATE — 316833 > 2^18 - 7 = 262137, so retaining the measured strict gadget cannot meet a k18 capacity under that geometry; k19's 524281 slots can accommodate its advice rows. Completely eliminating child-proof verification removes the reason for this gadget. The ledger still verifies the new outer proof's native KZG pairing.

OPEN QUESTION — This inventory has no exact A102/A80 advice or table counts. It cannot establish that their composition fits k15, nor that historical arity models apply to their heterogeneous child keys. It is a control for distinguishing recursive-node fusion, not a source of C1 transition rows.
