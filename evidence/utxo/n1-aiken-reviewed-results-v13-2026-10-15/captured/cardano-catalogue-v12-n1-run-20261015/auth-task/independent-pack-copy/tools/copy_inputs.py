"""Copy pinned DATA inputs (no checker source) into inputs/ and record hashes. Refuses to overwrite.

Usage: python3 -I tools/copy_inputs.py
"""
import datetime
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
V = Path("/home/charl/moriarty-eutxo-validation-2026-10-12")
SRC = {
    "catalogue-v1.1.json": (V / "cardano-catalogue-v11-hardening-20261015/candidate/data/CARDANO-PROPERTY-CATALOGUE-v1.1.json", None),
    "manifest-v1.1.json": (V / "cardano-catalogue-v11-hardening-20261015/candidate/data/CATALOGUE-v1.1-MANIFEST.json", None),
    "v0.json": (V / "cardano-proof-catalogue-20261015/CARDANO-PROPERTY-CATALOGUE.json",
                "a283133b87745999be91e797d205f05964793642f2201766111d57af153fe2b9"),
    "v1.json": (V / "cardano-catalogue-v1-implementation-20261015/CARDANO-PROPERTY-CATALOGUE-v1.json",
                "4cc79c59cd9adc2ba78c64102f0850d43117522b2ea56e6c0e26a612bd9198c4"),
}


def main():
    out = ROOT / "inputs"
    out.mkdir(exist_ok=True)
    rec = {"copiedAt": datetime.datetime.now(datetime.timezone.utc).isoformat(), "files": {}}
    for name, (src, pin) in SRC.items():
        b = src.read_bytes()
        h = hashlib.sha256(b).hexdigest()
        if pin and h != pin:
            raise SystemExit(f"{src} sha256 {h} != pin {pin}")
        dst = out / name
        if dst.exists():
            raise SystemExit(f"{dst} exists; refusing to overwrite")
        dst.write_bytes(b)
        if hashlib.sha256(dst.read_bytes()).hexdigest() != h:
            raise SystemExit("copy mismatch")
        rec["files"][name] = {"source": str(src), "sha256": h, "bytes": len(b), "pinnedBy": pin and "INPUT-PINS.json/pins.py"}
    (out / "COPY-RECORD.json").write_text(json.dumps(rec, indent=1) + "\n")
    print(json.dumps(rec, indent=1))


if __name__ == "__main__":
    main()
