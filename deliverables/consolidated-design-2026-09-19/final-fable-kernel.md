**Verdict (design consolidation only): changes_requested.** The candidate honors the user's steering on every controlling point. The remaining defects are consistency and boundary-wording issues, not scope failures. All open implementation exits are correctly labeled specified-only and are not counted against it.

**Scope check against the user's constraints**

- One roadmap, one design, one backend contract. P/C/K are aliases with retained obligations. Satisfied.
- Kernel boundary is explicit and optional. Direct Midnight use needs no federation. Satisfied.
- ZR01–ZR16 are concrete, owned and test-specified. Satisfied.
- March 2027 recursion is labeled a user planning assumption everywhere it appears. Full recursive/private multi-parent scope is the target, not a fallback. Satisfied.
- No Lean dependency. Permissionless public programs. MC03/MC06/MC-SP-G denominators preserved. Satisfied.
- Accepted phase partiality may create authorized duties. Request receipt creates no recipient duty. No literal cross-representation hash equality is required in the dispositions. No default perpetual authority. Historical interface findings are dated. Satisfied, with one wording exception in finding 2.

**Required changes**

1. **UNI-017 is inconsistent across the package.** The spec adds UNI-017 for the backend contract. The requirements file states UNI-001–016 are the normative clauses, and the traceability register maps no MPLR row or ZR row to UNI-017. Correction: change the requirements file to UNI-001–017. Map MPLR-027, MPLR-013 and MPLR-029 to UNI-017 in addition to UNI-009/010, and add a line tying ZR01–ZR16 to UNI-017 with owners U0/U1/U4/U5/U7. Reason: a normative clause that no traceability row owns is an orphan and would let a reviewer later argue it is non-binding.

2. **ZR03 "equality links" can be read as literal cross-representation hash equality.** The backend contract requires the application circuit to enforce "the required equality and refinement links" over program, intent, domain, predecessors, effects and duties. The traceability dispositions correctly say different representation hashes need correspondence, not literal equality. Correction: in ZR03, state that equality applies only within one canonical serialization, and that source/Core/ZKIR/ledger representations are related by a pinned, checkable correspondence commitment. Reason: without this the backend contract contradicts the disposition the user specifically preserved.

3. **U5 exit omits the federation trust parameters the design says must be explicit.** The design requires stated membership, threshold, ordering, equivocation, availability and epoch-change rules, and assigns ZR15 partly to U5. The roadmap U5 evidence column lists adapters, ZK/MPC/TEE statement, OWS, x402, reservations and two solvers, but not these parameters or epoch migration continuity. Correction: add to the U5 evidence column a signed federation policy naming those six parameters, plus an epoch-change test preserving duties and consumed authority under ZR15 and UNI-015. Reason: the bare-threshold bypass hazard the design names is only bounded if the threshold and epoch rules are part of the accepted artifact.

4. **"Admission" is not assigned a layer.** The conditional settlement section says admission must establish a viable closure/recovery path or reject the workflow. It is unclear whether this is the program's own acceptance relation or kernel service admission. Correction: state that viability admission is a program/ledger acceptance predicate under the user's signed policy, and that kernel service eligibility may decline service but cannot reject a program that Midnight accepts. Reason: otherwise a kernel operator gains a de facto validity gate, which contradicts the Permission row and UNI-001.

5. **Recovery authority lifetime should be explicitly signed.** The design permits a narrowly scoped recovery authority to outlive ordinary authority "under its signed conditions." It does not require that the recovery authority itself carry a signed termination rule, revocation path or terminal tombstone. Correction: add that recovery authority must declare its own expiry, or owner-chosen indefinite duration with revocation, and that exercising the exclusive terminal outcome tombstones it. Reason: this closes the last route to an implied perpetual right while keeping late-result recovery workable.

**Advisory, not blocking**

- Design history section says Midnight "will have comprehensive recursion in about six months." Change to "is assumed by the user to have." The qualifier follows, but the lead sentence reads as fact.
- Same section says general DAG rejection is "superseded" and bounded composition "remains required." Add one sentence that unbounded general DAG history stays out of scope, so the supersession is not read as unbounded.
- Backend contract typo: "a independently verified."
- The Pel template reads status, candidate and findings fields from a failed verify result whose schema differs from a review result. This is an explicitly open binding per the workflow file, not a design defect.

**Distinguished from findings:** every U milestone exit, ZR acceptance test, native recursion interface, private handoff mechanism and cost measurement is explicitly open. None of these is treated here as a defect. Approval of the consolidation should follow once findings 1 through 5 are applied.