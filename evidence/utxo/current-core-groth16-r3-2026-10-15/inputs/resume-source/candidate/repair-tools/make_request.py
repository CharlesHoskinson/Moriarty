#!/usr/bin/env python3
"""Build CORRECTED-EXECUTION-REQUEST.json for the review-repair successor (stdlib only).

Blocks this repair author agrees with literally are copied from the reviewer's
CORRECTED-EXECUTION-REQUEST.json by value, so identical-text vote counting is mechanical;
the "text_identity" section records, per block, whether it is JSON-identical to the
reviewer's block. Writes OUT exclusively.

Usage: python3 -I make_request.py REVIEWER_REQUEST INPUT_VERIFICATION REPAIR_FREEZE BUILD_MANIFEST OUT
"""
import hashlib
import json
import resource
import sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

rev_path, inv_path, freeze_path, bm_path, out = sys.argv[1:6]
rev_raw = open(rev_path, "rb").read()
rev = json.loads(rev_raw)
inv = json.load(open(inv_path))
freeze = json.load(open(freeze_path))
bm_sha = hashlib.sha256(open(bm_path, "rb").read()).hexdigest()
cmd = {c["id"]: c for c in rev["commands"]}
SUCC = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-compiler-review-repair-source-20261015"

expected_tests = {
    "generic_pipeline": ["installment_invoice_lifecycle_satisfies_the_compiled_relations", "conditional_reserve_lifecycle_satisfies_the_compiled_relations",
                         "frontend_yields_the_frozen_typed_programs_and_their_canonical_hashes", "contrasting_programs_get_distinct_bound_identities_from_the_same_compiler",
                         "fresh_payroll_program_pay_and_settle_satisfy_author_expectations", "compile_source_is_deterministic_and_never_reads_program_names"],
    "arithmetic": ["checked_addition_boundaries", "logical_cap_boundary", "checked_subtraction_boundaries", "checked_multiplication_keeps_the_256_bit_intermediate_exact",
                   "field_wrapped_product_is_refused_for_the_honest_and_the_hostile_witness", "comparisons_and_their_complements", "or_evaluates_both_operands", "limbs_and_bools_are_canonical"],
    "duties_frame": ["present_zero_is_not_absence", "absent_duty_with_a_hidden_amount_is_refused_in_isolation", "discharge_requires_presence_and_bounded_amount",
                     "self_transfer_keeps_the_debit_check", "extra_frame_cells_are_preserved", "shared_dag_nodes_are_reevaluated_per_statement"],
    "tamper": ["every_single_slot_change_is_refused_except_the_valid_present_zero_pre_state", "every_header_field_is_a_bound_constant",
               "public_cap_is_a_claimed_upper_bound_on_w", "comparison_flag_and_strict_less_than_cannot_be_forged"],
    "accounting": ["oracle_lifecycles_expose_exact_gross_debit_and_fees", "self_transfer_counts_toward_gross_debit", "summary_limbs_are_bound_and_canonical"],
    "refusals": ["staged_programs_are_admitted_by_stage_load_then_refused_with_their_plan", "staged_not_advanced_costs_match_the_staged_oracle",
                 "core_admission_refusals_are_core_errors", "unknown_action_external_duty_and_static_failures_are_refused", "represented_snapshot_bounds_are_checked_conservatively",
                 "identity_inputs_are_mandatory", "emit_is_lowered_but_refused_whole_program_by_the_initial_ledger_target", "claimant_bound_abi_v1_is_not_adopted_here",
                 "coverage_rows_exist_for_every_construct"],
    "propose": ["reference_proposals_equal_the_frozen_oracle_and_satisfy_the_relation", "a_reference_refusal_proposes_nothing", "a_reference_success_is_not_trusted"],
    "assignment_shape": ["well_shaped_assignments_still_reach_the_constraint_system", "malformed_assignment_lengths_are_refused_before_synthesis"],
}
assert sum(len(v) for v in expected_tests.values()) == 41

tc = inv["toolchain"]
req = {
    "format": "repair-corrected-execution-request/1",
    "status": "PROPOSED ONLY by the SOURCEONLY review-repair author (claude-opus-5-5). It authorizes nothing; the author holds no runtime authority and ran no toolchain, crypto, SDK, VM, ledger or network command. Before ANY E0-E5 command: (1) a new exact full-candidate and delta Opus audit of the successor bytes named by its SOURCE-MANIFEST; (2) root authorization; (3) the existing two-substantive-agreeing-vote rule for the resource decision, counted on identical texts only (REPAIR-VOTES.md).",
    "supersedes_for_review_purposes": {
        "reviewer_corrected_request": {"path": "current-core-groth16-compiler-independent-source-review-20261015/CORRECTED-EXECUTION-REQUEST.json", "sha256": hashlib.sha256(rev_raw).hexdigest(), "edited": False},
        "candidate_request": {"path": "EXECUTION-REQUEST.json (byte-identical copy of the origin r2 file, kept at the successor root as history)", "sha256": "522f0d34d3d1aa7f57ef8a6a5b5f1ea38067834241f562448478eede0d7e5cbf", "edited": False},
    },
    "candidate": {
        "directory": SUCC,
        "source_manifest": "SOURCE-MANIFEST.json of the successor; its SHA-256 is in SOURCE-MANIFEST.json.sha256 and must be recorded by the root at handoff (a manifest cannot name its own digest)",
        "build_manifest_sha256": bm_sha,
        "build_manifest_origin_r2_sha256": "2c943b511bb6cb912889728ed2c5de90d4257939ded53b1d626b998edd3beb21",
        "core_closure_sha256": inv["core_closure"]["recorded_closure_sha256"],
        "core_head": inv["core_closure"]["recorded_head"],
        "core_closure_live_check": f"70/70 files matched the live checkout at {inv['core_closure']['live_head_from_git_metadata']} when this request was written (repair-tools/verify_inputs.py, INPUT-VERIFICATION.json); re-verify in E0",
        "build_identity_delta": "BUILD-IDENTITY-DELTA.json",
    },
    "inherited_counters": {
        "mori_r1cs_execution_attempts_any_candidate": 0,
        "authorized_runs": 0,
        "retries_consumed": 0,
        "native_sdk_authority": 0,
        "reviewer_request_state": "proposed, unauthorized, never run",
        "candidate_request_state": "proposed, unauthorized, never run; ineligible as submitted (review)",
        "budget": "No execution budget exists for this lane; none is created, reset or debited here.",
        "historical_status": "Root guarded status (SP01.6 loan-swap-subset blockers) is operational history, unchanged, and grants nothing to this lane.",
        "other_lanes_untouched": "Quint r3 26/40 budget and the ABI RG execution record (failed R3, no retry) are separate lanes' counters; not inherited, not modified.",
        "retry_rule": "No retry inside one authorization; a failure is recorded verbatim and routed to fresh review (inherited from both requests).",
    },
    "preconditions": [
        "PC-0 (repair author, new) A new exact full-candidate and delta Opus audit of the successor bytes (SOURCE-MANIFEST sidecar digest; DELTA.json and DIFFS/ against origin SOURCE-MANIFEST 063beb95...) is complete and recorded before any E0-E5 command. Missing or failed audit means nothing runs.",
        "PC-1r (repair author's amendment of reviewer PC-1) F-01 is repaired in the successor: tests/common `compile` renamed to `compile_ok` at its definition and its 24 call sites across all 7 test files, and duties_frame.rs calls `mori_r1cs::compile::<ark_bls12_381::Fr>` explicitly. Unlike PC-1's literal text, BUILD-MANIFEST does NOT stay 2c943b51...: the mandatory F-04 (src/coverage.rs, src/adapter.rs) and F-12 (src/circuit.rs, src/error.rs) changes move it to " + bm_sha + " (BUILD-IDENTITY-DELTA.json).",
        rev["preconditions"][1],
        "PC-2a (repair author, additive) The authorization record also names: the toolchain binary hashes in toolchain.repair_author_rehash_read_only, the lock-union registry-subset digest in lock_identity, REPAIR-STATIC-CHECKS.json and DELTA.json digests, and this request's sha256.",
        rev["preconditions"][2],
        "PC-3a (repair author, additive) Also record REPAIR-EXPECTATIONS/FREEZE.json aggregate " + freeze["aggregate_sha256"] + " (frozen " + freeze["frozen_at_utc"] + "), labelled REPAIR-AUTHOR and NOT independent.",
        "PC-4r (repair author's amendment of reviewer PC-4) The resource decision keeps the existing two substantive agreeing votes, counted only on identical texts. The reviewer's vote is in its REPORT.md sections 8.2 (C15) and 10 (the literal PC-4 cites section 9, which holds hostile proposals); the repair author's votes are in REPAIR-VOTES.md. Neither is a runtime authorization.",
    ],
    "isolated_layout": rev["isolated_layout"],
    "toolchain": {
        "selection": rev["toolchain"]["selection"],
        "pre_run_hashes_observed_by_reviewer_read_only": rev["toolchain"]["pre_run_hashes_observed_by_reviewer_read_only"],
        "control": rev["toolchain"]["control"],
        "repair_author_rehash_read_only": {
            "cargo": tc["bin/cargo"], "rustc": tc["bin/rustc"], "librustc_driver": tc["librustc_driver"],
            "equal_to_reviewer": tc["bin/cargo"]["sha256"] == "e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14" and tc["bin/rustc"]["sha256"] == "f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f",
            "rustup_settings_toml_observed": tc["rustup_settings_toml"],
            "cargo_config_files_present": tc["cargo_config_files_present"],
            "rust_toolchain_files_on_layout_ancestors": tc["rust_toolchain_files_on_task_ancestors"],
            "note": "Hashed as bytes only; no binary was executed. F-06: the rustup default is stable, so only the absolute 1.99.0 binaries plus RUSTUP_TOOLCHAIN select 1.99.0; the authorization must name these three hashes and STOP on any difference.",
        },
    },
    "lock_identity": {
        "union_file": {"path": "review/LOCK-UNION-ANALYSIS.json", "sha256": "328d157e00db80259f986f978f8ee222daa762277fc5b4377a3d2c0f8cc1c5f9", "packages": inv["lock_union"]["packages"],
                       "registry_packages": inv["lock_union"]["registry_packages"], "registry_crate_sha_equal_to_lock_checksum": inv["lock_union"]["crate_sha_matches_lock"]},
        "registry_subset_digest": inv["lock_union"]["registry_subset_sha256"],
        "registry_subset_rule": inv["lock_union"]["registry_subset_rule"],
        "predicate_F07": "Every registry package of a generated Cargo.lock must equal one (name, version, checksum) row of the 102 reviewed registry rows; no git source; no midnight-* crate; non-registry packages only the allowed local path crates. Tool: review/tools/lock_subset_check.py (sha256 a3215bd0df3a4fb04326dd87f11edd450225a785bdbca7a44aaacf87e8fb9435), unchanged.",
        "pair_predicate": "For E1 and E1h, every registry crate name present in both locks has the identical (version, checksum) set. Proposed tool: r1cs/repair-tools/lock_pair_compare.py (repair-author; positive and negative synthetic controls in repair-tools/testdata behaved as specified; needs review before use).",
        "record": "Record each generated lock's bytes, sha256 and the digest of its registry (name, version, checksum) subset under registry_subset_rule before any build.",
    },
    "identity_binding": {
        "already_bound_in_circuit": "Header constants: relation version, canonical Program hash, profile digest, action index, shape digest, compiler build digest = SHA-256(intent::canonical(CompilerBuild{compiler_source_manifest_sha256, core_closure_sha256})), W, and the cap slot under PublicCap. Bound by equality with public inputs under DirectFieldV0 (test-only).",
        "caller_supplied_today_F08": "Both CompilerBuild inputs are supplied by the caller; compile() rejects only all-zero values. The candidate's own tests supply placeholders 0x5a../0xc0.. (test-only), and the reviewer harness supplies the origin r2 digest 2c943b51..., stale for the successor (BUILD-IDENTITY-DELTA.json). Neither E4 nor E5 output is build-identity evidence.",
        "recomputation_required_in_a_future_run": "E0 recomputes the successor BUILD-MANIFEST sha256 and the Core closure digest (closure_sha256_rule) from the copied bytes; any relation identity reported as evidence must come from a harness that passes exactly those recomputed values. Today no reviewed harness does this; a reviewer-revised harness or a reviewed identity step is needed before identity rows are scored.",
        "proposed_catalogue_admission_metadata_not_in_circuit": ["build_manifest_sha256 (recomputed)", "core_closure_sha256 (recomputed)", "cargo_lock_sha256", "lock registry-subset digest", "toolchain: cargo, rustc and librustc_driver sha256 plus host triple", "observed compiler_build_digest and profile digest", "observed r1cs_sha256 per relation", "setup transcript linking r1cs_sha256 to a VK (future; setup is never-listed here)"],
        "status_of_proposal": "Proposed metadata only. No catalogue schema, admission rule or ABI v1 mapping is created or adopted. Putting lock or toolchain identity into CompilerBuild would change the in-circuit identity format and needs its own vote and review.",
        "permissionless_boundary": "These identities describe which bytes produced a relation. They are maintainer evidence controls for maintainer-run work only and never become public developer compile/prove/deploy permissions.",
    },
    "environment": rev["environment"],
    "sandbox": rev["sandbox"],
    "commands": [
        {"id": "E0", "argv": "python3 -I review/tools/copy_verify.py --copy <checkout> " + SUCC + " <review> <root>",
         "expect": "exit 0; 70 moriarty + successor SOURCE-MANIFEST files (+ SOURCE-MANIFEST.json) + 42 REVIEW-MANIFEST files verified after copy; record recomputed BUILD-MANIFEST and Core closure digests",
         "note": "run from a directory outside <root>; nobody has run --copy; copy_verify.py reads <candidate>/CORE-CLOSURE-PINS.json and <candidate>/SOURCE-MANIFEST.json, both present in the successor in the same format"},
        cmd["E1"], cmd["E1v"], cmd["E1h"],
        {"id": "E1hv", "argv": cmd["E1hv"]["argv"] + " && python3 -I r1cs/repair-tools/lock_pair_compare.py r1cs/source/mori-r1cs/Cargo.lock review/proposals/heldout-harness/Cargo.lock",
         "expect": "both PASS / SAME (exit 0); STOP on exit 3 from either", "amends": "reviewer E1hv 'compare with a reviewed diff' is made exact by the proposed comparator (needs review)"},
        cmd["E2"], cmd["E3a"],
        {"id": "E3b", "argv": cmd["E3b"]["argv"], "expect": "exit 0 is predicted after the F-01 repair (prediction only; nothing compiled). A compile error is a recorded finding: stop E4, no edit, no retry"},
        cmd["E3c"],
        {"id": "E4", "argv": cmd["E4"]["argv"], "expect": "41 named tests (expected_tests below): 39 inherited (one renamed under F-11) plus 2 repair-author F-12 tests. Author/oracle-reused and repair-author evidence only; not independent", "depends_on": ["E3b"]},
        {"id": "E5", "argv": cmd["E5"]["argv"],
         "expect": cmd["E5"]["expect"] + " Successor note: the harness's hard-coded BUILD_MANIFEST_SHA256 2c943b51... is stale for the successor; satisfiability outcomes are unaffected (encoder and circuit read the same supplied constant) but every identity and r1cs digest it prints names the origin build, so those columns are not evidence. M-14 stays expected SATISFIABLE (F-04 is relabelled, not fixed).",
         "depends_on": ["E3a"]},
    ],
    "expected_tests": expected_tests,
    "scoring": {
        "independent": rev["scoring"]["independent"],
        "expected_documented_gaps": rev["scoring"]["expected_documented_gaps"],
        "fail_handling": rev["scoring"]["fail_handling"],
        "repair_author_rows": "tests/assignment_shape.rs rows R12-* are repair-author expectations (REPAIR-EXPECTATIONS/F12-ASSIGNMENT-SHAPE.json), never independent.",
    },
    "never": rev["never"],
    "never_additional": ["using DirectFieldV0 as a claim ABI or binding claimant/outref through it", "any ABI v1 mapping while that ABI's full review is pending",
                          "treating initial_ledger_target_admits as enforced (no adapter calls it; F-10)", "reading any result as debit-authority, signed-intent, frame-completeness or P3 evidence"],
    "interpretation": rev["interpretation"],
    "required_and_open": "All of /2 staged lowering, external duties (F-02), the conservative bound (F-03), balance-row presence (F-04 premise), the duty frame and extra rows (F-05), a rejection relation (F-14), all 20 patterns, general obligations, full physical effects (P3), residual duties, recursion/PCD, private handoff, multi-parent composition, every U0-U7, M1-M17, MC01-MC08, SP01-SP12 and G01-G24 exit. No adoption and no target claim.",
}

identity = {}
for key in ("isolated_layout", "environment", "sandbox", "never", "interpretation"):
    identity[key] = req[key] == rev[key]
identity["toolchain.selection"] = req["toolchain"]["selection"] == rev["toolchain"]["selection"]
identity["toolchain.control"] = req["toolchain"]["control"] == rev["toolchain"]["control"]
identity["preconditions.PC-2"] = req["preconditions"][2] == rev["preconditions"][1]
identity["preconditions.PC-3"] = req["preconditions"][4] == rev["preconditions"][2]
for cid in ("E1", "E1v", "E1h", "E2", "E3a", "E3c"):
    identity["commands." + cid] = next(c for c in req["commands"] if c["id"] == cid) == cmd[cid]
for cid in ("E0", "E1hv", "E3b", "E4", "E5"):
    identity["commands." + cid] = next(c for c in req["commands"] if c["id"] == cid) == cmd[cid]
identity["scoring.independent/expected_documented_gaps/fail_handling"] = all(req["scoring"][k] == rev["scoring"][k] for k in ("independent", "expected_documented_gaps", "fail_handling"))
req["text_identity_with_reviewer_request"] = {
    "rule": "true = JSON-identical to the reviewer's block, so a literal AGREE by this author is a second vote on that exact text; false = amended or new text with this author's first vote only.",
    "blocks": identity,
}
with open(out, "x") as f:
    json.dump(req, f, indent=2)
    f.write("\n")
print(json.dumps(identity, indent=1))
