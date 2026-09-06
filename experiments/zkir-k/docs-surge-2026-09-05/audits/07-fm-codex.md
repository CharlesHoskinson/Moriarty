I could not write `experiments/zkir-k/docs-surge-2026-09-05/audits/07-fm-codex.md`: this session enforces read-only filesystem access. The report follows.

# Audit of 07-instruction-reference.md (formal methods expert)

Verdict: REVISE

Checks performed:

- Compared all 34 entries with the syntax, witness rules, operation dispatch, static checks, and constraint evaluators.
- Compared Rust instruction dispatch and chip initialization with the pinned ledger sources.
- Verified every named corpus example contains the stated operation.
- Confirmed the counts: 19 assertions across 13 ledger programs, 63 equality constraints across 24 programs, and private inputs in 16 programs.
- Compared cited divergence results with the recorded receipt.
- Ran the chapter’s corpus-search command successfully.
- Attempted the runner example using the manifest preimage through standard input; `uv` exited before execution because it could not create its cache lock on the read-only filesystem. Runtime results were not reproduced.
- Confirmed 34 entries, one top-level title, no em-dashes, and no prohibited drafting or placeholder terms.

Source paths below are repository-relative. `S/` denotes `experiments/zkir-k/semantics/`; chapter locations refer to `experiments/zkir-k/docs/07-instruction-reference.md`.

## Findings

### F1 blocking Machinery shared by every entry: missing-register outcomes

Claim: Chapter lines 29 and 42 say an absent register gives `unknown`.

Evidence: Repository observation: `S/zkir-constraints.k:160–162` makes the first non-`holds` outcome win. The width rule at lines 312–313 returns `synthErr` before inspecting registers. Thus a `lessThan` gate with width 253 and a missing operand returns `synthErr`, not `unknown`. Likewise, lines 280–282 detect inversion of zero before checking whether its output exists.

Fix: Replace the blanket statement with: “A missing register produces `unknown` when its check determines the outcome. Earlier width, chip, type, or value checks can instead determine `synthErr` or `violated`; `#and` preserves the first non-`holds` outcome.”

### F2 major Hashing: quoted synthesis messages omit prefixes

Claim: Chapter lines 258 and 265 present option/compress failures as `synthErr("synthesis: …")`.

Evidence: Repository observation: `S/zkir-constraints.k:474–475` produces decoder errors, which pass through `#std4` at line 488 and `#matches` at line 193. The latter prepends the operation label supplied at lines 444–445. The recorded `k07` result confirms `persistent_hash output: synthesis: …` in `evidence/zkir-k-divergence-tests-2026-09-05b.txt:55`.

Fix: Prefix both persistent-hash messages with `persistent_hash output: ` and both Keccak messages with `keccak256 output: `. Alternatively, label the existing text explicitly as the decoder error before wrapping.

### F3 major Summary: incorrect location of `alignedBytes`

Claim: Chapter lines 69–70 locate `alignedBytes` alongside the digest functions in `zkir-hash.k`.

Evidence: Repository observation: `alignedBytes` is declared and defined in `S/zkir-ops.k:236–240`. The digest functions are in `S/zkir-hash.k`.

Fix: Give separate locations: `alignedBytes` (`zkir-ops.k`) and `sha256Bytes` / `keccak256Bytes` (`zkir-hash.k`).

### F4 major reconstitute_field: cited cases do not establish a holding gate

Claim: Chapter line 237 associates the hypothetical modulo-wrapping gate behavior with divergence cases `f01` and `f10`.

Evidence: Repository observation: `evidence/zkir-k-divergence-tests-2026-09-05b.txt:22–24` records `f10` as a static width rejection with gate result `n/a`. Lines 60–62 record `f01` as `unknown` because `%o` is absent. The hypothetical relation follows from `S/zkir-constraints.k:350–353`, rather than a recorded holding verdict.

Fix: State that, with a supplied output equal to the reduced sum and satisfied operand bounds, the relation holds by inspection. State separately that `f01` records overflow rejection and an unknown gate; move `f10` to the excessive-width discussion.

### F5 major Corpus introduction: same-named extension fixtures differ

Claim: Chapter line 323 says same-named extension programs exercise the same instructions.

Evidence: Repository observation: `experiments/zkir-k/corpus/ledger9-92e8bdd3-tests/test_reverse_bytes_proof.zkir:16,21` uses `reverse_bytes`; the same-named file under `midnight-zkir-2ffe2d1-tests/:16,21` uses `reverse`.

Fix: Qualify the statement and name this exception, with a pointer to the extension chapter.

### F6 minor Entry ordering differs from the required syntax order

Claim: The chapter supplies instruction entries grouped by family.

Evidence: Repository observation: the brief at `experiments/zkir-k/docs-surge-2026-09-05/briefs/07.md:10` also requires syntax order. For example, `divModPowerOfTwo` precedes `add` in `S/zkir-syntax.k:104,110`, but their entries appear in the opposite order at chapter lines 202 and 226.

Fix: Arrange entries in syntax order, repeating family headings where needed.

## Coverage

- **Partly:** All 34 instructions have entries, constructors, operation descriptions, gate relations, applicable checks, and corpus references. Ordering and factual corrections remain.
- **Covered:** Entries use the requested families and a consistent structure with shared conventions.
- **Partly:** The summary table contains every required column; the `alignedBytes` source location is incorrect.
- **Covered:** Extension instructions receive a pointer to chapter 09 rather than separate entries.