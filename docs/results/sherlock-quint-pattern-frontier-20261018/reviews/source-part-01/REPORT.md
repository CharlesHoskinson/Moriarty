ACK-QUINT-B10-SAME8A-SOURCE-REVIEW-PART1

### Reviewer Identity & Part 1/2 Scope
- **Conversation ID**: `8a6a2b55-186d-4a62-a8ae-142ea13ba7e7` (continuing same non-author reviewer conversation at original CWD `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/formal-three-profile-fresh-review-20261018/quint-batch01/fresh-provider-cwd`; one continuing non-author opinion, not a second vote or execution result).
- **Requested Route / Observed Model Identity**: Requested standing Opus quota-outage fallback `Gemini4ArgonHIGH`; actual returned model identity `Gemini 4 Argon (High)`.
- **Execution Discipline**: Zero tools invoked, zero commands run, zero file reads/writes, zero delegation, zero model/frontend execution, zero private context disclosure.
- **Part 1/2 Disposition**: **ACK ONLY** for Manual Part 1/2 (`Bodies 1–7`). Final candidate source and two-operation dispatch verdict is withheld until Manual Part 2/2 (`Bodies 8–23`) is delivered and read in full.

---

### 1. Exact Readable Body Coverage (Bodies 1–7 of 23)

All 7 bodies in Part 1/2 were received and read in full without truncation:

| Body # | File / Artifact | Bytes | SHA-256 | First Visible Line / Marker | Last Visible Line / Marker | Status |
|---|---|---|---|---|---|---|
| **1** | `nominalConstruction.qnt` | `18,761` | `77a487c5b7ce84569540faa0a780991d67cf8217d3a1d68a81916426d8943008` | `module nominalConstruction {` | `val completedWitness = machine.outcome == Done(uint(18))` / `}` (`END BODY 1`) | Complete & readable |
| **2** | `patternFrontier.qnt` | `19,257` | `a583233ecd20d82dd92202537bf2364235f1ed53bf04d315091828905b7d778b` | `module patternFrontier {` | `frontierRepeat(prefixes.nth(cut), input.analysisBounds.advances - cut) == completed)` / `}` (`END BODY 2`) | Complete & readable |
| **3** | `patternFrontierInputs.qnt` | `15,962` | `76bd87614a4f7c22e734ea358f5a57b6815d10a54ee2771f7ac075dcb75e7b7f` | `module patternFrontierInputs {` | `pure val frontierCohort: List[F::FrontierInput] = List(frontierInput0, frontierInput1, frontierInput2, frontierInput3, frontierInput4)` / `}` (`END BODY 3`) | Complete & readable |
| **4** | `patternFrontierTest.qnt` | `3,393` | `715c44aa74d004937e39ab2f7bceb786301039295d85ee6b756397bcf8fe29ed` | `module patternFrontierTest {` | `I::frontierCohort.foldl(false, (seen, input) => seen or frontierHasRetainedParentWitness(input)),` / `}` (`END BODY 4`) | Complete & readable |
| **5** | `prepare_safe_frontier_inputs.py` | `7,464` | `ef40319ede16990a49b37e07c56b63c04c6dccae2e3912dcd1c803b902990ab2` | `"""Source-only preparation of immutable input tables/literals. No matching/evaluator.` | `if __name__ == '__main__':` / `    main()` (`END BODY 5`) | Complete & readable |
| **6** | `SAFE-FRONTIER-INPUTS.compact.json` | `6,764` | `c58873d13f5574515f41404dc82e80a1917c105a5cbd3ad354852468152fb8ce` | `{"status":"SAFE_INPUT_AND_PREPARATION_WITNESSES_ONLY_NO_EXPECTED_RESULTS_TRACES_OR_WORK"...` | `..."domain":"Finite acyclic well-formed typed positive nominal graphs and linear arm binders; current source admission/owner integration remains external."}` (`END BODY 6`) | Complete & readable |
| **7** | `SAFE-SOURCE-TO-TABLE-ALL-ROWS.compact.json` | `18,631` | `bafb950674ad3bcf123c636173c69992343dd16cb4a2b9e64812ef828d393782` | `{"status":"INPUT_TABLES_AND_LITERAL_REPRESENTATION_WITNESSES_ONLY_NO_MATCH_RESULTS_OR_WORK"...` | `...{"armIndex":1,"sourceArmId":7051,"path":[],"patternRow":6,"sourcePattern":{"Bind":13}}]}]}` (`END BODY 7`) | Complete & readable |

- **Last Readable Marker**: `END-QUINT-B10-SAME8A-SOURCE-REVIEW-PART1` (immediately following `END BODY 7`).

---

### 2. Provisional Source, Table & Test Notes Retained for Part 2/2
1. **Unchanged B09 Reference (`Body 1`, `nominalConstruction.qnt`)**: Matches the accepted B09 module (`18,761 B`, `sha256: 77a487c5...`), supplying `Token`, `Value`, `Environment`, `Layout`, `PatternToken`, `Program`, `tokenArity`, `validValue`, `distinct`, and `nestedPatternMatch` as the pure reference operand.
2. **Row-Local Frontier Transition Isolation (`Body 2`, `patternFrontier.qnt`)**:
   - `frontierInspect`, `frontierField`, and `frontierAdvance` read only `input.arms`, `input.valueRoot`, `input.patternRows`, `input.valueRows`, `input.patternFields`, and `input.valueFields`, with at most 1 pattern-row read (`+1` value-header read on `PatternConstructor`) or 1 field-slot pair (`patternFields.nth(...)`, `valueFields.nth(...)`) per transition, and contain zero fixture IDs or references to `decodedValues`, `encodedPatterns`, `fieldsOf`, or `nestedPatternMatch`.
   - `frontierProjection`, `frontierReference`, `frontierInputsValid`, `frontierVisitsOrdered`, `frontierPendingValid`, `frontierLocalReadBound`, `frontierCaseCorresponds`, and `frontierAllCuts` are cleanly separated outside `frontierAdvance`.
3. **Source-to-Table Coherence & 11 Tests (`Bodies 3–7`)**:
   - All 5 inputs (`frontierInput0`..`frontierInput4` / `sourceMatchId: 701..705`) in `patternFrontierInputs.qnt` match `SAFE-FRONTIER-INPUTS.compact.json` (`Body 6`) and `SAFE-SOURCE-TO-TABLE-ALL-ROWS.compact.json` (`Body 7`) as prepared by `prepare_safe_frontier_inputs.py` (`Body 5`).
   - `patternFrontierTest.qnt` (`Body 4`) defines the 5 correspondence `run` definitions, 5 33-cut deterministic replay `run` definitions (`analysisBounds.advances = 32` -> `33` prefixes `0..32`), and 1 `frontierDirectedCoverageTest` over `I::frontierCohort`.

Ready for Manual Part 2/2 (`Bodies 8–23`). No final verdict or dispatch eligibility is issued in Part 1/2.

