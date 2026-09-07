# A4 tasks

Whole-phase implementation and acceptance tasks remain open in this contract.
Narrower admitted implementation units are identified in the refinements below.
Existing work must receive evidence intake, not duplicate implementation.

- [ ] Verify the exact dependencies and source pins.
- [ ] Read XML phase A4 and its named scenario inventory.
- [ ] Complete its concrete implementation plan before behavioral edits.
- [ ] Add the specified behavioral tests before implementation.
- [ ] Retain compiling RED sources and terminal receipts.
- [ ] Produce the exact outputs listed in README.md.
- [ ] Execute each positive and failure scenario.
- [ ] Preserve complete source closures and raw results.
- [ ] Write `evidence/s02-candidate-a-completion/a4/manifest.json`.
- [ ] Recompute `evidence/s02-candidate-a-completion/a4/validation.json` from evidence.
- [ ] Obtain the required independent review.
- [ ] Commit only the admitted package files.

## Native case transport refinement

Experimental plan5829639 adopts CT-001–CT-010 in the case-sharded-transport
specification. It preserves all78cases/1557events and fixes a measured local
tool string-size incompatibility. The following remain implementation gates:

- [x] Implement and independently review strict bounded JSON transport (42 tests; `62b7b30` and `fdb81c7`).
- [x] Implement all78 literal native wrappers and exact global ordinal controls (`8be3cef`, complete aggregate typecheck, 12 native tests and root original evidence intake).
- [x] Admit bounded recorder artifact hashing (`b08a2da`, original guarded-read failure and 15 passing controls).
- [x] Admit structural sharded-exporter source (`9a263d7`, original omission/reorder failures and 55 non-package controls; four actual-package tests remain pending).
- [x] Admit measured native-invocation source (`1df388a`, 41 short-process controls and independent original-byte intake; no Candidate A resource-feasibility claim).
- [ ] Retain largest-case native pilot bytes, full terminal witness and peak RSS.
- [ ] Export all78 native cases with original source/runtime/terminal receipts.
- [ ] Stage case bytes, obtain independent root case-pin admission, then seal.
- [ ] Run complete streaming semantic/provenance admission and every mutant triple.

## Sharded adversarial refinement

T6S001–007 are specified in sharded-adversarial-tests. Plan `cd06756` preserves
all required control families. Sharded checker `26c9cc5` passes its focused
79-test checker/utility run; it is not complete package or Task6 acceptance.

- [ ] Retain the substitution behavioral RED and restore the exact checker source.
- [ ] Pass every synthetic semantic, locator, lexical and path control with original artifacts.
- [ ] Admit the actual native package against the final test source pins.
- [ ] Pass all seven package controls and eight complete honest traversals.
- [ ] Run the final suite and direct CLI with no exclusions.

## Specification structure check command

```bash
openspec validate s02-candidate-a-a4 --strict --no-interactive
```

This command validates specification structure only.
Use the XML commands for existing boundary runtime checks.
Derive exact new runtime commands in the phase implementation plan.
Do not mark runtime tasks complete from this command.
