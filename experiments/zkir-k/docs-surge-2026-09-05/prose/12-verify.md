# Preservation check of 12-oracles-and-differential-testing.md

Verdict: PRESERVED

Inputs compared: the original at
`/tmp/claude-1000/-home-charl/7cf55fca-6b70-4fce-8aa0-2e793cb52ef2/scratchpad/prose/before/12-oracles-and-differential-testing.md`
and the edited chapter at
`/home/charl/Moriarty/.worktrees/zkir-k-semantics/experiments/zkir-k/docs/12-oracles-and-differential-testing.md`.
The unified diff touches 22 prose lines in 13 hunks. No code block, table row, link target, inline code span, number, receipt name or protocol step differs between the two versions.

## Meaning changes

None. Every hunk is a rewording with identical meaning. The two sentences added by inference were checked against the sources:

### Added sentence 1 (Results and reproduction): "The first three checks run with:"
Placed between the four-row receipt table and the three-command block (`unit_values.py`, `unit_hash.py`, `check_corpus.py`), which in the original stood after two blank lines with no introduction. Table rows 1 to 3 are the unit-values, unit-hash and corpus-check receipts; row 4 is the divergence receipt, whose command follows separately. The mapping holds:
- `tools/unit_values.py` line 227 prints `N/N checks passed`; `evidence/zkir-k-unit-values-2026-09-05c.txt` ends `43/43 checks passed`.
- `tools/unit_hash.py` line 135 prints the same form; `evidence/zkir-k-unit-hash-2026-09-05c.txt` ends `18/18 checks passed`.
- `tools/check_corpus.py` line 102 prints `N programs, N as expected, N unexpected, Ts`; `evidence/zkir-k-milestone2-corpus-check-2026-09-05c.txt` ends `63 programs, 63 as expected, 0 unexpected, 7.7s`.
- `tools/divergence_tests.py` line 248 prints `N/N divergence cases behave as expected`, matching row 4 and the separate reproduction block below it.
Judgement: correct; the sentence states what the receipts and tools show. Not a finding.

### Added sentence 2 (k-rust): "The tutorial run returns `Yellow` in 2.9 s, and the syntax probe returns a `Native` term in 3.5 s."
Replaces "The first probes return `Yellow` in 2.9 s and a `Native` term in 3.5 s." The preceding sentence (unchanged) names the four commands in block order as "the build, the tutorial run, the syntax probe, and the bounded compilation attempt". `evidence/k-rust-compatibility-2026-09-05.txt` item 1 is `krust krun toolchain-check/lesson-02-a.k ... -e 'colorOf(Banana())'`: exit 0 in 2.9 s, result Yellow. Item 2 is `krust kast semantics/zkir-syntax.k --module ZKIR-SYNTAX --sort IrType --expression 'native()'`: exit 0 in 3.5 s, result `` `Native`(.KList) ``. The attribution of each result to its command is correct, and the times are unchanged.
Judgement: correct. Not a finding.

### Other hunks checked against sources
- Step 3 (Seed inputs), "if the program's manifest entry has a nonempty `test_preimage.inputs`, those literal values seed the preimage, together with whichever of binding input, private transcript and public transcript outputs the entry carries": matches `tools/diff_test.py` lines 110 to 114 (`if seed_pre and seed_pre.get('inputs')`, then copy of `inputs`, `private_transcript`, `public_transcript_outputs`, `binding_input` when present) and lines 205 to 208 (entry matched by `file == path.name`). Identical meaning to the original, stated more precisely.
- Step 4 (Discover transcript needs), "an active transcript read in `#input` that runs past the end of its transcript records `needPubOut(T)` or `needPriv(T)` and temporarily binds a default": matches `semantics/zkir-vm.k` lines 436 to 438 and 446 to 448 (`<genMode> true`, `requires Idx +Int encodedLen(T) >Int size(...)`, `M[O <- defaultValue(T)]`, `needs` extended). Identical meaning to "missing active transcript reads ... temporarily bind defaults".
- k-rust paragraph, remaining claims: exit 124 after 25 minutes with no KORE emitted (receipt item 4) and no result within ten minutes for the full-definition execution attempt (receipt item 3) are unchanged.
- Results table and summary prose: 358/358 (46, 309, 3, 387 s) and 418/418 (50, 364, 4, 281 s) match the last lines of `evidence/zkir-k-differential-92e8bdd3-2026-09-05c.txt` and `evidence/zkir-k-differential-ext-2ffe2d1-2026-09-05c.txt`; both `oracle 2:` lines report 0. Unchanged.
- Haskell backend paragraphs: the sentence "Omitting the explicit cells leaves decoding stuck" moved from the end of the paragraph to follow the `#Top` sentence; the "therefore" still follows "The claim module is not in the repository". No claim added, dropped or altered.

## Dropped material

- "record these experiment observations" became "record the following". Harmless cut.

No other sentence, claim or list item is absent from the edited chapter.

## Rule violations

None.
- No em or en dash in the edited chapter.
- Exactly one `#` title.
- No placeholder or TODO.
- No process words. The only match for "review" is the path `experiments/zkir-k/review-2026-09-05/CONSOLIDATED.md`, present in both versions and a real repository path, not process prose.
- All five code blocks and all five tables are byte-identical between versions; every link target and every inline code span is unchanged.

## Residual machine tells

None that rise above the original. Two pre-existing sentences carry a three-item cadence that the edit kept: "it is informational, excludes perturbations, and does not affect the exit code" and "does not invoke `circuit`, key generation, or proof verification". Both are accurate enumerations of three facts; leaving them is acceptable.
