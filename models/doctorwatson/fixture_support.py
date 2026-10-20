"""Public producer-shaped fixture builders for DoctorWatson tests.

Test/capture support only: no product module imports this file and no
execution premise may depend on it. Shapes are transcribed from the
actual producer sources, not invented: guard packet fields from the
native-compile-owner successor guard; settle result fields from the
harness settleParent serializer with assertReadbackAction /
assertSuccessfulSubmitReceipt return values (not the producer test's
injected serialization stub). Values are public placeholders; nothing
here is native evidence.
"""
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


def identity(root: Path):
    source = pin(write(root / "source.py", b"public source\n"))
    tool = pin(write(root / "tool", b"public tool\n"))
    config = pin(write(root / "config.json", b"{}\n"))
    return {"integration_base": "current-source-cut", "source": [source],
            "tools": [tool], "config": [config], "oracle_sha256": "a" * 64}


def task(root, name, adapter, inputs=None, attempt=None, deps=None,
         disposition="active"):
    return {"id": name, "owner": "Owner", "adapter": adapter,
            "attempt": attempt or (name + "-001"),
            "disposition": disposition, "launch_state": "not-started",
            "identity": identity(root), "inputs": inputs or {},
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


def guard_packet(root: Path, elf_rel="debug/deps/mori_backend-deadbeef01"):
    """Write guard-shaped packet + target ELF; return raw output list.

    resource-receipt path is declared but written by the caller (it is
    bound to task/attempt/fingerprint by the outer operator).
    """
    packet = root / "packet"
    target = root / "target"
    elf_path = write(target / elf_rel, b"\x7fELF public fixture bytes\n")
    elf_sha = pin(elf_path)["sha256"]
    check = {"host_outputs_verified_unchanged": 3,
             "build_script_run_units_verified_not_reexecuted": 2,
             "fresh_mori_backend_executables": [
                 {"relative": elf_rel, "sha256": elf_sha,
                  "bytes": elf_path.stat().st_size}]}
    write(packet / "CACHED_POST_COMPILE_CHECK.json", encoded(check))
    write(packet / "F_QUARANTINED_OUTPUTS.json", encoded({
        "all_outputs": {elf_rel: elf_sha}, "host_outputs": [],
        "host_fingerprints": [], "build_script_outputs": [],
        "acceptance": "UNACCEPTED: independent result review required; "
                      "no artifact executed"}))
    write(packet / "F_CACHED_CURRENT_RESULT.json", encoded({
        "phase": "F", "route": "cached-current-f",
        "base_head": "b" * 40, "cargo_lock_sha256": "c" * 64,
        "historical_debit_state_start": {"c": "unknown-fixture"},
        "historical_debit_state_end": {"c": "unknown-fixture"},
        "source_home_reconstitution": {"files": 1},
        "cached_target_seed_counts": {"seeded": 1},
        "post_compile_check": check,
        "f_result": {"phase": "F",
                     "processes": [{"argv": ["cargo-fixture"],
                                    "audit_worker": False,
                                    "child_exit": 0}],
                     "environment": {}, "wall_seconds": 1.5,
                     "log_bytes": {"stdout": 12, "stderr": 0}}}))
    write(packet / "F.stdout", b"fixture stdout\n")
    write(packet / "F.stderr", b"")
    return [
        {"role": "f-result", "path": str(packet / "F_CACHED_CURRENT_RESULT.json")},
        {"role": "f-outputs", "path": str(packet / "F_QUARANTINED_OUTPUTS.json")},
        {"role": "post-compile-check",
         "path": str(packet / "CACHED_POST_COMPILE_CHECK.json")},
        {"role": "stdout", "path": str(packet / "F.stdout")},
        {"role": "stderr", "path": str(packet / "F.stderr")},
        {"role": "backend-elf", "path": str(elf_path)},
        {"role": "resource-receipt", "path": str(packet / "cleanup.json")},
    ]


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
