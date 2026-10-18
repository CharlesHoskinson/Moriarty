# SOURCE900 mechanical source fidelity

This is a mechanical audit of the complete public SOURCE900 opinion, not a fresh source review, corrected opinion, adoption vote, security judgment or execution permission. The original terminal object is copied unchanged as `SOURCE900-TERMINAL-REVIEW.json`. RESULT900 was not inspected or used to establish these findings.

The whole is 49,460 bytes, SHA256 `540994d00294bc8f2b179b940af58c1e3b49443f3047148a8889c65dd1e26592`. Its subject, candidate f91f2c7/tree ed832/base71c and 1,001,567-byte PRIMARY binding match. All 18 candidate coverage hashes and literal anchors match. All 151 declared section IDs match the index in order; the 62 context IDs match. These facts do not attest actual model reads, comprehension, file-tool use, read-once behavior or backend identity. `actualModel` is a declaration, not independent attestation.

Material source mismatches are present independently of any RESULT opinion:

| Opinion predicate / claim | Actual pinned source |
| --- | --- |
| public-financial-harness: quoted `run_check`, `json_args`, `quiet_args`, `exit_code` and last-argument validation | C17 has none. `Case` has `argv` and `human_argv` (15,18); `check_case` resolves fixtures inline (47), copies into `CaseDirectory`, and calls `Command::new(CARGO_BIN_EXE_mori)` directly twice (57–79). Its actual whole-output/status assertions remain present. |
| guidelines-and-effects: quoted `fixture_root()` at C17 26–28 | No such function. These lines obtain a time nonce. Fixture lookup is inside `check_case` at47. |
| core-financial-reports: externally tagged `Transfer`/`Discharge` and `Await` objects; singular `binder` | C11 has flat `kind` fields (30,35); C13 has `binders` array (28), `guard` (34), `kind: Await` (35), `label` (36). Public IR declares `serde(tag = "kind")` and `binders: Vec<InputDecl>`. The alleged exact JSON is a different wire shape. |
| beta-expected-reports: `CheckReport::from_checked` | X052 defines `CheckReport::new` (85) and private `fill` (114); no `from_checked` occurs. X046 calls `CheckReport::new` at59. |
| cli-profiles: quoted two individual header assertions | C15 292–294 uses a header loop; the exclusion assertion follows at295–298. C16 exact quote also drops the actual `pub` at19. Help strings themselves remain represented in actual code. |
| original-baseline: `fn unicode_byte_offsets()` | C17 129 is `fn beta_unicode_byte_offsets()`. |
| acceptance-boundary / byte-oracle-exposure / guide-consumers: alleged exact C01/C18 prose | The supplied quoted passages do not occur in those bodies. Actual C01 11–20 discloses exposed author expectations/order history;22–27 bounds check serialization. Actual C18 section begins20, compilation paragraph119–125 differs;129 opens native examples containing compile at134–135. Related qualification concepts do not make invented text literal evidence. |

`QUOTE-CHECKS.json` records all38 quote/anchor objects with exact opinion text, claimed line ranges, actual lines and classifications. All18 coverage anchors match. Of20 predicate quotations,3 match exact lines,3 are authentic at shifted lines (Beta action blocks110–125/98–113; parser397–404),1 is an authentic guideline prefix with an unmarked final-line continuation, and13 contain altered or absent text. This distinguishes offset errors and harmless prefix truncation from invented APIs or wrong wire structure.

Preservation facts were checked separately rather than inferred from the opinion:

- All85 Git comparisons match:18 candidate,5 complete before-context bodies and62 context bodies, including the explicitly bounded runtime excerpt. All18 candidate bodies also match their recorded historical body commits. All213 files listed in the original packet manifest retain their recorded hashes.
- The original8 manifest row JSON values are identical. Raw line169 changes `}` to `},` to append cases, so the opinion's byte-identical lines1–169 wording is inaccurate. All16 original fixtures match base, head and packet bytes.
- The complete original131-line harness is an exact byte prefix. Six financial wrappers are appended. The old author-candidate trailing-newline qualification H03 is a different historical comparison.
- All12 financial source/expected pins match H01. Literal expected Beta reports have8/7 declarations, the stated actions/intents/support and empty diagnostics; name-span byte slices match source bytes. Core fixtures retain the declared actions, root ExprId numbers and stage labels. No parser/checker/evaluator execution was performed for this audit.
- “Preserving all5modifiedbasebodies” in the summary must not mean unchanged candidate bodies. All5 are modified; the detailed exact-integration reason correctly says their full before bodies are supplied. C18's unchanged suffix begins candidate128/base17 (552lines). C16's doc comment is unchanged; only long_help changes there.
- Actual R12 records6passed/8filtered/0.03s. This source task makes no current passing claim for the original8, no walkthrough execution claim from `bash -n`, and no independent financial-oracle claim.

`NARRATIVE-CHECKS.json` covers all10 source predicates and every per-path coverage assessment materially. `SOURCE-FACT-CHECKS.json` contains byte/source comparisons and literal JSON metadata. `HISTORICAL-CHECKS.json` contains historical/H01 comparisons. Eleven complete directly contradicted public source bodies are preserved with provenance for convenient inspection; all other pinned originals remain accessible in the unchanged packet.

No original opinion has been repaired, spliced, erased or reclassified as a new vote. Root determines review eligibility and adoption. Parent roadmap/financial/compiler/composition/proof/ledger exits and all old failed reviews/caps remain unchanged. No provider/private/auth/cache input, product/native/SDK execution, shared source edit or Git mutation occurred.
