"""Focused SOURCE07/SOURCE08 native-adapter tests (ordinary mocks only).

Mapped to the independently authored pre-source expectations:
SINGLETON, FIXED_ARGV, CURRENT_RECEIPT, LIFECYCLE, FRESH_ELF,
ACCOUNTING, NO_PROMOTION. No real sudo/operator/cgroup/native/Cargo
execution ever happens here: stages are injected mocks, the capture
module is a fake returning records, and the only real subprocess is
an ordinary python -c through the default runner's stream plumbing.
These mocks establish factory source behavior only, never native
compatibility or acceptance.
"""
import copy
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

import delivery
import fixture_support as fx
import run_selected as rs

EXPECTED_POLICY = {
    "route": {
        "kind": "policy",
        "policyAdoptionSha256": fx.FX_ADOPTION,
        "policyHoldbackS": fx.FX_HOLDBACK,
        "policyAllowanceS": fx.FX_ALLOWANCE,
        "policyRemainingTotalWallMaxS": fx.FX_TOTAL_WALL_S,
        "policyRemainingFWallMaxS": fx.FX_F_WALL_S,
        "priorEspUs": fx.FX_PRIOR_ESP_US,
        "priorResidualUs": fx.FX_PRIOR_RESIDUAL_US,
        "native21DeltasEspUs": fx.FX_NATIVE21_DELTAS_ESP_US,
        "reserveAbUs": fx.FX_RESERVE_AB_US,
        "supervisorFloorBeforeNative21S": fx.FX_SUPERVISOR_FLOOR_S,
        "derivedCurrentDisposition":
            fx.debit_state("policy")["current_disposition"],
    },
    "derivedCurrentDisposition":
        fx.debit_state("policy")["current_disposition"],
    "baseHead": fx.FX_BASE,
    "cargoLockSha256": fx.FX_LOCK,
    "policyCandidateSha256": fx.FX_CANDIDATE,
    "policyHoldbackS": fx.FX_HOLDBACK,
    "policySupervisorAllowanceS": fx.FX_ALLOWANCE,
    "policyTotalWallMaxS": fx.FX_TOTAL_WALL_S,
    "policyFWallMaxS": fx.FX_F_WALL_S,
    "expectedFloorsS": fx.FX_FLOORS_S,
    "preservedDFreezeSha256": fx.FX_D_FREEZE,
    "preservedEResolvedSha256": fx.FX_E_RESOLVED,
    "totalWallSeconds": fx.FX_TOTAL_WALL_S,
    "fWallSeconds": fx.FX_F_WALL_S,
}
EXPECTED_ESTABLISHED = {
    "route": {
        "kind": "established",
        "wrapperCUs": 123,
        "priorEspUs": fx.FX_PRIOR_ESP_US,
        "priorResidualUs": fx.FX_PRIOR_RESIDUAL_US,
        "native21DeltasEspUs": fx.FX_NATIVE21_DELTAS_ESP_US,
        "reserveAbUs": fx.FX_RESERVE_AB_US,
        "supervisorFloorBeforeNative21S": fx.FX_SUPERVISOR_FLOOR_S,
        "derivedCurrentDisposition":
            fx.debit_state("established")["current_disposition"],
    },
    "derivedCurrentDisposition":
        fx.debit_state("established")["current_disposition"],
    "baseHead": fx.FX_BASE,
    "cargoLockSha256": fx.FX_LOCK,
    "policyCandidateSha256": fx.FX_CANDIDATE,
    "preservedDFreezeSha256": fx.FX_D_FREEZE,
    "preservedEResolvedSha256": fx.FX_E_RESOLVED,
}


def load_json(path):
    return json.loads(Path(path).read_bytes())


class FactoryCase(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=Path(__file__).parent)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.outer = fx.outer_fixture(self.root)
        self.task = fx.task(self.root, "build", "moriarty-sdk-build",
                            outer=self.outer)
        self.plan_path = fx.plan_file(self.root, [self.task])
        self.fp = delivery.fingerprint(
            delivery.load_plan(self.plan_path), "build")
        self.sel_path = self.root / "selection.json"
        self.write_selection()

    def write_selection(self, sel=None, sync_plan=True):
        if sel is None and sync_plan:
            self.task["identity"] = fx.identity(self.root, self.outer)
            self.plan_path = fx.plan_file(self.root, [self.task])
            self.fp = delivery.fingerprint(
                delivery.load_plan(self.plan_path), "build")
        sel = sel or fx.selection_dict(self.outer, self.fp)
        self.sel_path.write_bytes(fx.encoded(sel))
        return sel

    def go(self, runner, attempt="att"):
        return rs.run_selected_once(
            "build", str(self.plan_path), str(self.sel_path),
            str(self.root / attempt), runner=runner)

    def resolved(self):
        task = rs.resolve_task(str(self.plan_path), "build")
        return rs.resolve_selection(str(self.sel_path), task)


class SingletonFixedArgvTests(FactoryCase):
    def test_native_argv_exact_s_placement(self):
        sel = self.resolved()
        argv = rs.build_native_argv(sel)
        self.assertEqual(argv[:6], ["/usr/bin/sudo", "-n", "--",
                                    "/bin/sh", "-eu", "-c"])
        script = argv[6]
        s_dir = ("/sys/fs/cgroup/moriarty-fixture-envelope07"
                 "/supervisor")
        lines = script.splitlines()
        self.assertEqual(lines[0], f"echo $$ > {s_dir}/cgroup.procs")
        self.assertTrue(lines[1].startswith(
            "exec /usr/bin/setpriv --reuid=1000 --regid=1000 "
            "--clear-groups --inh-caps=-all --bounding-set=-all "
            "--no-new-privs -- /usr/bin/env -i LANG=C "
            "PATH=/usr/bin:/bin /usr/bin/python3 -I -B -S "))
        self.assertTrue(lines[1].endswith(
            f"{sel['guard']['path']} --cached-current-f "
            f"--config {sel['config']['path']} "
            f"--config-sha256 {sel['config']['sha256']}"))
        self.assertNotIn(str(os.getpid()), script)
        stages = rs.build_outer_stages(sel, self.root / "att-x")
        joined = {n: " ".join(s["argv"]) for n, s in stages.items()}
        for name, text in joined.items():
            if name != "native" and not name.startswith("counters-"):
                self.assertNotIn("cgroup.procs", text, name)
        self.assertIn("cgroup.procs", joined["native"])

    def test_direct_guard_argv_never_built(self):
        sel = self.resolved()
        stages = rs.build_outer_stages(sel, self.root / "att-y")
        for stage in stages.values():
            self.assertNotEqual(
                stage["argv"][:2], [sys.executable, sel["guard"]["path"]])

    def test_success_stage_order_and_single_native(self):
        runner = fx.outer_runner(self.outer)
        code, record = self.go(runner)
        self.assertEqual(code, rs.EXIT_OK)
        self.assertEqual(runner.calls, [
            "prelaunch-verify", "counters-before", "native",
            "counters-after", "collect-evidence", "teardown"])
        self.assertEqual(record["cleanupState"], "complete")

    def test_op0_to_op6_absent_from_stages_and_refused_by_operator_on_provisioned_host(self):
        sel = self.resolved()
        stages = rs.build_outer_stages(sel, self.root / "att-z")
        for forbidden in ("op0", "op1", "op2", "op3", "op4", "op5", "op6"):
            self.assertNotIn(forbidden, stages)
        runner = fx.outer_runner(self.outer)
        for forbidden in ("op0", "op1", "op2", "op3", "op4", "op5", "op6"):
            rc = runner({"stage": forbidden, "argv": []})
            self.assertEqual(rc, 2, f"{forbidden} must fail against provisioned host")

    def test_selection_extra_keys_zero_calls(self):
        for mutate in (
                lambda s: s.update(argv=["x"]),
                lambda s: s["outer"].update(cleanupHook="x"),
                lambda s: s["outputs"][0].update(path="/tmp/evil"),
                lambda s: s["outputs"].append(
                    {"role": "extra", "path": "/tmp/extra"}),
                lambda s: s.update(schema=
                                   "doctorwatson.execution-selection/1")):
            with self.subTest(mutate=mutate):
                sel = fx.selection_dict(self.outer, self.fp)
                mutate(sel)
                self.write_selection(sel)
                runner = fx.outer_runner(self.outer)
                with self.assertRaises((rs.BadInput, rs.Blocked)):
                    self.go(runner, attempt="att-" + str(id(mutate)))
                self.assertEqual(runner.calls, [])

    def test_request_config_disagreement_zero_calls(self):
        req = load_json(self.outer["request_path"])
        req["cgroup"]["envelope_path"] = "/different-envelope"
        self.outer["request_path"].write_bytes(fx.encoded(req))
        self.write_selection()
        runner = fx.outer_runner(self.outer)
        with self.assertRaises(rs.Blocked):
            self.go(runner)
        self.assertEqual(runner.calls, [])

    def test_guard_without_candidate_constant_blocked(self):
        self.outer["guard"].write_bytes(
            f"BASE = '{fx.FX_BASE}'\nLOCK = '{fx.FX_LOCK}'\n".encode())
        cfg = load_json(self.outer["config_path"])
        cfg["bindings"]["runner"] = fx.pin(self.outer["guard"])
        self.outer["config_path"].write_bytes(fx.encoded(cfg))
        self.write_selection()
        runner = fx.outer_runner(self.outer)
        with self.assertRaises(rs.Blocked):
            self.go(runner)
        self.assertEqual(runner.calls, [])

    def test_stale_retained_dir_zero_calls(self):
        self.outer["retained"].mkdir()
        runner = fx.outer_runner(self.outer)
        with self.assertRaises(rs.Blocked):
            self.go(runner)
        self.assertEqual(runner.calls, [])

    def test_helper_not_guard_sibling_or_wrong_name_or_digest_mismatch_blocked(self):
        # 1) Wrong filename (not cached_current_build.py)
        wrong_name = self.outer["guard"].parent / "other_helper.py"
        wrong_name.write_bytes(self.outer["helper"].read_bytes())
        orig_helper = self.outer["helper"]
        self.outer["helper"] = wrong_name
        self.write_selection()
        runner = fx.outer_runner(self.outer)
        with self.assertRaises(rs.Blocked):
            self.go(runner, attempt="att-wrong-name")
        self.assertEqual(runner.calls, [])

        # 2) Different parent directory
        other_dir = self.root / "other-dir"
        other_dir.mkdir()
        other_sibling = other_dir / "cached_current_build.py"
        other_sibling.write_bytes(orig_helper.read_bytes())
        self.outer["helper"] = other_sibling
        self.write_selection()
        with self.assertRaises(rs.Blocked):
            self.go(runner, attempt="att-other-dir")
        self.assertEqual(runner.calls, [])

        # 3) Sibling named cached_current_build.py but digest != guard's CACHED_CURRENT_BUILD_SHA256
        self.outer["helper"] = orig_helper
        orig_helper.write_bytes(b"HELPER = 'tampered-bytes'\n")
        self.write_selection()
        with self.assertRaises(rs.Blocked):
            self.go(runner, attempt="att-bad-sha")
        self.assertEqual(runner.calls, [])

    def test_missing_or_mismatched_outer_pin_in_task_identity_blocked(self):
        pin_keys = ("guard", "helper", "operator", "capture",
                    "time_tool", "config_path", "request_path")
        for key in pin_keys:
            with self.subTest(missing_pin=key):
                full_id = fx.identity(self.root, self.outer)
                target_path = str(self.outer[key])
                for cat in ("source", "tools", "config"):
                    full_id[cat] = [
                        item for item in full_id[cat]
                        if item["path"] != target_path
                    ]
                self.task["identity"] = full_id
                self.plan_path = fx.plan_file(self.root, [self.task])
                self.fp = delivery.fingerprint(
                    delivery.load_plan(self.plan_path), "build")
                self.write_selection(
                    fx.selection_dict(self.outer, self.fp),
                    sync_plan=False)
                runner = fx.outer_runner(self.outer)
                with self.assertRaises(rs.Blocked):
                    self.go(runner, attempt=f"att-missing-{key}")
                self.assertEqual(runner.calls, [])

        # Verify mutating any of the 7 pinned files changes delivery.fingerprint
        base_id = fx.identity(self.root, self.outer)
        self.task["identity"] = base_id
        self.plan_path = fx.plan_file(self.root, [self.task])
        base_fp = delivery.fingerprint(
            delivery.load_plan(self.plan_path), "build")
        for cat in ("source", "tools", "config"):
            for idx in range(len(base_id[cat])):
                mut_id = copy.deepcopy(base_id)
                mut_id[cat][idx]["sha256"] = "f" * 64
                self.task["identity"] = mut_id
                self.plan_path = fx.plan_file(self.root, [self.task])
                mut_fp = delivery.fingerprint(
                    delivery.load_plan(self.plan_path), "build")
                self.assertNotEqual(base_fp, mut_fp)


class CaptureTranslationTests(FactoryCase):
    def _fake_capture_stage(self, result_src):
        mod = self.root / "fake_capture.py"
        mod.write_text("def capture(op, phase):\n"
                       f"    return {result_src}\n")
        ref = fx.pin(mod)
        return {"stage": "native", "argv": [], "cwd": str(self.root),
                "env": {}, "wallSeconds": 1, "stdout": None,
                "stderr": None,
                "capture": {"module": ref,
                            "phaseDir": str(self.root / "phase"),
                            "op": {"id": "x"}}}

    def test_capture_null_exit_is_unknown_not_terminal(self):
        stage = self._fake_capture_stage(
            "{'id':'x','actual_exit':None,'elapsed_seconds':1.0,"
            "'controller_failure':'wall deadline','streams':[]}")
        with self.assertRaises(rs.StageUnknown):
            rs._default_runner(stage)

    def test_capture_controller_failure_with_exit_is_unknown(self):
        stage = self._fake_capture_stage(
            "{'id':'x','actual_exit':0,'elapsed_seconds':1.0,"
            "'controller_failure':'stdout capture cap exceeded',"
            "'streams':[]}")
        with self.assertRaises(rs.StageUnknown):
            rs._default_runner(stage)

    def test_capture_clean_exit_passes_through(self):
        stage = self._fake_capture_stage(
            "{'id':'x','actual_exit':7,'elapsed_seconds':1.0,"
            "'controller_failure':None,'streams':[]}")
        self.assertEqual(rs._default_runner(stage), 7)

    def test_default_runner_ordinary_stage_streams(self):
        out = self.root / "o.bin"
        err = self.root / "e.bin"
        stage = {"stage": "op0",
                 "argv": [sys.executable, "-c",
                          "import sys; print('hi'); sys.exit(3)"],
                 "cwd": str(self.root), "env": dict(os.environ),
                 "wallSeconds": 10, "stdout": str(out),
                 "stderr": str(err)}
        self.assertEqual(rs._default_runner(stage), 3)
        self.assertEqual(out.read_bytes(), b"hi\n")


class CurrentReceiptTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=Path(__file__).parent)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)

    def packet(self, route="policy", doc_mutator=None, **kw):
        shutil = __import__("shutil")
        for sub in ("packet", "target"):
            shutil.rmtree(self.root / sub, ignore_errors=True)
        elf = fx.write_packet(self.root / "packet", self.root / "target",
                              route=route, doc_mutator=doc_mutator, **kw)
        return {"f-result": str(self.root / "packet/F_CACHED_CURRENT_RESULT.json"),
                "f-outputs": str(self.root / "packet/F_QUARANTINED_OUTPUTS.json"),
                "post-compile-check": str(self.root / "packet/CACHED_POST_COMPILE_CHECK.json"),
                "stdout": str(self.root / "packet/F.stdout"),
                "stderr": str(self.root / "packet/F.stderr"),
                "backend-elf": str(elf),
                "resource-receipt": str(self.root / "packet/cleanup.json")}

    def decode(self, raw, expected=EXPECTED_POLICY, **kw):
        return rs.decode_cached_current_f(raw, expected, **kw)

    def test_policy_positive_preserved_fields(self):
        out = self.decode(self.packet())
        r = out["receipt"]
        self.assertEqual(r["routeKind"], "policy")
        self.assertEqual(r["supervisorCeilingKind"],
                         "POLICY_ALLOWANCE_NOT_REMAINING_BALANCE")
        self.assertEqual(r["policyHoldbackSWithheld"], 113)
        self.assertIsNone(r["wrapperCUs"])
        self.assertIsNone(r["exactRemainingSupervisorS"])
        self.assertTrue(out["built"])
        self.assertFalse(out["test_bodies_executed"])

    def test_established_positive(self):
        out = self.decode(self.packet(route="established"),
                          EXPECTED_ESTABLISHED)
        self.assertEqual(out["receipt"]["wrapperCUs"], 123)
        self.assertEqual(out["receipt"]["exactRemainingSupervisorS"], 10361)

    def test_p9_and_p10_actual_guard_launch_process_shapes_positive(self):
        for shape, expected_lens, expected_seps in (
            ("p10", [236, 126, 205], [214, 109, 184]),
            ("p9", [230, 126, 205], [208, 109, 184]),
        ):
            with self.subTest(shape=shape):
                procs = fx.guard_launch_processes(shape)
                self.assertEqual([len(p["argv"]) for p in procs],
                                 expected_lens)
                self.assertEqual([p["argv"].index("--") for p in procs],
                                 expected_seps)
                self.assertEqual([p["argv"][0] for p in procs],
                                 ["--die-with-parent"] * 3)
                raw = self.packet(
                    doc_mutator=lambda d, pr=procs:
                        d["f_result"].update(processes=pr))
                out = self.decode(raw)
                self.assertTrue(out["built"])

    def test_null_rusage_positive(self):
        def mut(doc):
            doc["supervisor_self_observation"] = fx.self_observation(
                null_rusage=True)
        out = self.decode(self.packet(doc_mutator=mut))
        obs = out["receipt"]["supervisorSelfObservation"]
        self.assertIsNone(obs["start_at_cached_current_f_entry"]["self_usec"])

    def test_refusals(self):
        cases = {
            "ten-key-old": lambda d: d.pop("supervisor_self_observation"),
            "arbitrary-extra": lambda d: d.update(extra=1),
            "missing-policy-key": lambda d: d.pop("policy_adoption_sha256"),
            "c-zero-promotion": lambda d: d["historical_debit_state_start"].update(wrapper_c_us=0),
            "exactS-numeric": lambda d: d["historical_debit_state_start"]["current_disposition"].update(exact_remaining_supervisor_s=5),
            "allowance-as-balance": lambda d: d["historical_debit_state_start"]["current_disposition"].update(remaining_supervisor_cpu_seconds=5),
            "duration-mismatch": lambda d: d["supervisor_self_observation"].update(duration_monotonic_ns=1),
            "negative-clock": lambda d: d["supervisor_self_observation"]["start_at_cached_current_f_entry"].update(monotonic_ns=-1),
            "basis-tamper": lambda d: d["supervisor_self_observation"].update(basis="other"),
            "ceiling-kind-rename": lambda d: d.update(supervisorCeilingKind="REMAINING_BALANCE"),
            "holdback-mismatch": lambda d: d.update(policy_holdback_s_withheld=114),
            "mixed-rusage-null": lambda d: d["supervisor_self_observation"]["start_at_cached_current_f_entry"].update(self_usec=None),
            "missing-disp-key-collecterror": lambda d: d["historical_debit_state_start"]["current_disposition"].pop("exact_remaining_supervisor_s"),
            "missing-self-obs-start-collecterror": lambda d: d["supervisor_self_observation"].pop("start_at_cached_current_f_entry"),
            "missing-hist-r3-key": lambda d: d["historical_debit_state_start"]["historical_debits"].pop("envelope_usec"),
            "extra-hist-r3-key": lambda d: d["historical_debit_state_start"]["historical_debits"].update(extra_r3=1),
            "bool-hist-r3-val": lambda d: d["historical_debit_state_start"]["historical_debits"].update(envelope_usec=True),
            "missing-p10-key": lambda d: d["historical_debit_state_start"]["post_p10_debits"].pop("envelope_usec"),
            "missing-p11-key": lambda d: d["historical_debit_state_start"]["post_p11_debits"].pop("envelope_usec"),
            "effective-sum-mismatch-end": lambda d: d["historical_debit_state_end"].update(effective_envelope_usec_at_start=999999),
            "live-regressed-end": lambda d: d["historical_debit_state_end"].update(live_envelope_usec_at_start=0, effective_envelope_usec_at_start=fx.FX_EFFECTIVE_ESP_US[0]),
            "start-total-wall-floor-exceeded": lambda d: d["historical_debit_state_start"]["current_disposition"].update(policy_remaining_total_wall_max_s=9999),
            "end-f-wall-floor-exceeded": lambda d: d["historical_debit_state_end"]["current_disposition"].update(policy_remaining_f_wall_max_s=999),
            "reconstitution-missing-key": lambda d: d["source_home_reconstitution"].pop("frozen_index_cohort"),
            "reconstitution-bad-d-freeze-sha": lambda d: d["source_home_reconstitution"].update(preserved_d_freeze_sha256="0" * 64),
            "reconstitution-bad-e-resolved-sha": lambda d: d["source_home_reconstitution"].update(preserved_e_resolved_sha256="0" * 64),
            "reconstitution-cargo-ok-bad-sha": lambda d: d["source_home_reconstitution"]["reconstructed_cargo_ok"][0].update(sha256="z" * 64),
            "reconstitution-delta-bad-kind": lambda d: d["source_home_reconstitution"]["dated_bookkeeping_deltas"][0].update(kind="unknown"),
            "reconstitution-cohort-count-mismatch": lambda d: d["source_home_reconstitution"]["frozen_index_cohort"].update(index_destinations_staged_from_cohort=99),
            "seed-counts-missing-key": lambda d: d["cached_target_seed_counts"].pop("owner_literal_workspace_deps"),
            "seed-counts-zero": lambda d: d["cached_target_seed_counts"].update(owner_literal_workspace_deps=0),
            "seed-counts-bool": lambda d: d["cached_target_seed_counts"].update(owner_literal_workspace_deps=True),
            "f-env-missing-key": lambda d: d["f_result"]["environment"].pop("CARGO_HOME"),
            "f-env-extra-key": lambda d: d["f_result"]["environment"].update(RUSTFLAGS="-Cdebuginfo=0"),
            "f-proc-empty": lambda d: d["f_result"].update(processes=[]),
            "f-proc-argv-not-list": lambda d: d["f_result"]["processes"][0].update(argv="--die-with-parent"),
            "f-proc-argv-empty": lambda d: d["f_result"]["processes"][0].update(argv=[]),
            "f-proc-argv-over-4096": lambda d: d["f_result"]["processes"][0].update(argv=["--die-with-parent"] * 4097),
            "f-proc-argv-non-str-element": lambda d: d["f_result"]["processes"][0].update(argv=["--die-with-parent", 123, "--", "/usr/bin/python3", "-I", "-B", "-S", "/proc/self/fd/3"]),
            "f-proc-argv-empty-str-element": lambda d: d["f_result"]["processes"][0].update(argv=["--die-with-parent", "", "--", "/usr/bin/python3", "-I", "-B", "-S", "/proc/self/fd/3"]),
            "f-proc-argv0-not-die-with-parent": lambda d: d["f_result"]["processes"][0].update(argv=["/usr/bin/python3", "--", "/usr/bin/python3", "-I", "-B", "-S", "/proc/self/fd/3"]),
            "f-proc-argv-missing-separator": lambda d: d["f_result"]["processes"][0].update(argv=["--die-with-parent", "/usr/bin/python3", "-I", "-B", "-S", "/proc/self/fd/3"]),
            "f-proc-argv-bad-cmd-prefix-after-sep": lambda d: d["f_result"]["processes"][0].update(argv=["--die-with-parent", "--as-pid-1", "--", "/usr/bin/cargo", "build", "--offline", "--locked"]),
            "f-proc-nonbool-audit": lambda d: d["f_result"]["processes"][0].update(audit_worker=1),
            "f-proc-nonzero-exit": lambda d: d["f_result"]["processes"][0].update(child_exit=1),
        }
        for name, mut in cases.items():
            with self.subTest(name=name):
                with self.assertRaises(rs.CollectError):
                    self.decode(self.packet(doc_mutator=mut))

    def test_n2_independent_disposition_mutations_refused_start_and_end(self):
        def adjust_prior(state, axis, delta):
            disp = state["current_disposition"]
            eff_key = f"effective_prior_{axis}_us"
            eff_start_key = {
                "E": "effective_envelope_usec_at_start",
                "S": "effective_supervisor_usec_at_start",
                "P": "effective_phases_usec_at_start",
            }[axis]
            disp[eff_key] += delta
            state[eff_start_key] += delta

        def adjust_cum_and_eff(state, axis, delta):
            state["current_disposition"][
                f"cumulative_before_wrapper_{axis}_us"] += delta
            adjust_prior(state, axis, delta)

        mutations = {
            "cum-E-plus-1": lambda s: adjust_cum_and_eff(s, "E", 1),
            "cum-S-plus-1": lambda s: adjust_cum_and_eff(s, "S", 1),
            "cum-P-plus-1": lambda s: adjust_cum_and_eff(s, "P", 1),
            "eff-E-only-reserve-mismatch": lambda s: adjust_prior(s, "E", 1),
            "eff-S-only-reserve-mismatch": lambda s: adjust_prior(s, "S", 1),
            "eff-P-only-mismatch": lambda s: adjust_prior(s, "P", 1),
            "reserve-both-E-S-plus-1": lambda s: (
                adjust_prior(s, "E", 1), adjust_prior(s, "S", 1)),
            "holdback-plus-1": lambda s: s["current_disposition"].update(
                policy_holdback_s=fx.FX_HOLDBACK + 1),
            "allowance-minus-1": lambda s: s["current_disposition"].update(
                policy_supervisor_allowance_s=fx.FX_ALLOWANCE - 1),
            "rem-total-9999": lambda s: s["current_disposition"].update(
                policy_remaining_total_wall_max_s=9999),
            "rem-total-4161": lambda s: s["current_disposition"].update(
                policy_remaining_total_wall_max_s=fx.FX_TOTAL_WALL_S - 1),
            "rem-f-999": lambda s: s["current_disposition"].update(
                policy_remaining_f_wall_max_s=999),
            "rem-f-351": lambda s: s["current_disposition"].update(
                policy_remaining_f_wall_max_s=fx.FX_F_WALL_S - 1),
        }
        for state_key in ("historical_debit_state_start",
                          "historical_debit_state_end"):
            for label, fn in mutations.items():
                with self.subTest(state=state_key, mutation=label):
                    def mut(doc, sk=state_key, f=fn):
                        f(doc[sk])
                    with self.assertRaises(rs.CollectError):
                        self.decode(self.packet(doc_mutator=mut))

    def test_quarantined_outputs_tamper_refusals(self):
        for mut in (
                lambda q: q.pop("host_outputs"),
                lambda q: q.update(acceptance="ACCEPTED"),
                lambda q: q.update(extra_field=1),
                lambda q: q["host_outputs"].append(
                    {"path": "../evil", "source_root": "x", "identity": "y",
                     "name": "z", "target_name": "t", "kind": "build-script",
                     "sha256": "a" * 64})):
            with self.subTest(mut=mut):
                raw = self.packet()
                qpath = Path(raw["f-outputs"])
                qdoc = load_json(qpath)
                mut(qdoc)
                qpath.write_bytes(fx.encoded(qdoc))
                with self.assertRaises(rs.CollectError):
                    self.decode(raw)

    def test_wrong_expected_pins_refuse(self):
        raw = self.packet()
        bad = copy.deepcopy(EXPECTED_POLICY)
        bad["policyCandidateSha256"] = "0" * 64
        with self.assertRaises(rs.CollectError):
            self.decode(raw, bad)
        bad2 = copy.deepcopy(EXPECTED_POLICY)
        bad2["route"]["policyAdoptionSha256"] = "1" * 64
        with self.assertRaises(rs.CollectError):
            self.decode(raw, bad2)

    def test_route_mismatch_refuses(self):
        with self.assertRaises(rs.CollectError):
            self.decode(self.packet(route="policy"), EXPECTED_ESTABLISHED)

    def test_alias_requires_preservation_mapping(self):
        raw = self.packet()
        live = Path(raw["backend-elf"])
        alias = self.root / "retained/backend-elf"
        alias.parent.mkdir()
        alias.write_bytes(live.read_bytes())
        raw["backend-elf"] = str(alias)
        with self.assertRaises(rs.CollectError):
            self.decode(raw)
        sha = fx.pin(alias)["sha256"]
        good = {"actualRelative": fx.NEW_ELF_REL, "sourcePath": str(live),
                "retainedPath": str(alias), "sha256": sha,
                "bytes": alias.stat().st_size}
        self.decode(raw, preservation=good)
        bad = dict(good, actualRelative="debug/deps/mori_backend-old")
        with self.assertRaises(rs.CollectError):
            self.decode(raw, preservation=bad)

    def test_multiple_fresh_candidates_refuse(self):
        raw = self.packet(extra_fresh=("debug/deps/mori_backend-second",))
        with self.assertRaises(rs.CollectError):
            self.decode(raw)


class LifecycleTests(FactoryCase):
    def ledger(self, attempt="att"):
        return load_json(self.root / attempt / "outer-stages.json")

    def receipt(self):
        return load_json(self.outer["retained"] / "cleanup.json")

    def test_prelaunch_verify_failure_preserves_preexisting_host_without_abort(self):
        runner = fx.outer_runner(self.outer,
                                 failures={"prelaunch-verify": 5})
        code, record = self.go(runner)
        self.assertEqual(code, rs.EXIT_FAILED)
        self.assertEqual(runner.calls, ["prelaunch-verify"])
        self.assertNotIn("native", runner.calls)
        self.assertNotIn("abort-prelaunch-cleanup", runner.calls)
        self.assertNotIn("teardown", runner.calls)
        self.assertFalse(runner.host_state["aborted_or_torn_down"])
        self.assertTrue(runner.host_state["provisioned"])
        self.assertTrue(runner.host_state["cgroup_active"])
        self.assertTrue(bool(runner.host_state["mounts_active"]))
        self.assertFalse((self.root / "att/consumer-result.json").exists())
        self.assertEqual(record["cleanupState"], "owed")
        self.assertEqual(self.receipt()["cleanup"], "owed")
        self.assertEqual(self.receipt()["restoration"], "owed")

    def test_prelaunch_verify_unknown_preserves_host_with_unknown_cleanup(self):
        inner = fx.outer_runner(self.outer)

        def runner(stage):
            if stage["stage"] == "prelaunch-verify":
                raise rs.StageUnknown("prelaunch probe timed out")
            return inner(stage)
        code, record = self.go(runner)
        self.assertEqual(code, rs.EXIT_UNKNOWN)
        self.assertEqual(inner.calls, [])
        self.assertFalse(inner.host_state["aborted_or_torn_down"])
        self.assertEqual(record["cleanupState"], "unknown")
        self.assertEqual(self.receipt()["cleanup"], "unknown")
        self.assertEqual(self.receipt()["restoration"], "unknown")

    def test_counters_before_failure_blocks_native_without_destroying_host(self):
        runner = fx.outer_runner(self.outer,
                                 failures={"counters-before": 1})
        code, record = self.go(runner)
        self.assertEqual(code, rs.EXIT_FAILED)
        self.assertEqual(runner.calls, ["prelaunch-verify",
                                        "counters-before"])
        self.assertNotIn("native", runner.calls)
        self.assertNotIn("abort-prelaunch-cleanup", runner.calls)
        self.assertNotIn("teardown", runner.calls)
        self.assertFalse(runner.host_state["aborted_or_torn_down"])
        self.assertEqual(record["cleanupState"], "owed")
        self.assertEqual(self.receipt()["restoration"], "owed")

    def test_missing_provisioned_host_prerequisites_refused_zero_calls(self):
        shutil = __import__("shutil")
        # 1) Missing Q-OBS observation_output
        self.outer["qobs_path"].unlink()
        runner = fx.outer_runner(self.outer)
        with self.assertRaises(rs.Blocked):
            self.go(runner, attempt="att-no-qobs")
        self.assertEqual(runner.calls, [])
        self.outer["qobs_path"].write_bytes(fx.encoded({"schema": "q-obs/1"}))

        # 2) Preexisting ONE_SHOT_STARTED marker
        marker = self.outer["compile_root"] / "ONE_SHOT_STARTED"
        marker.write_bytes(b"stale\n")
        with self.assertRaises(rs.Blocked):
            self.go(runner, attempt="att-stale-marker")
        self.assertEqual(runner.calls, [])
        marker.unlink()

        # 3) Missing execution_root
        shutil.rmtree(self.outer["compile_root"])
        with self.assertRaises(rs.Blocked):
            self.go(runner, attempt="att-no-exec-root")
        self.assertEqual(runner.calls, [])

    def test_zero_byte_precreated_time_output_allowed_and_nonempty_refused(self):
        # 0-byte regular file (precreated by /usr/bin/time -v -o before execvp)
        self.outer["time_output"].write_bytes(b"")
        runner = fx.outer_runner(self.outer)
        code, _ = self.go(runner)
        self.assertEqual(code, rs.EXIT_OK)
        preexec = load_json(self.root / "att/PREEXEC.json")
        self.assertEqual(
            preexec["outerTimeWrapper"]["timeOutputStateAtPreexec"],
            "precreated-zero-byte")

        # Non-empty stale timeOutput must be refused with zero calls
        self.outer["time_output"].write_bytes(b"Command being timed: ...\n")
        runner2 = fx.outer_runner(self.outer)
        with self.assertRaises(rs.Blocked):
            self.go(runner2, attempt="att-nonempty-time")
        self.assertEqual(runner2.calls, [])

        # Symlink timeOutput must be refused with zero calls
        self.outer["time_output"].unlink()
        target = self.root / "time-target.txt"
        target.write_bytes(b"")
        self.outer["time_output"].symlink_to(target)
        with self.assertRaises(rs.Blocked):
            self.go(runner2, attempt="att-symlink-time")
        self.assertEqual(runner2.calls, [])

    def test_mountpoint_still_active_in_mountinfo_prevents_restoration_complete(self):
        req = load_json(self.outer["request_path"])
        for mp_key in ("execution_root", "source_home", "packet"):
            with self.subTest(mount_key=mp_key):
                sub_root = self.root / f"sub-{mp_key}"
                sub_root.mkdir()
                outer = fx.outer_fixture(sub_root)
                task = fx.task(sub_root, "build", "moriarty-sdk-build",
                               outer=outer)
                plan_path = fx.plan_file(sub_root, [task])
                fp = delivery.fingerprint(
                    delivery.load_plan(plan_path), "build")
                sel_path = sub_root / "selection.json"
                sel_path.write_bytes(
                    fx.encoded(fx.selection_dict(outer, fp)))
                sub_req = load_json(outer["request_path"])
                mp = sub_req["paths"][mp_key]
                fake_mountinfo = (
                    f"999 28 0:55 / {mp} rw,nosuid,nodev - tmpfs tmpfs rw\n"
                )
                runner = fx.outer_runner(outer)
                with mock.patch.object(rs, "_read_mountinfo",
                                       return_value=fake_mountinfo):
                    code, record = rs.run_selected_once(
                        "build", str(plan_path), str(sel_path),
                        str(sub_root / "att"), runner=runner)
                self.assertEqual(code, rs.EXIT_OK)
                receipt = load_json(outer["retained"] / "cleanup.json")
                self.assertEqual(receipt["restoration"], "unknown")
                self.assertEqual(receipt["cleanup"], "unknown")
                self.assertEqual(record["cleanupState"], "unknown")
                obs = load_json(sub_root / "att/outer-observations.json")
                probe = obs["restorationProbe"]
                self.assertFalse(probe["mountsAbsent"])
                self.assertFalse(probe["restored"])
                self.assertIn(os.path.normpath(mp),
                              probe["activeRequestMountpoints"])

    def test_dynamic_allowance_derived_from_guard_and_config_not_hardcoded_170(self):
        # 1) Mismatch between config allowance and floor/guard constant is Blocked
        cfg = load_json(self.outer["config_path"])
        cfg["resource_disposition"]["policy_supervisor_allowance_s"] = 150
        self.outer["config_path"].write_bytes(fx.encoded(cfg))
        self.write_selection()
        with self.assertRaises(rs.Blocked):
            self.resolved()

        # 2) Consistent guard + config floor/allowance update derives 150
        # ceil((6931787 + 4556857) / 1_000_000) == 12; 275 - 12 - 113 == 150
        guard_text = self.outer["guard"].read_text(encoding="utf-8")
        guard_text = guard_text.replace(
            f"EXPECTED_FLOORS_S = {fx.FX_FLOORS_S!r}",
            "EXPECTED_FLOORS_S = (4275, 465, 275)")
        guard_text = guard_text.replace(
            f"POLICY_SUPERVISOR_ALLOWANCE_S = {fx.FX_ALLOWANCE}",
            "POLICY_SUPERVISOR_ALLOWANCE_S = 150")
        self.outer["guard"].write_bytes(guard_text.encode("utf-8"))
        cfg["bindings"]["runner"] = fx.pin(self.outer["guard"])
        cfg["supervisor_cpu_seconds"] = 150
        cfg["resource_disposition"]["supervisor_floor_before_native21_s"] = 275
        cfg["resource_disposition"]["policy_supervisor_allowance_s"] = 150
        self.outer["config_path"].write_bytes(fx.encoded(cfg))
        self.write_selection()
        sel = self.resolved()
        exp = rs._decoder_expected(sel)
        self.assertEqual(exp["route"]["policyAllowanceS"], 150)
        self.assertEqual(exp["policySupervisorAllowanceS"], 150)
        self.assertEqual(
            exp["derivedCurrentDisposition"]["policy_supervisor_allowance_s"],
            150)

    def test_n2_missing_or_mismatched_guard_and_helper_real_constants_blocked(self):
        orig_guard = self.outer["guard"].read_text(encoding="utf-8")
        orig_helper = self.outer["helper"].read_text(encoding="utf-8")
        orig_cfg = load_json(self.outer["config_path"])

        def sync_guard_and_config(g_text, h_text=orig_helper):
            self.outer["helper"].write_bytes(h_text.encode("utf-8"))
            h_sha = fx.pin(self.outer["helper"])["sha256"]
            g_patched = re_sub_helper(g_text, h_sha)
            self.outer["guard"].write_bytes(g_patched.encode("utf-8"))
            cfg = copy.deepcopy(orig_cfg)
            cfg["bindings"]["runner"] = fx.pin(self.outer["guard"])
            self.outer["config_path"].write_bytes(fx.encoded(cfg))
            self.write_selection()

        def re_sub_helper(text, h_sha):
            import re
            return re.sub(
                r"^CACHED_CURRENT_BUILD_SHA256\s*=\s*'[0-9a-f]{64}'",
                f"CACHED_CURRENT_BUILD_SHA256 = '{h_sha}'",
                text, flags=re.M)

        for bad_guard in (
            orig_guard.replace(
                f"POLICY_TOTAL_WALL_MAX_S = {fx.FX_TOTAL_WALL_S}\n", ""),
            orig_guard.replace(
                f"POLICY_TOTAL_WALL_MAX_S = {fx.FX_TOTAL_WALL_S}",
                "POLICY_TOTAL_WALL_MAX_S = 9999"),
            orig_guard.replace(
                f"POLICY_F_WALL_MAX_S = {fx.FX_F_WALL_S}",
                "POLICY_F_WALL_MAX_S = 999"),
            orig_guard.replace(
                f"EXPECTED_RESERVE_AB_US = {fx.FX_RESERVE_AB_US}",
                "EXPECTED_RESERVE_AB_US = 1"),
        ):
            with self.subTest(bad_guard=bad_guard[:60]):
                sync_guard_and_config(bad_guard)
                runner = fx.outer_runner(self.outer)
                with self.assertRaises(rs.Blocked):
                    self.go(runner, attempt="att-bad-g-" + str(id(bad_guard)))
                self.assertEqual(runner.calls, [])

        for bad_helper in (
            "# helper without PRESERVED_PACKET_PINS\n",
            orig_helper.replace("'D_FREEZE.json'", "'OTHER.json'"),
            orig_helper.replace("'E_RESOLVED.json'", "'OTHER2.json'"),
        ):
            with self.subTest(bad_helper=bad_helper[:40]):
                sync_guard_and_config(orig_guard, bad_helper)
                runner = fx.outer_runner(self.outer)
                with self.assertRaises(rs.Blocked):
                    self.go(runner, attempt="att-bad-h-" + str(id(bad_helper)))
                self.assertEqual(runner.calls, [])

        sync_guard_and_config(orig_guard, orig_helper)

    def test_n3_retained_dir_parent_missing_or_symlink_blocked_before_dispatch(self):
        # 1) Missing parent directory
        missing_parent_ret = self.root / "no-such-parent" / "retained07"
        self.outer["retained"] = missing_parent_ret
        self.write_selection()
        runner = fx.outer_runner(self.outer)
        att_missing = self.root / "att-missing-parent"
        with self.assertRaises(rs.Blocked):
            self.go(runner, attempt="att-missing-parent")
        self.assertEqual(runner.calls, [])
        self.assertFalse((att_missing / "PREEXEC.json").exists())

        # 2) Symlink parent directory
        real_parent = self.root / "real-ret-parent"
        real_parent.mkdir()
        sym_parent = self.root / "sym-ret-parent"
        sym_parent.symlink_to(real_parent)
        self.outer["retained"] = sym_parent / "retained07"
        self.write_selection()
        att_sym = self.root / "att-sym-parent"
        with self.assertRaises(rs.Blocked):
            self.go(runner, attempt="att-sym-parent")
        self.assertEqual(runner.calls, [])
        self.assertFalse((att_sym / "PREEXEC.json").exists())

        # 3) Regular file parent
        file_parent = self.root / "file-ret-parent"
        file_parent.write_bytes(b"not a dir\n")
        self.outer["retained"] = file_parent / "retained07"
        self.write_selection()
        att_file = self.root / "att-file-parent"
        with self.assertRaises(rs.Blocked):
            self.go(runner, attempt="att-file-parent")
        self.assertEqual(runner.calls, [])
        self.assertFalse((att_file / "PREEXEC.json").exists())

    def test_n3_postdispatch_retained_dir_oserror_records_unknown_and_retains_debt(self):
        shutil = __import__("shutil")
        reconciliation = __import__("reconciliation")
        ret_parent = self.root / "ephemeral-ret-parent"
        ret_parent.mkdir()
        self.outer["retained"] = ret_parent / "retained07"
        self.write_selection()
        inner = fx.outer_runner(self.outer)

        def runner(stage):
            rc = inner(stage)
            if stage["stage"] == "teardown":
                # Simulate retainedDir parent disappearing before post-dispatch
                # cleanup.json receipt write so retained_dir.mkdir raises OSError.
                shutil.rmtree(ret_parent)
            return rc

        code, record = self.go(runner, attempt="att-post-oserr")
        self.assertEqual(code, rs.EXIT_UNKNOWN)
        self.assertEqual(record["terminal"], "unknown-submission")
        self.assertEqual(record["unknownScope"], "local-process")
        self.assertEqual(record["cleanupState"], "unknown")
        self.assertIsNone(record["consumerResult"])
        att_dir = self.root / "att-post-oserr"
        self.assertTrue((att_dir / "PREEXEC.json").is_file())
        self.assertTrue((att_dir / "outer-observations.json").is_file())
        self.assertTrue((att_dir / "outer-stages.json").is_file())
        self.assertTrue((att_dir / "result.json").is_file())
        self.assertFalse((att_dir / "consumer-result.json").exists())
        persisted = load_json(att_dir / "result.json")
        self.assertEqual(persisted["terminal"], "unknown-submission")
        self.assertEqual(persisted["cleanupState"], "unknown")

        # Verify reconciliation.load_readiness over this unknown raw history
        # retains debt and blocks any rerun without reconciliation.
        artifacts = []
        for role in delivery.required_roles("moriarty-sdk-build"):
            raw = b"fixture\n"
            if role == "observation":
                raw = fx.encoded({
                    "schema": "doctorwatson.moriarty-sdk-build-observation/1",
                    "task": "build", "attempt": self.task["attempt"],
                    "fingerprint": self.fp, "built": True,
                    "test_bodies_executed": False})
            if role == "resource-receipt":
                raw = fx.encoded({
                    "schema": "doctorwatson.cleanup-observation/1",
                    "task": "build", "attempt": self.task["attempt"],
                    "fingerprint": self.fp,
                    "cleanup": persisted["cleanupState"],
                    "restoration": persisted["cleanupState"]})
            p_art = fx.write(self.root / ("art-oserr-" + role), raw)
            artifacts.append({"role": role, **fx.pin(p_art)})
        hist_path = self.root / "history-oserr.json"
        hist_path.write_bytes(fx.encoded({
            "schema": "doctorwatson.delivery-history/1",
            "results": [{
                "task": "build",
                "attempt": self.task["attempt"],
                "fingerprint": self.fp,
                "adapter": "moriarty-sdk-build",
                "terminal": persisted["terminal"],
                "exit_code": persisted["exit_code"],
                "signal": persisted["signal"],
                "cleanup": persisted["cleanupState"],
                "submission": "unknown",
                "artifacts": artifacts,
                "review": None,
                "adoption": None,
            }],
        }))
        readiness = reconciliation.load_readiness(
            str(self.plan_path), str(hist_path), None)
        row = next(
            r for r in readiness["report"]["tasks"] if r["id"] == "build")
        self.assertFalse(row["selection_candidate"])
        self.assertEqual(row["state"], "unknown-reconcile")
        self.assertEqual(readiness["report"]["ready_for_root_selection"], [])
        self.assertIn(reconciliation.UNKNOWN_BLOCKER, row["blockers"])
        plan_obj = delivery.load_plan(self.plan_path)
        res_obj = delivery.load_result(
            json.loads(hist_path.read_bytes())["results"][0])
        self.assertEqual(
            delivery.attempt_obligations(
                plan_obj.by_id["build"], res_obj, "current"),
            (True, True))
        rerun_runner = fx.outer_runner(self.outer)
        with self.assertRaises(rs.Blocked):
            rs.run_selected_once(
                "build", str(self.plan_path), str(self.sel_path),
                str(self.root / "att-post-oserr-rerun"),
                runner=rerun_runner, history_path=str(hist_path))
        self.assertEqual(rerun_runner.calls, [])

    def test_cli_main_and_default_runner_positive_mock_roundtrip(self):
        # Exercise rs.main(["--execute", ...]) -> run_selected_once ->
        # _default_runner subprocess execution without any privileged commands
        # by patching build_outer_stages to use sys.executable stub scripts.
        dw_dir = str(Path(__file__).parent)
        op_script = self.root / "stub_stage.py"
        op_script.write_text(
            "import json, pathlib, sys\n"
            f"sys.path.insert(0, {dw_dir!r})\n"
            "stage = sys.argv[1]\n"
            "meta = json.loads(pathlib.Path(sys.argv[2]).read_text())\n"
            "if stage in ('counters-before', 'counters-after'):\n"
            "    raw = {k: 'v\\n' for k in ('cpu.stat', 'memory.events',\n"
            "           'pids.events', 'cgroup.procs', 'cgroup.events')}\n"
            "    print(json.dumps({'monotonic_ns': 100,\n"
            "          'nodes': {n: {'raw': raw} for n in ('E','S','P')}}),\n"
            "          end='')\n"
            "elif stage == 'collect-evidence':\n"
            "    import fixture_support as fx\n"
            "    croot = pathlib.Path(meta['compile_root'])\n"
            "    ev = pathlib.Path(meta['evidence'])\n"
            "    (ev / 'packet').mkdir(parents=True, exist_ok=True)\n"
            "    pkt = croot / 'packet'\n"
            "    files = {'ONE_SHOT_STARTED': fx.pin(croot / 'ONE_SHOT_STARTED')['sha256']}\n"
            "    for f in pkt.iterdir():\n"
            "        (ev / 'packet' / f.name).write_bytes(f.read_bytes())\n"
            "        files['packet/' + f.name] = fx.pin(f)['sha256']\n"
            "    (ev / 'EVIDENCE_SHA256_MANIFEST.json').write_bytes(\n"
            "        fx.encoded({'request_sha256': fx.pin(pathlib.Path(meta['request_path']))['sha256'],\n"
            "                    'provisioner_sha256': fx.pin(pathlib.Path(meta['operator']))['sha256'],\n"
            "                    'observation_sha256': fx.pin(pathlib.Path(meta['qobs_path']))['sha256'],\n"
            "                    'files': files}))\n"
        )
        meta_file = self.root / "outer_meta.json"
        meta_file.write_text(json.dumps(
            {k: str(v) for k, v in self.outer.items()
             if isinstance(v, Path)}))
        cap_mod = self.outer["capture"]
        croot_str = str(self.outer["compile_root"])
        cap_mod.write_text(
            "import pathlib, fixture_support as fx\n"
            f"CROOT = pathlib.Path({croot_str!r})\n"
            "def capture(op, phase):\n"
            "    (CROOT / 'ONE_SHOT_STARTED').write_bytes(b'started\\n')\n"
            "    fx.write_packet(CROOT / 'packet', CROOT / 'target')\n"
            "    return {'id': op['id'], 'actual_exit': 0, 'elapsed_seconds': 0.1,\n"
            "            'controller_failure': None, 'streams': []}\n"
        )
        self.write_selection()
        orig_build = rs.build_outer_stages

        def stub_stages(selection, attempt):
            stages = orig_build(selection, attempt)
            for name, st in stages.items():
                if name != "native":
                    st["argv"] = [sys.executable, str(op_script),
                                  name, str(meta_file)]
            return stages

        self.outer["time_output"].write_bytes(b"")
        with mock.patch.object(rs, "build_outer_stages",
                               side_effect=stub_stages):
            rc = rs.main([
                "--execute", "build",
                "--plan", str(self.plan_path),
                "--selection", str(self.sel_path),
                "--attempt", str(self.root / "att-cli"),
            ])
        self.assertEqual(rc, rs.EXIT_OK)
        final_rec = load_json(self.root / "att-cli/result-final.json")
        self.assertEqual(final_rec["terminal"], "succeeded")
        self.assertEqual(final_rec["cleanupState"], "complete")

        # Also test actual /usr/bin/time -v -o <timeOutput> wrapping a child
        # python process running rs.main, proving time's precreated 0-byte
        # file is accepted at preexec and populated on exit.
        if Path("/usr/bin/time").is_file():
            import shutil, subprocess
            shutil.rmtree(self.outer["retained"], ignore_errors=True)
            shutil.rmtree(self.outer["evidence"], ignore_errors=True)
            (self.outer["compile_root"] / "ONE_SHOT_STARTED").unlink(
                missing_ok=True)
            self.outer["time_output"].unlink(missing_ok=True)
            wrapper_entry = self.root / "run_with_stub_stages.py"
            wrapper_entry.write_text(
                "import pathlib, sys\n"
                "from unittest import mock\n"
                f"sys.path.insert(0, {dw_dir!r})\n"
                "import run_selected as rs\n"
                "orig = rs.build_outer_stages\n"
                "def stub(sel, att):\n"
                "    st = orig(sel, att)\n"
                "    for n, s in st.items():\n"
                "        if n != 'native':\n"
                f"            s['argv'] = [sys.executable, {str(op_script)!r}, n, {str(meta_file)!r}]\n"
                "    return st\n"
                "with mock.patch.object(rs, 'build_outer_stages', side_effect=stub):\n"
                "    sys.exit(rs.main(sys.argv[1:]))\n"
            )
            proc = subprocess.run([
                "/usr/bin/time", "-v", "-o", str(self.outer["time_output"]),
                "--", sys.executable, str(wrapper_entry),
                "--execute", "build",
                "--plan", str(self.plan_path),
                "--selection", str(self.sel_path),
                "--attempt", str(self.root / "att-time-wrap"),
            ], capture_output=True, text=True, timeout=15)
            self.assertEqual(proc.returncode, 0, proc.stderr)
            self.assertGreater(self.outer["time_output"].stat().st_size, 0)
            preexec_w = load_json(self.root / "att-time-wrap/PREEXEC.json")
            self.assertEqual(
                preexec_w["outerTimeWrapper"]["timeOutputStateAtPreexec"],
                "precreated-zero-byte")

    def test_native_nonzero_with_packet_preserves_and_tears_down(self):
        inner = fx.outer_runner(self.outer)

        def runner(stage):
            rc = inner(stage)
            return 101 if stage["stage"] == "native" else rc
        code, record = self.go(runner)
        self.assertEqual(code, rs.EXIT_FAILED)
        self.assertTrue((self.outer["retained"] / "backend-elf").exists())
        self.assertIn("teardown", inner.calls)
        self.assertFalse((self.root / "att/consumer-result.json").exists())

    def test_native_unknown_never_promoted(self):
        inner = fx.outer_runner(self.outer)

        def runner(stage):
            rc = inner(stage)
            if stage["stage"] == "native":
                raise rs.StageUnknown("capture actual_exit=None")
            return rc
        code, record = self.go(runner)
        self.assertEqual(code, rs.EXIT_UNKNOWN)
        self.assertEqual(record["terminal"], "unknown-submission")
        self.assertFalse((self.root / "att/consumer-result.json").exists())
        self.assertIn("teardown", inner.calls)

    def test_collect_evidence_failure_blocks_collector_cleanup_owed(self):
        runner = fx.outer_runner(self.outer,
                                 failures={"collect-evidence": 2})
        code, record = self.go(runner)
        self.assertEqual(code, rs.EXIT_FAILED)
        self.assertIn("teardown", runner.calls)
        self.assertEqual(self.receipt()["cleanup"], "owed")
        self.assertFalse((self.root / "att/consumer-result.json").exists())

    def test_teardown_failure_result_carries_cleanup_debt(self):
        runner = fx.outer_runner(self.outer, failures={"teardown": 3})
        code, record = self.go(runner)
        result = load_json(self.root / "att/consumer-result.json")
        self.assertEqual(result["cleanup"], "owed")
        self.assertEqual(self.receipt()["restoration"], "owed")
        self.assertEqual(record["cleanupState"], "owed")

    def test_multiple_fresh_blocks_preservation_and_collector(self):
        runner = fx.outer_runner(
            self.outer, extra_fresh=("debug/deps/mori_backend-second",))
        code, _ = self.go(runner)
        self.assertEqual(code, rs.EXIT_FAILED)
        self.assertIn("collect-evidence", runner.calls)
        self.assertIn("teardown", runner.calls)
        self.assertFalse((self.root / "att/consumer-result.json").exists())
        self.assertIn("not unique",
                      self.ledger()["preservationDetail"])

    def test_marker_absent_uses_abort_not_teardown(self):
        runner = fx.outer_runner(self.outer, marker=False)
        code, _ = self.go(runner)
        self.assertEqual(code, rs.EXIT_FAILED)
        self.assertIn("abort-prelaunch-cleanup", runner.calls)
        self.assertNotIn("collect-evidence", runner.calls)
        self.assertNotIn("teardown", runner.calls)

    def test_malformed_counters_after_never_complete(self):
        inner = fx.outer_runner(self.outer)

        def runner(stage):
            rc = inner(stage)
            if stage["stage"] == "counters-after":
                Path(stage["stdout"]).write_bytes(b"garbage")
            return rc
        code, record = self.go(runner)
        self.assertNotEqual(self.receipt()["cleanup"], "complete")
        self.assertNotEqual(record["cleanupState"], "complete")

    def test_envelope_still_present_never_complete(self):
        runner = fx.outer_runner(self.outer)
        with mock.patch.object(rs, "_cgroup_absent", return_value=False):
            code, record = self.go(runner)
        self.assertEqual(self.receipt()["restoration"], "unknown")
        self.assertNotEqual(record["cleanupState"], "complete")

    def test_preservation_mapping_and_bytes(self):
        runner = fx.outer_runner(self.outer)
        code, _ = self.go(runner)
        self.assertEqual(code, rs.EXIT_OK)
        pres = load_json(self.root / "att/preservation.json")
        self.assertEqual(pres["actualRelative"], fx.NEW_ELF_REL)
        retained = Path(pres["retainedPath"]).read_bytes()
        source = Path(pres["sourcePath"]).read_bytes()
        self.assertEqual(retained, source)
        self.assertFalse(pres["executed"])

    def test_no_promotion_and_accounting_unknowns(self):
        runner = fx.outer_runner(self.outer)
        code, _ = self.go(runner)
        self.assertEqual(code, rs.EXIT_OK)
        result = load_json(self.root / "att/consumer-result.json")
        self.assertIsNone(result["review"])
        self.assertIsNone(result["adoption"])
        decoded = load_json(self.root / "att/native-receipt-decoded.json")
        self.assertTrue(decoded["acceptance"].startswith("UNACCEPTED"))
        self.assertIsNone(decoded["receipt"]["wrapperCUs"])
        obs = load_json(self.root / "att/outer-observations.json")
        self.assertIsNone(obs["scope"]["sudoPrefixCpuUs"])
        self.assertIsNone(obs["scope"]["exactAllDescendantCpuUs"])
        preexec = load_json(self.root / "att/PREEXEC.json")
        self.assertEqual(preexec["outerTimeWrapper"]["output"],
                         str(self.outer["time_output"]))
        self.assertFalse(self.outer["time_output"].exists(),
                         "factory must not write time's own output")


if __name__ == "__main__":
    unittest.main()
