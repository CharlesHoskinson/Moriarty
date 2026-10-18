from pathlib import Path
import hashlib, json
root = Path(__file__).resolve().parent
sha = lambda data: hashlib.sha256(data).hexdigest()
pins = json.loads((root / "SOURCE-PINS.json").read_text())
for pin in pins["files"]:
    data = (root / "source" / pin["path"]).read_bytes()
    assert sha(data) == pin["sha256"], pin["path"]
    header = ("blob " + str(len(data)) + "\0").encode()
    assert hashlib.sha1(header + data).hexdigest() == pin["gitBlob"], pin["path"]
for relative in ["print-run-independent-oracle", "source/crates/fixtures/financial-report-contract"]:
    packet = root / relative
    for line in (packet / "FREEZE.sha256").read_text().splitlines():
        digest, name = line.split("  ", 1)
        assert sha((packet / name).read_bytes()) == digest, name
print(json.dumps({"status": "STATIC_PUBLIC_SOURCE_BLOB_AND_ORACLE_PINS_MATCH", "sourceFiles": len(pins["files"]), "productExecution": 0}))
