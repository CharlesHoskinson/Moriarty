#!/usr/bin/env python3
"""Package existing public captures only; never execute a captured command.

Usage: python3 package_public_evidence.py CHECKOUT PUBLIC_RUNTIME
Output is confined to this script's directory and PUBLIC_RUNTIME/delivery-package.json.
Archive member order, modes, ownership, timestamps and gzip header are fixed.
"""
import gzip
import hashlib
import io
import json
from pathlib import Path
import sys
import tarfile

CHECKOUT, RUNTIME = map(lambda x: Path(x).resolve(), sys.argv[1:])
OUT = Path(__file__).resolve().parent
GROUPS = (
    "ordinary-01", "host-01", "host-01-outer", "image-01", "image-01-outer",
    "quint-01", "quint-context-fix", "work-summary-01",
)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def load(group, name):
    return json.loads((RUNTIME / group / name).read_text())


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


members = {}
origins = {}
for group in GROUPS:
    for path in sorted((RUNTIME / group).rglob("*")):
        rel = path.relative_to(RUNTIME / group)
        if any(part in {"build", "tmp", "__pycache__"} for part in rel.parts):
            continue
        if path.is_symlink():
            raise ValueError(f"symlink refused: {group}/{rel}")
        if path.is_file():
            data = path.read_bytes()
            name = f"{group}/{rel.as_posix()}"
            members[name] = data
            origins[name] = f"recorded-public-attempt:{name}"

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

# Recover only exact recorded bytes. Existing public snapshots precede checkout
# fallback; a changed current file cannot silently replace the executed source.
snapshot_index = {}
for name, data in members.items():
    if "/source/" in name:
        snapshot_index[(name.split("/source/", 1)[1], sha(data))] = (name, data)
recovered_context_path = RUNTIME / "recovered-public/source_observations-pre-H1.rs"
if recovered_context_path.is_file():
    recovered_context = recovered_context_path.read_bytes()
    expected_context_digest = "afa116d7180ec657fe40add0a53953199a02362a33eabdd7b46489da3d86b908"
    assert sha(recovered_context) == expected_context_digest
    snapshot_index[("models/sherlock/reference/source_observations.rs", expected_context_digest)] = (
        "recovered-public/source_observations-pre-H1.rs; context-source-prepared-never-executed-not-compile-closure",
        recovered_context,
    )
bindings = []
unrecovered_context_sources = []
for group, sources in source_sets.items():
    for rel, expected in sorted(sources.items()):
        key = (rel, expected)
        recovered = snapshot_index.get(key)
        if recovered:
            origin, data = recovered
        else:
            path = CHECKOUT / rel
            data = path.read_bytes() if path.is_file() else b""
            if sha(data) != expected:
                if group.endswith("-outer"):
                    unrecovered_context_sources.append({"attempt": group, "path": rel,
                                                        "sha256": expected,
                                                        "status": "recorded outer context hash only; exact old bytes unavailable"})
                    continue
                raise ValueError(f"recorded source unavailable: {group}:{rel}:{expected}")
            origin = f"checkout-recovered-exact-digest:{rel}"
        name = f"{group}/source/{rel}"
        if name in members and members[name] != data:
            raise ValueError(f"snapshot mismatch: {name}")
        members[name] = data
        origins.setdefault(name, origin)
        bindings.append({"attempt": group, "path": rel, "sha256": expected,
                         "archive_member": name, "recovered_from": origins[name]})

# Check every receipt-local digest-bearing file that is actually included.
# Executable/toolchain references remain in the original receipts, uncopied.
checked = []
excluded_digest_references = 0


def check_references(value):
    global excluded_digest_references
    if isinstance(value, dict):
        path, digest = value.get("path"), value.get("sha256")
        if isinstance(path, str) and isinstance(digest, str):
            try:
                name = Path(path).relative_to(RUNTIME).as_posix()
            except ValueError:
                excluded_digest_references += 1
            else:
                if name in members:
                    data = members[name]
                    assert sha(data) == digest, name
                    if "bytes" in value:
                        assert len(data) == value["bytes"], name
                    checked.append(name)
                else:
                    excluded_digest_references += 1
        for item in value.values():
            check_references(item)
    elif isinstance(value, list):
        for item in value:
            check_references(item)


for name, data in list(members.items()):
    if name.endswith(("/receipt.json", "/result.json", "/invocations.json")):
        check_references(json.loads(data))
capture_hashes = load("quint-01", "capture-hashes.json")
for name, digest in capture_hashes.items():
    assert sha(members[f"quint-01/captures/{name}"]) == digest
itf_capture_count = sum(name.endswith(".itf.json") for name in capture_hashes)
assert itf_capture_count == 216
context_log_digest_checks = 0
for command in context["commands"]:
    for stream in ("stdout", "stderr"):
        name = f"quint-context-fix/{command['name']}.{stream}"
        assert sha(members[name]) == command[stream + "_sha256"], name
        context_log_digest_checks += 1
    if "script_sha256" in command:
        assert sha(members["quint-context-fix/actual-context-check.py"]) == command["script_sha256"]
assert members["quint-01/compare.stdout"] == members["quint-context-fix/actual-compare.stdout"]

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
    assert len(observations["programs"]) == 8
    assert len(runs) == totals["run_count"] == 216
    assert len(records) == totals["record_count"] == 5355
    assert sum(record["charge"] for record in records) == totals["charged_sum"] == 5139
    assert sum(record["charge"] == 0 for record in records) == totals["publication_count"] == 216
assert raw_states == 5579 and stutters == 8

archive = OUT / "public-evidence.tar.gz"


def create_archive(path):
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


create_archive(archive)
# Verify the gzip bytes by a second deterministic construction, then verify each
# archive member against the selected source bytes without filesystem extraction.
second = OUT / ".public-evidence-recheck.tar.gz"
create_archive(second)
assert sha(archive.read_bytes()) == sha(second.read_bytes())
second.unlink()
with tarfile.open(archive, "r:gz") as tar:
    archived = tar.getmembers()
    assert [x.name for x in archived] == sorted(members)
    for item in archived:
        assert item.isfile() and item.mode == 0o644
        assert tar.extractfile(item).read() == members[item.name]

# Small original receipts/logs are also directly reviewable. Raw observations,
# ITF and source bytes remain in the archive, with complete member hashes.
direct = []
for name, data in sorted(members.items()):
    if "/source/" not in name and "/captures/" not in name and len(data) <= 100_000:
        target = OUT / "records" / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
        direct.append("records/" + name)
write_json(OUT / "SOURCE-BINDINGS.json", {"verified": bindings,
                                         "unrecovered_outer_context": unrecovered_context_sources})
write_json(OUT / "ARCHIVE-MEMBERS.json", [
    {"path": name, "sha256": sha(data), "bytes": len(data), "origin": origins[name]}
    for name, data in sorted(members.items())
])

def steps(receipt):
    return [{"name": s["name"], "state": s["state"],
             **({"exit": s["child"]["exit_code"],
                 "seconds": s["child"]["duration_seconds"],
                 "timeout_seconds": s["child"]["timeout_seconds"],
                 "timed_out": s["child"]["timed_out"]} if "child" in s else {})}
            for s in receipt["steps"]]


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
    limitations.append("Some broader outer-wrapper context hashes have no available exact old source bytes; SOURCE-BINDINGS.json lists them. They are not replaced with changed current source. The host/image receipt source closures are fully archived.")
result = {
    "format": "sherlock-parallel-public-delivery/1", "date": "2026-10-16",
    "evidence_class": "captured-local-public-normal-language-evidence",
    "acceptance": "none", "native_authority": 0,
    "ordinary": {"attempt": "ordinary-01", "exit": ordinary["exit"],
                 "seconds": ordinary["elapsed_seconds"], "python_tests": 140,
                 "source_unchanged_in_attempt": ordinary["source_unchanged"]},
    "host": {"attempt": "host-01", "terminal_state": host["terminal_state"],
             "source_files": len(host["sources"]), "rust_tests": 14,
             "outer_exit": load("host-01-outer", "result.json")["exit"],
             "outer_seconds": load("host-01-outer", "result.json")["elapsed_seconds"],
             "steps": steps(host)},
    "image": {"attempt": "image-01", "terminal_state": image["terminal_state"],
              "source_files": len(image["sources"]), "tests": "not-selected",
              "outer_exit": load("image-01-outer", "result.json")["exit"],
              "outer_seconds": load("image-01-outer", "result.json")["elapsed_seconds"],
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
                     "archive_reproducible_bytes": True,
                     "copied_original_bytes_unchanged": True,
                     "commands_reexecuted_for_packaging": False},
    "archive": {"path": archive.name, "sha256": sha(archive.read_bytes()),
                "bytes": archive.stat().st_size, "members": len(members),
                "uncompressed_member_bytes": sum(map(len, members.values())),
                "metadata": "gzip mtime=0/no filename; sorted USTAR regular files, mode 0644, uid/gid/mtime=0, empty owner names"},
    "direct_record_files": direct, "limitations": limitations,
}
write_json(OUT / "RESULT.json", result)
public_files = [p for p in OUT.rglob("*") if p.is_file()]
delivery = {"format": result["format"], "package": str(OUT.relative_to(CHECKOUT)),
            "public_file_count": len(public_files), "public_bytes": sum(p.stat().st_size for p in public_files),
            "archive": result["archive"], "result_sha256": sha((OUT / "RESULT.json").read_bytes()),
            "source_digest_bindings_checked": len(bindings), "limitations": limitations}
write_json(RUNTIME / "delivery-package.json", delivery)
print(json.dumps(delivery, indent=2))
