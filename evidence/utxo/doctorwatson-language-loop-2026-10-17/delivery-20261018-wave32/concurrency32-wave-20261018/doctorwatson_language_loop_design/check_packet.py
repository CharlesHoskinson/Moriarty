from pathlib import Path
import hashlib
import json

root = Path(__file__).resolve().parent
pins = json.loads((root / "SOURCE-PINS.json").read_text())
for item in pins["files"]:
    content = (root / "source" / item["path"]).read_bytes()
    assert len(content) == item["bytes"], item["path"]
    assert hashlib.sha256(content).hexdigest() == item["sha256"], item["path"]
for name in ["start.report.expected.json", "delivery-false.report.expected.json"]:
    path = root / "source/crates/fixtures/core2-escrow-complete" / name
    report = json.loads(path.read_text())
    assert set(report) == {
        "status", "advanced", "segment", "continuation", "attempted",
        "committed", "residual", "qualification", "authentication", "consumption",
    }
    assert report["status"] == "waiting"
    assert report["committed"] == report["continuation"]["logical"]["committed"]
    assert report["qualification"] == "local-reference-stage"
    assert report["authentication"] == "none"
    assert report["consumption"] == "untracked"
print("Static source hashes and existing full waiting fixture shapes checked; product predicate UNRUN.")
