#!/usr/bin/env python3
"""Package explicitly selected public captures only; never execute a captured command.

Usage: python3 package_public_evidence.py CHECKOUT PUBLIC_RUNTIME SELECTION
SELECTION is {"format": SELECTION_FORMAT, "members": [...]}; each member gives its
archive "path", the PUBLIC_RUNTIME-relative file it copies ("from"), and that
file's "sha256", "bytes" and provenance "origin". Only selected files are read;
runtime folders are not scanned and no bytes are read from CHECKOUT.
Output is confined to this script's directory and PUBLIC_RUNTIME/delivery-package.json.
Archive member order, modes, ownership, timestamps and gzip header are fixed.
Every integrity check raises PackagingError, so python -O skips none of them.
"""
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import re
import stat
import sys
import tarfile

OUT = Path(__file__).resolve().parent
USAGE = "usage: package_public_evidence.py CHECKOUT PUBLIC_RUNTIME SELECTION"
SELECTION_FORMAT = "sherlock-parallel-public-selection/1"
GROUPS = (
    "ordinary-01", "host-01", "host-01-outer", "image-01", "image-01-outer",
    "quint-01", "quint-context-fix", "work-summary-01",
)
EXCLUDED_PARTS = {"build", "tmp", "__pycache__"}
MEMBER_KEYS = {"path", "from", "sha256", "bytes", "origin"}
DIGEST = re.compile(r"[0-9a-f]{64}")
# Frozen October 16 normal-corpus totals; every projection must match them.
EXPECTED = {"programs": 8, "runs": 216, "records": 5355, "charged": 5139, "publications": 216,
            "itf_captures": 216, "raw_states": 5579, "terminal_stutters": 8}


class PackagingError(Exception):
    """A selection or integrity check failed before any output file was written or replaced.

    Archive checks may leave a newly created, empty output folder. Filesystem
    errors while writing outputs are OSError, not refusals, and may leave
    partial output: publication is validated first but is not atomic.
    """


def require(condition, message):
    if not condition:
        raise PackagingError(message)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def strict_json(data, what):
    def unique(pairs):
        keys = [key for key, _ in pairs]
        require(len(keys) == len(set(keys)), f"duplicate JSON key in {what}")
        return dict(pairs)
    try:
        return json.loads(data, object_pairs_hook=unique)
    except ValueError as error:
        raise PackagingError(f"invalid JSON in {what}: {error}") from None


def relative_parts(text, what):
    require(isinstance(text, str) and text and not text.startswith("/")
            and "\\" not in text and "\0" not in text, f"{what} is not a relative POSIX path: {text!r}")
    parts = text.split("/")
    require(all(part not in ("", ".", "..") for part in parts), f"{what} is not normalized: {text!r}")
    return parts


def check_member_name(name):
    parts = relative_parts(name, "archive member")
    require(len(parts) > 1 and parts[0] in GROUPS, f"archive member outside the delivery groups: {name}")
    require(not EXCLUDED_PARTS.intersection(parts), f"build/tmp/__pycache__ member refused: {name}")


def check_member_folders(names):
    # A member that is also another member's folder cannot be written to records/.
    folders = {"/".join(name.split("/")[:i]) for name in names for i in range(1, name.count("/") + 1)}
    clashes = sorted(folders.intersection(names))
    require(not clashes, f"archive member is also a folder of another member: {clashes[0] if clashes else ''}")


def read_selected(runtime, rel):
    """Read one regular file below runtime without following a symlink."""
    path = runtime
    for part in relative_parts(rel, "selected source"):
        path = path / part
        require(not path.is_symlink(), f"symlink refused: {rel}")
    require(path.is_file(), f"selected file is missing or not regular: {rel}")
    try:
        fd = os.open(path, os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0))
    except OSError as error:
        raise PackagingError(f"selected file unreadable: {rel}: {error.strerror}") from None
    with os.fdopen(fd, "rb") as handle:
        require(stat.S_ISREG(os.fstat(handle.fileno()).st_mode), f"selected file is missing or not regular: {rel}")
        return handle.read()


def select_members(runtime, selection_path):
    """Return selected member bytes, origins and runtime-file index, all digest-checked."""
    try:
        raw = selection_path.read_bytes()
    except OSError as error:
        raise PackagingError(f"selection unreadable: {error.strerror}") from None
    selection = strict_json(raw, "selection")
    require(isinstance(selection, dict) and set(selection) == {"format", "members"}
            and selection["format"] == SELECTION_FORMAT,
            f"selection must be {{\"format\": \"{SELECTION_FORMAT}\", \"members\": [...]}}")
    entries = selection["members"]
    require(isinstance(entries, list) and entries, "selection members must be a nonempty list")
    members, origins, by_runtime_file = {}, {}, {}
    for entry in entries:
        require(isinstance(entry, dict) and set(entry) == MEMBER_KEYS,
                f"selection member needs exactly {sorted(MEMBER_KEYS)}")
        name = entry["path"]
        check_member_name(name)
        require(name not in members, f"duplicate archive member: {name}")
        require(isinstance(entry["sha256"], str) and DIGEST.fullmatch(entry["sha256"]),
                f"selection digest malformed: {name}")
        require(type(entry["bytes"]) is int and entry["bytes"] >= 0, f"selection size malformed: {name}")
        require(isinstance(entry["origin"], str) and entry["origin"], f"selection origin missing: {name}")
        data = read_selected(runtime, entry["from"])
        require(len(data) == entry["bytes"] and sha(data) == entry["sha256"],
                f"selected bytes differ from selection: {name} <- {entry['from']}")
        members[name] = data
        origins[name] = entry["origin"]
        by_runtime_file.setdefault(entry["from"], name)
    check_member_folders(members)
    return members, origins, by_runtime_file, sha(raw)


def create_archive(path, members):
    with path.open("wb") as raw:
        with gzip.GzipFile(fileobj=raw, filename="", mode="wb", mtime=0, compresslevel=9) as compressed:
            with tarfile.open(fileobj=compressed, mode="w", format=tarfile.USTAR_FORMAT) as tar:
                for name, data in sorted(members.items()):
                    info = tarfile.TarInfo(name)
                    info.size = len(data)
                    info.mode = 0o644
                    info.uid = info.gid = info.mtime = 0
                    info.uname = info.gname = ""
                    tar.addfile(info, io.BytesIO(data))


def steps(receipt):
    return [{"name": s["name"], "state": s["state"],
             **({"exit": s["child"]["exit_code"],
                 "seconds": s["child"]["duration_seconds"],
                 "timeout_seconds": s["child"]["timeout_seconds"],
                 "timed_out": s["child"]["timed_out"]} if "child" in s else {})}
            for s in receipt["steps"]]


def package(checkout, runtime, selection_path, out, expected=EXPECTED):
    require(out.is_relative_to(checkout), f"output folder is outside the checkout: {out}")
    members, origins, by_runtime_file, selection_digest = select_members(runtime, selection_path)

    def record(group, name):
        member = f"{group}/{name}"
        require(member in members, f"required record not selected: {member}")
        return members[member]

    def load(group, name):
        return json.loads(record(group, name))

    host, image = (load(g, "receipt.json") for g in ("host-01", "image-01"))
    quint = load("quint-01", "result.json")
    context = load("quint-context-fix", "recheck.json")
    work = load("work-summary-01", "invocations.json")
    ordinary = load("ordinary-01", "result.json")
    source_sets = {"ordinary-01": ordinary["before"], "quint-01": quint["sources_before"],
                   "quint-context-fix": context["source_sha256"],
                   "work-summary-01": work["source_sha256"]}
    for group, receipt in (("host-01", host), ("image-01", image)):
        source_sets[group] = {s["path"]: s["sha256_before"] for s in receipt["sources"]}
    for group in ("host-01-outer", "image-01-outer"):
        source_sets[group] = load(group, "result.json")["source_before"]

    # Each recorded source must be selected with its exact recorded digest. Only
    # broader outer-wrapper context may stay unarchived, listed by hash alone.
    bindings = []
    unrecovered_context_sources = []
    for group, sources in source_sets.items():
        for rel, digest in sorted(sources.items()):
            name = f"{group}/source/{rel}"
            if name not in members:
                require(group.endswith("-outer"), f"recorded source not selected: {group}:{rel}:{digest}")
                unrecovered_context_sources.append({"attempt": group, "path": rel, "sha256": digest,
                                                    "status": "recorded outer context hash only; exact bytes not selected"})
                continue
            require(sha(members[name]) == digest, f"selected source differs from recorded digest: {name}")
            bindings.append({"attempt": group, "path": rel, "sha256": digest,
                             "archive_member": name, "recovered_from": origins[name]})
    bound = {binding["archive_member"] for binding in bindings}
    unbound = [name for name in sorted(members) if name.split("/")[1] == "source" and name not in bound]
    require(not unbound, f"selected source has no recorded digest: {unbound[0] if unbound else ''}")

    # Check every receipt-local digest-bearing file that is actually included.
    # Executable/toolchain references remain in the original receipts, uncopied.
    checked = []
    excluded_digest_references = 0

    def check_references(value):
        nonlocal excluded_digest_references
        if isinstance(value, dict):
            path, digest = value.get("path"), value.get("sha256")
            if isinstance(path, str) and isinstance(digest, str):
                try:
                    rel = Path(path).relative_to(runtime).as_posix()
                except ValueError:
                    excluded_digest_references += 1
                else:
                    name = by_runtime_file.get(rel)
                    if name is None:
                        excluded_digest_references += 1
                    else:
                        data = members[name]
                        require(sha(data) == digest, f"receipt digest mismatch: {name}")
                        if "bytes" in value:
                            require(len(data) == value["bytes"], f"receipt size mismatch: {name}")
                        checked.append(name)
            for item in value.values():
                check_references(item)
        elif isinstance(value, list):
            for item in value:
                check_references(item)

    for name, data in sorted(members.items()):
        if name.endswith(("/receipt.json", "/result.json", "/invocations.json")):
            check_references(json.loads(data))
    capture_hashes = load("quint-01", "capture-hashes.json")
    for name, digest in capture_hashes.items():
        require(sha(record("quint-01", f"captures/{name}")) == digest,
                f"capture digest mismatch: quint-01/captures/{name}")
    itf_capture_count = sum(name.endswith(".itf.json") for name in capture_hashes)
    require(itf_capture_count == expected["itf_captures"],
            f"itf_captures count: observed {itf_capture_count}, expected {expected['itf_captures']}")
    context_log_digest_checks = 0
    for command in context["commands"]:
        for stream in ("stdout", "stderr"):
            name = f"{command['name']}.{stream}"
            require(sha(record("quint-context-fix", name)) == command[stream + "_sha256"],
                    f"context log digest mismatch: quint-context-fix/{name}")
            context_log_digest_checks += 1
        if "script_sha256" in command:
            require(sha(record("quint-context-fix", "actual-context-check.py")) == command["script_sha256"],
                    "context script digest mismatch: quint-context-fix/actual-context-check.py")
    require(record("quint-01", "compare.stdout") == record("quint-context-fix", "actual-compare.stdout"),
            "context comparison differs from quint-01/compare.stdout")

    summaries = {}
    for layer, filename in (("host", "host-summary.stdout"),
                            ("image", "image-summary-corrected.stdout"),
                            ("quint", "quint-summary.stdout")):
        summary = load("work-summary-01", filename)
        summaries[layer] = summary["cross_run"]
    qobs = load("quint-01", "quint-observations.json")
    raw_states = sum(len(trace["itf"]["states"]) for trace in qobs["raw_traces"])
    stutters = sum(qobs["projection"]["terminal_stutters"])
    for layer, group, filename in (("host", "host-01", "host-observations.json"),
                                  ("image", "image-01", "image-observations.json"),
                                  ("quint", "quint-01", "quint-observations.json")):
        artifact = load(group, filename)
        observations = artifact.get("observations", artifact)
        runs = observations["runs"]
        records = [record for run in runs for record in run["records"]]
        totals = summaries[layer]["prefix_runs"]
        observed = {"programs": len(observations["programs"]), "runs": len(runs),
                    "records": len(records),
                    "charged": sum(record["charge"] for record in records),
                    "publications": sum(record["charge"] == 0 for record in records)}
        summarized = {"runs": totals["run_count"], "records": totals["record_count"],
                      "charged": totals["charged_sum"], "publications": totals["publication_count"]}
        for key, value in observed.items():
            require(value == expected[key] and summarized.get(key, value) == value,
                    f"{layer} {key} count: observed {value}, summary {summarized.get(key)}, expected {expected[key]}")
    for key, value in (("raw_states", raw_states), ("terminal_stutters", stutters)):
        require(value == expected[key], f"{key} count: observed {value}, expected {expected[key]}")

    # Everything published below is derived before the output folder is touched,
    # so a missing record or field stops here with no output written or replaced.
    # Small original receipts/logs are also directly reviewable. Raw observations,
    # ITF and source bytes remain in the archive, with complete member hashes.
    direct = [name for name, data in sorted(members.items())
              if "/source/" not in name and "/captures/" not in name and len(data) <= 100_000]
    limitations = [
        "Captured-local results bind their recorded source digests; they are not a fresh execution of the final PR tree.",
        "Outer-wrapper source maps are broader context, not compile/read closure declarations. Recovered source_observations.rs is source-prepared context never executed in these attempts; later edits do not make the old whole context manifest current.",
        "The first ordinary capture passed 140 Python tests before five diagnostic-context regressions were added; a fresh full 145-test check remains with the integrator.",
        "The image interpreter is an existing separate flat-ID host interpreter; its compile closure excludes reference.rs, which is a closure observation, not proof of independence. No image unit suite was selected.",
        "Stdlib current-body, image and raw diagnostics are source preparation and private cfg(test) host proposals; Candidate B representation/native admission is unestablished.",
        "No actual K capture is included. Lean reference producer failure and repair remain separately owned; no Lean result is claimed here.",
        "No Clean constraint, handwritten circuit, native, proof, ledger or Midnight stagenet result is included. Native authority remains zero.",
        "No source-Core, bounded scalar or streaming feature result is claimed; those next source tasks are not delivered by this capture.",
        "Constructors begin at the post-startup boundary; startup execution, canonical serialization, full grammar/numeric coverage and refinement remain open.",
        "The finite Quint corpus is deterministic normal-case evidence, not exhaustive model checking or correspondence proof; raw terminal stutters are separate from projected semantic records.",
        "Logical work statistics do not establish helper costs, physical/circuit costs, DUST, fees or financial/economic outcomes.",
        "Local digests bind copied bytes; system runtime, sysroot/linker and tool dependencies are incompletely pinned. No authenticity or security certification is inferred.",
        "Author observations are recorded separately from independent review. Final candidate source/result review and Opus-owned pre-push review remain with the integrator.",
        "All U0-U7, M1-M17 and MC/SP/G parent exits remain open; public developer use stays permissionless.",
    ]
    if unrecovered_context_sources:
        limitations.append("Some broader outer-wrapper context hashes are not archived because their exact bytes were not selected; SOURCE-BINDINGS.json lists them. They are not replaced with changed current source. The host/image receipt source closures are fully archived.")
    host_outer, image_outer = load("host-01-outer", "result.json"), load("image-01-outer", "result.json")
    result = {
        "format": "sherlock-parallel-public-delivery/1", "date": "2026-10-16",
        "evidence_class": "captured-local-public-normal-language-evidence",
        "acceptance": "none", "native_authority": 0,
        "ordinary": {"attempt": "ordinary-01", "exit": ordinary["exit"],
                     "seconds": ordinary["elapsed_seconds"], "python_tests": 140,
                     "source_unchanged_in_attempt": ordinary["source_unchanged"]},
        "host": {"attempt": "host-01", "terminal_state": host["terminal_state"],
                 "source_files": len(host["sources"]), "rust_tests": 14,
                 "outer_exit": host_outer["exit"], "outer_seconds": host_outer["elapsed_seconds"],
                 "steps": steps(host)},
        "image": {"attempt": "image-01", "terminal_state": image["terminal_state"],
                  "source_files": len(image["sources"]), "tests": "not-selected",
                  "outer_exit": image_outer["exit"], "outer_seconds": image_outer["elapsed_seconds"],
                  "steps": steps(image)},
        "quint": {"attempt": "quint-01", "terminal_state": quint["terminal_state"],
                  "seconds": quint["elapsed_seconds"], "deterministic_itf_captures": itf_capture_count,
                  "raw_states": raw_states, "terminal_stutters": stutters,
                  "steps": [{k: s[k] for k in ("name", "exit", "elapsed_seconds", "timeout_seconds", "timed_out")}
                            for s in quint["steps"]]},
        "common_profile": "sherlock-k-a/1", "common_boundary": "post-startup",
        "logical_work_by_layer": summaries,
        "diagnostic_context_fix": {"attempt": "quint-context-fix", "red_exit": load("quint-context-fix", "red.json")["exit"],
                                   "green_layer_tests": 14, "related_regressions": 33,
                                   "commands": [{"name": c["name"], "exit": c["exit"]} for c in context["commands"]],
                                   "actual_quint_comparison_byte_identical": True,
                                   "model_reexecuted": False},
        "work_summary": {"attempt": "work-summary-01", "python_tests": 8,
                         "artifact_summaries": 3,
                         "commands": [{"name": c["name"], "exit": c["exit_code"]} for c in work["commands"]],
                         "failed_image_filename_attempt_preserved": True},
        "review": {"author_observations": "captured and packaged",
                   "independent_final_source_result_review": "pending integrator",
                   "reviewer_timeout_predictions": "advisory estimates; actual host/image receipts succeeded within every 30-second child cap",
                   "security_review": "Opus-owned; no packaging-agent judgment"},
        "verification": {"source_digest_bindings_checked": len(bindings),
                         "unrecovered_outer_context_sources": unrecovered_context_sources,
                         "included_receipt_digest_references_checked": len(checked),
                         "excluded_executable_tool_or_other_digest_references": excluded_digest_references,
                         "quint_itf_digests_checked": itf_capture_count,
                         "quint_capture_manifest_digest_checked": True,
                         "context_recheck_log_digests_checked": context_log_digest_checks,
                         "archive_members_checked": len(members),
                         "explicit_selection_sha256": selection_digest,
                         "archive_reproducible_bytes": True,
                         "copied_original_bytes_unchanged": True,
                         "commands_reexecuted_for_packaging": False},
        "direct_record_files": ["records/" + name for name in direct], "limitations": limitations,
    }

    # Build twice, verify bytes and every member in memory, then publish the
    # verified candidate. A failed archive check writes and replaces nothing.
    # Later writes are not atomic: a filesystem error can leave partial output.
    out.mkdir(parents=True, exist_ok=True)
    archive = out / "public-evidence.tar.gz"
    candidate = out / ".public-evidence-candidate.tar.gz"
    second = out / ".public-evidence-recheck.tar.gz"
    try:
        create_archive(candidate, members)
        create_archive(second, members)
        require(sha(candidate.read_bytes()) == sha(second.read_bytes()), "archive construction is not reproducible")
        with tarfile.open(candidate, "r:gz") as tar:
            archived = tar.getmembers()
            require([x.name for x in archived] == sorted(members), "archive member list differs from selection")
            for item in archived:
                require(item.isfile() and item.mode == 0o644, f"archive member is not a 0644 file: {item.name}")
                require(tar.extractfile(item).read() == members[item.name],
                        f"archive member bytes differ: {item.name}")
        os.replace(candidate, archive)
    finally:
        for path in (candidate, second):
            if path.exists():
                path.unlink()

    for name in direct:
        target = out / "records" / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(members[name])
    write_json(out / "SOURCE-BINDINGS.json", {"verified": bindings,
                                             "unrecovered_outer_context": unrecovered_context_sources})
    write_json(out / "ARCHIVE-MEMBERS.json", [
        {"path": name, "sha256": sha(data), "bytes": len(data), "origin": origins[name]}
        for name, data in sorted(members.items())
    ])
    result["archive"] = {"path": archive.name, "sha256": sha(archive.read_bytes()),
                         "bytes": archive.stat().st_size, "members": len(members),
                         "uncompressed_member_bytes": sum(map(len, members.values())),
                         "metadata": "gzip mtime=0/no filename; sorted USTAR regular files, mode 0644, uid/gid/mtime=0, empty owner names"}
    write_json(out / "RESULT.json", result)
    public_files = [p for p in out.rglob("*") if p.is_file()]
    delivery = {"format": result["format"], "package": str(out.relative_to(checkout)),
                "public_file_count": len(public_files), "public_bytes": sum(p.stat().st_size for p in public_files),
                "archive": result["archive"], "result_sha256": sha((out / "RESULT.json").read_bytes()),
                "source_digest_bindings_checked": len(bindings), "limitations": limitations}
    write_json(runtime / "delivery-package.json", delivery)
    return delivery


def main(argv=None, out=OUT):
    args = sys.argv[1:] if argv is None else list(argv)
    if len(args) != 3:
        raise SystemExit(USAGE)
    checkout, runtime, selection = (Path(arg).resolve() for arg in args)
    try:
        delivery = package(checkout, runtime, selection, out)
    except PackagingError as error:
        raise SystemExit(f"package_public_evidence: refused: {error}") from None
    print(json.dumps(delivery, indent=2))


if __name__ == "__main__":
    main()
