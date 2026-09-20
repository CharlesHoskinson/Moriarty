**Verdict for the final consolidated design: approved.**

The Mina addition (MNR01–MNR08) is evidence-grounded, stays inside the existing ZR01–ZR16 contract, and does not import a Mina backend. All prior consolidation corrections remain intact. The user's March 2027 full-recursion planning assumption, no Lean, and no public permission gate are preserved in every governing file. The findings below are precision notes, not blocking defects.

**Why MNR01–08 hold up as refinements**

- **Each row traces to root-verified source.** MNR01–07 cite M01–M08/E01–E16 and D01–D10, whose hashes and excerpt ranges were independently checked. MNR08 cites the 2023 audit and explicitly says the study is not a new audit.
- **No backend import.** MNR01 disclaims Pasta/IPA encoding transfer, MNR06/07 disclaim exploit claims from the SRS TODO and empty-batch convention, and the closing paragraph disclaims interchangeable proofs and theorem transfer.
- **No contradiction with existing ZR text.** MNR01's "semantically changed parent ordering rejects" is compatible with ZR08's permitted order-insensitive batching. MNR02's constrained masks are compatible with ZR07's permitted optional guards. MNR03/MNR07 keep the settled distinction between per-stage fan-in and growing finite ancestry (ZR09).
- **Ledger induction versus native recursion** is unchanged: UNI-009, the design's history section, and the MC03 disposition all still forbid closure through induction.

**Non-blocking findings**

1. **`docs/MORIARTY-BACKEND-REQUIREMENTS.md`, Mina refinements table, MNR08 source basis.** It cites "physical pp3/5/6/7/8". RESULT.md states only pages 3, 5 and 6 were visually checked; pages 7 and 8 were read from extracted text. Correction: annotate pp7/8 as "extracted text only". Reasoning: the dossier standard (UNI-016) requires evidence class to match the claim.

2. **Same file, MNR table versus "Responsibility and acceptance ownership" table.** MNR rows have no primary implementation owner column, and the ownership table was not extended. Correction: add one sentence stating that each MNR row inherits the implementation owner of the ZR rows it refines. Reasoning: requirements.md says MNR owners "are recorded in the backend contract", but only U owners are recorded in traceability.

3. **Same file, MNR01 wording.** "cross-field/limb conversion" is Pasta-cycle language. Correction: qualify as "where the qualified backend verifies non-natively". Reasoning: in-circuit KZG/pairing verification on Midnight's curve will likely need non-native arithmetic, so the obligation is real, but the phrasing should not presuppose a curve cycle.

4. **`traceability.md`, Mina ownership table, MNR06.** ZR15 is owned U4/U5 and kernel-operated provers appear at U5, but MNR06 lists U1/U4/U7. Correction: add U5 for kernel prover cache provenance. Reasoning: consistency with the ZR14/ZR15 U5 profiles.

**Open items that are correctly left open, not defects**

- No Midnight transcript, deferred-obligation schedule, or batching contract exists yet. MNR01/02/07 specify what U1/U4 must obtain, which is the intended posture.
- The developer report says ZR08 finalization was not inspected on its side. The semantic report covers it (M02/M04), so MNR02 is grounded when both reports are taken together, as the table's source basis indicates.
- Mina gitlink mismatch for o1js and Snarky is disclosed and no integrated tuple is claimed.

Next step for the maintainers is the four wording edits above; none changes an acceptance predicate, owner milestone, or scope claim.