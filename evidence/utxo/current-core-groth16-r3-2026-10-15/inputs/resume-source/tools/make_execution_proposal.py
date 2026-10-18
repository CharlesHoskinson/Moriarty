"""Write EXECUTION-PROPOSAL.json (exclusive). Stdlib only; hashes referenced files read-only.
A proposal for review: it requests and allocates no authority and runs nothing."""
import hashlib, json, os, resource

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
OWN = BASE + "/current-core-groth16-evidence-repair-resume-source-20261015"
PKT = BASE + "/current-core-groth16-compiler-independent-source-review-20261015"
R2 = BASE + "/current-core-groth16-compiler-repair-independent-review-20261015/plumbing-r2"
RUN = BASE + "/current-core-groth16-env-corrected-offline-execution-20261015"
CHECKOUT = "/home/charl/Moriarty/.worktrees/utxo-pr54-currentness-20261015"
ROOT = BASE + "/current-core-groth16-evidence-repair-check-r3-1"
TCB = "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu"


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


r2req = json.load(open(BASE + "/current-core-groth16-compiler-repair-independent-review-20261015/R2-PLUMBING-EXECUTION-REQUEST.json"))
e4_tests = sorted(n for g, ns in r2req["expected_tests"].items() if g != "heldout_harness_r2" for n in ns)
assert len(e4_tests) == 41
e5_tests = ["heldout_vectors_core_and_relation", "permissive_hostile_claims_are_refused", "targeted_mutations", "absent_duty_discharge_control",
            "static_refusals", "build_identity_is_recomputed_from_the_copied_bytes", "reencoded_hostile_claims_addendum3",
            "financial_accounting_against_frozen", "gross_debit_and_fees_relation", "rejected_phase_and_work_attempted",
            "hw25_duplicate_balance_key", "hw_traceability", "probe_reports_and_identities", "addendum4_dated_cases"]
harness = open(OWN + "/review-overlay/proposals/heldout-harness-r3/tests/heldout.rs").read()
declared = [l.split("fn ", 1)[1].split("(", 1)[0] for l, prev in zip(harness.splitlines()[1:], harness.splitlines()) if prev.strip() == "#[test]"]
assert sorted(declared) == sorted(e5_tests), (declared, e5_tests)

pins = {
    "candidate_source_manifest": sha(OWN + "/candidate/SOURCE-MANIFEST.json"),
    "candidate_build_manifest": sha(OWN + "/candidate/BUILD-MANIFEST.json"),
    "core_closure": "f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed",
    "review_manifest": sha(PKT + "/REVIEW-MANIFEST.json"),
    "r2_manifest": sha(R2 + "/R2-MANIFEST.json"),
    "r3_manifest": sha(OWN + "/review-overlay/R3-MANIFEST.json"),
    "copy_verify_r3.py": sha(OWN + "/review-overlay/r3-tools/copy_verify_r3.py"),
    "copy_verify_r2.py (reviewed base)": sha(R2 + "/r2-tools/copy_verify_r2.py"),
    "lock_gate.py (r2, reviewed, executed)": sha(R2 + "/r2-tools/lock_gate.py"),
    "lock_subset_check.py (frozen packet)": sha(PKT + "/tools/lock_subset_check.py"),
    "LOCK-UNION-ANALYSIS.json (frozen packet)": sha(PKT + "/LOCK-UNION-ANALYSIS.json"),
    "OPTION-A-PLAN.json (executed run)": sha(RUN + "/OPTION-A-PLAN.json"),
    "run_pathb.py (reviewed operator, executed)": sha(RUN + "/tools/run_pathb.py"),
    "libtest_rows.py (proposed helper)": sha(OWN + "/operator-proposals/libtest_rows.py"),
    "OPERATOR-RESULT-PARSER.diff": sha(OWN + "/operator-proposals/OPERATOR-RESULT-PARSER.diff"),
    "PREDICTED-ROWS.json": sha(OWN + "/records/PREDICTED-ROWS.json"),
    "HELDOUT-ADDENDUM-4.json": sha(OWN + "/review-overlay/heldout-addendum-4/HELDOUT-ADDENDUM-4.json"),
}
assert pins["candidate_build_manifest"] == "edbe3193c8eabc51a1d5595757c69c0bbe2caeb89e53cc3dc25564cead44b41a"
assert pins["LOCK-UNION-ANALYSIS.json (frozen packet)"] == "328d157e00db80259f986f978f8ee222daa762277fc5b4377a3d2c0f8cc1c5f9"
assert pins["run_pathb.py (reviewed operator, executed)"] == "c9cd72377f463b28cf2da791364b3f419018e5ff7ba560656983c207b28b28e1"

CARGO, RUSTC, RUSTDOC = TCB + "/bin/cargo", TCB + "/bin/rustc", TCB + "/bin/rustdoc"
LOCK_C = "r1cs/source/mori-r1cs/Cargo.lock"
LOCK_H = "review/proposals/heldout-harness-r3/Cargo.lock"
ALLOW = ["mori-r1cs", "mori-core", "mori-lexer", "mori-diagnostics", "mori-span"]
U = "328d157e00db80259f986f978f8ee222daa762277fc5b4377a3d2c0f8cc1c5f9"
steps = [
    {"id": "E0r3", "cpu_s": 60, "wrapper": "none (python3 -I from the operator task directory, outside <root>, as E0r2)",
     "argv": ["/usr/bin/python3", "-I", OWN + "/review-overlay/r3-tools/copy_verify_r3.py", "--copy", CHECKOUT, OWN + "/candidate", PKT, R2, OWN + "/review-overlay", ROOT,
              "--expect", "source_manifest=" + pins["candidate_source_manifest"], "build_manifest=" + pins["candidate_build_manifest"],
              "core_closure=" + pins["core_closure"], "review_manifest=" + pins["review_manifest"], "r2_manifest=" + pins["r2_manifest"],
              "r3_manifest=" + pins["r3_manifest"]],
     "pass_requires": {"verdict": "PASS", "counts": {"moriarty": 70, "r1cs": 127, "review": 64, "review_frozen": 42, "review_r2": 13, "review_r3": 9},
                       "core_closure_recomputed": pins["core_closure"], "core_closure_files": 70, "build_manifest_recomputed": pins["candidate_build_manifest"],
                       "build_manifest_entries": 16, "compiler_build_digest_predicted": "0b896adbe183b79d82455fde7b2c5e988b166a63a5fde972d3621eb7ad98bf19"}},
    {"id": "E1", "cpu_s": 60, "argv": [CARGO, "generate-lockfile", "--offline", "--manifest-path", "r1cs/source/mori-r1cs/Cargo.toml"]},
    {"id": "E1v", "cpu_s": 60, "argv": ["python3", "-I", "review/tools/lock_subset_check.py", LOCK_C, "review/LOCK-UNION-ANALYSIS.json"]},
    {"id": "E1v+", "cpu_s": 60, "argv": ["python3", "-I", "review/r2-tools/lock_gate.py", "subset", LOCK_C, "review/LOCK-UNION-ANALYSIS.json", U] + ALLOW},
    {"id": "E1h-r3", "cpu_s": 60, "argv": [CARGO, "generate-lockfile", "--offline", "--manifest-path", "review/proposals/heldout-harness-r3/Cargo.toml"]},
    {"id": "E1hv-r3.subset", "cpu_s": 60, "argv": ["python3", "-I", "review/r2-tools/lock_gate.py", "subset", LOCK_H, "review/LOCK-UNION-ANALYSIS.json", U, "mori-r1cs-heldout-r3"] + ALLOW},
    {"id": "E1hv-r3.pair", "cpu_s": 60, "argv": ["python3", "-I", "review/r2-tools/lock_gate.py", "pair", LOCK_C, LOCK_H]},
    {"id": "E2", "cpu_s": 60, "argv": [CARGO, "tree", "--offline", "--locked", "--manifest-path", "r1cs/source/mori-r1cs/Cargo.toml", "-e", "normal,build,dev", "--features", "groth16"]},
    {"id": "E3a", "cpu_s": 900, "argv": [CARGO, "build", "--offline", "--locked", "--lib", "--manifest-path", "r1cs/source/mori-r1cs/Cargo.toml"]},
    {"id": "E3b", "cpu_s": 900, "argv": [CARGO, "build", "--offline", "--locked", "--tests", "--manifest-path", "r1cs/source/mori-r1cs/Cargo.toml"]},
    {"id": "E3c", "cpu_s": 900, "argv": [CARGO, "build", "--offline", "--locked", "--features", "groth16", "--manifest-path", "r1cs/source/mori-r1cs/Cargo.toml"]},
    {"id": "E4", "cpu_s": 900, "argv": [CARGO, "test", "--offline", "--locked", "--no-fail-fast", "--manifest-path", "r1cs/source/mori-r1cs/Cargo.toml", "--", "--test-threads=2"]},
    {"id": "E5-r3", "cpu_s": 900, "argv": [CARGO, "test", "--offline", "--locked", "--manifest-path", "review/proposals/heldout-harness-r3/Cargo.toml", "--", "--nocapture", "--test-threads=1"]},
]
deps = {"E1v": ["E1"], "E1v+": ["E1", "E1v"], "E1h-r3": ["E1v+"], "E1hv-r3.subset": ["E1h-r3"], "E1hv-r3.pair": ["E1hv-r3.subset"], "E2": ["E1v+"],
        "E3a": ["E1v+"], "E3b": ["E3a"], "E3c": ["E3a"], "E4": ["E3b"], "E5-r3": ["E3a", "E1hv-r3.pair"]}

proposal = {
    "format": "evidence-repair-resume-execution-proposal/1",
    "status": "PROPOSED ONLY by the evidence-repair resume source author (claude-opus-5-5). It authorizes nothing, allocates no budget, resets no counter and was never run. Before any command: a fresh non-author source/effect review of exactly the candidate (SOURCE-MANIFEST below), the R3 overlay (R3-MANIFEST below) with DIFFS/review/, the proposed helper and operator diff, and this proposal; a second substantive vote on each consequential block; root authorization; and a separately reviewed operator implementing these steps.",
    "inherited_counters": {
        "lineage_actual_runs": "1 consumed: current-core-groth16-env-corrected-offline-execution-20261015 (REPORT 18afb040...4cc1), all steps exit 0, accepted-qualified for its exact bytes only",
        "this_proposal": "requests no authority; consumes, resets or creates no execution counter or budget",
        "native_sdk_authority": 0,
        "retry_rule": "one attempt per authorization; no retry; a failure is recorded verbatim and routed to fresh review",
    },
    "pins_sha256": pins,
    "toolchain": {"cargo": CARGO, "rustc": RUSTC, "rustdoc": RUSTDOC, "RUSTUP_TOOLCHAIN": "1.99.0-x86_64-unknown-linux-gnu",
                  "four_pins": {"bin/cargo": [36160240, "e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14"],
                                "bin/rustc": [647376, "f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f"],
                                "lib/librustc_driver-999a121de2f042be.so": [158319840, "f45c41337f1516bb9f4d504a2a507202a01c431fc2063334913dde5480ffee03"],
                                "bin/rustdoc": [12568360, "a1321a60578a6b2e3e0ff428201b8618bbd9a9fbcd475c0d0f151bbf17cb0a9e"]},
                  "control": "operator re-hashes the four pins immediately before E1 and after E5-r3; STOP on any difference (pins copied from the reviewed operator text, not re-hashed by this author)"},
    "scratch_root": {"path": ROOT, "must_not_exist": True, "observed_absent_by_author_name_only": True,
                     "never_reuse": BASE + "/current-core-groth16-offline-check-1 (the consumed run's scratch)"},
    "isolated_layout": {"moriarty/": "70 Core closure files (CORE-CLOSURE-PINS, f47318a2...) from the currentness checkout", "r1cs/": "126 candidate SOURCE-MANIFEST files + SOURCE-MANIFEST.json (127)",
                        "review/": "42 frozen packet files + 12 r2 files + R2-MANIFEST.json + 8 r3 overlay files + R3-MANIFEST.json (64); no path collides",
                        "relative_paths": "identical to the executed r2 layout; the r3 crate sits at review/proposals/heldout-harness-r3/ like r2"},
    "environment": {"wrapper": "/usr/bin/unshare --user --map-root-user --net -- /usr/bin/env -i",
                    "env": ["PATH=/usr/bin:/bin", f"HOME={ROOT}/empty-home", f"TMPDIR={ROOT}/tmp", f"CARGO_HOME={ROOT}/cargo-home", f"CARGO_TARGET_DIR={ROOT}/target",
                            "CARGO_NET_OFFLINE=true", "CARGO_INCREMENTAL=0", "CARGO_BUILD_JOBS=2", "RUSTUP_TOOLCHAIN=1.99.0-x86_64-unknown-linux-gnu", f"RUSTC={RUSTC}", f"RUSTDOC={RUSTDOC}"],
                    "same_as": "the literal ENV-r2 block executed by the consumed run (11 variables), with the new root"},
    "limits": {"RLIMIT_AS": "8 GiB", "RLIMIT_CPU_s": "60 (E0r3, E1 family, lock gates, E2) or 900 (E3a, E3b, E3c, E4, E5-r3)", "wall_s": 1800, "RLIMIT_CORE": 0, "nice": 10,
               "disk": "stop below 3 GiB free", "failure_classification": "a failure attributable to a limit is BLOCKED(resource), recorded verbatim, never a candidate defect, never retried"},
    "cargo_home": {"option": "A (hermetic)", "union": "review/LOCK-UNION-ANALYSIS.json 328d157e... (102 registry crates)", "plan": "the executed run's OPTION-A-PLAN.json (pinned above), unchanged",
                   "why_sufficient": "the candidate Cargo.toml (660d88a5...) is unchanged and the r3 harness declares exactly the r2 dev-dependencies, so both locks must stay inside the same union; the lock gates enforce this rather than assume it"},
    "steps": steps, "dependencies": deps,
    "e5_step": "E5-r3",
    "expected_tests": {"E4": e4_tests, "E5-r3": e5_tests},
    "predictions": {"file": "records/PREDICTED-ROWS.json", "status": "AUTH predictions only; an actual run prints the real rows",
                    "E5-r3_tests": 14, "E4_tests": 41, "identity_lines": 3,
                    "identity_expected": {"build_manifest_sha256": pins["candidate_build_manifest"], "core_closure_sha256": pins["core_closure"],
                                          "compiler_build_digest_predicted": "0b896adbe183b79d82455fde7b2c5e988b166a63a5fde972d3621eb7ad98bf19"}},
    "post_run": {"raw": "run/ logs and RESULT.json are never edited after the run",
                 "result_json_parser": "operator-proposals/libtest_rows.py result_tests() via OPERATOR-RESULT-PARSER.diff (frozen and hash-checked by the operator)",
                 "analysis": "python3 -I libtest_rows.py analyze --run <task>/run --request EXECUTION-PROPOSAL.json --hw-trace review/heldout-addendum-4/HW-TRACEABILITY-2.json --out-dir <task>/analysis --e0 <task>/run/E0r3.stdout; every analysis is a new dated file; the first (even if defective) is kept"},
    "scoring": {
        "independent": "reviewer-frozen rows only: held-out vectors (Core differential + relation), hostile claims, mutations M-01..M-14, addendum-2 rows, static S-1..S-9 (the frozen requirement), frozen accounting fields and frozen phase/work_attempted",
        "auth": "addendum 3 and addendum 4 rows (re-encoded claims, S-9 constructions, HW-xx-r3 cases, booleanity isolation, gross/fees relation predictions, phase/work observation method) are author-derived and never independent",
        "internal_agreement": "E4 (41 author/oracle-reused tests)",
        "open_by_design": "OPEN rows (premises, not-decodable frozen claims, target gaps, contradiction, unrepresented planes, HW trace entries) are never counted as passes",
        "expected_documented_gaps": "M-14 SAT (F-04); M-06 relation half and M-07 (F-05) target obligations; M-11b contradiction; HW-26 padding not applicable",
        "fail_handling": "record verbatim; classify as candidate defect, harness defect, oracle defect, AUTH-prediction defect or library-API defect; no refit of frozen expectations; no retry",
    },
    "never": r2req["never"] + r2req["never_additional"] + ["keys, setup, proving or verification of any kind", "editing the candidate, the frozen packet, the r2 or r3 overlay, old reports or raw logs"],
}
data = (json.dumps(proposal, indent=1) + "\n").encode()
fd = os.open(OWN + "/EXECUTION-PROPOSAL.json", os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
with os.fdopen(fd, "wb") as f:
    f.write(data)
print(hashlib.sha256(data).hexdigest(), len(steps), len(e4_tests), len(e5_tests))
