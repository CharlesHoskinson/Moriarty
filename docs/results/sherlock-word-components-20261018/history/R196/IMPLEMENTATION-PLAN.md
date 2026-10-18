# Typed word arithmetic implementation plan

Status: SOURCE_ONLY / UNRUN. Scope is existing adopted revision03 UIREQ030, not a new language or backend design. The public declarations were frozen before this plan and before any test or implementation body.

Goal: implement named 32/64-bit host arithmetic components consumed by exact checked UInt128 entrypoints, retaining carry/borrow/full256 before error10 selection.

Architecture: u32 widens to u64, and u64 widens to u128. Addition/subtraction split exact totals into one word and a typed bit. Multiplication first exposes named low/high words; a whole-word multiply-accumulate carry consumes that product and addCarry. Fixed 4x32 or2x64 arrays implement UInt128 chains and schoolbook full multiplication. No big integers, recursion or allocations are needed by the component.

Choices considered within the approved semantics: replace the existing private byte auxiliary (rejected: changes an unrelated consumer); expose components without checked consumers (rejected: unused helper); add distinct typed host functions and exact checked consumers (selected: meets original UIREQ030 while preserving the byte auxiliary). No layout/circuit-cost choice is made.

Files owned here: afterimages/crates/mori-core/src/sherlock/word_arithmetic.rs; afterimages/crates/mori-core/src/sherlock/word_arithmetic_tests.rs; afterimages/docs/sherlock/typed-word-arithmetic.md; immutable declarations/contract, preimages and result metadata. Root alone changes W or Git.

- [x] Read installed moriarty-dev skill, W AGENTS/product contract, original030/interface equations, current reference error10 and private byte auxiliary. Run read-only status.
- [x] Confirm R196 unused; freeze declaration-only API and exact original normal input strings, then send metadata to Root and independent oracle owner.
- [x] Write ordinary test SOURCE first: named component results, distinct carry types, cross-limb checked normal arithmetic and complete product shape for both layouts. Use independent language arithmetic only as ordinary author assertions; never read withheld oracle values.
- [x] Implement exact arithmetic bodies with fixed arrays; every checked API consumes component functions and returns the retained full result before Ok/Err selection.
- [x] Inspect all indexing and intermediate bounds, scan public declarations against the frozen file, and freeze final SHA256 manifest. These are source observations only.
- [x] Supply registration/check recipe to Root. No candidate compilation/execution, SDK, provider, native or circuit operation occurs here.

Falsifiable predicate for Root's later selected checks: both layouts expose correct named component outputs and complete retained intermediates, and their checked UInt128 results match independent original030 normal outcomes. This does not close UIREQ030 circuit obligations or full Sherlock acceptance.

Testing limitation: tests are written first, but red/green execution is intentionally deferred to Root selection and fresh current review under the task assignment. No TDD execution claim is made.
