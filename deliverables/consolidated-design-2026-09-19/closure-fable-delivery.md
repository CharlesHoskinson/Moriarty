**Verdict (final consolidated design only): approved**, with four non-blocking consistency corrections listed below. MNR01–MNR08 are evidence-grounded refinements of ZR01–ZR16 and do not import a Mina backend. The user's constraints are preserved: full native recursion around March 2027 stays a planning assumption and target scope, no Lean, no public permission gate, ZKIRv3 pinned, kernel optional.

**Why MNR01–08 pass the grounding test**

- Each row traces to inspected source findings (M01–M08, D01–D10) with retained pins, file hashes and excerpt ranges, and each hostile case is a direct restatement of an observed mechanism (default-false shouldVerify, empty-batch success, SRS equal-size TODO, proofsEnabled:false dummy proofs, auxiliary output outside the statement, two-parent width, accumulator check as separate entry-point step).
- Each row is phrased as a contract on the qualified Midnight backend, not as adoption of Pickles/Kimchi/IPA/Pasta artifacts. The disclaimer that theorems and audits do not transfer is explicit in all three documents.
- Nothing in MNR03/MNR05 creates a project gate. Key authorization is owner/program policy; production-mode assertion is an evidence requirement.
- ZR16, UNI-017 and the traceability crosswalk were updated consistently; RESULT.md numbering matches the backend table.

**Findings requiring correction**

1. `openspec/changes/consolidated-language-kernel/requirements.md`, final paragraph. It states MNR owners "are recorded in the backend contract and traceability register." The backend contract's MNR table and Responsibility table carry no owner column. Either add an owner column to the MNR table in `docs/MORIARTY-BACKEND-REQUIREMENTS.md` or change the sentence to say owners are in traceability and tests in the backend contract. Reason: a traceability claim must be literally true.

2. `docs/MORIARTY-BACKEND-REQUIREMENTS.md`, MNR03 requirement text, "domains/chunks/rounds". Chunking and opening-round counts are Kimchi/IPA shape parameters. Replace with "backend-specific proof-shape parameters (domain size and, where the qualified backend has them, chunking or opening arity)". Same for MNR01 "cross-field/limb conversion": add "where non-native arithmetic is used". Reason: avoids lexical import of Mina-specific structure into a KZG contract while keeping the requirement.

3. `openspec/changes/consolidated-language-kernel/traceability.md`, Mina-derived ownership table. MNR03 and MNR06 refine ZR15 (owner U4/U5) but list no U5; MNR04 refines ZR10 (owner U2/U3/U4) but omits U3. Either add U5 and U3 respectively, or add a note that MNR rows own only the recursion-specific subset of each refined ZR. Reason: the crosswalk should not silently drop the kernel-epoch and concurrency scopes those ZR rows already assign.

4. `docs/MORIARTY-BACKEND-REQUIREMENTS.md`, MNR08 source basis "physical pp3/5/6/7/8". RESULT.md states all pages were read as extracted text and only pp3/5/6 were visually checked. State "extracted text pp3–8; visual check pp3/5/6" so the reading scope matches the retained record.

**Not defects**

- All MNR rows specified-only, no executed tests: explicitly acceptable for a proposal.
- MNR02 dropping ZR06 from the source report's "ZR06/08/16" tag: acceptable, deferred obligations belong to ZR08; ZR06 remains covered by MNR01/MNR08.
- Overlap between MNR07 and ZR09 on tree joins: complementary, not contradictory.