#!/usr/bin/env python3
"""Bounded stdlib-only helper to refresh 1,755 control-metadata rows in FINAL_CURRENT_STAGING_INPUTS.json."""
import argparse
import collections
import copy
import datetime
import hashlib
import json
import os
import pathlib
import re
import resource
import signal
import stat
import sys
import tempfile
import unittest

EXPECTED_BASE_SHA256 = "b2dfd3d459d33e38349ca95a46439e1384c319a51ee4c8e89ceadb40ac128d4f"
EXPECTED_BASE_BYTES = 1155113
CARGO_ROOT = "/home/charl/.cargo"
REGISTRY_NAME = "index.crates.io-1949cf8c6b5b557f"
GIT_DB_PREFIX = "git/db/midnight-ledger-b2f9c59d942dfdca/"

EXPECTED_TOP_KEYS = (
    "observation",
    "time_utc",
    "inputs",
    "rust_pins",
    "native_pins",
    "exclusions",
    "gaps",
    "counts",
    "all_staging_files_have_hashes",
    "scope",
)

EXPECTED_CATEGORY_COUNTS = {
    "registry-index": 1735,
    "registry-archive-container": 423,
    "git-db-metadata": 20,
    "git-loose-container-known-commit-tree-or-nonpayload-source": 6,
    "git-loose-unknown": 52,
    "git-pack-container-or-index": 7,
}

SELECTED_CONTROL_CATEGORIES = frozenset({"registry-index", "git-db-metadata"})
NONSELECTED_CATEGORIES = frozenset(
    set(EXPECTED_CATEGORY_COUNTS.keys()) - SELECTED_CONTROL_CATEGORIES
)

NORMAL_ROW_KEYS = frozenset(
    {
        "bytes",
        "category",
        "destination_relative_to_future_home",
        "mode",
        "observation",
        "sha256",
        "source",
    }
)
UNKNOWN_LOOSE_ROW_KEYS = frozenset(
    {
        "bytes",
        "category",
        "destination_relative_to_future_home",
        "hash_gap",
        "mode",
        "source",
    }
)
EXPECTED_HASH_GAP = "Unknown/excluded loose-object content; stat only"

MAX_FILE_BYTES_BY_CATEGORY = {
    "registry-index": 8 * 1024 * 1024,  # 8 MiB (old max: 4,487,830 B)
    "git-db-metadata": 4096,            # 4 KiB (old max: 177 B)
}
MAX_CONTROL_TOTAL_BYTES = 90_000_000    # 90 MB (projected: 87,570,322 B)
EXPECTED_NONCONTROL_TOTAL_BYTES = 113_836_232
MAX_STAGE_COPY_BYTES = 268_435_456      # 256 MiB A_copy cap

GIT_METADATA_TAILS = frozenset(
    {
        "HEAD",
        "config",
        "FETCH_HEAD",
        "description",
        "hooks/README.sample",
        "info/exclude",
    }
)
GIT_COMMIT_REF_RE = re.compile(r"^refs/commit/[0-9a-f]{40}$")
GIT_LOOSE_RE = re.compile(r"^objects/[0-9a-f]{2}/[0-9a-f]{38}$")
GIT_PACK_RE = re.compile(
    r"^objects/pack/pack-[0-9a-f]{40}\.(pack|idx|rev|mtimes)$"
)


class Refusal(RuntimeError):
    pass


def need(cond, msg):
    if not cond:
        raise Refusal(msg)


def apply_process_bounds():
    def _set_rlimit(res, soft, hard):
        try:
            cur_soft, cur_hard = resource.getrlimit(res)
            new_hard = min(cur_hard, hard) if cur_hard != resource.RLIM_INFINITY else hard
            new_soft = min(soft, new_hard)
            resource.setrlimit(res, (new_soft, new_hard))
        except (ValueError, OSError):
            pass

    _set_rlimit(resource.RLIMIT_CORE, 0, 0)
    _set_rlimit(resource.RLIMIT_FSIZE, 4 * 1024 * 1024, 4 * 1024 * 1024)
    _set_rlimit(resource.RLIMIT_DATA, 128 * 1024 * 1024, 128 * 1024 * 1024)
    _set_rlimit(resource.RLIMIT_AS, 256 * 1024 * 1024, 256 * 1024 * 1024)
    _set_rlimit(resource.RLIMIT_CPU, 25, 25)

    def _on_alarm(signum, frame):
        raise Refusal("wall-clock timeout exceeded (30s)")

    signal.signal(signal.SIGALRM, _on_alarm)
    signal.setitimer(signal.ITIMER_REAL, 30.0)


def no_dup_pairs(pairs):
    out = {}
    for k, v in pairs:
        need(k not in out, f"duplicate JSON key: {k}")
        out[k] = v
    return out


def check_non_symlink_ancestors(path_str, stop_root=None):
    p = pathlib.Path(path_str)
    need(p.is_absolute(), f"non-absolute path: {path_str}")
    need(str(p) == os.path.normpath(path_str), f"non-canonical path: {path_str}")
    stop = pathlib.Path(stop_root) if stop_root else None
    cur = p
    while True:
        st = os.lstat(cur)
        need(not stat.S_ISLNK(st.st_mode), f"symlink forbidden: {cur}")
        if stop is not None and cur == stop:
            break
        if cur.parent == cur:
            break
        cur = cur.parent


def stable_stat(path_str, stop_root=None):
    check_non_symlink_ancestors(path_str, stop_root=stop_root)
    st = os.lstat(path_str)
    need(stat.S_ISREG(st.st_mode), f"not a regular file: {path_str}")
    return {
        "dev": st.st_dev,
        "ino": st.st_ino,
        "size": st.st_size,
        "mtime_ns": st.st_mtime_ns,
        "ctime_ns": st.st_ctime_ns,
        "mode": st.st_mode,
    }


def read_and_hash_regular_file(path_str, max_bytes, stop_root=None):
    before = stable_stat(path_str, stop_root=stop_root)
    need(1 <= before["size"] <= max_bytes, f"file size {before['size']} outside [1, {max_bytes}]: {path_str}")
    fd = os.open(path_str, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        fst = os.fstat(fd)
        need(stat.S_ISREG(fst.st_mode), f"opened fd not regular: {path_str}")
        need(
            (fst.st_dev, fst.st_ino, fst.st_size, fst.st_mtime_ns, fst.st_ctime_ns, fst.st_mode)
            == (before["dev"], before["ino"], before["size"], before["mtime_ns"], before["ctime_ns"], before["mode"]),
            f"stat mismatch on open: {path_str}",
        )
        h = hashlib.sha256()
        total = 0
        while True:
            chunk = os.read(fd, 65536)
            if not chunk:
                break
            total += len(chunk)
            need(total <= max_bytes, f"read exceeded max_bytes on {path_str}")
            h.update(chunk)
        need(total == before["size"], f"short/long read on {path_str}: {total} != {before['size']}")
    finally:
        os.close(fd)
    after = stable_stat(path_str, stop_root=stop_root)
    need(after == before, f"file mutated during read: {path_str}")
    return before, h.hexdigest()


def load_and_verify_base_manifest(path_str, expected_sha256=EXPECTED_BASE_SHA256, expected_bytes=EXPECTED_BASE_BYTES):
    before = stable_stat(path_str)
    need(
        before["size"] == expected_bytes,
        f"base manifest size mismatch: {before['size']} != {expected_bytes}",
    )
    fd = os.open(path_str, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        raw = b""
        while True:
            chunk = os.read(fd, 65536)
            if not chunk:
                break
            raw += chunk
            need(len(raw) <= expected_bytes, "base manifest grew during read")
    finally:
        os.close(fd)
    after = stable_stat(path_str)
    need(after == before, "base manifest mutated during read")
    got_sha = hashlib.sha256(raw).hexdigest()
    need(
        got_sha == expected_sha256,
        f"base manifest sha256 mismatch: {got_sha} != {expected_sha256}",
    )
    return json.loads(raw.decode("utf-8"), object_pairs_hook=no_dup_pairs)


def validate_row_destination_and_source(row, cargo_root=CARGO_ROOT):
    rel = row.get("destination_relative_to_future_home")
    src = row.get("source")
    cat = row.get("category")
    need(isinstance(rel, str) and rel, "missing destination_relative_to_future_home")
    need(isinstance(src, str) and src, "missing source")
    need(isinstance(cat, str) and cat in EXPECTED_CATEGORY_COUNTS, f"unexpected category: {cat}")
    need(not os.path.isabs(rel), f"destination must be relative: {rel}")
    need(os.path.normpath(rel) == rel, f"non-canonical relative destination: {rel}")
    p = pathlib.PurePosixPath(rel)
    need(".." not in p.parts and "." not in p.parts, f"invalid path parts in {rel}")
    expected_src = f"{cargo_root.rstrip('/')}/{rel}"
    need(src == expected_src, f"source/destination mismatch: {src} != {expected_src}")

    idx_prefix = f"registry/index/{REGISTRY_NAME}/"
    cache_prefix = f"registry/cache/{REGISTRY_NAME}/"
    if cat == "registry-index":
        need(
            rel == f"{idx_prefix}config.json" or rel.startswith(f"{idx_prefix}.cache/"),
            f"invalid registry-index destination: {rel}",
        )
    elif cat == "registry-archive-container":
        need(
            rel.startswith(cache_prefix) and rel.endswith(".crate") and len(p.parts) == 4,
            f"invalid registry-archive-container destination: {rel}",
        )
    else:
        need(rel.startswith(GIT_DB_PREFIX), f"invalid git destination prefix: {rel}")
        tail = rel[len(GIT_DB_PREFIX) :]
        if cat == "git-db-metadata":
            need(
                tail in GIT_METADATA_TAILS or bool(GIT_COMMIT_REF_RE.fullmatch(tail)),
                f"invalid git-db-metadata tail: {tail}",
            )
        elif cat in (
            "git-loose-container-known-commit-tree-or-nonpayload-source",
            "git-loose-unknown",
        ):
            need(bool(GIT_LOOSE_RE.fullmatch(tail)), f"invalid git loose tail: {tail}")
        elif cat == "git-pack-container-or-index":
            need(bool(GIT_PACK_RE.fullmatch(tail)), f"invalid git pack tail: {tail}")
    return rel


def refresh_staging_manifest(doc, cargo_root=CARGO_ROOT, base_sha256=EXPECTED_BASE_SHA256):
    need(isinstance(doc, dict), "manifest root must be dict")
    need(tuple(doc.keys()) == EXPECTED_TOP_KEYS, f"unexpected top-level keys: {list(doc.keys())}")
    need(doc["all_staging_files_have_hashes"] is False, "all_staging_files_have_hashes must be False")
    inputs = doc["inputs"]
    need(isinstance(inputs, list) and len(inputs) == 2243, f"expected 2243 inputs, got {len(inputs) if isinstance(inputs, list) else type(inputs)}")

    counts = doc["counts"]
    need(isinstance(counts, dict), "counts must be dict")
    need(counts.get("total_selected_files") == 2243, "counts.total_selected_files != 2243")
    for k, v in EXPECTED_CATEGORY_COUNTS.items():
        need(counts.get(k) == v, f"counts[{k}] != {v}")

    cat_counter = collections.Counter()
    seen_dests = set()
    new_inputs = []

    control_total_bytes = 0
    noncontrol_total_bytes = 0
    changed_rows = []
    unchanged_control_count = 0
    preserved_noncontrol_count = 0

    for idx, row in enumerate(inputs):
        need(isinstance(row, dict), f"row {idx} not a dict")
        rel = validate_row_destination_and_source(row, cargo_root=cargo_root)
        need(rel not in seen_dests, f"duplicate destination at row {idx}: {rel}")
        seen_dests.add(rel)
        cat = row["category"]
        cat_counter[cat] += 1

        if cat == "git-loose-unknown":
            need(set(row.keys()) == UNKNOWN_LOOSE_ROW_KEYS, f"row {idx} unexpected keys for git-loose-unknown")
            need(row["hash_gap"] == EXPECTED_HASH_GAP, f"row {idx} unexpected hash_gap")
            need("sha256" not in row and "observation" not in row, f"row {idx} must not have sha256/observation")
        else:
            need(set(row.keys()) == NORMAL_ROW_KEYS, f"row {idx} unexpected keys for {cat}")
            need(isinstance(row["sha256"], str) and len(row["sha256"]) == 64, f"row {idx} invalid sha256")

        if cat in SELECTED_CONTROL_CATEGORIES:
            max_b = MAX_FILE_BYTES_BY_CATEGORY[cat]
            st_info, new_sha = read_and_hash_regular_file(
                row["source"], max_bytes=max_b, stop_root=cargo_root
            )
            new_bytes = st_info["size"]
            new_mode = oct(st_info["mode"])
            control_total_bytes += new_bytes
            need(
                control_total_bytes <= MAX_CONTROL_TOTAL_BYTES,
                f"control_total_bytes {control_total_bytes} exceeded cap {MAX_CONTROL_TOTAL_BYTES}",
            )
            size_diff = new_bytes - row["bytes"]
            sha_changed = new_sha != row["sha256"]
            mode_changed = new_mode != row["mode"]
            updated_row = dict(row)
            updated_row["bytes"] = new_bytes
            updated_row["mode"] = new_mode
            updated_row["sha256"] = new_sha
            if size_diff != 0 or sha_changed or mode_changed:
                updated_row["observation"] = (
                    f"refreshed final-current control-metadata observation; "
                    f"updated from {base_sha256[:8]}"
                )
                changed_rows.append(
                    {
                        "index": idx,
                        "category": cat,
                        "destination_relative_to_future_home": rel,
                        "old_bytes": row["bytes"],
                        "new_bytes": new_bytes,
                        "delta_bytes": size_diff,
                        "old_sha256": row["sha256"],
                        "new_sha256": new_sha,
                        "old_mode": row["mode"],
                        "new_mode": new_mode,
                    }
                )
            else:
                unchanged_control_count += 1
            new_inputs.append(updated_row)
        else:
            need(cat in NONSELECTED_CATEGORIES, f"unhandled category: {cat}")
            # lstat-only verification; NEVER open or read non-control files
            st_info = stable_stat(row["source"], stop_root=cargo_root)
            need(
                st_info["size"] == row["bytes"],
                f"non-control row {idx} ({cat}) size drifted: {st_info['size']} != {row['bytes']} ({rel})",
            )
            need(
                oct(st_info["mode"]) == row["mode"],
                f"non-control row {idx} ({cat}) mode drifted: {oct(st_info['mode'])} != {row['mode']} ({rel})",
            )
            noncontrol_total_bytes += row["bytes"]
            preserved_noncontrol_count += 1
            new_inputs.append(copy.deepcopy(row))

    need(dict(cat_counter) == EXPECTED_CATEGORY_COUNTS, f"category counts mismatch: {dict(cat_counter)}")
    need(preserved_noncontrol_count == 488, f"expected 488 non-control rows, got {preserved_noncontrol_count}")
    need(len(changed_rows) + unchanged_control_count == 1755, "control row count != 1755")
    need(
        noncontrol_total_bytes == EXPECTED_NONCONTROL_TOTAL_BYTES,
        f"noncontrol_total_bytes mismatch: {noncontrol_total_bytes} != {EXPECTED_NONCONTROL_TOTAL_BYTES}",
    )
    total_bytes = control_total_bytes + noncontrol_total_bytes
    need(
        total_bytes <= MAX_STAGE_COPY_BYTES,
        f"total staging bytes {total_bytes} exceeds A copy cap {MAX_STAGE_COPY_BYTES}",
    )

    now_utc = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    out_doc = collections.OrderedDict()
    out_doc["observation"] = (
        f"FINAL_CURRENT_STAGING_INPUTS control-metadata refresh from {base_sha256}; "
        f"1755 control-metadata rows hashed ({len(changed_rows)} updated, {unchanged_control_count} unchanged); "
        f"488 non-control rows preserved verbatim without file open."
    )
    out_doc["time_utc"] = now_utc
    out_doc["inputs"] = new_inputs
    out_doc["rust_pins"] = copy.deepcopy(doc["rust_pins"])
    out_doc["native_pins"] = copy.deepcopy(doc["native_pins"])
    out_doc["exclusions"] = copy.deepcopy(doc["exclusions"])
    out_doc["gaps"] = copy.deepcopy(doc["gaps"])
    out_doc["counts"] = copy.deepcopy(doc["counts"])
    out_doc["all_staging_files_have_hashes"] = False
    out_doc["scope"] = copy.deepcopy(doc["scope"])

    summary = {
        "status": "OK",
        "base_manifest_sha256": base_sha256,
        "total_rows": 2243,
        "selected_control_rows_hashed": 1755,
        "control_rows_changed_count": len(changed_rows),
        "control_rows_unchanged_count": unchanged_control_count,
        "noncontrol_rows_preserved_unopened": preserved_noncontrol_count,
        "control_total_bytes": control_total_bytes,
        "noncontrol_total_bytes": noncontrol_total_bytes,
        "total_staging_bytes": total_bytes,
        "stage_copy_cap_bytes": MAX_STAGE_COPY_BYTES,
        "headroom_under_stage_copy_cap_bytes": MAX_STAGE_COPY_BYTES - total_bytes,
        "changed_rows": changed_rows,
    }
    return out_doc, summary


def atomic_write_readonly_json(path_str, payload_obj):
    p = pathlib.Path(path_str)
    need(p.is_absolute(), f"output path must be absolute: {path_str}")
    need(str(p) == os.path.normpath(path_str), f"non-canonical output path: {path_str}")
    check_non_symlink_ancestors(str(p.parent))
    raw = (json.dumps(payload_obj, indent=2) + "\n").encode("utf-8")
    fd = os.open(path_str, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC, 0o444)
    try:
        view = memoryview(raw)
        written = 0
        while written < len(raw):
            n = os.write(fd, view[written:])
            need(n > 0, f"zero write to {path_str}")
            written += n
        os.fsync(fd)
        os.fchmod(fd, 0o444)
    finally:
        os.close(fd)
    return len(raw), hashlib.sha256(raw).hexdigest()


class SelfTests(unittest.TestCase):
    """Deterministic in-memory/tempdir supplied-data unit checks."""

    def _build_synthetic_fixture(self, root_dir):
        inputs = []
        # 1735 registry-index rows (row 0 = config.json, rows 1..1734 = .cache/...)
        for i in range(1735):
            if i == 0:
                rel = f"registry/index/{REGISTRY_NAME}/config.json"
            else:
                rel = f"registry/index/{REGISTRY_NAME}/.cache/aa/bb/crate_{i:04d}"
            full = os.path.join(root_dir, rel)
            os.makedirs(os.path.dirname(full), exist_ok=True)
            content = f"index-{i}\n".encode("utf-8")
            with open(full, "wb") as f:
                f.write(content)
            st = os.lstat(full)
            inputs.append(
                {
                    "bytes": len(content),
                    "category": "registry-index",
                    "destination_relative_to_future_home": rel,
                    "mode": oct(st.st_mode),
                    "observation": "final-current staging input observation",
                    "sha256": hashlib.sha256(content).hexdigest(),
                    "source": full,
                }
            )
        # 423 registry-archive-container rows
        for i in range(423):
            rel = f"registry/cache/{REGISTRY_NAME}/crate_{i:04d}-1.0.0.crate"
            full = os.path.join(root_dir, rel)
            os.makedirs(os.path.dirname(full), exist_ok=True)
            content = b"A" * 10
            with open(full, "wb") as f:
                f.write(content)
            st = os.lstat(full)
            inputs.append(
                {
                    "bytes": len(content),
                    "category": "registry-archive-container",
                    "destination_relative_to_future_home": rel,
                    "mode": oct(st.st_mode),
                    "observation": "final-current staging input observation",
                    "sha256": "0" * 64,
                    "source": full,
                }
            )
        # 20 git-db-metadata rows
        meta_tails = [
            "HEAD",
            "config",
            "FETCH_HEAD",
            "description",
            "hooks/README.sample",
            "info/exclude",
        ] + [f"refs/commit/{i:040x}" for i in range(14)]
        for i, tail in enumerate(meta_tails):
            rel = f"{GIT_DB_PREFIX}{tail}"
            full = os.path.join(root_dir, rel)
            os.makedirs(os.path.dirname(full), exist_ok=True)
            content = f"meta-{i}\n".encode("utf-8")
            with open(full, "wb") as f:
                f.write(content)
            st = os.lstat(full)
            inputs.append(
                {
                    "bytes": len(content),
                    "category": "git-db-metadata",
                    "destination_relative_to_future_home": rel,
                    "mode": oct(st.st_mode),
                    "observation": "final-current staging input observation",
                    "sha256": hashlib.sha256(content).hexdigest(),
                    "source": full,
                }
            )
        # 6 known loose rows
        for i in range(6):
            rel = f"{GIT_DB_PREFIX}objects/00/{i:038x}"
            full = os.path.join(root_dir, rel)
            os.makedirs(os.path.dirname(full), exist_ok=True)
            content = b"K" * 8
            with open(full, "wb") as f:
                f.write(content)
            st = os.lstat(full)
            inputs.append(
                {
                    "bytes": len(content),
                    "category": "git-loose-container-known-commit-tree-or-nonpayload-source",
                    "destination_relative_to_future_home": rel,
                    "mode": oct(st.st_mode),
                    "observation": "final-current staging input observation",
                    "sha256": "1" * 64,
                    "source": full,
                }
            )
        # 52 unknown loose rows (hash_gap, no sha256/observation)
        for i in range(52):
            rel = f"{GIT_DB_PREFIX}objects/ff/{i:038x}"
            full = os.path.join(root_dir, rel)
            os.makedirs(os.path.dirname(full), exist_ok=True)
            content = b"U" * 9
            with open(full, "wb") as f:
                f.write(content)
            st = os.lstat(full)
            inputs.append(
                {
                    "bytes": len(content),
                    "category": "git-loose-unknown",
                    "destination_relative_to_future_home": rel,
                    "hash_gap": EXPECTED_HASH_GAP,
                    "mode": oct(st.st_mode),
                    "source": full,
                }
            )
        # 7 pack rows (sized so noncontrol total == EXPECTED_NONCONTROL_TOTAL_BYTES in metadata)
        rem = EXPECTED_NONCONTROL_TOTAL_BYTES - (423 * 10 + 6 * 8 + 52 * 9)
        for i in range(7):
            ext = "pack" if i % 2 == 0 else "idx"
            rel = f"{GIT_DB_PREFIX}objects/pack/pack-{i:040x}.{ext}"
            full = os.path.join(root_dir, rel)
            os.makedirs(os.path.dirname(full), exist_ok=True)
            sz = (rem // 7) + (rem % 7 if i == 0 else 0)
            with open(full, "wb") as f:
                f.truncate(sz)
            st = os.lstat(full)
            inputs.append(
                {
                    "bytes": sz,
                    "category": "git-pack-container-or-index",
                    "destination_relative_to_future_home": rel,
                    "mode": oct(st.st_mode),
                    "observation": "final-current staging input observation",
                    "sha256": "2" * 64,
                    "source": full,
                }
            )
        doc = collections.OrderedDict(
            [
                ("observation", "orig"),
                ("time_utc", "2026-10-16T00:00:00Z"),
                ("inputs", inputs),
                ("rust_pins", {"cargo_lock": "dfffb853"}),
                ("native_pins", {"sdk": "9f48d1f2"}),
                ("exclusions", []),
                ("gaps", []),
                (
                    "counts",
                    {
                        "total_selected_files": 2243,
                        **EXPECTED_CATEGORY_COUNTS,
                    },
                ),
                ("all_staging_files_have_hashes", False),
                ("scope", {"future_home": True}),
            ]
        )
        return doc

    def test_1_refresh_updates_size_and_same_size_etag_while_never_opening_noncontrol(self):
        with tempfile.TemporaryDirectory() as tmp:
            doc = self._build_synthetic_fixture(tmp)
            # Mutate 1 registry-index file with size growth and 1 with same-size content change
            grow_path = doc["inputs"][1]["source"]
            with open(grow_path, "wb") as f:
                f.write(b"grown-index-payload-extra-bytes\n")
            same_size_path = doc["inputs"][2]["source"]
            old_len = doc["inputs"][2]["bytes"]
            with open(same_size_path, "wb") as f:
                f.write(b"Z" * old_len)
            # Chmod all 488 non-control files to 0o000 would change mode, so instead wrap os.open
            real_open = os.open
            opened_paths = []

            def tracked_open(p, flags, *args, **kwargs):
                opened_paths.append(p)
                return real_open(p, flags, *args, **kwargs)

            os.open = tracked_open
            try:
                out_doc, summary = refresh_staging_manifest(
                    doc, cargo_root=tmp, base_sha256=EXPECTED_BASE_SHA256
                )
            finally:
                os.open = real_open

            self.assertEqual(len(opened_paths), 1755)
            self.assertEqual(summary["control_rows_changed_count"], 2)
            self.assertEqual(summary["control_rows_unchanged_count"], 1753)
            self.assertEqual(summary["noncontrol_rows_preserved_unopened"], 488)
            # Verify 52 git-loose-unknown rows keep hash_gap and no sha256
            unknowns = [r for r in out_doc["inputs"] if r["category"] == "git-loose-unknown"]
            self.assertEqual(len(unknowns), 52)
            for r in unknowns:
                self.assertEqual(set(r.keys()), UNKNOWN_LOOSE_ROW_KEYS)

    def test_2_refuses_noncontrol_size_drift(self):
        with tempfile.TemporaryDirectory() as tmp:
            doc = self._build_synthetic_fixture(tmp)
            crate_row = next(r for r in doc["inputs"] if r["category"] == "registry-archive-container")
            with open(crate_row["source"], "ab") as f:
                f.write(b"drift")
            with self.assertRaisesRegex(Refusal, "non-control row .* size drifted"):
                refresh_staging_manifest(doc, cargo_root=tmp)

    def test_3_refuses_symlink_in_control_row(self):
        with tempfile.TemporaryDirectory() as tmp:
            doc = self._build_synthetic_fixture(tmp)
            idx_path = doc["inputs"][1]["source"]
            os.unlink(idx_path)
            os.symlink("/etc/passwd", idx_path)
            with self.assertRaisesRegex(Refusal, "symlink forbidden"):
                refresh_staging_manifest(doc, cargo_root=tmp)

    def test_4_refuses_duplicate_destination(self):
        with tempfile.TemporaryDirectory() as tmp:
            doc = self._build_synthetic_fixture(tmp)
            doc["inputs"][2]["destination_relative_to_future_home"] = doc["inputs"][1]["destination_relative_to_future_home"]
            doc["inputs"][2]["source"] = doc["inputs"][1]["source"]
            with self.assertRaisesRegex(Refusal, "duplicate destination"):
                refresh_staging_manifest(doc, cargo_root=tmp)


def main(argv=None):
    apply_process_bounds()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="Run deterministic supplied-data self-tests and exit.")
    parser.add_argument("--base-manifest", help="Absolute path to pinned FINAL_CURRENT_STAGING_INPUTS.json")
    parser.add_argument("--output-manifest", help="Absolute path to new read-only refreshed manifest JSON")
    parser.add_argument("--summary-output", help="Optional absolute path to write summary JSON")
    args = parser.parse_args(argv)

    if args.self_test:
        suite = unittest.defaultTestLoader.loadTestsFromTestCase(SelfTests)
        result = unittest.TextTestRunner(stream=sys.stderr, verbosity=2).run(suite)
        if not result.wasSuccessful():
            return 1
        sys.stdout.write(json.dumps({"self_test": "PASS", "tests_run": result.testsRun}) + "\n")
        return 0

    need(bool(args.base_manifest and args.output_manifest), "--base-manifest and --output-manifest are required")
    need(args.base_manifest != args.output_manifest, "output-manifest must differ from base-manifest")

    base_doc = load_and_verify_base_manifest(args.base_manifest)
    out_doc, summary = refresh_staging_manifest(base_doc)
    out_bytes, out_sha256 = atomic_write_readonly_json(args.output_manifest, out_doc)
    summary["output_manifest_path"] = args.output_manifest
    summary["output_manifest_bytes"] = out_bytes
    summary["output_manifest_sha256"] = out_sha256

    if args.summary_output:
        need(args.summary_output not in (args.base_manifest, args.output_manifest), "distinct --summary-output required")
        atomic_write_readonly_json(args.summary_output, summary)

    compact_stdout = {
        "status": summary["status"],
        "base_manifest_sha256": summary["base_manifest_sha256"],
        "output_manifest_path": summary["output_manifest_path"],
        "output_manifest_bytes": summary["output_manifest_bytes"],
        "output_manifest_sha256": summary["output_manifest_sha256"],
        "selected_control_rows_hashed": summary["selected_control_rows_hashed"],
        "control_rows_changed_count": summary["control_rows_changed_count"],
        "control_rows_unchanged_count": summary["control_rows_unchanged_count"],
        "noncontrol_rows_preserved_unopened": summary["noncontrol_rows_preserved_unopened"],
        "control_total_bytes": summary["control_total_bytes"],
        "noncontrol_total_bytes": summary["noncontrol_total_bytes"],
        "total_staging_bytes": summary["total_staging_bytes"],
        "headroom_under_stage_copy_cap_bytes": summary["headroom_under_stage_copy_cap_bytes"],
    }
    sys.stdout.write(json.dumps(compact_stdout, sort_keys=True) + "\n")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Refusal as exc:
        sys.stderr.write(f"REFUSAL: {exc}\n")
        sys.exit(2)
