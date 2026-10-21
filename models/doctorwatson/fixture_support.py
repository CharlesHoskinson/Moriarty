"""Public producer-shaped fixture builders for DoctorWatson tests.

Test/capture support only: no product module imports this file and no
execution premise may depend on it. Shapes are transcribed from the
actual producer sources, not invented: guard packet fields from the
native-compile-owner successor guard (bounded_sdk_compile.py 6694bbdd...)
and its pinned sibling helper (cached_current_build.py e439a2cd...);
settle result fields from the harness settleParent serializer with
assertReadbackAction / assertSuccessfulSubmitReceipt return values (not
the producer test's injected serialization stub). Values are public
placeholders; nothing here is native evidence.
"""
import copy
import hashlib
import json
from pathlib import Path

CELL_KEYS = (
    "instance_id", "keys_id", "a102_ctx_lo", "a102_ctx_hi", "a80_ctx_lo",
    "a80_ctx_hi", "program_c", "genesis_root", "root", "genesis_height",
    "genesis_batch", "height", "batch_no",
)
HUGE = (1 << 240) + 9007199254740993


def encoded(value):
    return (json.dumps(value, sort_keys=True, separators=(",", ":"),
                       allow_nan=False) + "\n").encode()


def pin(path: Path):
    data = Path(path).read_bytes()
    return {"path": str(path), "bytes": len(data),
            "sha256": hashlib.sha256(data).hexdigest()}


def write(path: Path, raw: bytes):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(raw)
    return path


FX_BASE = "03496bffd944f4b46c9d2997863f996f00c85017"
FX_LOCK = ("946b097d0fd0a7e1343e67f72846dee4b4676fa0a87676340f6d97"
           "cf571e3993")
FX_CANDIDATE = ("bec36a3f3fa522e9f8041b879a2357f7dcc0d9e9b8c7457770f"
                "44af2f17d410d")
FX_ADOPTION = ("8566624c17e6d7ea50ced5d4f3b699ce17112fe6d984b6a8b8"
               "bb4507a35fe35a")
FX_D_FREEZE = ("f8408c7ae5b5ccb495c129a9ba4a7a7a376ee9cb4b911b6f53a"
               "588b07cd839aa")
FX_E_RESOLVED = ("fd3db35861d3119f3ef925b682e8c53fcbd322478b7be531b92"
                 "41b9789f65e23")
FX_BOOKKEEPING_SHA = ("a0b64c884bf060b507ef48ee67ba124de2608e08068b9"
                      "18728b613ad6929b6a9")
FX_GIT_PROJ_SHA = ("a36fd57bd8f09dfa05a478584ba4010a6773204518c6dd0d"
                   "00357755633aa61c")
FX_COHORT_SHA = "33" * 32
FX_PRIOR_ESP_US = (2427120416, 316932500, 2110187910)
FX_PRIOR_RESIDUAL_US = 6
FX_NATIVE21_DELTAS_ESP_US = (400517240, 6931787, 393585452)
FX_RESERVE_AB_US = 4556857
FX_SUPERVISOR_FLOOR_S = 10373
FX_FLOORS_S = (4275, 465, FX_SUPERVISOR_FLOOR_S)
FX_CUMULATIVE_ESP_US = (
    FX_PRIOR_ESP_US[0] + FX_NATIVE21_DELTAS_ESP_US[0],
    FX_PRIOR_ESP_US[1] + FX_NATIVE21_DELTAS_ESP_US[1],
    FX_PRIOR_ESP_US[2] + FX_NATIVE21_DELTAS_ESP_US[2],
)
FX_EFFECTIVE_ESP_US = (
    FX_CUMULATIVE_ESP_US[0] + FX_RESERVE_AB_US,
    FX_CUMULATIVE_ESP_US[1] + FX_RESERVE_AB_US,
    FX_CUMULATIVE_ESP_US[2],
)
FX_HOLDBACK = 113
FX_ALLOWANCE = 10248
FX_TOTAL_WALL_S = 4162
FX_F_WALL_S = 352
HELPER_STUB_BYTES = (
    "# fixture helper bytes (never executed)\n"
    "PRESERVED_PACKET_PINS = {\n"
    "    'C_PREPARED.json': "
    "'f9c15946950c1cf2f0cf353dd2afac5c12b4f21c3cc30beaf21681c2c5048a20',\n"
    f"    'D_FREEZE.json': '{FX_D_FREEZE}',\n"
    f"    'E_RESOLVED.json': '{FX_E_RESOLVED}',\n"
    "    'D_CLEANUP.json': "
    "'41e078c9c261c61614c1dd8da7d2af32589328070f74263b060d295843fb16e0',\n"
    "    'E_CLEANUP.json': "
    "'6d635bbbd8352527d106170752ccac2a22526d6d7c1b30b278016867f5eb0783',\n"
    "}\n"
).encode()
FX_HELPER_SHA = hashlib.sha256(HELPER_STUB_BYTES).hexdigest()
OBSERVATION_BASIS = ("monotonic_ns + RUSAGE_SELF/CHILDREN at "
                     "cached_current_f entry and before result receipt; "
                     "outer Root process tail excluded (P-6 remains "
                     "Root record)")
NEW_ELF_REL = "debug/deps/mori_backend-9f8e7d6c5b4a3210"

FX_HISTORICAL_POST_R3_DEBITS = {
    "prior_post_r3_envelope_usec": 346198712,
    "prior_post_r3_supervisor_usec": 75915030,
    "prior_post_r3_phases_usec": 270283681,
    "retired_p2_p3_envelope_usec": 13455030,
    "retired_p2_p3_supervisor_usec": 13455030,
    "retired_p2_p3_phases_usec": 0,
    "retired_p4_envelope_usec": 13735290,
    "retired_p4_supervisor_usec": 13735290,
    "retired_p4_phases_usec": 0,
    "retired_p5_envelope_usec": 37863695,
    "retired_p5_supervisor_usec": 37863695,
    "retired_p5_phases_usec": 0,
    "retired_p6_envelope_usec": 39019876,
    "retired_p6_supervisor_usec": 39019876,
    "retired_p6_phases_usec": 0,
    "retired_p7_envelope_usec": 187121653,
    "retired_p7_supervisor_usec": 15162810,
    "retired_p7_phases_usec": 171958842,
    "retired_p8_envelope_usec": 198086463,
    "retired_p8_supervisor_usec": 12326620,
    "retired_p8_phases_usec": 185759842,
    "retired_p9_envelope_usec": 127323100,
    "retired_p9_supervisor_usec": 15815495,
    "retired_p9_phases_usec": 111507604,
    "retired_cached_attempts_envelope_usec": 616605107,
    "retired_cached_attempts_supervisor_usec": 147378816,
    "retired_cached_attempts_phases_usec": 469226288,
    "envelope_usec": 962803819,
    "supervisor_usec": 223293846,
    "supervisor_cpu_seconds_ceil": 224,
    "phases_usec": 739509969,
    "f_stdout_bytes": 0,
    "f_stderr_bytes": 43081,
    "r3_outer_wall_seconds_ceil": 24,
    "r3_inner_wall_seconds_ceil": 13,
    "prior_a0_to_e_wall_seconds_ceil": 40,
    "p2_refusal_wall_seconds_ceil": 5,
    "p3_refusal_wall_seconds_ceil": 11,
    "p4_refusal_wall_seconds_ceil": 15,
    "p5_refusal_wall_seconds_ceil": 40,
    "p6_refusal_wall_seconds_ceil": 44,
    "p7_outer_wall_seconds_ceil": 135,
    "p8_outer_wall_seconds_ceil": 133,
    "p9_outer_wall_seconds_ceil": 121,
    "remaining_total_wall_seconds_max": 6038,
    "remaining_f_wall_seconds_max": 2133,
    "remaining_supervisor_cpu_seconds_max": 10576,
    "gross_supervisor_cpu_seconds_max": 10800,
}

FX_POST_P10_DEBITS = {
    "prior_post_p9_envelope_usec": 962803819,
    "prior_post_p9_supervisor_usec": 223293846,
    "prior_post_p9_phases_usec": 739509969,
    "retired_p10_envelope_usec": 116543574,
    "retired_p10_supervisor_usec": 19626044,
    "retired_p10_phases_usec": 96917530,
    "envelope_usec": 1079347393,
    "supervisor_usec": 242919890,
    "supervisor_cpu_seconds_ceil": 243,
    "phases_usec": 836427499,
    "independent_envelope_minus_supervisor_minus_phases_usec": 4,
    "p10_f_stdout_bytes": 0,
    "p10_f_stderr_bytes": 16326,
    "f_stdout_bytes": 0,
    "f_stderr_bytes": 59407,
    "p10_prephase_outer_wall_seconds_ceil": 5,
    "p10_outer_wall_seconds_ceil": 98,
    "remaining_total_wall_seconds_max": 5935,
    "remaining_f_wall_seconds_max": 2035,
    "remaining_supervisor_cpu_seconds_max": 10557,
    "gross_supervisor_cpu_seconds_max": 10800,
}

FX_POST_P11_DEBITS = {
    "retired_p11_envelope_usec": 12906928,
    "retired_p11_supervisor_usec": 9308089,
    "retired_p11_phases_usec": 3598838,
    "retired_p11_independent_residual_usec": 1,
    "envelope_usec": 1092254321,
    "supervisor_usec": 252227979,
    "supervisor_cpu_seconds_ceil": 253,
    "phases_usec": 840026337,
    "independent_envelope_minus_supervisor_minus_phases_usec": 5,
    "p11_f_stdout_bytes": 0,
    "p11_f_stderr_bytes": 112,
    "p11_outer_controller_stderr_bytes_separate": 39,
    "f_stdout_bytes": 0,
    "f_stderr_bytes": 59519,
    "p11_outer_wall_seconds_ceil": 14,
    "remaining_total_wall_seconds_max": 5921,
    "remaining_f_wall_seconds_max": 2021,
    "remaining_supervisor_cpu_seconds_max": 10547,
    "gross_supervisor_cpu_seconds_max": 10800,
}


def identity(root: Path, outer: dict | None = None):
    source = pin(write(root / "source.py", b"public source\n"))
    tool = pin(write(root / "tool", b"public tool\n"))
    config = pin(write(root / "config.json", b"{}\n"))
    if outer is None:
        return {"integration_base": "current-source-cut",
                "source": [source], "tools": [tool], "config": [config],
                "oracle_sha256": "a" * 64}
    return {
        "integration_base": "current-source-cut",
        "source": [
            source,
            pin(outer["guard"]),
            pin(outer["helper"]),
            pin(outer["operator"]),
            pin(outer["capture"]),
        ],
        "tools": [
            tool,
            pin(outer["time_tool"]),
        ],
        "config": [
            config,
            pin(outer["config_path"]),
            pin(outer["request_path"]),
        ],
        "oracle_sha256": "a" * 64,
    }


def task(root, name, adapter, inputs=None, attempt=None, deps=None,
         disposition="active", outer=None):
    if outer is None and adapter == "moriarty-sdk-build":
        outer = outer_fixture(root)
    return {"id": name, "owner": "Owner", "adapter": adapter,
            "attempt": attempt or (name + "-001"),
            "disposition": disposition, "launch_state": "not-started",
            "identity": identity(root, outer), "inputs": inputs or {},
            "dependencies": deps or [], "eligible_reviewers": ["Reviewer"]}


def plan_file(root: Path, tasks):
    path = root / "plan.json"
    path.write_bytes(encoded({"schema": "doctorwatson.delivery-plan/1",
                              "integrator": "Root", "tasks": tasks}))
    return path


def cleanup_receipt(root: Path, name, task_id, attempt, fingerprint,
                    cleanup="complete", restoration="complete"):
    return write(root / name, encoded({
        "schema": "doctorwatson.cleanup-observation/1", "task": task_id,
        "attempt": attempt, "fingerprint": fingerprint,
        "cleanup": cleanup, "restoration": restoration}))


def wrapper_observation(monotonic_ns, self_usec=100, children_usec=200):
    return {"monotonic_ns": monotonic_ns, "self_usec": self_usec,
            "children_usec": children_usec}


def self_observation(start_ns=1_000_000, end_ns=4_500_000,
                     null_rusage=False):
    pair = (None, None) if null_rusage else (100, 200)
    pair_end = (None, None) if null_rusage else (300, 900)
    return {
        "start_at_cached_current_f_entry": wrapper_observation(
            start_ns, pair[0], pair[1]),
        "end_before_result_receipt": wrapper_observation(
            end_ns, pair_end[0], pair_end[1]),
        "duration_monotonic_ns": end_ns - start_ns,
        "basis": OBSERVATION_BASIS, "tail_after_receipt_excluded": True}


def debit_state(route="policy", live=(10, 20, 30)):
    """Typed enforce_current_resource_disposition-shaped state."""
    if route == "policy":
        disp = {"disposition_kind": "CARRIED_UNKNOWN_POLICY_SPEND",
                "cumulative_before_wrapper_E_us": FX_CUMULATIVE_ESP_US[0],
                "cumulative_before_wrapper_S_us": FX_CUMULATIVE_ESP_US[1],
                "cumulative_before_wrapper_P_us": FX_CUMULATIVE_ESP_US[2],
                "effective_prior_E_us": FX_EFFECTIVE_ESP_US[0],
                "effective_prior_S_us": FX_EFFECTIVE_ESP_US[1],
                "effective_prior_P_us": FX_EFFECTIVE_ESP_US[2],
                "wrapper_c_us": None,
                "exact_remaining_supervisor_s": None,
                "policy_holdback_s": FX_HOLDBACK,
                "policy_supervisor_allowance_s": FX_ALLOWANCE,
                "policy_remaining_total_wall_max_s": FX_TOTAL_WALL_S,
                "policy_remaining_f_wall_max_s": FX_F_WALL_S}
        wrapper_c = None
    else:
        wrapper_c = 123
        disp = {"cumulative_before_wrapper_E_us": FX_CUMULATIVE_ESP_US[0],
                "cumulative_before_wrapper_S_us": FX_CUMULATIVE_ESP_US[1],
                "cumulative_before_wrapper_P_us": FX_CUMULATIVE_ESP_US[2],
                "effective_prior_E_us": FX_EFFECTIVE_ESP_US[0] + wrapper_c,
                "effective_prior_S_us": FX_EFFECTIVE_ESP_US[1] + wrapper_c,
                "effective_prior_P_us": FX_EFFECTIVE_ESP_US[2],
                "remaining_supervisor_cpu_seconds": 10361,
                "exact_remaining_supervisor_s": 10361,
                "supervisor_spend_ceiling_s": 10361,
                "disposition_kind": "ESTABLISHED_PINNED"}
    return {
        "historical_debits": copy.deepcopy(FX_HISTORICAL_POST_R3_DEBITS),
        "post_p10_debits": copy.deepcopy(FX_POST_P10_DEBITS),
        "post_p11_debits": copy.deepcopy(FX_POST_P11_DEBITS),
        "current_disposition": disp,
        "wrapper_c_us": wrapper_c,
        "live_envelope_usec_at_start": live[0],
        "live_supervisor_usec_at_start": live[1],
        "live_phases_usec_at_start": live[2],
        "effective_envelope_usec_at_start":
            disp["effective_prior_E_us"] + live[0],
        "effective_supervisor_usec_at_start":
            disp["effective_prior_S_us"] + live[1],
        "effective_phases_usec_at_start":
            disp["effective_prior_P_us"] + live[2],
    }


def source_home_reconstitution():
    """Authentic helper-shaped SOURCE_HOME_RECONSTITUTION report."""
    ok_sha = hashlib.sha256(b'{"v":1}').hexdigest()
    return {
        "preserved_d_freeze_sha256": FX_D_FREEZE,
        "new_d_freeze_sha256": "44" * 32,
        "preserved_h_s": "11" * 32,
        "new_h_s": "22" * 32,
        "authenticated_source_files_verified": 4500,
        "expected_generated_files_verified": 25,
        "reconstructed_cargo_ok_count": 1,
        "reconstructed_cargo_ok": [
            {"relative": "registry/src/index/.cargo-ok",
             "bytes": 7, "sha256": ok_sha},
        ],
        "bookkeeping_hash_check_sha256": FX_BOOKKEEPING_SHA,
        "git_control_snapshot_projection_sha256": FX_GIT_PROJ_SHA,
        "dated_bookkeeping_deltas": [
            {
                "relative": ".global-cache",
                "kind": "cargo-lockfile",
                "historical_d_sha256": (
                    "f7138d717dcd66b67a55a88c41e2e80b7c1ed9c60e601a6b"
                    "ad1b2f0de42d6447"),
                "current_sha256": (
                    "390ffa709f11c02f40e6484d8d58dd009dd23f572ab29875"
                    "41b41068692154fa"),
                "bytes": 356352,
                "bookkeeping_check_sha256": FX_BOOKKEEPING_SHA,
                "immutable_snapshot_path":
                    "/fixture/global-cache.immutable.snapshot",
            },
        ],
        "source_epoch_ns": 1_760_000_000_000_000_000,
        "frozen_index_cohort": {
            "manifest_sha256": FX_COHORT_SHA,
            "file_count": 120,
            "index_destinations_staged_from_cohort": 120,
        },
        "current_host_units_remapped": 56,
        "preserved_e_resolved_sha256": FX_E_RESOLVED,
    }


def cached_target_seed_counts():
    """Authentic helper-shaped seed_snapshot['partition_counts']."""
    return {
        "owner_literal_workspace_fingerprint": 22,
        "owner_literal_workspace_deps": 22,
        "owner_literal_workspace_incremental": 669,
        "unadopted_other_cache_candidate": 2843,
    }


def guard_launch_process_argv(total_len: int, sep_index: int,
                              mode: str) -> list[str]:
    """Construct guard.launch-shaped argv: bwrap_opts + ['--', *cmd]."""
    bwrap = [
        "--die-with-parent", "--new-session", "--unshare-user",
        "--unshare-pid", "--unshare-net", "--unshare-ipc",
        "--unshare-uts", "--disable-userns", "--cap-drop", "ALL",
        "--ro-bind", "/", "/",
    ]
    while len(bwrap) + 4 <= sep_index:
        idx = len(bwrap)
        bwrap.extend(["--ro-bind", f"/fixture/ro/{idx}",
                      f"/fixture/ro/{idx}"])
    rem = sep_index - 1 - len(bwrap)
    if rem == 2:
        bwrap.extend(["--chdir", "/fixture/wt"])
    elif rem == 1:
        bwrap.append("--clearenv")
    bwrap.append("--as-pid-1")
    assert len(bwrap) == sep_index
    cmd = ["/usr/bin/python3", "-I", "-B", "-S", "/proc/self/fd/3", mode]
    while len(bwrap) + 1 + len(cmd) < total_len:
        cmd.append(f"--arg-{len(cmd)}")
    argv = bwrap + ["--"] + cmd
    assert len(argv) == total_len and argv[sep_index] == "--"
    return argv


def guard_launch_processes(shape: str = "p10") -> list[dict]:
    """Return 3 Stage F guard.launch process dicts matching P10 or P9 shape."""
    first_len, first_sep = (230, 208) if shape == "p9" else (236, 214)
    specs = (
        (first_len, first_sep, "build", True),
        (126, 109, "audit_verify", False),
        (205, 184, "smoke_test", True),
    )
    return [
        {
            "argv": guard_launch_process_argv(tlen, sep, mode),
            "audit_worker": audit_worker,
            "child_exit": 0,
        }
        for tlen, sep, mode, audit_worker in specs
    ]


def f_result_doc(check, route="policy", null_rusage=False, paths=None,
                 proc_shape="p10"):
    env_paths = paths or {
        "source_home": "/fixture/compile07/source-home",
        "empty_home": "/fixture/compile07/empty-home",
        "tmp": "/fixture/compile07/tmp",
        "target": "/fixture/compile07/target",
    }
    f_env = {
        "PATH": "/usr/bin:/bin",
        "CARGO_HOME": env_paths["source_home"],
        "HOME": env_paths["empty_home"],
        "TMPDIR": env_paths["tmp"],
        "LANG": "C",
        "CARGO_TARGET_DIR": env_paths["target"],
    }
    doc = {
        "phase": "F", "route": "cached-current-f",
        "base_head": FX_BASE, "cargo_lock_sha256": FX_LOCK,
        "historical_debit_state_start": debit_state(route),
        "historical_debit_state_end": debit_state(route, (40, 50, 60)),
        "supervisor_self_observation": self_observation(
            null_rusage=null_rusage),
        "source_home_reconstitution": source_home_reconstitution(),
        "cached_target_seed_counts": cached_target_seed_counts(),
        "post_compile_check": check,
        "f_result": {
            "phase": "F",
            "processes": guard_launch_processes(proc_shape),
            "environment": f_env,
            "wall_seconds": 1.5,
            "log_bytes": {"stdout": 15, "stderr": 0},
        },
    }
    if route == "policy":
        doc.update({
            "supervisorCeilingKind":
                "POLICY_ALLOWANCE_NOT_REMAINING_BALANCE",
            "dispositionKind": "CARRIED_UNKNOWN_POLICY_SPEND",
            "policy_candidate_sha256": FX_CANDIDATE,
            "policy_adoption_sha256": FX_ADOPTION,
            "policy_holdback_s_withheld": FX_HOLDBACK})
    return doc


def write_packet(packet: Path, target: Path, elf_rel=NEW_ELF_REL,
                 route="policy", elf_bytes=b"\x7fELF public fixture\n",
                 check_mutator=None, doc_mutator=None,
                 extra_fresh=()):
    """Write current-helper-shaped packet + target ELF under given dirs."""
    elf_path = write(target / elf_rel, elf_bytes)
    elf_sha = pin(elf_path)["sha256"]
    fresh = [{"relative": elf_rel, "sha256": elf_sha,
              "bytes": elf_path.stat().st_size}]
    all_outputs = {elf_rel: elf_sha}
    for rel in extra_fresh:
        extra = write(target / rel, b"\x7fELF second fixture\n")
        fresh.append({"relative": rel, "sha256": pin(extra)["sha256"],
                      "bytes": extra.stat().st_size})
        all_outputs[rel] = pin(extra)["sha256"]
    check = {"host_outputs_verified_unchanged": 3,
             "build_script_run_units_verified_not_reexecuted": 2,
             "fresh_mori_backend_executables": fresh}
    if check_mutator:
        check_mutator(check)
    write(packet / "CACHED_POST_COMPILE_CHECK.json", encoded(check))
    write(packet / "F_QUARANTINED_OUTPUTS.json", encoded({
        "all_outputs": all_outputs, "host_outputs": [],
        "host_fingerprints": [], "build_script_outputs": [],
        "acceptance": "UNACCEPTED: independent result review required; "
                      "no artifact executed"}))
    compile_root = target.parent
    paths = {
        "source_home": str(compile_root / "source-home"),
        "empty_home": str(compile_root / "empty-home"),
        "tmp": str(compile_root / "tmp"),
        "target": str(target),
    }
    doc = f_result_doc(check, route, paths=paths)
    if doc_mutator:
        doc_mutator(doc)
    write(packet / "F_CACHED_CURRENT_RESULT.json", encoded(doc))
    write(packet / "F.stdout", b"fixture stdout\n")
    write(packet / "F.stderr", b"")
    return elf_path


def guard_packet(root: Path, elf_rel=NEW_ELF_REL, route="policy"):
    """Write guard-shaped packet + target ELF; return raw output list.

    resource-receipt path is declared but written by the caller (it is
    bound to task/attempt/fingerprint by the outer operator).
    """
    packet = root / "packet"
    target = root / "target"
    elf_path = write_packet(packet, target, elf_rel, route)
    return [
        {"role": "f-result",
        "path": str(packet / "F_CACHED_CURRENT_RESULT.json")},
        {"role": "f-outputs",
         "path": str(packet / "F_QUARANTINED_OUTPUTS.json")},
        {"role": "post-compile-check",
         "path": str(packet / "CACHED_POST_COMPILE_CHECK.json")},
        {"role": "stdout", "path": str(packet / "F.stdout")},
        {"role": "stderr", "path": str(packet / "F.stderr")},
        {"role": "backend-elf", "path": str(elf_path)},
        {"role": "resource-receipt", "path": str(packet / "cleanup.json")},
    ]


def guard_stub(root: Path, helper_sha: str = FX_HELPER_SHA):
    return write(root / "bounded_sdk_compile.py", (
        "# fixture guard bytes (never executed)\n"
        f"BASE = '{FX_BASE}'\n"
        f"LOCK = '{FX_LOCK}'\n"
        f"CANDIDATE_DISPOSITION_SHA256 = '{FX_CANDIDATE}'\n"
        f"CACHED_CURRENT_BUILD_SHA256 = '{helper_sha}'\n"
        f"EXPECTED_PRIOR_ESP_US = {FX_PRIOR_ESP_US!r}\n"
        f"EXPECTED_PRIOR_RESIDUAL_US = {FX_PRIOR_RESIDUAL_US}\n"
        f"EXPECTED_NATIVE21_DELTAS_ESP_US = {FX_NATIVE21_DELTAS_ESP_US!r}\n"
        f"EXPECTED_RESERVE_AB_US = {FX_RESERVE_AB_US}\n"
        f"EXPECTED_FLOORS_S = {FX_FLOORS_S!r}\n"
        f"POLICY_HOLDBACK_S = {FX_HOLDBACK}\n"
        f"POLICY_SUPERVISOR_ALLOWANCE_S = {FX_ALLOWANCE}\n"
        f"POLICY_TOTAL_WALL_MAX_S = {FX_TOTAL_WALL_S}\n"
        f"POLICY_F_WALL_MAX_S = {FX_F_WALL_S}\n").encode())


def outer_fixture(root: Path, route="policy"):
    """Pinned outer sources + pre-provisioned host07-shaped fixture state.

    Models an already-provisioned host where OP0..OP6 have already
    completed prior to final config issuance: compile_root + its 6
    subdirectories and cfg_dir/Q-OBS.fixture.json already exist, while
    ONE_SHOT_STARTED, evidence_dir, retainedDir, and time_output are
    absent. Nothing is executed.
    """
    compile_root = root / "compile07"
    cfg_dir = root / "cfg07"
    evidence = root / "evidence07"
    retained = root / "retained07"
    worktree = root / "wt"
    worktree.mkdir(parents=True, exist_ok=True)
    cfg_dir.mkdir(parents=True, exist_ok=True)
    for sub in ("packet", "target", "tmp", "source-home",
                "empty-home", "mask-empty"):
        (compile_root / sub).mkdir(parents=True, exist_ok=True)
    helper = write(root / "cached_current_build.py", HELPER_STUB_BYTES)
    guard = guard_stub(root, helper_sha=pin(helper)["sha256"])
    operator = write(root / "operator_provisioner.py",
                     b"# fixture operator bytes (never executed)\n")
    capture = write(root / "run_tail_four.py",
                    b"# fixture capture bytes (never executed)\n"
                    b"def capture(op, phase):\n    raise AssertionError\n")
    time_tool = Path("/usr/bin/time")
    if not time_tool.is_file():
        time_tool = write(root / "time", b"fixture time tool\n")
    paths_cfg = {
        "execution_root": str(compile_root),
        "packet": str(compile_root / "packet"),
        "target": str(compile_root / "target"),
        "tmp": str(compile_root / "tmp"),
        "source_home": str(compile_root / "source-home"),
        "empty_home": str(compile_root / "empty-home"),
        "mask_empty": str(compile_root / "mask-empty"),
        "worktree": str(worktree),
        "config": str(cfg_dir / "config.final.json")}
    base_rd = {
        "adopted_prior_E_us": FX_PRIOR_ESP_US[0],
        "adopted_prior_S_us": FX_PRIOR_ESP_US[1],
        "adopted_prior_P_us": FX_PRIOR_ESP_US[2],
        "adopted_prior_residual_us": FX_PRIOR_RESIDUAL_US,
        "native21_delta_E_us": FX_NATIVE21_DELTAS_ESP_US[0],
        "native21_delta_S_us": FX_NATIVE21_DELTAS_ESP_US[1],
        "native21_delta_P_us": FX_NATIVE21_DELTAS_ESP_US[2],
        "reserve_ab_us": FX_RESERVE_AB_US,
        "supervisor_floor_before_native21_s": FX_SUPERVISOR_FLOOR_S,
    }
    if route == "policy":
        rd = {
            **base_rd,
            "wrapper_c_status": "CARRIED_UNKNOWN_POLICY_SPEND",
            "wrapper_c_us": None,
            "wrapper_c_basis": None,
            "policy_adoption": {"path": str(cfg_dir / "adoption.json"),
                                "sha256": FX_ADOPTION},
            "policy_holdback_s": FX_HOLDBACK,
            "policy_supervisor_allowance_s": FX_ALLOWANCE,
            "remaining_total_wall_seconds_max": FX_TOTAL_WALL_S,
            "remaining_f_wall_seconds_max": FX_F_WALL_S,
        }
    else:
        rd = {
            **base_rd,
            "wrapper_c_status": "ESTABLISHED_PINNED",
            "wrapper_c_us": 123,
            "wrapper_c_basis": "fixture-established-basis",
            "remaining_total_wall_seconds_max": FX_TOTAL_WALL_S,
            "remaining_f_wall_seconds_max": FX_F_WALL_S,
        }
    config = {
        "mode": "tier1-and-compile", "version": 1,
        "cgroup": {"envelope_path": "/moriarty-fixture-envelope07",
                   "supervisor_leaf_name": "supervisor",
                   "phase_parent_name": "phases",
                   "envelope_memory_max": 21474836480,
                   "envelope_pids_max": 1088,
                   "envelope_cpu_max": "500000 100000"},
        "paths": paths_cfg,
        "bindings": {"runner": pin(guard)},
        "resource_disposition": rd,
        "total_wall_seconds": FX_TOTAL_WALL_S,
        "supervisor_cpu_seconds": 1000,
        "limits": {"F": {"wall_seconds": FX_F_WALL_S}}}
    config_path = write(cfg_dir / "config.final.json", encoded(config))
    qobs_path = write(cfg_dir / "Q-OBS.fixture.json", encoded({
        "schema_version": 1,
        "envelope_path": "/moriarty-fixture-envelope07",
        "mountpoints": [
            paths_cfg["execution_root"],
            paths_cfg["source_home"],
            paths_cfg["packet"],
        ],
        "provisioned": True,
    }))
    request = {
        "schema_version": 1, "operator_uid": 1000, "operator_gid": 1000,
        "cgroup": {"envelope_path": "/moriarty-fixture-envelope07",
                   "supervisor_leaf_name": "supervisor",
                   "phase_parent_name": "phases",
                   "envelope_memory_max": 21474836480,
                   "envelope_pids_max": 1088,
                   "envelope_cpu_max": "500000 100000",
                   "supervisor_leaf_limits": {
                       "cpu_max": "100000 100000",
                       "memory_max": 1610612736, "pids_max": 64},
                   "phase_parent_limits": {
                       "cpu_max": "400000 100000",
                       "memory_max": 18534076416, "pids_max": 1024}},
        "paths": {
            "execution_root": paths_cfg["execution_root"],
            "packet": paths_cfg["packet"],
            "target": paths_cfg["target"],
            "tmp": paths_cfg["tmp"],
            "source_home": paths_cfg["source_home"],
            "empty_home": paths_cfg["empty_home"],
            "mask_empty": paths_cfg["mask_empty"],
            "config_dir": str(cfg_dir),
            "evidence_dir": str(evidence),
            "observation_output": str(qobs_path)},
        "quota_mounts": {}}
    request_path = write(root / "request.current-compile.json",
                         encoded(request))
    host_state = {
        "provisioned": True,
        "cgroup_active": True,
        "mounts_active": [
            paths_cfg["execution_root"],
            paths_cfg["source_home"],
            paths_cfg["packet"],
        ],
        "aborted_or_torn_down": False,
    }
    return {"compile_root": compile_root, "cfg_dir": cfg_dir,
            "qobs_path": qobs_path, "evidence": evidence,
            "retained": retained, "worktree": worktree, "guard": guard,
            "helper": helper, "operator": operator, "capture": capture,
            "time_tool": time_tool, "config_path": config_path,
            "request_path": request_path, "route": route,
            "time_output": root / "whole-sequence-time.txt",
            "host_state": host_state}


def selection_dict(fx_outer, fingerprint, task_id="build",
                   adapter="moriarty-sdk-build"):
    evidence = fx_outer["evidence"]
    retained = fx_outer["retained"]
    outputs = [
        {"role": "f-result",
         "path": str(evidence / "packet/F_CACHED_CURRENT_RESULT.json")},
        {"role": "f-outputs",
         "path": str(evidence / "packet/F_QUARANTINED_OUTPUTS.json")},
        {"role": "post-compile-check",
         "path": str(evidence / "packet/CACHED_POST_COMPILE_CHECK.json")},
        {"role": "stdout", "path": str(evidence / "packet/F.stdout")},
        {"role": "stderr", "path": str(evidence / "packet/F.stderr")},
        {"role": "backend-elf", "path": str(retained / "backend-elf")},
        {"role": "resource-receipt", "path": str(retained / "cleanup.json")},
    ]
    return {"schema": "doctorwatson.execution-selection/2",
            "task": task_id, "adapter": adapter,
            "fingerprint": fingerprint,
            "outerSequence": "bounded-sdk-compile-cached-current-f/2",
            "guard": pin(fx_outer["guard"]),
            "config": pin(fx_outer["config_path"]),
            "outer": {
                "operator": pin(fx_outer["operator"]),
                "operatorRequest": pin(fx_outer["request_path"]),
                "capture": pin(fx_outer["capture"]),
                "helper": pin(fx_outer["helper"]),
                "retainedDir": str(retained),
                "timeOutput": str(fx_outer["time_output"]),
                "timeTool": pin(fx_outer["time_tool"])},
            "outputs": outputs}


def counters_json(monotonic_ns=123456789):
    node = {"path": "/sys/fs/cgroup/fixture",
            "raw": {"cpu.stat": "usage_usec 42\n",
                    "memory.events": "max 0\n", "pids.events": "max 0\n",
                    "cgroup.procs": "", "cgroup.events": "populated 0\n"}}
    return encoded({"monotonic_ns": monotonic_ns, "observer_pid": 999,
                    "nodes": {"E": node, "S": node, "P": node}})


def outer_runner(fx_outer, failures=None, marker=True, elf_rel=NEW_ELF_REL,
                 write_outputs=True, doc_mutator=None, extra_fresh=()):
    """Mock stage runner; simulates operator/counters/native/evidence.

    Enforces real operator_provisioner.py absence/presence predicates:
    - op0..op6 refuse (exit 2) when config_dir, execution_root, or Q-OBS
      already exists on the provisioned host (so source07 fails red).
    - prelaunch-verify requires execution_root, config_dir, and Q-OBS to
      exist and ONE_SHOT_STARTED/evidence_dir to be absent.
    - teardown / abort-prelaunch-cleanup mark host_state torn down.
    """
    failures = failures or {}
    calls = []
    compile_root = fx_outer["compile_root"]
    cfg_dir = fx_outer["cfg_dir"]
    qobs_path = fx_outer.get("qobs_path", cfg_dir / "Q-OBS.fixture.json")
    host_state = fx_outer.get("host_state", {
        "provisioned": True, "cgroup_active": True,
        "mounts_active": [str(compile_root)],
        "aborted_or_torn_down": False,
    })

    def run(stage):
        calls.append(stage["stage"])
        name = stage["stage"]
        if name in failures:
            bad = failures[name]
            if isinstance(bad, Exception):
                raise bad
            return bad
        if name in ("op0", "op1", "op2", "op3", "op4", "op5", "op6"):
            # operator_provisioner.py cmd_op0/cmd_op1/cmd_op6 require
            # execution_root, config_dir, and observation_output absent.
            if (compile_root.exists() or cfg_dir.exists()
                    or qobs_path.exists()):
                return 2
            return 0
        if name == "prelaunch-verify":
            if (not compile_root.is_dir() or not cfg_dir.is_dir()
                    or not qobs_path.is_file()
                    or (compile_root / "ONE_SHOT_STARTED").exists()
                    or fx_outer["evidence"].exists()
                    or not host_state.get("provisioned", True)
                    or not host_state.get("cgroup_active", True)
                    or not host_state.get("mounts_active")):
                return 2
            return 0
        if name.startswith("counters-"):
            Path(stage["stdout"]).write_bytes(counters_json())
            return 0
        if name == "native":
            if write_outputs:
                for sub in ("packet", "target", "tmp", "source-home",
                            "empty-home", "mask-empty"):
                    (compile_root / sub).mkdir(parents=True,
                                                exist_ok=True)
                if marker:
                    write(compile_root / "ONE_SHOT_STARTED", b"no retry\n")
                write_packet(compile_root / "packet",
                             compile_root / "target", elf_rel,
                             fx_outer["route"], doc_mutator=doc_mutator,
                             extra_fresh=extra_fresh)
            return 0
        if name == "collect-evidence":
            evidence = fx_outer["evidence"]
            (evidence / "packet").mkdir(parents=True)
            files = {}
            marker_src = compile_root / "ONE_SHOT_STARTED"
            write(evidence / "ONE_SHOT_STARTED", marker_src.read_bytes())
            files["ONE_SHOT_STARTED"] = pin(
                evidence / "ONE_SHOT_STARTED")["sha256"]
            for item in sorted((compile_root / "packet").iterdir()):
                write(evidence / "packet" / item.name, item.read_bytes())
                files["packet/" + item.name] = pin(
                    evidence / "packet" / item.name)["sha256"]
            write(evidence / "EVIDENCE_SHA256_MANIFEST.json", encoded({
                "request_sha256": pin(fx_outer["request_path"])["sha256"],
                "provisioner_sha256": pin(fx_outer["operator"])["sha256"],
                "observation_sha256": pin(qobs_path)["sha256"],
                "files": files}))
            return 0
        if name in ("teardown", "abort-prelaunch-cleanup"):
            host_state["cgroup_active"] = False
            host_state["mounts_active"] = []
            host_state["aborted_or_torn_down"] = True
            return 0
        return 0

    run.calls = calls
    run.host_state = host_state
    return run


def public_observation(cells, validity_has, validity_size, settled_has,
                       settled_size):
    return {"cells": {k: str(cells[k]) for k in CELL_KEYS},
            "validity_has_stmt": validity_has,
            "validity_size": validity_size,
            "settled_has_stmt": settled_has,
            "settled_size": settled_size, "balance_empty": True}


FIXTURE_ADDRESS = "aa" * 32  # even-length lowercase hex, normAddress-shaped
FIXTURE_TX = "bb" * 32       # 64 lowercase hex, normTxHash-shaped


def settle_result(expected_root=None, batch_no=7, network="stagenet",
                  address=FIXTURE_ADDRESS, tx_hash=FIXTURE_TX):
    """Harness-shaped settle result with pre validity 8 -> post 7."""
    cells = {k: HUGE + i for i, k in enumerate(CELL_KEYS)}
    cells["batch_no"] = batch_no
    cells["height"] = 1000
    root_value = HUGE + 99 if expected_root is None else int(expected_root)
    pre = public_observation(cells, True, 8, False, 11)
    post_cells = dict(cells, root=root_value, height=1002,
                      batch_no=batch_no + 1)
    post = public_observation(post_cells, False, 7, True, 12)
    baseline = public_observation(cells, False, 7, False, 11)
    return {
        "schema": "sherlock-settle-result/1",
        "observation_schema": "sherlock-chain-observations/1",
        "phase": "SETTLED_FINALIZED_READBACK_OK", "env": network,
        "network_id": "fixture-network-id", "address": address,
        "instance_id": "11" * 32, "batch_no": str(batch_no),
        "next_batch_no": str(batch_no + 1), "stmt": "22" * 32,
        "settle_tx_hash": tx_hash, "transaction_sha256": "d" * 64,
        "receipt": {"status": "SUCCESS", "finalized": True,
                    "bytes_match": True, "hash": tx_hash},
        "receipt_check": {"tx_hash": tx_hash, "receipt_hash": tx_hash,
                          "transaction_sha256": "d" * 64,
                          "status": "SUCCESS", "finalized": True,
                          "bytes_match": True},
        "action_check": {"address": address, "tx_hash": tx_hash,
                         "unshielded_balances": "empty"},
        "pre_observation": pre, "post_observation": post,
        "settle_assertion_baseline": {
            "kind": "derived-before-accept-baseline",
            "derivation": "pre_observation with validity_has_stmt=false, "
                          "validity_size-1, settled_has_stmt=false, "
                          "balance_empty=true",
            "observation": baseline},
        "expected_root": str(root_value),
        "pre_cells": dict(pre["cells"]), "post_cells": dict(post["cells"]),
        "pre_state_sha256": "e" * 64, "post_state_sha256": "f" * 64,
        "p6": "fixture-scope"}


def settle_inputs(root_value=str(HUGE + 99), batch=7):
    return {"network": "stagenet", "anchor": FIXTURE_ADDRESS,
            "deployment_sha256": "b" * 64, "binding_sha256": "c" * 64,
            "batch": batch, "expected_root": root_value}
