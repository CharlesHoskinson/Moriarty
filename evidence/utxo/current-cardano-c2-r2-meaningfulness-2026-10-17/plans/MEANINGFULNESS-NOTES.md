# C2r2 meaningfulness controls (S33:182-188): author notes

Author: claude-opus-5-5, high effort. Tools: Read, Write and Edit only.

I did not run, import, compile or hash anything. I made no web, shell or Git access, and wrote nothing except the three files in r2-meaningfulness-source-output. Every runtime statement below is a prediction made by reading source.

This is author source, not a vote. It does not claim sourceReady or execution approval. Before any run, two fresh non-author Opus 5.5 reviews of the materialized whole mutants, the plan and the exact envelope must agree, and then root decides.

## Sources read

**Packet bookkeeping.** MANIFEST.json and SEGMENT-INDEX.json.

**Originals, all 10 segments in order.** Every line of every original was read exactly once:
- the candidate, lines 1-508, and IMPLEMENTATION-NOTES, 1-118;
- the test, 1-1102;
- S33, 1-213, and S34, 1-36;
- S11 (v1), 1-357, S22 (host), 1-656, and S23, 1-420;
- the fixtures S15 (K2), 1-44, and S16 (K6), 1-64;
- S51;
- PREVIOUS-TEST-SOURCE-REVIEW;
- the ROOT-RED decision, result and stderr;
- PROPOSED-ORDINARY-GREEN-ENVELOPE;
- the DOWNSTREAM-R4 contract and its conditional adoption.

**Read directly.**
- ACTUAL-GREEN-DECISION, -RESULT, -stderr, -stdout (empty), -AUDIT and -AUDIT-PROCESS;
- ROOT-GREEN-ACCEPTANCE;
- ROOT-STDERR-PREDICTION-CORRECTION.

**Not opened.** PRECEDING-SOURCE-AUDIT(-PROCESS), ROOT-RED-stdout, the raw original files (read only through the segments), and every originalPublicSource or provenance path.

## Runner facts the predictions depend on

**No r2 pin.**
- setUpModule (test:142-158) and T00 (test:705-714) pin only host, v1, K6 and K2.
- `_prepare` (test:173-184) reads the r2 file with a 131072-byte cap and prints its SHA-256. It never compares that SHA to anything.
- So replacing the r2 bytes cannot trip a provenance or pin check. Every kill must come from behaviour.
- The only text checks on r2 are content checks: T01's import-line scan (test:761-764) and T15's EFFECT_TOKENS scan (test:1023-1026).
  - M02-M04 only delete text, and I checked each join boundary for new tokens.
  - M01 has the same two import lines as r2, and T15 stops at test:992 before its token scan.
- T01's "not the v1 module" check (test:784) compares object identity, not bytes, so M01's fresh module passes it.

**SHA line.**
- T00 never loads r2, so T01 is the first loader.
- The FIX['prepare'] cache and the non-printing `_module` re-execs (T01 at test:800, T15 at test:963) mean the SHA substring is printed exactly once, inline after the T01 description.
- This matches the accepted GREEN and the root correction: count occurrences of the substring, not standalone lines.

**Failure counting.**
- `check` (test:685-701) wraps each case in `subTest(label, case=name)`. A failure inside one subTest is recorded and the method continues.
- unittest counts each failing subTest as one failure, so the summary reads `FAILED (failures=N)` with N equal to the number of failing subTests plus method-level failures.
- `refused` (test:660-669) catches only ValueError:
  - if r2 returns a result, the case fails with `accepted; expected C2I-X`;
  - if r2 raises with another stage prefix, the `assertTrue` at test:667 fails.
- Both outcomes are AssertionError, so every predicted kill is a FAIL. None is an ERROR.

**T16 reuses cases.** T16 runs every T05b case plus some T04/T06/T14 cases (test:638-639). Any mutant that weakens one of those cases therefore also fails the matching T16 subTest. I predict these failures explicitly rather than treating them as surprises.

## Mutants (exact literals are in MUTATION-PLAN.json)

| Id | Change | Delta | Designated kill (S33) | Predicted failures |
|---|---|---|---|---|
| M01 | v1 bytes 23c3f99d… copied unchanged as c2_issue_artifacts_r2.py | 15231 B | T02 (12), T04 driver one-char and provingKeyBytes 2**64, all 57 T05b | 185: T02 12, T04 2, T05 92, T06 10, T14 2, T15 1, T16 65, T17 1 |
| M02 | `_is_text`: delete lines 264-268 (isascii fast path and ord scan); exact-str check kept | -156 B | T06 relationFormat, logicalCostProfile, fieldModulusLeHex, export action alone, intentAnchor alone | 13: T06 9, T16 4 |
| M03 | delete lines 382-385 (PROFILE BLAKE2b(shape bytes) vs abi2ShapeDigest) | -255 B | T05b abi2ShapeBytesHex one byte flipped | 3: T05b 1, T05c 1, T16 1 |
| M04 | `_is_decimal`: delete lines 278-280 (ASCII 0-9 loop); length, leading-zero and bound kept | -83 B | T05b logicalWork '+1', '-1', Arabic-Indic two-eight, '1_0', ' 28', '28' plus LF | 12: T05b 6, T16 6 |

All four predictions also share these properties:
- exit 1;
- 18 tests run;
- 0 errors;
- stdout empty;
- T00 and T01 pass.

### M02 (CP04 scan removed)

**Case outcomes.**
- **Admitted outright:** the relationFormat, logicalCostProfile and fieldModulusLeHex cases. Nothing after PROFILE reads those fields.
- **Refused late, at RELATION:** relation action alone and export action alone. The action strings differ, so the RELATION identity check refuses them.
- **Refused late, at CARDANO:** catalogue name, intentTargetHashV2 and intentAnchor alone. The derived section differs, so CARDANO refuses them.
- **Fully admitted:** the "consistent action" case. Relation action, export action and the derived catalogue name are equal, and BUILD never uses the action.

**Must still pass.**
- The REDUNDANT extra-key case, in both T06 and T16. The closed-key check (candidate:315-317) still refuses it, so if it fails, that is STOP rather than a kill.
- statementHex, proofHex and x1. They are deferred kinds in both versions.

**Output safety.** No message or subTest name contains U+D800, so writing stderr cannot raise.

### M03 (shape-digest comparison removed)

**Designated kill.** The one-byte flip changes the K2 shape bytes' leading `4d` to `4c`. The value keeps its length and stays lowercase hex, so only the removed comparison could refuse it, and no later stage reads the shape bytes.

**Additional failures.**
- T05c (setup digest set to 00x32) now gets past PROFILE and is refused at RELATION, through the identity check on abi2ShapeDigest.
- T16 repeats the flip case.

**Untouched checks.** The shape-hex syntax checks, the RELATION identity, the STATEMENT header shapeDigest and CARDANO all remain.

### M04 (int() without the ASCII check)

**Not masked; no STOP or dissent.**
- The checks before the removed loop (exact str, length 1..39) let the six discriminating values through.
- The retained checks after it (no leading zero, at most 2**128-1) do not reject them.
- `int()` parses each of them without raising. Under CPython it accepts sign, surrounding whitespace, underscores and Unicode Nd digits:

  | Input | int() result |
  |---|---|
  | '+1' | 1 |
  | '-1' | -1 |
  | Arabic-Indic two-eight | 28 |
  | '1_0' | 10 |
  | ' 28' | 28 |
  | '28\n' | 28 |

  So the mutant admits them, and admission is the kill.
- The other six logicalWork cases are refused in both versions by retained checks, so they are not discriminators: '' and '40 digits' and '5000 digits' (length), '01' (leading zero), '2**128 as decimal' (bound) and int 28 (type).

**Untested weakening.** The mutant would also let `int()` raise an uncoded ValueError on non-numeric text. No frozen case reaches that path, and I do not claim it as a kill.

### M01 (v1 under the r2 name): why the extra failures are expected

S33:185 designates T02, T04 and T05b, and expects T05a. The other failures follow from v1's frozen semantics. They are not looseness in the criteria:
- **T05a (34):** v1 reports relation faults as C2I-SETUP, but r2 expects C2I-PROFILE.
- **T05c (1):** v1 has no PROFILE shape-digest check, so the case reaches RELATION.
- **T06 (10):** v1 has no CP04 scan and an open relation schema, and it owns export str faults at RELATION or CARDANO rather than EXPORT. So even the REDUNDANT case fails here.
- **T14 (2):**
  - SETUP/PROFILE: v1's driver is free text and it has no language check, so v1 admits.
  - PROFILE/EXPORT: v1 refuses at EXPORT instead.
- **T15 (1):** test:992 compares v1's 8 labels with the 16 r2 labels, at method level.
- **T16 (65):** by definition, every T16 case is one that v1 admits.
- **T17 (1):** format /1 instead of /2.

**What passes.**
- T01 passes. v1 reaches all 7 APIs through the proxy because it reads module attributes at call time, even though it passes them to `_host` as values. This is concrete evidence that T01 cannot discriminate CP01/L01 aliasing or value-passing.
- T03 and T07-T13 pass, because they are "=" rows where v1's stage equals r2's.

## Acceptance (exact; see MUTATION-PLAN acceptancePerMutant)

**MUTANT_KILLED_AS_PREDICTED** requires all of A01-A11:
- one attempt, exit status 1, and empty stdout;
- "Ran 18 tests" and the exact predicted summary line;
- no ERROR;
- the set of FAIL header identities equals exactly the designated plus predicted set;
- every must-not-fail case and every predicted-pass method is ok;
- the root-computed mutant SHA appears exactly once;
- no missing-r2 phrase;
- the six files and their modes are unchanged.

If the designated kills are present but other outcomes differ from the prediction, the status is KILL_OBSERVED_PREDICTION_DISCREPANCY: a STOP for review, never accepted, with no rerun.

Any of the following is STOP, and root halts the remaining runs:
- a designated kill is missing;
- any ERROR;
- a run count other than 18;
- a SHA or startup anomaly;
- a pin or mode change;
- a timeout or signal;
- a materialization occurrence count other than 1.

I set no "any failure" criterion.

## Source-only residues (unchanged even if every mutant is killed)

These remain exact-byte source-audit discharges of 67fb3b3e… and nothing more:
- CP04 never calls `str.encode`, makes no input-proportional copy, and checks length before scanning;
- no host alias and no host patching (L01, L02);
- no hidden, cached or non-container state (L03, I06).

None of the four mutants targets these properties, and no test outcome observes them. The M01 T01 pass shows this directly for aliasing. Root should not cite meaningfulness results as evidence for any of them.

## Non-claims

The mutant results would show only that specific frozen methods and subTests discriminate four weakenings. The following are not claimed:
- Rust acceptance, or any subset, equivalence or inclusion relation;
- Groth16 pairing, or VK, curve, circuit or key correspondence (circuitVkCorrespondence stays OPEN);
- ledger, context authentication, ExUnits, fees, deployment, Preview or Preprod;
- CP08 reliance, builder reliance, an R4 run, or native authority;
- closure of any of the 20 UTxO families or of the financial or PCD tracks.

K6 remains a captured public proof whose context is not authenticated. T17 remains SYNTHETIC_NOT_PROOF_NOT_ORACLE.

## Limitations

**Unverified assumptions.**
- The Python version is not stated in the packet. The verbosity-2 subTest rendering, the `[label] (case=repr)` header form and the subTest failure counting are CPython 3.11+ behaviour from knowledge, consistent with the RED and GREEN transcripts, which show 3.11+ style descriptions and carets. If header text differs only in format, the result is KILL_OBSERVED_PREDICTION_DISCREPANCY and goes to review. That is deliberately strict.
- The literal before-strings assume 4-space indentation, LF line endings and no trailing whitespace. I saw only the segments, not the raw bytes. The occurrence-count-1 rule turns any mismatch into STOP before execution.

**Root-supplied inputs.**
- Byte counts of the trusted files are root pins.
- Mutant byte counts are arithmetic over the literals.
- Mutant SHA-256 values for M02-M04 come from root after materialization.

**Independence.** The author and the reviewers share a provider: their contexts are fresh, but there is no provider diversity.
