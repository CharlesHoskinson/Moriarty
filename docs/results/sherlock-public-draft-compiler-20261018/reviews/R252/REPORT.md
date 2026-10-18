{
  "coverage": {
    "inline_complete": true,
    "same_owner_prior_context": true,
    "gaps": [
      "Historical provenance arrays and execution-time pin arrays from R221/R239/R235 remain factored SHA-256 facts rather than freshly re-audited files.",
      "Public negative/hostile compile_draft_host refusal tests and CLI subprocess invocations (--source-profile, --entry) remain unexecuted.",
      "UIREQ-026 privately retained Fenrir Clean integration pin/evidence record remains outside public scope and unverified by this audit."
    ]
  },
  "literal_corrections": {
    "actual26IDs": [
      "draft-to-b.program/1",
      "draft-to-b.function/1",
      "draft-to-b.literal/1",
      "draft-to-b.var/1",
      "draft-to-b.let/1",
      "draft-to-b.if/1",
      "draft-to-b.call/1",
      "draft-to-b.match/1",
      "draft-to-b.binary-add/1",
      "draft-to-b.binary-sub/1",
      "draft-to-b.binary-mul/1",
      "draft-to-b.binary-eq/1",
      "draft-to-b.binary-lt/1",
      "draft-to-b.binary-cons/1",
      "draft-to-b.binary-branch/1",
      "draft-to-b.unary-leaf/1",
      "draft-to-b.unary-not/1",
      "draft-to-b.pattern-var/1",
      "draft-to-b.pattern-nil/1",
      "draft-to-b.pattern-cons/1",
      "draft-to-b.pattern-leaf/1",
      "draft-to-b.pattern-branch/1",
      "draft-to-b.sort-uint/1",
      "draft-to-b.sort-bool/1",
      "draft-to-b.sort-list/1",
      "draft-to-b.sort-tree/1"
    ],
    "actual_taxonomy": {
      "total_obligations": 26,
      "all_statuses": "named-unproved",
      "construct_mappings_22": {
        "program_and_function_2": [
          "draft-to-b.program/1 (Program)",
          "draft-to-b.function/1 (Function)"
        ],
        "core_expressions_6": [
          "draft-to-b.literal/1 (Literal)",
          "draft-to-b.var/1 (Var)",
          "draft-to-b.let/1 (Let)",
          "draft-to-b.if/1 (If)",
          "draft-to-b.call/1 (Call)",
          "draft-to-b.match/1 (Match)"
        ],
        "binary_operators_7": [
          "draft-to-b.binary-add/1 (Binary.Add)",
          "draft-to-b.binary-sub/1 (Binary.Sub)",
          "draft-to-b.binary-mul/1 (Binary.Mul)",
          "draft-to-b.binary-eq/1 (Binary.Eq)",
          "draft-to-b.binary-lt/1 (Binary.Lt)",
          "draft-to-b.binary-cons/1 (Binary.Cons)",
          "draft-to-b.binary-branch/1 (Binary.Branch)"
        ],
        "unary_operators_2": [
          "draft-to-b.unary-leaf/1 (Unary.Leaf)",
          "draft-to-b.unary-not/1 (Unary.Not)"
        ],
        "patterns_5": [
          "draft-to-b.pattern-var/1 (Pattern.Var)",
          "draft-to-b.pattern-nil/1 (Pattern.Nil)",
          "draft-to-b.pattern-cons/1 (Pattern.Cons)",
          "draft-to-b.pattern-leaf/1 (Pattern.Leaf)",
          "draft-to-b.pattern-branch/1 (Pattern.Branch)"
        ]
      },
      "dedicated_sorts_4": [
        "draft-to-b.sort-uint/1 (Sort.UInt)",
        "draft-to-b.sort-bool/1 (Sort.Bool)",
        "draft-to-b.sort-list/1 (Sort.List)",
        "draft-to-b.sort-tree/1 (Sort.Tree)"
      ],
      "per_row_fields": [
        "id",
        "construct",
        "target",
        "returned_values",
        "defined_failures",
        "order",
        "resource_obligation (all 'draft-to-b.resource-observations/1')",
        "effects_and_duties (all 'not-applicable: pure DedicatedSorts with FinanceInput::Absent')",
        "status (all 'named-unproved')"
      ],
      "top_level_companion_sections": {
        "resources": "id='draft-to-b.resource-observations/1' (a_clock='sherlock-k-a/1...', b_clock='sherlock-helper-microstep/1...', helper_mapping, same_fuel='not-claimed...', failure_classes, required_comparison, unmeasured)",
        "finance": "source_profile='pure-dedicated-sorts', target_input='FinanceInput::Absent', effects='not-applicable in this grammar/profile; no transfers/fees are represented', residual_duties='not-applicable in this grammar/profile; no liabilities/wait/resume are represented'",
        "proof": "semantic_correspondence='unproved', runtime_values_failures_order_resources='unobserved-by-compile-only-preparation', witness_plan='missing', constraint_plan='missing', native_export='unsupported', native_proof_target='unrun', gap='No source-to-B semantic proof or B-to-Clean/handwritten circuit correspondence/native exporter is supplied; host evaluation cannot supply missing circuit evidence'"
      }
    },
    "unsupported_R245_examples_withdrawn": [
      "Withdrawn R245 taxonomy description '22 construct/value/order/failure/resource rows and 4 applicability/effect/duty rows': replaced by the actual 22 construct rows + 4 DedicatedSorts rows (Sort.UInt, Sort.Bool, Sort.List, Sort.Tree), with resource/finance/proof captured in top-level sections and per-row resource_obligation/effects_and_duties fields.",
      "Withdrawn R245 non-existent obligation IDs and construct names in 022-P03: 'function-reference', 'sequence', 'arith-overflow-and-shift-failure', 'match-scrutinee-order-and-exhaustiveness', 'defined-failure-vs-refusal-boundary'.",
      "Withdrawn R245 non-existent obligation IDs in 022-P04: 'step-fuel-accounting', 'heap-allocation-accounting', 'observation-digest-equivalence' (actual shared resource obligation ID is 'draft-to-b.resource-observations/1').",
      "Withdrawn R245 non-existent obligation IDs in 022-P05: 'effect-free-core-computation', 'host-only-execution-boundary', 'financial-domain-absence', 'dedicated-sort-checking-applicability'.",
      "Withdrawn R245 paraphrased proof/evidence strings in 022-P08 ('capability_Decision', 'semantic_equivalence=unproved-draft-lowering-v1', 'client_authenticity=unauthenticated-local-artifact') in favor of the literal BODY 01 proof fields and DraftCompileArtifact evidence fields.",
      "Withdrawn R245 compressed 'Owner Gap O1..O4' prose in favor of the exact five R238 owner gaps GAP-1..GAP-5 verbatim from BODY 02."
    ],
    "corrected_statement": "In CORRESPONDENCE-EXPECTED.json (BODY 01, 20139B, sha256=35284d3bcd2e55eb8f0b0792a0a9cfda51190b03a75e37c523a9f11e54c311e7), all 26 entries in obligations[] are named-unproved rows spanning 22 language constructs (Program, Function, Literal, Var, Let, If, Call, Match, Binary.{Add,Sub,Mul,Eq,Lt,Cons,Branch}, Unary.{Leaf,Not}, Pattern.{Var,Nil,Cons,Leaf,Branch}) and 4 dedicated sorts (Sort.{UInt,Bool,List,Tree}). Each row explicitly specifies target, returned_values, defined_failures (e.g., runtime codes 3, 5, 6, 7, 8, 9, 10, 11 vs static parse/type/copy/Tooling diagnostics), order, resource_obligation='draft-to-b.resource-observations/1', effects_and_duties='not-applicable: pure DedicatedSorts with FinanceInput::Absent', and status='named-unproved', complemented by top-level resources, finance, and proof objects."
  },
  "exact_five_retained_owner_gaps": [
    "GAP-1 (B iterative Drop allocations): `impl Drop for CoreProgram`, `CoreExpr`, and `CoreTree` in `owner_prepare` (F031 `031-01.txt`, anchor `fn drain_worklists(ew: &mut Vec<Box<CoreExpr>>, tw: &mut Vec<Box<CoreTree>>)`) avoid call-stack recursion but allocate transient heap worklist entries and `Box::new(dummy_expr())` / `Box::new(dummy_tree())` replacement boxes",
    "GAP-2 (A recursive Arc Drop): Input `a::Program`, `a::ExprRef` (`Arc<Expr>`), and `a::Tree` (`Arc<Tree>`) in `reference_types.rs` (F030 `030-01.txt`, anchor `pub type ExprRef = Arc<Expr>;`) use standard compiler-generated recursive `Drop`",
    "GAP-3 (Owner prepare/admit/admit_run allocations): `owner_prepare::{prepare_core,check_program_view,admit,admit_run}` (F032 `032-01.txt`, anchor `let view = prepare_core(p, b, l)?;`) perform internal host table/diagnostic allocations not governed by `SourceCopyBudget`",
    "GAP-4 (Report/provenance/diagnostic/serialization allocations): `SourceProvenance` collection (F019 `019-02.txt`, anchor `fn collect_sites(`), `DraftCompileArtifact` projection (F009 `009-01.txt`, anchor `pub(crate) fn project_admitted_preparation(`), `OwnerWire` JSON construction, and `serde_json` serialization allocate unmetered host memory",
    "GAP-5 (Public route / global H1 resource guarantees): Bounded parser/checker/copy budgets enforce structural preprocessing counts only and do not establish process-wide host heap, native stack-byte, or wall-clock time admission (H1)"
  ],
  "original022": {
    "recommendation_after_correction": "SURVIVES_RECOMMEND_SCOPED_ORIGINAL_UIREQ022_CLOSURE_FOR_ROOT_ADOPTION",
    "scope": "Scoped original UIREQ-022 closure recommendation survives the literal prose corrections because the R239 tests executed against the actual source and actual CORRESPONDENCE-EXPECTED.json (BODY 01) and EXPECTED.json goldens. For a new nested-call/list program (ChainTotal, plus BranchTotal) compiled through the public mori_core::sherlock_compile_artifact::compile_draft_host pipeline into sherlock-host-artifact/2, the emitted mapping provides all 26 literal named-unproved obligations (22 constructs + 4 dedicated sorts) covering returned_values, defined_failures, order, resource_obligation ('draft-to-b.resource-observations/1'), and effects_and_duties ('not-applicable: pure DedicatedSorts with FinanceInput::Absent'), while proof.gap explicitly records that host evaluation cannot supply missing circuit evidence.",
    "remaining_gaps": [
      "All 26 obligations in BODY 01 have status='named-unproved' and proof.semantic_correspondence='unproved'; report equality is literal structural reporting, not a semantic theorem or circuit proof.",
      "proof.runtime_values_failures_order_resources='unobserved-by-compile-only-preparation': R239 compile-only preparation does not execute B microsteps or observe runtime 7/8 values, defined runtime failures (codes 3, 5, 6, 7, 8, 9, 10, 11, or rejection 4), evaluation order traces, consumed work, or effects.",
      "proof.witness_plan='missing', proof.constraint_plan='missing', proof.native_export='unsupported', proof.native_proof_target='unrun'.",
      "Public negative/hostile compile_draft_host refusal tests and mori CLI subprocess execution (--source-profile, --entry) remain unrun.",
      "All five exact R238 owner gaps (GAP-1 through GAP-5 in BODY 02) remain open."
    ]
  },
  "result_acceptance_after_correction": {
    "unit_r221_rev02": "SURVIVES_UNCHANGED: APPROVE_OBSERVED_R221_REV02_BOUNDED_UNIT_RESULTS (planSha256=97272d867326d0764f9110b12a773878bbd07b2110d8eb67a004c26f2ae8333f, resultSha256=e4f40b09ffc94de9d2ac344ef1813e03113300c0527b54f9533c688ad30d77e4).",
    "public_r239": "SURVIVES_WITH_LITERAL_PROSE_CORRECTION: APPROVE_OBSERVED_R239_BOUNDED_PUBLIC_COMPILER_RESULTS (planSha256=47a61a287301e5b31845c2a867e12582b99fe66d372d1c05a03d311ad08c035d, resultSha256=a9dffaef3b364f9f677acdced9d51b715b95cafc1232909c778edb6fdee67923). The executed binary asserted exact equality against actual CORRESPONDENCE-EXPECTED.json (sha256=35284d3bcd2e55eb8f0b0792a0a9cfda51190b03a75e37c523a9f11e54c311e7, BODY 01) and EXPECTED.json (sha256=24cc457da9858ecbb6358f2f146f151151ab59ac4213bc4757f44b88060f121b); only R245's summary examples required correction."
  },
  "document_compatibility": {
    "025_current_ref": {
      "decision": "RECOMMEND_DOC_REFERENCE_REFRESH",
      "pointer": "/requirements/24/currentness/sourcePins/35",
      "path": "docs/sherlock/reference/host-source-compiler.md",
      "old_sha256": "fd3a7848d9545c496d50cae410093fb0da5b6cdce6a04dd3d6d1775036fac59c",
      "old_bytes": 7233,
      "current_sha256": "08f50a3f93c0cc687faf7bc4911096ba7f27ffd7b8dea248f6fbcacaa3e15c57",
      "current_bytes": 7835,
      "basis": "BODY 05-BODY 07 confirm the entire Core1 scalar sherlock-host-artifact/1 reporting body, 11-row table, 12-field ProgramView description, CompileFailure description, unbuilt CLI contract, and observed result boundary are preserved verbatim. The diff only relabels line 3 as 'Historical Core1 result cut, 2026-10-17:', inserts a 7-line cross-reference to draft-source-compiler.md and its 2026-10-18 result explicitly stating 'that result does not rerun or broaden the historical Core1 captures below', and updates 5 line-number anchors (lib.rs#L9->#L11, compile_artifact.rs#L121->#L165, #L26->#L35, #L87->#L96, compile.rs#L8->#L9) matching the accepted R235 source diffs. No new artifact/2 proof claim or broadening of UIREQ-025's C1-C4 Pure-models-only reporting closure is introduced."
    },
    "026_three_current_refs": {
      "decision": "RECOMMEND_THREE_DOC_REFERENCE_REFRESHES",
      "pointers": [
        "/requirements/25/validation/evidenceRefs/2",
        "/requirements/25/currentness/sourcePins/2",
        "/requirements/25/citations/3"
      ],
      "path": "docs/SHERLOCK-CORRECTNESS-LOOP.md",
      "line_range": {
        "startLine": 1,
        "endLine": 300
      },
      "old_sha256": "50e821540973b82c48d59c555d522e921211634dd9d115cc7a4f21893c050645",
      "old_bytes": 36558,
      "intermediate_R216_sha256": "deb72d31ebe826581870c0666270d113dc6c88ca853759827c61624b610a2bed",
      "intermediate_R216_bytes": 40820,
      "current_sha256": "e6c11b578c7325b31ddd46721fdd8a233001ac6655f52ff9b12100698489c88f",
      "current_bytes": 44445,
      "basis": "BODY 08-BODY 12 confirm current-correctness-loop.md (44445B) equals the exact 36558B ledger-bound old body (lines 1-472, with lines 1-300 100% byte-identical) plus literal R216 append (4262B, sha256=27befbbd0f317106eda57404825661591bb085d20ae8c403d563790e292c1c80) plus literal R247 append (3625B, sha256=6a9ce3ce98d5dd021e44f734c79fe243dd0b0b3f2b9aa8a76d11806acb435b5f). Pointer /requirements/25/validation/evidenceRefs/2 is a current design-workflow documentation locator (startLine: 1, endLine: 300) for Scenario 'Iterate reference models and handwritten circuits', distinct from historical actual-result execution receipts. Both appended sections (R216 and R247) explicitly preserve all missing-layer, runtime, circuit, native-authority-0, and open-goal boundaries without claiming any private Fenrir Clean binding or new closure count."
    },
    "026_remains_open": true,
    "historical_validation_and_evidenceCatalog_preserved": true,
    "limitations": [
      "Refreshing UIREQ-025 /requirements/24/currentness/sourcePins/35 and UIREQ-026's three current-doc pointers updates only those four documentation digests; all row statuses (UIREQ-025 closed/adopted/observed; UIREQ-026 open/pending-root-semantic-review/partial), predicates, historical execution receipts, recordedConsumerClosure, and evidenceCatalog remain unchanged.",
      "Documentation appends in SHERLOCK-CORRECTNESS-LOOP.md (R216 and R247) and cross-links in host-source-compiler.md supply no new artifact/2 runtime, circuit/Clean, private Fenrir binding, or native proof evidence, and manufacture no currentness/closure count.",
      "Notice in R247 append text (BODY 11): R247 was authored at 2026-10-18T02:40:02Z and accurately states 'R245 same-owner actual-result/compatibility review is preparing; original UIREQ022 remains OPEN until Root adopts a supported result'; this dated snapshot statement does not conflict with subsequent Root review/adoption actions."
    ]
  },
  "findings": [
    {
      "id": "R252-CORR-01",
      "severity": "RESOLVED_SELF_CORRECTION",
      "literal_anchor": "BODY 01 (CORRESPONDENCE-EXPECTED.json obligations[0..25], resources, finance, proof) & BODY 02 (R238 preserved_owner_gaps[0..4]) vs BODY 03 (R245 response)",
      "reason": "R245 summary prose used paraphrased obligation examples ('arith-overflow-and-shift-failure', 'sequence', 'function-reference', 'step-fuel-accounting', etc.), a '22+4 applicability' label instead of '22 constructs + 4 DedicatedSorts', and compressed 'Owner Gap O1..O4' prose instead of the verbatim R238 GAP-1..GAP-5 strings.",
      "required_change": "Corrected in this response: all 26 verbatim obligation IDs and the exact 22-construct + 4-sort taxonomy from BODY 01 plus the verbatim GAP-1..GAP-5 strings from BODY 02 now govern the R245/R252 record."
    }
  ],
  "execution_authority": 0,
  "end_marker": "END-LITERAL-QUALIFICATION-DOC-COMPAT-NATIVE0"
}

