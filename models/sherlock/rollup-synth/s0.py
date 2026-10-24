"""Frozen PUBLIC_SYNTHETIC S0 corpus replay and independent envelope encoding.
No Poseidon, key/proof loading, SDK, subprocess, financial acceptance or release.
"""
from pathlib import Path
import struct
import tempfile
from common import Error, canonical, digest, load_json, parse_json, regular, relative, sha
from cut import _document, check_cut
from frtf import encode_view, th, validate

SEED = 0x5354524157
VERSION = "fsyn/2.0"

class VerificationError(Error):
    """A committed fixture differs from the independently regenerated payload."""
    pass

def _source(cut, golden):
    checked = check_cut(cut)
    doc = _document(cut)
    golden = Path(golden).resolve()
    bound = {str(Path(row["path"]).expanduser().resolve()): row for rows in doc["groups"].values() for row in rows}
    # All bytes read as corpus authority must be direct members of the checked cut.
    for name in ("MANIFEST.json", "params-v3.json", "workloads.json"):
        if str(golden / name) not in bound: raise Error("golden authority absent from source cut: " + name)
    manifest = load_json(golden / "MANIFEST.json")
    if not isinstance(manifest, dict) or not manifest: raise Error("invalid FRTF manifest")
    payload = {}
    for name, expected in manifest.items():
        relative(name)
        sha(expected)
        if not name.endswith((".json", ".jsonl", ".frtf")): raise Error("unexpected corpus member kind")
        path = golden / name
        if not path.resolve().is_relative_to(golden): raise Error("corpus member escapes root")
        raw = regular(path)
        if digest(raw) != expected: raise Error("stale golden member: " + name)
        payload[name] = raw
    return checked, payload

def generate(cut, golden, out, *, seed=SEED, profile="both", scale="S0", mix="strawman"):
    if type(seed) is not int or seed != SEED or profile != "both" or scale != "S0" or mix != "strawman":
        raise Error("this first slice supports only frozen S0 seed 0x5354524157, both profiles, strawman mix")
    out = Path(out)
    if out.exists(): raise Error("fixture destination already exists")
    checked, payload = _source(cut, golden)
    rebuilt = 0
    for name in sorted(payload):
        if name.startswith(("profile-a/", "profile-b/")) and name.endswith(".frtf"):
            view_name = name[:-5] + ".json"
            if view_name not in payload: raise Error("missing envelope view: " + view_name)
            view = parse_json(payload[view_name])
            try:
                encoded = encode_view(view)
                validate(encoded)
            except ValueError as exc: raise Error("envelope reconstruction failed: " + name + ": " + str(exc)) from exc
            if encoded != payload[name]: raise Error("independent FRTF encoding differs: " + name)
            payload[name] = encoded
            rebuilt += 1
    identity = {"cut_digest": checked["cut_digest"], "generator_version": VERSION, "seed": seed,
                "profile": profile, "scale": scale, "mix": mix}
    fid = th("moriarty/fsyn/fixture/v1", bytes.fromhex(checked["cut_digest"]), VERSION.encode("utf-8"),
             struct.pack("<Q", seed), profile.encode("utf-8"), scale.encode("utf-8"), mix.encode("utf-8")).hex()
    meta = {"schema": "fsyn-s0-fixture/1", "fixture_id": fid, **identity,
            "public_synthetic_only": True,
            "release_status": "unreleased", "golden_members": len(payload), "positive_envelopes_rebuilt": rebuilt,
            "scope": "full public golden replay; independent positive envelope encoding and hash-free checks",
            "checks": {"golden_bytes": "pass", "positive_envelope_encoding": "pass", "hash_free_validation": "pass",
                       "rtf_verify": "not-run", "poseidon_recomputation": "not-run", "quint": "not-run", "cross_host": "not-run"},
            "recipe": {"command": "python3 models/sherlock/rollup-synth/fsyn.py generate", "seed": "0x5354524157",
                       "scale": scale, "profile": profile, "mix": mix,
                       "cut": "openspec/changes/sherlock-synthetic-rollup-data/design-cut.json",
                       "golden_root": "~/projectfenrir/recursion/rollup-test-format/v3", "out": "NEW_OUTPUT_DIRECTORY"},
            "recipe_is_not_an_actual_execution_receipt": True}
    files = {"corpus/" + name: raw for name, raw in payload.items()}
    files["corpus/MANIFEST.json"] = regular(Path(golden) / "MANIFEST.json")
    files["FIXTURE.json"] = canonical(meta) + b"\n"
    rows = [{"path": name, "bytes": len(raw), "sha256": digest(raw)} for name, raw in sorted(files.items())]
    manifest = {"schema": "fsyn-fixture-manifest/1", "fixture_id": fid, "files": rows,
                "content_digest": digest(canonical(rows))}
    files["MANIFEST.json"] = canonical(manifest) + b"\n"
    # Validate every source before creating output. Exclusive writes preserve partial output on IO failure.
    out.mkdir(parents=True, exist_ok=False)
    for name, raw in sorted(files.items()):
        path = out / name
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open("xb") as stream: stream.write(raw)
    return {"fixture_id": fid, "files": len(files), "golden_members": len(payload), "positive_envelopes_rebuilt": rebuilt,
            "status": "internal-replay-pass", "release_status": "unreleased"}

def verify(fixture, cut, golden):
    fixture = Path(fixture)
    if not fixture.is_dir() or fixture.is_symlink(): raise Error("invalid fixture directory")
    with tempfile.TemporaryDirectory(prefix="fsyn-replay-") as directory:
        expected = Path(directory) / "expected"
        report = generate(cut, golden, expected)
        want = {str(p.relative_to(expected)): p.read_bytes() for p in expected.rglob("*") if p.is_file()}
        observed = {}
        for path in fixture.rglob("*"):
            if path.is_symlink(): raise Error("fixture symlinks are not members")
            if path.is_file(): observed[str(path.relative_to(fixture))] = regular(path)
            elif not path.is_dir(): raise Error("unexpected fixture member")
        if observed.keys() != want.keys(): raise VerificationError("fixture membership differs")
        for name in want:
            if observed[name] != want[name]: raise VerificationError("fixture bytes differ: " + name)
        return report
