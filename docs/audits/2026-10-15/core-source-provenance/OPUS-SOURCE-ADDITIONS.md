I've written the assembly-integrity validator and its unit tests into `relation/mod.rs`, but none of it has been compiled, formatted or run. No cargo or rustfmt command was run; only `git diff --check` passed on the file. This gives no security approval, no GO and no command classification. Native authority is still zero (gross 4334 / remaining 0), and nothing is staged or committed.

**What changed (only `crates/mori-ledger/src/relation/mod.rs`)**
- **Validator:** `validate_statement_emissions` is called once, in the mapped-role `compile_source_static_roles`, before the manifest is built. Static reopen recompiles through the same function, so the checks run on both export and reopen. It refuses with `native-relation` / `source-static-emission`.
- **Checks from the design:** one entry per statement, contiguous ranges, offsets strictly ascending, and each offset below its group's `field_count`.
- **Extra checks I added:**
  - statement index and root expression must match the source statement;
  - the IR must end with one `Output`, group *k* must be the *k*-th `Impact`, and each `Impact`'s width must equal its group's `field_count`;
  - `abi-root` groups must form exactly the leading prologue, and the first entry must start right after the last of them;
  - group ranges must match the `Impact` positions inside each instruction range;
  - every `Variable` input to an `Impact` must appear in the map.
- **Unchanged:** existing manifest fields, IR, `Group`, the ordinary and signed paths, `Builder::op` refusals and all size and work limits. No new limit was added and the new types still have no `Deserialize`.
- **Tests:** `relation::emission_tests` runs the real validator on hand-built data with no compiler, SDK relation check, VM or proof. It has four tests that should pass and 22 single changes that should each be refused. I traced each one by hand only.

**Corrections to my earlier design (the original OPUS-DESIGN.md is unchanged)**
- **Withdrawn:** "a listed `Immediate` only occurs where the statement's root is a literal" was wrong. A literal-amount transfer produces `Variable` replacements, and `let x = 5; next s = x` produces an `Immediate` one. The correct rule is that every `Variable` must be listed, and listed `Immediate`s are allowed.
- **Qualified:** Let and Requires statements can emit groups when they read state. The oracle's step 2 Let has three.
- **Superseded:** my illustrative test fixtures are replaced by the oracle's five-step source.
- **Still unobserved:** that IR bytes stay unchanged, because no baseline capture exists.
- **New compatibility point:** the test code sits inside `mod.rs`, which is hashed into both the ordinary `compiler_hash` and `source_closure()`. Any later test-only edit there changes ordinary manifest bytes and static bundle build identities. Moving the tests to their own file would avoid this, but that needs a path outside my write scope.

**Still open**
- **H2–H5** were not implemented. H4 matters most: the validator only checks the compiler's own output. Rejecting a tampered map depends entirely on reopen's fresh-recompile equality check, and H4 is the test for that.
- Testing the validator against real faulty compiler output would need a test hook in `compiler.rs` or `arithmetic.rs`, which is outside my scope.
- Classifying the commands for these unit tests and for Sol's integration test is still pending.

**Changes I didn't make:** `crates/mori/tests/mapped_role_static_io.rs` now differs from the hash in Sol's plumbing record. `docs/NATIVE-CORE.md` also has a new uncommitted section about the map. Both were written after that record, and I haven't attributed or reviewed them.

The full report, with the exact code, the checks table, test expectations and remaining limitations, is in `R/OPUS-SOURCE-ADDITIONS-DETAIL.md`.
