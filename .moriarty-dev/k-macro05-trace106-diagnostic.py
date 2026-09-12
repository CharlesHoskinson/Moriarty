#!/usr/bin/env python3
"""Preflight then os.execve for the one retained macro05 trace106 diagnostic.

No child collector, process-tree parser, signal heuristic, invocation counter,
cleanup claim, or observation-file producer. Existing runner owns capture.
"""
from __future__ import annotations

import hashlib
import json
import os
import re
import resource
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
DEFAULT_PINS = HERE / "k-macro05-trace106-pins.json"
PREFLIGHT_EXIT = 2
STACK_SOFT = 8388608
WRAPPER_PATH_RE = re.compile(r"PATH='(/nix/store/[^']+/bin)'\$PATH")


class PreflightError(RuntimeError):
    def __init__(self, code: str, detail: str = ""):
        super().__init__(code)
        self.code = code
        self.detail = detail


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_pins(path: Path | None = None) -> dict:
    pins_path = Path(path) if path is not None else DEFAULT_PINS
    data = json.loads(pins_path.read_text())
    if type(data) is not dict or data.get("kind") != "macro05-trace106-exec-shim-pins/1":
        raise PreflightError("PINS_SCHEMA")
    required = [
        "stackSoftBytes", "coreSoftBytes", "compileCalls", "retries",
        "recordedArgv", "diagnosticArgv", "childEnv", "queryEnv", "runnerBounds",
        "wrapperPathDirs", "parserShebang", "selectedExecutables", "retained",
        "binding", "manifests",
    ]
    if any(key not in data for key in required):
        raise PreflightError("PINS_SCHEMA")
    if data["stackSoftBytes"] != STACK_SOFT or data["coreSoftBytes"] != 0:
        raise PreflightError("PINS_BOUNDS")
    if data["compileCalls"] != 0 or data["retries"] != 0:
        raise PreflightError("PINS_BOUNDS")
    bounds = data["runnerBounds"]
    if (
        bounds.get("timeoutSeconds") != 20
        or bounds.get("graceSeconds") != 5
        or bounds.get("startupCleanupSeconds") != 30
        or bounds.get("chargedSeconds") != 60
    ):
        raise PreflightError("PINS_BOUNDS", "runnerBounds")
    if data["diagnosticArgv"][8:] != data["recordedArgv"]:
        raise PreflightError("ARGV_MISMATCH")
    if data["diagnosticArgv"][0] != "/usr/bin/strace":
        raise PreflightError("ARGV_MISMATCH", "strace")
    query_env = data["queryEnv"]
    if query_env.get("NIX_USER_CONF_FILES") != "/dev/null" or query_env.get("NIX_CONFIG") != "":
        raise PreflightError("PINS_BOUNDS", "queryEnv")
    return data


def _check_file(path_text: str, expected: str, role: str) -> None:
    path = Path(path_text)
    if not path.is_file() and not path.is_symlink():
        raise PreflightError("PIN_MISSING", role)
    if not path.exists():
        raise PreflightError("PIN_MISSING", role)
    digest = sha256_file(path)
    if digest != expected:
        raise PreflightError("PIN_MISMATCH", role)


def _check_symlink(path_text: str, expected_target: str | None) -> None:
    if expected_target is None:
        return
    path = Path(path_text)
    if path.is_symlink():
        actual = os.readlink(path)
        if actual != expected_target:
            raise PreflightError("SYMLINK_DRIFT", path_text)


def child_env(pins: dict, home: str | None = None) -> dict[str, str]:
    spec = pins["childEnv"]
    value = home if home is not None else os.environ.get("HOME")
    if type(value) is not str or not value:
        raise PreflightError("HOME_MISSING")
    env = {
        "PATH": spec["PATH"],
        "HOME": value,
        "LANG": spec["LANG"],
        "LC_ALL": spec["LC_ALL"],
        "PYTHONDONTWRITEBYTECODE": spec["PYTHONDONTWRITEBYTECODE"],
        "PYTHONNOUSERSITE": spec["PYTHONNOUSERSITE"],
        "PYTHONSAFEPATH": spec["PYTHONSAFEPATH"],
    }
    for name in spec["forbidden"]:
        if name in env:
            raise PreflightError("ENV_FORBIDDEN", name)
    return env


def query_env(pins: dict, home: str | None = None) -> dict[str, str]:
    env = child_env(pins, home=home)
    extra = pins["queryEnv"]
    env["NIX_USER_CONF_FILES"] = extra["NIX_USER_CONF_FILES"]
    env["NIX_CONFIG"] = extra["NIX_CONFIG"]
    for name in pins["childEnv"]["forbidden"]:
        if name in env:
            raise PreflightError("ENV_FORBIDDEN", name)
    return env


def namespace_uid_for_host(uid_map_text: str, host_uid: int, overflow_uid: int) -> int:
    for line in uid_map_text.splitlines():
        parts = line.split()
        if len(parts) != 3:
            raise PreflightError("UID_MAP")
        ns_id, host_id, count = (int(part) for part in parts)
        if count < 0:
            raise PreflightError("UID_MAP")
        if host_id <= host_uid < host_id + count:
            return ns_id + (host_uid - host_id)
    return overflow_uid


def read_namespace_mapping(ownership: dict) -> tuple[str, int]:
    text = Path(ownership["uidMapPath"]).read_text()
    overflow = int(Path(ownership["overflowUidPath"]).read_text().strip())
    if overflow != ownership["overflowUid"]:
        raise PreflightError("OVERFLOW_UID")
    expected = namespace_uid_for_host(text, ownership["hostRootUid"], overflow)
    if expected not in (ownership["hostRootUid"], ownership["namespaceMappedUid"]):
        raise PreflightError("UID_MAP")
    return text, expected


def resolve_on_path(name: str, directories: list[str]) -> Path | None:
    for directory in directories:
        candidate = Path(directory) / name
        if candidate.exists():
            return candidate
    return None


def verify_selected(pins: dict) -> int:
    count = 0
    for item in pins["selectedExecutables"] + pins["retained"]:
        _check_file(item["path"], item["sha256"], item.get("role", item["path"]))
        _check_symlink(item["path"], item.get("symlinkTarget"))
        count += 1
    return count


def verify_binding_artifacts(pins: dict) -> int:
    meta = pins["binding"]
    _check_file(meta["path"], meta["sha256"], "binding")
    binding = json.loads(Path(meta["path"]).read_text())
    if binding.get("krunInvocations") != 106:
        raise PreflightError("PIN_MISMATCH", "krunInvocations")
    k_root = Path(meta["kRoot"])
    kompiled = Path(meta["kompiled"])
    for name, expected in binding["sources"].items():
        _check_file(str(k_root / name), expected, "source:" + name)
    count = 0
    for name, expected in binding["artifacts"].items():
        _check_file(str(kompiled / name), expected, "artifact:" + name)
        count += 1
    if count != meta["artifactCount"]:
        raise PreflightError("PIN_MISMATCH", "artifactCount")
    command = json.loads(Path(next(item["path"] for item in pins["retained"] if item["role"] == "command")).read_text())
    if command.get("argv") != pins["recordedArgv"]:
        raise PreflightError("ARGV_MISMATCH", "recordedArgv")
    return count


def verify_wrapper_and_parser(pins: dict) -> None:
    krun = next(item for item in pins["selectedExecutables"] if item["role"] == "krun")
    text = Path(krun["path"]).read_text()
    dirs = WRAPPER_PATH_RE.findall(text)
    if dirs != pins["wrapperPathDirs"]:
        raise PreflightError("WRAPPER_PATH_DRIFT")
    missing = set(pins.get("wrapperPathDirsMissing") or [])
    for directory in dirs:
        present = Path(directory).is_dir()
        if directory in missing:
            if present:
                raise PreflightError("WRAPPER_PATH_UNEXPECTED", directory)
        elif not present:
            raise PreflightError("WRAPPER_PATH_MISSING", directory)
    parser = next(item for item in pins["retained"] if item["role"] == "parser")
    first = Path(parser["path"]).read_text().splitlines()[0]
    shebang = pins["parserShebang"]
    if first != shebang["line"]:
        raise PreflightError("PARSER_SHEBANG")
    mutated = list(pins["wrapperPathDirs"]) + pins["childEnv"]["PATH"].split(":")
    resolved = resolve_on_path(shebang["argument"], mutated)
    if resolved is None or str(resolved) != shebang["resolvedOnWrapperPath"]:
        raise PreflightError("PARSER_PYTHON", str(resolved))
    if str(resolved.resolve()) != shebang["resolvedTarget"]:
        raise PreflightError("PARSER_PYTHON_TARGET")
    if sha256_file(resolved) != shebang["sha256"]:
        raise PreflightError("PIN_MISMATCH", "parser-python")


def _check_manifest(here: Path, name: str, expected: str | None) -> None:
    if expected:
        _check_file(str(here / name), expected, name)


def verify_k_package(pins: dict, here: Path) -> int:
    _check_manifest(here, pins["manifests"]["kPackage"], pins["manifests"].get("kPackageSHA256"))
    manifest = json.loads((here / pins["manifests"]["kPackage"]).read_text())
    if manifest.get("kind") != "macro05-trace106-k-package-map/1" or manifest.get("count") != 255:
        raise PreflightError("K_PACKAGE_COUNT")
    if len(manifest["files"]) != 255:
        raise PreflightError("K_PACKAGE_COUNT")
    for item in manifest["files"]:
        path = Path(item["path"])
        if item.get("symlink"):
            if not path.is_symlink() or os.readlink(path) != item["target"]:
                raise PreflightError("SYMLINK_DRIFT", item["relative"])
        _check_file(item["path"], item["sha256"], "k:" + item["relative"])
    return 255


def load_host_evidence(ownership: dict) -> dict[str, dict]:
    path = Path(ownership["hostEvidencePath"])
    if sha256_file(path) != ownership["hostEvidenceSHA256"]:
        raise PreflightError("PIN_MISMATCH", "host-evidence")
    observation = json.loads(path.read_text())
    if observation.get("kind") != ownership.get("hostEvidenceKind"):
        raise PreflightError("HOST_EVIDENCE_SCHEMA")
    if observation.get("observerUidMap") != ownership.get("hostEvidenceUidMap"):
        raise PreflightError("HOST_EVIDENCE_UID_MAP")
    if len(observation.get("entries") or []) != ownership.get("hostEvidenceCount"):
        raise PreflightError("HOST_EVIDENCE_COUNT")
    mapping = {}
    for entry in observation["entries"]:
        if "hostUid" not in entry or "lstatInode" not in entry or "lstatDevice" not in entry:
            raise PreflightError("HOST_EVIDENCE_SCHEMA", "hostUid")
        mapping[entry["path"]] = entry
    if len(mapping) != ownership["hostEvidenceCount"]:
        raise PreflightError("HOST_EVIDENCE_COUNT")
    return mapping


def _host_evidence_paths(ownership: dict) -> dict[str, dict]:
    return load_host_evidence(ownership)


def bind_path_for_ownership(path_text: str, host_entries: dict[str, dict]) -> str | None:
    if path_text in host_entries:
        return path_text
    path = Path(path_text)
    if path.is_symlink():
        resolved = str(path.resolve())
        if resolved in host_entries:
            return resolved
    parts = Path(path_text).parts
    if len(parts) >= 4 and parts[1] == "nix" and parts[2] == "store":
        root = "/" + "/".join(parts[1:4])
        if root in host_entries:
            return root
    return None


def verify_nonwritable(path_text: str, expected_ns_uid: int) -> None:
    path = Path(path_text)
    if not path.exists():
        raise PreflightError("PIN_MISSING", path_text)
    if os.access(path, os.W_OK):
        raise PreflightError("WRITABLE", path_text)
    if path.lstat().st_uid != expected_ns_uid:
        raise PreflightError("NAMESPACE_UID", path_text)


def verify_ownership(
    path_text: str,
    host_entries: dict[str, dict],
    expected_ns_uid: int,
    host_root_uid: int,
) -> None:
    path = Path(path_text)
    if not path.exists():
        raise PreflightError("PIN_MISSING", path_text)
    if os.access(path, os.W_OK):
        raise PreflightError("WRITABLE", path_text)
    if path_text not in host_entries:
        raise PreflightError("ARBITRARY_OVERFLOW", path_text)
    host = host_entries[path_text]
    if host["hostUid"] != host_root_uid:
        raise PreflightError("HOST_OWNERSHIP", path_text)
    if host.get("writableByHostUser"):
        raise PreflightError("HOST_WRITABLE", path_text)
    st = path.lstat()
    if st.st_ino != host["lstatInode"] or st.st_dev != host["lstatDevice"]:
        raise PreflightError("HOST_STAT_DRIFT", path_text)
    if (st.st_mode & 0o7777) != host["mode"]:
        raise PreflightError("HOST_STAT_DRIFT", path_text + ":mode")
    if st.st_uid != expected_ns_uid:
        raise PreflightError("NAMESPACE_UID", path_text)


def _nix_store_query(query_tool: str, argv: list[str], env: dict[str, str], nix_run=None):
    runner = nix_run if nix_run is not None else subprocess.run
    return runner(
        argv,
        executable=query_tool,
        capture_output=True,
        text=True,
        check=False,
        env=env,
    )


def verify_requisites(pins: dict, here: Path, home: str | None = None, nix_run=None) -> int:
    _check_manifest(here, pins["manifests"]["ownership"], pins["manifests"].get("ownershipSHA256"))
    _check_manifest(here, pins["manifests"]["requisites"], pins["manifests"].get("requisitesSHA256"))
    ownership = json.loads((here / pins["manifests"]["ownership"]).read_text())
    requisites = json.loads((here / pins["manifests"]["requisites"]).read_text())
    if requisites.get("count") != 159 or len(requisites["entries"]) != 159:
        raise PreflightError("REQUISITE_COUNT")
    if ownership.get("kind") != "macro05-trace106-ownership/1":
        raise PreflightError("OWNERSHIP_SCHEMA")
    pointer = here / ownership["hostEvidencePointer"]
    if sha256_file(pointer) != ownership["hostEvidencePointerSHA256"]:
        raise PreflightError("PIN_MISMATCH", "host-evidence-pointer")
    if pointer.read_text().splitlines()[0] != ownership["hostEvidencePath"]:
        raise PreflightError("HOST_EVIDENCE_POINTER")
    if sha256_file(Path(ownership["nixMetadataObservationPath"])) != ownership["nixMetadataObservationSHA256"]:
        raise PreflightError("PIN_MISMATCH", "nix-metadata-observation")
    if sha256_file(Path(requisites["observationPath"])) != requisites["observationSHA256"]:
        raise PreflightError("PIN_MISMATCH", "observation")
    if sha256_file(Path(requisites["batchOrderControlPath"])) != requisites["batchOrderControlSHA256"]:
        raise PreflightError("PIN_MISMATCH", "batch-order")
    host_entries = load_host_evidence(ownership)
    _uid_map_text, expected_ns_uid = read_namespace_mapping(ownership)
    query_tool = next(item for item in pins["selectedExecutables"] if item["role"] == "nix-query")
    _check_file(query_tool["path"], query_tool["sha256"], "nix-query")
    if sha256_file(Path(requisites["queryTool"])) != requisites["queryToolSHA256"]:
        raise PreflightError("PIN_MISMATCH", "queryTool")
    paths = [entry["path"] for entry in requisites["entries"]]
    host_root = ownership["hostRootUid"]
    verify_ownership(ownership["storeRoot"], host_entries, expected_ns_uid, host_root)
    for path_text in paths:
        verify_ownership(path_text, host_entries, expected_ns_uid, host_root)
    for item in pins["selectedExecutables"]:
        bound = bind_path_for_ownership(item["path"], host_entries)
        if bound is None:
            verify_nonwritable(item["path"], expected_ns_uid)
        else:
            verify_ownership(bound, host_entries, expected_ns_uid, host_root)
            verify_nonwritable(item["path"], expected_ns_uid)
    env = query_env(pins, home=home)
    set_result = _nix_store_query(
        query_tool["path"],
        [requisites["queryArgv0"], "--query", "--requisites", requisites["kRoot"]],
        env,
        nix_run=nix_run,
    )
    if set_result.returncode != 0:
        raise PreflightError("NAR_QUERY", (set_result.stderr or "")[-200:])
    actual_set = set(line for line in set_result.stdout.splitlines() if line)
    if actual_set != set(paths) or len(actual_set) != 159:
        raise PreflightError("REQUISITE_SET")
    result = _nix_store_query(
        query_tool["path"],
        [requisites["queryArgv0"], "--query", "--hash", *paths],
        env,
        nix_run=nix_run,
    )
    if result.returncode != 0:
        raise PreflightError("NAR_QUERY", (result.stderr or "")[-200:])
    hashes = result.stdout.splitlines()
    if len(hashes) != 159:
        raise PreflightError("NAR_QUERY_COUNT")
    for entry, nar in zip(requisites["entries"], hashes):
        if nar != entry["narHash"]:
            raise PreflightError("NAR_DRIFT", entry["path"])
    return 159


def apply_limits(pins: dict, getrlimit=resource.getrlimit, setrlimit=resource.setrlimit) -> None:
    stack_soft, stack_hard = getrlimit(resource.RLIMIT_STACK)
    if stack_soft != pins["stackSoftBytes"]:
        raise PreflightError("STACK_LIMIT")
    setrlimit(resource.RLIMIT_CORE, (pins["coreSoftBytes"], pins["coreSoftBytes"]))
    setrlimit(resource.RLIMIT_STACK, (pins["stackSoftBytes"], stack_hard if stack_hard == -1 or stack_hard >= pins["stackSoftBytes"] else stack_hard))


def preflight(pins: dict, here: Path | None = None, home: str | None = None, nix_run=None) -> dict:
    here = here or HERE
    selected = verify_selected(pins)
    artifacts = verify_binding_artifacts(pins)
    verify_wrapper_and_parser(pins)
    k_files = verify_k_package(pins, here)
    requisites = verify_requisites(pins, here, home=home, nix_run=nix_run)
    return {
        "selected": selected,
        "artifacts": artifacts,
        "kFiles": k_files,
        "requisites": requisites,
    }


def run_shim(
    pins: dict | None = None,
    pins_path: Path | None = None,
    execve=None,
    getrlimit=resource.getrlimit,
    setrlimit=resource.setrlimit,
    home: str | None = None,
    here: Path | None = None,
) -> dict:
    pins = pins if pins is not None else load_pins(pins_path)
    stats = preflight(pins, here=here or HERE, home=home)
    apply_limits(pins, getrlimit=getrlimit, setrlimit=setrlimit)
    argv = list(pins["diagnosticArgv"])
    env = child_env(pins, home=home)
    target = argv[0]
    if execve is None:
        os.execve(target, argv, env)
        raise PreflightError("EXEC_RETURNED")
    execve(target, argv, env)
    return {"status": "exec-injected", "argv": argv, "env": env, "preflight": stats}


def main(argv: list[str] | None = None) -> int:
    args = list(sys.argv[1:] if argv is None else argv)
    if args:
        sys.stderr.write(json.dumps({"status": "PreflightFailed", "code": "UNEXPECTED_ARGV"}) + "\n")
        return PREFLIGHT_EXIT
    try:
        run_shim()
    except PreflightError as exc:
        sys.stderr.write(json.dumps({"status": "PreflightFailed", "code": exc.code, "detail": exc.detail}) + "\n")
        return PREFLIGHT_EXIT
    return PREFLIGHT_EXIT


if __name__ == "__main__":
    sys.exit(main())
