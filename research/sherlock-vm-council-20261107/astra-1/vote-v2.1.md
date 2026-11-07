# VOTE astra-1 ACCEPT — DRAFT v2.1

I **ACCEPT** the Sherlock VM consensus change, DRAFT v2.1. This supersedes my v1 OBJECT and v1.1 pending review. All six numbered objections and the two v2 cleanup items are resolved. I have no remaining soundness or security objection to this design specification.

Reviewed artifact: the 13-file snapshot in `astra-1/review-v2.1/`, identified by `astra-1/review-v2.1.sha256`. At 2026-11-07 21:28Z, every snapshot hash matched the chair's live bundle. The structural check found 56 unique requirement IDs, 56 unique capability headings, no missing requirement-to-spec or traceability entry, and correctly aligned assurance O1-O7 rows. The result is saved in `review-v2.1-checks.json`. These checks establish document structure and snapshot identity only.

The three most important assurance decisions are:

1. `assurance.md` separates finite release evidence from universal obligations, states the invariant/configuration premises, and distinguishes IR preprocessing, constraint rejection, native verification and target observations. SAS-003/004 preserve rejection boundaries, valid baselines, raw malicious witnesses and weakened controls. The independent trace oracle remains separate from the application witness stream.
2. SVM-002/SAS-010 and tasks 4.6/4.9 require authenticated initial carriers, invariant preservation and shared-carrier premise checks. SRC-005 uses an explicit genesis realization with matching tests, without treating transcript hashing as a cancellation proof. Dispatch coverage includes complete guard dependencies, directed admitted-rule cases and honest heuristic labels; known omissions cannot earn the receipt.
3. Qualification depends on declared source/spec/key/target/artifact dependencies and exact candidate receipts. Frozen inventory R precedes fresh P3; P3 parity and local verification precede submission. All applicable security gates remain mandatory. Optional B work stays optional while Sherlock fee/DUST evidence belongs to mandatory A8p/A10p checks; adopting an optimization activates its assurance gates.

Remaining work is explicitly implementation work, not council dissent: execute the specified checks and native proof/ledger path; establish the universal properties if they are later claimed; measure the leaf shape and complete-path resource cost; and preserve the deployed-source reproduction limitation in SMN-004. The council vote is not proof execution, deployment acceptance or a completed release.

I remain active until the chair's DONE or the brief's cutoff, then will write `final.md`.
