"""Frozen in-repository S0 golden (SSRD-001, SSRD-005, SSRD-030).

The golden is the PUBLIC_SYNTHETIC S0 corpus under `golden/s0/`, with its FRTF
`MANIFEST.json`. `golden/s0.json` pins the manifest, the corpus authorities and the
member count; its `golden_digest` is the fixture authority. Every byte is checked
before use. Nothing outside the golden root is read.
"""
from pathlib import Path
from common import Error, canonical, digest, load_json, regular, relative, sha

HERE = Path(__file__).resolve().parent
ROOT = HERE / "golden/s0"
PIN = HERE / "golden/s0.json"
SCHEMA = "fsyn-s0-golden/1"
AUTHORITIES = ("params-v3.json", "workloads.json")
KINDS = (".json", ".jsonl", ".frtf")


def _row(raw):
    return {"bytes": len(raw), "sha256": digest(raw)}


def _digest(doc):
    return digest(canonical({k: v for k, v in doc.items() if k != "golden_digest"}))


def _members(root):
    """Every regular file under root, refusing symlinks and special files."""
    found = {}
    for path in sorted(root.rglob("*")):
        if path.is_symlink(): raise Error("golden symlinks are not members: " + str(path))
        if path.is_file(): found[path.relative_to(root).as_posix()] = path
        elif not path.is_dir(): raise Error("unexpected golden member: " + str(path))
    return found


def load(root=ROOT, pin=PIN):
    """Return (golden_digest, payload, manifest bytes) after checking every pin."""
    doc = load_json(pin)
    if not isinstance(doc, dict) or doc.get("schema") != SCHEMA: raise Error("unknown golden pin schema")
    if sha(doc.get("golden_digest")) != _digest(doc): raise Error("golden pin digest mismatch")
    root = Path(root).resolve()
    if not root.is_dir(): raise Error("golden root is not a directory")
    manifest_raw = regular(root / "MANIFEST.json")
    if _row(manifest_raw) != doc.get("manifest"): raise Error("stale golden member: MANIFEST.json")
    manifest = load_json(root / "MANIFEST.json")
    if not isinstance(manifest, dict) or not manifest: raise Error("invalid FRTF manifest")
    if len(manifest) != doc.get("members"): raise Error("golden member count differs from pin")
    payload = {}
    for name, expected in manifest.items():
        relative(name)
        sha(expected)
        if not name.endswith(KINDS): raise Error("unexpected corpus member kind")
        path = root / name
        if not path.resolve().is_relative_to(root): raise Error("corpus member escapes root")
        raw = regular(path)
        if digest(raw) != expected: raise Error("stale golden member: " + name)
        payload[name] = raw
    authorities = doc.get("authorities")
    if not isinstance(authorities, dict) or sorted(authorities) != sorted(AUTHORITIES):
        raise Error("golden pin must name the corpus authorities")
    for name in AUTHORITIES:
        if name not in payload or _row(payload[name]) != authorities[name]: raise Error("stale golden member: " + name)
    extra = sorted(set(_members(root)) - set(payload) - {"MANIFEST.json"})
    if extra: raise Error("unlisted golden member: " + extra[0])
    return doc["golden_digest"], payload, manifest_raw


def pin_document(root, provenance):
    """Pin a golden tree. Used once when a golden is frozen; never by generation."""
    root = Path(root).resolve()
    manifest = load_json(root / "MANIFEST.json")
    files = {name: regular(root / name) for name in manifest}
    doc = {"schema": SCHEMA, "scale": "S0", "seed": "0x5354524157",
           "manifest": _row(regular(root / "MANIFEST.json")), "members": len(manifest),
           "authorities": {name: _row(files[name]) for name in AUTHORITIES},
           "provenance": provenance}
    doc["golden_digest"] = _digest(doc)
    return doc


def check(root=ROOT, pin=PIN):
    golden_digest, payload, _ = load(root, pin)
    return {"schema": "fsyn-golden-check/1", "golden_digest": golden_digest, "members": len(payload),
            "bytes": sum(len(raw) for raw in payload.values()), "status": "pins-match"}
