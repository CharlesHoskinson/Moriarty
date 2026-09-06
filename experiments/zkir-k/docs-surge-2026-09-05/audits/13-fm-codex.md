Cannot save `experiments/zkir-k/docs-surge-2026-09-05/audits/13-fm-codex.md`: this session permits filesystem reads only. Report follows.

# Audit of 13-known-divergences.md (formal methods expert)

Verdict: REVISE

Checks performed:

- Compared described behavior with K rules and pinned Rust sources.
- Verified upstream dispositions against the recorded wiki findings.
- Confirmed all 20 corpus programs match the harness definitions.
- Compared expected outcomes with the receipt’s 20 PASS entries.
- Recomputed the Jubjub parity example, Curve25519 order-2 example, non-canonical decoding to 5, and padded bit widths.
- Confirmed one H1 title and no em-dashes.
- Did not rerun `divergence_tests.py`: lines 198–203 create directories and rewrite corpus files, contrary to this audit’s write restrictions.

## Findings

### F1 blocking “The one place where the circuit is weaker”

Claim: Chapter line 83 says `assert` is the only place where the circuit is weaker than `preprocess`.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-constraints.k:314` evaluates `lessThan` with padded bounds. With inputs 8 and 9, nominal width 3, and supplied output 1, that gate holds under the padded width 4. `experiments/zkir-k/semantics/zkir-ops.k:342` rejects 8 under the nominal width 3. The chapter itself describes this discrepancy at line 95.

Fix: Replace the exclusivity claim with: “For this input, the circuit’s non-zero constraint holds although `preprocess` rejects the non-boolean condition. Booleanity is a producer obligation.”

### F2 blocking “accept any pair of the same type”

Claim: Chapter line 87 says both `selectV` and `constrainEqV` accept any same-type pair.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-ops.k:132` requires both matching types and equal values for `constrainEqV` to return `cOk()`. Line 133 returns `cErr("Equality constraint failed")` for unequal same-type values. The pinned Rust implementation likewise rejects unequal values at `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir_instructions/constrain_eq.rs:51`.

Fix: State: “`selectV` supports any same-type pair. `constrainEqV` supports every same-type pair but succeeds only when the values are equal.”

### F3 blocking Incorrect oversized-limb threshold

Claim: Chapter line 107 describes the historical panic threshold as “192 bits or more.”

Evidence: Source fact: `wiki/zkir/zkir-v3-divergence-review.md:42` records the threshold as values **at least `2^192`**, which require at least 193 bits. A 192-bit value can be below that threshold. The decoder’s strict batch-bound comparison appears in `midnight-circuits-7.2.4/src/field/foreign/field_chip.rs:151–156` under the Cargo registry.

Fix: Replace with: “The panic on encoded batches with values at least `2^192` is fixed in midnight-circuits 7.2.4.”

### F4 major Placeholder presented as a K rule

Claim: Chapter line 22 presents `fsqrt(...)` within a fenced K rule.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-curves.k:150–151` contains the full two-argument square-root expression and a coordinate-range guard. The chapter’s fragment is neither verbatim nor a usable K rule. COMMON.md hard rules 1 and 4 require verbatim fragments and prohibit placeholders.

Fix: Quote the actual rule and its guard, or remove the code block and describe the formula in prose.

### F5 major All three maintainer notes are stale

Claim: Chapter lines 145–147 identify overlapping extension rules and two comments that incorrectly say “error.”

Evidence: Repository observations:

- `experiments/zkir-k/semantics/zkir-ext.k:181–184` now explicitly delegates mixed-length `test_eq` to the generic different-types rule. The two alleged overlapping rules are absent.
- `experiments/zkir-k/semantics/zkir-vm.k:450–452` already describes the event as `panic(...)`.
- `experiments/zkir-k/tools/divergence_tests.py:175` now says “K reports the same panic.”

Fix: Remove these resolved notes. Remove the section if no current maintainer issue remains.

### F6 blocking Missing concrete extension reproduction

Claim: Chapter line 75 reports a hand-checked extension example using `zkir_run.py ... --ext`.

Evidence: Repository observation: the paragraph supplies no complete program, raw preimage, executable command, or receipt locator for that particular run. `experiments/zkir-k/tools/divergence_tests.py:51–182` contains no extension case. The Rust arms at `repos/_build/midnight-zkir-2ffe2d1/zkir/src/ir_instructions/eq.rs:50` and `:107` support the general behavior, but do not substantiate the claimed experiment. The chapter brief requires a reproduction and expected result for each divergence.

Fix: Supply the complete program and preimage, root-relative K and oracle commands, and a preserved result locator. Until that evidence is available, describe the behavior as established by source inspection rather than a reproduced run.

## Coverage

- **Recorded divergences and one section per divergence: partly.** All named topics appear, but findings 6 and 7 share a section. Split them to meet the explicit section requirement.
- **Each side’s behavior, K rule, reproduction, expected result, security relevance, and recorded upstream status: partly.** Most items are present; the extension reproduction is incomplete, and the correctness issues above require repair.
- **Summary table: covered.**